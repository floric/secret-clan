mod async_repo;
mod repo_client;

use sled::IVec;
use std::{clone::Clone, convert::TryFrom, fmt::Debug};

pub use self::async_repo::{AsyncRepository, Command};
pub use self::repo_client::RepoClient;

pub trait Persist: Into<IVec> + TryFrom<IVec> + Clone + Debug {
    fn id(&self) -> &str;
}
