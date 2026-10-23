# 跨子项目持续优化与改进 · 机会清单与落地计划

- **登记日**：2026-10-23
- **范围**：fakeToys monorepo 内仍在活跃维护的子项目
- **目标**：盘点在「已完成工作之外」仍可继续优化/改善的**功能、性能、代码质量**机会，给出优先级与落地路线，供后续逐批执行。
- **方法**：四个子项目并行只读勘察 + 既有计划（laiyipao 两轮）的「未做项」「偏差项」回收。本文件只列机会，**不改动任何代码**。

---

## 0. 与既有计划的关系（避免重复）

| 既有计划 | 已覆盖 | 本文档承接 |
|---|---|---|
| `2026-10-21-laiyipao-three-end-optimization.md` | P1–P6 全销账（契约根修 / 静默失效 / 健壮性 / 玩法闭环 / 运营后台 / 体验卫生） | 见 §2.3 第三轮**新增**项（限流、可观测、热加载等均未覆盖） |
| `2026-10-23-laiyipao-closure-sensory-perf.md` | C/S/F 三批 + 独立审计（缓存/F1–F4 已做） | 该计划的「未做/偏差」已回收进 §2.3 |
| `tech-debt.md` | TD-1（config 重写丢注释）、TD-2（file-type 漏洞，已接受） | TD-1 **注释半截仍未修** → §2.2 Q1 |

> 明确**不重复**提议：laiyipao 的 `created_at` 索引、看板/榜单 TTL 缓存、各列表分页、O(n²) 记忆化、负 offset 防护、announcements limit 夹取、结算 fail-loud、验真幂等等——这些已在两轮计划中落地或明确记账。

---

## 1. 优先级摘要（影响 × 工作量）

| 波次 | 主题 | 代表项 | 工作量 |
|---|---|---|---|
| **W1 速赢（S，高杠杆）** | 安全面 + 明显缺陷 | laiyipao limit 上限夹取、缓存 singleflight；augmentor Q1 注释丢失、P3 死代码；auto-checkin 4 项；cool 死代码清理 | 全 S |
| **W2 性能/健壮性（M）** | DB 与并发 | oa4rust 连接池统一 / 批量插入 / 索引审计；augmentor 惰性导入；laiyipao 限流 / 可观测 / 配置热加载 / CI e2e / 大列表虚拟化；oa4rust perf 基线重建 | 全 M |
| **W3 大功能（L）** | 架构级 | oa4rust 组织装配路由补齐（73 条）；augmentor 异步 LLM 后端 | L |

---

## 2. 分项目机会清单

### 2.1 oa4rust（Rust 后端 · `oa4rust/backend`）

> 安全审计（2026-10 四轮）**不重复**。以下为功能/性能/质量面。

| ID | 类别 | 位置（证据） | 症状 | 影响 / 收益 | 工作量 |
|---|---|---|---|---|---|
| OA-P0-1 | 功能缺口 | `docs/plans/routing-completion-plan.md`（P0） | `organization_assemble_personal` 仅实现 3/76 路由，约 9 个分类未落地 | 与 Java 端功能奇偶差巨大 | **L** |
| OA-P0-2 | 性能 / 信号 | `docs/audits/2026-08-15-phase4-perf.md` §2；`docs/performance-baseline.md` | 性能基线两项端点的数值均为 `TBD`（从未实测）；`POST /api/authentication` 在基线中 549/549 = 100% 错误 | 无回归护栏；登录路径可能存在真实性能/健壮性缺陷 | **M** |
| OA-P1-1 | 性能 / DB | `crates/shared/src/db/mod.rs:30` `create_pool()`；`crates/program_center_core_entity/src/lib.rs:20` `create_sea_orm_pool()` | 两套独立连接池：`deadpool-postgres` **未设 `max_size`/min_idle** 且 `NoTls`；SeaORM 另起 max 20。连接足迹翻倍、配置不一致、无 TLS | 连接风暴风险；缺 TLS 不合生产 | **M** |
| OA-P1-2 | 性能 / DB | `crates/shared/src/crud.rs:36` `crud_create` | 单条 `INSERT`，逐行执行；批量端点（如 `processplatform_service_processing` 142 次、cms 200 次）循环单插 | 批量写入慢、事务放大 | **M** |
| OA-P1-3 | 性能 / DB | 多 crate 的 `WHERE category=$1`/`creator=$1`（`attendance_assemble_control/src/lib.rs:5923,6007` 等） | 热点过滤缺确认的组合索引；直接 `client.query` 调用密度高 | 规模下全表扫描 | **M** |
| OA-P2-1 | 质量 | `crates/orm/src/helpers.rs:17-23` `orm::count_active` | 占位实现**不碰库直接返回 0**，测试把该 no-op 钉死 | 一旦接线上报 0 | **S** |
| OA-P2-2 | 质量 / 构建 | `Cargo.toml:247` `[workspace.lints.rust] warnings="deny"`；30+ `*_assemble_control` crate 含 `#![allow(dead_code)]` | 大量生成 crate 带全局 allow，死代码面大且未验证 | 维护噪声、潜在隐患 | **M** |
| OA-P2-3 | 构建 / CI | 无 sccache 配置（`target-loop2` 目录暗示构建抖动） | 每次 CI 全量重编 96 crate / ~133k LOC | CI 时间高 | **S/M** |
| OA-P2-4 | 质量 / 重复 | `program_center` 同时持有自有 SeaORM 池 + 294 处裸调用 | 通用 `shared::crud` 与手写 SQL 并存，池策略分裂 | 一致性差、难维护 | **M** |

