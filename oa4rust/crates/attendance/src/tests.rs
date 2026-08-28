#[cfg(test)]
mod tests {
    use crate::AdminInfo;
    use crate::EmployeeConfigInfo;
    use crate::StatisticalCycleInfo;
    use shared::response::ActionResult;

    #[test]
    fn test_action_result_success_structure() {
        let result: ActionResult<String> = ActionResult::success("test".to_string());
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some("test".to_string()));
        // Java 成功信封 message 恒为空串（2026-08-25 行为对比结论）
        assert_eq!(result.message, Some(String::new()));
    }

    #[test]
    fn test_action_result_error_structure() {
        let result: ActionResult<String> = ActionResult::error("test error");
        assert_eq!(result.r#type, Some("error".to_string()));
        assert_eq!(result.message, Some("test error".to_string()));
    }

    #[test]
    fn test_admin_info_dto_serialization() {
        let admin = AdminInfo {
            id: "test-id".to_string(),
            unit_name: "研发部".to_string(),
            unit_ou: "ou-test".to_string(),
            admin_name: "张三".to_string(),
            admin: "zhangsan".to_string(),
            admin_level: "TOPUNIT".to_string(),
        };

        let json = serde_json::to_value(&admin).unwrap();
        assert_eq!(json["id"], "test-id");
        assert_eq!(json["adminName"], "张三");
        assert_eq!(json["adminLevel"], "TOPUNIT");
    }

    #[test]
    fn test_employee_config_info_dto_serialization() {
        let config = EmployeeConfigInfo {
            id: "config-id".to_string(),
            top_unit_name: "集团总部".to_string(),
            top_unit_ou: "top-ou".to_string(),
            unit_name: "研发部".to_string(),
            unit_ou: "ou-test".to_string(),
            employee_name: "李四".to_string(),
            employee_number: "EMP002".to_string(),
            config_type: "REQUIRED".to_string(),
            emp_in_top_unit_time: "2021-06-01".to_string(),
        };

        let json = serde_json::to_value(&config).unwrap();
        assert_eq!(json["employeeName"], "李四");
        assert_eq!(json["configType"], "REQUIRED");
        assert_eq!(json["employeeNumber"], "EMP002");
    }

    #[test]
    fn test_statistical_cycle_info_dto_serialization() {
        let cycle = StatisticalCycleInfo {
            id: "cycle-id".to_string(),
            top_unit_name: "集团总部".to_string(),
            unit_name: "研发部".to_string(),
            cycle_year: "2024".to_string(),
            cycle_month: "01".to_string(),
            cycle_start_date_string: "2024-01-01".to_string(),
            cycle_end_date_string: "2024-01-31".to_string(),
            description: "1月统计周期".to_string(),
        };

        let json = serde_json::to_value(&cycle).unwrap();
        assert_eq!(json["cycleYear"], "2024");
        assert_eq!(json["cycleMonth"], "01");
        assert_eq!(json["description"], "1月统计周期");
    }

    #[test]
    fn test_list_admins_response_shape() {
        let result = ActionResult::success(serde_json::json!({
            "count": 2,
            "data": [
                {
                    "id": "admin-1",
                    "unitName": "研发部",
                    "unitOu": "ou-test",
                    "adminName": "张三",
                    "admin": "zhangsan",
                    "adminLevel": "TOPUNIT"
                },
                {
                    "id": "admin-2",
                    "unitName": "产品部",
                    "unitOu": "ou-prod",
                    "adminName": "李四",
                    "admin": "lisi",
                    "adminLevel": "UNIT"
                }
            ]
        }));

        assert_eq!(result.r#type, Some("success".to_string()));
        let data = result.data.unwrap();
        assert_eq!(data["count"], 2);
        assert_eq!(data["data"].as_array().unwrap().len(), 2);
        assert_eq!(data["data"][0]["adminName"], "张三");
    }

    #[test]
    fn test_list_employee_configs_response_shape() {
        let result = ActionResult::success(serde_json::json!({
            "count": 1,
            "data": [
                {
                    "id": "config-1",
                    "topUnitName": "集团总部",
                    "topUnitOu": "top-ou",
                    "unitName": "研发部",
                    "unitOu": "ou-test",
                    "employeeName": "王五",
                    "employeeNumber": "EMP002",
                    "configType": "REQUIRED",
                    "empInTopUnitTime": "2021-06-01"
                }
            ]
        }));

        assert_eq!(result.r#type, Some("success".to_string()));
        let data = result.data.unwrap();
        assert_eq!(data["count"], 1);
        assert_eq!(data["data"][0]["employeeName"], "王五");
        assert_eq!(data["data"][0]["configType"], "REQUIRED");
    }

    #[test]
    fn test_list_statistical_cycles_response_shape() {
        let result = ActionResult::success(serde_json::json!({
            "count": 1,
            "data": [
                {
                    "id": "cycle-1",
                    "topUnitName": "集团总部",
                    "unitName": "研发部",
                    "cycleYear": "2024",
                    "cycleMonth": "01",
                    "cycleStartDateString": "2024-01-01",
                    "cycleEndDateString": "2024-01-31",
                    "description": "1月统计周期"
                }
            ]
        }));

        assert_eq!(result.r#type, Some("success".to_string()));
        let data = result.data.unwrap();
        assert_eq!(data["count"], 1);
        assert_eq!(data["data"][0]["cycleYear"], "2024");
        assert_eq!(data["data"][0]["description"], "1月统计周期");
    }
}
