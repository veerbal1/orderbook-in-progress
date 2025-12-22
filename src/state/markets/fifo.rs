use std::cmp::Ordering;

use crate::{
    quantities::{BaseLots, Ticks, WrapperU64},
    state::{RestingOrder, Side},
};

#[repr(C)]
#[derive(Eq, PartialEq, Debug, Default, Copy, Clone)]
pub struct FIFOOrderId {
    pub price_in_ticks: Ticks,
    pub order_sequence_number: u64,
}

impl FIFOOrderId {
    pub fn new(ticks: Ticks, order_sequence_number: u64) -> Self {
        Self {
            price_in_ticks: ticks,
            order_sequence_number,
        }
    }

    pub fn new_from_untyped(val1: u64, val2: u64) -> Self {
        Self {
            price_in_ticks: Ticks::new(val1),
            order_sequence_number: val2,
        }
    }
}

impl PartialOrd for FIFOOrderId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let side = Side::from_order_sequence_number(self.order_sequence_number);
        let (price_cmp, seq_cmp) = match side {
            Side::Ask => (
                self.price_in_ticks.partial_cmp(&other.price_in_ticks)?,
                self.order_sequence_number
                    .partial_cmp(&other.order_sequence_number)?,
            ),
            Side::Bid => (
                other.price_in_ticks.partial_cmp(&self.price_in_ticks)?,
                other
                    .order_sequence_number
                    .partial_cmp(&self.order_sequence_number)?,
            ),
        };

        if price_cmp == Ordering::Equal {
            Some(seq_cmp)
        } else {
            Some(price_cmp)
        }
    }
}

impl Ord for FIFOOrderId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct FIFORestingOrder {
    pub trader_index: u64,
    pub num_base_lots: BaseLots,
    pub last_valid_slot: u64,
    pub last_valid_unix_timestamp_in_seconds: u64,
}

impl FIFORestingOrder {
    pub fn new_default(trader_index: u64, num_base_lots: BaseLots) -> Self {
        Self {
            trader_index,
            num_base_lots,
            last_valid_slot: 0,
            last_valid_unix_timestamp_in_seconds: 0,
        }
    }

    pub fn new(
        trader_index: u64,
        num_base_lots: BaseLots,
        last_valid_slot: u64,
        last_valid_unix_timestamp_in_seconds: u64,
    ) -> Self {
        Self {
            trader_index,
            num_base_lots,
            last_valid_slot,
            last_valid_unix_timestamp_in_seconds,
        }
    }

    pub fn new_with_last_valid_slot(
        trader_index: u64,
        num_base_lots: BaseLots,
        last_valid_slot: u64,
    ) -> Self {
        Self {
            trader_index,
            num_base_lots,
            last_valid_slot,
            last_valid_unix_timestamp_in_seconds: 0,
        }
    }

    pub fn new_with_last_valid_unix_timestamp(
        trader_index: u64,
        num_base_lots: BaseLots,
        last_valid_unix_timestamp_in_seconds: u64,
    ) -> Self {
        FIFORestingOrder {
            trader_index,
            num_base_lots,
            last_valid_slot: 0,
            last_valid_unix_timestamp_in_seconds,
        }
    }
}

impl RestingOrder for FIFORestingOrder {
    fn size(&self) -> u64 {
        self.num_base_lots.as_u64()
    }

    fn is_expired(&self, current_slot: u64, current_timestamp: u64) -> bool {
        // Expired if slot limit exceeded
        (self.last_valid_slot != 0 && self.last_valid_slot < current_slot)
        ||  // OR
        // Expired if time limit exceeded
        (self.last_valid_unix_timestamp_in_seconds != 0 
         && self.last_valid_unix_timestamp_in_seconds < current_timestamp)
    }
    
    fn last_valid_slot(&self) -> Option<u64> {
        if self.last_valid_slot == 0 {
            None
        } else {
            Some(self.last_valid_slot)
        }
    }

    fn last_valid_unix_timestamp_in_seconds(&self) -> Option<u64> {
        if self.last_valid_unix_timestamp_in_seconds == 0 {
            None
        } else {
            Some(self.last_valid_unix_timestamp_in_seconds)
        }
    }
}
