use crate::abi;
use crate::utils;
use alloy::{
    contract::Interface,
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    primitives::{Address, U256},
};
use eyre::Result;
use std::process::Command;
use std::str::FromStr;

#[derive(serde::Deserialize, Debug)]
pub struct Params {
    content: String,
    params_type: String,
}

/// Assemble the data we need to call into raw calldata, and then send it directly to the blockchain
pub async fn get_guessed_calldata(
    runtime_code: &str,
    function_name: &str,
    params: &[Params],
) -> Result<String> {
    let abi_json = abi::abi_json_wrapper(&runtime_code)
        .map_err(|e| eyre::eyre!("ABI generation failed: {}", e))?;
    let abi = serde_json::from_str::<JsonAbi>(&abi_json)
        .map_err(|e| eyre::eyre!("ABI parsing failed: {}", e))?;

    let interface = Interface::new(abi);

    // Convert Params to DynSolValue
    let mut args = Vec::new();
    for param in params {
        let content = param.content.clone();
        let params_type = param.params_type.clone();

        // we could add more type
        if params_type == "address" {
            let arg_address = DynSolValue::from(Address::from_str(&content).unwrap());
            args.push(arg_address);
        } else if params_type == "string" {
            let arg_string = DynSolValue::from(content);
            args.push(arg_string);
        } else if params_type == "uint256" {
            let arg_uint256 = DynSolValue::from(U256::from_str(&content).unwrap());
            args.push(arg_uint256);
        } else if params_type == "bool" {
            let arg_bool = DynSolValue::from(bool::from_str(&content).unwrap());
            args.push(arg_bool);
        } else if params_type == "bytes" {
            let arg_bytes = DynSolValue::from(hex::decode(&content).unwrap());
            args.push(arg_bytes);
        } else if params_type == "int256" {
            let arg_int256 = DynSolValue::from(i128::from_str(&content).unwrap());
            args.push(arg_int256);
        } else {
            return Err(eyre::eyre!("Unsupported parameter type: {}", params_type));
        }
    }

    let calldata = interface.encode_input(function_name, &args)?;

    Ok(format!("0x{}", hex::encode(calldata)))
}

/// Simulate the call, and return the result
/// TODO: We should not rely on local CLI tools; here we are just using Foundry's toolchain for convenience,
///       and in the future, we need to replace it.
pub async fn simulate_call(chain_id: u64, from: &str, to: &str, data: &str) -> Result<String> {
    let rpc_url = utils::get_rpc_url(chain_id)?;
    let etherscan_api_key = utils::get_etherscan_api_key(chain_id)?;

    let output = Command::new("cast")
        .arg("call")
        .arg("--from")
        .arg(from)
        .arg("--data")
        .arg(data)
        .arg(to)
        .arg("--rpc-url")
        .arg(rpc_url)
        .arg("--trace")
        .arg("--etherscan-api-key")
        .arg(etherscan_api_key)
        .output()
        .map_err(|e| eyre::eyre!("Failed to execute cast command: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(eyre::eyre!("Command failed with error: {}", stderr))
    }
}
