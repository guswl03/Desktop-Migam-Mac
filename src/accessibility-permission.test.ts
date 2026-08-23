import { describe, expect, it } from "vitest";
import { permissionPresentation } from "./accessibility-permission";

describe("permissionPresentation", () => {
  it("allows intervention only after Accessibility permission is granted", () => {
    expect(permissionPresentation("granted")).toEqual({
      canIntervene: true,
      label: "손쉬운 사용 권한이 허용되었습니다.",
      showRequest: false,
    });
    expect(permissionPresentation("denied")).toEqual({
      canIntervene: false,
      label: "방해 앱 감지에는 macOS 손쉬운 사용 권한이 필요합니다.",
      showRequest: true,
    });
    expect(permissionPresentation("unavailable")).toEqual({
      canIntervene: false,
      label: "이 환경에서는 macOS 손쉬운 사용 기능을 확인할 수 없습니다.",
      showRequest: false,
    });
  });
});
