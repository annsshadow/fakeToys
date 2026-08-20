use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_control_config, list_control_sections, update_control_config,
    document_id_view_count, commend_list_paging, queryview_flag_definition, application_id,
    document_search,
    anonymous_document_id_view,
    data_document_id_array_data, data_document_id_mockdeletetoget, data_document_id_mockputtopost,
    data_document_id_path0, data_document_id_path0_mockdeletetoget, data_document_id_path0_mockputtopost,
    data_document_id_path0_path1, data_document_id_path0_path1_mockdeletetoget, data_document_id_path0_path1_mockputtopost,
    data_document_id_path0_path1_path2, data_document_id_path0_path1_path2_mockdeletetoget, data_document_id_path0_path1_path2_mockputtopost,
    data_document_id_path0_path1_path2_path3, data_document_id_path0_path1_path2_path3_mockdeletetoget, data_document_id_path0_path1_path2_path3_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4, data_document_id_path0_path1_path2_path3_path4_mockdeletetoget, data_document_id_path0_path1_path2_path3_path4_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4_path5, data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget, data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4_path5_path6, data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget, data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7, data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget, data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost,
    fileinfo_id, fileinfo_id_document_documentId, fileinfo_id_mockdeletetoget,
    anonymous_fileinfo_download_document_id, fileinfo_download_document_id,
    fileinfo_upload_document_docId,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/cms_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/cms_assemble_control/list/control/sections", get(list_control_sections))
        .route("/jaxrs/cms_assemble_control/update/control/config", get(update_control_config))
        .route("/jaxrs/document/{id}/view/count", post(document_id_view_count))
        .route("/jaxrs/commend/list/paging/{docId}", get(commend_list_paging))
        .route("/jaxrs/queryview/flag/{view_flag}/definition/{query_flag}", get(queryview_flag_definition))
        .route("/jaxrs/application/{id}", get(application_id))
        .route("/jaxrs/cms_assemble_control/document/search", get(document_search))
        .route("/jaxrs/anonymous/document/{id}/view", get(anonymous_document_id_view))
        .route("/jaxrs/data/document/{id}/array/data", get(data_document_id_array_data))
        .route("/jaxrs/data/document/{id}/mockdeletetoget", get(data_document_id_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/mockputtopost", post(data_document_id_mockputtopost))
        .route("/jaxrs/data/document/{id}/path0", get(data_document_id_path0))
        .route("/jaxrs/data/document/{id}/path0/mockdeletetoget", get(data_document_id_path0_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/path0/mockputtopost", post(data_document_id_path0_mockputtopost))
        .route("/jaxrs/data/document/{id}/path0/path1", get(data_document_id_path0_path1))
        .route("/jaxrs/data/document/{id}/path0/path1/mockdeletetoget", get(data_document_id_path0_path1_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/path0/path1/mockputtopost", post(data_document_id_path0_path1_mockputtopost))
        .route("/jaxrs/data/document/{id}/path0/path1/path2", get(data_document_id_path0_path1_path2))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/mockdeletetoget", get(data_document_id_path0_path1_path2_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/mockputtopost", post(data_document_id_path0_path1_path2_mockputtopost))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3", get(data_document_id_path0_path1_path2_path3))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/mockputtopost", post(data_document_id_path0_path1_path2_path3_mockputtopost))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4", get(data_document_id_path0_path1_path2_path3_path4))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_mockputtopost))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5", get(data_document_id_path0_path1_path2_path3_path4_path5))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6", get(data_document_id_path0_path1_path2_path3_path4_path5_path6))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/path7", get(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/path7/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost))
        .route("/jaxrs/anonymous/fileinfo/download/document/{id}", get(anonymous_fileinfo_download_document_id))
        .route("/jaxrs/fileinfo/download/document/{id}", get(fileinfo_download_document_id))
        .route("/jaxrs/fileinfo/upload/document/{id}", post(fileinfo_upload_document_docId))
        .route("/jaxrs/fileinfo/{id}", get(fileinfo_id))
        .route("/jaxrs/fileinfo/{id}/document/{docId}", get(fileinfo_id_document_documentId))
        .route("/jaxrs/fileinfo/{id}/mockdeletetoget", get(fileinfo_id_mockdeletetoget))
        .layer(Extension(pool))
}

pub fn cms_assemble_control_router(pool: Pool) -> Router {
    router(pool)
}

