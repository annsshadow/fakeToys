# o2server

## Responsibility

组织控制模块，提供人员、单位、角色、用户组的完整 CRUD 业务编排和权限管理。

## Core Classes and Interfaces

- com.x.organization.assemble.control.AbstractFactory
- com.x.organization.assemble.control.ApplicationServletContextListener
- com.x.organization.assemble.control.Business
- com.x.organization.assemble.control.ExceptionRoleFactory
- com.x.organization.assemble.control.MappingItem
- com.x.organization.assemble.control.MappingItemValueType
- com.x.organization.assemble.control.ThisApplication
- com.x.organization.assemble.control.factory.GroupFactory
- com.x.organization.assemble.control.factory.IdentityFactory
- com.x.organization.assemble.control.factory.PermissionSettingFactory

## Key Flows

- 组织实体软删：identity/person/unit/role/group/permissionsetting 的 `{flag}/mockdeletetoget` 族 → `UPDATE x_org_* SET deleted_at = NOW()`（软删除）→ 返回 ActionResult 布尔结果
- 用户组成员管理：`GET .../group/{flag}/add/member` → INSERT INTO `x_org_group_member`（ON CONFLICT DO NOTHING）；delete member → DELETE FROM `x_org_group_member`
- 检索与导出：group/identity/role/unitduty 的 like/pinyin/paging 列表 → ILIKE/分页查询 `x_org_identity`/`x_org_duty` 等；`GET .../export/export/all` → INSERT INTO `x_org_export`（pending）登记导出任务

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_general_core_entity

**Rust（oa4rust/crates/organization_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、deadpool-postgres、tokio

## REST Endpoints



