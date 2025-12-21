use crate::quantities::{Ticks, WrapperU64};

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
