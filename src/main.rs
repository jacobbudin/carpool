extern crate liner;

use liner::Context;
use liner::KeyBindings;
use std::collections::HashMap;
use std::io::ErrorKind;

fn main() {
    let mut cache = Box::new(HashMap::new());

    let empty_value = String::from("");

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
                    s if s.starts_with("set ") =>  {
                        let (_, key_value) = s.split_at(4);
                        match key_value.find(' ') {
                            Some(i) => {
                                let (key, value) = key_value.split_at(i);
                                let key_trimmed = String::from(key.trim());
                                let value_trimmed = String::from(value.trim());
                                cache.insert(key_trimmed, value_trimmed);
                            }
                            None => println!("no value specified")
                        }
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
