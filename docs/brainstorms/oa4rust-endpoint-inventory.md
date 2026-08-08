# OA4Rust 端点清单文档

> 生成日期：2026-08-06
> 参考来源：R34-R35、Implementation Units (U4-U7)、各 crate `src/lib.rs` / `src/routes.rs`

## 总览

| 波次 | 优先级 | 包含 crate 数 | 说明 |
|------|--------|--------------|------|
| Wave 1 (U4) | 高 | 5 | 组织控制与个人信息（control、personal、personal_extend、program_init、auth） |
| Wave 2 (U5) | 中 | 11 | 文件、日历、考勤、综合管控（file、calendar、attendance、general_assemble_control 及其 _core_entity/_assemble_control） |
| Wave 3 (U6) | 中 | 31 | 流程、消息、会议、门户、查询、CMS |
| Wave 4 (U7) | 低 | 33 | 基础设施与边缘模块（AI、组件、热点、推送、思维导图、BBS、快递、控制台、表达式、关联关系、序列号等） |

## 实现状态说明

| 状态 | 说明 |
|------|------|
| 已完成 | 已接入 PostgreSQL 真实业务逻辑，返回真实数据 |
| 部分完成 | 部分端点有真实 DB 查询，部分为 mock 数据或 stub |
| 桩代码 | 返回硬编码 mock 数据或 `ActionResult::success(Value::Null)` |

---

## Wave 1 — 高优先级（U4）

### auth

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/authentication | POST | login | 已完成 | 登录，支持 bcrypt/MD5/DES |
| /jaxrs/authentication | GET | whoami | 已完成 | 当前用户信息 |
| /jaxrs/authentication | DELETE | logout | 已完成 | 登出 |
| /jaxrs/authentication/refresh | POST | refresh | 已完成 | 刷新令牌 |
| /jaxrs/authentication/code/credential/{credential} | GET | code_send | 已完成 | 发送短信验证码 |
| /jaxrs/authentication/code | POST | code_verify | 已完成 | 验证短信验证码 |
| /jaxrs/authentication/captcha | GET | captcha_image | 已完成 | 验证码图片 |
| /jaxrs/authentication/oauth/{provider} | GET | oauth_authorize | 已完成 | OAuth 授权 |
| /jaxrs/authentication/oauth/{provider}/callback | GET | oauth_callback | 已完成 | OAuth 回调 |
| /jaxrs/organization/person/list | GET | person::list | 已完成 | 人员列表 |
| /jaxrs/organization/unit/list | GET | unit::list | 已完成 | 单位列表 |
| /jaxrs/organization/role/list | GET | role::list | 已完成 | 角色列表 |

### control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/person | POST | person::create | 已完成 | 创建人员 |
| /jaxrs/person/{flag} | GET | person::get | 已完成 | 查询人员 |
| /jaxrs/person/{flag} | PUT | person::update | 已完成 | 更新人员 |
| /jaxrs/person/{flag} | DELETE | person::delete | 已完成 | 删除人员（软删除） |
| /jaxrs/person/list/{flag}/next/{count} | GET | person::list_next | 已完成 | 游标分页下一页 |
| /jaxrs/person/list/{flag}/prev/{count} | GET | person::list_prev | 已完成 | 游标分页上一页 |
| /jaxrs/group | POST | group::create | 已完成 | 创建用户组 |
| /jaxrs/group/{flag} | GET | group::get | 已完成 | 查询用户组 |
| /jaxrs/group/{flag} | PUT | group::update | 已完成 | 更新用户组 |
| /jaxrs/group/{flag} | DELETE | group::delete | 已完成 | 删除用户组 |
| /jaxrs/group/list/{flag}/next/{count} | GET | group::list_next | 已完成 | 游标分页下一页 |
| /jaxrs/group/list/{flag}/prev/{count} | GET | group::list_prev | 已完成 | 游标分页上一页 |
| /jaxrs/role | POST | role::create | 已完成 | 创建角色 |
| /jaxrs/role/{flag} | GET | role::get | 已完成 | 查询角色 |
| /jaxrs/role/{flag} | PUT | role::update | 已完成 | 更新角色 |
| /jaxrs/role/{flag} | DELETE | role::delete | 已完成 | 删除角色 |
| /jaxrs/role/list/{flag}/next/{count} | GET | role::list_next | 已完成 | 游标分页下一页 |
| /jaxrs/role/list/{flag}/prev/{count} | GET | role::list_prev | 已完成 | 游标分页上一页 |
| /jaxrs/unit | POST | unit::create | 已完成 | 创建单位 |
| /jaxrs/unit/list | GET | unit::list | 已完成 | 单位列表 |
| /jaxrs/unit/{flag} | GET | unit::get | 已完成 | 查询单位 |
| /jaxrs/unit/{flag} | PUT | unit::update | 已完成 | 更新单位 |
| /jaxrs/unit/{flag} | DELETE | unit::delete | 已完成 | 删除单位 |
| /jaxrs/unit/list/{flag}/next/{count} | GET | unit::list_next | 已完成 | 游标分页下一页 |
| /jaxrs/unit/list/{flag}/prev/{count} | GET | unit::list_prev | 已完成 | 游标分页上一页 |

### personal

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/person | GET | get_person | 已完成 | 查询当前用户信息 |
| /jaxrs/person | PUT | edit_person | 已完成 | 更新当前用户信息 |
| /jaxrs/person/mockputtopost | POST | edit_person | 已完成 | 兼容前端 POST 更新 |
| /jaxrs/person/password | PUT | password::change | 已完成 | 修改密码 |
| /jaxrs/reset/check/credential/{credential} | GET | reset::check_credential | 已完成 | 检查重置凭据 |
| /jaxrs/reset/check/password/{password} | GET | reset::check_password | 已完成 | 检查重置密码 |
| /jaxrs/reset/code/credential/{credential} | GET | reset::send_code | 已完成 | 发送重置验证码 |
| /jaxrs/reset | PUT | reset::reset_password | 已完成 | 重置密码 |
| /jaxrs/reset/password/anonymous | POST | reset::reset_password_anonymous | 已完成 | 匿名重置密码 |

