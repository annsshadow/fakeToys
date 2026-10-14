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
| R4-7（**本轮修尺子后新露出**） | `processplatform_assemble_surface/src/lib.rs:697 application_list_complex_manage_person`，注册为 `GET /api/processplatform/assemble/surface/application/list/complex/manage/{person}`（`routes.rs`，链式同路由的第二个方法） | **真匿名可达的 GET 写**：路径落在 `AUTH_EXEMPT_PREFIXES`（`shared/src/middleware/constants.rs:434-435`），handler 不取 Session，函数体第一条 SQL 即 `UPDATE PP_E_APPLICATION SET "xupdateTime" = NOW() WHERE xid = $1`，随后回读该行 createTime/updateTime 返回。匿名按 ID 改任意应用元数据时间戳（审计字段伪造）+ 任意应用存在性与时间信息泄漏。此前对两把中间件与路由级尺子同时隐形（GET + 链式）。**本批已修**：收会话 + admin，改法与读数见 §三.5 / §四第五批 |
| R4-4 | 中-高 | `processplatform_assemble_surface`：约 20 个 `*_mockdeletetoget`（routes.rs:419/430/470/505/529/542/558/566/569/574/622/657…）+ `correlation_job_job_delete:2778` + `mode_id_delete:8694` | 无 Session、按 xid **物理 DELETE** `PP_C_WORK`/`PP_C_TASK`/`PP_C_WORKCOMPLETED`/`PP_C_DRAFT`/`PP_C_JOB`/`PP_C_DOC_SIGN`/`PP_C_READ` 等；其中 8 个 `data_work_id_path*_mockdeletetoget`(4252 起) 本应只删数据键却整行删 `PP_C_WORK`，8 个 `applicationdict_*_mockdeletetoget`(1237 起) 物理删设计字典 |
| R4-5 | 中 | 同 crate `create_surface:81`（creator 硬编码 `"system"`，未会话化）/ `publish_surface:295` / `delete_surface:366` / `save_surface:430` | 任意改删 `x_process_surface` 应用门户配置 |
| R4-6 | 中 | `processplatform_core_express/src/lib.rs:16 work_terminate` | 注册为 **GET** `/api/processplatform/work/terminate/{id}`：无 Session 终止任意 work；GET 状态变更同时绕过 CSRF 中间件（只覆盖 POST/PUT/PATCH/DELETE），SameSite=Lax 下仍有顶层导航触发面 |

**第三批收口状态**：R4-1 的 13 把与 R4-6 的 1 把已全部补「会话 + 归属」双门并在真 PG 实测
（改法见 §三.3、读数见 §四）；R4-2（`u2.rs` 47 处写）、R4-3（designer 的脚本 / flow / form / xform /
item-access 四族）、R4-4（surface `*_mockdeletetoget` 物理删族）、R4-5（surface 应用门户配置写）待收。

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

### 二.3 CMS + 查询族越权/写门禁（A2，已完成：候选 201 行 → 14 条发现）

匿名可达写端点 0（cms/query 的豁免项全为只读）。下表「复核」列写 **已验** 的才是我逐行读过代码确认的；
其余为代理给出的 file:line 证据，本批未逐条二次取证（下一批入口）。

| id | 级别 | 落点 | 攻击面 | 复核 | 修法 |
| --- | --- | --- | --- | --- | --- |
| A2-F1 | 高 | `query_assemble_surface:223/262`、`query_assemble_designer:506/544/618/685` | 按 IDOR 覆写/删除他人查询设计与 surface | 未复核 | 复用同 crate `guard_write` / `require_admin_gate` |
| A2-F2 | 高 | `query_assemble_surface:2261` + `u2_closures:725/752/782/812` | `table/row/delete/all/{tableFlag}` **物理清空**任意数据表全部行 | 未复核 | 表属主 `guard_write` + 改软删 |
| A2-F3 | 高 | `query_assemble_designer:154/203/231`（路由 `:862`） | 任意用户触发建表/加列 DDL（标识符已校验，权限错位） | 未复核 | `require_admin` |
| A2-F4 | 高 | `cms_assemble_control:5881/5902`（GET 路由 `:477/:701`） | GET 全表软删权限缓存 → 权限判定消费者失效 + 绕 CSRF | 未复核 | `u2_require_admin` + 移出 GET |
| A2-F5 | 高 | cms 遗留 CRUD 与 `mock*` 写族（20+ 处） | 无 session 无门禁写/软删 cms 内容表，含 GET 变体 | 未复核 | `u2_require_admin` / `u2_check_owner` |
| A2-F6 | 中高 | `cms_assemble_control:165 update_control_config` | 任意用户翻 `allow_anonymous`，把只读豁免面自行扩大 | 未复核 | `require_admin`（hotpic S5 先例） |
| A2-F7 | 中 | `cms:4030 file_id_upload` + `file_copy_u3` / `fileinfo_upload_with_url_u3` | 按任意 id upsert `x_cms_file`（含 content_base64）→ 文件内容替换 | 未复核 | `fileinfo_u2_doc_gate` |
| A2-F8 | 中 | `cms:10113/10162 correlation_create_u3/update_u3` | 与已门禁的 `correlation_u2_doc_delete` 自相矛盾 | 未复核 | `u2_gate_by_sql` |
| A2-F9 | 中 | `cms:7556/7585 input_create_impl/input_cover_impl` | 绕开 `appinfo_u2_create` 的 admin 门直接写 `x_cms_appinfo` | 未复核 | `u2_require_admin` |
| A2-F10 | 中 | `query_service_processing:254 reset_service` 等运维/重建类写 | 无 admin 判定 | 未复核 | `require_admin` |
| A2-F11 | 中 | designer/surface 共 9 处 `creator` 硬编码 "system" | create 无门禁 → 无限建对象；而 edit/delete 走 `require_owner` 时普通用户永不可自管 | 未复核 | creator 会话化 + create 加门 |
| **A2-F12** | **高** | `query_core_express:27 execute_query`（`POST /api/query/core/express/execute`） | **任意认证用户读全库任意表**：仅判 `Statement::Query`，行级过滤靠列名拼接，自带 `x_identity`/`x_unit_id` 别名即贯穿；**且 `get_permission_filters` 取不到身份/单位时 `where_clause` 为空 → 原样执行未过滤 SQL**（`:79-83` 的 else 分支） | **已验** | **本轮已修**：admin 门 + 谓词为空即拒（§三） |
| A2-F13 | 低中 | `query_core_entity:208`、cms 点赞/密文发布、`cms_core_entity:558/573/589` | 署名取自 body/path；`x_cms_column_manager` 全仓无消费方（仅 migration 093），故不构成实际提权 | 部分已验（列管理表无读者） | 身份会话化 + owner gate |
| A2-F14 | 低中 | cms 7 处 `*_u2_create` | 向他人应用注入表单/脚本/视图设计 | 未复核 | `u2_gate_by_sql`（`x_cms_appinfo.manager`） |

A2 开放问题的取证结果见 §二.7（注册表 Public 与中间件兜底的实际关系）。

### 二.4 中量 crate 越权/写门禁（A3，已完成：候选 306 行 → 13 条发现）

| id | 级别 | 落点 | 攻击面 | 复核 | 修法 |
| --- | --- | --- | --- | --- | --- |
| **A3-N1** | **高** | `portal_assemble_designer/u2_script.rs:38 create`、`:85 update`、`:141 delete`（路由 `lib.rs:564-569`） | 脚本正文由 portal surface 读回渲染，CSP 允许 `script-src 'self' 'unsafe-eval'` → 覆写他人脚本 = 在受害者门户执行存储型 JS；`delete` **连 session 都不取** | **已验**（并更正代理表述：create/update 其实已取 session，缺的是**归属/角色门**而非身份） | **本轮已修**：三把写收 crate-local `require_admin`（§三） |
| **A3-N2** | **高** | `portal_assemble_surface:2625/2648/2718 dict_*_data_delete/post/put`、`:2808 script_portal_portal_name_post` | 二轮 S5 只门了同族兄弟，这 4 把漏网：任一登录用户删/覆写任意 `portal_flag` 字典数据、往任意门户插脚本 | **已验**（顺带取证：同 crate 的 `dict_*_data_mockdeletetoget` 名字带 delete、实际只 SELECT，非写） | **本轮已修**：复用同 crate 既有 `require_admin` |
| **A3-N3** | **高** | `meeting_assemble_control:2237 accept`、`:2361/:2384 confirm_allow/deny`、`:2292 checkin`、`:2260 add_invite`、`:2094 create` / `:2151 save` | 完全不取 session：任意批准/驳回会议、以他人身份打卡、任意塞受邀人、creator 冒名；accept/confirm/checkin **还注册为 GET**（`routes.rs:112` 等）→ 跨站 GET 可驱动他人审批 | **已验** | **本轮已修**：写族要求会话；状态/归属类走 `require_owner`（先读 `x_meeting.creator`）；creator/person 改会话权威 |
| A3-N4 | 中高 | `attendance_assemble_control` 规则/全局配置/申诉/详情签收 9 处 | 任意用户翻模块配置与申诉状态；`personId` 取 body 可代他人「签收」考勤 | 未复核 | 规则/配置 `require_admin`，签收 owner-or-admin |
| A3-N5 | 中高 | `jpush_assemble_control` 全 crate 0 门禁（`:354` 解绑全部、`:859 message_send`、`:105` 配置） | 推送劫持 / 伪造 "system" 推送 / 推送 DoS；多处 GET+POST 双注册 | 未复核 | `u2_require_admin_opt` + creator 会话化 + 撤 GET |
| A3-N6 | 中高 | `program_init:341/395/491/531/553/658` | 任意登录用户写入外部数据源 **host/port/user/password** 并置 applied；`server_execute`/`server_stop` 为 GET；`restore_upload` 落 ≤200 MB 临时文件；`external_datasources_validate` 用裸 `tokio::net::TcpStream` 探任意 host:port（**绕过 `shared::netguard`** = 内网端口扫描） | 未复核 | 全族 `require_admin` + 出站走 netguard |
| A3-N7 | 中 | `mind_assemble_control:227/257/309/413` + `u2.rs:399-424` | u2 侧正确使用 `person_can_manage_mind`，lib.rs 文件夹族无 session 即按 id 改名/移动/软删他人脑图 | 未复核 | 复用 `person_can_manage_mind` |
| A3-N8 | 中 | `ai_assemble_control:1326/1808/1886/1511/1288` | `DELETE FROM x_ai_file/x_ai_index WHERE id=$1` 无归属判定 → 销毁他人知识库 | 未复核（同族 `chat_delete_clue_id` 确有 owner 门，代理已排除） | `require_owner` + 同步类 `require_admin` |
| A3-N9 | 中 | `personal_extend/personal.rs:328/343/359` 经 `shared/crud.rs:35/76/115` | **任意用户创建/覆写/软删他人 `x_person_face` 生物特征行**；`CrudSpec` 通用写本身无 owner 谓词，波及 11 个 crate | 未复核（危害最大的一条通用面，优先级高） | `CrudSpec` 增 owner 列或 face 写 `require_admin` |
| A3-N10 | 中 | `file:723 permission_set`、`:557/656/713/282` | 任意 upsert `FILE_PERMISSION`（本仓暂无读者 = 潜在 ACL 绕过）、按 id 改名/软删他人文件夹 | 未复核 | owner / admin 门 |
| A3-N11 | 中低 | `bbs:318`、`calendar:100`、`jpush:105`、`mind:73` 的 `update_control_config` | 模块级全局开关（含 `allowAnonymous`）任意登录用户可翻 | 未复核 | `require_admin`（S5 先例） |
| A3-N12 | 中低 | GET 注册的写端点跨切族（calendar/jpush 配置、meeting accept/confirm/checkin、attendance 两处） | 同时绕开 csrf（只覆盖 POST/PUT/PATCH/DELETE）与 `requires_admin`（`matches!(method, POST\|PUT\|DELETE)`） | **已验**（csrf 与 requires_admin 的方法口径本轮两处独立复核） | 改 POST/PUT/DELETE；若 o2server 对齐必须保留 GET，则 handler 层门 |
| A3-N13 | 低 | `general_assemble_control:1837/1875/1913 invoice_upload*` | creator 取请求体 → 冒名归属 | 未复核 | `session.person_unique`（S2 先例） |

