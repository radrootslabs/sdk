import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const CONTRACT_PATH = "contracts/generated_package_licenses.v1.json";
const EXPECTED_FILES = Object.freeze(["LICENSE-APACHE", "LICENSE-MIT"]);

function fail(message) {
  throw new Error(`supply_chain_invalid: ${message}`);
}

function exactKeys(value, expected, context) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    fail(`${context} must be an object`);
  }
  const actual = Object.keys(value).sort();
  if (JSON.stringify(actual) !== JSON.stringify([...expected].sort())) {
    fail(`${context} has invalid keys`);
  }
}

function sha256(path) {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

export function validateGeneratedPackageLicenses(root, packageDirectories, expectedRevision) {
  const contract = JSON.parse(readFileSync(join(root, CONTRACT_PATH), "utf8"));
  exactKeys(
    contract,
    ["schema", "source_repository", "source_revision", "files"],
    "generated package license contract",
  );
  if (
    contract.schema !== "radroots.sdk.generated-package-licenses.v1" ||
    contract.source_repository !== "https://github.com/radrootslabs/lib" ||
    contract.source_revision !== expectedRevision
  ) {
    fail("generated package license authority drifted");
  }
  if (!Array.isArray(contract.files) || contract.files.length !== EXPECTED_FILES.length) {
    fail("generated package license inventory drifted");
  }

  for (let index = 0; index < EXPECTED_FILES.length; index += 1) {
    const file = contract.files[index];
    exactKeys(file, ["name", "sha256"], `generated package license ${index}`);
    if (file.name !== EXPECTED_FILES[index] || !/^[0-9a-f]{64}$/.test(file.sha256)) {
      fail("generated package license identity drifted");
    }
    for (const packageDirectory of packageDirectories) {
      if (sha256(join(packageDirectory, file.name)) !== file.sha256) {
        fail("generated package license bytes drifted");
      }
    }
  }
}
