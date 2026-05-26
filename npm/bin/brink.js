#!/usr/bin/env node
"use strict";

const { spawnSync } = require("child_process");
const { resolveBinary, platformId } = require("../lib/resolve-binary");

const bin = resolveBinary();
if (!bin) {
  console.error(
    `brink-git: native binary not found for ${platformId()}`
  );
  console.error("");
  console.error("Options:");
  console.error("  1. cargo build --release  (from the repo root, then reinstall)");
  console.error("  2. npm run build:binaries (maintainer: copies binary into npm/binaries/)");
  console.error("  3. export BRINK_BIN=/path/to/brink");
  process.exit(1);
}

const result = spawnSync(bin, process.argv.slice(2), { stdio: "inherit" });
if (result.error) {
  console.error(`brink-git: failed to run ${bin}: ${result.error.message}`);
  process.exit(1);
}
process.exit(result.status === null ? 1 : result.status);
