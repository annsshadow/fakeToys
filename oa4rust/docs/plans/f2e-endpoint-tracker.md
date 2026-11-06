// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

# F2-E 越权读端点台账（按端点逐条推进）

状态：`待修` / `已修` / `已证明不可修（需业务裁决）`。
每修一条：先证明泄露（handler 级原文），再改归属过滤，补双向用例，
破坏实验（去掉过滤）必须红，最后独立提交。

| # | 端点 | 归属字段 | 现状 | 提交 |
|---|---|---|---|---|
| E1 | `GET .../attendancedetail/filter/list` | `person_id` | 已修（管理员全公司 / 本人） | F2-1 `fix(attendance)` |
| E1b | `GET .../attendancedetail/filter/list/{id}/next\|prev/{count}` | `person_id` | 已修 | F2-1 |
| E2 | `GET .../attendancedetail/mobile/filter/list/page/{p}/count/{c}` | `person_id` | 已修 | F2-1 |
| E3 | `PUT .../attendanceappealInfo/filter/list/...` | 请求体 `personId` | 待修：与 Session 比对（管理员豁免） | — |
| E4 | `PUT .../attendanceselfholiday/filter/list/...` | 请求体 `personId` | 待修：同上 | — |
| E5 | `PUT .../statisticshow/filter/personMonth/list/...` | 请求体 `personId` | 待修：同上 | — |
| E6 | `PUT /api/mind/.../mind/filter/list/...`（receive/recycle/shared 同族） | `creator` | 待修 | — |
| E7 | `GET /api/message/.../instant/list/...` | `consume`/`sender` | 待修 | — |
| E8 | `GET /api/message/.../consume/list/{consume}/count/{c}` | 路径 `consume` | 待修：等于本人或管理员 | — |
| E9 | `GET /api/meeting/.../attachment/list/{id}/next/{count}` | 会议 owner/参与人 | 待修 | — |
| E10 | `GET /api/meeting/list` | `creator`/参与人 | 待修 | — |
| E11 | `GET /api/ai/chat/list/paging/...` | `person` | 待修 | — |
| E12 | `GET /api/ai/conversation/list` | `user_id` | 待修 | — |
| E13 | `PUT /api/anonymous/document/filter/list/...` | `status`/`creator` | **待修（P0 之后）**：复核后匿名实际 401；真实缺陷是登录用户可读他人 draft；口径 `published OR creator/author_id=本人`，无会话只 `published` | — |
| my-1 | `POST .../draft/list/my/paging/...` | `xperson` | 待修 | — |
| my-2 | `POST .../work/list/my/paging/...` | `creator` | 待修 | — |
| my-3 | `GET .../task/list/my/paging/...`（taskcompleted/read/review/handover 同构） | `person` | 待修 | — |

## 写入侧 P0 / T3（审计第四轮，优先于 E13）

