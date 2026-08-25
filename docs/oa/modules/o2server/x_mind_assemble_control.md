# o2server

## Responsibility

思维导图管控模块，处理导图配置和文件夹管理。

## Core Classes and Interfaces

- com.x.mind.assemble.common.date.DateOperation
- com.x.mind.assemble.control.AbstractFactory
- com.x.mind.assemble.control.ApplicationServletContextListener
- com.x.mind.assemble.control.Business
- com.x.mind.assemble.control.MessageFactory
- com.x.mind.assemble.control.ThisApplication
- com.x.mind.assemble.control.factory.MindBaseInfoFactory
- com.x.mind.assemble.control.factory.MindContentInfoFactory
- com.x.mind.assemble.control.factory.MindFolderInfoFactory
- com.x.mind.assemble.control.factory.MindIconInfoFactory

## Key Flows

- 文件夹树管理：`POST .../folder/save` INSERT `x_mind`（uuid 主键）；`GET .../folder/tree/my` 查 parent_id IS NULL AND deleted_at IS NULL；`folder/{id}/update` 按 payload 部分更新 name/content
- 移动与删除：`POST .../folder/move/{folderId}` 更新 parent_id（空值置 NULL 实现移到根）；`folder/{id}/force` 软删 deleted_at = NOW()
- 控制配置：`GET .../config` 读 `x_mind_assemble_control_config`，`config/update` 更新 config_data

## Dependencies



- x_base_core_project
- x_organization_core_express
- x_mind_core_entity
- x_general_core_entity

**Rust（oa4rust/crates/mind_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints



- `GET /jaxrs/mind/assemble/control/config`
- `POST /jaxrs/mind/assemble/control/config/update`
- `POST /jaxrs/mind/assemble/control/folder/move/{folderId}`
- `POST /jaxrs/mind/assemble/control/folder/save`
- `GET /jaxrs/mind/assemble/control/folder/tree/my`
- `GET /jaxrs/mind/assemble/control/folder/{id}`
- `POST /jaxrs/mind/assemble/control/folder/{id}/force`
- `POST /jaxrs/mind/assemble/control/folder/{id}/update`
