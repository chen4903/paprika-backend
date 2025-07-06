#![allow(non_snake_case)]
#![allow(unused_variables)]
use crate::abi;
use crate::cache::{get_cached_runtime_code, get_cached_signature};
use crate::call;
use crate::call::Params;
use crate::compare_by_cfg;
use crate::constants::DEFAULT_API_PORT;
use crate::disassemble;
use crate::guess_magic_result;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Serialize;
use serde_json;
use structopt::StructOpt;
use tracing::debug;

#[derive(Debug, StructOpt)]
#[structopt(name = "Paprika", about = "A tool for analyzing EVM bytecode")]
pub struct Cli {
    #[structopt(short, long, default_value = DEFAULT_API_PORT)]
    pub port: u16,
}

#[derive(serde::Deserialize)]
struct CfgParams {
    chain_id: u64,
    address1: String,
    address2: String,
}

#[derive(serde::Deserialize)]
struct GetJsonAbiParams {
    chain_id: u64,
    address: String,
}

#[derive(serde::Deserialize)]
struct GetSignatureBySelectorParams {
    chain_id: u64,
    address: String,
}

#[derive(serde::Deserialize, Debug)]
struct GetCalldataParams {
    chain_id: u64,
    address: String,
    function_name: String,
    params: Vec<Params>,
}

#[derive(serde::Deserialize, Debug)]
struct SimulateCallParams {
    chain_id: u64,
    from: String,
    to: String,
    impl_address: String,
    function_name: String,
    params: Vec<Params>,
}

#[derive(serde::Deserialize, Debug)]
struct GetGuessMagicResultParams {
    hex_string: String,
}

#[derive(serde::Deserialize)]
struct GetUiAbiParams {
    chain_id: u64,
    address: String,
}

#[derive(serde::Deserialize)]
struct DisassembleParams {
    chain_id: u64,
    address1: String,
    address2: String,
}

#[derive(Serialize)]
struct ApiResponse {
    status: bool,
    result: serde_json::Value,
    #[serde(skip_serializing_if = "String::is_empty")]
    error: String,
}

fn check_valid_address(address: &str) -> bool {
    address.starts_with("0x")
        && address.len() == 42
        && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}

#[post("/compare_by_cfg")]
async fn api_compare_by_cfg(params: web::Json<CfgParams>) -> impl Responder {
    if !check_valid_address(&params.address1) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid address1"),
        });
    }
    if !check_valid_address(&params.address2) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid address2"),
        });
    }

    let bytecode1 = get_cached_runtime_code(params.chain_id, &params.address1)
        .await
        .unwrap();
    let bytecode2 = get_cached_runtime_code(params.chain_id, &params.address2)
        .await
        .unwrap();

    let result = compare_by_cfg::compare_by_cfg(&bytecode1, &bytecode2).await;
    HttpResponse::Ok().json(ApiResponse {
        status: true,
        result: serde_json::to_value(result).unwrap_or_default(),
        error: String::new(),
    })
}

#[post("/get_json_abi")]
async fn api_get_json_abi(params: web::Json<GetJsonAbiParams>) -> impl Responder {
    if !check_valid_address(&params.address) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid address"),
        });
    }

    let bytecode = match get_cached_runtime_code(params.chain_id, &params.address).await {
        Ok(code) => code,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                status: false,
                result: serde_json::Value::Null,
                error: format!("Failed to get runtime code: {}", e),
            });
        }
    };

    let result = abi::abi_json_wrapper(&bytecode).unwrap();
    HttpResponse::Ok().json(ApiResponse {
        status: true,
        result: serde_json::to_value(result).unwrap_or_default(),
        error: String::new(),
    })
}

#[post("/get_guess_magic_result")]
async fn api_guess_magic_result(params: web::Json<GetGuessMagicResultParams>) -> impl Responder {
    let result = guess_magic_result::magic_guess_result(&params.hex_string);
    HttpResponse::Ok().json(ApiResponse {
        status: true,
        result: serde_json::to_value(result).unwrap_or_default(),
        error: String::new(),
    })
}

#[post("/get_signature_by_selector")]
async fn api_get_signature_by_selector(
    params: web::Json<GetSignatureBySelectorParams>,
) -> impl Responder {
    if !check_valid_address(&params.address) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid address"),
        });
    }

    let bytecode = match get_cached_runtime_code(params.chain_id, &params.address).await {
        Ok(code) => code,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                status: false,
                result: serde_json::Value::Null,
                error: format!("Failed to get runtime code: {}", e),
            });
        }
    };

    debug!("bytecode: {}", bytecode);
    let selectors = match abi::get_selectors(&bytecode) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                status: false,
                result: serde_json::Value::Null,
                error: format!("Failed to get selectors: {}", e),
            });
        }
    };

    let mut results = Vec::new();
    for selector in selectors {
        debug!("api_get_signature_by_selector::selector: {}", selector);
        match get_cached_signature(&selector).await {
            Ok(signature) => {
                results.push(vec![selector, signature]);
            }
            Err(e) => {
                println!(
                    "Warning: Failed to get signature for selector {}: {}",
                    selector, e
                );
                continue;
            }
        }
    }

    HttpResponse::Ok().json(ApiResponse {
        status: true,
        result: serde_json::to_value(results).unwrap_or_default(),
        error: String::new(),
    })
}

