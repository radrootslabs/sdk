use radroots_events::{kinds, listing::RADROOTS_LISTING_PRODUCT_TAG_KEYS};

pub fn constants_module() -> String {
    format!(
        "import type {{ RadrootsListingProductTagKeys }} from \"./types.js\";\n\nexport const RADROOTS_LISTING_PRODUCT_TAG_KEYS: RadrootsListingProductTagKeys = {};",
        render_string_array(&RADROOTS_LISTING_PRODUCT_TAG_KEYS)
    )
}

pub fn kinds_module() -> String {
    render_number_constants(EVENT_KIND_EXPORTS)
}

const EVENT_KIND_EXPORTS: &[(&str, u32)] = &[
    ("KIND_PROFILE", kinds::KIND_PROFILE),
    ("KIND_POST", kinds::KIND_POST),
    ("KIND_FOLLOW", kinds::KIND_FOLLOW),
    ("KIND_REACTION", kinds::KIND_REACTION),
    ("KIND_SEAL", kinds::KIND_SEAL),
    ("KIND_MESSAGE", kinds::KIND_MESSAGE),
    ("KIND_MESSAGE_FILE", kinds::KIND_MESSAGE_FILE),
    ("KIND_GIFT_WRAP", kinds::KIND_GIFT_WRAP),
    ("KIND_COMMENT", kinds::KIND_COMMENT),
    ("KIND_GEOCHAT", kinds::KIND_GEOCHAT),
    ("KIND_LIST_MUTE", kinds::KIND_LIST_MUTE),
    ("KIND_LIST_PINNED_NOTES", kinds::KIND_LIST_PINNED_NOTES),
    (
        "KIND_LIST_READ_WRITE_RELAYS",
        kinds::KIND_LIST_READ_WRITE_RELAYS,
    ),
    ("KIND_LIST_BOOKMARKS", kinds::KIND_LIST_BOOKMARKS),
    ("KIND_LIST_COMMUNITIES", kinds::KIND_LIST_COMMUNITIES),
    ("KIND_LIST_PUBLIC_CHATS", kinds::KIND_LIST_PUBLIC_CHATS),
    ("KIND_LIST_BLOCKED_RELAYS", kinds::KIND_LIST_BLOCKED_RELAYS),
    ("KIND_LIST_SEARCH_RELAYS", kinds::KIND_LIST_SEARCH_RELAYS),
    ("KIND_LIST_SIMPLE_GROUPS", kinds::KIND_LIST_SIMPLE_GROUPS),
    ("KIND_LIST_RELAY_FEEDS", kinds::KIND_LIST_RELAY_FEEDS),
    ("KIND_LIST_INTERESTS", kinds::KIND_LIST_INTERESTS),
    ("KIND_LIST_MEDIA_FOLLOWS", kinds::KIND_LIST_MEDIA_FOLLOWS),
    ("KIND_LIST_EMOJIS", kinds::KIND_LIST_EMOJIS),
    ("KIND_LIST_DM_RELAYS", kinds::KIND_LIST_DM_RELAYS),
    (
        "KIND_LIST_GOOD_WIKI_AUTHORS",
        kinds::KIND_LIST_GOOD_WIKI_AUTHORS,
    ),
    (
        "KIND_LIST_GOOD_WIKI_RELAYS",
        kinds::KIND_LIST_GOOD_WIKI_RELAYS,
    ),
    ("KIND_LIST_SET_FOLLOW", kinds::KIND_LIST_SET_FOLLOW),
    ("KIND_LIST_SET_GENERIC", kinds::KIND_LIST_SET_GENERIC),
    ("KIND_LIST_SET_RELAY", kinds::KIND_LIST_SET_RELAY),
    ("KIND_LIST_SET_BOOKMARK", kinds::KIND_LIST_SET_BOOKMARK),
    ("KIND_LIST_SET_CURATION", kinds::KIND_LIST_SET_CURATION),
    ("KIND_LIST_SET_VIDEO", kinds::KIND_LIST_SET_VIDEO),
    ("KIND_LIST_SET_PICTURE", kinds::KIND_LIST_SET_PICTURE),
    ("KIND_LIST_SET_KIND_MUTE", kinds::KIND_LIST_SET_KIND_MUTE),
    ("KIND_LIST_SET_INTEREST", kinds::KIND_LIST_SET_INTEREST),
    ("KIND_LIST_SET_EMOJI", kinds::KIND_LIST_SET_EMOJI),
    (
        "KIND_LIST_SET_RELEASE_ARTIFACT",
        kinds::KIND_LIST_SET_RELEASE_ARTIFACT,
    ),
    (
        "KIND_LIST_SET_APP_CURATION",
        kinds::KIND_LIST_SET_APP_CURATION,
    ),
    ("KIND_LIST_SET_CALENDAR", kinds::KIND_LIST_SET_CALENDAR),
    (
        "KIND_LIST_SET_STARTER_PACK",
        kinds::KIND_LIST_SET_STARTER_PACK,
    ),
    (
        "KIND_LIST_SET_MEDIA_STARTER_PACK",
        kinds::KIND_LIST_SET_MEDIA_STARTER_PACK,
    ),
    ("KIND_FARM", kinds::KIND_FARM),
    ("KIND_PLOT", kinds::KIND_PLOT),
    ("KIND_COOP", kinds::KIND_COOP),
    ("KIND_DOCUMENT", kinds::KIND_DOCUMENT),
    ("KIND_RESOURCE_AREA", kinds::KIND_RESOURCE_AREA),
    (
        "KIND_RESOURCE_HARVEST_CAP",
        kinds::KIND_RESOURCE_HARVEST_CAP,
    ),
    ("KIND_ACCOUNT_CLAIM", kinds::KIND_ACCOUNT_CLAIM),
    ("KIND_APP_DATA", kinds::KIND_APP_DATA),
    ("KIND_LISTING", kinds::KIND_LISTING),
    ("KIND_APPLICATION_HANDLER", kinds::KIND_APPLICATION_HANDLER),
    (
        "KIND_TRADE_LISTING_VALIDATE_REQ",
        kinds::KIND_TRADE_LISTING_VALIDATE_REQ,
    ),
    (
        "KIND_TRADE_LISTING_VALIDATE_RES",
        kinds::KIND_TRADE_LISTING_VALIDATE_RES,
    ),
    (
        "KIND_WORKER_TRADE_TRANSITION_PROOF_REQ",
        kinds::KIND_WORKER_TRADE_TRANSITION_PROOF_REQ,
    ),
    (
        "KIND_WORKER_TRADE_TRANSITION_PROOF_RES",
        kinds::KIND_WORKER_TRADE_TRANSITION_PROOF_RES,
    ),
    ("KIND_TRADE_ORDER_REQUEST", kinds::KIND_TRADE_ORDER_REQUEST),
    (
        "KIND_TRADE_ORDER_RESPONSE",
        kinds::KIND_TRADE_ORDER_RESPONSE,
    ),
    (
        "KIND_TRADE_ORDER_DECISION",
        kinds::KIND_TRADE_ORDER_DECISION,
    ),
    (
        "KIND_TRADE_ORDER_REVISION",
        kinds::KIND_TRADE_ORDER_REVISION,
    ),
    (
        "KIND_TRADE_ORDER_REVISION_RESPONSE",
        kinds::KIND_TRADE_ORDER_REVISION_RESPONSE,
    ),
    ("KIND_TRADE_QUESTION", kinds::KIND_TRADE_QUESTION),
    ("KIND_TRADE_ANSWER", kinds::KIND_TRADE_ANSWER),
    (
        "KIND_TRADE_DISCOUNT_REQUEST",
        kinds::KIND_TRADE_DISCOUNT_REQUEST,
    ),
    (
        "KIND_TRADE_DISCOUNT_OFFER",
        kinds::KIND_TRADE_DISCOUNT_OFFER,
    ),
    (
        "KIND_TRADE_DISCOUNT_ACCEPT",
        kinds::KIND_TRADE_DISCOUNT_ACCEPT,
    ),
    (
        "KIND_TRADE_FORBIDDEN_3431",
        kinds::KIND_TRADE_FORBIDDEN_3431,
    ),
    ("KIND_TRADE_CANCEL", kinds::KIND_TRADE_CANCEL),
    (
        "KIND_TRADE_FULFILLMENT_UPDATE",
        kinds::KIND_TRADE_FULFILLMENT_UPDATE,
    ),
    ("KIND_TRADE_RECEIPT", kinds::KIND_TRADE_RECEIPT),
    (
        "KIND_TRADE_VALIDATION_RECEIPT",
        kinds::KIND_TRADE_VALIDATION_RECEIPT,
    ),
    (
        "KIND_TRADE_LISTING_ORDER_REQ",
        kinds::KIND_TRADE_LISTING_ORDER_REQ,
    ),
    (
        "KIND_TRADE_LISTING_ORDER_RES",
        kinds::KIND_TRADE_LISTING_ORDER_RES,
    ),
    (
        "KIND_TRADE_LISTING_ORDER_REVISION_REQ",
        kinds::KIND_TRADE_LISTING_ORDER_REVISION_REQ,
    ),
    (
        "KIND_TRADE_LISTING_ORDER_REVISION_RES",
        kinds::KIND_TRADE_LISTING_ORDER_REVISION_RES,
    ),
    (
        "KIND_TRADE_LISTING_QUESTION_REQ",
        kinds::KIND_TRADE_LISTING_QUESTION_REQ,
    ),
    (
        "KIND_TRADE_LISTING_ANSWER_RES",
        kinds::KIND_TRADE_LISTING_ANSWER_RES,
    ),
    (
        "KIND_TRADE_LISTING_DISCOUNT_REQ",
        kinds::KIND_TRADE_LISTING_DISCOUNT_REQ,
    ),
    (
        "KIND_TRADE_LISTING_DISCOUNT_OFFER_RES",
        kinds::KIND_TRADE_LISTING_DISCOUNT_OFFER_RES,
    ),
    (
        "KIND_TRADE_LISTING_DISCOUNT_ACCEPT_REQ",
        kinds::KIND_TRADE_LISTING_DISCOUNT_ACCEPT_REQ,
    ),
    (
        "KIND_TRADE_LISTING_CANCEL_REQ",
        kinds::KIND_TRADE_LISTING_CANCEL_REQ,
    ),
    (
        "KIND_TRADE_LISTING_FULFILLMENT_UPDATE_REQ",
        kinds::KIND_TRADE_LISTING_FULFILLMENT_UPDATE_REQ,
    ),
    (
        "KIND_TRADE_LISTING_RECEIPT_REQ",
        kinds::KIND_TRADE_LISTING_RECEIPT_REQ,
    ),
    ("KIND_JOB_REQUEST_MIN", kinds::KIND_JOB_REQUEST_MIN),
    ("KIND_JOB_REQUEST_MAX", kinds::KIND_JOB_REQUEST_MAX),
    ("KIND_JOB_RESULT_MIN", kinds::KIND_JOB_RESULT_MIN),
    ("KIND_JOB_RESULT_MAX", kinds::KIND_JOB_RESULT_MAX),
    ("KIND_JOB_FEEDBACK", kinds::KIND_JOB_FEEDBACK),
];

fn render_number_constants(exports: &[(&str, u32)]) -> String {
    let mut rendered = String::new();
    for (name, value) in exports {
        rendered.push_str("export const ");
        rendered.push_str(name);
        rendered.push_str(" = ");
        rendered.push_str(&value.to_string());
        rendered.push_str(";\n");
    }
    rendered
}

fn render_string_array(values: &[&str]) -> String {
    let items = values
        .iter()
        .map(|value| format!("{value:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{items}]")
}
