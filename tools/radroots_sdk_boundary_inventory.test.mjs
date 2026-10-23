import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";

import {
  GENERATED_FILES,
  validateProductionSources,
  validateTrackedInventory,
} from "./radroots_sdk_boundary_lib.mjs";

test("generated source inventory is exact", () => {
  assert.doesNotThrow(() => validateTrackedInventory(GENERATED_FILES));
  assert.throws(
    () => validateTrackedInventory(GENERATED_FILES.slice(1)),
    /generated source inventory is not exact/,
  );
  assert.throws(
    () => validateTrackedInventory([...GENERATED_FILES, "generated/extra.ts"]),
    /generated source inventory is not exact/,
  );
});

test("both native language artifacts retain their complete data notice", () => {
  for (const language of ["swift", "kotlin"]) {
    assert.throws(
      () => validateTrackedInventory(GENERATED_FILES.filter((path) => path !== `generated/${language}/NOTICE`)),
      /generated source inventory is not exact/,
    );
  }
});

test("tracked credential paths fail closed", () => {
  assert.throws(
    () => validateTrackedInventory([...GENERATED_FILES, "packages/core/.env"]),
    /sensitive credential path is tracked/,
  );
});

test("production credential material fails closed", () => {
  const root = mkdtempSync(join(tmpdir(), "radroots-sdk-boundary-"));
  const path = "packages/core/src/index.ts";
  mkdirSync(dirname(join(root, path)), { recursive: true });
  writeFileSync(join(root, path), `export const leaked = "AKIAABCDEFGHIJKLMNOP";\n`);
  assert.throws(
    () => validateProductionSources(root, [path]),
    /production source contains credential material/,
  );
});
