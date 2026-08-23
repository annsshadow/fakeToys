use axum::{
    extract::{Extension, Path},
    Json, Router, routing::get, routing::post, routing::put, routing::delete,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult, response::row_to_json};
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

pub mod routes;
pub mod u2_closures;

use u2_closures::{ensure_limit, validate_single_select};

#[derive(Debug, Deserialize)]
pub struct CreateDesignerRequest {
    pub name: Option<String>,
    pub query: Option<String>,
    pub category: Option<String>,
}

pub async fn get_designer(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, query_definition, creator, create_time, update_time \
             FROM x_query_design WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("query".to_string(), Value::String(row.get("query_definition"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("query design not found"))),
    }
}

pub async fn create_designer(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateDesignerRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let query_definition = req.query.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_query_design (id, name, category, query_definition, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &category, &query_definition, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("category".to_string(), Value::String(category)),
        ("query".to_string(), Value::String(query_definition)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn list_designers(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, create_time, update_time FROM x_query_design \
             WHERE category = $1 AND deleted_at IS NULL ORDER BY update_time DESC",
            &[&category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn save_designer(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<CreateDesignerRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let query_definition = req.query.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_design SET name = $1, category = $2, query_definition = $3, update_time = NOW() \
             WHERE id = $4 AND deleted_at IS NULL",
            &[&name, &category, &query_definition, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query design not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Number(serde_json::Number::from(result as i64))),
            ("name".to_string(), Value::String(name)),
            ("query".to_string(), Value::String(query_definition)),
        ]),
    ))))
}

