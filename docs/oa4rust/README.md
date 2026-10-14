# OA 子项目开发者文档

本目录包含 `oa` 子项目的完整开发者文档，面向新加入的开发者和需要了解系统架构、模块结构、二次开发方式的维护者。

## 文档入口

- [架构概览](architecture.md) — 系统整体架构图与模块依赖关系图
- [模块卡片](modules/o2server/) — `o2server` 各 Maven 模块的职责、依赖和端点摘要
- [组件卡片](modules/o2web/) — `o2web` 各前端组件的职责和关键配置
- [开发环境](development/o2server-setup.md) — `o2server` 与 `o2web` 的本地搭建指南
- [部署指南](deployment/windows.md) — Windows 与 Linux 环境部署说明
- [配置参考](reference/configuration.md) — `configSample/` 配置文件逐项说明
- [API 参考](api/README.md) — 代码驱动的 API 文档提取结果
- [数据模型](reference/data-models.md) — 核心功能域的 JPA 实体类关系
- [业务功能](guide/business-functions.md) — 按功能域组织的平台能力说明
- [低代码能力](guide/low-code-capabilities.md) — 表单、页面、流程设计器的使用方式
- [扩展 o2server](guide/extending-o2server.md) — 新增后端模块的步骤与约定
- [扩展 o2web](guide/extending-o2web.md) — 新增前端组件的步骤与约定
- [核心概念](guide/core-concepts.md) — 实体命名、Express 脚本、服务发现等平台约定

## 说明

- 本目录聚焦于**开发者视角**的架构、模块和二次开发，不替代 `oa/README.md` 中的官方产品介绍。
- 模块卡片和组件卡片采用统一模板，便于后续批量更新。
- API 文档基于代码自动提取，与代码变更保持同步。
