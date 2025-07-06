use serde_json::json;
use tracing::debug;

/// Use evmole to guess the ABI, and ultimately return an ABI in the form of paprika_guess_12345678(address)
pub fn abi_ui_wrapper(bytecode: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // remove the prefix '0x' (if present)
    let bytecode = if bytecode.starts_with("0x") || bytecode.starts_with("0X") {
        &bytecode[2..]
    } else {
        bytecode
    };

    let code = hex::decode(bytecode)?;

    debug!("Succeed to get ABI from bytecode, len: {}", bytecode.len());

    let evmole_result = evmole::contract_info(
        evmole::ContractInfoArgs::new(&code)
            .with_selectors()
            .with_arguments()
            .with_state_mutability(),
    );

    let mut result = vec![];
    for func_item in evmole_result.functions.unwrap() {
        let selector = &func_item
            .selector
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("");

        let mut arguments = "".to_string();
        for arg_item in func_item.arguments.unwrap() {
            arguments.push_str(&arg_item.to_string());
            arguments.push_str(",");
        }
        // remove the last extra comma
        if arguments.ends_with(",") {
            arguments.pop();
        }

        let arguments = format!("({})", arguments);
        let abi_item = format!("paprika_guess_{}{}", selector, arguments);
        result.push(abi_item);
    }

    Ok(result)
}

/// Use evmole to guess the ABI, and ultimately return an ABI in the form of JSON
pub fn abi_json_wrapper(bytecode: &str) -> Result<String, Box<dyn std::error::Error>> {
    // remove the prefix '0x' (if present)
    let bytecode = if bytecode.starts_with("0x") || bytecode.starts_with("0X") {
        &bytecode[2..]
    } else {
        bytecode
    };

    let code = hex::decode(bytecode)?;

    debug!("Succeed to get ABI from bytecode, len: {}", bytecode.len());

    let evmole_result = evmole::contract_info(
        evmole::ContractInfoArgs::new(&code)
            .with_selectors()
            .with_arguments()
            .with_state_mutability(),
    );

    // Convert functions to JSON format
    if let Some(functions) = evmole_result.functions {
        let json_functions: Vec<serde_json::Value> = functions.iter().map(|f| {
            let mut inputs = Vec::new();
            if let Some(args) = &f.arguments {
                for (i, arg) in args.iter().enumerate() {
                    inputs.push(json!({
                        "name": format!("arg{}", i),
                        "type": arg.to_string()
                    }));
                }
            }

            json!({
                "name": format!("paprika_guessed_{:02x}{:02x}{:02x}{:02x}", f.selector[0], f.selector[1], f.selector[2], f.selector[3]),
                "type": "function",
                "inputs": inputs,
                "outputs": [
                    {
                        "name": "",
                        "type": "bytes"
                    }
                ],
                "stateMutability": "payable",
                "payable": true,
            })
        }).collect();

        return Ok(serde_json::to_string(&json_functions)?);
    }

    Ok("".to_string())
}

/// Use evmole to get the selectors of the contract
pub fn get_selectors(bytecode: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let bytecode = if bytecode.starts_with("0x") || bytecode.starts_with("0X") {
        &bytecode[2..]
    } else {
        bytecode
    };

    let bytecode = hex::decode(bytecode)?;

    let result = evmole::contract_info(
        evmole::ContractInfoArgs::new(&bytecode)
            .with_selectors()
            .with_arguments()
            .with_state_mutability(),
    );

    let mut selectors = vec![];
    for selector in result.functions.unwrap() {
        selectors.push(
            selector
                .selector
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<Vec<String>>()
                .join(""),
        );
    }

    Ok(selectors)
}