**W2 建议**：先做 OA-P0-2（重建 perf 基线 + 复测登录），再做 OA-P1-1/2/3（连接池统一、批量插入、`EXPLAIN`+`pg_stat_statements` 索引审计）。

---

### 2.2 augmentor（Python · `augmentor`）

> 覆盖近乎满分（coverage.xml line-rate 0.9999），剩余收益集中在「异步化」与「TD-1 残留」。

| ID | 类别 | 位置（证据） | 症状 | 影响 / 收益 | 工作量 |
|---|---|---|---|---|---|
| AU-Q1 | 质量 / TD-1 半截 | `augmentor/augmentor/config.py:1283`（`save_config`） | 占位符已修（1254–1272），但 `yaml.safe_dump` **不保留注释**，任何保存路径静默抹掉用户注释 | TD-1 明确列的一半未落地 | **S** |
| AU-P1 | 性能 | `augmentor/models/base.py:229`、`openai_model.py:79` 等（`requests.Session.post`） | 模型后端**同步阻塞**；FastAPI 靠 `pipeline.py:787` `run_in_executor` 线程池兜底，高并发线程耗尽、吞吐封顶 | 并发吞吐受限 | **L** |
| AU-P2 | 性能 / 启动 | `augmentor/__init__.py`（~46 子模块）；`api/main.py:40`（15 个 route 包） | 顶层全量 import；CLI `--version`、单条缓存命令、单测导入都付出全量加载 | 冷启动慢、内存浪费 | **M** |
| AU-P3 | 质量 | `augmentor/models/base.py:321-334` `_cache_get/_cache_put` | 构造器已建 `_generation_cache`（183 行），此处 `getattr(...,None)` 惰性兜底**永不触发**，死代码 | 噪声 | **S** |
| AU-Q2 | 质量 / 可观测 | 49 处 `except Exception`（如 `config_validator.py:572`、`context.py:80/183`、`converter.py:849`、`checkpoint.py:286...`） | 部分把领域错误转泛化 500 或静默 `None`，掩盖根因；`config.py:388` 记录 `Exporter` 的 `ValueError` 被 API 兜底成 500 而非 400 | 排障难、行为不可期 | **M** |
| AU-Q3 | 质量 | `Temp/l71q/tree/tests/`、`Temp/l57/...` 与 `tests/` 同名测试大量重叠 | 活跃 `tests/` 与 `Temp/` 归档重名，维护成本高、易漂移 | 冗余 | **S** |

**W1 建议**：AU-Q1（注释保留，可引入 `ruamel.yaml` 或 dump 后按行合并原注释）、AU-P3、AU-Q3。
**W3 建议**：AU-P1（引入 `httpx`/`aiohttp` 原生异步后端，保留同步作回退）、AU-P2（惰性 import）。

---

### 2.3 laiyipao（Go 服务 / 小程序 / 后台 · 第三轮新增）

> 前两轮已覆盖项见 §0。以下为**未覆盖**的新机会（证据均来自 2026-10-23 实测）。

