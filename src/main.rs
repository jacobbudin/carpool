extern crate liner;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod cache;
mod config;
mod mem;
mod server;

use liner::Context;
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

    loop {
        let res = con.read_line("> ", &mut |_| {});

        match res {
            // Process request
            Ok(res) => {
                server::handle(&mut cache, res.as_str());
                con.history.push(res.into()).unwrap();
            }
            // Or handle interrupt
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
    fn test_get_size_of_string() {
        assert_eq!(mem::get_size_of_string(&String::from("hello")), 5usize);
        assert_eq!(mem::get_size_of_string(&String::from("❤️")), 6usize);
    }
}
