// oauth_list
#[allow(non_snake_case)]
pub async fn oauth_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"list\": []}))))
}
// oauth_qywx_config
#[allow(non_snake_case)]
pub async fn oauth_qywx_config(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({}))))
}
// oauth_dingding_config
#[allow(non_snake_case)]
pub async fn oauth_dingding_config(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({}))))
}
// oauth_name
#[allow(non_snake_case)]
pub async fn oauth_name(pool: Extension<Pool>, Path(name): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"name\": name}))))
}
// oauth_login_name_code_redirecturi
#[allow(non_snake_case)]
pub async fn oauth_login_name_code_redirecturi(pool: Extension<Pool>, Path(name): Path<String>, axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = q.get(\"code\").cloned().unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({\"name\": name, \"code\": code}))))
}
// oauth_login_qywx_code
#[allow(non_snake_case)]
pub async fn oauth_login_qywx_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"code\": code}))))
}
// oauth_login_dingding_code
#[allow(non_snake_case)]
pub async fn oauth_login_dingding_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"code\": code}))))
}
// oauth_bind_name_code_redirecturi
#[allow(non_snake_case)]
pub async fn oauth_bind_name_code_redirecturi(pool: Extension<Pool>, Path(name): Path<String>, axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = q.get(\"code\").cloned().unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({\"name\": name, \"code\": code}))))
}
// mpweixin_login_code
#[allow(non_snake_case)]
pub async fn mpweixin_login_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"code\": code}))))
}
// mpweixin_bind_openid
#[allow(non_snake_case)]
pub async fn mpweixin_bind_openid(pool: Extension<Pool>, Path(openid): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"openid\": openid}))))
}
// mpweixin_bind_code
#[allow(non_snake_case)]
pub async fn mpweixin_bind_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"code\": code}))))
}
// mpweixin_menu_test_send_to
#[allow(non_snake_case)]
pub async fn mpweixin_menu_test_send_to(pool: Extension<Pool>, Path(person): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"person\": person}))))
}
// qiyeweixin_code
#[allow(non_snake_case)]
pub async fn qiyeweixin_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"code\": code}))))
}
// qiyeweixin_update_person_detail
#[allow(non_snake_case)]
pub async fn qiyeweixin_update_person_detail(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"code\": code}))))
}
// welink_code
#[allow(non_snake_case)]
pub async fn welink_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"code\": code}))))
}
// zhengwudingding_code
#[allow(non_snake_case)]
pub async fn zhengwudingding_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"code\": code}))))
}
// authentication_bind_meta_get
#[allow(non_snake_case)]
pub async fn authentication_bind_meta_get(pool: Extension<Pool>, Path(meta): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"meta\": meta}))))
}
// authentication_bind_meta_post
#[allow(non_snake_case)]
pub async fn authentication_bind_meta_post(pool: Extension<Pool>, Path(meta): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({\"meta\": meta}))))
}
// andfx_moa_sso_token_enter
#[allow(non_snake_case)]
pub async fn andfx_moa_sso_token_enter(pool: Extension<Pool>, axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = q.get(\"token\").cloned().unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({\"token\": token}))))
}
