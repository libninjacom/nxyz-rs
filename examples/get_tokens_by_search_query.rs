use nxyz::NxyzClient;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let query = "your query";
    let response = client.get_tokens_by_search_query(query).send().await.unwrap();
    println!("{:#?}", response);
}
