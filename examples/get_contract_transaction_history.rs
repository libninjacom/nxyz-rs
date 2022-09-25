use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let contract_address = "your contract address";
    let response = client
        .get_contract_transaction_history(contract_address)
        .cursor("your cursor")
        .chain_id("your chain id")
        .limit(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
