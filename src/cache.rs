use std::collections::HashMap;
use std::collections::hash_map::Keys;
use config;
use mem;

type Data = HashMap<String, String>;

#[allow(dead_code)]
pub struct Cache {
    data: Box<Data>,
    config: config::Config,
    bytes: usize
}

impl Cache {
    pub fn new(config: config::Config) -> Cache {
        Self {
            config: config,
            data: Box::new(HashMap::new()),
            bytes: 0,
        }
    }

    pub fn empty(&mut self) {
        self.data.clear();
        self.bytes = 0;
    }

    pub fn get(&mut self, key: &String) -> Option<&String> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        if let Some(previous_value) = self.data.get(&key) {
            self.bytes -= mem::get_size_of_string(previous_value);
        }
        else {
            self.bytes += mem::get_size_of_string(&key);
        }
        self.bytes += mem::get_size_of_string(&value);

        self.data.insert(key, value);
    }

    pub fn delete(&mut self, key: &String) {
        if let Some(value) = self.data.remove(key) {
            self.bytes -= mem::get_size_of_string(&value) + mem::get_size_of_string(key);
        }
    }

    pub fn keys(&self) -> Keys<String, String> {
        self.data.keys()
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn size(&self) -> usize {
        self.bytes
    }
}
