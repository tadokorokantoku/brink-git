"use strict";

const { execSync } = require("child_process");
const path = require("path");
const { PLATFORMS } = require("./platforms");

const only = process.argv.slice(2);
const list = only.length
  ? PLATFORMS.filter((p) => only.includes(p.id))
  : PLATFORMS;

if (only.length && list.length !== only.length) {
  const known = new Set(PLATFORMS.map((p) => p.id));
  const missing = only.filter((id) => !known.has(id));
  console.error(`Unknown platform id(s): ${missing.join(", ")}`);
  process.exit(1);
}

execSync("node scripts/sync-version.js", {
  cwd: path.join(__dirname, ".."),
  stdio: "inherit",
});

for (const p of list) {
  execSync(`node scripts/build-platform.js ${p.id}`, {
    cwd: path.join(__dirname, ".."),
    stdio: "inherit",
  });
}

console.log(`Built ${list.length} platform(s).`);
