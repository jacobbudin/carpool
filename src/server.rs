use cache::Cache;
use std::fmt;

/// Process a request against the cache
pub fn handle(cache: &mut Cache, command: &str) -> String {
    match command {
        s if s.trim() == "" =>  {}
        s if s.starts_with("get ") =>  {
            let (_, key) = s.split_at(4);
            let key_trimmed = String::from(key.trim());
            if key_trimmed.find(' ').is_some() {
                println!("key cannot contain space");
                return "".to_owned()
            }
            return fmt::format(format_args!("{}", cache.get(&key_trimmed).unwrap_or(&"".to_owned())));
        }
        s if s.starts_with("del ") =>  {
            let (_, key) = s.split_at(4);
            let key_trimmed = String::from(key.trim());
            if key_trimmed.find(' ').is_some() {
                println!("key cannot contain space");
                return "".to_owned()
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
        "prune" =>  {
            cache.prune();
        }
        "reset" =>  {
            cache.empty();
        }
        "count" =>  {
            return fmt::format(format_args!("{}", cache.count()));
        }
        "size" =>  {
            return fmt::format(format_args!("{} bytes", cache.size()));
        }
        "keys" =>  {
            for key in cache.keys() {
                println!("{}", key);
            }
        }
        _ => {
            println!("operation not defined")
        }
    }

    "".to_owned()
}
