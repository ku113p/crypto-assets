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
    auth_token: String,
}

impl BalanceStore {
    pub fn new(app_state: Arc<AppState>, auth_token: String) -> Self {
        Self { app_state, auth_token }
    }

    pub async fn list(&self) -> Vec<BalanceView> {
        let storage = self.app_state.storage.lock().await;
        let Some(ws) = storage.get(&self.auth_token) else {
            return vec![];
        };
        ws.balances.iter()
            .filter_map(|b| {
                ws.tokens
                    .get(&b.token_id)
                    .map(|token| BalanceView::new(token.symbol.clone(), b.amount))
            })
            .collect()
    }

    pub async fn create_or_update(&self, balance: BalanceView) -> bool {
        let mut storage = self.app_state.storage.lock().await;
        let ws = storage.get_or_create(&self.auth_token);

        let token_id = ws.get_or_create_token_id(&balance.symbol);

        if let Some(existing) = ws.balances.iter_mut().find(|b| b.token_id == token_id) {
            existing.amount = balance.amount;
            false
        } else {
            ws.balances.push(Balance::new(token_id, balance.amount));
            let mut balances = ws.balances.clone();
            balances.sort_by_key(
                |b| ws.get_token_symbol(&b.token_id).unwrap_or_default()
            );
            ws.balances = balances;
            true
        }
    }

    pub async fn remove(&self, symbol: String) -> bool {
        let mut storage = self.app_state.storage.lock().await;
        let Some(ws) = storage.get_mut(&self.auth_token) else {
            return false;
        };

        if let Some(token_id) = ws.get_token_id(&symbol) {
            ws.balances.retain(|b| b.token_id != token_id);
            return true;
        }

        false
    }
}
