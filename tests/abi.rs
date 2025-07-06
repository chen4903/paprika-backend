use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    primitives::{address, U256},
    providers::Provider,
};
use alloy_network::TransactionBuilder;
use alloy_provider::ProviderBuilder;
use alloy_rpc_types_eth::TransactionRequest;
use eyre::Ok;
use paprika::abi;

const HELLO_RUNTIME_BYTECODE: &str ="608060405234801561000f575f80fd5b506004361061004a575f3560e01c806338d52e0f1461004e5780633e4c38a91461006c57806370a082311461008a57806376809ce3146100ba575b5f80fd5b6100566100d8565b6040516100639190610113565b60405180910390f35b6100746100dd565b604051610081919061019c565b60405180910390f35b6100a4600480360381019061009f919061021a565b6100e9565b6040516100b19190610113565b60405180910390f35b6100c26100f3565b6040516100cf9190610260565b60405180910390f35b5f5481565b606060de5f8190555090565b5f80549050919050565b5f6012905090565b5f819050919050565b61010d816100fb565b82525050565b5f6020820190506101265f830184610104565b92915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f61016e8261012c565b6101788185610136565b9350610188818560208601610146565b61019181610154565b840191505092915050565b5f6020820190508181035f8301526101b48184610164565b905092915050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6101e9826101c0565b9050919050565b6101f9816101df565b8114610203575f80fd5b50565b5f81359050610214816101f0565b92915050565b5f6020828403121561022f5761022e6101bc565b5b5f61023c84828501610206565b91505092915050565b5f60ff82169050919050565b61025a81610245565b82525050565b5f6020820190506102735f830184610251565b9291505056fea2646970667358221220d41a840b2b3f5ec6a0c32dc701790b0012bdab6fbf98396368602d955801bbd864736f6c634300081a0033";
const HELLO_INIT_BYTECODE: &str ="6080604052606f5f553480156012575f80fd5b506102af806100205f395ff3fe608060405234801561000f575f80fd5b506004361061004a575f3560e01c806338d52e0f1461004e5780633e4c38a91461006c57806370a082311461008a57806376809ce3146100ba575b5f80fd5b6100566100d8565b6040516100639190610113565b60405180910390f35b6100746100dd565b604051610081919061019c565b60405180910390f35b6100a4600480360381019061009f919061021a565b6100e9565b6040516100b19190610113565b60405180910390f35b6100c26100f3565b6040516100cf9190610260565b60405180910390f35b5f5481565b606060de5f8190555090565b5f80549050919050565b5f6012905090565b5f819050919050565b61010d816100fb565b82525050565b5f6020820190506101265f830184610104565b92915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f61016e8261012c565b6101788185610136565b9350610188818560208601610146565b61019181610154565b840191505092915050565b5f6020820190508181035f8301526101b48184610164565b905092915050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6101e9826101c0565b9050919050565b6101f9816101df565b8114610203575f80fd5b50565b5f81359050610214816101f0565b92915050565b5f6020828403121561022f5761022e6101bc565b5b5f61023c84828501610206565b91505092915050565b5f60ff82169050919050565b61025a81610245565b82525050565b5f6020820190506102735f830184610251565b9291505056fea2646970667358221220d41a840b2b3f5ec6a0c32dc701790b0012bdab6fbf98396368602d955801bbd864736f6c634300081a0033";

#[test]
fn test_abi_ui_wrapper() {
    for value in paprika::abi::abi_ui_wrapper(HELLO_RUNTIME_BYTECODE).unwrap() {
        assert_eq!(value.starts_with("paprika_guess_"), true);
        println!("{}", value);
    }
}

#[test]
fn test_abi_json_wrapper() {
    let result = paprika::abi::abi_json_wrapper(HELLO_RUNTIME_BYTECODE).unwrap();
    println!("{}", result);
    assert!(result.len() > 0);
}

