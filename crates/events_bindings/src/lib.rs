pub use radroots_events as upstream;

pub const TYPES_TS: &str = include_str!("typescript/types.ts");
pub const CONSTANTS_TS: &str = include_str!("typescript/constants.ts");
pub const KINDS_TS: &str = include_str!("typescript/kinds.ts");

#[cfg(test)]
mod tests {
    use super::{CONSTANTS_TS, KINDS_TS, TYPES_TS};

    #[test]
    fn preserves_event_type_exports() {
        assert!(TYPES_TS.contains("export type RadrootsListing"));
        assert!(TYPES_TS.contains("export type RadrootsJobInput"));
        assert!(TYPES_TS.contains("export type RadrootsTradeOrderRequested"));
    }

    #[test]
    fn preserves_event_constant_exports() {
        assert!(CONSTANTS_TS.contains("RADROOTS_LISTING_PRODUCT_TAG_KEYS"));
        assert!(KINDS_TS.contains("KIND_LISTING"));
        assert!(KINDS_TS.contains("KIND_TRADE_LISTING_ORDER_REQ"));
    }
}
