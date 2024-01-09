use vertex_sdk::sanity::execute::execute_sanity_check;
use vertex_sdk::sanity::indexer::indexer_sanity_check;
use vertex_sdk::sanity::query::query_sanity_check;

#[tokio::main]
async fn main() {
    if contains_arg("--execute-sanity") {
        execute_sanity_check().await.unwrap();
    } else if contains_arg("--query-sanity") {
        query_sanity_check().await.unwrap();
    } else if contains_arg("--indexer-sanity") {
        indexer_sanity_check().await.unwrap();
    } else {
        execute_sanity_check().await.unwrap();
        query_sanity_check().await.unwrap();
        indexer_sanity_check().await.unwrap();
    }
}

fn contains_arg(arg: &str) -> bool {
    std::env::args().any(|x| x == *arg)
}
