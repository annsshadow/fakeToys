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
//! 四轮 R4-2 追加：本 crate `u2` 面的 16 把工作流实体写——
//! `PUT/DELETE /api/processplatform/service/processing/work/{id}`、
//! `DELETE work/{id}/draft`、`POST work/v3/retract`、`DELETE task/{id}`、
//! `DELETE taskcompleted/{id}`、`DELETE readcompleted/{id}`、
//! `PUT/DELETE record/{id}`、`DELETE review/{id}`、
//! `DELETE/PUT attachment/{id}`、`PUT attachment/edit/{id}/text`、
//! `PUT applicationdict/{id}`、`DELETE job/{job}`、`DELETE snap/{id}`——
//! 原本「登录即可」，任一有效会话按 ID 就能改标题、软删任务、物理删他人 work
//! （含级联删 x_task / x_draft）或动字典 / job / 快照。
//! 现在两档：admin 档（字典、引擎 job、快照）非 admin 403；归属档按行的
//! 发起人 / 负责人 / 作者走 require_owner，且**空归属的脏行只允许 admin**
//! （不用「空归属 = 人人可写」把数据缺陷洗成权限放行）；
//! `work_v3_retract_body` 的留痕 creator 同样由 `"system"` 改会话权威。
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

    // ── R4-2：u2 工作流实体写（改 / 删 / 级联物理删）─────────────────────────

    /// 离线臂：16 把写一律先要会话，无会话 401 且不触库。
    #[tokio::test]
    async fn u2_entity_writes_reject_without_session() {
        let pool = shared::testing::test_pool();
        let p = |id: &str| axum::extract::Path(id.to_string());
        let j = |v: serde_json::Value| Json(v);
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "att_delete_id",
                crate::u2::att_delete_id(Extension(pool.clone()), None, p("a1"))
                    .await
                    .map(|_| ()),
            ),
            (
                "att_edit_id",
                crate::u2::att_edit_id(
                    Extension(pool.clone()),
                    None,
                    p("a1"),
                    j(serde_json::json!({ "name": "x" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "att_edit_text",
                crate::u2::att_edit_text(
                    Extension(pool.clone()),
                    None,
                    p("a1"),
                    j(serde_json::json!({ "text": "x" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "task_delete",
                crate::u2::task_delete(Extension(pool.clone()), None, p("t1"))
                    .await
                    .map(|_| ()),
            ),
            (
                "taskcompleted_delete",
                crate::u2::taskcompleted_delete(Extension(pool.clone()), None, p("r1"))
                    .await
                    .map(|_| ()),
            ),
            (
                "readcompleted_delete",
                crate::u2::readcompleted_delete(Extension(pool.clone()), None, p("r1"))
                    .await
                    .map(|_| ()),
            ),
            (
                "record_edit",
                crate::u2::record_edit(
                    Extension(pool.clone()),
                    None,
                    p("c1"),
                    j(serde_json::json!({ "content": "x" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "record_delete",
                crate::u2::record_delete(Extension(pool.clone()), None, p("c1"))
                    .await
                    .map(|_| ()),
            ),
            (
                "review_delete",
                crate::u2::review_delete(Extension(pool.clone()), None, p("v1"))
                    .await
                    .map(|_| ()),
            ),
            (
                "work_edit",
                crate::u2::work_edit(
                    Extension(pool.clone()),
                    None,
                    p("w1"),
                    j(serde_json::json!({ "title": "x" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_delete",
                crate::u2::work_delete(Extension(pool.clone()), None, p("w1"))
                    .await
                    .map(|_| ()),
            ),
            (
                "work_draft_delete",
                crate::u2::work_draft_delete(Extension(pool.clone()), None, p("w1"))
                    .await
                    .map(|_| ()),
            ),
            (
                "work_v3_retract_body",
                crate::u2::work_v3_retract_body(
                    Extension(pool.clone()),
                    None,
                    j(serde_json::json!({ "work": "w1" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_edit",
                crate::u2::dict_edit(
                    Extension(pool.clone()),
                    None,
                    p("d1"),
                    j(serde_json::json!({ "name": "x" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "job_delete",
                crate::u2::job_delete(Extension(pool.clone()), None, p("j1"))
                    .await
                    .map(|_| ()),
            ),
            (
                "snap_delete",
                crate::u2::snap_delete(Extension(pool.clone()), None, p("s1"))
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

    /// 离线臂：admin 档三把（字典表 / 引擎 job / snap）非 admin → 403。
    /// `is_admin` 查不到人即 false，故本臂不需要 admin 数据即可证拒绝侧。
    #[tokio::test]
    async fn u2_admin_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let s = Some(Extension(sess("u-r42-plain")));
        let dict = crate::u2::dict_edit(
            Extension(pool.clone()),
            s.clone(),
            axum::extract::Path("d1".to_string()),
            Json(serde_json::json!({ "name": "x" })),
        )
        .await;
        let job = crate::u2::job_delete(
            Extension(pool.clone()),
            s.clone(),
            axum::extract::Path("j1".to_string()),
        )
        .await;
        let snap = crate::u2::snap_delete(
            Extension(pool.clone()),
            s,
            axum::extract::Path("s1".to_string()),
        )
        .await;
        assert!(matches!(dict, Err(AppError::Forbidden)), "非 admin 改字典必须 403");
        assert!(matches!(job, Err(AppError::Forbidden)), "非 admin 结束他人 job 必须 403");
        assert!(matches!(snap, Err(AppError::Forbidden)), "非 admin 删快照必须 403");
    }

    /// 造一个真正带 admin 角色的人（`is_admin` 按 auth_role.name='admin' 判定），
    /// 返回 (person, role_id) 供臂末清理。
    async fn seed_admin(pool: &Pool, tag: &str) -> (String, String) {
        let client = pool.get().await.unwrap();
        let person = format!("u-r42-{tag}-admin");
        let role_id = format!("r42-role-{tag}");
        client
            .execute(
                "INSERT INTO auth_role (id, name, disable) VALUES ($1, 'admin', false)",
                &[&role_id],
            )
            .await
            .unwrap();
        client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                 VALUES ($1, $2, $3, 'x') ON CONFLICT (id) DO NOTHING",
                &[&person, &person, &person],
            )
            .await
            .unwrap();
        client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ($1, $2, $3) \
                 ON CONFLICT DO NOTHING",
                &[&person, &role_id, &format!("r42-unit-{tag}")],
            )
            .await
            .unwrap();
        (person, role_id)
    }

    async fn unseed_admin(pool: &Pool, person: &str, role_id: &str) {
        let client = pool.get().await.unwrap();
        client
            .execute("DELETE FROM auth_person_role WHERE person_id = $1", &[&person.to_string()])
            .await
            .unwrap();
        client
            .execute("DELETE FROM auth_person WHERE id = $1", &[&person.to_string()])
            .await
            .unwrap();
        client
            .execute("DELETE FROM auth_role WHERE id = $1", &[&role_id])
            .await
            .unwrap();
    }

    /// DB 臂：非发起人删他人 work 必须 403，且级联物理删一行都不许动；
    /// 发起人放行后 x_work / x_task / x_draft 同批消失。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn work_delete_requires_work_creator() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let work_id = format!("r42-w-{}", uuid::Uuid::new_v4());
        let task_id = format!("r42-t-{}", uuid::Uuid::new_v4());
        let creator = "u-r42-creator";
        seed_work(&pool, &work_id, creator).await;
        seed_task(&pool, &task_id, &work_id, "u-r42-assignee", "active").await;
        {
            let client = pool.get().await.unwrap();
            client
                .execute(
                    "INSERT INTO x_draft (id, work_id, creator) VALUES ($1, $2, $3)",
                    &[&format!("{work_id}-d"), &work_id, &creator.to_string()],
                )
                .await
                .unwrap();
        }

        let intruder = crate::u2::work_delete(
            Extension(pool.clone()),
            Some(Extension(sess("u-r42-intruder"))),
            axum::extract::Path(work_id.clone()),
        )
        .await;
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非发起人删他人 work 必须 403"
        );
        let client = pool.get().await.unwrap();
        let alive: i64 = client
            .query_one("SELECT COUNT(*) AS c FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .get("c");
        assert_eq!(alive, 1, "403 之后 work 行必须还在（级联物理删一行都不能动）");
        let drafts: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_draft WHERE work_id = $1",
                &[&work_id],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(drafts, 1, "403 之后草稿必须还在");

        let owner = crate::u2::work_delete(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            axum::extract::Path(work_id.clone()),
        )
        .await;
        assert!(owner.is_ok(), "发起人删除自己的 work 应放行");
        let gone: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_work WHERE id = $1",
                &[&work_id],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(gone, 0, "删除须落地");
        let tasks_left: i64 = client
            .query_one("SELECT COUNT(*) AS c FROM x_task WHERE work = $1", &[&work_id])
            .await
            .unwrap()
            .get("c");
        assert_eq!(tasks_left, 0, "级联须删掉任务");

        client
            .execute("DELETE FROM x_draft WHERE work_id = $1", &[&work_id])
            .await
            .unwrap();
        client
            .execute("DELETE FROM x_task WHERE id = $1", &[&task_id])
            .await
            .unwrap();
        client
            .execute("DELETE FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap();
    }

    /// DB 臂：非负责人软删他人任务必须 403 且 `deleted_at` 保持为空；
    /// 负责人放行后 `deleted_at` 落。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn task_delete_requires_task_person() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let work_id = format!("r42-w-{}", uuid::Uuid::new_v4());
        let task_id = format!("r42-t-{}", uuid::Uuid::new_v4());
        let assignee = "u-r42-assignee";
        seed_work(&pool, &work_id, "u-r42-creator").await;
        seed_task(&pool, &task_id, &work_id, assignee, "active").await;

        let intruder = crate::u2::task_delete(
            Extension(pool.clone()),
            Some(Extension(sess("u-r42-intruder"))),
            axum::extract::Path(task_id.clone()),
        )
        .await;
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非负责人删他人任务必须 403"
        );
        let client = pool.get().await.unwrap();
        let not_deleted: bool = client
            .query_one(
                "SELECT (deleted_at IS NULL) AS nd FROM x_task WHERE id = $1",
                &[&task_id],
            )
            .await
            .unwrap()
            .get("nd");
        assert!(not_deleted, "403 之后任务不得被软删");

        let owner = crate::u2::task_delete(
            Extension(pool.clone()),
            Some(Extension(sess(assignee))),
            axum::extract::Path(task_id.clone()),
        )
        .await;
        assert!(owner.is_ok(), "负责人删除自己的任务应放行");
        let deleted: bool = client
            .query_one(
                "SELECT (deleted_at IS NOT NULL) AS d FROM x_task WHERE id = $1",
                &[&task_id],
            )
            .await
            .unwrap()
            .get("d");
        assert!(deleted, "软删须落地");

        cleanup(&pool, &work_id, &task_id).await;
    }

    /// DB 臂：非作者物理删他人留痕必须 403 且行仍在；作者放行后行消失。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn record_delete_requires_record_creator() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let work_id = format!("r42-w-{}", uuid::Uuid::new_v4());
        let record_id = format!("r42-c-{}", uuid::Uuid::new_v4());
        let author = "u-r42-author";
        seed_work(&pool, &work_id, "u-r42-creator").await;
        {
            let client = pool.get().await.unwrap();
            client
                .execute(
                    "INSERT INTO x_record (id, work_id, record_type, content, creator) \
                     VALUES ($1, $2, $3, $4, $5)",
                    &[
                        &record_id,
                        &work_id,
                        &"info".to_string(),
                        &"r42".to_string(),
                        &author.to_string(),
                    ],
                )
                .await
                .unwrap();
        }

        let intruder = crate::u2::record_delete(
            Extension(pool.clone()),
            Some(Extension(sess("u-r42-intruder"))),
            axum::extract::Path(record_id.clone()),
        )
        .await;
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非作者删他人留痕必须 403"
        );
        let client = pool.get().await.unwrap();
        let alive: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_record WHERE id = $1",
                &[&record_id],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(alive, 1, "403 之后留痕行必须还在（物理删未执行）");

        let owner = crate::u2::record_delete(
            Extension(pool.clone()),
            Some(Extension(sess(author))),
            axum::extract::Path(record_id.clone()),
        )
        .await;
        assert!(owner.is_ok(), "作者删除自己的留痕应放行");
        let gone: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_record WHERE id = $1",
                &[&record_id],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(gone, 0, "删除须落地");

        cleanup(&pool, &work_id, "r42-none").await;
    }

    /// DB 臂：空发起人的 work（历史脏行）走严格口径——非 admin 一律 403，
    /// 真 admin（auth_role.name='admin'）才放行。证明「空归属」没被写成人人可改。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn work_edit_with_empty_creator_requires_admin() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let work_id = format!("r42o-w-{}", uuid::Uuid::new_v4());
        let (admin, role_id) = seed_admin(&pool, "ownerless").await;
        {
            let client = pool.get().await.unwrap();
            client
                .execute(
                    "INSERT INTO x_work (id, title, process, application, work_status, creator) \
                     VALUES ($1, $2, $3, $4, $5, '')",
                    &[
                        &work_id,
                        &"r4-2 orphan".to_string(),
                        &"r42o".to_string(),
                        &"r42o".to_string(),
                        &"processing".to_string(),
                    ],
                )
                .await
                .unwrap();
        }

        let plain = crate::u2::work_edit(
            Extension(pool.clone()),
            Some(Extension(sess("u-r42-plain"))),
            axum::extract::Path(work_id.clone()),
            Json(serde_json::json!({ "title": "越权改名" })),
        )
        .await;
        assert!(
            matches!(plain, Err(AppError::Forbidden)),
            "空发起人的 work 非 admin 改名必须 403"
        );
        let client = pool.get().await.unwrap();
        let title: String = client
            .query_one("SELECT title FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .get("title");
        assert_eq!(title, "r4-2 orphan", "403 之后标题不得被改");

        let admin_res = crate::u2::work_edit(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
            axum::extract::Path(work_id.clone()),
            Json(serde_json::json!({ "title": "管理员整理" })),
        )
        .await;
        assert!(admin_res.is_ok(), "admin 整理无主 work 应放行");
        let title: String = client
            .query_one("SELECT title FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .get("title");
        assert_eq!(title, "管理员整理", "admin 侧写须真落地");

        client
            .execute("DELETE FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap();
        unseed_admin(&pool, &admin, &role_id).await;
    }

    /// DB 臂：admin 档三把的放行侧——真 admin 改字典确实落库（拒绝侧由离线臂证）。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn dict_edit_allowed_for_admin_and_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let dict_id = format!("r42-d-{}", uuid::Uuid::new_v4());
        let (admin, role_id) = seed_admin(&pool, "dict").await;

        let res = crate::u2::dict_edit(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
            axum::extract::Path(dict_id.clone()),
            Json(serde_json::json!({ "name": "r42 字典" })),
        )
        .await;
        assert!(res.is_ok(), "admin 写字典应放行");
        let client = pool.get().await.unwrap();
        let name: String = client
            .query_one(
                "SELECT name FROM x_application_dict WHERE id = $1",
                &[&dict_id],
            )
            .await
            .unwrap()
            .get("name");
        assert_eq!(name, "r42 字典", "字典名须真落库");

        client
            .execute("DELETE FROM x_application_dict WHERE id = $1", &[&dict_id])
            .await
            .unwrap();
        unseed_admin(&pool, &admin, &role_id).await;
    }
}
