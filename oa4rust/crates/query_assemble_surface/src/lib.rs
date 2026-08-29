use axum::{
    extract::{Extension, Path},
    Json, Router, routing::get, routing::post, routing::put, routing::delete,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;
pub mod u2_closures;

#[derive(Debug, Deserialize)]
pub struct CreateSurfaceRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub query: Option<String>,
    pub template: Option<String>,
}

pub async fn get_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, content, creator, create_time, update_time \
             FROM x_query_surface WHERE id = $1",
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
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("query surface not found"))),
    }
}

pub async fn create_surface(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let content = req.query.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_query_surface (id, name, category, content, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &category, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("category".to_string(), Value::String(category)),
        ("content".to_string(), Value::String(content)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn list_surfaces(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, content, creator, create_time, update_time \
             FROM x_query_surface WHERE category = $1 ORDER BY update_time DESC",
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
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn save_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let content = req.query.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_surface SET name = $1, category = $2, content = $3, update_time = NOW() \
             WHERE id = $4",
            &[&name, &category, &content, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query surface not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Number(serde_json::Number::from(result as i64))),
            ("name".to_string(), Value::String(name)),
            ("category".to_string(), Value::String(category)),
            ("content".to_string(), Value::String(content)),
        ]),
    ))))
}

pub async fn delete_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_query_surface WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query surface not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn preview_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("preview_url".to_string(), Value::String(format!("/preview/query/{}", id))),
            ("html".to_string(), Value::String("<div>Query Preview</div>".to_string())),
        ]),
    ))))
}

