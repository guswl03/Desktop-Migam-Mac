import { describe, expect, it } from "vitest";
import { costumeDragAlignment, GAMCHA_NOTICE_DURATION_MILLISECONDS, isGamchaRarity, rarityLabel, rouletteDelay } from "./gamcha-model";

describe("GAMCHA view model", () => {
  it("labels every drawable rarity", () => {
    expect(rarityLabel("common")).toBe("COMMON");
    expect(rarityLabel("special")).toBe("SPECIAL");
    expect(isGamchaRarity("default")).toBe(false);
  });

  it("slows the roulette toward the reveal", () => {
    expect(rouletteDelay(0)).toBe(45);
    expect(rouletteDelay(12)).toBeGreaterThan(rouletteDelay(3));
    expect(rouletteDelay(100)).toBe(190);
  });
  it("converts preview dragging into clamped costume coordinates", () => {
    expect(costumeDragAlignment({ x: -4, y: -30 }, 30, -15, 3)).toEqual({ x: 6, y: -35 });
    expect(costumeDragAlignment({ x: 75, y: -75 }, 30, -30, 3)).toEqual({ x: 80, y: -80 });
  });

  it("keeps a ticket notification visible for thirty seconds", () => {
    expect(GAMCHA_NOTICE_DURATION_MILLISECONDS).toBe(30_000);
  });
});
