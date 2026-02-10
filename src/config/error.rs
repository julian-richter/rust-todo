use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    Missing(&'static str),
    Invalid {
        key: &'static str,
        value: String
    },
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