use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// CMS extended entities CRUD lifecycle (U2.2)
//
// Verifies the realized handlers for appinfo + categoryinfo + comment perform
// genuine DB reads/writes end-to-end through the real HTTP layer:
//   create → list (visible) → get-by-id → update → soft-delete → list (gone) →
//   get-by-id (404)
//
// Unique ids (pid + nanosecond timestamp) avoid collisions with other scenarios
// and with repeated local runs.
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

    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");

    let base = format!("http://{}", _addr);
    let auth_header = format!("Bearer {}", token);

    // ── appinfo lifecycle ────────────────────────────────────────────────────
    let app_id = uid("app-ext");
    let create_app = client
        .post(format!("{}/jaxrs/cms_assemble_control/appinfo/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "id": app_id,
            "appType": "cms",
            "alias": "ext-app",
            "icon": "icon.png",
            "enabled": true,
            "manager": "it-admin",
            "creator": "it-admin"
        }))
        .send()
        .await
        .expect("create appinfo failed");
    assert_eq!(create_app.status(), reqwest::StatusCode::OK);
    let create_app_body: serde_json::Value = create_app.json().await.expect("bad create app");
    assert_eq!(create_app_body["data"]["id"].as_str(), Some(app_id.as_str()));
    assert_eq!(create_app_body["data"]["status"].as_str(), Some("created"));
    info!(id = %app_id, "appinfo created");

    let list_app = client
        .get(format!("{}/jaxrs/cms_assemble_control/appinfo", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list appinfo failed");
    assert_eq!(list_app.status(), reqwest::StatusCode::OK);
    let list_app_body: serde_json::Value = list_app.json().await.expect("bad list app");
    let apps = list_app_body["data"]["data"].as_array().expect("app array missing");
    assert!(apps.iter().any(|a| a["id"].as_str() == Some(app_id.as_str())));

    let get_app = client
        .get(format!("{}/jaxrs/cms_assemble_control/appinfo/{}", base, app_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get appinfo failed");
    assert_eq!(get_app.status(), reqwest::StatusCode::OK);
    let get_app_body: serde_json::Value = get_app.json().await.expect("bad get app");
    assert_eq!(get_app_body["data"]["id"].as_str(), Some(app_id.as_str()));
    assert_eq!(get_app_body["data"]["app_type"].as_str(), Some("cms"));
    assert_eq!(get_app_body["data"]["alias"].as_str(), Some("ext-app"));
    assert_eq!(get_app_body["data"]["manager"].as_str(), Some("it-admin"));

    let upd_app = client
        .post(format!("{}/jaxrs/cms_assemble_control/appinfo/{}/update", base, app_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "alias": "ext-app-renamed", "enabled": false }))
        .send()
        .await
        .expect("update appinfo failed");
    assert_eq!(upd_app.status(), reqwest::StatusCode::OK);
    let upd_app_body: serde_json::Value = upd_app.json().await.expect("bad upd app");
    assert_eq!(upd_app_body["data"]["updated"].as_bool(), Some(true));

    let get_app2 = client
        .get(format!("{}/jaxrs/cms_assemble_control/appinfo/{}", base, app_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get appinfo after update failed");
    let get_app2_body: serde_json::Value = get_app2.json().await.expect("bad get app2");
    assert_eq!(get_app2_body["data"]["alias"].as_str(), Some("ext-app-renamed"));
    assert_eq!(get_app2_body["data"]["enabled"].as_bool(), Some(false));

    let del_app = client
        .post(format!("{}/jaxrs/cms_assemble_control/appinfo/{}/delete", base, app_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete appinfo failed");
    assert_eq!(del_app.status(), reqwest::StatusCode::OK);
    let del_app_body: serde_json::Value = del_app.json().await.expect("bad del app");
    assert_eq!(del_app_body["data"]["deleted"].as_bool(), Some(true));

    let get_app3 = client
        .get(format!("{}/jaxrs/cms_assemble_control/appinfo/{}", base, app_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get appinfo after delete failed");
    assert_eq!(get_app3.status(), reqwest::StatusCode::NOT_FOUND);
    info!(id = %app_id, "appinfo soft-deleted and verified gone");

    // ── categoryinfo lifecycle ───────────────────────────────────────────────
    let cat_id = uid("cat-ext");
    let create_cat = client
        .post(format!("{}/jaxrs/cms_assemble_control/categoryinfo/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "id": cat_id,
            "name": "Extended Category",
            "parentId": "",
            "appId": app_id,
            "sortOrder": 3,
            "status": "enabled",
            "extContent": "some content",
            "creator": "it-admin"
        }))
        .send()
        .await
        .expect("create categoryinfo failed");
    assert_eq!(create_cat.status(), reqwest::StatusCode::OK);
    let create_cat_body: serde_json::Value = create_cat.json().await.expect("bad create cat");
    assert_eq!(create_cat_body["data"]["id"].as_str(), Some(cat_id.as_str()));

    let get_cat = client
        .get(format!("{}/jaxrs/cms_assemble_control/categoryinfo/{}", base, cat_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get categoryinfo failed");
    assert_eq!(get_cat.status(), reqwest::StatusCode::OK);
    let get_cat_body: serde_json::Value = get_cat.json().await.expect("bad get cat");
    assert_eq!(get_cat_body["data"]["name"].as_str(), Some("Extended Category"));
    assert_eq!(get_cat_body["data"]["app_id"].as_str(), Some(app_id.as_str()));
    assert_eq!(get_cat_body["data"]["sort_order"].as_i64(), Some(3));
    assert_eq!(get_cat_body["data"]["ext_content"].as_str(), Some("some content"));

    let upd_cat = client
        .post(format!("{}/jaxrs/cms_assemble_control/categoryinfo/{}/update", base, cat_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "name": "Extended Category (Renamed)", "sortOrder": 7 }))
        .send()
        .await
        .expect("update categoryinfo failed");
    assert_eq!(upd_cat.status(), reqwest::StatusCode::OK);

    let get_cat2 = client
        .get(format!("{}/jaxrs/cms_assemble_control/categoryinfo/{}", base, cat_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get categoryinfo after update failed");
    let get_cat2_body: serde_json::Value = get_cat2.json().await.expect("bad get cat2");
    assert_eq!(get_cat2_body["data"]["name"].as_str(), Some("Extended Category (Renamed)"));
    assert_eq!(get_cat2_body["data"]["sort_order"].as_i64(), Some(7));

    let del_cat = client
        .post(format!("{}/jaxrs/cms_assemble_control/categoryinfo/{}/delete", base, cat_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete categoryinfo failed");
    assert_eq!(del_cat.status(), reqwest::StatusCode::OK);
    let del_cat_body: serde_json::Value = del_cat.json().await.expect("bad del cat");
    assert_eq!(del_cat_body["data"]["deleted"].as_bool(), Some(true));

    let get_cat3 = client
        .get(format!("{}/jaxrs/cms_assemble_control/categoryinfo/{}", base, cat_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get categoryinfo after delete failed");
    assert_eq!(get_cat3.status(), reqwest::StatusCode::NOT_FOUND);
    info!(id = %cat_id, "categoryinfo soft-deleted and verified gone");

    // ── comment lifecycle ────────────────────────────────────────────────────
    let comment_id = uid("comment-ext");
    let create_cmt = client
        .post(format!("{}/jaxrs/cms_assemble_control/comment/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "id": comment_id,
            "docId": "doc-ext-001",
            "personId": "it-admin",
            "content": "Nice article",
            "parentId": ""
        }))
        .send()
        .await
        .expect("create comment failed");
    assert_eq!(create_cmt.status(), reqwest::StatusCode::OK);
    let create_cmt_body: serde_json::Value = create_cmt.json().await.expect("bad create cmt");
    assert_eq!(create_cmt_body["data"]["id"].as_str(), Some(comment_id.as_str()));

    let get_cmt = client
        .get(format!("{}/jaxrs/cms_assemble_control/comment/{}", base, comment_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get comment failed");
    assert_eq!(get_cmt.status(), reqwest::StatusCode::OK);
    let get_cmt_body: serde_json::Value = get_cmt.json().await.expect("bad get cmt");
    assert_eq!(get_cmt_body["data"]["doc_id"].as_str(), Some("doc-ext-001"));
    assert_eq!(get_cmt_body["data"]["person_id"].as_str(), Some("it-admin"));
    assert_eq!(get_cmt_body["data"]["content"].as_str(), Some("Nice article"));

    let upd_cmt = client
        .post(format!("{}/jaxrs/cms_assemble_control/comment/{}/update", base, comment_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "content": "Nice article (edited)" }))
        .send()
        .await
        .expect("update comment failed");
    assert_eq!(upd_cmt.status(), reqwest::StatusCode::OK);

    let get_cmt2 = client
        .get(format!("{}/jaxrs/cms_assemble_control/comment/{}", base, comment_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get comment after update failed");
    let get_cmt2_body: serde_json::Value = get_cmt2.json().await.expect("bad get cmt2");
    assert_eq!(get_cmt2_body["data"]["content"].as_str(), Some("Nice article (edited)"));

    let del_cmt = client
        .post(format!("{}/jaxrs/cms_assemble_control/comment/{}/delete", base, comment_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete comment failed");
    assert_eq!(del_cmt.status(), reqwest::StatusCode::OK);
    let del_cmt_body: serde_json::Value = del_cmt.json().await.expect("bad del cmt");
    assert_eq!(del_cmt_body["data"]["deleted"].as_bool(), Some(true));

    let get_cmt3 = client
        .get(format!("{}/jaxrs/cms_assemble_control/comment/{}", base, comment_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get comment after delete failed");
    assert_eq!(get_cmt3.status(), reqwest::StatusCode::NOT_FOUND);
    info!(id = %comment_id, "comment soft-deleted and verified gone");

    // ── file lifecycle ───────────────────────────────────────────────────────
    let file_id = uid("file-ext");
    let create_file = client
        .post(format!("{}/jaxrs/cms_assemble_control/file/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "id": file_id,
            "appId": "app-ext",
            "name": "report.pdf",
            "size": 2048,
            "contentType": "application/pdf",
            "contentBase64": "JVBERi0xLjQK",
            "creator": "it-admin"
        }))
        .send()
        .await
        .expect("create file failed");
    assert_eq!(create_file.status(), reqwest::StatusCode::OK);
    let create_file_body: serde_json::Value = create_file.json().await.expect("bad create file");
    assert_eq!(create_file_body["data"]["id"].as_str(), Some(file_id.as_str()));

    let get_file = client
        .get(format!("{}/jaxrs/cms_assemble_control/file/{}", base, file_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get file failed");
    assert_eq!(get_file.status(), reqwest::StatusCode::OK);
    let get_file_body: serde_json::Value = get_file.json().await.expect("bad get file");
    assert_eq!(get_file_body["data"]["id"].as_str(), Some(file_id.as_str()));
    assert_eq!(get_file_body["data"]["app_id"].as_str(), Some("app-ext"));
    assert_eq!(get_file_body["data"]["name"].as_str(), Some("report.pdf"));
    assert_eq!(get_file_body["data"]["size"].as_i64(), Some(2048));
    assert_eq!(get_file_body["data"]["content_type"].as_str(), Some("application/pdf"));
    assert_eq!(get_file_body["data"]["content_base64"].as_str(), Some("JVBERi0xLjQK"));

    let upd_file = client
        .post(format!("{}/jaxrs/cms_assemble_control/file/{}/update", base, file_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "name": "report-v2.pdf", "size": 4096 }))
        .send()
        .await
        .expect("update file failed");
    assert_eq!(upd_file.status(), reqwest::StatusCode::OK);

    let get_file2 = client
        .get(format!("{}/jaxrs/cms_assemble_control/file/{}", base, file_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get file after update failed");
    let get_file2_body: serde_json::Value = get_file2.json().await.expect("bad get file2");
    assert_eq!(get_file2_body["data"]["name"].as_str(), Some("report-v2.pdf"));
    assert_eq!(get_file2_body["data"]["size"].as_i64(), Some(4096));

    let del_file = client
        .post(format!("{}/jaxrs/cms_assemble_control/file/{}/delete", base, file_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete file failed");
    assert_eq!(del_file.status(), reqwest::StatusCode::OK);
    let del_file_body: serde_json::Value = del_file.json().await.expect("bad del file");
    assert_eq!(del_file_body["data"]["deleted"].as_bool(), Some(true));

    let get_file3 = client
        .get(format!("{}/jaxrs/cms_assemble_control/file/{}", base, file_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get file after delete failed");
    assert_eq!(get_file3.status(), reqwest::StatusCode::NOT_FOUND);
    info!(id = %file_id, "file soft-deleted and verified gone");

    // ── form lifecycle ───────────────────────────────────────────────────────
    let form_id = uid("form-ext");
    let create_form = client
        .post(format!("{}/jaxrs/cms_assemble_control/form/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "id": form_id,
            "appId": "app-ext",
            "name": "Feedback Form",
            "definition": "{\"fields\":[{\"name\":\"q1\"}]}",
            "status": "published",
            "creator": "it-admin"
        }))
        .send()
        .await
        .expect("create form failed");
    assert_eq!(create_form.status(), reqwest::StatusCode::OK);
    let create_form_body: serde_json::Value = create_form.json().await.expect("bad create form");
    assert_eq!(create_form_body["data"]["id"].as_str(), Some(form_id.as_str()));

    let get_form = client
        .get(format!("{}/jaxrs/cms_assemble_control/form/{}", base, form_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get form failed");
    assert_eq!(get_form.status(), reqwest::StatusCode::OK);
    let get_form_body: serde_json::Value = get_form.json().await.expect("bad get form");
    assert_eq!(get_form_body["data"]["id"].as_str(), Some(form_id.as_str()));
    assert_eq!(get_form_body["data"]["app_id"].as_str(), Some("app-ext"));
    assert_eq!(get_form_body["data"]["name"].as_str(), Some("Feedback Form"));
    assert_eq!(get_form_body["data"]["definition"].as_str(), Some("{\"fields\":[{\"name\":\"q1\"}]}"));
    assert_eq!(get_form_body["data"]["status"].as_str(), Some("published"));

    let upd_form = client
        .post(format!("{}/jaxrs/cms_assemble_control/form/{}/update", base, form_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "name": "Feedback Form v2", "status": "draft" }))
        .send()
        .await
        .expect("update form failed");
    assert_eq!(upd_form.status(), reqwest::StatusCode::OK);

    let get_form2 = client
        .get(format!("{}/jaxrs/cms_assemble_control/form/{}", base, form_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get form after update failed");
    let get_form2_body: serde_json::Value = get_form2.json().await.expect("bad get form2");
    assert_eq!(get_form2_body["data"]["name"].as_str(), Some("Feedback Form v2"));
    assert_eq!(get_form2_body["data"]["status"].as_str(), Some("draft"));

    let del_form = client
        .post(format!("{}/jaxrs/cms_assemble_control/form/{}/delete", base, form_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete form failed");
    assert_eq!(del_form.status(), reqwest::StatusCode::OK);
    let del_form_body: serde_json::Value = del_form.json().await.expect("bad del form");
    assert_eq!(del_form_body["data"]["deleted"].as_bool(), Some(true));

    let get_form3 = client
        .get(format!("{}/jaxrs/cms_assemble_control/form/{}", base, form_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get form after delete failed");
    assert_eq!(get_form3.status(), reqwest::StatusCode::NOT_FOUND);
    info!(id = %form_id, "form soft-deleted and verified gone");

    // ── view lifecycle ───────────────────────────────────────────────────────
    let view_id = uid("view-ext");
    let create_view = client
        .post(format!("{}/jaxrs/cms_assemble_control/view/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "id": view_id,
            "appId": "app-ext",
            "categoryId": cat_id,
            "name": "Main View",
            "viewConfig": "{\"layout\":\"grid\"}",
            "creator": "it-admin"
        }))
        .send()
        .await
        .expect("create view failed");
    assert_eq!(create_view.status(), reqwest::StatusCode::OK);
    let create_view_body: serde_json::Value = create_view.json().await.expect("bad create view");
    assert_eq!(create_view_body["data"]["id"].as_str(), Some(view_id.as_str()));

    let get_view = client
        .get(format!("{}/jaxrs/cms_assemble_control/view/{}", base, view_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get view failed");
    assert_eq!(get_view.status(), reqwest::StatusCode::OK);
    let get_view_body: serde_json::Value = get_view.json().await.expect("bad get view");
    assert_eq!(get_view_body["data"]["id"].as_str(), Some(view_id.as_str()));
    assert_eq!(get_view_body["data"]["app_id"].as_str(), Some("app-ext"));
    assert_eq!(get_view_body["data"]["category_id"].as_str(), Some(cat_id.as_str()));
    assert_eq!(get_view_body["data"]["name"].as_str(), Some("Main View"));
    assert_eq!(get_view_body["data"]["view_config"].as_str(), Some("{\"layout\":\"grid\"}"));

    let upd_view = client
        .post(format!("{}/jaxrs/cms_assemble_control/view/{}/update", base, view_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "name": "Main View v2", "viewConfig": "{\"layout\":\"list\"}" }))
        .send()
        .await
        .expect("update view failed");
    assert_eq!(upd_view.status(), reqwest::StatusCode::OK);

    let get_view2 = client
        .get(format!("{}/jaxrs/cms_assemble_control/view/{}", base, view_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get view after update failed");
    let get_view2_body: serde_json::Value = get_view2.json().await.expect("bad get view2");
    assert_eq!(get_view2_body["data"]["name"].as_str(), Some("Main View v2"));
    assert_eq!(get_view2_body["data"]["view_config"].as_str(), Some("{\"layout\":\"list\"}"));

    let del_view = client
        .post(format!("{}/jaxrs/cms_assemble_control/view/{}/delete", base, view_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete view failed");
    assert_eq!(del_view.status(), reqwest::StatusCode::OK);
    let del_view_body: serde_json::Value = del_view.json().await.expect("bad del view");
    assert_eq!(del_view_body["data"]["deleted"].as_bool(), Some(true));

    let get_view3 = client
        .get(format!("{}/jaxrs/cms_assemble_control/view/{}", base, view_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get view after delete failed");
    assert_eq!(get_view3.status(), reqwest::StatusCode::NOT_FOUND);
    info!(id = %view_id, "view soft-deleted and verified gone");

    server_handle.abort();
    let _ = server_handle.await;
}
