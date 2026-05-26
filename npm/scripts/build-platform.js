"use strict";

const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const { PLATFORMS } = require("./platforms");

const platformId = process.argv[2];
if (!platformId) {
  console.error("Usage: node build-platform.js <platform-id>");
  console.error("IDs:", PLATFORMS.map((p) => p.id).join(", "));
  process.exit(1);
}

const platform = PLATFORMS.find((p) => p.id === platformId);
if (!platform) {
  console.error(`Unknown platform id: ${platformId}`);
  process.exit(1);
}

const npmRoot = path.join(__dirname, "..");
const repoRoot = path.join(npmRoot, "..");
const profile = process.env.BRINK_BUILD_PROFILE || "release";
const destDir = path.join(npmRoot, "platforms", platform.id);
const dest = path.join(destDir, platform.binary);

fs.mkdirSync(destDir, { recursive: true });

const buildArgs = ["build", `--${profile}`, "--target", platform.target];
execSync(`cargo ${buildArgs.join(" ")}`, { cwd: repoRoot, stdio: "inherit" });

const built = path.join(
  repoRoot,
  "target",
  platform.target,
  profile,
  platform.binary
);
if (!fs.existsSync(built)) {
  console.error(`Build output not found: ${built}`);
  process.exit(1);
}

fs.copyFileSync(built, dest);
if (platform.binary === "brink") {
  fs.chmodSync(dest, 0o755);
}

console.log(`Built ${platform.target} -> ${dest}`);
