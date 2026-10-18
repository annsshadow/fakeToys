// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 四轮审计第八批（R4-2 续 / N-07）：CMS 装配控制台的**元数据写族**此前整族「登录即可」——
//! 既不取会话也不校归属，`creator` 恒写 `"system"`。这类表（`x_cms_dict` / `x_cms_form` /
//! `x_cms_xform` / `x_cms_view` / `x_cms_templateform`）是**结构定义**而不是业务数据：
//! 任一有效会话可改表单/视图定义，把后续所有读取与提交导向攻击者的字段（一次改动影响全站），
//! `update_control_config` 还能直接翻安全相关开关，`permission_management_refresh_all`
//! 是一条无门的全表权限缓存重刷。按「无属主的系统级数据 = admin」档收口，与 §三.4 的
//! `dict_edit` / `job_delete` 同族。`is_admin` 无库时 fail-closed，故拒绝侧离线即可证。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::{Extension, Path};
    use axum::Json;
    use serde_json::Value;
    use shared::error::AppError;

    /// 17 把元数据/配置写：非 admin（含无库 fail-closed）全部必须 403，逐把点名。
    #[tokio::test]
    async fn cms_metadata_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin-cms");
        let body = || Json(serde_json::json!({ "name": "n", "content": "{}" }));
        let p = || Path("x1".to_string());
        let ext = || Extension(sess.clone());

        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "update_control_config",
                crate::update_control_config(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "permission_management_refresh_all",
                crate::permission_management_refresh_all(Extension(pool.clone()), ext())
                    .await
                    .map(|_| ()),
            ),
            (
                "dict_create",
                crate::dict_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "dict_save",
                crate::dict_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "dict_delete",
                crate::dict_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "form_create",
                crate::form_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "form_save",
                crate::form_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "form_delete",
                crate::form_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "xform_create",
                crate::xform_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "xform_save",
                crate::xform_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "xform_delete",
                crate::xform_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_create",
                crate::view_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_save",
                crate::view_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_delete",
                crate::view_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "templateform_create",
                crate::templateform_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "templateform_save",
                crate::templateform_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "templateform_delete",
                crate::templateform_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
        ];

        assert_eq!(cases.len(), 17, "本批 17 把写必须逐条点名，少一把即漏格");
        let bad: Vec<&str> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(n, _)| *n)
            .collect();
        assert!(
            bad.is_empty(),
            "以下写未返回 403（门缺失或门禁晚于 SQL）：{bad:?}"
        );
    }

    /// 尺子自证：同一池、同一会话，去掉门禁直接落进业务体时错误形态**不是** 403
    /// （离线拿不到库 → `Internal`）。否则上面的「全部 403」会因为库缺失而恒真。
    #[tokio::test]
    async fn gate_is_what_rejects_not_the_missing_database() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin-cms");

        let gated = crate::dict_delete(
            Extension(pool.clone()),
            Extension(sess.clone()),
            Path("x1".to_string()),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(gated, Err(AppError::Forbidden)),
            "带门必须 Forbidden，实际 {gated:?}"
        );

        let ungated = shared::crud_delete(&pool, &crate::dict_spec(), "x1")
            .await
            .map(|_| ());
        assert!(
            !matches!(ungated, Err(AppError::Forbidden)),
            "无门路径不应伪装成 403，否则上面的断言恒真：{ungated:?}"
        );
    }

    // ══ 第十四批（R4-14）：GET 动词别名写族 `mockdeletetoget` ═══════════
    //
    // `mockdeletetoget` 把 DELETE 语义挂在 GET 上，而 `csrf_middleware` 与 `requires_admin`
    // 两道中间件**只看方法** ⇒ GET 一律放行，这一族是全仓最危险的写面。修前它们既不取会话
    // 也不校归属，任一有效会话（乃至旧签名下根本不读会话）可删任意行。现按「删的是谁的行」
    // 分三档收口：owner（本人或 admin）、admin（无属主的结构/系统表）、派生（经父资源属主）。
    //
    // 别名不再各自复制删除逻辑，而是共用 `soft_delete_after_gate`，故先钉它的三态真值表。

    /// `soft_delete_after_gate` 三态：Forbidden→403、NotFound→200+error、
    /// Allowed→**继续落笔**（无库时 500 `Internal`）。
    /// Allowed 那格必须是 `Internal` 而不是 `Forbidden`——这就是「403 来自门、
    /// 不来自缺库」的反证；若哪天有人把 Allowed 分支也写成拒绝，本臂当场红。
    #[tokio::test]
    async fn soft_delete_after_gate_three_states_are_distinct() {
        let pool = shared::testing::mock_pool();
        let call = |g: crate::U2Gate| async {
            crate::soft_delete_after_gate(&pool, g, "x_cms_form", "x1", "gone")
                .await
                .map(|Json(v)| serde_json::to_value(v).unwrap())
        };

        let forbidden = call(crate::U2Gate::Forbidden).await;
        assert!(
            matches!(forbidden, Err(AppError::Forbidden)),
            "Forbidden 格必须 403，实际 {forbidden:?}"
        );

        let body = call(crate::U2Gate::NotFound)
            .await
            .expect("NotFound 格应返回 200 信封");
        assert_eq!(
            body["type"], "error",
            "NotFound 格应是错误信封而不是成功：{body}"
        );
        assert_eq!(body["message"], "gone", "NotFound 格应回显调用方给的文案");

        let allowed = call(crate::U2Gate::Allowed).await;
        assert!(
            matches!(allowed, Err(AppError::Internal)),
            "Allowed 格必须走到落笔（无库 → Internal），不得伪装成 403：{allowed:?}"
        );
    }

    /// admin 档 3 把别名：非 admin（含无库 fail-closed）必须 403，逐把点名。
    /// `is_admin` 取不到库时判假，而**路径上任何一次真落笔都会先变成 500**，
    /// 所以「403 而非 500」本身就是门在写之前的顺序证明。
    #[tokio::test]
    async fn cms_get_alias_admin_arms_forbid_non_admin() {
        let pool = shared::testing::mock_pool();
        let ext = || Extension(shared::testing::test_session("u-nonadmin-alias"));
        let p = || Path("x1".to_string());

        /// 这三把 admin 面的统一返回形状：`clippy::type_complexity` 只认别名（对齐 `u2.rs`
        /// 里 `type ApiResult = Result<Json<ActionResult<Value>>, AppError>;` 的既有先例）。
        type ApiResult = Result<Json<shared::response::ActionResult<Value>>, AppError>;
        let cases: Vec<(&str, ApiResult)> = vec![
            (
                "design_appdict_id_mockdeletetoget",
                crate::design_appdict_id_mockdeletetoget(Extension(pool.clone()), ext(), p()).await,
            ),
            (
                "templateform_id_mockdeletetoget",
                crate::templateform_id_mockdeletetoget(Extension(pool.clone()), ext(), p()).await,
            ),
            (
                "permission_management_refresh_category_categoryId",
                crate::permission_management_refresh_category_categoryId(
                    Extension(pool.clone()),
                    ext(),
                    p(),
                )
                .await,
            ),
        ];

        assert_eq!(cases.len(), 3, "admin 档 3 把必须逐条点名，少一把即漏格");
        let bad: Vec<&str> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(n, _)| *n)
            .collect();
        assert!(
            bad.is_empty(),
            "以下 GET 别名未返回 403（门缺失或门禁晚于写）：{bad:?}"
        );
    }

    // ── 真库臂：owner / 派生档逐把点名 ─────────────────────────────
    //
    // 用 `test_pool()` 且 `#[ignore]`（与 surface 的 R4-8/R4-10/R4-11 同法）：
    // 运行时守卫会把「没起库」静默跳过的臂伪装成通过，这里要的是显式整族跑。

    const OWNER_ALIASES: [&str; 14] = [
        "appinfo_id_mockdeletetoget",
        "appinfo_erase_app_id",
        "appinfo_erase_app_id_mockdeletetoget",
        "categoryinfo_id_mockdeletetoget",
        "categoryinfo_erase_category_id",
        "categoryinfo_erase_category_id_mockdeletetoget",
        "comment_id_mockdeletetoget",
        "file_flag_mockdeletetoget",
        "fileinfo_id_mockdeletetoget",
        "form_id_mockdeletetoget",
        "script_id_mockdeletetoget",
        "view_id_mockdeletetoget",
        "viewcategory_id_mockdeletetoget",
        "viewfieldconfig_id_mockdeletetoget",
    ];

    /// 别名 → 被软删的那张表（`deleted_at` 落库与否就看它）。
    fn alias_table(name: &str) -> &'static str {
        match name {
            "appinfo_id_mockdeletetoget"
            | "appinfo_erase_app_id"
            | "appinfo_erase_app_id_mockdeletetoget" => "x_cms_appinfo",
            "categoryinfo_id_mockdeletetoget"
            | "categoryinfo_erase_category_id"
            | "categoryinfo_erase_category_id_mockdeletetoget" => "x_cms_categoryinfo",
            "comment_id_mockdeletetoget" => "x_cms_comment",
            "file_flag_mockdeletetoget" => "x_cms_file",
            "fileinfo_id_mockdeletetoget" => "x_cms_fileinfo",
            "form_id_mockdeletetoget" => "x_cms_form",
            "script_id_mockdeletetoget" => "x_cms_script",
            "view_id_mockdeletetoget" => "x_cms_view",
            "viewcategory_id_mockdeletetoget" => "x_cms_viewcategory",
            "viewfieldconfig_id_mockdeletetoget" => "x_cms_viewfieldconfig",
            other => panic!("未点名的 owner/派生档别名：{other}"),
        }
    }

    const SEED_APP: &str = "r49-alias-app";
    const SEED_DOC: &str = "r49-alias-doc";
    const SEED_VIEW: &str = "r49-alias-parent-view";
    const OWNER: &str = "r49-alias-owner";
    const STRANGER: &str = "r49-alias-stranger";

    /// 建一行属主为 `OWNER` 的目标行（派生资源连父行一起建）。
    /// SQL 里的标识符全是本文件常量，无外部输入。
    async fn seed_owner_row(client: &deadpool_postgres::Client, name: &str, id: &str) {
        let sqls: Vec<String> = match name {
            "appinfo_id_mockdeletetoget"
            | "appinfo_erase_app_id"
            | "appinfo_erase_app_id_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_appinfo (id, app_type, alias, manager) VALUES ('{id}', 'cms', '{id}', '{OWNER}')"),
            ],
            "categoryinfo_id_mockdeletetoget"
            | "categoryinfo_erase_category_id"
            | "categoryinfo_erase_category_id_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_appinfo (id, app_type, alias, manager) VALUES ('{SEED_APP}', 'cms', '{SEED_APP}', '{OWNER}') ON CONFLICT (id) DO NOTHING"),
                format!("INSERT INTO x_cms_categoryinfo (id, name, app_id) VALUES ('{id}', 'r49 类别', '{SEED_APP}')"),
            ],
            "comment_id_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_data_document (id, title, creator) VALUES ('{SEED_DOC}', 'r49 文档', '{OWNER}') ON CONFLICT (id) DO NOTHING"),
                format!("INSERT INTO x_cms_comment (id, doc_id, person_id) VALUES ('{id}', '{SEED_DOC}', '{OWNER}')"),
            ],
            "file_flag_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_file (id, name, size, creator) VALUES ('{id}', 'r49.bin', 1, '{OWNER}')"),
            ],
            "fileinfo_id_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_fileinfo (id, doc_id, original_name, upload_person) VALUES ('{id}', '{SEED_DOC}', 'r49.pdf', '{OWNER}')"),
            ],
            "form_id_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_form (id, app_id, name, creator) VALUES ('{id}', '{SEED_APP}', 'r49Form', '{OWNER}')"),
            ],
            "script_id_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_script (id, app_id, name, creator) VALUES ('{id}', '{SEED_APP}', 'r49Script', '{OWNER}')"),
            ],
            "view_id_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_view (id, name, creator) VALUES ('{id}', 'r49View', '{OWNER}')"),
            ],
            "viewcategory_id_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_viewcategory (id, name, creator) VALUES ('{id}', 'r49VC', '{OWNER}')"),
            ],
            "viewfieldconfig_id_mockdeletetoget" => vec![
                format!("INSERT INTO x_cms_view (id, name, creator) VALUES ('{SEED_VIEW}', 'r49ParentView', '{OWNER}') ON CONFLICT (id) DO NOTHING"),
                format!("INSERT INTO x_cms_viewfieldconfig (id, view_id, field_name) VALUES ('{id}', '{SEED_VIEW}', 'r49Field')"),
            ],
            other => panic!("未点名的 owner/派生档别名：{other}"),
        };
        for sql in sqls {
            client
                .execute(&sql, &[])
                .await
                .unwrap_or_else(|e| panic!("建数据失败（{name}）：{e} ｜ {sql}"));
        }
    }

    async fn wipe_alias_rows(client: &deadpool_postgres::Client, ids: &[&str]) {
        for t in [
            "x_cms_appinfo",
            "x_cms_categoryinfo",
            "x_cms_comment",
            "x_cms_file",
            "x_cms_fileinfo",
            "x_cms_form",
            "x_cms_script",
            "x_cms_view",
            "x_cms_viewcategory",
            "x_cms_viewfieldconfig",
            "x_cms_data_document",
            "x_cms_data_document_field",
        ] {
            for id in ids {
                let _ = client
                    .execute(
                        &format!("DELETE FROM {t} WHERE id = $1"),
                        &[&id.to_string()],
                    )
                    .await;
            }
        }
    }

    async fn is_soft_deleted(pool: &deadpool_postgres::Pool, table: &str, id: &str) -> bool {
        let client = pool.get().await.expect("取库失败");
        let row = client
            .query_opt(
                &format!("SELECT (deleted_at IS NOT NULL) AS gone FROM {table} WHERE id = $1"),
                &[&id.to_string()],
            )
            .await
            .unwrap_or_else(|e| panic!("{table} 回读失败：{e}"));
        row.map(|r| r.get::<_, bool>("gone")).unwrap_or(false)
    }

    /// 14 把（12 别名 + 2 正档）owner/派生档逐把点名调用。写成 match 而不是函数指针表：
    /// 少一把会在编译期点名不了、多一把落 `panic`，两头都不许悄悄溜过去。
    async fn call_owner_alias(
        name: &str,
        pool: &deadpool_postgres::Pool,
        person: &str,
        id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let ext = Extension(shared::testing::test_session(person));
        let p = Extension(pool.clone());
        let path = Path(id.to_string());
        let r = match name {
            "appinfo_id_mockdeletetoget" => crate::appinfo_id_mockdeletetoget(p, ext, path).await,
            "appinfo_erase_app_id" => crate::appinfo_erase_app_id(p, ext, path).await,
            "appinfo_erase_app_id_mockdeletetoget" => {
                crate::appinfo_erase_app_id_mockdeletetoget(p, ext, path).await
            }
            "categoryinfo_id_mockdeletetoget" => {
                crate::categoryinfo_id_mockdeletetoget(p, ext, path).await
            }
            "categoryinfo_erase_category_id" => {
                crate::categoryinfo_erase_category_id(p, ext, path).await
            }
            "categoryinfo_erase_category_id_mockdeletetoget" => {
                crate::categoryinfo_erase_category_id_mockdeletetoget(p, ext, path).await
            }
            "comment_id_mockdeletetoget" => crate::comment_id_mockdeletetoget(p, ext, path).await,
            "file_flag_mockdeletetoget" => crate::file_flag_mockdeletetoget(p, ext, path).await,
            "fileinfo_id_mockdeletetoget" => crate::fileinfo_id_mockdeletetoget(p, ext, path).await,
            "form_id_mockdeletetoget" => crate::form_id_mockdeletetoget(p, ext, path).await,
            "script_id_mockdeletetoget" => crate::script_id_mockdeletetoget(p, ext, path).await,
            "view_id_mockdeletetoget" => crate::view_id_mockdeletetoget(p, ext, path).await,
            "viewcategory_id_mockdeletetoget" => {
                crate::viewcategory_id_mockdeletetoget(p, ext, path).await
            }
            "viewfieldconfig_id_mockdeletetoget" => {
                crate::viewfieldconfig_id_mockdeletetoget(p, ext, path).await
            }
            other => panic!("未点名的 owner/派生档别名：{other}"),
        };
        r.map(|Json(v)| serde_json::to_value(v).unwrap())
    }

    /// 他人会话 403 且**行未被软删**；属主会话成功且 `deleted_at` 真落库。
    /// 两半都要：只断 403 抓不住「门在、但写也先执行了」；只断落库抓不住「门恒放行」。
    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn cms_get_alias_owner_arms_forbid_stranger_and_land_for_owner() {
        let pool = shared::testing::test_pool();
        let mut checked = 0usize;
        for name in OWNER_ALIASES {
            let id = format!("r49-alias-{name}");
            let table = alias_table(name);
            {
                let client = pool.get().await.expect("取库失败");
                wipe_alias_rows(&client, &[&id, SEED_APP, SEED_DOC, SEED_VIEW]).await;
                seed_owner_row(&client, name, &id).await;
            }
            assert!(
                !is_soft_deleted(&pool, table, &id).await,
                "{name}：建数据后目标行已是删除态，后面的断言全部恒真"
            );

            let r = call_owner_alias(name, &pool, STRANGER, &id).await;
            assert!(
                matches!(r, Err(AppError::Forbidden)),
                "{name}：他人会话未被 403，实际 {r:?}"
            );
            assert!(
                !is_soft_deleted(&pool, table, &id).await,
                "{name}：403 之后行却被真删了（门晚于写）"
            );

            let r = call_owner_alias(name, &pool, OWNER, &id).await;
            let body = r.unwrap_or_else(|e| panic!("{name}：属主会话未放行，实际 Err({e:?})"));
            assert_eq!(
                body["data"]["deleted"], true,
                "{name}：属主放行却回出非成功形状：{body}"
            );
            assert!(
                is_soft_deleted(&pool, table, &id).await,
                "{name}：属主放行后未真落库"
            );

            let client = pool.get().await.expect("取库失败");
            wipe_alias_rows(&client, &[&id]).await;
            checked += 1;
        }
        assert_eq!(
            OWNER_ALIASES.len(),
            14,
            "owner/派生档是 12 别名 + 2 正档 = 14 把，名单缩水即红"
        );
        assert_eq!(checked, OWNER_ALIASES.len(), "owner 档 14 把须逐把跑满");
    }

    /// 宏 `data_path_write_handlers` 的 8 把 `$mockdel`（path0…path7）：修前不取会话、无门，
    /// 任一有效会话可删任意文档的任意字段。现与 `$del` 同走 `write_doc_field_path(Remove)`，
    /// 即按文档 `creator` 门。逐把点名：他人 403 且字段行未删，属主成功且字段行软删。
    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn cms_data_document_path_alias_gate_lands() {
        const ARMS: [(&str, &[&str]); 8] = [
            ("data_document_id_path0_mockdeletetoget", &["f0"]),
            (
                "data_document_id_path0_path1_mockdeletetoget",
                &["f0", "f1"],
            ),
            (
                "data_document_id_path0_path1_path2_mockdeletetoget",
                &["f0", "f1", "f2"],
            ),
            (
                "data_document_id_path0_path1_path2_path3_mockdeletetoget",
                &["f0", "f1", "f2", "f3"],
            ),
            (
                "data_document_id_path0_path1_path2_path3_path4_mockdeletetoget",
                &["f0", "f1", "f2", "f3", "f4"],
            ),
            (
                "data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget",
                &["f0", "f1", "f2", "f3", "f4", "f5"],
            ),
            (
                "data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget",
                &["f0", "f1", "f2", "f3", "f4", "f5", "f6"],
            ),
            (
                "data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget",
                &["f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7"],
            ),
        ];
        let pool = shared::testing::test_pool();
        let doc = "r49-alias-docpath";

        async fn call(
            name: &str,
            pool: &deadpool_postgres::Pool,
            person: &str,
            doc: &str,
            segs: &[&str],
        ) -> Result<serde_json::Value, AppError> {
            let p = Extension(pool.clone());
            let s = Extension(shared::testing::test_session(person));
            // 宏生成的 Path<[String; $arity]> 第一段就是文档 id（write_doc_field_path 里 split_first）
            let mut owned: Vec<String> = vec![doc.to_string()];
            owned.extend(segs.iter().map(|x| x.to_string()));
            let r = match name {
                "data_document_id_path0_mockdeletetoget" => {
                    crate::data_document_id_path0_mockdeletetoget(p, s, Path(owned.try_into().unwrap()))
                        .await
                }
                "data_document_id_path0_path1_mockdeletetoget" => {
                    crate::data_document_id_path0_path1_mockdeletetoget(p, s, Path(owned.try_into().unwrap()))
                        .await
                }
                "data_document_id_path0_path1_path2_mockdeletetoget" => {
                    crate::data_document_id_path0_path1_path2_mockdeletetoget(p, s, Path(owned.try_into().unwrap()))
                        .await
                }
                "data_document_id_path0_path1_path2_path3_mockdeletetoget" => {
                    crate::data_document_id_path0_path1_path2_path3_mockdeletetoget(p, s, Path(owned.try_into().unwrap()))
                        .await
                }
                "data_document_id_path0_path1_path2_path3_path4_mockdeletetoget" => {
                    crate::data_document_id_path0_path1_path2_path3_path4_mockdeletetoget(p, s, Path(owned.try_into().unwrap()))
                        .await
                }
                "data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget" => {
                    crate::data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget(p, s, Path(owned.try_into().unwrap()))
                        .await
                }
                "data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget" => {
                    crate::data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget(p, s, Path(owned.try_into().unwrap()))
                        .await
                }
                "data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget" => {
                    crate::data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget(p, s, Path(owned.try_into().unwrap()))
                        .await
                }
                other => panic!("未点名的 $mockdel 档：{other}"),
            };
            r.map(|Json(v)| serde_json::to_value(v).unwrap())
        }

        for (name, segs) in ARMS {
            let field = segs.join(".");
            {
                let client = pool.get().await.expect("取库失败");
                wipe_alias_rows(&client, &[doc]).await;
                // 字段行按 doc_id 挂，`id` 回擦擦不到——不清这层，第二轮插同名 id 直接撞主键。
                let _ = client
                    .execute(
                        "DELETE FROM x_cms_data_document_field WHERE doc_id = $1",
                        &[&doc.to_string()],
                    )
                    .await;
                client
                    .execute(
                        &format!(
                            "INSERT INTO x_cms_data_document (id, title, creator) VALUES ('{doc}', 'r49 文档', '{OWNER}')"
                        ),
                        &[],
                    )
                    .await
                    .unwrap_or_else(|e| panic!("建文档失败：{e}"));
                client
                    .execute(
                        "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) VALUES ($1, $2, $3, $4)",
                        &[&format!("r49-fl-{field}"), &doc.to_string(), &field, &"1".to_string()],
                    )
                    .await
                    .unwrap_or_else(|e| panic!("建字段行失败（{field}）：{e}"));
            }

            let r = call(name, &pool, STRANGER, doc, segs).await;
            assert!(
                matches!(r, Err(AppError::Forbidden)),
                "{name}：他人会话未被 403，实际 {r:?}"
            );
            let still = {
                let client = pool.get().await.expect("取库失败");
                client
                    .query_one(
                        "SELECT (deleted_at IS NULL) AS alive FROM x_cms_data_document_field WHERE doc_id = $1 AND field_name = $2",
                        &[&doc.to_string(), &field.clone()],
                    )
                    .await
                    .unwrap()
                    .get::<_, bool>("alive")
            };
            assert!(still, "{name}：403 之后字段行却被删了（门晚于写）");

            let r = call(name, &pool, OWNER, doc, segs).await;
            let body = r.unwrap_or_else(|e| panic!("{name}：属主会话未放行，实际 Err({e:?})"));
            assert_eq!(
                body["data"]["deleted"], true,
                "{name}：属主放行却回出非删除形状：{body}"
            );
        }
        let client = pool.get().await.expect("取库失败");
        let _ = client
            .execute(
                "DELETE FROM x_cms_data_document_field WHERE doc_id = $1",
                &[&doc.to_string()],
            )
            .await;
        wipe_alias_rows(&client, &[doc]).await;
    }

    /// `file_copy_u3`（GET 往任意 app 复制任意文件、连正文落库）：按源文件 creator 门。
    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn cms_file_copy_alias_gate_lands() {
        let pool = shared::testing::test_pool();
        let src = "r49-alias-copysrc";
        let app = "r49-alias-copyapp";
        {
            let client = pool.get().await.expect("取库失败");
            wipe_alias_rows(&client, &[src]).await;
            let _ = client
                .execute(
                    &format!("DELETE FROM x_cms_file WHERE app_id = '{app}'"),
                    &[],
                )
                .await;
            seed_owner_row(&client, "file_flag_mockdeletetoget", src).await;
        }
        let copies = || async {
            let client = pool.get().await.expect("取库失败");
            client
                .query_one(
                    "SELECT count(*) AS n FROM x_cms_file WHERE app_id = $1",
                    &[&app.to_string()],
                )
                .await
                .unwrap()
                .get::<_, i64>("n")
        };
        assert_eq!(
            copies().await,
            0,
            "副本槽位本该为空，否则下面的增量断言恒真"
        );

        let call = |person: &str| {
            crate::file_copy_u3(
                Extension(pool.clone()),
                Extension(shared::testing::test_session(person)),
                Path((src.to_string(), app.to_string())),
            )
        };
        let r = call(STRANGER)
            .await
            .map(|Json(v)| serde_json::to_value(v).unwrap());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "file_copy_u3：他人会话未被 403，实际 {r:?}"
        );
        assert_eq!(copies().await, 0, "file_copy_u3：403 之后仍复制出了行");

        let r = call(OWNER)
            .await
            .map(|Json(v)| serde_json::to_value(v).unwrap());
        assert!(r.is_ok(), "file_copy_u3：属主会话未放行，实际 {r:?}");
        assert_eq!(copies().await, 1, "file_copy_u3：属主放行后副本未落库");

        let client = pool.get().await.expect("取库失败");
        let _ = client
            .execute(
                &format!("DELETE FROM x_cms_file WHERE app_id = '{app}'"),
                &[],
            )
            .await;
        wipe_alias_rows(&client, &[src]).await;
    }

    /// `fileinfo_id_doc_docId_change_seqnumber_seqNumber`（GET 改附件排序时间戳）：按文档写者门。
    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn cms_fileinfo_seqnumber_alias_gate_lands() {
        let pool = shared::testing::test_pool();
        let doc = "r49-alias-seqdoc";
        let file = "r49-alias-seqfile";
        {
            let client = pool.get().await.expect("取库失败");
            wipe_alias_rows(&client, &[doc, file]).await;
            client
                .execute(
                    &format!("INSERT INTO x_cms_data_document (id, title, creator) VALUES ('{doc}', 'r49 文档', '{OWNER}')"),
                    &[],
                )
                .await
                .unwrap_or_else(|e| panic!("建文档失败：{e}"));
            client
                .execute(
                    &format!("INSERT INTO x_cms_fileinfo (id, doc_id, file_id, original_name, create_time, upload_person) VALUES ('{file}', '{doc}', 'r49-alias-seqbody', 'r49.pdf', TIMESTAMP '2020-01-01 00:00:00', '{OWNER}')"),
                    &[],
                )
                .await
                .unwrap_or_else(|e| panic!("建附件行失败：{e}"));
        }
        let create_time = {
            let client = pool.get().await.expect("取库失败");
            client
                .query_one(
                    &format!(
                        "SELECT create_time::text AS t FROM x_cms_fileinfo WHERE id = '{file}'"
                    ),
                    &[],
                )
                .await
                .unwrap()
                .get::<_, String>("t")
        };

        let call = |person: &str| {
            let pool = pool.clone();
            let person = person.to_string();
            async move {
                crate::fileinfo_id_doc_docId_change_seqnumber_seqNumber(
                    Extension(pool),
                    Extension(shared::testing::test_session(&person)),
                    Path((
                        "r49-alias-seqbody".to_string(),
                        doc.to_string(),
                        "1".to_string(),
                    )),
                )
                .await
            }
        };

        let r = call(STRANGER)
            .await
            .map(|Json(v)| serde_json::to_value(v).unwrap());
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "seqnumber 别名：他人会话未被 403，实际 {r:?}"
        );
        let after = {
            let client = pool.get().await.expect("取库失败");
            client
                .query_one(
                    &format!(
                        "SELECT create_time::text AS t FROM x_cms_fileinfo WHERE id = '{file}'"
                    ),
                    &[],
                )
                .await
                .unwrap()
                .get::<_, String>("t")
        };
        assert_eq!(
            after, create_time,
            "seqnumber 别名：403 之后 create_time 却被改了（门晚于写）"
        );

        let r = call(OWNER).await;
        let body = serde_json::to_value(r.map(|Json(v)| v).expect("属主会话未放行"))
            .expect("响应体应可序列化");
        assert_eq!(
            body["type"], "success",
            "seqnumber 别名：属主放行但回出非成功信封（写未落地）：{body}"
        );
        let client = pool.get().await.expect("取库失败");
        wipe_alias_rows(&client, &[doc, file]).await;
    }

    // 第四十八批（R7 收口）：input_compare / input_compare_mockputtopost 收会话门，
    // 无会话必须 401（门在 pool.get() 之前，dead pool 即证）。
    #[tokio::test]
    async fn b48_input_compare_reject_without_session() {
        let mgr = deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        let pool = deadpool_postgres::Pool::builder(mgr).build().unwrap();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "input_compare",
                crate::input_compare(Extension(pool.clone()), None, Json(serde_json::json!({})))
                    .await
                    .map(|_| ()),
            ),
            (
                "input_compare_mockputtopost",
                crate::input_compare_mockputtopost(
                    Extension(pool.clone()),
                    None,
                    Json(serde_json::json!({})),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 2, "本批 2 把必须逐把点名");
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
}
