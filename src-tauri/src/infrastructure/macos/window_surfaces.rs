use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSurface {
    pub window_id: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct WindowSurfaceContext {
    pub work_x: i32,
    pub work_y: i32,
    pub work_width: u32,
    pub work_height: u32,
    pub scale_factor: f64,
}

#[cfg(target_os = "macos")]
mod platform {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
        process,
    };

    use axuielement::AXUIElement;
    use objc2_app_kit::{NSApplicationActivationPolicy, NSWorkspace};

    use super::{WindowSurface, WindowSurfaceContext};

    const MINIMUM_SURFACE_WIDTH: f64 = 96.0;
    const MINIMUM_SURFACE_HEIGHT: f64 = 48.0;

    fn intersects_work_area(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        context: WindowSurfaceContext,
    ) -> bool {
        let scale = context.scale_factor.max(1.0);
        let work_left = context.work_x as f64 / scale;
        let work_top = context.work_y as f64 / scale;
        let work_right = work_left + context.work_width as f64 / scale;
        let work_bottom = work_top + context.work_height as f64 / scale;
        x < work_right && x + width > work_left && y < work_bottom && y + height > work_top
    }

    fn stable_window_id(pid: i32, index: usize, identifier: &str, subrole: &str) -> String {
        let mut hasher = DefaultHasher::new();
        pid.hash(&mut hasher);
        if identifier.is_empty() {
            index.hash(&mut hasher);
        } else {
            identifier.hash(&mut hasher);
        }
        subrole.hash(&mut hasher);
        format!("macos:{}:{:016x}", pid, hasher.finish())
    }

    pub fn climbable_windows(context: WindowSurfaceContext) -> Vec<WindowSurface> {
        if !axuielement::is_process_trusted() {
            return Vec::new();
        }

        let current_pid = process::id() as i32;
        let scale = context.scale_factor.max(1.0);
        let mut surfaces = Vec::new();
        let applications = NSWorkspace::sharedWorkspace().runningApplications();

        for application in applications.iter() {
            let pid = application.processIdentifier();
            if pid <= 0
                || pid == current_pid
                || application.isHidden()
                || application.isTerminated()
                || application.activationPolicy() != NSApplicationActivationPolicy::Regular
            {
                continue;
            }

            let Some(app_element) = AXUIElement::from_pid(pid) else {
                continue;
            };
            let Ok(windows) = app_element.element_array_attribute("AXWindows") else {
                continue;
            };
            for (index, window) in windows.into_iter().enumerate() {
                let minimized = window
                    .bool_attribute("AXMinimized")
                    .ok()
                    .flatten()
                    .unwrap_or(false);
                let role = window
                    .string_attribute("AXRole")
                    .ok()
                    .flatten()
                    .unwrap_or_default();
                if minimized || role != "AXWindow" {
                    continue;
                }

                let Some(position) = window.point_attribute("AXPosition").ok().flatten() else {
                    continue;
                };
                let Some(size) = window.size_attribute("AXSize").ok().flatten() else {
                    continue;
                };
                let width = size.width.max(0.0);
                let height = size.height.max(0.0);
                if width < MINIMUM_SURFACE_WIDTH
                    || height < MINIMUM_SURFACE_HEIGHT
                    || !intersects_work_area(position.x, position.y, width, height, context)
                {
                    continue;
                }

                let identifier = window
                    .string_attribute("AXIdentifier")
                    .ok()
                    .flatten()
                    .unwrap_or_default();
                let subrole = window
                    .string_attribute("AXSubrole")
                    .ok()
                    .flatten()
                    .unwrap_or_default();
                surfaces.push(WindowSurface {
                    window_id: stable_window_id(pid, index, &identifier, &subrole),
                    x: (position.x * scale).round() as i32,
                    y: (position.y * scale).round() as i32,
                    width: (width * scale).round().max(0.0) as u32,
                    height: (height * scale).round().max(0.0) as u32,
                });
            }
        }

        surfaces
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    use super::{WindowSurface, WindowSurfaceContext};

    pub fn climbable_windows(_context: WindowSurfaceContext) -> Vec<WindowSurface> {
        Vec::new()
    }
}

pub use platform::climbable_windows;
