// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use super::db::TestPool;
use anyhow::Context as _;
use base64::Engine;
use chrono::NaiveDateTime;
use deadpool_postgres::Pool;
use hmac::{KeyInit, Mac};
use oa4rust::create_app_with_sea_orm;
use shared::{rate_limit::RateLimiter, session::SessionManager};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

// ──────────────────────────────────────────────────────────────────────────────
// helpers — shared utilities for integration scenario tests
// ──────────────────────────────────────────────────────────────────────────────

/// Spin up the full oa4rust application on a random available port.
///
/// Returns (SocketAddr, JoinHandle, admin_token). The JoinHandle must be
/// aborted by the caller when the scenario is done.
///
/// Requires PostgreSQL pool (application uses deadpool_postgres::Pool).
pub async fn setup_test_server(
    pool: Arc<TestPool>,
) -> anyhow::Result<(SocketAddr, JoinHandle<()>, String)> {
    super::error_capture::init();
    let pg_pool = pool.as_pg().cloned().expect(
        "setup_test_server requires PostgreSQL pool; MySQL scenario tests are not yet supported",
    );

    let session_manager = SessionManager::with_pool(pg_pool.clone());
    let rate_limiter = RateLimiter::new();

    // `create_app` 的 SeaORM 连接取自 DATABASE_URL（测试进程里是开发库），
    // 而 deadpool 池指向一次性测试库；不注入同库 SeaORM 连接时，*_core_entity
    // 的写入会落到开发库，测试按测试库断言必然读不到行。这里按池的实际库名
    // 构造 SeaORM 连接，保证应用内两种连接同源。
    let db_name = {
        let client = pg_pool
            .get()
            .await
            .context("failed to acquire client for database name")?;
        let row = client
            .query_one("SELECT current_database()", &[])
            .await
            .context("failed to read current_database()")?;
        row.get::<_, String>(0)
    };
    let base_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/postgres".to_string());
    let mut url = url::Url::parse(&base_url).context("invalid DATABASE_URL")?;
    url.set_path(&db_name);
    let sea_orm_pool = shared::db::create_sea_orm_pool_with_url(url.to_string())
        .await
        .context("failed to build SeaORM pool for the test database")?;

    let app = create_app_with_sea_orm(
        pg_pool.clone(),
        sea_orm_pool,
        session_manager.clone(),
        rate_limiter,
    )
    .await
    .context("failed to build app for integration test")?;

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("failed to bind test server")?;
    let addr = listener.local_addr()?;

    let token = seed_test_data(&pool, &session_manager).await?;

    let handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        {
            tracing::warn!(error = %e, "test server terminated with error");
        }
    });

    tokio::time::sleep(Duration::from_millis(200)).await;

    Ok((addr, handle, token))
}

/// Insert a test admin user and role assignments, create a session,
/// and return the raw token (no HMAC signature when no SESSION_HMAC_SECRET).
///
/// Accepts `&TestPool`; only PostgreSQL is currently supported for seeding.
pub async fn seed_test_data(
    pool: &TestPool,
    session_manager: &SessionManager,
) -> anyhow::Result<String> {
    let pg_pool = pool
        .as_pg()
        .expect("seed_test_data requires PostgreSQL pool; MySQL seeding not yet supported");

    let client = pg_pool
        .get()
        .await
        .context("failed to get pool client for seeding")?;

    client
        .execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, deleted_at) \
             VALUES ($1, $2, $3, $4, false, NULL) \
             ON CONFLICT (unique_id) DO UPDATE SET name = EXCLUDED.name",
            &[
                &"person-it-admin",
                &"it-admin",
                &"IT Admin",
                &"$2b$04$eKzG7rDxhGXY/bIqz.WZHO0E4XLhK.1qZJ9IoG6q7oB7IoG6q7oBm",
            ],
        )
        .await
        .context("insert admin person failed")?;

    client
        .execute(
            "INSERT INTO auth_role (id, name, description, disable, deleted_at) \
             VALUES ($1, $2, $3, false, NULL) \
             ON CONFLICT (id) DO NOTHING",
            &[&"role-it-admin", &"admin", &"Integration test admin role"],
        )
        .await
        .context("insert admin role failed")?;

    client
        .execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, $2, $3) \
             ON CONFLICT (person_id, role_id, unit_id) DO NOTHING",
            &[&"person-it-admin", &"role-it-admin", &"unit-it"],
        )
        .await
        .context("insert person_role failed")?;

    client
        .execute(
            "INSERT INTO auth_unit (id, name, level) VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING",
            &[&"unit-it", &"IT", &1i32],
        )
        .await
        .context("insert unit failed")?;

    let token = "it-token-integration-test-001".to_string();

    let session = session_manager
        .create_session("it-admin".to_string(), token.clone())
        .await?;

    let signed_token = if let Ok(secret) = std::env::var("SESSION_HMAC_SECRET") {
        let mut mac =
            hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC init");
        mac.update(session.token.as_bytes());
        let sig = mac.finalize().into_bytes();
        format!(
            "{}.{}",
            session.token,
            base64::engine::general_purpose::URL_SAFE.encode(sig)
        )
    } else {
        session.token.clone()
    };

    let _ = client
        .execute(
            "INSERT INTO auth_session (token, person_id, expires_at, created_at) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT (token) DO UPDATE SET expires_at = $3",
            &[
                &signed_token,
                &session.person_unique,
                &session.expires_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                &session.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            ],
        )
        .await;

    session_manager
        .sessions
        .write()
        .await
        .insert(token.clone(), session);

    Ok(token)
}
