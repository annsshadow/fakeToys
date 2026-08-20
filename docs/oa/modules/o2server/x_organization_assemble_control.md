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

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_general_core_entity

## REST Endpoints



- `GET /jaxrs/identity/{id}`
- `GET /jaxrs/organization/assemble/control/export/export/all`
- `GET /jaxrs/organization/assemble/control/export/result/flag/{flag}`
- `GET /jaxrs/organization/assemble/control/export/zhengwudingding/person`
- `GET /jaxrs/organization/assemble/control/group/list/like`
- `GET /jaxrs/organization/assemble/control/group/list/like/mockputtopost`
- `GET /jaxrs/organization/assemble/control/group/list/like/pinyin`
- `GET /jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost`
- `GET /jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/direct`
- `GET /jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/nested`
- `GET /jaxrs/organization/assemble/control/group/list/pinyininitial`
- `GET /jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost`
- `GET /jaxrs/organization/assemble/control/group/list/role/{roleFlag}`
- `GET /jaxrs/organization/assemble/control/group/list/{flag}/next/{count}`
- `GET /jaxrs/organization/assemble/control/group/list/{flag}/prev/{count}`
- `GET /jaxrs/organization/assemble/control/group/list/{flag}/sub/direct`
- `GET /jaxrs/organization/assemble/control/group/list/{flag}/sub/nested`
- `GET /jaxrs/organization/assemble/control/group/list/{flag}/sup/direct`
- `GET /jaxrs/organization/assemble/control/group/list/{flag}/sup/nested`
- `GET /jaxrs/organization/assemble/control/group/{flag}`
- `GET /jaxrs/organization/assemble/control/group/{flag}/add/member`
- `GET /jaxrs/organization/assemble/control/group/{flag}/add/member/mockputtopost`
- `GET /jaxrs/organization/assemble/control/group/{flag}/delete/member`
- `GET /jaxrs/organization/assemble/control/group/{flag}/delete/member/mockputtopost`
- `GET /jaxrs/organization/assemble/control/group/{flag}/mockdeletetoget`
- `GET /jaxrs/organization/assemble/control/group/{flag}/mockputtopost`
- `GET /jaxrs/organization/assemble/control/identity/list/like`
- `GET /jaxrs/organization/assemble/control/identity/list/like/mockputtopost`
- `GET /jaxrs/organization/assemble/control/identity/list/like/pinyin`
- `GET /jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost`
- `GET /jaxrs/organization/assemble/control/identity/list/person/{personFlag}`
- `GET /jaxrs/organization/assemble/control/identity/list/pinyininitial`
- `GET /jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost`
- `GET /jaxrs/organization/assemble/control/identity/list/unit/{unitFlag}`
- `GET /jaxrs/organization/assemble/control/identity/list/unitduty/name/{unitDutyName}`
- `GET /jaxrs/organization/assemble/control/identity/list/{flag}/next/{count}`
- `GET /jaxrs/organization/assemble/control/identity/list/{flag}/prev/{count}`
- `GET /jaxrs/organization/assemble/control/identity/list/{flag}/unitduty/name/{unitDutyName}`
- `GET /jaxrs/organization/assemble/control/identity/{flag}`
- `GET /jaxrs/organization/assemble/control/identity/{flag}/mockdeletetoget`
- `GET /jaxrs/organization/assemble/control/identity/{flag}/mockputtopost`
- `GET /jaxrs/organization/assemble/control/identity/{flag}/order/before/{followFlag}`
- `GET /jaxrs/organization/assemble/control/inputperson/result/flag/{flag}`
- `GET /jaxrs/organization/assemble/control/inputperson/template`
- `GET /jaxrs/organization/assemble/control/inputperson/wipe`
- `GET /jaxrs/organization/assemble/control/loginrecord/{stream}`
- `GET /jaxrs/organization/assemble/control/permissionsetting/list`
- `GET /jaxrs/organization/assemble/control/permissionsetting/{flag}`
- `GET /jaxrs/organization/assemble/control/permissionsetting/{flag}/mockdeletetoget`
- `GET /jaxrs/organization/assemble/control/permissionsetting/{flag}/mockputtopost`
- `POST /jaxrs/organization/assemble/control/person/list/like`
- `GET /jaxrs/organization/assemble/control/personattribute/list/person/{personFlag}`
- `GET /jaxrs/organization/assemble/control/personattribute/list/{flag}/next/{count}`
- `GET /jaxrs/organization/assemble/control/personattribute/list/{flag}/prev/{count}`
- `GET /jaxrs/organization/assemble/control/personattribute/{flag}`
- `GET /jaxrs/organization/assemble/control/personattribute/{flag}/mockdeletetoget`
- `GET /jaxrs/organization/assemble/control/personattribute/{flag}/mockputtopost`
- `GET /jaxrs/organization/assemble/control/personcard/createCode/{cardId}`
- `GET /jaxrs/organization/assemble/control/personcard/createQR/{cardId}`
- `GET /jaxrs/organization/assemble/control/personcard/listPersonalVCf/{idList}`
- `GET /jaxrs/organization/assemble/control/personcard/listVCf/{idList}`
- `GET /jaxrs/organization/assemble/control/personcard/listgrouptypes`
- `GET /jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}`
- `GET /jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}/mockputtopost`
- `GET /jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}`
- `GET /jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}/mockputtopost`
- `GET /jaxrs/organization/assemble/control/personcard/mylist`
- `GET /jaxrs/organization/assemble/control/personcard/{flag}`
- `GET /jaxrs/organization/assemble/control/personcard/{flag}/mockdeletetoget`
- `GET /jaxrs/organization/assemble/control/role/list/group/{groupFlag}`
- `GET /jaxrs/organization/assemble/control/role/list/like`
- `GET /jaxrs/organization/assemble/control/role/list/like/mockputtopost`
- `GET /jaxrs/organization/assemble/control/role/list/like/pinyin`
- `GET /jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost`
- `GET /jaxrs/organization/assemble/control/role/list/person/{personFlag}`
- `GET /jaxrs/organization/assemble/control/role/list/pinyininitial`
- `GET /jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost`
- `GET /jaxrs/organization/assemble/control/role/list/{flag}/next/{count}`
- `GET /jaxrs/organization/assemble/control/role/list/{flag}/prev/{count}`
- `GET /jaxrs/organization/assemble/control/role/{flag}`
- `GET /jaxrs/organization/assemble/control/role/{flag}/mockdeletetoget`
- `GET /jaxrs/organization/assemble/control/role/{flag}/mockputtopost`
- `GET /jaxrs/organization/assemble/control/unit/list/{flag}/next/{count}`
- `GET /jaxrs/organization/assemble/control/unit/list/{flag}/sub/nested`
- `GET /jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested`
- `GET /jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested/type/{type}`
- `GET /jaxrs/organization/assemble/control/unit/{flag}`
- `GET /jaxrs/organization/assemble/control/unitattribute/list/unit/{flag}`
- `GET /jaxrs/organization/assemble/control/unitattribute/list/{flag}/next/{count}`
- `GET /jaxrs/organization/assemble/control/unitattribute/list/{flag}/prev/{count}`
- `GET /jaxrs/organization/assemble/control/unitattribute/{flag}`
- `GET /jaxrs/organization/assemble/control/unitattribute/{flag}/mockdeletetoget`
- `GET /jaxrs/organization/assemble/control/unitattribute/{flag}/mockputtopost`
- `GET /jaxrs/organization/assemble/control/unitduty/distinct/name`
- `GET /jaxrs/organization/assemble/control/unitduty/distinct/name/like/{key}`
- `GET /jaxrs/organization/assemble/control/unitduty/list/identity/{identityFlag}`
- `GET /jaxrs/organization/assemble/control/unitduty/list/like`
- `GET /jaxrs/organization/assemble/control/unitduty/list/name/{name}`
- `GET /jaxrs/organization/assemble/control/unitduty/list/unit/{unitFlag}`
- `GET /jaxrs/organization/assemble/control/unitduty/list/{flag}/next/{count}`
- `GET /jaxrs/organization/assemble/control/unitduty/list/{flag}/prev/{count}`
- `GET /jaxrs/organization/assemble/control/unitduty/update/member`
- `GET /jaxrs/organization/assemble/control/unitduty/{flag}`
- `GET /jaxrs/organization/assemble/control/unitduty/{flag}/mockdeletetoget`
- `GET /jaxrs/organization/assemble/control/unitduty/{flag}/mockputtopost`
