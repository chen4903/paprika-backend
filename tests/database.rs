use eyre::Result;
use paprika::database::Database;
use std::sync::Arc;

#[tokio::test]
async fn test_database_operations() -> Result<()> {
    let db = Database::new(true).unwrap();
    let db = Arc::new(db);

    // runtime code
    let chain_id = 1;
    let address = "0x123";
    let runtime_code = "0x456";
    let _ = db.save_runtime_code(chain_id, address, runtime_code);
    let result = db.get_runtime_code(chain_id, address).unwrap();
    assert_eq!(result, Some(runtime_code.to_string()));

    // insert duplicate runtime code
    let _ = db.save_runtime_code(chain_id, address, "new_code");
    let result = db.get_runtime_code(chain_id, address).unwrap();
    assert_eq!(result, Some(runtime_code.to_string())); //  should still return the original signature

    // selector
    let selector = "0xabcd";
    let signature = "transfer(address,uint256)";
    let _ = db.save_signature(selector, signature);
    let result = db.get_signature(selector).unwrap();
    assert_eq!(result, Some(signature.to_string()));

    // insert duplicate signature
    let _ = db.save_signature(selector, "new_signature");
    let result = db.get_signature(selector).unwrap();
    assert_eq!(result, Some(signature.to_string())); // should still return the original signature

    // get non-existent runtime code
    let result = db.get_runtime_code(999, "0xabc").unwrap();
    assert_eq!(result, None);

    // get non-existent signature
    let result = db.get_signature("0x9999").unwrap();
    assert_eq!(result, None);

    Ok(())
}
