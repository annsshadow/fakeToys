// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// CMS extended entities CRUD lifecycle (U2.2)
//
// O20 词表修正（B）：原先全部打 `/api/cms_assemble_control/{entity}/...`，
// 这批前缀在 backend 已不存在；前端 frontend/packages/apis/src/index.ts 用的
// 是下面的 canonical 路由。证据见台账 O20 节。
//
// 同时按 O20 的 C 类删除两类断言（能力缺口，已确认无人使用）：
//   - appinfo / categoryinfo / comment 的 update：canonical 只有 create/get/
//     delete(+list)，无更新路由；前端也没有这三者的更新调用。
//   - categoryinfo 的 sortOrder / extContent 写入：canonical create 只接收
//     name / parentCategoryId / appId，这两个字段没有写入面。
//
// 覆盖：create → get（真实 DB 行）→（file/form/view 还有 update）→ delete
// → get/list 确认消失。
// ──────────────────────────────────────────────────────────────────────────────

fn uid(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{}-{}-{}", prefix, std::process::id(), nanos)
}

#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn cms_extended_crud_flow() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();

    let (_addr, server_handle, token) =
        crate::integration_tests::helpers::setup_test_server(pool.clone())
            .await
            .expect("failed to start test server");

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");

    let base = format!("http://{}", _addr);
    let auth_header = format!("Bearer {}", token);

    // ── appinfo: create → list → get → delete → get(gone) ────────────────────
    let app_id = uid("app-ext");
    let create_app = client
        .post(format!("{}/api/appinfo", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "id": app_id,
            "appType": "cms",
            "alias": "ext-app",
            "icon": "icon.png",
            "manager": "it-admin"
        }))
        .send()
        .await
        .expect("create appinfo request failed");
    assert_eq!(
        create_app.status(),
        reqwest::StatusCode::OK,
        "create appinfo failed: {}",
        create_app.text().await.unwrap_or_default()
    );
    let create_app_body: serde_json::Value = create_app.json().await.expect("bad create app");
    assert_eq!(
        create_app_body["data"]["id"].as_str(),
        Some(app_id.as_str())
    );

    let list_app = client
        .get(format!("{}/api/appinfo/list/all", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list appinfo failed");
    assert_eq!(list_app.status(), reqwest::StatusCode::OK);
    let list_app_body: serde_json::Value = list_app.json().await.expect("bad list app");
    let apps = list_app_body["data"]
        .as_array()
        .expect("appinfo list data array missing");
    assert!(apps
        .iter()
        .any(|a| a["id"].as_str() == Some(app_id.as_str())));

    let get_app = client
        .get(format!("{}/api/appinfo/{}", base, app_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get appinfo failed");
    assert_eq!(get_app.status(), reqwest::StatusCode::OK);
    let get_app_body: serde_json::Value = get_app.json().await.expect("bad get app");
    assert_eq!(get_app_body["data"]["id"].as_str(), Some(app_id.as_str()));
    assert_eq!(get_app_body["data"]["appType"].as_str(), Some("cms"));
    assert_eq!(get_app_body["data"]["alias"].as_str(), Some("ext-app"));
    assert_eq!(get_app_body["data"]["manager"].as_str(), Some("it-admin"));

    info!(id = %app_id, "appinfo created");

    // ── categoryinfo: create → get → delete → get(gone) ──────────────────────
    // 注意：canonical create 不收 sortOrder/extContent（C 类，见文件头）。
    let create_cat = client
        .post(format!("{}/api/categoryinfo", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "name": "Extended Category",
            "parentCategoryId": "",
            "appId": app_id
        }))
        .send()
        .await
        .expect("create categoryinfo failed");
    assert_eq!(
        create_cat.status(),
        reqwest::StatusCode::OK,
        "create categoryinfo failed: {}",
        create_cat.text().await.unwrap_or_default()
    );
    let create_cat_body: serde_json::Value = create_cat.json().await.expect("bad create cat");
    let cat_id = create_cat_body["data"]["id"]
        .as_str()
        .expect("create categoryinfo missing id")
        .to_string();

    let get_cat = client
        .get(format!("{}/api/categoryinfo/{}", base, cat_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get categoryinfo failed");
    assert_eq!(get_cat.status(), reqwest::StatusCode::OK);
    let get_cat_body: serde_json::Value = get_cat.json().await.expect("bad get cat");
    assert_eq!(
        get_cat_body["data"]["name"].as_str(),
        Some("Extended Category")
    );
    assert_eq!(
        get_cat_body["data"]["appId"].as_str(),
        Some(app_id.as_str())
    );

    let del_cat = client
        .delete(format!("{}/api/categoryinfo/{}", base, cat_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete categoryinfo failed");
    assert_eq!(del_cat.status(), reqwest::StatusCode::OK);
    let del_cat_body: serde_json::Value = del_cat.json().await.expect("bad del cat");
    assert_eq!(del_cat_body["data"]["deleted"].as_bool(), Some(true));

    let get_cat_gone = client
        .get(format!("{}/api/categoryinfo/{}", base, cat_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get deleted categoryinfo failed");
    assert_eq!(get_cat_gone.status(), reqwest::StatusCode::OK);
    let gone_cat: serde_json::Value = get_cat_gone.json().await.expect("bad gone cat");
    assert_eq!(gone_cat["message"].as_str(), Some("category not found"));
    info!(id = %cat_id, "categoryinfo created and soft-deleted");

    // ── comment: seed doc → create → get → delete → get(gone) ────────────────
    let doc_id = uid("doc-ext");
    {
        let pg = pool
            .as_pg()
            .expect("pg pool")
            .get()
            .await
            .expect("pg client");
        pg.execute(
            "INSERT INTO x_cms_data_document (id, title, content, author_id, status, creator) \
             VALUES ($1, 'Integration Doc', '', 'it-admin', 'published', 'it-admin')",
            &[&doc_id],
        )
        .await
        .expect("seed document failed");
    }
    let create_cmt = client
        .post(format!("{}/api/comment", base))
        .header("Authorization", &auth_header)
        .json(&json!({ "docId": doc_id, "content": "Nice article" }))
        .send()
        .await
        .expect("create comment failed");
    assert_eq!(
        create_cmt.status(),
        reqwest::StatusCode::OK,
        "create comment failed: {}",
        create_cmt.text().await.unwrap_or_default()
    );
    let create_cmt_body: serde_json::Value = create_cmt.json().await.expect("bad create cmt");
    let comment_id = create_cmt_body["data"]["id"]
        .as_str()
        .expect("create comment missing id")
        .to_string();
    assert_eq!(
        create_cmt_body["data"]["docId"].as_str(),
        Some(doc_id.as_str())
    );

    let get_cmt = client
        .get(format!("{}/api/comment/{}", base, comment_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get comment failed");
    assert_eq!(get_cmt.status(), reqwest::StatusCode::OK);
    let get_cmt_body: serde_json::Value = get_cmt.json().await.expect("bad get cmt");
    assert_eq!(
        get_cmt_body["data"]["docId"].as_str(),
        Some(doc_id.as_str())
    );
    assert_eq!(get_cmt_body["data"]["personId"].as_str(), Some("it-admin"));
    assert_eq!(
        get_cmt_body["data"]["content"].as_str(),
        Some("Nice article")
    );

    let del_cmt = client
        .delete(format!("{}/api/comment/{}", base, comment_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete comment failed");
    assert_eq!(del_cmt.status(), reqwest::StatusCode::OK);
    let del_cmt_body: serde_json::Value = del_cmt.json().await.expect("bad del cmt");
    assert_eq!(del_cmt_body["data"]["deleted"].as_bool(), Some(true));

    let get_cmt_gone = client
        .get(format!("{}/api/comment/{}", base, comment_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get deleted comment failed");
    assert_eq!(get_cmt_gone.status(), reqwest::StatusCode::OK);
    let gone_cmt: serde_json::Value = get_cmt_gone.json().await.expect("bad gone cmt");
    assert_eq!(gone_cmt["message"].as_str(), Some("comment not found"));
    info!(id = %comment_id, "comment created and soft-deleted");

    // ── file: create → get → update → get → delete → get(gone) ───────────────
    let create_file = client
        .post(format!("{}/api/file", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "appId": app_id,
            "name": "report.pdf",
            "size": 2048,
            "contentType": "application/pdf",
            "contentBase64": "JVBERi0xLjQK"
        }))
        .send()
        .await
        .expect("create file failed");
    assert_eq!(
        create_file.status(),
        reqwest::StatusCode::OK,
        "create file failed: {}",
        create_file.text().await.unwrap_or_default()
    );
    let create_file_body: serde_json::Value = create_file.json().await.expect("bad create file");
    let file_id = create_file_body["data"]["id"]
        .as_str()
        .expect("create file missing id")
        .to_string();

    let get_file = client
        .get(format!("{}/api/file/{}", base, file_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get file failed");
    assert_eq!(get_file.status(), reqwest::StatusCode::OK);
    let get_file_body: serde_json::Value = get_file.json().await.expect("bad get file");
    assert_eq!(get_file_body["data"]["id"].as_str(), Some(file_id.as_str()));
    assert_eq!(
        get_file_body["data"]["appId"].as_str(),
        Some(app_id.as_str())
    );
    assert_eq!(get_file_body["data"]["name"].as_str(), Some("report.pdf"));
    assert_eq!(get_file_body["data"]["size"].as_i64(), Some(2048));
    assert_eq!(
        get_file_body["data"]["contentType"].as_str(),
        Some("application/pdf")
    );

    // canonical 更新面是 POST /api/file/{id}/mockputtopost；它只更新
    // name/contentType/contentBase64（size 不在更新面，C 类）。
    let upd_file = client
        .post(format!("{}/api/file/{}/mockputtopost", base, file_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "name": "report-v2.pdf", "contentType": "text/plain" }))
        .send()
        .await
        .expect("update file failed");
    assert_eq!(upd_file.status(), reqwest::StatusCode::OK);
    let upd_file_body: serde_json::Value = upd_file.json().await.expect("bad upd file");
    assert_eq!(upd_file_body["data"]["updated"].as_bool(), Some(true));

    let get_file2 = client
        .get(format!("{}/api/file/{}", base, file_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get file after update failed");
    let get_file2_body: serde_json::Value = get_file2.json().await.expect("bad get file2");
    assert_eq!(
        get_file2_body["data"]["name"].as_str(),
        Some("report-v2.pdf")
    );
    assert_eq!(
        get_file2_body["data"]["contentType"].as_str(),
        Some("text/plain")
    );

    let del_file = client
        .delete(format!("{}/api/file/{}", base, file_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete file failed");
    assert_eq!(del_file.status(), reqwest::StatusCode::OK);
    let del_file_body: serde_json::Value = del_file.json().await.expect("bad del file");
    assert_eq!(del_file_body["data"]["deleted"].as_bool(), Some(true));

    let get_file_gone = client
        .get(format!("{}/api/file/{}", base, file_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get deleted file failed");
    assert_eq!(get_file_gone.status(), reqwest::StatusCode::OK);
    let gone_file: serde_json::Value = get_file_gone.json().await.expect("bad gone file");
    assert_eq!(gone_file["message"].as_str(), Some("file not found"));
    info!(id = %file_id, "file created, updated and soft-deleted");

    // ── form: create → get → update → get → delete → get(gone) ───────────────
    let create_form = client
        .post(format!("{}/api/form", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "appId": app_id,
            "name": "Feedback Form",
            "definition": {"fields": [{"name": "q1"}]},
            "status": "published"
        }))
        .send()
        .await
        .expect("create form failed");
    assert_eq!(
        create_form.status(),
        reqwest::StatusCode::OK,
        "create form failed: {}",
        create_form.text().await.unwrap_or_default()
    );
    let create_form_body: serde_json::Value = create_form.json().await.expect("bad create form");
    let form_id = create_form_body["data"]["id"]
        .as_str()
        .expect("create form missing id")
        .to_string();

    let get_form = client
        .get(format!("{}/api/form/{}", base, form_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get form failed");
    assert_eq!(get_form.status(), reqwest::StatusCode::OK);
    let get_form_body: serde_json::Value = get_form.json().await.expect("bad get form");
    assert_eq!(get_form_body["data"]["id"].as_str(), Some(form_id.as_str()));
    assert_eq!(
        get_form_body["data"]["appId"].as_str(),
        Some(app_id.as_str())
    );
    assert_eq!(
        get_form_body["data"]["name"].as_str(),
        Some("Feedback Form")
    );
    assert_eq!(
        get_form_body["data"]["definition"],
        json!({"fields": [{"name": "q1"}]})
    );
    assert_eq!(get_form_body["data"]["status"].as_str(), Some("published"));

    let upd_form = client
        .put(format!("{}/api/form/{}", base, form_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "name": "Feedback Form v2", "status": "draft" }))
        .send()
        .await
        .expect("update form failed");
    assert_eq!(upd_form.status(), reqwest::StatusCode::OK);

    let get_form2 = client
        .get(format!("{}/api/form/{}", base, form_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get form after update failed");
    let get_form2_body: serde_json::Value = get_form2.json().await.expect("bad get form2");
    assert_eq!(
        get_form2_body["data"]["name"].as_str(),
        Some("Feedback Form v2")
    );
    assert_eq!(get_form2_body["data"]["status"].as_str(), Some("draft"));

    let del_form = client
        .delete(format!("{}/api/form/{}", base, form_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete form failed");
    assert_eq!(del_form.status(), reqwest::StatusCode::OK);
    let del_form_body: serde_json::Value = del_form.json().await.expect("bad del form");
    assert_eq!(del_form_body["data"]["deleted"].as_bool(), Some(true));

    let get_form_gone = client
        .get(format!("{}/api/form/{}", base, form_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get deleted form failed");
    assert_eq!(get_form_gone.status(), reqwest::StatusCode::OK);
    let gone_form: serde_json::Value = get_form_gone.json().await.expect("bad gone form");
    assert_eq!(gone_form["message"].as_str(), Some("form not found"));
    info!(id = %form_id, "form created, updated and soft-deleted");

    // ── view: create → list(get) → update → list → delete → list(gone) ───────
    // canonical GET /api/view/{id} 实际返回整表列表（view_id 无 id 参数），
    // 故用 /api/view/list/all 断言存在性。
    let create_view = client
        .post(format!("{}/api/view", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "appId": app_id,
            "categoryId": cat_id,
            "name": "Main View",
            "viewConfig": "{\"layout\":\"grid\"}"
        }))
        .send()
        .await
        .expect("create view failed");
    assert_eq!(
        create_view.status(),
        reqwest::StatusCode::OK,
        "create view failed: {}",
        create_view.text().await.unwrap_or_default()
    );
    let create_view_body: serde_json::Value = create_view.json().await.expect("bad create view");
    let view_id = create_view_body["data"]["id"]
        .as_str()
        .expect("create view missing id")
        .to_string();

    let list_view = client
        .get(format!("{}/api/view/list/all", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list view failed");
    assert_eq!(list_view.status(), reqwest::StatusCode::OK);
    let list_view_body: serde_json::Value = list_view.json().await.expect("bad list view");
    let views = list_view_body["data"]
        .as_array()
        .expect("view list data array missing");
    let created_view = views
        .iter()
        .find(|v| v["id"].as_str() == Some(view_id.as_str()))
        .expect("created view missing from list");
    assert_eq!(created_view["name"].as_str(), Some("Main View"));
    assert_eq!(
        created_view["view_config"].as_str(),
        Some("{\"layout\":\"grid\"}")
    );

    let upd_view = client
        .put(format!("{}/api/view/{}", base, view_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "name": "Main View v2", "viewConfig": "{\"layout\":\"list\"}" }))
        .send()
        .await
        .expect("update view failed");
    assert_eq!(upd_view.status(), reqwest::StatusCode::OK);
    let upd_view_body: serde_json::Value = upd_view.json().await.expect("bad upd view");
    assert_eq!(upd_view_body["data"]["updated"].as_bool(), Some(true));

    let list_view2 = client
        .get(format!("{}/api/view/list/all", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list view after update failed");
    let list_view2_body: serde_json::Value = list_view2.json().await.expect("bad list view2");
    let views2 = list_view2_body["data"]
        .as_array()
        .expect("view list data array missing");
    let updated_view = views2
        .iter()
        .find(|v| v["id"].as_str() == Some(view_id.as_str()))
        .expect("updated view missing from list");
    assert_eq!(updated_view["name"].as_str(), Some("Main View v2"));
    assert_eq!(
        updated_view["view_config"].as_str(),
        Some("{\"layout\":\"list\"}")
    );

    let del_view = client
        .delete(format!("{}/api/view/{}", base, view_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete view failed");
    assert_eq!(del_view.status(), reqwest::StatusCode::OK);
    let del_view_body: serde_json::Value = del_view.json().await.expect("bad del view");
    assert_eq!(del_view_body["data"]["deleted"].as_bool(), Some(true));

    let list_view3 = client
        .get(format!("{}/api/view/list/all", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list view after delete failed");
    let list_view3_body: serde_json::Value = list_view3.json().await.expect("bad list view3");
    let views3 = list_view3_body["data"]
        .as_array()
        .expect("view list data array missing");
    assert!(!views3
        .iter()
        .any(|v| v["id"].as_str() == Some(view_id.as_str())));
    info!(id = %view_id, "view created, updated and soft-deleted");

    // ── appinfo 收尾：category/file/form/view 都以它为 appId 门，放在最后删 ──
    let del_app = client
        .delete(format!("{}/api/appinfo/{}", base, app_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete appinfo failed");
    assert_eq!(del_app.status(), reqwest::StatusCode::OK);
    let del_app_body: serde_json::Value = del_app.json().await.expect("bad del app");
    assert_eq!(del_app_body["data"]["deleted"].as_bool(), Some(true));

    let get_app_gone = client
        .get(format!("{}/api/appinfo/{}", base, app_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get deleted appinfo failed");
    assert_eq!(get_app_gone.status(), reqwest::StatusCode::OK);
    let gone_app: serde_json::Value = get_app_gone.json().await.expect("bad gone app");
    assert_eq!(gone_app["type"].as_str(), Some("error"));
    assert_eq!(gone_app["message"].as_str(), Some("appinfo not found"));
    info!(id = %app_id, "appinfo soft-deleted");

    server_handle.abort();
    let _ = server_handle.await;
}