### 二.7 一条新的**跨切机制缺陷**：注册表 Public 早退吞掉中间件 admin 兜底（本轮自查 + 尺子升级取证）

取证链（三段都能在代码里逐字对上）：

1. `authorize_middleware`（`shared/src/middleware/rbac.rs:535-541`）在 `get_permission(path)` 命中
   `Public` 时 **`return next.run(request)` 提前放行**，根本不再调 `check_permission`。
2. 而 `requires_admin`（中间件 admin 兜底）**只有 `check_permission` 一个调用点**
   （`rbac.rs:451`，全仓 grep 无第二个），即那把兜底长在早退路径的下游。
3. `get_permission` 的前缀匹配是裸 `path.starts_with(prefix)`，**没有段边界**
   （`rbac.rs:412`），所以 `/api/person/custom` 这条 Public 会把 `/api/person/customX/...` 一并吞进去。

结论与边界（避免误读）：

- **不构成匿名可达**：`auth_middleware`（`token.rs:135-154`）在 `authorize` 之外层，非豁免路径
  无会话仍是 401。所以 Public ≠ 匿名，A2 关于「`/api/document/batch` 注册 Public 是否有意」的疑问，
  答案是「 Public 只影响 authorize 的 Admin 判定，不影响认证」。
- **构成兜底缺失**：`ADMIN_WRITE_PREFIXES`（6 条）与注册表 34 条 Public 前缀的交集有 **15 条**
  （`/api/person/{icon,list,detail,has/role,custom,definition,nick/name,list/all}`、
  `/api/group/{list,has/role}`、`/api/role/list`、`/api/unit/{list,check,identity,list/all}`），
  落在这 15 条之下的写路由**没有中间件 admin 兜底，handler 层是唯一门禁**。
- 尺子已按该口径升级（新增 `registryPublic()` 复刻段无边界前缀匹配），分桶读数：
  **G-Public早退吞掉admin兜底且无体内门禁 = 4**、H（有体内门禁）= 1、A（真兜底）= 27、
  B（匿名且无门）= 7、C = 2、D（登录即可写且无门）与 F/E 不变。
- **G 的 4 行逐条取证后均为「按会话本人」的自助写，判为良性**：
  `personal/u2.rs:238 custom_edit`、`:280 custom_delete`、`:441 definition_edit` 三处都先
  `resolve_current_person_unique(...)`；`personal_extend/avatar.rs:53 upload` 走
  `validate_session` 后只写本人 `auth_person.icon`。故 G 当前不是漏洞，而是**兜底缺失的暴露面**：
  一旦同前缀下新增不带归属判定的写 handler，中间件不会兜住。建议下一轮加守卫：
  「`ADMIN_WRITE_PREFIXES` ∩ 注册表 Public 的差集必须逐条点名，且新增者必须有体内归属门」。
- 同一尺子新出的 B 行里 `program_init:139/186 /api/secret/set(+/cancel)` 落在
  `AUTH_EXEMPT_PREFIXES` 的 `/api/secret` 之下 = **匿名可写系统初始化面**，与 A3-N6 同族，
  已并入 §五 待修（这是本轮唯一「真匿名写」新增格）。

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
| 工具 | `scripts/audit_write_gating.mjs` | 新增确定性接线尺（§二.0）；**本轮再升级**：复刻 `PermissionRegistry` 的 Public 段无边界前缀匹配，新增 G/H 两桶（§二.7） |


### 三.2 第二批（A2/A3 收口，代码）

| 项 | 文件 | 内容 |
| --- | --- | --- |
| **A2-F12** | `crates/query_core_express/src/lib.rs` + `security_gate_tests.rs` | `execute_query` 加 admin 门并把 `where_clause` 为空那一支从「原样执行未过滤 SQL」改成 `Err(Forbidden)`；`cache_query_result` 补 session + admin 门，`creator` 由硬写 "system" 改会话权威。离线回归两支：非 admin 双 403 + `build_where_clause(&None,&None)` 必为空（钉住 fail-closed 的触发条件本身） |
| **A3-N1** | `crates/portal_assemble_designer/src/u2_script.rs` + `security_gate_tests.rs` | 该 crate 零门禁引用：`create/update/delete` 三把脚本写加 crate-local `require_admin`（`is_admin` 无 DB fail-closed），`delete` 补 `Extension<Session>`。存储型 JS 面（surface 读回 + CSP `unsafe-eval`） |
| **A3-N2** | `crates/portal_assemble_surface/src/lib.rs` + `security_gate_tests.rs` | S5 漏收的 4 把（dict data delete/post/put + script post）复用同 crate 既有 `require_admin`；新增 4 支 403 断言。附带取证更正：`dict_*_data_mockdeletetoget` 名为 delete 实为 SELECT |
| **A3-N3** | `crates/meeting_assemble_control/src/lib.rs` + `security_gate_tests.rs` | 新增 `u2_meeting_owner_gate`（读 `x_meeting.creator` → `require_owner`，行不存在放行保 not-found）；`save_meeting`/`confirm_allow`/`confirm_deny`/`add_invite` 接归属门，`create_meeting` 的 creator 与 `checkin` 的 person 改会话权威，`accept` 至少要求有效会话（会议级状态翻转的产品口径留 §五 待拍）。GET 注册本批未撤（前端自检面板在用，改方法需同批改前端） |
| **F3** | `crates/portal/src/lib.rs` + `security_gate_tests.rs` | `save_page` / `delete_page`（后者是物理 `DELETE FROM x_portal_page`）加 crate-local `require_admin`（`shared::middleware::is_admin`，无 DB fail-closed）；离线回归 `portal_page_writes_forbidden_for_non_admin` 断言两把 403 |
| **F6** | `crates/ai/src/index.rs` + `crates/ai/src/tests_generated.rs` | `index_delete` 加 `Extension<Session>` + `is_admin` 门（ownerless 系统数据，按 S5 裁决收 admin）；原直调测试 `_result` 弃读改成 403 断言（离线，无 DB 时 fail-closed）。路由仍挂 GET——改 HTTP 方法需前端同批，列为跨切项不做单边改动 |
| **F9** | `crates/bbs/src/subject.rs` | 发帖 `author_id` 由「完全取请求体」改为以会话身份为权威（`session.person_unique`），杜绝任意 person 冒名发帖；既有 `test_subject_create` 走 oneshot 无会话→仍 500，不掩盖变更 |
| **F2** | `crates/portal_core_entity/src/lib.rs` + `security_gate_tests.rs` | 同表另一条写路径 `page_create`（**函数级索引漏收**，路由级尺子收到）/ `page_update` / `page_remove` 同加 `require_admin`。本 crate 走 sea-orm `DatabaseConnection`、拿不到 deadpool `Pool`（router 收下 `_pool` 却从未 layer 注入），故按 `is_admin` 同一条 SQL 就地判 admin 并 fail-closed；门禁回归需真实 PG，标 `#[ignore]` 并在文件头写明跑法 |

### 三.3 第三批（A3-N6 / R4-6 / R4-1，代码）

