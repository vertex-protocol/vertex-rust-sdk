use std::collections::HashMap;
use vertex_sdk::engine::SymbolsResponseData;
use vertex_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let client = VertexClient::new(ClientMode::SepoliaTest);

    // optionally specify product type or specific symbol
    let spot_symbols = client
        .get_symbols(None, Some("spot".to_string()))
        .await
        .unwrap();

    let symbols_to_product_info: HashMap<String, SymbolsResponseData> = spot_symbols.symbols;

    // get product info like product id
    let btc = symbols_to_product_info.get("BTC").unwrap();
    println!("BTC product_id: {}", btc.product_id);

    let perp_symbols = client
        .get_symbols(None, Some("perp".to_string()))
        .await
        .unwrap();

    let symbols_to_product_info: HashMap<String, SymbolsResponseData> = perp_symbols.symbols;

    let sol_perp = symbols_to_product_info.get("SOL-PERP").unwrap();
    println!("SOL-PERP product_id: {}", sol_perp.product_id);
}
