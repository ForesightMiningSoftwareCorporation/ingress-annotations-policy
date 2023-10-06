use crate::LOG_DRAIN;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use slog::info;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub(crate) struct Settings {
    pub annotations: HashMap<String, String>,
}

impl Default for Settings {
    fn default() -> Self {
        let mut default_annotations: HashMap<String, String> = HashMap::new();
        default_annotations.insert("validated_by".to_string(), "kubewarden".to_string());
        Self {
            annotations: default_annotations,
        }
    }
}

impl kubewarden::settings::Validatable for Settings {
    fn validate(&self) -> Result<(), String> {
        info!(LOG_DRAIN, "starting settings validation");
        if self.annotations.is_empty() {
            return Err("no annotations provided, and at least on must be provided".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::settings::Validatable;

    #[test]
    fn validate_settings() -> Result<(), ()> {
        let settings: Settings = Default::default();
        assert!(settings.validate().is_ok());
        let settings: Settings = Settings {
            annotations: HashMap::new(),
        };
        assert!(settings.validate().is_err());
        Ok(())
    }
}
