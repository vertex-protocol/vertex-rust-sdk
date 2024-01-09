use crate::core::VertexQuery;
use crate::engine::FeeRatesResponse;
use crate::math::{div_x18, mul_x18, to_i128_x18};
use crate::utils::client_error::none_error;
use crate::{fields_to_vars, vertex_builder};
use eyre::Result;

vertex_builder!(
    FeeCalculator,
    VertexQuery,
    subaccount: [u8; 32],
    product_id: u32,
    amount: i128,
    taker: bool,
    include_sequencer_fee: bool;

    pub async fn calc_fee(&self) -> i128 {
        self.calc_fee_inner().await.unwrap()
    }

    async fn calc_fee_inner(&self) -> Result<i128> {
        fields_to_vars!(self, amount, taker, include_sequencer_fee);

        let fee_rates = self.get_fee_rates().await?;
        let keep_rate = self.get_keep_rate(&fee_rates)?;

        let mut new_amt = if amount.is_negative() {
            div_x18(amount, keep_rate)
        } else {
            mul_x18(amount, keep_rate)
        };
        if taker && include_sequencer_fee {
            new_amt = new_amt.saturating_sub(fee_rates.taker_sequencer_fee);
        }
        Ok(amount.saturating_sub(new_amt))
    }

    async fn get_fee_rates(&self) -> Result<FeeRatesResponse> {
        let subaccount = self.subaccount.unwrap_or(self.vertex.subaccount()?);
        self.vertex.get_fee_rates(subaccount).await
    }

    fn get_keep_rate(&self, fee_rates: &FeeRatesResponse) -> Result<i128> {
        fields_to_vars!(self, product_id, taker);
        let keep_rate = if taker {
            to_i128_x18(1) - fee_rates.taker_fee_rates_x18[product_id as usize]
        } else {
            to_i128_x18(1) - fee_rates.maker_fee_rates_x18[product_id as usize]
        };
        Ok(keep_rate)
    }


);
