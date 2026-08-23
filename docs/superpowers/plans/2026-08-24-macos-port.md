# migam desktop macOS Port Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Convert the existing Windows-only Tauri desktop pet into a macOS 14+ Apple Silicon application while preserving its features and implementing safe Accessibility-based distraction-window minimization.

**Architecture:** Keep the existing domain and application services, replace Win32 infrastructure with focused macOS adapters, and expose only platform capability state through the current Tauri command boundary. Native behavior fails closed: missing Accessibility permission or uncertain window identity disables intervention without affecting the rest of the app.

**Tech Stack:** Tauri 2.8, Rust 2021, objc2/ApplicationServices FFI, sysinfo, Vite 7, TypeScript 5.9, Vitest 3

**Spec:** `docs/superpowers/specs/2026-08-24-macos-port-design.md`

## Global Constraints

- Target only Apple Silicon M2 or newer with `aarch64-apple-darwin`.
- Require macOS 14.0 or newer.
- Preserve Tauri 2 + Vite + plain TypeScript + DOM API; do not add React.
- Intervention remains disabled by default.
- Never minimize unless the post-warning snapshot revalidates the same target.
- Missing permission, unsupported AX attributes, protected targets, or uncertain identity must skip intervention safely.
- Do not execute AppleScript, synthesize keys, terminate apps, or hide whole applications.
- Do not store secrets or signing credentials in the repository.
- Do not publish releases, sign with a Developer ID, notarize, commit, or push without separate authorization.

---

