use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
    message: String,
}

#[allow(dead_code)]
impl ErrorResponse {
    pub fn new(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
        }
    }

    pub fn error(&self) -> &str {
        &self.error
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_error_response() {
        let resp = ErrorResponse::new("not_found", "Todo not found");
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["error"], "not_found");
        assert_eq!(json["message"], "Todo not found");
    }

    #[test]
    fn accessors_return_expected_values() {
        let resp = ErrorResponse::new("bad_request", "Invalid input");
        assert_eq!(resp.error(), "bad_request");
        assert_eq!(resp.message(), "Invalid input");
    }
}
