use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let wallet_address = "your wallet address";
    let response = client
        .get_wallet_transactions(wallet_address)
        .cursor("your cursor")
        .limit(1)
        .chain_id("your chain id")
        .token_type("your token type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
