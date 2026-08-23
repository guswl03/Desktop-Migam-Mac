use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Manager,
};

pub fn build(app: &App) -> tauri::Result<()> {
    let show_pet = MenuItem::with_id(app, "show_pet", "펫 표시", true, None::<&str>)?;
    let show_timer = MenuItem::with_id(app, "show_timer", "타이머", true, None::<&str>)?;
    let show_settings = MenuItem::with_id(app, "show_settings", "설정", true, None::<&str>)?;
    let emergency_stop = MenuItem::with_id(app, "emergency_stop", "긴급 중지", true, None::<&str>)?;
    let resume_pet = MenuItem::with_id(app, "resume_pet", "펫 다시 시작", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "종료", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[
            &show_pet,
            &show_timer,
            &show_settings,
            &emergency_stop,
            &resume_pet,
            &quit,
        ],
    )?;

    let mut tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show_pet" => show_window(app, "pet"),
            "show_timer" => show_window(app, "timer"),
            "show_settings" => show_window(app, "settings"),
            "emergency_stop" => {
                let state = app.state::<crate::app_state::AppState>();
                let _ = crate::presentation::commands::emergency_stop(app.clone(), state);
            }
            "resume_pet" => {
                let state = app.state::<crate::app_state::AppState>();
                let _ = crate::presentation::commands::resume_pet(app.clone(), state);
            }
            "quit" => app.exit(0),
            _ => {}
        });

    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }
    tray.build(app)?;
    Ok(())
}

fn show_window(app: &tauri::AppHandle, label: &str) {
    if let Some(window) = app.get_webview_window(label) {
        let _ = window.show();
        let _ = window.set_focus();
    }
}
