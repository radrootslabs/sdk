import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

export const HISTORICAL_AUTHORITY_PATH =
  "contracts/historical_authority.v1.json";

export const HISTORICAL_ARTIFACTS = Object.freeze([
  Object.freeze({
    path: "contracts/api_baselines/radroots-0.1.0-alpha.txt",
    role: "public_api_baseline",
    sha256: "e0c21fc096715fba3b3273bb1a8a880d6e871161772c5a1019404064aa0becb1",
  }),
  Object.freeze({
    path: "contracts/api_baselines/radroots_sdk-0.1.0-alpha.txt",
    role: "public_api_baseline",
    sha256: "97dfccb393fcc7953a59f0b12f03485daf9b7c625e9a4529fd883fcf0306e618",
  }),
  Object.freeze({
    path: "contracts/architecture/deviations.toml",
    role: "historical_deviation_ledger",
    sha256: "888d581264cddf6fe0b4a09a2171535094ef9fb3c422f7866e4e0c802359ea1f",
  }),
  Object.freeze({
    path: "contracts/crates/release_v1/radroots_crates_release_v1.dot",
    role: "historical_release_graph",
    sha256: "d47de10be596a4d33fee102a4f0617f66700b49515a75a1f42d62c9710043059",
  }),
  Object.freeze({
    path: "contracts/crates/release_v1/radroots_crates_release_v1.sha256",
    role: "captured_stale_checksum_manifest",
    sha256: "5759ceaae30a9435346320c2791cfda9eb1559b779ea79fd2e94810e14efa281",
  }),
  Object.freeze({
    path: "contracts/crates/release_v1/radroots_crates_release_v1.toml",
    role: "historical_release_catalog",
    sha256: "1dc18437200dcd65b52090493306f452dade89b5116401d71be4ba4127239b19",
  }),
  Object.freeze({
    path: "contracts/crates/release_v1/radroots_crates_release_v1_inventory.csv",
    role: "historical_release_inventory",
    sha256: "5020875c2cda4b2c9568c8b3f0fad5cd96756c3c779a72481a9652e558f77891",
  }),
]);

const RETIRED_HUMAN_ARTIFACTS = Object.freeze([
  Object.freeze({
    former_path:
      "docs/decisions/0001-public-api-leakage-migration-baseline.md",
    parent_path:
      "docs/oss/sdk/release-v1-history/decisions/0001-public-api-leakage-migration-baseline.md",
    sha256: "c5f2367bc85c84ce5a3af0d066d3fc8dab6d4e9f1061bb6af35b15d9e4b6e2b1",
  }),
  Object.freeze({
    former_path: "docs/specs/radroots_crates_release_v1.md",
    parent_path:
      "docs/oss/sdk/release-v1-history/release-v1-specification.md",
    sha256: "6f98eb958a29921919147c44ff6a80565df367adf362f7ac872ce2588a09a1e5",
  }),
]);

const CAPTURED_CHECKSUM_MANIFEST =
  "5a11c6ad90cf03162ca2ce4d1692192d01fa57a31c03fd07d61f70093da5e703  radroots_crates_release_v1.md\n" +
  "7db533f32c70306b29adea35f85686e67287ae33abd33d1013a61590449f1296  radroots_crates_release_v1.toml\n" +
  "5020875c2cda4b2c9568c8b3f0fad5cd96756c3c779a72481a9652e558f77891  radroots_crates_release_v1_inventory.csv\n" +
  "d47de10be596a4d33fee102a4f0617f66700b49515a75a1f42d62c9710043059  radroots_crates_release_v1.dot\n";

function exactKeys(value, expected, context) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`${context} must be an object`);
  }
  const actual = Object.keys(value).sort();
  const wanted = [...expected].sort();
  if (JSON.stringify(actual) !== JSON.stringify(wanted)) {
    throw new Error(`${context} has invalid keys`);
  }
}

