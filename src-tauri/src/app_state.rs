use std::sync::{atomic::AtomicBool, RwLock};

use crate::{application::settings_service::SettingsService, domain::settings::Settings};

pub struct AppState {
    pub settings: RwLock<Settings>,
    pub settings_service: SettingsService,
    pub emergency_stopped: AtomicBool,
    pub emergency_shortcut_available: AtomicBool,
    pub tray_available: AtomicBool,
}

impl AppState {
    pub fn new(settings: Settings, settings_service: SettingsService) -> Self {
        Self {
            settings: RwLock::new(settings),
            settings_service,
            emergency_stopped: AtomicBool::new(false),
            emergency_shortcut_available: AtomicBool::new(false),
            tray_available: AtomicBool::new(false),
        }
    }
}
