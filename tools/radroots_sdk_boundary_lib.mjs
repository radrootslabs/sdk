import { lstatSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

export const GENERATED_FILES = Object.freeze([
  "generated/kotlin/source.lock",
  "generated/kotlin/uniffi/radroots_sdk/radroots_sdk.kt",
  "generated/swift/radroots_sdk.swift",
  "generated/swift/radroots_sdkFFI.h",
  "generated/swift/radroots_sdkFFI.modulemap",
  "generated/swift/source.lock",
  "packages/core-bindings/src/generated/types.ts",
  "packages/event-bindings/src/generated/constants.ts",
  "packages/event-bindings/src/generated/kinds.ts",
  "packages/event-bindings/src/generated/types.ts",
  "packages/identity-bindings/src/generated/constants.ts",
  "packages/replica-schema-bindings/src/generated/types.ts",
  "packages/trade-bindings/src/generated/types.ts",
]);

const SENSITIVE_PATH = /(^|\/)(\.env|id_rsa|id_ed25519|credentials|[^/]+\.(pem|key|p12|pfx|jks|keystore))$/i;
const CREDENTIAL_MATERIAL = /-----BEGIN (?:[A-Z0-9 ]+ )?PRIVATE KEY-----|AKIA[0-9A-Z]{16}|gh[pousr]_[A-Za-z0-9_]{36,}|nsec1[023456789acdefghjklmnpqrstuvwxyz]{40,}/;
const TEXT_SOURCE = /\.(?:h|js|json|kt|md|mjs|modulemap|rs|swift|toml|ts)$/;

function fail(message) {
  throw new Error(`boundary_invalid: ${message}`);
}

function exactInventory(actual, expected, context) {
  const sorted = [...actual].sort();
  if (JSON.stringify(sorted) !== JSON.stringify(expected)) {
    fail(`${context} is not exact`);
  }
}

export function validateTrackedInventory(paths) {
  for (const path of paths) {
    if (SENSITIVE_PATH.test(path)) fail("sensitive credential path is tracked");
  }
  const generated = paths.filter(
    (path) => path.startsWith("generated/") || /\/src\/generated\//.test(path),
  );
  exactInventory(generated, GENERATED_FILES, "generated source inventory");
}

export function validateProductionSources(root, paths) {
  for (const path of paths) {
    if (
      !TEXT_SOURCE.test(path) ||
      !["crates/", "generated/", "packages/"].some((prefix) => path.startsWith(prefix))
    ) {
      continue;
    }
    const fullPath = resolve(root, path);
    const metadata = lstatSync(fullPath);
    if (!metadata.isFile() || metadata.isSymbolicLink()) {
      fail("production source must be a regular non-symlink file");
    }
    if (CREDENTIAL_MATERIAL.test(readFileSync(fullPath, "utf8"))) {
      fail("production source contains credential material");
    }
  }
}