### personal_extend

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/personal/info | GET | personal::get_info | 已完成 | 个人详情 |
| /jaxrs/personal/update | PUT | personal::update_info | 已完成 | 更新个人信息 |
| /jaxrs/personal/detail/{id} | GET | personal::get_detail | 已完成 | 个人详情（按ID） |
| /jaxrs/person/icon | PUT | avatar::upload | 已完成 | 上传头像 |
| /jaxrs/person/icon | GET | avatar::get_current_icon | 已完成 | 获取当前用户头像 |
| /jaxrs/icon/{person} | GET | avatar::get_icon | 已完成 | 获取指定用户头像 |

### program_init

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/secret/check | GET | check | 已完成 | 检查系统初始化状态 |
| /jaxrs/secret/set | POST | set | 已完成 | 设置初始化密钥（AES-GCM 加密） |
| /jaxrs/secret/set/cancel | GET | set_cancel | 已完成 | 清除初始化密钥 |

---

## Wave 2 — 中优先级（U5）

### attendance

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/attendance/admin/list/all | GET | list_admins | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/employee/config/list/all | GET | list_employee_configs | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/statistical/cycle/list/all | GET | list_statistical_cycles | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/record/list | GET | list_check_in_records | 桩代码 | 待实现 |
| /jaxrs/attendance/rule/list | GET | list_schedule_rules | 桩代码 | 待实现 |
| /jaxrs/attendance/appeal/list | GET | list_appeal_records | 桩代码 | 待实现 |
| /jaxrs/attendance/appeal/submit | POST | submit_appeal | 桩代码 | 待实现 |
| /jaxrs/attendance/appeal/audit | POST | audit_appeal | 桩代码 | 待实现 |
| /jaxrs/attendance/appeal/archive/{id} | POST | archive_appeal | 桩代码 | 待实现 |

### attendance_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/attendance/assemble/control/rule/list | GET | list_control_rules | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/assemble/control/rule/{id}/toggle | POST | toggle_control_rule | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/assemble/control/attendanceadmin/list/all | GET | attendanceadmin_list_all | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/assemble/control/attendanceadmin/{id} | GET | attendanceadmin_id | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/assemble/control/attendanceappealInfo/appeal/{id} | POST | attendanceappealInfo_appeal_id | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/assemble/control/attendanceselfholiday/list/all | GET | stub_... | 桩代码 | 待实现 |
| ...（共 97 个路由，其余均为 stub） | | | | |

### attendance_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/attendance/core/entity/record/list | GET | record_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/core/entity/rule/list | GET | rule_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/attendance/core/entity/appeal/list | GET | appeal_list | 部分完成 | 真实 DB 查询 |

### calendar

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/calendar/calendar/list/public | GET | calendar_list_public | 部分完成 | 真实 DB 查询 |
| /jaxrs/calendar/calendar/list/my | GET | calendar_list_my | 部分完成 | 真实 DB 查询 |
| /jaxrs/calendar/calendar/{id} | GET | calendar_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/calendar/calendar/create | POST | calendar_create | 桩代码 | 待实现 |
| /jaxrs/calendar/calendar/update | POST | calendar_update | 桩代码 | 待实现 |
| /jaxrs/calendar/calendar/remove | POST | calendar_remove | 桩代码 | 待实现 |
| /jaxrs/calendar/event/create | POST | event_create | 桩代码 | 待实现 |
| /jaxrs/calendar/event/update | POST | event_update | 桩代码 | 待实现 |
| /jaxrs/calendar/event/remove | POST | event_remove | 桩代码 | 待实现 |
| /jaxrs/calendar/event/list/{calendarId} | GET | event_list | 桩代码 | 待实现 |

### calendar_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/calendar/assemble/control/config/get | GET | get_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/calendar/assemble/control/calendars | GET | list_control_calendars | 部分完成 | 真实 DB 查询 |
| /jaxrs/calendar/assemble/control/config/update | GET | update_control_config | 部分完成 | 真实 DB 查询 |

### calendar_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/calendar/core/entity/calendar/list | GET | calendar_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/calendar/core/entity/calendar/{id} | GET | calendar_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/calendar/core/entity/calendar/create | POST | calendar_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/calendar/core/entity/calendar/update | POST | calendar_update | 部分完成 | 真实 DB 查询 |
| /jaxrs/calendar/core/entity/calendar/delete/{id} | POST | calendar_delete | 部分完成 | 真实 DB 查询 |

### file

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/file/folder/list/top | GET | folder_list_top | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/folder/list/{id} | GET | folder_list_with_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/complex/top | GET | complex_top | 部分完成 | 真实 DB 查询 |

### file_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/file/assemble/control/config/get | GET | get_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/assemble/control/storage/pools | GET | list_storage_pools | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/assemble/control/config/update | GET | update_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/assemble/control/categories | GET | list_control_categories | 部分完成 | 真实 DB 查询 |

### file_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/file/core/entity/folder/list | GET | folder_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/core/entity/folder/{id} | GET | folder_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/core/entity/folder/create | POST | folder_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/core/entity/folder/update/{id} | POST | folder_update | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/core/entity/folder/delete/{id} | POST | folder_delete | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/core/entity/file/list | GET | file_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/core/entity/file/{id} | GET | file_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/core/entity/file/create | POST | file_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/core/entity/file/update/{id} | POST | file_update | 部分完成 | 真实 DB 查询 |
| /jaxrs/file/core/entity/file/delete/{id} | POST | file_delete | 部分完成 | 真实 DB 查询 |

