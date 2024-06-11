use crate::math::to_i128_fp;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Product {
    #[serde(rename_all = "camelCase")]
    Spot {
        product_id: u32,
        symbol: String,
        name: String,
        quote: String,
        long_weight_initial: f64,
        long_weight_maintenance: f64,
        short_weight_initial: f64,
        short_weight_maintenance: f64,
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
        feed_id: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Perp {
        product_id: u32,
        symbol: String,
        name: String,
        quote: String,
        long_weight_initial: f64,
        long_weight_maintenance: f64,
        short_weight_initial: f64,
        short_weight_maintenance: f64,
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
        feed_id: Option<String>,
    },
}

impl Product {
    pub fn id(&self) -> u32 {
        match self {
            Product::Spot { product_id, .. } => *product_id,
            Product::Perp { product_id, .. } => *product_id,
        }
    }

    pub fn min_size(&self) -> i128 {
        match self {
            Product::Spot { min_size, .. } | Product::Perp { min_size, .. } => {
                to_i128_fp(*min_size)
            }
        }
    }

    pub fn is_spot(&self) -> bool {
        matches!(self, Product::Spot { .. })
    }

    pub fn quote(&self) -> String {
        match self {
            Product::Spot { quote, .. } => quote.clone(),
            Product::Perp { quote, .. } => quote.clone(),
        }
    }

    pub fn feed_id(&self) -> Option<String> {
        match self {
            Product::Spot { feed_id, .. } => feed_id.clone(),
            Product::Perp { feed_id, .. } => feed_id.clone(),
        }
    }

    pub fn asset_id(&self) -> String {
        match self {
            Product::Spot { price_asset_id, .. } => price_asset_id.clone(),
            Product::Perp { price_asset_id, .. } => price_asset_id.clone(),
        }
    }
}
