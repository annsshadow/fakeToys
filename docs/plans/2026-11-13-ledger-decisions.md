# 各项目剩余账本 · 拍板记录（2026-11-13 起）

> **这份文件是什么**：把五个子项目账本里**需要人拍板**的条目逐条列出来，
> 一条一条与用户过，**当场记下「谁、何时、选了哪条、为什么」**，再动手实现。
>
> **口径**（承 `docs/plans/2026-10-29-remaining-work-plan.md` §四）：
> **账本记载只作线索，结论以本轮复跑现量为准**；每条决策的**证据**（实测读数）随决策一起记，
> 免得半年后只剩一句「当时说改成那样」。
>
> **只记真需要拍板的**：能按现量自证的、已过时的、已被外部条件挡住的，一律**不占这里的行**
> （它们走「按现量收口」那一路，结论写进各自账本那一行）。
>
> **本轮的新能力**：本机 **docker 可用** ⇒ 真库到位 ⇒ 交接文档里那三条「没有真库所以没法验」
> 的条目**现在能量了**（见 §三）。

---

## 一、议程（按「能不能自证 → 要不要拍板 → 值不值」排序）

| # | 条目 | 归属 | 性质 | 状态 |
|---|---|---|---|---|
| 1 | **A301 + A308**：三份默认目录（备份 / 注册表） | augmentor | 行为变更 vs 写实 | **待拍板**（§二 决策 #1） |
| 2 | **A287**：`similarity_to_a` / `similarity_to_b` 两个恒零键 | augmentor | 接计算（要拍语义）vs 删键（破坏性）vs 写实 | 待拍板 |
| 3 | **A335**：备份回执七键里没有 `name` | augmentor | 响应面 +1 键 vs 维持 | 待拍板 |
| 4 | **A370**：静态闸判据与 `re` 权威的三处口径差 | augmentor | 「误伤算不算对外契约」 | 待拍板 |
| 5 | **P1-1**：attendance 五张配置表写侧只写 `creator_person` | oa4rust | 要不要同时写 `creator`（多写一列数据） | 待拍板（读数已备好，见 §三） |
| 6 | **P0-1**：`x_portal` 共享读臂 | oa4rust | 要不要做「共享给指定人」这个**新需求** | 待拍板（产品面） |
| 7 | KNOWN_LIMITS 第 3 / 12 / 13 / 23 条 | laiyipao | 产品语义 | 待拍板 |
| 8 | KNOWN_LIMITS 第 1 / 2 条 | laiyipao | 要 AppID / 商户号 / AppSecret | **要凭证**（不是拍板） |
| 9 | KNOWN_LIMITS 第 4 / 6 / 8 条 | laiyipao | 要人工 / 真机 | **要人工**（不是拍板） |
| 10 | KNOWN_LIMITS 第 15 条（按波次轮换地形） | laiyipao | 引擎级改动（双端） | 待拍板（工程量级） |

---

## 二、逐条决策

### 决策 #1 · A301 + A308：三个子项目里「三份默认目录」这一族（同根，合并一条）

**条目原文**（卷二 `OPTIMIZATION_LOOP_2.md`）：

- A301：「『默认备份目录』在仓内有**三份**，而端点那一份与另外两份**不是同一个目录**」
- A308：「『默认注册表目录』在仓内有**三份**，而端点那一份与另外两份不是同一个地方」

**本轮实测**（`augmentor`）：

| 面 | 备份目录 | 注册表目录 |
|---|---|---|
| 端点（省略参数） | `<白名单首根>/.backups` = `augmentor/data/.backups` | `<白名单首根>/.dependency_registry` |
| CLI 默认 | `.backups`（**CWD 相对**） | `.dependency_registry`（CWD 相对） |
| SDK 默认 | `.backups`（CWD 相对） | `.dependency_registry`（CWD 相对） |

**症状实测**：SDK 在 CWD 下建出的备份落 `.backups/backup_20261113_115356.json` ⇒
端点默认目录里**有没有它 = False**；SDK 注册的 1 个数据集 ⇒ 端点
`GET /api/system/dependency/datasets` 回 **0 条** ⇒ **CLI/SDK 建的，页面看不见；反之亦然**。
（文档里已给过绕法：`--registry-path data/.dependency_registry`。）

