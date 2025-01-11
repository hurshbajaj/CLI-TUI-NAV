// config.rs
use std::collections::HashMap;

pub fn get_config() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("spawn", "/mnt/c/Users/Hursh Bajaj/Desktop"),
    ])
}