| # | 端点 | 归属/身份字段 | 现状 | 提交 |
|---|---|---|---|---|
| P0-1 | `POST .../record`（attendance_core_entity） | body `user_id` | **已修**：`user_id` 取会话人；请求体传他人 → 非管理员 403；管理员代建显式放行并写 `created_by`（迁移 117）。用例：伪造 403 且不落库 / 无 userId 落会话人 / 管理员代建 200 + 留痕。破坏实验：改回请求体取值 → 断言红（200 vs 403） | 见本批 PR |
| P0-2 | `POST .../record/{id}/update` | `Path(id)` | **已修**：归属谓词写进 SQL WHERE（本人 OR 管理员）。双向用例 A→B 403 / B→A 403 / 本人 200 且落库生效。破坏实验：谓词换成 `$2=$2` → 两个方向都红（200 vs 403） | 见本批 PR |
| P0-3 | `POST .../record/{id}/delete` | `Path(id)` | **已修**：同 P0-2 的 DELETE 版本；0 行时按存在性返回 403/404。破坏实验同上，红 | 见本批 PR |
| P0-4 | 同 crate 其它 `Path(id)` 写端点 | `Path(id)` | **已核查**：`rule/{id}/update`、`rule/{id}/delete`、`rule/create` 无「本人」列（全局配置）→ 收口为管理员（`require_admin`）；`record/list`、`rule/list` 为读端点，登记到读 IDOR 批次 | 见本批 PR |
| P0-5 | `attendance_assemble_control` 默认「今天」 | UTC 日 vs 本地日历列 | **已修**（同族）：`business_today_from_unix(secs + 8h)`，3 处 `.unwrap_or_else(business_today)`；固定时刻用例覆盖早窗进/出与前一自然日 | 见本批 PR |
| P0-b | `processing` / `terminate`（`u4_require_actor`） | actor（可空） | **已修**：空 actor → 仅 admin（`is_admin` 为准，fail-closed），不再 Ok(()) 放行；`processplatform_core_express::work_terminate` 内联副本同改（`filter(!empty)` → `None` 分支仅 admin）；全仓扫 14 处 `require_owner` 同族，其余均为 fail-closed（空归属 → 非 admin 403） | 见本批 PR |
| L1-1 | `bbs_subject_info` `create_subject` | body `author_id` / `creator` | 待修：与 T3-2 同一根，合并做 | — |
| L1-2 | `x_mind_base_info` / `x_mind_folder_info` | body `creator` | 待修：与「硬编码 system」合并做 | — |
| L1-3 | `/anonymous/file/{id}/download` | `Path(id)` | 待修（纯读泄露，最便宜，优先） | — |
| L1-4 | `/api/file/upload` | form `person` | 待修：归属统一到会话人，保留 admin 代传合法路径 | — |
| L1-5 | 流程状态机 | body `status` | 待修（最小拦截）：非法迁移 400；完整状态机另开一轮 | — |
| L1-6 | calendar `x_cal_event` | — | **不改**：写了没人读，待确认是否死表（台账留痕） | — |
| T3-1 | message `sender` | body `sender` | 待修：改取会话人；后果链已实证（revoke 403/200） | — |
| T3-2 | bbs 帖子 `author_id` + `my/list` 的 `creator` | body | 待修：改取会话人；`my/list` 按会话人过滤 | — |
| T3-3 | bbs 回复 `creator` + `author_id` | body | 待修：同上，回复与帖子一起修 | — |
| T3-4 | calendar `createor` | body | 待修（降级）：先确认 `x_cal_event` 是否「写了没人读」的死表 | — |

## 口径约定

- 管理员：`shared::middleware::is_admin` 为真 → 看全公司；
- 普通用户：按上表归属字段等于 `session.person_unique`；
- 请求体/路径里带人员 id 的端点（E3/E4/E5/E8）：普通用户传他人 id → 403；
  管理员可代查；
- `/my/` 语义：无条件按会话人过滤，不做管理员豁免（管理员要看全量走管理端点）。

## 纪律：判据不退化

守卫必须能区分「修好了」和「没修」；断言本身不能依赖运气、真实钟点或环境。

范式（来自 503→401 论证，2026-11-05）：

1. **正向断言**：改后行为（匿名 → 401）。
2. **反证一（路由存在性）**：同一 helper 对未注册路径必须 404 → 证明 401 不是「路由不在也通过」。
3. **反证二（不吞故障）**：注入会话但依赖仍不可用 → 仍 503 → 证明 401 只是优先级，不是掩盖依赖故障。
4. **时间类判据**：注入 `now` 或构造跨边界数据，任何时刻跑都断言同一件事；
   只在特定钟点有效（读墙钟）的守卫等于没有守卫。
5. **破坏实验**：把修复逐条撤回，对应断言必须红；红的位置要落在行为断言上，
   不能只落在常量断言上（否则证明不了行为被覆盖）。
6. **门禁即硬前置**（2026-11-06）：`python scripts/security_regression_gate.py` 未 PASS 即不许合并；
   非 PASS 条数只能等于或少于 `tests/security_regression_baseline.txt`，修好一条必须同步减基线
   （集合相等判定，修了不减也红）。`UNVERIFIED`（404/5xx/种子漂移/2xx 但行未变）**不算已修**。
   门禁已接入 CI `security-regression` job（required check），不依赖个人本地记忆。
   工作树绿 ≠ main 绿：门禁读工作树，修完必须提交、推送、等 CI。
