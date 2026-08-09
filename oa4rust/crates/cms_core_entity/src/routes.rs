use crate::{
    article_create, article_get, article_list, category_create, category_get, category_list,
    cms_core_entity_router,
};

pub async fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_core_entity_router(pool).await
}
