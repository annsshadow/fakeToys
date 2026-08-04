#[cfg(test)]
mod tests {
    use crate::{CalendarItem, MyCalendarResponse};
    use shared::response::ActionResult;

    #[test]
    fn test_calendar_item_serialization() {
        let item = CalendarItem {
            id: "cal-001".to_string(),
            name: "Test Calendar".to_string(),
            calendar_type: "PERSON".to_string(),
            target: "person-001".to_string(),
            color: "#1462be".to_string(),
            description: Some("Test description".to_string()),
            source: Some("PERSON".to_string()),
            createor: "admin".to_string(),
            is_public: true,
            status: "OPEN".to_string(),
        };

        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["id"], "cal-001");
        assert_eq!(json["name"], "Test Calendar");
        assert_eq!(json["type"], "PERSON");
        assert_eq!(json["isPublic"], true);
        assert_eq!(json["status"], "OPEN");
    }

    #[test]
    fn test_my_calendar_response_serialization() {
        let response = MyCalendarResponse {
            my_calendars: vec![],
            unit_calendars: vec![],
            follow_calendars: vec![],
        };

        let json = serde_json::to_value(&response).unwrap();
        assert!(json["my_calendars"].is_array());
        assert!(json["unit_calendars"].is_array());
        assert!(json["follow_calendars"].is_array());
    }

    #[test]
    fn test_action_result_success_wraps_data() {
        let item = CalendarItem {
            id: "cal-001".to_string(),
            name: "Test".to_string(),
            calendar_type: "PERSON".to_string(),
            target: "t1".to_string(),
            color: "#000".to_string(),
            description: None,
            source: None,
            createor: "a".to_string(),
            is_public: false,
            status: "OPEN".to_string(),
        };
        let result: ActionResult<CalendarItem> = ActionResult::success(item);
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        assert_eq!(result.message, None);
    }

    #[test]
    fn test_action_result_error_wraps_message() {
        let result: ActionResult<CalendarItem> = ActionResult::error("not found");
        assert_eq!(result.r#type, Some("error".to_string()));
        assert_eq!(result.message, Some("not found".to_string()));
        assert!(result.data.is_none());
    }

    #[test]
    fn test_calendar_type_classification() {
        let unit = CalendarItem {
            id: "unit-1".to_string(),
            name: "Unit".to_string(),
            calendar_type: "UNIT".to_string(),
            target: "unit-1".to_string(),
            color: "#000".to_string(),
            description: None,
            source: None,
            createor: "admin".to_string(),
            is_public: true,
            status: "OPEN".to_string(),
        };

        let person = CalendarItem {
            id: "person-1".to_string(),
            name: "Person".to_string(),
            calendar_type: "PERSON".to_string(),
            target: "person-1".to_string(),
            color: "#000".to_string(),
            description: None,
            source: None,
            createor: "user".to_string(),
            is_public: false,
            status: "OPEN".to_string(),
        };

        assert!(unit.calendar_type.eq_ignore_ascii_case("UNIT"));
        assert!(person.calendar_type.eq_ignore_ascii_case("PERSON"));
        assert!(!unit.calendar_type.eq_ignore_ascii_case("PERSON"));
    }
}