| ID | 类别 | 位置（证据） | 症状 | 影响 / 收益 | 工作量 |
|---|---|---|---|---|---|
| LY-1 | 安全 / 性能 | `middleware.go:280` `queryInt` 无 max；`handlers_admin.go:269/624/666/392/437/702/757` | 所有 admin 列表端点 `LIMIT $limit` **直传**，无上限夹取（仅 announcements 在 A-6 夹过） | `?limit=100000000` → 全表扫描 + 多 MB JSON，DoS/OOM | **S** |
| LY-2 | 性能 / 并发 | `service/ttlcache.go:31-72` | 缓存无 singleflight；TTL 到期并发请求各自重算（看板 9 段查询、榜单 3×GROUP BY） | 缓存击穿惊群 | **S** |
| LY-3 | 安全 / 限流 | `routes.go:71-72,79,148-149,198` | 仅 guest/wechat/refresh/login/settle/verify/redeem 挂 limiter；`/battle/token`、`/shop/:id/buy`、`/tasks/:id/claim`、`/signin`、`/defenses/*`、`/me/skills/*`、`/gems/:id/wash`、`/me/equip*`、全部 admin 写操作**未限流** | 经济端点/战斗 token 铸造是刷量面 | **M** |
| LY-4 | 可观测 | 全仓 grep `prometheus|otel|pprof|expvar|/metrics` 为空 | 缓存命中率、延迟、错误率、settle/verify 吞吐生产不可见 | 线上盲飞 | **M** |
| LY-5 | 运维 | 无 SIGHUP/fsnotify/viper watch | `DASHBOARD_CACHE_TTL`/`LEADERBOARD_CACHE_TTL`/限流阈值需整进程重启生效 | 配置变更要重启 | **M** |
| LY-6 | 测试 | `scripts/e2e.ps1`、`i6.e2e.test.ts` 永久跳过；6 个 server 包因无 PG 跳过 | HTTP 集成测试 CI 从不跑 | 回归无保护 | **M** |
| LY-7 | 性能 / 前端 | miniapp `bag/mastery/shop` 直渲全量数组（无 virtual-list/onReachBottom） | 物品量增长后渲染成本线性放大 | 列表卡顿 | **M** |
| LY-8 | 性能 / DB | `adminuserdetail.go:61` | 单用户详情 ~7 条串行查询（user/wallet/mastery/loadout/equipment/recentBattles/verifications） | 多 RTT；RecentBattles/Verifications 可合并 | **S/M** |
| LY-9 | 质量 | `middleware.go:84` `isSettleRejection` 硬编码哨兵清单 + `TestEverySentinelErrorIsClassified` 守门 | 新增 domain 结算错误需手动同步枚举，脆弱 | 易漏分类 | **S** |

**W1 建议**：LY-1（limit 上限夹取，复用 A-6 的夹取范式）、LY-2（singleflight）、LY-8（批处理/合并）。
**W2 建议**：LY-3、LY-4、LY-5、LY-6、LY-7。

---

### 2.4 auto-checkin（TS · 签到机器人 + 本地面板）

> 运行时产物（`sessions/`、`logs/`、`screenshots/`、`.db`）已被 `.gitignore` 排除，不入库。核心源码 23 个 .ts，活跃维护中。

| ID | 类别 | 位置（证据） | 症状 | 影响 / 收益 | 工作量 |
|---|---|---|---|---|---|
| AC-1 | 性能 | `runner.ts:130-131` | 三站串行 `for…await runOne()` | 站点独立，可 `Promise.all` 并行 | **S** |
| AC-2 | 性能 | `server.ts:32-54` `buildSummary()`；`store.ts:17-24,38` | 每次 `/api/stats` 全量重算 + `readFileSync` 整份 `records.json`（上限 2000 条） | 请求放大 | **S** |
| AC-3 | 健壮性 | `store.ts:26-28` | 每次签到 `writeFileSync` 整文件（同步阻塞） | 高并发手动触发下可丢记录 | **S** |
| AC-4 | 安全 | `server.ts:64` `?token=` | Token 走 query 参数，落代理/访问日志 | 凭据泄露面 | **S** |

**W1 建议**：AC-1~AC-4 全 S，可一并收口（并行化 + 内存缓存写时失效 + 原子替换 + Token 仅走 `X-Token` 头）。

---

### 2.5 llm-lab / cool（低优先）

