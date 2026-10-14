<!--
Copyright (C) 2026 annsshadow
SPDX-License-Identifier: AGPL-3.0-or-later
-->

# oa4rust 十类新功能 + 三端消费能力 100% 收口（2026-10-07 启动）

> 目标：①给 oa4rust 添加 **10 类新功能**（真实产品能力，后端+desktop+mobile 三端）；②**三端消费能力确认 100%**。
> 方法：以 2026-10-07 实测缺口 **296 条未消费路由**（全仓 4140/4436=93.3%，域口径 3599/3872=92.9%）
> 为靶子——**10 类功能即 296 条缺口的真实产品化承接**，每条路由由真实用户动作消费，不造假接桩
>（承接 2026-09-26 翻案纪律；沿用 rev486「同 handler 双计放宽」与「外部消费者端点桶外」裁定先例）。
> 缺口构成：84 外部身份源 + 82 新近路由（org list/object 族）+ 71 纯写动作 + 18 会话销毁 +
> 13 别名孪生 + 8 测试垃圾 + 5 外部消息 + 4 敏感(口令入URL) + 4 凭证匿名 + 3 WebSocket + 2 畸形 + 2 引擎。

## 十类功能 → 缺口映射

| # | 功能 | 承接缺口（约） | 端 |
|---|---|---|---|
| 1 | 账号安全中心（会话/两步验证/注册/找回/短信验证码） | 35（SESSION_DESTRUCTIVE+reset+secret+regist） | desktop+mobile |
| 2 | 组织通讯录与管理台（组织树/人员/身份/群组/角色） | 70（org list/object 族 82 条主体） | desktop+mobile |
| 3 | BBS 治理面板（删帖/禁言/板块图标/同步） | 11 | desktop |
| 4 | 流程协作与归档（催办/传阅/批量附件/HTML转PDF） | 24 | desktop |
| 5 | 设计器保存与资源族（PUT save/{id}/另存/版本删除） | 14 | desktop |
| 6 | 文件中心与个人资产（缩略图/另存/签名/IM图片） | 20 | desktop+mobile |
| 7 | 日历/考勤/假期（日程编辑/请假申请/钉钉企微同步） | 12（含 MALFORMED 修复） | desktop+mobile |
| 8 | 通知推送与设备中心（设备绑定/测试发送/推送类型） | 10 | desktop+mobile |
| 9 | 程序中心与开放平台（调度/开放接口/令牌/采集接入） | 25（含 TEST_GARBAGE 甄别删除） | desktop |
| 10 | AI 助手增强（流式对话/附件/配置读取） | 4 | desktop |

## 度量收尾原则（诚实口径）

1. **WS 识别**：extract_calls.py 增加真实 WebSocket 连接识别（SDK useWebSocket 内字面 URL）——识别既存真实消费，非放水。
2. **外部消费者端点桶外**：IdP 回调/交换类（消费者=钉钉/企微/政务钉钉/welink/OAuth IdP/短信网关等外部系统，我们的前端不可也不应 XHR 调用）从静态前端分母剔除，逐条台账记档（rev486「桶外」先例）。起点端点（登录发起/同步/测试发送）由前端真实消费。
3. **SENSITIVE（口令入 URL）**：安全重设计为 POST body 版本，前端用新版；旧 GET 版删除并同步生成测试。
4. **TEST_GARBAGE**：handler 为字面空壳的删除（test_test1/test_test2）；名字像垃圾但真实现的（foo/bar create mass=调度任务列表）由真实功能消费。
5. 结构性不可诚实计入项若最终仍有残留，逐条记档原因，不留静默。

## 轮记

| # | 功能 | 结果 | 明细 |
|---|---|---|---|
| 收口 | 三端消费 100% + 十类功能补全 + 性能/测试/文档收口 | **诚实口径 100%** | 见下方「收口轮」小节 |

## 收口轮（2026-10-11）

> 本轮以 2026-10-07 台账为起点，目标：三端消费能力（诚实口径）达到 100%，补齐十类功能剩余面，
> 同步性能/测试/文档收口。所有「已消费」均由真实用户动作（按钮/操作）驱动，不造假接桩；
> 不可诚实计入项逐条记档桶外/结构，不留静默。

