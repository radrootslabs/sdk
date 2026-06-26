pub use radroots_events as upstream;

mod model;

pub use model::{constants_module, kinds_module};

#[cfg(test)]
mod tests {
    use radroots_events::{kinds, listing::RADROOTS_LISTING_PRODUCT_TAG_KEYS};

    use super::{constants_module, kinds_module};

    #[test]
    fn preserves_event_constant_exports() {
        let constants = constants_module();
        let kinds_ts = kinds_module();
        assert!(constants.contains("RADROOTS_LISTING_PRODUCT_TAG_KEYS"));
        assert!(constants.contains(RADROOTS_LISTING_PRODUCT_TAG_KEYS[0]));
        assert!(kinds_ts.contains("KIND_LISTING"));
        assert!(kinds_ts.contains(&kinds::KIND_LISTING.to_string()));
        assert!(kinds_ts.contains("KIND_ORDER_REQUEST"));
        assert!(kinds_ts.contains(&kinds::KIND_ORDER_REQUEST.to_string()));
    }
}
