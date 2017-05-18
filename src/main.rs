extern crate liner;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use liner::Context;
use liner::KeyBindings;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::mem;
use std::path::Path;

type Data = HashMap<String, String>;

#[derive(Deserialize)]
struct CacheConfig {
    ttl: usize,
}

#[derive(Deserialize)]
struct Config {
    cache: CacheConfig,
}

struct Cache {
    data: Box<Data>,
    config: Config,
    bytes: usize
}

impl Cache {
    fn clear(&mut self) {
        self.data.clear();
        self.bytes = 0;
    }

    fn get(&mut self, key: &String) -> Option<&String> {
        self.data.get(key)
    }

    fn set(&mut self, key: String, value: String) {
        self.bytes += mem::size_of_val(&key) + mem::size_of_val(&value);
        self.data.insert(key, value);
    }

    fn delete(&mut self, key: &String) {
        let value = self.data.remove(key);
        self.bytes -= mem::size_of_val(key) + mem::size_of_val(&value);
    }

    fn keys(&self) -> Keys<String, String> {
        self.data.keys()
    }

    fn size(&self) -> usize {
        self.bytes
    }
}

fn main() {
    // Open and read configuration
    let config_path = Path::new("etc/carpool.toml");
    let mut config_file = match File::open(&config_path) {
        Err(_) => panic!("couldn't open {:?}", config_path),
        Ok(file) => file,
    };

    let mut config_content = String::new();
    let _ = config_file.read_to_string(&mut config_content);

    let config: Config = toml::from_str(config_content.as_str()).unwrap();

    // Set up cache
    let mut cache = Cache {
        config: config,
        data: Box::new(HashMap::new()),
        bytes: 0,
    };

    let empty_value = String::from("");

    // Start REPL
    let mut con = Context::new();

    loop {
        let res = con.read_line("> ", &mut |_| {});

        match res {
            Ok(res) => {
                match res.as_str() {
                    s if s.trim() == "" =>  {}
                    s if s.starts_with("get ") =>  {
                        let (_, key) = s.split_at(4);
                        let key_trimmed = String::from(key.trim());
                        if key_trimmed.find(' ').is_some() {
                            println!("key cannot contain space");
                            continue
                        }
                        println!("{}", cache.get(&key_trimmed).unwrap_or(&empty_value));
                    }
                    s if s.starts_with("del ") =>  {
                        let (_, key) = s.split_at(4);
                        let key_trimmed = String::from(key.trim());
                        if key_trimmed.find(' ').is_some() {
                            println!("key cannot contain space");
                            continue
                        }
                        let value = cache.delete(&key_trimmed);
                    }
                    s if s.starts_with("set ") =>  {
                        let (_, key_value) = s.split_at(4);
                        match key_value.find(' ') {
                            Some(i) => {
                                let (key, value) = key_value.split_at(i);
                                let key_trimmed = String::from(key.trim());
                                let value_trimmed = String::from(value.trim());
                                cache.set(key_trimmed, value_trimmed);
                            }
                            None => println!("no value specified")
                        }
                    }
                    "size" =>  {
                        println!("{} bytes", cache.size());
                    }
                    "keys" =>  {
                        for key in cache.keys() {
                            println!("{}", key);
                        }
                    }
                    "emacs" => {
                        con.key_bindings = KeyBindings::Emacs;
                        println!("emacs mode");
                    }
                    "vi" | "vim" =>  {
                        con.key_bindings = KeyBindings::Vi;
                        println!("vi mode");
                    }
                    "exit" =>  {
                        break;
                    }
                    _ => {
                        println!("operation not defined")
                    }
                }

                con.history.push(res.into()).unwrap();
            }
            Err(e) => {
                match e.kind() {
                    // ctrl-c pressed
                    ErrorKind::Interrupted => {}
                    // ctrl-d pressed
                    ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => panic!("error: {:?}", e),
                }
            }
        }
    }
}
