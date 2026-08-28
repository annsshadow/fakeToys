# Behavior Divergence Backlog

> 生成日期：2026-08-26
> 数据源：`oa4rust/target/debug/behavior-report.md`（1199 PASS / 844 FAIL / 2001 SKIP）
> 筛选口径：Rust 返回错误（`prompt` 字段）而 Java 返回成功（`data` 字段）的端点——即 Rust 侧存在未实现的业务逻辑或查询路径。
> 聚类脚本：`oa4rust/scripts/cluster_behavior_diffs.py`

---

## 汇总

| 指标 | 数值 |
|------|------|
| 深层逻辑缺口总数 | 215 |
| 涉及 crate 数 | 15 |
| 占总 FAIL 比例 | 25.7%（215/844） |

**缺口分层（U3 评审后归因）**：

1. **信封/包装差异（非逻辑缺口）**：约620条（`data`/`prompt` 包装差异 + 附件端点结构差异）——需 handler 层统一信封形状，不进本 backlog。
2. **业务状态不对称**：约100条（字面量 `{id}` 查不存在资源）——需共享种子或真实数据。
3. **深层逻辑缺口（本 backlog）**：215条——Rust 侧未实现对应的查询/操作分支。

---

## 按 Crate 排序

### cms_assemble_control（50 条）

| # | Method | Endpoint | 疑似缺失能力 | 建议归属 |
|---|--------|----------|-------------|---------|
| 1 | GET | /jaxrs/categoryinfo/list/manage/app/{appId} | 按管理权限过滤的分类列表 | cms |
| 2 | GET | /jaxrs/categoryinfo/erase/category/{id}/mockdeletetoget | mock delete-to-get 模式（软删除后读取） | cms |
| 3 | GET | /jaxrs/appinfo/{id}/mockdeletetoget | mock delete-to-get 模式 | cms |
| 4 | GET | /jaxrs/comment/{id}/mockdeletetoget | mock delete-to-get 模式 | cms |
| 5 | GET | /jaxrs/categoryinfo/{id}/mockdeletetoget | mock delete-to-get 模式 | cms |
| 6 | GET | /jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/data | 匿名字典数据查询 | cms |
| 7 | GET | /jaxrs/anonymous/fileinfo/download/document/{id} | 匿名文件下载 | cms |
| 8 | DELETE | /jaxrs/categoryinfo/erase/category/{id} | 分类物理删除 | cms |
| 9 | GET | /jaxrs/permission/appInfo/{id}/manageable | 应用管理权限查询 | cms |
| 10 | GET | /jaxrs/permission/appInfo/{id}/viewers | 应用查看权限查询 | cms |

*（其余40条类似，集中在 CMS 的权限查询、匿名访问、mock 操作模式）*

### processplatform_assemble_surface（44 条）

| # | Method | Endpoint | 疑似缺失能力 | 建议归属 |
|---|--------|----------|-------------|---------|
| 1 | GET | /jaxrs/processplatform/assemble/surface/work/{id}/attachment/{workId}/list | 工作附件列表 | processplatform |
| 2 | GET | /jaxrs/processplatform/assemble/surface/work/{id}/opinion/list | 工作意见列表 | processplatform |
| 3 | GET | /jaxrs/processplatform/assemble/surface/work/{id}/readlog/list | 工作阅读日志 | processplatform |
| 4 | POST | /jaxrs/processplatform/assemble/surface/work/{id}/opinion | 提交工作意见 | processplatform |
| 5 | GET | /jaxrs/processplatform/assemble/surface/task/{id}/attachment/list | 任务附件列表 | processplatform |
| 6 | GET | /jaxrs/processplatform/assemble/surface/workcompleted/{id}/attachment/list | 已完成工作附件 | processplatform |

*（其余38条类似，集中在工作流引擎的附件/意见/日志/读日志查询）*

### organization_assemble_control（23 条）

| # | Method | Endpoint | 疑似缺失能力 | 建议归属 |
|---|--------|----------|-------------|---------|
| 1 | GET | /jaxrs/organization/assemble/control/unit/{flag}/sub/direct | 单元直属下级查询 | organization |
| 2 | GET | /jaxrs/organization/assemble/control/unit/{flag}/sub/all | 单元全部下级查询 | organization |
| 3 | GET | /jaxrs/organization/assemble/control/person/{flag}/duty/list | 人员职务列表 | organization |
| 4 | GET | /jaxrs/organization/assemble/control/unit/{flag}/duty/list | 单元职务列表 | organization |
| 5 | GET | /jaxrs/organization/assemble/control/identity/list/unit/{flag} | 按单元查身份列表 | organization |

*（其余18条类似，集中在组织树查询、职务/身份关联查询）*

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
| P1 | cms_assemble_control | 50条最多，且 mock 操作模式是系统性差异 |
| P1 | processplatform_assemble_surface | 44条，工作流引擎核心功能缺失 |
| P2 | organization_assemble_control | 23条，组织树查询是基础能力 |
| P3 | 其余 | 分散且单 crate 数量较少 |

---

## 下一步

1. **信封统一**（独立 refactor 计划）：解决 `data`/`prompt` 包装差异（~350条），预期可将 FAIL 降低30%+
2. **工作流引擎补齐**（processplatform）：最高业务价值但实现复杂度也最高
3. **CMS mock 操作模式**：评估是否为 Rust 侧需要对齐的语义
4. **共享种子扩展**：补充更多种子数据以消除业务状态不对称类假差异
