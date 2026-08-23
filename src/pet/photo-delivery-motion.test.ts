import { describe, expect, it } from "vitest";
import { calculatePhotoDeliveryLayout, photoDeliveryDelayMilliseconds } from "./photo-delivery-motion";

describe("photo delivery layout", () => {
  it("uses equal 12px side insets and one centered horizontal path", () => {
    const left = calculatePhotoDeliveryLayout(1200, 800, 400, 300, true);
    const right = calculatePhotoDeliveryLayout(1200, 800, 400, 300, false);

    expect(left).toEqual({ photoX: 12, y: 250, targetX: 12, startX: -535 });
    expect(right).toEqual({ photoX: 788, y: 250, targetX: 665, startX: 1212 });
  });

  it("keeps the inset when the photo nearly fills the viewport", () => {
    expect(calculatePhotoDeliveryLayout(320, 240, 300, 240, true)).toEqual({
      photoX: 10,
      y: 0,
      targetX: 10,
      startX: -433,
    });
  });

  it("schedules automatic delivery between 20 and 40 minutes", () => {
    expect(photoDeliveryDelayMilliseconds(0)).toBe(20 * 60_000);
    expect(photoDeliveryDelayMilliseconds(0.5)).toBe(30 * 60_000);
    expect(photoDeliveryDelayMilliseconds(1)).toBe(40 * 60_000);
  });});