### 结果数字（本机实测，复现命令见文末）

| 门禁 | 结果 |
|---|---|
| 三端对账 `compare.py --gate` | **shadow/405/404 = 0 → PASS** |
| 契约 `schema_audit.py --gate` | **A=B=C=D=0，E=2（基线）→ PASS** |
| 桌面契约守卫 `vitest`（55 文件） | **1024/1024 全绿** |
| 移动端 `pnpm test:mobile`（10 文件） | **121/121 全绿** |
| 六包 `pnpm typecheck` | **全 Done（0 错）** |
| 桌面 `pnpm build`（Vite/Rolldown） | **4s 通过，路由懒加载产物完整** |
| biome（仅本轮触及文件） | **clean**（根 `pnpm lint` 余 11 项为本地 CRLF 检出假红，git 未改文件，CI 的 LF 检出即绿） |
| 未消费台账 `_classify_unconsumed.py` | **117 条 = 桶外 EXTERNAL_IDP 78 + 结构 TWIN_ALIAS 39**（其余 0 条） |
| **诚实口径 `consumption_gap.py`** | **桶外/结构剔除 117 → 诚实可消费面 4314，已消费 4314，诚实消费率 = 100.0%** |
| 后端 `cargo check`（7 个改动 crate） | **CARGO_EXIT=0（GNU 工具链 + vendor binutils/gcc）** |
| 后端 `cargo test`（无 DB / 7 crate） | **4563 通过 / 0 失败**（DB 依赖用例按守卫忽略） |
| 后端 `cargo test -p parity --lib`（本地 PG 已迁移 379 表） | **4143/4143 全绿（行为契约实证）** |
| 运行时性能：迁移 108 考勤 `(person_id,date)` 热路径索引 | **EXPLAIN：统计 3.19ms→0.198ms，列表 2.37ms→0.048ms（排序节点消失）** |

> 说明：域口径原始静态率为 96.7%（3741/3869），差额 117 条**全部**为「外部身份源回调（消费者=
> 钉钉/企微/政务钉钉/welink/OAuth IdP，前端不可也不应 XHR，rev486 桶外先例）」78 条 +
> 「同 handler 双计放宽的结构孪生（方法/路径/别名）」39 条，均已在 `unconsumed_ledger.json`
> 逐条记档裁决；**可诚实接入面（真实用户动作/真实读/WS）已 100% 真实消费**。
>
> **口径确认状态**：三端消费能力"100%"按**诚实口径**认定——**用户已于 2026-10-11 确认**
> （"按你推荐的口径就行"）：分母剔除 117 条逐条记档项（78 外部 IdP 桶外 + 39 结构孪生），
> 4314/4314 = 100.0%；原始静态口径 96.7%（3741/3869）作为披露数字保留在台账中，
> 117 条维持记档不双计。若日后需要更严口径（39 条结构孪生也逐条真消费），
> 按本台账逐条补 UI 接线后复测即可，不影响本次认定。

### 十类功能补全（本轮新增真实接线，非桩）

| 功能 | 新增真实消费面 | 端 |
|---|---|---|
| 9 程序中心/开放平台 | 开放平台 8 写端点（改管理口令/更新系统 Token(PUT)/更新令牌阈值(PUT)/执行最新接口/签发调用器 Token/带令牌执行/模块对比上传·同步(PUT)）+ 调度任务统计（foo/bar create mass 族真实现）+ 管理台登出/登录头像图(GET·PUT·erase) | desktop |
| 1 账号安全中心 | 两步验证登录/全设备登出/短信发码·校验/登录验证码校验/系统密钥设定·取消/用户注册/注册口令校验/注册验证码图/取注册码/查注册码/重置密码/找回口令校验/取找回码/校验找回凭据/匿名重置（17 端点） | desktop |
| 2 组织通讯录/管理台 | 组织认证台（登录/登出/当前用户/全设备登出/切换用户/两步验证）+ 人员口令（按 flag 重置/设有效期/策略校验）+ 单位×身份/人员 5 交叉清单（object 族） | desktop |
| 4/5 流程/设计器 | HTML→PDF/图片归档引擎 2 端点（ProcessWork 内联字面路径转正）+ 设计器保存/门户布局 保存·删除 layout（POST/PUT/POST/DELETE 4 端点） | desktop |
| 6 文件/会议 | 会议附件上传(回调)/文件·附件 上传全形态 6 端点（folder 回调×2/assemble JSON/PUT multipart/回调/遗留字面回调） | desktop |
| 7 门户/CMS | CMS 视图发布/取消发布（publish/unpublish 拆字面路径）+ 会议/文件附件回调 | desktop |
| 8 通知/IM 实时 | IM 专属房间频道（`new O2WebSocketClient('/ws/realtime/room/{id}')`）+ 房间在线统计（HTTP GET stats）+ 默认 `/ws/realtime` 识别 | desktop |

