use std::{sync::atomic::Ordering, time::Instant};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::{
    app_state::AppState,
    application::{
        foreground_monitor::DetectionState,
        gamcha_service::{CostumeAlignment, GamchaDrawResult, GamchaSnapshot},
        pomodoro_service::TimerState,
    },
    domain::{
        pomodoro::{PomodoroEvent, PomodoroPhase},
        settings::{ResourceResponseMode, Settings},
    },
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapState {
    settings: Settings,
    emergency_stopped: bool,
    emergency_shortcut_available: bool,
    tray_available: bool,
}

#[tauri::command]
pub fn get_bootstrap_state(state: State<'_, AppState>) -> Result<BootstrapState, String> {
    let settings = state
        .settings
        .read()
        .map_err(|_| "settings state is unavailable".to_owned())?
        .clone();
    Ok(BootstrapState {
        settings,
        emergency_stopped: state.emergency_stopped.load(Ordering::SeqCst),
        emergency_shortcut_available: state.emergency_shortcut_available.load(Ordering::SeqCst),
        tray_available: state.tray_available.load(Ordering::SeqCst),
    })
}

#[tauri::command]
pub fn save_settings(
    app: AppHandle,
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<Settings, String> {
    let normalized = settings.validate().map_err(|error| error.to_string())?;
    state.settings_service.save(&normalized)?;
    state
        .pomodoro_service
        .update_settings(&normalized.pomodoro)?;
    *state
        .settings
        .write()
        .map_err(|_| "settings state is unavailable".to_owned())? = normalized.clone();
    let _ = app.emit("settings://saved", &normalized);
    Ok(normalized)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMetricsState {
    cpu_percent: u8,
    memory_percent: u8,
    mode: ResourceResponseMode,
}

#[tauri::command]
pub fn get_system_metrics(state: State<'_, AppState>) -> Result<SystemMetricsState, String> {
    let metrics = state.system_metrics_monitor.poll()?;
    let mode = state
        .settings
        .read()
        .map_err(|_| "settings state is unavailable".to_owned())?
        .pet
        .resource_response_mode;
    Ok(SystemMetricsState {
        cpu_percent: metrics.cpu_percent,
        memory_percent: metrics.memory_percent,
        mode,
    })
}

fn dispatch_timer(
    app: &AppHandle,
    state: &AppState,
    event: PomodoroEvent,
) -> Result<TimerState, String> {
    if event == PomodoroEvent::Tick {
        return tick_timer(app, state);
    }
    let (snapshot, changed) = state.pomodoro_service.dispatch(event, Instant::now())?;
    if changed {
        app.emit("timer://state", &snapshot)
            .map_err(|_| "timer state notification failed".to_owned())?;
    }
    Ok(snapshot)
}

pub(crate) fn tick_timer(app: &AppHandle, state: &AppState) -> Result<TimerState, String> {
    let (snapshot, changed, focus_completed) = state.pomodoro_service.tick(Instant::now())?;
    if changed {
        let _ = app.emit("timer://state", &snapshot);
    }
    if focus_completed {
        let gamcha = state.gamcha_service.award_ticket()?;
        let _ = app.emit("gamcha://ticket-earned", &gamcha);
        let _ = show_gamcha_reward(app);
    }
    Ok(snapshot)
}

#[tauri::command]
pub fn get_timer_state(app: AppHandle, state: State<'_, AppState>) -> Result<TimerState, String> {
    dispatch_timer(&app, &state, PomodoroEvent::Tick)
}

#[tauri::command]
pub fn get_detection_state(state: State<'_, AppState>) -> Result<DetectionState, String> {
    state.foreground_monitor.state()
}

#[tauri::command]
pub fn get_gamcha_state(state: State<'_, AppState>) -> Result<GamchaSnapshot, String> {
    state.gamcha_service.snapshot()
}

#[tauri::command]
pub fn draw_gamcha(state: State<'_, AppState>) -> Result<GamchaDrawResult, String> {
    state.gamcha_service.draw()
}

#[tauri::command]
pub fn equip_gamcha_costume(
    app: AppHandle,
    state: State<'_, AppState>,
    costume_id: Option<String>,
) -> Result<GamchaSnapshot, String> {
    let snapshot = state.gamcha_service.equip(costume_id)?;
    let _ = app.emit("gamcha://equipped", &snapshot);
    Ok(snapshot)
}

#[tauri::command]
pub fn set_gamcha_costume_alignment(
    app: AppHandle,
    state: State<'_, AppState>,
    costume_id: String,
    alignment: Option<CostumeAlignment>,
) -> Result<GamchaSnapshot, String> {
    let snapshot = state.gamcha_service.set_alignment(costume_id, alignment)?;
    let _ = app.emit("gamcha://equipped", &snapshot);
    Ok(snapshot)
}

#[tauri::command]
pub fn complete_intervention(
    app: AppHandle,
    state: State<'_, AppState>,
    intervention_id: u64,
) -> Result<bool, String> {
    let timer = dispatch_timer(&app, &state, PomodoroEvent::Tick)?;
    let settings = state
        .settings
        .read()
        .map_err(|_| "settings state is unavailable".to_owned())?
        .focus_guard
        .clone();
    let minimized = state.foreground_monitor.complete(
        intervention_id,
        Instant::now(),
        timer.phase == PomodoroPhase::Focus,
        state.emergency_stopped.load(Ordering::SeqCst),
        &settings,
    )?;
    if let Some(card) = app.get_webview_window("card") {
        let _ = card.hide();
    }
    Ok(minimized)
}

#[tauri::command]
pub fn cancel_intervention(
    app: AppHandle,
    state: State<'_, AppState>,
    intervention_id: u64,
) -> Result<(), String> {
    state.foreground_monitor.cancel(intervention_id)?;
    if let Some(card) = app.get_webview_window("card") {
        let _ = card.hide();
    }
    Ok(())
}

#[tauri::command]
pub fn start_focus(app: AppHandle, state: State<'_, AppState>) -> Result<TimerState, String> {
    dispatch_timer(&app, &state, PomodoroEvent::Start)
}

#[tauri::command]
pub fn pause_timer(app: AppHandle, state: State<'_, AppState>) -> Result<TimerState, String> {
    dispatch_timer(&app, &state, PomodoroEvent::Pause)
}

#[tauri::command]
pub fn resume_timer(app: AppHandle, state: State<'_, AppState>) -> Result<TimerState, String> {
    dispatch_timer(&app, &state, PomodoroEvent::Resume)
}

#[tauri::command]
pub fn skip_phase(app: AppHandle, state: State<'_, AppState>) -> Result<TimerState, String> {
    dispatch_timer(&app, &state, PomodoroEvent::Skip)
}

#[tauri::command]
pub fn stop_timer(app: AppHandle, state: State<'_, AppState>) -> Result<TimerState, String> {
    dispatch_timer(&app, &state, PomodoroEvent::Stop)
}

#[tauri::command]
pub fn emergency_stop(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    state.emergency_stopped.store(true, Ordering::SeqCst);
    let _ = state.foreground_monitor.cancel_all()?;
    let _ = dispatch_timer(&app, &state, PomodoroEvent::Pause)?;
    for label in ["pet", "card", "gamcha-notice", "gamcha"] {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.hide();
        }
    }
    app.emit("app://emergency-stopped", ())
        .map_err(|_| "emergency stop notification failed".to_owned())
}

#[tauri::command]
pub fn resume_pet(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    state.emergency_stopped.store(false, Ordering::SeqCst);
    if let Some(window) = app.get_webview_window("pet") {
        window
            .show()
            .map_err(|_| "pet window could not be shown".to_owned())?;
    }
    Ok(())
}

pub fn place_timer_bubble(app: &AppHandle) -> Result<(), String> {
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "pet window is unavailable".to_owned())?;
    let timer = app
        .get_webview_window("timer")
        .ok_or_else(|| "timer window is unavailable".to_owned())?;
    let pet_position = pet
        .outer_position()
        .map_err(|_| "pet position is unavailable".to_owned())?;
    let pet_size = pet
        .outer_size()
        .map_err(|_| "pet size is unavailable".to_owned())?;
    let timer_size = timer
        .outer_size()
        .map_err(|_| "timer size is unavailable".to_owned())?;
    let monitor = pet
        .current_monitor()
        .map_err(|_| "pet monitor is unavailable".to_owned())?
        .ok_or_else(|| "pet monitor is unavailable".to_owned())?;
    let work_area = monitor.work_area();

    let minimum_x = work_area.position.x;
    let minimum_y = work_area.position.y;
    let maximum_x = minimum_x + work_area.size.width as i32 - timer_size.width as i32;
    let maximum_y = minimum_y + work_area.size.height as i32 - timer_size.height as i32;
    let desired_x = pet_position.x + pet_size.width as i32 / 2 - timer_size.width as i32 / 2;
    let desired_y = pet_position.y - timer_size.height as i32 + 12;
    timer
        .set_position(tauri::PhysicalPosition::new(
            desired_x.clamp(minimum_x, maximum_x.max(minimum_x)),
            desired_y.clamp(minimum_y, maximum_y.max(minimum_y)),
        ))
        .map_err(|_| "timer bubble could not be positioned".to_owned())
}

#[tauri::command]
pub fn position_timer_bubble(app: AppHandle) -> Result<(), String> {
    place_timer_bubble(&app)
}

pub fn place_gamcha_notice_bubble(app: &AppHandle) -> Result<(), String> {
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "pet window is unavailable".to_owned())?;
    let gamcha = app
        .get_webview_window("gamcha-notice")
        .ok_or_else(|| "GAMCHA notice is unavailable".to_owned())?;
    let pet_position = pet
        .outer_position()
        .map_err(|_| "pet position is unavailable".to_owned())?;
    let pet_size = pet
        .outer_size()
        .map_err(|_| "pet size is unavailable".to_owned())?;
    let gamcha_size = gamcha
        .outer_size()
        .map_err(|_| "GAMCHA size is unavailable".to_owned())?;
    let monitor = pet
        .current_monitor()
        .map_err(|_| "pet monitor is unavailable".to_owned())?
        .ok_or_else(|| "pet monitor is unavailable".to_owned())?;
    let work_area = monitor.work_area();
    let minimum_x = work_area.position.x;
    let minimum_y = work_area.position.y;
    let maximum_x = minimum_x + work_area.size.width as i32 - gamcha_size.width as i32;
    let maximum_y = minimum_y + work_area.size.height as i32 - gamcha_size.height as i32;
    let desired_x = pet_position.x + pet_size.width as i32 / 2 - gamcha_size.width as i32 / 2;
    let desired_y = pet_position.y - gamcha_size.height as i32 + 14;
    gamcha
        .set_position(tauri::PhysicalPosition::new(
            desired_x.clamp(minimum_x, maximum_x.max(minimum_x)),
            desired_y.clamp(minimum_y, maximum_y.max(minimum_y)),
        ))
        .map_err(|_| "GAMCHA bubble could not be positioned".to_owned())
}

