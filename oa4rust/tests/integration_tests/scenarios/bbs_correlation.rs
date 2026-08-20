use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;
use uuid;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// BBS Post �?Comment �?Correlation cross-crate happy path
//
// Verifies: a BBS post can be created, a comment added, a correlation
// record inserted linking the post to another entity, and the correlation
// can be retrieved �?all through the real HTTP layer.
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn bbs_correlation_flow() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();

    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");

    let base = format!("http://{}", _addr);
    let auth_header = format!("Bearer {}", token);

    // Step 1: Create a BBS forum and section directly in DB for prerequisites
    {
        let client_db = pool.as_pg().unwrap().get().await.expect("failed to get pool client");
        client_db
            .execute(
                "INSERT INTO bbs_forum_info (id, name, description, disable) VALUES ($1, $2, $3, false) \
                 ON CONFLICT (id) DO NOTHING",
                &[&"forum-it-1", &"Integration Test Forum", &"Test forum for CI"],
            )
            .await
            .expect("insert forum failed");

        client_db
            .execute(
                "INSERT INTO bbs_section_info (id, forum_id, name, disable) VALUES ($1, $2, $3, false) \
                 ON CONFLICT (id) DO NOTHING",
                &[&"section-it-1", &"forum-it-1", &"Integration Test Section"],
            )
            .await
            .expect("insert section failed");
    }

    // Step 2: Create a BBS post (subject)
    let post_resp = client
        .post(format!("{}/jaxrs/bbs/subject/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "title": "Integration Test Post",
            "sectionId": "section-it-1",
            "authorId": "test-admin",
            "content": "This is a test post for the integration test pipeline"
        }))
        .send()
        .await
        .expect("create post request failed");

    assert_eq!(
        post_resp.status(),
        reqwest::StatusCode::OK,
        "create post failed: {}",
        post_resp.text().await.unwrap_or_default()
    );

    let post_body: serde_json::Value = post_resp.json().await.expect("invalid post response");
    let post_id = post_body["data"]["id"]
        .as_str()
        .expect("post id missing")
        .to_string();
    assert!(!post_id.is_empty(), "post id should not be empty");
    info!(post_id = %post_id, "bbs post created");

    // Step 3: Add a comment to the post directly in DB
    {
        let client_db = pool.as_pg().unwrap().get().await.expect("failed to get pool client");
        let comment_id = uuid::Uuid::new_v4().to_string();
        client_db
            .execute(
                "INSERT INTO bbs_comment_info (id, subject_id, author_id, content) VALUES ($1, $2, $3, $4)",
                &[&comment_id, &post_id, &String::from("test-admin"), &String::from("Great post!")],
            )
            .await
            .expect("insert comment failed");
        info!(comment_id = %comment_id, "comment added to post");
    }

    // Step 4: Create a correlation linking the post to another entity
    {
        let client_db = pool.as_pg().unwrap().get().await.expect("failed to get pool client");
        let corr_id = uuid::Uuid::new_v4().to_string();
        client_db
            .execute(
                "INSERT INTO x_correlation (id, source_type, source_id, target_type, target_id, weight) \
                 VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &corr_id,
                    &String::from("bbs_subject"),
                    &post_id,
                    &String::from("meeting"),
                    &"meeting-it-1",
                    &1,
                ],
            )
            .await
            .expect("insert correlation failed");
        info!(corr_id = %corr_id, "correlation record created");
    }

    // Step 5: Verify - retrieve the correlation via the correlation core entity endpoint
    let corr_resp = client
        .get(format!("{}/jaxrs/correlation/core/entity/list/by/bbs_subject/{}", base, post_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get correlation request failed");

    assert_eq!(
        corr_resp.status(),
        reqwest::StatusCode::OK,
        "get correlation failed: {}",
        corr_resp.text().await.unwrap_or_default()
    );

    let corr_body: serde_json::Value = corr_resp.json().await.expect("invalid correlation response");
    let correlations = corr_body["data"]["data"]
        .as_array()
        .expect("data array missing");
    assert!(
        !correlations.is_empty(),
        "expected at least one correlation"
    );

    let found = correlations.iter().any(|c| {
        c["sourceId"].as_str() == Some(post_id.as_str())
            && c["sourceType"].as_str() == Some("bbs_subject")
    });
    assert!(found, "correlation for bbs post not found in results");

    // Step 6: Verify - retrieve the post via the view endpoint
    let view_resp = client
        .get(format!("{}/jaxrs/bbs/subject/view/{}", base, post_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("view post request failed");

    assert_eq!(
        view_resp.status(),
        reqwest::StatusCode::OK,
        "view post failed: {}",
        view_resp.text().await.unwrap_or_default()
    );

    let view_body: serde_json::Value = view_resp.json().await.expect("invalid view response");
    assert_eq!(view_body["data"]["id"].as_str(), Some(post_id.as_str()));
    assert_eq!(
        view_body["data"]["title"].as_str(),
        Some("Integration Test Post")
    );

    // Shutdown the server
    server_handle.abort();
    let _ = server_handle.await;
}
