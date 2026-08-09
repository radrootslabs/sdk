import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import {
  copyFileSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

import {
  HISTORICAL_ARTIFACTS,
  HISTORICAL_AUTHORITY_PATH,
  validateHistoricalAuthority,
} from "./radroots_sdk_contract_lib.mjs";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function fixture() {
  const target = mkdtempSync(join(tmpdir(), "radroots-sdk-contract-"));
  for (const relative of [
    HISTORICAL_AUTHORITY_PATH,
    ...HISTORICAL_ARTIFACTS.map((artifact) => artifact.path),
  ]) {
    const destination = join(target, relative);
    mkdirSync(dirname(destination), { recursive: true });
    copyFileSync(join(root, relative), destination);
  }
  return target;
}

test("checked-in historical authority is exact", () => {
  assert.doesNotThrow(() => validateHistoricalAuthority(root));
});

test("historical authority rejects artifact drift", () => {
  const target = fixture();
  const artifact = HISTORICAL_ARTIFACTS[0].path;
  writeFileSync(join(target, artifact), "tampered\n");
  assert.throws(
    () => validateHistoricalAuthority(target),
    /artifact digest mismatch/,
  );
});

test("historical authority rejects coordinated artifact and manifest drift", () => {
  const target = fixture();
  const artifact = HISTORICAL_ARTIFACTS[0].path;
  const replacement = "coordinated tamper\n";
  writeFileSync(join(target, artifact), replacement);
  const path = join(target, HISTORICAL_AUTHORITY_PATH);
  const manifest = JSON.parse(readFileSync(path, "utf8"));
  manifest.artifacts[0].sha256 = createHash("sha256")
    .update(replacement)
    .digest("hex");
  writeFileSync(path, `${JSON.stringify(manifest, null, 2)}\n`);
  assert.throws(
    () => validateHistoricalAuthority(target),
    /artifact 0 identity is invalid/,
  );
});

test("historical authority rejects unknown manifest fields", () => {
  const target = fixture();
  const path = join(target, HISTORICAL_AUTHORITY_PATH);
  const manifest = JSON.parse(readFileSync(path, "utf8"));
  manifest.unreviewed = true;
  writeFileSync(path, `${JSON.stringify(manifest, null, 2)}\n`);
  assert.throws(() => validateHistoricalAuthority(target), /invalid keys/);
});

test("historical authority rejects public documentation and workflows", () => {
  for (const forbidden of ["docs", ".github", ".act"]) {
    const target = fixture();
    mkdirSync(join(target, forbidden));
    assert.throws(
      () => validateHistoricalAuthority(target),
      { message: `forbidden capsule root exists: ${forbidden}` },
    );
  }
});
