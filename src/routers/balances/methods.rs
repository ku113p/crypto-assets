use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::models::models::Balance;

#[derive(Serialize, Deserialize, Clone)]
pub struct BalanceView {
    pub symbol: String,
    pub amount: f32,
}

impl BalanceView {
    fn new(symbol: String, amount: f32) -> Self {
        Self { symbol, amount }
    }
}

pub struct BalanceStore {
    app_state: Arc<AppState>,
}

impl BalanceStore {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn list(&self) -> Vec<BalanceView> {
        let storage = self.app_state.storage.lock().await;
        storage.balances.iter()
            .filter_map(|b| {
                storage.tokens
                    .get(&b.token_id)
                    .map(|token| BalanceView::new(token.symbol.clone(), b.amount))
            })
            .collect()
    }

    pub async fn create_or_update(&self, balance: BalanceView) -> bool {
        let mut storage = self.app_state.storage.lock().await;

        let token_id = storage.get_or_create_token_id(&balance.symbol);

        if let Some(existing) = storage.balances.iter_mut().find(|b| b.token_id == token_id) {
            existing.amount = balance.amount;
            false
        } else {
            storage.balances.push(Balance::new(token_id, balance.amount));
            let mut balances = storage.balances.clone();
            balances.sort_by_key(
                |b| storage.get_token_symbol(&b.token_id).unwrap_or_default()
            );
            storage.balances = balances;
            true
        }
    }

    pub async fn remove(&self, symbol: String) -> bool {
        let mut storage = self.app_state.storage.lock().await;

        if let Some(token_id) = storage.get_token_id(&symbol) {
            storage.balances.retain(|b| b.token_id != token_id);
            return true;
        }

        false
    }
}
