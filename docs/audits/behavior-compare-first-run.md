# 行为对比首次实测记录（plan002 U9a→U3 前置验证）

日期：2026-08-24
环境：Windows / PowerShell 5.1，Java o2server v9.5.2 容器 `http://localhost:18080`（凭据 xadmin/o2oa@2022），Rust 测试服务进程内启动（PostgreSQL，docker compose postgres 服务）
结论先行：**7 个只读端点全部完成两侧实测，HTTP 层全部可达；结构一致性 0 PASS / 7 FAIL / 0 SKIP**。FAIL 由少数几个系统性根因造成（见「关键发现」），其中响应信封双层嵌套是阻断性问题，须在 U3 全量对比前决策处理。

## 一、执行流程与产物

- 驱动文件：`oa4rust/tests/behavior_compare_sample.rs`（新增，小样本驱动；全量驱动 `tests/behavior_compare.rs` 因 ENDPOINTS 清单 java_war/java_action 全为空暂不可用）
- 框架微调：`oa4rust/tests/behavior_comparison/comparator.rs`（见下）
- 自动报告：`oa4rust/target/debug/behavior-report-sample.md`（本次实测原始输出）
- 复现方式：
  ```powershell
  $env:BEHAVIOR_COMPARE_SAMPLE = "1"
  $env:JAVA_SERVICE_URL = "http://localhost:18080"
  cargo test --test behavior_compare_sample -- --nocapture
  ```

### comparator.rs 微调说明（非业务源码，测试脚手架）

1. **认证头修正**：实测证实 O2OA v9 只认 `x-token` header（或同名 Cookie）；`Authorization: Bearer` 被当作 anonymous（whoami 实测返回 `tokenType:"anonymous"`）。原实现只发 Bearer，Java 受保护端点全部失真。现同时发送两种头。
2. **双 token 支持**：Rust 与 Java 登录 token 互不通用，原实现 `with_auth_token(java_token)` 会把 Java token 发给 Rust 侧。新增 `with_tokens(rust_token, java_token)` 按 base URL 分别选取；旧接口行为保持兼容。
3. 未改动 diff 算法与路径回退链（cc15aa46 已修复部分保持原样）。

## 二、Java 测试账户创建结果 ✅

testadmin/testadmin 创建成功并可正常登录（tokenType=user）。所用 REST API（均为 xadmin x-token 鉴权）：

| 步骤 | API | 说明 |
|------|-----|------|
| 1. 创建 person | `POST /x_organization_assemble_control/jaxrs/person`，body `{"name":"testadmin","employee":"90001","mobile":"13800000001","mail":"testadmin@o2oa.net"}` | 返回 `data.id`（f474dae3-5744-49fd-97b6-8357cadc5e34）；unique 为随机 UUID |
| 2. 设置密码 | `PUT /x_organization_assemble_control/jaxrs/person/{id}/set/password`，body `{"value":"testadmin"}` | v9 的 Wi 继承 WrapString，密码放在 `value` 字段（从 war 内 describe/sources 反查确认） |
| 3. 解锁（如触发） | `GET /x_organization_assemble_control/jaxrs/person/unlock/{id}` | **只认 id，不认 name** |

排错记录：
- `PUT person/{id}` 带 password 字段：成功但被忽略（不是改密通道）
- `{id}/reset/password` 仅支持 GET 且重置为初始密码而非指定密码；PUT/POST 均 405
- 连续登录失败约 3 次即锁定 5 分钟（ExceptionFailureLocked），需走 unlock API

## 三、样本端点实测结果

| # | Method | 端点（rust_path=java_action 映射） | Rust HTTP | Java HTTP | 判定 | 主要差异 |
|---|--------|-----------------------------------|-----------|-----------|------|----------|
| 1 | GET | `/jaxrs/unit/list/(0)/next/20` ↔ control `unit/list/(0)/next/20` | 200 | 200 | **不一致** | 根因 A/B/C + 数据集差异 |
| 2 | GET | `/jaxrs/person/list/(0)/next/20` ↔ control `person/list/(0)/next/20` | 200 | 200 | **不一致** | 根因 A/B/C + 数据集差异 |
| 3 | GET | `/jaxrs/role/list/(0)/next/20` ↔ control `role/list/(0)/next/20` | 200 | 200 | **不一致** | 根因 A/B/C（Java 25 个系统角色 vs Rust 3 条测试角色） |
| 4 | GET | `/jaxrs/group/list/(0)/next/20` ↔ control `group/list/(0)/next/20` | 200 | 200 | **不一致** | 数据两侧均为空数组，仍 FAIL → 纯结构差异（根因 A/B/C） |
| 5 | GET | `/jaxrs/person/xadmin` ↔ control `person/xadmin` | 200（error envelope "person not found"） | 200（person 详情） | **不一致** | 根因 D + 测试数据不对齐（Rust 库无 xadmin person） |
| 6 | GET | `/jaxrs/unit/list` ↔ control `unit/list` | 200（3 units） | 500（"组织:list, 不存在."） | **不一致** | 根因 E：Java 将其解析为 `unit/{flag}=list`，无独立路由；Rust 有独立路由 |
| 7 | GET | `/jaxrs/authentication` ↔ authentication war `authentication` | 200 | 200 | **不一致** | 根因 F：whoami 字段覆盖悬殊（Rust data 4 字段 vs Java 30+ 字段） |

