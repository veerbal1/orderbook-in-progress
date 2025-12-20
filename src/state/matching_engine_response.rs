use crate::quantities::{BaseLots, QuoteLots};

/// Response from the matching engine after processing an order
#[derive(Debug, Clone, Copy, Default)]
pub struct MatchingEngineResponse {
    /// Number of base lots matched
    pub num_base_lots_matched: BaseLots,

    /// Number of quote lots matched
    pub num_quote_lots_matched: QuoteLots,

    /// Number of base lots posted to the book (not matched)
    pub num_base_lots_posted: BaseLots,

    /// Number of free quote lots to release (for failed orders)
    pub num_quote_lots_to_release: QuoteLots,

    /// Number of free base lots to release
    pub num_base_lots_to_release: BaseLots,
}
