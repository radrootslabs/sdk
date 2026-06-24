pub use radroots_events as upstream;

mod model;

pub use model::{constants_module, kinds_module};

#[cfg(test)]
mod tests {
    use super::{constants_module, kinds_module};

    #[test]
    fn preserves_event_constant_exports() {
        let constants = constants_module().render();
        let kinds = kinds_module().render();
        assert!(constants.contains("RADROOTS_LISTING_PRODUCT_TAG_KEYS"));
        assert!(kinds.contains("KIND_LISTING"));
        assert!(kinds.contains("KIND_TRADE_LISTING_ORDER_REQ"));
    }
}
