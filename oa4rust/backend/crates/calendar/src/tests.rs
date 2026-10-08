// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
        // o2server 成功信封 message 恒为空串（2026-08-25 行为对比结论）
        assert_eq!(result.message, Some(String::new()));
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

    // 轮74 回归：前端（桌面+移动）发 camelCase，CreateEventRequest 必须能解，
    // 否则 calendar_id 落 None → event_create 400，建事件实跑必败。
    #[test]
    fn create_event_request_accepts_camel_case_from_frontends() {
        let req: crate::CreateEventRequest = serde_json::from_str(
            r#"{"calendarId":"cal-1","title":"评审","startTime":"2026-10-01 09:00","endTime":"2026-10-01 10:00","location":"会议室"}"#,
        )
        .unwrap();
        assert_eq!(req.calendar_id.as_deref(), Some("cal-1"));
        assert_eq!(req.start_time.as_deref(), Some("2026-10-01 09:00"));
        assert_eq!(req.end_time.as_deref(), Some("2026-10-01 10:00"));
    }

    // snake_case 旧调用方仍须可解（alias 双向兼容）
    #[test]
    fn create_event_request_still_accepts_snake_case() {
        let req: crate::CreateEventRequest =
            serde_json::from_str(r#"{"calendar_id":"cal-2","start_time":"t1","end_time":"t2"}"#)
                .unwrap();
        assert_eq!(req.calendar_id.as_deref(), Some("cal-2"));
        assert_eq!(req.start_time.as_deref(), Some("t1"));
    }
}
