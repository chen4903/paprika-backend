use heimdall_disassembler::{disassemble, DisassemblerArgsBuilder};
use tracing::debug;

/// Build the disassemble from bytecode
pub async fn build_disassemble_from_bytecode(
    bytecode: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // remove the prefix '0x' (if present)
    let bytecode = if bytecode.starts_with("0x") || bytecode.starts_with("0X") {
        &bytecode[2..]
    } else {
        bytecode
    };

    debug!("Disassembling bytecode: {}", bytecode);
    let args = DisassemblerArgsBuilder::new()
        .target(bytecode.to_string())
        .build()
        .unwrap();

    let result = match disassemble(args).await {
        Ok(res) => res,
        Err(_) => return Ok(bytecode.to_string()),
    };
    debug!("Succeed to build cfg, bytecode length: {}", bytecode.len());

    Ok(result)
}
