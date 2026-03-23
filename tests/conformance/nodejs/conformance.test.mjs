/**
 * Conformance tests for the Node.js binding.
 *
 * Run:
 *   cd bindings/nodejs && napi build --platform --release
 *   cd tests/conformance/nodejs && node --test
 */
import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const fixturesDir = resolve(__dirname, "../../fixtures");

// Import the native addon built by napi-rs.
const { convert } = await import("../../../bindings/nodejs/index.js");

const inputFiles = readdirSync(join(fixturesDir, "inputs"))
  .filter((f) => f.endsWith(".html"));

for (const file of inputFiles) {
  const name = file.replace(/\.html$/, "");

  test(`conformance: ${name}`, () => {
    const html     = readFileSync(join(fixturesDir, "inputs",   file),           "utf8");
    const expected = readFileSync(join(fixturesDir, "expected", `${name}.md`), "utf8").trim();

    const result = convert(html);
    assert.equal(result.markdown.trim(), expected);
  });
}
