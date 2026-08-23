import type { AccessibilityPermissionState } from "./contracts";

export interface PermissionPresentation {
  canIntervene: boolean;
  label: string;
  showRequest: boolean;
}

export function permissionPresentation(state: AccessibilityPermissionState): PermissionPresentation {
  if (state === "granted") {
    return {
      canIntervene: true,
      label: "손쉬운 사용 권한이 허용되었습니다.",
      showRequest: false,
    };
  }
  if (state === "denied") {
    return {
      canIntervene: false,
      label: "방해 앱 감지에는 macOS 손쉬운 사용 권한이 필요합니다.",
      showRequest: true,
    };
  }
  return {
    canIntervene: false,
    label: "이 환경에서는 macOS 손쉬운 사용 기능을 확인할 수 없습니다.",
    showRequest: false,
  };
}