### general_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/general/assemble/control/status | GET | get_general_control_status | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/status/update | POST | update_general_control_status | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/permissions/{module} | GET | get_module_permissions | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/attendscope/list | GET | attendscope_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/attendscope/{id} | GET | attendscope_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/attendscope/create | POST | attendscope_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/attendscope/save/{id} | POST | attendscope_save | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/attendscope/delete/{id} | POST | attendscope_delete | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/area/list | GET | stub_... | 桩代码 | 待实现 |
| /jaxrs/general/assemble/control/area/create | POST | area_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/area/{id} | GET | area_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/area/update/{id} | POST | area_update | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/area/delete/{id} | POST | area_delete | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/qrcode/list | GET | qrcode_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/qrcode/{id} | GET | qrcode_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/qrcode/delete/{id} | POST | qrcode_delete | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/securityclearance/create | POST | securityclearance_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/securityclearance/{id} | GET | securityclearance_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/securityclearance/update/{id} | POST | securityclearance_update | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/assemble/control/securityclearance/delete/{id} | POST | securityclearance_delete | 部分完成 | 真实 DB 查询 |
| ...（共 71 个路由，其余均为 stub） | | | | |

### general_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/general/core/entity/dict/list | GET | dict_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/dict/{id} | GET | dict_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/dict/create | POST | dict_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/dict/update/{id} | POST | dict_update | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/dict/delete/{id} | POST | dict_delete | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/dict/item/list/{dictId} | GET | dict_item_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/dict/item/create | POST | dict_item_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/dict/item/{id} | GET | dict_item_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/dict/item/update/{id} | POST | dict_item_update | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/dict/item/delete/{id} | POST | dict_item_delete | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/file/create | POST | file_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/file/{id} | GET | file_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/file/update/{id} | POST | file_update | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/file/delete/{id} | POST | file_delete | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/file/download/{id} | GET | file_download | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/invoice/list | GET | invoice_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/invoice/{id} | GET | invoice_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/invoice/create | POST | invoice_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/invoice/update/{id} | POST | invoice_update | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/core/entity/invoice/delete/{id} | POST | invoice_delete | 部分完成 | 真实 DB 查询 |

---

## Wave 3 — 中优先级（U6）

### meeting

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/meeting/room/list | GET | room_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/building/list | GET | building_list | 桩代码 | 待实现 |
| /jaxrs/meeting/openmeeting/list/room | GET | openmeeting_list_room | 桩代码 | 待实现 |
| /jaxrs/meeting/create | POST | create_meeting | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/{id} | GET | get_meeting | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/list | GET | list_meetings | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/schedule/days/{days} | GET | list_schedule | 桩代码 | 待实现 |
| /jaxrs/meeting/{meetingId}/participant/add | POST | add_participant | 桩代码 | 待实现 |
| /jaxrs/meeting/{meetingId}/participant/list | GET | list_participants | 桩代码 | 待实现 |

### meeting_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/meeting/assemble/control/list/{meetingId} | GET | list_meeting_controls | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/assemble/control/create | POST | create_meeting_control | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/assemble/control/delete/{id} | DELETE | delete_meeting_control | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/assemble/control/meeting/create | POST | create_meeting | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/assemble/control/meeting/{id} | GET | meeting_id | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/assemble/control/meeting/save/{id} | POST | save_meeting | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/assemble/control/meeting/delete/{id} | POST | delete_meeting | 部分完成 | 真实 DB 查询 |
| ...（共 92 个路由，其余均为 stub） | | | | |

### meeting_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/meeting/core/entity/room/list | GET | room_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/core/entity/meeting/list | GET | meeting_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/core/entity/meeting/list/by/{roomId} | GET | meeting_list_by_room | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/core/entity/meeting/create | POST | create_meeting | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/core/entity/meeting/{id} | GET | get_meeting | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/core/entity/meeting/save/{id} | POST | update_meeting | 部分完成 | 真实 DB 查询 |
| /jaxrs/meeting/core/entity/meeting/delete/{id} | POST | delete_meeting | 部分完成 | 真实 DB 查询 |

### message

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/message/consume/list/{consume}/count/{count} | GET | consume_list | 已完成 | 真实 DB 查询 |
| /jaxrs/message/consume/{id}/type/{type} | GET | update_single | 已完成 | 真实 DB 查询 |
| /jaxrs/message/custom/create | POST | custom_create | 已完成 | 真实 DB 查询 |
| /jaxrs/message/mark_read/{id} | POST | mark_read | 已完成 | 真实 DB 查询 |
| /jaxrs/message/unread/count/{consume} | GET | unread_count | 已完成 | 真实 DB 查询 |

### message_assemble_communicate

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/message/assemble/communicate/send | POST | send_message | 部分完成 | mock 数据 |
| /jaxrs/message/assemble/communicate/receive/{consume} | GET | receive_list | 部分完成 | mock 数据 |
| /jaxrs/message/assemble/communicate/mark_read/{id} | POST | mark_read | 部分完成 | mock 数据 |
| /jaxrs/message/assemble/communicate/consume/list/{consume}/count/{count} | GET | stub_... | 部分完成 | 真实 DB 查询 |
| ...（共 118 个路由，其余均为 stub） | | | | |

### message_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/message/core/entity/list | GET | list | 部分完成 | 真实 DB 查询 |
| /jaxrs/message/core/entity/list/by/{consume} | GET | list_by_consume | 部分完成 | 真实 DB 查询 |
| /jaxrs/message/core/entity/unread/count/{consume} | GET | unread_count | 部分完成 | 真实 DB 查询 |