#[post("/get_calldata")]
async fn api_get_calldata(params: web::Json<GetCalldataParams>) -> impl Responder {
    if !check_valid_address(&params.address) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid address"),
        });
    }

    let bytecode = match get_cached_runtime_code(params.chain_id, &params.address).await {
        Ok(code) => code,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                status: false,
                result: serde_json::Value::Null,
                error: format!("Failed to get runtime code: {}", e),
            });
        }
    };

    let result = call::get_guessed_calldata(&bytecode, &params.function_name, &params.params)
        .await
        .unwrap();
    HttpResponse::Ok().json(ApiResponse {
        status: true,
        result: serde_json::to_value(result).unwrap_or_default(),
        error: String::new(),
    })
}

#[post("/simulate_call")]
async fn api_simulate_call(params: web::Json<SimulateCallParams>) -> impl Responder {
    if !check_valid_address(&params.from) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid from address"),
        });
    }

    if !check_valid_address(&params.to) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid to address"),
        });
    }

    let bytecode = match get_cached_runtime_code(params.chain_id, &params.to).await {
        Ok(code) => code,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                status: false,
                result: serde_json::Value::Null,
                error: format!("Failed to get runtime code: {}", e),
            });
        }
    };

    let calldata = match call::get_guessed_calldata(
        &bytecode,
        &params.function_name,
        &params.params,
    )
    .await
    {
        Ok(data) => data,
        Err(e) => {
            // If the calldata cannot be obtained using the to address's bytecode, try using the impl_address
            match get_cached_runtime_code(params.chain_id, &params.impl_address).await {
                Ok(impl_code) => {
                    match call::get_guessed_calldata(
                        &impl_code,
                        &params.function_name,
                        &params.params,
                    )
                    .await
                    {
                        Ok(data) => data,
                        Err(impl_err) => {
                            return HttpResponse::BadRequest().json(ApiResponse {
                                status: false,
                                result: serde_json::Value::Null,
                                error: format!("Unable to obtain calldata. Failed with to address: {}, failed with impl address:{}", e, impl_err),
                            });
                        }
                    }
                }
                Err(impl_err) => {
                    return HttpResponse::BadRequest().json(ApiResponse {
                        status: false,
                        result: serde_json::Value::Null,
                        error: format!(
                            "Runtime code that cannot obtain the impl address: {}",
                            impl_err
                        ),
                    });
                }
            }
        }
    };

    let result = call::simulate_call(params.chain_id, &params.from, &params.to, &calldata)
        .await
        .unwrap();
    HttpResponse::Ok().json(ApiResponse {
        status: true,
        result: serde_json::to_value(result).unwrap_or_default(),
        error: String::new(),
    })
}

#[post("/get_ui_abi")]
async fn api_get_ui_abi(params: web::Json<GetUiAbiParams>) -> impl Responder {
    if !check_valid_address(&params.address) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid address"),
        });
    }

    let bytecode = get_cached_runtime_code(params.chain_id, &params.address)
        .await
        .unwrap();

    let result = abi::abi_ui_wrapper(&bytecode).unwrap();
    HttpResponse::Ok().json(ApiResponse {
        status: true,
        result: serde_json::to_value(result).unwrap_or_default(),
        error: String::new(),
    })
}

#[post("/disassemble")]
async fn api_disassemble_bytecode(params: web::Json<DisassembleParams>) -> impl Responder {
    if !check_valid_address(&params.address1) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid address1"),
        });
    }

    if !check_valid_address(&params.address2) {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: false,
            result: serde_json::Value::Null,
            error: format!("Invalid address2"),
        });
    }

    let bytecode1 = match get_cached_runtime_code(params.chain_id, &params.address1).await {
        Ok(code) => code,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                status: false,
                result: serde_json::Value::Null,
                error: format!("Failed to get runtime code: {}", e),
            });
        }
    };

    let bytecode2 = match get_cached_runtime_code(params.chain_id, &params.address2).await {
        Ok(code) => code,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                status: false,
                result: serde_json::Value::Null,
                error: format!("Failed to get runtime code: {}", e),
            });
        }
    };

    let result1 = disassemble::build_disassemble_from_bytecode(&bytecode1)
        .await
        .unwrap();
    let result2 = disassemble::build_disassemble_from_bytecode(&bytecode2)
        .await
        .unwrap();

    HttpResponse::Ok().json(ApiResponse {
        status: true,
        result: serde_json::to_value(vec![result1, result2]).unwrap_or_default(),
        error: String::new(),
    })
}
