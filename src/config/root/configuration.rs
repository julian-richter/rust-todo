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

    /// RAII guard that sets env vars on creation and removes them on drop,
    /// ensuring cleanup even if a test panics.
    struct TestEnvGuard;

    impl TestEnvGuard {
        fn new(name: &str, port: &str) -> Self {
            unsafe {
                env::set_var("APP_SERVICE_NAME", name);
                env::set_var("APP_SERVICE_PORT", port);
            }
            Self
        }

        fn empty() -> Self {
            unsafe {
                env::remove_var("APP_SERVICE_NAME");
                env::remove_var("APP_SERVICE_PORT");
            }
            Self
        }
    }

    impl Drop for TestEnvGuard {
        fn drop(&mut self) {
            unsafe {
                env::remove_var("APP_SERVICE_NAME");
                env::remove_var("APP_SERVICE_PORT");
            }
        }
    }

    #[test]
    fn from_env_builds_full_config() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = TestEnvGuard::new("integration", "4000");
        let config = Configuration::from_env().unwrap();
        assert_eq!(config.app.service_name(), "integration");
        assert_eq!(config.app.service_port(), 4000);
    }

    #[test]
    fn from_env_propagates_error() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = TestEnvGuard::empty();
        let result = Configuration::from_env();
        assert!(result.is_err());
    }
}
