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
    endpoints_attr::{
        distinguishedname_list, empower_list_identity_object, empowerlog_create,
        personattr_append_person_name, personattr_list_attribute_person_name,
        personattr_list_name_person, personattr_list_person_object, personattr_set_person_name,
        unitattr_append_unit_name, unitattr_list_attribute_unit_name, unitattr_list_name_unit,
        unitattr_list_unit_object, unitattr_set_unit_name,
    },
    endpoints_duty2::{
        role_list_object, role_list_person_object, unitduty_find_by_unit_name,
        unitduty_list_identity_unit_name, unitduty_list_identity_unit_name_object,
        unitduty_list_name_identity, unitduty_list_unit_object,
    },
    endpoints_group2::{
        group_has_role, group_list_group_sub_direct, group_list_group_sub_direct_object,
        group_list_group_sub_nested, group_list_group_sub_nested_object,
        group_list_group_sup_direct, group_list_group_sup_direct_object,
        group_list_group_sup_nested, group_list_group_sup_nested_object, group_list_group_tree,
        group_list_identity, group_list_identity_object, group_list_person_object,
    },
    endpoints_org::{
        group_list, group_list_object, group_list_person, role_list, role_list_person,
        unit_check_unit_has_person, unit_list, unit_list_all, unit_list_all_object,
        unit_list_object, unit_list_unit_sub_direct, unit_list_unit_sub_direct_object,
        unit_list_unit_sub_nested, unit_list_unit_sub_nested_object, unit_list_unit_sup_direct,
        unit_list_unit_sup_direct_object, unit_list_unit_sup_nested,
        unit_list_unit_sup_nested_object, unitduty_list_name, unitduty_list_name_unit,
    },
    endpoints_org2::{
        identity_list_group, identity_list_group_object, identity_list_major_person,
        identity_list_major_person_object, identity_list_person_object,
        identity_list_unit_person, identity_list_unit_person_object,
        identity_list_unit_sub_direct_object, identity_list_unit_sub_nested_object,
    },
    endpoints_person2::{
        person_detail_flag, person_list_group_object, person_list_identity_object,
        person_list_login_after, person_list_login_after_object, person_list_login_recent,
        person_list_login_recent_object, person_list_pair_identity,
        person_list_person_sub_direct, person_list_person_sub_direct_object,
        person_list_person_sub_nested, person_list_person_sub_nested_object,
        person_list_person_sup_direct, person_list_person_sup_direct_object,
        person_list_person_sup_nested, person_list_person_sup_nested_object,
        person_list_personattribute, person_list_personattribute_object, person_list_role_object,
        person_list_unit_sub_direct, person_list_unit_sub_direct_like,
        person_list_unit_sub_direct_like_object, person_list_unit_sub_direct_object,
        person_list_unit_sub_nested, person_list_unit_sub_nested_like,
        person_list_unit_sub_nested_like_object, person_list_unit_sub_nested_object,
    },
    endpoints_unit2::{
        unit_get_with_identity_with_level, unit_get_with_identity_with_level_object,
        unit_get_with_identity_with_type, unit_get_with_identity_with_type_object,
        unit_check_unit_has_identity, unit_check_unit_has_unit, unit_list_identity,
        unit_list_identity_object, unit_list_identity_sup_nested,
        unit_list_identity_sup_nested_object, unit_list_level, unit_list_level_name_object,
        unit_list_level_object, unit_list_person, unit_list_person_object,
        unit_list_person_sup_nested, unit_list_person_sup_nested_object, unit_list_type_type_object,
        unit_list_types, unit_list_types_object, unit_list_unit_tree, unit_list_unitattribute,
        unit_list_unitattribute_object, unit_list_unitduty, unit_list_unitduty_object,
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
        // ── U2 收尾新增（98 条）───────────────────────────────────────────
        // person 剩余（27）
        .route("/jaxrs/person/list/login/after", post(person_list_login_after))
        .route("/jaxrs/person/list/login/after/object", post(person_list_login_after_object))
        .route("/jaxrs/person/list/login/recent", post(person_list_login_recent))
        .route("/jaxrs/person/list/login/recent/object", post(person_list_login_recent_object))
        .route("/jaxrs/person/list/pair/identity", post(person_list_pair_identity))
        .route("/jaxrs/person/detail/{flag}", post(person_detail_flag))
        .route("/jaxrs/person/list/group/object", post(person_list_group_object))
        .route("/jaxrs/person/list/identity/object", post(person_list_identity_object))
        .route("/jaxrs/person/list/personattribute", post(person_list_personattribute))
        .route(
            "/jaxrs/person/list/personattribute/object",
            post(person_list_personattribute_object),
        )
        .route("/jaxrs/person/list/person/sub/direct", post(person_list_person_sub_direct))
        .route(
            "/jaxrs/person/list/person/sub/direct/object",
            post(person_list_person_sub_direct_object),
        )
        .route("/jaxrs/person/list/person/sub/nested", post(person_list_person_sub_nested))
        .route(
            "/jaxrs/person/list/person/sub/nested/object",
            post(person_list_person_sub_nested_object),
        )
        .route("/jaxrs/person/list/person/sup/direct", post(person_list_person_sup_direct))
        .route(
            "/jaxrs/person/list/person/sup/direct/object",
            post(person_list_person_sup_direct_object),
        )
        .route("/jaxrs/person/list/person/sup/nested", post(person_list_person_sup_nested))
        .route(
            "/jaxrs/person/list/person/sup/nested/object",
            post(person_list_person_sup_nested_object),
        )
        .route("/jaxrs/person/list/role/object", post(person_list_role_object))
        .route("/jaxrs/person/list/unit/sub/direct", post(person_list_unit_sub_direct))
        .route(
            "/jaxrs/person/list/unit/sub/direct/object",
            post(person_list_unit_sub_direct_object),
        )
        .route("/jaxrs/person/list/unit/sub/nested", post(person_list_unit_sub_nested))
        .route(
            "/jaxrs/person/list/unit/sub/nested/object",
            post(person_list_unit_sub_nested_object),
        )
        .route("/jaxrs/person/list/unit/sub/direct/like", post(person_list_unit_sub_direct_like))
        .route(
            "/jaxrs/person/list/unit/sub/direct/like/object",
            post(person_list_unit_sub_direct_like_object),
        )
        .route("/jaxrs/person/list/unit/sub/nested/like", post(person_list_unit_sub_nested_like))
        .route(
            "/jaxrs/person/list/unit/sub/nested/like/object",
            post(person_list_unit_sub_nested_like_object),
        )
        // unit 剩余（29）
        .route("/jaxrs/unit/identity/level", post(unit_get_with_identity_with_level))
        .route("/jaxrs/unit/identity/level/object", post(unit_get_with_identity_with_level_object))
        .route("/jaxrs/unit/identity/type", post(unit_get_with_identity_with_type))
        .route("/jaxrs/unit/identity/type/object", post(unit_get_with_identity_with_type_object))
        .route("/jaxrs/unit/list/identity", post(unit_list_identity))
        .route("/jaxrs/unit/list/identity/object", post(unit_list_identity_object))
        .route("/jaxrs/unit/list/identity/sup/nested", post(unit_list_identity_sup_nested))
        .route(
            "/jaxrs/unit/list/identity/sup/nested/object",
            post(unit_list_identity_sup_nested_object),
        )
        .route("/jaxrs/unit/list/level", post(unit_list_level))
        .route("/jaxrs/unit/list/level/object", post(unit_list_level_object))
        .route("/jaxrs/unit/list/level/name/object", post(unit_list_level_name_object))
        .route("/jaxrs/unit/list/person", post(unit_list_person))
        .route("/jaxrs/unit/list/person/object", post(unit_list_person_object))
        .route("/jaxrs/unit/list/person/sup/nested", post(unit_list_person_sup_nested))
        .route(
            "/jaxrs/unit/list/person/sup/nested/object",
            post(unit_list_person_sup_nested_object),
        )
        .route("/jaxrs/unit/list/unitattribute", post(unit_list_unitattribute))
        .route("/jaxrs/unit/list/unitattribute/object", post(unit_list_unitattribute_object))
        .route("/jaxrs/unit/list/unitduty", post(unit_list_unitduty))
        .route("/jaxrs/unit/list/unitduty/object", post(unit_list_unitduty_object))
        .route("/jaxrs/unit/list/unit/sub/direct/object", post(unit_list_unit_sub_direct_object))
        .route("/jaxrs/unit/list/unit/sub/nested/object", post(unit_list_unit_sub_nested_object))
        .route("/jaxrs/unit/list/unit/sup/direct/object", post(unit_list_unit_sup_direct_object))
        .route("/jaxrs/unit/list/unit/sup/nested/object", post(unit_list_unit_sup_nested_object))
        .route("/jaxrs/unit/list/unit/tree", post(unit_list_unit_tree))
        .route("/jaxrs/unit/check/unit/has/identity", post(unit_check_unit_has_identity))
        .route("/jaxrs/unit/check/unit/has/unit", post(unit_check_unit_has_unit))
        .route("/jaxrs/unit/list/types", post(unit_list_types))
        .route("/jaxrs/unit/list/types/object", post(unit_list_types_object))
        .route("/jaxrs/unit/list/type/{type}/object", get(unit_list_type_type_object))
        // identity 剩余（9）
        .route("/jaxrs/identity/list/person/object", post(identity_list_person_object))
        .route(
            "/jaxrs/identity/list/unit/sub/direct/object",
            post(identity_list_unit_sub_direct_object),
        )
        .route(
            "/jaxrs/identity/list/unit/sub/nested/object",
            post(identity_list_unit_sub_nested_object),
        )
        .route("/jaxrs/identity/list/unit/person", post(identity_list_unit_person))
        .route("/jaxrs/identity/list/unit/person/object", post(identity_list_unit_person_object))
        .route("/jaxrs/identity/list/group", post(identity_list_group))
        .route("/jaxrs/identity/list/group/object", post(identity_list_group_object))
        .route("/jaxrs/identity/list/major/person", post(identity_list_major_person))
        .route("/jaxrs/identity/list/major/person/object", post(identity_list_major_person_object))
        // group 剩余（13）
        .route("/jaxrs/group/has/role", post(group_has_role))
        .route("/jaxrs/group/list/group/sub/direct", post(group_list_group_sub_direct))
        .route(
            "/jaxrs/group/list/group/sub/direct/object",
            post(group_list_group_sub_direct_object),
        )
        .route("/jaxrs/group/list/group/sub/nested", post(group_list_group_sub_nested))
        .route(
            "/jaxrs/group/list/group/sub/nested/object",
            post(group_list_group_sub_nested_object),
        )
        .route("/jaxrs/group/list/group/sup/direct", post(group_list_group_sup_direct))
        .route(
            "/jaxrs/group/list/group/sup/direct/object",
            post(group_list_group_sup_direct_object),
        )
        .route("/jaxrs/group/list/group/sup/nested", post(group_list_group_sup_nested))
        .route(
            "/jaxrs/group/list/group/sup/nested/object",
            post(group_list_group_sup_nested_object),
        )
        .route("/jaxrs/group/list/person/object", post(group_list_person_object))
        .route("/jaxrs/group/list/identity", post(group_list_identity))
        .route("/jaxrs/group/list/identity/object", post(group_list_identity_object))
        .route("/jaxrs/group/list/group/tree", post(group_list_group_tree))
        // role 剩余（2）
        .route("/jaxrs/role/list/object", post(role_list_object))
        .route("/jaxrs/role/list/person/object", post(role_list_person_object))
        // unitduty 剩余（5）
        .route("/jaxrs/unitduty/list/identity/unit/name", post(unitduty_list_identity_unit_name))
        .route(
            "/jaxrs/unitduty/list/identity/unit/name/object",
            post(unitduty_list_identity_unit_name_object),
        )
        .route("/jaxrs/unitduty/list/name/identity", post(unitduty_list_name_identity))
        .route("/jaxrs/unitduty/list/unit/object", post(unitduty_list_unit_object))
        .route("/jaxrs/unitduty/find/by/unit/name", post(unitduty_find_by_unit_name))
        // personattribute（5）
        .route("/jaxrs/personattribute/list/name/person", post(personattr_list_name_person))
        .route(
            "/jaxrs/personattribute/list/attribute/person/name",
            post(personattr_list_attribute_person_name),
        )
        .route("/jaxrs/personattribute/list/person/object", post(personattr_list_person_object))
        .route("/jaxrs/personattribute/set/person/name", post(personattr_set_person_name))
        .route("/jaxrs/personattribute/append/person/name", post(personattr_append_person_name))
        // unitattribute（5）
        .route("/jaxrs/unitattribute/list/name/unit", post(unitattr_list_name_unit))
        .route(
            "/jaxrs/unitattribute/list/attribute/unit/name",
            post(unitattr_list_attribute_unit_name),
        )
        .route("/jaxrs/unitattribute/list/unit/object", post(unitattr_list_unit_object))
        .route("/jaxrs/unitattribute/set/unit/name", post(unitattr_set_unit_name))
        .route("/jaxrs/unitattribute/append/unit/name", post(unitattr_append_unit_name))
        // empower / empowerlog / distinguishedname（3）
        .route("/jaxrs/empower/list/identity/object", post(empower_list_identity_object))
        .route("/jaxrs/empowerlog", post(empowerlog_create))
        .route("/jaxrs/distinguishedname/list", post(distinguishedname_list))
        .layer(Extension(pool))
}