pub async fn delete_designer(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_design SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query design not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub fn query_assemble_designer_router(pool: Option<Pool>) -> Router {
    use u2_closures as u2;
    let router = Router::new()
        .route("/jaxrs/query/assemble/designer/get/{id}", get(get_designer))
        .route("/jaxrs/query/assemble/designer/create", post(create_designer))
        .route("/jaxrs/query/assemble/designer/list/{category}", get(list_designers))
        .route("/jaxrs/query/assemble/designer/save/{id}", post(save_designer))
        .route("/jaxrs/query/assemble/designer/delete/{id}", post(delete_designer))
        .route("/jaxrs/query/assemble/designer/{id}/{count}", get(crate::id_count))
        .route("/jaxrs/query/assemble/designer/importmodel/{id}", post(crate::importmodel_id))
        .route("/jaxrs/query/assemble/designer/importmodel/permission/{id}", post(crate::importmodel_id_permission))
        .route("/jaxrs/query/assemble/designer/importmodel/list/{query}/{flag}", post(crate::importmodel_list_query_flag))
        .route("/jaxrs/query/assemble/designer/neural/generate/model/{modelFlag}", get(crate::neural_generate_model_modelFlag))
        .route("/jaxrs/query/assemble/designer/neural/learn/model/{modelFlag}", get(crate::neural_learn_model_modelFlag))
        .route("/jaxrs/query/assemble/designer/neural/model/{modelFlag}", get(crate::neural_model_modelFlag))
        .route("/jaxrs/query/assemble/designer/neural/model/reset/{modelFlag}/{status}", post(crate::neural_model_modelFlag_reset_status))
        .route("/jaxrs/query/assemble/designer/neural/stop/generating/model/{modelFlag}", get(crate::neural_stop_generating_model_modelFlag))
        .route("/jaxrs/query/assemble/designer/neural/stop/learn/model/{modelFlag}", get(crate::neural_stop_learn_model_modelFlag))
        .route("/jaxrs/query/assemble/designer/output/select/file/{flag}", get(crate::output_flag_select_file))
        .route("/jaxrs/query/assemble/designer/output/select/{queryFlag}", get(crate::output_queryFlag_select))
        .route("/jaxrs/query/assemble/designer/entity/entity/properties/{query}/{category}/{entityCategory}", get(crate::query_entity_entity_category_entityCategory_properties))
        .route("/jaxrs/query/assemble/designer/icon/{query}/{flag}", get(crate::query_flag_icon))
        .route("/jaxrs/query/assemble/designer/permission/{query}/{id}", get(crate::query_id_permission))
        .route("/jaxrs/query/assemble/designer/list/querycategory/{query}/{queryCategory}", get(crate::query_list_querycategory_queryCategory))
        .route("/jaxrs/query/assemble/designer/list/summary/querycategory/{query}/{queryCategory}", get(crate::query_list_summary_querycategory_queryCategory))
        .route("/jaxrs/query/assemble/designer/stat/{id}", get(crate::stat_id))
        .route("/jaxrs/query/assemble/designer/stat/permission/{id}", get(crate::stat_id_permission))
        .route("/jaxrs/query/assemble/designer/stat/simulate/{id}", get(crate::stat_id_simulate))
        .route("/jaxrs/query/assemble/designer/stat/list/{id}/{next}/{count}", get(crate::stat_list_id_next_count))
        .route("/jaxrs/query/assemble/designer/stat/list/{query}/{flag}", get(crate::stat_list_query_flag))
        .route("/jaxrs/query/assemble/designer/table/export/{tableFlag}/{count}/{count}", get(crate::table_export_tableFlag_count_count))
        .route("/jaxrs/query/assemble/designer/table/{flag}", get(crate::table_flag))
        .route("/jaxrs/query/assemble/designer/table/execute/{flag}", post(crate::table_flag_execute))
        .route("/jaxrs/query/assemble/designer/table/build/{flag}/{status}", get(crate::table_flag_status_build))
        .route("/jaxrs/query/assemble/designer/table/draft/{flag}/{status}", get(crate::table_flag_status_draft))
        .route("/jaxrs/query/assemble/designer/table/permission/{id}", get(crate::table_id_permission))
        .route("/jaxrs/query/assemble/designer/table/list/{query}/{flag}", get(crate::table_list_query_flag))
        .route("/jaxrs/query/assemble/designer/table/list/row/{tableFlag}/{id}/{next}/{count}", get(crate::table_list_tableFlag_row_id_next_count))
        .route("/jaxrs/query/assemble/designer/table/list/row/select/where/where/{tableFlag}", get(crate::table_list_tableFlag_row_select_where_where))
        .route("/jaxrs/query/assemble/designer/table/build/dispatch/{query}", get(crate::table_query_build_dispatch))
        .route("/jaxrs/query/assemble/designer/table/row/{tableFlag}", get(crate::table_tableFlag_row))
        .route("/jaxrs/query/assemble/designer/table/row/where/where/{tableFlag}/{count}", get(crate::table_tableFlag_row_count_where_where))
        .route("/jaxrs/query/assemble/designer/table/row/delete/all/{tableFlag}", post(crate::table_tableFlag_row_delete_all))
        .route("/jaxrs/query/assemble/designer/table/row/{tableFlag}/{id}", get(crate::table_tableFlag_row_id))
        .route("/jaxrs/query/assemble/designer/table/row/save/{tableFlag}", post(crate::table_tableFlag_row_save))
        .route("/jaxrs/query/assemble/designer/bundle/{view}/{id}", get(crate::view_id_bundle))
        .route("/jaxrs/query/assemble/designer/simulate/{view}/{id}", get(crate::view_id_simulate))
        .route("/jaxrs/query/assemble/designer/list/{view}/{id}/{next}/{count}", get(crate::view_list_id_next_count))
        .route("/jaxrs/query/assemble/designer/list/{view}/{query}/{flag}", get(crate::view_list_query_flag))
        .route("/jaxrs/query/assemble/designer/delete/{id}", delete(delete_designer))
        .route("/jaxrs/query/assemble/designer/save/{id}", put(save_designer))
        .route("/jaxrs/query/assemble/designer/table/row/delete/all/{tableFlag}", delete(table_tableFlag_row_delete_all))
        .route("/jaxrs/query/assemble/designer/table/row/save/{tableFlag}", put(table_tableFlag_row_save))
        // ── plan002 U2：已实现未注册 handler 补挂 ──
        .route("/jaxrs/query/assemble/designer/search", post(designer_search))
        .route("/jaxrs/query/assemble/designer/input/compare", put(input_compare))
        .route("/jaxrs/query/assemble/designer/input/cover", put(input_cover))
        .route("/jaxrs/query/assemble/designer/input/create", put(input_create))
        .route("/jaxrs/query/assemble/designer/input/prepare/cover", put(input_prepare_cover))
        .route("/jaxrs/query/assemble/designer/input/prepare/create", put(input_prepare_create))
        .route("/jaxrs/query/assemble/designer/neural/list/model", get(neural_list_model))
        .route("/jaxrs/query/assemble/designer/neural/model", post(neural_model))
        .route("/jaxrs/query/assemble/designer/output/list", get(output_list))
        .route("/jaxrs/query/assemble/designer/query/{flag}", get(query_flag))
        .route("/jaxrs/query/assemble/designer/list/all", get(query_list_all))
        .route("/jaxrs/query/assemble/designer/list/summary", get(query_list_summary))
        .route("/jaxrs/query/assemble/designer/querycategory/list", get(query_querycategory_list))
        .route("/jaxrs/query/assemble/designer/stat/list/{id}/prev/{count}", get(stat_list_id_prev_count))
        .route("/jaxrs/query/assemble/designer/table/list/manage", get(table_list_manage))
        .route("/jaxrs/query/assemble/designer/table/reload/dynamic", get(table_reload_dynamic))
        .route("/jaxrs/query/assemble/designer/table/list/row/{tableFlag}/{id}/prev/{count}", get(table_list_tableFlag_row_id_prev_count))
        .route("/jaxrs/query/assemble/designer/view/{id}", get(view_id))
        .route("/jaxrs/query/assemble/designer/view/permission/{id}", get(view_id_permission))
        .route("/jaxrs/query/assemble/designer/view/list/{id}/prev/{count}", get(view_list_id_prev_count))
        // ── plan002 U2：statement 全族（CRUD + 执行）──
        .route("/jaxrs/query/assemble/designer/statement", post(u2::statement_create))
        .route("/jaxrs/query/assemble/designer/statement/{flag}", get(u2::statement_get_flag).put(u2::statement_edit).delete(u2::statement_delete))
        .route("/jaxrs/query/assemble/designer/statement/list/manage", get(u2::statement_manage_list))
        .route("/jaxrs/query/assemble/designer/statement/list/query/{queryFlag}", post(u2::statement_list_with_query))
        .route("/jaxrs/query/assemble/designer/statement/permission/{id}", post(u2::statement_permission))
        .route("/jaxrs/query/assemble/designer/statement/execute/{flag}/page/{page}/size/{size}", post(u2::statement_execute_v2))
        .route("/jaxrs/query/assemble/designer/statement/execute/{flag}/mode/{mode}/page/{page}/size/{size}", post(u2::statement_execute_mode_v2))
        // ── plan002 U2：importmodel / neural / stat / table / view CRUD 缺口 ──
        .route("/jaxrs/query/assemble/designer/importmodel", post(u2::importmodel_create))
        .route("/jaxrs/query/assemble/designer/importmodel/edit/{id}", put(u2::importmodel_edit))
        .route("/jaxrs/query/assemble/designer/importmodel/delete/{id}", delete(u2::importmodel_delete))
        .route("/jaxrs/query/assemble/designer/neural/delete/model/{modelFlag}", delete(u2::neural_delete_model_modelFlag))
        .route("/jaxrs/query/assemble/designer/neural/update/model/{modelFlag}", put(u2::neural_update_model_modelFlag))
        .route("/jaxrs/query/assemble/designer/stat", post(u2::stat_create))
        .route("/jaxrs/query/assemble/designer/stat/edit/{id}", put(u2::stat_edit))
        .route("/jaxrs/query/assemble/designer/stat/delete/{id}", delete(u2::stat_delete))
        .route("/jaxrs/query/assemble/designer/table", post(u2::table_create))
        .route("/jaxrs/query/assemble/designer/table/edit/{flag}", put(u2::table_edit))
        .route("/jaxrs/query/assemble/designer/table/delete/{flag}", delete(u2::table_delete))
        .route("/jaxrs/query/assemble/designer/table/row/insert/{tableFlag}", post(u2::table_tableFlag_row_insert))
        .route("/jaxrs/query/assemble/designer/table/row/update/{tableFlag}/{id}", put(u2::table_tableFlag_row_update))
        .route("/jaxrs/query/assemble/designer/table/row/delete/{tableFlag}/{id}", delete(u2::table_tableFlag_row_delete))
        .route("/jaxrs/query/assemble/designer/table/build/query/{query}", get(table_query_query_build))
        .route("/jaxrs/query/assemble/designer/view", post(u2::view_create))
        .route("/jaxrs/query/assemble/designer/view/edit/{id}", put(u2::view_edit))
        .route("/jaxrs/query/assemble/designer/view/delete/{id}", delete(u2::view_delete))
        .route("/jaxrs/query/assemble/designer/icon/set/{flag}", put(u2::query_set_icon));

    if let Some(pool) = pool {
        router.layer(Extension(pool))
    } else {
        router
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_assemble_designer_router(Some(pool))
}



pub async fn designer_search(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, create_time, update_time FROM x_query_design WHERE deleted_at IS NULL ORDER BY update_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn id_count(
    pool: Extension<Pool>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one("SELECT COUNT(*) as cnt FROM x_query_design WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let total: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("total".to_string(), Value::Number(serde_json::Number::from(total))),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn importmodel_list_query_flag(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, model_flag, query_flag, creator, create_time FROM x_query_import_model WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn importmodel_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, model_flag, query_flag, content, creator, create_time FROM x_query_import_model WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

pub async fn importmodel_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_import_model WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("permission".to_string(), Value::String(row.get("permission"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

pub async fn input_compare(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();

    let row = client
        .query_opt(
            "SELECT id, content FROM x_query_input WHERE id = $1",
            &[&input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let old_content: Option<String> = row.get("content");
            let new_content = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();
            let old_str = old_content.unwrap_or_default();
            let compared = !old_str.is_empty() && old_str == new_content;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(input_id.to_string())),
                    ("oldContent".to_string(), Value::String(old_str)),
                    ("newContent".to_string(), Value::String(new_content.to_string())),
                    ("compared".to_string(), Value::Bool(compared)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("input not found"))),
    }
}

pub async fn input_cover(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let content_str = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_input SET content = $1, update_time = NOW() WHERE id = $2",
            &[&content_str, &input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("input not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(input_id.to_string())),
            ("covered".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn input_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = "system";

    let result = client
        .execute(
            "INSERT INTO x_query_input (id, content, creator, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn input_prepare_cover(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let row = client
        .query_opt(
            "SELECT id, content FROM x_query_input WHERE id = $1",
            &[&input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(input_id.to_string())),
                    ("content".to_string(), content.map(Value::String).unwrap_or(Value::String("".to_string()))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("input not found"))),
    }
}

pub async fn input_prepare_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = "system";

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "INSERT INTO x_query_input (id, content, creator, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn neural_generate_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'generating', update_time = NOW() WHERE flag = $1",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            ("generating".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn neural_learn_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'learning', update_time = NOW() WHERE flag = $1",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            ("learning".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn neural_list_model(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, status, creator, create_time FROM x_query_neural_model WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn neural_model(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    let flag = body.get("flag").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = "system";

    let id = uuid::Uuid::new_v4().to_string();
    let result = client
        .execute(
            "INSERT INTO x_query_neural_model (id, name, flag, status, creator, create_time) VALUES ($1, $2, $3, 'idle', $4, NOW())",
            &[&id, &name, &flag, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name.to_string())),
            ("flag".to_string(), Value::String(flag.to_string())),
            ("created".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn neural_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, status, creator, create_time FROM x_query_neural_model WHERE flag = $1 AND deleted_at IS NULL",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("neural model not found"))),
    }
}

pub async fn neural_model_modelFlag_reset_status(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'idle', update_time = NOW() WHERE flag = $1",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            ("reset".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn neural_stop_generating_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'idle', update_time = NOW() WHERE flag = $1 AND status = 'generating'",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found or not generating")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            ("stopped".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn neural_stop_learn_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'idle', update_time = NOW() WHERE flag = $1 AND status = 'learning'",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found or not learning")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            ("stopped".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn output_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_query_output WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn output_flag_select_file(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, select_file FROM x_query_output WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("selectFile".to_string(), Value::String(row.get("select_file"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("output not found"))),
    }
}

pub async fn output_queryFlag_select(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_query_output WHERE query_flag = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn query_entity_entity_category_entityCategory_properties(
    pool: Extension<Pool>,
    Path(entity): Path<String>,
    Path(entity_category): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT field_name, field_label, field_type FROM x_query_entity_property WHERE entity = $1 AND category = $2 ORDER BY sort_order",
            &[&entity, &entity_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("fieldName".to_string(), Value::String(row.get("field_name"))),
                ("fieldLabel".to_string(), Value::String(row.get("field_label"))),
                ("fieldType".to_string(), Value::String(row.get("field_type"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("properties".to_string(), Value::Array(data)),
    ])))))
}

pub async fn query_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_query_design WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn query_list_querycategory_queryCategory(
    pool: Extension<Pool>,
    Path(query_category): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_query_design WHERE category = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&query_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn query_list_summary(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_query_design WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn query_list_summary_querycategory_queryCategory(
    pool: Extension<Pool>,
    Path(query_category): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category FROM x_query_design WHERE category = $1 AND deleted_at IS NULL ORDER BY name",
            &[&query_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn query_querycategory_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT DISTINCT category FROM x_query_design WHERE deleted_at IS NULL AND category IS NOT NULL ORDER BY category",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("category".to_string(), Value::String(row.get("category"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn query_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, query_definition, creator, create_time FROM x_query_design WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("query".to_string(), Value::String(row.get("query_definition"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("query not found"))),
    }
}

pub async fn query_flag_icon(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, icon FROM x_query_design WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("icon".to_string(), Value::String(row.get("icon"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("query not found"))),
    }
}

pub async fn query_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_design WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("permission".to_string(), Value::String(row.get("permission"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("query not found"))),
    }
}

pub async fn stat_list_query_flag(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, query_flag, stat_type, creator, create_time FROM x_query_stat WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("statType".to_string(), Value::String(row.get("stat_type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stat_list_id_next_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, query_flag, stat_type, creator, create_time FROM x_query_stat WHERE id > $1 ORDER BY id ASC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("statType".to_string(), Value::String(row.get("stat_type"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stat_list_id_prev_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, query_flag, stat_type, creator, create_time FROM x_query_stat WHERE id < $1 ORDER BY id DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("statType".to_string(), Value::String(row.get("stat_type"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stat_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, query_flag, stat_type, config, creator, create_time FROM x_query_stat WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("statType".to_string(), Value::String(row.get("stat_type"))),
                ("config".to_string(), Value::String(row.get("config"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("stat not found"))),
    }
}

pub async fn stat_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_stat WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("permission".to_string(), Value::String(row.get("permission"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("stat not found"))),
    }
}

pub async fn stat_id_simulate(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, config FROM x_query_stat WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("config".to_string(), Value::String(row.get("config"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("stat not found"))),
    }
}

pub async fn table_export_tableFlag_count_count(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 LIMIT $2::bigint",
            &[&table_flag, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn table_list_manage(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, table_flag, creator, create_time FROM x_query_table WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn table_list_query_flag(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, table_flag, query_flag, creator, create_time FROM x_query_table WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn table_list_tableFlag_row_select_where_where(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(_where): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            &format!("SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND data ILIKE $2 ORDER BY id DESC"),
            &[&table_flag, &format!("%{}%", _where)],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn table_list_tableFlag_row_id_next_count(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id > $2 ORDER BY id ASC LIMIT $3::bigint",
            &[&table_flag, &id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn table_list_tableFlag_row_id_prev_count(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id < $2 ORDER BY id DESC LIMIT $3::bigint",
            &[&table_flag, &id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn table_query_query_build(
    pool: Extension<Pool>,
    Path(query): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET status = 'build', reloaded = FALSE, update_time = NOW() WHERE query_flag = $1 AND deleted_at IS NULL",
            &[&query],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("queryFlag".to_string(), Value::String(query)),
            ("built".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn table_reload_dynamic(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET reloaded = true, update_time = NOW() WHERE reloaded = false",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("reloaded".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn table_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, table_flag, creator, create_time FROM x_query_table WHERE table_flag = $1 LIMIT 1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("table not found"))),
    }
}

pub async fn table_flag_execute(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let sql = body.get("sql").and_then(|v| v.as_str()).unwrap_or_default();

    validate_single_select(sql).map_err(AppError::BadRequest)?;

    let limited_sql = ensure_limit(sql, 500);
    let rows = client
        .query(&limited_sql, &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(row_to_json).collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn table_flag_status_build(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET status = 'build', update_time = NOW() WHERE table_flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("table not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(flag)),
            ("status".to_string(), Value::String("build".to_string())),
        ]),
    ))))
}

pub async fn table_flag_status_draft(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET status = 'draft', update_time = NOW() WHERE table_flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("table not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(flag)),
            ("status".to_string(), Value::String("draft".to_string())),
        ]),
    ))))
}

pub async fn table_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_table WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("permission".to_string(), Value::String(row.get("permission"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("table not found"))),
    }
}

pub async fn table_query_build_dispatch(
    pool: Extension<Pool>,
    Path(query): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET status = 'build', update_time = NOW() WHERE query_flag = $1 AND deleted_at IS NULL",
            &[&query],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("queryFlag".to_string(), Value::String(query)),
            ("built".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn table_tableFlag_row(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 ORDER BY id DESC LIMIT 100",
            &[&table_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn table_tableFlag_row_count_where_where(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(_where): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            &format!("SELECT COUNT(*) as cnt FROM x_query_table_data WHERE table_flag = $1 AND data ILIKE $2"),
            &[&table_flag, &format!("%{}%", _where)],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(table_flag)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn table_tableFlag_row_delete_all(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_query_table_data WHERE table_flag = $1",
            &[&table_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(table_flag)),
            ("deleted".to_string(), Value::Number(serde_json::Number::from(result as i64))),
            ("count".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn table_tableFlag_row_save(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "INSERT INTO x_query_table_data (id, table_flag, data, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &table_flag, &data_str],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("tableFlag".to_string(), Value::String(table_flag)),
            ("saved".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn table_tableFlag_row_id(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id = $2",
            &[&table_flag, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("row not found"))),
    }
}

pub async fn view_list_query_flag(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, view_flag, query_flag, creator, create_time FROM x_query_view WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn view_list_id_next_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, view_flag, creator, create_time FROM x_query_view WHERE id > $1 ORDER BY id ASC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn view_list_id_prev_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, view_flag, creator, create_time FROM x_query_view WHERE id < $1 ORDER BY id DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn view_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, view_flag, query_flag, content, creator, create_time FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn view_id_bundle(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, view_flag, bundle_data FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                    ("bundle".to_string(), Value::String(row.get("bundle_data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn view_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("permission".to_string(), Value::String(row.get("permission"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn view_id_simulate(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("content"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}



