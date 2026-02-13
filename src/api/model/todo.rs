use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow, Serialize)]
pub struct Todo {
    id: i64,
    title: String,
    description: Option<String>,
    completed: bool,
    created_at: String,
    updated_at: String,
}

#[allow(dead_code)]
impl Todo {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_produces_expected_json() {
        let todo = Todo {
            id: 1,
            title: "Test".to_string(),
            description: Some("A description".to_string()),
            completed: false,
            created_at: "2024-01-01T00:00:00".to_string(),
            updated_at: "2024-01-01T00:00:00".to_string(),
        };
        let json = serde_json::to_value(&todo).unwrap();
        assert_eq!(json["id"], 1);
        assert_eq!(json["title"], "Test");
        assert_eq!(json["description"], "A description");
        assert_eq!(json["completed"], false);
    }

    #[test]
    fn serialize_with_null_description() {
        let todo = Todo {
            id: 2,
            title: "No desc".to_string(),
            description: None,
            completed: true,
            created_at: "2024-01-01T00:00:00".to_string(),
            updated_at: "2024-01-01T00:00:00".to_string(),
        };
        let json = serde_json::to_value(&todo).unwrap();
        assert!(json["description"].is_null());
        assert_eq!(json["completed"], true);
    }

    #[test]
    fn accessors_return_expected_values() {
        let todo = Todo {
            id: 3,
            title: "Accessor".to_string(),
            description: Some("desc".to_string()),
            completed: false,
            created_at: "2024-01-01T00:00:00".to_string(),
            updated_at: "2024-01-02T00:00:00".to_string(),
        };
        assert_eq!(todo.id(), 3);
        assert_eq!(todo.title(), "Accessor");
        assert_eq!(todo.description(), Some("desc"));
        assert!(!todo.completed());
        assert_eq!(todo.created_at(), "2024-01-01T00:00:00");
        assert_eq!(todo.updated_at(), "2024-01-02T00:00:00");
    }
}
