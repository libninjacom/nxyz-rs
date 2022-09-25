use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let token_id = "your token id";
    let contract_address = "your contract address";
    let wallet_address = "your wallet address";
    let response = client
        .get_token_gate(token_id, contract_address, wallet_address)
        .chain_id("your chain id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
