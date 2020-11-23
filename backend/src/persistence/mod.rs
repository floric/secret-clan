use sled::{Db, IVec};
use std::clone::Clone;

pub trait Persistable<T: Into<IVec> + Clone>: Into<IVec> + Clone {
    fn persistence_path(&self) -> String;
    fn id(&self) -> &str;

    fn persist(&self) -> Result<bool, String> {
        let tree = self.open_tree();
        tree.insert(self.id(), self.clone())
            .expect("Creating item failed");
        self.flush(&tree).map_err(|e| e.to_string()).map(|_| true)
    }

    fn update(&self, id: &str) -> Result<bool, String> {
        let tree = self.open_tree();
        tree.insert(id, self.clone()).expect("Updating item failed");
        self.flush(&tree).map_err(|e| e.to_string()).map(|_| true)
    }

    fn delete(&self, id: &str) -> Result<bool, String> {
        let tree = self.open_tree();
        tree.remove(id).expect("Deleting item failed");
        self.flush(&tree).map_err(|e| e.to_string()).map(|_| true)
    }

    fn flush(&self, tree: &Db) -> Result<bool, sled::Error> {
        tree.flush().map(|_| true)
    }

    fn open_tree(&self) -> Db {
        sled::open(format!(
            "{:?}{}",
            std::env::temp_dir(),
            self.persistence_path()
        ))
        .expect("opening database has failed")
    }
}