function regularFile(path, context) {
  let metadata;
  try {
    metadata = lstatSync(path);
  } catch (error) {
    throw new Error(`${context} is missing: ${error.code ?? error.message}`);
  }
  if (!metadata.isFile() || metadata.isSymbolicLink()) {
    throw new Error(`${context} must be a regular non-symlink file`);
  }
}

function pathExists(path) {
  try {
    lstatSync(path);
    return true;
  } catch (error) {
    if (error.code === "ENOENT") {
      return false;
    }
    throw error;
  }
}

function sha256(path) {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

export function validateHistoricalAuthority(root) {
  for (const forbidden of ["docs", ".github", ".act"]) {
    if (pathExists(resolve(root, forbidden))) {
      throw new Error(`forbidden capsule root exists: ${forbidden}`);
    }
  }

  const manifestPath = resolve(root, HISTORICAL_AUTHORITY_PATH);
  regularFile(manifestPath, HISTORICAL_AUTHORITY_PATH);
  const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
  exactKeys(
    manifest,
    [
      "schema_version",
      "contract_id",
      "status",
      "source_revision",
      "parent_human_owner",
      "capsule_human_docs_forbidden",
      "artifacts",
      "retired_human_artifacts",
      "captured_checksum_manifest",
    ],
    "historical authority",
  );
  if (
    manifest.schema_version !== 1 ||
    manifest.contract_id !== "radroots.sdk.historical_authority.v1" ||
    manifest.status !== "historical" ||
    manifest.source_revision !==
      "bcda74b3ebfff3f711670cc25b7910f27360fba7" ||
    manifest.parent_human_owner !== "docs/oss/sdk/release-v1-history" ||
    manifest.capsule_human_docs_forbidden !== true
  ) {
    throw new Error("historical authority identity is invalid");
  }

  if (
    !Array.isArray(manifest.artifacts) ||
    manifest.artifacts.length !== HISTORICAL_ARTIFACTS.length
  ) {
    throw new Error("historical artifact inventory is not exact");
  }
  for (let index = 0; index < HISTORICAL_ARTIFACTS.length; index += 1) {
    const artifact = manifest.artifacts[index];
    const expected = HISTORICAL_ARTIFACTS[index];
    exactKeys(artifact, ["path", "role", "sha256"], `artifact ${index}`);
    if (
      artifact.path !== expected.path ||
      artifact.role !== expected.role ||
      artifact.sha256 !== expected.sha256
    ) {
      throw new Error(`artifact ${index} identity is invalid`);
    }
    if (!/^[0-9a-f]{64}$/.test(artifact.sha256)) {
      throw new Error(`artifact ${index} digest is invalid`);
    }
    const artifactPath = resolve(root, artifact.path);
    regularFile(artifactPath, artifact.path);
    if (sha256(artifactPath) !== artifact.sha256) {
      throw new Error(`artifact digest mismatch: ${artifact.path}`);
    }
  }

  if (
    JSON.stringify(manifest.retired_human_artifacts) !==
    JSON.stringify(RETIRED_HUMAN_ARTIFACTS)
  ) {
    throw new Error("retired human artifact inventory is not exact");
  }

  exactKeys(
    manifest.captured_checksum_manifest,
    ["path", "status", "current_digest_authority"],
    "captured checksum manifest",
  );
  if (
    manifest.captured_checksum_manifest.path !==
      "contracts/crates/release_v1/radroots_crates_release_v1.sha256" ||
    manifest.captured_checksum_manifest.status !==
      "historical_stale_capture" ||
    manifest.captured_checksum_manifest.current_digest_authority !==
      HISTORICAL_AUTHORITY_PATH
  ) {
    throw new Error("captured checksum disposition is invalid");
  }
  if (
    readFileSync(
      resolve(root, manifest.captured_checksum_manifest.path),
      "utf8",
    ) !== CAPTURED_CHECKSUM_MANIFEST
  ) {
    throw new Error("captured stale checksum manifest changed");
  }
}
