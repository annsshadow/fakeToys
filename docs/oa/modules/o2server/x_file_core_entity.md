# o2server

## Responsibility

文件核心实体模块，定义文件和文件夹数据模型及基础 CRUD。

## Core Classes and Interfaces

- com.x.file.core.entity.PersistenceProperties
- com.x.file.core.entity.open.File
- com.x.file.core.entity.open.FileConfig
- com.x.file.core.entity.open.FileConfigProperties
- com.x.file.core.entity.open.FileConfig_
- com.x.file.core.entity.open.FileStatus
- com.x.file.core.entity.open.FileType
- com.x.file.core.entity.open.File_
- com.x.file.core.entity.open.OriginFile
- com.x.file.core.entity.open.OriginFile_

## Key Flows

- 顶层文件夹：`GET /jaxrs/file/core/entity/folder/list/top` → `folder_list_top` 查 file_folder 过滤 Superior 为 ""/NULL 且 DeletedAt IS NULL，Name 升序 limit 50，输出 id/name/person/superior 及占位 attachmentCount/size/folderCount
- 子文件夹列表：`GET .../folder/list/{id}` → 按 Superior=id + DeletedAt IS NULL，Name 升序 limit 50
- 文件列表：`GET .../file/list` → `file_list` 查 file_file 过滤 DeletedAt IS NULL，Name 升序 limit 50，输出含 referenceType（键名为带引号的 `"\"referenceType\""`）/extension/length
- 复合视图：`GET .../complex/top` → `complex_top` 并行取顶层文件夹 limit 20 与文件 limit 20，输出 folderList+attachmentList
- 文件夹创建/删除：`POST .../folder` 校验 name/person 非空否则 error("name and person are required")；`DELETE .../folder/{id}` find_by_id 无则 NotFound，软删 deleted_at=Utc now
- 文件创建：`POST .../file` 校验 name/person/reference_type 非空，length 默认 0，uuid v4 主键
- 路由注册：`file_core_entity_router(_pool)` 挂 folder 4 条 + file 2 条 + complex 1 条共 7 条路由

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/file_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
