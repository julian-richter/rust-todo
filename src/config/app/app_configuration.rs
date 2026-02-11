use crate::config::environment::variables::Application;
use crate::config::environment::{read::read_optional, read::read_required};
use crate::config::error::ConfigError;

#[derive(Clone, Debug)]
pub struct AppConfiguration {
    service_name: String,
    service_port: u16,
}

impl AppConfiguration {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            service_name: read_optional(Application::ServiceName, "todo".to_string())?,
            service_port: read_required(Application::ServicePort)?,
        })
    }

    // explicit accessors (API stability)
    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn cleanup() {
        unsafe {
            env::remove_var("APP_SERVICE_NAME");
            env::remove_var("APP_SERVICE_PORT");
        }
    }

    #[test]
    fn from_env_with_all_vars_set() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            env::set_var("APP_SERVICE_NAME", "my-service");
            env::set_var("APP_SERVICE_PORT", "3000");
        }
        let config = AppConfiguration::from_env().unwrap();
        assert_eq!(config.service_name(), "my-service");
        assert_eq!(config.service_port(), 3000);
        cleanup();
    }

    #[test]
    fn from_env_uses_default_service_name() {
        let _lock = ENV_LOCK.lock().unwrap();
        cleanup();
        unsafe { env::set_var("APP_SERVICE_PORT", "5000") };
        let config = AppConfiguration::from_env().unwrap();
        assert_eq!(config.service_name(), "todo");
        assert_eq!(config.service_port(), 5000);
        cleanup();
    }

    #[test]
    fn from_env_fails_when_port_missing() {
        let _lock = ENV_LOCK.lock().unwrap();
        cleanup();
        let result = AppConfiguration::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn from_env_fails_when_port_invalid() {
        let _lock = ENV_LOCK.lock().unwrap();
        cleanup();
        unsafe { env::set_var("APP_SERVICE_PORT", "xyz") };
        let result = AppConfiguration::from_env();
        assert!(result.is_err());
        cleanup();
    }

    #[test]
    fn clone_preserves_values() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            env::set_var("APP_SERVICE_NAME", "clone-test");
            env::set_var("APP_SERVICE_PORT", "9090");
        }
        let config = AppConfiguration::from_env().unwrap();
        let cloned = config.clone();
        assert_eq!(cloned.service_name(), "clone-test");
        assert_eq!(cloned.service_port(), 9090);
        cleanup();
    }
}
