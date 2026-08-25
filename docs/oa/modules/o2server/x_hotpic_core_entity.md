# o2server

## Responsibility

热点图片核心实体模块，定义热点图片数据模型。

## Core Classes and Interfaces

- com.x.hotpic.entity.HotPictureInfo
- com.x.hotpic.entity.HotPictureInfo_
- com.x.hotpic.entity.PersistenceProperties

## Key Flows

- 热图列表：`GET /jaxrs/hotpic/core/entity/list` → `list` 查 `x_hotpic`，CreateTime 倒序 limit 20，输出 id/application/infoId/title/base64（NULL 回退空串）
- 按应用与信息查询：`GET .../list/by/{application}/{infoId}` → 过滤 Application+InfoId，CreateTime 倒序 limit 20
- 存在性检查：`GET .../exists/check/{application}/{infoId}` → `exists_check` 用 PaginatorTrait count，输出 allExists=count>0 与 count
- 创建热图：`POST .../create` → uuid v4、base64 可选、create_time=Utc now、deleted_at 初始 None
- 删除热图：`DELETE .../delete/{id}` → find_by_id 无则 error("hotpic not found")；软删置 deleted_at=Utc now
- 路由注册：`hotpic_core_entity_router(_pool)` 挂 list/by-source/exists/create/delete 共 5 条路由

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/hotpic_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
