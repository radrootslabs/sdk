import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";

import {
  ARTIFACT_ROUTES,
  artifactRoute,
  resolveCanonicalSourceRoot,
  validateConsumerRoot,
} from "./radroots_sdk_artifact_lib.mjs";

test("consumer marker fails closed", () => {
  const root = mkdtempSync(join(tmpdir(), "radroots-sdk-consumer-"));
  writeFileSync(join(root, ".radroots-consumer-root"), "mobile\n");
  assert.throws(() => validateConsumerRoot(root), /exactly sdk/);
});

test("source root rejects relative and symlink paths", () => {
  const root = mkdtempSync(join(tmpdir(), "radroots-sdk-source-"));
  const source = join(root, "source");
  const link = join(root, "source-link");
  mkdirSync(source);
  symlinkSync(source, link);
  assert.throws(() => resolveCanonicalSourceRoot("relative"), /absolute/);
  assert.throws(() => resolveCanonicalSourceRoot(link), /real directory/);
});

test("artifact routes own fixed safe outputs", () => {
  assert.deepEqual(Object.keys(ARTIFACT_ROUTES), [
    "typescript",
    "wasm",
    "swift",
    "kotlin",
  ]);
  for (const route of Object.values(ARTIFACT_ROUTES)) {
    assert.match(route.output, /^contracts\/provenance\/sdk\/[a-z]+\.json$/);
    assert.ok(!route.output.includes(".."));
  }
  assert.throws(() => artifactRoute("../swift"), /unsupported/);
});
