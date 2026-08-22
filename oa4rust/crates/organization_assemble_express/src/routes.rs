use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    endpoints::{
        identity_list, identity_list_object, identity_list_person,
        identity_list_unit_sub_direct, identity_list_unit_sub_nested, person_auth_info_flag,
        person_has_role, person_list, person_list_all, person_list_all_object,
        person_list_filter_page_size, person_list_group, person_list_identity,
        person_list_object, person_list_role, person_mobile_flag, person_nick_name_flag,
    },
    endpoints_org::{
        group_list, group_list_object, group_list_person, role_list, role_list_person,
        unit_check_unit_has_person, unit_list, unit_list_all, unit_list_all_object,
        unit_list_object, unit_list_unit_sub_direct, unit_list_unit_sub_nested,
        unit_list_unit_sup_direct, unit_list_unit_sup_nested, unitduty_list_name,
        unitduty_list_name_unit,
    },
    get_express_config, get_express_status, list_organization_units, sync_organization_data,
};

/// plan002 U2：路由对齐 Java x_organization_assemble_express 契约路径
/// （/jaxrs/{resource}/...，与 control/express 等 crate 的既有注册互不冲突：
/// GET /jaxrs/person/{flag}、GET /jaxrs/unit/list 已被 control 占用，
/// 本模块仅注册其余方法+路径组合）。
pub fn router(pool: Pool) -> axum::Router {
    Router::new()
        // 既有（U2 之前的占位端点，保留）
        .route("/jaxrs/organization/assemble/express/config/get", get(get_express_config))
        .route("/jaxrs/organization/assemble/express/units/list", get(list_organization_units))
        .route("/jaxrs/organization/assemble/express/data/sync", get(sync_organization_data))
        .route("/jaxrs/organization/assemble/express/status/get", get(get_express_status))
        // person（12）
        .route("/jaxrs/person/auth/info/{flag}", get(person_auth_info_flag))
        .route("/jaxrs/person/nick/name/{flag}", get(person_nick_name_flag))
        .route("/jaxrs/person/mobile/{flag}", get(person_mobile_flag))
        .route("/jaxrs/person/list", post(person_list))
        .route("/jaxrs/person/list/object", post(person_list_object))
        .route("/jaxrs/person/list/all", get(person_list_all))
        .route("/jaxrs/person/list/all/object", get(person_list_all_object))
        .route("/jaxrs/person/has/role", post(person_has_role))
        .route("/jaxrs/person/list/identity", post(person_list_identity))
        .route("/jaxrs/person/list/group", post(person_list_group))
        .route("/jaxrs/person/list/role", post(person_list_role))
        .route(
            "/jaxrs/person/list/filter/{page}/size/{size}",
            post(person_list_filter_page_size),
        )
        // identity（5）
        .route("/jaxrs/identity/list", post(identity_list))
        .route("/jaxrs/identity/list/object", post(identity_list_object))
        .route("/jaxrs/identity/list/person", post(identity_list_person))
        .route("/jaxrs/identity/list/unit/sub/direct", post(identity_list_unit_sub_direct))
        .route("/jaxrs/identity/list/unit/sub/nested", post(identity_list_unit_sub_nested))
        // unit（9）
        .route("/jaxrs/unit/list", post(unit_list))
        .route("/jaxrs/unit/list/object", post(unit_list_object))
        .route("/jaxrs/unit/list/all", get(unit_list_all))
        .route("/jaxrs/unit/list/all/object", get(unit_list_all_object))
        .route("/jaxrs/unit/list/unit/sub/direct", post(unit_list_unit_sub_direct))
        .route("/jaxrs/unit/list/unit/sub/nested", post(unit_list_unit_sub_nested))
        .route("/jaxrs/unit/list/unit/sup/direct", post(unit_list_unit_sup_direct))
        .route("/jaxrs/unit/list/unit/sup/nested", post(unit_list_unit_sup_nested))
        .route("/jaxrs/unit/check/unit/has/person", post(unit_check_unit_has_person))
        // group（3）
        .route("/jaxrs/group/list", post(group_list))
        .route("/jaxrs/group/list/object", post(group_list_object))
        .route("/jaxrs/group/list/person", post(group_list_person))
        // role（2）
        .route("/jaxrs/role/list", post(role_list))
        .route("/jaxrs/role/list/person", post(role_list_person))
        // unitduty（2）
        .route("/jaxrs/unitduty/list/name", post(unitduty_list_name))
        .route("/jaxrs/unitduty/list/name/unit", post(unitduty_list_name_unit))
        .layer(Extension(pool))
}
