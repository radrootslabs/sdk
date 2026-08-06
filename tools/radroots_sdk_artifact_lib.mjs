import { lstatSync, readFileSync, realpathSync } from "node:fs";
import { isAbsolute, resolve } from "node:path";
import { spawnSync } from "node:child_process";

export const ARTIFACT_ROUTES = Object.freeze({
  typescript: Object.freeze({
    target: "typescript",
    language: "typescript",
    output: "contracts/provenance/sdk/typescript.json",
  }),
  wasm: Object.freeze({
    target: "wasm",
    language: "javascript",
    output: "contracts/provenance/sdk/wasm.json",
  }),
  swift: Object.freeze({
    target: "ffi",
    language: "swift",
    output: "contracts/provenance/sdk/swift.json",
  }),
  kotlin: Object.freeze({
    target: "ffi",
    language: "kotlin",
    output: "contracts/provenance/sdk/kotlin.json",
  }),
});

export function validateConsumerRoot(root) {
  const marker = readFileSync(resolve(root, ".radroots-consumer-root"), "utf8");
  if (marker !== "sdk\n") {
    throw new Error("consumer marker must contain exactly sdk followed by LF");
  }
}

export function resolveCanonicalSourceRoot(raw) {
  if (!raw || !isAbsolute(raw)) {
    throw new Error("RADROOTS_LIB_SOURCE_ROOT must be an absolute path");
  }
  const requested = resolve(raw);
  const metadata = lstatSync(requested);
  if (!metadata.isDirectory() || metadata.isSymbolicLink()) {
    throw new Error("RADROOTS_LIB_SOURCE_ROOT must be a real directory");
  }
  const canonical = realpathSync(requested);
  if (canonical !== requested) {
    throw new Error("RADROOTS_LIB_SOURCE_ROOT must not traverse symlinks");
  }
  return canonical;
}

export function artifactRoute(name) {
  const route = ARTIFACT_ROUTES[name];
  if (!route) {
    throw new Error(`unsupported SDK artifact route: ${name ?? "<missing>"}`);
  }
  return route;
}

export function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd,
    env: options.env,
    stdio: options.capture ? "pipe" : "inherit",
    encoding: options.capture ? "utf8" : undefined,
  });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new Error(`${command} exited with status ${result.status ?? "unknown"}`);
  }
  return options.capture ? result.stdout.trim() : "";
}

export function sourceDateEpoch(sourceRoot, revision, env) {
  const value = run(
    "git",
    ["show", "-s", "--format=%ct", revision],
    { cwd: sourceRoot, env, capture: true },
  );
  if (!/^[0-9]+$/.test(value)) {
    throw new Error("canonical source commit timestamp is invalid");
  }
  return value;
}
