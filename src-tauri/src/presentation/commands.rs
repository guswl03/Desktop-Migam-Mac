use std::sync::atomic::Ordering;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::{app_state::AppState, domain::settings::Settings};

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
pub fn save_settings(state: State<'_, AppState>, settings: Settings) -> Result<Settings, String> {
    let normalized = settings.validate().map_err(|error| error.to_string())?;
    state.settings_service.save(&normalized)?;
    *state
        .settings
        .write()
        .map_err(|_| "settings state is unavailable".to_owned())? = normalized.clone();
    Ok(normalized)
}

#[tauri::command]
pub fn emergency_stop(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    state.emergency_stopped.store(true, Ordering::SeqCst);
    for label in ["pet", "card"] {
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
