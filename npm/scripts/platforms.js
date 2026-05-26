"use strict";

/** @type {readonly { id: string, target: string, binary: string, os: string[], cpu: string[] }[]} */
const PLATFORMS = [
  {
    id: "darwin-arm64",
    target: "aarch64-apple-darwin",
    binary: "brink",
    os: ["darwin"],
    cpu: ["arm64"],
  },
  {
    id: "darwin-x64",
    target: "x86_64-apple-darwin",
    binary: "brink",
    os: ["darwin"],
    cpu: ["x64"],
  },
  {
    id: "linux-x64",
    target: "x86_64-unknown-linux-gnu",
    binary: "brink",
    os: ["linux"],
    cpu: ["x64"],
  },
  {
    id: "linux-arm64",
    target: "aarch64-unknown-linux-gnu",
    binary: "brink",
    os: ["linux"],
    cpu: ["arm64"],
  },
  {
    id: "win32-x64",
    target: "x86_64-pc-windows-msvc",
    binary: "brink.exe",
    os: ["win32"],
    cpu: ["x64"],
  },
];

function optionalPackageName(platformId) {
  return `brink-git-${platformId}`;
}

function readVersionFromCargo(repoRoot) {
  const fs = require("fs");
  const path = require("path");
  const cargo = fs.readFileSync(path.join(repoRoot, "Cargo.toml"), "utf8");
  const m = cargo.match(/^version\s*=\s*"([^"]+)"/m);
  if (!m) {
    throw new Error("version not found in Cargo.toml");
  }
  return m[1];
}

module.exports = { PLATFORMS, optionalPackageName, readVersionFromCargo };
