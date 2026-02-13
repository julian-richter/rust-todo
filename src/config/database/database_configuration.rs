use crate::config::environment::variables::Database;
use crate::config::environment::{read_optional, read_required};
use crate::config::error::ConfigError;

#[derive(Clone, Debug)]
pub struct DatabaseConfiguration {
    url: String,
    max_connections: u32,
}

impl DatabaseConfiguration {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            url: read_required(Database::Url)?,
            max_connections: read_optional(Database::MaxConnections, 5)?,
        })
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
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
    struct EnvGuard {
        url: Option<&'static str>,
        max_connections: Option<&'static str>,
    }

    impl EnvGuard {
        fn new(url: Option<&'static str>, max_connections: Option<&'static str>) -> Self {
            unsafe {
                env::remove_var("APP_DATABASE_URL");
                env::remove_var("APP_DATABASE_MAX_CONNECTIONS");
            }
            if let Some(u) = url {
                unsafe { env::set_var("APP_DATABASE_URL", u) };
            }
            if let Some(m) = max_connections {
                unsafe { env::set_var("APP_DATABASE_MAX_CONNECTIONS", m) };
            }
            Self {
                url,
                max_connections,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            unsafe {
                if self.url.is_some() {
                    env::remove_var("APP_DATABASE_URL");
                }
                if self.max_connections.is_some() {
                    env::remove_var("APP_DATABASE_MAX_CONNECTIONS");
                }
            }
        }
    }

    #[test]
    fn from_env_with_all_vars_set() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = EnvGuard::new(Some("sqlite:todo.db"), Some("10"));
        let config = DatabaseConfiguration::from_env().unwrap();
        assert_eq!(config.url(), "sqlite:todo.db");
        assert_eq!(config.max_connections(), 10);
    }

    #[test]
    fn from_env_uses_default_max_connections() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = EnvGuard::new(Some("sqlite:todo.db"), None);
        let config = DatabaseConfiguration::from_env().unwrap();
        assert_eq!(config.url(), "sqlite:todo.db");
        assert_eq!(config.max_connections(), 5);
    }

    #[test]
    fn from_env_fails_when_url_missing() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = EnvGuard::new(None, None);
        let result = DatabaseConfiguration::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn from_env_fails_when_max_connections_invalid() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = EnvGuard::new(Some("sqlite:todo.db"), Some("xyz"));
        let result = DatabaseConfiguration::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn clone_preserves_values() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = EnvGuard::new(Some("sqlite:clone.db"), Some("3"));
        let config = DatabaseConfiguration::from_env().unwrap();
        let cloned = config.clone();
        assert_eq!(cloned.url(), "sqlite:clone.db");
        assert_eq!(cloned.max_connections(), 3);
    }
}
