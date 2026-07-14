export * from "./generated/types.js";
export * from "./generated/constants.js";
export * from "./generated/kinds.js";

export const RADROOTS_EVENT_TIMESTAMP_MAX_SAFE_INTEGER = Number.MAX_SAFE_INTEGER;

export function radrootsEventTimestampFromJsonNumber(value: number): number {
  return radrootsEventTimestampJsonNumber(value);
}

export function radrootsEventTimestampToJsonNumber(value: number): number {
  return radrootsEventTimestampJsonNumber(value);
}

function radrootsEventTimestampJsonNumber(value: number): number {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new RangeError(
      `Radroots event timestamp must be a non-negative safe integer no greater than Number.MAX_SAFE_INTEGER (${Number.MAX_SAFE_INTEGER})`,
    );
  }
  return value;
}
