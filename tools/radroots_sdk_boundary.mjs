#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  validateProductionSources,
  validateTrackedInventory,
} from "./radroots_sdk_boundary_lib.mjs";
import { validateHistoricalAuthority } from "./radroots_sdk_contract_lib.mjs";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const tracked = execFileSync("git", ["ls-files", "-z"], {
  cwd: root,
  encoding: "utf8",
})
  .split("\0")
  .filter(Boolean);

validateHistoricalAuthority(root);
validateTrackedInventory(tracked);
validateProductionSources(root, tracked);
process.stdout.write(`SDK boundaries: OK (${tracked.length} tracked files)\n`);
