use nxyz::NxyzClient;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let contract_address = "your contract address";
    let response = client
        .get_contract_transaction_history(contract_address)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
