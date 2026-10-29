# 剩余工作项规划（2026-10-29）

> **用途**：把散落在安全审计账本、覆盖率计划、优化台账里的剩余项收敛成一份可执行清单。
> **口径**：每项都写「判据 / 落点 / 规模 / 验收 / 依赖」；判据以**复跑现量**为准，
> 账本记载只作线索（§四 记了一个账本已过时的反例）。

---

## 一、当前基线

| 项 | 状态 |
|---|---|
| 全仓门禁 `node scripts/check-warnings.mjs` | **12 通过 / 0 失败 / 1 跳过**（跳过项见 D1） |
| 今日推送 | 13 个提交（`325296f38 → 500259e94`） |
| auto-checkin | 102 passed；覆盖率棘轮九档全绿 |
| laiyipao | service / httpapi / cmd / config 真库全绿；配置热加载已落地 |
| augmentor | 10173 passed / 4 failed（4 条为基线既红：pillow 环境性 + 3 条棘轮）；web 416 tests 全绿 |
| oa4rust | 97 crates 编译零告警；`shared` 覆盖率 **93.28%**（floor 93 已达成） |
| oa4rust 定位 | **独立系统**，不再与 Java 对齐（CI 的 Java 双栈对比门禁已删，换成启动冒烟） |

**已关闭、不要再排期**：OA-P0-1（Java 契约奇偶，前提消失）、LY-5（配置热加载，已落地）、
LY-7（小程序虚拟化，实测列表全部有界）、AU-P1（异步后端，传输层 + 消费者 + API/CLI/前端均已接）。

---

## 二、剩余项

### A 类 · oa4rust 安全收口（判据：写门禁尺子）

尺子：`oa4rust/backend/scripts/audit_write_gating.mjs`（复跑命令见 §五）。
**先读口径**：这些数字是「尺子的读数」，不是漏洞清单 —— 每一档的判据不同，
引用前必须看该行落在哪一档。

| 编号 | 读数（2026-10-30 复跑） | 判据 | 规模 |
|---|---|---|---|
| **A1** | R4 匿名可达 + 体内无门禁 **13 条** | 路由匿名可达 ∧ handler 体内无任何会话/admin 门 | 含写 SQL 6 / 只读挂写 7 / 体内不可解析 0 |
| **A2** | R6 已认证 + 读会话无门禁 **14 条** | 有会话但只读不校验 | 含写 SQL **0**（已收口，原 134） / 只读挂写 14 |
| **A3** | R7 已认证 + 体内无会话无门禁 **631 条** | 全部为「只读挂写方法」 | 含写 0，即**真写面为 0**，属"方法名带写、体内不写"的收口残留 |
| **A4** | GET 写（csrf / requires_admin 双盲区）**2 条** | GET 路由 ∧ 体内写 SQL ∧ 体内无门禁 | 均为有意保留：企微回调 + 初始化 set_cancel（K1 匿名） |
| **A5** | 豁免通配段碰撞 **0 条** | 「本意给只读详情的 `{param}` 豁免」把写路由一并匿名放行 | 已清零（AUTH_EXEMPT_BLOCKED_PATHS 点名排除） |
| **A6** | 真匿名可达面 **192 条** | auth 豁免 ∧ 注册表 Public | A1 14 / A2 29 / A3 149 |

