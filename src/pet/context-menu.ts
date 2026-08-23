import { invoke } from "@tauri-apps/api/core";
import { Menu } from "@tauri-apps/api/menu";

export async function attachPetContextMenu(target: HTMLElement): Promise<() => void> {
  const menu = await Menu.new({
    items: [
      {
        id: "pet-gamcha",
        text: "GAMCHA!",
        action: () => void invoke("show_utility_window", { label: "gamcha" }),
      },
      { item: "Separator" },
      {
        id: "pet-timer",
        text: "타이머 표시",
        action: () => void invoke("show_utility_window", { label: "timer" }),
      },
      {
        id: "pet-focus-start",
        text: "집중 시작",
        action: () => void invoke("start_focus").then(
          () => invoke("show_utility_window", { label: "timer" }),
        ),
      },
      {
        id: "pet-timer-pause",
        text: "일시정지",
        action: () => void invoke("pause_timer"),
      },
      {
        id: "pet-timer-resume",
        text: "재개",
        action: () => void invoke("resume_timer"),
      },
      {
        id: "pet-timer-stop",
        text: "타이머 중지",
        action: () => void invoke("stop_timer"),
      },
      { item: "Separator" },
      {
        id: "pet-settings",
        text: "설정",
        action: () => void invoke("show_utility_window", { label: "settings" }),
      },
      { item: "Separator" },
      {
        id: "pet-emergency-stop",
        text: "긴급 중지",
        action: () => void invoke("emergency_stop"),
      },
      {
        id: "pet-resume",
        text: "펫 다시 시작",
        action: () => void invoke("resume_pet"),
      },
      { item: "Separator" },
      {
        id: "pet-quit",
        text: "종료",
        action: () => void invoke("quit_application"),
      },
    ],
  });

  const openMenu = (event: MouseEvent): void => {
    event.preventDefault();
    void menu.popup();
  };
  target.addEventListener("contextmenu", openMenu);

  return () => {
    target.removeEventListener("contextmenu", openMenu);
    void menu.close();
  };
}
