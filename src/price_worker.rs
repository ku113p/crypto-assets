use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use log::{info, error};
use tokio::sync::Mutex;
use tokio::time::interval;
use crate::app_state::{AppState, PriceStatus};

fn symbol_to_coingecko_id(symbol: &str) -> Option<&'static str> {
    match symbol.to_uppercase().as_str() {
        "BTC" => Some("bitcoin"),
        "ETH" => Some("ethereum"),
        "SOL" => Some("solana"),
        "USDT" => Some("tether"),
        "USDC" => Some("usd-coin"),
        "BNB" => Some("binancecoin"),
        "XRP" => Some("ripple"),
        "ADA" => Some("cardano"),
        "DOGE" => Some("dogecoin"),
        "AVAX" => Some("avalanche-2"),
        "DOT" => Some("polkadot"),
        "MATIC" | "POL" => Some("matic-network"),
        "LINK" => Some("chainlink"),
        "UNI" => Some("uniswap"),
        "ATOM" => Some("cosmos"),
        "LTC" => Some("litecoin"),
        "FIL" => Some("filecoin"),
        "APT" => Some("aptos"),
        "ARB" => Some("arbitrum"),
        "OP" => Some("optimism"),
        "SUI" => Some("sui"),
        "NEAR" => Some("near"),
        "AAVE" => Some("aave"),
        "MKR" => Some("maker"),
        "SNX" => Some("havven"),
        "CRV" => Some("curve-dao-token"),
        "RUNE" => Some("thorchain"),
        "INJ" => Some("injective-protocol"),
        "TIA" => Some("celestia"),
        "SEI" => Some("sei-network"),
        "STX" => Some("blockstack"),
        "RENDER" | "RNDR" => Some("render-token"),
        "FET" => Some("fetch-ai"),
        "JUP" => Some("jupiter-exchange-solana"),
        "WIF" => Some("dogwifcoin"),
        "PEPE" => Some("pepe"),
        "SHIB" => Some("shiba-inu"),
        "TON" => Some("the-open-network"),
        "TRX" => Some("tron"),
        _ => None,
    }
}

fn now_iso() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}

async fn update_status(price_status: &Mutex<PriceStatus>, result: &str, updated: u32) {
    let mut status = price_status.lock().await;
    status.last_result = Some(result.to_string());
    status.last_updated = Some(now_iso());
    status.tokens_updated = updated;
}

pub fn spawn_price_worker(state: Arc<AppState>) {
    let storage = state.storage.clone();
    let price_status = state.price_status.clone();

    tokio::spawn(async move {
        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
        {
            Ok(c) => c,
            Err(err) => {
                error!("Price worker: failed to build HTTP client: {err}");
                update_status(&price_status, &format!("client build failed: {err}"), 0).await;
                return;
            }
        };

        let mut interval = interval(Duration::from_secs(3600));

        loop {
            interval.tick().await;

            let symbols = {
                let storage = storage.lock().await;
                storage.all_token_symbols()
            };

            if symbols.is_empty() {
                info!("Price worker: no tokens to update");
                update_status(&price_status, "no tokens", 0).await;
                continue;
            }

            let mut symbol_to_cg: HashMap<String, String> = HashMap::new();
            let mut cg_ids: Vec<String> = Vec::new();

            for symbol in &symbols {
                if let Some(cg_id) = symbol_to_coingecko_id(symbol) {
                    symbol_to_cg.insert(cg_id.to_string(), symbol.clone());
                    cg_ids.push(cg_id.to_string());
                }
            }

            if cg_ids.is_empty() {
                info!("Price worker: no mappable tokens found (symbols: {:?})", symbols);
                update_status(&price_status, &format!("no mappable tokens: {:?}", symbols), 0).await;
                continue;
            }

            let ids_param = cg_ids.join(",");
            let url = format!(
                "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
                ids_param
            );

            info!("Price worker: fetching prices for {} tokens (ids: {})", cg_ids.len(), ids_param);

            match client.get(&url).send().await {
                Ok(resp) => {
                    let status_code = resp.status();
                    if !status_code.is_success() {
                        let body = resp.text().await.unwrap_or_default();
                        error!("Price worker: CoinGecko returned status {} body: {}", status_code, body);
                        update_status(&price_status, &format!("HTTP {}: {}", status_code, body), 0).await;
                        continue;
                    }

                    match resp.json::<HashMap<String, HashMap<String, f64>>>().await {
                        Ok(prices) => {
                            let mut storage = storage.lock().await;
                            let mut updated = 0;

                            for (cg_id, price_map) in &prices {
                                if let Some(usd_price) = price_map.get("usd") {
                                    if let Some(symbol) = symbol_to_cg.get(cg_id) {
                                        for ws in storage.workspaces.values_mut() {
                                            for token in ws.tokens.values_mut() {
                                                if token.symbol == *symbol {
                                                    token.exchange_rate = *usd_price as f32;
                                                    updated += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            info!("Price worker: updated {} token exchange rates", updated);
                            update_status(&price_status, "ok", updated as u32).await;
                        }
                        Err(err) => {
                            error!("Price worker: failed to parse response: {err}");
                            update_status(&price_status, &format!("parse error: {err}"), 0).await;
                        }
                    }
                }
                Err(err) => {
                    error!("Price worker: request failed: {err}");
                    update_status(&price_status, &format!("request failed: {err}"), 0).await;
                }
            }
        }
    });
}
