mod async_repo;
mod repository;

use sled::IVec;
use std::{clone::Clone, convert::TryFrom, fmt::Debug};

pub use self::async_repo::{AsyncRepository, Command};
pub use self::repository::Repository;

pub trait Persist: Into<IVec> + TryFrom<IVec> + Clone + Debug {
    fn id(&self) -> &str;
}
