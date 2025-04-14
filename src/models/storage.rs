use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use serde::{Deserialize, Serialize};
use crate::models::models::{Allocation, Balance, Scheme, Token};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Storage {
    pub tokens: HashMap<u8, Token>,
    pub schemes: HashMap<u16, Scheme>,
    pub balances: Vec<Balance>,
    pub allocations: Vec<Allocation>,
}

impl Storage {
    pub fn get_or_create_token_id(&mut self, symbol: &str) -> u8 {
        match self.get_token_id(symbol) {
            Some(token_id) => token_id,
            None => {
                let new_id = (self.tokens.len() as u8) + 1;
                let token = Token::new(new_id, symbol.to_string(), 0.0);
                self.tokens.insert(new_id, token);
                new_id
            }
        }
    }

    pub fn get_token_id(&self, symbol: &str) -> Option<u8> {
        self.tokens.iter().find_map(
            |(&id, s)| if s.symbol == symbol { Some(id) } else { None }
        )
    }

    pub fn get_or_create_scheme_id(&mut self, name: &str) -> u16 {
        match self.get_scheme_id(name) {
            Some(scheme_id) => scheme_id,
            None => {
                let new_id = (self.schemes.len() as u16) + 1;
                let scheme = Scheme::new(new_id, name.to_string());
                self.schemes.insert(new_id, scheme);
                new_id
            }
        }
    }

    pub fn get_scheme_id(&self, name: &str) -> Option<u16> {
        self.schemes.iter().find_map(
            |(&id, s)| if s.name == name { Some(id) } else { None }
        )
    }

    pub fn get_token_symbol(&self, id: &u8) -> Option<String> {
        self.tokens.get(id).map(|t| t.symbol.clone())
    }

    pub fn get_scheme_name(&self, id: &u16) -> Option<String> {
        self.schemes.get(id).map(|s| s.name.clone())
    }
}

#[derive(Clone)]
pub struct StorageOperator {
    filename: String,
}

impl StorageOperator {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }

    pub fn save(&self, storage: &Storage) -> Result<(), SaveError> {
        save(&self.filename, storage)
    }

    pub fn load(&self) -> Result<Option<Storage>, LoadError> {
        load(&self.filename)
    }
}

#[derive(Debug)]
pub enum SaveError {
    Serialize,
    CreateFile,
    Write
}

#[derive(Debug)]
pub enum LoadError {
    OpenFile,
    Read,
    Deserialize,
}

fn save(filename: &str, storage: &Storage) -> Result<(), SaveError> {
    let encoded: Vec<u8> = bincode::serialize(storage).map_err(|_| SaveError::Serialize)?;
    let mut file = File::create(filename).map_err(|_| SaveError::CreateFile)?;
    file.write_all(&encoded).map_err(|_| SaveError::Write)?;
    Ok(())
}

fn load(filename: &str) -> Result<Option<Storage>, LoadError> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(LoadError::OpenFile),
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|_| LoadError::Read)?;
    let decoded: Storage = bincode::deserialize(&buffer).map_err(|_| LoadError::Deserialize)?;
    Ok(Some(decoded))
}
