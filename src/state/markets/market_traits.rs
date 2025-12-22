pub trait RestingOrder {
    fn size(&self) -> u64;
    fn last_valid_slot(&self) -> Option<u64>;
    fn last_valid_unix_timestamp_in_seconds(&self) -> Option<u64>;
    fn is_expired(&self, current_slot: u64, current_timestamp: u64) -> bool;
}