> **A 类收口状态（2026-10-30）**：A1/A2/A4/A5 均已按「先逐条定性，保留设计上必须
> 匿名的认证前置族（登录/注册/口令重置/系统初始化/企微回调），其余补门或改方法」
> 完成收口。A2 的 R6 残余 14 条已逐条定性：全部是只读挂写方法（list/filter/search
> 挂 POST/PUT，纯 SELECT），含写 SQL 0，无需补门；机器守卫见
> `tests/write_gating_guards.rs` 的 A2③。
>
> **R4 残余 13 条已逐条定性（2026-10-30 第二轮）**：含写 6 条全部是设计上必须匿名的
> 认证前置族——login（契约路径 + 自造别名 + org_auth 契约别名，三条注册点同一
> handler）、`reset_password`/`reset_password_anonymous`（一次性验证码 + 限流）、
> `set`（系统初始化，条件匿名）；只读 7 条为验证码校验/双因素校验/登出/应用列表
> 查询等无写面端点；不可解析 3 条（org_auth 别名）在尺子缺陷 30 修复后全部解析，
> 归入上述含写/只读族。逐条核对 handler 实现确认无超出语义的滥用面。
>
> **本轮新修两个尺子缺陷**：
> - 缺陷 30：`fnBody` 跨 crate 限定符（`auth::login`）解析不到 crate 根 `lib.rs`，
>   org_auth 契约别名委托的写面被读成 A3-只读。补 lib.rs/模块文件回退后，
>   `route_handler_unresolved_refs.tsv` 从 69 行降到 47 行，R4 不可解析 3→0。
> - 缺陷 31：`gateEvidence` 的裸名 helper 只查路由所在 crate，跨 crate 的
>   `check_owner` 门查不到（personal 委托 `empower::delete` 被误读成 K3 无门）。
>   补全局函数索引回退（仅门检测），`check_owner` 作证 8→12，GET 写清单回到 2 条。
>
> **顺带修复真实限流缺口**：org_auth 登录别名不在 `AUTH_RATE_LIMIT_EXACT`，
> `rate_limit_middleware` 对「豁免且未列限流」直接跳过 ⇒ 该别名成为无上限的登录
> 暴力破解入口（契约路径限 30/min/IP，别名不限）。已补入精确名单，并新增
> `org_auth_login_alias_is_rate_limited_like_canonical` parity 测试钉住。

**A 类怎么排**（按"危害 × 收敛速度"）：

1. **A1 + A4 + A5**（14 + 10 + 6 = 30 条）——匿名可达或绕过中间件，是真正的入口面。
   每条都要看 handler 本体：有写 SQL 的补门，纯只读的把 GET 改 POST 或补读门。

   ⚠️ **先逐条定性，不要一律收 admin**：匿名可达里有**本来就该匿名**的族 ——
   登录（`POST /api/authentication`、`/api/authentication/login`）、注册、口令重置
   （`/api/reset`、`/api/reset/password/anonymous`）、以及首次引导的 `/api/secret/set*`
   （收 admin 会直接打断初始化，账本 §三.3 已按"保留匿名 + 堵住探测"处置）。
   对这族要问的是「它能不能被用来做超出自身语义的事」，而不是「它有没有门」。
2. **A2 的 134 条含写 SQL**——已认证但无门禁，危害次一档，但量大，按 crate 递减收。
3. **A3 的 629 条**——**真写面为 0**（含写 SQL 0），本质是"方法名带写、体内不写"的
   收口残留。优先级最低：**不要**按 629 这个数字排期，它不等于 629 个漏洞。
   集中度前 5：`processplatform_assemble_surface` 227、`cms_assemble_control` 66、
   `organization_assemble_control` 43、`program_center` 40、`attendance_assemble_control` 38。
4. **A6 的 192 条**——是"匿名可达"的**全集**，与 A1/A2/A3 有交集，不是新增工作量；
   它适合当**守卫靶面**（新增路由落进这个集合就要有人解释），而不是当清单收。

### B 类 · 机制性缺陷（不是单点漏洞，是"尺子/接线/测试"本身的缺口）

| 编号 | 项 | 判据 / 落点 | 规模 | 依赖 |
|---|---|---|---|---|
| **B1** | 尺子缺陷 28：ORM-blindness | `writeEvidence` 只扫 deadpool SQL 字面量，不识别 sea_orm `ActiveModel::insert/update/delete` ⇒ ORM 型写被读成"只读挂写" | 修一处判据 | 需先用 `general_core_entity` 的 12 把 CRUD 做正反样本 |
| **B2** | 遗留 29：invoke token 零验签 | `/invoke/{flag}/client/{client}/token/{token}/execute` 的 token 只判非空；`x_program_invoke` 表**无 token 存储列** | 1 条路由 + 1 次迁移 | 需拍"token 从哪来"（产品/对接方） |
| **B3** | 遗留 30：`*_core_entity` 接线 | **已实测最小样本**：`file_core_entity::file_core_entity_router(_pool: Pool)` 参数被忽略、全程不挂 `Extension<DatabaseConnection>`，而该 crate 有 7 处 handler 要它 ⇒ 这些路由的 HTTP 面必 500 | 账本记 16 个 crate，**现量 19 个**，需逐 crate 复核（见 §四 反例） | 每 crate 一批，与门禁回归同批 |