| 项 | 文件 | 内容 |
| --- | --- | --- |
| **A3-N6 + §二.7 新 B 格** | `crates/program_init/src/lib.rs` + `security_gate_tests.rs` | 外部数据源 host/port/账密写（`external_datasources_set` / `_set_cancel` / `_validate`）、`restore_upload` / `_cancel`（200 MB 落盘）、GET `server_execute` / `server_stop`（停服）7 把全部补 `Extension<Session>` + crate-local `require_admin`。`/api/secret/set(+cancel)` 按引导流程**保持匿名**，但「已初始化」探测原为 `.map_err(\|_\| Internal).map(...).unwrap_or(false)`——查询一出错就当作「未初始化」放行，DB 抖动时匿名可重写引导密钥，两处均改 `?` 传播（fail-closed）。回归 `program_init_ops_writes_forbidden_for_non_admin` 断 5 把 403。**未收**：`external_datasources_validate` 的裸 `TcpStream` 探任意 host:port 仍绕过 `shared::netguard`（需改走 netguard 的出站口径，留 §五） |
| **R4-6** | `crates/processplatform_core_express/src/lib.rs` + `security_gate_tests.rs` | GET `/api/processplatform/work/terminate/{id}` 原不取会话不校归属。改 `Option<Extension<Session>>`（无会话 401）+ 读 `x_work.creator` 走 `require_owner`；行不存在放行以保 not-found 语义。同文件的 `work_retract` 名为 retract 实为只读 SELECT（同 `mockdeletetoget` 一族），不在门禁面内。离线两支（路由级 + handler 级 401），DB 一支 `#[ignore]` |
| **R4-1（processing，13 把）** | `crates/processplatform_service_processing/src/lib.rs` + `security_gate_tests.rs` + `tests.rs` | 新增两把局部门 `u4_session`（无会话 401，先于任何 SQL）与 `u4_require_actor`（负责人 / 发起人非空 → `shared::middleware::require_owner`，admin 旁路、无库 fail-closed；为空的历史行只要求有效会话）。接门：`task_claim` / `task_complete` / `task_reject` / `task_transfer`（按 `x_task.person`）、`work_v2_id_terminate` / `work_v2_id_retract` / `work_v3_retract` / `work_start` / `work_complete` / `work_id_processing` / `execute_process` / `cancel_process_instance` / `gateway_join`（按 `x_work.creator`）。`gateway_fork` 一次按 gateway 实例跨多个 work 批量插任务、无单一属主，且只有运维面板调用 → 比照二轮 S5 收 **admin**。冒名同批收口：`task_claim` / `work_v2_id_terminate` / `gateway_join` 的 `x_record.creator` 由硬写 `"system"` 改 `session.person_unique` |
| **R4-1 的旧断言返工** | `crates/processplatform_service_processing/src/tests.rs` | 加门后 3 条既有用例点名变红，逐条按修复后口径改写而非删除：`test_execute_process_route_exists` / `test_cancel_process_instance_route_exists` 原以 **500** 为「路由存在」的证据（无会话直穿到 DB 层），现断 401；`test_work_start_status_transition_db_connected` 原走 router 断 200（router 只 layer Pool，必然 401），改按 handler 直调 + 同名会话取放行分支，并补验 `work_status` 真落库与 Start Event 任务确实创建（收尾删净）。**教训**：`route_exists` 型断言把状态码钉死在某个具体值上，任何门禁收紧都会打到它——改写方向应是「门拒绝」而不是「放行」 |
| **R4-1 的行为副作用（需观察）** | 同上 | 引擎 `work_start` 创建的 Start Event 任务 `person` 写成 `"system"`，收紧后该类任务只有 admin（或字面为 `system` 的会话）能代办。已在 `u4_require_actor` 文件头点名。若后续发现真实流程依赖普通用户代办 Start Event，应按「引擎内部推进」补专用路径，而不是放宽归属门 |


### 三.4 第四批（R4-2 前两 tranche：`u2` 实体写 16 + 13 把，代码）

| 项 | 文件 | 内容 |
| --- | --- | --- |
| **R4-2 批 1（16 把）** | `crates/processplatform_service_processing/src/u2.rs` + `security_gate_tests.rs` + `tests.rs` | `work_edit` / `work_delete`（事务内 `SELECT creator … FOR UPDATE`，含级联物理删 x_task / x_draft）/ `work_draft_delete` / `work_v3_retract_body` / `task_delete` / `taskcompleted_delete` / `readcompleted_delete` / `record_edit` / `record_delete` / `review_delete` / `att_delete_id` / `att_edit_id` / `att_edit_text` 走归属档；`dict_edit` / `job_delete` / `snap_delete` 属引擎运维面、行上无属主（`x_snap` 连 creator 列都没有），收 admin 档。新增三把局部工具：`u4_actor_sql`（按实体取归属列）、`u4_actor_of`（读归属）、`u4_require_actor_of`。**严格口径**：归属非空 → `require_owner`（admin 旁路）；归属**为空或缺失 → 只允许 admin**，不用「空归属 = 人人可写」把数据缺陷洗成权限放行。`work_v3_retract_body` 的 `x_record.creator` 由 `"system"` 改会话权威 |
| **R4-2 批 2a（13 把）** | 同上 | `att_delete_with_work` / `att_delete_with_workcompleted`：在**原有 IDOR 等值校验之后**再校 `x_attachment.creator`，两道门都在。<br>`att_copy_to_work`：复制是往目标写，校**目标 work 发起人**，并把 `entity_exists` 换成读 creator 的归属查询（不存在仍报 `work not found`）。<br>`read_delete` / `read_processing` / `read_replace` / `read_reset`：按 `x_read.person` 严格校。<br>`documentversion_create` / `work_add_identity_matrix` / `work_processing_signal`：事务内锁 work（或 `x_task` 的 `person`）后校归属，留痕 creator 取会话权威。<br>建单三只 `work_create` / `work_create_for_process` / `work_create_with_serial`：`work_start_impl` 现收 `person` 参数，creator 不再取请求体——冒名建单（body 写受害者身份）失效 |
| **旧断言返工（批 1 + 2a 共 8 支）** | `crates/processplatform_service_processing/src/tests.rs` | 共享 `send()` 助手前置断言「HTTP 必 200」（ActionResult 契约：业务错也 200），而新门返回真 401/403，故 8 支用例点名变红。逐条按修复后口径改写、不删不 ignore：新增 `send_raw_as(method, uri, body, person)` 注入指定身份会话且不前置断言；`u2_work_v3_retract_…` 另断 `record.creator == 发起人本人`；`u2_dict_path_…` 的改名步改断 403（放行侧由 `dict_edit_allowed_for_admin_and_lands` 真跑取证的 admin 臂覆盖）；`u2_attachment_copy_dedups_by_name` 补闯入者 403 + 目标 work 零写入；`u2_documentversion_…` 两支断 body 的 `creator` 被忽略、入库仍为会话身份；`u2_work_create_via_process_path_…` 断 `x_work.creator='u42-booter'`（而非 body 里的 `u42-victim`） |
| **allow 侧真取证** | `security_gate_tests.rs` 的 `seed_admin` / `unseed_admin` | admin 档不能只断「非 admin 403」——那是半边证明。`dict_edit_allowed_for_admin_and_lands` 往 `auth_role`(`name='admin'`) / `auth_person` / `auth_person_role` 插**真实** admin 行，跑通放行并验证数据落库，收尾清理。 |
| **已知行为副作用（沿用 §三.3 那条，未放宽）** | 同上 | 引擎自建行（Start Event 任务 `person="system"`、空 creator 的历史/孤儿 work）在严格口径下变成 admin-only。 remedy 方向已记：补「引擎内部推进」专用路径，而不是放宽归属门。`work_processing_signal` 与 `work_start_impl` 同承此风险 |

### 三.5 第五批（R4-7 + 尺子缺陷 3，代码）

| 项 | 文件 | 内容 |
| --- | --- | --- |
| **R4-7** | `crates/processplatform_assemble_surface/src/lib.rs`（新增 crate-local `require_admin_opt` + handler 加 `Option<Extension<Session>>`）、`routes.rs`（别名路由闭包透传会话）、`security_gate_tests.rs`（**新建并在 lib.rs 注册 `mod`**，两支离线臂） | 见 §二.1 R4-7。按应用管理面收 admin：无会话 401（先于 `pool.get()`，可离线证）、非 admin 403（`is_admin` 无库 fail-closed）。**已知行为差**：桌面 `ProcessWork.vue:2345` 的运维控制台按钮（弹窗手输人员）非 admin 会改判 401/403；该调用不是终端用户流程，按 §三.4 admin 档处理，若产品要求「本人 touch 自己的应用」再按 self-service 档改 |
| **尺子缺陷 3（门禁词表不全）** | `scripts/audit_write_gating.mjs` | `GATE_RE` 旧写法 `\b(require_admin|…)\b` 匹配不到 `require_admin_opt(`：`require_admin` 之后紧跟 `_` 仍是词字符，尾部 `\b` 落空。后果是**加了门的 handler 仍被计成无门禁**——本批 surface 那格修完还留在 K1，靠这条读数才发现词表问题。改为按完整标识符匹配，补 `require_admin_opt / require_owner_opt / require_actor / u4_require_actor(_of) / u4_session / require_owner / validate_session / extract_token_from_headers` 等 |

### 三.6 第六批（R4-2 批 3：六把 touch 写 + 尺子词表改前缀匹配，代码）

