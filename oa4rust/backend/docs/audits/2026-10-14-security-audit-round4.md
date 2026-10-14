# 2026-10-14 全量安全审计 · 四轮复核（/goal：全面找出 oa4rust 所有漏洞并修复验证）

本轮在独立 worktree（分支 `oa4rust-vuln-audit-r4`，基于 `optimism20261012` @ 6e42e35ad）上进行，
避免污染主工作区。目标：在前 3 轮（2026-10-12 / 10-13 / 10-14 quinn 供应链）之上，
对**当前代码树**做一轮全新的全栈复扫，找出所有遗留/盲点漏洞并修复验证。

## 〇、前提事实

- **代码树冻结**：`git log 0612ce604..HEAD -- oa4rust/backend/{crates,src} oa4rust/frontend mobile`
  为空——自二轮审计（2026-10-13）之后 oa4rust 代码零变更，仅 `Cargo.lock`（quinn 0.11.12 /
  quinn-proto 0.11.19）与 `security/rustsec-db.rev`（pin 抬至 6aed8466）及文档有提交。
  故本轮定位为「全库复扫 + 回归确认 + 前轮盲点排查」，任何新发现都意味着前轮漏网。
- 96 个 backend crate + 主二进制 + desktop/mobile/SDK 前端 + MCP 独立服务。

## 一、依赖面复扫（本轮执行）

### Rust（RustSec advisory-db @ 最新）

- 最新 RustSec 库 HEAD = `6aed84668c23d7f535077a9fe918bf1fa0f6a5bc`，**与仓库 pin
  （`security/rustsec-db.rev` = 6aed8466）逐字一致**——即三轮抬 pin 时已是当日最新，
  `6aed8466..HEAD` 无任何新通告提交。
- 对当前 `Cargo.lock`（593 个 registry 包）× 1290 条 crate 通告全量逐条版本匹配
  （`.r4-tools/rustsec-match.mjs` 手工匹配器，caret/tilde/复合区间语义完整实现）：
  - **命中 12 条，全部为既有记录**：
    - `lopdf@0.26.0` RUSTSEC-2026-0187（栈溢出 DoS）——**既有例外有效**（至 2026-10-28，
      genpdf 生成链，无 load* 入口；解析/签名走 crates/signature 的 lopdf 0.42.0，lock 双版本
      共存已核实）。
    - `rsa@0.9.10` RUSTSEC-2023-0071（Marvin）——**既有例外有效**（至 2026-11-08，
      仅 dev-dependency MySQL 测试驱动链）。
    - 10 条 unmaintained 信息级（adler / encoding / lzw / proc-macro-error / ring /
      rusttype / stb_truetype / stdweb / ttf-parser / unsafe-libyaml）——与三轮「信息级
      记录在案」清单逐条一致。
  - **无新增可处置命中**。三轮的 quinn-proto 3 条 DoS 通告保持清零（quinn 0.11.12 /
    quinn-proto 0.11.19 已在 lock，`cargo tree -i quinn` 无构建反向依赖的结论不变）。
- 编译器/标准库通告（`rust/std|cargo|rustdoc`，19 条）：patched 版本全部 ≤ Rust 1.58.1，
  CI 钉版 Rust 1.98.1 与本机 1.99.0 均远在新修复版之上——**不适用**。

### 前端 / 移动端（pnpm audit，oa4rust 根工作区）

- 命中 3 条（1 high + 2 moderate），与一/二/三轮「保留 3 条」清单**逐条一致**：
  - `braces <=3.0.3`（无修复版；chokidar dev 工具链）
  - `file-type >=13 <21.3.1`（上游已出 21.3.1 修复版，但 jimp ^16 硬 pin 阻断采用；
    uni-app 构建期图片探测，非运行时）
  - `postcss-selector-parser <7.1.6`（uni 工具链硬 pin 6.x，已 override 至 6.x 末版 6.1.3）
- **无新增** CVE。`pnpm-workspace.yaml` 全部安全 override（vue/@vue 3.5.42 家族、
  source-map-js 1.2.2、esbuild 0.25.12、vite 6.4.3、ws 8.21.0 等）生效。

