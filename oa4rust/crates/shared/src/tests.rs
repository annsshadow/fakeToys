#[cfg(test)]
mod tests {
    use crate::error::AppError;
    use crate::response::ActionResult;
    use axum::response::IntoResponse;
    use serde_json::json;

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<String> = ActionResult::success("test".to_string());
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some("test".to_string()));
        assert_eq!(result.message, None);
    }

    #[test]
    fn test_action_result_error() {
        let result: ActionResult<String> = ActionResult::error("test error");
        assert_eq!(result.r#type, Some("error".to_string()));
        assert_eq!(result.message, Some("test error".to_string()));
    }

    #[test]
    fn test_action_result_serialization() {
        let result: ActionResult<i32> = ActionResult::success(42);
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(json["type"], "success");
        assert_eq!(json["data"], 42);
        assert_eq!(json["message"], serde_json::Value::Null);
    }

    #[test]
    fn test_action_result_error_serialization() {
        let result: ActionResult<i32> = ActionResult::error("something went wrong");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "something went wrong");
        assert_eq!(json["data"], serde_json::Value::Null);
    }

    #[test]
    fn test_action_result_with_count() {
        let mut result: ActionResult<serde_json::Value> = ActionResult::success(json!({"items": []}));
        result.count = Some(10);
        result.size = Some(20);

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["count"], 10);
        assert_eq!(json["size"], 20);
    }

    #[test]
    fn test_action_result_with_message() {
        let mut result: ActionResult<String> = ActionResult::success("data".to_string());
        result.message = Some("operation completed".to_string());

        assert_eq!(result.message, Some("operation completed".to_string()));
    }

    #[test]
    fn test_app_error_database() {
        let err = AppError::Database(sqlx::Error::RowNotFound);
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_bad_request() {
        let err = AppError::BadRequest("invalid input".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_app_error_unauthorized() {
        let err = AppError::Unauthorized;
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_app_error_not_found() {
        let err = AppError::NotFound;
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_internal() {
        let err = AppError::Internal;
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
