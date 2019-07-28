use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new()
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!();
    }

    pub fn set(&self, key: String, val: String) -> () {
        unimplemented!();
    }

    pub fn remove(&self, key: String) -> () {
        unimplemented!();
    }

}