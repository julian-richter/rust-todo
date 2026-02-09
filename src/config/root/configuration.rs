use crate::config::app::app_configuration::AppConfiguration;
use crate::config::error::ConfigError;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub app: AppConfiguration,
}

impl Configuration {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self{
            app: AppConfiguration::from_env()?,
        })
    }
}