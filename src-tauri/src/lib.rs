pub mod app_state;
pub mod application;
pub mod domain;
pub mod presentation;

use app_state::AppState;
use application::settings_service::SettingsService;
use tauri::Manager;
#[cfg(windows)]
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg(windows)]
fn emergency_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::F12)
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let settings_service = SettingsService::new(app.path().app_data_dir()?);
            let settings = settings_service.load_or_default();
            app.manage(AppState::new(settings, settings_service));
            let tray_available = presentation::tray::build(app).is_ok();
            app.state::<AppState>()
                .tray_available
                .store(tray_available, std::sync::atomic::Ordering::SeqCst);
            #[cfg(windows)]
            {
                let plugin_available = app
                    .handle()
                    .plugin(
                        tauri_plugin_global_shortcut::Builder::new()
                            .with_handler(|app, shortcut, event| {
                                if event.state() == ShortcutState::Pressed
                                    && shortcut
                                        .matches(Modifiers::CONTROL | Modifiers::SHIFT, Code::F12)
                                {
                                    let state = app.state::<AppState>();
                                    let _ =
                                        presentation::commands::emergency_stop(app.clone(), state);
                                }
                            })
                            .build(),
                    )
                    .is_ok();
                let registered = plugin_available
                    && app.global_shortcut().register(emergency_shortcut()).is_ok();
                app.state::<AppState>()
                    .emergency_shortcut_available
                    .store(registered, std::sync::atomic::Ordering::SeqCst);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            presentation::commands::get_bootstrap_state,
            presentation::commands::save_settings,
            presentation::commands::emergency_stop,
            presentation::commands::resume_pet
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() != "pet" {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running the desktop pet application");
}
