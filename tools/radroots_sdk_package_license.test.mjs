import assert from "node:assert/strict";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

import { validateGeneratedPackageLicenses } from "./radroots_sdk_package_license_lib.mjs";

const sourceRoot = fileURLToPath(new URL("..", import.meta.url));
const revision = "e55c1749902811732167a2ce5c099e22e964624f";

function fixture() {
  const root = mkdtempSync(join(tmpdir(), "radroots-sdk-licenses-"));
  const packageDirectory = join(root, "packages/example");
  mkdirSync(join(root, "contracts"), { recursive: true });
  mkdirSync(packageDirectory, { recursive: true });
  copyFileSync(
    join(sourceRoot, "contracts/generated_package_licenses.v1.json"),
    join(root, "contracts/generated_package_licenses.v1.json"),
  );
  for (const name of ["LICENSE-APACHE", "LICENSE-MIT", "NOTICE"]) {
    copyFileSync(join(sourceRoot, "packages/core-bindings", name), join(packageDirectory, name));
  }
  return { root, packageDirectory };
}

test("generated package licenses bind the selected producer revision", () => {
  const { root, packageDirectory } = fixture();
  assert.doesNotThrow(() => validateGeneratedPackageLicenses(root, [packageDirectory], revision));
  assert.throws(
    () => validateGeneratedPackageLicenses(root, [packageDirectory], "0".repeat(40)),
    /license authority drifted/,
  );
});

test("generated package license drift fails closed", () => {
  const { root, packageDirectory } = fixture();
  const path = join(packageDirectory, "LICENSE-MIT");
  writeFileSync(path, `${readFileSync(path, "utf8")}drift\n`);
  assert.throws(
    () => validateGeneratedPackageLicenses(root, [packageDirectory], revision),
    /license bytes drifted/,
  );
});

test("complete data notice bytes are required in every generated package", () => {
  for (const missing of [false, true]) {
    const { root, packageDirectory } = fixture();
    const path = join(packageDirectory, "NOTICE");
    if (missing) unlinkSync(path);
    else writeFileSync(path, "truncated data attribution\n");
    assert.throws(
      () => validateGeneratedPackageLicenses(root, [packageDirectory], revision),
      missing ? /ENOENT/ : /license bytes drifted/,
    );
  }
});

test("data notice cannot be removed from the selected producer inventory", () => {
  const { root, packageDirectory } = fixture();
  const path = join(root, "contracts/generated_package_licenses.v1.json");
  const contract = JSON.parse(readFileSync(path, "utf8"));
  contract.files = contract.files.filter((file) => file.name !== "NOTICE");
  writeFileSync(path, JSON.stringify(contract));
  assert.throws(
    () => validateGeneratedPackageLicenses(root, [packageDirectory], revision),
    /license inventory drifted/,
  );
});
