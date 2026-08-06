#!/usr/bin/env node

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  artifactRoute,
  resolveCanonicalSourceRoot,
  run,
  sourceDateEpoch,
  validateConsumerRoot,
} from "./radroots_sdk_artifact_lib.mjs";

const consumerRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const [mode, routeName] = process.argv.slice(2);
if (mode !== "source-check" && !["check", "write"].includes(mode)) {
  throw new Error("usage: radroots_sdk_artifact.mjs source-check | <check|write> <typescript|wasm|swift|kotlin>");
}

validateConsumerRoot(consumerRoot);
const sourceRoot = resolveCanonicalSourceRoot(process.env.RADROOTS_LIB_SOURCE_ROOT);
const sourceLock = readFileSync(resolve(consumerRoot, "radroots.lib.source-lock.v1.toml"), "utf8");
const revision = sourceLock.match(/^revision = "([0-9a-f]{40})"$/m)?.[1];
if (!revision) {
  throw new Error("source lock must contain one full lowercase revision");
}

const commandEnv = { ...process.env };
if (process.env.RADROOTS_OFFLINE === "1") {
  commandEnv.CARGO_NET_OFFLINE = "true";
}

run("cargo", ["extbuild", "doctor"], { cwd: sourceRoot, env: commandEnv });
run(
  "cargo",
  [
    "extbuild",
    "run",
    "--",
    "cargo",
    "xtask",
    "source-lock",
    "--consumer-root",
    consumerRoot,
  ],
  { cwd: sourceRoot, env: commandEnv },
);

if (mode !== "source-check") {
  const route = artifactRoute(routeName);
  run(
    "cargo",
    [
      "extbuild",
      "run",
      "--",
      "cargo",
      "xtask",
      "artifact",
      "--product",
      "sdk",
      "--target",
      route.target,
      "--language",
      route.language,
      "--mode",
      mode,
      "--consumer-root",
      consumerRoot,
      "--source-root",
      sourceRoot,
      "--output",
      route.output,
      "--source-date-epoch",
      sourceDateEpoch(sourceRoot, revision, commandEnv),
      "--builder-id",
      "radroots_sdk_artifact_v1",
    ],
    { cwd: sourceRoot, env: commandEnv },
  );
}
