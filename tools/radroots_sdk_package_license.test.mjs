import assert from "node:assert/strict";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

import { validateGeneratedPackageLicenses } from "./radroots_sdk_package_license_lib.mjs";

const sourceRoot = fileURLToPath(new URL("..", import.meta.url));
const revision = "be9db78e060ebc0000fa7827ac32efa3f6504f53";

function fixture() {
  const root = mkdtempSync(join(tmpdir(), "radroots-sdk-licenses-"));
  const packageDirectory = join(root, "packages/example");
  mkdirSync(join(root, "contracts"), { recursive: true });
  mkdirSync(packageDirectory, { recursive: true });
  copyFileSync(
    join(sourceRoot, "contracts/generated_package_licenses.v1.json"),
    join(root, "contracts/generated_package_licenses.v1.json"),
  );
  for (const name of ["LICENSE-APACHE", "LICENSE-MIT"]) {
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
