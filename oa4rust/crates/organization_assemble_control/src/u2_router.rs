use super::{u2_misc, u2_org, u2_person};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/jaxrs/organization/assemble/control/person", axum::routing::post(u2_person::person_create))
        .route(
            "/jaxrs/organization/assemble/control/person/{flag}",
            axum::routing::get(u2_person::person_get)
                .put(u2_person::person_edit)
                .delete(u2_person::person_delete),
        )
        .route("/jaxrs/organization/assemble/control/person/{flag}/mockputtopost", axum::routing::post(u2_person::person_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/person/{flag}/mockdeletetoget", axum::routing::get(u2_person::person_mock_delete_to_get))
        .route("/jaxrs/organization/assemble/control/person/{flag}/reserve", axum::routing::delete(u2_person::person_reserve_delete))
        .route("/jaxrs/organization/assemble/control/person/{flag}/reserve/mockdeletetoget", axum::routing::get(u2_person::person_reserve_mock_delete_to_get))
        .route("/jaxrs/organization/assemble/control/person/list/{flag}/next/{count}", axum::routing::get(u2_person::person_list_next))
        .route("/jaxrs/organization/assemble/control/person/list/{flag}/prev/{count}", axum::routing::get(u2_person::person_list_prev))
        .route("/jaxrs/organization/assemble/control/person/list/group/{groupFlag}/sub/direct", axum::routing::get(u2_person::person_list_group_sub_direct))
        .route("/jaxrs/organization/assemble/control/person/list/group/{groupFlag}/sub/nested", axum::routing::get(u2_person::person_list_group_sub_nested))
        .route("/jaxrs/organization/assemble/control/person/list/role/{roleFlag}", axum::routing::get(u2_person::person_list_with_role))
        .route("/jaxrs/organization/assemble/control/person/list/pinyininitial", axum::routing::put(u2_person::person_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/person/list/pinyininitial/mockputtopost", axum::routing::post(u2_person::person_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/person/list/like", axum::routing::put(u2_person::person_list_like))
        .route("/jaxrs/organization/assemble/control/person/list/like/mockputtopost", axum::routing::post(u2_person::person_list_like))
        .route("/jaxrs/organization/assemble/control/person/list/like/pinyin", axum::routing::put(u2_person::person_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/person/list/like/pinyin/mockputtopost", axum::routing::post(u2_person::person_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/person/list/filter/{page}/size/{size}", axum::routing::post(u2_person::person_list_filter_paging))
        .route("/jaxrs/organization/assemble/control/person/list/delete/{page}/size/{size}", axum::routing::post(u2_person::person_list_delete_paging))
        .route("/jaxrs/organization/assemble/control/person/{flag}/set/password", axum::routing::put(u2_person::person_set_password))
        .route("/jaxrs/organization/assemble/control/person/{flag}/set/password/mockputtopost", axum::routing::post(u2_person::person_set_password_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/person/{flag}/reset/password", axum::routing::get(u2_person::person_reset_password))
        .route("/jaxrs/organization/assemble/control/person/check/password/{password}", axum::routing::get(u2_person::person_check_password))
        .route(
            "/jaxrs/organization/assemble/control/person/{flag}/icon",
            axum::routing::get(u2_person::person_get_icon).put(u2_person::person_set_icon),
        )
        .route("/jaxrs/organization/assemble/control/person/{flag}/icon/mockputtopost", axum::routing::post(u2_person::person_set_icon_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/person/{flag}/set/password/expired/time/{date}", axum::routing::get(u2_person::person_set_password_expired_time))
        .route("/jaxrs/organization/assemble/control/person/lock/{flag}", axum::routing::post(u2_person::person_lock))
        .route("/jaxrs/organization/assemble/control/person/unlock/{flag}", axum::routing::get(u2_person::person_unlock))
        .route("/jaxrs/organization/assemble/control/person/ban/{flag}", axum::routing::post(u2_person::person_ban))
        .route("/jaxrs/organization/assemble/control/person/unban/{flag}", axum::routing::post(u2_person::person_unban))
        .route("/jaxrs/organization/assemble/control/unit", axum::routing::post(u2_org::unit_create))
        .route(
            "/jaxrs/organization/assemble/control/unit/{flag}",
            axum::routing::put(u2_org::unit_edit).delete(u2_org::unit_delete),
        )
        .route("/jaxrs/organization/assemble/control/unit/{flag}/mockputtopost", axum::routing::post(u2_org::unit_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/unit/{flag}/mockdeletetoget", axum::routing::get(u2_org::unit_mock_delete_to_get))
        .route("/jaxrs/organization/assemble/control/unit/{flag}/sup/direct", axum::routing::get(u2_org::unit_get_sup_direct))
        .route("/jaxrs/organization/assemble/control/unit/get/root", axum::routing::get(u2_org::unit_get_root))
        .route("/jaxrs/organization/assemble/control/unit/list", axum::routing::post(u2_org::unit_list_by_body))
        .route("/jaxrs/organization/assemble/control/unit/list/controller", axum::routing::post(u2_org::unit_list_controller))
        .route("/jaxrs/organization/assemble/control/unit/list/top", axum::routing::get(u2_org::unit_list_top_root))
        .route("/jaxrs/organization/assemble/control/unit/list/top/type/{type}", axum::routing::get(u2_org::unit_list_top_with_type))
        .route("/jaxrs/organization/assemble/control/unit/list/control/top", axum::routing::get(u2_org::unit_control_top))
        .route("/jaxrs/organization/assemble/control/unit/list/type", axum::routing::get(u2_org::unit_list_types))
        .route("/jaxrs/organization/assemble/control/unit/list/{flag}/prev/{count}", axum::routing::get(u2_org::unit_list_prev))
        .route("/jaxrs/organization/assemble/control/unit/list/{flag}/sub/direct", axum::routing::get(u2_org::unit_list_sub_direct))
        .route("/jaxrs/organization/assemble/control/unit/list/{flag}/sub/direct/type/{type}", axum::routing::get(u2_org::unit_list_sub_direct_with_type))
        .route("/jaxrs/organization/assemble/control/unit/identity/{identityFlag}/level/{level}", axum::routing::get(u2_org::unit_get_with_identity_level))
        .route("/jaxrs/organization/assemble/control/unit/identity/{identityFlag}/type/{type}", axum::routing::get(u2_org::unit_get_with_identity_type))
        .route("/jaxrs/organization/assemble/control/unit/list/unit/type", axum::routing::put(u2_org::unit_list_with_unit_type))
        .route("/jaxrs/organization/assemble/control/unit/list/unit/type/mockputtopost", axum::routing::post(u2_org::unit_list_with_unit_type))
        .route("/jaxrs/organization/assemble/control/unit/list/pinyininitial", axum::routing::put(u2_org::unit_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/unit/list/pinyininitial/mockputtopost", axum::routing::post(u2_org::unit_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/unit/list/like", axum::routing::put(u2_org::unit_list_like))
        .route("/jaxrs/organization/assemble/control/unit/list/like/mockputtopost", axum::routing::post(u2_org::unit_list_like))
        .route("/jaxrs/organization/assemble/control/unit/list/like/pinyin", axum::routing::put(u2_org::unit_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/unit/list/like/pinyin/mockputtopost", axum::routing::post(u2_org::unit_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/identity", axum::routing::post(u2_org::identity_create))
        .route(
            "/jaxrs/organization/assemble/control/identity/{flag}",
            axum::routing::put(u2_org::identity_edit).delete(u2_org::identity_delete),
        )
        .route("/jaxrs/organization/assemble/control/identity/{flag}/mockputtopost", axum::routing::post(u2_org::identity_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/identity/list/like", axum::routing::put(u2_org::identity_list_like))
        .route("/jaxrs/organization/assemble/control/identity/list/like/mockputtopost", axum::routing::post(u2_org::identity_list_like))
        .route("/jaxrs/organization/assemble/control/identity/list/like/pinyin", axum::routing::put(u2_org::identity_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost", axum::routing::post(u2_org::identity_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/identity/list/pinyininitial", axum::routing::put(u2_org::identity_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost", axum::routing::post(u2_org::identity_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/group", axum::routing::post(u2_org::group_create))
        .route(
            "/jaxrs/organization/assemble/control/group/{flag}",
            axum::routing::put(u2_org::group_edit).delete(u2_org::group_delete),
        )
        .route("/jaxrs/organization/assemble/control/group/{flag}/mockputtopost", axum::routing::post(u2_org::group_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/group/{flag}/add/member", axum::routing::put(u2_org::group_add_member))
        .route("/jaxrs/organization/assemble/control/group/{flag}/add/member/mockputtopost", axum::routing::post(u2_org::group_add_member_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/group/{flag}/delete/member", axum::routing::put(u2_org::group_delete_member))
        .route("/jaxrs/organization/assemble/control/group/{flag}/delete/member/mockputtopost", axum::routing::post(u2_org::group_delete_member_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/group/list/like", axum::routing::put(u2_org::group_list_like))
        .route("/jaxrs/organization/assemble/control/group/list/like/mockputtopost", axum::routing::post(u2_org::group_list_like))
        .route("/jaxrs/organization/assemble/control/group/list/like/pinyin", axum::routing::put(u2_org::group_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost", axum::routing::post(u2_org::group_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/group/list/pinyininitial", axum::routing::put(u2_org::group_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost", axum::routing::post(u2_org::group_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/role", axum::routing::post(u2_org::role_create))
        .route(
            "/jaxrs/organization/assemble/control/role/{flag}",
            axum::routing::put(u2_org::role_edit).delete(u2_org::role_delete),
        )
        .route("/jaxrs/organization/assemble/control/role/{flag}/mockputtopost", axum::routing::post(u2_org::role_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/role/list/like", axum::routing::put(u2_org::role_list_like))
        .route("/jaxrs/organization/assemble/control/role/list/like/mockputtopost", axum::routing::post(u2_org::role_list_like))
        .route("/jaxrs/organization/assemble/control/role/list/like/pinyin", axum::routing::put(u2_org::role_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost", axum::routing::post(u2_org::role_list_like_pinyin))
        .route("/jaxrs/organization/assemble/control/role/list/pinyininitial", axum::routing::put(u2_org::role_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost", axum::routing::post(u2_org::role_list_pinyininitial))
        .route("/jaxrs/organization/assemble/control/unitduty", axum::routing::post(u2_org::duty_create))
        .route(
            "/jaxrs/organization/assemble/control/unitduty/{flag}",
            axum::routing::put(u2_org::duty_edit).delete(u2_org::duty_delete),
        )
        .route("/jaxrs/organization/assemble/control/unitduty/{flag}/mockputtopost", axum::routing::post(u2_org::duty_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/unitduty/update/member", axum::routing::post(u2_org::duty_update_member))
        .route("/jaxrs/organization/assemble/control/unitduty/list/like", axum::routing::put(u2_org::duty_list_like))
        .route("/jaxrs/organization/assemble/control/unitattribute", axum::routing::post(u2_misc::unit_attribute_create))
        .route(
            "/jaxrs/organization/assemble/control/unitattribute/{flag}",
            axum::routing::put(u2_misc::unit_attribute_edit).delete(u2_misc::unit_attribute_delete),
        )
        .route("/jaxrs/organization/assemble/control/unitattribute/{flag}/mockputtopost", axum::routing::post(u2_misc::unit_attribute_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/personattribute", axum::routing::post(u2_misc::person_attribute_create))
        .route(
            "/jaxrs/organization/assemble/control/personattribute/{flag}",
            axum::routing::put(u2_misc::person_attribute_edit).delete(u2_misc::person_attribute_delete),
        )
        .route("/jaxrs/organization/assemble/control/personattribute/{flag}/mockputtopost", axum::routing::post(u2_misc::person_attribute_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/permissionsetting", axum::routing::post(u2_misc::permission_setting_create))
        .route(
            "/jaxrs/organization/assemble/control/permissionsetting/{flag}",
            axum::routing::put(u2_misc::permission_setting_edit).delete(u2_misc::permission_setting_delete),
        )
        .route("/jaxrs/organization/assemble/control/permissionsetting/{flag}/mockputtopost", axum::routing::post(u2_misc::permission_setting_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/personcard", axum::routing::post(u2_misc::person_card_create))
        .route(
            "/jaxrs/organization/assemble/control/personcard/{flag}",
            axum::routing::put(u2_misc::person_card_edit).delete(u2_misc::person_card_delete),
        )
        .route("/jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}", axum::routing::put(u2_misc::person_card_listpaging))
        .route("/jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}/mockputtopost", axum::routing::post(u2_misc::person_card_listpaging_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}", axum::routing::put(u2_misc::person_card_listpaging_with_group))
        .route("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}/mockputtopost", axum::routing::post(u2_misc::person_card_listpaging_with_group_mock_put_to_post))
        .route("/jaxrs/organization/assemble/control/inputperson", axum::routing::post(u2_misc::input_person_import))
}