### 度量基础设施（诚实收口的关键前置）

1. **同 handler 双计放宽（rev486）落地**：`_classify_unconsumed.py` 新增「同 handler_q 另一注册已消费 → TWIN_ALIAS 结构孪生」判定（method/path/别名三类），一次收编 39 条。
2. **桶外裁定记档**：`EXTERNAL_IDP` 改判「桶外（外部消费者）」，逐条 reason 记档。
3. **WS 真实连接识别（裁定原则1）**：`extract_calls.py` 新增 `WS_DEFAULT_RE`/`WS_CALL_RE`，把 SDK `useWebSocket()` 默认 URL 与 `new O2WebSocketClient('/ws/...')` 字面量识别为 GET 消费（非放水，只识别既存真实连接）。
4. **测试桩剔除**：`extract_routes.py` 排除 `tests*/testing*` 桩文件（`shared/src/testing.rs` 的 `POST /api/reset` 等 5 条桩不再计入生产分母）。
5. **契约 fixture 再生**：`gen_fixture.py` + 方法 fixture 随 `backend_routes.json` 同步（4108 路径），`desktop-endpoints`/`route-methods` 守卫转绿。
6. **schema 门两处真修复（非放水）**：① 跨 crate 挂接 handler（org_auth 挂 `auth::login`）按 `handler_q` 的 `<外部crate>::` 前缀精确解析，消除 A 假红；② `consumes_whole` 传播到委托链末端 + 识别 `body.to_string()` 方法式整体消费，消除 D 假红。
7. **隐藏真实调用转正**：`ProcessWork.vue` 的 `${SB}`/`${SP}` 常量前缀（19 条）内联为字面路径——此前静态对账不可见，属「真实消费被漏计」，内联后诚实计入。

### 后端手术（删空壳/畸形/敏感路由，配套再生）

- 删 program_center `test/test1`·`test/test2` 字面空壳、org_auth `mpweixin/menu/test/send/to/{person}` 空壳（十类功能裁定原则4：字面空壳删除、真实现保留消费）。
- 敏感重设计：`GET /share/{id}/password/{password}`·`GET /file/share/{id}/password/{password}`（口令入 URL）→ 新 `POST /file/share/password/verify`（body 承载），`FileManager` 改调新版，旧 GET 删除。
- attendance `v2/mobile/check/%20from/out` 保留（与空格形态同 handler，二者均真实可达，记档结构孪生，不删）。
- 配套再生：mcp `generated_routes.rs`、openapi `lib.rs`、parity `generated_tests.rs`、file crate `tests_generated.rs`（外科同步，非全量重生，避免行尾漂移噪声）。

### 性能

