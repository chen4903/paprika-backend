use paprika::abi::get_selectors;
use paprika::selector::get_signature_by_selector;

#[tokio::test]
async fn test_get_signature_by_selector() {
    let selector = "0x06fdde03";
    let result = get_signature_by_selector(&selector).await.unwrap();
    assert_eq!(result, "name()");
}

#[tokio::test]
async fn test_get_selectors() {
    let bytecode = "6080604052348015600e575f80fd5b50600436106030575f3560e01c80632125b65b146034578063b69ef8a8146044575b5f80fd5b6044603f3660046046565b505050565b005b5f805f606084860312156057575f80fd5b833563ffffffff811681146069575f80fd5b925060208401356001600160a01b03811681146083575f80fd5b915060408401356001600160e01b0381168114609d575f80fd5b80915050925092509256";
    let result = get_selectors(&bytecode).unwrap();
    for selector in result {
        let result = get_signature_by_selector(&selector).await.unwrap();
        println!("{}: {}", selector, result);
    }
}
