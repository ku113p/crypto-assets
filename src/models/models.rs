use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    id: u8,
    pub symbol: String,
    pub exchange_rate: f32,
}

impl Token {
    pub fn new(id: u8, symbol: String, exchange_rate: f32) -> Self {
        Self { id, symbol, exchange_rate }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scheme {
    pub id: u16,
    pub name: String,
}

impl Scheme {
    pub fn new(id: u16, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    pub token_id: u8,
    pub amount: f32,
}

impl Balance {
    pub fn new(token_id: u8, amount: f32) -> Self {
        Self { token_id, amount }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Allocation {
    pub token_id: u8,
    pub scheme_id: u16,
    pub amount: f32,
}

impl Allocation {
    pub fn new(token_id: u8, scheme_id: u16, amount: f32) -> Self {
        Self { token_id, scheme_id, amount }
    }
}