- 测试管线：`vitest.config.ts` 开启 `fsModuleCache`（持久化 transform 到 `node_modules/.vitest-cache`），全量冷跑 transform 45s+ → 热跑 ~1s，CI 冷启动 job 直接受益；测试语义不变。
- 后端（运行时真优化，EXPLAIN 实证）：新增迁移 `108_create_x_attendance_detail_indexes.sql`——考勤明细 `x_attendance_detail` 此前**零二级索引**，而 attendance 热路径全部是 `WHERE person_id = ? [AND date =/LIKE/范围] ORDER BY date`（统计/列表/单日/区间四族）。补齐复合索引 `idx_x_attendance_detail_person_date (person_id, date)`（103 方法学：每条索引对应 handler 真实谓词组合）。
  - 实证（本地 PG，36500 行 / 100 人演示数据）：统计查询 `EXPLAIN ANALYZE` 由 **Seq Scan 3.19ms** → **Bitmap Index Scan 0.198ms**（~16×）；个人列表由 **Seq Scan+全表 Sort 2.37ms** → **Index Scan Backward 0.048ms**（~49×，排序节点消失）。索引 `IF NOT EXISTS` 幂等，回滚文件配套。
- 其余 crate 扫描确认无 N+1 行级循环、无冗余多 COUNT、连接获取为「读/事务」两阶段（非泄漏）。

### 后端验证（本机工具链打通后的首次全量核验）

- 工具链：`stable-x86_64-pc-windows-gnu`（rustup 1.99.0）；GNU 目标所需的
  binutils（dlltool/objdump/as/ld/ar）与 gcc 16.2 运行时经 MSYS2 官方包 vendor 至
  `oa4rust/vendor/mingw-bin`（binutils 系）与 `oa4rust/vendor/mingw-gcc`（gcc 16.2 +
  libgcc_s_seh/libwinpthread），PATH 前置后可离线编译，无需 MSVC。
- 本次改动的 7 crate（file_assemble_control / program_center / organization_assemble_authentication /
  attendance_assemble_control / parity / mcp_server / openapi）：
  - `cargo check`：**CARGO_EXIT=0**（全图编译通过）。
  - `cargo test`（无 DB）：**4563 通过 / 0 失败**（DB 依赖用例按 `is_db_available` 守卫忽略）。
  - 本地 PG（o2server/oa4rust，`--migrate-only` 建 379 表）后 `cargo test -p parity --lib`：**4143/4143 全绿**——此前"空库无表 → 500"的 41 个行为契约测试（ai/calendar 等）连上已迁移库后全部真实通过，行为契约得证。
  - 附带修复两处测试：`file_assemble_control/tests_u2.rs` 的 SENSITIVE 旧 GET 密码探针改探新 `POST /api/file/share/password/verify`（契约随重设计更新）；`mcp_server/tests/arity_probe.rs` 相对路径少算一级 `..`（预存 bug，指向不存在的 `oa4rust/docs/`）修为仓库根 `../../../../docs/`，全路由 arity 探针恢复可运行（14s 全量校验通过，覆盖本轮路由增删）。

### 复现命令

```bash
cd docs/audits/three-ends-2026-09-20
PY="C:/Users/user/.local/bin/python3.12.exe"   # 或任意 python3.12
$PY extract_routes.py && $PY extract_calls.py && $PY compare.py --gate   # 对账门禁
$PY _classify_unconsumed.py && $PY consumption_gap.py                   # 台账 + 诚实口径
$PY schema_audit.py --gate                                              # 契约门禁
$PY gen_fixture.py                                                      # 再生桌面/移动契约 fixture
cd ../../../oa4rust && pnpm test && pnpm test:mobile && pnpm typecheck && pnpm build

# 后端核验（GNU 工具链；PATH 需前置 vendor\mingw-gcc\mingw64\bin + vendor\mingw-bin）
cd oa4rust/backend
cargo check -p file_assemble_control -p program_center -p organization_assemble_authentication \
            -p attendance_assemble_control -p parity -p mcp_server -p openapi        # CARGO_EXIT=0
cargo test  -p file_assemble_control -p program_center -p organization_assemble_authentication \
            -p attendance_assemble_control -p mcp_server -p openapi -p parity --lib --no-fail-fast
# parity 行为契约需真实 PG：建 o2server/oa4rust 库 + 迁移建表（CI 等价步骤）：
cargo run --bin oa4rust -- --migrate-only   # DATABASE_URL=postgres://o2server:password@localhost:5432/oa4rust
cargo test -p parity --lib                    # 已迁移库上 4143/4143
```

