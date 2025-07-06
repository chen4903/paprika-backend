/// https://www.4byte.directory/docs/
pub const FOUR_BYTE_SIGNATURE_API_URL: &str =
    "https://www.4byte.directory/api/v1/signatures/?hex_signature=";

/// Database path
pub const DEFAULT_DATABASE_PATH: &str = "data/data.db"; // default path for the database

/// Default log level, can be overridden by LOG_LEVEL env var
/// Available values: "error", "warn", "info", "debug", "trace"
pub const DEFAULT_LOG_LEVEL: &str = "info";

/// RPC url
pub const DEFAULT_ETHEREUM_RPC_URL: &str = "https://eth.llamarpc.com"; // mainnet
pub const DEFAULT_OPTIMISM_RPC_URL: &str = "https://optimism.llamarpc.com"; // optimism
pub const DEFAULT_BSC_RPC_URL: &str = "https://binance.llamarpc.com"; // bsc
pub const DEFAULT_POLYGON_RPC_URL: &str = "https://rpc.ankr.com/polygon"; // polygon
pub const DEFAULT_ARBITRUM_RPC_URL: &str = "https://arbitrum.llamarpc.com"; // arbitrum
pub const DEFAULT_AVALANCHE_RPC_URL: &str = "https://avax.meowrpc.com"; // avalanche
pub const DEFAULT_BASE_RPC_URL: &str = "https://developer-access-mainnet.base.org"; // base
pub const DEFAULT_BERACHAIN_RPC_URL: &str = "https://rpc.berachain.com/"; // berachain
/// Testing network
pub const DEFAULT_SEPOLIA_RPC_URL: &str = "https://ethereum-sepolia-rpc.publicnode.com"; // sepolia

/// Chain ids
pub const CHAIN_ID_ETHEREUM: u64 = 0x1;
pub const CHAIN_ID_OPTIMISM: u64 = 0x0a;
pub const CHAIN_ID_BSC: u64 = 0x38;
pub const CHAIN_ID_POLYGON: u64 = 0x89;
pub const CHAIN_ID_ARBITRUM: u64 = 0xa4b1;
pub const CHAIN_ID_AVALANCHE: u64 = 0xa86a;
pub const CHAIN_ID_BASE: u64 = 0x2105;
pub const CHAIN_ID_BERACHAIN: u64 = 0x138de;
/// Testing network
pub const CHAIN_ID_SEPOLIA: u64 = 0xaa36a7;

/// Should auto clean-up of expired data be enabled, default is enabled.
/// Once enabled, the database will clean up expired data every EXPIRED_TIME seconds.
/// This is designed to prevent the database from becoming too large, making the program small and exquisite.
pub const CLEAN_EXPIRED_DATA: bool = true;
pub const EXPIRED_TIME: i64 = 2 * 24 * 60 * 60; // 2 days
pub const CLEAN_INTERVAL: u64 = 30 * 60; // 30 minutes

/// Should the image data of CFG be deleted, default is off. Highly recommended to turn off,
/// otherwise it will generate a large amount of unnecessary file data.
pub const CLEAN_CFG_IMAGE_DATA: bool = true;

/// Default api port
pub const DEFAULT_API_PORT: &str = "1234";
