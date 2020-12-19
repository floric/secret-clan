mod client;
mod database;

use sled::IVec;
use std::{
    clone::Clone,
    collections::HashSet,
    convert::TryFrom,
    fmt::{self, Debug},
};
use tokio::sync::oneshot;

pub use self::client::Client;
pub use self::database::Database;

pub trait Persist: Into<IVec> + TryFrom<IVec> + Clone + Debug + Send {
    fn id(&self) -> &str;
}
#[derive(Debug, Clone)]
pub struct QueryError {
    message: String,
}

impl QueryError {
    pub fn new(message: &str) -> Self {
        QueryError {
            message: String::from(message),
        }
    }

    pub fn from_sled(error: sled::Error) -> Self {
        QueryError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub enum Command<T: Persist> {
    Get {
        key: String,
        responder: oneshot::Sender<Result<Option<T>, sled::Error>>,
    },
    Scan {
        scan_function: fn(&T) -> bool,
        responder: oneshot::Sender<Result<HashSet<String>, sled::Error>>,
    },
    Persist {
        value: T,
        responder: oneshot::Sender<Result<bool, sled::Error>>,
    },
    Remove {
        key: String,
        responder: oneshot::Sender<Result<bool, sled::Error>>,
    },
    Count {
        responder: oneshot::Sender<usize>,
    },
}
