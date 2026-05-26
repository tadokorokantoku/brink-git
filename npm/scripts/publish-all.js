"use strict";

const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const { PLATFORMS, optionalPackageName } = require("./platforms");

const npmRoot = path.join(__dirname, "..");
const dryRun = process.argv.includes("--dry-run");
const skipPlatforms = process.argv.includes("--skip-platforms");
const onlyBuilt = process.argv.includes("--only-built");
const flags = dryRun ? "--dry-run" : "";

function run(cmd, cwd) {
  execSync(cmd, { cwd, stdio: "inherit", env: process.env });
}

function binaryPath(platform) {
  return path.join(npmRoot, "platforms", platform.id, platform.binary);
}

function assertBinary(platform) {
  const bin = binaryPath(platform);
  if (!fs.existsSync(bin)) {
    if (onlyBuilt) {
      return false;
    }
    throw new Error(
      `Missing ${bin}. Run: npm run build:platform -- ${platform.id}`
    );
  }
  return true;
}

execSync("node scripts/sync-version.js", { cwd: npmRoot, stdio: "inherit" });

if (!skipPlatforms) {
  for (const p of PLATFORMS) {
    if (!assertBinary(p)) {
      console.log(`Skipping ${optionalPackageName(p.id)} (no binary)`);
      continue;
    }
    const dir = path.join(npmRoot, "platforms", p.id);
    console.log(`\nPublishing ${optionalPackageName(p.id)}...`);
    run(`npm publish ${flags}`.trim(), dir);
  }
}

console.log("\nPublishing brink-git...");
run(`npm publish ${flags}`.trim(), npmRoot);

console.log(dryRun ? "\nDry run complete." : "\nPublish complete.");
