use crate::{template_form_list, uuid_random, view_list_all, entities::cms_view};
use axum::{extract::{Extension, Json, Path}, routing::{get, post}, Router};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub async fn view_publish(
    db: Extension<DatabaseConnection>,
    Path(xid): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = cms_view::Entity::find_by_id(&xid)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let active = cms_view::ActiveModel {
                xid: Set(m.xid.clone()),
                xname: Set(m.xname.clone()),
                xapp_id: Set(m.xapp_id.clone()),
                deleted_at: Set(None),
            };
            active.update(&db.0).await.map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.xid)),
                ("name".to_string(), Value::String(m.xname)),
                ("appId".to_string(), Value::String(m.xapp_id)),
            ])))))
        }
        None => Ok(Json(ActionResult::error("cms view not found"))),
    }
}

pub async fn view_unpublish(
    db: Extension<DatabaseConnection>,
    Path(xid): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = cms_view::Entity::find_by_id(&xid)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let active = cms_view::ActiveModel {
                xid: Set(m.xid.clone()),
                xname: Set(m.xname.clone()),
                xapp_id: Set(m.xapp_id.clone()),
                deleted_at: Set(Some(chrono::Utc::now().naive_utc())),
            };
            active.update(&db.0).await.map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.xid)),
                ("name".to_string(), Value::String(m.xname)),
                ("appId".to_string(), Value::String(m.xapp_id)),
            ])))))
        }
        None => Ok(Json(ActionResult::error("cms view not found"))),
    }
}

pub fn cms_express_router() -> Router {
    Router::new()
        .route("/jaxrs/cms/uuid/random", get(uuid_random))
        .route("/jaxrs/cms/templateform/list", get(template_form_list))
        .route("/jaxrs/cms/view/list/all", get(view_list_all))
        .route("/jaxrs/cms/view/publish/{id}", post(view_publish))
        .route("/jaxrs/cms/view/unpublish/{id}", post(view_unpublish))
}

pub fn router(pool: Pool) -> axum::Router {
    cms_express_router().layer(axum::extract::Extension(pool))
}
