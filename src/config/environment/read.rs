use std::{env, str::FromStr};

use crate::config::error::ConfigError;
use super::variables::Application;

pub fn read_required<T>(var: Application) -> Result<T, ConfigError>
where
    T: FromStr,
{
    match env::var(var.as_str()) {
        Ok(raw) => raw.parse::<T>().map_err(|_| ConfigError::Invalid {
            key: var.as_str(),
            value: raw,
        }),

        Err(env::VarError::NotPresent) => {
            Err(ConfigError::Missing(var.as_str()))
        }

        Err(env::VarError::NotUnicode(os_string)) => {
            Err(ConfigError::Invalid {
                key: var.as_str(),
                value: os_string.to_string_lossy().into_owned(),
            })
        }
    }
}

pub fn read_optional<T>(var: Application, fallback: T) -> Result<T, ConfigError>
where
    T: FromStr,
{
    match env::var(var.as_str()) {
        Ok(raw) => raw.parse::<T>().map_err(|_| ConfigError::Invalid {
            key: var.as_str(),
            value: raw,
        }),

        Err(env::VarError::NotPresent) => Ok(fallback),

        Err(env::VarError::NotUnicode(os_string)) => {
            Err(ConfigError::Invalid {
                key: var.as_str(),
                value: os_string.to_string_lossy().into_owned(),
            })
        }
    }
}
