use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let contract_address = "your contract address";
    let token_id = "your token id";
    let response = client
        .get_token_transfers(contract_address, token_id)
        .chain_id("your chain id")
        .limit(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
