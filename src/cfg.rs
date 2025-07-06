use crate::opcode::{OpCode, OpCodeInfo};
use heimdall_cfg::{cfg, CfgArgsBuilder, CfgResult};
use regex::Regex;
use std::str::FromStr;
use tracing::debug;

/// Build the CFG from bytecode
pub async fn build_cfg_from_bytecode(
    bytecode: &str,
) -> Result<CfgResult, Box<dyn std::error::Error>> {
    let args = CfgArgsBuilder::new().target(bytecode.to_string()).build()?;

    let result = cfg(args).await?;
    debug!("Succeed to build cfg, bytecode length: {}", bytecode.len());

    Ok(result)
}

/// Build the CFG from address
pub async fn build_cfg_from_address(
    address: &str,
) -> Result<CfgResult, Box<dyn std::error::Error>> {
    let args = CfgArgsBuilder::new().target(address.to_string()).build()?;

    let result = cfg(args).await?;
    debug!("Succeed to build cfg, address: {}", address);

    Ok(result)
}

/// build_cfg => get_nodes => loop: Starting from the CFG of the first bytecode, traverse the CFG of the second
/// bytecode, and record the one with the highest similarity.
pub fn get_nodes(cfg_result: CfgResult) -> Vec<String> {
    let mut result = vec![];
    for value in cfg_result.graph.node_indices() {
        let node = node_to_bytecode(cfg_result.graph.node_weight(value).unwrap());
        debug!("get_nodes(): {}", node);

        result.push(node);
    }
    return result;
}

/// Convert the node, which is from CFG, to bytecode
pub fn node_to_bytecode(node_str: &String) -> String {
    // regular expression deletion: all content between \n and spaces
    let node_str = format!("{}{}", "\n", node_str);
    let node_str = remove_content_between_space_and_gg(&node_str);

    // split string by space
    let parts: Vec<&str> = node_str.split_whitespace().collect();

    // further modification
    let mut previous_one: Option<String> = None;
    let processed_parts: Vec<String> = parts
        .iter()
        .map(|part| {
            let previous_opcode_str = match previous_one.clone() {
                Some(prev) => {
                    format!("{}", prev)
                }
                None => {
                    format!("")
                }
            };

            let previous_opcode = OpCode::from_str(&previous_opcode_str).unwrap();
            let previous_opcode_info: OpCodeInfo = previous_opcode.into();
            let previous_opcode_value = previous_opcode_info.opcode_value;

            if *part == "0" && previous_opcode_info.opcode_value == 0x5F {
                return "".to_string();
            }

            if *part == "0" {
                return "00".to_string();
            }

            previous_one = Some(part.to_string());

            if part.starts_with("0x") {
                if previous_opcode_value >= 0x60 && previous_opcode_value <= 0x6F {
                    // `PUSH?`
                    let bit_num = (previous_opcode_value - 0x5f) * 2;
                    match u32::from_str_radix(part.trim_start_matches("0x"), 16) {
                        Ok(value) => {
                            return format!("{:01$x}", value, bit_num as usize);
                        }
                        Err(e) => println!("U32 conversion failed: {}", e),
                    }
                }

                return part.trim_start_matches("0x").to_string();
            }

            let current_opcode = OpCode::from_str(part).unwrap();
            let current_opcode_info: OpCodeInfo = current_opcode.into();
            let current_opcode_value = current_opcode_info.opcode_value;
            if current_opcode_value != 0x2F {
                return format!("{:02x}", current_opcode_value);
            }

            return part.to_string();
        })
        .collect();

    return processed_parts.join("");
}

/// Remove the content between space and gg
fn remove_content_between_space_and_gg(input: &str) -> String {
    let re = Regex::new(r"(?m)^[^\S\n]*[^ \n]+ ").unwrap();
    re.replace_all(input, "").to_string()
}
