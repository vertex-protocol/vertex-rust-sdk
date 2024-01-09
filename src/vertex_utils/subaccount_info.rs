use std::collections::HashMap;

use eyre::{eyre, Result};

use crate::bindings::querier::{HealthInfo, PerpBalance, PerpProduct, SpotBalance};
use crate::engine::{AllProductsResponse, SpotProduct, SubaccountInfoResponse};

impl SubaccountInfoResponse {
    pub fn get_spot_balance(&self, product_id: u32) -> Result<&SpotBalance> {
        let balance = self
            .spot_balances
            .iter()
            .find(|&balance| balance.product_id == product_id)
            .ok_or(eyre!(
                "spot balance not found for product_id: {}",
                product_id
            ))?;
        Ok(balance)
    }

    pub fn get_perp_balance(&self, product_id: u32) -> Result<&PerpBalance> {
        let balance = self
            .perp_balances
            .iter()
            .find(|&balance| balance.product_id == product_id)
            .ok_or(eyre!(
                "spot balance not found for product_id: {}",
                product_id
            ))?;
        Ok(balance)
    }

    pub fn get_spot_product(&self, product_id: u32) -> Result<&SpotProduct> {
        let product = self
            .spot_products
            .iter()
            .find(|&product| product.product_id == product_id)
            .ok_or(eyre!(
                "spot product not found for product_id: {}",
                product_id
            ))?;
        Ok(product)
    }

    pub fn get_perp_product(&self, product_id: u32) -> Result<&PerpProduct> {
        let product = self
            .perp_products
            .iter()
            .find(|&product| product.product_id == product_id)
            .ok_or(eyre!(
                "perp product not found for product_id: {}",
                product_id
            ))?;
        Ok(product)
    }

    pub fn get_product_balances(&self, product_ids: Vec<u32>) -> Result<Vec<i128>> {
        let mut ret = vec![];
        for product_id in &product_ids {
            if *product_id != 0 && *product_id % 2 == 0 {
                let balance = self.get_perp_balance(*product_id)?;
                ret.push(balance.balance.amount);
            } else {
                let balance = self.get_spot_balance(*product_id)?;
                ret.push(balance.balance.amount);
            }
        }
        let mut quote_bias = 0;
        for product_id in [2, 4, 6] {
            let balance = self.get_perp_balance(product_id)?;
            quote_bias += balance.balance.v_quote_balance;
        }
        for index in 0..product_ids.len() {
            if product_ids[index] == 0 {
                ret[index] += quote_bias;
            }
        }
        Ok(ret)
    }

    pub fn get_lp_balances(&self, product_ids: Vec<u32>) -> Result<Vec<i128>> {
        let mut ret = vec![];
        for product_id in &product_ids {
            if *product_id != 0 && *product_id % 2 == 0 {
                let balance = self.get_perp_balance(*product_id)?;
                ret.push(balance.lp_balance.amount);
            } else {
                let balance = self.get_spot_balance(*product_id)?;
                ret.push(balance.lp_balance.amount);
            }
        }
        Ok(ret)
    }

    pub fn with_corrected_fees(&mut self, mut all_products: AllProductsResponse) -> Self {
        let mut subaccount_info = self.clone();
        let quote = all_products.spot_products.remove(0);
        all_products.spot_products.push(quote);

        let mut spot_products_without_collected_fees = all_products.spot_products.clone();
        let mut perp_products_without_collected_fees = all_products.perp_products.clone();
        for spot_product in &mut spot_products_without_collected_fees {
            spot_product.book_info.collected_fees = 0;
        }
        for perp_product in &mut perp_products_without_collected_fees {
            perp_product.book_info.collected_fees = 0;
        }
        assert_eq!(
            subaccount_info.spot_products,
            spot_products_without_collected_fees
        );
        assert_eq!(
            subaccount_info.perp_products,
            perp_products_without_collected_fees
        );
        subaccount_info.spot_products = all_products.spot_products;
        subaccount_info.perp_products = all_products.perp_products;
        subaccount_info
    }

    pub fn validate_size_increments(&self) {
        let mut size_increments: HashMap<u32, i128> = HashMap::new();
        for perp_product in self.perp_products.iter() {
            if perp_product.book_info.size_increment == 0
                && perp_product.risk.long_weight_initial_x18 == 0
            {
                // placeholder product
                continue;
            }
            let open_interest = perp_product.state.open_interest;
            let size_increment = perp_product.book_info.size_increment;
            size_increments.insert(perp_product.product_id, size_increment);
            assert_eq!(open_interest % size_increment, 0);
        }
        for perp_balance in self.perp_balances.iter() {
            if !size_increments.contains_key(&perp_balance.product_id) {
                continue;
            }
            let balance = perp_balance.balance.amount;
            let size_increment = size_increments.get(&perp_balance.product_id).unwrap();
            assert_eq!(balance % size_increment, 0);
        }
    }

    pub fn get_states(&self, product_ids: Vec<u32>) -> Result<Vec<Vec<i128>>> {
        let mut ret = vec![];
        for product_id in product_ids {
            let mut state = vec![];
            if product_id != 0 && product_id % 2 == 0 {
                let perp = self.get_perp_product(product_id)?;
                state.push(perp.state.cumulative_funding_long_x18);
                state.push(perp.state.cumulative_funding_short_x18);
                state.push(perp.state.open_interest);
            } else {
                let spot = self.get_spot_product(product_id)?;
                state.push(spot.state.cumulative_deposits_multiplier_x18);
                state.push(spot.state.cumulative_borrows_multiplier_x18);
                state.push(spot.state.total_deposits_normalized);
                state.push(spot.state.total_borrows_normalized);
            }
            ret.push(state);
        }
        Ok(ret)
    }

    pub fn get_lp_states(&self, product_ids: Vec<u32>) -> Result<Vec<Vec<i128>>> {
        let mut ret = vec![];
        for product_id in product_ids {
            let mut lp_state = vec![];
            if product_id != 0 && product_id % 2 == 0 {
                let perp = self.get_perp_product(product_id)?;
                lp_state.push(perp.lp_state.supply);
                lp_state.push(perp.lp_state.base);
                lp_state.push(perp.lp_state.quote);
            } else {
                let spot = self.get_spot_product(product_id)?;
                lp_state.push(spot.lp_state.supply);
                lp_state.push(spot.lp_state.base.amount);
                lp_state.push(spot.lp_state.quote.amount);
            }
            ret.push(lp_state)
        }
        Ok(ret)
    }

    pub fn get_health_info(&self) -> (HealthInfo, HealthInfo, HealthInfo) {
        (
            self.healths[0].clone(),
            self.healths[1].clone(),
            self.healths[2].clone(),
        )
    }
}
