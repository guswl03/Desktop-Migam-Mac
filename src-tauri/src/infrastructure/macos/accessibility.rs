use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessibilityPermissionState {
    Granted,
    Denied,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AccessibilityPermissionService;

impl AccessibilityPermissionService {
    pub fn status(&self) -> AccessibilityPermissionState {
        status(false)
    }

    pub fn request(&self) -> AccessibilityPermissionState {
        status(true)
    }
}

#[cfg(target_os = "macos")]
fn status(prompt: bool) -> AccessibilityPermissionState {
    let trusted = if prompt {
        axuielement::is_process_trusted_with_prompt()
    } else {
        axuielement::is_process_trusted()
    };
    if trusted {
        AccessibilityPermissionState::Granted
    } else {
        AccessibilityPermissionState::Denied
    }
}

#[cfg(not(target_os = "macos"))]
fn status(_prompt: bool) -> AccessibilityPermissionState {
    AccessibilityPermissionState::Unavailable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn permission_is_unavailable_off_macos() {
        assert_eq!(
            AccessibilityPermissionService.status(),
            AccessibilityPermissionState::Unavailable
        );
    }
}
