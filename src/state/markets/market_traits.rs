use borsh::{BorshDeserialize, BorshSerialize};
use sokoban::OrderedNodeAllocatorMap;

use crate::{quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick}, state::Side};

pub trait RestingOrder {
    fn size(&self) -> u64;
    fn last_valid_slot(&self) -> Option<u64>;
    fn last_valid_unix_timestamp_in_seconds(&self) -> Option<u64>;
    fn is_expired(&self, current_slot: u64, current_timestamp: u64) -> bool;
}

pub trait OrderId {
    fn price_in_ticks(&self) -> u64;
}

pub trait Market<
    MarketTraderId: BorshDeserialize + BorshSerialize + Copy,
    MarketOrderId: OrderId,
    MarketRestingOrder: RestingOrder,
>
{
    fn get_taker_fee_bps(&self) -> u64;
    fn get_tick_size(&self) -> QuoteLotsPerBaseUnitPerTick;
    fn get_base_lots_per_base_unit(&self) -> BaseLotsPerBaseUnit;
    fn get_sequence_number(&self) -> u64;
    fn get_book(
        &self,
        side: Side,
    ) -> &dyn OrderedNodeAllocatorMap<MarketOrderId, MarketRestingOrder>;
}
