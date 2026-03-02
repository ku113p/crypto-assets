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
    auth_token: String,
}

impl AllocationStore {
    pub fn new(app_state: Arc<AppState>, auth_token: String) -> Self {
        Self { app_state, auth_token }
    }

    pub async fn list(&self) -> Vec<AllocationView> {
        let storage = self.app_state.storage.lock().await;
        let Some(ws) = storage.get(&self.auth_token) else {
            return vec![];
        };
        ws.allocations.iter()
            .filter_map(|a| {
                let token = ws.tokens.get(&a.token_id)?;
                let scheme = ws.schemes.get(&a.scheme_id)?;
                Some(AllocationView::new(scheme.name.clone(), token.symbol.clone(), a.amount))
            })
            .collect()
    }

    pub async fn create_or_update(&self, allocation: AllocationView) -> bool {
        let mut storage = self.app_state.storage.lock().await;
        let ws = storage.get_or_create(&self.auth_token);

        let token_id = ws.get_or_create_token_id(&allocation.symbol);
        let scheme_id = ws.get_or_create_scheme_id(&allocation.scheme_name);

        if let Some(existing) = ws.allocations.iter_mut()
            .find(|a| a.token_id == token_id && a.scheme_id == scheme_id) {
            existing.amount = allocation.amount;
            false
        } else {
            ws.allocations.push(Allocation::new(token_id, scheme_id, allocation.amount));
            let mut allocations = ws.allocations.clone();
            allocations.sort_by_key(|a| (
                ws.get_scheme_name(&a.scheme_id).unwrap_or_default(),
                ws.get_token_symbol(&a.token_id).unwrap_or_default()
            ));
            ws.allocations = allocations;
            true
        }
    }

    pub async fn remove(&self, scheme_name: String, symbol: String) -> bool {
        let mut storage = self.app_state.storage.lock().await;
        let Some(ws) = storage.get_mut(&self.auth_token) else {
            return false;
        };

        if let Some(scheme_id) = ws.get_scheme_id(&scheme_name) {
            if let Some(token_id) = ws.get_token_id(&symbol) {
                ws.allocations.retain(|a| !(a.token_id == token_id && a.scheme_id == scheme_id));
                return true;
            }
        }
        false
    }
}
