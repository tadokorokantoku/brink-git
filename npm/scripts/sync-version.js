"use strict";

const fs = require("fs");
const path = require("path");
const { PLATFORMS, optionalPackageName, readVersionFromCargo } = require("./platforms");

const npmRoot = path.join(__dirname, "..");
const repoRoot = path.join(npmRoot, "..");
const version = process.argv[2] || readVersionFromCargo(repoRoot);

function writeJson(filePath, data) {
  fs.writeFileSync(filePath, `${JSON.stringify(data, null, 2)}\n`);
}

const mainPkgPath = path.join(npmRoot, "package.json");
const mainPkg = JSON.parse(fs.readFileSync(mainPkgPath, "utf8"));
mainPkg.version = version;

const optionalDependencies = {};
for (const p of PLATFORMS) {
  optionalDependencies[optionalPackageName(p.id)] = version;
}
mainPkg.optionalDependencies = optionalDependencies;
writeJson(mainPkgPath, mainPkg);

for (const p of PLATFORMS) {
  const dir = path.join(npmRoot, "platforms", p.id);
  fs.mkdirSync(dir, { recursive: true });
  writeJson(path.join(dir, "package.json"), {
    name: optionalPackageName(p.id),
    version,
    description: `Native brink binary for ${p.id}`,
    license: "MIT",
    repository: mainPkg.repository,
    os: p.os,
    cpu: p.cpu,
    files: [p.binary],
    preferUnplugged: true,
  });
}

console.log(`Synced npm version to ${version} (${PLATFORMS.length} platform packages)`);
