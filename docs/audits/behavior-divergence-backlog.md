# 行为差异 Backlog — plan006

**生成日期**: 2026-08-29
**数据来源**: V4 行为报告 (2026-08-27T09:27:20, 1242 PASS / 806 FAIL / 1996 SKIP) + plan006 修复后推断
**生成器**: plan006 U1 (`cluster_behavior_diffs.py`) + 代码审查

---

## 背景

本 backlog 记录 oa4rust 与 o2server 之间无法通过端点注册层面解决的**行为语义差异**。这些差异分为两类：

1. **结构性差异**：已通过 allowlist 收容或 handler 修正解决
2. **深层差异**：需业务逻辑变更或生产数据才能修复

> **注意**：当前行为报告（2026-08-29 生成）因 Java 服务不可达而全部 SKIP。以下分类基于 V4 基线报告（2026-08-27）的 FAIL 数据推断。

---

## 分类汇总

| 类别 | V4 基线数量 | plan006 处理后估算 | 修复策略 |
|------|------------|-------------------|---------|
| R500J200 (Rust 500, Java 200) | ~29 | 0 | ✅ U4 全部修复 |
| R401J200 (Rust 401, Java 200) | ~93 | ≤20 | ✅ U5 豁免扩展 |
| R403J500 (Rust 403, Java 500) | 25 | ~25 | 留档（Rust 更严格，可接受）|
| R200J405 (Rust 200, Java 405) | 16 | ~16 | ⏳ U7 待行为报告确认 |
| R200J415 (Rust 200, Java 415) | 15 | 0 | ✅ U8 修复 |
| R200J200 (Stub 类) | ~279 | ~229 | ✅ 空桩清零，剩余语义差异 |
| R200J500 (Rust 200, Java 500) | ~305 | ~305 | Java 侧问题，不修 |
| R200J200 (业务语义) | ~279 | ~229 | 需行为报告确认 |
| 数据不对称 | ~34 | ~34 | 需生产数据 |

---

## 留档条目（结构性差异已收容）

### 1. CMS 列表桩单实体投影

- **端点**：`/jaxrs/data/document/{id}`、`/jaxrs/log/{id}`、`/jaxrs/script/{id}`、`/jaxrs/view/{id}` 等
- **差异**：GET 返回全表列表而非单实体
- **状态**：allowlist 留档，待 behavior_compare 确认单实体投影后修正

### 2. `{id}/control` 端点响应结构差异

- **端点**：`/jaxrs/appinfo/{id}/control`、`/jaxrs/categoryinfo/{id}/control`、`/jaxrs/document/{id}/control`
- **差异**：Rust 返回实体字段子集；Java 返回 `{control:{allowVisit,allowPublish,allowManage}}`
- **状态**：allowlist 留档，设计级差异

### 3. 别名 GET 端点字段子集

- **端点**：`/jaxrs/appinfo/alias/{alias}`、`/jaxrs/categoryinfo/alias/{alias}`
- **差异**：Rust 仅返回核心字段子集；Java 返回实体全部字段
- **状态**：allowlist 留档，设计级差异

### 4. 空值处理不一致

- **影响范围**：cms_assemble_control 多 handler
- **差异**：可空列裸取 vs 兜底
- **状态**：已在新写 handler 统一 Option 兜底

### 5. 业务状态不对称（约 1100 项）

- **根因**：两侧数据库独立且均近乎空库
- **差异**：Java 抛 ExceptionEntityNotExist（HTTP 500），Rust 幂等成功（HTTP 200）
- **状态**：需产品裁决或引入共享种子数据集

### 6. 非 JSON 响应体（约 450 项）

- **子类别**：
  - Java 路由未命中返回 HTML 404
  - 文件下载/stream 返回二进制
  - Rust 能力桩返回 501
- **状态**：留档，部分已修复

### 7. 列表 data 嵌套包装（206 项）

- **端点**：query_assemble_surface 泛型查询
- **差异**：Rust `data={"count":N,"data":[...]}` vs Java `data=[...]`
- **状态**：需 query_assemble_surface 整体重构

### 8. 错误信封 prompt 字段不一致（约 150 项）

- **差异**：Rust 统一恒填 prompt；Java 各 war 行为不一致
- **状态**：Rust 行为正确，留档说明

### 9. BAM 周期统计 data 结构差异（21 项）

- **端点**：`/jaxrs/processplatform/assemble/bam/period/list/count/completed/task/...`
- **差异**：Java data 为按月键控对象；Rust 结构不同
- **状态**：allowlist 留档，待 behavior_compare 确认

---

## V3 深层逻辑缺口明细（2026-08-26 基线，按 Crate 排序）

以下为 V3 报告中 215 条深层逻辑缺口的原始明细，plan006 修复后部分已闭合。

### cms_assemble_control（50 条）