| 项 | 文件 | 内容 |
| --- | --- | --- |
| **processing 两把 touch** | `crates/processplatform_service_processing/src/lib.rs` + `security_gate_tests.rs` | `service_work_id_touch` / `_p2` 是**薄封装**（函数体只有 `…_core(pool, id).await` 一行），core 不取会话、不校归属，`INSERT INTO x_record` 的 creator 硬写 `"system"`。改为 core 收 `&Session`：先按已 SELECT 出的 `x_work.creator` 走 `u4_require_actor`，留痕 creator 改 `session.person_unique`；两把 wrapper 用 `u4_session` 先 401（在任何 SQL 之前）。离线臂 `touch_writes_reject_without_session` + DB 臂 `touch_requires_work_creator_and_pins_real_actor`（闯入者 403 **且 x_record 零留痕**；发起人放行且 creator 落本人） |
| **surface 四把 touch 路由** | `crates/processplatform_assemble_surface/src/lib.rs` + `security_gate_tests.rs` | `service_work_id_touch` / `_mockputtopost`（PUT/POST × 正式/别名 = 4 条路由）按 ID 直接 `UPDATE PP_C_WORK."xupdateTime"` 并回读时间字段，原先不取会话。新增 crate-local `u4_require_session`（无会话 401 先于取连接）与 `require_actor_opt`（归属非空 → 本人或 admin；**空归属 / 行缺失 → 只允许 admin**，沿用 §三.4 严格口径），写前先 `SELECT "xcreatorPerson" FROM PP_C_WORK WHERE xid=$1` 取归属再校。离线臂 `surface_touch_writes_reject_without_session` 逐条断四把 401 |
| **尺子缺陷 3 的复发修正** | `scripts/audit_write_gating.mjs` | 上一批把 `GATE_RE` 改成「完整标识符全名单」，本批新长出的 `require_actor_opt` / `u4_require_session` 又不在名单里，**同一类假红第二轮复现**（代码已修、读数仍挂 R6）。改为**前缀匹配**：`require_admin*` / `require_owner*` / `require_actor*` / `require_session*` / `u4_*` / `u2_require*` / `guard_write*` / `owner_gate*` 等一律算体内门禁，从此不需要每次改名同步。**没被收成恒真**的反证：R7 仍 1165、函数级仍 991 行 / 残余 11 行完全不动，只有 R6 262→228、R5 371→405、GET 写 165→162（K2 16→13）在变 |

### 三.7 第七批（豁免/注册表双锁收口 + 初始化密钥销毁侧同锁 + 尺子缺陷 9，代码）

本批的入口不是「又找到一条漏洞」，而是**上一批那句结论说反了**。§三.5/§四 曾把
「写路由被同层 `{id}` 通配豁免吞掉」记成「还有注册表第二把锁兜着，实测 401，所以不是匿名面」——
把请求真打一遍之后这个口径整体倒转：**豁免早退时不插 Session，而 authorize 只在注册表 Public 时放行**，
于是「豁免 ∧ ¬Public」这条组合对**带合法会话的 admin 也是 401**。它不是安全兜底，是**半开的双锁 = 全员不可达的可用性死态**；
而且「修好它」会顺带**打开**一批此前被死态掩盖的写面，所以必须与归属权威同批做。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| **F1 双锁一致性** | `crates/shared/src/middleware/token.rs` | auth 层的豁免早退由「只看豁免表」改为 `is_auth_exempt(&path) && (init_window \|\| (!secret_hit && is_registry_public(&path)))`：两把锁必须同开才允许匿名，`SECRET_INIT_PATHS` 的初始化窗口保持原有严格性不变。**为什么改 auth 侧而不是把 19 条路径逐个补进注册表**：注册表是「这条路由公开吗」的唯一权威，让 auth 层单方面按豁免表放行等于把同一个问题留了两份答案——只要两份表还可能各自演化，就会反复长出死态或漏放行 |
| **F2 登录前置流程显式公开** | `crates/shared/src/middleware/rbac.rs` | 新增 `is_registry_public()` 供 auth 层判；`with_defaults()` 补两条 `register_exact`：`/api/person/regist`、`/api/secret/captcha/verify`（注册与验证码是登录前置流程，按设计必须匿名可达，此前只在豁免表里，落在差集里 = 死态） |
| **F3 公开即限流** | `crates/shared/src/middleware/constants.rs`（`AUTH_RATE_LIMIT_EXACT`） | 上面两条同时进认证族限流名单。`rate_limit_middleware` 对「豁免且未列限流」直接跳过，所以「公开但不限流」= 匿名无上限批量建号 + 验证码暴力枚举；限流是 F2 的前置条件，不是配套 |
| **F4 初始化密钥销毁侧同锁** | `crates/shared/src/middleware/constants.rs`（`SECRET_INIT_PATHS`） | 补 `/api/secret/set/cancel`。`path_matches` 要求**段数相等**，`/api/secret/set` 不吞 `/api/secret/set/cancel`，所以 `set` 只在未初始化窗口匿名、`set/cancel` 却**永久匿名可达**——一个匿名 GET 就能 `DELETE FROM secret_config WHERE id='init-secret'`。handler 体内 `has_person` 是唯一一道锁，而它在「全库用户被锁定 / 软删」的维护态下同样失效。§四 曾把这条记成 K1「待定性」，本批定性与收口一并做掉。**顺带验真**：`AUTH_RATE_LIMIT_PREFIXES` 里的 `/api/secret/cancel` 是打不中任何路由的死键，但真实路径已由 `/api/secret/set` 前缀（段边界匹配）覆盖，故限流面不缺口，那条死键留给死键普查批处理 |
| **F5 冒名归属面随死态打开而收口** | `crates/general_assemble_control/src/lib.rs` | `invoice_upload` / `invoice_upload_for_create` / `invoice_upload_with_url` 三把的 `creator` 由「取请求体、缺省 `system`」改为 `session.person_unique`（与同 crate `invoice_create` 同口径），handler 加 `Extension<Session>`。**这批必须和 F1 同批**：F1 让这几条从「谁都打不通」变成「登录可达」，如果 creator 仍取请求体，修一条可用性就开一个「任意登录用户把单据挂到别人名下」的写面 |
| **尺子缺陷 9（注册表建模只认一种写法）** | `scripts/audit_write_gating.mjs` | 建模时只复刻 `registry.register(...)`，漏掉 `register_exact` / `register_prefix` 与跨行写法 ⇒ 大量在册注册被当成「没注册」，`registryPublic` 假阴。6c 首跑因此报 57 条（含生产显然活的 `/api/queryview/list`），修解析后 19 条、真匿名面同时 132→170。**新旧读数口径断裂，不可直接比**；另补「解析到 N 条 exact + M 条 prefix」的自证行，让尺子下次少解析时会自己喊 |
| **6c 新桶 + 读数自校准** | `scripts/audit_write_gating.mjs`、`docs/audits/artifacts/dead_exempt_routes.tsv` | 新增「auth 豁免 ∧ ¬注册表 Public」差集（7 列，带 `eaten_by` 与 `registry_level`）。桶名**不写死**：脚本自己去 `token.rs` 读 F1 是否接线（`F1_WIRED`），未接线读作「死态：所有用户不可达」，已接线读作「惰性豁免条目（需登录，豁免本身失效）」——回退 F1 时读数会自动改口径，不会继续把真死态报成噪音 |
| **A1 匿名写行动面改为点名门禁** | `scripts/audit_write_gating.mjs` | 「真匿名 + 体内写 SQL + 无门禁无会话」是匿名侧唯一真能落笔的面，写成**集合相等**（现名单 3 条：`auth POST /api/authentication`、`auth POST /api/authentication/login`、`personal POST /api/person/regist`）而非「数量 ≤ N」：新增匿名写路由要多打一条，名单里的条目被修掉也要多划一条，两个方向都必须红。注入臂取证见 §四 |
| **两把全栈守卫** | `tests/anonymous_write_guard.rs`、`tests/exempt_authorize_parity.rs`（均新建） | 见 §四 第七批读数。前者按「匿名请求 → 状态码 + 库面行数 + 文件系统面」三重判据打真实全栈；后者只量可达性下限（带合法会话不得 401、设计上匿名的不得 401），逐臂打印状态码与响应体 |

本批另记三条**推翻自己先前假设**的取证，避免下一轮再顺着旧口径走：

1. **「匿名可在门禁前消耗请求体」被证伪**：axum 0.8.9 的 `Multipart::from_request` 只 parse boundary、
   不读体，且默认体限 2MB——`icon/upload` 那类上传面不会因为豁免而变成匿名资源消耗面。
   副作用发现：`MAX_FILE_SIZE = 5MB` 在 2MB 默认限下是**死代码**（fail-closed 的策略不一致，待拍是否让二者对齐）。
2. **「content-type 不对所以 400」不是门禁有效**：icon 臂首跑 400 而非 401，查下来是我的 multipart 构造
   被 content-type 校验拒了；改成发真 PNG magic 的文件，让 401 由门禁给出，才量到该量的东西。
3. **修可用性会自己造出残留**：`invoice_upload` 的 creator 走 body 缺省 `system` 时，按 `creator=会话人`
   过滤的清场漏删它落的空名行；一次性清扫真跑到并删除 1 行（`create_time` 在该表是 TEXT，
   不能和 `timestamptz` 直接比，改按 `to_char` 前缀做字典序窗口），清扫测试用完即从文件删除。
   同批把「顺手删」的 `let _ = client.execute(...)` 兜底改成**失败即响**的残留断言——
   会议臂本应 400 不落行，一旦真落行必须先红，而不是被静默的 `let _ =` 抹掉证据。

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
- `cargo test --lib -p portal`：**13 passed / 0 failed**，含新增 `security_gate_tests::portal_page_writes_forbidden_for_non_admin`（F3 离线 403 双断言）。
- `cargo test --lib -p ai`：**51 passed / 0 failed**，其中 F6 的 `test_index_delete` 由「弃读 `_result`」改成 `Err(AppError::Forbidden)` 断言。
- `cargo test --lib -p bbs`：**7 passed / 0 failed**（F9 改会话权威后既有 oneshot 测试仍绿）。
- `cargo test --lib -p portal_core_entity`：**18 passed / 0 failed / 1 ignored**（F2 首跑编译断裂两处——sea-orm 1.1.20 的 `QueryResult::try_get::<T>(pre, col)` 需要两参，已按 `try_get::<bool>("", "is_admin")` 返工；`ignored` 即 `portal_page_entity_writes_forbidden_for_non_admin`，需真实 PG，跑法 `cargo test --lib -p portal_core_entity -- --ignored`）。