async fn test_contract_interaction(abi_str: &str, input_func: &str) -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_anvil_with_wallet();
    let abi = serde_json::from_str::<JsonAbi>(abi_str).unwrap();

    let bytecode = hex::decode(HELLO_INIT_BYTECODE).unwrap();

    let instance_address = provider
        .send_transaction(TransactionRequest::default().with_deploy_code(bytecode))
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap()
        .contract_address
        .unwrap();

    let contract = ContractInstance::new(instance_address, provider, Interface::new(abi));

    if input_func == "decimal" {
        // check the decimal is 18
        let result = contract
            .function(input_func, &[])
            .unwrap()
            .call()
            .await
            .unwrap();
        assert_eq!(result[0].as_uint().unwrap().0, U256::from(18));

        // write
        let tx_hash = contract
            .function("changeBalance", &[])
            .unwrap()
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();
        println!("tx_hash: {tx_hash}");
    } else if input_func == "paprika_guessed_70a08231" {
        let arg_address = DynSolValue::from(address!("0x0000000000000000000000000000000000000000"));
        let result = contract
            .function(input_func, &[arg_address]) // balanceOf(address) - paprika_guessed_70a08231
            .unwrap()
            .call_raw()
            .await
            .unwrap();

        let bytes_data = result.as_ref();
        println!("Raw bytes data (hex): 0x{}", hex::encode(bytes_data));

        let tx_hash = contract
            .function("paprika_guessed_3e4c38a9", &[])? // changeBalance() - paprika_guessed_3e4c38a9
            .send()
            .await?
            .watch()
            .await?;

        println!("Set asset from 111 to 222: {tx_hash}");

        // balanceOf(address)
        let address = DynSolValue::from(address!("0x0000000000000000000000000000000000000000"));
        let result = contract
            .function(input_func, &[address]) // balanceOf(address) - paprika_guessed_70a08231
            .unwrap()
            .call_raw()
            .await
            .unwrap();

        let bytes_data = result.as_ref();
        println!("Raw bytes data (hex): 0x{}", hex::encode(bytes_data));
    }

    Ok(())
}

#[tokio::test]
async fn test_call_verify_abi() {
    let _ = test_contract_interaction(
        r#"[{"inputs": [],"name": "asset","outputs": [{"internalType": "uint256","name": "","type": "uint256"}],"stateMutability": "view","type": "function"},{"inputs": [{"internalType": "address","name": "","type": "address"}],"name": "balanceOf","outputs": [{"internalType": "uint256","name": "","type": "uint256"}],"stateMutability": "view","type": "function"},{"inputs": [],"name": "changeBalance","outputs": [{"internalType": "bytes","name": "","type": "bytes"}],"stateMutability": "nonpayable","type": "function"},{"inputs": [],"name": "decimal","outputs": [{"internalType": "uint8","name": "","type": "uint8"}],"stateMutability": "pure","type": "function"}]"#,
        "decimal"
    ).await;
}

#[tokio::test]
async fn test_call_guessed_abi1() {
    let _ = test_contract_interaction(
        r#"[{"inputs":[],"name":"paprika_guessed_38d52e0f","outputs":[{"name":"","type":"bytes"}],"payable":true,"stateMutability":"payable","type":"function"},{"inputs":[],"name":"paprika_guessed_3e4c38a9","outputs":[{"name":"","type":"bytes"}],"payable":true,"stateMutability":"payable","type":"function"},{"inputs":[{"name":"arg0","type":"address"}],"name":"paprika_guessed_70a08231","outputs":[{"name":"","type":"bytes"}],"payable":true,"stateMutability":"payable","type":"function"},{"inputs":[],"name":"paprika_guessed_76809ce3","outputs":[{"name":"","type":"bytes"}],"payable":true,"stateMutability":"payable","type":"function"}]"#,
        "paprika_guessed_70a08231" // balanceOf(address)
    ).await;
}

#[tokio::test]
async fn test_call_guessed_abi2() -> eyre::Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.c
    let provider = ProviderBuilder::new().on_anvil_with_wallet();

    // init code
    let bytecode = hex::decode(HELLO_INIT_BYTECODE).unwrap();
    let tx = TransactionRequest::default().with_deploy_code(bytecode);

    let instance_address = provider
        .send_transaction(tx)
        .await?
        .get_receipt()
        .await?
        .contract_address
        .expect("Failed to get contract address");

    let abi = abi::abi_json_wrapper(&HELLO_RUNTIME_BYTECODE).unwrap();
    let abi = serde_json::from_str::<JsonAbi>(&abi).unwrap();
    let contract = ContractInstance::new(instance_address, provider.clone(), Interface::new(abi));

    let arg_address = DynSolValue::from(address!("0x0000000000000000000000000000000000000000"));

    // get calldata, so that we could pass it to the front-end
    let function = contract.function("paprika_guessed_70a08231", &[arg_address.clone()])?;
    let calldata = function.calldata();
    println!("calldata: {:#?}", calldata);

    let tx_hash = contract
        .function("paprika_guessed_70a08231", &[arg_address])?
        .send()
        .await?
        .watch()
        .await?;

    println!("tx_hash: {tx_hash}");

    let receipt = provider.get_transaction_by_hash(tx_hash).await;
    println!("receipt: {:?}", receipt);

    let result = provider.get_transaction_receipt(tx_hash);
    println!("result: {:?}", result);

    Ok(())
}

/*

contract Hello {
    uint256 public asset = 111;

    function decimal() public pure returns(uint8) {
        return 18;
    }

    function balanceOf(address) public view returns(uint256) {
        return asset;
    }

    function changeBalance() public returns(bytes memory){
        asset = 222;
    }
}

*/