### 结论

依赖面（Rust + JS + rustc 工具链）**零新增可处置漏洞**；12 Rust 命中 + 3 JS 命中
全部在既有例外/保留/信息级记录之内。

## 二、代码面全库复扫（4 个并行只读审计子代理）

审计面划分（各代理先读 1–3 轮记录避免重报，只报有代码证据的新发现）：

1. **越权/写门禁**：96 crate 全部 INSERT/UPDATE/DELETE handler 的 admin/属主/会话门禁
   逐格对账（含 AUTH_EXEMPT × 写、rbac Public × 写、creator 取请求体冒名、物理删、
   全局配置写、加组成员提权路径）。
2. **注入/DoS/密码学**：SQL 拼接落点、命令执行、路径穿越、SSRF/出站、弱加密/不安全
   随机、无界资源（缓存/递归/大 alloc/ReDoS）、unsafe 清单、日志泄密。
3. **前端/移动端/SDK**：XSS（v-html/URL 直渲染/postMessage）、外泄通道、协议导航、
   SW/iframe 消息源、移动端特有面、依赖 override 回归。
4. **运行时/配置/部署**：bind 地址/TLS、env 缺失 fail-open、cookie/会话参数回归、
   限流 key 可伪造、migrations 数据面、WebSocket/LDAP/SMS/AI 出站、仓库内泄密、
   CORS/错误回显。

（结果与发现明细见下 §二.1–§二.4、§三 修复、§四 验证。）

### 二.0 一把确定性尺子（新增工具，可复跑）

`scripts/audit_write_gating.mjs`（node；本机无 Python，故不沿用 `scripts/*.py` 口径）把三件事实做机器对账：

1. 路由注册面：扫 `crates/**/*.rs` 的 `.route("PATH", method(handler))` → 3844 条路由 / 其中写路由（POST/PUT/DELETE/PATCH）**1689 条**；
2. 中间件级门禁：`shared/src/middleware/constants.rs` 的 `AUTH_EXEMPT_PATHS`(302) / `AUTH_EXEMPT_PREFIXES`(18) / `ADMIN_WRITE_PREFIXES`(6)，匹配语义复刻 `security.rs` 的 `path_matches` 与 `path_starts_with_segment`；
3. handler 体内信号：会话形参（含 `Extension<shared::session::Session>` 全路径写法）与门禁惯用名（`require_admin` / `require_owner` / `owner_gate` / `u2_require_admin_opt` / `u2_check_owner` / `guard_write` / `require_session*` / `is_admin` …）。

本轮读数（写路由 1689）：

| 桶 | 条数 | 含义 |
| --- | --- | --- |
| R1 中间件 admin | 66 | `/api/person|unit|role|group|organization/person|organization/group` 前缀下的写路由，由 rbac 中间件兜 admin |
| R2 匿名可达 + 体内门禁 | 2 | 豁免路径但 handler 自带门禁 |
| R3 匿名可达 + 读会话 | 1 | 豁免路径且读会话 |
| **R4 匿名可达 + 体内无门禁** | **13** | 匿名可写、handler 层无信号——逐条人工定性见 §二.5 |
| R5 已认证 + 体内门禁 | 245 | 有门禁信号 |
| R6 已认证 + 读会话无门禁 | 231 | 属主/租户范围需人工取证 |
| **R7 已认证 + 无会话无门禁信号** | **1131** | 最大残余面；**其中绝大多数是「写 SQL 在 helper 里、门禁在路由包装层」的尺子盲区**，不能直接读成 1131 个漏洞 |

尺子的已知盲区（写在这里防止误读）：只看 handler 首块，不跟进被调用函数；不认识宏注册/`#[debug_handler]` 别名；`ADMIN_WRITE_DEPRECATED_PREFIXES`（constants.rs:419，已撤销中间件兜底的 `/api/processplatform` 等）在本尺读作「无 admin」——这是**正确**的，因为那层兜底确实已不存在，handler 层是唯一门禁。