> **B3 已复核完成（2026-10-30）**：逐一检查全部 19 个 `*_core_entity` crate 的 router
> 签名与 `Extension` 注入——16 个需要 sea_orm 的 crate 全部为
> `router(db: DatabaseConnection)` 且 `.layer(Extension(db))`；`component_core_entity`
> 与 `query_core_entity` 的 handler 零 DatabaseConnection 引用（纯 deadpool Pool，注入正常）；
> `program_center_core_entity` 的 sea_orm 仅用于 `#[cfg(test)]` mock router，真实路由走
> deadpool Pool + `None` 连接选项（已删空转池创建，见该文件注释）。无漏接。
| **B4** | 遗留 31：bbs `create_reply` 功能桩 | handler 不落库却返回假 success，前端会误信回复已建 | 1 次迁移 + 1 个 handler | 需 `x_bbs_reply` 表设计 |
| **B5** | 6 个游离 `tests.rs` | 文件在、`lib.rs` 无 `mod` 声明 ⇒ 里面的断言**一条都不参与编译**：`console` / `ldap` / `openapi` / `orm` / `processplatform_assemble_surface` / `processplatform_core_express` | 6 个 crate | 注册时需两处返工：`use super::*` 改 `use crate::*`；DB 臂补 `is_db_available()` skip |
| **B6** | 17 条惰性豁免条目 | 豁免表里有、注册表不 Public ⇒ 豁免本身失效（引信不是噪音） | 7 写 + 10 读 | 逐条定性：要么删豁免，要么补 Public 并解释 |
| **B7** | 三把建议守卫 | ①GET 写禁令（`csrf_middleware` 只覆盖 POST/PUT/PATCH/DELETE，`requires_admin` 同）②Public 兜底点名（`ADMIN_WRITE_PREFIXES` ∩ 注册表 Public 的 15 条交集逐条点名）③测试在册 ≠ 测试在跑（B5 的机械守卫） | 3 个测试 | ①需同批改前端（`MeetingApp.vue` 自检面板仍在用 GET） |

> **B7 已全部落地（2026-10-30）**：①+②合并为 `oa4rust/backend/tests/write_gating_guards.rs`
> （纯 std 扫描，与 B7③同风格）：①扫全部 GET 路由的写 SQL + 门信号，豁免表仅
> `qiyeweixin_update_person_detail`（企微回调）与 `set_cancel`（初始化引导）两条；
> ②扫「admin 写前缀 ∩ 注册表 Public」内真写 handler 的体内门，豁免表仅注册/重置
> 认证前置族。判据复刻尺子的 `WRITE_SQL_RE`（含注释剥离）、ORM 写、GATE_RE 前缀
> 匹配、自限定写=归属门（缺陷 26）与死档排除（缺陷 18）。③见 `tests/test_module_registration_guard.rs`。

### C 类 · 覆盖率条件项

| 编号 | 项 | 现状 | 缺口性质 |
|---|---|---|---|
| **C1** | `shared` 覆盖率 93.28% → 94% | floor 93 已达成 | 需 ~50 行，全部依赖**进程隔离**（`security` 的 OnceLock 环境臂与 parity 测试互斥）或**公网夹具**（`netguard fetch_limited` 被自身 SSRF 守卫挡回环）。属条件项，不作为默认目标 |

### D 类 · 环境与工具

