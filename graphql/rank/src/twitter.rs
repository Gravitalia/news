//! Twitter-related trends.

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, &'static str> = {
        let mut country = HashMap::new();
        country.insert("WORLD", "1");
        country.insert("BR", "23424768");
        country.insert("CA", "23424775");
        country.insert("DE", "23424829");
        country.insert("UK", "23424975");
        country.insert("US", "23424977");
        country
    };
}

const _API_URL: &str = "https://api.x.com/1.1";
const _TRENDS_PATH: &str = "/trends/place.json";
