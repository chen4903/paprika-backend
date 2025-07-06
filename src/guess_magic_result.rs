use ethabi::{decode, ParamType};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuessResult {
    param_type: String,
    result: String,
}

/// Guess the result of the decoded data from the hex string
/// Returns a vector containing all successfully decoded results
pub fn magic_guess_result(hex_string: &str) -> Vec<GuessResult> {
    let hex_data = match hex::decode(hex_string) {
        Ok(data) => data,
        Err(_) => return Vec::new(),
    };

    // we could add more possible param types here
    let param_types = vec![
        ParamType::Address,
        ParamType::Bytes,
        ParamType::Int(256),
        ParamType::Uint(256),
        ParamType::Bool,
        ParamType::String,
        ParamType::Tuple(vec![ParamType::Uint(256), ParamType::Uint(256)]),
        ParamType::Tuple(vec![
            ParamType::Uint(256),
            ParamType::Uint(256),
            ParamType::Uint(256),
        ]),
        ParamType::Tuple(vec![ParamType::Address, ParamType::Address]),
        ParamType::Tuple(vec![
            ParamType::Uint(160),
            ParamType::Int(24),
            ParamType::Uint(16),
            ParamType::Uint(16),
            ParamType::Uint(16),
            ParamType::Uint(8),
            ParamType::Bool,
        ]),
        ParamType::Tuple(vec![
            ParamType::Int(56),
            ParamType::Uint(160),
            ParamType::Uint(32),
        ]),
        ParamType::FixedBytes(32),
        ParamType::FixedArray(Box::new(ParamType::Uint(256)), 32),
        ParamType::Array(Box::new(ParamType::Uint(256))),
        ParamType::Array(Box::new(ParamType::Bool)),
        ParamType::Array(Box::new(ParamType::String)),
        ParamType::Array(Box::new(ParamType::FixedBytes(32))),
        ParamType::Array(Box::new(ParamType::Address)),
        ParamType::Array(Box::new(ParamType::Bytes)),
        ParamType::Array(Box::new(ParamType::Int(256))),
        ParamType::Array(Box::new(ParamType::Uint(256))),
        ParamType::Array(Box::new(ParamType::Bool)),
        ParamType::Array(Box::new(ParamType::String)),
        ParamType::Array(Box::new(ParamType::FixedBytes(32))),
    ];

    let mut results = Vec::new();

    for param_type in param_types {
        if let Ok(decoded) = decode(&[param_type.clone()], &hex_data) {
            if let Some(value) = decoded.get(0) {
                results.push(GuessResult {
                    param_type: param_type.to_string(),
                    result: value.to_string(),
                });
            }
        }
    }

    results
}
