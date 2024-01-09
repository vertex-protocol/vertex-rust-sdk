use serde::{Deserialize, Serialize};

use ethers::types::H160;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub node_url: String,
    pub quote: H160,
    pub clearinghouse: H160,
    pub endpoint: H160,
    pub spot_engine: H160,
    pub perp_engine: H160,
    pub querier: H160,
    pub fee_calculator: H160,
}
