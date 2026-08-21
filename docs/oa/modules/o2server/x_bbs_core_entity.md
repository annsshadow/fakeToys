# o2server

## Responsibility

论坛核心实体模块，定义论坛、版块、主题等 BBS 数据模型。

## Core Classes and Interfaces

- com.x.bbs.entity.BBSConfigSetting
- com.x.bbs.entity.BBSConfigSetting_
- com.x.bbs.entity.BBSForumInfo
- com.x.bbs.entity.BBSForumInfo_
- com.x.bbs.entity.BBSOperationRecord
- com.x.bbs.entity.BBSOperationRecord_
- com.x.bbs.entity.BBSPermissionInfo
- com.x.bbs.entity.BBSPermissionInfo_
- com.x.bbs.entity.BBSPermissionRole
- com.x.bbs.entity.BBSPermissionRole_

## Key Flows

- 论坛列表：`GET /jaxrs/bbs/core/entity/forum/list` → `forum_list` 查 `x_bbs_forum_info`（CreateTime 升序 limit 50），输出 id/name，description 存在时附加
- 版块列表：`GET .../section/list/{forumId}` → `section_list` 按 ForumId 过滤、OrderNumber 升序 limit 50，输出 id/name/forumId/sort
- 主题列表：`GET .../subject/top/{sectionId}` → `subject_top_list` 过滤 SectionId + IsTop=true + Disable=false，CreateTime 倒序 limit 20；`GET .../subject/list/{sectionId}` → `subject_list` 仅过滤 Disable=false，limit 50
- 论坛 CRUD：`POST /jaxrs/bbs/core/entity/forum` 创建（uuid v4、create_time=chrono Utc now、order_number=0）；`POST .../forum/{id}` 更新（name/description 缺省沿用旧值，不存在返回 error("forum not found")）；`DELETE .../forum/{id}` 先查后删
- 版块 CRUD：`create_section`（取 forumId/sort/description）、`update_section`（缺省沿用旧值）、`delete_section`
- 主题 CRUD：`create_subject`（replyCount/viewCount 初始 0、isTop/disable=false）、`update_subject`（可改 title/sectionId/isTop/disable）、`delete_subject`
- 回复：`POST .../reply` → `create_reply` 仅生成 uuid 并回显 topicId，未写库（源码注释：reply 表不在本实体迁移范围）
- 主题搜索：`GET .../subject/search?keyword=` → `search_subjects` 以 Title LIKE `%keyword%` 且 Disable=false 查询，CreateTime 倒序 limit 20
- 路由注册：`bbs_core_entity_router(_pool)` 挂 forum/section/subject/reply/search 共 16 条路由；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/bbs_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
