import {
    KIND_LISTING,
    RADROOTS_LISTING_PRODUCT_TAG_KEYS,
} from "@radroots/events-bindings";
import { RADROOTS_USERNAME_MIN_LEN } from "@radroots/identity-bindings";
import type { RadrootsCoreMoney } from "@radroots/core-bindings";
import type {
    RadrootsEventsIndexedManifest,
    RadrootsEventsIndexedShardId,
} from "@radroots/events-indexed-bindings";
import type { RadrootsListing } from "@radroots/events-bindings";
import type { Farm } from "@radroots/replica-db-schema-bindings";
import type { RadrootsTradeListingTotal } from "@radroots/trade-bindings";
import type { IError } from "@radroots/types-bindings";

const amount: RadrootsCoreMoney = {
    amount: "1.00",
    currency: "USD",
};

const shardId: RadrootsEventsIndexedShardId = "us-1";

const manifest: RadrootsEventsIndexedManifest = {
    country: "US",
    total: 0,
    shard_size: 1000,
    first_published_at: 0,
    last_published_at: 0,
    shards: [],
};

const farm = undefined as unknown as Farm;
const listing = undefined as unknown as RadrootsListing;
const total = undefined as unknown as RadrootsTradeListingTotal;
const error = undefined as unknown as IError<string>;

export const smoke = {
    amount,
    error,
    farm,
    kind: KIND_LISTING,
    listing,
    manifest,
    productKeys: RADROOTS_LISTING_PRODUCT_TAG_KEYS,
    shardId,
    total,
    usernameMinLen: RADROOTS_USERNAME_MIN_LEN,
};
