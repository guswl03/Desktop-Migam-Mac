import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type TimerPhase = "stopped" | "focus" | "shortBreak" | "longBreak" | "paused";

export interface TimerState {
  phase: TimerPhase;
  remainingSeconds: number;
  completedFocusSessions: number;
}

const phaseLabels: Record<TimerPhase, string> = {
  stopped: "대기",
  focus: "집중",
  shortBreak: "짧은 휴식",
  longBreak: "긴 휴식",
  paused: "일시정지",
};

export function formatRemaining(totalSeconds: number): string {
  const safeSeconds = Math.max(0, Math.floor(totalSeconds));
  const minutes = Math.floor(safeSeconds / 60);
  const seconds = safeSeconds % 60;
  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
}

export function timerControls(phase: TimerPhase): {
  start: boolean;
  pause: boolean;
  resume: boolean;
  skip: boolean;
  stop: boolean;
} {
  const running = phase === "focus" || phase === "shortBreak" || phase === "longBreak";
  return {
    start: phase === "stopped",
    pause: running,
    resume: phase === "paused",
    skip: running,
    stop: phase !== "stopped",
  };
}

export async function mountTimer(container: HTMLElement): Promise<() => void> {
  container.innerHTML = `
    <main class="timer-panel" aria-labelledby="timer-heading">
      <section class="timer-bubble">
        <button class="timer-bubble-close" type="button" data-window-command="hide" aria-label="타이머 닫기">×</button>
        <div class="timer-readout">
          <p id="timer-phase" class="timer-phase" aria-live="polite">불러오는 중</p>
          <p id="timer-remaining" class="timer-remaining" aria-label="남은 시간">--:--</p>
        </div>
        <h1 id="timer-heading" class="sr-only">뽀모도로 집중 타이머</h1>
        <span id="timer-error" class="timer-error" role="alert"></span>
      </section>
    </main>
  `;

  const phase = container.querySelector<HTMLElement>("#timer-phase");
  const remaining = container.querySelector<HTMLElement>("#timer-remaining");
  const sessions = container.querySelector<HTMLElement>("#timer-sessions");
  const error = container.querySelector<HTMLElement>("#timer-error");
  const buttons = Array.from(container.querySelectorAll<HTMLButtonElement>("[data-command]"));
  container.querySelector<HTMLButtonElement>("[data-window-command='hide']")?.addEventListener(
    "click",
    () => void invoke("hide_utility_window", { label: "timer" }),
  );
  let disposed = false;
  let requestInFlight = false;

  const render = (state: TimerState): void => {
    if (disposed) return;
    container.dataset.timerPhase = state.phase;
    if (phase) phase.textContent = phaseLabels[state.phase];
    if (remaining) remaining.textContent = formatRemaining(state.remainingSeconds);
    if (sessions) sessions.textContent = `완료한 집중 ${state.completedFocusSessions}회`;
    const controls = timerControls(state.phase);
    const availability: Record<string, boolean> = {
      start_focus: controls.start,
      pause_timer: controls.pause,
      resume_timer: controls.resume,
      skip_phase: controls.skip,
      stop_timer: controls.stop,
    };
    for (const button of buttons) {
      const available = availability[button.dataset.command ?? ""];
      button.hidden = !available;
      button.disabled = requestInFlight;
    }
  };

  let currentState = await invoke<TimerState>("get_timer_state");
  render(currentState);

  const runCommand = async (command: string): Promise<void> => {
    if (requestInFlight) return;
    requestInFlight = true;
    if (error) error.textContent = "";
    render(currentState);
    try {
      currentState = await invoke<TimerState>(command);
      render(currentState);
    } catch {
      if (error) error.textContent = "타이머를 제어하지 못했습니다.";
    } finally {
      requestInFlight = false;
      render(currentState);
    }
  };

  for (const button of buttons) {
    button.addEventListener("click", () => void runCommand(button.dataset.command ?? ""));
  }

  const unlisten = await listen<TimerState>("timer://state", ({ payload }) => {
    currentState = payload;
    render(currentState);
  });
  const pollTimer = window.setInterval(async () => {
    if (disposed || requestInFlight) return;
    try {
      currentState = await invoke<TimerState>("get_timer_state");
      render(currentState);
    } catch {
      if (error) error.textContent = "타이머 상태를 불러오지 못했습니다.";
    }
  }, 500);
  const positionTimer = window.setInterval(() => {
    void invoke("position_timer_bubble");
  }, 250);

  return () => {
    disposed = true;
    window.clearInterval(pollTimer);
    window.clearInterval(positionTimer);
    unlisten();
  };
}
