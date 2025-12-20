use crate::quantities::{BaseLots, QuoteLots};

/// Tracks a trader's state in the market
#[derive(Debug, Clone, Copy, Default)]
pub struct TraderState {
    /// Quote lots locked in open orders
    pub quote_lots_locked: QuoteLots,

    /// Quote lots free (not in orders)
    pub quote_lots_free: QuoteLots,

    /// Base lots locked in open orders
    pub base_lots_locked: BaseLots,

    /// Base lots free (not in orders)
    pub base_lots_free: BaseLots,
}
