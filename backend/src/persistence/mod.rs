use log::warn;
use sled::{Db, IVec};
use std::{clone::Clone, convert::TryFrom};

pub trait Persist<T: Into<IVec> + TryFrom<IVec> + Clone>:
    Into<IVec> + TryFrom<IVec> + Clone
{
    fn persistence_path(_: Option<Self>) -> String;

    fn open_tree(_: Option<Self>) -> Db {
        sled::open(format!(
            "{}/.sled/{}",
            dirs::home_dir()
                .expect("No user dir known")
                .to_str()
                .unwrap(),
            Persist::persistence_path(None::<Self>)
        ))
        .expect("opening database has failed")
    }

    fn id(&self) -> &str;

    fn persist(&self) -> Result<bool, String> {
        let tree = Persist::open_tree(None::<Self>);
        tree.insert(self.id(), self.clone())
            .expect("Persisting item failed");
        Persist::flush(None::<Self>, &tree)
            .map_err(|e| e.to_string())
            .map(|_| true)
    }

    fn find_by_id(_: Option<Self>, id: &str) -> Option<T> {
        let tree = Persist::open_tree(None::<Self>);
        let success = tree.get(id);
        match success {
            Ok(res) => res.and_then(|g| T::try_from(g).ok()),
            Err(_) => None,
        }
    }

    fn delete(&self, id: &str) -> Result<bool, String> {
        let tree = Persist::open_tree(None::<Self>);
        tree.remove(id).expect("Deleting item failed");
        Persist::flush(None::<Self>, &tree)
            .map_err(|e| e.to_string())
            .map(|_| true)
    }

    fn flush(_: Option<Self>, tree: &Db) -> Result<bool, sled::Error> {
        tree.flush().map(|_| true)
    }

    fn purge_data(_: Option<Self>) {
        let tree = Persist::open_tree(None::<Self>);
        let res = tree.drop_tree(Persist::persistence_path(None::<Self>));
        if res.is_err() {
            warn!("Cleaning database has failed");
        }
    }
}