- 尺子读数与明细落盘：`docs/audits/artifacts/write_handlers.tsv`（函数级矩阵 991 行）、`write_gating_residual.tsv`、`write_routes_ungated.tsv`（R4 13 条 + R7 1131 条），复跑命令 `node scripts/audit_write_gating.mjs`。

第二批（§三.2）读数：

- `cargo test --lib -p query_core_express`：**13 passed / 0 failed**，含新增
  `query_express_writes_and_raw_sql_forbidden_for_non_admin`（execute/cache 双 403）与
  `empty_permission_predicate_is_not_a_free_pass`。
- `cargo test --lib -p portal_assemble_designer`：**25 passed / 0 failed / 1 ignored**，按名复核
  `designer_script_writes_forbidden_for_non_admin` = ok。
- `cargo test --lib -p portal_assemble_surface`：**26 passed / 0 failed**，按名复核
  `portal_dict_and_script_writes_forbidden_for_non_admin` = ok；既有 `soft_delete_lifecycle_tests`
  两支 DB 生命周期测试改为需 `OA4RUST_TEST_ADMIN_PERSON` 才跑（A3-N2 后这 4 把写仅 admin，
  未配置 admin 人选即显式 skip，不伪装成绿）。
- `cargo test --lib -p meeting_assemble_control`：**60 passed / 0 failed / 25 ignored**，
  二轮既有 `config_manage_requires_session` / `config_manage_forbidden_for_non_admin` 两支仍在册并绿，
  新增 `meeting_state_writes_forbidden_for_non_owner_non_admin`（`#[ignore]`，需真 PG）。
- 尺子升级后分桶：G=4（逐条取证为会话本人自助写，良性，见 §二.7）、H=1、A=27、B=7、C=2；
  B 里新增真匿名写格 `program_init /api/secret/set(+/cancel)`。

第三批（§三.3）读数——**本机 PostgreSQL 可达，故本轮 DB 臂全部实测而非 skip**：

- `cargo test --lib -p program_init`：**24 passed / 0 failed / 0 ignored**，按名复核
  `program_init_ops_writes_forbidden_for_non_admin` = ok；
  `cargo test --lib -p parity program_init`：**15 passed / 0 failed / 4128 filtered out**（加门未破路由存在性对账）。
- `cargo test --lib -p processplatform_core_express`：**8 passed / 0 failed / 1 ignored**，
  路由级与 handler 级两支离线 401 按名 ok；`-- --ignored` 实跑属主臂：闯入者 403、属主放行、
  `work_status` 落 `terminated`。
- `cargo test --lib -p processplatform_service_processing`：**60 passed / 0 failed / 4 ignored**。
  三支离线臂按名 ok：`engine_runtime_writes_reject_without_session`（task 4 + work 3 共 7 把 401）、
  `instance_progression_writes_reject_without_session`（execute/cancel/processing/start/complete/
  gateway join·fork 共 7 把 401）、`gateway_fork_forbidden_for_non_admin`（403）。
  `-- --ignored --nocapture security_gate_tests`：**3 passed**——实测非负责人退回 403、负责人放行且
  `x_record.creator` 落负责人本人、非发起人撤销 403 且 work 状态**未变**（拒绝路径不留半写）、
  发起人撤销后 `work_status=retracted` 且其 in-flight task 连带 `cancelled`、发起人启动后
  `work_status=processing` 且 Start Event 任务确实创建。
  `cargo test --lib -p parity processplatform_service_processing`：**184 passed / 0 failed**。
- **本轮新增取证纪律**：`#[ignore]` 的 DB 臂内部仍带 `is_db_available()` 显式 skip 分支，
  所以「1 passed」并不等于「真跑了」——必须 `-- --ignored --nocapture` 复跑并确认输出中**没有**
  `skip:` 才能把该臂记成实测。上一条 portal_core_entity 的 F2 读数当时只跑到 `-- --ignored`，
  严格说那是「通过」而非「已证」；本轮三处 processing/core_express 读数均已按此口径补证。
- **尺子复跑（本批收口后）**：`node scripts/audit_write_gating.mjs` 会重新扫描 handler 体，
  新加的 `session` 参数与门禁调用会被识别，因此游离名单是**活读数**而非静态快照。
  **函数级**矩阵 991 行，分桶 7 B-匿名可达且无体内门禁 / 4 G-Public 早退吞掉 admin 兜底且无体内门禁
  / 1 H-Public 早退但体内有门禁（残余待取证 11 行 = B+G，写 `write_gating_residual.tsv`）；
  **路由级**写路由 1689 条，本轮 R4 由 13 → 11、R7 由 1131 → 1095（共 27 行出列）。
  离场的两条 R4 是 `meeting_assemble_control`
  `POST /api/meeting/assemble/control/meeting[/{id}] create_meeting`（A3-N3 后 creator 取会话权威）。
  R4(11) + R7(1095) 明细已写 `docs/audits/artifacts/write_routes_ungated.tsv`。
  **口径提醒**：同一把 `create_meeting` 在函数级仍计 B、在路由级已出列——两把尺子的门禁信号源不同，
  引用读数时必须写明是哪一级。

第四批（§三.4，R4-2 批 1 + 批 2a）读数——同口径，DB 臂一律 `--ignored --nocapture` 真跑：

- `cargo test --lib -p processplatform_service_processing`：**63 passed / 0 failed / 10 ignored**。
  ignored 名单逐只点名可对账（`-- --ignored --list`）：9 只 `security_gate_tests` DB 臂
  （`work_start_requires_work_creator`、`task_reject_requires_task_assignee_and_pins_real_actor`、
  `work_retract_requires_work_creator`、`work_delete_requires_work_creator`、
  `task_delete_requires_task_person`、`record_delete_requires_record_creator`、
  `work_edit_with_empty_creator_requires_admin`、`dict_edit_allowed_for_admin_and_lands`、
  `att_copy_requires_target_work_creator`）+ 1 只既有
  `tests_generated::tests::test_gateway_join`。即 4（三批收口时）→ 9 → 10，增行全部来自本批新增，
  没有来路不明的空过臂。
- 五支离线拒绝臂按名各跑一次 ok：`u2_entity_writes_reject_without_session`（批 1 的 16 把 401）、
  `u2_admin_writes_forbidden_for_non_admin`（字典 / job / 快照 403）、
  `u2_read_copy_and_create_writes_reject_without_session`（批 2a 的 13 把 401）、
  以及三批既有的 `engine_runtime_writes_reject_without_session` /
  `instance_progression_writes_reject_without_session`。
- `-- --ignored --nocapture security_gate_tests`：**9 passed / 0 failed，0.09s，输出中无 `skip:` 行**
  ——按上一条纪律这 9 只记为实测。关键行为差：闯入者 403 且 `x_work` 行仍在、`deleted_at` 仍 NULL、
  目标 work 未收到复制的附件；属主 / 真 admin 放行且写落库；空 creator 的 work 非 admin 一律 403。
- 6 支既有 DB 用例按新口径改写后按名复核各 1 passed：`u2_work_v3_retract_cancels_active_tasks_transactionally`、
  `u2_dict_path_set_then_delete_roundtrip`、`u2_attachment_copy_dedups_by_name`、
  `u2_attachment_delete_with_wrong_work_is_idor_rejected`、`u2_read_processing_moves_row_into_completed_exactly_once`、
  `u2_documentversion_…`（两支）、`u2_work_create_via_process_path_bootstraps_instance`。
- `cargo test --lib -p parity processplatform_service_processing`：**184 passed / 0 failed**（加门未破路由存在性对账）。
- **尺子复跑（第四批收口后）**：函数级矩阵仍 991 行、残余待取证仍 11 行（本批 29 把原在
  「已认证+无门禁」桶，不在 B/G/H 残余内）；路由级 **R4 仍 11 条**（同上，u2 这批不是匿名格）、
  **R7 由 1095 → 1071，出列 24 行**，且 24 行逐行均属 `processplatform_service_processing`
  （`git diff` 明细核对）。这 24 = 批 1 的 12 条 + 批 2a 的 12 条写路由。
  **口径差新证据**：批 2a 的 `work_processing_signal` 注册在 **GET**
  `/…/work/{id}/series/{series}/activitytoken/{activityToken}/processing/signal` 上，路由级尺子只扫
  POST/PUT/PATCH/DELETE，所以它永远不会出现在 R7 名单里——这正是 §五「GET 写禁令」守卫要补的盲区。
  同样地，`work_edit`（PUT）/ `work_delete`（DELETE）/ `att_edit_id`（PUT）三把不在出列名单里——
  原因见下一条「尺子缺陷 1」：它们与 `work_get` 链式挂在同一条 `.route()` 上，旧解析器只取首方法，
  这三把**从未进过路由级分母**（既不是 R6 也不是 R7）。**R4(11) + R7(1071)** 明细已重写
  `docs/audits/artifacts/write_routes_ungated.tsv`。

第四批收口时自查出**尺子本身的两个计量缺陷**（比单点漏洞更要紧：它让账面进度好看）：

- **缺陷 1：链式方法只取首个**。`ROUTE_RE` 抓到 `.route("p", …)` 的体内，只用 `HANDLER_RE` 取
  第一个方法-handler 对，于是 `.route("…/work/{id}", get(work_get).put(work_edit).delete(work_delete))`
  只计 GET 一条，**两把写路由从未进过任何分母**；`attachment/{id}` 的 `.put(att_edit_id)`、
  `record/{id}` 的 `.delete(record_delete)`、`applicationdict/{id}/{p0}/data` 的
  `.put/.post/.delete` 宏族 24 条同被吞掉。解析器级取证：只改这一处，
  `processplatform_service_processing/routes.rs` 的路由条目 **184 → 207**。
- **缺陷 2：路由级只扫 POST/PUT/PATCH/DELETE**。这与 `csrf_middleware` / `requires_admin`
  的覆盖面一致，也就与它们的**盲区**一致——GET 上挂的状态写（A3-N12 / `mockdeletetoget` 族）
  对尺子和对两把中间件同时隐形。新增第 5 段单独枚举「非写方法路由 + handler 体内含写 SQL + 体内无门禁」。
