use super::*;
use axum::{extract::Extension, Json};
use deadpool_postgres::{Manager, Pool, tokio_postgres::NoTls};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[test]
fn test_correlation_router_builds() {
    let pool = Pool::builder(Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        NoTls,
    ))
    .build()
    .unwrap();

    let _ = correlation_router(pool);
}

#[test]
fn test_list_cms_correlations_returns_error_without_db() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = Pool::builder(Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            NoTls,
        ))
        .build()
        .unwrap();

        let result: Result<Json<ActionResult<Value>>, AppError> =
            routes::list_cms_correlations(Extension(pool)).await;

        match result {
            Ok(_) => panic!("expected error without DB"),
            Err(AppError::Internal) => {}
            Err(_) => panic!("expected Internal error"),
        }
    });
}

#[test]
fn test_list_process_platform_correlations_returns_error_without_db() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = Pool::builder(Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            NoTls,
        ))
        .build()
        .unwrap();

        let result: Result<Json<ActionResult<Value>>, AppError> =
            routes::list_process_platform_correlations(Extension(pool)).await;

        match result {
            Ok(_) => panic!("expected error without DB"),
            Err(AppError::Internal) => {}
            Err(_) => panic!("expected Internal error"),
        }
    });
}

#[test]
fn test_check_cms_readable_returns_error_without_db() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = Pool::builder(Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            NoTls,
        ))
        .build()
        .unwrap();

        let result: Result<Json<ActionResult<Value>>, AppError> =
            routes::check_cms_readable(Extension(pool)).await;

        match result {
            Ok(_) => panic!("expected error without DB"),
            Err(AppError::Internal) => {}
            Err(_) => panic!("expected Internal error"),
        }
    });
}
