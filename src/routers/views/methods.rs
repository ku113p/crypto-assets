use derive_builder::Builder;
use std::collections::HashMap;
use std::sync::Arc;
use log::error;
use serde::Serialize;
use crate::app_state::AppState;

pub struct Views {
    app_state: Arc<AppState>,
}

#[derive(Debug, Default, Builder, Serialize)]
pub struct TokenInfo {
    pub symbol: String,
    pub amount: f32,
    pub defi_amount: f32,
    pub defi_self_percentage: f32,
    pub usdt_rate: f32,
    pub usdt_amount: f32,
    pub usdt_percentage: f32,
    pub usdt_defi_amount: f32,
    pub usdt_defi_percentage: f32,
}

impl TokenInfo {
    fn new_broken(name: Option<String>) -> Self {
        let mut token_info = TokenInfo::default();
        token_info.symbol = name.unwrap_or("BROKEN".to_string());

        token_info
    }

    fn builder_with_default() -> TokenInfoBuilder {
        TokenInfoBuilder::default()
            .symbol("null".to_string())
            .amount(0.0)
            .defi_amount(0.0)
            .defi_self_percentage(0.0)
            .usdt_rate(0.0)
            .usdt_amount(0.0)
            .usdt_percentage(0.0)
            .usdt_defi_amount(0.0)
            .usdt_defi_percentage(0.0)
            .to_owned()
    }
}

impl Views {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn info(&self) -> Vec<TokenInfo> {
        let storage = self.app_state.storage.lock().await;

        let mut builders = storage.balances.iter()
            .map(|b| (b.token_id, TokenInfo::builder_with_default()
                .amount(b.amount)
                .symbol(storage.tokens
                    .get(&b.token_id)
                    .map(|t| t.symbol.to_owned())
                    .unwrap_or_default()
                ).to_owned()
            ))
            .collect::<HashMap<_, _>>();

        for a in storage.allocations.iter() {
            if let Some(b) = builders.get_mut(&a.token_id) {
                b.defi_amount(a.amount + b.defi_amount.unwrap_or_default());
            }
        }

        for b in builders.values_mut() {
            let usdt_rate = 1.0;
            let amount = b.amount.unwrap_or_default();
            let usdt_amount = usdt_rate * amount;
            let defi_amount = b.defi_amount.unwrap_or_default();
            let usdt_defi_amount = usdt_rate * defi_amount;
            let defi_self_percentage = defi_amount / amount;

            b.usdt_rate(usdt_rate);
            b.usdt_amount(usdt_amount);
            b.usdt_defi_amount(usdt_defi_amount);
            b.defi_self_percentage(defi_self_percentage);
        }

        builders
            .values_mut()
            .map(|b| {
                b
                    .build()
                    .unwrap_or_else(|e| {
                        error!("{}", e);
                        TokenInfo::new_broken(b.symbol.to_owned())
                    })
            })
            .collect::<Vec<_>>()
    }
}
