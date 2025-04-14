use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::models::models::Allocation;

#[derive(Serialize, Deserialize, Clone)]
pub struct AllocationView {
    pub scheme_name: String,
    pub symbol: String,
    pub amount: f32,
}

impl AllocationView {
    fn new(scheme_name: String, symbol: String, amount: f32) -> Self {
        Self { scheme_name, symbol, amount }
    }
}

pub struct AllocationStore {
    app_state: Arc<AppState>,
}

impl AllocationStore {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn list(&self) -> Vec<AllocationView> {
        let storage = self.app_state.storage.lock().await;
        storage.allocations.iter()
            .filter_map(|a| {
                let token = storage.tokens.get(&a.token_id)?;
                let scheme = storage.schemes.get(&a.scheme_id)?;
                Some(AllocationView::new(scheme.name.clone(), token.symbol.clone(), a.amount))
            })
            .collect()
    }

    pub async fn create_or_update(&self, allocation: AllocationView) -> bool {
        let mut storage = self.app_state.storage.lock().await;

        let token_id = storage.get_or_create_token_id(&allocation.symbol);
        let scheme_id = storage.get_or_create_scheme_id(&allocation.scheme_name);

        if let Some(existing) = storage.allocations.iter_mut()
            .find(|a| a.token_id == token_id && a.scheme_id == scheme_id) {
            existing.amount = allocation.amount;
            false
        } else {
            storage.allocations.push(Allocation::new(token_id, scheme_id, allocation.amount));
            let mut allocations = storage.allocations.clone();
            allocations.sort_by_key(|a| (
                storage.get_scheme_name(&a.scheme_id).unwrap_or_default(),
                storage.get_token_symbol(&a.token_id).unwrap_or_default()
            ));
            storage.allocations = allocations;
            true
        }
    }

    pub async fn remove(&self, scheme_name: String, symbol: String) -> bool {
        let mut storage = self.app_state.storage.lock().await;

        if let Some(scheme_id) = storage.get_scheme_id(&scheme_name) {
            if let Some(token_id) = storage.get_token_id(&symbol) {
                storage.allocations.retain(|a| !(a.token_id == token_id && a.scheme_id == scheme_id));
                return true;
            }
        }
        false
    }
}
