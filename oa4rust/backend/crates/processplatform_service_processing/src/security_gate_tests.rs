// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 R4-1）：引擎运行面的 14 把状态/归属写
//! （`POST /api/task/{id}/claim|complete|reject`、`POST /api/task/{id}/transfer/{person}`、
//! `POST /api/work/{id}/terminate|retract`、
//! `GET /api/processplatform/service/processing/v3/retract/{work}`（物理状态写注册在 GET 上，
//! 方法本身属 A3-N12 GET-写族，本批先补归属门）、
//! `POST /api/work/{id}/start|complete`、`PUT /api/work/{id}/processing`、
//! `POST /api/processplatform/service/processing/execute|cancel/{id}`、
//! `POST /api/gateway/{work_id}/{token}/join`、`POST .../gateway/fork/{gateway_instance_id}`）
//! 此前既不取 Session 也不校归属：任一登录用户按 ID 即可完成/退回/转办他人待办，
//! 按 ID 启动、推进、终止或撤销他人流程。
//! 现在：无会话 401（不触库，可离线证）；有会话但非任务负责人 / 非 work 发起人 → 403
//! （`shared::middleware::require_owner`，admin 旁路）；gateway fork 属引擎编排面、
//! 一次跨多个 work 批量插任务且无单一属主，收 admin。
//! 顺带收口冒名：`task_claim` / `work_v2_id_terminate` / `gateway_join` 的
//! `x_record.creator` 原硬编码 `"system"`，现写真实操作人。
//!
//! DB 依赖臂需真实 PostgreSQL：
//! `cargo test --lib -p processplatform_service_processing -- --ignored`
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use axum::Json;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    fn sess(person: &str) -> shared::session::Session {
        shared::testing::test_session(person)
    }

    async fn seed_work(pool: &Pool, work_id: &str, creator: &str) {
        let client = pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO x_work (id, title, process, application, work_status, creator) \
                 VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &work_id.to_string(),
                    &"r4-1 gate".to_string(),
                    &"r41".to_string(),
                    &"r41".to_string(),
                    &"processing".to_string(),
                    &creator.to_string(),
                ],
            )
            .await
            .unwrap();
    }

    async fn seed_task(pool: &Pool, task_id: &str, work_id: &str, person: &str, status: &str) {
        let client = pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[
                    &task_id.to_string(),
                    &"r4-1 task".to_string(),
                    &work_id.to_string(),
                    &"approve".to_string(),
                    &"t1".to_string(),
                    &person.to_string(),
                    &status.to_string(),
                ],
            )
            .await
            .unwrap();
    }

    async fn cleanup(pool: &Pool, work_id: &str, task_id: &str) {
        let client = pool.get().await.unwrap();
        for sql in [
            "DELETE FROM x_record WHERE work_id = $1",
            "DELETE FROM x_task WHERE id = $1",
            "DELETE FROM x_work WHERE id = $1",
        ] {
            let arg = if sql.contains("x_task WHERE id") { task_id } else { work_id };
            client.execute(sql, &[&arg.to_string()]).await.unwrap();
        }
    }

    /// 离线臂：7 把写全部要求有效会话，无会话 401 且不触库。
    #[tokio::test]
    async fn engine_runtime_writes_reject_without_session() {
        let pool = shared::testing::test_pool();
        let cases: [(&str, Result<(), AppError>); 7] = [
            (
                "task_claim",
                crate::task_claim(Extension(pool.clone()), None, axum::extract::Path("t1".into()))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_complete",
                crate::task_complete(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("t1".into()),
                    None,
                )
                .await
                .map(|_| ()),
            ),
            (
                "task_reject",
                crate::task_reject(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("t1".into()),
                    Some(Json(serde_json::json!({"opinion": "x"}))),
                )
                .await
                .map(|_| ()),
            ),
            (
                "task_transfer",
                crate::task_transfer(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path(("t1".into(), "u-other".into())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_v2_id_terminate",
                crate::work_v2_id_terminate(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".into()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_v2_id_retract",
                crate::work_v2_id_retract(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".into()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_v3_retract",
                crate::work_v3_retract(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".into()),
                )
                .await
                .map(|_| ()),
            ),
        ];
        for (name, res) in cases {
            assert!(
                matches!(res, Err(AppError::Unauthorized)),
                "{name} 无会话必须 401"
            );
        }
    }

    /// 离线臂：work/实例推进族（execute、cancel、work start/complete/processing、gateway join/fork）
    /// 同样一律 401，不落到 SQL。
    #[tokio::test]
    async fn instance_progression_writes_reject_without_session() {
        let pool = shared::testing::test_pool();
        let cases: [(&str, Result<(), AppError>); 7] = [
            (
                "execute_process",
                crate::execute_process(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".into()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "cancel_process_instance",
                crate::cancel_process_instance(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".into()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_id_processing",
                crate::work_id_processing(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".into()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_start",
                crate::work_start(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".into()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_complete",
                crate::work_complete(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".into()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "gateway_join",
                crate::gateway_join(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path(("w1".into(), "t1".into())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "gateway_fork",
                crate::gateway_fork(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("g1".into()),
                )
                .await
                .map(|_| ()),
            ),
        ];
        for (name, res) in cases {
            assert!(
                matches!(res, Err(AppError::Unauthorized)),
                "{name} 无会话必须 401"
            );
        }
    }

    /// 离线臂：并行网关 fork 是引擎编排面（一次按 gateway 实例批量插任务、跨多个 work），
    /// 收 admin；`is_admin` 在查不到人 / 无库时都 fail-closed，故本臂离线即可证。
    #[tokio::test]
    async fn gateway_fork_forbidden_for_non_admin() {
        let res = crate::gateway_fork(
            Extension(shared::testing::test_pool()),
            Some(Extension(sess("u-nonadmin"))),
            axum::extract::Path("g1".into()),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非 admin 触发 gateway fork 必须 403"
        );
    }

    /// DB 臂：非发起人启动他人 work 必须 403，且不得留下状态写与 Start Event 任务；
    /// 发起人放行后 work 置 processing、任务确实创建。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn work_start_requires_work_creator() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let work_id = format!("r41b-w-{}", uuid::Uuid::new_v4());
        let creator = "u-r41b-creator";
        let client = pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO x_work (id, title, process, application, work_status, creator) \
                 VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &work_id.to_string(),
                    &"r4-1b gate".to_string(),
                    &"r41b".to_string(),
                    &"r41b".to_string(),
                    &"pending".to_string(),
                    &creator.to_string(),
                ],
            )
            .await
            .unwrap();

        let intruder = crate::work_start(
            Extension(pool.clone()),
            Some(Extension(sess("u-r41b-intruder"))),
            axum::extract::Path(work_id.clone()),
        )
        .await;
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非发起人启动他人 work 必须 403"
        );
        let status: String = client
            .query_one("SELECT work_status FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .get("work_status");
        assert_eq!(status, "pending", "403 之后 work 不得被启动");

        let owner = crate::work_start(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            axum::extract::Path(work_id.clone()),
        )
        .await;
        assert!(owner.is_ok(), "发起人启动自己的 work 应放行");
        let status: String = client
            .query_one("SELECT work_status FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .get("work_status");
        assert_eq!(status, "processing", "启动状态须落地");

        client
            .execute("DELETE FROM x_task WHERE work = $1", &[&work_id])
            .await
            .unwrap();
        client
            .execute("DELETE FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap();
    }

    /// DB 臂：非负责人退回他人任务必须 403；负责人本人放行，且审批记录 creator
    /// 落的是负责人（不是 "system"，也不是请求体传值）。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn task_reject_requires_task_assignee_and_pins_real_actor() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let work_id = format!("r41-w-{}", uuid::Uuid::new_v4());
        let task_id = format!("r41-t-{}", uuid::Uuid::new_v4());
        let assignee = "u-r41-assignee";
        seed_work(&pool, &work_id, "u-r41-creator").await;
        seed_task(&pool, &task_id, &work_id, assignee, "active").await;

        let intruder = crate::task_reject(
            Extension(pool.clone()),
            Some(Extension(sess("u-r41-intruder"))),
            axum::extract::Path(task_id.clone()),
            Some(Json(serde_json::json!({"opinion": "越权退回"}))),
        )
        .await;
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非负责人退回他人任务必须 403"
        );

        let owner = crate::task_reject(
            Extension(pool.clone()),
            Some(Extension(sess(assignee))),
            axum::extract::Path(task_id.clone()),
            Some(Json(serde_json::json!({"opinion": "事实不符"}))),
        )
        .await;
        assert!(owner.is_ok(), "负责人退回自己的任务应放行");

        let client = pool.get().await.unwrap();
        let status: String = client
            .query_one("SELECT task_status FROM x_task WHERE id = $1", &[&task_id])
            .await
            .unwrap()
            .get("task_status");
        assert_eq!(status, "rejected", "退回状态须落地");
        let record_creator: String = client
            .query_one(
                "SELECT creator FROM x_record WHERE task_id = $1 AND record_type = $2",
                &[&task_id, &"reject".to_string()],
            )
            .await
            .unwrap()
            .get("creator");
        assert_eq!(
            record_creator, assignee,
            "审批记录必须记真实负责人，不能是 system 或客户端传值"
        );

        cleanup(&pool, &work_id, &task_id).await;
    }

    /// DB 臂：非发起人撤销他人 work 必须 403；发起人放行且 work 与其 task 同批落状态。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn work_retract_requires_work_creator() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let work_id = format!("r41-w-{}", uuid::Uuid::new_v4());
        let task_id = format!("r41-t-{}", uuid::Uuid::new_v4());
        let creator = "u-r41-creator";
        seed_work(&pool, &work_id, creator).await;
        seed_task(&pool, &task_id, &work_id, "u-r41-assignee", "active").await;

        let intruder = crate::work_v2_id_retract(
            Extension(pool.clone()),
            Some(Extension(sess("u-r41-intruder"))),
            axum::extract::Path(work_id.clone()),
        )
        .await;
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非发起人撤销他人 work 必须 403"
        );
        // 拒绝路径不得留下任何状态写
        let client = pool.get().await.unwrap();
        let status: String = client
            .query_one("SELECT work_status FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .get("work_status");
        assert_eq!(status, "processing", "403 之后 work 状态不得变化");

        let owner = crate::work_v2_id_retract(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            axum::extract::Path(work_id.clone()),
        )
        .await;
        assert!(owner.is_ok(), "发起人撤销自己的 work 应放行");
        let status: String = client
            .query_one("SELECT work_status FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .get("work_status");
        assert_eq!(status, "retracted", "撤销状态须落地");
        let task_status: String = client
            .query_one("SELECT task_status FROM x_task WHERE id = $1", &[&task_id])
            .await
            .unwrap()
            .get("task_status");
        assert_eq!(task_status, "cancelled", "撤销须连带取消在办任务");

        cleanup(&pool, &work_id, &task_id).await;
    }
}