| 编号 | 项 | 现状 | 处置 |
|---|---|---|---|
| **D1** | llm-lab pytest 门禁 SKIP | ~~本机 `C:/Python314` 有 pytest；门禁默认解释器没有~~ | **已消解（2026-10-30）**：环境更新后默认 `python` 已带 pytest 9.1.1，门禁 `llm-lab (pytest 行为面)` 真实 PASS（6.7s），不再是 SKIP。门禁脚本的 `requires` 守门会在解释器缺 pytest 时显式 SKIP 并说明，无需改代码 |

---

## 三、建议顺序（每批可独立验收）

| 批次 | 范围 | 验收 |
|---|---|---|
| **第 1 批** | B5（6 个游离 tests.rs 注册）+ B7③（机械守卫） | 6 个 crate 的测试真正参与编译；守卫对"文件在、无 mod"直接红 |
| **第 2 批** | B1（尺子缺陷 28）+ 复跑刷新产物 | 用 `general_core_entity` 的 12 把 CRUD 做正反样本：修前读成只读挂写、修后进"含写" |
| **第 3 批** | A1 + A4 + A5（30 条入口面） | 每条补门/改方法后，尺子对应档位归零或下降；每 crate 补无 DB 的 401 臂 |
| **第 4 批** | B3（`*_core_entity` 接线，逐 crate） | 该 crate 的 HTTP 臂从 500 变 200/401；名实矛盾的旧断言同批改 |
| **第 5 批** | A2 的 134 条含写 SQL（按 crate 递减） | 每 crate 收完，"含写 SQL"计数下降且离线 401 臂通过 |
| **第 6 批** | B2 / B4 / B6（各需一次产品拍板或迁移） | 见各行依赖 |
| **条件项** | C1（覆盖率 94）、A3 的 629 条 | 有资源再说；A3 不按数字排期 |

---

## 四、明确不做 / 需拍板

**不做（已定）**

- OA-P0-1「补 73 条路由」——oa4rust 是独立系统，Java 奇偶不是目标。
- LY-7 小程序虚拟化——实测 `skills` 42 / `equipment` 18 / `gems` 8 / `shop` 15 / `levels` 100（按 6 章分组渲染），全部有界。
- `augmentor` DiskCache 的 O(N²) 短路优化——与"孤儿元数据清理"的既有保证冲突，且用例把
  `_get_cache_path(key).stat()` 钉成了判据，改它等于改判据。

**需拍板**

- `meeting_id_accept` 的会议级状态翻转：被邀请人 / 属主 / admin，谁能做？
  （现只加了有效会话要求，未加归属判定。）
- B2 的 invoke token 来源与轮换策略。
- B4 的 `x_bbs_reply` 表结构。

**账本过时的一个反例（提醒：引用前先复核）**

审计账本 §五 遗留 30 写「`program_center_core_entity` 是唯一正确模式（`pub async fn router` +
`create_sea_orm_pool().await`）」。实测该 crate **现在没有 `router()` 函数**，而
`file_core_entity` 才是能复现问题的最小样本（`router(_pool)` 忽略参数、handler 有 7 处
`Extension<DatabaseConnection>`）。⇒ 该条目的**计数（16）与样本（谁是正确模式）都已漂移**，
排期前必须逐 crate 复核，不要按 16 直接开工。

---

## 五、复跑命令

```bash
# 全仓门禁（12 项）
node scripts/check-warnings.mjs

# 写门禁尺子（会刷新 docs/audits/artifacts/*.tsv，复跑后注意是否需要提交）
cd oa4rust/backend && node scripts/audit_write_gating.mjs

# 单 crate 覆盖率
cd oa4rust/backend && cargo llvm-cov -p shared --lib --summary-only

# 子项目测试
cd auto-checkin && npx tsx --test test/*.test.ts && npm run test:ratchet
cd laiyipao/server && TEST_DATABASE_URL='postgres://user:pass@localhost:5432/laiyipao?sslmode=disable' go test ./internal/...
cd augmentor && python -m pytest -q
cd augmentor/web && npx vitest run
```

> 复跑尺子后若 `artifacts/*.tsv` 有变化，那是**读数与代码不同步**，应单独提交并说明差异性质，
> 不要混进功能提交里（否则"这次改动让读数变了"和"读数本来就旧"分不开）。