pub fn query_assemble_surface_router() -> Router {
    use u2_closures as u2;
    Router::new()
        .route("/jaxrs/query/assemble/surface/get/{id}", get(get_surface))
        .route("/jaxrs/query/assemble/surface/create", post(create_surface))
        .route("/jaxrs/query/assemble/surface/list/{category}", get(list_surfaces))
        .route("/jaxrs/query/assemble/surface/save/{id}", post(save_surface))
        .route("/jaxrs/query/assemble/surface/delete/{id}", post(delete_surface))
        .route("/jaxrs/query/assemble/surface/preview/{id}", get(preview_surface))
        .route("/jaxrs/queryview/{view}/application/{app}/execute", get(view_flag_flag_query_queryFlag_execute))
        .route("/jaxrs/queryview/{view}/application/{app}/execute/page/{page}/size/{size}", get(view_flag_flag_query_queryFlag_execute_v2_page_page_size_size))
        .route("/jaxrs/importmodel/id/{id}/execute", post(importmodel_id_execute))
        .route("/jaxrs/queryview/importmodel/execute/record/{recordId}", post(crate::importmodel_execute_record_recordId))
        .route("/jaxrs/queryview/importmodel/{flag}/{flag}/{query}/{queryFlag}", post(crate::importmodel_flag_flag_query_queryFlag))
        .route("/jaxrs/queryview/importmodel/{id}", post(crate::importmodel_id))
        .route("/jaxrs/queryview/importmodel/list/{query}/{queryFlag}", post(crate::importmodel_list_query_queryFlag))
        .route("/jaxrs/queryview/importmodel/list/record/item/paging/{page}/{size}/{size}", post(crate::importmodel_list_record_item_paging_page_size_size))
        .route("/jaxrs/queryview/importmodel/list/record/paging/{page}/{size}/{size}", post(crate::importmodel_list_record_paging_page_size_size))
        .route("/jaxrs/queryview/importmodel/record/{recordId}", post(crate::importmodel_record_recordId))
        .route("/jaxrs/queryview/importmodel/record/mockdeletetoget/{recordId}", post(crate::importmodel_record_recordId_mockdeletetoget))
        .route("/jaxrs/queryview/importmodel/record/{recordId}/{status}", post(crate::importmodel_record_recordId_status))
        .route("/jaxrs/queryview/neural/list/calculate/model/{modelFlag}/{work}/{workId}", get(crate::neural_list_calculate_model_modelFlag_work_workId))
        .route("/jaxrs/queryview/{query}/{flag}", get(crate::query_flag))
        .route("/jaxrs/queryview/list/{query}/{key}/{key}", get(crate::query_list_key_key))
        .route("/jaxrs/queryview/table/{flag}", get(crate::table_flag))
        .route("/jaxrs/queryview/table/list/{id}/{next}/{count}", get(crate::table_list_id_next_count))
        .route("/jaxrs/queryview/table/list/paging/{page}/{size}/{size}", get(crate::table_list_paging_page_size_size))
        .route("/jaxrs/queryview/table/list/row/{tableFlag}/{id}/{next}/{count}", get(crate::table_list_tableFlag_row_id_next_count))
        .route("/jaxrs/queryview/table/list/row/select/{tableFlag}", get(crate::table_list_tableFlag_row_select))
        .route("/jaxrs/queryview/table/list/row/select/where/where/{tableFlag}", get(crate::table_list_tableFlag_row_select_where_where))
        .route("/jaxrs/queryview/table/list/table/row/paging/{tableFlag}/{page}/{size}/{size}", get(crate::table_list_table_tableFlag_row_paging_page_size_size))
        .route("/jaxrs/queryview/table/row/{tableFlag}", get(crate::table_tableFlag_row))
        .route("/jaxrs/queryview/table/row/where/where/{tableFlag}/{count}", get(crate::table_tableFlag_row_count_where_where))
        .route("/jaxrs/queryview/table/row/delete/all/{tableFlag}", post(crate::table_tableFlag_row_delete_all))
        .route("/jaxrs/queryview/table/row/delete/all/mockdeletetoget/{tableFlag}", post(crate::table_tableFlag_row_delete_all_mockdeletetoget))
        .route("/jaxrs/queryview/table/row/{tableFlag}/{id}", get(crate::table_tableFlag_row_id))
        .route("/jaxrs/queryview/table/row/mockdeletetoget/{tableFlag}/{id}", post(crate::table_tableFlag_row_id_mockdeletetoget))
        .route("/jaxrs/queryview/table/row/mockputtopost/{tableFlag}/{id}", post(crate::table_tableFlag_row_id_mockputtopost))
        .route("/jaxrs/queryview/table/row/part/update/{tableFlag}/{id}", post(crate::table_tableFlag_row_id_part_update))
        .route("/jaxrs/queryview/table/row/one/{tableFlag}", get(crate::table_tableFlag_row_one))
        .route("/jaxrs/queryview/excel/result/{view}/{flag}", get(crate::view_excel_result_flag))
        .route("/jaxrs/queryview/{view}/{flag}/{flag}/{query}/{queryFlag}", get(crate::view_flag_flag_query_queryFlag))
        .route("/jaxrs/queryview/bundle/{view}/{flag}/{flag}/{query}/{queryFlag}", get(crate::view_flag_flag_query_queryFlag_bundle))
        .route("/jaxrs/queryview/bundle/mockputtopost/{view}/{flag}/{flag}/{query}/{queryFlag}", post(crate::view_flag_flag_query_queryFlag_bundle_mockputtopost))
        .route("/jaxrs/queryview/excel/{view}/{flag}/{flag}/{query}/{queryFlag}", get(crate::view_flag_flag_query_queryFlag_excel))
        .route("/jaxrs/queryview/excel/mockputtopost/{view}/{flag}/{flag}/{query}/{queryFlag}", post(crate::view_flag_flag_query_queryFlag_excel_mockputtopost))
        .route("/jaxrs/queryview/execute/mockputtopost/{view}/{flag}/{flag}/{query}/{queryFlag}", post(crate::view_flag_flag_query_queryFlag_execute_mockputtopost))
        .route("/jaxrs/queryview/bundle/{view}/{id}", get(crate::view_id_bundle))
        .route("/jaxrs/queryview/bundle/mockputtopost/{view}/{id}", post(crate::view_id_bundle_mockputtopost))
        .route("/jaxrs/queryview/bundle/v2/{view}/{id}", get(crate::view_id_bundle_v2))
        .route("/jaxrs/queryview/excel/{view}/{id}", get(crate::view_id_excel))
        .route("/jaxrs/queryview/excel/mockputtopost/{view}/{id}", post(crate::view_id_excel_mockputtopost))
        .route("/jaxrs/queryview/execute/{view}/{id}", get(crate::view_id_execute))
        .route("/jaxrs/queryview/execute/mockputtopost/{view}/{id}", post(crate::view_id_execute_mockputtopost))
        .route("/jaxrs/queryview/execute/v2/{view}/{id}/{page}/{size}", get(crate::view_id_execute_v2_page_page_size_size))
        // ── plan002 U2：已实现未注册 handler 补挂 ──
        .route("/jaxrs/queryview/importmodel/uuid", get(importmodel_uuid))
        .route("/jaxrs/queryview/importmodel/record/delete/{recordId}", delete(u2::importmodel_record_delete))
        .route("/jaxrs/queryview/importmodel/execute/record/{recordId}", get(u2::importmodel_reexecute_record))
        .route("/jaxrs/queryview/list", get(query_list))
        .route("/jaxrs/queryview/table/list/{id}/prev/{count}", get(table_list_id_prev_count))
        .route("/jaxrs/queryview/table/list/row/{tableFlag}/{id}/prev/{count}", get(table_list_tableFlag_row_id_prev_count))
        .route("/jaxrs/queryview/table/reload/dynamic", get(table_reload_dynamic))
        .route("/jaxrs/queryview/view/{id}", get(view_id))
        .route("/jaxrs/queryview/view/list/query/{queryFlag}", get(view_list_query_queryFlag))
        // ── plan002 U2：statement / stat / search / morelikethis 缺口 ──
        .route("/jaxrs/queryview/statement/{id}/format", get(u2::statement_get_format))
        .route("/jaxrs/queryview/statement/{id}", get(u2::statement_get_id))
        .route("/jaxrs/queryview/statement/execute/{flag}/mode/{mode}/page/{page}/size/{size}", post(u2::statement_execute_mode_v2))
        .route("/jaxrs/queryview/statement/execute/{flag}/page/{page}/size/{size}", post(u2::statement_execute))
        .route("/jaxrs/queryview/statement/list/query/{queryFlag}", post(u2::statement_list_with_query))
        .route("/jaxrs/queryview/stat/flag/{flag}/query/{queryFlag}", get(u2::stat_get_with_query))
        .route("/jaxrs/queryview/stat/list/query/{queryFlag}", get(u2::stat_list_with_query))
        .route("/jaxrs/queryview/stat/{id}", get(u2::stat_get_id))
        .route("/jaxrs/queryview/stat/{id}/execute", put(u2::stat_execute))
        .route("/jaxrs/queryview/stat/{id}/execute/mockputtopost", post(u2::stat_execute))
        .route("/jaxrs/queryview/stat/execute/mockputtopost/{id}", post(u2::stat_execute))
        .route("/jaxrs/queryview/search", post(u2::search_post))
        .route("/jaxrs/queryview/morelikethis", post(u2::morelikethis_post))
        // ── plan002 U2：table 行级 + view bundle v2 缺口 ──
        .route("/jaxrs/queryview/table/row/delete/{tableFlag}/{id}", delete(u2::table_row_delete))
        .route("/jaxrs/queryview/table/row/insert/{tableFlag}", post(u2::table_row_insert))
        .route("/jaxrs/queryview/table/row/one/insert/{tableFlag}", post(u2::table_row_insert_one))
        .route("/jaxrs/queryview/bundle/v2/post/{id}", post(u2::view_bundle_v2_post))
        // ── plan002 U2 v9：Java 精确路径/动词闭合（权威清单 docs/audits/java-endpoint-inventory.json）──
        .route("/jaxrs/queryview/importmodel/flag/{flag}/query/{queryFlag}", get(crate::importmodel_flag_flag_query_queryFlag))
        .route("/jaxrs/queryview/importmodel/list/query/{queryFlag}", get(crate::importmodel_list_query_queryFlag))
        .route("/jaxrs/queryview/importmodel/list/record/item/paging/{page}/size/{size}", post(crate::importmodel_list_record_item_paging_page_size_size))
        .route("/jaxrs/queryview/importmodel/list/record/paging/{page}/size/{size}", post(crate::importmodel_list_record_paging_page_size_size))
        .route("/jaxrs/queryview/importmodel/record/{recordId}", get(crate::importmodel_record_recordId))
        .route("/jaxrs/queryview/importmodel/record/{recordId}/mockdeletetoget", get(crate::importmodel_record_recordId_mockdeletetoget))
        .route("/jaxrs/queryview/importmodel/record/{recordId}/status", get(crate::importmodel_record_recordId_status))
        .route("/jaxrs/queryview/importmodel/record/{recordId}", delete(u2::importmodel_record_delete))
        .route("/jaxrs/queryview/importmodel/{id}", get(crate::importmodel_id))
        .route("/jaxrs/queryview/importmodel/{id}/execute", post(crate::importmodel_id_execute))
        .route("/jaxrs/queryview/neural/list/calculate/model/{modelFlag}/work/{workId}", get(crate::neural_list_calculate_model_modelFlag_work_workId))
        .route("/jaxrs/queryview/query/list", get(query_list))
        .route("/jaxrs/queryview/query/list/key/{key}", get(crate::query_list_key_key))
        .route("/jaxrs/queryview/query/{flag}", get(crate::query_flag))
        .route("/jaxrs/queryview/stat/flag/{flag}/query/{queryFlag}/execute", put(u2::stat_execute_with_query_put))
        .route("/jaxrs/queryview/stat/flag/{flag}/query/{queryFlag}/execute/mockputtopost", post(u2::stat_execute_with_query_mock))
        .route("/jaxrs/queryview/statement/{flag}/execute/mode/{mode}/page/{page}/size/{size}", post(u2::statement_execute_mode_v2))
        .route("/jaxrs/queryview/statement/{flag}/execute/page/{page}/size/{size}", post(u2::statement_execute))
        .route("/jaxrs/queryview/table/list/paging/{page}/size/{size}", post(crate::table_list_paging_page_size_size))
        .route("/jaxrs/queryview/table/list/table/{tableFlag}/row/paging/{page}/size/{size}", post(crate::table_list_table_tableFlag_row_paging_page_size_size))
        .route("/jaxrs/queryview/table/list/{id}/next/{count}", get(crate::table_list_id_next_count))
        .route("/jaxrs/queryview/table/list/{id}/row/select", post(u2::table_row_select_post))
        .route("/jaxrs/queryview/table/list/{id}/row/select/where/{where}", get(crate::table_list_tableFlag_row_select_where_where))
        .route("/jaxrs/queryview/table/list/{id}/row/{rid}/next/{count}", get(crate::table_list_tableFlag_row_id_next_count))
        .route("/jaxrs/queryview/table/list/{id}/row/{rid}/prev/{count}", get(crate::table_list_tableFlag_row_id_prev_count))
        .route("/jaxrs/queryview/table/{flag}/row", post(u2::table_row_insert))
        .route("/jaxrs/queryview/table/{flag}/row/count/where/{where}", get(crate::table_tableFlag_row_count_where_where))
        .route("/jaxrs/queryview/table/{flag}/row/delete/all", delete(crate::table_tableFlag_row_delete_all))
        .route("/jaxrs/queryview/table/{flag}/row/delete/all/mockdeletetoget", get(crate::table_tableFlag_row_delete_all_mockdeletetoget))
        .route("/jaxrs/queryview/table/{flag}/row/one", post(u2::table_row_insert_one))
        .route("/jaxrs/queryview/table/{flag}/row/{rid}", get(crate::table_tableFlag_row_id).put(crate::table_tableFlag_row_id_mockputtopost).delete(u2::table_row_delete))
        .route("/jaxrs/queryview/table/{flag}/row/{rid}/mockdeletetoget", get(crate::table_tableFlag_row_id_mockdeletetoget))
        .route("/jaxrs/queryview/table/{flag}/row/{rid}/mockputtopost", post(crate::table_tableFlag_row_id_mockputtopost))
        .route("/jaxrs/queryview/table/{flag}/row/{rid}/part/update", post(crate::table_tableFlag_row_id_part_update))
        .route("/jaxrs/queryview/view/excel/result/{flag}", get(crate::view_excel_result_flag))
        .route("/jaxrs/queryview/view/flag/{flag}/query/{queryFlag}", get(crate::view_flag_flag_query_queryFlag))
        .route("/jaxrs/queryview/view/flag/{flag}/query/{queryFlag}/bundle", put(u2::view_flag_query_bundle_put))
        .route("/jaxrs/queryview/view/flag/{flag}/query/{queryFlag}/bundle/mockputtopost", post(u2::view_flag_query_bundle_mock))
        .route("/jaxrs/queryview/view/flag/{flag}/query/{queryFlag}/excel", put(u2::view_flag_query_excel_put))
        .route("/jaxrs/queryview/view/flag/{flag}/query/{queryFlag}/excel/mockputtopost", post(u2::view_flag_query_excel_mock))
        .route("/jaxrs/queryview/view/flag/{flag}/query/{queryFlag}/execute", put(u2::view_flag_query_execute_put))
        .route("/jaxrs/queryview/view/flag/{flag}/query/{queryFlag}/execute/mockputtopost", post(u2::view_flag_query_execute_mock))
        .route("/jaxrs/queryview/view/flag/{flag}/query/{queryFlag}/execute/v2/page/{page}/size/{size}", post(u2::view_execute_v2_flag_query))
        .route("/jaxrs/queryview/view/{id}/bundle", put(u2::view_id_bundle_put))
        .route("/jaxrs/queryview/view/{id}/bundle/mockputtopost", post(u2::view_id_bundle_mock))
        .route("/jaxrs/queryview/view/{id}/bundle/v2", post(u2::view_bundle_v2_post))
        .route("/jaxrs/queryview/view/{id}/excel", put(u2::view_id_excel_put))
        .route("/jaxrs/queryview/view/{id}/excel/mockputtopost", post(crate::view_id_excel_mockputtopost))
        .route("/jaxrs/queryview/view/{id}/execute", put(u2::view_id_execute_put))
        .route("/jaxrs/queryview/view/{id}/execute/mockputtopost", post(crate::view_id_execute_mockputtopost))
        .route("/jaxrs/queryview/view/{id}/execute/v2/page/{page}/size/{size}", post(u2::view_execute_v2_id))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_assemble_surface_router().layer(axum::extract::Extension(pool))
}



