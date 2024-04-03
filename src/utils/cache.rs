use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use serde_json::Value;

pub const CACHE_TIME: u64 = 300; // 5mn

pub type Cache = Arc<Mutex<HashMap<String, (SystemTime, Value)>>>;

// TODO:
// Wait for https://github.com/hyperium/http/pull/574
// Then cache Response-s instead of Values
// In which case we can cache same fetches for different routes
pub async fn cached_fetch<Fut>(
    cache: Cache,
    key: String,
    fetch_and_parse: impl FnOnce() -> Fut,
) -> Value
where
    Fut: Future<Output = color_eyre::Result<Value>>,
{
    if let Some((system_time, value)) = cache.lock().unwrap().get(&key) {
        if system_time.elapsed().unwrap().as_secs() < CACHE_TIME {
            println!("cache: {}", key);

            return value.clone();
        }
    }

    println!("fetch: {}", key);

    let value = fetch_and_parse().await;

    if value.is_err() {
        if let Some((_, value)) = cache.lock().unwrap().get(&key) {
            println!("fetch failed, getting cache: {}", key);
            return value.clone();
        }
    }

    let value = value.unwrap();

    cache
        .lock()
        .unwrap()
        .insert(key, (SystemTime::now(), value.clone()));

    value
}