### portal

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/portal/{id} | GET | get_portal | 桩代码 | mock 数据 |
| /jaxrs/portal/list | GET | list_portal | 桩代码 | mock 数据 |
| /jaxrs/portalcategory/list | GET | list_portal_category | 桩代码 | mock 数据 |
| /jaxrs/portal/page/{id} | GET | get_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/page/create | POST | create_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/page/save/{id} | POST | save_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/page/delete/{id} | POST | delete_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/dict/list | GET | dict_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/widget/list | GET | widget_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/script/list | GET | script_list | 部分完成 | 真实 DB 查询 |

### portal_assemble_designer

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/portal/assemble/designer/page/list/{category} | GET | list_pages_by_category | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/assemble/designer/page/{id} | GET | get_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/assemble/designer/page/create | POST | create_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/assemble/designer/page/save/{id} | POST | save_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/assemble/designer/page/delete/{id} | POST | delete_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/assemble/designer/create | POST | create_design | 桩代码 | mock 数据 |
| /jaxrs/portal/assemble/designer/get/{id} | GET | get_design | 桩代码 | mock 数据 |
| /jaxrs/portal/assemble/designer/list | GET | list_designs | 桩代码 | mock 数据 |
| /jaxrs/portal/assemble/designer/save/{id} | POST | save_design | 桩代码 | mock 数据 |

### portal_assemble_surface

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/portal/assemble/surface/get/{id} | GET | get_surface | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/assemble/surface/create | POST | create_surface | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/assemble/surface/list/{category} | GET | list_surfaces | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/assemble/surface/save/{id} | POST | save_surface | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/assemble/surface/preview/{id} | GET | preview_surface | 桩代码 | mock 数据 |
| /jaxrs/portal/assemble/surface/publish/{id} | POST | publish_surface | 桩代码 | mock 数据 |
| /jaxrs/portal/assemble/surface/delete/{id} | POST | delete_surface | 部分完成 | 真实 DB 查询 |

### portal_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/portal/core/entity/page/list | GET | page_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/core/entity/page/{id} | GET | get_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/core/entity/page/create | POST | create_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/core/entity/page/save/{id} | POST | save_page | 部分完成 | 真实 DB 查询 |
| /jaxrs/portal/core/entity/page/delete/{id} | POST | delete_page | 部分完成 | 真实 DB 查询 |

### process_designer

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/process/application/list/summary | GET | application_list_summary | 部分完成 | 真实 DB 查询 |
| /jaxrs/process/designer/route/{id} | GET | designer_get_route | 部分完成 | 真实 DB 查询 |

### process_express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/process/task/count/{credential} | GET | task_count | 部分完成 | 真实 DB 查询 |
| /jaxrs/process/read/count/{credential} | GET | read_count | 部分完成 | 真实 DB 查询 |
| /jaxrs/process/application/list | GET | application_list | 部分完成 | 真实 DB 查询 |

### process_bam

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/process/state/summary | GET | state_summary | 部分完成 | 真实 DB 查询 |
| /jaxrs/process/state/running | GET | state_running | 部分完成 | 真实 DB 查询 |
| /jaxrs/process/state/organization | GET | state_organization | 部分完成 | 真实 DB 查询 |

### process_surface

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/process/list/ids | GET | list_ids | 部分完成 | 真实 DB 查询 |
| /jaxrs/process/{flag} | GET | get_by_flag | 部分完成 | 真实 DB 查询 |
| /jaxrs/process/record/list/workorworkcompleted/{workOrWorkCompleted} | GET | record_list | 部分完成 | 真实 DB 查询 |

### processplatform_assemble_bam

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/processplatform/assemble/bam/get/{id} | GET | get_bam_config | 部分完成 | mock 数据 |
| /jaxrs/processplatform/assemble/bam/create | POST | create_bam | 部分完成 | mock 数据 |
| /jaxrs/processplatform/assemble/bam/list/{category} | GET | list_bams | 部分完成 | mock 数据 |
| /jaxrs/processplatform/assemble/bam/delete/{id} | POST | delete_bam | 部分完成 | mock 数据 |
| /jaxrs/processplatform/assemble/bam/status/{id} | GET | get_bam_status | 部分完成 | mock 数据 |
| ...（共 20+ 个统计路由，均为 stub） | | | | |

### processplatform_assemble_designer

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/processplatform/assemble/designer/create | POST | create_flow | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/assemble/designer/get/{id} | GET | get_flow | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/assemble/designer/list/{category} | GET | list_flows | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/assemble/designer/save/{id} | POST | save_flow | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/assemble/designer/delete/{id} | POST | delete_flow | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/assemble/designer/preview/{id} | GET | preview_flow | 桩代码 | mock 数据 |
| ...（共 20+ 个应用路由，均为 stub） | | | | |

### processplatform_assemble_surface

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/processplatform/assemble/surface/get/{id} | GET | get_surface | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/assemble/surface/create | POST | create_surface | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/assemble/surface/list/{category} | GET | list_surfaces | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/assemble/surface/save/{id} | POST | save_surface | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/assemble/surface/preview/{id} | GET | preview_surface | 桩代码 | mock 数据 |
| /jaxrs/processplatform/assemble/surface/publish/{id} | POST | publish_surface | 桩代码 | mock 数据 |
| /jaxrs/processplatform/assemble/surface/delete/{id} | POST | delete_surface | 部分完成 | 真实 DB 查询 |
| ...（共 20+ 个匿名/应用路由，均为 stub） | | | | |

