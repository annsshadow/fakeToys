use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    dict_create, dict_delete, dict_get, dict_item_create, dict_item_delete, dict_item_get,
    dict_item_list, dict_list, dict_update, file_create, file_delete, file_download, file_get,
    file_list, file_update, general_core_entity_router, invoice_create, invoice_delete,
    invoice_get, invoice_list, invoice_update,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::general_core_entity_router(pool)
}

