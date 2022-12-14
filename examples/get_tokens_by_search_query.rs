use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let query = "your query";
    let response = client
        .get_tokens_by_search_query(query)
        .chain_id("your chain id")
        .limit(1)
        .cursor("your cursor")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