### processplatform_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/processplatform/work/list | GET | work_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/task/list | GET | task_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/workcompleted/list | GET | work_completed_list | 桩代码 | mock 数据 |
| /jaxrs/processplatform/ticket/list | GET | ticket_list | 部分完成 | 真实 DB 查询 |

### processplatform_core_express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/processplatform/work/terminate/{id} | GET | work_terminate | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/work/retract/{id} | GET | work_retract | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/work/processing/{id} | GET | work_processing | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/task/processing/{id} | GET | task_processing | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/work/count/with/person/{id} | GET | work_count_with_person | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/task/count/with/person/{id} | GET | task_count_with_person | 部分完成 | 真实 DB 查询 |

### processplatform_service_processing

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/processplatform/service/processing/get/{id} | GET | get_process | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/service/processing/create | POST | create_process | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/service/processing/list/{category} | GET | list_processes | 桩代码 | mock 数据 |
| /jaxrs/processplatform/service/processing/execute/{id} | POST | execute_process | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/service/processing/instance/{executionId} | GET | get_process_instance | 部分完成 | 真实 DB 查询 |
| /jaxrs/processplatform/service/processing/cancel/{executionId} | POST | cancel_process_instance | 部分完成 | 真实 DB 查询 |
| ...（共 30+ 个应用字典路由，均为 stub） | | | | |

### cms_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/cms/assemble/control/config/get | GET | get_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/cms/assemble/control/sections | GET | list_control_sections | 部分完成 | 真实 DB 查询 |
| /jaxrs/cms/assemble/control/config/update | POST | update_control_config | 部分完成 | 真实 DB 查询 |

### cms_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/cms_control/get/control/config | GET | get_control_config | 已完成 | 真实 DB 查询 |
| /jaxrs/cms_control/list/control/sections | GET | list_control_sections | 已完成 | 真实 DB 查询 |

### cms_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/cms/category/list | GET | category_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/cms/app/list | GET | app_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/cms/app/config/list/{appId} | GET | app_config_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/cms/category/ext/list/{categoryId} | GET | category_ext_list | 部分完成 | 真实 DB 查询 |

### cms_core_express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/cms/core/express/article/list | GET | article_list | 桩代码 | 待实现 |
| /jaxrs/cms/core/express/article/{id} | GET | article_get | 桩代码 | 待实现 |
| /jaxrs/cms/core/express/article/create | POST | article_create | 桩代码 | 待实现 |
| /jaxrs/cms/core/express/article/update/{id} | POST | article_update | 桩代码 | 待实现 |
| /jaxrs/cms/core/express/article/delete/{id} | POST | article_delete | 桩代码 | 待实现 |

### cms_express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/cms/uuid/random | GET | uuid_random | 桩代码 | mock 数据 |
| /jaxrs/cms/templateform/list | GET | template_form_list | 桩代码 | mock 数据 |
| /jaxrs/cms/view/list/all | GET | view_list_all | 桩代码 | mock 数据 |

### query_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/query/item/list | GET | item_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/query/item/access/list/{itemId} | GET | item_access_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/query/view/list | GET | view_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/query/import/model/list | GET | import_model_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/query/import/record/list | GET | import_record_list | 部分完成 | 真实 DB 查询 |

### query_core_express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/query/core/express/query/list | GET | query_list | 桩代码 | mock 数据 |

### query_express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/query/list | GET | query_list | 桩代码 | mock 数据 |

### query_service

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/query/service/neural/generate/{model_flag} | POST | neural_generate_model | 已完成 | 真实 DB 查询 |
| /jaxrs/query/service/neural/list | GET | neural_list_model | 已完成 | 真实 DB 查询 |

### query_assemble_designer

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/query/assemble/designer/query/list | GET | query_list | 桩代码 | 待实现 |
| /jaxrs/query/assemble/designer/query/{id} | GET | query_get | 桩代码 | 待实现 |
| /jaxrs/query/assemble/designer/query/create | POST | query_create | 桩代码 | 待实现 |
| /jaxrs/query/assemble/designer/query/save/{id} | POST | query_save | 桩代码 | 待实现 |
| /jaxrs/query/assemble/designer/query/delete/{id} | POST | query_delete | 桩代码 | 待实现 |

### query_assemble_surface

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/query/assemble/surface/view/list | GET | view_list | 桩代码 | 待实现 |
| /jaxrs/query/assemble/surface/view/{id} | GET | view_get | 桩代码 | 待实现 |
| /jaxrs/query/assemble/surface/view/create | POST | view_create | 桩代码 | 待实现 |
| /jaxrs/query/assemble/surface/view/save/{id} | POST | view_save | 桩代码 | 待实现 |
| /jaxrs/query/assemble/surface/view/delete/{id} | POST | view_delete | 桩代码 | 待实现 |

### query_service_processing

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/query/service/processing/execute | POST | execute_query | 桩代码 | 待实现 |
| /jaxrs/query/service/processing/result/{flag} | GET | get_result | 桩代码 | 待实现 |
| /jaxrs/query/service/processing/export/{flag} | GET | export_result | 桩代码 | 待实现 |

---

## Wave 4 — 低优先级（U7）

### ai

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/ai/config/get | GET | config_get | 部分完成 | mock 数据 |
| /jaxrs/ai/config/list/enable/model | GET | list_enable_model | 部分完成 | mock 数据 |
| /jaxrs/ai/index/sync/to/knowledge | GET | sync_to_knowledge | 部分完成 | mock 数据 |
| /jaxrs/ai/app/list | GET | app_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/ai/model/list | GET | model_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/ai/conversation/list | GET | conversation_list | 部分完成 | 真实 DB 查询 |

