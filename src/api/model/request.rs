use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTodoRequest {
    title: String,
    description: Option<String>,
}

impl CreateTodoRequest {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateTodoRequest {
    title: String,
    description: Option<String>,
    completed: bool,
}

impl UpdateTodoRequest {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_create_request_with_description() {
        let json = r#"{"title":"Buy milk","description":"From the store"}"#;
        let req: CreateTodoRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.title(), "Buy milk");
        assert_eq!(req.description(), Some("From the store"));
    }

    #[test]
    fn deserialize_create_request_without_description() {
        let json = r#"{"title":"Buy milk"}"#;
        let req: CreateTodoRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.title(), "Buy milk");
        assert!(req.description().is_none());
    }

    #[test]
    fn deserialize_update_request() {
        let json = r#"{"title":"Updated","description":"New desc","completed":true}"#;
        let req: UpdateTodoRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.title(), "Updated");
        assert_eq!(req.description(), Some("New desc"));
        assert!(req.completed());
    }

    #[test]
    fn deserialize_create_request_fails_without_title() {
        let json = r#"{"description":"No title"}"#;
        let result = serde_json::from_str::<CreateTodoRequest>(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_update_request_fails_without_completed() {
        let json = r#"{"title":"Test"}"#;
        let result = serde_json::from_str::<UpdateTodoRequest>(json);
        assert!(result.is_err());
    }
}
