use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

pub trait WrapperU64 {
    fn new(value: u64) -> Self;
    fn as_u64(&self) -> u64;
}

macro_rules! basic_u64 {
    ($type_name:ident) => {
        impl WrapperU64 for $type_name {
            fn new(value: u64) -> Self {
                Self { inner: value }
            }

            fn as_u64(&self) -> u64 {
                self.inner
            }
        }

        impl $type_name {
            pub const ZERO: Self = $type_name { inner: 0 };
            pub const ONE: Self = $type_name { inner: 1 };
        }

        impl Add for $type_name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                $type_name::new(self.inner + other.inner)
            }
        }

        impl Sub for $type_name {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                $type_name::new(self.inner - other.inner)
            }
        }

        impl Display for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }
    };
}

macro_rules! allow_multiply {
    ($type_1:ident, $type_2:ident, $type_result:ident) => {
        // Type1 × Type2 = Result
        impl Mul<$type_2> for $type_1 {
            type Output = $type_result;
            fn mul(self, other: $type_2) -> $type_result {
                $type_result::new(self.inner * other.inner)
            }
        }

        // Type2 × Type1 = Result (reverse order)
        impl Mul<$type_1> for $type_2 {
            type Output = $type_result;
            fn mul(self, other: $type_1) -> $type_result {
                $type_result::new(self.inner * other.inner)
            }
        }
    };
}

// ============================================
// CORE QUANTITY TYPES
// ============================================

/// Quote lots - smallest unit of quote currency in the orderbook
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct QuoteLots {
    inner: u64,
}

/// Base lots - smallest unit of base currency in the orderbook
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BaseLots {
    inner: u64,
}

/// Ticks - discrete price unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Ticks {
    inner: u64,
}

// Apply the macro to generate all methods!
basic_u64!(QuoteLots);
basic_u64!(BaseLots);
basic_u64!(Ticks);

// ============================================
// CONVERSION FACTOR TYPES
// ============================================
/// Base atoms per base lot (conversion factor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BaseAtomsPerBaseLot {
    inner: u64,
}
/// Quote atoms per quote lot (conversion factor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct QuoteAtomsPerQuoteLot {
    inner: u64,
}
/// Base lots per base unit (conversion factor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BaseLotsPerBaseUnit {
    inner: u64,
}
// Apply macro
basic_u64!(BaseAtomsPerBaseLot);
basic_u64!(QuoteAtomsPerQuoteLot);
basic_u64!(BaseLotsPerBaseUnit);
// Define multiplication rules
allow_multiply!(BaseLots, BaseAtomsPerBaseLot, QuoteLots);

/// Adjusted quote lots (for fee calculations)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct AdjustedQuoteLots {
    inner: u64,
}
basic_u64!(AdjustedQuoteLots);
