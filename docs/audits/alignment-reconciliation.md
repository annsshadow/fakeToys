# 覆盖计数对账报告（plan002 U2 / U9a-U3）

生成时间：2026-08-23
输入：`docs/audits/java-endpoint-inventory.json`（新口径：method+归一化路径去重）
方法：脚本化提取 `.route(` 注册（支持多行格式、`&fmt()` 间接寻址、链式 `get(..).put(..)` 多方法注册），
参数 `{x}` 统一归一化为 `{}` 后与 Java 清单逐条比对。

---

## 一、bbs 模块对账结论

### 1.1 当前真实状态：106/106 = 100%

| 项目 | 数值 |
|------|------|
| Java 唯一端点（新口径） | 106 |
| `crates/bbs_assemble_control/src/routes.rs` 的 `.route(` 调用 | 137 |
| 其中唯一归一化路径 | 135 |
| **Java 端点精确覆盖（method+path 全匹配）** | **106/106 = 100%** |
| Rust-only 扩展/兼容注册（不属 Java 口径） | 33 条路径 |
| **`lib.rs` 路由注册数** | **0** |

- 专项代理的 **106/106 结论正确**，但其论据"大量 lib.rs 注册"**错误**：
  该 crate 全部路由集中在 `routes.rs`（含 1 处多行 `.route(` 格式），`lib.rs` 无任何 `.route(`。
- 33 条 Rust-only 为 U2 之前的 legacy 扩展端点（如 `/config`、`/topic/create`、
  `/delete/forum`、`/uuid` 兼容别名等），保留作向后兼容，不影响 Java 对齐口径。

### 1.2 "57/106" 差异根因：两份报告口径混用

"57" 出自 `docs/audits/o2server-parity-report.md`（2026-08-12，scripts/o2server_parity_audit_v2.py 生成）：

> | x_bbs_assemble_control | bbs_assemble_control | **57** | 354 | 10 | 部分实现 |

该表中 **57 = Rust handler 数量**（测试覆盖率指标的分母，10/57=18%），
354 是旧扫描器的 Java @Path 注解计数（类级+方法级重复计入，非去重端点数）。
同数字亦见于 `oa4rust/docs/brainstorms/oa4rust-endpoint-inventory.md`（17 路由/57 handler/29.8%）。

而"Java 106 唯一端点"出自新的 java-endpoint-inventory.json（@Path 拼接后按
(method, 归一化路径) 去重）。**"Java 106 但 Rust 仅覆盖 57"是把 A 报告的分母
和 B 报告的分子拼在一起的伪命题**——不是扫描器漏扫多行 `.route(`（那只影响 1 处），
而是两个不同时期、不同度量维度的数字被错误对接。

git 历史（routes.rs）佐证时间线：

```
314c7a75  alignment 36.6%（17 条路由，legacy 路径方案）
0c03610d  36.9%->55.2%（44 条）
a5fe9cc1  56.7%（49 条）
32b12f83  full endpoint closure 106/106（137 条，fmt()+JAVA_BASE 统一前缀）
```

注意：中间版本（≤a5fe9cc1）采用 legacy 自造路径方案，按 Java 清单比对均为 0/106；
只有 32b12f83 起路径才与 v9 Java 端点逐条对齐。

---

## 二、processplatform_assemble_surface 去伪对账

### 2.1 总量与"134.6%"来源

| 项目 | 数值 |
|------|------|
| Java 唯一端点（新口径） | 659 |
| Rust `.route(` 调用（routes.rs；lib.rs 为 0） | 1036 |
| Rust 唯一归一化路径 | 943 |
| 旧口径 Java @Path 数（parity-report） | 701 |

**134.6% ≈ 943（Rust 唯一路径）÷ 701（旧口径 @Path 数）**。旧报告拿 Rust 路由数除以
未去重的注解计数，得出"超额注册"；按新口径 659 计算，比值为 143.1%（943/659）。

### 2.2 去伪后的真实对齐率：631/659 = 95.75%

| 层级 | 端点数 | 占比 |
|------|--------|------|
| exact（method+归一化路径完全一致） | 627 | 95.14% |
| casefold（仅大小写差异） | +2 | |
| literal_shift（参数↔字面量位移变体，段数相同） | +2 | |
| **合计对齐** | **631** | **95.75%** |
| 真缺失 | 28 | 4.25% |

### 2.3 重复变体（同一 Java 端点的多条 Rust 影子路径）：270 个端点受累

自动生成路由时发生系统性形变，产生影子副本（客户端调 v9 正确路径会 404，属真实缺陷）：

- 字面量段被参数化：Java `/anonymous/read/count/{}` ↔ Rust 额外注册了 `/anonymous/read/{}/{}`
- 参数被字面量化：Java `/correlation/job/{}` ↔ Rust `/correlation/job/job`
- 段序颠倒：Java `/application/{}/icon` ↔ Rust `/application/icon/{}`
- applicationdict/data/work/workcompleted 的 path0..path7 mock 变体族成片重复

