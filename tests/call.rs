use dotenv::dotenv;
use paprika::call;

#[ctor::ctor]
fn init() {
    dotenv().ok();
}

#[tokio::test]
async fn test_simulate_call() {
    let result = call::simulate_call(1, "0x0B63dac9ae136D953164B8a17FA2Fe81c21De36E", "0xdAC17F958D2ee523a2206206994597C13D831ec7", "0xa9059cbb0000000000000000000000007a457b6170000c77d2678b28d701b2713c66fa440000000000000000000000000000000000000000000000000000000000000001").await.unwrap();
    println!("{:?}", result);
}