### ai_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/ai/assemble/control/config/get | GET | get_ai_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/ai/assemble/control/models | GET | list_ai_models | 部分完成 | 真实 DB 查询 |
| /jaxrs/ai/assemble/control/config/update | GET | update_ai_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/ai/assemble/control/usage/stats | GET | get_usage_stats | 部分完成 | 真实 DB 查询 |
| ...（共 30+ 个 MCP/文件/索引路由，均为 stub） | | | | |

### ai_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/ai/core/entity/app/list | GET | app_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/ai/core/entity/model/list | GET | model_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/ai/core/entity/conversation/list | GET | conversation_list | 部分完成 | 真实 DB 查询 |

### base

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/base/echo/get | GET | echo_get | 已完成 | 固定 pong 响应 |
| /jaxrs/base/cache/detail | GET | cache_detail | 部分完成 | 真实 DB 查询（pg_class） |
| /jaxrs/base/openapi/info | GET | openapi_info | 已完成 | 固定 OpenAPI 信息 |

### bbs

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/bbs/forum/view/all | GET | forum::view_all | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/forum/view/{id} | GET | forum::view_one | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/section/viewforum/{forumId} | GET | section::view_forum | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/section/view/all | GET | section::view_all | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/subject/top/{sectionId} | GET | subject::top | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/subject/list/{sectionId} | GET | subject::list | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/subject/view/{id} | GET | subject::view | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/subject/create | POST | subject::create | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/subject/search | GET | subject::search | 部分完成 | 真实 DB 查询 |

### bbs_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/bbs/assemble/control/config/get | GET | get_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/assemble/control/sections | GET | list_control_sections | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/assemble/control/config/update | GET | update_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/assemble/control/forum/list | GET | list_forums | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/assemble/control/forum/{id} | GET | get_forum | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/assemble/control/topic/create | POST | create_topic | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/assemble/control/topic/list/{forumId} | GET | list_topics_by_forum | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/assemble/control/reply/create | POST | create_reply | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/assemble/control/reply/list/sub/{id} | GET | stub_... | 桩代码 | 待实现 |
| /jaxrs/bbs/assemble/control/subject/view/{id} | GET | stub_... | 桩代码 | 待实现 |
| /jaxrs/bbs/assemble/control/subject/top/{sectionId} | GET | stub_... | 桩代码 | 待实现 |
| /jaxrs/bbs/assemble/control/permission/section/{sectionId} | GET | stub_... | 桩代码 | 待实现 |
| /jaxrs/bbs/assemble/control/permission/subject/{subjectId} | GET | stub_... | 桩代码 | 待实现 |
| /jaxrs/bbs/assemble/control/section/viewforum/{forumId} | GET | stub_... | 桩代码 | 待实现 |

### bbs_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/bbs/core/entity/forum/list | GET | forum_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/forum/{id} | GET | forum_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/forum/create | POST | create_forum | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/forum/update/{id} | POST | update_forum | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/forum/delete/{id} | POST | delete_forum | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/section/list/{forumId} | GET | section_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/section/create | POST | create_section | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/section/update/{id} | POST | update_section | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/section/delete/{id} | POST | delete_section | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/subject/top/{sectionId} | GET | subject_top_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/subject/list/{sectionId} | GET | subject_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/bbs/core/entity/subject/create | POST | create_subject | 部分完成 | 真实 DB 查询 |

### component

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/component/list/all | GET | list_all | 部分完成 | 真实 DB 查询 |
| /jaxrs/component/count | GET | count | 部分完成 | 真实 DB 查询 |
| /jaxrs/component/{flag} | GET | get_component | 部分完成 | 真实 DB 查询 |

### component_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/component/assemble/control/config/get | GET | get_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/component/assemble/control/categories | GET | list_control_categories | 部分完成 | 真实 DB 查询 |
| /jaxrs/component/assemble/control/config/update | GET | update_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/component/assemble/control/component/list | GET | list_components | 部分完成 | 真实 DB 查询 |
| /jaxrs/component/assemble/control/component/get/{id} | GET | get_component | 部分完成 | 真实 DB 查询 |
| /jaxrs/component/assemble/control/component/create | POST | create_component | 部分完成 | 真实 DB 查询 |
| /jaxrs/component/assemble/control/component/save/{id} | POST | save_component | 部分完成 | 真实 DB 查询 |
| /jaxrs/component/assemble/control/component/delete/{id} | POST | delete_component | 部分完成 | 真实 DB 查询 |

### component_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/component/core/entity/list/all | GET | component_list_all | 桩代码 | mock 数据 |
| /jaxrs/component/core/entity/{flag} | GET | component_get | 桩代码 | mock 数据 |
| /jaxrs/component/core/entity/count | GET | component_count | 桩代码 | mock 数据 |

### console

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/console/status | GET | get_status | 桩代码 | mock 数据 |
| /jaxrs/console/logs/{type} | GET | get_logs | 桩代码 | mock 数据 |
| /jaxrs/console/send/message | POST | send_message | 桩代码 | mock 数据 |
| /jaxrs/console/cache/clear/{type} | POST | clear_cache | 桩代码 | mock 数据 |
| /jaxrs/console/metric/{name} | GET | get_metric | 桩代码 | mock 数据 |
| /jaxrs/console/command/execute | POST | execute_command | 桩代码 | mock 数据 |
| /jaxrs/console/system/info | GET | get_system_info | 桩代码 | mock 数据 |

### correlation

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/correlation/type/cms/list | GET | list_cms_correlations | 部分完成 | 真实 DB 查询 |
| /jaxrs/correlation/type/processplatform/list | GET | list_process_platform_correlations | 部分完成 | 真实 DB 查询 |
| /jaxrs/correlation/type/cms/readable | GET | check_cms_readable | 部分完成 | 真实 DB 查询 |

