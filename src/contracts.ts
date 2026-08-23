export interface DistractionRule {
  processName?: string;
  windowTitle?: string;
}

export interface Settings {
  schemaVersion: number;
  pet: { visualScalePercent: number };
  pomodoro: {
    focusMinutes: number;
    shortBreakMinutes: number;
    longBreakMinutes: number;
    sessionsBeforeLongBreak: number;
  };
  focusGuard: {
    interventionEnabled: boolean;
    rules: DistractionRule[];
  };
}

export interface BootstrapState {
  settings: Settings;
  emergencyStopped: boolean;
  emergencyShortcutAvailable: boolean;
  trayAvailable: boolean;
}
