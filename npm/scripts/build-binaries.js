"use strict";

const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const { platformId } = require("../lib/resolve-binary");

const repoRoot = path.join(__dirname, "..", "..");
const profile = process.env.BRINK_BUILD_PROFILE || "release";
const outDir = path.join(__dirname, "..", "binaries");

fs.mkdirSync(outDir, { recursive: true });

execSync(`cargo build --${profile}`, {
  cwd: repoRoot,
  stdio: "inherit",
});

const built = path.join(repoRoot, "target", profile, "brink");
const dest = path.join(outDir, `brink-${platformId()}`);
fs.copyFileSync(built, dest);
fs.chmodSync(dest, 0o755);

console.log(`Copied ${built} -> ${dest}`);