明细落盘：`write_routes_ungated.tsv`（R4 全部 + R7 全部）、`write_gating_residual.tsv`（函数级矩阵 × 路由）。
函数级矩阵 `write_handlers.tsv`（991 行）本身有**两处口径缺陷**（A1 取证发现，已在尺子中修正）：`session` 列漏识别全路径写法的 `Extension<shared::session::Session>`；`gate` 列漏识别 `u2_check_owner` 与 `guard_write` 两类 helper——故矩阵的 `-/-`/`S/-` 只能当候选，不能当结论。

### 二.1 越权/写门禁（A1：processplatform 族，200 行候选，已收口取证）

前置事实：rbac 把 `/api/processplatform` 注册为 `Authenticated`（`crates/shared/src/middleware/rbac.rs:224`），且它在 `ADMIN_WRITE_DEPRECATED_PREFIXES` 内——**中间件不再兜 admin，handler 层是唯一门禁**。而下列 handler 层为 0：

| id | 级别 | 落点 | 攻击面（任意登录用户，除非注明） |
| --- | --- | --- | --- |
| R4-1 | 高 | `processplatform_service_processing/src/lib.rs`：`task_complete:3265` / `task_reject:3363` / `task_transfer:3436` / `task_claim:3188` / `work_start:3085` / `work_complete:3135` / `work_id_processing:385` / `work_v2_id_terminate:408` / `work_v2_id_retract:439` / `execute_process:220` / `cancel_process_instance:354` / `gateway_join:3485` / `gateway_fork:3532` | 不取 Session、不校身份：完成/退回**他人**待办并把 `x_record` 的 creator 写成任务负责人（= 冒名伪造审批、推进他人流程）；转办任意任务给任意人；启停/终止/撤销任意 work。这批路由挂在 `/api/work`、`/api/task`、`/api/gateway`，连 `/api/processplatform` 前缀规则都不覆盖 |
| R4-2 | 高 | 同 crate `src/u2.rs`（47 处写 SQL，除 8 个一轮已修的 `touch/*` 外全部无 Session）：`work_delete:1917`（级联物理删 `x_work`+`x_task`+`x_draft`）/ `work_edit:1888` / `data_work_create|update|delete|delete_path:489-536` / `data_wc_update*:538,552` / `job_delete:782` / `task_delete:1397` / `taskcompleted_delete:1416` / `read_delete:899` / `review_delete:1258` / `record_delete:1137` / `record_edit:1090` / `snap_delete:1277` / `wc_rollback_flag:2209` / `wc_merge_flag:2122` / `wc_shift_time:2179` / `work_v3_retract_body:1985` / `dict_edit:145`、`dict_set_N`/`dict_del_N`(宏 262/270) / `att_delete_id:569`、`att_edit_id:649`、`att_edit_text:681`、`att_copy_to_work:707` / `lib.rs:2223 event_add_update_table` | 按 ID 整体覆写/物理删除**任意他人**流程实例的表单数据 JSON、附件、记录；`event_add_update_table` 可向任意 work 插伪造 `x_record` |
| R4-3 | 高 | `processplatform_assemble_designer`：`save_flow:289`（POST `/designer/save/{id}`，无 Session，整体重写 `x_process_definition.process_definition` 流程 JSON=审批人/环节全可改）/ `delete_flow:332`（物理删流程定义）/ `u2_script.rs:67 update`（有 Session 但仅用作 creator 记录，UPDATE 只按 `xid=$1`→任意改**任意脚本 xcode**，脚本是引擎执行单元，等价 RCE 面）/ `u2_script.rs:120 delete`（软删任意脚本）/ 经 `shared::crud`（`crates/shared/src/crud.rs:76,115`，无属主概念）的 `form_save:5010`/`form_delete:5026`/`xform_save:5136`/`xform_delete:5152`/`dict_save:4894`/`dict_delete:4910` / `item_access_delete_process_processId_path_path:1565` / `item_access_bach_save:4032` | 设计态篡改→提权链：改流程定义/改脚本代码/改表单与字段权限配置 |
| R4-4 | 中-高 | `processplatform_assemble_surface`：约 20 个 `*_mockdeletetoget`（routes.rs:419/430/470/505/529/542/558/566/569/574/622/657…）+ `correlation_job_job_delete:2778` + `mode_id_delete:8694` | 无 Session、按 xid **物理 DELETE** `PP_C_WORK`/`PP_C_TASK`/`PP_C_WORKCOMPLETED`/`PP_C_DRAFT`/`PP_C_JOB`/`PP_C_DOC_SIGN`/`PP_C_READ` 等；其中 8 个 `data_work_id_path*_mockdeletetoget`(4252 起) 本应只删数据键却整行删 `PP_C_WORK`，8 个 `applicationdict_*_mockdeletetoget`(1237 起) 物理删设计字典 |
| R4-5 | 中 | 同 crate `create_surface:81`（creator 硬编码 `"system"`，未会话化）/ `publish_surface:295` / `delete_surface:366` / `save_surface:430` | 任意改删 `x_process_surface` 应用门户配置 |
| R4-6 | 中 | `processplatform_core_express/src/lib.rs:16 work_terminate` | 注册为 **GET** `/api/processplatform/work/terminate/{id}`：无 Session 终止任意 work；GET 状态变更同时绕过 CSRF 中间件（只覆盖 POST/PUT/PATCH/DELETE），SameSite=Lax 下仍有顶层导航触发面 |

