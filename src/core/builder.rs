use crate::builders::execute::burn_lp::BurnLpBuilder;
use crate::builders::execute::cancellation::CancellationBuilder;
use crate::builders::execute::cancellation_products::CancellationProductsBuilder;
use crate::builders::execute::deposit_collateral::DepositCollateralBuilder;
use crate::builders::execute::deposit_insurance::DepositInsuranceBuilder;
use crate::builders::execute::link_signer::LinkSignerBuilder;
use crate::builders::execute::liquidate_subaccount::LiquidateSubaccountBuilder;
use crate::builders::execute::mint_lp::MintLpBuilder;
use crate::builders::execute::place_isolated_order::PlaceIsolatedOrderBuilder;
use crate::builders::execute::place_order::PlaceOrderBuilder;
use crate::builders::execute::slow_mode::SubmitSlowModeTxBuilder;
use crate::builders::execute::transfer_quote::TransferQuoteBuilder;
use crate::builders::execute::withdraw_collateral::WithdrawCollateralBuilder;
use crate::builders::indexer::account_snapshots::AccountSnapshotsBuilder;
use crate::builders::indexer::candlesticks::CandlesticksBuilder;
use crate::builders::indexer::events::EventsBuilder;
use crate::builders::indexer::foundation_taker_rewards::FoundationTakerRewardsBuilder;
use crate::builders::indexer::historical_orders::HistoricalOrdersBuilder;
use crate::builders::indexer::interest_and_funding::InterestAndFundingTicksBuilder;
use crate::builders::indexer::leaderboard::LeaderboardBuilder;
use crate::builders::indexer::maker_statistics::MakerStatisticsBuilder;
use crate::builders::indexer::market_snapshots::MarketSnapshotsBuilder;
use crate::builders::indexer::matches::MatchesBuilder;
use crate::builders::indexer::multi_product_snapshots::MultiProductSnapshotsBuilder;
use crate::builders::indexer::product_snapshots::ProductSnapshotsBuilder;
use crate::builders::indexer::rewards::RewardsBuilder;
use crate::builders::indexer::subaccounts::SubaccountsBuilder;
use crate::builders::indexer::trades::TradesParamsBuilder;
use crate::builders::query::max_lp_mintable::MaxLpMintableBuilder;
use crate::builders::query::max_order_size::MaxOrderSizeBuilder;
use crate::builders::query::max_withdrawable::MaxWithdrawableBuilder;
use crate::builders::trigger::list_trigger_orders::ListTriggerOrdersBuilder;
use crate::builders::utils::fee_calculator::FeeCalculator;

use crate::core::execute::VertexExecute;
use crate::core::indexer::VertexIndexer;

#[doc(hidden)]
#[macro_export]
macro_rules! get_vertex_builder {
    (pub $method_name:ident, $builder_name:ident) => {
        pub fn $method_name(&self) -> $builder_name<Self> {
            $builder_name::new(&self)
        }
    };
    ($method_name:ident, $builder_name:ident) => {
        fn $method_name(&self) -> $builder_name<Self> {
            $builder_name::new(&self)
        }
    };
}

pub trait VertexBuilder: VertexExecute + VertexIndexer {
    get_vertex_builder!(deposit_collateral_builder, DepositCollateralBuilder);
    get_vertex_builder!(place_order_builder, PlaceOrderBuilder);
    get_vertex_builder!(place_isolated_order_builder, PlaceIsolatedOrderBuilder);
    get_vertex_builder!(list_trigger_orders_builder, ListTriggerOrdersBuilder);
    get_vertex_builder!(cancellation_builder, CancellationBuilder);
    get_vertex_builder!(cancellation_products_builder, CancellationProductsBuilder);
    get_vertex_builder!(liquidate_subaccount_builder, LiquidateSubaccountBuilder);
    get_vertex_builder!(withdraw_collateral_builder, WithdrawCollateralBuilder);
    get_vertex_builder!(mint_lp_builder, MintLpBuilder);
    get_vertex_builder!(burn_lp_builder, BurnLpBuilder);
    get_vertex_builder!(link_signer_builder, LinkSignerBuilder);
    get_vertex_builder!(transfer_quote_builder, TransferQuoteBuilder);
    get_vertex_builder!(submit_slow_mode_tx_builder, SubmitSlowModeTxBuilder);
    get_vertex_builder!(deposit_insurance_builder, DepositInsuranceBuilder);

    get_vertex_builder!(get_max_order_size_builder, MaxOrderSizeBuilder);
    get_vertex_builder!(get_max_withdrawable_builder, MaxWithdrawableBuilder);
    get_vertex_builder!(get_max_lp_mintable_builder, MaxLpMintableBuilder);

    get_vertex_builder!(get_candlesticks_builder, CandlesticksBuilder);
    get_vertex_builder!(get_historical_orders_builder, HistoricalOrdersBuilder);
    get_vertex_builder!(get_events_builder, EventsBuilder);
    get_vertex_builder!(get_maker_statistics_builder, MakerStatisticsBuilder);
    get_vertex_builder!(get_matches_builder, MatchesBuilder);
    get_vertex_builder!(get_product_snapshots_builder, ProductSnapshotsBuilder);
    get_vertex_builder!(
        get_multi_product_snapshots_builder,
        MultiProductSnapshotsBuilder
    );
    get_vertex_builder!(get_account_snapshots_builder, AccountSnapshotsBuilder);
    get_vertex_builder!(get_subaccounts_builder, SubaccountsBuilder);
    get_vertex_builder!(
        get_interest_and_funding_builder,
        InterestAndFundingTicksBuilder
    );
    get_vertex_builder!(get_market_snapshots_builder, MarketSnapshotsBuilder);
    get_vertex_builder!(get_trades_builder, TradesParamsBuilder);
    get_vertex_builder!(fee_calculator, FeeCalculator);
    get_vertex_builder!(get_leaderboard_builder, LeaderboardBuilder);
    get_vertex_builder!(get_rewards_builder, RewardsBuilder);
    get_vertex_builder!(
        get_foundation_taker_rewards_builder,
        FoundationTakerRewardsBuilder
    );
}
