use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let wallet_address = "your wallet address";
    let response = client
        .get_wallet_mints(wallet_address)
        .chain_id("your chain id")
        .cursor("your cursor")
        .limit(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
