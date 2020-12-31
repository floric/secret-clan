use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Party {
    Good,
    Bad,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    name: String,
    party: Party,
    description: String,
}

impl Role {
    pub fn new(name: &str, party: Party, description: &str) -> Self {
        Role {
            name: String::from(name),
            description: String::from(description),
            party,
        }
    }

    pub fn party(&self) -> &Party {
        &self.party
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
