use nxyz::NxyzClient;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let response = client.get_tokens_suggestions_results().send().await.unwrap();
    println!("{:#?}", response);
}
