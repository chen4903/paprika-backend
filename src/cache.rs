use crate::database::Database;
use crate::selector::get_signature_by_selector;
use crate::utils;
use alloy_provider::ProviderBuilder;
use lazy_static::lazy_static;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::sync::Mutex;
use tracing::info;

pub const MAX_CACHE_SIZE: usize = 100;

#[derive(Clone)]
pub struct CacheItem {
    pub code: String,
    pub usage_count: u64,
}

#[derive(Clone)]
pub struct SignatureCacheItem {
    pub signature: String,
    pub usage_count: u64,
}

lazy_static! {
    pub static ref DB: Database = Database::new(false).expect("Failed to initialize database");
    pub static ref RUNTIME_CODE_CACHE: Mutex<HashMap<(u64, String), CacheItem>> =
        Mutex::new(HashMap::new());
    pub static ref SELECTOR_SIGNATURE_CACHE: Mutex<HashMap<String, SignatureCacheItem>> =
        Mutex::new(HashMap::new());
}

/// Get cached runtime code data, if not hit, then query from blockchain and save to database
pub async fn get_cached_runtime_code(chain_id: u64, address: &str) -> eyre::Result<String> {
    let cache_key = (chain_id, address.to_string());

    // Check cache first.
    {
        let mut cache = RUNTIME_CODE_CACHE.lock().unwrap();
        if let Some(cached_item) = cache.get_mut(&cache_key) {
            cached_item.usage_count += 1;
            info!("[runtime code] cache hit: {}", address);
            return Ok(cached_item.code.clone());
        }
    }

    // Check database
    if let Some(code) = DB.get_runtime_code(chain_id, address)? {
        let mut cache = RUNTIME_CODE_CACHE.lock().unwrap();
        cache.insert(
            cache_key.clone(),
            CacheItem {
                code: code.clone(),
                usage_count: 1,
            },
        );
        info!("[runtime code] database hit: {}", address);
        return Ok(code);
    }

    let rpc_url = utils::get_rpc_url(chain_id)?;
    // Initialize or get provider.
    let provider = ProviderBuilder::new().on_http(rpc_url.parse().unwrap());
    info!("✅ initialize provider successfully");

    // Get runtime code.
    let runtime_code = utils::get_runtime_code(&provider, address).await?;

    // Update both cache and database
    {
        let mut cache = RUNTIME_CODE_CACHE.lock().unwrap();
        if cache.len() >= MAX_CACHE_SIZE {
            let mut heap = BinaryHeap::new();
            for (key, item) in cache.iter() {
                heap.push(Reverse((item.usage_count, key.clone())));
            }

            if let Some(Reverse((_, key_to_remove))) = heap.pop() {
                cache.remove(&key_to_remove);
            }
        }

        cache.insert(
            cache_key,
            CacheItem {
                code: runtime_code.clone(),
                usage_count: 1,
            },
        );
    }

    // Save to database
    DB.save_runtime_code(chain_id, address, &runtime_code)?;

    Ok(runtime_code)
}

/// Get cached signature data, if not hit, then query from 4-byte website and save to database
pub async fn get_cached_signature(selector: &str) -> eyre::Result<String> {
    {
        let mut cache = SELECTOR_SIGNATURE_CACHE.lock().unwrap();
        if let Some(cached_item) = cache.get_mut(selector) {
            cached_item.usage_count += 1;
            info!("[signature] cache hit: {}", selector);
            return Ok(cached_item.signature.clone());
        }
    }

    // Check database
    if let Some(signature) = DB.get_signature(selector)? {
        let mut cache = SELECTOR_SIGNATURE_CACHE.lock().unwrap();
        cache.insert(
            selector.to_string(),
            SignatureCacheItem {
                signature: signature.clone(),
                usage_count: 1,
            },
        );
        info!("[signature] database hit: {}", selector);
        return Ok(signature);
    }

    let signature = get_signature_by_selector(selector).await.unwrap();

    {
        let mut cache = SELECTOR_SIGNATURE_CACHE.lock().unwrap();

        if cache.len() >= MAX_CACHE_SIZE {
            let mut heap = BinaryHeap::new();
            for (key, item) in cache.iter() {
                heap.push(Reverse((item.usage_count, key.clone())));
            }

            if let Some(Reverse((_, key_to_remove))) = heap.pop() {
                cache.remove(&key_to_remove);
            }
        }

        cache.insert(
            selector.to_string(),
            SignatureCacheItem {
                signature: signature.clone(),
                usage_count: 1,
            },
        );
    }

    // Save to database
    DB.save_signature(selector, &signature)?;

    Ok(signature)
}
