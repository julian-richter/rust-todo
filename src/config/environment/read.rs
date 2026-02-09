use std::env;
use std::str::FromStr;
use crate::config::environment::variables::Application;
use crate::config::error::ConfigError;

pub fn read_required<T>(var: Application) -> Result<T, ConfigError> where T: FromStr {
    let raw = env::var(var.as_str()).map_err(|_| ConfigError::Missing(var.as_str()))?;

    raw.parse::<T>().map_err(|_| ConfigError::Invalid {
        key: var.as_str(),
        value: raw,
    })
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
        Err(_) => Ok(fallback),
    }
}