7. **读数必须基于与 `origin/main` 同步且干净的工作树**（2026-11-06）：runner 跑前打印
   `git rev-parse HEAD`、`git status --porcelain` 是否干净、与 `origin/main` 的 ahead/behind；
   **`behind > 0` 直接拒绝运行**，必须先 `git fetch origin main && git merge --ff-only origin/main`
   （或 rebase）再跑。旧代码上出的读数会让基线棘轮判错：修了却判 STALE、没修却以为修了。
   工作树不干净时打印清单；该读数只代表当前工作树，不代表 main，最终以提交/推送后的 CI 读数为准。

## 时区同族扫描（2026-11-06）

背景：`processplatform_service_processing` 维护作业按业务日去重时，用
`date_trunc('day', NOW() AT TIME ZONE 'Asia/Shanghai')` 得到「北京墙钟 naive」，
直接与 `x_record.create_time`（UTC 裸时间）比较 → 北京 00:00–08:00 必现重复插入、
其余时段必不现（按钟点决定红绿）。修法：边界换算到 UTC 再比较，写入时基不动。

| 位置 | 形态 | 判定 | 处置 |
|---|---|---|---|
| `processplatform_service_processing/src/u2.rs`（`loglongdetained` / `touchdelay`） | ① 本地时区算边界 vs naive 列 | **确认为缺陷** | **已修**：`business_day_start_utc(now)`；写入仍 `$2`=UTC naive；用例注入固定 `now` |
| `attendance_assemble_control/src/lib.rs`（默认考勤日，3 处） | ① 反向：UTC 算「今天」vs 本地日历列 | **同族缺陷**（北京 00:00–08:00 默认考勤日错一天） | **已修**（P0 批 PR #187）：`business_today_from_unix(secs + 8h)`，固定 epoch 用例 |
| `bbs_core_entity/src/lib.rs:35-37`（`now_text`，3 处写 `create_time`） | ② 写本地时间，同列另有 UTC 写入路径（`bbs_assemble_control::now_iso`） | **同族缺陷**（容器 TZ=UTC 时不显现，开发机 CST 显现；TEXT 列按字典序排序会混基） | 未并入本 PR；随 T3 bbs 批次做 |
| `shared/src/response.rs:225-227`（`legacy_date_now`） | ② 读/格式化：响应 `date` 用本地时间，库内是 UTC | 非同族（不参与比较），但疑为 T3-4「API 比落库快 8 小时」的根 | 留待 T3-4 定「落库错 or 读出错」后再修 |
| `file/src/lib.rs:406/410/481/485/691/743/814`；`console/src/lib.rs:221` | 写本地时间到 **响应 JSON** | 非缺陷（不落库、不比较） | 不改 |
| `program_center_core_entity/.../application.rs:216` | 写本地时间到 TEXT `deleted_at` | 该列只做非空判定/展示，无跨基比较 | 不改 |
| `program_center/src/lib.rs`（`DATE(create_time) = $2::date`） | 按天过滤 | 两侧同基（`create_time` 为 UTC naive，`$2` 由调用方传入 `NaiveDate`） | 不改 |
| 全仓 `AT TIME ZONE` / `date_trunc('day'` / `CURRENT_DATE` / `LOCALTIMESTAMP` | — | 除上表外无其它命中（`date_trunc('second')` 仅截秒文本，无关） | — |

## 审计第六/七轮：path-IDOR 全量（分批执行，未完成）

来源：`docs/audits/behavioral-audit-2026-11-05/report-6.md`（path-IDOR 全量）、
`report-7.md`（`NO-OWNER-COL` 分档）。逐条判定，**不批量替换**；每批一次提交，
修完跑安全回归门禁并同步减基线。

