use nxyz::NxyzClient;
use nxyz::model::*;
#[tokio::main]
async fn main() {
    let client = NxyzClient::from_env();
    let query = "your query";
    let response = client
        .get_search_results(query)
        .cursor("your cursor")
        .limit(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