**四个选项**：

| # | 方案 | 代价 |
|---|---|---|
| ① | **CLI 默认改成 `data/.backups` / `data/.dependency_registry`**（仍相对 CWD）⇒ CLI 与 API 在「项目根目录」这一常态下自动同源；**SDK 库级默认不动** | CLI 默认值变了 ⇒ 不传参的脚本落点从 `<根>/.backups` 变 `<根>/data/.backups`（**旧文件仍在原地**、不会被迁移）⇒ 有人会说「昨天建的备份不见了」 |
| ② | 三面全收敛到白名单首根 | **SDK 层没有白名单概念** ⇒ 要把白名单下沉到 SDK（面大、且对库使用者凭空多一个 `data/` 约定）⇒ 不建议 |
| ③ | 维持现状 + 写实（CLI 帮助与文档写明「相对当前工作目录」与那个绕法） | 零行为变更；用户困惑留着，且要**记住**绕法 |
| ④ | 端点**读两处**（`data/.backups` 合并旧 `.backups` 列表），写入仍只写 `data/.backups` | 兼容老落点、不改 CLI 默认；代价是「默认目录」继续有两个 |

**推荐**：**①**（端点侧的白名单是硬约束、SDK 没有白名单概念 ⇒ ② 要么把白名单下沉、要么把「三份」变「两份」；
① 只动 CLI 一处默认值，让**项目内的工具**与 API 同源，SDK 保持库该有的 CWD 相对惯例）。
若要**零破坏**则取 **③**；**④** 可与 ① 一起上（兼容老落点），也可单独上。

**决策**：（待用户拍板）

---

## 三、本轮用真库量出来的三格（**不是**拍板项，是「按现量收口」）

| 条目 | 本轮读数 | 处置 |
|---|---|---|
| oa4rust 交接文档 **P1-2**（Go 侧未复跑门禁 / sabotage） | docker 起 Postgres 16（`auggate-pg`，host 5432，`pg_hba` 改 trust ⇒ 与 CI 的 postgres service 同口径）+ 库 `oa4rust_test` / `oa4rust` / `laiyipao` ⇒ `python scripts/security_regression_gate.py go` ⇒ **cases=29 pass=29 leak=0 unverified=0 both=18**；`cargo test --test security_regression` ⇒ **cases=346 pass=346 leak=0 unverified=0 integrity=0 excluded=2 both=211 fwd-only=121** | **收口**（有读数了；原先那句「本轮无 Postgres ⇒ 没法验」的前提不再成立） |
| oa4rust 交接文档 **P0-2**（`x_message_mass.creator` 从不写入） | 源码里 `mass_create` **已经**同时写 `creator_person` 与 `creator`（两者都取 `session.person_unique`），且仓里**已有**断言 `r7_mass_create_writes_session_creator` ⇒ 首跑报红 `creator = "system"` 是**旧构建产物**（`fakeToys-gate-final` 那棵树的 target 缓存）；`touch` 源文件强制重编后该支 **ok** | **账本过时**（那一格早已修好，且守卫已在仓里）⇒ 交接口径要改 |
| oa4rust 交接文档 **P1-1**（attendance 五张配置表写侧只写 `creator_person`） | 写侧确实**只**写 `creator_person`（`x_attendance_employee_config` / `schedule_setting` / `setting` / `statistical_cycle` 等的 INSERT 列名里没有 `creator`）；读侧谓词是 `owner_scope(&["creator", "creator_person"], "$2")` ⇒ 两列都认 ⇒ **写侧不写 `creator` 不造成读写不一致**，但「归属事实只落一半列」这件事仍在 | **拍板**（议程 #5）：要不要让写侧同时写 `creator`（= 多写一列数据，行为不变） |

**顺带记一笔环境债**（交接文档 P2-1 的第一格，现量仍然成立）：
`utoipa-swagger-ui` 的 build.rs 要从 GitHub 下载 `v5.32.6.zip`，断网时编译失败
⇒ 本轮按文档给的复用办法，从 `target/debug/build/utoipa-swagger-ui-*/out/` 里拷了一份到缺的那几格 ⇒ 编译通过。