- **修好之后的活读数（口径断裂，与上文任何数字都不可直接比）**：写路由 1689 → **1923**，
  全路由 3844 → **4202**；R1 102 / R2 4 / R3 3 / **R4 16** / R5 317 / R6 308 / **R7 1173**。
  R4 新露出的 5 条也全部是链式吞掉的：`auth POST /api/authentication/bind/meta/{meta}`、
  `auth POST /api/authentication/captcha/width/{width}/height/{height}`、
  `personal POST|PUT /api/reset/check/credential/{credential}`（check_password / reset_password）、
  `personal POST /api/reset/password/anonymous register`。
  **但这 5 条不等于 5 个漏洞**，逐条取证：`bind_confirm` 其实是 gated 的——它不走 axum 的
  `Extension<Session>` 而是手工 `extract_token_from_headers` + `session_manager.validate_session`
  （`auth/src/bind.rs:134-146`），尺子的 `SESSION_RE` / `GATE_RE` 词表没有这一路，于是把它误计成
  「匿名可达且体内无门禁」；这是**缺陷 3：门禁信号词表不全**（与 §二.7 的注册表早退同性质，
  都是尺子口径问题而非代码问题）。其余 4 条落在登录 / 验证码 / 匿名改密这一族**设计上就必须匿名**
  的引导入口，需逐格定性，不能一把收成 admin。
  函数级矩阵仍 991 行 / 残余 11 行（那一层本就逐函数扫描，不受链式影响）。
- **GET 写新面（首次可见）**：体内无门禁 **173 条** = K1 匿名可达 2 + K2 读会话 22 + K3 已认证无会话 149；
  按 crate 集中度高的是 `processplatform_assemble_surface` 60、`processplatform_service_processing` 24、
  `program_center` 23、`cms_assemble_control` 15。明细写
  `docs/audits/artifacts/write_routes_get_writes.tsv`（新增产物）。
- **由缺陷 1/2 直接长出的一格真漏洞（记 §二.1 R4-7）**：
  `processplatform_assemble_surface` 的
  `GET /api/processplatform/assemble/surface/application/list/complex/manage/{person}`
  （`lib.rs:697 application_list_complex_manage_person`）**不取会话**，且落在
  `AUTH_EXEMPT_PREFIXES`（`shared/src/middleware/constants.rs:434-435` 的
  `/api/processplatform/assemble/surface/application/list`）里——**真匿名可达**，
  函数体第一条 SQL 就是 `UPDATE PP_E_APPLICATION SET "xupdateTime" = NOW() WHERE xid = $1`，
  随后回读并返回该行的 createTime/updateTime。即匿名按 ID 改任意应用元数据时间戳（审计字段伪造）
  + 任意应用的存在性与时间信息泄漏。修法按 §三.4 口径：`Option<Extension<Session>>` 无会话 401，
  再收 crate-local `require_admin_opt`（设计态/运维面 touch，比照字典 / job / 快照收 admin）。

第五批（§三.5）读数：

- `cargo test --lib -p processplatform_assemble_surface`：**604 passed / 0 failed / 0 ignored**。
  两支新臂按名复核各 1 passed（`r47_manage_touch_rejects_without_session` 0.03s、
  `r47_manage_touch_forbidden_for_non_admin` 0.07s，均为离线断言）。
  `cargo test --lib -p parity processplatform_assemble_surface`：**945 passed / 0 failed**。
- **尺子复跑（缺陷 3 修正后）**：函数级矩阵仍 991 行 / 残余 11 行；路由级 R1 102 / R2 6 / R3 3 /
  **R4 14** / R5 369 / R6 258 / **R7 1171**；GET 写无门禁 **165** = K1 匿名 **1** + K2 读会话 16 + K3 无会话 148。
  与上一条（第四批）对比：R4 16 → 14（`auth bind_confirm`、`auth refresh` 原被误计，
  它们走 `extract_token_from_headers` + `validate_session` 的手工会话路）、
  R5 317 → 369 / R6 308 → 258（processing 本批已加门的 handler 从 R6 正确落到 R5）、
  GET 写 173 → 165。**K1 现只剩 1 条**：`program_init GET /api/secret/set/cancel set_cancel`——
  这是 §三.3 为不打断首次引导而**有意保留匿名**的那格，属待定性而非待修残留。
- 取证纪律的第三次自证：**「尺子读数出列」不等于「代码修好了」**。本批 surface 那格代码已修、
  测试已绿，读数却仍挂着 K1——不是修复无效，而是尺子的门禁词表认不出新加的 `require_admin_opt`。
  反向同理：词表补全后 R4/R7 会同时把「真漏洞」与「只是没被认出」两类混在一起变化，
  所以每次改词表都要按上面的桶间迁移逐条对账，不能只看总数下降。

第六批（§三.6）读数：

- `cargo test --lib -p processplatform_service_processing`：**64 passed / 0 failed / 11 ignored**
  （63→64、10→11，新增的两支都在册）。离线臂 `touch_writes_reject_without_session` 按名 1 passed；
  DB 臂 `-- --ignored --nocapture touch_requires_work_creator` → **1 passed / 0.07s，输出无 `skip:` 行**。
- `cargo test --lib -p processplatform_assemble_surface`：**605 passed / 0 failed**（604→605），
  `surface_touch_writes_reject_without_session` 按名 1 passed。
- parity：`cargo test --lib -p parity processplatform_assemble_surface` **945 passed / 0 failed**；
  `… parity processplatform_service_processing` **184 passed / 0 failed**——加门未破路由存在性对账。
- 尺子：`service_work_id_touch*` 在 `write_routes_ungated.tsv` 的命中数 **6 → 0**（含 surface 四条
  别名/正式路由）。路由级 R4 仍 14、R7 1165、GET 写无门禁 162；函数级 991 行 / 残余 11 行不变。

第七批（§三.7）读数：

- `cargo test --test exempt_authorize_parity -- --ignored --nocapture`：**12 臂全活 / 红名单 0 条**，
  输出无 `skip:`。逐臂：`invoice_create`、`invoice_upload` 200（修前为带合法会话仍 401 的死态）；
  `meeting_create_alias` / `meeting_create_root` 400 `title is required`（**到了 handler 的业务校验**）；
  `meeting_get_by_id` / `queryview_view_by_id` 200 + `not found`；`queryview_list` /
  `surface_task_list_work` / `surface_attachment_list_work` 200 空列表；
  `captcha_verify`、`person_regist`（匿名臂）422——422 是 handler 侧抽取/校验，不是 401，
  即「双锁已开」。同臂打印 `creator 权威臂：invoice_upload 落库 creator=Some("admin")（会话人=admin）`，
  证明请求体里伪造的 `r48-spoofed-actor` 没有落库。
- `cargo test --test anonymous_write_guard -- --ignored --nocapture`：**1 passed / 0 failed**，无 `skip:`。
  `invoice/upload`、`invoice/create`、`meeting/create`、`PUT /api/person/custom/*`、
  `POST /api/person/icon/upload`、`PUT /api/person/icon`、`POST /api/secret/set` 全 401 且零行零文件；
  `POST /api/reset/password/anonymous` 设计上匿名可达、由验证码挡下（`验证码不存在`）；
  **`GET /api/secret/set/cancel` 修后 401**。三层仪器自证都在：行数尺 1→0、头像目录 0→1→0、
  放行正控（admin 会话打 `invoice/create` 200 且 creator=admin）——没有正控，前面所有
  「匿名 401 + 零行」都可能只是「这条路由本来就写不进测试库」的假绿。
- **红绿对账（F4）**：临时把 `SECRET_INIT_PATHS` 改回两条再跑同一支测试 →
  `GET /api/secret/set/cancel` 读数变 **HTTP 200**、body 是 handler 的
  `cannot cancel secret when system is already initialized with users`，断言按名喊红
  （「必须与 set 同锁，实测 200 OK」），退出码 1。这条红同时钉住两件事：**该路径修前确实永久匿名可达**，
  且 handler 体内 `has_person` 是当时唯一一道锁。复原常量后复跑转绿。
  同一批把「密钥行存在与否」做成可读判据：测试库通常没有 `init-secret` 行（逐字比较会退化成
  `None == None` 恒真），故本臂插入哨兵行 `r48-cancel-sentinel` 让「被删」读得到，打完删掉复原；
  红跑那次因 panic 跳过收尾、把哨兵留下 1 行，下一轮的**预清扫**把它收掉并打印来源，
  使这一臂对自己中途退出产生的残留也是自愈的。
- **尺子**（`node scripts/audit_write_gating.mjs`，退出码 0）：注册表解析自证「入表 4806（未解析 3）」；
  函数级 991 行 / 残余 11 行（本批未动）；W 碰撞 6 条（W2 3 / W3 3）；真匿名面 **172**
  （A1 真匿名+写 3、A1 条件匿名+写 1、A2 13、A3 只读 153 + 条件 1）；豁免 ∧ ¬Public 差集 **17**
  （D1 惰性豁免+写 SQL 7、D2 惰性豁免+只读 10），桶名带「F1 接线自证：已接线」字样。
  口径链：修缺陷 9 前 6c 报 57（假阳）、修后 19；F2 让 `regist` / `captcha_verify` 离开差集 → 17；
  F4 让 `set/cancel` 从 A1 真匿名写迁入 A1 条件匿名 → A1 真匿名写 4→3。
- **A1 点名门禁的注入臂**：把副本脚本名单里的 `/api/person/regist` 改成 `/api/person/registX` 后跑，
  尺子**两个方向同时红**（「未登记的新匿名写面：personal POST /api/person/regist」+
  「名单里已消失的条目：…registX」）并 `EXIT=1`；副本随后删除。证明这条门禁不是恒真断言。