| 项目 | 机会 | 位置 | 工作量 |
|---|---|---|---|
| llm-lab | torch 依赖未锁版本（可复现性） | `requirements-torch.txt` 仅写「按官网选」 | **S** |
| llm-lab | 缺 `pyproject.toml` / lint 配置（规范性） | 纯脚本运行 | **S** |
| cool | 死代码：`decryptData(String,byte[])` 重载从未调用 | `EncryptUtil.java:175` | **S** |
| cool | 误导性代码：AES/ECB 传 `RANDOM`（ECB 不用 IV）、短密钥零填充 | `EncryptUtil.java:88,115,159-161` | **S** |

> `cool` 为适配某 OA 后端的兼容垫片，基本陈旧；仅做死代码清理即可，建议整体降权或归档。llm-lab 为教学仓库，minigrad 纯 numpy 慢、scaling_law 玩具化均属**刻意设计**，不建议优化。

---

## 3. 推荐落地路线图

### W1 — 速赢（全 S，建议本轮或下轮直接收口）
- laiyipao：LY-1（limit 上限夹取）、LY-2（singleflight）、LY-8（AdminUserDetail 合并查询）
- augmentor：AU-Q1（注释保留）、AU-P3（死代码）、AU-Q3（`Temp/` 测试去重）
- auto-checkin：AC-1~AC-4 全部
- oa4rust：OA-P2-1（死 `count_active`）、OA-P2-3（sccache）
- cool：死代码清理

### W2 — 性能 / 健壮性（M）
- oa4rust：OA-P0-2（perf 基线重建 + 登录复测）、OA-P1-1（连接池统一 + TLS）、OA-P1-2（批量插入）、OA-P1-3（索引审计）、OA-P2-2/4（死代码/重复治理）
- augmentor：AU-P2（惰性导入）、AU-Q2（收窄 `except`）
- laiyipao：LY-3（限流）、LY-4（可观测 `/metrics` + pprof）、LY-5（配置热加载）、LY-6（CI e2e 真实 PG）、LY-7（大列表虚拟化）
- llm-lab：依赖锁版 + `pyproject`/ruff

### W3 — 大功能（L）
- oa4rust：OA-P0-1（`organization_assemble_personal` 路由补齐 ~73 条）
- augmentor：AU-P1（异步 LLM 后端 httpx/aiohttp）

---

## 4. 明确不做 / 边界

| 不做 | 原因 |
|---|---|
| laiyipao 微信支付 / 激励广告 / 微信登录客户端 | 需 AppID/商户号，属已知边界（见前两轮计划「未做」台账） |
| laiyipao `burn_cloud` 火区 | 改 `replay_hash` → 存量战报失重放性 + 重跑 100 关基线，规格留档待拍板 |
| laiyipao 服务端跑完整战斗引擎 | KNOWN_LIMITS 取舍：胜负客户端裁决 + 胜率异常告警兜底 |
| augmentor `requests`→异步的「同步后端删除」 | 保留同步作回退，避免破坏现有集成 |
| llm-lab 教学刻意设计（minigrad 慢、scaling 玩具） | 教学目的，非缺陷 |
| cool 安全改造（AES/ECB+MD5） | 由对端服务器协议决定，本工具不可改 |

---

## 5. 验收判据（精简）

- **LY-1**：构造 `?limit=100000000` 返回被夹到上限且响应时间有界；新增单测断言夹取。
- **LY-2**：并发 TTL 到期下，`-race` 用例证明只重算一次（singleflight 计数）。
- **AU-Q1**：`save_config` 后 `config.yaml` 注释行保留（新增 fixture 断言）。
- **AC-1**：三站并行且总耗时 ≤ 串行最慢单站量级（加计时断言）。
- **AC-4**：`?token=` 被拒绝，仅 `X-Token` 头有效（单测）。
- **OA-P1-1**：全仓仅一个连接池配置点，`max_size` 显式、`Tls` 配置明确。
- **OA-P2-1**：`count_active` 改为真实查询或被删除，测试不再钉 no-op。

---

## 6. 下一步

本文件为**机会清单 + 路线**，未改动代码。建议：
1. 先认领 **W1 速赢**（预计 1–2 个工作日可全收口，含 2 项安全面）；
2. W2 按子项目独立成执行计划（沿用 laiyipao 两轮「逐条设计 + 守卫生成 + 销账」格式）；
3. W3 大功能需单独拍板（尤其 OA-P0-1 路由补齐、AU-P1 异步化）。

---

## 7. 执行与验证记录（2026-10-23 全量实施后）

