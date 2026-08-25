# o2server

## Responsibility

通用核心实体模块，定义字典、文件、发票等通用数据模型。

## Core Classes and Interfaces

- com.x.general.core.entity.ApplicationDict
- com.x.general.core.entity.ApplicationDictItem
- com.x.general.core.entity.ApplicationDictItem_
- com.x.general.core.entity.ApplicationDict_
- com.x.general.core.entity.GeneralFile
- com.x.general.core.entity.GeneralFile_
- com.x.general.core.entity.Invoice
- com.x.general.core.entity.InvoiceDetail
- com.x.general.core.entity.InvoiceProperties
- com.x.general.core.entity.Invoice_

## Key Flows

- 字典列表与创建：`GET /jaxrs/general/dict/list` Name 升序 limit 20；`POST .../dict/create` uuid v4 主键
- 字典查询/更新/删除：`GET .../dict/{id}` 无则 error("dict not found")；`POST .../dict/update/{id}` 覆盖 name/application；`POST .../dict/delete/{id}` 物理删除 delete_by_id
- 字典项：`GET .../dict/item/list/{dictId}` 过滤 DictId；item 的 create/get/update/delete 同 CRUD 模式，缺失时 error("dict item not found")
- 通用文件：`GET .../file/list` CreateTime 倒序 limit 20；`POST .../file/create` size 默认 0、creator 默认 "system"、create_time=NotSet；`GET .../file/download/{id}` 返回文件元数据而非二进制流
- 发票：`GET .../invoice/list` CreateTime 倒序 limit 20，amount 经 Number::from_f64 失败回退 0；`POST .../invoice/create` status 默认 "draft"、creator 默认 "system"；get/update/delete 缺失时 error("invoice not found")
- 路由注册：`general_core_entity_router(_pool)` 挂 dict 10 条 + file 6 条 + invoice 5 条共 21 条路由

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/general_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
