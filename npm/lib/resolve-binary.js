"use strict";

const fs = require("fs");
const path = require("path");
const { optionalPackageName } = require("../scripts/platforms");

function platformId() {
  const arch =
    process.arch === "x64"
      ? "x64"
      : process.arch === "arm64"
        ? "arm64"
        : process.arch;
  return `${process.platform}-${arch}`;
}

function resolveFromOptionalDependency() {
  const id = platformId();
  const pkg = optionalPackageName(id);
  try {
    const pkgJson = require.resolve(`${pkg}/package.json`);
    const dir = path.dirname(pkgJson);
    const win = process.platform === "win32";
    const candidate = path.join(dir, win ? "brink.exe" : "brink");
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  } catch {
    // optional dependency not installed for this platform
  }
  return null;
}

function resolveBinary() {
  if (process.env.BRINK_BIN) {
    return process.env.BRINK_BIN;
  }

  const fromOptional = resolveFromOptionalDependency();
  if (fromOptional) {
    return fromOptional;
  }

  const pkgRoot = path.join(__dirname, "..");
  const id = platformId();
  const win = process.platform === "win32";
  const fromPlatformsDir = path.join(
    pkgRoot,
    "platforms",
    id,
    win ? "brink.exe" : "brink"
  );
  if (fs.existsSync(fromPlatformsDir)) {
    return fromPlatformsDir;
  }

  const vendored = path.join(pkgRoot, "binaries", `brink-${platformId()}`);
  if (fs.existsSync(vendored)) {
    return vendored;
  }

  const repoRoot = path.join(pkgRoot, "..");
  const binName = win ? "brink.exe" : "brink";
  for (const profile of ["release", "debug"]) {
    const candidate = path.join(repoRoot, "target", profile, binName);
    if (fs.existsSync(candidate)) {
      return candidate;
    }
    const hostTarget = path.join(repoRoot, "target", profile, "brink");
    if (fs.existsSync(hostTarget)) {
      return hostTarget;
    }
  }

  return null;
}

module.exports = { resolveBinary, platformId, optionalPackageName };