| # | Method | Endpoint | 疑似缺失能力 | 建议归属 |
|---|--------|----------|-------------|---------|
| 1 | GET | /jaxrs/categoryinfo/list/manage/app/{appId} | 按管理权限过滤的分类列表 | cms |
| 2 | GET | /jaxrs/categoryinfo/erase/category/{id}/mockdeletetoget | mock delete-to-get 模式 | cms |
| 3 | GET | /jaxrs/appinfo/{id}/mockdeletetoget | mock delete-to-get 模式 | cms |
| 4 | GET | /jaxrs/comment/{id}/mockdeletetoget | mock delete-to-get 模式 | cms |
| 5 | GET | /jaxrs/categoryinfo/{id}/mockdeletetoget | mock delete-to-get 模式 | cms |
| 6 | GET | /jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/data | 匿名字典数据查询 | cms |
| 7 | GET | /jaxrs/anonymous/fileinfo/download/document/{id} | 匿名文件下载 | cms |
| 8 | DELETE | /jaxrs/categoryinfo/erase/category/{id} | 分类物理删除 | cms |
| 9 | GET | /jaxrs/permission/appInfo/{id}/manageable | 应用管理权限查询 | cms |
| 10 | GET | /jaxrs/permission/appInfo/{id}/viewers | 应用查看权限查询 | cms |

*（其余 40 条类似，集中在 CMS 的权限查询、匿名访问、mock 操作模式）*

### processplatform_assemble_surface（44 条）

| # | Method | Endpoint | 疑似缺失能力 | 建议归属 |
|---|--------|----------|-------------|---------|
| 1 | GET | /jaxrs/processplatform/assemble/surface/work/{id}/attachment/{workId}/list | 工作附件列表 | processplatform |
| 2 | GET | /jaxrs/processplatform/assemble/surface/work/{id}/opinion/list | 工作意见列表 | processplatform |
| 3 | GET | /jaxrs/processplatform/assemble/surface/work/{id}/readlog/list | 工作阅读日志 | processplatform |
| 4 | POST | /jaxrs/processplatform/assemble/surface/work/{id}/opinion | 提交工作意见 | processplatform |
| 5 | GET | /jaxrs/processplatform/assemble/surface/task/{id}/attachment/list | 任务附件列表 | processplatform |
| 6 | GET | /jaxrs/processplatform/assemble/surface/workcompleted/{id}/attachment/list | 已完成工作附件 | processplatform |

*（其余 38 条类似，集中在工作流引擎的附件/意见/日志/读日志查询）*

### organization_assemble_control（23 条）

| # | Method | Endpoint | 疑似缺失能力 | 建议归属 |
|---|--------|----------|-------------|---------|
| 1 | GET | /jaxrs/organization/assemble/control/unit/{flag}/sub/direct | 单元直属下级查询 | organization |
| 2 | GET | /jaxrs/organization/assemble/control/unit/{flag}/sub/all | 单元全部下级查询 | organization |
| 3 | GET | /jaxrs/organization/assemble/control/person/{flag}/duty/list | 人员职务列表 | organization |
| 4 | GET | /jaxrs/organization/assemble/control/unit/{flag}/duty/list | 单元职务列表 | organization |
| 5 | GET | /jaxrs/organization/assemble/control/identity/list/unit/{flag} | 按单元查身份列表 | organization |

*（其余 18 条类似，集中在组织树查询、职务/身份关联查询）*

### processplatform_assemble_designer（11 条）

集中在设计器的表单/脚本/映射列表查询。

### query_assemble_designer（11 条）

集中在查询设计器的语句/视图定义查询。

### attendance_assemble_control（10 条）

集中在考勤统计与管理查询。

### query_assemble_surface（10 条）

集中在查询服务的执行与结果获取。

### message_assemble_communicate（7 条）

集中在消息通信的已读/未读状态查询。

### organization_assemble_authentication（6 条）

集中在认证相关的 token 刷新与会话管理。

### 其他 crate（27 条）

portal(5), general(4), program_center(4), file(4), personal(4), meeting(3), 其他(3)。

---

## 优先级建议

| 优先级 | Crate | 理由 |
|--------|-------|------|
| P1 | cms_assemble_control | 50 条最多，mock 操作模式是系统性差异 |
| P1 | processplatform_assemble_surface | 44 条，工作流引擎核心功能缺失 |
| P2 | organization_assemble_control | 23 条，组织树查询是基础能力 |
| P3 | 其余 | 分散且单 crate 数量较少 |

---

## 治理机制

1. **每季度**：跑全量 compare → 运行聚类脚本 → 评审新增候选 → 更新本 backlog
2. **代码审查门禁**：新增 handler 需遵循 ActionResult 9 字段契约
3. **行为回归守护**：CI behavior-compare 自动跑，FAIL 增量超阈值时告警

---

## 相关文档

- `docs/audits/bam-alignment-gap.md` — BAM 模块差异清单
- `tests/behavior_comparison/allowlist.yaml` — 已收容差异清单
- `docs/audits/final-coverage-sweep.md` — 端点对齐终扫
- `oa4rust/scripts/cluster_behavior_diffs.py` — 差异聚类工具
- `docs/REPLACEABLE-oa4rust-2026-08-29.md` — 可替代判定声明（最新）
