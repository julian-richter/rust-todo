use crate::config::app::app_configuration::AppConfiguration;
use crate::config::error::ConfigError;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub app: AppConfiguration,
}

impl Configuration {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            app: AppConfiguration::from_env()?,
        })
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
    fn from_env_builds_full_config() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            env::set_var("APP_SERVICE_NAME", "integration");
            env::set_var("APP_SERVICE_PORT", "4000");
        }
        let config = Configuration::from_env().unwrap();
        assert_eq!(config.app.service_name(), "integration");
        assert_eq!(config.app.service_port(), 4000);
        cleanup();
    }

    #[test]
    fn from_env_propagates_error() {
        let _lock = ENV_LOCK.lock().unwrap();
        cleanup();
        let result = Configuration::from_env();
        assert!(result.is_err());
    }
}
