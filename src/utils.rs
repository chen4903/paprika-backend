use alloy::primitives::Address;
use alloy_provider::Provider;
use eyre::Result;
use std::env;
use std::str::FromStr;

use crate::constants::{
    CHAIN_ID_ARBITRUM, CHAIN_ID_AVALANCHE, CHAIN_ID_BASE, CHAIN_ID_BERACHAIN, CHAIN_ID_BSC,
    CHAIN_ID_ETHEREUM, CHAIN_ID_OPTIMISM, CHAIN_ID_POLYGON, CHAIN_ID_SEPOLIA,
    DEFAULT_ARBITRUM_RPC_URL, DEFAULT_AVALANCHE_RPC_URL, DEFAULT_BASE_RPC_URL,
    DEFAULT_BERACHAIN_RPC_URL, DEFAULT_BSC_RPC_URL, DEFAULT_ETHEREUM_RPC_URL,
    DEFAULT_OPTIMISM_RPC_URL, DEFAULT_POLYGON_RPC_URL, DEFAULT_SEPOLIA_RPC_URL,
};

/// Get the rpc url of the chain
/// We use public RPC as the default value
pub fn get_rpc_url(chain_id: u64) -> Result<String> {
    let rpc_url = match chain_id {
        0x1 => env::var("MAINNET_RPC_URL").unwrap_or_else(|_| DEFAULT_ETHEREUM_RPC_URL.to_string()),
        0x0a => {
            env::var("OPTIMISM_RPC_URL").unwrap_or_else(|_| DEFAULT_OPTIMISM_RPC_URL.to_string())
        }
        0x38 => env::var("BSC_RPC_URL").unwrap_or_else(|_| DEFAULT_BSC_RPC_URL.to_string()),
        0x89 => env::var("POLYGON_RPC_URL").unwrap_or_else(|_| DEFAULT_POLYGON_RPC_URL.to_string()),
        0xa4b1 => {
            env::var("ARBITRUM_RPC_URL").unwrap_or_else(|_| DEFAULT_ARBITRUM_RPC_URL.to_string())
        }
        0xa86a => {
            env::var("AVALANCHE_RPC_URL").unwrap_or_else(|_| DEFAULT_AVALANCHE_RPC_URL.to_string())
        }
        0x2105 => env::var("BASE_RPC_URL").unwrap_or_else(|_| DEFAULT_BASE_RPC_URL.to_string()),
        0xaa36a7 => {
            env::var("SEPOLIA_RPC_URL").unwrap_or_else(|_| DEFAULT_SEPOLIA_RPC_URL.to_string())
        }
        0x138de => {
            env::var("BERACHAIN_RPC_URL").unwrap_or_else(|_| DEFAULT_BERACHAIN_RPC_URL.to_string())
        }
        _ => return Err(eyre::eyre!("Unsupported chain id: {}", chain_id)),
    };
    Ok(rpc_url)
}

/// Get the etherscan api key of the chain
pub fn get_etherscan_api_key(chain_id: u64) -> Result<String> {
    let api_key = match chain_id {
        CHAIN_ID_ETHEREUM => env::var("ETHERSCAN_API_KEY")?,
        CHAIN_ID_OPTIMISM => env::var("OPTIMISM_ETHERSCAN_API_KEY")?,
        CHAIN_ID_BSC => env::var("BSC_ETHERSCAN_API_KEY")?,
        CHAIN_ID_POLYGON => env::var("POLYGON_ETHERSCAN_API_KEY")?,
        CHAIN_ID_ARBITRUM => env::var("ARBITRUM_ETHERSCAN_API_KEY")?,
        CHAIN_ID_AVALANCHE => env::var("AVALANCHE_ETHERSCAN_API_KEY")?,
        CHAIN_ID_BASE => env::var("BASE_ETHERSCAN_API_KEY")?,
        CHAIN_ID_SEPOLIA => env::var("ETHERSCAN_API_KEY")?,
        CHAIN_ID_BERACHAIN => env::var("BERACHAIN_ETHERSCAN_API_KEY")?,
        _ => return Err(eyre::eyre!("Unsupported chain id: {}", chain_id)),
    };
    Ok(api_key)
}

/// Get the runtime code of the contract
pub async fn get_runtime_code(provider: &dyn Provider, address: &str) -> Result<String> {
    let address = Address::from_str(address).unwrap();
    let runtime_code = provider.get_code_at(address).await?;
    let runtime_code = hex::encode(runtime_code);

    Ok(runtime_code)
}

/// Check if all required configuration parameters are set
pub fn check_required_configs() -> Result<()> {
    let chain_ids = [
        (CHAIN_ID_ETHEREUM, "ETHERSCAN_API_KEY"),
        (CHAIN_ID_OPTIMISM, "OPTIMISM_ETHERSCAN_API_KEY"),
        (CHAIN_ID_BSC, "BSC_ETHERSCAN_API_KEY"),
        (CHAIN_ID_POLYGON, "POLYGON_ETHERSCAN_API_KEY"),
        (CHAIN_ID_ARBITRUM, "ARBITRUM_ETHERSCAN_API_KEY"),
        (CHAIN_ID_AVALANCHE, "AVALANCHE_ETHERSCAN_API_KEY"),
        (CHAIN_ID_BASE, "BASE_ETHERSCAN_API_KEY"),
        (CHAIN_ID_SEPOLIA, "ETHERSCAN_API_KEY"),
        (CHAIN_ID_BERACHAIN, "BERACHAIN_ETHERSCAN_API_KEY"),
    ];

    let mut missing_keys = Vec::new();
    for (chain_id, key_name) in chain_ids {
        if env::var(key_name).is_err() {
            missing_keys.push((chain_id, key_name));
        }
    }

    if !missing_keys.is_empty() {
        let missing_keys_str = missing_keys
            .iter()
            .map(|(chain_id, key_name)| format!("0x{:x} ({})", chain_id, key_name))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(eyre::eyre!("Missing required configuration parameters. Please check `.env.example` file for the following parameters: {}", missing_keys_str));
    }

    Ok(())
}
