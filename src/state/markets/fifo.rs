use std::cmp::Ordering;

use crate::{
    quantities::{Ticks, WrapperU64},
    state::Side,
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