| 批次 | 范围 | 判据 | 状态 |
|---|---|---|---|
| 内联副本 | `processplatform_core_express::work_terminate:34` | 与 P0-b helper 同根 | **已修**（本批 PR #188）：两处破坏实验均红 |
| 假门 3 条 | cms：`is_admin` 只算返回布尔、不参与拦截 → 200 返回他人数据 | 门必须参与拦截（静态扫描「看起来有门」也要按行为验） | **已修**（本批）：`control` / `permission/read` 不可读改 403；`cipher` 的 `person` 改取会话（非 admin 传他人 → 403）。双向用例（A↔B 403、本人 200、published 只读）；门禁 6→8 PASS、基线 142→140；`sabotage correct` 仍红并点名 `gate-cms-readable` |
| 写 IDOR 24 条 | 按 crate 分批，每批一次提交 | 归属写进 SQL `WHERE`（不是先查后判）+ 双向用例 + 破坏实验红 | **批次 1 已修**：`general_core_entity` 4 条（`file_update/delete`、`invoice_update/delete`）用 `CREATOR_OWNER_PREDICATE`（creator=本人 OR admin）写进 UPDATE/DELETE 的 WHERE；4 条门禁用例 LEAK→PASS（pass 8→12、基线 140→136）；`sabotage general` 红并点名 `general_core_entity|file_delete`。附带修创建冒名：`file_create`/`invoice_create` 的 `creator` 改取会话（忽略请求体）。**批次 2 已修**：`organization_core_entity` 4 条（`definition_update/delete`、`identity_update/delete`）复用同一 `CREATOR_OWNER_PREDICATE` 写进 UPDATE 的 `WHERE`（删除为软删），0 行时二次读只用于区分 403/404；4 条门禁用例 LEAK→PASS（pass 12→16、基线 136→132，leak=121 / unverified=11）；`sabotage organization` 红并点名 4 条。用例为双向 + admin 分支仍可写 + create 的 `creator` 取会话（`x_org_definition`/`x_org_identity` 的 `creator` 列此前未映射进实体，已补）。**批次 3 已修**：`program_center` 4 条——`collect_save/collect_delete`（归属列 `person_id`）、`module_remove_structure_id/module_write_flag`（归属列 `creator`，两条本是只读桩，按只读加谓词）。两条谓词都写进 SQL `WHERE`；非属主由「200 + `deleted:false`」改为 **403/404**（原口径无法与「不存在」区分，也不满足门禁 401/403 判据）。7 个用例：双向、本人 200 且真落库、admin 分支、无会话 401、create 归属取会话人（载荷冒用 `personId` 无效）。门禁 pass 16→20、基线 132→128（leak=121 / unverified 11→7）；`sabotage program_center` 4 条 NEW-FAIL。**批次 4 已完成（代码+用例）**：`hotpic` 2 条（`delete_by_id`、`user_hotpic_delete_by_ids`）用 `HOTPIC_OWNER_PREDICATE` —— **全局行**（`application`/`info_id` 两列皆空，见 `visible_scope`）**仅管理员可写**，专属行 `creator=本人 OR 管理员`，与 `create_hotpic`/`save_hotpic` 的 admin-only 口径一致（照搬 `creator=本人` 会锁死全局热点图维护入口）；`mind` 2 条（`update_mind`、`update_folder`）用 `CREATOR_OWNER_PREDICATE`（两表无 `deleted_at`）。谓词均写进 SQL `WHERE`，0 行时二次读只用于区分 403/404。门禁读数 pass 20→24（leak=117 / unverified=7）；**基线已在 192 条门禁落地后按实跑读数减**：rebase 到 `5fbc15326` 后跑门禁为「基线 160 / 非 PASS 156」，`STALE-BASELINE` 正好点名这 4 条，逐条删除后门禁通过（非 PASS 156 = leak 133 / unverified 23 / integrity 0）。`sabotage hotpic_mind` 在新门禁下复验成立（基线 156 → 非 PASS 160，4 条 NEW-FAIL，3 个文件已还原）。**剩余 7 条**（此前口头口径的 16 是粗估，按用例表逐条点算实为 11 起步、本批后剩 7：file folder 1、attendance appeal 1、cms 2+1、pp 2） |
| 读 IDOR 117 条 | 拆 E18–E24 七批 | **先做「A 读 B / B 读 A」双向都 200 的**（谓词整体失效），再做单向 | 待做 |
| NO-OWNER-COL A 档 12 条 | 有归属列且生产路径确实写入 | 同写 IDOR 判据 | 待做 |
| NO-OWNER-COL B 档 5 条 | 列存在但生产路径从不写入、恒 NULL（典型 `x_cms_fileinfo.upload_person`，五处 INSERT 均不带该列） | **先补写归属值再写谓词，或改为跟随父文档 ACL**（二选一，理由入台账）。判据必须造出「有归属值」行验证谓词真会过滤，否则谓词恒不命中也会假绿 | 待做 |
| NO-OWNER-COL C 档 14 条 | 真·全局配置 / 无主体列 | **保守默认**：非管理员只读、写仅管理员（安全侧默认，业务可推翻） | 待做 |
| 软门 2 条 | empower `manager_create` / `manager_update`：返回 `ActionResult::error` 但 HTTP 200 | 改 403 + 「客户端能区分成功与失败」用例 | 待做 |
| 无门读 11 条 | 权限串 / ACL 元数据端点未取权限 | 最保守：管理员 + 本人 | 待做 |
| 明确不做 | `SKIP-NO-TABLE` 9 条、`EMPTY` 13 条（file 族疑有 `xid`/`file_status` 二次过滤）、`FAIL-500` 26 条（bbs 6 条第五轮已实证） | 登记，不动 | 已登记 |
| 不可判 4 条 | 报告 §四索引 | 不碰 | — |