pub fn show_gamcha_reward(app: &AppHandle) -> Result<(), String> {
    place_gamcha_notice_bubble(app)?;
    app.get_webview_window("gamcha-notice")
        .ok_or_else(|| "GAMCHA notice is unavailable".to_owned())?
        .show()
        .map_err(|_| "GAMCHA notice could not be shown".to_owned())
}

#[tauri::command]
pub fn position_gamcha_bubble(app: AppHandle) -> Result<(), String> {
    place_gamcha_notice_bubble(&app)
}

#[tauri::command]
pub fn show_pet_context_menu(app: AppHandle, x: i32, y: i32) -> Result<(), String> {
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "pet window is unavailable".to_owned())?;
    let menu = app
        .get_webview_window("pet-menu")
        .ok_or_else(|| "pet menu is unavailable".to_owned())?;
    let menu_size = menu
        .outer_size()
        .map_err(|_| "pet menu size is unavailable".to_owned())?;
    let monitor = pet
        .current_monitor()
        .map_err(|_| "pet monitor is unavailable".to_owned())?
        .ok_or_else(|| "pet monitor is unavailable".to_owned())?;
    let work_area = monitor.work_area();
    let minimum_x = work_area.position.x;
    let minimum_y = work_area.position.y;
    let maximum_x = minimum_x + work_area.size.width as i32 - menu_size.width as i32;
    let maximum_y = minimum_y + work_area.size.height as i32 - menu_size.height as i32;
    let desired_x = if x + 10 + menu_size.width as i32 <= minimum_x + work_area.size.width as i32 {
        x + 10
    } else {
        x - menu_size.width as i32 - 10
    };
    let desired_y = if y + 10 + menu_size.height as i32 <= minimum_y + work_area.size.height as i32
    {
        y + 10
    } else {
        y - menu_size.height as i32 - 10
    };
    menu.set_position(tauri::PhysicalPosition::new(
        desired_x.clamp(minimum_x, maximum_x.max(minimum_x)),
        desired_y.clamp(minimum_y, maximum_y.max(minimum_y)),
    ))
    .map_err(|_| "pet menu could not be positioned".to_owned())?;
    menu.show()
        .map_err(|_| "pet menu could not be shown".to_owned())?;
    menu.set_focus()
        .map_err(|_| "pet menu could not be focused".to_owned())
}

