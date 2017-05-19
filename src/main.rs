extern crate liner;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod cache;
mod config;
mod mem;

use liner::Context;
use liner::KeyBindings;
use std::io::ErrorKind;
use std::path::Path;

#[cfg(not(test))]
fn main() {
    // Load configuration
    let config_path = Path::new("etc/carpool.toml");
    let config = config::load_config(&config_path);

    // Set up cache
    let mut cache = cache::Cache::new(config);

    // Start REPL
    let mut con = Context::new();
    let empty_value = String::from("");

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
                        let _ = cache.delete(&key_trimmed);
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
                    "reset" =>  {
                        cache.empty();
                    }
                    "count" =>  {
                        println!("{}", cache.count());
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_size_of_simple_string() {
        assert_eq!(mem::get_size_of_string(&String::from("hello")), 5usize);
    }

    #[test]
    fn get_size_of_emoji_string() {
        assert_eq!(mem::get_size_of_string(&String::from("❤️")), 6usize);
    }
}
