use anyhow::Context as _;
use axum::Router;
use shared::db::create_pool;
use shared::rate_limit::RateLimiter;
use shared::session::SessionManager;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("o2server=debug".parse()?))
        .init();

    dotenvy::dotenv().ok();

    let pool = create_pool().await.context("failed to create database pool")?;

    let session_manager = SessionManager::new();
    let rate_limiter = RateLimiter::new();

    let app = Router::new()
        .merge(shared::router::router(session_manager.clone(), rate_limiter.clone()))
        .merge(auth::router(pool.clone(), rate_limiter.clone(), session_manager.clone()))
        .merge(personal::router(pool.clone()))
        .merge(cms_control::cms_control_router())
        .merge(control::control_router(pool.clone()))
        .merge(personal_extend::personal_extend_router(pool.clone(), session_manager));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}