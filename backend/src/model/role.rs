use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Party {
    Good,
    Bad,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Role {
    name: String,
    party: Party,
}

impl Role {
    pub fn new(name: &str, party: Party) -> Self {
        Role {
            name: String::from(name),
            party,
        }
    }

    pub fn party(&self) -> &Party {
        &self.party
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