### correlation_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/correlation/core/entity/list | GET | list | 部分完成 | 真实 DB 查询 |
| /jaxrs/correlation/core/entity/list/by/{sourceType}/{sourceId} | GET | list_by_source | 部分完成 | 真实 DB 查询 |

### correlation_core_express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/correlation/core/express/status | GET | get_status | 部分完成 | 真实 DB 查询 |
| /jaxrs/correlation/core/express/sync | GET | sync_correlation | 部分完成 | 真实 DB 查询 |

### correlation_service_processing

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/correlation/service/processing/list/{personId} | GET | list_correlations | 部分完成 | 真实 DB 查询 |
| /jaxrs/correlation/service/processing/{id} | GET | get_correlation | 部分完成 | 真实 DB 查询 |
| /jaxrs/correlation/service/processing/create | POST | create_correlation | 部分完成 | 真实 DB 查询 |
| /jaxrs/correlation/service/processing/save/{id} | POST | save_correlation | 部分完成 | 真实 DB 查询 |
| /jaxrs/correlation/service/processing/delete/{id} | POST | delete_correlation | 部分完成 | 真实 DB 查询 |
| /jaxrs/correlation/service/processing/link/{sourceType}/{sourceId} | GET | get_link | 桩代码 | mock 数据 |
| /jaxrs/correlation/service/processing/link | POST | link_service | 桩代码 | mock 数据 |
| /jaxrs/correlation/service/processing/unlink/{sourceType}/{sourceId}/{targetType}/{targetId} | POST | unlink_service | 桩代码 | mock 数据 |
| ...（共 10+ 个 CMS/流程关联路由，均为 stub） | | | | |

### express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/express/query | GET | get_express_info | 桩代码 | mock 数据（硬编码快递信息） |
| /jaxrs/express/companies | GET | list_express_companies | 桩代码 | mock 数据 |
| /jaxrs/express/subscribe | POST | subscribe_express | 桩代码 | mock 数据 |

### general

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/general/area/list | GET | area_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/general/securityclearance/enable | GET | security_clearance_enable | 桩代码 | mock 数据 |
| /jaxrs/general/worktime/isworkday/{date} | GET | is_workday | 桩代码 | mock 数据 |

### hotpic

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/hotpic/user/hotpic/exists/check | GET | exists_check | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/user/hotpic/{id} | GET | get_by_id | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/user/hotpic/{application}/{infoId} | GET | list_by_application_and_info_id | 部分完成 | 真实 DB 查询 |

### hotpic_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/hotpic/assemble/control/config/get | GET | get_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/assemble/control/panels | GET | list_control_panels | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/assemble/control/config/update | GET | update_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/assemble/control/applications | GET | list_control_applications | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/assemble/control/hotpic/list | GET | list_hotpics | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/assemble/control/hotpic/get/{id} | GET | get_hotpic | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/assemble/control/hotpic/create | POST | create_hotpic | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/assemble/control/hotpic/save/{id} | POST | save_hotpic | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/assemble/control/hotpic/delete/{id} | POST | delete_hotpic | 部分完成 | 真实 DB 查询 |
| ...（共 10+ 个用户轮播图路由，均为 stub） | | | | |

### hotpic_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/hotpic/core/entity/list | GET | list | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/core/entity/list/by/{application}/{infoId} | GET | list_by_app_and_info | 部分完成 | 真实 DB 查询 |
| /jaxrs/hotpic/core/entity/exists/check/{application}/{infoId} | GET | exists_check | 部分完成 | 真实 DB 查询 |

### jpush

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/jpush/device/list | GET | device_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/device/{id} | GET | device_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/device/create | POST | device_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/template/list | GET | template_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/template/{id} | GET | template_get | 部分完成 | 真实 DB 查询 |

### jpush_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/jpush/assemble/control/config/get | GET | get_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/assemble/control/apps | GET | list_control_apps | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/assemble/control/config/update | GET | update_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/assemble/control/message/list | GET | list_jpushs | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/assemble/control/message/get/{id} | GET | get_jpush | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/assemble/control/message/send | POST | create_jpush | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/assemble/control/message/save/{id} | POST | save_jpush | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/assemble/control/message/delete/{id} | POST | delete_jpush | 部分完成 | 真实 DB 查询 |
| ...（共 8 个设备管理路由，均为 stub） | | | | |

### jpush_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/jpush/core/entity/device/list | GET | device_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/core/entity/device/{id} | GET | device_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/core/entity/device/create | POST | device_create | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/core/entity/template/list | GET | template_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/jpush/core/entity/template/{id} | GET | template_get | 部分完成 | 真实 DB 查询 |

### mind

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/mind/mind/{id} | GET | get_mind_with_id | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/mind | POST | create_mind | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/mind/{id} | POST | update_mind | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/mind/{id} | DELETE | delete_mind | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/folder/tree/my | GET | list_my_folders | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/folder | POST | create_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/folder/{id} | POST | update_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/folder/{id} | DELETE | delete_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/mind/list/{id}/version | GET | list_versions_with_mind_id | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/version | POST | create_version | 部分完成 | 真实 DB 查询 |

### mind_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/mind/assemble/control/config | GET | get_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/assemble/control/config/update | POST | update_control_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/assemble/control/folder/tree/my | GET | list_folders | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/assemble/control/folder/{id} | GET | get_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/assemble/control/folder/save | POST | save_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/assemble/control/folder/{id}/update | POST | update_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/assemble/control/folder/move/{folderId} | POST | stub_mind_assemble_control_folder_move_folderId | 桩代码 | 待实现 |
| /jaxrs/mind/assemble/control/folder/{id}/force | POST | stub_mind_assemble_control_folder_id_force | 桩代码 | 待实现 |

