pub trait EnvVariable: Copy {
    fn as_str(self) -> &'static str;
}

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

impl EnvVariable for Application {
    fn as_str(self) -> &'static str {
        self.as_str()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Database {
    Url,
    MaxConnections,
}

impl Database {
    pub const fn as_str(self) -> &'static str {
        match self {
            Database::Url => "APP_DATABASE_URL",
            Database::MaxConnections => "APP_DATABASE_MAX_CONNECTIONS",
        }
    }
}

impl EnvVariable for Database {
    fn as_str(self) -> &'static str {
        self.as_str()
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

    #[test]
    fn database_url_maps_to_env_var() {
        assert_eq!(Database::Url.as_str(), "APP_DATABASE_URL");
    }

    #[test]
    fn database_max_connections_maps_to_env_var() {
        assert_eq!(
            Database::MaxConnections.as_str(),
            "APP_DATABASE_MAX_CONNECTIONS"
        );
    }

    #[test]
    fn database_enum_is_copy() {
        let a = Database::Url;
        let b = a; // Copy
        assert_eq!(a.as_str(), b.as_str());
    }
}
