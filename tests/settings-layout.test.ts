import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const styles = readFileSync(new URL("../src/styles.css", import.meta.url), "utf8");

describe("settings debugger layout", () => {
  it("keeps the command line flush with the document edges", () => {
    expect(styles).toMatch(
      /\.settings-panel \.debug-document > \.debug-command-line\s*\{[^}]*margin-right:\s*0;[^}]*margin-left:\s*0;/s,
    );
  });
});