方法论：审计实证「A 看不到 B」单向验证不够 —— 实测 A→B 与 B→A **双向都 200**，
匿名 401、管理员 200，是谓词整体失效。每条修复用例必须双向；只验拒绝会放过
「反向被 admin 分支挡住」的假修复。

## 门禁纪律（`security-regression` 已设为 required check）

- `protect-main` 的 `bypass_actors` 保留 admin `always`，hotfix 通道不关死。但任何用
  `--admin` 强合的 PR，必须在**同一批提交**里写明「红灯原因 + 为什么不能等 + 计划何时
  转绿」，并在 24 小时内提交修复把该检查转绿。**不允许「先强合、后面再说」**——这正是
  历史上「红门禁 + `--admin` 绕过变成常态」的根因。
- required check 对**旧分支**等于永远不满足：长命 PR 必须定期把 `main` 合回本分支，
  否则会被一个其 head 上根本不存在的检查永久卡死。#182 即此例（其 head 的 workflow
  尚无该 job），处理方式为把 `main` 合入其分支后重跑 CI。
- required check + `pull_request.paths` 过滤 = 某些 PR 永远等不到结果。已删除
  `oa4rust-ci.yml` 中 `pull_request` 的 `paths` 过滤（`push` 保留），确保 10 项必需检查
  对每个 PR 都存在；仅文档 PR 由临时 PR 实测 10 项全部出现。
- 改 `.github/workflows/**` 的 PR 目前**只能用本地 merge+push 合入**：`gh` OAuth app 缺
  `workflow` scope，`mergePullRequest` 会直接拒绝（`--admin` 同样被拒）。修法：
  `gh auth refresh -s workflow` 补齐该 scope。
  - 本地 push 必须是 **fast-forward**（`non_fast_forward` 规则会挡非 ff）。
  - required checks 规则不评估直推的新提交，所以这条路径**不靠检查放行**，等价于 admin
    绕过；使用前提是**被推送树对应分支上的必需检查已全绿**，并按下面的纪律留痕。
- **直推 `main` 只允许一种情形**：PR 触及 `.github/workflows/**`，而 `gh` 因 OAuth app 缺
  `workflow` scope 无法用 API 合并（`gh pr merge --admin` 同样被拒）时。**其它任何情况一律
  走 PR**。直推的提交信息必须带 `[direct-push: workflow-scope]` 前缀——否则事后无法从
  history 分辨「合规例外」还是「顺手绕」。
- **直推前必须「先绿后推」**：先在被推分支上跑完整 CI 并全绿，再推；**不是推完等绿**，
  后者等于在 `main` 上做实验。每次直推把该次 run id 记一行（见下方「直推记录」）。
  直推只能是 fast-forward；`non_fast_forward` 挡的是强推/非 ff 推送，不挡合法 ff 推进，
  所以本地 merge + push 是当前唯一可行路径（API 路径被 workflow scope 拒绝）。
- **待用户手动执行**：`gh auth refresh -s workflow` 需要交互式登录，agent 无法代做。
  执行后可直接用 API 合并 workflow PR，上述直推例外即应关闭。
