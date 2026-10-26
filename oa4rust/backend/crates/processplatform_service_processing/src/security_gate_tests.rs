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
//! 第十六批追加：本 crate 的 **GET 面**（注册 27 条 / handler 22 把，前缀均为
//! `/api/processplatform/service/processing`）——`GET snap/work/{workId}/type/*`、
//! `GET snap/workcompleted/{id}/type/*`、`GET snap/{id}/restore`、
//! `GET workcompleted/rollback/{id}`、`GET work/v2/{id}/goback|rollback|reroute/{p2}`、
//! `GET task/{id}/urge|replace|press|expire|pass_expired`、`GET task/v2/{id}/pause|resume`，
//! 以及 `u2` 面的同名快照 / 催办族——原为 K3 档「已认证、无会话参数、无门禁」：
//! 任一登录用户按 ID 即可暂停 / 恢复 / 转办 / 催办 / 过期他人待办，给他人单据落
//! abandoned / suspend 快照，甚至回滚他人的完成记录。GET 之所以是盲区，
//! `csrf_middleware` 与 `requires_admin` 都只看方法名。
//! 现在三档归属：work 键沿用 `u4_require_work_owner`（严格校 `x_work.creator`）；
//! task 键取 `x_task.person` 与 `x_work.creator` **两列 OR**——催办、过期、暂停的合法
//! 操作人本来就是流程双方（发起人催别人手里的办，负责人转办 / 暂停自己的办）；
//! workcompleted / snap 两族自身无可用属主（migration 033 给 `x_snap` 加了
//! creator / person，本 crate 写入路径从不填，全是 NULL），hop 到 `x_work.creator`。
//! admin 兜底与 `shared::middleware::require_owner` 同口径，空归属行只允许 admin。
//!
//! DB 依赖臂需真实 PostgreSQL：
//! `cargo test --lib -p processplatform_service_processing -- --ignored`
#[cfg(test)]
#[allow(clippy::module_inception)]
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
            let arg = if sql.contains("x_task WHERE id") {
                task_id
            } else {
                work_id
            };
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
                crate::task_claim(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("t1".into()),
                )
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
        assert!(
            matches!(dict, Err(AppError::Forbidden)),
            "非 admin 改字典必须 403"
        );
        assert!(
            matches!(job, Err(AppError::Forbidden)),
            "非 admin 结束他人 job 必须 403"
        );
        assert!(
            matches!(snap, Err(AppError::Forbidden)),
            "非 admin 删快照必须 403"
        );
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
        shared::testing::upsert_fixture_person(&client, &person, &person, &person).await;
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
            .execute(
                "DELETE FROM auth_person_role WHERE person_id = $1",
                &[&person.to_string()],
            )
            .await
            .unwrap();
        client
            .execute(
                "DELETE FROM auth_person WHERE id = $1",
                &[&person.to_string()],
            )
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
            .query_one(
                "SELECT COUNT(*) AS c FROM x_work WHERE id = $1",
                &[&work_id],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(
            alive, 1,
            "403 之后 work 行必须还在（级联物理删一行都不能动）"
        );
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
            .query_one(
                "SELECT COUNT(*) AS c FROM x_task WHERE work = $1",
                &[&work_id],
            )
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
                crate::u2::read_delete(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("r1".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "read_processing",
                crate::u2::read_processing(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("r1".to_string()),
                )
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
                    axum::extract::Path(("w1".to_string(), "s1".to_string(), "t1".to_string())),
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

        c.execute(
            "DELETE FROM x_record WHERE work_id = $1",
            &[&work.to_string()],
        )
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
        seed_task(
            &pool,
            &format!("r48-{tag}-task"),
            &work,
            &assignee,
            "active",
        )
        .await;

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
        let okres =
            crate::u2::review_init_for_view(Extension(pool.clone()), Some(Extension(sess(&admin))))
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
        c.execute(
            "DELETE FROM x_review WHERE work_id = $1",
            &[&work.to_string()],
        )
        .await
        .unwrap();
        unseed_admin(&pool, &admin, &role_id).await;
        c.execute(
            "DELETE FROM x_task WHERE work_id = $1",
            &[&work.to_string()],
        )
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
        assert_eq!(
            nonadmin.len(),
            4,
            "深度首末两档（1 段 / 8 段）各证一把写与删"
        );
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
        assert_eq!(
            dict_row_count(&pool, &id).await,
            0,
            "拒绝路径不得新建 x_application_dict 行"
        );

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
        assert_eq!(
            dict_row_count(&pool, &id).await,
            1,
            "admin 臂应确实落出字典行"
        );

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
        assert!(ok.is_ok(), "发起人软删自己的 work 应放行（实际 {ok:?}）");
        let deleted: bool = client
            .query_one(
                "SELECT (deleted_at IS NOT NULL) AS d FROM x_work WHERE id = $1",
                &[&work_id.clone()],
            )
            .await
            .unwrap()
            .get("d");
        assert!(
            deleted,
            "放行臂必须真的把 deleted_at 写进去（未命中行也会回 ok，落库才是凭据）"
        );

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

    // ── 第十六批：GET 面 27 条注册 / 22 把 handler 补归属门 ──────────────────
    //
    // 本轮之前这 22 把既不取 Session 也不校归属：任一登录用户（GET 面还落在
    // csrf / require_admin 两把中间件的方法盲区内）按 ID 就能催办、过期、转办、
    // 暂停他人的待办，把自己的单据回滚·改道，或物理删除他人的完成记录。
    // 收紧成三档：work 键走 `u4_require_work_owner`（发起人，空发起人→admin）；
    // task 键走 `u4_require_actors(Task)`（负责人或发起人，双方都空→admin）；
    // workcompleted / snap 键按 hop 到 x_work.creator 后同 task 键口径。

    async fn task_status(pool: &Pool, id: &str) -> String {
        let id = id.to_string();
        let client = pool.get().await.unwrap();
        client
            .query_one("SELECT task_status FROM x_task WHERE id = $1", &[&id])
            .await
            .unwrap()
            .get::<_, Option<String>>("task_status")
            .unwrap_or_default()
    }

    async fn work_status(pool: &Pool, id: &str) -> String {
        let id = id.to_string();
        let client = pool.get().await.unwrap();
        client
            .query_one("SELECT work_status FROM x_work WHERE id = $1", &[&id])
            .await
            .unwrap()
            .get::<_, Option<String>>("work_status")
            .unwrap_or_default()
    }

    /// 带一个参数的 COUNT 读数（SQL 只收字面量，id 走绑定）。
    async fn count_by(pool: &Pool, sql: &'static str, id: &str) -> i64 {
        let id = id.to_string();
        let client = pool.get().await.unwrap();
        client
            .query_one(sql, &[&id])
            .await
            .unwrap()
            .get::<_, i64>("c")
    }

    /// 离线臂：22 把 GET 面写逐把点名——无会话一律 401。
    /// 池在此不可达，所以「回 401」本身就证明门在 `pool.get()` 之前。
    #[tokio::test]
    async fn ppsvc_get_writes_reject_without_session() {
        let pool = shared::testing::test_pool();
        let p1 = |id: &str| axum::extract::Path(id.to_string());
        let p2 = |a: &str, b: &str| axum::extract::Path((a.to_string(), b.to_string()));
        let p3 = |a: &str, b: &str, c: &str| {
            axum::extract::Path((a.to_string(), b.to_string(), c.to_string()))
        };
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "work_v2_id_goback_p2",
                crate::work_v2_id_goback_p2(Extension(pool.clone()), None, p2("w", "w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "work_v2_id_rollback_p2",
                crate::work_v2_id_rollback_p2(Extension(pool.clone()), None, p2("w", "w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "work_v2_id_reroute_p2",
                crate::work_v2_id_reroute_p2(Extension(pool.clone()), None, p2("w", "w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_id_urge",
                crate::task_id_urge(Extension(pool.clone()), None, p1("t"))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_id_replace",
                crate::task_id_replace(Extension(pool.clone()), None, p1("t"))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_id_press",
                crate::task_id_press(Extension(pool.clone()), None, p1("t"))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_id_expire",
                crate::task_id_expire(Extension(pool.clone()), None, p1("t"))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_id_pass_expired",
                crate::task_id_pass_expired(Extension(pool.clone()), None, p1("t"))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_v2_id_pause",
                crate::task_v2_id_pause(Extension(pool.clone()), None, p1("t"))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_v2_id_resume",
                crate::task_v2_id_resume(Extension(pool.clone()), None, p1("t"))
                    .await
                    .map(|_| ()),
            ),
            (
                "snap_work_workId_type_abandoned",
                crate::snap_work_workId_type_abandoned(
                    Extension(pool.clone()),
                    None,
                    p3("a", "w", "b"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "snap_work_workId_type_suspend",
                crate::snap_work_workId_type_suspend(
                    Extension(pool.clone()),
                    None,
                    p3("a", "w", "b"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "snap_workcompleted_workCompletedId_type_abandonedworkcompleted",
                crate::snap_workcompleted_workCompletedId_type_abandonedworkcompleted(
                    Extension(pool.clone()),
                    None,
                    p2("wc", "t"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "snap_id_restore",
                crate::snap_id_restore(Extension(pool.clone()), None, p1("s"))
                    .await
                    .map(|_| ()),
            ),
            (
                "workcompleted_flag_rollback",
                crate::workcompleted_flag_rollback(Extension(pool.clone()), None, p1("wc"))
                    .await
                    .map(|_| ()),
            ),
            (
                "u2::snap_type_abandoned",
                crate::u2::snap_type_abandoned(Extension(pool.clone()), None, p1("w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "u2::snap_type_snap",
                crate::u2::snap_type_snap(Extension(pool.clone()), None, p1("w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "u2::snap_type_suspend",
                crate::u2::snap_type_suspend(Extension(pool.clone()), None, p1("w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "u2::snap_wc_type_abandoned",
                crate::u2::snap_wc_type_abandoned(Extension(pool.clone()), None, p1("wc"))
                    .await
                    .map(|_| ()),
            ),
            (
                "u2::snap_wc_type_snap",
                crate::u2::snap_wc_type_snap(Extension(pool.clone()), None, p1("wc"))
                    .await
                    .map(|_| ()),
            ),
            (
                "u2::taskcompleted_press",
                crate::u2::taskcompleted_press(Extension(pool.clone()), None, p2("rc", "w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "u2::wc_merge_flag",
                crate::u2::wc_merge_flag(Extension(pool.clone()), None, p1("wc"))
                    .await
                    .map(|_| ()),
            ),
        ];
        let bad: Vec<&str> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(n, _)| *n)
            .collect();
        assert!(bad.is_empty(), "以下 GET 面无会话未回 401：{bad:?}");
        assert_eq!(cases.len(), 22, "本批 22 把必须逐把点名，少一把即漏门");
    }

    /// DB 臂（task 键）：陌生会话 403 且状态机一步不动；负责人·发起人各自放行并落笔；
    /// 双方都空的孤儿任务只对 admin 开放。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn ppsvc_get_task_writes_require_flow_actor_and_land() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let salt = uuid::Uuid::new_v4();
        let work_id = format!("r43-w-{salt}");
        let task_id = format!("r43-t-{salt}");
        let orphan = format!("r43-t-{salt}-orphan");
        let creator = "u-r43-creator";
        let assignee = "u-r43-assignee";
        let intruder = "u-r43-intruder";
        seed_work(&pool, &work_id, creator).await;
        seed_task(&pool, &task_id, &work_id, assignee, "active").await;
        let p1 = |id: &str| axum::extract::Path(id.to_string());

        let r = crate::task_id_press(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&task_id),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "流程之外的会话按 ID 办他人待办必须 403（实际 {r:?}）"
        );
        assert_eq!(
            task_status(&pool, &task_id).await,
            "active",
            "403 之后 x_task.task_status 不得被推动"
        );

        let r = crate::task_id_press(
            Extension(pool.clone()),
            Some(Extension(sess(assignee))),
            p1(&task_id),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "负责人处理自己的待办应放行（实际 {r:?}）");
        assert_eq!(task_status(&pool, &task_id).await, "pressed");

        // 这一格是 task 键取「person + work.creator」两列的理由：催办来自发起人，
        // 只校 x_task.person 会把正常流程判成越权。
        let r = crate::task_id_urge(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p1(&task_id),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "发起人催办自己单据上的待办应放行（实际 {r:?}）");
        assert_eq!(task_status(&pool, &task_id).await, "urged");
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_record WHERE task_id = $1 AND record_type = 'urge'",
                &task_id
            )
            .await,
            1,
            "放行后催办留痕必须落库"
        );

        seed_task(&pool, &orphan, "r43-no-such-work", "", "active").await;
        let r = crate::task_id_expire(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&orphan),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "负责人与发起人都为空的 task 不得对普通会话开放（实际 {r:?}）"
        );
        assert_eq!(task_status(&pool, &orphan).await, "active");
        let (admin, role_id) = seed_admin(&pool, "r43").await;
        let r = crate::task_id_expire(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
            p1(&orphan),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "admin 打无主 task 的过期写应放行（实际 {r:?}）");
        assert_eq!(
            task_status(&pool, &orphan).await,
            "expired",
            "admin 放行后落笔必须可见"
        );
        unseed_admin(&pool, &admin, &role_id).await;

        let client = pool.get().await.unwrap();
        for t in [task_id.as_str(), orphan.as_str()] {
            client
                .execute("DELETE FROM x_record WHERE task_id = $1", &[&t.to_string()])
                .await
                .unwrap();
            client
                .execute("DELETE FROM x_task WHERE id = $1", &[&t.to_string()])
                .await
                .unwrap();
        }
        client
            .execute("DELETE FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap();
    }

    /// DB 臂（work 键）：`u4_require_work_owner` 严格档 + 「归属门先于 IDOR」+
    /// 空发起人只归 admin。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn ppsvc_get_work_writes_require_creator_and_land() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let salt = uuid::Uuid::new_v4();
        let work_id = format!("r44-w-{salt}");
        let work2 = format!("r44-w2-{salt}");
        let orphan = format!("r44-wo-{salt}");
        let rc = format!("r44-rc-{salt}");
        let creator = "u-r44-creator";
        let intruder = "u-r44-intruder";
        seed_work(&pool, &work_id, creator).await;
        seed_work(&pool, &work2, creator).await;
        seed_work(&pool, &orphan, "").await;
        {
            let client = pool.get().await.unwrap();
            client
                .execute(
                    "INSERT INTO x_readcompleted (id, work_id, person) VALUES ($1, $2, $3)",
                    &[&rc, &work2, &"u-r44-reader".to_string()],
                )
                .await
                .unwrap();
        }
        let p1 = |id: &str| axum::extract::Path(id.to_string());
        let p2 = |a: &str, b: &str| axum::extract::Path((a.to_string(), b.to_string()));
        let p3 = |a: &str, b: &str, c: &str| {
            axum::extract::Path((a.to_string(), b.to_string(), c.to_string()))
        };

        let r = crate::u2::snap_type_suspend(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&work_id),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "陌生会话挂起他人单据必须 403（实际 {r:?}）"
        );
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_snap WHERE work_id = $1",
                &work_id
            )
            .await,
            0,
            "403 之后不得留下任何快照"
        );
        assert_eq!(work_status(&pool, &work_id).await, "processing");

        let r = crate::u2::snap_type_suspend(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p1(&work_id),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "发起人挂起自己的单据应放行（实际 {r:?}）");
        assert_eq!(work_status(&pool, &work_id).await, "suspended");
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_snap WHERE work_id = $1 AND snap_type = 'suspend'",
                &work_id
            )
            .await,
            1,
            "放行后挂起快照必须落库"
        );

        let r = crate::snap_work_workId_type_abandoned(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p3("abandoned", &work_id, "abandoned"),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "发起人放弃自己的单据应放行（实际 {r:?}）");
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_snap WHERE work_id = $1 AND snap_type = 'abandoned'",
                &work_id
            )
            .await,
            1,
            "lib 面的 abandoned 快照注册同样要落笔"
        );

        let r = crate::work_v2_id_reroute_p2(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p2(&work_id, &work_id),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "陌生会话改道他人单据必须 403（实际 {r:?}）"
        );
        assert_eq!(work_status(&pool, &work_id).await, "suspended");
        let r = crate::work_v2_id_reroute_p2(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p2(&work_id, &work_id),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "发起人改道自己的单据应放行（实际 {r:?}）");
        assert_eq!(work_status(&pool, &work_id).await, "rerouted");

        // 归属门先于 IDOR：发起人打「别人的记录 + 自己的 work」过得了归属门，
        // 再由 work 等值校验回业务错误——两道门各自可证，不会把 403 当 IDOR 读数。
        let r = crate::u2::taskcompleted_press(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p2(&rc, &work_id),
        )
        .await;
        assert!(
            r.is_ok(),
            "发起人过归属门后由 IDOR 返回业务错误信封（HTTP 200）"
        );
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_record WHERE work_id = $1 AND record_type = 'press'",
                &work_id
            )
            .await,
            0,
            "IDOR 拒绝后不得留痕"
        );
        let r = crate::u2::taskcompleted_press(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p2(&rc, &work2),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "陌生会话催办他人已完成项必须 403（实际 {r:?}）"
        );
        let r = crate::u2::taskcompleted_press(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p2(&rc, &work2),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "发起人催办自己单据应放行（实际 {r:?}）");
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_record WHERE work_id = $1 AND record_type = 'press'",
                &work2
            )
            .await,
            1,
            "放行后催办留痕必须落库"
        );

        // 空发起人：普通会话 403、admin 放行并落笔。
        let r = crate::u2::snap_type_abandoned(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&orphan),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "空发起人单据不得对普通会话开放（实际 {r:?}）"
        );
        let (admin, role_id) = seed_admin(&pool, "r44").await;
        let r = crate::u2::snap_type_abandoned(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
            p1(&orphan),
        )
        .await
        .map(|_| ());
        assert!(
            r.is_ok(),
            "admin 打空发起人单据的快照写应放行（实际 {r:?}）"
        );
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_snap WHERE work_id = $1 AND snap_type = 'abandoned'",
                &orphan
            )
            .await,
            1,
            "admin 放行后落笔必须可见"
        );
        unseed_admin(&pool, &admin, &role_id).await;

        let client = pool.get().await.unwrap();
        client
            .execute("DELETE FROM x_readcompleted WHERE id = $1", &[&rc])
            .await
            .unwrap();
        for w in [work_id.as_str(), work2.as_str(), orphan.as_str()] {
            client
                .execute("DELETE FROM x_record WHERE work_id = $1", &[&w.to_string()])
                .await
                .unwrap();
            client
                .execute("DELETE FROM x_snap WHERE work_id = $1", &[&w.to_string()])
                .await
                .unwrap();
            client
                .execute("DELETE FROM x_work WHERE id = $1", &[&w.to_string()])
                .await
                .unwrap();
        }
    }

    /// 第四十三批（R7 含写收口）：`work/v2/{id}/goback|rollback|reroute|add_split`（POST/PUT，
    /// 原薄封装别名无门，按 ID 回退/回滚/改派/拆分他人 work）与 `/data/job|work/...` 数据面写
    /// （PUT/POST，按 job 或 work 改/建/删数据）。前者按 `x_work.creator` 归属，后者 job 收 admin、
    /// work 收 `x_work.creator` 归属。离线臂：无会话一律 401（门在 `pool.get()` 之前）。
    #[tokio::test]
    async fn ppsvc_r7_data_and_work_v2_writes_reject_without_session() {
        let pool = shared::testing::test_pool();
        let p1 = |id: &str| axum::extract::Path(id.to_string());
        let p2 = |a: &str, b: &str| axum::extract::Path((a.to_string(), b.to_string()));
        let empty = || Json(serde_json::json!({}));
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "work_v2_id_goback",
                crate::work_v2_id_goback(Extension(pool.clone()), None, p1("w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "work_v2_id_rollback",
                crate::work_v2_id_rollback(Extension(pool.clone()), None, p1("w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "work_v2_id_add_split",
                crate::work_v2_id_add_split(Extension(pool.clone()), None, p1("w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "work_v2_id_reroute",
                crate::work_v2_id_reroute(Extension(pool.clone()), None, p1("w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "data_job_put",
                crate::u2::data_job_put(Extension(pool.clone()), None, p1("j"), empty())
                    .await
                    .map(|_| ()),
            ),
            (
                "data_job_put_path",
                crate::u2::data_job_put_path(Extension(pool.clone()), None, p2("j", "p"), empty())
                    .await
                    .map(|_| ()),
            ),
            (
                "data_work_create",
                crate::u2::data_work_create(Extension(pool.clone()), None, p1("w"), empty())
                    .await
                    .map(|_| ()),
            ),
            (
                "data_work_update",
                crate::u2::data_work_update(Extension(pool.clone()), None, p1("w"), empty())
                    .await
                    .map(|_| ()),
            ),
            (
                "data_work_create_path",
                crate::u2::data_work_create_path(
                    Extension(pool.clone()),
                    None,
                    p2("w", "p"),
                    empty(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "data_work_update_path",
                crate::u2::data_work_update_path(
                    Extension(pool.clone()),
                    None,
                    p2("w", "p"),
                    empty(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "data_work_delete",
                crate::u2::data_work_delete(Extension(pool.clone()), None, p1("w"))
                    .await
                    .map(|_| ()),
            ),
            (
                "data_work_delete_path",
                crate::u2::data_work_delete_path(Extension(pool.clone()), None, p2("w", "p"))
                    .await
                    .map(|_| ()),
            ),
        ];
        let bad: Vec<&str> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(n, _)| *n)
            .collect();
        assert!(bad.is_empty(), "以下 R7 写面无会话未回 401：{bad:?}");
        assert_eq!(cases.len(), 12, "本批 12 把必须逐把点名，少一把即漏门");
    }

    /// DB 臂（第四十三批）：① work 族陌生会话 403 且不留痕、发起人放行且落库；
    /// ② job 族收 admin 门——非 admin 403、admin 放行；③ work 空发起人普通会话 403、admin 放行。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn ppsvc_r7_data_and_work_v2_writes_require_owner_and_admin() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let salt = uuid::Uuid::new_v4();
        let work_id = format!("r43-w-{salt}");
        let orphan = format!("r43-wo-{salt}");
        let creator = "u-r43-creator";
        let intruder = "u-r43-intruder";
        seed_work(&pool, &work_id, creator).await;
        seed_work(&pool, &orphan, "").await;
        let client = pool.get().await.unwrap();
        client
            .execute(
                "DELETE FROM x_data WHERE scope='work' AND bundle=$1",
                &[&work_id],
            )
            .await
            .unwrap();
        let p1 = |id: &str| axum::extract::Path(id.to_string());
        let body = || Json(serde_json::json!({ "k": "v" }));

        // work_v2_id_goback：陌生会话 403，发起人放行。
        let r = crate::work_v2_id_goback(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&work_id),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "陌生会话回退他人 work 必须 403（实际 {r:?}）"
        );
        let r = crate::work_v2_id_goback(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p1(&work_id),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "发起人回退自己的 work 应放行（实际 {r:?}）");

        // data_work_create：陌生会话 403，发起人放行且落库。
        let r = crate::u2::data_work_create(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&work_id),
            body(),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "非属主写他人 work 数据必须 403（实际 {r:?}）"
        );
        let r = crate::u2::data_work_create(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p1(&work_id),
            body(),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "属主写自己 work 数据应放行（实际 {r:?}）");
        let n: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_data WHERE scope='work' AND bundle=$1",
                &[&work_id],
            )
            .await
            .unwrap()
            .get("c");
        assert!(n >= 1, "放行后 work 数据必须落库");

        // data_job_put：admin 门——非 admin 403，admin 放行。
        let r = crate::u2::data_job_put(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p1("r43-job"),
            body(),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "非 admin 写 job 数据必须 403（实际 {r:?}）"
        );
        let (admin, role_id) = seed_admin(&pool, "r43").await;
        let r = crate::u2::data_job_put(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
            p1("r43-job"),
            body(),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "admin 写 job 数据应放行（实际 {r:?}）");
        unseed_admin(&pool, &admin, &role_id).await;

        // 空发起人 work：普通会话 403，admin 放行。
        let r = crate::u2::data_work_create(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&orphan),
            body(),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "空发起人 work 不得对普通会话开放（实际 {r:?}）"
        );
        let (admin, role_id) = seed_admin(&pool, "r43o").await;
        let r = crate::u2::data_work_create(
            Extension(pool.clone()),
            Some(Extension(sess(&admin))),
            p1(&orphan),
            body(),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "admin 写空发起人 work 应放行（实际 {r:?}）");
        unseed_admin(&pool, &admin, &role_id).await;

        client
            .execute(
                "DELETE FROM x_data WHERE scope='work' AND bundle=$1",
                &[&work_id],
            )
            .await
            .unwrap();
        client
            .execute("DELETE FROM x_data WHERE scope='job'", &[])
            .await
            .unwrap();
        for w in [work_id.clone(), orphan.clone()] {
            client
                .execute("DELETE FROM x_work WHERE id=$1", &[&w])
                .await
                .unwrap();
        }
    }

    /// DB 臂（workcompleted / snap 键）：完成项与快照行自身没有可用属主，
    /// 必须 hop 到 x_work.creator；放行侧要能看见物理删与状态写真的落库。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn ppsvc_get_completed_snap_writes_hop_to_work_creator() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let salt = uuid::Uuid::new_v4();
        let work_id = format!("r45-w-{salt}");
        let wc_id = format!("r45-wc-{salt}");
        let wc_dup = format!("r45-wc2-{salt}");
        let snap_id = format!("r45-s-{salt}");
        let creator = "u-r45-creator";
        let intruder = "u-r45-intruder";
        seed_work(&pool, &work_id, creator).await;
        let client = pool.get().await.unwrap();
        for id in [wc_id.as_str(), wc_dup.as_str()] {
            client
                .execute(
                    "INSERT INTO x_workcompleted (id, work_id) VALUES ($1, $2)",
                    &[&id.to_string(), &work_id],
                )
                .await
                .unwrap();
        }
        client
            .execute(
                "INSERT INTO x_snap (id, work_id, snap_type, snap_data) VALUES ($1, $2, 'snap', 'null'::jsonb)",
                &[&snap_id, &work_id],
            )
            .await
            .unwrap();
        let p1 = |id: &str| axum::extract::Path(id.to_string());
        let p2 = |a: &str, b: &str| axum::extract::Path((a.to_string(), b.to_string()));

        // ① snap_wc_type_abandoned：hop 到发起人才能放行；陌生会话零快照。
        let r = crate::u2::snap_wc_type_abandoned(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&wc_id),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "陌生会话给他人完成项打放弃快照必须 403（实际 {r:?}）"
        );
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_snap WHERE work_id = $1 AND snap_type = 'abandonedWorkCompleted'",
                &work_id
            )
            .await,
            0
        );
        let r = crate::u2::snap_wc_type_abandoned(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p1(&wc_id),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "发起人给完成项打快照应放行（实际 {r:?}）");
        // 完成项快照按 work_id 落键（读回侧一律按 work_id 查；写成 wc_id 会存进去读不到）。
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_snap WHERE work_id = $1 AND snap_type = 'abandonedWorkCompleted'",
                &work_id
            )
            .await,
            1,
            "完成项快照必须按真实 work_id 落键"
        );

        // ② lib 面同一把键（workcompleted 路径注册）：陌生会话 403。
        let r = crate::snap_workcompleted_workCompletedId_type_abandonedworkcompleted(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p2(&wc_id, "abandonedworkcompleted"),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "陌生会话按 workcompleted ID 写放弃快照必须 403（实际 {r:?}）"
        );

        // ③ snap_id_restore：hop 经 x_snap.work_id。
        let r = crate::snap_id_restore(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&snap_id),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "陌生会话用他人快照回滚单据必须 403（实际 {r:?}）"
        );
        assert_eq!(work_status(&pool, &work_id).await, "processing");
        let r = crate::snap_id_restore(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p1(&snap_id),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "发起人用自家快照回滚应放行（实际 {r:?}）");
        assert_eq!(
            work_status(&pool, &work_id).await,
            "restored",
            "放行后 work_status 必须被写"
        );

        // ④ wc_merge_flag：destroyive 物理删，陌生会话一行都不许掉。
        let r = crate::u2::wc_merge_flag(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&wc_id),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "陌生会话合并他人完成记录必须 403（实际 {r:?}）"
        );
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_workcompleted WHERE work_id = $1",
                &work_id
            )
            .await,
            2,
            "403 之后重复项必须原样保留"
        );
        let r = crate::u2::wc_merge_flag(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p1(&wc_id),
        )
        .await
        .map(|_| ());
        assert!(
            r.is_ok(),
            "发起人合并自己的重复完成记录应放行（实际 {r:?}）"
        );
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_workcompleted WHERE work_id = $1",
                &work_id
            )
            .await,
            1,
            "放行后仅保留 flag 指定项"
        );

        // ⑤ workcompleted_flag_rollback：删完成记录 + work 拉回 pending。
        let wc3 = format!("r45-wc3-{salt}");
        client
            .execute(
                "INSERT INTO x_workcompleted (id, work_id) VALUES ($1, $2)",
                &[&wc3, &work_id],
            )
            .await
            .unwrap();
        let r = crate::workcompleted_flag_rollback(
            Extension(pool.clone()),
            Some(Extension(sess(intruder))),
            p1(&wc3),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "陌生会话回滚他人完成项必须 403（实际 {r:?}）"
        );
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_workcompleted WHERE id = $1",
                &wc3
            )
            .await,
            1,
            "403 之后完成记录不得被物理删"
        );
        let r = crate::workcompleted_flag_rollback(
            Extension(pool.clone()),
            Some(Extension(sess(creator))),
            p1(&wc3),
        )
        .await
        .map(|_| ());
        assert!(r.is_ok(), "发起人回滚自家完成项应放行（实际 {r:?}）");
        assert_eq!(
            count_by(
                &pool,
                "SELECT COUNT(*) AS c FROM x_workcompleted WHERE id = $1",
                &wc3
            )
            .await,
            0,
            "放行后完成记录必须被删"
        );
        assert_eq!(work_status(&pool, &work_id).await, "pending");

        client
            .execute("DELETE FROM x_record WHERE work_id = $1", &[&work_id])
            .await
            .unwrap();
        client
            .execute("DELETE FROM x_snap WHERE work_id = $1", &[&work_id])
            .await
            .unwrap();
        client
            .execute(
                "DELETE FROM x_workcompleted WHERE work_id = $1",
                &[&work_id],
            )
            .await
            .unwrap();
        client
            .execute("DELETE FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 第五十三批（2026-10 四轮）：processplatform_service_processing 剩余写面 26 把的门。
//
// 这批是前几批未覆盖的「引擎运行面续集」：建单、推进/重置待办、追加任务、快照上传、
// 事件留痕、附件复制、完成记录合并/回滚、定时器起停、u2 契约面的完成记录数据写、
// 阅读回执造数、记录造数、下一身份设置、人工流转后置、按流程批量补完成记录、
// 批量平移完成时间。全部为真 DB 写（INSERT/UPDATE/DELETE），原不取会话、无门禁。
//
// 分档：
// - work 归属（`u4_require_work_owner` / `u4_require_actor`）：按 `x_work.creator`；
// - task/workcompleted 归属（`u4_require_actors`）：两列 OR（task.person/work.creator、
//   wc.creator/work.creator），空归属行只允许 admin；
// - admin（`require_admin_opt`）：跨多 work 的批量运维作业（按流程补完成记录、批量平移时间）；
// - 建单（`create_process`）：新资源无既有属主，只要求有效会话。
// 所有门都插在 `pool.get()` 之前（`u4_session`/`require_admin_opt` 先于任何读库），
// 故「无会话 401」可离线证；`record_work_terminate`/`work_manual_after` 先 `u4_session`
// 再保留原 not-found 语义、最后校归属，避免把「库连不上」误报成 401。
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod b53_gate_tests {
    use axum::extract::{Extension, Path};
    use axum::Json;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn non_admin() -> shared::session::Session {
        shared::testing::test_session("b53-nonadmin")
    }

    fn timer() -> Extension<crate::timer::TimerRegistry> {
        Extension(crate::timer::TimerRegistry::new())
    }

    /// 无会话臂：连不上库的 pool + None 会话，26 把必须全 401——
    /// 出现 Internal 即门插在 pool.get() 之后。
    #[tokio::test]
    async fn b53_r7_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            // ── lib.rs ────────────────────────────────────────────────
            (
                "create_process",
                crate::create_process(
                    Extension(pool.clone()),
                    None,
                    Json(crate::CreateProcessRequest {
                        name: None,
                        description: None,
                        category: None,
                    }),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_v2_id_add_split_p2",
                crate::work_v2_id_add_split_p2(
                    Extension(pool.clone()),
                    None,
                    Path(("w".to_string(), "w1".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_manual_after_processing",
                crate::work_manual_after_processing(
                    Extension(pool.clone()),
                    None,
                    Path("w1".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "task_id_processing",
                crate::task_id_processing(Extension(pool.clone()), None, Path("t1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_v2_id_reset",
                crate::task_v2_id_reset(Extension(pool.clone()), None, Path("t1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "task_v3_id_add",
                crate::task_v3_id_add(Extension(pool.clone()), None, Path("w1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "taskcompleted_id_press_work_work",
                crate::taskcompleted_id_press_work_work(
                    Extension(pool.clone()),
                    None,
                    Path(("wc1".to_string(), "w1".to_string(), "x".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "snap_upload",
                crate::snap_upload(Extension(pool.clone()), None, Json(json!({})))
                    .await
                    .map(|_| ()),
            ),
            (
                "event_add_update_table",
                crate::event_add_update_table(Extension(pool.clone()), None, Json(json!({})))
                    .await
                    .map(|_| ()),
            ),
            (
                "attachment_copy_work_workId",
                crate::attachment_copy_work_workId(
                    Extension(pool.clone()),
                    None,
                    Path(("a1".to_string(), "w1".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "attachment_copy_workcompleted_workCompletedId",
                crate::attachment_copy_workcompleted_workCompletedId(
                    Extension(pool.clone()),
                    None,
                    Path("wc1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "workcompleted_flag_merge",
                crate::workcompleted_flag_merge(
                    Extension(pool.clone()),
                    None,
                    Path("wc1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "start_timer",
                crate::start_timer(Extension(pool.clone()), timer(), None, Json(json!({})))
                    .await
                    .map(|_| ()),
            ),
            (
                "cancel_timer",
                crate::cancel_timer(
                    Extension(pool.clone()),
                    timer(),
                    None,
                    Path("j1".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            // ── u2.rs ─────────────────────────────────────────────────
            (
                "data_wc_update",
                crate::u2::data_wc_update(
                    Extension(pool.clone()),
                    None,
                    Path("wc1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "data_wc_update_path",
                crate::u2::data_wc_update_path(
                    Extension(pool.clone()),
                    None,
                    Path(("wc1".to_string(), "p".to_string())),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "read_create_with_work",
                crate::u2::read_create_with_work(
                    Extension(pool.clone()),
                    None,
                    Path("w1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "read_create_with_workcompleted",
                crate::u2::read_create_with_workcompleted(
                    Extension(pool.clone()),
                    None,
                    Path("wc1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "record_create_with_job",
                crate::u2::record_create_with_job(
                    Extension(pool.clone()),
                    None,
                    Path("j1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "record_work_processing",
                crate::u2::record_work_processing(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "work": "w1" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "record_work_terminate",
                crate::u2::record_work_terminate(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "work": "w1" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "taskcompleted_update_next_identity",
                crate::u2::taskcompleted_update_next_identity(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "id": "t1", "nextTaskIdentity": "x" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_manual_after",
                crate::u2::work_manual_after(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "work": "w1" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "wc_process_flag",
                crate::u2::wc_process_flag(Extension(pool.clone()), None, Path("p1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "wc_shift_time",
                crate::u2::wc_shift_time(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "minutes": 5 })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "wc_rollback_flag",
                crate::u2::wc_rollback_flag(Extension(pool.clone()), None, Path("wc1".to_string()))
                    .await
                    .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 26, "本批 26 把必须逐把点名");
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "无会话未回 401（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }

    /// admin 臂：跨多 work 的批量运维作业，非管理员会话 403
    /// （`is_admin` 对不可用库 fail-closed，亦 403，故本臂离线稳健）。
    #[tokio::test]
    async fn b53_admin_writes_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "wc_process_flag",
                crate::u2::wc_process_flag(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Path("p1".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "wc_shift_time",
                crate::u2::wc_shift_time(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Json(json!({ "minutes": 5 })),
                )
                .await
                .map(|_| ()),
            ),
        ];
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "非管理员未回 403（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }

    /// 归属臂（需真实 PostgreSQL）：行不存在 → 归属列全空 → 非 admin 一律 403
    /// （严格口径，不把「缺失行」洗成放行）。`cargo test ... -- --ignored` 触发。
    #[tokio::test]
    #[ignore = "需真实 PostgreSQL（DATABASE_URL）"]
    async fn b53_owner_writes_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "task_id_processing",
                crate::task_id_processing(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Path("b53-missing-task".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "task_v2_id_reset",
                crate::task_v2_id_reset(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Path("b53-missing-task".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "snap_upload",
                crate::snap_upload(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Json(json!({ "workId": "b53-missing-work" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "read_create_with_work",
                crate::u2::read_create_with_work(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Path("b53-missing-work".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "taskcompleted_id_press_work_work",
                crate::taskcompleted_id_press_work_work(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Path((
                        "b53-missing-wc".to_string(),
                        "b53-missing-work".to_string(),
                        "x".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
            (
                "wc_rollback_flag",
                crate::u2::wc_rollback_flag(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Path("b53-missing-wc".to_string()),
                )
                .await
                .map(|_| ()),
            ),
        ];
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "非管理员未回 403（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }
}
