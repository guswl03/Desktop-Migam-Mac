import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import gamjabotSprite from "../images/characters/gamjabot/pack/idle/0.png";
import type { BootstrapState, Settings } from "./contracts";
import "./styles.css";

const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("The application root is missing.");
}

const windowLabel = getCurrentWindow().label;
document.body.dataset.window = windowLabel;

function renderPet(): void {
  app!.innerHTML = `
    <main class="pet-shell" aria-label="Gamjabot desktop pet">
      <img class="pet-placeholder" src="${gamjabotSprite}" alt="Gamjabot desktop pet" draggable="false" />
    </main>
  `;
}

function renderTimer(): void {
  app!.innerHTML = `
    <main class="panel">
      <p class="eyebrow">집중 타이머</p>
      <h1>뽀모도로</h1>
      <p class="muted">타이머 제어는 다음 구현 단계에서 연결됩니다.</p>
    </main>
  `;
}

function numberValue(form: FormData, name: string): number {
  return Number(form.get(name));
}

function renderSettings(
  settings: Settings,
  emergencyShortcutAvailable: boolean,
  trayAvailable: boolean,
): void {
  app!.innerHTML = `
    <main class="panel settings-panel">
      <p class="eyebrow">Desktop Pet MVP</p>
      <h1>설정</h1>
      ${emergencyShortcutAvailable ? "" : '<p class="warning" role="alert">Ctrl+Shift+F12 긴급 중지 단축키를 등록하지 못했습니다. 트레이의 긴급 중지 메뉴를 사용해 주세요.</p>'}
      ${trayAvailable ? "" : '<p class="warning" role="alert">시스템 트레이를 사용할 수 없습니다. 앱 창을 닫으면 복구 메뉴에 접근하지 못할 수 있습니다.</p>'}
      <form id="settings-form">
        <fieldset>
          <legend>펫</legend>
          <label>크기 (%)<input name="visualScalePercent" type="number" min="50" max="200" value="${settings.pet.visualScalePercent}" /></label>
        </fieldset>
        <fieldset>
          <legend>뽀모도로</legend>
          <label>집중 시간<input name="focusMinutes" type="number" min="1" max="120" value="${settings.pomodoro.focusMinutes}" /></label>
          <label>짧은 휴식<input name="shortBreakMinutes" type="number" min="1" max="60" value="${settings.pomodoro.shortBreakMinutes}" /></label>
          <label>긴 휴식<input name="longBreakMinutes" type="number" min="1" max="90" value="${settings.pomodoro.longBreakMinutes}" /></label>
          <label>긴 휴식 주기<input name="sessionsBeforeLongBreak" type="number" min="1" max="12" value="${settings.pomodoro.sessionsBeforeLongBreak}" /></label>
        </fieldset>
        <fieldset>
          <legend>집중 보호</legend>
          <label class="checkbox-row"><input name="interventionEnabled" type="checkbox" ${settings.focusGuard.interventionEnabled ? "checked" : ""} disabled /> 안전 규칙을 추가한 뒤 활성화할 수 있습니다.</label>
          <p class="muted">창 개입은 기본적으로 꺼져 있으며 현재 등록된 규칙은 ${settings.focusGuard.rules.length}개입니다.</p>
        </fieldset>
        <div class="actions"><button type="submit">저장</button><span id="save-status" role="status"></span></div>
      </form>
    </main>
  `;

  const form = document.querySelector<HTMLFormElement>("#settings-form");
  const status = document.querySelector<HTMLSpanElement>("#save-status");
  form?.addEventListener("submit", async (event) => {
    event.preventDefault();
    const values = new FormData(form);
    const next: Settings = {
      ...settings,
      pet: { visualScalePercent: numberValue(values, "visualScalePercent") },
      pomodoro: {
        focusMinutes: numberValue(values, "focusMinutes"),
        shortBreakMinutes: numberValue(values, "shortBreakMinutes"),
        longBreakMinutes: numberValue(values, "longBreakMinutes"),
        sessionsBeforeLongBreak: numberValue(values, "sessionsBeforeLongBreak"),
      },
    };
    try {
      settings = await invoke<Settings>("save_settings", { settings: next });
      if (status) status.textContent = "저장했습니다.";
    } catch {
      if (status) status.textContent = "저장하지 못했습니다. 입력값을 확인해 주세요.";
    }
  });
}

async function start(): Promise<void> {
  if (windowLabel === "pet" || windowLabel === "card") {
    renderPet();
    return;
  }
  if (windowLabel === "timer") {
    renderTimer();
    return;
  }
  try {
    const bootstrap = await invoke<BootstrapState>("get_bootstrap_state");
    renderSettings(
      bootstrap.settings,
      bootstrap.emergencyShortcutAvailable,
      bootstrap.trayAvailable,
    );
  } catch {
    app!.innerHTML = `<main class="panel"><h1>설정을 불러오지 못했습니다.</h1><p class="muted">앱을 다시 시작해 주세요.</p></main>`;
  }
}

void start();
