use std::sync::{atomic::AtomicBool, RwLock};

use crate::{
    application::{
        foreground_monitor::ForegroundMonitor, pomodoro_service::PomodoroService,
        settings_service::SettingsService,
    },
    domain::settings::Settings,
    infrastructure::windows::{PlatformForegroundWindowSource, PlatformWindowMinimizer},
};

pub struct AppState {
    pub settings: RwLock<Settings>,
    pub settings_service: SettingsService,
    pub pomodoro_service: PomodoroService,
    pub foreground_monitor: ForegroundMonitor,
    pub emergency_stopped: AtomicBool,
    pub emergency_shortcut_available: AtomicBool,
    pub tray_available: AtomicBool,
}

impl AppState {
    pub fn new(settings: Settings, settings_service: SettingsService) -> Self {
        let pomodoro_service = PomodoroService::new(&settings.pomodoro);
        Self {
            settings: RwLock::new(settings),
            settings_service,
            pomodoro_service,
            foreground_monitor: ForegroundMonitor::new(
                Box::new(PlatformForegroundWindowSource::new()),
                Box::new(PlatformWindowMinimizer),
                std::process::id(),
            ),
            emergency_stopped: AtomicBool::new(false),
            emergency_shortcut_available: AtomicBool::new(false),
            tray_available: AtomicBool::new(false),
        }
    }
}
