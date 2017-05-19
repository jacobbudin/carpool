use std::io::Read;
use std::path::Path;
use std::fs::File;
use toml;

#[allow(dead_code)]
#[derive(Deserialize)]
struct CacheConfig {
    ttl: usize,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Config {
    cache: CacheConfig,
}

/// Open and read Carpool configuration from file path
pub fn load_config(path: &Path) -> Config {
    let mut file = match File::open(path) {
        Err(_) => panic!("couldn't open {:?}", path),
        Ok(file) => file,
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let config: Config = toml::from_str(content.as_str()).unwrap();
    config
}
