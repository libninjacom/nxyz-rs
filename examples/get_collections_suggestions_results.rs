use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let response = client
        .get_collections_suggestions_results()
        .query("your query")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