- `GET /api/identity/{id}`
- `GET /api/organization/assemble/control/export/export/all`
- `GET /api/organization/assemble/control/export/result/flag/{flag}`
- `GET /api/organization/assemble/control/export/zhengwudingding/person`
- `GET /api/organization/assemble/control/group/list/like`
- `GET /api/organization/assemble/control/group/list/like/mockputtopost`
- `GET /api/organization/assemble/control/group/list/like/pinyin`
- `GET /api/organization/assemble/control/group/list/like/pinyin/mockputtopost`
- `GET /api/organization/assemble/control/group/list/person/{personFlag}/sup/direct`
- `GET /api/organization/assemble/control/group/list/person/{personFlag}/sup/nested`
- `GET /api/organization/assemble/control/group/list/pinyininitial`
- `GET /api/organization/assemble/control/group/list/pinyininitial/mockputtopost`
- `GET /api/organization/assemble/control/group/list/role/{roleFlag}`
- `GET /api/organization/assemble/control/group/list/{flag}/next/{count}`
- `GET /api/organization/assemble/control/group/list/{flag}/prev/{count}`
- `GET /api/organization/assemble/control/group/list/{flag}/sub/direct`
- `GET /api/organization/assemble/control/group/list/{flag}/sub/nested`
- `GET /api/organization/assemble/control/group/list/{flag}/sup/direct`
- `GET /api/organization/assemble/control/group/list/{flag}/sup/nested`
- `GET /api/organization/assemble/control/group/{flag}`
- `GET /api/organization/assemble/control/group/{flag}/add/member`
- `GET /api/organization/assemble/control/group/{flag}/add/member/mockputtopost`
- `GET /api/organization/assemble/control/group/{flag}/delete/member`
- `GET /api/organization/assemble/control/group/{flag}/delete/member/mockputtopost`
- `GET /api/organization/assemble/control/group/{flag}/mockdeletetoget`
- `GET /api/organization/assemble/control/group/{flag}/mockputtopost`
- `GET /api/organization/assemble/control/identity/list/like`
- `GET /api/organization/assemble/control/identity/list/like/mockputtopost`
- `GET /api/organization/assemble/control/identity/list/like/pinyin`
- `GET /api/organization/assemble/control/identity/list/like/pinyin/mockputtopost`
- `GET /api/organization/assemble/control/identity/list/person/{personFlag}`
- `GET /api/organization/assemble/control/identity/list/pinyininitial`
- `GET /api/organization/assemble/control/identity/list/pinyininitial/mockputtopost`
- `GET /api/organization/assemble/control/identity/list/unit/{unitFlag}`
- `GET /api/organization/assemble/control/identity/list/unitduty/name/{unitDutyName}`
- `GET /api/organization/assemble/control/identity/list/{flag}/next/{count}`
- `GET /api/organization/assemble/control/identity/list/{flag}/prev/{count}`
- `GET /api/organization/assemble/control/identity/list/{flag}/unitduty/name/{unitDutyName}`
- `GET /api/organization/assemble/control/identity/{flag}`
- `GET /api/organization/assemble/control/identity/{flag}/mockdeletetoget`
- `GET /api/organization/assemble/control/identity/{flag}/mockputtopost`
- `GET /api/organization/assemble/control/identity/{flag}/order/before/{followFlag}`
- `GET /api/organization/assemble/control/inputperson/result/flag/{flag}`
- `GET /api/organization/assemble/control/inputperson/template`
- `GET /api/organization/assemble/control/inputperson/wipe`
- `GET /api/organization/assemble/control/loginrecord/{stream}`
- `GET /api/organization/assemble/control/permissionsetting/list`
- `GET /api/organization/assemble/control/permissionsetting/{flag}`
- `GET /api/organization/assemble/control/permissionsetting/{flag}/mockdeletetoget`
- `GET /api/organization/assemble/control/permissionsetting/{flag}/mockputtopost`
- `POST /api/organization/assemble/control/person/list/like`
- `GET /api/organization/assemble/control/personattribute/list/person/{personFlag}`
- `GET /api/organization/assemble/control/personattribute/list/{flag}/next/{count}`
- `GET /api/organization/assemble/control/personattribute/list/{flag}/prev/{count}`
- `GET /api/organization/assemble/control/personattribute/{flag}`
- `GET /api/organization/assemble/control/personattribute/{flag}/mockdeletetoget`
- `GET /api/organization/assemble/control/personattribute/{flag}/mockputtopost`
- `GET /api/organization/assemble/control/personcard/createCode/{cardId}`
- `GET /api/organization/assemble/control/personcard/createQR/{cardId}`
- `GET /api/organization/assemble/control/personcard/listPersonalVCf/{idList}`
- `GET /api/organization/assemble/control/personcard/listVCf/{idList}`
- `GET /api/organization/assemble/control/personcard/listgrouptypes`
- `GET /api/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}`
- `GET /api/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}/mockputtopost`
- `GET /api/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}`
- `GET /api/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}/mockputtopost`
- `GET /api/organization/assemble/control/personcard/mylist`
- `GET /api/organization/assemble/control/personcard/{flag}`
- `GET /api/organization/assemble/control/personcard/{flag}/mockdeletetoget`
- `GET /api/organization/assemble/control/role/list/group/{groupFlag}`
- `GET /api/organization/assemble/control/role/list/like`
- `GET /api/organization/assemble/control/role/list/like/mockputtopost`
- `GET /api/organization/assemble/control/role/list/like/pinyin`
- `GET /api/organization/assemble/control/role/list/like/pinyin/mockputtopost`
- `GET /api/organization/assemble/control/role/list/person/{personFlag}`
- `GET /api/organization/assemble/control/role/list/pinyininitial`
- `GET /api/organization/assemble/control/role/list/pinyininitial/mockputtopost`
- `GET /api/organization/assemble/control/role/list/{flag}/next/{count}`
- `GET /api/organization/assemble/control/role/list/{flag}/prev/{count}`
- `GET /api/organization/assemble/control/role/{flag}`
- `GET /api/organization/assemble/control/role/{flag}/mockdeletetoget`
- `GET /api/organization/assemble/control/role/{flag}/mockputtopost`
- `GET /api/organization/assemble/control/unit/list/{flag}/next/{count}`
- `GET /api/organization/assemble/control/unit/list/{flag}/sub/nested`
- `GET /api/organization/assemble/control/unit/list/{flag}/sup/nested`
- `GET /api/organization/assemble/control/unit/list/{flag}/sup/nested/type/{type}`
- `GET /api/organization/assemble/control/unit/{flag}`
- `GET /api/organization/assemble/control/unitattribute/list/unit/{flag}`
- `GET /api/organization/assemble/control/unitattribute/list/{flag}/next/{count}`
- `GET /api/organization/assemble/control/unitattribute/list/{flag}/prev/{count}`
- `GET /api/organization/assemble/control/unitattribute/{flag}`
- `GET /api/organization/assemble/control/unitattribute/{flag}/mockdeletetoget`
- `GET /api/organization/assemble/control/unitattribute/{flag}/mockputtopost`
- `GET /api/organization/assemble/control/unitduty/distinct/name`
- `GET /api/organization/assemble/control/unitduty/distinct/name/like/{key}`
- `GET /api/organization/assemble/control/unitduty/list/identity/{identityFlag}`
- `GET /api/organization/assemble/control/unitduty/list/like`
- `GET /api/organization/assemble/control/unitduty/list/name/{name}`
- `GET /api/organization/assemble/control/unitduty/list/unit/{unitFlag}`
- `GET /api/organization/assemble/control/unitduty/list/{flag}/next/{count}`
- `GET /api/organization/assemble/control/unitduty/list/{flag}/prev/{count}`
- `GET /api/organization/assemble/control/unitduty/update/member`
- `GET /api/organization/assemble/control/unitduty/{flag}`
- `GET /api/organization/assemble/control/unitduty/{flag}/mockdeletetoget`
- `GET /api/organization/assemble/control/unitduty/{flag}/mockputtopost`
