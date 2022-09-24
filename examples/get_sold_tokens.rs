use nxyz::NxyzClient;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let wallet_address = "your wallet address";
    let response = client.get_sold_tokens(wallet_address).send().await.unwrap();
    println!("{:#?}", response);
}
