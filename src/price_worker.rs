use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use log::{info, error};
use tokio::sync::Mutex;
use tokio::time::interval;
use crate::models::storage::MultiStorage;

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

pub fn spawn_price_worker(storage: Arc<Mutex<MultiStorage>>) {
    tokio::spawn(async move {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        let mut interval = interval(Duration::from_secs(3600));

        loop {
            interval.tick().await;

            let symbols = {
                let storage = storage.lock().await;
                storage.all_token_symbols()
            };

            if symbols.is_empty() {
                info!("Price worker: no tokens to update");
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
                info!("Price worker: no mappable tokens found");
                continue;
            }

            let ids_param = cg_ids.join(",");
            let url = format!(
                "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
                ids_param
            );

            info!("Price worker: fetching prices for {} tokens", cg_ids.len());

            match client.get(&url).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        error!("Price worker: CoinGecko returned status {}", resp.status());
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
                        }
                        Err(err) => error!("Price worker: failed to parse response: {err}"),
                    }
                }
                Err(err) => error!("Price worker: request failed: {err}"),
            }
        }
    });
}
