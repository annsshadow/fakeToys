use axum::{
    extract::Query,
    Json, Router, routing::get, routing::post,
};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct ExpressQuery {
    pub code: Option<String>,
    pub company: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExpressSubscribeRequest {
    pub code: Option<String>,
    pub company: Option<String>,
    pub callback: Option<String>,
}

pub async fn get_express_info(
    Query(params): Query<ExpressQuery>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = params.code.unwrap_or_default();
    let company = params.company.unwrap_or_default();

    let data = Value::Object(serde_json::Map::from_iter([
        ("code".to_string(), Value::String(code)),
        ("company".to_string(), Value::String(company)),
        ("status".to_string(), Value::String("delivered".to_string())),
        ("traces".to_string(), Value::Array(vec![
            Value::Object(serde_json::Map::from_iter([
                ("time".to_string(), Value::String("2024-01-01T10:00:00Z".to_string())),
                ("desc".to_string(), Value::String("Package delivered".to_string())),
            ])),
        ])),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn list_express_companies() -> Result<Json<ActionResult<Value>>, AppError> {
    let companies = vec![
        Value::Object(serde_json::Map::from_iter([
            ("code".to_string(), Value::String("SF".to_string())),
            ("name".to_string(), Value::String("顺丰速运".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("code".to_string(), Value::String("STO".to_string())),
            ("name".to_string(), Value::String("申通快递".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("code".to_string(), Value::String("YTO".to_string())),
            ("name".to_string(), Value::String("圆通速递".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(companies.len() as i64))),
            ("data".to_string(), Value::Array(companies)),
        ]),
    ))))
}

pub async fn subscribe_express(
    Json(payload): Json<ExpressSubscribeRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = payload.code.unwrap_or_default();
    let company = payload.company.unwrap_or_default();
    let callback = payload.callback.unwrap_or_default();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("code".to_string(), Value::String(code)),
            ("company".to_string(), Value::String(company)),
            ("callback".to_string(), Value::String(callback)),
            ("subscribed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub fn express_router() -> Router {
    Router::new()
        .route("/jaxrs/express/query", get(get_express_info))
        .route("/jaxrs/express/companies", get(list_express_companies))
        .route("/jaxrs/express/subscribe", post(subscribe_express))
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/express/health", axum::routing::get(|| async { "TODO: express - real implementation needed" }))
}