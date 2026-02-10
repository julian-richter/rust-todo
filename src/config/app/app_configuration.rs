use crate::config::environment::{
    read::read_required,
    read::read_optional};
use crate::config::environment::variables::Application;
use crate::config::error::ConfigError;

#[derive(Clone, Debug)]
pub struct AppConfiguration {
    service_name:String,
    service_port:u16,
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