A1 同时核实**无需重报**的部分：bam `create_bam`（有 admin 门）、surface `touch_expire/passexpired/touchdetained`、`attachment_u2b_batch_delete_manage`、`serialnumber_u2_create`、`handover_u2_create`、`snap_u2_upload`（一轮 V2/V4 已修属实）、`snap_u2_delete/restore`、`attachment_u2_delete_by_work`、`attachment_id`、`attachment_u2b_delete_by_workcompleted`、`work_u2_v3_retract`（均有 `u2_check_owner`/已办持有校验）、designer 的 11 个 `guard_write` handler（`application_id_update/icon/permission`、`applicationdict_id_delete`、`mapping_id_update/delete`、`mergeitemplan_id_update/delete`、`process_id_update/permission_save/edition_delete`）、processing 8 个 `touch/*`；`u2_att_store_new`/`general_file_store`/`doc_to_word_for_job` 为内部 helper，公开包装器均传 `session.person_unique`。
→ 结论：designer 是「部分修复后遗留旁路」（11 个已走 `guard_write`，脚本/flow/form/xform/item-access 四族被漏），非全 crate 空白。

### 二.2 注入 / DoS / 文件访问 / 出站 / 密码学 / 泄漏（A5，已完成）

覆盖面（含所跑 pattern）：无界解压解析、路径穿越、命令执行、SQL 拼接、SSRF、ReDoS、弱随机/弱哈希、`unsafe`、日志泄密。

| id | 级别 | 落点 | 判定 |
| --- | --- | --- | --- |
| N4-01 | 中（DoS） | `file_assemble_control/src/lib.rs:read_zip_entry`（XLSX `xl/sharedStrings.xml`、PPTX 每张 `slideN.xml` 都经它） | **本轮已修**：`XLSX_MAX_ROWS`/`PPTX_MAX_SLIDES` 只在解压**之后**限输出行/页数，不限驻留内存；zip 炸弹附件预览可把单 worker 撑到 OOM。补 `ZIP_ENTRY_MAX_BYTES = 4 MiB` 上限 + 9 MiB 条目截断回归 |
| N4-02 | 中（DoS） | `processplatform_assemble_surface/src/u2_render.rs:275 image_to_png`、`:384 image_to_pdf_bytes`，经 `lib.rs:25938 attachment_u2b_preview_image_page` / `:25902 attachment_u2b_preview_pdf` 触达 | **待修**：`image::load_from_memory` 无 `Limits`，解码器按图片头声明的 `w×h×channels` 分配——上传体积极小但声称超大尺寸的 PNG 即解码炸弹；上传体积上限对此无效（分配由头部驱动）。注意本 crate 钉的是 `image 0.23.12`（workspace 另有 0.25），`Limits`/`ImageReader::with_limits` API 在 0.23 不可用，修法需先定「抬 image 版本」还是「按格式手工读头部尺寸再拒绝」 |
| N4-03 | 低/信息 | `program_center/src/lib.rs:1834 code_create_mobile_mobile` | **待定性**：`format!("{:06}", Utc::now().timestamp_subsec_millis() % 1000000)`——`subsec_millis()` 只返回 0–999，取模是空操作，「验证码」实为 3 位时间可推值；且消费方 `code_validate_*`(`:1963/:1985`) 查询列是 `WHERE flag=$1` 而写入布局不同→校验恒不命中（fail-closed），故**今天不是活的鉴权旁路**，但是弱随机 + 真逻辑缺陷。待拍：该流是否被任何在用登录/重置流程消费（若在用则升为高，并须一并修校验列） |