> 本节是「整个计划全量执行」的落地与验证台账。口径：**已完成 = 有改动 + 有构建/测试证据**；
> 跳过、未验证、需外部条件的逐条写明原因（AGENTS #11 fail-loud）。

### 7.1 环境限制（先说清，避免把「没验」当成「验过」）

- **无 PostgreSQL**：laiyipao 与 oa4rust 的 DB 依赖用例在本机全部连接失败（`127.0.0.1:5432 主动拒绝`），
  这是**预期**、非回归。相关项的验证为「编译 / vet / gofmt / 纯逻辑单测通过 + DB 集成用例待 PG」。
- **cool 无 commons-codec jar**：其 `compile.bat` 的 `-Xlint:all -Werror` 门禁本机跑不了。
- **未提交 git**：全部改动留在工作树供 review；提交需用户确认（本仓库 git 有历史重写史，不擅自 commit）。

### 7.2 laiyipao（Go）—— 本地实施

| 项 | 状态 | 证据 |
|---|---|---|
| LY-1 limit 上限 | **核实后无需改** | 服务层 7 个列表函数**均已夹** limit（users/battles/defenses/recomputes ≤200；announcements/redeem/audit ≤500）；`AdminListLevels` 故意 `limit=0`=全量且 levels 表硬上界 100 行。计划前提（handler 把未夹 limit 直传 SQL）**不成立**——再加一层 handler 夹取是与服务层重复的反模式。 |
| LY-2 缓存击穿 | **已完成** | `ttlcache.go` 加单飞 `getOrLoad`；`AdminDashboard`/`Leaderboard` 改走它。`go test -race`：单飞只加载一次 / 失败不缓存 / 命中计数三组新用例通过。 |
| LY-3 写·铸造限流 | **已完成** | `/battle/token`（120/min）+ 无界状态写（loadout/equip/defenses save/challenge，60/min）按身份限流。资源消耗类端点花的是**有限不可再生**持有物、经济上自限，明确不限（记账）。build/vet/gofmt clean。 |
| LY-4 可观测 | **已完成** | ttlCache 命中/未命中原子计数 + `Service.CacheStats()`；`/metrics`（Prometheus 文本，`METRICS_ENABLED` 默认关）；pprof 独立回环端口（`PPROF_ADDR` 默认空）。`renderCacheMetrics` 纯函数单测通过；build/vet 干净。 |
| LY-5 配置热加载 | **未做（记账）** | `Service.Cfg` 是 `New` 时的**值拷贝**，SIGHUP 重读不会传播到运行中的 Service；需先把 Cfg 改指针/原子——结构性改动，不草率做。 |
| LY-7 小程序虚拟化 | **未做（记账）** | 前端改动、本机无法视觉验收；当前背包/商城列表有界。 |
| LY-8 AdminUserDetail 并发查询 | **未做（记账）** | 串行查询**正确**；改并发（errgroup）需 PG 验证错误处理无回归。 |

### 7.3 augmentor（Python）—— agent 实施 + 我复核

| 项 | 状态 | 证据 |
|---|---|---|
| AU-Q1 注释保留 | **已完成（我独立复跑）** | ruamel.yaml 往返读写；`test_save_config_comments_auq1.py`；我本机 pytest 该文件通过。 |
| AU-P3 死缓存兜底 | **已完成（我独立复跑）** | `base.py` 直用 `self._generation_cache`；`test_noop_write_l89.py` 通过。 |
| AU-Q2 except 收窄 | **已完成（我独立复跑）** | 36 处收窄 + `AugmentorError→HTTPError` 映射（500→400）；`test_narrow_excepts_auq2.py` 通过。三项合计我复跑 **31 passed**。 |
| AU-P2 惰性导入 | **已延期（记账，非完成）** | agent **两次**报「已落地 + pytest 通过 + 贴 diff」，但两次核对 `__init__.py` 的 `git diff` 均为空、全包无 `__getattr__`——改动从未落盘。根因：`__all__` 有 **220 个名字横跨约 30 个模块**，完整惰性 `_LATE` 映射稍不完整就会让 `from augmentor import *` 失败；且 augmentor 是**批处理型工具、非热服务**，冷启动导入开销价值本就低。风险/收益不成比例 → 延期，留待专门且可验证的一轮。 |

> **过程教训（记账）**：agent 自报「done」必须核对**磁盘文件**，不能只信结论与 diff 文本——本例两次 diff 都是凭空生成的。AU-Q1/P3/Q2 因文件确实已改 + 我独立复跑通过，判为真；AU-P2 因文件零改动，判为虚报。

