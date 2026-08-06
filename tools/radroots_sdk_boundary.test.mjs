import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function rustSources(path, found = []) {
  for (const entry of readdirSync(path, { withFileTypes: true })) {
    const child = join(path, entry.name);
    if (entry.isDirectory()) {
      rustSources(child, found);
    } else if (entry.isFile() && entry.name.endsWith(".rs")) {
      found.push(relative(root, child));
    }
  }
  return found.sort();
}

test("the Rust workspace is an exact zero-logic source-lock capsule", () => {
  assert.deepEqual(rustSources(join(root, "crates")), [
    "crates/source_lock/src/lib.rs",
  ]);
  assert.equal(
    readFileSync(join(root, "crates/source_lock/src/lib.rs"), "utf8"),
    "#![no_std]\n",
  );
  assert.deepEqual(rustSources(join(root, "tools")), []);
  const workspace = readFileSync(join(root, "Cargo.toml"), "utf8");
  const capsule = readFileSync(join(root, "crates/source_lock/Cargo.toml"), "utf8");
  assert.match(workspace, /^members = \["crates\/source_lock"\]$/m);
  assert.match(capsule, /^publish = false$/m);
  assert.doesNotMatch(workspace, /path\s*=/);
  assert.doesNotMatch(capsule, /path\s*=/);
});

test("the source-lock dependency is immutable and exact", () => {
  const workspace = readFileSync(join(root, "Cargo.toml"), "utf8");
  assert.match(workspace, /git = "https:\/\/github\.com\/radrootslabs\/lib"/);
  assert.match(workspace, /rev = "[0-9a-f]{40}"/);
  assert.match(workspace, /version = "=0\.1\.0-alpha"/);
  assert.doesNotMatch(workspace, /branch\s*=|tag\s*=/);
});
