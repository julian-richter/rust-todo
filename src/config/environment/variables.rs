#[derive(Debug, Clone, Copy)]
pub enum Application {
    ServiceName,
    ServicePort
}

impl Application {
    pub const fn as_str(self) -> &'static str {
        match self {
            Application::ServiceName => "APP_SERVICE_NAME",
            Application::ServicePort => "APP_SERVICE_PORT"
        }
    }
}