其中 152 条为纯扩展（任何 Java 端点都对不上）：legacy CRUD（`/get/{id}`、`/create`、
`/publish/{}`、`/delete/{id}`）及更深层的 mockdeletetoget/mockputtopost 组合。

### 2.4 真缺失清单（28 条，按域分组）

| 域 | 条数 | 代表 |
|----|------|------|
| /snap/* | 6 | POST /snap/list/filter/{}/size/{}/manage 等过滤管理族 |
| /attachment/* | 4 | GET /attachment/download/{}/work{,completed}/{}/stream/{}.{}（带扩展名流式下载）|
| /review/* | 3 | POST /review/filter/attribute、GET /review/filter/create/entry、POST /review/v2/search |
| /draft/*、/keylock/*、/serialnumber/* | 各 2 | PUT /draft、PUT /keylock/lock、POST /serialnumber 等（Rust 侧仅有 GET 变体或缺失）|
| 其余 13 条 | 各 1 | POST /handover、GET /openapi、POST */filter/attribute(/filter)、POST /route/list/mockputtopost、POST /work/v3/retract、POST /workcompleted/shift/time |

模式提示：**凡 Java 为 POST 而 Rust 只注册了 GET 的 filter 类端点成批缺失**
（read/readcompleted/task/taskcompleted/review 的 filter 族），建议 U3 批量补 PUT/POST 方法变体。

---

## 三、行为对比桥接修复记录（comparator.rs 登录路径）

### 3.1 实测依据（o2server 容器 http://localhost:18080，O2OA v9.5.2）

| 实验 | 结果 |
|------|------|
| POST `/x_organization_assemble_authentication/jaxrs/authentication`
  body `{"credential":"xadmin","password":"o2oa@2022"}` | **HTTP 200**，`type=success`，
  **token 位于 `data.token`**（实测 43 字符），另含 tokenType/roleList/distinguishedName |
| 同路径错误密码 | HTTP **500**，`type=error`，无 data 字段 → 现有 `is_success()` 门禁可正确拒绝 |
| POST `/jaxrs/authentication/login`（旧缺陷路径） | 本容器实测 **404 快速返回**（0.2s）；
  ops 文档所述"挂起"未复现（可能针对 GET 或特定状态），但候选顺序仍按防御性设计 |

### 3.2 代码变更（tests/behavior_comparison/comparator.rs，唯一代码改动文件）

`login()` 由单一硬编码 `{base}/jaxrs/authentication/login` 改为**候选路径顺序回退**：

1. `/x_organization_assemble_authentication/jaxrs/authentication`（O2OA v9 真实路径，首选）
2. `/jaxrs/authentication/login`（Rust legacy 别名）
3. `/jaxrs/authentication`（Rust 主认证路径，crates/auth 同时服务此形状）

- token 提取逻辑不变（`data.token`）——实测确认 Rust（ActionResult<LoginResponse>）与
  Java v9 响应的 token 字段位置一致。
- 函数签名未变，`behavior_compare.rs` 调用点无需改动。
- 排序理由：Java 首跳命中真路径，不会触达未知裸 `/jaxrs/*`；Rust 侧第 1 候选立即 404 后回退。

### 3.3 验证结果

- `cargo check --test behavior_compare`：**通过**（5.7s；comparator 相关警告全部为存量问题：
  unused import/status/java_path/dead fields，与本次改动无关）。
- 小样本实测：Java 侧登录契约经 curl 实测闭环（见 3.1 表）。
- **全量 BEHAVIOR_COMPARE=1 实测未执行**，原因（不阻塞）：Rust 服务未运行于 :3000
  （Test-NetConnection 失败），且其依赖的 oa4rust-postgres-1 容器处于 Exited(255) 状态；
  启动整链路超出本任务范围。

### 3.4 遗留冲突（需决策，未擅改）

任务要求"JAVA_SERVICE_URL 默认 http://localhost:18080"，但该默认值位于
`tests/behavior_compare.rs:23`（当前默认 8080），而本任务禁改文件白名单仅含
comparator.rs 与本报告。环境变量支持本身已存在（behavior_compare.rs:61），
运行时显式设置即可：

```powershell
$env:JAVA_SERVICE_URL = "http://localhost:18080"
$env:BEHAVIOR_TEST_CREDENTIAL = "xadmin"
$env:BEHAVIOR_TEST_PASSWORD = "o2oa@2022"
$env:BEHAVIOR_COMPARE = "1"
cargo test --test behavior_compare
```

后续如允许改 behavior_compare.rs，建议将 `DEFAULT_JAVA_BASE_URL` 改为 18080 与容器文档对齐。