### mind_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/mind/core/entity/mind/list | GET | list | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/mind/{id} | GET | get_mind | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/mind/create | POST | create_mind | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/mind/update/{id} | POST | update_mind | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/mind/delete/{id} | POST | delete_mind | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/folder/list | GET | folder_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/folder/create | POST | create_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/folder/update/{id} | POST | update_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/folder/delete/{id} | POST | delete_folder | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/version/list/{mindId} | GET | version_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/mind/core/entity/version/create | POST | create_version | 部分完成 | 真实 DB 查询 |

### organization_assemble_control

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/organization/assemble/control/role/list/{flag}/next/{count} | GET | organization_assemble_control_role_list_flag_next_count | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/assemble/control/role/{flag} | GET | organization_assemble_control_role_flag | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/assemble/control/unit/list/{flag}/next/{count} | GET | organization_assemble_control_unit_list_flag_next_count | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/assemble/control/unit/{flag} | GET | organization_assemble_control_unit_flag | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/assemble/control/person/list/like | POST | organization_assemble_control_person_list_like | 部分完成 | 真实 DB 查询 |
| ...（共 20+ 个导出/统计路由，均为 stub） | | | | |

### organization_assemble_express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/organization/assemble/express/config/get | GET | get_express_config | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/assemble/express/units | GET | list_organization_units | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/assemble/express/sync | GET | sync_organization_data | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/assemble/express/status | GET | get_express_status | 部分完成 | 真实 DB 查询 |

### organization_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/organization/definition/list | GET | definition_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/group/list | GET | group_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/identity/list | GET | identity_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/person/list | GET | person_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/custom/list/{identityId} | GET | custom_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/bind/list | GET | bind_list | 部分完成 | 真实 DB 查询 |

### organization_core_express

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/organization/core/express/status | GET | get_status | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/core/express/sync | GET | sync_organization | 部分完成 | 真实 DB 查询 |
| /jaxrs/organization/core/express/config | GET | get_config | 部分完成 | 真实 DB 查询 |

### program_center

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/program/applications | GET | applications | 部分完成 | 真实 DB 查询 |
| /jaxrs/program/appstyle/current/style | GET | current_style | 部分完成 | 真实 DB 查询 |
| /jaxrs/program/datastructure/modules/all | GET | modules_all | 桩代码 | mock 数据 |
| /jaxrs/program_center/collect/list | GET | collect_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/program_center/collect/add | POST | collect_add | 部分完成 | 真实 DB 查询 |
| /jaxrs/program_center/collect/remove/{id} | POST | collect_remove | 部分完成 | 真实 DB 查询 |
| /jaxrs/program_center/config/get/{key} | GET | config_get | 部分完成 | 真实 DB 查询 |
| /jaxrs/program_center/config/save | POST | config_save | 部分完成 | 真实 DB 查询 |

### program_center_core_entity

| 路由路径 | HTTP 方法 | Handler 函数名 | 实现状态 | 备注 |
|----------|-----------|----------------|----------|------|
| /jaxrs/program_center/application/list | GET | application_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/program_center/script/list | GET | script_list | 部分完成 | 真实 DB 查询 |
| /jaxrs/program_center/invoke/list | GET | invoke_list | 桩代码 | mock 数据 |
| /jaxrs/program_center/agent/list | GET | agent_list | 桩代码 | mock 数据 |
| /jaxrs/program_center/structure/list | GET | structure_list | 桩代码 | mock 数据 |

---

## 附录：未完整枚举的 crate

以下 crate 的 `routes.rs` 仅为转发，实际路由定义在 `lib.rs` 中，且 `lib.rs` 文件过大（>1000 行），此处仅列出主要路由前缀：

| Crate | 路由前缀 | 实现状态 |
|-------|----------|----------|
| query_assemble_designer | /jaxrs/query/assemble/designer/* | 桩代码 |
| query_assemble_surface | /jaxrs/query/assemble/surface/* | 桩代码 |
| query_service_processing | /jaxrs/query/service/processing/* | 桩代码 |
| processplatform_assemble_surface | /jaxrs/processplatform/assemble/surface/* | 部分完成（大量 stub） |
| processplatform_assemble_designer | /jaxrs/processplatform/assemble/designer/* | 部分完成（大量 stub） |
| processplatform_assemble_bam | /jaxrs/processplatform/assemble/bam/* | 部分完成（大量 stub） |
| processplatform_service_processing | /jaxrs/processplatform/service/processing/* | 部分完成（大量 stub） |
| message_assemble_communicate | /jaxrs/message/assemble/communicate/* | 部分完成（大量 stub） |
| portal_assemble_designer | /jaxrs/portal/assemble/designer/* | 部分完成（大量 stub） |
| correlation_service_processing | /jaxrs/correlation/service/processing/* | 部分完成（大量 stub） |
| program_center | /jaxrs/program_center/* | 部分完成 |
| organization_assemble_control | /jaxrs/organization/assemble/control/* | 部分完成（大量 stub） |

## 实施建议

1. **Wave 1**：auth、control、personal、personal_extend、program_init、message 已完成真实业务逻辑，优先作为参考基准（R53）。
2. **Wave 2**：attendance、calendar、file、general_assemble_control 及其 _core_entity/_assemble_control 已有部分真实查询，优先补全 CRUD 和业务编排逻辑。
3. **Wave 3**：meeting_*、portal_*、process_*、query_*、cms_* 为核心用户工作流，按业务频率分优先级实施（message_* 已完成）。
4. **Wave 4**：基础设施模块允许简化或保留接口 stub，优先保证 CRUD 和查询接口对齐。
