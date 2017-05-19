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
