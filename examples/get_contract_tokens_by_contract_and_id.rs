use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let contract_addresses = "your contract addresses";
    let token_identifiers = "your token identifiers";
    let response = client
        .get_contract_tokens_by_contract_and_id(contract_addresses, token_identifiers)
        .chain_id("your chain id")
        .limit(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