### Task 1: Make the native project macOS-only

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/src/infrastructure/mod.rs`
- Modify: `src-tauri/src/app_state.rs`
- Create: `src-tauri/tauri.macos.conf.json`
- Modify: `src-tauri/tauri.conf.json`

**Interfaces:**
- Consumes: existing `ForegroundWindowSource`, `WindowMinimizer`, and `SystemMetricsMonitor` consumers
- Produces: `infrastructure::macos::{PlatformForegroundWindowSource, PlatformWindowMinimizer, SystemMetricsMonitor}` and macOS bundle configuration

- [ ] Remove the Windows GUI subsystem attribute and Win32-only Cargo dependency.
- [ ] Add macOS-native dependencies only under `cfg(target_os = "macos")` and add `sysinfo` for host metrics.
- [ ] Point application state at `infrastructure::macos`.
- [ ] Remove NSIS-only configuration from the common config and create a macOS override with `minimumSystemVersion: "14.0"` and `targets: ["app", "dmg"]`.
- [ ] Run `cargo check --manifest-path src-tauri/Cargo.toml` and confirm platform-independent code still compiles on the current host or capture the expected macOS-only boundary.

### Task 2: Generalize foreground target identity and safety rules

**Files:**
- Modify: `src-tauri/src/domain/foreground.rs`
- Modify: `src-tauri/src/domain/distraction.rs`
- Modify: `src-tauri/src/application/foreground_monitor.rs`

**Interfaces:**
- Produces: `WindowTargetId { process_id: u32, fingerprint: u64 }`
- Produces: snapshots with `bundle_id: Option<String>`
- Consumes: the same target identity in `WindowMinimizer::minimize(&WindowTargetId)`

- [ ] Add failing unit tests proving bundle-ID protection and rejection when target identity changes.
- [ ] Replace `isize` HWND identity with an opaque, cloneable macOS-safe target ID.
- [ ] Include bundle ID in snapshots and protected-target evaluation.
- [ ] Update fake sources/minimizers and run the focused Rust tests.

### Task 3: Implement Accessibility permission and macOS foreground-window adapters

**Files:**
- Create: `src-tauri/src/infrastructure/macos/mod.rs`
- Create: `src-tauri/src/infrastructure/macos/accessibility.rs`
- Create: `src-tauri/src/infrastructure/macos/foreground_window.rs`
- Modify: `src-tauri/src/app_state.rs`
- Modify: `src-tauri/src/presentation/commands.rs`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Produces: `AccessibilityPermissionState::{Granted, Denied, Unavailable}` serialized as camel-case text
- Produces: `AccessibilityPermissionService::status(prompt: bool)`
- Produces: `PlatformForegroundWindowSource::foreground_window()`
- Produces: `PlatformWindowMinimizer::minimize(&WindowTargetId)`
- Produces: Tauri commands `get_accessibility_permission` and `request_accessibility_permission`

- [ ] Add permission-state tests around a fake trust probe.
- [ ] Bind `AXIsProcessTrustedWithOptions`, obtain the frontmost `NSRunningApplication`, and inspect its focused AX window.
- [ ] Read title, position, size, minimized state, bundle ID, and full-screen geometry into `WindowSnapshot`.
- [ ] Re-read the current target during minimization and set only `kAXMinimizedAttribute` after identity validation.
- [ ] Return `AccessDenied` or `InspectionFailed` without fallback automation.
- [ ] Register permission commands and run Rust tests.

### Task 4: Replace Windows resource metrics

**Files:**
- Create: `src-tauri/src/infrastructure/macos/system_metrics.rs`
- Delete: `src-tauri/src/infrastructure/windows/system_metrics.rs`
- Delete: `src-tauri/src/infrastructure/windows/foreground_window.rs`
- Delete: `src-tauri/src/infrastructure/windows/mod.rs`

**Interfaces:**
- Produces: unchanged `SystemMetricsSnapshot { cpu_percent: u8, memory_percent: u8 }`
- Produces: unchanged `SystemMetricsMonitor::poll()` with a 750ms cache

- [ ] Add a focused test that values remain between 0 and 100.
- [ ] Use `sysinfo::System` refresh methods for global CPU and used/total memory.
- [ ] Preserve smoothing and cache behavior required by the existing pet runtime.
- [ ] Run Rust tests and Clippy.

### Task 5: Add permission-aware macOS settings UI

**Files:**
- Modify: `src/contracts.ts`
- Modify: `src/main.ts`
- Modify: `src/settings-help.ts`
- Modify: `src/styles.css`

**Interfaces:**
- Consumes: `BootstrapState.accessibilityPermission`
- Consumes: `request_accessibility_permission`
- Produces: a visible permission status, request button, and disabled intervention toggle when permission is absent

- [ ] Add TypeScript unit coverage for permission-state presentation logic in a focused helper module.
- [ ] Rename executable terminology to macOS app/bundle terminology and use examples such as `com.google.Chrome`.
- [ ] Render permission status and a request button in Focus Guard settings.
- [ ] Keep saved intervention off until permission becomes granted.
- [ ] Replace `Ctrl+Shift+F12` text with `Command+Shift+F12`.
- [ ] Run Vitest, typecheck, and Vite production build.

### Task 6: Configure the macOS shortcut, icon, and CI validation

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Create: `.github/workflows/macos.yml`
- Modify: `README.md`
- Modify: `docs/11-test-plan.md`

**Interfaces:**
- Produces: global shortcut `Command+Shift+F12`
- Produces: macOS CI job for test, format check, Clippy, frontend build, and `aarch64-apple-darwin` Tauri build

- [ ] Register the global shortcut only with macOS Command modifier.
- [ ] Add a macOS runner workflow that never publishes and uses ad-hoc signing for build validation.
- [ ] Document Node, Rust, Xcode command-line tools, Accessibility permission, development run, and DMG build commands.
- [ ] Record the manual M2/M3 test matrix without claiming it has passed.
- [ ] Run every locally available verification command.

### Task 7: Reconcile project tracking and handoff

**Files:**
- Modify: `docs/01-scope-and-priorities.md`
- Modify: `docs/13-progress-board.md`
- Modify: `docs/17-session-handoff.md`

**Interfaces:**
- Produces: documentation that names macOS as the active target and distinguishes automated Windows-host validation from pending real-Mac checks

- [ ] Remove the stale statement that macOS is excluded and record the approved Apple Silicon/macOS 14 scope.
- [ ] Record exact verification outputs and failures.
- [ ] Set completion only for gates actually passed.
- [ ] Leave exact M2/M3 manual checks as pending handoff work.
