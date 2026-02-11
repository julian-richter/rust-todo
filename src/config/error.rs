use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    Missing(&'static str),
    Invalid { key: &'static str, value: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Missing(key) => {
                write!(f, "Missing configuration key: {}", key)
            }
            ConfigError::Invalid { key, value } => {
                write!(f, "Invalid configuration key: {}, value: {}", key, value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_missing_error() {
        let err = ConfigError::Missing("APP_SERVICE_PORT");
        assert_eq!(
            err.to_string(),
            "Missing configuration key: APP_SERVICE_PORT"
        );
    }

    #[test]
    fn display_invalid_error() {
        let err = ConfigError::Invalid {
            key: "APP_SERVICE_PORT",
            value: "abc".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Invalid configuration key: APP_SERVICE_PORT, value: abc"
        );
    }

    #[test]
    fn debug_missing_error() {
        let err = ConfigError::Missing("KEY");
        let debug = format!("{:?}", err);
        assert!(debug.contains("Missing"));
        assert!(debug.contains("KEY"));
    }

    #[test]
    fn debug_invalid_error() {
        let err = ConfigError::Invalid {
            key: "KEY",
            value: "bad".to_string(),
        };
        let debug = format!("{:?}", err);
        assert!(debug.contains("Invalid"));
        assert!(debug.contains("KEY"));
        assert!(debug.contains("bad"));
    }
}