统计：实测 7 / 一致 0 / 不一致 7 / SKIP 0。

## 四、关键发现（系统性根因）

**A.【阻断】分页响应信封双层嵌套**
所有 list/paging 端点：Rust 顶层信封（type/count/data/date/message/position/size/spent）字段齐全且值正确，但 `data` 内又嵌套一层 `{count, data:[...], size}`；Java 的 `data` 直接是数组。
影响面：全部分页端点（ENDPOINTS 中占比很高）。U3 全量对比前必须先决策：修 Rust 响应封装（对齐 O2OA 单层信封）或在 comparator 加归一化层。倾向前者（业务语义对齐优于测试侧遮蔽），需另行评估改动范围。

**B. 顶层多余 `prompt` 字段**：Rust 信封恒有 `prompt:null`，Java 无此字段。

**C. 元数据类型差异**（同一字段名、不同类型）：
- `date`：Rust null / Java `"yyyy-MM-dd HH:mm:ss"`
- `message`：Rust null / Java `""`
- `position`：Rust `"next"/"prev"` 字符串 / Java 数字 0
- `spent`：Rust null / Java 耗时毫秒数
这些属"允许列表"候选：若产品决策为接受差异，应加入 allowlist.yaml 而非逐端点修。

**D. 错误语义不一致**：资源不存在时 Rust 返回 HTTP 200 + `type:error` envelope；Java 返回 HTTP 4xx/5xx + error envelope。影响所有 not-found 类断言。

**E. 路由歧义**：Java `GET /jaxrs/unit/list` 被 `unit/{flag}` 吞掉（flag="list" 报"组织不存在"），Rust 注册了独立精确路由。同类风险适用于所有"短名词恰好撞 {flag}"的路径（如 role/list、group/list）。

**F. whoami 字段覆盖**：Rust data 仅 authenticated/id/name/unique；Java 另有 tokenType/roleList/identityList/distinguishedName/mobile/mail/failureCount/topUnitList 等 30+ 字段。

**G. 测试数据不对齐**（非代码缺陷）：Rust 测试库无 xadmin person；角色/组织种子数据与 Java 新库不同。后续做全量对比需要一套两侧等价的 seed 方案。

## 五、框架遗留问题清单（U3 前待办）

1. `tests/behavior_comparison/endpoints.rs`（自动生成）所有条目 `java_war`/`java_action` 为空字符串 → 全量 `behavior_compare.rs` 目前产出的 Java URL 全部无效。生成器需补 war/action 映射（本次手工映射的 7 条可作种子）。
2. `is_service_reachable` 探测 `GET /health`：O2OA 无此端点（404），全量跑会把 Java 判为不可达而全部 SKIP。建议改为探测已知 200 端点（如登录或 `/x_organization_assemble_authentication/jaxrs/authentication`）。
3. `JAVA_SERVICE_URL` 默认值仍为 8080（tests/behavior_compare.rs:23）——本次通过环境变量绕过，建议改默认值为 18080。
4. comparator diff 文本把整个 JSON 子树打进每条差异（报告单行截断到 2000 字符仍超长），可读性差，建议改为仅打印路径与两侧类型摘要。
5. comparator.rs:197 存在死代码 `let status = ...`（编译警告 unused variable，原有问题，未动）。
6. CI 冷启动等待 ≥10 分钟（docs/ops/o2server-container.md 已知项 3）。

## 六、结论

U9a→U3 前置验证达成：
- 登录语义成立：testadmin/testadmin 两侧均可登录，token 提取路径（v9 war 路径 + data.token）实测有效；
- 端点可达性成立：7/7 双侧拿到 HTTP 响应，无 SKIP；
- 结构等效性**未达成**：0/7 PASS，但归因为 4 个系统性根因（A/B/C/D）+ 2 个端点级问题（E/F）+ 数据不对齐（G）；
- 建议 U3 启动顺序：先决策 A（响应信封）、C（allowlist 化元数据差异）、D（错误码语义），再补 endpoints.java_war 生成器，最后扩样本至全量。
