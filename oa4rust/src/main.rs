use axum::Router;
use shared::db::create_pool;
use tracing_subscriber::EnvFilter;

mod shared;
mod auth;
mod personal;
mod cms_control;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("o2server=debug".parse()?))
        .init();

    dotenvy::dotenv().ok();

    let pool = create_pool().await?;

    let app = Router::new()
        .merge(shared::router::router())
        .merge(auth::router::router(pool.clone()))
        .merge(personal::router::router(pool.clone()))
        .merge(cms_control::cms_control_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
