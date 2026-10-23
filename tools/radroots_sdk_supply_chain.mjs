import { readFileSync, readdirSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { validateGeneratedPackageLicenses } from "./radroots_sdk_package_license_lib.mjs";

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const expectedLicense = "MIT OR Apache-2.0";
const expectedRepository = "git+https://github.com/radrootslabs/sdk.git";
const expectedPackages = [
  "core-bindings",
  "event-bindings",
  "event-codec-wasm",
  "identity-bindings",
  "replica-schema-bindings",
  "replica-store-wasm",
  "replica-sync-wasm",
  "trade-bindings",
];

function fail(message) {
  throw new Error(`supply_chain_invalid: ${message}`);
}

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf8"));
}

function packageDirectories() {
  return readdirSync(join(root, "packages"), { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => join(root, "packages", entry.name))
    .filter((path) => {
      try {
        readFileSync(join(path, "package.json"));
        return true;
      } catch {
        return false;
      }
    })
    .sort();
}

function verifyManifest(path) {
  const manifest = readJson(join(path, "package.json"));
  if (manifest.license !== expectedLicense) fail("package license drifted");
  if (
    manifest.repository?.type !== "git" ||
    manifest.repository?.url !== expectedRepository ||
    manifest.repository?.directory !== `packages/${basename(path)}`
  ) {
    fail("package repository identity drifted");
  }
  if (
    !Array.isArray(manifest.files) ||
    !manifest.files.includes("LICENSE-MIT") ||
    !manifest.files.includes("LICENSE-APACHE") ||
    !manifest.files.includes("NOTICE")
  ) {
    fail("package artifact omits governed license files");
  }
  for (const section of ["dependencies", "devDependencies", "optionalDependencies", "peerDependencies"]) {
    for (const value of Object.values(manifest[section] ?? {})) {
      if (!String(value).startsWith("workspace:")) {
        fail("package dependency is not workspace-locked");
      }
    }
  }
}

const manifest = readJson(join(root, "package.json"));
if (manifest.packageManager !== "pnpm@10.25.0") fail("package manager drifted");
if (manifest.devDependencies?.typescript !== "^5.9.0") fail("TypeScript input drifted");
if (Object.keys(manifest.devDependencies ?? {}).length !== 1) fail("root dependency inventory drifted");

const lock = readFileSync(join(root, "pnpm-lock.yaml"), "utf8");
for (const required of [
  "lockfileVersion: '9.0'",
  "typescript@5.9.3:",
  "specifier: ^5.9.0",
  "sha512-jl1vZzPDinLr9eUt3J/t7V6FgNEw9QjvBPdysz9KfQDD41fQrC2Y4vKQdiaUpFT4bXlb1RHhLpp8wtm6M5TgSw==",
]) {
  if (!lock.includes(required)) fail("pnpm lock authority drifted");
}
for (const forbidden of ["git+", "github:", "http://"]) {
  if (lock.includes(forbidden)) fail("pnpm lock contains an unapproved source");
}

const sourceLock = readFileSync(join(root, "radroots.lib.source-lock.v1.toml"), "utf8");
if (!/^repository = "https:\/\/github\.com\/radrootslabs\/lib"$/m.test(sourceLock)) {
  fail("Lib repository identity drifted");
}
if (!/^revision = "[0-9a-f]{40}"$/m.test(sourceLock)) fail("Lib revision is not immutable");
if (!/^lockfile_sha256 = "[0-9a-f]{64}"$/m.test(sourceLock)) fail("Cargo lock digest is absent");
const sourceRevision = sourceLock.match(/^revision = "([0-9a-f]{40})"$/m)?.[1];
if (!sourceRevision) fail("Lib revision is not immutable");

const packages = packageDirectories();
if (
  packages.length !== expectedPackages.length ||
  packages.some((path, index) => basename(path) !== expectedPackages[index])
) {
  fail("package inventory drifted");
}
for (const path of packages) verifyManifest(path);
validateGeneratedPackageLicenses(root, packages, sourceRevision);

console.log(`supply-chain ok: ${packages.length} packages`);