fn prepare_gamcha_overlay(app: &AppHandle) -> Result<(), String> {
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "pet window is unavailable".to_owned())?;
    let gamcha = app
        .get_webview_window("gamcha")
        .ok_or_else(|| "GAMCHA window is unavailable".to_owned())?;
    let monitor = pet
        .current_monitor()
        .map_err(|_| "pet monitor is unavailable".to_owned())?
        .ok_or_else(|| "pet monitor is unavailable".to_owned())?;
    gamcha
        .set_fullscreen(false)
        .map_err(|_| "GAMCHA overlay could not be reset".to_owned())?;
    gamcha
        .set_position(*monitor.position())
        .map_err(|_| "GAMCHA monitor could not be selected".to_owned())?;
    gamcha
        .set_size(*monitor.size())
        .map_err(|_| "GAMCHA overlay could not fit the monitor".to_owned())
}

#[tauri::command]
pub fn show_utility_window(app: AppHandle, label: String) -> Result<(), String> {
    if !matches!(
        label.as_str(),
        "timer" | "settings" | "gamcha" | "gamcha-notice" | "pet-menu"
    ) {
        return Err("unsupported utility window".to_owned());
    }
    if label == "timer" {
        place_timer_bubble(&app)?;
    } else if label == "gamcha" {
        prepare_gamcha_overlay(&app)?;
        if let Some(notice) = app.get_webview_window("gamcha-notice") {
            let _ = notice.hide();
        }
    } else if label == "gamcha-notice" {
        place_gamcha_notice_bubble(&app)?;
    }
    let window = app
        .get_webview_window(&label)
        .ok_or_else(|| "utility window is unavailable".to_owned())?;
    window
        .show()
        .map_err(|_| "utility window could not be shown".to_owned())?;
    window
        .set_focus()
        .map_err(|_| "utility window could not be focused".to_owned())
}

#[tauri::command]
pub fn hide_utility_window(app: AppHandle, label: String) -> Result<(), String> {
    if !matches!(
        label.as_str(),
        "timer" | "settings" | "gamcha" | "gamcha-notice" | "pet-menu"
    ) {
        return Err("unsupported utility window".to_owned());
    }
    let result = app
        .get_webview_window(&label)
        .ok_or_else(|| "utility window is unavailable".to_owned())?
        .hide()
        .map_err(|_| "utility window could not be hidden".to_owned());
    if label == "gamcha" {
        if let Some(gamcha) = app.get_webview_window("gamcha") {
            let _ = gamcha.set_fullscreen(false);
        }
    }
    result
}

#[tauri::command]
pub fn quit_application(app: AppHandle) {
    app.exit(0);
}
