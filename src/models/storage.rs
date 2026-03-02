use std::collections::{HashMap, HashSet};
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MultiStorage {
    pub workspaces: HashMap<String, Storage>,
}

impl MultiStorage {
    pub fn get(&self, auth_token: &str) -> Option<&Storage> {
        self.workspaces.get(auth_token)
    }

    pub fn get_mut(&mut self, auth_token: &str) -> Option<&mut Storage> {
        self.workspaces.get_mut(auth_token)
    }

    pub fn get_or_create(&mut self, auth_token: &str) -> &mut Storage {
        self.workspaces.entry(auth_token.to_string()).or_default()
    }

    pub fn all_token_symbols(&self) -> HashSet<String> {
        self.workspaces.values()
            .flat_map(|ws| ws.tokens.values().map(|t| t.symbol.clone()))
            .collect()
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

    pub fn save(&self, storage: &MultiStorage) -> Result<(), SaveError> {
        save(&self.filename, storage)
    }

    pub fn load(&self) -> Result<Option<MultiStorage>, LoadError> {
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

const MULTI_STORAGE_VERSION: u8 = 0x02;

fn save(filename: &str, storage: &MultiStorage) -> Result<(), SaveError> {
    let encoded: Vec<u8> = bincode::serialize(storage).map_err(|_| SaveError::Serialize)?;
    let mut file = File::create(filename).map_err(|_| SaveError::CreateFile)?;
    file.write_all(&[MULTI_STORAGE_VERSION]).map_err(|_| SaveError::Write)?;
    file.write_all(&encoded).map_err(|_| SaveError::Write)?;
    Ok(())
}

fn load(filename: &str) -> Result<Option<MultiStorage>, LoadError> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(LoadError::OpenFile),
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|_| LoadError::Read)?;

    if buffer.is_empty() {
        return Ok(None);
    }

    // Version byte 0x02 = MultiStorage format
    if buffer[0] == MULTI_STORAGE_VERSION {
        let decoded: MultiStorage = bincode::deserialize(&buffer[1..]).map_err(|_| LoadError::Deserialize)?;
        return Ok(Some(decoded));
    }

    // Legacy migration: entire buffer is old Storage (or unversioned MultiStorage)
    if let Ok(decoded) = bincode::deserialize::<MultiStorage>(&buffer) {
        return Ok(Some(decoded));
    }

    let legacy: Storage = bincode::deserialize(&buffer).map_err(|_| LoadError::Deserialize)?;
    let mut multi = MultiStorage::default();
    multi.workspaces.insert("0".to_string(), legacy);
    Ok(Some(multi))
}
