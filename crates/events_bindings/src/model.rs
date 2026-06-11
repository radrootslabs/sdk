use radroots_sdk_binding_model::{self as ts, TsValue};

pub fn types_module() -> ts::TsModule {
    ts::module(vec![
        ts::type_alias(
            "JobFeedbackStatus",
            ts::union(vec![
                ts::string_literal("payment_required"),
                ts::string_literal("processing"),
                ts::string_literal("error"),
                ts::string_literal("success"),
                ts::string_literal("partial"),
            ]),
        ),
        ts::type_alias(
            "JobInputType",
            ts::union(vec![
                ts::string_literal("url"),
                ts::string_literal("event"),
                ts::string_literal("job"),
                ts::string_literal("text"),
            ]),
        ),
        ts::type_alias(
            "JobPaymentRequest",
            ts::object(vec![
                ts::field("amount_sat", ts::number()),
                ts::optional_field("bolt11", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsAccountClaim",
            ts::object(vec![
                ts::field("username", ts::string()),
                ts::field("pubkey", ts::string()),
                ts::optional_field("nip05", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias_params(
            "RadrootsActiveTradeEnvelope",
            &["T"],
            ts::object(vec![
                ts::field("version", ts::number()),
                ts::field("domain", ts::reference("RadrootsTradeDomain")),
                ts::field("type", ts::reference("RadrootsActiveTradeMessageType")),
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("payload", ts::reference("T")),
            ]),
        ),
        ts::type_alias(
            "RadrootsActiveTradeFulfillmentState",
            ts::union(vec![
                ts::string_literal("accepted_not_fulfilled"),
                ts::string_literal("preparing"),
                ts::string_literal("ready_for_pickup"),
                ts::string_literal("out_for_delivery"),
                ts::string_literal("delivered"),
                ts::string_literal("seller_cancelled"),
            ]),
        ),
        ts::type_alias(
            "RadrootsActiveTradeMessageType",
            ts::union(vec![
                ts::string_literal("TradeOrderRequested"),
                ts::string_literal("TradeOrderDecision"),
                ts::string_literal("TradeOrderRevisionProposed"),
                ts::string_literal("TradeOrderRevisionDecision"),
                ts::string_literal("TradeOrderCancelled"),
                ts::string_literal("TradeFulfillmentUpdated"),
                ts::string_literal("TradeBuyerReceipt"),
                ts::string_literal("TradePaymentRecorded"),
                ts::string_literal("TradeSettlementDecision"),
            ]),
        ),
        ts::type_alias(
            "RadrootsAppData",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("content", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsComment",
            ts::object(vec![
                ts::field("root", ts::reference("RadrootsNostrEventRef")),
                ts::field("parent", ts::reference("RadrootsNostrEventRef")),
                ts::field("content", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsCoop",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("name", ts::string()),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("website", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("picture", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("banner", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location",
                    ts::union(vec![ts::reference("RadrootsCoopLocation"), ts::null()]),
                ),
                ts::optional_field("tags", ts::union(vec![ts::array(ts::string()), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsCoopLocation",
            ts::object(vec![
                ts::optional_field("primary", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("city", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("region", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("country", ts::union(vec![ts::string(), ts::null()])),
                ts::field("gcs", ts::reference("RadrootsGcsLocation")),
            ]),
        ),
        ts::type_alias(
            "RadrootsCoopRef",
            ts::object(vec![
                ts::field("pubkey", ts::string()),
                ts::field("d_tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsDocument",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("doc_type", ts::string()),
                ts::field("title", ts::string()),
                ts::field("version", ts::string()),
                ts::optional_field("summary", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("effective_at", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("body_markdown", ts::union(vec![ts::string(), ts::null()])),
                ts::field("subject", ts::reference("RadrootsDocumentSubject")),
                ts::optional_field("tags", ts::union(vec![ts::array(ts::string()), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsDocumentSubject",
            ts::object(vec![
                ts::field("pubkey", ts::string()),
                ts::optional_field("address", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsFarm",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("name", ts::string()),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("website", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("picture", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("banner", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location",
                    ts::union(vec![ts::reference("RadrootsFarmLocation"), ts::null()]),
                ),
                ts::optional_field("tags", ts::union(vec![ts::array(ts::string()), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsFarmLocation",
            ts::object(vec![
                ts::optional_field("primary", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("city", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("region", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("country", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "gcs",
                    ts::union(vec![ts::reference("RadrootsGcsLocation"), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsFarmRef",
            ts::object(vec![
                ts::field("pubkey", ts::string()),
                ts::field("d_tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsFollow",
            ts::object(vec![ts::field(
                "list",
                ts::array(ts::reference("RadrootsFollowProfile")),
            )]),
        ),
        ts::type_alias(
            "RadrootsFollowProfile",
            ts::object(vec![
                ts::field("published_at", ts::number()),
                ts::field("public_key", ts::string()),
                ts::optional_field("relay_url", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("contact_name", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsGcsLocation",
            ts::object(vec![
                ts::field("lat", ts::number()),
                ts::field("lng", ts::number()),
                ts::field("geohash", ts::string()),
                ts::field("point", ts::reference("RadrootsGeoJsonPoint")),
                ts::field("polygon", ts::reference("RadrootsGeoJsonPolygon")),
                ts::optional_field("accuracy", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("altitude", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("tag_0", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("label", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("area", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("elevation", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("soil", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("climate", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_admin1_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_admin1_name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_country_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_country_name", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsGeoChat",
            ts::object(vec![
                ts::field("geohash", ts::string()),
                ts::field("content", ts::string()),
                ts::optional_field("nickname", ts::union(vec![ts::string(), ts::null()])),
                ts::field("teleported", ts::boolean()),
            ]),
        ),
        ts::type_alias(
            "RadrootsGeoJsonPoint",
            ts::object(vec![
                ts::field("type", ts::string()),
                ts::field("coordinates", ts::tuple(vec![ts::number(), ts::number()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsGeoJsonPolygon",
            ts::object(vec![
                ts::field("type", ts::string()),
                ts::field(
                    "coordinates",
                    ts::array(ts::array(ts::tuple(vec![ts::number(), ts::number()]))),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsGiftWrap",
            ts::object(vec![
                ts::field("recipient", ts::reference("RadrootsGiftWrapRecipient")),
                ts::field("content", ts::string()),
                ts::optional_field("expiration", ts::union(vec![ts::number(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsGiftWrapRecipient",
            ts::object(vec![
                ts::field("public_key", ts::string()),
                ts::optional_field("relay_url", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsJobFeedback",
            ts::object(vec![
                ts::field("kind", ts::number()),
                ts::field("status", ts::reference("JobFeedbackStatus")),
                ts::optional_field("extra_info", ts::union(vec![ts::string(), ts::null()])),
                ts::field("request_event", ts::reference("RadrootsNostrEventPtr")),
                ts::optional_field("customer_pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "payment",
                    ts::union(vec![ts::reference("JobPaymentRequest"), ts::null()]),
                ),
                ts::optional_field("content", ts::union(vec![ts::string(), ts::null()])),
                ts::field("encrypted", ts::boolean()),
            ]),
        ),
        ts::type_alias(
            "RadrootsJobInput",
            ts::object(vec![
                ts::field("data", ts::string()),
                ts::field("input_type", ts::reference("JobInputType")),
                ts::optional_field("relay", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("marker", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsJobParam",
            ts::object(vec![
                ts::field("key", ts::string()),
                ts::field("value", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsJobRequest",
            ts::object(vec![
                ts::field("kind", ts::number()),
                ts::field("inputs", ts::array(ts::reference("RadrootsJobInput"))),
                ts::optional_field("output", ts::union(vec![ts::string(), ts::null()])),
                ts::field("params", ts::array(ts::reference("RadrootsJobParam"))),
                ts::optional_field("bid_sat", ts::union(vec![ts::number(), ts::null()])),
                ts::field("relays", ts::array(ts::string())),
                ts::field("providers", ts::array(ts::string())),
                ts::field("topics", ts::array(ts::string())),
                ts::field("encrypted", ts::boolean()),
            ]),
        ),
        ts::type_alias(
            "RadrootsJobResult",
            ts::object(vec![
                ts::field("kind", ts::number()),
                ts::field("request_event", ts::reference("RadrootsNostrEventPtr")),
                ts::optional_field("request_json", ts::union(vec![ts::string(), ts::null()])),
                ts::field("inputs", ts::array(ts::reference("RadrootsJobInput"))),
                ts::optional_field("customer_pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "payment",
                    ts::union(vec![ts::reference("JobPaymentRequest"), ts::null()]),
                ),
                ts::optional_field("content", ts::union(vec![ts::string(), ts::null()])),
                ts::field("encrypted", ts::boolean()),
            ]),
        ),
        ts::type_alias(
            "RadrootsList",
            ts::object(vec![
                ts::field("content", ts::string()),
                ts::field("entries", ts::array(ts::reference("RadrootsListEntry"))),
            ]),
        ),
        ts::type_alias(
            "RadrootsListEntry",
            ts::object(vec![
                ts::field("tag", ts::string()),
                ts::field("values", ts::array(ts::string())),
            ]),
        ),
        ts::type_alias(
            "RadrootsListSet",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("content", ts::string()),
                ts::field("entries", ts::array(ts::reference("RadrootsListEntry"))),
                ts::optional_field("title", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("description", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("image", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsListing",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("farm", ts::reference("RadrootsFarmRef")),
                ts::field("product", ts::reference("RadrootsListingProduct")),
                ts::field("primary_bin_id", ts::string()),
                ts::field("bins", ts::array(ts::reference("RadrootsListingBin"))),
                ts::optional_field(
                    "resource_area",
                    ts::union(vec![ts::reference("RadrootsResourceAreaRef"), ts::null()]),
                ),
                ts::optional_field(
                    "plot",
                    ts::union(vec![ts::reference("RadrootsPlotRef"), ts::null()]),
                ),
                ts::optional_field(
                    "discounts",
                    ts::union(vec![
                        ts::array(ts::reference("RadrootsCoreDiscount")),
                        ts::null(),
                    ]),
                ),
                ts::optional_field(
                    "inventory_available",
                    ts::union(vec![ts::reference("RadrootsCoreDecimal"), ts::null()]),
                ),
                ts::optional_field(
                    "availability",
                    ts::union(vec![
                        ts::reference("RadrootsListingAvailability"),
                        ts::null(),
                    ]),
                ),
                ts::optional_field(
                    "delivery_method",
                    ts::union(vec![
                        ts::reference("RadrootsListingDeliveryMethod"),
                        ts::null(),
                    ]),
                ),
                ts::optional_field(
                    "location",
                    ts::union(vec![ts::reference("RadrootsListingLocation"), ts::null()]),
                ),
                ts::optional_field(
                    "images",
                    ts::union(vec![
                        ts::array(ts::reference("RadrootsListingImage")),
                        ts::null(),
                    ]),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsListingAvailability",
            ts::union(vec![
                ts::object(vec![
                    ts::field("kind", ts::string_literal("window")),
                    ts::field(
                        "amount",
                        ts::object(vec![
                            ts::optional_field("start", ts::union(vec![ts::number(), ts::null()])),
                            ts::optional_field("end", ts::union(vec![ts::number(), ts::null()])),
                        ]),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("status")),
                    ts::field(
                        "amount",
                        ts::object(vec![ts::field(
                            "status",
                            ts::reference("RadrootsListingStatus"),
                        )]),
                    ),
                ]),
            ]),
        ),
        ts::type_alias(
            "RadrootsListingBin",
            ts::object(vec![
                ts::field("bin_id", ts::string()),
                ts::field("quantity", ts::reference("RadrootsCoreQuantity")),
                ts::field(
                    "price_per_canonical_unit",
                    ts::reference("RadrootsCoreQuantityPrice"),
                ),
                ts::optional_field(
                    "display_amount",
                    ts::union(vec![ts::reference("RadrootsCoreDecimal"), ts::null()]),
                ),
                ts::optional_field(
                    "display_unit",
                    ts::union(vec![ts::reference("RadrootsCoreUnit"), ts::null()]),
                ),
                ts::optional_field("display_label", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "display_price",
                    ts::union(vec![ts::reference("RadrootsCoreMoney"), ts::null()]),
                ),
                ts::optional_field(
                    "display_price_unit",
                    ts::union(vec![ts::reference("RadrootsCoreUnit"), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsListingDeliveryMethod",
            ts::union(vec![
                ts::object(vec![ts::field("kind", ts::string_literal("pickup"))]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("local_delivery"),
                )]),
                ts::object(vec![ts::field("kind", ts::string_literal("shipping"))]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("other")),
                    ts::field(
                        "amount",
                        ts::object(vec![ts::field("method", ts::string())]),
                    ),
                ]),
            ]),
        ),
        ts::type_alias(
            "RadrootsListingImage",
            ts::object(vec![
                ts::field("url", ts::string()),
                ts::optional_field(
                    "size",
                    ts::union(vec![ts::reference("RadrootsListingImageSize"), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsListingImageSize",
            ts::object(vec![
                ts::field("w", ts::number()),
                ts::field("h", ts::number()),
            ]),
        ),
        ts::type_alias(
            "RadrootsListingLocation",
            ts::object(vec![
                ts::field("primary", ts::string()),
                ts::optional_field("city", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("region", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("country", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lat", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("lng", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("geohash", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsListingProduct",
            ts::object(vec![
                ts::field("key", ts::string()),
                ts::field("title", ts::string()),
                ts::field("category", ts::string()),
                ts::optional_field("summary", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("process", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lot", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("location", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("profile", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("year", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsListingProductTagKeys",
            ts::readonly_tuple(vec![
                ts::string_literal("key"),
                ts::string_literal("title"),
                ts::string_literal("category"),
                ts::string_literal("summary"),
                ts::string_literal("process"),
                ts::string_literal("lot"),
                ts::string_literal("location"),
                ts::string_literal("profile"),
                ts::string_literal("year"),
            ]),
        ),
        ts::type_alias(
            "RadrootsListingStatus",
            ts::union(vec![
                ts::object(vec![ts::field("kind", ts::string_literal("active"))]),
                ts::object(vec![ts::field("kind", ts::string_literal("sold"))]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("other")),
                    ts::field("amount", ts::object(vec![ts::field("value", ts::string())])),
                ]),
            ]),
        ),
        ts::type_alias(
            "RadrootsMessage",
            ts::object(vec![
                ts::field(
                    "recipients",
                    ts::array(ts::reference("RadrootsMessageRecipient")),
                ),
                ts::field("content", ts::string()),
                ts::optional_field(
                    "reply_to",
                    ts::union(vec![ts::reference("RadrootsNostrEventPtr"), ts::null()]),
                ),
                ts::optional_field("subject", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsMessageFile",
            ts::object(vec![
                ts::field(
                    "recipients",
                    ts::array(ts::reference("RadrootsMessageRecipient")),
                ),
                ts::field("file_url", ts::string()),
                ts::optional_field(
                    "reply_to",
                    ts::union(vec![ts::reference("RadrootsNostrEventPtr"), ts::null()]),
                ),
                ts::optional_field("subject", ts::union(vec![ts::string(), ts::null()])),
                ts::field("file_type", ts::string()),
                ts::field("encryption_algorithm", ts::string()),
                ts::field("decryption_key", ts::string()),
                ts::field("decryption_nonce", ts::string()),
                ts::field("encrypted_hash", ts::string()),
                ts::optional_field("original_hash", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("size", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field(
                    "dimensions",
                    ts::union(vec![
                        ts::reference("RadrootsMessageFileDimensions"),
                        ts::null(),
                    ]),
                ),
                ts::optional_field("blurhash", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("thumb", ts::union(vec![ts::string(), ts::null()])),
                ts::field("fallbacks", ts::array(ts::string())),
            ]),
        ),
        ts::type_alias(
            "RadrootsMessageFileDimensions",
            ts::object(vec![
                ts::field("w", ts::number()),
                ts::field("h", ts::number()),
            ]),
        ),
        ts::type_alias(
            "RadrootsMessageRecipient",
            ts::object(vec![
                ts::field("public_key", ts::string()),
                ts::optional_field("relay_url", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsNostrEvent",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("author", ts::string()),
                ts::field("created_at", ts::number()),
                ts::field("kind", ts::number()),
                ts::field("tags", ts::array(ts::array(ts::string()))),
                ts::field("content", ts::string()),
                ts::field("sig", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsNostrEventPtr",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::optional_field("relays", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsNostrEventRef",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("author", ts::string()),
                ts::field("kind", ts::number()),
                ts::optional_field("d_tag", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "relays",
                    ts::union(vec![ts::array(ts::string()), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsPlot",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("farm", ts::reference("RadrootsFarmRef")),
                ts::field("name", ts::string()),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location",
                    ts::union(vec![ts::reference("RadrootsPlotLocation"), ts::null()]),
                ),
                ts::optional_field("tags", ts::union(vec![ts::array(ts::string()), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsPlotLocation",
            ts::object(vec![
                ts::optional_field("primary", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("city", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("region", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("country", ts::union(vec![ts::string(), ts::null()])),
                ts::field("gcs", ts::reference("RadrootsGcsLocation")),
            ]),
        ),
        ts::type_alias(
            "RadrootsPlotRef",
            ts::object(vec![
                ts::field("pubkey", ts::string()),
                ts::field("d_tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsPost",
            ts::object(vec![ts::field("content", ts::string())]),
        ),
        ts::type_alias(
            "RadrootsProfile",
            ts::object(vec![
                ts::field("name", ts::string()),
                ts::optional_field("display_name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("nip05", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("website", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("picture", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("banner", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lud06", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lud16", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("bot", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsProfileType",
            ts::union(vec![
                ts::string_literal("individual"),
                ts::string_literal("farm"),
                ts::string_literal("coop"),
                ts::string_literal("any"),
                ts::string_literal("radrootsd"),
            ]),
        ),
        ts::type_alias(
            "RadrootsReaction",
            ts::object(vec![
                ts::field("root", ts::reference("RadrootsNostrEventRef")),
                ts::field("content", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsRelayDocument",
            ts::object(vec![
                ts::optional_field("name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("description", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("contact", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "supported_nips",
                    ts::union(vec![ts::array(ts::number()), ts::null()]),
                ),
                ts::optional_field("software", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("version", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsResourceArea",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("name", ts::string()),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::field("location", ts::reference("RadrootsResourceAreaLocation")),
                ts::optional_field("tags", ts::union(vec![ts::array(ts::string()), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsResourceAreaLocation",
            ts::object(vec![
                ts::optional_field("primary", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("city", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("region", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("country", ts::union(vec![ts::string(), ts::null()])),
                ts::field("gcs", ts::reference("RadrootsGcsLocation")),
            ]),
        ),
        ts::type_alias(
            "RadrootsResourceAreaRef",
            ts::object(vec![
                ts::field("pubkey", ts::string()),
                ts::field("d_tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsResourceHarvestCap",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("resource_area", ts::reference("RadrootsResourceAreaRef")),
                ts::field("product", ts::reference("RadrootsResourceHarvestProduct")),
                ts::field("start", ts::bigint()),
                ts::field("end", ts::bigint()),
                ts::field("cap_quantity", ts::reference("RadrootsCoreQuantity")),
                ts::optional_field(
                    "display_amount",
                    ts::union(vec![ts::reference("RadrootsCoreDecimal"), ts::null()]),
                ),
                ts::optional_field(
                    "display_unit",
                    ts::union(vec![ts::reference("RadrootsCoreUnit"), ts::null()]),
                ),
                ts::optional_field("display_label", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("tags", ts::union(vec![ts::array(ts::string()), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsResourceHarvestProduct",
            ts::object(vec![
                ts::field("key", ts::string()),
                ts::optional_field("category", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsSeal",
            ts::object(vec![ts::field("content", ts::string())]),
        ),
        ts::type_alias(
            "RadrootsTradeAnswer",
            ts::object(vec![ts::field("question_id", ts::string())]),
        ),
        ts::type_alias(
            "RadrootsTradeBuyerReceipt",
            ts::object(vec![
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("buyer_pubkey", ts::string()),
                ts::field("seller_pubkey", ts::string()),
                ts::field("received", ts::boolean()),
                ts::optional_field("issue", ts::union(vec![ts::string(), ts::null()])),
                ts::field("received_at", ts::bigint()),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeDiscountDecision",
            ts::union(vec![
                ts::object(vec![
                    ts::field("kind", ts::string_literal("accept")),
                    ts::field(
                        "amount",
                        ts::object(vec![ts::field(
                            "value",
                            ts::reference("RadrootsCoreDiscountValue"),
                        )]),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("decline")),
                    ts::field(
                        "amount",
                        ts::object(vec![ts::optional_field(
                            "reason",
                            ts::union(vec![ts::string(), ts::null()]),
                        )]),
                    ),
                ]),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeDiscountOffer",
            ts::object(vec![
                ts::field("discount_id", ts::string()),
                ts::field("value", ts::reference("RadrootsCoreDiscountValue")),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeDiscountRequest",
            ts::object(vec![
                ts::field("discount_id", ts::string()),
                ts::field("value", ts::reference("RadrootsCoreDiscountValue")),
            ]),
        ),
        ts::type_alias("RadrootsTradeDomain", ts::string_literal("trade:listing")),
        ts::type_alias(
            "RadrootsTradeEconomicActor",
            ts::union(vec![
                ts::string_literal("buyer"),
                ts::string_literal("seller"),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeEconomicEffect",
            ts::union(vec![
                ts::string_literal("increase"),
                ts::string_literal("decrease"),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeEconomicLineKind",
            ts::union(vec![
                ts::string_literal("listing_discount"),
                ts::string_literal("basket_adjustment"),
                ts::string_literal("revision_adjustment"),
            ]),
        ),
        ts::type_alias_params(
            "RadrootsTradeEnvelope",
            &["T"],
            ts::object(vec![
                ts::field("version", ts::number()),
                ts::field("domain", ts::reference("RadrootsTradeDomain")),
                ts::field("type", ts::reference("RadrootsTradeMessageType")),
                ts::optional_field("order_id", ts::union(vec![ts::string(), ts::null()])),
                ts::field("listing_addr", ts::string()),
                ts::field("payload", ts::reference("T")),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeFulfillmentStatus",
            ts::union(vec![
                ts::object(vec![ts::field("kind", ts::string_literal("preparing"))]),
                ts::object(vec![ts::field("kind", ts::string_literal("shipped"))]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("ready_for_pickup"),
                )]),
                ts::object(vec![ts::field("kind", ts::string_literal("delivered"))]),
                ts::object(vec![ts::field("kind", ts::string_literal("cancelled"))]),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeFulfillmentUpdate",
            ts::object(vec![ts::field(
                "status",
                ts::reference("RadrootsTradeFulfillmentStatus"),
            )]),
        ),
        ts::type_alias(
            "RadrootsTradeFulfillmentUpdated",
            ts::object(vec![
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("buyer_pubkey", ts::string()),
                ts::field("seller_pubkey", ts::string()),
                ts::field(
                    "status",
                    ts::reference("RadrootsActiveTradeFulfillmentState"),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeInventoryCommitment",
            ts::object(vec![
                ts::field("bin_id", ts::string()),
                ts::field("bin_count", ts::number()),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeListingCancel",
            ts::object(vec![ts::optional_field(
                "reason",
                ts::union(vec![ts::string(), ts::null()]),
            )]),
        ),
        ts::type_alias(
            "RadrootsTradeListingParseError",
            ts::union(vec![
                ts::object(vec![ts::field("InvalidKind", ts::number())]),
                ts::object(vec![ts::field("MissingTag", ts::string())]),
                ts::object(vec![ts::field("InvalidTag", ts::string())]),
                ts::object(vec![ts::field("InvalidNumber", ts::string())]),
                ts::string_literal("InvalidUnit"),
                ts::string_literal("InvalidCurrency"),
                ts::object(vec![ts::field("InvalidJson", ts::string())]),
                ts::object(vec![ts::field("InvalidDiscount", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeListingValidateRequest",
            ts::object(vec![ts::optional_field(
                "listing_event",
                ts::union(vec![ts::reference("RadrootsNostrEventPtr"), ts::null()]),
            )]),
        ),
        ts::type_alias(
            "RadrootsTradeListingValidateResult",
            ts::object(vec![
                ts::field("valid", ts::boolean()),
                ts::field(
                    "errors",
                    ts::array(ts::reference("RadrootsTradeListingValidationError")),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeListingValidationError",
            ts::union(vec![
                ts::object(vec![
                    ts::field("kind", ts::string_literal("invalid_kind")),
                    ts::field("amount", ts::object(vec![ts::field("kind", ts::number())])),
                ]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_listing_id"),
                )]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("listing_event_not_found")),
                    ts::field(
                        "amount",
                        ts::object(vec![ts::field("listing_addr", ts::string())]),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("listing_event_fetch_failed")),
                    ts::field(
                        "amount",
                        ts::object(vec![ts::field("listing_addr", ts::string())]),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("parse_error")),
                    ts::field(
                        "amount",
                        ts::object(vec![ts::field(
                            "error",
                            ts::reference("RadrootsTradeListingParseError"),
                        )]),
                    ),
                ]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("invalid_seller"),
                )]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_farm_profile"),
                )]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_farm_record"),
                )]),
                ts::object(vec![ts::field("kind", ts::string_literal("missing_title"))]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_description"),
                )]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_product_type"),
                )]),
                ts::object(vec![ts::field("kind", ts::string_literal("missing_bins"))]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_primary_bin"),
                )]),
                ts::object(vec![ts::field("kind", ts::string_literal("invalid_bin"))]),
                ts::object(vec![ts::field("kind", ts::string_literal("missing_price"))]),
                ts::object(vec![ts::field("kind", ts::string_literal("invalid_price"))]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_inventory"),
                )]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("invalid_inventory"),
                )]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_availability"),
                )]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_location"),
                )]),
                ts::object(vec![ts::field(
                    "kind",
                    ts::string_literal("missing_delivery_method"),
                )]),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeMessagePayload",
            ts::union(vec![
                ts::object(vec![
                    ts::field("kind", ts::string_literal("listing_validate_request")),
                    ts::field(
                        "amount",
                        ts::reference("RadrootsTradeListingValidateRequest"),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("listing_validate_result")),
                    ts::field(
                        "amount",
                        ts::reference("RadrootsTradeListingValidateResult"),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("trade_order_requested")),
                    ts::field("amount", ts::reference("RadrootsTradeOrderRequested")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("order_response")),
                    ts::field("amount", ts::reference("RadrootsTradeOrderResponse")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("order_revision")),
                    ts::field("amount", ts::reference("RadrootsTradeOrderRevision")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("order_revision_accept")),
                    ts::field(
                        "amount",
                        ts::reference("RadrootsTradeOrderRevisionResponse"),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("order_revision_decline")),
                    ts::field(
                        "amount",
                        ts::reference("RadrootsTradeOrderRevisionResponse"),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("question")),
                    ts::field("amount", ts::reference("RadrootsTradeQuestion")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("answer")),
                    ts::field("amount", ts::reference("RadrootsTradeAnswer")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("discount_request")),
                    ts::field("amount", ts::reference("RadrootsTradeDiscountRequest")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("discount_offer")),
                    ts::field("amount", ts::reference("RadrootsTradeDiscountOffer")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("discount_accept")),
                    ts::field("amount", ts::reference("RadrootsTradeDiscountDecision")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("discount_decline")),
                    ts::field("amount", ts::reference("RadrootsTradeDiscountDecision")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("cancel")),
                    ts::field("amount", ts::reference("RadrootsTradeListingCancel")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("fulfillment_update")),
                    ts::field("amount", ts::reference("RadrootsTradeFulfillmentUpdate")),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("receipt")),
                    ts::field("amount", ts::reference("RadrootsTradeReceipt")),
                ]),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeMessageType",
            ts::union(vec![
                ts::string_literal("listing_validate_request"),
                ts::string_literal("listing_validate_result"),
                ts::string_literal("order_request"),
                ts::string_literal("order_response"),
                ts::string_literal("order_revision"),
                ts::string_literal("order_revision_accept"),
                ts::string_literal("order_revision_decline"),
                ts::string_literal("question"),
                ts::string_literal("answer"),
                ts::string_literal("discount_request"),
                ts::string_literal("discount_offer"),
                ts::string_literal("discount_accept"),
                ts::string_literal("discount_decline"),
                ts::string_literal("cancel"),
                ts::string_literal("fulfillment_update"),
                ts::string_literal("receipt"),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderCancelled",
            ts::object(vec![
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("buyer_pubkey", ts::string()),
                ts::field("seller_pubkey", ts::string()),
                ts::field("reason", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderChange",
            ts::union(vec![
                ts::object(vec![
                    ts::field("kind", ts::string_literal("bin_count")),
                    ts::field(
                        "amount",
                        ts::object(vec![
                            ts::field("item_index", ts::number()),
                            ts::field("bin_count", ts::number()),
                        ]),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("item_add")),
                    ts::field(
                        "amount",
                        ts::object(vec![ts::field(
                            "item",
                            ts::reference("RadrootsTradeOrderItem"),
                        )]),
                    ),
                ]),
                ts::object(vec![
                    ts::field("kind", ts::string_literal("item_remove")),
                    ts::field(
                        "amount",
                        ts::object(vec![ts::field("item_index", ts::number())]),
                    ),
                ]),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderDecision",
            ts::union(vec![
                ts::object(vec![
                    ts::field("decision", ts::string_literal("accepted")),
                    ts::field(
                        "inventory_commitments",
                        ts::array(ts::reference("RadrootsTradeInventoryCommitment")),
                    ),
                ]),
                ts::object(vec![
                    ts::field("decision", ts::string_literal("declined")),
                    ts::field("reason", ts::string()),
                ]),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderDecisionEvent",
            ts::object(vec![
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("buyer_pubkey", ts::string()),
                ts::field("seller_pubkey", ts::string()),
                ts::field("decision", ts::reference("RadrootsTradeOrderDecision")),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderEconomicItem",
            ts::object(vec![
                ts::field("bin_id", ts::string()),
                ts::field("bin_count", ts::number()),
                ts::field("quantity_amount", ts::reference("RadrootsCoreDecimal")),
                ts::field("quantity_unit", ts::reference("RadrootsCoreUnit")),
                ts::field("unit_price_amount", ts::reference("RadrootsCoreDecimal")),
                ts::field("unit_price_currency", ts::reference("RadrootsCoreCurrency")),
                ts::field("line_subtotal", ts::reference("RadrootsCoreMoney")),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderEconomicLine",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("kind", ts::reference("RadrootsTradeEconomicLineKind")),
                ts::field("actor", ts::reference("RadrootsTradeEconomicActor")),
                ts::field("effect", ts::reference("RadrootsTradeEconomicEffect")),
                ts::field("amount", ts::reference("RadrootsCoreMoney")),
                ts::field("reason", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderEconomicTotals",
            ts::object(vec![
                ts::field("subtotal", ts::reference("RadrootsCoreMoney")),
                ts::field("discount_total", ts::reference("RadrootsCoreMoney")),
                ts::field("adjustment_total", ts::reference("RadrootsCoreMoney")),
                ts::field("total", ts::reference("RadrootsCoreMoney")),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderEconomics",
            ts::object(vec![
                ts::field("quote_id", ts::string()),
                ts::field("quote_version", ts::number()),
                ts::field("pricing_basis", ts::reference("RadrootsTradePricingBasis")),
                ts::field("currency", ts::reference("RadrootsCoreCurrency")),
                ts::field(
                    "items",
                    ts::array(ts::reference("RadrootsTradeOrderEconomicItem")),
                ),
                ts::field(
                    "discounts",
                    ts::array(ts::reference("RadrootsTradeOrderEconomicLine")),
                ),
                ts::field(
                    "adjustments",
                    ts::array(ts::reference("RadrootsTradeOrderEconomicLine")),
                ),
                ts::field("subtotal", ts::reference("RadrootsCoreMoney")),
                ts::field("discount_total", ts::reference("RadrootsCoreMoney")),
                ts::field("adjustment_total", ts::reference("RadrootsCoreMoney")),
                ts::field("total", ts::reference("RadrootsCoreMoney")),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderItem",
            ts::object(vec![
                ts::field("bin_id", ts::string()),
                ts::field("bin_count", ts::number()),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderRequested",
            ts::object(vec![
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("buyer_pubkey", ts::string()),
                ts::field("seller_pubkey", ts::string()),
                ts::field("items", ts::array(ts::reference("RadrootsTradeOrderItem"))),
                ts::field("economics", ts::reference("RadrootsTradeOrderEconomics")),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderResponse",
            ts::object(vec![
                ts::field("accepted", ts::boolean()),
                ts::optional_field("reason", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderRevision",
            ts::object(vec![
                ts::field("revision_id", ts::string()),
                ts::field(
                    "changes",
                    ts::array(ts::reference("RadrootsTradeOrderChange")),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderRevisionDecision",
            ts::union(vec![
                ts::object(vec![ts::field("decision", ts::string_literal("accepted"))]),
                ts::object(vec![
                    ts::field("decision", ts::string_literal("declined")),
                    ts::field("reason", ts::string()),
                ]),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderRevisionDecisionEvent",
            ts::object(vec![
                ts::field("revision_id", ts::string()),
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("buyer_pubkey", ts::string()),
                ts::field("seller_pubkey", ts::string()),
                ts::field("root_event_id", ts::string()),
                ts::field("prev_event_id", ts::string()),
                ts::field(
                    "decision",
                    ts::reference("RadrootsTradeOrderRevisionDecision"),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderRevisionProposed",
            ts::object(vec![
                ts::field("revision_id", ts::string()),
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("buyer_pubkey", ts::string()),
                ts::field("seller_pubkey", ts::string()),
                ts::field("root_event_id", ts::string()),
                ts::field("prev_event_id", ts::string()),
                ts::field("items", ts::array(ts::reference("RadrootsTradeOrderItem"))),
                ts::field("economics", ts::reference("RadrootsTradeOrderEconomics")),
                ts::field("reason", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderRevisionResponse",
            ts::object(vec![
                ts::field("accepted", ts::boolean()),
                ts::optional_field("reason", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeOrderStatus",
            ts::union(vec![
                ts::string_literal("draft"),
                ts::string_literal("validated"),
                ts::string_literal("requested"),
                ts::string_literal("questioned"),
                ts::string_literal("revised"),
                ts::string_literal("accepted"),
                ts::string_literal("declined"),
                ts::string_literal("cancelled"),
                ts::string_literal("fulfilled"),
                ts::string_literal("completed"),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradePaymentMethod",
            ts::union(vec![
                ts::string_literal("cash"),
                ts::string_literal("manual_transfer"),
                ts::string_literal("other"),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradePaymentRecorded",
            ts::object(vec![
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("buyer_pubkey", ts::string()),
                ts::field("seller_pubkey", ts::string()),
                ts::field("root_event_id", ts::string()),
                ts::field("previous_event_id", ts::string()),
                ts::field("agreement_event_id", ts::string()),
                ts::field("quote_id", ts::string()),
                ts::field("quote_version", ts::number()),
                ts::field("economics_digest", ts::string()),
                ts::field("amount", ts::reference("RadrootsCoreDecimal")),
                ts::field("currency", ts::reference("RadrootsCoreCurrency")),
                ts::field("method", ts::reference("RadrootsTradePaymentMethod")),
                ts::optional_field("reference", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("paid_at", ts::union(vec![ts::number(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradePricingBasis",
            ts::string_literal("listing_event"),
        ),
        ts::type_alias(
            "RadrootsTradeQuestion",
            ts::object(vec![ts::field("question_id", ts::string())]),
        ),
        ts::type_alias(
            "RadrootsTradeReceipt",
            ts::object(vec![
                ts::field("acknowledged", ts::boolean()),
                ts::field("at", ts::bigint()),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeSettlementDecision",
            ts::union(vec![
                ts::string_literal("accepted"),
                ts::string_literal("rejected"),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeSettlementDecisionEvent",
            ts::object(vec![
                ts::field("order_id", ts::string()),
                ts::field("listing_addr", ts::string()),
                ts::field("seller_pubkey", ts::string()),
                ts::field("buyer_pubkey", ts::string()),
                ts::field("root_event_id", ts::string()),
                ts::field("previous_event_id", ts::string()),
                ts::field("agreement_event_id", ts::string()),
                ts::field("payment_event_id", ts::string()),
                ts::field("quote_id", ts::string()),
                ts::field("quote_version", ts::number()),
                ts::field("economics_digest", ts::string()),
                ts::field("amount", ts::reference("RadrootsCoreDecimal")),
                ts::field("currency", ts::reference("RadrootsCoreCurrency")),
                ts::field("decision", ts::reference("RadrootsTradeSettlementDecision")),
                ts::optional_field("reason", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "RadrootsTradeTransportLane",
            ts::union(vec![
                ts::string_literal("service"),
                ts::string_literal("public"),
            ]),
        ),
    ])
}

pub fn constants_module() -> ts::TsModule {
    ts::module(vec![
        ts::import_type(&["RadrootsListingProductTagKeys"], "./types.js"),
        ts::const_decl(
            "RADROOTS_LISTING_PRODUCT_TAG_KEYS",
            Some(ts::reference("RadrootsListingProductTagKeys")),
            TsValue::Array(vec![
                TsValue::String("key".to_owned()),
                TsValue::String("title".to_owned()),
                TsValue::String("category".to_owned()),
                TsValue::String("summary".to_owned()),
                TsValue::String("process".to_owned()),
                TsValue::String("lot".to_owned()),
                TsValue::String("location".to_owned()),
                TsValue::String("profile".to_owned()),
                TsValue::String("year".to_owned()),
            ]),
        ),
    ])
}

pub fn kinds_module() -> ts::TsModule {
    ts::module(vec![
        ts::const_number("KIND_PROFILE", 0),
        ts::const_number("KIND_POST", 1),
        ts::const_number("KIND_FOLLOW", 3),
        ts::const_number("KIND_REACTION", 7),
        ts::const_number("KIND_SEAL", 13),
        ts::const_number("KIND_MESSAGE", 14),
        ts::const_number("KIND_MESSAGE_FILE", 15),
        ts::const_number("KIND_GIFT_WRAP", 1059),
        ts::const_number("KIND_COMMENT", 1111),
        ts::const_number("KIND_GEOCHAT", 20000),
        ts::const_number("KIND_LIST_MUTE", 10000),
        ts::const_number("KIND_LIST_PINNED_NOTES", 10001),
        ts::const_number("KIND_LIST_READ_WRITE_RELAYS", 10002),
        ts::const_number("KIND_LIST_BOOKMARKS", 10003),
        ts::const_number("KIND_LIST_COMMUNITIES", 10004),
        ts::const_number("KIND_LIST_PUBLIC_CHATS", 10005),
        ts::const_number("KIND_LIST_BLOCKED_RELAYS", 10006),
        ts::const_number("KIND_LIST_SEARCH_RELAYS", 10007),
        ts::const_number("KIND_LIST_SIMPLE_GROUPS", 10009),
        ts::const_number("KIND_LIST_RELAY_FEEDS", 10012),
        ts::const_number("KIND_LIST_INTERESTS", 10015),
        ts::const_number("KIND_LIST_MEDIA_FOLLOWS", 10020),
        ts::const_number("KIND_LIST_EMOJIS", 10030),
        ts::const_number("KIND_LIST_DM_RELAYS", 10050),
        ts::const_number("KIND_LIST_GOOD_WIKI_AUTHORS", 10101),
        ts::const_number("KIND_LIST_GOOD_WIKI_RELAYS", 10102),
        ts::const_number("KIND_LIST_SET_FOLLOW", 30000),
        ts::const_number("KIND_LIST_SET_GENERIC", 30001),
        ts::const_number("KIND_LIST_SET_RELAY", 30002),
        ts::const_number("KIND_LIST_SET_BOOKMARK", 30003),
        ts::const_number("KIND_LIST_SET_CURATION", 30004),
        ts::const_number("KIND_LIST_SET_VIDEO", 30005),
        ts::const_number("KIND_LIST_SET_PICTURE", 30006),
        ts::const_number("KIND_LIST_SET_KIND_MUTE", 30007),
        ts::const_number("KIND_LIST_SET_INTEREST", 30015),
        ts::const_number("KIND_LIST_SET_EMOJI", 30030),
        ts::const_number("KIND_LIST_SET_RELEASE_ARTIFACT", 30063),
        ts::const_number("KIND_LIST_SET_APP_CURATION", 30267),
        ts::const_number("KIND_LIST_SET_CALENDAR", 31924),
        ts::const_number("KIND_LIST_SET_STARTER_PACK", 39089),
        ts::const_number("KIND_LIST_SET_MEDIA_STARTER_PACK", 39092),
        ts::const_number("KIND_FARM", 30340),
        ts::const_number("KIND_PLOT", 30350),
        ts::const_number("KIND_COOP", 30360),
        ts::const_number("KIND_DOCUMENT", 30361),
        ts::const_number("KIND_RESOURCE_AREA", 30370),
        ts::const_number("KIND_RESOURCE_HARVEST_CAP", 30371),
        ts::const_number("KIND_ACCOUNT_CLAIM", 30380),
        ts::const_number("KIND_APP_DATA", 30078),
        ts::const_number("KIND_LISTING", 30402),
        ts::const_number("KIND_APPLICATION_HANDLER", 31990),
        ts::const_number("KIND_TRADE_LISTING_VALIDATE_REQ", 5321),
        ts::const_number("KIND_TRADE_LISTING_VALIDATE_RES", 6321),
        ts::const_number("KIND_WORKER_TRADE_TRANSITION_PROOF_REQ", 5322),
        ts::const_number("KIND_WORKER_TRADE_TRANSITION_PROOF_RES", 6322),
        ts::const_number("KIND_TRADE_ORDER_REQUEST", 3422),
        ts::const_number("KIND_TRADE_ORDER_RESPONSE", 3423),
        ts::const_number("KIND_TRADE_ORDER_DECISION", 3423),
        ts::const_number("KIND_TRADE_ORDER_REVISION", 3424),
        ts::const_number("KIND_TRADE_ORDER_REVISION_RESPONSE", 3425),
        ts::const_number("KIND_TRADE_QUESTION", 3426),
        ts::const_number("KIND_TRADE_ANSWER", 3427),
        ts::const_number("KIND_TRADE_DISCOUNT_REQUEST", 3428),
        ts::const_number("KIND_TRADE_DISCOUNT_OFFER", 3429),
        ts::const_number("KIND_TRADE_DISCOUNT_ACCEPT", 3430),
        ts::const_number("KIND_TRADE_FORBIDDEN_3431", 3431),
        ts::const_number("KIND_TRADE_CANCEL", 3432),
        ts::const_number("KIND_TRADE_FULFILLMENT_UPDATE", 3433),
        ts::const_number("KIND_TRADE_RECEIPT", 3434),
        ts::const_number("KIND_TRADE_VALIDATION_RECEIPT", 3440),
        ts::const_number("KIND_TRADE_LISTING_ORDER_REQ", 3422),
        ts::const_number("KIND_TRADE_LISTING_ORDER_RES", 3423),
        ts::const_number("KIND_TRADE_LISTING_ORDER_REVISION_REQ", 3424),
        ts::const_number("KIND_TRADE_LISTING_ORDER_REVISION_RES", 3425),
        ts::const_number("KIND_TRADE_LISTING_QUESTION_REQ", 3426),
        ts::const_number("KIND_TRADE_LISTING_ANSWER_RES", 3427),
        ts::const_number("KIND_TRADE_LISTING_DISCOUNT_REQ", 3428),
        ts::const_number("KIND_TRADE_LISTING_DISCOUNT_OFFER_RES", 3429),
        ts::const_number("KIND_TRADE_LISTING_DISCOUNT_ACCEPT_REQ", 3430),
        ts::const_number("KIND_TRADE_LISTING_CANCEL_REQ", 3432),
        ts::const_number("KIND_TRADE_LISTING_FULFILLMENT_UPDATE_REQ", 3433),
        ts::const_number("KIND_TRADE_LISTING_RECEIPT_REQ", 3434),
        ts::const_number("KIND_JOB_REQUEST_MIN", 5000),
        ts::const_number("KIND_JOB_REQUEST_MAX", 5999),
        ts::const_number("KIND_JOB_RESULT_MIN", 6000),
        ts::const_number("KIND_JOB_RESULT_MAX", 6999),
        ts::const_number("KIND_JOB_FEEDBACK", 7000),
    ])
}
