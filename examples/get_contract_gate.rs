use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let contract_address = "your contract address";
    let wallet_address = "your wallet address";
    let response = client
        .get_contract_gate(contract_address, wallet_address)
        .chain_id("your chain id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
