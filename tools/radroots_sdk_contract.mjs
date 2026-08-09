#!/usr/bin/env node

import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { validateHistoricalAuthority } from "./radroots_sdk_contract_lib.mjs";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
validateHistoricalAuthority(root);
process.stdout.write("SDK contracts: OK\n");
