use nxyz::NxyzClient;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let wallet_address = "your wallet address";
    let response = client.get_wallet_balances(wallet_address).send().await.unwrap();
    println!("{:#?}", response);
}