- **直推记录**：`2026-11-05T20:12:52Z` #192（head `c74d8720`）—— 推前 pull_request 全套
  检查全绿：`CI` run `42099512280`、`OA4Rust web quality` `42099512175`、
  `three-ends contract` `42099512320`、`supply-chain` `42099512237`、
  `laiyipao CI` `42099512224`、`warnings-gate` `42099512217`、
  `auto-checkin` `42099512133`。符合「先绿后推」。
- **扫描器误报：补扫描器，不补基线**。静态扫描（`fi002-column-scan` 等）把合法代码判红时，
  先判「真缺陷」还是「扫描器盲区」；是盲区就改识别逻辑，**不往基线加条目**——基线一旦开始
  塞误报，很快会退化成「我们知道但不管」的清单。实例：043 用 `DO $$ ... RENAME COLUMN`
  条件改名（`type_`→`type`）骗过扫描器（原实现只认 `RENAME TO` 表改名），已补
  `RENAME COLUMN old TO new` 解析（含 `DO` 块内语句），基线未动。
- **`RENAME` 回扫**（2026-11-06）：全仓 grep 只命中 043 的列改名（其 `_rollback.sql` 被扫描器
  跳过）与若干 `RENAME TO`（表改名，原已处理），未发现第二处盲区；无裸写
  `RENAME old TO new`（省略 `COLUMN` 关键字）的列改名。
- **减基线前先确认工作树与 `origin/main` 同步**（`git rev-parse HEAD` 对 `origin/main`）。
  在旧代码上跑 `baseline` 会把「尚未修好的」写进基线，等于把待修项永久合法化。
  `STALE-BASELINE` 的含义是「已修复但基线未减」，不是可以压平的数字差异。
- **门禁版本升级期间，基线以新门禁读数为准**：审计线正在把门禁从 148 条升到 192 条
  （新增 drift / admin_policy / 深分页 / 写正控制），同步后非 PASS 读数会从 128 变 160。
  先在 main 上确认新门禁已落地，再 rebase 重新取数；**不得按旧数字手改**。代码/用例与
  基线更新可以分两次提交 —— 宁可分两次，也不要写错基线。

## 已知相邻（不并入本轮）

- E9 附件若需「会议参与人」判定，要先确认 `x_meeting` 参与人表结构，单独评估；
- `query table_flag=$1`、file 列表归属过滤、org `next`（实为 offset）为**负证据**，不改。

## 写侧归属列常量身份扫描（2026-11-06）

背景：写侧把 `creator`/`owner` 一类归属列赋值为字符串字面量（本次命中值均为 `'system'`），
读侧再加谓词也只是在错的归属上打补丁。全仓只读扫描后按「是否已产生功能后果」排序修复，
每批独立分支 + PR。

### 已确认缺陷（已修）

| # | 位置 | 表 | 写入字面量 | 读侧后果 | 判定 / 处置 |
|---|---|---|---|---|---|
| W1 | `query_assemble_designer/src/u2_closures.rs::table_create` | `x_query_table` | `'system'` | 该 handler **未挂路由**；线上 `POST .../designer/table` 走 `create_table_definition`（admin 门） | **确认缺陷（写入口无身份）**；补 `require_session`（DB 前 401）+ `creator` 取会话；线上 admin 口径不放宽。批1 PR #218 |
| W2 | `general_assemble_control/src/lib.rs::qrcode_width_width_height_height_text_text` | `x_general_assemble_qrcode` | `'system'` | `qrcode_get` / `qrcode_list` 目前无会话、无谓词 | **确认缺陷（潜在）**；写侧改会话，读侧仍公开，收口属独立批次。批2 PR #220 |
| W3 | `query_assemble_designer/src/u2_closures.rs::statement_create` | `x_query_statement` | `'system'` | 删除侧按 `creator` + `require_owner`，端点 admin 门内无差异 | **确认缺陷（潜在）**；写侧改会话。批2 PR #220 |
| W4 | `query_service/src/lib.rs::processing_execute` | `x_query_processing` | `'system'` | 无读端点（只写） | **确认缺陷（潜在）**；写侧改会话；顺带修 `params` 的 JSONB 错误绑定（原先把 `String` 绑进 JSONB，任何调用 500）。批2 PR #220 |
| W5 | `ai_assemble_control/src/lib.rs::file_upload` | `x_ai_file` | `'system'` | 删除侧 `require_owner` 判 403 → 非管理员删不掉自己文件 | **确认缺陷（已出功能后果）**；先红后绿用例。批3 PR #221 |
| W6 | `ai_assemble_control/src/lib.rs::file_copy_file` | `x_ai_file` | `'system'` | 同上，复制出的新行归属错误 | **确认缺陷（已出功能后果）**；批3 PR #221 |
| W7 | `file_assemble_control/src/lib.rs::create_file_entity` | `x_file` | `'system'` | 删除侧按 `creator` 判 403 → 非管理员删不掉自己文件 | **确认缺陷（已出功能后果）**；先红后绿用例。批3 PR #221 |

