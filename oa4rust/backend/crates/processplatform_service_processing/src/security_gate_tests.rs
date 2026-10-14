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
//! 四轮 R4-2 第二批追加：同面再 13 把写（前缀均为
//! `/api/processplatform/service/processing`）——
//! `DELETE attachment/{id}/work/{workId}`、`DELETE attachment/{id}/workcompleted/{workCompletedId}`、
//! `POST attachment/copy/work/{workId}`、
//! `DELETE read/{id}`、`PUT read/{id}/processing`、`POST read/{id}/replace`、`POST read/{id}/reset`、
//! `POST documentversion/work/{work}`、
//! `GET work/{id}/series/{series}/activitytoken/{activityToken}/processing/signal`（GET 上挂状态写，属 A3-N12 族，本批先补归属门）、
//! `POST work/v2/{id}/add/manual/task/identity/matrix`、
//! `POST work`、`POST work/process/{processId}`、
//! `POST work/process/{processId}/name/{name}/serial`——
//! 原为「登录即可」，任一会话可按 ID 删他人附件、把他人已读回执搬进 completed、
//! 给他人 work 造版本行或插身份矩阵。
//! 现在：附件按 `x_attachment.creator` 校归属，且**在原有 IDOR 等值校验之后**再校，
//! 两道门都在；`attachment/copy` 校**目标** work 的发起人（复制是往目标写）；
//! 已读族按 `x_read.person` 严格校；版本 / 身份矩阵 / 运行信号在事务内
//! `SELECT … FOR UPDATE` 锁行后校归属；建单三只的 creator 由 body 传入改会话权威
//! （`work_start_impl` 现收 `person`，冒名建单不再生效）。
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

    /// R4-2 第二批离线臂：阅读回执 / 附件复制 / 文档版本 / 信号与建单族 13 把，
    /// 无会话一律 401 且在取连接之前就退回。
    #[tokio::test]
    async fn u2_read_copy_and_create_writes_reject_without_session() {
        let pool = shared::testing::test_pool();
        let s = Json(serde_json::json!({ "person": "someone" }));
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "att_delete_with_work",
                crate::u2::att_delete_with_work(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path(("a1".to_string(), "w1".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "att_delete_with_workcompleted",
                crate::u2::att_delete_with_workcompleted(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path(("a1".to_string(), "wc1".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "att_copy_to_work",
                crate::u2::att_copy_to_work(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".to_string()),
                    Json(serde_json::json!({ "attachmentList": ["a1"] })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "read_delete",
                crate::u2::read_delete(Extension(pool.clone()), None, axum::extract::Path("r1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "read_processing",
                crate::u2::read_processing(Extension(pool.clone()), None, axum::extract::Path("r1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "read_replace",
                crate::u2::read_replace(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("r1".to_string()),
                    s.clone(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "read_reset",
                crate::u2::read_reset(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("r1".to_string()),
                    s.clone(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "documentversion_create",
                crate::u2::documentversion_create(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".to_string()),
                    s.clone(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_processing_signal",
                crate::u2::work_processing_signal(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        "w1".to_string(),
                        "s1".to_string(),
                        "t1".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_add_identity_matrix",
                crate::u2::work_add_identity_matrix(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("w1".to_string()),
                    Json(serde_json::json!({ "identityList": ["p@P"] })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_create",
                crate::u2::work_create(Extension(pool.clone()), None, s.clone())
                    .await
                    .map(|_| ()),
            ),
            (
                "work_create_for_process",
                crate::u2::work_create_for_process(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("p1".to_string()),
                    s.clone(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_create_with_serial",
                crate::u2::work_create_with_serial(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path(("p1".to_string(), "n1".to_string())),
                    s,
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

    /// DB 臂：非目标 work 发起人不得复制附件进去（403 且零写入）；
    /// 发起人放行后 `x_attachment.work_id` 落到目标 work。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn att_copy_requires_target_work_creator() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let src_work = format!("r42c-src-{}", uuid::Uuid::new_v4());
        let dst_work = format!("r42c-dst-{}", uuid::Uuid::new_v4());
        let src_att = format!("r42c-a-{}", uuid::Uuid::new_v4());
        let owner = "u-r42c-owner";
        seed_work(&pool, &src_work, "u-r42c-other").await;
        seed_work(&pool, &dst_work, owner).await;
        {
            let client = pool.get().await.unwrap();
            client
                .execute(
                    "INSERT INTO x_attachment (id, work_id, name, creator) VALUES ($1, $2, $3, $4)",
                    &[
                        &src_att,
                        &src_work,
                        &"r42c.pdf".to_string(),
                        &"u-r42c-other".to_string(),
                    ],
                )
                .await
                .unwrap();
        }

        let stranger = crate::u2::att_copy_to_work(
            Extension(pool.clone()),
            Some(Extension(sess("u-r42c-stranger"))),
            axum::extract::Path(dst_work.clone()),
            Json(serde_json::json!({ "attachmentList": [src_att.clone()] })),
        )
        .await;
        assert!(
            matches!(stranger, Err(AppError::Forbidden)),
            "非目标 work 发起人不得往里复制附件"
        );
        let client = pool.get().await.unwrap();
        let landed: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_attachment WHERE work_id = $1",
                &[&dst_work],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(landed, 0, "403 之后目标 work 不得多出附件");

        let ok_res = crate::u2::att_copy_to_work(
            Extension(pool.clone()),
            Some(Extension(sess(owner))),
            axum::extract::Path(dst_work.clone()),
            Json(serde_json::json!({ "attachmentList": [src_att.clone()] })),
        )
        .await;
        assert!(ok_res.is_ok(), "目标 work 发起人复制附件应放行");
        let landed: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_attachment WHERE work_id = $1",
                &[&dst_work],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(landed, 1, "放行侧须真落库");

        client
            .execute(
                "DELETE FROM x_attachment WHERE work_id = ANY($1)",
                &[&vec![src_work.clone(), dst_work.clone()]],
            )
            .await
            .unwrap();
        for w in [src_work, dst_work] {
            client
                .execute("DELETE FROM x_work WHERE id = $1", &[&w])
                .await
                .unwrap();
        }
    }

    /// 四轮 R4-2 追加：两把 touch 路由（`PUT …/service/work/{id}/touch`、
    /// `POST …/service/touch/{work}/{id}`）经薄封装转发到 `service_work_id_touch_core`，
    /// core 原本既不取会话也不校归属，任一有效会话按 work ID 就能往他人 work 插
    /// `x_record` 留痕，且 creator 硬写 `"system"`。无会话 → 401（先于任何 SQL）。
    #[tokio::test]
    async fn touch_writes_reject_without_session() {
        let pool = shared::testing::test_pool();
        let cases: [(&str, Result<(), AppError>); 2] = [
            (
                "service_work_id_touch",
                crate::service_work_id_touch(
                    Extension(pool.clone()),
                    axum::extract::Path("w1".into()),
                    None,
                )
                .await
                .map(|_| ()),
            ),
            (
                "service_work_id_touch_p2",
                crate::service_work_id_touch_p2(
                    Extension(pool.clone()),
                    axum::extract::Path(("w1".into(), "w2".into())),
                    None,
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

    /// 实测（需真实 PG）：闯入者 touch 他人 work → 403 **且一条留痕都不落**；
    /// 发起人本人 touch → 放行，且 `x_record.creator` 是本人而不是 `"system"`。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn touch_requires_work_creator_and_pins_real_actor() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: touch_requires_work_creator_and_pins_real_actor (no DATABASE_URL)");
            return;
        }
        let pool = shared::testing::test_pool();
        let owner = "r47-touch-owner";
        let work = "r47-touch-work";
        seed_work(&pool, work, owner).await;

        let intruder = crate::service_work_id_touch(
            Extension(pool.clone()),
            axum::extract::Path(work.to_string()),
            Some(Extension(sess("r47-touch-intruder"))),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非发起人 touch 他人 work 必须 403（Ok 或其它 Err 都算门失效）"
        );

        let c = pool.get().await.unwrap();
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) AS c FROM x_record WHERE work_id = $1",
                &[&work.to_string()],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 0, "拒绝路径不得留下任何 x_record 留痕");

        let ok = crate::service_work_id_touch(
            Extension(pool.clone()),
            axum::extract::Path(work.to_string()),
            Some(Extension(sess(owner))),
        )
        .await;
        assert!(ok.is_ok(), "发起人本人 touch 应放行");
        let rec_creator: Option<String> = c
            .query_opt(
                "SELECT creator FROM x_record WHERE work_id = $1 ORDER BY create_time DESC LIMIT 1",
                &[&work.to_string()],
            )
            .await
            .unwrap()
            .map(|r| r.get("creator"));
        assert_eq!(
            rec_creator.as_deref(),
            Some(owner),
            "留痕 creator 必须是真实操作人，不能硬写 system"
        );

        c.execute("DELETE FROM x_record WHERE work_id = $1", &[&work.to_string()])
            .await
            .unwrap();
        c.execute("DELETE FROM x_work WHERE id = $1", &[&work.to_string()])
            .await
            .unwrap();
    }

    /// 四轮 N-01 活口（第八批）：`review/create/work`、`review/create/workcompleted`、
    /// `review/init/review` 三把 `x_review` 写既不取会话也不校归属，reviewer 由请求体
    /// 任意指定——任一匿名/登录用户可给任意 work 挂评审人（伪造审批人，且该 (work,person)
    /// 之后永久去重、真人无法再入列）。审批链按系统级数据收 admin：无会话 401、非 admin 403。
    #[tokio::test]
    async fn review_chain_writes_reject_without_admin() {
        let pool = shared::testing::test_pool();
        let body = || Json(serde_json::json!({ "work": "w1", "personList": ["p1"] }));

        let anon_cases: [(&str, Result<(), AppError>); 4] = [
            (
                "u2::review_create_work",
                crate::u2::review_create_work(Extension(pool.clone()), None, body())
                    .await
                    .map(|_| ()),
            ),
            (
                "u2::review_create_workcompleted",
                crate::u2::review_create_workcompleted(
                    Extension(pool.clone()),
                    None,
                    Json(serde_json::json!({ "workCompleted": "w1", "personList": ["p1"] })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2::review_init_for_view",
                crate::u2::review_init_for_view(Extension(pool.clone()), None)
                    .await
                    .map(|_| ()),
            ),
            (
                "review_init_review",
                crate::review_init_review(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("r1".to_string()),
                )
                .await
                .map(|_| ()),
            ),
        ];
        for (name, res) in &anon_cases {
            assert!(
                matches!(res, Err(AppError::Unauthorized)),
                "{name} 无会话必须 401（先于任何 SQL），实际 {res:?}"
            );
        }

        let nonadmin_cases: [(&str, Result<(), AppError>); 3] = [
            (
                "u2::review_create_work",
                crate::u2::review_create_work(
                    Extension(pool.clone()),
                    Some(Extension(sess("u-r48-nonadmin"))),
                    body(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2::review_create_workcompleted",
                crate::u2::review_create_workcompleted(
                    Extension(pool.clone()),
                    Some(Extension(sess("u-r48-nonadmin"))),
                    Json(serde_json::json!({ "workCompleted": "w1", "personList": ["p1"] })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2::review_init_for_view",
                crate::u2::review_init_for_view(
                    Extension(pool.clone()),
                    Some(Extension(sess("u-r48-nonadmin"))),
                )
                .await
                .map(|_| ()),
            ),
        ];
        for (name, res) in &nonadmin_cases {
            assert!(
                matches!(res, Err(AppError::Forbidden)),
                "{name} 非 admin 必须 403（无库时 is_admin fail-closed），实际 {res:?}"
            );
        }
    }

    /// DB 臂：非 admin 触发评审补齐必须 403 **且 x_review 一行都不增**；
    /// admin 触发才真正建行，臂末清理。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn review_init_for_view_requires_admin_and_lands_no_rows_otherwise() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: review_init_for_view_requires_admin_and_lands_no_rows_otherwise (no DATABASE_URL)");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let work = format!("r48-{tag}-work");
        let assignee = format!("u-r48-{tag}-assignee");
        seed_work(&pool, &work, "u-r48-creator").await;
        seed_task(&pool, &format!("r48-{tag}-task"), &work, &assignee, "active").await;

        let before: i64 = pool
            .get()
            .await
            .unwrap()
            .query_one("SELECT COUNT(*) AS c FROM x_review", &[])
            .await
            .unwrap()
            .get("c");

        let intruder = crate::u2::review_init_for_view(
            Extension(pool.clone()),
            Some(Extension(sess("u-r48-nonadmin"))),
        )
        .await;
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非 admin 触发全表评审补齐必须 403"
        );
        let after: i64 = pool
            .get()
            .await
            .unwrap()
            .query_one("SELECT COUNT(*) AS c FROM x_review", &[])
            .await
            .unwrap()
            .get("c");
        assert_eq!(before, after, "拒绝路径不得新建任何 x_review 行");

        let (admin, role_id) = seed_admin(&pool, &tag).await;
        let okres = crate::u2::review_init_for_view(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
        )
        .await
        .map(|_| ());
        assert!(okres.is_ok(), "admin 触发应放行，实际 {okres:?}");
        let landed: i64 = pool
            .get()
            .await
            .unwrap()
            .query_one(
                "SELECT COUNT(*) AS c FROM x_review WHERE work_id = $1 AND reviewer = $2",
                &[&work.to_string(), &assignee.to_string()],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(landed, 1, "admin 臂应确实为 (work, assignee) 建出评审行");

        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_review WHERE work_id = $1", &[&work.to_string()])
            .await
            .unwrap();
        unseed_admin(&pool, &admin, &role_id).await;
        c.execute("DELETE FROM x_task WHERE work_id = $1", &[&work.to_string()])
            .await
            .unwrap();
        c.execute("DELETE FROM x_work WHERE id = $1", &[&work.to_string()])
            .await
            .unwrap();
    }

    // 四轮 §三.9：宏 `dict_data_fns!` 生成的 16 把 applicationdict 深度路径写
    // （dict_set_1..8 注册在 POST+PUT、dict_del_1..8 注册在 DELETE，共 24 条写路由）
    // 模板里既不取 Session 也不校任何门——任一登录用户按 `/{id}/{p0}…/data` 就能改写或摘除
    // 任意应用的 `x_application_dict.data`。x_application_dict 按本 crate §applicationdict
    // 口径属系统级数据 → admin 档，与 dict_edit 复用同一把 require_admin_opt。
    // 这族洞此前量不到，是尺子缺陷 12/13（宏生成的 handler 没有字面 fn → 体内读成空串）造成的。
    #[tokio::test]
    async fn dict_depth_writes_reject_without_admin() {
        let pool = shared::testing::test_pool();
        let id = "r49-dict-gate";
        let v = || serde_json::json!({ "value": 1 });

        // 无会话 → 401（不触库，可离线证）
        let anon: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "dict_set_1",
                crate::u2::dict_set_1(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((id.to_string(), "p0".to_string())),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_set_2",
                crate::u2::dict_set_2(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((id.to_string(), "p0".to_string(), "p1".to_string())),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_set_3",
                crate::u2::dict_set_3(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                    )),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_set_4",
                crate::u2::dict_set_4(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                    )),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_set_5",
                crate::u2::dict_set_5(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                    )),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_set_6",
                crate::u2::dict_set_6(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                        "p5".to_string(),
                    )),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_set_7",
                crate::u2::dict_set_7(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                        "p5".to_string(),
                        "p6".to_string(),
                    )),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_set_8",
                crate::u2::dict_set_8(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                        "p5".to_string(),
                        "p6".to_string(),
                        "p7".to_string(),
                    )),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_1",
                crate::u2::dict_del_1(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((id.to_string(), "p0".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_2",
                crate::u2::dict_del_2(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((id.to_string(), "p0".to_string(), "p1".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_3",
                crate::u2::dict_del_3(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_4",
                crate::u2::dict_del_4(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_5",
                crate::u2::dict_del_5(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_6",
                crate::u2::dict_del_6(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                        "p5".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_7",
                crate::u2::dict_del_7(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                        "p5".to_string(),
                        "p6".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_8",
                crate::u2::dict_del_8(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                        "p5".to_string(),
                        "p6".to_string(),
                        "p7".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(anon.len(), 16, "宏生成的 16 把深度写必须逐条点名");
        let bad: Vec<&str> = anon
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(n, _)| *n)
            .collect();
        assert!(bad.is_empty(), "无会话必须 401，实际不是的：{bad:?}");

        // 有会话但非 admin → 403（DB 不可达时 is_admin 兜底为 false，同样离线可证）
        let nonadmin: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "dict_set_1",
                crate::u2::dict_set_1(
                    Extension(pool.clone()),
                    Some(Extension(sess("u-r49-nonadmin"))),
                    axum::extract::Path((id.to_string(), "p0".to_string())),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_1",
                crate::u2::dict_del_1(
                    Extension(pool.clone()),
                    Some(Extension(sess("u-r49-nonadmin"))),
                    axum::extract::Path((id.to_string(), "p0".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_set_8",
                crate::u2::dict_set_8(
                    Extension(pool.clone()),
                    Some(Extension(sess("u-r49-nonadmin"))),
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                        "p5".to_string(),
                        "p6".to_string(),
                        "p7".to_string(),
                    )),
                    Json(v()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_del_8",
                crate::u2::dict_del_8(
                    Extension(pool.clone()),
                    Some(Extension(sess("u-r49-nonadmin"))),
                    axum::extract::Path((
                        id.to_string(),
                        "p0".to_string(),
                        "p1".to_string(),
                        "p2".to_string(),
                        "p3".to_string(),
                        "p4".to_string(),
                        "p5".to_string(),
                        "p6".to_string(),
                        "p7".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(nonadmin.len(), 4, "深度首末两档（1 段 / 8 段）各证一把写与删");
        let bad2: Vec<&str> = nonadmin
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(n, _)| *n)
            .collect();
        assert!(bad2.is_empty(), "非 admin 必须 403，实际不是的：{bad2:?}");
    }

    // DB 臂：证「拒的是门禁、不是缺库」——非 admin 403 且零落行，admin 放行且确实落行/删段。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn dict_depth_write_lands_only_for_admin() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: dict_depth_write_lands_only_for_admin (no DATABASE_URL)");
            return;
        }
        async fn dict_row_count(pool: &Pool, id: &str) -> i64 {
            pool.get()
                .await
                .unwrap()
                .query_one(
                    "SELECT COUNT(*) AS c FROM x_application_dict WHERE id = $1",
                    &[&id.to_string()],
                )
                .await
                .unwrap()
                .get::<_, i64>("c")
        }

        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let id = format!("r49-{tag}");

        let intruder = crate::u2::dict_set_1(
            Extension(pool.clone()),
            Some(Extension(sess("u-r49-nonadmin"))),
            axum::extract::Path((id.clone(), "p0".to_string())),
            Json(serde_json::json!({ "value": 1 })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非 admin 写字典必须 403，实际 {intruder:?}"
        );
        assert_eq!(dict_row_count(&pool, &id).await, 0, "拒绝路径不得新建 x_application_dict 行");

        let (admin, role_id) = seed_admin(&pool, &tag).await;
        let okres = crate::u2::dict_set_1(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
            axum::extract::Path((id.clone(), "p0".to_string())),
            Json(serde_json::json!({ "value": 1 })),
        )
        .await
        .map(|_| ());
        assert!(okres.is_ok(), "admin 写字典应放行，实际 {okres:?}");
        assert_eq!(dict_row_count(&pool, &id).await, 1, "admin 臂应确实落出字典行");

        let delres = crate::u2::dict_del_1(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
            axum::extract::Path((id.clone(), "p0".to_string())),
        )
        .await
        .map(|_| ());
        assert!(delres.is_ok(), "admin 删字典段应放行，实际 {delres:?}");

        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_application_dict WHERE id = $1", &[&id])
            .await
            .unwrap();
        unseed_admin(&pool, &admin, &role_id).await;
    }

    // ══ 四轮 R4-10 第三批：`data/delete/{work}/{id}` 与 `data/path/delete/{work}/{id}` ══
    //
    // 这两把 POST 此前**既不取会话也不校归属**：未认证请求按 ID 就能
    // `UPDATE x_work SET deleted_at = NOW()`（软删流程实例），或按 work 批量把
    // `x_task.task_status` 改成 deleted / `x_review.deleted_at` 落下。
    // 门沿用本 crate 的 `u4_session`（无会话 401，先于任何 SQL）+ 发起人归属，
    // 并取 destructive 口径：**空发起人只允许 admin**（比 `u4_require_actor`
    // 的「空归属放行任意会话」更严，因为这里是删除而不是认领）。

    /// 离线臂：两把删除路由都必须无会话 401，且 401 发生在 `pool.get()` 之前。
    #[tokio::test]
    async fn data_delete_routes_reject_without_session() {
        let pool = shared::testing::test_pool();
        let work_id = "r410-w-no-session".to_string();
        let a = crate::data_work_id_delete(
            Extension(pool.clone()),
            None,
            axum::extract::Path(("w-ignored".to_string(), work_id.clone())),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(a, Err(AppError::Unauthorized)),
            "data/delete 无会话必须 401（实际 {a:?}）——它是按 ID 软删他人 work 的入口"
        );
        let b = crate::data_work_id_path_delete(
            Extension(pool.clone()),
            None,
            axum::extract::Path((work_id, "tasks".to_string())),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(b, Err(AppError::Unauthorized)),
            "data/path/delete 无会话必须 401（实际 {b:?}）"
        );
    }

    /// DB 臂：① 他人会话 403 且 `deleted_at` / `task_status` 一格都不许动；
    /// ② 发起人放行且软删确实落库；③ 空发起人行不得被「人人可删」洗白——
    /// 普通会话 403、admin 200。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn data_delete_routes_require_work_creator_and_land() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let work_id = format!("r410-w-{tag}");
        let task_id = format!("r410-t-{tag}");
        let creator = format!("u-r410-creator-{tag}");
        let intruder = format!("u-r410-intruder-{tag}");
        seed_work(&pool, &work_id, &creator).await;
        seed_task(&pool, &task_id, &work_id, "u-r410-assignee", "active").await;

        // ① 他人会话打两把删除 → 403，且不留任何写痕迹
        let r = crate::data_work_id_delete(
            Extension(pool.clone()),
            Some(Extension(sess(&intruder))),
            axum::extract::Path(("w-ignored".to_string(), work_id.clone())),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "非发起人按 ID 软删他人 work 必须 403（实际 {r:?}）"
        );
        let r = crate::data_work_id_path_delete(
            Extension(pool.clone()),
            Some(Extension(sess(&intruder))),
            axum::extract::Path((work_id.clone(), "tasks".to_string())),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "非发起人按 work 批量软删其下任务必须 403（实际 {r:?}）"
        );
        let client = pool.get().await.unwrap();
        let not_deleted: bool = client
            .query_one(
                "SELECT (deleted_at IS NULL) AS a FROM x_work WHERE id = $1",
                &[&work_id.clone()],
            )
            .await
            .unwrap()
            .get("a");
        assert!(not_deleted, "403 之后 work 的 deleted_at 必须仍为空");
        let status: String = client
            .query_one(
                "SELECT task_status FROM x_task WHERE id = $1",
                &[&task_id.clone()],
            )
            .await
            .unwrap()
            .get::<_, Option<String>>("task_status")
            .unwrap_or_default();
        assert_eq!(status, "active", "403 之后其下任务状态一格都不许改");

        // ② 发起人 → 放行且软删落库
        let ok = crate::data_work_id_delete(
            Extension(pool.clone()),
            Some(Extension(sess(&creator))),
            axum::extract::Path(("w-ignored".to_string(), work_id.clone())),
        )
        .await
        .map(|_| ());
        assert!(
            ok.is_ok(),
            "发起人软删自己的 work 应放行（实际 {ok:?}）"
        );
        let deleted: bool = client
            .query_one(
                "SELECT (deleted_at IS NOT NULL) AS d FROM x_work WHERE id = $1",
                &[&work_id.clone()],
            )
            .await
            .unwrap()
            .get("d");
        assert!(deleted, "放行臂必须真的把 deleted_at 写进去（未命中行也会回 ok，落库才是凭据）");

        // ③ 空发起人：普通会话 403、admin 200
        let orphan = format!("r410-w-{tag}-orphan");
        seed_work(&pool, &orphan, "").await;
        let r = crate::data_work_id_delete(
            Extension(pool.clone()),
            Some(Extension(sess(&intruder))),
            axum::extract::Path(("w-ignored".to_string(), orphan.clone())),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "空发起人 work 不得对普通会话开放删除（实际 {r:?}）"
        );
        let (admin, role_id) = seed_admin(&pool, &tag).await;
        let ok = crate::data_work_id_path_delete(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
            axum::extract::Path((orphan.clone(), "tasks".to_string())),
        )
        .await
        .map(|_| ());
        assert!(
            ok.is_ok(),
            "admin 打空发起人 work 的批量删除应放行（实际 {ok:?}）"
        );
        unseed_admin(&pool, &admin, &role_id).await;
        client
            .execute("DELETE FROM x_task WHERE id = $1", &[&task_id])
            .await
            .unwrap();
        for w in [work_id, orphan] {
            client
                .execute("DELETE FROM x_work WHERE id = $1", &[&w])
                .await
                .unwrap();
        }
    }
}
