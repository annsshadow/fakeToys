# o2server

## Responsibility

思维导图核心实体模块，定义导图、文件夹和版本数据模型。

## Core Classes and Interfaces

- com.x.mind.entity.MindBaseInfo
- com.x.mind.entity.MindBaseInfo_
- com.x.mind.entity.MindContentInfo
- com.x.mind.entity.MindContentInfo_
- com.x.mind.entity.MindFolderInfo
- com.x.mind.entity.MindFolderInfo_
- com.x.mind.entity.MindIconInfo
- com.x.mind.entity.MindIconInfo_
- com.x.mind.entity.MindRecycleInfo
- com.x.mind.entity.MindRecycleInfo_

## Key Flows

- 导图列表：`GET /jaxrs/mind/core/entity/list` → `list` 查 mind_mind，CreateTime 倒序 limit 20，可选字段经 `build_json_object` 为 None 时省略键
- 文件夹列表：`GET .../folder/list` 查 mind_folder，OrderNumber 升序 limit 20
- 版本列表：`GET .../version/list/{mindId}` 过滤 MindId，CreateTime 倒序 limit 20，输出含 fileVersion/createTime
- 导图 CRUD：`POST .../mind` 创建（creator 默认 "system"、create_time=Set(None)）；`POST .../mind/{id}` 更新（find_by_id 无则 AppError::NotFound，缺省字段回退原值）；`DELETE .../mind/{id}` 物理删除，rows_affected==0 时 error("mind not found")
- 文件夹 CRUD：`POST .../folder` 创建（parentId 键名为带引号的 `"\"parentId\""`、orderNumber 默认 0、creator 默认 "system"）；`POST .../folder/{id}` 更新；`DELETE .../folder/{id}` rows_affected==0 时 error("folder not found")
- 版本创建：`POST .../version` → fileVersion 默认 1、shared 默认 false、creatorUnit 缺省空串
- 路由注册：`mind_core_entity_router(_pool)` 挂 list 3 条 + mind 3 条 + folder 3 条 + version 1 条共 10 条路由

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/mind_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
