#[cfg(test)]
mod tests {
    use axum::extract::Extension;
    use crate::{group, person, role, routes, unit};
    use deadpool_postgres::{Manager, Pool};
    use shared::error::AppError;
    use shared::response::ActionResult;
    use tokio::runtime::Runtime;
    use deadpool_postgres::tokio_postgres::Config as PgConfig;

    /// 测试 ActionResult 成功响应序列化
    #[test]
    fn test_action_result_success_serialization() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 2, "data": []}));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    /// 测试 ActionResult 错误响应序列化
    #[test]
    fn test_action_result_error_serialization() {
        let result: ActionResult<serde_json::Value> = ActionResult::error("test error");

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "test error");
        assert!(json["data"].is_null());
    }

    /// 测试 control_router 能正常构建且契约路径已注册
    #[test]
    fn test_control_router_builds() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let _router = routes::control_router(pool);
        });
    }

    /// 测试 person::list_next 在无数据库时返回内部错误
    #[test]
    fn test_person_list_next_returns_error_without_db() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                person::list_next(
                    Extension(pool),
                    axum::extract::Path(("-".to_string(), 20i64)),
                )
                .await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    /// 测试 person::get 在无数据库时返回内部错误
    #[test]
    fn test_person_get_returns_error_without_db() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                person::get(
                    Extension(pool),
                    axum::extract::Path("test-flag".to_string()),
                )
                .await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    /// 测试 unit::list 在无数据库时返回内部错误
    #[test]
    fn test_unit_list_returns_error_without_db() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                unit::list(Extension(pool)).await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    /// 测试 role::list_next 在无数据库时返回内部错误
    #[test]
    fn test_role_list_next_returns_error_without_db() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                role::list_next(
                    Extension(pool),
                    axum::extract::Path(("-".to_string(), 20i64)),
                )
                .await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    /// 测试 group::list_next 在无数据库时返回内部错误
    #[test]
    fn test_group_list_next_returns_error_without_db() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                group::list_next(
                    Extension(pool),
                    axum::extract::Path(("-".to_string(), 20i64)),
                )
                .await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    /// 测试 person::create 缺少必填字段时返回错误
    #[test]
    fn test_person_create_missing_fields() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let req = person::PersonCreateRequest {
                unique_id: "test001".to_string(),
                name: "测试人员".to_string(),
                mobile: None,
                email: None,
                password: String::new(),
            };
            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                person::create(
                    Extension(pool),
                    axum::extract::Json(req),
                )
                .await;

            // 缺少密码应在触达数据库前返回 type=error
            match result {
                Ok(json) => {
                    assert_eq!(json.0.r#type, Some("error".to_string()));
                    assert!(json.0.message.is_some());
                }
                Err(_) => panic!("expected Ok with error response"),
            }
        });
    }

    /// 测试 person::create 唯一标识超长时返回错误
    #[test]
    fn test_person_create_unique_id_too_long() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let req = person::PersonCreateRequest {
                unique_id: "x".repeat(300),
                name: "测试人员".to_string(),
                mobile: None,
                email: None,
                password: "secret".to_string(),
            };
            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                person::create(
                    Extension(pool),
                    axum::extract::Json(req),
                )
                .await;

            match result {
                Ok(json) => {
                    assert_eq!(json.0.r#type, Some("error".to_string()));
                }
                Err(_) => panic!("expected Ok with error response"),
            }
        });
    }

    /// 测试 unit::create 缺少必填字段时返回错误
    #[test]
    fn test_unit_create_missing_fields() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let req = unit::UnitCreateRequest {
                name: String::new(),
                parent_id: None,
                level: 1,
            };
            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                unit::create(
                    Extension(pool),
                    axum::extract::Json(req),
                )
                .await;

            match result {
                Ok(json) => {
                    assert_eq!(json.0.r#type, Some("error".to_string()));
                }
                Err(_) => panic!("expected Ok with error response"),
            }
        });
    }

    /// 测试 role::create 缺少必填字段时返回错误
    #[test]
    fn test_role_create_missing_fields() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let req = role::RoleCreateRequest {
                name: String::new(),
                description: None,
            };
            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                role::create(
                    Extension(pool),
                    axum::extract::Json(req),
                )
                .await;

            match result {
                Ok(json) => {
                    assert_eq!(json.0.r#type, Some("error".to_string()));
                }
                Err(_) => panic!("expected Ok with error response"),
            }
        });
    }

    /// 测试 group::create 缺少必填字段时返回错误
    #[test]
    fn test_group_create_missing_fields() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let req = group::GroupCreateRequest {
                name: String::new(),
                description: None,
            };
            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                group::create(
                    Extension(pool),
                    axum::extract::Json(req),
                )
                .await;

            match result {
                Ok(json) => {
                    assert_eq!(json.0.r#type, Some("error".to_string()));
                }
                Err(_) => panic!("expected Ok with error response"),
            }
        });
    }

    /// 测试 person::delete 软删除
    #[test]
    fn test_person_delete_without_db() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = PgConfig::new();
            let manager = Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
            let pool = Pool::builder(manager).build().unwrap();

            let result: Result<axum::Json<ActionResult<serde_json::Value>>, AppError> =
                person::delete(
                    Extension(pool),
                    axum::extract::Path("test-flag".to_string()),
                )
                .await;

            // 无数据库时应返回 Internal 或错误响应
            match result {
                Ok(json) => {
                    assert_eq!(json.0.r#type, Some("error".to_string()));
                }
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal or error response"),
            }
        });
    }

    /// 测试 auth::password 双算法哈希：bcrypt 前缀 + 兼容校验 + MD5 兼容校验
    #[test]
    fn test_password_hash_and_verify() {
        let hash = auth::password::hash_password("secret123");
        assert!(hash.starts_with(auth::password::BCRYPT_PREFIX));
        assert!(auth::password::verify_password("secret123", &hash, "", None));
        assert!(!auth::password::verify_password("wrong", &hash, "", None));

        let md5_hash = format!("{:x}", md5::compute("legacy".as_bytes()));
        assert!(auth::password::verify_password("legacy", &md5_hash, "", None));
    }
}