use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let wallet_address = "your wallet address";
    let response = client
        .get_wallet_balances(wallet_address)
        .limit(1)
        .chain_id("your chain id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