- **回归波及面**：`cargo test -p shared --lib` → **151 passed / 0 failed / 11 ignored**（F1 改的是 auth 层语义，
  这一族中间件测试全绿；其中 2 支限流测试是既有 `#[ignore]`「redis mock not configured」，非本批新增静默跳过）。
- **必须点名的一次「不绿但不该记在我批」**：`cargo test -p general_assemble_control -- --ignored` →
  **33 passed / 2 failed**，红的是 `test_status_route_accessible` 与 `test_permissions_route_accessible`
  （实际 200、断言 500）。归因证据：这两条路由的 handler 与 `src/tests.rs` 都不在我的 diff 里
  （`git diff` 对这两处为空 ⇒ 与 HEAD 逐字相同），故与本批的 invoice 三把改动无关。
  真因是**一支环境耦合的断言族**：全库 272 处 `assert_eq!(response.status(), INTERNAL_SERVER_ERROR)`
  分布在 58 个 crate 的 `tests.rs`，它们把「测试库连不上 ⇒ handler 取不到 pool ⇒ 500」当成了路由可达性判据，
  一旦本机 PostgreSQL 真的在跑就整体转红。这是 §四 早已记过的「测试在册 ≠ 测试在跑」的另一面：
  **这些测试只在库坏掉时才跑绿**。本批不吞掉这 2 条红、也不去改那 272 处（改法是断言「非 404」= 路由匹配，
  属独立一整格工作），列进 §五 待修。
- **共享工作树的编译被打断两次**（均由并行会话的在制品造成，本会话只做**最小语法复原**、不动其语义）：
  `processplatform_service_processing/src/lib.rs` 的 `row.as_ref().and_then(...)`（改为 `row.get(...)`）、
  `portal_assemble_designer/src/lib.rs` 文档注释第二行丢 `///` 前缀（补回）。
  两处都留在并行会话自己的未提交改动集里，**没有**被本批 commit 收走。

本轮两处自伤（记下来防止再犯）：

- **管道假绿**：`cargo test ... | tail -40` 的退出码是 `tail` 的，两处「exit 0」实际是编译红
  （meeting 模块重复声明、surface 既有 DB 测试旧元数）。此后一律 grep 出 `^error` 与
  `test result` 行，或直接 `; echo EXIT=${PIPESTATUS[0]}`。
- **按名复核不可省**：`cargo test -p meeting ... security_gate` 一度筛出 **0 命中 / 82 filtered out**，
  暴露我在消除重复 `mod security_gate_tests;` 时把原声明一起删了——整族门禁测试静默不参与编译。
  补回声明后同一过滤命中 3 支。**测试「存在」不等于测试「在跑」**，收口必须按名过滤复核一次。
- **Write 覆盖既有文件**：`meeting_assemble_control/src/security_gate_tests.rs` 本已存在（二轮
  2 支 config 门测试），我的 Write 直接盖掉，随后 `git checkout HEAD --` 复原并把新测试追加进
  同一 mod（`git diff --stat` 复核为 +69/-0）。规矩：**已存在的文件先 Read 再 Edit**。



## 五、处置与遗留（下一轮入口）

- 已修并验证：N1、N4-01、N2、preview 加固、F3、F6、F9（离线 403/会话权威断言）、F2（编译与既有测试绿，真门 `#[ignore]` 待 PG）、**A2-F12、A3-N1、A3-N2、A3-N3**（第二批读数见 §四）、**A3-N6（program_init 7 把写收 admin + 匿名 init 判定改 fail-closed）、R4-6、R4-1 全 13 把**（第三批：§三.3 改法、§四 读数，其中 processing/core_express/program_init 的 DB 臂已按 `--ignored --nocapture` 真跑取证）、**R4-2 批 1 + 批 2a 共 29 把**（第四批：§三.4 / §四第四批）、**R4-7**（第五批：§三.5 / §四第五批，含尺子两处计量缺陷 + 门禁词表缺陷的自修）、**R4-2 批 3：processing 两把 touch + surface 四把 touch 路由**（第六批：§三.6 / §四第六批，DB 臂已 `--ignored --nocapture` 真跑取到「闯入者 403 且零留痕、发起人放行且 creator 落本人」的行为差）。
- 待修（有明确落点与修法，按 §二.1 与 §二.6 顺序收口；processplatform 族因 `ADMIN_WRITE_DEPRECATED_PREFIXES` 已撤销中间件兜底，handler 层是唯一门禁，优先级最高；F5 与 §二.1 的 R4-3/R4-5 同为「ownerless 系统数据写 = admin」一族，可一把收）：**R4-2（批 1+2a+批 3 已收 29 + 2 把；`processplatform_service_processing` 路由级现余 81 条 /
  68 个 handler 名，含 `dict_set_1..8` / `dict_del_1..8` 宏族 16 条、`data_*` 内容族、
  `attachment_copy_*` 与 `attachment_edit_id_text` 等别名/mock 族——逐条明细读
  `write_routes_ungated.tsv`，**不要**再引用「47 处」这个函数级旧数）/ R4-3 / R4-4 / R4-5**、F1 / F4 / F5 / F7 / F8 / F10、N4-02、N4-03。
- **修尺子后新开的两格**：①**GET 写族有了活读数**——`write_routes_get_writes.tsv` 首次量出
  **173 条**「非写方法路由 + 体内写 SQL + 体内无门禁」（K1 匿名 2 / K2 读会话 22 / K3 无会话 149），
  集中度前三 `processplatform_assemble_surface` 60、`processplatform_service_processing` 24、
  `program_center` 23；这正是 §五「GET 写禁令」守卫的靶面，守卫现在可以直接建在这段尺子上。
  ②**尺子缺陷 3：门禁信号词表不全**——`extract_token_from_headers` + `validate_session` 这一路
  手工会话校验不被 `SESSION_RE`/`GATE_RE` 识别，`auth bind_confirm` 被误计成匿名无门禁；
  补词表前先别把 R4 名单当漏洞名单用。
- **第五批收口后的刷新（覆盖上一条的两个数字）**：①已修——R4-7 那格（surface 匿名 GET 写）收
  会话 + admin，两支离线臂 + parity 945 绿（§三.5 / §四第五批）；②已修——`GATE_RE` 改按完整标识符
  匹配。**GET 写活读数现为 165 条**（K1 匿名仅剩 `program_init GET /api/secret/set/cancel` 一格，
  属 §三.3 有意保留匿名的引导族、待产品定性；K2 读会话 16 / K3 无会话 148），路由级现读
  R4 14 / R7 1171。**下一格按 K3 的 148 条按 crate 递减收**，首选
  `processplatform_assemble_surface`——它与 R4-4 的 `*_mockdeletetoget` 物理删族高度重叠，
  一把收完可同时出两格。
- **第二批新增待修（按危害排序，均有 file:line 证据，明细见 §二.3 / §二.4 / §二.7）**：
  1. ~~`program_init` 写族~~（A3-N6 + §二.7 的匿名 B 格）——**本批已收 7 把写 + init 判定 fail-closed**（§三.3）。
     `/api/secret/set` / `/api/secret/set/cancel` 按首次引导流程**保留匿名**（收 admin 会打断初始化），
     改为只堵「探测出错即当作未初始化放行」那一格。**残留未收**：`external_datasources_validate`
     仍用裸 `TcpStream` 探任意 host:port，未经 `shared::netguard`（SSRF 面），`restore_upload`
     的落盘目标路径与 200 MB 上限也还需复核。
  2. `shared/src/crud.rs` 的 `CrudSpec` 通用写无 owner 谓词（A3-N9，波及 11 crate，含他人 `x_person_face` 生物特征行的增改软删）。
  3. `query_assemble_*` 写族（A2-F1/F2/F3）：IDOR 覆写删除设计对象、`table/row/delete/all/{tableFlag}` 物理清空任意表、DDL 执行权错位。
  4. `cms_assemble_control` 遗留 CRUD 与 `mock*` 写族 + `update_control_config`（A2-F4/F5/F6，含 GET 注册与 `allow_anonymous` 自翻）。
  5. 模块级 `update_control_config` 家族（A3-N11：bbs/calendar/jpush/mind）、`jpush`（A3-N5）、`attendance`（A3-N4）、`mind`（A3-N7）、`ai_assemble_control`（A3-N8）、`file`（A3-N10）。
  6. 归属冒名残留：A2-F11（`creator` 硬编码 "system" 9 处）、A3-N13（invoice 三把）、A2-F13 / F14。
- 建议再加三把守卫（本轮取证出的是**机制性缺口**，不是单点漏洞）：
  - **GET 写禁令**：`csrf_middleware` 只覆盖 POST/PUT/PATCH/DELETE（`token.rs:70-74`），`requires_admin` 同样只认这三个方法（`security.rs:129`）；凡 `.route("...", get(handler))` 且 handler 体内含 `INSERT/UPDATE/DELETE` 即应红。已知落点：`hotpic routes.rs:53`、`ai routes.rs:47`、`message routes.rs:19`、meeting accept/confirm/checkin、calendar 与 jpush 的 `update/control/config`、attendance 两处、`processplatform_core_express`。撤 GET 需同批改前端（`MeetingApp.vue` 自检面板仍在用 GET）。
  - **Public 兜底吞掉点名**：`ADMIN_WRITE_PREFIXES` ∩ 注册表 Public 的 15 条交集必须逐条点名，落在此集合内新增的写 handler 必须有体内归属/角色门（§二.7）。
  - **测试在册 ≠ 测试在跑**：6 个 crate 的 `src/tests.rs` 是游离文件——文件在，`lib.rs` 没有
    `mod tests;`，里面的断言一条都不参与编译（`console` / `ldap` / `openapi` / `orm` /
    `processplatform_assemble_surface` / `processplatform_core_express`）。core_express 那份尤其刺眼：
    `test_work_terminate_returns_success` 断言「无会话 GET 也返回 200」，钉住的正是 R4-6 漏洞本身
    （本批已把该断言改写为 401，但文件仍未注册，注册时才真正生效）。注册需两处返工：
    嵌套 `mod tests { use super::* }` 的 `super` 指向文件模块本身（须改 `use crate::*`），
    以及 DB 依赖臂补 `is_db_available()` 显式 skip。守卫写法：凡 `crates/*/src/tests*.rs` 存在，
    其模块路径必须在同 crate 的 `lib.rs` 有对应 `mod` 声明。
