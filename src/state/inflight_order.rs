use crate::quantities::{BaseLots, QuoteLots};
use crate::state::Side;
/// An order that is currently being processed
#[derive(Debug, Clone, Copy)]
pub struct InflightOrder {
    /// Which side: buy or sell
    pub side: Side,
    /// Base lots in this order
    pub num_base_lots: BaseLots,
    /// Quote lots in this order (for price calculation)
    pub num_quote_lots: QuoteLots,
}
