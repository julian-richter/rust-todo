use crate::config::app::app_configuration::AppConfiguration;
use crate::config::database::database_configuration::DatabaseConfiguration;
use crate::config::error::ConfigError;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub app: AppConfiguration,
    pub database: DatabaseConfiguration,
}

impl Configuration {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            app: AppConfiguration::from_env()?,
            database: DatabaseConfiguration::from_env()?,
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
        fn new(name: &str, port: &str, db_url: &str) -> Self {
            unsafe {
                env::set_var("APP_SERVICE_NAME", name);
                env::set_var("APP_SERVICE_PORT", port);
                env::set_var("APP_DATABASE_URL", db_url);
            }
            Self
        }

        fn empty() -> Self {
            unsafe {
                env::remove_var("APP_SERVICE_NAME");
                env::remove_var("APP_SERVICE_PORT");
                env::remove_var("APP_DATABASE_URL");
                env::remove_var("APP_DATABASE_MAX_CONNECTIONS");
            }
            Self
        }
    }

    impl Drop for TestEnvGuard {
        fn drop(&mut self) {
            unsafe {
                env::remove_var("APP_SERVICE_NAME");
                env::remove_var("APP_SERVICE_PORT");
                env::remove_var("APP_DATABASE_URL");
                env::remove_var("APP_DATABASE_MAX_CONNECTIONS");
            }
        }
    }

    #[test]
    fn from_env_builds_full_config() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = TestEnvGuard::new("integration", "4000", "sqlite:test.db");
        let config = Configuration::from_env().unwrap();
        assert_eq!(config.app.service_name(), "integration");
        assert_eq!(config.app.service_port(), 4000);
        assert_eq!(config.database.url(), "sqlite:test.db");
        assert_eq!(config.database.max_connections(), 5);
    }

    #[test]
    fn from_env_propagates_error() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = TestEnvGuard::empty();
        let result = Configuration::from_env();
        assert!(result.is_err());
    }
}
