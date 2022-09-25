use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let wallet_address = "your wallet address";
    let response = client
        .get_wallet_tokens(wallet_address)
        .filter_airdrops(true)
        .cursor("your cursor")
        .chain_id("your chain id")
        .limit(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
