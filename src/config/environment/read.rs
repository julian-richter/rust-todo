use std::{env, str::FromStr};

use super::variables::Application;
use crate::config::error::ConfigError;

pub fn read_required<T>(var: Application) -> Result<T, ConfigError>
where
    T: FromStr,
{
    match env::var(var.as_str()) {
        Ok(raw) => raw.parse::<T>().map_err(|_| ConfigError::Invalid {
            key: var.as_str(),
            value: raw,
        }),

        Err(env::VarError::NotPresent) => Err(ConfigError::Missing(var.as_str())),

        Err(env::VarError::NotUnicode(os_string)) => Err(ConfigError::Invalid {
            key: var.as_str(),
            value: os_string.to_string_lossy().into_owned(),
        }),
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

        Err(env::VarError::NotUnicode(os_string)) => Err(ConfigError::Invalid {
            key: var.as_str(),
            value: os_string.to_string_lossy().into_owned(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    // SAFETY: env::set_var and env::remove_var are unsafe in edition 2024
    // because they are not thread-safe. We hold ENV_LOCK to ensure exclusive
    // access and run CI with --test-threads=1 for cross-module safety.

    #[test]
    fn read_required_parses_valid_u16() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::set_var("APP_SERVICE_PORT", "8080") };
        let result: Result<u16, _> = read_required(Application::ServicePort);
        assert_eq!(result.unwrap(), 8080);
        unsafe { env::remove_var("APP_SERVICE_PORT") };
    }

    #[test]
    fn read_required_parses_valid_string() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::set_var("APP_SERVICE_NAME", "my-app") };
        let result: Result<String, _> = read_required(Application::ServiceName);
        assert_eq!(result.unwrap(), "my-app");
        unsafe { env::remove_var("APP_SERVICE_NAME") };
    }

    #[test]
    fn read_required_returns_missing_when_unset() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::remove_var("APP_SERVICE_PORT") };
        let result: Result<u16, _> = read_required(Application::ServicePort);
        assert!(matches!(
            result,
            Err(ConfigError::Missing("APP_SERVICE_PORT"))
        ));
    }

    #[test]
    fn read_required_returns_invalid_for_unparseable_value() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::set_var("APP_SERVICE_PORT", "not_a_number") };
        let result: Result<u16, _> = read_required(Application::ServicePort);
        match result {
            Err(ConfigError::Invalid { key, value }) => {
                assert_eq!(key, "APP_SERVICE_PORT");
                assert_eq!(value, "not_a_number");
            }
            other => panic!("expected Invalid error, got {:?}", other),
        }
        unsafe { env::remove_var("APP_SERVICE_PORT") };
    }

    #[test]
    fn read_optional_returns_value_when_set() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::set_var("APP_SERVICE_NAME", "custom") };
        let result: Result<String, _> =
            read_optional(Application::ServiceName, "default".to_string());
        assert_eq!(result.unwrap(), "custom");
        unsafe { env::remove_var("APP_SERVICE_NAME") };
    }

    #[test]
    fn read_optional_returns_fallback_when_unset() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::remove_var("APP_SERVICE_NAME") };
        let result: Result<String, _> =
            read_optional(Application::ServiceName, "fallback".to_string());
        assert_eq!(result.unwrap(), "fallback");
    }

    #[test]
    fn read_optional_returns_invalid_for_unparseable_value() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::set_var("APP_SERVICE_PORT", "bad") };
        let result: Result<u16, _> = read_optional(Application::ServicePort, 3000);
        match result {
            Err(ConfigError::Invalid { key, value }) => {
                assert_eq!(key, "APP_SERVICE_PORT");
                assert_eq!(value, "bad");
            }
            other => panic!("expected Invalid error, got {:?}", other),
        }
        unsafe { env::remove_var("APP_SERVICE_PORT") };
    }

    #[test]
    fn read_required_port_zero() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::set_var("APP_SERVICE_PORT", "0") };
        let result: Result<u16, _> = read_required(Application::ServicePort);
        assert_eq!(result.unwrap(), 0);
        unsafe { env::remove_var("APP_SERVICE_PORT") };
    }

    #[test]
    fn read_required_port_max() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::set_var("APP_SERVICE_PORT", "65535") };
        let result: Result<u16, _> = read_required(Application::ServicePort);
        assert_eq!(result.unwrap(), 65535);
        unsafe { env::remove_var("APP_SERVICE_PORT") };
    }

    #[test]
    fn read_required_port_overflow() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { env::set_var("APP_SERVICE_PORT", "65536") };
        let result: Result<u16, _> = read_required(Application::ServicePort);
        assert!(matches!(result, Err(ConfigError::Invalid { .. })));
        unsafe { env::remove_var("APP_SERVICE_PORT") };
    }
}
