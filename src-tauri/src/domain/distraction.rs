use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DistractionRule {
    #[serde(default)]
    pub process_name: Option<String>,
    #[serde(default)]
    pub window_title: Option<String>,
}
#[derive(Debug, Error, Eq, PartialEq)]
pub enum DistractionRuleError {
    #[error("a distraction rule requires a process name or window title")]
    MissingConditions,
}

impl DistractionRule {
    pub fn validate(&self) -> Result<(), DistractionRuleError> {
        if self.process_condition().is_none() && self.title_condition().is_none() {
            return Err(DistractionRuleError::MissingConditions);
        }
        Ok(())
    }

    pub fn matches(&self, process_name: &str, window_title: &str) -> bool {
        if self.validate().is_err() {
            return false;
        }

        let process_matches = self
            .process_condition()
            .is_none_or(|expected| expected.eq_ignore_ascii_case(process_name));
        let title_matches = self.title_condition().is_none_or(|expected| {
            window_title
                .to_lowercase()
                .contains(&expected.to_lowercase())
        });

        process_matches && title_matches
    }

    fn process_condition(&self) -> Option<&str> {
        Self::populated(self.process_name.as_deref())
    }

    fn title_condition(&self) -> Option<&str> {
        Self::populated(self.window_title.as_deref())
    }

    fn populated(value: Option<&str>) -> Option<&str> {
        value.map(str::trim).filter(|value| !value.is_empty())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn rule(process_name: Option<&str>, window_title: Option<&str>) -> DistractionRule {
        DistractionRule {
            process_name: process_name.map(str::to_owned),
            window_title: window_title.map(str::to_owned),
        }
    }

    #[test]
    fn process_name_matches_case_insensitively_but_not_as_a_substring() {
        let rule = rule(Some("Chrome.EXE"), None);

        assert!(rule.matches("chrome.exe", "Any title"));
        assert!(!rule.matches("my-chrome.exe", "Any title"));
    }

    #[test]
    fn window_title_matches_a_case_insensitive_substring() {
        let rule = rule(None, Some("YouTube"));

        assert!(rule.matches("msedge.exe", "music - YOUTUBE"));
        assert!(!rule.matches("msedge.exe", "Documentation"));
    }

    #[test]
    fn all_populated_conditions_must_match() {
        let rule = rule(Some("chrome.exe"), Some("YouTube"));

        assert!(rule.matches("CHROME.EXE", "Video - youtube"));
        assert!(!rule.matches("firefox.exe", "Video - YouTube"));
        assert!(!rule.matches("chrome.exe", "Mail"));
    }

    #[test]
    fn a_rule_with_neither_populated_condition_is_rejected() {
        let empty = rule(None, None);
        let whitespace = rule(Some("  "), Some("\t"));

        assert_eq!(
            empty.validate(),
            Err(DistractionRuleError::MissingConditions)
        );
        assert_eq!(
            whitespace.validate(),
            Err(DistractionRuleError::MissingConditions)
        );
        assert!(!empty.matches("chrome.exe", "YouTube"));
    }
}
