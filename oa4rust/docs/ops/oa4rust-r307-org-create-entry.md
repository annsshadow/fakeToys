# R307 组织/部门新建入口与 unit_create 类型绑定修复

## 背景
t58 审计（§233）确认 /app/org 组织树与 /app/directory 部门树为空的根因是后端数据真空 + 前端缺新建入口，而非解析 bug。本轮在两页补齐新建入口，走产品自身真实 UI 流程补数。

## 前端改动
- OrgViewer.vue：新增「新建组织组」按钮与表单（data-testid org-create-group-btn/name/submit/error），调用 api.post(/api/organization/assemble/control/group, { name })，成功后 void loadOrgTree() 刷新；失败中文错误提示。保留 t14 的 withTimeout、api.put group/list/like、重试按钮与 onMounted。
- OrgDirectoryApp.vue：部门树面板新增「新建部门」按钮与表单（data-testid org-create-unit-btn/name/parent/submit/error），调用 api.post(/api/organization/assemble/control/unit, { name, parentId })，成功后 void loadDeptTree() 刷新；失败中文错误提示。保留 t28 的 withTimeout、错误态、重试按钮与 onMounted。
- 契约 OrgCreateEntry.R307.contract.test.ts：6 项断言覆盖按钮/输入/提交存在、端点调用、成功刷新、t14/t28 兜底不回退；先 RED（4 failed / 2 passed）后 GREEN（19 tests passed）。

## 后端根因与修复（test-first）
真实浏览器新建部门时 POST /api/organization/assemble/control/unit 返回 HTTP 500。后端运行日志坐实根因：
    crates/organization_assemble_control/src/u2_org.rs:60: Error { kind: ToSql(3), cause: Some(WrongType { postgres: Int4, rust: "alloc::string::String" }) }
unit_create 的 INSERT 使用 `$4::int`（PostgreSQL INT4）。真实三轮修复与日志实证：原代码把 sort 绑定为 `String` 报 `WrongType { postgres: Int4, rust: "alloc::string::String" }`；改为 `i64` 报 `WrongType { postgres: Int4, rust: "i64" }`；用 `unit_edit` 的 `.to_string()` 范式仍报 String 不匹配。库级实证 `<i32 as ToSql>::accepts(&Type::INT4)==true`、i64/String 均为 false，最终修正是 `as_i64().map(|v| v as i32).unwrap_or_default()`，与 INT4 类型严格匹配。DDL 022 中 sort INTEGER DEFAULT 0；deleted_at 可空、type 列由迁移补齐，均非根因。
- RED 测试：tests_u2::unit_create_sort_is_bound_as_int_not_string 先失败（sort 被字符串化）。
- 最小修复：u2_org.rs 中 unit_create 的 sort 改为 `as_i64().map(|v| v as i32).unwrap_or_default()`（i32 是唯一接受 INT4 的 Rust 整数类型），其余逻辑不动。
- GREEN：该测试通过；cargo test -p organization_assemble_control --lib 138 passed / 0 failed。

## 测试服实测
前端产物原子部署到 <YOUR_TEST_DOMAIN> 的静态目录，后端 Linux ELF 二进制（用与目标环境 glibc 兼容的 Linux 镜像构建；构建时不把工具链真值写入仓库）原子替换 + 服务重启，部署脚本含替换前备份、健康检查失败自动回滚。Playwright Chromium headless 真浏览器、真实登录、真实 UI 流程：
1. /app/directory 点「新建部门」填名提交 → 部门树出现新节点，刷新后仍在，接口 /api/departments/tree 返回非空且含新部门。
2. /app/org 点「新建组织组」填名提交 → 组织树出现新节点，刷新后仍在，接口 group/list/like 返回含新组。
3. 全程 0 pageerror；新建失败时显示中文错误提示与可重试状态。接口探测需带正确 Content-Type，GET 类端点用 axios 客户端的 PUT/JSON 语义，避免 415 误判。
证据：oa4rust/test-results/r307-create-entry/（截图 + evidence.json），来源标注 Playwright Chromium headless against deployed test server; not a physical device（本地渲染，非物理真机）。凭据经 SSH 在运行时从远端读取，未入库。

## 约束
- 不改后端其他逻辑；创建接口仍为既有 POST 路由。
- 文档中的域名/主机/端口均用 <YOUR_TEST_DOMAIN> 占位，真值只从本机环境变量与远端凭据读取。