### 历史行可见性变化（不回填，走既有兜底）

统一口径：历史行 `creator='system'` 保留原值，非管理员视为「无归属」，仅管理员可读/可删/可写。

| 表 | 历史行对非管理员 | 新行对非管理员 | 说明 |
|---|---|---|---|
| `x_query_table` | 无影响 | 无影响 | handler 未挂路由，生产无该路径写入的行 |
| `x_general_assemble_qrcode` | 无变化（读端点无谓词，仍公开可读） | 读侧加谓词后才可读；本批未改读侧 | 写侧归属已可区分 |
| `x_query_statement` | 无变化（端点 admin 门） | 同左 | 只影响删除侧 owner 臂能否命中真实行 |
| `x_query_processing` | 无读端点，无可见性 | 同左 | — |
| `x_ai_file` | 仍仅管理员可删（与修复前一致） | 本人可删 | 读端点仍无会话/谓词，本批未改 |
| `x_file` | 仍仅管理员可删（与修复前一致） | 本人可删 | 读侧按各端点现状，本批未改 |

### 合理默认，不改

以下字面量属于「历史兼容的合理默认」，或写端点本身已 admin-only，不纳入本轮：

- ai config / admin 类、correlation（读用 `person_id`）、jpush（admin + `target='all'`）；
- `portal_*`（写 admin）、program_center（admin / 全局表）、query designer 其余创建（admin）、
  query_surface（admin）、general attend / securityclearance（admin）。

### 通用检查项：归属谓词的判据行必须来自真实写入路径

**适用范围：所有新增/修改归属谓词的批次，不只 `x_portal_*`。** 每次给读端点加
`creator=本人 OR admin`（或 `owner`/`person` 同类谓词）时，必须同时说明并验证：

1. **判据行来源**：用例里的归属行必须由真实写入路径（handler/端点）产生；
   只靠 seed 直接插 `creator=某人` 会让「谓词恒不命中」也显示绿。
2. **真实行上的三条判据**：本人可命中；他人访问同一行被 403 或返回空集且行仍存在；管理员放行。
3. **写侧前置条件**：先确认该表的写入已落会话身份。写侧仍硬写常量时，owner 臂对真实数据
   **永不命中**，门禁绿只说明「谁都没命中」，不代表 IDOR 已修。
4. **admin-only 表的例外**：若确认该表恒 admin-only，必须明文登记「owner 臂永不命中、
   保护来自 admin 门」，不得在台账记为「已加归属谓词防住」。
5. **发现假绿的处理**：列为 `FALSE-GREEN`（写侧常量 + 读侧 owner 谓词），要么改写入，
   要么拆除无效谓词并固定 admin-only 口径；在处置前该端点不得计入「已防住」数量。

### ⚠️ `x_portal_*`：通用检查项的一个实例（假绿风险）

`portal_assemble_designer` 的生产写入仍硬写 `creator='system'`：

- `create_design` → `x_portal_design`
- `create_page` → `x_portal_page`

若给 `x_portal_*` 读端点加 `creator=本人 OR admin` 谓词，**owner 臂对真实数据永不命中**
（真实行 creator 恒 `'system'`），非管理员只能拿到空集。门禁会显示 PASS，但保护来自
「谁都不命中」，不是归属判定生效；用例若只用 seed 造出 `creator=某人` 的行，还会掩盖这一点。

处置：要么先把上述写入改为会话身份，要么明确这些行恒 admin-only 并固定口径。
在此之前，`x_portal_*` 的相关门禁绿**不得记为「已防住」**，按上面通用检查项的第 3–5 条处理。
