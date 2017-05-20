use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::hash_map::Keys;
use std::time::Instant;
use config;
use mem;

/// Cache entry
pub struct Entry {
    value: String,
    created: Instant,
    modified: Instant,
}

/// Cache datastore
pub type Data = HashMap<String, Entry>;

#[allow(dead_code)]
pub struct Cache {
    config: config::Config,
    data: Box<Data>,
    keys: Box<VecDeque<String>>,
    bytes: usize
}

impl Cache {
    /// Create new Cache from a cache configuration
    pub fn new(config: config::Config) -> Cache {
        Self {
            config: config,
            data: Box::new(HashMap::new()),
            keys: Box::new(VecDeque::new()),
            bytes: 0,
        }
    }

    /// Empty cache
    pub fn empty(&mut self) {
        self.data.clear();
        self.bytes = 0;
    }

    /// Purge old entries
    pub fn prune(&mut self) {
        let now = Instant::now();
        let mut keys_to_delete: Vec<String> = Vec::new();
        let mut i = 0;

        for key in self.keys.iter() {
            if let Some(entry) = self.data.get(key) {
                let modified_elapsed_secs = now.duration_since(entry.modified).as_secs();

                if modified_elapsed_secs > self.config.cache.ttl {
                    keys_to_delete.push(key.clone());
                }
                else {
                    let created_elapsed_secs = now.duration_since(entry.created).as_secs();
                    if created_elapsed_secs <= self.config.cache.ttl {
                        break;
                    }
                }

                i += 1;
            }
        }

        for key in keys_to_delete.iter() {
            self.delete(key);
        }

        self.keys = Box::new(self.keys.split_off(i));
    }

    /// Get an entry from cache by key
    pub fn get(&self, key: &String) -> Option<&String> {
        let entry = self.data.get(key);

        if let Some(entry) = entry {
            // Ensure entry isn't stale
            let now = Instant::now();
            let modified_elapsed_secs = now.duration_since(entry.modified).as_secs();

            if modified_elapsed_secs > self.config.cache.ttl {
                return None
            }
        }

        entry.and_then(|e| Some(&e.value))
    }

    /// Insert an entry into cache
    pub fn set(&mut self, key: String, value: String) {
        let now = Instant::now();
        let entry: Entry;

        // Update existing entry at this key
        if let Some(previous_entry) = self.data.get(&key) {
            self.bytes -= mem::get_size_of_string(&previous_entry.value);

            entry = Entry {
                value: value,
                modified: now,
                .. *previous_entry
            };
        }
        // Create a new entry at this key
        else {
            self.bytes += mem::get_size_of_string(&key);

            entry = Entry {
                value: value,
                created: now,
                modified: now,
            };
        }

        self.bytes += mem::get_size_of_string(&entry.value);

        self.data.insert(key.clone(), entry);
        self.keys.push_back(key);
    }

    /// Delete a cache entry
    pub fn delete(&mut self, key: &String) {
        if let Some(entry) = self.data.remove(key) {
            self.bytes -= mem::get_size_of_string(&entry.value) + mem::get_size_of_string(key);
        }
    }

    /// Retrieve all cache entries' keys
    pub fn keys(&self) -> Keys<String, Entry> {
        self.data.keys()
    }

    /// Get number of cache entries
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Get approximate size of cache
    pub fn size(&self) -> usize {
        self.bytes
    }
}
