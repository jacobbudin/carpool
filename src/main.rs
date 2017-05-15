extern crate liner;

use liner::Context;
use liner::KeyBindings;
use std::collections::HashMap;
use std::io::ErrorKind;

fn main() {
    let mut cache: HashMap<String, String> = HashMap::new();

    // TODO: Implement actual op-parsing
    let dummy_key = String::from("some_key");
    let dummy_value = String::from("123");
    let empty_value = String::from("0");

    let mut con = Context::new();

    loop {
        let res = con.read_line("> ", &mut |_| {});

        match res {
            Ok(res) => {
                match res.as_str() {
                    "emacs" => {
                        con.key_bindings = KeyBindings::Emacs;
                        println!("emacs mode");
                    }
                    "vi" | "vim" =>  {
                        con.key_bindings = KeyBindings::Vi;
                        println!("vi mode");
                    }
                    s if s.starts_with("get") =>  {
                        println!("{}", cache.get(&dummy_key).unwrap_or(&empty_value));
                    }
                    s if s.starts_with("set") =>  {
                        cache.insert(dummy_key.clone(), dummy_value.clone());
                    }
                    "exit" =>  {
                        break;
                    }
                    _ => {}
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
