#[derive(Debug, Clone, Copy)]
pub enum Application {
    ServiceName,
    ServicePort,
}

impl Application {
    pub const fn as_str(self) -> &'static str {
        match self {
            Application::ServiceName => "APP_SERVICE_NAME",
            Application::ServicePort => "APP_SERVICE_PORT",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_name_maps_to_env_var() {
        assert_eq!(Application::ServiceName.as_str(), "APP_SERVICE_NAME");
    }

    #[test]
    fn service_port_maps_to_env_var() {
        assert_eq!(Application::ServicePort.as_str(), "APP_SERVICE_PORT");
    }

    #[test]
    fn enum_is_copy() {
        let a = Application::ServiceName;
        let b = a; // Copy
        assert_eq!(a.as_str(), b.as_str());
    }
}