pub async fn importmodel_execute_record_recordId(
    pool: Extension<Pool>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, model_flag, data FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn importmodel_flag_flag_query_queryFlag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, model_flag, query_flag FROM x_query_import_model WHERE flag = $1 AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
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
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

pub async fn importmodel_list_query_queryFlag(
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn importmodel_list_record_item_paging_page_size_size(
    pool: Extension<Pool>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, model_flag, data, creator, create_time FROM x_query_import_model_record ORDER BY create_time DESC LIMIT $2::int OFFSET ($1 - 1) * $2",
            &[&page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn importmodel_list_record_paging_page_size_size(
    pool: Extension<Pool>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, model_flag, data, creator, create_time FROM x_query_import_model_record ORDER BY create_time DESC LIMIT $2::int OFFSET ($1 - 1) * $2",
            &[&page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn importmodel_record_recordId(
    pool: Extension<Pool>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, model_flag, data, creator, create_time FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn importmodel_record_recordId_mockdeletetoget(
    pool: Extension<Pool>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, model_flag, data FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn importmodel_record_recordId_status(
    pool: Extension<Pool>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, status FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("status".to_string(), Value::String(row.get("status"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn importmodel_uuid(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("uuid".to_string(), Value::String(id)),
        ]),
    ))))
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

pub async fn importmodel_id_execute(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, model_flag FROM x_query_import_model WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let record_id = uuid::Uuid::new_v4().to_string();
            let model_flag = row.get::<_, Option<String>>("model_flag").unwrap_or_default();
            let result = client
                .execute(
                    "INSERT INTO x_query_import_model_record (id, model_flag, import_model_id, create_time) VALUES ($1, $2, $3, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'))",
                    &[&record_id, &model_flag, &id],
                )
                .await
                .map_err(|_| AppError::Internal)?;

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("recordId".to_string(), Value::String(record_id)),
                    ("executed".to_string(), Value::Number(serde_json::Number::from(result as i64))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

pub async fn neural_list_calculate_model_modelFlag_work_workId(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, model_flag, work_id, result, creator, create_time FROM x_query_neural_calculate WHERE model_flag = $1 AND work_id = $2 ORDER BY create_time DESC",
            &[&model_flag, &work_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("workId".to_string(), Value::String(row.get("work_id"))),
                ("result".to_string(), Value::String(row.get("result"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn query_list(
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn query_list_key_key(
    pool: Extension<Pool>,
    Path(key): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_query_design WHERE name ILIKE $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&format!("%{}%", key)],
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

pub async fn table_list_paging_page_size_size(
    pool: Extension<Pool>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, table_flag, creator, create_time FROM x_query_table WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2::int OFFSET ($1 - 1) * $2",
            &[&page, &size],
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn table_list_table_tableFlag_row_paging_page_size_size(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 ORDER BY id DESC LIMIT $3::int OFFSET ($2 - 1) * $3",
            &[&table_flag, &page, &size],
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn table_list_id_next_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE id > $1 ORDER BY id ASC LIMIT $2::int",
            &[&id, &count],
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn table_list_id_prev_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE id < $1 ORDER BY id DESC LIMIT $2::int",
            &[&id, &count],
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn table_list_tableFlag_row_select(
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn table_list_tableFlag_row_select_where_where(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(_where): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            &format!("SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND data ILIKE $2 ORDER BY id DESC LIMIT 100"),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id > $2 ORDER BY id ASC LIMIT $3::int",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id < $2 ORDER BY id DESC LIMIT $3::int",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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
            ("success".to_string(), Value::Number(serde_json::Number::from(result as i64))),
            ("value".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

pub async fn table_tableFlag_row_delete_all_mockdeletetoget(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 ORDER BY id DESC",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn table_tableFlag_row_one(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 LIMIT 1",
            &[&table_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("row not found"))),
    }
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
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("row not found"))),
    }
}

pub async fn table_tableFlag_row_id_mockdeletetoget(
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
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("row not found"))),
    }
}

pub async fn table_tableFlag_row_id_mockputtopost(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table_data SET data = $1, update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE table_flag = $2 AND id = $3",
            &[&data_str, &table_flag, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("row not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("tableFlag".to_string(), Value::String(table_flag)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn table_tableFlag_row_id_part_update(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table_data \
             SET data = jsonb_set(COALESCE(data,'{}')::jsonb, ARRAY[$1], $2::jsonb, true), \
                 update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') \
             WHERE table_flag = $3 AND id = $4",
            &[&"/part", &data_str, &table_flag, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("row not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("tableFlag".to_string(), Value::String(table_flag)),
            ("saved".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn view_excel_result_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, view_flag, excel_data, creator, create_time FROM x_query_view_excel WHERE view_flag = $1 ORDER BY create_time DESC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("excelData".to_string(), Value::String(row.get("excel_data"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn view_flag_flag_query_queryFlag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, view_flag, query_flag, content, creator, to_char(create_time,'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
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

pub async fn view_flag_flag_query_queryFlag_bundle(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, view_flag, bundle_data FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
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

pub async fn view_flag_flag_query_queryFlag_bundle_mockputtopost(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let bundle_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_view SET bundle_data = $1, update_time = NOW() WHERE view_flag = $2 AND query_flag = $3",
            &[&bundle_str, &flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("viewFlag".to_string(), Value::String(flag)),
            ("queryFlag".to_string(), Value::String(query_flag)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn view_flag_flag_query_queryFlag_excel(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, view_flag, excel_data FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                    ("excelData".to_string(), Value::String(row.get("excel_data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn view_flag_flag_query_queryFlag_excel_mockputtopost(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let excel_str = body.get("excelData").and_then(|v| v.as_str()).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_view SET excel_data = $1, update_time = NOW() WHERE view_flag = $2 AND query_flag = $3",
            &[&excel_str, &flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("viewFlag".to_string(), Value::String(flag)),
            ("queryFlag".to_string(), Value::String(query_flag)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn view_flag_flag_query_queryFlag_execute(
    pool: Extension<Pool>,
    Path((view, app)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, view_flag, query_flag, content, creator, create_time::text, update_time::text FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 LIMIT 1",
            &[&view, &app],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content_str: Option<String> = row.get("content");
            let content = content_str
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(serde_json::Value::Null);
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                    ("viewFlag".to_string(), Value::String(view)),
                    ("queryFlag".to_string(), Value::String(app)),
                    ("content".to_string(), content),
                    ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                    ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
                    ("updateTime".to_string(), Value::String(row.get::<_, Option<String>>("update_time").unwrap_or_default())),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn view_flag_flag_query_queryFlag_execute_mockputtopost(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_view SET content = $1, update_time = NOW() WHERE view_flag = $2 AND query_flag = $3",
            &[&data_str, &flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("viewFlag".to_string(), Value::String(flag)),
            ("queryFlag".to_string(), Value::String(query_flag)),
            ("executed".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn view_flag_flag_query_queryFlag_execute_v2_page_page_size_size(
    pool: Extension<Pool>,
    Path((view, app, page, size)): Path<(String, String, i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, view_flag, query_flag, content, creator, create_time FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 ORDER BY create_time DESC LIMIT $4::int OFFSET ($3 - 1) * $4",
            &[&view, &app, &page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn view_list_query_queryFlag(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, view_flag, query_flag, creator, to_char(create_time,'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_query_view WHERE query_flag = $1 ORDER BY create_time DESC",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

pub async fn view_id_bundle_mockputtopost(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let bundle_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_view SET bundle_data = $1, update_time = NOW() WHERE id = $2",
            &[&bundle_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn view_id_bundle_v2(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, view_flag, bundle_data_v2 FROM x_query_view WHERE id = $1",
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
                    ("bundle".to_string(), Value::String(row.get("bundle_data_v2"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn view_id_excel(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, view_flag, excel_data FROM x_query_view WHERE id = $1",
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
                    ("excelData".to_string(), Value::String(row.get("excel_data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn view_id_excel_mockputtopost(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let excel_str = body.get("excelData").and_then(|v| v.as_str()).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_view SET excel_data = $1, update_time = NOW() WHERE id = $2",
            &[&excel_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn view_id_execute(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, view_flag, query_flag, content, creator, create_time, update_time FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content_str: Option<String> = row.get("content");
            let content = content_str
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(serde_json::Value::Null);
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                    ("viewFlag".to_string(), Value::String(row.get::<_, Option<String>>("view_flag").unwrap_or_default())),
                    ("queryFlag".to_string(), Value::String(row.get::<_, Option<String>>("query_flag").unwrap_or_default())),
                    ("content".to_string(), content),
                    ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                    ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
                    ("updateTime".to_string(), Value::String(row.get::<_, Option<String>>("update_time").unwrap_or_default())),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn view_id_execute_mockputtopost(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_view SET content = $1, update_time = NOW() WHERE id = $2",
            &[&data_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("executed".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn view_id_execute_v2_page_page_size_size(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, content, creator, create_time FROM x_query_view WHERE id = $1 ORDER BY create_time DESC LIMIT $3 OFFSET ($2 - 1) * $3",
            &[&id, &page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
