use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Product {
    #[serde(rename_all = "camelCase")]
    Spot {
        symbol: String,
        name: String,
        quote: String,
        health_group: u32,
        long_weight_initial: f64,
        long_weight_maintenance: f64,
        short_weight_initial: f64,
        short_weight_maintenance: f64,
        large_position_penalty: f64,
        size_increment: f64,
        price_increment: f64,
        lp_spread: f64,
        grid_depth: f64,
        interest_inflection_util: f64,
        interest_floor: f64,
        interest_small_cap: f64,
        interest_large_cap: f64,
        initial_price: f64,
        address: String,
        decimals: u8,
        price_asset_id: String,
        min_size: f64,
    },
    #[serde(rename_all = "camelCase")]
    Perp {
        symbol: String,
        name: String,
        quote: String,
        health_group: u32,
        long_weight_initial: f64,
        long_weight_maintenance: f64,
        short_weight_initial: f64,
        short_weight_maintenance: f64,
        large_position_penalty: f64,
        size_increment: f64,
        price_increment: f64,
        lp_spread: f64,
        grid_depth: f64,
        initial_price: f64,
        price_asset_id: String,
        spot_index_asset_id: String,
        min_size: f64,
        // 0 for no limit
        max_open_interest: Option<f64>,
    },
}