A5 判为干净（附取证口径）：命令执行仅 `console`/`preview`，白名单 + arg 向量、无 shell；SQL 全为静态列 + `$N` 绑定，动态 `op/order` 是 bool 字面量选择，`escape_like` 在用；用户可控 URL 一律走 `netguard::fetch_limited`，其余出站为 env/admin 配置；全仓 `unsafe` 0 处命中；日志只记计数不记口令/token；`signature/src/lib.rs:546 lopdf::Document::load_mem` 走 lopdf **0.42**（已修版本，与 RUSTSEC-2026-0187 的 0.26.0 例外链并存，符合既有记录），未新增条数上限。

**A5 对前三处待提交修复的复核结论**：`is_safe_avatar_rel` 正确且完备（拒空/NUL/`\`/前导 `/`，要求 `data/avatar/` 精确前缀 + 单段文件名，写侧 `data/avatar/{uuid}.{ext}` 恒放行）；`preview::safe_base_name` 正确（双分隔符 rsplit、拒 `.`/`..`/NUL/空，且 `download()` 另有 `starts_with(temp_dir)` + `ParentDir` 栅栏；注意 preview crate 未挂主服务，属纵深防御）；`docx_to_html` 上限对 docx 正确但**同族漏了 XLSX/PPTX**（即 N4-01），且 `DOCX_MAX_PARAGRAPHS=200` 会把长文静默截断到前 200 段——预览可接受，但未向调用方标注。

### 二.3 / 二.4 CMS+查询族、中量 crate 分诊

（A2 / A3 代理仍在跑，收齐后填写；R7=1131 的人工定性依赖这两份报告。）

### 二.6 越权/写门禁 · 长尾 crate（A4，已完成：53 行 / 15 crate）

范围：`ai attendance bbs component_assemble_control console correlation_service_processing express hotpic_assemble_control meeting meeting_core_entity message portal portal_core_entity process_designer search`；全部经 `src/lib.rs:97-151` 确认已 `merge(...router(pool))` 挂载。分诊合计：53 = 28 行「调用链已有门禁」（矩阵漏判或 S4/S5 已修：console 8 处 `is_admin`、attendance 3 处 `is_admin||is_attendance_admin`、ai `chat_delete/file_delete` 的 `require_owner`、message `custom_create` 的 `require_admin` 等）+ 24 行进入下列发现 + 1 行死代码。**F1–F10 均非匿名可达**（`/api/express/subscribe` 亦未被豁免），档位是「任一登录用户」。

| id | 级别 | 落点（file:line） | 攻击路径 + 影响 | 建议修法 |
| --- | --- | --- | --- | --- |
| F1 | 高 | `correlation_service_processing/src/lib.rs:228,259,291,316`（`create/save/delete_correlation`、`unlink_service`）+ `src/u2.rs:236,246,317,327,393,403`，内部 `u2.rs:121,256,337`；路由 `lib.rs:355-385` | 全部无 `Extension<Session>`、无门禁：任一登录用户物理删任意关联行；`…/correlation/update/type/cms/document/{document}` 的整段先 `DELETE … WHERE from_type=$1 AND from_bundle=$2`，bundle 纯路径参数与调用者无关 → 整站清空他人关联；create 的 `person_id` 取请求体（`let creator = "system"`）→ 冒名归属 | ownerless 系统关联表 → `require_admin`（沿用 S5 `process_designer` 裁决），`person` 改 `session.person_unique` |
| F2 | 高 | `portal_core_entity/src/lib.rs:220 page_update`、`:269 page_remove`、**索引漏收的 `:170 page_create`**（SeaORM `active.insert()` `:208`）；路由 `:344-346` | `x_portal_page` 无 creator 列（`migrations/029:284`）：任一登录用户改/软删任意门户页面 → 门户内容篡改（S5 修了 `portal_assemble_surface`，core 侧漏网） | crate-local `require_admin`（同 `portal_assemble_surface` 写法） |
| F3 | 高 | `portal/src/lib.rs:379 save_page`、`:425 delete_page`；路由 `src/routes.rs:21-22` | `delete_page` 是**物理** `DELETE FROM x_portal_page WHERE id=$1`，无属主无 admin → 硬删任意门户页不可恢复 | 同表 ownerless → `require_admin` |
| F4 | 中 | `hotpic_assemble_control/src/lib.rs:551 user_hotpic_changeTitle`、`:711 user_hotpic_id` DELETE 分支、`:770 user_hotpic_delete_by_ids`；路由 `routes.rs:53-54(GET!)、104-105、145-150` | `x_hotpic` **有** creator 列（`migrations/033:37`）但 user 家族三处按 id 盲写；S5 只补了 admin 家族(`:111,286,320,355`)，user 家族漏网 → 任一登录用户重命名/软删他人轮播 | 复用 crate 既有 `owner_gate`/`require_owner`（先查该行 creator，行不存在放行保 not-found） |
| F5 | 中 | `search/src/lib.rs:308,323,339 ftsearch_create/save/delete`；路由 `:357-361` | `x_ftsearch_document` 全站共享全文索引，`shared::crud_create` 从不落 creator 值（`shared/src/crud.rs:44`）→ 无属主可依据；任一登录用户投毒检索结果/软删他人条目 | ownerless → `require_admin`（S5 先例） |
| F6 | 中 | `ai/src/index.rs:68 index_delete`；路由 `ai/src/routes.rs:47` **注册为 GET** | 无 session 的 `DELETE FROM x_ai_index WHERE id=$1` → 销毁 AI 知识索引条目；GET 形态另开 CSRF 面 | 加 `Extension<Session>` + `require_admin`，路由改 POST/DELETE |
| F7 | 中-低 | `message/src/lib.rs:99 update_single`（GET `routes.rs:19`）、`:167 mark_read`（POST `:21`） | `x_msg_message` 有 `xconsumer` 收件人列（`migrations/034:336`），两处只判 `xid` 存在即 `SET xconsumed = true` → 任一用户枚举 id 替他人「已读」（通知丢失/完整性） | 注入 session，先读该行 `xconsumer` 再 `require_owner`；GET 变体改方法 |
| F8 | 中 | `meeting/src/lib.rs:442 add_participant`（POST `/api/meeting/{meetingId}/participant/add` `routes.rs:27-29`）、`:244 create_meeting` | 按路径 `meetingId` 向任意会议插邀请，不校调用者是否组织者 → 会议邀请注入/骚扰；create 的 `creator` 取请求体并默认 `"system"` → 冒名 | add：查 `x_meeting.creator` + `require_owner`；create：creator 会话化 |
| F9 | 低-中 | `bbs/src/subject.rs:262 create`（POST `/api/bbs/subject/create` `routes.rs:25`） | `author_id` 完全取请求体、无会话 → 以任意 person 身份发帖（身份伪造，配合 S7 类前端渲染可放大） | `session.person_unique` 覆写 authorId |
| F10 | 低 | `express/src/lib.rs:110 subscribe_express`（POST `/api/express/subscribe`） | 无 session、无容量/唯一约束地 INSERT `x_express_subscribe` → 表膨胀 DoS。`callback` 目前全仓无读者；若日后接通出站须走 `shared::netguard` | 加会话 + 去重/限流 |

跨切观察（应做成守卫，而非逐格修）：CSRF 中间件只覆盖 POST/PUT/PATCH/DELETE（`shared/src/middleware/token.rs:70-74`），而 F4/F6/F7 的写 handler 另有 **GET 注册**（`hotpic routes.rs:53`、`ai routes.rs:47`、`message routes.rs:19`）→ 这些状态变更 GET 完全不受 Origin 校验。建议加「写端点不得挂 GET」的普查守卫。
死代码观察（非漏洞）：`portal/src/lib.rs:323 create_page` 全仓无路由注册（`tests_generated.rs:97` 亦记 not accessible），一旦挂载即成「creator 取请求体」缺陷。
A4 开放问题：① F1 是否 o2server 侧本就以 job/document 创建者为隐含属主（需产品确认，否则按 ownerless 收 `require_admin`）；② F2/F3 与 S5 的 `portal_assemble_surface` 是否合并为单一「门户内容写 = admin」守卫点（两 crate 写同表 `x_portal_page`，列集不同）；③ `/api/ftsearch/*`、`/api/config/*`、`/api/server/deploy/*` 未在 `PermissionRegistry` 注册、只靠默认 Authenticated——是否显式注册前缀以防未来新增 `Public` 前缀误覆盖。


### 二.5 R4 匿名可写 13 条的逐条定性

- `auth` 的 6 条（`/api/authentication` POST/DELETE、`/login`、`/logout`、`/refresh`、`/code`）：登录族本身按设计匿名，**非漏洞**。
- `personal POST /api/reset/password/anonymous`：匿名重置口令，须核其令牌/验证码前置（一轮已审该族，本轮未见回归证据）。
- `program_init POST /api/secret/set`：初始化引导端点，handler 内 `has_person` 检查令系统已有用户时拒绝写入——设计内；残留风险是**空库窗口抢注**（先于管理员设置初始化密钥），与 o2server 对齐，属部署时序而非代码缺陷，留档。
- `general_assemble_control POST /api/general/assemble/control/invoice/upload`：尺子按 handler 名取体失败（该键写的是 `invoice_upload`，二轮 S2 的 creator 会话化改在包装层）——待 A2 报告定性。
- `meeting_assemble_control` 2 条 `create_meeting`：同上盲区，待 A3 定性。
- `processplatform_assemble_surface POST .../application/list/range`、`query_assemble_surface POST /api/queryview/morelikethis`：名为 list/morelikethis 却含写 SQL，须逐条看是否落库（待 A2）。

## 三、本轮修复清单（代码）

| 项 | 文件 | 内容 |
| --- | --- | --- |
| N1 | `crates/file_assemble_control/src/lib.rs` | `docx_to_html` 加 `DOCX_MAX_XML_BYTES`(4 MiB) + `DOCX_MAX_PARAGRAPHS`(200)；**并以完全限定 `std::io::Read::take` 落地**（`f.take(n)` 会被 std prelude 的 `Iterator::take` 抢走方法解析，`&mut f.take(n)` 亦然——原写法编译不过） |
| N1 回归 | `crates/file_assemble_control/src/tests.rs` | `test_docx_to_html_caps_paragraph_count`（恰好截断在 200 段、第 201 段不输出）+ `test_docx_to_html_caps_decompressed_size`（~10 MiB 纯 ASCII 条目，段落仍 200、HTML 规模有界） |
| N4-01 | `crates/file_assemble_control/src/lib.rs` | `read_zip_entry` 加 `ZIP_ENTRY_MAX_BYTES`(4 MiB)；回归 `test_read_zip_entry_caps_decompressed_size`（9 MiB 条目必须恰好截断在 4 MiB） |
| N2 | `crates/personal_extend/src/avatar.rs` | 头像读侧 `is_safe_avatar_rel` fail-closed 守卫 + 回归；**测试里的 Windows 路径字面量改 raw string**（`"C:\windows\..."`、`"data/avatar\..\..\evil"` 是非法 Rust 转义，原写法整个 crate 测试目标编译不过） |
| 加固 | `crates/preview/src/lib.rs` | `safe_base_name` 同时按 `/` 与 `\` 取末段，剥离契约不再依赖构建平台的 `Path::file_name()` 语义 + 跨平台断言 |
| 工具 | `scripts/audit_write_gating.mjs` | 新增确定性接线尺（§二.0） |

## 四、验证

本机 host 是 `x86_64-pc-windows-gnu` 且 rustup 不自带 mingw；`oa4rust/vendor/mingw-{bin,gcc}` 是 gitignore 的仅本机产物（见 `oa4rust/vendor/README.md`），worktree 里没有。把主工作区的 vendor 工具链加进 PATH 后即可原生编译：

```
export PATH="/c/Users/user/.cargo/bin:/d/WORKSPACE/fakeToys/oa4rust/vendor/mingw-gcc/mingw64/bin:/d/WORKSPACE/fakeToys/oa4rust/vendor/mingw-bin:/d/WORKSPACE/fakeToys/oa4rust/vendor/mingw-bin/mingw64/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
cd oa4rust/backend && cargo test --lib -p <crate>
```

已跑读数：

- `cargo test --lib -p preview -p personal_extend`：**EXIT=0**，`preview` 18 passed / 0 failed（含 `security_preview_tests::safe_base_name_blocks_traversal` 的跨平台新断言），`personal_extend` 5 passed / 0 failed（含 `avatar_read_side_guard_rejects_unsafe_rel`）。
- 关键取证：进入本轮时那 3 处「已修」代码**从未编译通过**——`personal_extend` 测试目标 5 个 `error: unknown character escape`、`file_assemble_control` 的 `f.take()` 报 ``ZipFile<'_> is not an iterator``。上一轮把它们记成了已完成，本轮按红读数逐格返工。
- `cargo test --lib -p file_assemble_control`：**91 passed / 0 failed / 27 ignored**（含本轮新增 `test_docx_to_html_caps_paragraph_count`、`test_docx_to_html_caps_decompressed_size`、`test_read_zip_entry_caps_decompressed_size`）。
- 尺子读数与明细落盘：`docs/audits/artifacts/write_handlers.tsv`（函数级矩阵 991 行）、`write_gating_residual.tsv`、`write_routes_ungated.tsv`（R4 13 条 + R7 1131 条），复跑命令 `node scripts/audit_write_gating.mjs`。


## 五、处置与遗留（下一轮入口）

- 已修并验证：N1、N4-01、N2、preview 加固（读数见 §四）。
- 待修（有明确落点与修法，按 §二.1 的 R4-1…R4-6 与 §二.6 的 F1…F10 顺序收口；processplatform 族因 `ADMIN_WRITE_DEPRECATED_PREFIXES` 已撤销中间件兜底，handler 层是唯一门禁，优先级最高；F2/F3/F5 与 §二.1 的 R4-3/R4-5 同为「ownerless 系统数据写 = admin」一族，可一把收）：R4-1 / R4-2 / R4-3 / R4-4 / R4-5 / R4-6、F1 / F2 / F3 / F4 / F5 / F6 / F7 / F8 / F9 / F10、N4-02、N4-03。
- 建议统一修法（沿用本仓既有先例，避免新抽象）：引擎运行面按一轮 V2/V4 口径至少加 `require_session_opt`；改变归属/状态/删他人行的走 `shared::middleware::require_owner`（admin 旁路、行不存在放行以保 not-found 语义）；设计态（script/flow/form/xform/dict/item-access/portal）比照同 crate `guard_write` 或二轮 S5 先例加 admin/属主门；GET 状态变更改 POST。每 crate 补 `security_gate_tests`（无 DB 的 401/403 各一）。

