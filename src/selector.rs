use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;
use tracing::debug;

use crate::constants::FOUR_BYTE_SIGNATURE_API_URL;

#[derive(Deserialize)]
struct ApiResponse {
    results: Vec<SignatureResult>,
}

#[derive(Deserialize)]
struct SignatureResult {
    id: usize,
    text_signature: String,
}

/// Get the signature by selector
/// Send a request to the 4-byte signature website, retry at most 3 times, with a direct interval of 1 second each time
pub async fn get_signature_by_selector(selector: &str) -> Result<String, reqwest::Error> {
    const MAX_RETRIES: u32 = 3;
    const BASE_DELAY_MS: u64 = 1000; // 1 second

    let url = format!("{}{}", FOUR_BYTE_SIGNATURE_API_URL, selector);

    let client = Client::new();

    for retry in 0..MAX_RETRIES {
        match async {
            let response = client.get(&url).send().await?;
            let api_response: ApiResponse = response.json().await?;

            if let Some(min_id_signature) = api_response.results.iter().min_by_key(|sig| sig.id) {
                let signature = min_id_signature.text_signature.clone();
                return Ok::<String, reqwest::Error>(signature);
            }
            debug!(
                "Signature not found, return the selector itself:{}",
                selector
            );
            return Ok::<String, reqwest::Error>(selector.to_string());
        }
        .await
        {
            Ok(result) => return Ok(result),
            Err(e) => {
                if retry < MAX_RETRIES - 1 {
                    debug!(
                        "The {}th retry failed, will retry in {}ms: {}",
                        retry + 1,
                        BASE_DELAY_MS,
                        e
                    );
                    sleep(Duration::from_millis(BASE_DELAY_MS)).await;
                }
            }
        }
    }

    debug!(
        "All retries failed, return the selector itself: {}",
        selector
    );

    Ok(selector.to_string())
}
