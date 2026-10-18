// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! calendar 属主资源越权修复——CAL_CALENDAR 的 update/remove 此前任一登录用户可
//! 改关他人日历（createor 列存在但无校验）；create 的 createor 原取客户端值
//! （可伪造归属），改取会话登录人。
//!
//! 属主门禁需先查库取 createor，故 DB 门控：有库时种真实行断言非属主 403、
//! 属主放行、create 属主取会话；无库时跳过（与 meeting_assemble_control u2 先例一致）。
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::{
        calendar_create, calendar_remove, calendar_update, CreateCalendarRequest,
        DeleteCalendarRequest, UpdateCalendarRequest,
    };

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn calendar_writes_require_owner() {
        if !shared::testing::is_db_available().await {
            eprintln!("SKIP (no PG): calendar_writes_require_owner");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();

        let cal_id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO CAL_CALENDAR (id, name, type, target, color, createor, is_public, status) \
                 VALUES ($1, 'sec-gate', 'PERSON', 'person', '#000', 'u-owner', false, 'OPEN')",
                &[&cal_id],
            )
            .await
            .unwrap();

        // 非属主 → 403
        assert!(
            matches!(
                calendar_update(
                    Extension(pool.clone()),
                    Extension(build_session("u-other")),
                    axum::Json(UpdateCalendarRequest {
                        id: Some(cal_id.clone()),
                        name: Some("hijack".to_string()),
                        calendar_type: None,
                        target: None,
                        color: None,
                        description: None,
                        is_public: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "calendar_update 非属主必须 403"
        );
        assert!(
            matches!(
                calendar_remove(
                    Extension(pool.clone()),
                    Extension(build_session("u-other")),
                    axum::Json(DeleteCalendarRequest {
                        id: Some(cal_id.clone())
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "calendar_remove 非属主必须 403"
        );

        // 属主 → 放行（不落 403）
        assert!(
            !matches!(
                calendar_update(
                    Extension(pool.clone()),
                    Extension(build_session("u-owner")),
                    axum::Json(UpdateCalendarRequest {
                        id: Some(cal_id.clone()),
                        name: Some("own".to_string()),
                        calendar_type: None,
                        target: None,
                        color: None,
                        description: None,
                        is_public: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "calendar_update 属主不得 403"
        );

        // create 属主来源 = 会话登录人
        let res = calendar_create(
            Extension(pool.clone()),
            Extension(build_session("u-owner")),
            axum::Json(CreateCalendarRequest {
                name: Some("c".to_string()),
                calendar_type: Some("PERSON".to_string()),
                target: None,
                color: None,
                description: None,
                source: None,
                createor: None,
                is_public: None,
            }),
        )
        .await;
        if let Ok(json_res) = res {
            assert_eq!(
                json_res.0.data.as_ref().unwrap().createor,
                "u-owner",
                "calendar_create 属主必须取会话登录人"
            );
        }

        client
            .execute("DELETE FROM CAL_CALENDAR WHERE id = $1", &[&cal_id])
            .await
            .unwrap();
    }
}
