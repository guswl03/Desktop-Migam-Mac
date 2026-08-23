use crate::domain::foreground::{
    ForegroundReadError, ForegroundWindowSource, WindowMinimizer, WindowSnapshot,
};

#[derive(Default)]
pub struct PlatformForegroundWindowSource;

#[derive(Default)]
pub struct PlatformWindowMinimizer;

impl PlatformForegroundWindowSource {
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use axuielement::{ax_attribute, system_wide};
    use objc2_app_kit::NSRunningApplication;

    use super::*;

    fn identity(pid: u32, title: &str, x: i32, y: i32, width: u32, height: u32) -> isize {
        let mut hasher = DefaultHasher::new();
        (pid, title, x, y, width, height).hash(&mut hasher);
        let value = hasher.finish() as isize;
        if value == 0 {
            1
        } else {
            value
        }
    }

    pub fn snapshot() -> Result<Option<WindowSnapshot>, ForegroundReadError> {
        if !axuielement::is_process_trusted() {
            return Err(ForegroundReadError::AccessDenied);
        }
        let system = system_wide().ok_or(ForegroundReadError::InspectionFailed)?;
        let app = system
            .focused_application()
            .map_err(|_| ForegroundReadError::InspectionFailed)?
            .ok_or(ForegroundReadError::InspectionFailed)?;
        let pid = u32::try_from(
            app.pid()
                .map_err(|_| ForegroundReadError::InspectionFailed)?,
        )
        .map_err(|_| ForegroundReadError::InspectionFailed)?;
        let window = app
            .element_attribute(ax_attribute::AX_FOCUSED_WINDOW_ATTRIBUTE)
            .map_err(|_| ForegroundReadError::InspectionFailed)?
            .ok_or(ForegroundReadError::InspectionFailed)?;
        window
            .set_timeout(0.25)
            .map_err(|_| ForegroundReadError::InspectionFailed)?;
        let title = window
            .string_attribute(ax_attribute::AX_TITLE_ATTRIBUTE)
            .map_err(|_| ForegroundReadError::InspectionFailed)?;
        let position = window
            .point_attribute(ax_attribute::AX_POSITION_ATTRIBUTE)
            .map_err(|_| ForegroundReadError::InspectionFailed)?
            .ok_or(ForegroundReadError::InspectionFailed)?;
        let size = window
            .size_attribute(ax_attribute::AX_SIZE_ATTRIBUTE)
            .map_err(|_| ForegroundReadError::InspectionFailed)?
            .ok_or(ForegroundReadError::InspectionFailed)?;
        let minimized = window
            .bool_attribute("AXMinimized")
            .map_err(|_| ForegroundReadError::InspectionFailed)?
            .unwrap_or(false);
        let x = position.x.round() as i32;
        let y = position.y.round() as i32;
        let width = size.width.round().max(0.0) as u32;
        let height = size.height.round().max(0.0) as u32;
        let running = NSRunningApplication::runningApplicationWithProcessIdentifier(pid as i32);
        let process_name = running
            .as_ref()
            .and_then(|app| app.localizedName())
            .map(|value| value.to_string());
        let bundle_id = running
            .as_ref()
            .and_then(|app| app.bundleIdentifier())
            .map(|value| value.to_string());
        let title_text = title.unwrap_or_default();
        Ok(Some(WindowSnapshot {
            window_id: identity(pid, &title_text, x, y, width, height),
            process_id: pid,
            process_name,
            bundle_id,
            title: (!title_text.is_empty()).then_some(title_text),
            is_visible: true,
            is_minimized: minimized,
            is_fullscreen: false,
            monitor_left: x,
            x,
            y,
            width,
            height,
        }))
    }

    pub fn minimize(expected_window_id: isize) -> Result<(), ForegroundReadError> {
        let fresh = snapshot()?.ok_or(ForegroundReadError::InspectionFailed)?;
        if fresh.window_id != expected_window_id || fresh.is_minimized || fresh.is_fullscreen {
            return Err(ForegroundReadError::InspectionFailed);
        }
        let system = system_wide().ok_or(ForegroundReadError::InspectionFailed)?;
        let app = system
            .focused_application()
            .map_err(|_| ForegroundReadError::InspectionFailed)?
            .ok_or(ForegroundReadError::InspectionFailed)?;
        let window = app
            .element_attribute(ax_attribute::AX_FOCUSED_WINDOW_ATTRIBUTE)
            .map_err(|_| ForegroundReadError::InspectionFailed)?
            .ok_or(ForegroundReadError::InspectionFailed)?;
        if !window.is_attribute_settable("AXMinimized").unwrap_or(false) {
            return Err(ForegroundReadError::InspectionFailed);
        }
        window
            .set_bool_attribute("AXMinimized", true)
            .map_err(|_| ForegroundReadError::InspectionFailed)
    }
}

impl ForegroundWindowSource for PlatformForegroundWindowSource {
    fn foreground_window(&self) -> Result<Option<WindowSnapshot>, ForegroundReadError> {
        #[cfg(target_os = "macos")]
        return platform::snapshot();
        #[cfg(not(target_os = "macos"))]
        Ok(None)
    }
}

impl WindowMinimizer for PlatformWindowMinimizer {
    fn minimize(&self, window_id: isize) -> Result<(), ForegroundReadError> {
        #[cfg(target_os = "macos")]
        return platform::minimize(window_id);
        #[cfg(not(target_os = "macos"))]
        {
            let _ = window_id;
            Err(ForegroundReadError::InspectionFailed)
        }
    }
}