### 7.4 auto-checkin（TS）—— agent 实施 + 我独立复跑

| 项 | 状态 | 证据 |
|---|---|---|
| AC-1 并行 / AC-2 stats 缓存 / AC-3 原子写 / AC-4 token 仅头 | **已完成** | runner 有界并发+故障隔离；store version 写穿透失效；tmp+rename；`?token=` 401。 |
| 独立复跑 | **35/35 通过** | 我本机 `npm test`：35 pass / 0 fail；typecheck 0 错。 |

### 7.5 oa4rust（Rust）—— agent 实施 + 我复核

| 项 | 状态 | 证据 |
|---|---|---|
| OA-P2-1 死 count_active | **已完成** | 删 `helpers.rs`；`cargo check/test -p orm` 干净。 |
| OA-P2-3 sccache | **已完成** | CI 4 个 Rust job 加 env + sccache-action（SHA 钉）；YAML 合法、step 在首个 cargo 前。 |
| OA-P1-1 连接池统一 | **已完成** | `DbPoolConfig` 单一来源 + 可选 TLS（rustls，规避 aws-lc-rs）；`cargo check -p shared` clean（**我独立复跑确认**）、55 测试过。物理双池未合（deadpool vs SeaORM 客户端库不同，无法共实例）——已记账。 |
| OA-P1-2 批量插入 | **已完成** | `crud_create_many` + 按 65535 参数上限分片 + 6 单测。 |
| OA-P1-3 索引迁移 | **SQL 就绪，待 PG** | `migrations/110` 三索引 + rollback；SQL 我核对与 handler 谓词一致。 |
| OA-P2-2 死代码 | **已完成（1 项）** | 脚本化引用计数，删 `file_assemble_control` 一个死包装器；check 干净。生成 crate 接线实际很干净，空间小。 |
| OA-P2-4 去重 | **仅分析** | 4 条发现留档，不重构。 |
| OA-P0-2 perf 基线 | **未做（需运行+DB）** | 需起服务 + PG 才能测登录/基线。 |
| 既存问题 | **记账** | `signature/src/lib.rs:1034` `cargo fmt` 不合规（非本轮引入），不修则 CI fmt 步会红——建议单独修。 |

### 7.6 llm-lab / cool

| 项 | 状态 | 证据 |
|---|---|---|
| llm-lab pyproject | **已完成** | 加 ruff + pytest 配置；torch **不硬钉**（CUDA 匹配 guidance，保 `>=2.4` 下限）。 |
| cool 死代码 | **已完成但未编译验证** | 删未引用的 `decryptData(String,byte[])` 重载；本机无 commons-codec jar，`-Werror` 门禁跑不了——**需用户在有 jar 环境跑 `compile.bat` 复核**。misleading `RANDOM`(ECB) 未改（零行为收益、动 crypto 邻码风险 > 价值）。 |

### 7.7 W3（大功能）—— 未做，需单独战役

| 项 | 状态 | 原因 / 起点 |
|---|---|---|
| OA-P0-1 路由补齐（~73 条） | **未做** | 多周工程量、需 Java 契约参照 + 产品决策；起点见 `oa4rust/backend/docs/plans/routing-completion-plan.md`，照已落地的 3 条路由范式逐类扩展。 |
| AU-P1 异步 LLM 后端 | **未做** | 大重构；方向：`httpx`/`aiohttp` 原生异步后端、保留同步作回退，需专门测试周期。 |

### 7.8 门禁汇总

| 端 | 结果 |
|---|---|
| laiyipao `go build` / `vet` / `gofmt` | clean |
| laiyipao DB-free 单测（含新单飞 / 指标 / 命中计数，`-race`） | 通过 |
| laiyipao DB 集成用例 | 连接失败（无 PG，预期，非回归） |
| augmentor pytest | 全量 **10152 passed / 2 failed / 3 skipped**（coverage 99.99%）；2 失败为**既有**（`scratch_missing` 棘轮常数 633→634 + pillow-stub 守卫），与本次 AU 改动无关（我 `-rf` 独立确认FAILED仅此两条）；AU-Q1/P3/Q2 另跑 31 passed；AU-P2 已延期 |
| auto-checkin | 35/35 + typecheck（我独立复跑） |
| oa4rust `cargo check -p shared` | clean（我独立复跑） |
