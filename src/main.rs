#[macro_use]
extern crate serde_derive;
extern crate toml;

mod cache;
mod config;
mod mem;
mod server;

use std::io::{BufReader, BufRead, Write};
use std::net::{TcpListener};
use std::path::Path;

#[cfg(not(test))]
fn main() {
    // Load configuration
    let config_path = Path::new("etc/carpool.toml");
    let config = config::load_config(&config_path);

    // Set up cache
    let mut cache = cache::Cache::new(config);

    // Start listening
    let listener = TcpListener::bind("127.0.0.1:8555").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Read all to newline
                let mut buffer = Vec::new();
                let mut reader = BufReader::new(stream);
                let _read_result = reader.read_until(b'\n', &mut buffer);
                let mut consumed_stream = reader.into_inner();

                // Get and write response
                let request = String::from_utf8(buffer).unwrap();
                let response = server::handle(&mut cache, request.as_str());
                let _write_result = consumed_stream.write_all(response.as_bytes());
            }
            Err(_) => {
                break;
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
