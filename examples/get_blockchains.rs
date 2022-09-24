use nxyz::NxyzClient;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let response = client.get_blockchains().send().await.unwrap();
    println!("{:#?}", response);
}
