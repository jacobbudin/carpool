extern crate liner;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use liner::Context;
use liner::KeyBindings;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::mem;
use std::path::Path;

#[derive(Deserialize)]
struct CacheConfig {
    ttl: usize,
}

#[derive(Deserialize)]
struct Config {
    cache: CacheConfig,
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
    let mut cache = Box::new(HashMap::new());
    let mut bytes_used:usize = 0;
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
                        let value = cache.remove(&key_trimmed);
                        bytes_used -= mem::size_of_val(&key_trimmed) + mem::size_of_val(&value);
                    }
                    s if s.starts_with("set ") =>  {
                        let (_, key_value) = s.split_at(4);
                        match key_value.find(' ') {
                            Some(i) => {
                                let (key, value) = key_value.split_at(i);
                                let key_trimmed = String::from(key.trim());
                                let value_trimmed = String::from(value.trim());
                                bytes_used += mem::size_of_val(&key_trimmed) + mem::size_of_val(&value_trimmed);
                                cache.insert(key_trimmed, value_trimmed);
                            }
                            None => println!("no value specified")
                        }
                    }
                    "size" =>  {
                        println!("{} bytes", bytes_used);
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
