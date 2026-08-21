use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// Data integrity scenarios (plan002 U9: concurrency / data integrity)
//
// concurrent_document_updates: 8 concurrent tasks atomically increment the same
// CMS document view counter (UPDATE ... SET view_count = view_count + 1);
// the final value must equal 8 — proving no lost updates under concurrency.
//
// soft_delete_isolation: create → soft-delete → the default list query must
// hide the row while the physical row keeps its deleted_at marker; the
// business layer exposes no restore path, so isolation must persist.
// ──────────────────────────────────────────────────────────────────────────────

pub async fn concurrent_document_updates() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();
    let pg = pool
        .as_pg()
        .cloned()
        .expect("data_integrity scenarios require PostgreSQL pool");

    let doc_id = "di-conc-doc-001";

    // Seed a CMS document row plus its view-count counter row (reset to 0).
    {
        let client = pg.get().await.expect("failed to get pool client");
        client
            .execute(
                "INSERT INTO x_cms_data_document (id, title, content, author_id, status) \
                 VALUES ($1, $2, $3, $4, $5) \
                 ON CONFLICT (id) DO UPDATE SET deleted_at = NULL",
                &[
                    &doc_id,
                    &"Data Integrity Concurrent Doc",
                    &"seed content for concurrent update scenario",
                    &"person-it-admin",
                    &"published",
                ],
            )
            .await
            .expect("seed document failed");

        client
            .execute(
                "INSERT INTO x_cms_document_view_count (id, doc_id, view_count) \
                 VALUES ($1, $2, 0) \
                 ON CONFLICT (doc_id) DO UPDATE SET view_count = 0",
                &[&"di-conc-vc-001", &doc_id],
            )
            .await
            .expect("seed view count failed");
    }

    // 8 concurrent tasks each perform an atomic increment on the same row.
    const CONCURRENCY: usize = 8;
    let mut handles = Vec::new();
    for task in 0..CONCURRENCY {
        let pg = pg.clone();
        handles.push(tokio::spawn(async move {
            let client = pg.get().await.expect("failed to get pool client");
            let rows = client
                .execute(
                    "UPDATE x_cms_document_view_count \
                     SET view_count = view_count + 1 \
                     WHERE doc_id = $1",
                    &[&doc_id],
                )
                .await
                .expect("atomic increment failed");
            assert_eq!(rows, 1, "task {} updated no rows", task);
        }));
    }
    for handle in handles {
        handle.await.expect("increment task panicked");
    }

    // Final value must equal the number of tasks — no lost updates.
    let client = pg.get().await.expect("failed to get pool client");
    let row = client
        .query_one(
            "SELECT view_count FROM x_cms_document_view_count WHERE doc_id = $1",
            &[&doc_id],
        )
        .await
        .expect("read final count failed");
    let final_count: i32 = row.get(0);
    assert_eq!(
        final_count, CONCURRENCY as i32,
        "lost update detected: expected {}, got {}",
        CONCURRENCY, final_count
    );
    info!(doc_id = %doc_id, final_count = %final_count, "concurrent increments all persisted");
}

pub async fn soft_delete_isolation() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();
    let pg = pool
        .as_pg()
        .cloned()
        .expect("data_integrity scenarios require PostgreSQL pool");

    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");

    let base = format!("http://{}", _addr);
    let auth_header = format!("Bearer {}", token);

    let doc_id = "di-softdel-doc-001";

    // Step 1: Create a document through the real HTTP layer.
    let create_resp = client
        .post(format!("{}/jaxrs/cms_assemble_control/data/document/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "id": doc_id,
            "appId": "app-it-di-001",
            "categoryId": "cat-it-di-001",
            "title": "Soft Delete Isolation Doc",
            "content": "Body of the soft delete isolation document.",
            "authorId": "person-it-admin",
            "status": "published"
        }))
        .send()
        .await
        .expect("create document request failed");
    assert_eq!(
        create_resp.status(),
        reqwest::StatusCode::OK,
        "create document failed: {}",
        create_resp.text().await.unwrap_or_default()
    );
    info!(doc_id = %doc_id, "document created");

    // Step 2: Default list filter (deleted_at IS NULL) must see the fresh row.
    {
        let db = pg.get().await.expect("failed to get pool client");
        let visible: i64 = db
            .query_one(
                "SELECT COUNT(*) FROM x_cms_data_document \
                 WHERE id = $1 AND deleted_at IS NULL",
                &[&doc_id],
            )
            .await
            .expect("visibility check failed")
            .get(0);
        assert_eq!(visible, 1, "fresh document invisible to default list filter");
    }

    // Step 3: Soft delete through the real HTTP layer.
    let del_resp = client
        .post(format!("{}/jaxrs/cms_assemble_control/data/document/{}/delete", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete document request failed");
    assert_eq!(del_resp.status(), reqwest::StatusCode::OK);
    let del_body: serde_json::Value = del_resp.json().await.expect("invalid delete response");
    assert_eq!(del_body["data"]["deleted"].as_bool(), Some(true));

    // Step 4: Row must be hidden from the default filter but keep its
    // deleted_at marker — soft delete, not a hard delete.
    {
        let db = pg.get().await.expect("failed to get pool client");
        let visible: i64 = db
            .query_one(
                "SELECT COUNT(*) FROM x_cms_data_document \
                 WHERE id = $1 AND deleted_at IS NULL",
                &[&doc_id],
            )
            .await
            .expect("post-delete visibility check failed")
            .get(0);
        assert_eq!(visible, 0, "soft-deleted document still visible to default filter");

        let marked: i64 = db
            .query_one(
                "SELECT COUNT(*) FROM x_cms_data_document \
                 WHERE id = $1 AND deleted_at IS NOT NULL",
                &[&doc_id],
            )
            .await
            .expect("deleted_at marker check failed")
            .get(0);
        assert_eq!(marked, 1, "physical row missing deleted_at marker after soft delete");
    }
    info!(doc_id = %doc_id, "soft-deleted: hidden from default filter, marker present");

    // Step 5: Business exposes no restore endpoint — isolation must persist
    // on both the HTTP list/get paths and the default DB filter.
    let list_resp = client
        .get(format!("{}/jaxrs/cms_assemble_control/data/document", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list after delete failed");
    assert_eq!(list_resp.status(), reqwest::StatusCode::OK);
    let list_body: serde_json::Value = list_resp.json().await.expect("invalid list response");
    let docs = list_body["data"]["data"].as_array().expect("data array missing");
    assert!(
        !docs.iter().any(|d| d["id"].as_str() == Some(doc_id)),
        "soft-deleted document still present in HTTP list"
    );

    let get_resp = client
        .get(format!("{}/jaxrs/cms_assemble_control/data/document/{}", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get after delete failed");
    assert_eq!(
        get_resp.status(),
        reqwest::StatusCode::NOT_FOUND,
        "soft-deleted document should 404 on get"
    );
    info!(doc_id = %doc_id, "isolation persisted after soft delete");

    server_handle.abort();
    let _ = server_handle.await;
}