- 产品口径待拍：`meeting_id_accept` 的会议级状态翻转谁能做（被邀请人 / 属主 / admin）——本批只加了有效会话要求，未加归属判定。
- 建议统一修法（沿用本仓既有先例，避免新抽象）：引擎运行面按一轮 V2/V4 口径至少加 `require_session_opt`；改变归属/状态/删他人行的走 `shared::middleware::require_owner`（admin 旁路、行不存在放行以保 not-found 语义）；设计态（script/flow/form/xform/dict/item-access/portal）比照同 crate `guard_write` 或二轮 S5 先例加 admin/属主门；GET 状态变更改 POST。每 crate 补 `security_gate_tests`（无 DB 的 401/403 各一）。
- 提交落点：`a14b84348`（返工三处「已修未编译」+ docx/zip 上限 + 接线尺）、`be07ecac8`（本文档）、`e4d6af0f6`（F2/F3/F6/F9 越权门）、`4931b351f`（第二批 A2-F12 / A3-N1 / N2 / N3）、`055dbb6d5`（尺子复刻注册表 Public 早退、G/H 两桶）、`95af154ad`（第二批账本）、`52ad4944e`（A3-N6 program_init 写族 + init 判定 fail-closed）、`494a93e40`（R4-6 work_terminate）、`c32bcbb64`（R4-1 首批 7 把）、`47d01df8d`（R4-1 次批 7 把 + 旧断言返工）、`c452d634c`（三批收口账本 + 尺子读数口径）、`2728b2c07`（R4-2 批 1：u2 实体写 16 把）、`36f613a98`（R4-2 批 2a：阅读回执 / 附件复制 / 建单与版本族 13 把）。第四批账本 + 尺子两处计量缺陷修复（链式方法枚举 `5dd1957bc`、GET 写枚举同批）+ 三张产物 TSV 同批提交；`f5d758d75`（R4-7 匿名 GET 写收口 + 尺子门禁词表缺陷 3 + 产物刷新）、`11f9f8947`（第五批账本）、`f97dda1f0`（R4-2 批 3：processing 两把 touch 经 core 收会话与归属 + surface 四把 touch 路由 + `GATE_RE` 改前缀匹配 + 产物刷新）。
- **一处读数矛盾已当场定性（并暴露尺子缺陷 4）**：`service_work_id_touch` /
  `service_work_id_touch_p2` 在 R7 名单里，而一轮记录写「touch 维护作业 handler 要求登录会话」。
  回源码复核（`lib.rs:2598 / 2606 / 2574`）：路由挂的是**薄封装**，本体只有一行
  `service_work_id_touch_core(pool, id).await`，而 core 既不取 `Session` 也无门禁，函数体直接
  `SELECT … FROM x_work` + `INSERT`。所以 **R7 这条是真阳性**（一轮那句说的是另外 8 把 touch），
  同时暴露尺子的第 4 个口径局限：**`fnBody` 只看路由层 handler 的前 200 行 / 到首个 `\n}` 为止，
  不跟进「薄封装 → core」的委托**——委托型门禁会一律被判成无门禁（假红方向，偏保守可用），
  而反方向（core 有门、wrapper 无门）也确实是真实缺陷，因为门根本不会被执行。引用某条 R7 前，
  凡是 body 只有一行 `.await` 转发的 handler，都要人工看一眼 core。分支 `oa4rust-vuln-audit-r4`，本轮未 push。

### 第七批收口后的刷新（§三.7 / §四 第七批）

- **已修并验证**：F1 双锁一致性（`token.rs`）、F2 登录前置流程显式 Public（`rbac.rs`）、
  F3 公开即限流（`constants.rs`）、**F4 初始化密钥销毁侧同锁**（`SECRET_INIT_PATHS` 补
  `/api/secret/set/cancel`，红绿两跑取证）、F5 invoice upload 三把 creator 会话权威。
  顺带把 §五 旧名单里的 **A3-N13（invoice 三把冒名）收掉**——它此前被记成「creator 取请求体」的
  归属缺陷，但真因是这三把当时谁都打不通，修归属必须与修可达性同批，否则会新开冒名面。
- **新待修（本批取证出来的两类，都不是单点漏洞）**：
  1. **17 条惰性豁免条目**（`dead_exempt_routes.tsv`，7 写 + 10 读）：豁免表里有、注册表不 Public，
     F1 之后不再致死人路，但「豁免」本身失效。**这是引信不是噪音**——哪天有人把这些路径或其前缀
     标成 `Public`，条目立刻变成匿名可达写面。收法二选一：从 `AUTH_EXEMPT_PATHS` 删掉这些条目，
     或在注册表点名它们该是什么级别；建议一把守卫把「豁免表条目必须被注册表识别」钉成集合关系。
  2. **272 处 `assert_eq!(status, 500)` 断言族（58 个 crate）**：判据写的是「测试库连不上时 handler
     取不到 pool」，所以**只在数据库坏掉时才绿**；本机 PG 在跑时 `general_assemble_control` 稳定红 2 条
     （见 §四 归因证据：那两条路由与 `tests.rs` 都不在我的 diff 里）。改法是把这些「route_accessible」
     类断言收成「非 404 = 路由匹配」，属独立一整格，需要按 crate 分批跑绿再收。
- **新发现但未修的策略不一致**：`MAX_FILE_SIZE = 5MB` 与 axum 默认体限 2MB 并存——上传面在 2MB 就被
  框架拒掉，5MB 那条上限是死代码。fail-closed（实际更严），所以不是漏洞；要么把默认体限调到 5MB，
  要么把常量降到与框架一致，属产品口径。
- **待拍产品口径（本批择了保守侧，可回退）**：`/api/queryview/*` 6 条与 3 条 processplatform surface 读
  在豁免表里按「公开」写、在注册表判 `Authenticated`，F1 之后统一变成**仅登录可达**。
  若这些只读列表本意允许匿名（门户/看板嵌入场景），需要产品明确后才把对应路径标 `Public`——
  注意标 `Public` 会连带打开同前缀下现被死锁掩盖的写面，必须按 A1 点名门禁逐条过。
  另：首次引导窗口内 `secret` 族（`check` / `set` / `set/cancel`）匿名是设计使然，本批只把三者锁到
  同一窗口；「公网部署时首次引导要不要网络侧声明」仍归部署口径。
- **提交落点**：`7a95789d3`（§三.7 全部代码 + 尺子 + 两把守卫 + 7 张产物 TSV）。
  本文档（§三.7 / §四 第七批 / 本节）为账本随后一笔。共享工作树上并行会话的在制品我只做了两处
  最小语法复原（`processplatform_service_processing`、`portal_assemble_designer`），**未**提交进我的批，
  仍留在他们自己的未提交改动集里。本轮未 push。

### 第七批补充（尺子缺陷 10 + 一次「误判成解析回归」的取证，代码 `81ac87eb4`）

- **缺陷 10**：R2/R3/R4 的匿名判据仍只看第一把锁（`isAuthExempt`），与 F1 之后的 auth 层语义脱节——
  只命中豁免表的写路由其实**要求有效会话**。改为双锁判据后用注入臂量了迁移：
  **R2 6→3、R3 4→0、R5 518→521、R6 276→280，而 R4 与 R7 一字不动**。
  这反过来定性了一件事：长期被当成「匿名可写行动面」的 **R4 那 14 条本来就两把锁同开**（真匿名），
  此前 §三.7 说「旧读数虚高行动面」只对 R2/R3 那 7 条成立，对 R4 不成立——账本口径据此更正，
  不要把「双锁修复」也写成 R4 的缩水理由。
- **一次自我纠偏（值得留档的取证纪律）**：拿 `write_routes_ungated.tsv` 做新旧对账时得到
  「93 出 / 70 条 `(crate,method,path)` 整行消失」，第一反应是**解析回归**。实际原因是该产物
  **只写 R4 与 R7 两桶**，那 70 条是 R7→R5 搬家（第六批 `GATE_RE` 改前缀匹配后认出它们体内确有门），
  抽查 `cms_assemble_control::dict_create` 证实体内 `u2_require_admin(&pool, &session)`。
  修法不是改口径而是**补产物**：新增 `write_routes_matrix.tsv`（全 7 桶逐条 + 按 crate 的 R7 集中度），
  从此「换桶」与「漏解析」两种迁移可机械区分。**教训**：对账前先确认产物的列/桶覆盖面对不对得上
  被比较的判据，否则一把只量两个桶的尺子会把「搬家」读成「失踪」。
- **按新分母复称的 R4-2 余量**（读 `write_routes_matrix.tsv`，R7 = 已认证 + 体内无会话无门禁）：
  `processplatform_service_processing` **81 条路由 / 68 个 handler 名**——与 §五 旧数**完全一致**，
  即双锁改造与缺陷 9/10 的修正没有改变这一族的分母，旧数可继续引用（此前挂着「要重量」的待办就此收掉）。
  同表读出的更大面是 **`processplatform_assemble_surface` 356 条 / 275 名**（全仓第一）、
  `cms_assemble_control` 164 / 152、`query_assemble_designer` 69、`query_assemble_surface` 58、
  `attendance_assemble_control` 56、`program_center` 55。下一轮的选题按这张表排，不再按 crate 名字顺序猜。
- 提交落点补充：`81ac87eb4`（尺子缺陷 10 + 全 7 桶矩阵产物 + 本补充段的前两条读数）。

