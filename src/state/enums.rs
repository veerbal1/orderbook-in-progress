#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

impl Side {
    // Return the opposite side
    pub fn opposite(&self) -> Self {
        match self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        }
    }
}

/// What to do when a trader's order would match their own resting order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfTradeBehavior {
    /// Execute the self-trade (decrement the taking order)
    DecrementTake,

    /// Cancel the resting (maker) order
    CancelProvide,

    /// Abort the entire transaction
    AbortTransaction,
}
