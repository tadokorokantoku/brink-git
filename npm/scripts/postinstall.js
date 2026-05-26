"use strict";

const { resolveBinary, platformId, optionalPackageName } = require("../lib/resolve-binary");

const bin = resolveBinary();
if (bin) {
  process.exit(0);
}

console.warn(
  `[brink-git] Native binary for ${platformId()} is not installed.`
);
console.warn(
  `[brink-git] Expected optional package: ${optionalPackageName(platformId())}`
);
console.warn(
  "[brink-git] Install from source: cargo install --path <brink-repo>"
);
console.warn("[brink-git] Or set BRINK_BIN=/path/to/brink");
