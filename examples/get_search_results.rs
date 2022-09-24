use nxyz::NxyzClient;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let query = "your query";
    let response = client.get_search_results(query).send().await.unwrap();
    println!("{:#?}", response);
}
