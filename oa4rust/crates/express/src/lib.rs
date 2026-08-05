/// 快递查询模块
///
/// 对外暴露快递信息查询相关接口。
pub mod routes;

pub use routes::express_router;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/express/health", axum::routing::get(|| async { "TODO: express - real implementation needed" }))
}