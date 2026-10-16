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

### 三.8 第八批（元数据 / 设计器 / 身份关联 / 评审链四族 + 尺子缺陷 11、12，代码）

本批按「**creator 恒写 `system` 的即系统级数据 → admin 档**」这条既有口径，把四个 crate 的
「登录即可写」元数据面收成 admin；同时被两把尺子缺陷挡在视野外的东西第一次出列（见下）。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| cms 元数据写 17 把 | `crates/cms_assemble_control/src/lib.rs` | `update_control_config`、`permission_management_refresh_all`、`dict`/`form`/`xform`/`view`/`templateform` 各族 `create`/`save`/`delete`（共 15 把）加 `Extension<Session>` + `u2_require_admin(&pool, &session).await?`。这族写的是「表单/视图/字典的结构定义」，不是某人的业务单据——改一条 `form` 定义会作用于该应用全部在跑的 work，任一登录用户即可 |
| portal 设计器写 26 把 | `crates/portal_assemble_designer/src/lib.rs` | 页面/门户设计器全 26 把写走 `crate::require_admin`（lib.rs:472）。判据同上：这些 handler 的 `creator` 一律硬编码 `"system"`，按 §二.3 定的分类法属「无属主的系统级数据」 |
| correlation 身份关联写 10 把 | `crates/correlation_service_processing/src/lib.rs`、`src/u2.rs` | 该 crate 原先没有 admin 助手，新增 `pub(crate) async fn require_admin(pool, session)`（走 `shared::middleware::is_admin`，DB 不可达即 fail-closed 403）；`lib.rs` 4 把 + `u2.rs` 6 把（`create_pp`/`create_cms`/`update_pp`/`update_cms`/`delete_pp`/`delete_cms`）全部先门后 SQL。`x_correlation` 是**身份关联表**——谁都能插一条「A 与 B 等价」，等于把别人的身份映射改掉 |
| pp 评审链活路由 3 把 | `crates/processplatform_service_processing/src/u2.rs` | `review_create_work`、`review_create_workcompleted`、`review_init_for_view` 走 `require_admin_opt`（无会话 401、非 admin 403）。评审链是「谁给谁打分」的记录面，属系统级数据 |
| 同名的**死 handler** | `crates/processplatform_service_processing/src/lib.rs` | `review_init_review` 与上面三把同族，但 `routes.rs` 注册的是 `u2::review_init_for_view`——lib.rs 那把根本不可达。本批给它也补了门（防御性，将来有人注册不至于裸奔），并在账里记死「**handler 存在 ≠ 路由可达**」：只看 handler 会以为修完，实测读数纹丝不动，正是这条把尺子缺陷 11 顶出来的 |
| 生产缺陷：`r#"…"#` 里混进字面反斜杠 | `crates/correlation_service_processing/src/lib.rs:243` | `create_correlation` 的 INSERT 语句里有孤立的 `\`，raw string 不做转义 ⇒ 发给 PostgreSQL 的 SQL 文本自带反斜杠，写路径**必 500**。这条不是加门加出来的，是一直在：本批加门后 DB 取证的 admin 臂（真放行那一路）才第一次把它撞红。修法=删掉那个 `\`。全仓用 `/d/tmp/rawsql.mjs` 扫过同类模式，仅此 1 处、现已归零 |
| 尺子缺陷 11（限定符被剥掉） | `scripts/audit_write_gating.mjs` | 见 §四 第八批：注册写的是 `post(u2::review_create_work)`，入表时 `.split("::").pop()` 把 `u2::` 丢了，取体内信号又按「遍历 crate 命中的**第一个**同名 fn」⇒ pp 的 `lib.rs` 里那把同名死 handler 先命中，已修的两条路由继续报 R7。**双向错**：也能把真漏洞报成已修。修法：保留 `handlerRef`，带限定符的按 Rust 模块路径解析到 `src/<mod>.rs` / `src/<mod>/mod.rs`，再过一层 `use X as Y;` 别名表与「限定符首段是另一个 crate」的跨 crate 解析；裸名维持旧口径不动（否则整张矩阵无谓搬家，把真实迁移淹在噪音里） |
| 尺子缺陷 12（宏生成 handler 读成空 body） | `scripts/audit_write_gating.mjs` | `macro_rules!` 展开出的 handler 在全 crate **没有字面 `fn <name>`**，取体返回空串，旧尺把空串一律读成「体内无会话无门禁」→ 落 R7 行动面 = 把「看不见」冒充成「有漏洞」；反向同样漏（空 body 让 GET-写探测恒假）。修法：新增 R8 桶「体内不可解析，本尺不背书」+ 名单落盘 `write_routes_unparsable.tsv`。归桶顺序上先判 R1（中间件 admin）与 R2-R4（匿名可达）再判 R8——首版把 `!body` 放在 exempt 之前，实测吞掉 2 条 R4，已改回 |

### 三.9 第九批（applicationdict 深度路径 24 条写收 admin + 尺子缺陷 13，代码）

缺陷 12 只是**承认**看不见；缺陷 13 把「看不见」变成「读得见」，本批的真洞正是这么冒出来的：

| 项 | 文件 | 内容 |
| --- | --- | --- |
| 尺子缺陷 13（宏模板取证） | `scripts/audit_write_gating.mjs` | 解析一级宏生成：收集本 crate 内模板文本含 `fn $` 的 `macro_rules! X`，再找 `X!` 的平衡实参组，组内出现该 handler 裸标识符即认定由此宏生成，取**宏模板全文**作体内信号；每条都记入 `write_routes_macro_resolved.tsv`（121 条）逐条可枚举，靠模板得到的门禁必须能被点名，不许混进普通 R5。R8 从 98 条降到 1 条（余 `organization_assemble_authentication POST /authentication/code`） |
| GATE_RE 追加 5 个宏模板里的门 | `scripts/audit_write_gating.mjs` | `gate_doc_writer`、`gate_topic_owner`、`gate_reply_owner`、`gate_attachment_owner`、`gate_appdict_writer`。**写成闭集点名、绝不写 `\bgate_` 前缀**：全仓 `gate_` 标识符普查共 6 个，其中 `gate_sql`(12 处)、`gate_table`(8 处) 是变量名不是门，收前缀会把两者当成门禁信号 |
| pp 字典深度路径写 24 条 | `crates/processplatform_service_processing/src/u2.rs` | `dict_data_fns!` 宏（`dict_set_1..8` 注册在 POST+PUT、`dict_del_1..8` 注册在 DELETE，共 24 条写路由）的模板里**既不取 Session 也不校任何门**：任一登录用户按 `/applicationdict/{id}/{p0}…/{p7}/data` 就能改写或摘除任意应用的 `x_application_dict.data`。按本文件 §applicationdict 既有口径（字典表 = admin 档，与 `dict_edit` 同档）收口：模板两把 handler 各加 `session: Option<Extension<Session>>` + `require_admin_opt(&pool.0, session.as_deref()).await?`，置于任何 SQL 之前 |
| 宏模板 R6 41 条按族抽查（不是漏洞，记账防重复劳动） | `crates/cms_assemble_control/src/lib.rs`、`crates/processplatform_assemble_surface` | 30 条 `surface_appdict_*_data_{post,put,delete}` → 模板只 `session` 入参，门在 `appdict_data_write`（lib.rs:6599 `gate_appdict_writer`）；6 条 `permission_save_{manager,publisher,viewer}_{app,category}_u3` → 门在 `u3_save_scope_permissions`（lib.rs:9962 `u3_can_manage_scope`）；5 条 `read/task/taskcompleted/review/readcompleted_u2_filter_attribute_post` → POST 上挂的读，无写面。三者都属本尺声明不扫的「helper 内部门禁」，落 R6 是口径而非洞 |

### 三.10 第十批（`mock*` 别名写族三 tranche + 尺子缺陷 14、15，代码）

§五 第九批点名的首选靶面（surface `applicationdict` 别名族）本批收完，并顺着同一形状
把两处同族洞一起收了：`surface data/work` 的 `mock*` 别名族（**未认证物理删除流程实例**）
与 `processplatform_service_processing` 的两把匿名软删 POST。三 tranche 同尺对账见 §四。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| tranche 1：surface applicationdict 别名写 16 把 | `crates/processplatform_assemble_surface/src/lib.rs` | `applicationdict_{flag}_{app}_{p0..p7}_data_mock{deletetoget,puttopost}` 8 档 × 2 族：handler 原先**不取 Session、不校任何门**，体内 `DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1`（同一条删两次）/ `UPDATE … SET "xupdateTime" = NOW()`。系统级元数据 → 按本 crate R4-7 口径收 admin（`require_admin_opt`，无会话 401 先于任何 SQL，非 admin 403 走 `is_admin` fail-closed）。16 处注册点里 7 档 `mockdeletetoget` 挂在 **GET** 上，中间件两道门都够不着，handler 是唯一一道门 |
| 同族正式路由的定性（**不是漏洞，记账防重复劳动**） | 同上 `lib.rs:1241` | `applicationdict_…_path{0..7}_data` 24 条「写方法」路由的本体只有一句 `SELECT` —— 同一把只读 handler 挂在 POST/PUT/DELETE 上。本批不给它加门（加了是假修），改由尺子新开「只读挂写方法」桶点名（见下 R7 重定基） |
| tranche 2：surface `data/work` 别名写 18 把 handler / 45 条注册点 | `crates/processplatform_assemble_surface/src/lib.rs`、`src/routes.rs` | `data_work_id_{,path0..path0_path1..path7}_mock{deletetoget,puttopost}`：无会话无归属，`DELETE FROM PP_C_WORK WHERE xid = $1`（删两次）与 `UPDATE PP_C_WORK SET "xupdateTime" = NOW()`。新增助手 `u4_require_work_actor(pool, session, id)`——`u4_require_session`（401 先于 `pool.get()`）→ 按 xid 读 `"xcreatorPerson"` → `require_actor_opt`（归属非空=本人或 admin；**空归属/行不存在=只允许 admin**）。45 条注册点里 10 条挂 GET，另 35 条走 POST/PUT/DELETE；正式路由 `data_work_id`（PUT/POST/DELETE 同 URI）本体仍是只读，故不收 |
| tranche 3：ppsvc 匿名软删 2 把 | `crates/processplatform_service_processing/src/lib.rs` | `data/delete/{work}/{id}`（`UPDATE x_work SET deleted_at = NOW()`）与 `data/path/delete/{work}/{id}`（按 work 批量把 `x_task.task_status` 改 deleted、`x_review.deleted_at` 落下）。新增 `u4_require_work_owner`：沿用本 crate 的 `u4_session`，但删除属 destructive，取比既有 `u4_require_actor`（空归属放行任意会话）**更严**的「空发起人只允许 admin」口径 |
| 尺子缺陷 14：转义引号表名的写 SQL 判不出来 | `scripts/audit_write_gating.mjs` | `WRITE_SQL_RE` 的 `UPDATE\s+[A-Za-z_]` 分支要求表名首字符是裸标识符，遇到 `UPDATE \"x_table\"`（Rust 源码里写 `r#"…"#`/`\` 转义引号）整条读成「非写面」——**方向是漏量**，且这个正则在 GET-写探测、豁免碰撞分档、R 桶拆分里被复用 6 次。全仓该写法 25 处、涉及 15 条路由。修法：正则补 `UPDATE\s+\x5C?["\x60]?` 分支，并加**判据自证段**——凡体内命中「转义引号表名」形态的路由必须被判为写面，否则抛错退出（现读数：15 条全判为写面、0 条漏量） |
| 尺子缺陷 15：门禁在 helper 里，handler 读成无门禁 | `scripts/audit_write_gating.mjs` | `GATE_RE` 只扫 handler 体内字面量，`gate_xxx(...).await?` 转手交给同 crate helper 内判定的那一族被一律判成「无门禁」→ 假阳性进 R 行动面。新增 `gateEvidence(crate, body)`：体内直判不中时，找同 crate 内被调用且「体内含 `GATE_RE` ∧ 会产 `AppError::Forbidden/Unauthorized`」的 helper，并要求调用点带 `.await?`（`callPropagates` 做括号配平坦扫，防「取了门但没传播错误」的假放行）。本批作证 **12 条**：`check_owner` 5、`import_checkin_rows` 2、`check_cms_publish` 2、`u2_gate_att_or_business_error` 2、`u2_attachment_guard` 1，明细落 `write_routes_indirect_gated.tsv`；替换 4 处 `GATE_RE.test` 调用点，注入臂证明守卫非恒真 |
| 行动面重定基：R7 不等于 1158 条洞 | 同上 + 新产物 `write_routes_readonly_on_writemethod.tsv` | 把「体内含写 SQL / 只读挂写方法 / 体内不可解析」三向拆分加进 R4/R6/R7 的报表与闭合断言（`real + readonly + unparsable == total` 才允许出数）。读数：**R7 1121 条里只 318 条真落笔**、803 条是只读 handler 挂在写方法上；R6 123 写 / 187 只读；R4 3 写 / 9 只读 / 2 不可解析。999 条「只读挂写方法」逐条落盘。**这一格的价值是把选题分母从 1158 收成 355（本批前）→ 318（本批后）**，否则下一轮仍会按 crate 名字顺序猜 |
| 旧契约断言返工（加门改了语义，改断言不动门槛） | `crates/processplatform_assemble_surface/src/tests_u2.rs` | `u2r_data_work_pathn_legacy_shape_extraction_contract` 原写「8 格一律 `assert_eq!(status, 500)`」，500 的来源是「无库时 `pool.get()` 失败」= 证明 Path 元组提取器命中。新门排在 `pool.get()` 之前，5 个别名格读数变 401（同样排除 400/404，且是更强的可达证据）。改成**逐格点名期望码**（3 格只读仍 500、5 格别名必须 401），而不是把判据放宽成「500 或 401 都行」 |
| 一处**被实测否证的类型猜测**（留档） | 同上 §四 第十批 | 我按「迁移 032 把 `pp_e_applicationdict.xupdateTime` 建成 TEXT，`SET … = NOW()` 是 timestamp→text 赋值、必报 42846 ⇒ 这一族即使放行也 500」写进了测试文档注释。DB 臂一跑即**否证**：PostgreSQL 允许经 I/O 转换的赋值转换，落库成 `2026-10-14 16:35:02.054779+00` 这类文本时间戳。注释当场改写，全仓 79 处同型写法由「硬错误」降级为「TEXT 列被写进与会话/其它列不同构的时间文本」的**语义分歧**，进 §五 单独定标 |
| 新增测试臂 | `processplatform_assemble_surface/src/security_gate_tests.rs`、`processplatform_service_processing/src/security_gate_tests.rs` | surface 4 支：`surface_applicationdict_alias_writes_reject_anonymous`（16 格逐条点名 401）、`…_forbidden_for_non_admin`（16 格 403）、`…_delete_alias_only_admin_lands`（DB：闯入者 403 且行必须还在 / admin 200 且行确实消失）、`…_put_alias_admin_write_lands`（DB：落库才算放行凭据，未命中行也回 200）＋ `surface_data_work_alias_writes_reject_anonymous`（45 格点名）、`surface_data_work_delete_alias_owner_gate_lands`（DB 四格：他人 403 零留痕 / 本人真删 / 空归属只 admin / admin 打满 45 档不出现 401·403）、`surface_data_work_touch_alias_owner_write_lands`（DB：403 不改时间戳、本人 200 且时间戳确实被改写）。ppsvc 2 支：`data_delete_routes_reject_without_session`（离线 401×2）、`data_delete_routes_require_work_creator_and_land`（DB 三格）。路由表由 `routes.rs` 机器枚举生成并 `assert_eq!(len, 16 / 45)`，注册点增减当场红 |

### 三.11 第十一批（`PP_C_JOB` / `PP_C_WORKCOMPLETED` 别名写族 18 把 + 尺子缺陷 16、17、18，代码）

第十批收完 `data/work` 后，`§五` 里同形状的另外两族仍裸奔在 R7 行动面：`data/workcompleted/{id}[/pathN]`
与 `data/job/{job}[/pathN]` 的 `mockputtopost` 别名。本批把它们一次收完，并顺着「读数为什么和代码不一致」
把尺子的三处**看不见**补掉——16（写 SQL 在 helper 里）、17（门的传播有两种写法）、18（这条路由根本进不到函数体）。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| tranche：18 把 handler / 36 条注册点（27 活 + 9 死档） | `crates/processplatform_assemble_surface/src/lib.rs`、`src/routes.rs` | `data_workcompleted_id{,_path0..path7}_mockputtopost` 9 把 + `data_job_job{,_path0..path7}_mockputtopost` 9 把。handler 原先**不取 Session、不校任何门**，体内第一条 SQL 是 `UPDATE PP_C_WORKCOMPLETED SET "xupdateTime" = NOW() WHERE xid = $1` / 同型 `PP_C_JOB`，随后按同一 xid 回读 `xid/xcreateTime/xupdateTime` 返回 ⇒ **未认证**可按 ID 改写任意流程归档件 / 任务实例的审计时间戳并拿到其存在性与时间信息（可批量枚举）。放行侧实测：摘门后匿名请求直接 200 且 `xupdateTime` 被改写（见 §四 注入臂 G1·G3） |
| helper 泛化（不是新门，是把上一批的门推广） | 同上 `lib.rs:60` | 第十批的 `u4_require_work_actor` 抽成 `u4_require_ppc_actor(pool, session, table: &'static str, id)`，work 版退化为一行委托 ⇒ 上一批 18 处调用点零改动。表名走 `&'static str` 常量拼进 `format!`，不接用户输入。口径与第十批一致：`u4_require_session`（**401 先于 `pool.get()`**，离线可证）→ 按 xid 读 `"xcreatorPerson"` → `require_actor_opt`（归属非空＝本人或 admin；**空归属／行不存在＝只允许 admin**） |
| 尺子缺陷 16：写 SQL 在 helper 里，handler 读成「只读挂写方法」 | `scripts/audit_write_gating.mjs` | 旧判据只扫 handler 本体的 `WRITE_SQL_RE`，`..._core`／`appdict_data_write` 一类转调全部看不见 ⇒ **方向是漏量**，且 GET-写探测与三向拆分共用这个判据，一处漏三处错。新增 `writeEvidence(crate, body, depth)`：本体不中则沿「被 await 的转调」递归 ≤3 层；`awaitCall` 与缺陷 15 的 `callPropagates` **有意不同**（写只要跑起来就落笔，`let _ = f().await` 同样拦不住）。现场：`workcompleted_list_count_application_*` 注册在 **GET** 上、本体零 SQL，转调体内就是那条 UPDATE。同批修的调用点正则：`body.matchAll(/([a-z_]\w*)\s*\(/g)` 会把 `row.get(` 当成「调 crate 内名为 `get` 的函数」，首版读数炸到 间接写 1033 / GET 写 711 / A1 27，加负向后视 `(?<![.\w:])` 后落回 319 / 227 / 9 |
| 尺子缺陷 17：门的拒判会经两种写法传播，且门常在同 crate 的第二层 | 同上 | `callPropagates` 旧写法只认 `)` 之后紧跟 `.await?`；尾位置 `.await`（`)` 后第一个非空字符是 `.`，且 `.await` 之后就是 `}`）同样把拒绝传到调用方，旧判据认不出。`gateEvidence` 旧写法又叠了两个多余前提：要求 helper 自身命中 `GATE_RE`、且只走一层。现改成「helper 会产 `AppError::Forbidden\|Unauthorized` ∧ 调用点传播 ∧ 递归 ≤3」。读数 **indirectGated 12 → 37**（`set_topic_flag` 18、`check_owner` 5、`write_doc_fields_base` 3…），A→B 轴上 4 条 cms `data_document_*` 由 R6 迁 R5 就是这格的功劳 |
| 尺子缺陷 18：死档注册（Path 段数不足，函数体永不执行） | 同上 + 新产物 `routes_dead_alias.tsv` | 注册路径的 `{}` 段数 **少于** handler 签名 `Path<…>` 需要的段数时，axum 在进入函数体之前就抛 500「Wrong number of path arguments」，SQL 与门都够不着；旧判据只看「方法 + 体内有无写 SQL」，把这类死档算成活写面（**方向是虚报行动面**）。判据 `pathParamCount` vs `pathExtractArity`，两处收窄都是踩过坑加的：① 只看**签名**（第一个 `{` 之前），否则 `Vec<u8>`、`impl Future<Output=…>` 让前扫吞掉十几处逗号，实测把 paging 族读成「需要 16 个参数」；② 抽出泛型实参后**剥掉 `(`/`)` 再按顶层逗号切段并丢掉空段**，否则 rustfmt 换行的 `Path<(\n String,\n String,\n)>` 尾逗号多数一档，实测 24 → 152 条，一半活路由被误杀。全仓现读 24 条死档（写方法 10 条 → 新桶 R9，其余 14 条挂在 GET 上、已从 GET 写面出列） |
| 死档的处置定拍（**不补段**） | `lib.rs`、`tests_u2.rs`、`security_gate_tests.rs` | 补上参数段等于**把一条今天打不通的路由激活成新攻击面**，本批不做；只逐格点名读数，并在测试注释里写死规则：「一旦有人补段，该格期望码必须跟着改成 401」。测试判据用「模板里有无 `{}` 段」这条可机器复核的规则，而不是硬编码「9 条」——`assert_eq!(dead, 9)` 只钉数量，判据本身仍是段数比对 |
| 判据见证段（非恒真自证，本批新增） | 同上 | 缺陷 16／17／18 各配正向见证（`appdict_data_write`、`workcompleted_list_count_..._core`、`provider_login`／`set_topic_flag`、`check_owner`、`u2_attachment_guard`／6 条死档行），缺陷 18 另配**反向**见证（4 条必须保持可达的活档，含 `POST /data/job/{job}/mockputtopost` 与 correlation `unlink`），不齐即 `process.exitCode = 1`。这一格是补前十批的方法学缺口：只看一次读数无法证明「修尺子」本身成立，也没有一支尺能量出「尺子退化」 |
| 旧契约断言返工（改判据不动门槛） | `crates/processplatform_assemble_surface/src/tests_u2.rs` | `u2r_data_job_pathn_extraction_contract` 写档改成逐格点名：`POST /j-1/mockputtopost` 必须 401（活档＋门在 SQL 前），`GET /job/mockputtopost` 必须 500（死档，且在注释里写明它 500 不是门失效而是入口到不了）。`u2r_data_workcompleted_pathn_legacy_shape_extraction_contract` 两格只读 500、三格别名 401。放宽成「500 或 401 都算过」会让「别名漏注册」「门被摘掉」两种红同时读成绿，故拒绝 |
| 新增测试臂 4 支 | `crates/processplatform_assemble_surface/src/security_gate_tests.rs` | 注册点由 `routes.rs` 机器枚举成 36 行 `ppc_alias_routes()` 并 `assert_eq!(v.len(), 36)`（增减当场红，注释按 handler 分组）。① `surface_ppc_alias_writes_reject_anonymous`：27 活格 401 + 9 死格 500 + `dead == 9`；② `surface_data_workcompleted_touch_alias_owner_gate_lands`、③ `surface_data_job_touch_alias_owner_gate_lands`（DB：他人 403 且 `xupdateTime` 哨兵 `'r50-before'` 不变 / 本人 200 且哨兵确实被改写 / 空归属行普通会话 403、admin 200）；④ `surface_ppc_alias_admin_passes_all_registrations`（admin 打 36 档不得出现 401·403，防门夹死放行侧）。造行按迁移 032 兜底建表（两张表 `"id" TEXT PRIMARY KEY` + `xid/xcreatorPerson/xcreateTime/xupdateTime` 全 TEXT，handler 放行后回读 `xcreateTime`，建表漏它就测不到那条回读） |

### 三.12 第十二批（尺子缺陷 19、20、21——纯尺子轴，本批零产品码）

第十一批 §五 把「先修尺子再动 cms」写成了硬前置：cms 86 条真落笔里 `surface_appdict` 一族占 32，
若这 32 条其实是「有门但尺子不认」，动手加门就是白加 32 道门。本批只做尺子轴，一次补掉三处**看不见**，
并把行动面读数从 621 修到 **498**——全部净差来自「尺子多认了 127 条已存在的门」，**本批没有修掉一个洞**。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| 尺子缺陷 19：`callPropagates` 的尾表达式臂在折行代码上恒假 | `scripts/audit_write_gating.mjs:645-670` | 旧写法先把 `)` 后 40 字符 `\s+`→`""` 压平去认 `.await`，再拿**未压平**的原串按 `i + 6` 切片去找闭括号。rustfmt 把长参数调用折成 `)\n            .await\n        }` 时，`)` 与 `.await` 之间已有 12 个以上空白，`i + 6` 仍落在空白里 ⇒ `/^\s*\}/` 面对的是 `.await…` 而非 `}` ⇒ **尾表达式转调永远不作证**。改成同一段原始文本上两条正则并存：`/^\s*\.await\s*\?/`（传播）与 `/^\s*\.await\s*\}/`（整个表达式是函数体尾值）。这是缺陷 20 的前置——不先修切片，`propagatesGateName` 在 helper 体内同样认不出 |
| 尺子缺陷 20：`helperCanReject` 只认 `AppError::Forbidden` 字面量 | 同上 `:672-695` | `shared::middleware::require_admin(pool, session).await?` / `require_owner(…).await?` 这一族门**由 shared 内部产错误**，helper 体内一个字面量都没有，旧判据一律读成「helper 不会拒绝」⇒ 已加门的档位掉进 R6 无门禁面（**方向是虚报行动面**）。新增 `propagatesGateName(body)`：helper 体内找 `GATE_RE` 命中的门禁调用 ∧ 调用点经 `callPropagates` 传播，两者同时成立才算会拒绝；仍要求传播，所以 `let ok = is_admin(…).await;` 这种「读了标志却照样落笔」不作证。现场：attendance `delete_admin_record`（体内 `require_admin`）、`delete_owned_record`（体内 `require_owner`） |
| 尺子缺陷 21：宏模板未按「生成函数」切段，读档借写档的门禁与落笔 | 同上 `:700-780`、`:macroTemplateFor` | 缺陷 13 只解决「体内信号取自 `macro_rules!` 模板」，但取的是**整张模板**。`appdict_depth_family!` 一次生成 5 把（匿名读、认证读、PUT、POST、DELETE），整张模板里既有写 SQL 又有 `appdict_data_write` ⇒ 两把读档借走邻居段的门禁（假绿、GET 面漏量）也借到落笔信号（假红、算进真落笔）。新增 `templateSegments`：按 `fn $param` 取平衡块切段，位置绑定**按 matcher 形参顺序**映射实参（`params.forEach((p, ix) => map.set(p, toks[ix]))`）——第一版按段序 `toks[ix+1]` 绑定，因 `$arity` 在不同 matcher 里可能在前也可能在后，会把整表偏移一档；`matcherParams` 遇到含 `$(` 的复数展开返回 `null`，该宏**回落整张模板**并在报告里当面点名。现读 121 条宏写路由 = 切段 62 + 整张 59，回落名单 `processplatform_service_processing/dict_data_fns` **16 把须人工复核** |
| 判据见证扩充 + 一支**独立反证尺** | 同上 见证段 | 缺陷 19 正向 4 条（cms `surface_appdict` PUT/POST/DELETE×2，helper `appdict_data_write`）、缺陷 20 正向 2 条 + 反向 3 条（`query_assemble_designer/delete_designer_bare`、`surface/attachment_u2b_doc_to_word`、`message/…/group/mockdeletetoget` 三条**不得**被作证）、缺陷 21 反向 2 条（两把 appdict 读档不得被 `appdict_data_write` 借证）+ 正向 1 条。**缺陷 20 的恒真放松点不到名**：临时探针在自然语料里实测 0 条「放松后新误证」的样本（`/d/tmp/probe_nonenforcing.cjs`），点名式反向见证否决不了它，故另写一支**不调用 `helperCanReject`、自己重写一遍「字面量 or 传播门禁」判据**的独立反证尺去扫全量 `indirectGated`；两支尺同读才计分 |
| 本批顺手量到的**真洞**（未修码，进 §五 首选） | `crates/cms_assemble_control/src/lib.rs:2742-2765` | 切段之后，`data_path_write_handlers!` 的 `$mockdel` 段（8 把 `GET /api/data/document/{id}/{path0…path7}/mockdeletetoget`）从「借邻居段门禁的假绿」里出列，落回 `K3-已认证无会话无门禁`：该段签名**根本不取 `Session`**，体内第一条语句就是 `UPDATE x_cms_data_document_field SET deleted_at = NOW() WHERE doc_id=$1 AND field_name=$2`，而同宏的 `$del`/`$put`/`$create` 三段一律走 `write_doc_field_path(&pool, &session, …)` 带归属门 ⇒ 同一条软删逻辑存在「有门的 DELETE」与「无门的 GET 别名」两份实现。本批**未动码**：改法是让 `$mockdel` 委托 `write_doc_field_path(… PathWriteMode::Remove)`，但返回体形状（现读 `ActionResult::success(row)`）与前端调用面需先核对 |

### 三.13 第十三批（尺子缺陷 22——动态表名的写语句读不出来，纯尺子轴）

第十二批收口后按 498 那张表排下一批代码轴，第一刀落在 cms 的 `GET …/mockdeletetoget` 族。
动手前抽查发现两条同形状档（`appinfo_id_mockdeletetoget`、`comment_id_mockdeletetoget`）**在任何产物里都查无此行**
——不是没注册（`routes.rs:595` / `:615` 都活着），而是尺子把它们读成了「不含写 SQL」。根因是一处新缺陷：

| 项 | 文件 | 内容 |
| --- | --- | --- |
| 尺子缺陷 22：`format!` 插值的表名位不被认作写语句 | `scripts/audit_write_gating.mjs:732` | `soft_delete_by_id(pool, table, id)` 这类「表名当参数传」的 helper 体内写作 `"UPDATE {} SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"`，`{` 不是标识符首字符 ⇒ `WRITE_SQL_RE` 的 `UPDATE\s+…` 一支落空 ⇒ **helper 判为只读**，缺陷 16 的递归转调也就无从作证。方向是**漏量**：全仓 14 个函数这样落笔，真落笔少算 14 条、GET 无门禁面少算 10 条。修法与缺陷 14 同形——表名位再允许一条 `{` 前缀（`{}` 与 `{ident}` 同形状），仍要求 `UPDATE` 后紧跟它，不把散文里的 "update " 算成写语句 |
| 刀口自证改打在 **helper 层** | 同上 `:959-981` | 缺陷 14 那条自证扫的是 handler 本体，这一族漏量全在 helper 里，照旧写法扫不到东西（等于恒真）。新增 `DYNAMIC_TABLE_RE`，直接扫 `indexCrateFns` 的**全部函数体**：凡体内出现 `(UPDATE\|DELETE FROM\|INSERT INTO)\s+\{` 都必须被 `WRITE_SQL_RE` 判为写面，否则当场 throw 并逐处点名函数名。现读「14 个函数全部判为写面，0 处漏量」 |
| 判据见证（正向 3 + helper 1 + 反向 3） | 同上 见证段 | 正向钉三条 GET 档必须进 GET 面（cms `appinfo` / `comment` / `viewfieldconfig` 的 `{id}/mockdeletetoget`）+ `soft_delete_by_id` 必须在间接写 SQL 名单；反向钉三条**不许**被拖进 GET 面（`data_document_id_mockdeletetoget` 有门走 `delete_doc_fields_base`、`appinfo_id_control` 真只读、organization `person/{flag}/mockdeletetoget` 已有门）——反向那一把同时兜住缺陷 21 切段与缺陷 22 放宽的合谋退化 |

### 三.14 第十四批（cms 的 GET 动词别名写族 27 把加门 + 尺子缺陷 23，代码轴）

第十三批浮出的那张名单在本批收口：27 把 handler 改动 = **22 把 `mockdeletetoget` 别名 + 2 把无门正档 +
3 把非别名的 GET 写**；同批立起尺子缺陷 **23**。选题理由照第十三批那格的问句回答——这一族处在
`csrf_middleware` 与 `requires_admin` 的**双盲区**（两道中间件只看方法，GET 一律放行），比 surface 那 79 条
「已认证才能碰」的面更靠前。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| 宏 `$mockdel` 段收成与 `$del` 同一条路径（一处宏改动收 8 把） | `crates/cms_assemble_control/src/lib.rs:2943+`（`data_path_write_handlers!` 宏体） | 段签名补 `Extension<Session>`，体内改判为 `write_doc_field_path(&pool, &session, doc_id, rest.to_vec(), None, PathWriteMode::Remove)` ⇒ 8 把 `GET /api/data/document/{id}/{path0…path7}/mockdeletetoget` 与同宏 DELETE 档共用**文档写者门**。**返回体形状按第十三批那格先拍后改**：旧段删完立刻用带 `deleted_at IS NULL` 的回读，永远吐 `ActionResult::error("field not found")`（连成功也报没找到）；改后是 `{docId, fieldName, deleted:true}`。核对调用面——前端只在 `ProcessWork.vue` 的**注释**里提到 `mockdeletetoget`，`DocumentApp.vue` 仅按状态码 smoke 探测，openapi 契约是空 stub，parity 只断言路由存在 ⇒ 没有调用方在判那枚恒 error |
| `soft_delete_by_id` 族 14 把别名 + 2 把正档补归属门 | 同上（`appinfo` / `categoryinfo` / `comment` / `file` / `fileinfo` / `form` / `script` / `view` / `viewcategory` / `viewfieldconfig` 各档） | 判据一律**照抄同表正档**：`x_cms_appinfo.manager`、`x_cms_comment.person_id`、`x_cms_fileinfo.upload_person`、`x_cms_file.creator`、`form` / `script` / `view` / `viewcategory` 的 `creator`；两张派生表走 SQL 门（`U2_CATEGORY_OWNER_SQL` 经所属应用 manager、`U2_VIEW_FIELD_OWNER_SQL` 经所属视图 creator）。`appinfo_erase_app_id`、`categoryinfo_erase_category_id` 两把**正档**同批补门（本批矩阵里那 2 条 R7→R5 就是它们）。两处旧体顺手改掉的分叉：`fileinfo_id_mockdeletetoget` / `form_id_mockdeletetoget` 原来是手写 `UPDATE … RETURNING *`，`file_flag_mockdeletetoget` 原来删完还按**不带** `deleted_at IS NULL` 的回读把整行（含 `contentBase64`）吐回去 |
| 新增形状 helper，正档与别名不再各写一份三态 | 同上 `:503-522` | `soft_delete_after_gate(pool, gate, table, id, not_found)`：Forbidden→403、NotFound→200+error、Allowed→`soft_delete_by_id` 后回 `{deleted:true}`。**只作形状、不作证**：`gateEvidence` 对「拒绝型 helper」只做预筛（缺陷 20 那格），所以没把它的名字塞进 `GATE_RE`——塞了会把「传入 `Allowed` 的调用者」也一起洗成有门 |
| admin 档 3 把 | 同上 | `templateform_id_mockdeletetoget`（删的是 **`x_cms_form_v2`**，与 `templateform_u2_delete` 的 `x_cms_templateform` **不是同一张表** ⇒ 按表定级 admin）、`design_appdict_id_mockdeletetoget`（旧体是一份 68 行手写副本，且回读恒 `appdict not found`，给不出可用响应体）、`permission_management_refresh_category_categoryId`（GET 语义 = 软删该分类下**全部**权限行，与 `…_refresh_all` 同档）。三把一律 `u2_require_admin`，另两把非别名写（`file_copy_u3`、`fileinfo_…_seqnumber`）按 owner / 文档写者档 |
| 定拍**不修** 5 把 | 同上 | `document_id_view_count`（只加浏览计数）、`document_u2_commend` / `document_u2_uncommend` / `comment_commend_u3` / `comment_uncommend_u3`（只动**本人** `person_id` 行）——按 o2server 语义任意已认证用户都可为，故留在原档不动 |
| 尺子缺陷 23：`GATE_RE` 词表缺 `u2_gate_by_sql` | `scripts/audit_write_gating.mjs:352` | 该词缺席 ⇒ 凡**直接**用 `u2_gate_by_sql` 定门的档一律读成「无门」。两个方向都错过：`DELETE /api/categoryinfo/{id}`、`DELETE /api/viewfieldconfig/{id}` 在 HEAD 上长期挂 **R6 假阳**（本批矩阵迁移逐条证实，见 §四 第十四批读数），本批新加门的 3 把别名又一度挂 **K 桶假阴**（只 relabel 成 K2 而不出列）。修法补词 + 3 正向 / 1 反向见证，反向钉 `GET /api/document/{id}/view/count` 不许被拖进 GET 面。另查清：全 crate 15 处 `u2_gate_by_sql` 命中**没有一处只在注释里**（`grep` 计数 0），故本轮不存在「词表被散文喂假绿」的第四条缺陷 |
| 缺陷 22 见证重钉（**见证过时 ≠ 判据退化**） | 同上 见证段 | 原正向 3 条钉「必须挂 K 桶」，代码修好之后它们出列 ⇒ 尺子当场退 1、喊「缺口 2 处」。这不是判据被摘，是见证把**读数**钉死了：改成钉「必须仍带写证据」（`indirectWrites` 成员断言），并把已加门的 3 把转成反向见证（不许被误判成 GET 写面）。run L 那次红是有效信号，按「见证钉判据、不钉读数」返工 |


### 三.15 第十五批（surface 的 mock 动词别名 GET 写族 22 把补归属门，纯代码轴）

第十四批把 cms 的同一族收掉后，surface 这格的理由用完了（当时写的是「尺子已量不出新东西，纯代码轴」）。
本批按**同一问句**选题：`mockdeletetoget` / `mockputtopost` 两条别名将 DELETE/PUT 语义挂在 GET 上，
而 `csrf_middleware` 与 `requires_admin` 只看方法 ⇒ GET 侧落在**双盲区**；这 22 把 handler 在 HEAD 上
体内直接 `DELETE` / `UPDATE … xupdateTime = NOW()`，**一个会话都不读**。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| 22 把别名 handler 补会话参数并把门插在第一句 `pool.get()` **之前** | `crates/processplatform_assemble_surface/src/lib.rs`（22 处签名 + 22 处门，+68 行） | 形状一律 `session: Option<Extension<shared::session::Session>>` + 首行判门 ⇒ 无会话先 401，**不触库、离线可证**（与第十/十一批同法）。档位按「行上到底有没有可认的属主列」定，不是按表名猜 |
| 新增 helper `u4_require_ppc_person` | 同上 `:92-115` | 本 crate 的条目表（待阅 / 已阅 / 待办 / 已办 / 草稿 / 流水号 / 签核）**没有 `xcreatorPerson`**，行上属主列是 `xperson`（同 crate 的待办待阅列表本来就按 `WHERE xperson = <person_unique>` 过滤，lib.rs:592 / :631 为证）。口径与 `u4_require_ppc_actor` 同锁：无会话 401 → 归属非空则本人或 admin → 归属为空或行不存在只允许 admin |
| 定档依据 = information_schema 探针，不靠类比 | 探针日志 `/d/tmp/b15_probe.log` | `xperson` **11** 把（DRAFT 1 / READ 2 / READCOMPLETED 1 / SERIALNUMBER 2 / DOC_SIGN 2 / TASK 2 / TASKCOMPLETED 1）走新 helper；`xcreatorPerson` **3** 把（REVIEW 1 / WORKCOMPLETED 2）走 `u4_require_ppc_actor`；`PP_C_WORK` **6** 把走 `u4_require_work_actor`；**`PP_C_RECORD` 2 把走 `require_admin_opt`** |
| 新档：`PP_C_RECORD` 无属主列 ⇒ 只能 admin 档 | 同上 `:13670` / `:13732` | 探针实测该表列为 create_time / creator / creator_person / deleted_at / id / order_number / sequence / update_person / update_time / xcreateTime / xid / xjob / xtitle / xupdateTime / xwork，`xperson` 与 `xcreatorPerson` **都不存在** ⇒ 「按行上属主放行」这一档在此表**不可表达**，本批收 admin。测试侧把这条升成证伪格（见 §四 第十五批读数臂②） |
| 新发现（**本批只记账不修**）14 把 | 同上 各 `mockdeletetoget` 体 | 体内把**同一条 `DELETE … WHERE xid = $1` 连执行两次**：第二次必 0 行 ⇒ 走 `if result == 0` 分支 ⇒ **删成功也恒回 `ActionResult::error("… not found")`**。与第十四批 `templateform` 那格同族（旧体是 68 行手写副本、回读恒 not found）。改返回体形状要先核调用面，属另一批 |
| 尺子轴：**本批一字未动** | `scripts/audit_write_gating.mjs` | 真落笔 −22、GET 面 −22 全部来自代码，不像第十三/十四批那样掺尺子轴的假阳回收。R1/R2/R3 只是注入取证，未入库；R2 顺手暴露 `GATE_RE` 里 `require_actor` 是**无独立见证的冗余词**（详见 §四 注入臂那格），删词要连见证一起改，故本批不擅动 |

### 三.16 第十六批（ppsvc 的 GET 面 22 把 handler 补归属门 + 两类真缺陷顺带修正，纯代码轴）

第十五批 §五 首选那格按实测收掉：`processplatform_service_processing` 的 GET 面 **27 条注册 / 22 把 handler**，
**全部挂 K3**（已认证、无会话参数、无门禁）——`GET snap/work/{workId}/type/*`、`GET snap/workcompleted/{id}/type/*`、
`GET snap/{id}/restore`、`GET workcompleted/rollback/{id}`、`GET work/v2/{id}/goback|rollback|reroute/{p2}`、
`GET task/{id}/urge|replace|press|expire|pass_expired`、`GET task/v2/{id}/pause|resume`，以及 `u2` 面同名快照 / 催办族。
任一登录用户按 ID 即可暂停 / 恢复 / 转办 / 催办 / 过期他人待办、给他人单据落 abandoned / suspend 快照、
回滚他人的完成记录。选这批前按批 15 的提醒先核了宏段旧账：这 22 把**全是手写的具名 handler**，不在
「16 把宏未绑段 + 43 条整张模板」那格上，可以直接按 handler 切片动工。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| 22 把 handler 补会话参数、门插在函数体第一句 | `src/lib.rs` 15 把 + `src/u2.rs` 7 把（签名 + 门，lib.rs +129 行 / u2.rs +53 行） | 形状一律 `session: Option<Extension<shared::session::Session>>` 置于 `pool` 之后、判门先于 `pool.get()` ⇒ 无会话 401 且**不触库、离线可证**（与第十/十一/十五批同法）。`taskcompleted_press` 的门插在**原有 IDOR 等值校验之前**——先看是不是你的单据，再看是不是这个 work 的 |
| 新增 helper `u4_require_actors` + 键表 `u4_actor_key_sql` | `src/lib.rs:465-527` | 键 → **(整句字面量 SQL, 归属列名数组)** 的集中表：SQL 常量只写在这里，调用点只给键与 ID，**不做拼接**。多列口径与 `u4_require_work_owner` 同锁：逐列取非空且等于 `person_unique` 即放行，全不匹配 / 行不存在 → `is_admin` 兜底，否则 403（空归属的脏行只允许 admin，不把数据缺陷洗成权限放行） |
| 三档归属，按「行上到底有没有可认的属主列」定档（22 把逐档点名，合计与 §四 的格数互校：work 9 + task 7 + workcompleted 5 + snap 1 = 22） | 同上 | **work 键 9 把**（3 把 `work_v2_*_p2` + 2 把 `snap_work_*` + `u2` 的 3 把 `snap_type_*` 与 `taskcompleted_press`）沿用 `u4_require_work_owner` 严格校 `x_work.creator`；**task 键 7 把**（urge/replace/press/expire/pass_expired/pause/resume）取 `x_task.person` 与 `x_work.creator` **两列 OR**——催办、过期、暂停的合法操作人本来就是流程双方（发起人催别人手里的办，负责人转办 / 暂停自己的办）；**workcompleted 键 5 把 / snap 键 1 把** hop 到 `x_work.creator`：migration 033 虽给 `x_snap` 加了 creator / person，本 crate 写入路径从不填、全是 NULL，按「空归属」算等于没有 |
| **顺带修正的真缺陷①**（键写错，不是权限问题） | `src/u2.rs` `snap_take_for_wc` | 完成项快照把 **`x_workcompleted` 的 ID 写进了 `x_snap.work_id`**，而读回侧（`snap/workcompleted/{id}/type/snapworkcompleted`、rollback 取最新快照）一律按真实 work_id 查 ⇒ 这些快照**存进去就再也读不到**。改法：先 `SELECT work_id FROM x_workcompleted` 再拿真 work_id 落快照。注入臂无——它由 §四 第十六批臂②的「按 work_id 数快照」判据直接见证 |
| **顺带修正的真缺陷②**（JSONB / TEXT 双向错配 7 处） | `src/lib.rs` 写 4 处 + 读 3 处 | `x_snap.snap_data` 是 **JSONB**（migration 020），写入侧 4 条 INSERT 直接塞 TEXT 参数 ⇒ PostgreSQL 42804 类型不匹配、**必 500**；读回侧 3 处 `get::<String>` 读 JSONB ⇒ `error deserializing column`。改法沿用 u2.rs 既有 cast 口径：写 `($n::text)::jsonb`、读 `snap_data::text AS snap_data`。这 7 处**本批动手前就在**，是加门后真库臂第一次真落笔才暴露的——权限门与数据形状两条账要分开记 |
| 新发现（**本批只记账不修**）：`row_to_json` 静默丢列 | `crates/shared/src/response.rs:17-48` | 匹配臂只覆盖 BOOL / INT* / FLOAT* / TEXT / VARCHAR / NAME / BPCHAR，`_ => None` ⇒ JSONB / JSON / DATE / TIMESTAMPTZ / NUMERIC 一律**不进响应**且不报错。上面那条读回缺陷正是靠显式 `::text` 绕开，但根因在跨 crate 的公共序列化口，改它要连所有调用面的响应形状一起核，属另一批 |
| 尺子轴：**本批一字未动** | `scripts/audit_write_gating.mjs` | GET 面 −27、真落笔 −3 全部来自代码。矩阵里另有 **3 条 POST 行**同键从 R7 搬进 R5——同一把 handler 也挂在 POST 上，加门之后两行注册同步受益，不是新漏洞消失（见 §四 键级 diff 那格） |

### 三.17 第十七批（program_center 的 GET 面 19 条注册 / 17 把 handler 收 admin 档，纯代码轴）

`program_center` 是单文件 441KB 的 `lib.rs`，其 GET 面在尺子上挂 **21 条 K3**（已认证、无会话参数、无门禁），
是第十五/十六批之后「把物理状态写注册在 GET 上」这一族里剩得最整的一格。逐把读函数体后定性：
这 21 条里 **19 条**是真要收的写面（同步日志、组织回调、market 装卸、validation 超时、welink 同步），
其余 **2 条**（`captcha/v2/create/...`、`code/create/mobile/{mobile}`）在登录与验证码下发链路上，
收 admin 会直接打断认证，本批**不收、只记账**（见下「待拍」）。选批前按批 16 的提醒核过宏段旧账：
这 17 把全是手写具名 handler，不在 `u2h::` 未解析宏那格上。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| 17 把 handler 补会话参数、门插在函数体第一句 | `src/lib.rs`（handler 起始行含 `636 / 4669 / 4760 / 5437 / 5471 / 5700 / 6158 / 6232 / 7726 / 9054 / 9106 / 11851` 等）+ `src/tests_u3.rs` | 形状一律 `session: Option<Extension<shared::session::Session>>` 置于 `pool` 之后、`require_admin_opt(&pool, session.as_deref()).await?` 先于 `pool.get()` ⇒ 无会话 401 且**不触库、离线可证**（与第十/十一/十五/十六批同法）。`Option<Extension<..>>` 必须是第二个 extractor，`Json` 仍留末位 |
| 档位取 **admin** 而非归属，按「行上到底有没有可认的属主列」定 | `src/lib.rs:9189-9209`（`require_admin` / `require_admin_opt` 复用 crate 既有 helper） | 这 17 把只写**全局 / 系统表**（`x_program_sync_log`、`x_program_message_log`、`x_program_callback_registration`、`x_program_deploy`、`x_program_script`），行上没有可认属主列 ⇒ 归属门无从校起，只能收 admin；与 crate 自身的模板 handler `jest_clear_cache_source`（上一批收的）同锁，`/api/program_center` 前缀在 `ADMIN_WRITE_DEPRECATED_PREFIXES` 里、中间件不背书，门必须在 handler 体内 |
| 新增 2 支离线臂，**逐把点名 17** | `src/lib.rs:12728 / 12843 / 12977`（内联 `mod security_gate_tests`） | `b17_get_writes_reject_without_session`（`mock_pool()` + 无会话 ⇒ 必须 401）与 `b17_get_writes_forbidden_before_touching_db`（非 admin 会话 ⇒ 必须 403）。两支都带 `assert_eq!(cases.len(), 17, "本批 17 把必须逐把点名，少一把即漏门")`；第二支的失败消息把「出现 Internal」直接写成「门插在读库之后」的判据 ⇒ 门的位置由断言消息钉住，不靠人眼 |
| `reachable` 族 5 处断言由 500 改 401 | `src/tests_u3.rs:333-341 / 385-388 / 441-445 / 590-598` | 这 5 处原本 `assert_eq!(status, 500)`，加门后无会话变 401。改法沿用上一批 `jest/clear/cache` 的先例（断言跟着真实状态码走 + 注明批次）。**顺带把一条旧怀疑落成事实**：这批 500 断言钉的其实不是错误处理，而是**端点本来就坏**（见下条 schema drift） |
| 新发现①：**路径参数一律不被消费** | `src/lib.rs` 本批 17 把签名 | `/market/{flag}/uninstall`、`/market/{flag}/install/or/update`、`/invoke/{flag}/execute/get`、`/dingding/sync/organization/register/callback/{enable}`、`/validation/timeout/{timeout}` 这几条注册的路径段，在对应 handler 签名里**一个 `Path` extractor 都没有**（17 把签名逐个查过，全部只有 `pool` + `session`）⇒ `{flag}` 传什么执行的都是同一段代码，路径参数纯属装饰。这是行为缺陷而非权限缺陷，本批加门不掩盖它，单列待拍 |
| 新发现②：**program_center schema drift 族**（42703，早于本批存在） | 真库 `information_schema` + 逐句 `INSERT` 探针（事务内跑完回滚） | `x_program_sync_log` 实际**没有** `action` / `org_id` / `event_type` 列，`x_program_message_log` 没有 `user_id` / `message_type`，`x_program_callback_registration` 没有 `source` ⇒ 相关 INSERT 一律 42703、**必 500**，连上一批已收 admin 的 `jest_clear_cache` 也一样；而 `x_program_deploy` / `x_program_script` 的 INSERT 真能落笔，说明 `market_install_offline` 与 captcha / code 是**真落笔**写。本批的门插在读库之前，因此加门不改变这些端点的坏状态，也不会把坏状态洗成好状态 |
| 新发现③：**冒名落笔 `creator='system'` 4 处** | `src/lib.rs:1691 / 1844 / 5709 / 8722` | `x_program_script`（captcha、code 两类）、`x_program_deploy`、`x_program_config` 的 INSERT 把 creator 硬编码成 `'system'` ⇒ 谁落的笔在库里查不出来。`market_install_offline`（5709）本批已收 admin，写者至少限定到管理员；其余 3 处（含 2 把待拍）留作「creator 改会话权威」那一批，与第十六批 `work_start_impl` 同口径 |
| 第三方回调**零验签**族：本批只关伪造面 | 同上 5 把 `*callback*` handler | 钉钉 / 政务钉钉 / 企微的 sync、regist、callback 一族在整个 crate 里**找不到任何签名校验**，且因 schema drift 今天根本跑不通 ⇒ 收 admin 是在「不打断任何活的集成」的前提下把伪造回调的写面关掉。真验签（timestamp + nonce + 签名比对）是另一件事，本批不冒充做了 |
| 尺子轴：**本批一字未动** | `scripts/audit_write_gating.mjs` | GET 无门禁 −19、真落笔 −8 全部来自代码。矩阵里另有 **8 条 POST/PUT 行**同键从 R7 搬进 R5（`dingding/request/pull/sync`、`dingding/sync/organization/callback`、`market/flag/install/or/update`、`market/install/offline`、`qiyeweixin/send/getprivateinfo/message`、`zhengwudingding/sync/organization/callback`、`market/{flag}/install/or/update` 等）——同一把 handler 双方法注册同步受益，不是新漏洞消失 |

### 三.18 第十八批（query_service_processing 的 GET 运维面 18 把收 admin 档，纯代码轴）

第十七批 §五 首选那格按实测收掉：`query_service_processing` 在 GET 写档面上挂 **18 条 K3 / 18 把 handler**
——`touch/{high|low}/freq/{work|workcompleted|document}/node/{node}/{touch|reset}` 12 把、`touch/optimize/index/{node}/touch`、
`table/reload/dynamic`、`neural/{generate|stop/generating|learn|stop/learning}/model/{modelFlag}` 4 把。
这批的形状与批 16/17 都不同：**12 把由 `macro_rules! touch_endpoints!` 生成**（正是批 17 提醒要先核的那格宏账），
且整个 crate 此前**没有任何门禁先例**（`git show HEAD` 的 `src/u2.rs` 与 `src/lib.rs` 里 `session` 出现次数都是 0）。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| crate 首个门 helper | `src/u2.rs:300` | 新建 `require_admin(pool, Option<&Session>)`：`session.ok_or(AppError::Unauthorized)?` ⇒ 无会话 401，再 `shared::middleware::is_admin` 校 `auth_role.name='admin'`（fail-closed）⇒ 非 admin 403。名字不带 `u4_` / `require_owner` 前缀，走尺子 `GATE_RE` 认的 `require_admin*` 词形 |
| 12 把宏生成 handler：**改模板一处** | `src/u2.rs:379`（`macro_rules! touch_endpoints!`）、门在 `:387` / `:396` | 模板的 touch / reset 两个分支各插一句 `require_admin(&pool, session.as_deref()).await?;`，签名同步补 `session: Option<Extension<shared::session::Session>>`（置于 `pool` 之后、`Path` 之前）。宏段旧账已核：这 12 把在尺子上属「按生成函数切段取证」（缺陷 21）那档，不是「整张模板兜底」，所以逐把的门禁作证成立 |
| 6 把具名 handler | `src/u2.rs:436`（`optimize_index_touch`）、`:533`（`table_reload_dynamic`）、`:621 / :632 / :643 / :654`（4 把 neural） | 同形状：函数体第一句判门，先于 `pool.get()` ⇒ 无会话既不 500 也**不触库、离线可证**（与第十/十一/十五/十六/十七批同法）。`table_reload_dynamic` 只有 `pool` + `session`、无 `Path` |
| 档位取 **admin** 而非归属，理由写在行上有什么列 | `migrations/077_u2_residual_modules.sql` | 这 18 把落笔的表是 `x_query_index_state` / `x_query_index_extra` / `x_query_neural_job`（节点状态与模型作业状态），**没有可认的属主列**——既不是某人的单据，也不是某人的模型；且语义是「重建索引 / 停学习 / 重载动态实体工厂」这类运维动作。归属门无从校起，只能收 admin。同 crate 的写方法端点（design/search、table insert/update 等 5 条 POST）本批**未动**，仍记在 R 轴旧账里 |
| 第 19 条 GET 注册**不在本批面**，单列待拍 | `src/u2.rs:663`（`neural_list_calculate_with_work`） | `neural/list/calculate/model/{modelFlag}/work/{workId}` 体内只 `query_opt` 读 `x_query_neural_job` 后算推算，**不含写 SQL** ⇒ 不进 GET 写档面、尺子从来不复述它。但它是「任意 workId 即可取他人单据的推算字段」，属 correlation 读侧 IDOR 同族，本批不加门也不洗白，写进 §五 待拍 |
| 调用面核过：收 admin **不打断任何活的业务流** | `frontend/apps/desktop/src/views/QueryViewApp.vue:886-906`、`QueryManagerDeep.vue:702-722` | 全前端唯一真打这批路径的是 `QueryViewApp.vue` 那组**契约批量探针**：逐条 GET 用字面量 `0` 占位、结果只汇成 `toast('服务处理读C N 条已提交')`，不消费返回值 ⇒ 探针变 401/403 不伤功能。`QueryManagerDeep.vue` 打的 `neural/generate` 等是 `/api/query/assemble/designer/**`（另一个 crate，下一批入口），`QueryViewApp.vue:644` 的 `tableReload` 走 `/api/queryview/**`（surface 前缀），两处本批都没碰 |
| 真库契约臂的适配：补会话，不改断言 | `src/tests.rs:188-195`（`ADMIN_PERSON` + `app()` 注 `Extension(test_session(..))`）、`:300-325`（`ensure_schema()` 幂等 seed `auth_role('admin') / auth_person / auth_person_role`） | 加门后 4 支 u2 契约臂先红在 `tests.rs:308`（`assert_eq!(response.status(), OK)` 读到 401）。处置是**给测试客户端注入 admin 会话**而不是把断言降级——契约臂证的是「状态机 / upsert 去重」业务语义，权限层该由下面的点名臂管。seed 形状沿用 `processplatform_service_processing/src/tests.rs:728-752` 的先例（按 `is_admin` 的真实口径造角色链，不造假列） |
| 新增 2 支离线臂，**逐把点名 18** | `src/tests.rs:788`（`mod security_gate_tests`）、`:799` / `:925`（两支臂）、断言在 `:917-918` 与 `:1044` | `r418_get_writes_reject_without_session`（`mock_pool()` + 无会话 ⇒ 必须全 401）与 `r418_get_writes_forbidden_before_touching_db`（非 admin 会话 ⇒ 必须全 403，且失败消息把「出现 Internal」直接写成「门插在读库之后」）。两支都带 `assert_eq!(cases.len(), 18, "本批 18 把必须逐把点名，少一把即漏门")`；宏那 12 把在臂里**逐个具名调用**（`u2::high_freq_work_touch(..)` 等），不是靠宏展开顺带覆盖 |
| 尺子轴：**本批一字未动** | `scripts/audit_write_gating.mjs` | GET 无门禁 −18 全部来自代码。**R 轴增量为 0**：这 18 条注册全是 GET，`write_routes_matrix.tsv` 只收写方法 ⇒ 矩阵与 ungated 两张产物 md5 一字不差，只有 `write_routes_get_writes.tsv` 变了。GET 轴与 R 轴是两条账，本批正是「−18 与 0 同时成立」的那格，与批 17 的「−19 与 −8」互为对照 |

### 三.19 第十九批（surface 的 GET 面 16 把 handler 补归属门，纯代码轴）

第十八批 §五 首选那格按实测收掉：`processplatform_assemble_surface` 在 GET 写档面挂 **16 条 K3 / 16 把 handler**——
`{read|readcompleted|task|taskcompleted|work|workcompleted}/list/count/application/{applicationFlag}/process`
的 base 与 `_p2` 共 12 把，加 `draft/{id}/start`、`handover/{id}/cancel`、`handover/{id}/process`、`mode/{id}/delete` 共 4 把。
这一格与批 17/18 的**根本差别在档位**：批 18 已预告「surface 那 16 把不能整格收 admin」，本批逐把读体核实了这句话，
并把它落成**归属档**——16 把一支不收 admin。helper 也全是批 15 的既有资产，本批**一个新门 helper 都没建**。

| 项 | 文件 | 内容 |
| --- | --- | --- |
| 16 把 handler 补会话参数、门插在函数体第一句 | `src/lib.rs:7686 / 8460 / 8516 / 8966 / 9755 / 9765 / 11837 / 11847 / 16041 / 16051 / 18627 / 18637 / 20593 / 20603 / 23449 / 23459`（净增 32 行 = 16 句门 + 16 行参数） | 形状一律 `session: Option<Extension<shared::session::Session>>` 置于 `pool` 之后、`Path` 之前（`_p2` 那 6 把的 `Path` 是二元组 `(String, String)`，参数位次一样）；门先于 `pool.get()` ⇒ 无会话 401 且**不触库、离线可证**（与第十/十一/十五~十八批同法）。计数那 12 把保持「wrapper 加门 + 原逻辑留 `*_core`」的最小改动，`_core` 一行未动 |
| **不新建 helper**，两档门按「行上到底哪一列是属主」分工 | `src/lib.rs:97`（`u4_require_ppc_person`）、`:65`（`u4_require_ppc_actor`），两者都是**第十五批**已有资产 | **12 把走 `u4_require_ppc_person`**：`PP_C_READ` ×2、`PP_C_READCOMPLETED` ×2、`PP_C_TASK` ×2、`PP_C_TASKCOMPLETED` ×2、`PP_C_DRAFT`、`PP_C_HANDOVER` ×2、`PP_C_TASK_PROCESS_MODE`——条目表没有 `xcreatorPerson`，行上属主列是 `xperson`，与本 crate 待办/待阅列表本来就 `WHERE xperson = <person_unique>` 过滤同源；**4 把走 `u4_require_ppc_actor`**：`PP_C_WORK` ×2、`PP_C_WORKCOMPLETED` ×2——WORK 族行上**没有** `xperson`，属主列是 `"xcreatorPerson"`，给它编一个库里不存在的属主列就是假门禁。两档口径同锁：无会话 401 → 归属非空则本人或 admin → 归属为空或行不存在**只允许 admin**（四轮定下的严格口径，不用「空归属 = 人人可写」把数据缺陷洗成权限放行） |
| 档位定归属的证据：这 12 把是「读形状写」，收 admin 直接打断终端用户 | `src/lib.rs:9696-9719`（`read_list_count_..._core`）为代表 | 12 把计数 handler 的函数体只有一句 `UPDATE <表> SET "xupdateTime" = NOW() WHERE xid = $1` 加同 ID 回读 createTime/updateTime——路径叫「list/count」，落笔却是 touch 时间戳，属批 16/18 立的「只读挂写 / 读形状写」同族。真打它们的是所有人的待阅、待办计数刷新，收 admin 即把桌面端主流程打断 ⇒ 只能按「谁的和谁能 touch」收 |
| 新增 2 支臂，**逐把点名 16** | `src/security_gate_tests.rs:1340`（`mod r419_gate_tests`）、`:1578` / `:1597`（两支臂）、断言在 `:1590` / `:1611` / `:1629` / `:1640` | `r419_get_writes_reject_without_session`（`mock_pool()` + 无会话 ⇒ 必须全 401，失败消息直接把「出现 Internal」写成「门插在读库之后」的判据）与 `r419_get_writes_owner_gate_lands`（真库三段：他人会话按 ID touch 他人条目行必须 403 **且不落笔**、本人会话放行、空归属行只允许 admin）。两支都带 `assert_eq!(cases.len(), 16, "本批 16 把必须逐把点名，少一把即漏门")`；`call_all`（`:1364`）是 **16 次具名 handler 直调**（`one` / `two` 两个 Path 构造闭包），不靠 `ROWS` 表长顺带覆盖 |
| 真库臂造数与自清：两遍幂等 + 零残留行 | `src/security_gate_tests.rs:1350`（`ROWS`：9 张表 × 属主列 × `r419-*` 主键）、`:1513 / :1530 / :1540 / :1553`（`seed_rows` / `clear_actor` / `unseed` / `seed_admin_role`） | seed 与 admin 角色链一律 `ON CONFLICT DO NOTHING / DO UPDATE`，臂体首尾各 `unseed`；本轮按纪律连跑两遍，两遍读数一致（不是「第二遍靠上一遍留下的数据才绿」），收口后再按 `xid LIKE 'r419-%'` 逐表数过 `PP_C_READ / PP_C_WORK / PP_C_DRAFT / PP_C_HANDOVER` **全 0 行**。`seed_admin_role` 走 `is_admin` 的真实口径造 `auth_role / auth_person / auth_person_role`，不造假列 |
| 真缺陷①：**`draft/{id}/start` 必然 500**（早于本批存在，本批只加门不修） | `src/lib.rs:7689-7694` | SQL 是 `UPDATE PP_C_DRAFT SET xstatus = $1, "xupdateTime" = NOW() WHERE xid = $2`，参数却只绑一个 `&[&id]` ⇒ 报 `there is no parameter $2`；另外真库 `pp_c_draft` **没有 `xstatus` 这一列**（42703）。两个**互相独立**的成因，任一修掉另一个还在。真库臂因此对该把**明列豁免**：断言写成 `assert_eq!(broke, vec!["draft_id_start"], …)`（`:1626` 的失败消息把这条例外点名写死），既不放水成「整臂 skip」，也不让它的 `Internal` 冒充「门失效」 |
| 真缺陷②：**`handover/{id}/cancel` 与 `handover/{id}/process` 函数体逐字全同** | `src/lib.rs:8455-8475` 与 `:8511-8531` | 两把都只做 `UPDATE PP_C_HANDOVER SET "xupdateTime" = NOW() WHERE xid = $1` 加回读时间 ⇒ 「取消交权」和「处理交权」在 Rust 侧**没有任何区别**（o2server 里是两个不同动作，一个撤销、一个接受）。本批的门两把都覆盖，行为缺陷单列待拍 |
| 旧账口径**订正**：`NOW()` 写进 TEXT 列**不报错**，touch 族真落笔 | 真库探针（`information_schema` 核列 + 事务内 `UPDATE` 后回滚） | §五 里那句「`NOW()` 写进 TEXT 列 79 处」原本按**可用性**缺陷记（预期撞 42804）。本轮探针证明 PostgreSQL 对赋值走 assignment I/O coercion，`NOW()` 写得进 TEXT、语句照落笔 ⇒ 那 79 处是**数据形状**缺陷（时间戳存成文本、精度/时区口径不保），不是「端点跑不通」。**这条订正直接决定本批门禁的定性**：touch 既然真改得动，未认证/他人请求就不是「只探到 500」而是**真的伪造了审计时间戳** |
| 尺子轴：**本批一字未动** | `scripts/audit_write_gating.mjs` | GET 无门禁 −16 全部来自代码。**R 轴同步受益 4 行**：`draft/start/{id}`、`handover/{id}/cancel`、`handover/{id}/process`、`mode/delete/{id}` 这 4 把的 **POST 别名**从 R7 搬进 R5——`write_routes_matrix.tsv` 恰 4 行改桶、0 行丢失、0 行新增，`write_routes_ungated.tsv` −4，真落笔 461→**457**。两条判据必须分开记账：GET −16 是本批修的漏洞，R −4 是同一把 handler 双方法注册的镜像读数，**不是**又发现了 4 条新漏洞消失（与批 17 的「−19 与 −8 同源」、批 18 的「−18 与 0 并存」构成第三种形状） |

### 三.20 第二十批（designer 的 GET 面 14 把门：13 归属 + 1 admin；尺子缺陷 24 与 25 两处判据修复）

| 项 | 位置 | 取证与口径 |
| --- | --- | --- |
| 覆盖面：14 把门 = 13 条 GET 注册 + 1 条 POST 别名 | `crates/query_assemble_designer/src/lib.rs:1403 / 1435 / 1645 / 1655 / 1665 / 1699`（6 把神经模型 generate / learn / stop×2 / reset / reset_p2）、`:3057`（`table/build/query/{query}`）、`:3246 / 3256 / 3293 / 3303`（`{flag}/status/{build,draft}` 与 `build/{flag}/{status}`、`draft/{flag}/{status}` 四把）、`:3348`（`table/query/{query}/build`）、`src/u2_closures.rs:2051`（`table/{flag}/build/dispatch`） | 修前这 13 条 GET 全落 K3（已认证、handler 不取会话、体内无门禁）：任何登录用户按 flag 就能把**他人**查询表 / 神经模型的 `status`（build / draft / generating / learning / idle）与 `reloaded` 位改掉——单据状态伪造，且 flag 可枚举。`neural/model/reset/{modelFlag}/{status}` 这把 POST 别名与 GET 那把共用 `_core`，门一起收，所以**门数 14 而 GET 行 13**，两张账不互抄 |
| 本 crate 的头两把门 helper | `src/lib.rs:30`（`require_row_owner`）、`:59`（`require_admin_opt`） | `require_row_owner` 按 `(table, key_col, key)` 三参通用化：读本行归属 ⇒ 非空走 `shared::middleware::require_owner`，**空归属与行不存在一律只允许 admin**（不把数据缺陷洗成权限放行）。两把都写成「无会话先 `Err(Unauthorized)`，位置在任何 `pool.get()` **之前**」⇒ 连不上库的 pool 也能证 401。批 17/18 收的是 admin 档，本批收归属档，差别在下面这行 |
| 档位取**归属**而非 admin 的依据 | `src/lib.rs:42-43`（`coalesce(nullif(trim(creator_person), ''), creator, '')`） | 本 crate 的既有约定就是「`creator_person` 优先、空则回退 `creator`」（`x_query_table` / `x_query_neural_model` 两表都有这两列，且既有写档 handler 按同一口径判归属）。这 14 把是**用户自己的设计器资产**的状态写，不是运维动作，收成 admin 会把正常用户的全族功能打死；批 17/18 那两批收 admin 是因为面本身就是运维/全表级动作 |
| 唯一的例外档位：`table/reload/dynamic` 收 admin | `src/lib.rs:3084`（门）＋ `:3088`（`UPDATE x_query_table SET reloaded = true … WHERE reloaded = false`） | 这条**无键**、按条件批量改写全表 ⇒ 没有"本行归属"可判，只能收 admin。它是本批唯一一把 admin 门，也是注入臂 E4 单独点名的那把 |
| 尺子缺陷 24（本批抓到并修）：限定路径调用点整条隐形 | `scripts/audit_write_gating.mjs` 的 `gateEvidence` / `writeEvidence` 调用点正则 + 新增 `callHelperBody` | `src/u2_closures.rs:2051` 那把门写成 `crate::require_row_owner(…)`，而两处 helper 追踪的正向查表正则写成 `(?<![.\w:])([a-z_]\w*)\s*\(`——前视断言把 `:` 也排除掉了，`crate::` / `self::` / `u2_render::` 这类**限定调用点一个都扫不到**。同批 lib.rs 里 13 把裸名调用正常进了间接门禁，这把没有 ⇒ 14 把门只作证 13 把，该行修后仍挂无门禁名单（K3→K2 的那一格，见 §四 本批读数）。**首版修法（取末段查「本 crate 首个同名」索引）被本批自己否掉**：personal crate 里 `icon::upload` 与 `signature::upload` 两把同名，first-wins 命中哪把纯看文件遍历顺序——那等于把缺陷 11 刚堵掉的碰撞面再开一次。终版按模块路径解析（剥 `crate`/`self`、`use x as y` 别名换算、首段是别的 crate 名就换根目录），且**静默**：查不到就不作证，也不写进 `route_handler_unresolved_refs.tsv`（调用点绝大多数属外部 crate/std，落盘会把未解析名单刷成噪音）。已知欠账：`super::f(..)` 需要调用方文件上下文才能定位父模块，这里不猜 |
| 尺子缺陷 25（本批抓到并修）：门埋在链上第二层时整条链被否 | `scripts/audit_write_gating.mjs:746` `gateEvidence` | 旧写法在递归下一层之前先要求**这一跳自己**「有拒绝能力」。现场是 surface 的 `attachment_u2b_preview_pdf` / `_preview_image_page` 经 `read_attachment_blob(..)` 调 `u2_gate_att_or_business_error(..).await?`（体内 `u2_check_owner` ＋ 拒时 `return Err(Forbidden)`），而那一跳本体只写 `NotFound` / `Internal` ⇒ 前置闸当场否掉。缺陷 24 把它们的写 SQL（`u2_render::general_file_store` 的 INSERT）露出来以后，这 2 行立刻被读成「无门禁行动面」——**尺子假阳**。同族还有 surface 10 条附件 POST/PUT 转发行与 meeting 2 条（门埋在 `u2_attachment_update_inner` → `u2_attachment_guard`）。修法：先递归拿下一层的作证结果，**作证链上有拒绝能力那一层的 helper 名**而不是转发跳的名字；这一跳自己有拒绝能力时仍按旧口径作标本跳名——这样「缺陷 20 的独立反证尺」（逐条要求被作证的 helper 体内确有拒绝能力或传播的门禁调用）继续成立，不会因放宽而失效。`nested === true`（体内只有门名的文本、没有传播的调用点）而本跳无拒绝能力 ⇒ 不作证，正是缺陷 20 注释里「读了标志却照样落笔」那一形 |
| 缺陷 25 的**反向见证**顺带量出一个真洞（待修，不在本批面） | `src/lib.rs:26178`（`attachment_u2b_html_to_pdf`）、`:26147`（`_html_to_image`）附近；尺子见证段「缺陷 25 反向见证」 | 这两把 POST **确实没有门**：`workHtml` 直接来自 body，本地渲染成 PDF/图片后经 `general_file_store` 往 `x_general_assemble_general_file` INSERT 一条——任意登录用户可批量造文件占库（存储投毒 + 无界 CPU）。它们被钉成「必须继续挂无门禁行动面」的反向见证：哪天链式追踪给它们"作证"出门来，就是恒真化被踩住的信号。定性待拍（admin 档 / 限流 / 只允许本人），已进 §五 名单 |
| 前端消费面证据 | `frontend/apps/desktop/src/views/QueryManagerDeep.vue:692-711`（`qdReads2`）、`:712` 之后 rev385 同类段；`QueryViewApp.vue:886 / 891` 是 `/0/…` 形状的契约探针 | 本族 14 把在前端**只有这个「深度探针页」**消费：整段 `Promise.all` + 每个请求 `catch(() => null)`，只统计「命中数」弹 toast，不参与任何业务流程；`QueryViewApp.vue` 那两条打的是 query_service_processing 的占位 id。⇒ 加门不会打断活的业务流；非属主用户在该页会看到命中数下降，属预期行为，不是回归 |
| 契约臂的适配：500→401，不改判据强度 | `src/tests.rs`（`test_v9_neural_model_reset_status_legacy_path_route_exists`） | 该臂原断言 `INTERNAL_SERVER_ERROR`——它钉的是「pool 连不上所以炸」，加了门之后无会话请求先 401、根本走不到读库。按批 17/18 的先例改成 `UNAUTHORIZED` 并在臂上写明理由，**没有**顺手放宽成「任一 4xx 都算过」 |
| 测试互相踩的修复（不是门禁失效） | `src/tests.rs:1198-1200`（seed 注释）与 `:1221 / :1233`（`reloaded = true`） | 首轮 owner 臂红过一格：`他人 403 侧不得落笔` 读到 `x_query_table/b20-tb-flag-own => Some("2026-10-14 23:06:21")`。成因是同 crate 的既有契约臂并行调**无键全表写** `table_reload_dynamic`（`WHERE reloaded = false`），把本臂哨兵行改掉了——测试互相踩，不是门漏。修法是把本臂 seed 行的 `reloaded` 置 `true`（落在它的射程外）并在 seed 上方把这段因果写在行上 |
| 与本 crate 旧注释的**口径冲突**（不平均，直接探针裁决） | `src/tests.rs:879` 附近（优化二轮 49 的断言注释）vs 本轮真库探针 | 该注释声称「`NOW()` 写进 TEXT 列必然 500」，与批 19 的订正（§三.19 那行）正面冲突。本轮在 PG 18.6 的事务内探针（`UPDATE … SET txt = NOW()` → `UPDATE 1`、`INSERT … VALUES ('2', NOW())` → `INSERT 0 1`，随后 `ROLLBACK`）证明**没有 42804**：赋值走 assignment I/O coercion。⇒ 批 19 的订正继续成立，那句注释的历史 500 更可能是同臂自己 `ALTER TABLE … ADD COLUMN IF NOT EXISTS` 之前的**缺列（42703）**族。两条账都记账、不折中 |


### 三.21 第二十一批（attendance 的 GET 面 9 把门：8 admin + 1 归属；42883 真缺陷、尺子缺陷 26 取证与一次注入残留）

| 项 | 位置 | 取证与口径 |
| --- | --- | --- |
| 覆盖面：9 把门 = 9 条 GET 注册（8 admin + 1 归属） | `crates/attendance_assemble_control/src/lib.rs:542`（`attendanceappealInfo_archive_id`）、`:959`（`attendancedetail_analyse_id_id`）、`:1019`（`attendancedetail_analyse_startDate_endDate`）、`:1046`（`attendancedetail_archive_id`）、`:1080`（`attendancedetail_checkDetailWithPersonByCycle_cycleYear_cycleMonth`）、`:2462`（`attendancesetting_enable_type`）、`:6854`（`dingding_sync_start`）、`:6996`（`qywx_sync_start`）、`:5076`（`v2_group_refresh_participate`，归属档） | 修前这 9 条全落 K3（已认证、handler 不取会话、体内无门禁）⇒ 任何登录用户按 id / `{startDate}/{endDate}` / `{cycleYear}/{cycleMonth}` 就能把**他人**申诉与考勤明细归档、把他人考勤组的 `update_time` 与参与人刷掉、把考勤全局开关置位，还能**拉起钉钉/企微全量同步**（最后这一条不是改一行数据，是驱动一条对外部 SaaS 的批量任务）。GET 名单跨批逐行对账：批 20 的 62 行 → 本批 53 行，diff 恰 9 行删除、**0 行新增**，9 行全属 `attendance_assemble_control`，该 crate 出列后现 **0 行** |
| 档位：8 把取 admin 的**当场依据** | `x_attendance_sync_record` 列清单（真库 `information_schema` 探针：id/type/status/exception_message/start_date/end_date/create_time/update_time，**无 creator 列**）；`attendancesetting_enable_type` 的行是全局配置不是用户资产 | 同步档那两把**没有"本行归属"可判**——落笔对象是一张无属主列的同步记录表 + 一次第三方调用；开关档那把改的是全企业生效的 `enabled`。归档 / 分析这 4 把收 admin 的口径是「archive / analyse 属管理侧动作，且与 `attendancedetail_delete` 那把既有**归属档**同表不同档」⇒ **一表两档**是本批主动制造的分歧，进「新增待拍」第 1 格，不假装它已由权威定过 |
| 归属档那把的形态：会话前置 → 读归属 → 缺行回业务错 → 基座判 | `:5076`（`shared::middleware::require_owner`）＋ 其上方 `let Some(sess) = session.as_deref() else { return Err(AppError::Unauthorized); }` 与 `:5063-5075`（`SELECT creator_person FROM x_attendance_v2_group WHERE id = $1` → `Option<String>` → `unwrap_or_default()`） | 顺序钉死：无会话在任何 `pool.get()` **之前**回 401；行不存在回 `ActionResult::error("attendance group not found")`（业务错，不是权限错——不为「探测 id 是否存在」提供 oracle）；归属基座 `require_owner` **内含管理员放行支路**，所以这把同时覆盖 owner 与 admin 两侧，臂里分别量。**口径差异要记明**：本把只读 `creator_person` 单列、**不**做批 20 designer 那样的 `coalesce(creator_person, creator)` ⇒ 空归属行落进「非 admin 一律 403」，效果上等价于 admin-only，但**成因是列口径而非显式收口**，是否统一到 coalesce 进「新增待拍」第 4 格 |
| 本 crate 第一把门 helper：`require_admin_opt`（Option 版） | `src/lib.rs:4216` | 写成 `Option<&Session>` 而不是裸 `Extension<Session>` 的理由是**取证可行性**：裸 `Extension` 在会话缺失时由 axum 抛 500，「没门」与「门插在读库之后」两种成因读起来一模一样；`Option` 版让无会话请求先 401，于是可以用一张**连不上库的 pool**（`Manager::new(Config::new(), NoTls)`）离线作证 9 把逐把回 401，不需要真库。批内另有既有 `require_admin(pool, &session)`，这把只做「取会话」那一层，admin 判定复用原函数，不复制第二套判定逻辑 |
| 真缺陷：**`attendancedetail_checkDetailWithPersonByCycle` 必然 500**（42883） | `:1080` 那把的门 + 其体内 SQL；真库列型 `x_attendance_detail.cycle_year / cycle_month` 为 **integer**，handler 绑的是路径 `String` | PostgreSQL 报 `operator does not exist: integer = text`（42883）⇒ 这条端点加门前后的落笔都是 0，纯粹因为先炸在类型上。**这不是可用性缺陷被忽略**：门覆盖它、`untouched()` 哨兵里它永远 `false`，本批用「明列豁免」把它钉住——admin 段允许它 `Internal`，且失败消息把这把点名写死（`security_gate_tests.rs:446-458`），哪天类型口径修好、它开始真落笔，豁免断言会自己喊红，不会静默变成「它一直没风险」 |
| 尺子缺陷 26（本批抓到，**只记账、尺子未动**）：`gateEvidence` 判门禁时不剥注释 | `scripts/audit_write_gating.mjs:748`（`if (GATE_RE.test(body)) return true;`，`body` 是整段函数体文本，含 `//` 行） | 现场：注入臂 W1 把 `dingding_sync_start` 的门**摘掉**之后，尺子读数里这行**没有回到 GET 名单**——因为本批为这把写的 WHY 注释里出现了 `require_admin` 字面量，`GATE_RE` 把它当成门作证了。反证臂 W1R2 换成「体内摘门 + 注释不含门名」的那把（`attendancesetting_enable_type`），该行立刻回名单（attendance GET 行数 0→1）⇒ **同一形状，两臂读数相反**，缺陷坐实。它属**假阴方向**（把没门的读成有门），比批 20 的缺陷 24/25（假阳）更危险，因为它会让人以为已经修过。本批的**临时**处置是不改尺子、改说明文：lib.rs 里两处 WHY 注释改写掉字面门名（改写后新增注释行含 `GATE_RE` 词元数 **0**，且重跑尺子读数与产物 md5 与改写前**逐字节全等**，T2 = T3 ⇒ 本批读数无一格靠注释撑着）。**修复轴留下一批**，暴露面未普查：全仓约 120 行注释提到门名、其中缩进态约 40 行（多在测试文件），真实射程待尺子补「剥注释」判据后再量 |
| 注入取证的一次**残留事件**（如实披露，已恢复） | `/d/tmp/b17arms/inject_arm.mjs`（harness）与 `crates/.../src/lib.rs`（当时的磁盘态） | 首轮套件里 W1 被判「恒真」，harness 在 `try` 块内直接 `process.exit(4)` ⇒ **`finally` 的复原没跑**，摘掉的门留在磁盘上；随后 4 臂全部以「baseline not green」中止，中止消息 `dingding_sync_start => Some(Internal)` / `=> None` 正是暴露点——**抓出残留的是下一臂的基线，不是自我报告**。处置：按原行重插该门（8 处 admin 调用点逐点数过，`git diff` 只有插入、0 删除），并把 harness 改成「先记判定码，`finally` 无条件复原」后**整套 8 臂重跑**，本批注入表用的是重跑读数。教训两条：① 判定/中止路径也必须走 `finally`；② 脚本自报「已复原」不可信，磁盘真相由外部复验 |
| harness 的第二条缺陷（本轮**复验实际未生效**） | `inject_arm.mjs` 的 `restore()` 里 `git status --porcelain -- <target>`；日志每臂 RESTORE 行都是 `git_status=[fatal: not a git repository …]` | 脚本在非仓库 cwd（`/d/tmp/b17arms`）下起子进程，`git status` 当场 fatal、只把那串 fatal 文本打进日志，**没有断言它成功**⇒ 本轮的「外部磁盘复验」这一路是空的，磁盘真相实际只由 `hash_ok=true`（写回后读回比原始字节哈希）撑住。补法两条都已做：收尾在仓库根跑 `git status --porcelain`（结果为**空**，见本节门禁段），并把这条 harness 缺陷记进「新增待修」——下批注入前先把 git 复验的 cwd 固定到仓库根并断言 exit=0 |
| 契约臂的适配：按**逐条点名 expect**，不放宽判据 | `src/tests_u2_legacy.rs`（`u2l_dingding_core_routes_reachable`、`u2l_qywx_routes_reachable`）、`src/tests_u2.rs`（`u2_v2_group_static_paths_win_over_param_id`、`u2_verb_chains_align_with_legacy_methods`） | 加门后这 4 支既有臂原断言 500（「进到 handler 但 pool 连不上」）会红。改法是每条目带自己的 `expect`：被本批覆盖的 `sync/*/start` 与 `setting/enable/type` 钉 `UNAUTHORIZED`，其余仍钉 `INTERNAL_SERVER_ERROR`。刻意**没有**写成 `is_routed(500|401)` 这种「任一都算命中路由」的 helper——那样门哪天掉下去，路由可达臂照样绿，等于把门禁回归塞进自己的哨兵里（沿用批 15/17/18 先例） |
| 造数面的既有缺陷：三表**无主键** ⇒ `ON CONFLICT (id)` 撞 42P10 | `migrations/032_create_assemble_control_tables.sql`（`x_attendance_appeal_info` / `x_attendance_detail` / `x_attendance_setting` 均 `"id" TEXT` 且**无 PRIMARY KEY**）；`src/security_gate_tests.rs` 的 `seed()` 注释 | 首轮真库臂红在 42P10 `there is no unique or exclusion constraint matching the ON CONFLICT specification`。测试侧改法是 `unseed()` 再普通 INSERT（幂等仍成立，连跑两遍读数一致）；**产品侧缺陷照记**：这三张表按 id `UPDATE` 无法保证只改一行，重复 id 会让「归档某一条」写成「归档若干条」，与批 19/20 的「缺列 42703」族同属 schema 缺陷，进「新增待修」 |
| 定性**订正**（批 20 §五 那条「新增待修」读数的两处说过头） | `crates/processplatform_assemble_surface/src/lib.rs:26178`（`attachment_u2b_html_to_pdf`）、`:26147`（`_html_to_image`）；全仓 `crates/` 内 `DefaultBodyLimit` / `RequestBodyLimit` / `LimitLayer` 命中数 **0** | ① 这两把**不是 GET 面**，尺子矩阵里它们是 **POST、R6「已认证 + 读会话无门禁」**，不该占用 GET 的选题口径；② 它们经 `general_file_store(&pool, &session.person_unique, …)` 落的 `x_general_assemble_general_file` 行，`creator` 就是**调用者本人**，属「本人名的存储写」，不是伪造他人归属；③ 渲染 CPU 面受 axum 默认 2 MiB body 上限约束（无任何一处覆写）⇒「无界」这一半不成立。修正后的定性是**限流 / 配额族**（与 A1 真匿名写 9 条同族）：门仍然没收，仍然在 R6 待修名单，只是危害等级要按新口径重记。**批 20 那格的原文不回改**，以本行作订正链 |

### 三.22 修复批 R1（闭合 §三.21「新增待修」3/4/5：require_owner 归一化、42883、考勤三表主键）

本批**不新增任何路由门禁**，GET 写面暴露面不变，未重跑尺子。只把 §三.21「新增待修」里已取证的三个真缺陷连同测试基建一并修掉并验证；明细见独立小结 `docs/audits/2026-10-15-round4-rfixes-report.md`。

| 项 | 位置 | 修法与口径 |
| --- | --- | --- |
| 缺陷 1：`require_owner` 归一化不一致 | `crates/shared/src/middleware/rbac.rs::require_owner` | 两侧统一 `trim().to_lowercase()` 后比较（`admin` 放行支路不变）。同一身份的大小写/空白变体视为本人；归一化后确为他者仍 `Forbidden`，不放松越权。attendance 侧 `mobile_check_impl` / `v2_mobile_check_from_out` / `import_checkin_rows` 同步改拿 `normalize_key(&session.person_unique)` 再比，消除「归一化输入 vs 原始会话」的歧义 |
| 缺陷 2：42883（integer 列绑 text 路径参数） | `crates/attendance_assemble_control/src/lib.rs::attendancedetail_checkDetailWithPersonByCycle_cycleYear_cycleMonth` | 路径绑定 `Path<(String,String)>` → `Path<(i32,i32)>`，按真库 integer 口径绑定；非数字路径由 axum 400 拒（比暗到 500 更正确）。按 §三.21 要求**同步撤销** `security_gate_tests.rs` 里对该把的明列豁免：admin 段要求 9 把全放行，落笔回读 `checked` 由期望 `false` 改 `true`（造数行 9999-99 在射程内，UPDATE 真落笔） |
| 缺陷 3：三张考勤表无主键 | 新增 `migrations/109_add_attendance_pk.sql`(+ `_rollback.sql`) | 逐表补 `id` 空值→uuid、自连接删重复 id→`SET NOT NULL`→`ADD PRIMARY KEY (id)`。已对测试库应用并验证：三表均 `xxx_pkey`、`id` NOT NULL、`schema_migrations` 命中 109 |
| 测试基建 | `crates/shared/src/testing.rs` | `test_pool()` / `test_sea_orm_pool()` 支持 `DATABASE_URL` 覆盖（缺省回退默认 5432，CI 不受影响）。背景：Windows 主机 Docker 原生 `5432:5432` 端口代理不通，改用 `15432:5432` 发布 + `DATABASE_URL` 直连，弃用先前不稳的 Node 双栈代理 |
| 验证 | — | attendance crate `45 passed / 0 failed / 15 ignored`（含新回归 `r1_require_owner_case_insensitive_self_match`、`r3_attendance_tables_have_primary_key` 与更新后的 `b21_*` 两臂）；`require_owner` 全消费方对真实库门禁测试 **325 passed / 0 failed**（calendar / meeting / mind / ai / file / general + attendance）；全 workspace 96 crate 编译通过。整跑 `--lib` 受本机 Windows 构建资源抖动（`os error 5` / `os error 1455`）干扰，已用「全量编译 + 定向门禁测试全绿」作验收证据，环境噪声如实记账 |

### 三.23 第二十三批（org 导出写面收 admin 门 + auth 6 条定性）

本批两件事：①把 GET 写面队列里 `organization_assemble_control` 的两把**导出**写面收 admin 门（真实代码变更）；②对队列里 `auth` 那 6 条做定性（多为已验证的匿名回调流，非简单 IDOR，按账本「加门与验签分开记」的口径处置）。

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| org 导出两把收 admin 门 | `crates/organization_assemble_control/src/lib.rs` 的 `export_export_all`（`/export/export/all`）、`export_zhengwudingding_person`（`/export/zhengwudingding/person`）；新增 `u2_helpers::require_admin_opt` | 修前两把不取会话、无门禁，任意登录用户可触发全量组织/人员数据导出（往 `x_org_export` 建 `type='all'` / `'zhengwudingding'` 导出任务）⇒ 收 admin 门。用 `Option<Extension<Session>>` + `require_admin_opt`（对齐批 21 先例）：无会话 401、非 admin 403、admin 放行。回归 `org_export_writes_require_admin`：无会话离线臂两把全 401；真库臂非 admin 403 且不落笔、admin 放行且各落 1 行、收口零残留 |
| auth 6 条定性（多为假阳，非简单 IDOR） | `crates/auth/src/oauth.rs` / `oidc.rs` / `qiyeweixin.rs` | `oauth_login_qywx` / `oauth_login_dingding` / `oauth_login_name` / `oauth_bind_name` 四把：**已有** `validate_state`（CSRF）+ 可选 PKCE，且 code 经服务端与 app_secret 交换 ⇒ 合法匿名登录流，写面（建用户/会话）即登录本身，尺子只因其体含 INSERT 且 `GATE_RE` 不认 `validate_state` 而误计 K1。`oidc_callback`：JWT 经 JWKS RS256 验签 + iss/aud/exp 校验（验签在），但**未校验 `state`**（登录 CSRF 缺口）——此为「验签缺失仍在」一族，**分开记账**（见下）。`qiyeweixin_update_person_detail`：code 服务端交换（app_secret），且只按 `unique_id = qywxpos_{userid}` 更新对应本人，非 IDOR |
| 新增待修（本批点名） | `crates/auth/src/oidc.rs::oidc_callback` | **OIDC 回调未校验 `state`**（登录 CSRF / session 固定风险）。`oidc_authorize` 目前透传客户端 state、不落服务端 state，故回调无从校验。修法：比照 oauth 的服务端 `generate_state`/`validate_state`，`oidc_authorize` 生成并存储服务端 state 放入授权 URL，`oidc_callback` 校验。**本批未改**（改 authorize 的 state 语义属登录流行为变更，需端到端联调 IdP，单独一批），先记账 |
| 验证（本批代码变更） | — | `organization_assemble_control` `151 passed / 0 failed`（含新回归臂；无会话离线臂 401 绿）。真库臂（admin 落笔回读）因本机 Docker 守护进程不可达（`wsl.exe` 被沙箱策略拦截、Docker Desktop 无法在当前会话重启）而**跳过**——`require_admin_opt` 门已在批 21 对真库取证，`is_admin` fail-closed 令非 admin 恒 403（既有 `org_structure_writes_forbidden_for_non_admin` 离线即证），admin 放行落笔路径为确定性的同一 helper，判定风险低；真库臂待 Docker 重启后补跑 |

### 三.24 第二十四批（calendar / jpush 的 update_control_config 收 admin 门）

GET 写面队列里 `calendar_assemble_control` 与 `jpush_assemble_control` 各有一把 `update_control/config`（模块级 update_control_config 家族 A3-N11 的两格）。修前两把均不取会话、无门禁：

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| calendar 全局配置写 | `crates/calendar_assemble_control/src/lib.rs::update_control_config`（`/api/calendar_assemble_control/update/control/config`，GET） | 往 `cal_control_config` 写全局开关（enabled / 时区 / 共享），任一登录用户可改 ⇒ 收 admin 门：`Option<Extension<Session>>` + `is_admin`（无会话 401 / 非 admin 403）。回归：无会话 HTTP 臂 401、非 admin 直接调用 403（`is_admin` fail-closed） |
| jpush 推送广播写 | `crates/jpush_assemble_control/src/lib.rs::update_control_config`（GET 与 POST 双注册） | 往 `x_jpush` 建 `target='all'`、`creator` 硬编码 `system` 的广播，任一登录用户可触发全员推送 ⇒ 收 admin 门（同上口径）。回归：无会话 401、非 admin 403 |
| 契约臂返工 | 两 crate `src/tests.rs::test_update_control_config_route` | 原断言 500（无门 + 无库）。加门后无会话先于读库回 401，断言改为 `UNAUTHORIZED`（属「契约臂钉 500」族 272 处之一，行为真变、逐条钉期望，不放宽判据） |
| 验证 | — | `calendar_assemble_control` `34 passed / 0 failed`、`jpush_assemble_control` `28 passed / 0 failed`（mock pool，无需真库：无会话 401、非 admin 403 均由门 + `is_admin` fail-closed 离线即证）。admin 放行落笔臂待 Docker 重启补跑 |

### 三.25 第二十五批（query_assemble_surface 两把 GET 写面收 admin 门）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| table 批量 reload | `crates/query_assemble_surface/src/lib.rs::table_reload_dynamic`（`/api/queryview/table/reload/dynamic`，GET） | `UPDATE x_query_table SET reloaded=true WHERE reloaded=false` 批量维护写，任一登录用户可触发 ⇒ 收 admin 门：`Option<Extension<Session>>` + `is_admin`（无会话 401 / 非 admin 403） |
| importmodel 重执行 | `crates/query_assemble_surface/src/u2_closures.rs::importmodel_reexecute_record`（`/api/queryview/importmodel/execute/record/{recordId}`，GET） | 重置并新建导入模型执行记录（真实写库）属设计/维护写面 ⇒ 收 admin 门（用本 crate 既有 `require_admin_gate`，无会话 401 / 非 admin 403） |
| 契约臂返工 | `src/tests.rs::test_u2_importmodel_reexecute_record_route_exists` | 原断言 500（无门 + 无库）。加门后无会话先于读库回 401，改 `UNAUTHORIZED`；新增 `test_query_get_writes_non_admin_forbidden`（两把非 admin 403） |
| 验证 | — | `query_assemble_surface` `48 passed / 0 failed`（mock pool 离线即证 401/403）。admin 放行落笔臂待 Docker 重启补跑 |

### 三.26 第二十六批（ai_assemble_control 4 把 GET 写面：2 归属 + 1 admin + 1 既有门重构）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| file 删除 | `crates/ai_assemble_control/src/lib.rs::file_delete_flag`（`/api/ai_assemble_control/file/delete/{flag}`，GET） | `DELETE FROM x_ai_file WHERE id=$1` 无归属门，任意登录用户可删他人文件 ⇒ 收归属门：先 `SELECT creator`，`require_owner`（admin 放行 / 本人放行，归一化），行不存在回 `file not found` |
| index 删除 | `::index_delete_flag`（`/api/ai_assemble_control/index/delete/{flag}`，GET） | `DELETE FROM x_ai_index WHERE id=$1` 无归属门 ⇒ 同上收归属门（`x_ai_index.creator`） |
| index 批量同步 | `::index_sync_to_knowledge`（`/api/ai_assemble_control/index/sync/to/knowledge`，GET） | `UPDATE x_ai_index SET synced=TRUE WHERE enabled AND NOT synced` 批量维护写 ⇒ 收 admin 门（`require_admin`） |
| chat 线索删除 | `::chat_delete_clue_id`（`/api/ai_assemble_control/chat/delete/{clueId}`，GET） | 本有归属门但用**原始串比较** `user_id != session.person_unique`（无 admin 旁路、大小写变体误拒）⇒ 重构为 `require_owner`（归一化 + admin 旁路），并改 `Option<Session>` 令无会话 401（原裸 `Extension<Session>` 无会话 500） |
| 验证 | — | `ai_assemble_control` `54 passed / 0 failed`（含真库臂 `test_ai_get_writes_owner_admin_gate`：无会话四把全 401；非属主/非 admin 403 且不落笔；属主放行 file/index 真删、chat 软删；admin 旁路放行 + index_sync 跑通；收口零残留）。`test_u2_index_sync_no_json_body_required` 断言 500→401（行为真变）。尺子 GET 写面 44→40 |

### 三.27 第二十七批（file_assemble_control 2 把 GET 写面：1 admin + 1 归属）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| 批量清理文件 | `crates/file_assemble_control/src/lib.rs::file_clean_unused_referencetype_cmsdocument_manage`（`/api/file/clean/unused/referencetype/cmsdocument/manage`，GET） | `DELETE FROM file_file WHERE reference_type='cmsdocument_manage'` 批量删属管理维护写面 ⇒ 收 admin 门（无会话 401 / 非 admin 403） |
| 复制附件 | `::file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale`（GET，两条重复路径映射同一 handler） | 读源 FILE_FILE 行并在 FILE_FILE 新建一行（继承源 `person`）⇒ 收归属门：先取源 `person` 再 `require_owner`（admin 放行 / 本人放行），源不存在回 `attachment not found` |
| 验证 | — | `file_assemble_control` `92 passed / 0 failed`（含真库臂 `test_file_get_writes_owner_admin_gate`：无会话两把 401；非 admin 403 不删、非属主 403 不建复制行；属主 copy 放行建 1 行；admin clean_unused 删 cmsdocument_manage 行 + admin copy 放行；收口零残留）。尺子 GET 写面 40→37 |

### 三.28 第二十八批（hotpic_assemble_control 2 把 GET 写面收归属门）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| 改热图标题 | `crates/hotpic_assemble_control/src/lib.rs::user_hotpic_changeTitle`（`/api/hotpic_assemble_control/user/hotpic/changeTitle`，GET） | `UPDATE x_hotpic SET title=$1 WHERE id=$2` 按 id 改、无归属门 ⇒ 收归属门：先 `SELECT creator` 再 `require_owner`（admin 放行 / 本人放行），行不存在回 `hotpic not found` |
| 删热图 | `::user_hotpic_id`（`/api/hotpic_assemble_control/user/hotpic/{id}` 与 `/api/hotpic/assemble/control/user/hotpic/{id}` 两条重复路径，GET，handler 内含 DELETE 分支） | DELETE 分支 `UPDATE x_hotpic SET deleted_at=NOW() WHERE id=$1` 无归属门 ⇒ 收归属门（同口径）；GET/读分支不设门。**关键顺序**：`pool.get()` 移到会话门**之后**（原先在读库之前，无会话会先炸 Internal 而非 401——「门插在读库之后」） |
| 验证 | — | `hotpic_assemble_control` `31 passed / 0 failed`（含真库臂 `hotpic_user_writes_require_owner`：无会话两把 401；非属主 403 不改标题不软删；属主改标题 + 软删放行；admin 旁路放行；收口零残留）。尺子 GET 写面 37→34 |

### 三.29 第二十九批（jpush_assemble_control 3 把 GET 写面：2 admin + 1 归属）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| 广播推送创建 | `crates/jpush_assemble_control/src/lib.rs::create_jpush`（`/api/jpush_assemble_control/create/jpush`，GET） | `INSERT INTO x_jpush` 建广播（`creator` 硬编码 `system`），任意登录用户可触发全员推送 ⇒ 收 admin 门 |
| 批量解绑 | `::device_admin_unbind_all_person`（`/api/jpush/assemble/control/device/admin/unbind/all/person`，GET） | 按 body `personId` `DELETE FROM x_jpush WHERE creator=$1`，任意用户可删他人推送绑定 ⇒ 收 admin 门（名字即 admin） |
| 解绑设备 | `::device_unbind_new_deviceName_deviceType_pushType`（`/api/jpush_assemble_control/device/unbind/new/{deviceName}/{deviceType}/{pushType}`，GET） | 按 (deviceName,deviceType,pushType) `UPDATE x_jpush SET deleted_at`，**WHERE 无 creator** ⇒ 任意用户可按设备名解绑他人设备 ⇒ 收归属门：先取设备绑定 `creator` 再 `require_owner` |
| 验证 | — | `jpush_assemble_control` `29 passed / 0 failed`（含真库臂 `test_jpush_get_writes_owner_admin_gate`：无会话三把 401；非 admin 403 不落笔、非属主 403 不解绑；属主解绑软删放行；admin 建广播 + 批量解绑放行；收口零残留）。`test_jpush_create_legacy_path` 断言 500→401（行为真变）。尺子 GET 写面 34→31 |

### 三.30 第三十批（cms 浏览量自增收会话门 + pp_designer 3 把 process 生命周期收归属门）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| cms 浏览量自增 | `crates/cms_assemble_control/src/lib.rs::document_id_view_count`（`/api/document/{id}/view/count`，GET） | `UPDATE x_cms_document SET view_count = view_count + 1 WHERE id=$1` 无会话 ⇒ 收有效会话门（`Option<Session>`，无会话 401），防匿名刷量 |
| pp process 禁用 | `crates/processplatform_assemble_designer/src/lib.rs::process_id_disable`（GET，两条路径） | `UPDATE PP_E_PROCESS SET xstatus='disabled' WHERE xid=$1` 无归属门 ⇒ 收本 crate `guard_write`（`COALESCE(xcreatorPerson, creator_person)` 属主或 admin） |
| pp process 启用 | `::process_id_enable`（GET，两条路径） | 同上收 `guard_write` |
| pp process 升级 | `::process_id_upgrade`（GET） | `UPDATE PP_E_PROCESS SET xversion+1 WHERE xid=$1` 无归属门 ⇒ 同上收 `guard_write` |
| 验证 | — | `cms_assemble_control` 编译通过（view_count 契约臂 `!= NOT_FOUND`，401≠404 仍绿）；`processplatform_assemble_designer` `29 passed / 0 failed`（含真库臂 `test_pp_process_writes_require_owner`：非属主 403 不 bump、属主 disable+upgrade 放行、admin enable 放行、收口零残留）。尺子 GET 写面 31→26 |

**第三十批收口后 GET 写面队列余 26 条的性质（已逐 crate 核实）**：多为**假阳或非简单 IDOR**，非「写 by id 无会话」型真漏洞——`auth` 6（OAuth 回调，多为已验证匿名流，oidc 缺 state 已记账）、`calendar_assemble_control` 1（follow/cancel，写按 person=session 自限定）、`cms_assemble_control` 5（commend/uncommend，person_id=session 自限定）、`message_assemble_communicate` 2（is_conversation_member + person=session 自限定）、`mind_assemble_control` 1（restore，已有 person_can_manage_mind 门）、`program_center` 2（captcha/code，登录流匿名设计）、`program_init` 1（secret/set/cancel，有意匿名）、`meeting_assemble_control` 4（accept/checkin/manual_completed/reject，属账本「待拍」产品档位）、`organization_assemble_authentication` 2（qiyeweixin bind / oauth auth 回调，需同 auth 族定性）、`message` 1（consume 按 id 标记已消费，低危语义含混）、`general_assemble_control` 1（qrcode 生成，疑只读）。真漏洞型（写 by id、无会话）已在前 30 批收口。

### 三.31 第三十一批（尺子自限定识别：GATE_RE 补门助手 + 自限定判据，尺子缺陷 26 族）

纯尺子轴，零产品码。把「写 SQL 把会话身份绑进 person 列」的自限定形状认成门，出清一批假阳：

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| GATE_RE 补两个门助手 | `scripts/audit_write_gating.mjs::GATE_RE` | 补 `is_conversation_member`（msg_communicate 的成员门）、`person_can_manage_mind`（mind 的属主/管理门）——两者都是明确的 owner/member 判定入口 |
| 自限定判据 | 新增 `SELF_SCOPE_RE` + `SELF_SCOPE_SQL_RE`，接入 `gateEvidence` | 命中条件：体内既有 `&session.person_unique`（会话身份被绑定），又有「`person`/`person_id`/`user_id` 列 = 占位符」的写 SQL ⇒ 只改/建自己名下的行，对 IDOR 等价归属门。**只认 person 三列、不认 creator**：`SET creator=$session` 是按 id 更新的归属标注（非自限定），认了会把「按 id 改任意行」的 IDOR 误判成有门（假阴更危险）。反向见证内置：仅当日志、写 SQL 无 person 列不得命中 |
| 读数 | — | GET 写面 26→**19**（出清 calendar_follow_cancel、cms commend/uncommend×3、msg_communicate quit/self + single_delete_virtual、mind_restore 共 7 条假阳）；R6→R5 自限定重分类 24 条（POST/PUT/DELETE 面，spot-check `file recycle_empty` 确为 `WHERE person=$session` 自限定）；判据见证缺口 0 |
| 余 19 条性质 | — | auth 6（回调，验签族）、cms 2（view_count 计数器 + comment_commend_u3 自限定 INSERT，均合法已认证写）、general 1（qrcode 疑只读）、meeting 4（待拍产品档位）、message 1（consume 低危）、org_auth 2（回调验签族）、program_center 2（captcha/code 登录流匿名设计）、program_init 1（secret/set/cancel 有意匿名）。真 IDOR 漏洞型已全清 |

### 三.32 第三十二批（回调验签族：oidc state 校验 + 尺子认 validate_state）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| oidc state 校验（产品码） | `crates/auth/src/oauth.rs`（`generate_state`/`validate_state` 改 `pub(crate)`）、`crates/auth/src/oidc.rs::oidc_authorize` / `oidc_callback` | 修前 `oidc_authorize` 透传客户端 state、服务端不落，`oidc_callback` 不校验 state ⇒ 登录 CSRF / session 固定风险。修法（对齐 oauth 族）：`oidc_authorize` 生成并登记服务端 state 放入授权 URL，`oidc_callback` 先 `validate_state` 再 `exchange_code`，无效/过期 state 回 400 |
| 尺子认 validate_state | `scripts/audit_write_gating.mjs::GATE_RE` | 补 `validate_state`（OAuth/OIDC 回调的 CSRF 门）——此前 oauth_login_* 四把已用 validate_state 却因词表不认而挂 K1 假阳 |
| 验证 | — | auth `99 passed / 0 failed`（含新回归 `test_oidc_callback_state_is_validated`：state 一次性、未登记即拒）。尺子 GET 写面 19→**14**（auth 出列 5：oauth_login_qywx/dingding/name/bind + oidc_callback；余 auth 1 = qiyeweixin_update_person_detail，code 服务端交换且只改 unique_id=qywxpos_{userid} 本人，自限定-by-code，非 IDOR） |

**第三十二批收口后 GET 写面队列余 14 条**：auth 1（qiyeweixin_update_person_detail，code 自限定）、cms 2（view_count 计数器 + comment_commend_u3 自限定 INSERT，合法已认证写）、general 1（qrcode 疑只读）、meeting 4（待拍产品档位）、message 1（consume 低危）、org_auth 2（回调，待验签族收口）、program_center 2（captcha/code 登录流匿名设计）、program_init 1（secret/set/cancel 有意匿名）。

### 三.33 第三十三批（meeting 会议级状态翻转收归属门：收口 §三.21 待拍）

**产品档位拍板**（账本「meeting_id_accept 谁能做」待拍）：会议级状态翻转（accept/reject/manual completed）统一归属**会议属主或 admin**（`u2_meeting_owner_gate`，读 `x_meeting.creator`）。被邀请人的「接受邀请」语义若需保留，应在 `x_meeting_invite` 层做，不属本批会议级状态翻转。

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| accept | `crates/meeting_assemble_control/src/lib.rs::meeting_id_accept` | 修前仅要求有效会话、按 id 翻 `x_meeting.status` 无归属判定 ⇒ 加 `u2_meeting_owner_gate`（属主或 admin） |
| reject | `::meeting_id_reject` | 修前**不取会话**、无门禁 ⇒ 改 `Option<Session>`（无会话 401）+ `u2_meeting_owner_gate` |
| manual completed | `::meeting_id_manual_completed` | 修前**不取会话**、无门禁 ⇒ 同上 |
| checkin（假阳） | `::u2_meeting_checkin_get` | 写 `x_meeting_checkin` 时 `person = session.person_unique`（INSERT 自限定）⇒ 安全；尺子不认 INSERT 自限定（`person` 列可能绑请求体，扩词表会引入假阴），留作已定性假阳 |
| 验证 | — | meeting `security_gate_tests` 的 `meeting_state_writes_forbidden_for_non_owner_non_admin` 扩为 7 例（save/confirm_allow/confirm_deny/add_invite + accept/reject/manual_completed），自包含造数后 `--ignored` 真库跑 **1 passed / 0 failed**（非属主 7 把全 403）。尺子 GET 写面 14→**11**（meeting 出列 3；余 checkin 假阳） |

**注**：meeting 另有两条**先于本批存在**的环境相关失败（已 stash 复验与本次改动无关）：`u2_persist_blob_verified_fails_loud_without_backend`（`STORAGE_BACKEND` 有文件系统回退 + `is_db_available()` 在本沙箱返回 false）、`test_building_list/room_list_returns_internal_error_without_db`（`#[ignore]` 钉 500、DB 可达即漂移）——属账本「`assert_eq!(status,500)` 断言族」环境漂移，非本批引入。

### 三.34 第三十四批（crud 归属版收口 A3-N9 + 尺子认 crud 归属变体 + 修批 32 遗留的 A1 名单）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| crud 归属版（产品码） | `crates/shared/src/crud.rs` 新增 `crud_save_owned` / `crud_delete_owned`（+ 私有 `crud_owner_ok`）；`crates/shared/src/lib.rs` 补 re-export | A3-N9：通用 `crud_save`/`crud_delete` 只按 id 落笔、无 owner 谓词，任意登录用户可改/删他人行。归属版先按 `owner_column` 取属主、`require_owner`（admin 放行 / 本人放行，归一化），行不存在回 false（保 not-found）。`owner_column` 由调用方静态字面量传入（与 table/columns 同口径） |
| 首批迁移：x_person_face 生物特征 | `crates/personal_extend/src/personal.rs::face_save` / `face_delete` | 改 `Option<Session>`（无会话 401）+ `crud_save_owned`/`crud_delete_owned`（owner_column=`person_id`）——修「他人 x_person_face 行的增改软删」 |
| 尺子认 crud 归属变体 | `scripts/audit_write_gating.mjs::GATE_RE` | 补 `crud_save_owned`/`crud_delete_owned`（跨 crate helper，`gateEvidence` 不跟进跨 crate，故须点名）——face 4 条路由 R6→R5 |
| 修批 32 遗留的 A1 名单（尺子自证） | `A1_UNCONDITIONAL_WRITE` 名单 | 批 32 给 GATE_RE 补 `validate_state` 后，OAuth 三方登录 4 条（有 state CSRF 门）离开 A1，硬编码名单未同步 ⇒ 尺子 `A1 名单不一致` 退 exit 1（批 32 提交时该自证被 `| grep` 掩盖未察觉）。本批移除这 4 条（9→5），尺子复归 EXIT 0 |
| 验证 | — | `personal_extend` `19 passed / 0 failed`（含真库臂 `face_writes_require_owner`：无会话 401、非属主 403 不落笔、属主 save+delete 放行、admin 旁路放行、收口零残留）。尺子 EXIT 0、A1 实测 5=名单 5、判据见证缺口 0；face 4 条 R6→R5 |

**注**：A3-N9 波及约 12 crate，本批先做 shared 归属版能力 + 迁移最敏感的 `x_person_face`（生物特征）；其余 crate（bbs/cms/console/general/org/search/program_center/query/portal/pp_designer/ai）的 crud_save/delete 迁移留下一批（各自判定 owner_column）。

### 三.35 第三十五批（`row_to_json` 类型覆盖补全：JSONB / JSON / 时间 / UUID 静默丢列）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| 类型覆盖补全（产品码） | `crates/shared/src/response.rs::row_to_json` | 原匹配臂只覆盖 BOOL / INT2·4·8 / FLOAT4·8 / TEXT / VARCHAR / NAME / BPCHAR，JSONB / JSON / DATE / TIMESTAMPTZ / NUMERIC 一律走 `_ => None` **静默丢列**（连键都不出现，与 NULL 列同形）。本批补三族：JSON·JSONB 原样透传为 `serde_json::Value`；UUID 转规范小写连字符字符串；TIMESTAMP / TIMESTAMPTZ / DATE / TIME 字符串化，口径对齐 `::text`（timestamp 无时区、timestamptz 带 `+00`、date `YYYY-MM-DD`、time `HH:MM:SS[.fff]`，`%.f` 无小数位时连点省略）。NULL 列与仍未知类型保持省略键（o2server/Gson 语义） |
| 特性开关（构建面） | `Cargo.toml`（`[workspace.dependencies]` 新增 `postgres-types` 条目）+ `crates/shared/Cargo.toml`（`postgres-types.workspace = true`） | 原先 `tokio-postgres` 的 workspace 特性条目**无人引用**（各 crate 经 `deadpool-postgres` 走 tokio-postgres 默认特性），故 `with-chrono-0_4` 一直未真正生效。改为 `shared` 直依 `postgres-types` 并开 `with-chrono-0_4` / `with-serde_json-1` / `with-uuid-1`——`tokio_postgres::types` 即 `postgres_types`，特性合一后 `Row::get` 的 `FromSql` 随即可用（`cargo tree -e features -i postgres-types` 复核三族生效） |
| NUMERIC 残留（本批不覆盖，记账） | 同上 | `postgres-types` 0.2.14 **已移除 decimal 特性**（`with-bigdecimal-0_4` / `with-rust_decimal-1` 均不在可用列表）⇒ 客户端无 `FromSql` 可读 NUMERIC，`row.get::<_, Option<f64>>` 亦因类型不匹配不可用。要覆盖须在**调用面 SQL 侧**显式 `::float8` / `::text` 转写（或升 `postgres-types` 版本），属另一批；账本 §五 记明此残留 |
| 验证（本批代码变更） | — | `shared` `152 passed / 0 failed / 11 ignored`（含新真库臂 `row_to_json_covers_jsonb_time_uuid_numeric`：jsonb 透传对象、timestamp/timestamptz/date/time 字符串化、uuid 规范化、NULL 列省略键）。**调用面 8 crate 无回归**：auth 99/0/3、cms_assemble_control 336/0/4、cms_core_entity 17/0/0、file_assemble_control 92/0/27、organization_assemble_control 151/0/0、query_assemble_designer 51/0/0、query_assemble_surface 48/0/0、processplatform_service_processing 68/0/17；`cargo check --workspace` EXIT 0。尺子 EXIT 0、GET 写 11 不变、15 张产物零漂移（本批不触路由） |

**注**：本批只动「序列化口」，不改任何 handler 的门禁，故不改变 `write_routes_*` 读数。响应形状变化是**加列**（此前被丢弃的 jsonb / 时间 / uuid 列现出现在通用 SELECT 响应里），调用面既有断言全绿。A3-N9 其余 crate 的 crud 迁移与 NUMERIC 转写仍是下一批入口。

### 三.36 第三十六批（A3-N9 crud 迁移首批：search / bbs / org 设计元数据写收 admin 门）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| search ftsearch 三把 | `crates/search/src/lib.rs` `ftsearch_create` / `ftsearch_save` / `ftsearch_delete` | `x_ftsearch_document` 为搜索索引（系统数据）⇒ 收 admin 门：`Option<Extension<Session>>`（无会话 401）+ `is_admin`（非 admin 403）。此前三把连会话都不取，任意登录人可建/改/删索引 |
| bbs section 两把 | `crates/bbs_assemble_control/src/lib.rs` `section_create` / `section_update` | 版块为管理元数据（`section_delete` 早已 admin 门）⇒ 收 admin 门。`section_create` 原「有会话无门 + 可冒填 creator」，`section_update` 原「连会话都不取」；均改 `Option<Extension<Session>>` + `is_admin` |
| org threemember 三把 | `crates/organization_assemble_control/src/u2_person.rs` `threemember_create` / `threemember_save` / `threemember_delete` | 三员（系统管理员 / 安全保密管理员 / 安全审计员）为组织安全数据（`x_org_person`）⇒ 收 admin 门，用本 crate `require_admin_opt`（无会话 401 / 非 admin 403）。此前三把 `(pool, body/Path)` 无会话无门 |
| 已核实无需动 | ai `ann_*`（require_admin）、console `server_deploy_*`（is_admin）、general `general_control_*`（require_admin）、program_center `collect_*`（属主自限定）/`invoke_*`（require_admin）、cms `dict/form/view/templateform`（u2_require_admin）、portal `dict/widget`（require_admin） | 这些 crud 调用面**已有门**，A3-N9 在此为已收口 |
| 验证（本批代码变更） | — | search `8 passed / 0 failed`（新臂 `test_ftsearch_writes_require_admin`：无会话三把 401 + 非 admin 三把 403）；bbs `55 passed / 0 failed`（`test_section_crud_roundtrip` 改 admin 探针会话 + 新增非 admin 建版块 403 臂）；org `152 passed / 0 failed`（新臂 `threemember_writes_require_admin`）。尺子：**13 条 R6/R7 → R5**（bbs 3、org 5、search 5），R6 120→119、R7 1030→1018、ungated −12（12 条 R7 + 1 条 R6）、只读挂写 −13；EXIT 0、判据见证缺口 0 |

**新发现（尺子盲点，记账不修）**：本批 13 条路由**原本同时出现在 `write_routes_readonly_on_writemethod.tsv`**——原因是这些 handler 的写 SQL 在**跨 crate 的 `shared::crud_*`** 里，尺子只看 handler 本体的写 SQL 判据，故把它们读成「写方法上的只读 handler」。含义：**R6/R7 的「含写 SQL」计数对 crud 型写面系统性低估**（`shared::crud_create/save/delete` 及 `crud_*_owned` 的落笔不可见）。要么把 `crud_create/save/delete(_owned)` 加进「写 SQL 转调」helper 白名单，要么在 crud 调用点标注；属尺子缺陷族（与缺陷 12/15/16 同轴）。

**注**：A3-N9 波及约 12 crate，本批收 8 把（search/bbs/org），已核实 ai/console/general/program_center/cms/portal 六 crate 的 crud 面早已有门；**余 pp_designer（dict/form/xform 9 把）与 query_assemble_designer（importer/stat 6 把）** 为下一批（同为无会话无门的设计器元数据写）。

### 三.37 第三十七批（A3-N9 crud 迁移收口批：pp_designer / query_designer 设计器元数据写收 admin 门）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| pp_designer 九把 | `crates/processplatform_assemble_designer/src/lib.rs` `dict_create/save/delete`、`form_create/save/delete`、`xform_create/save/delete` | 应用字典（`pp_e_applicationdict`）/流程表单（`pp_e_form`）/扩展表单（`x_pp_xform`）为设计器元数据 ⇒ 收 admin 门：`Option<Extension<Session>>`（无会话 401）+ 本 crate `require_admin_gate`（非 admin 403）。此前九把 `(pool, body/Path)` 无会话无门 |
| query_designer 六把 | `crates/query_assemble_designer/src/lib.rs` `importer_create/save/delete`、`stat_create/save/delete` | 导入模型（`x_query_import_model`）/统计定义（`x_query_stat`）为设计器元数据 ⇒ 收 admin 门，用本 crate `require_admin_opt`（无会话 401 / 非 admin 403） |
| 验证（本批代码变更） | — | pp_designer `30 passed / 0 failed / 1 ignored`（新臂 `pp_designer_meta_writes_require_admin`：九把无会话 401 + 非 admin 403）；query_designer `52 passed / 0 failed`（新臂 `query_designer_meta_writes_require_admin`：六把 401/403）。尺子：**24 条 R7 → R5**，R6 119 不动、R7 1018→**994**、ungated −24、只读挂写 −24；EXIT 0、判据见证缺口 0 |

**A3-N9 收口声明**：本批后，全仓 `shared::crud_create/save/delete` 的调用面**已全部有门或已迁移归属版**——ai/console/general/cms/portal/program_center 六 crate 原已有门（`require_admin`/`is_admin`/`u2_require_admin`/属主自限定）；search/bbs/org（批 36）与 pp_designer/query_designer（本批）为新收 admin 门；`personal_extend::face_*` 已于批 34 迁 `crud_save_owned/crud_delete_owned`（`person_id` 归属）。**A3-N9 一格可结**（后续新写的 crud 调用点须自带门或走归属版）。

**一处路由别名（记账非缺陷）**：`DELETE /api/query/assemble/designer/stat/delete/{id}` 由 `u2::stat_delete`（同表软删）占用、早已 R5，本批改的 `crate::stat_delete` 只作用于该路径的 **POST** 别名——故本批净迁 **24** 条而非 25 条（逐条对账 `HEAD`：25 条目标路由中 24 条原 R7、1 条（该 DELETE）原 R5）。

### 三.38 第三十八批（ppsurface 表面生命周期 4 把收 admin 门 + `mode_save` 归属门 + 修 creator 冒名）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| 表面生命周期四把 | `crates/processplatform_assemble_surface/src/lib.rs` `create_surface` / `save_surface` / `publish_surface` / `delete_surface` | `x_process_surface` 为设计元数据 ⇒ 收 admin 门（本 crate `require_admin_opt`：无会话 401 / 非 admin 403）。此前四把 `(pool, body/Path)` 无会话无门，任意登录人可按 id 覆写内容 / 发布 / 物理删任意表面 |
| A2-F11 冒名（本 crate 一处） | `create_surface` 的 `let creator = "system"` | 硬编码 `creator='system'` ⇒ 改取会话登录人 `sess.person_unique`（与收门同批；creator 与真实作者不再脱钩） |
| `mode_save` | 同上 `mode_save` | 写 `PP_C_TASK_PROCESS_MODE` 按 body `id` ⇒ 与**同表** `mode_id_delete` 同锁：`u4_require_ppc_person(&pool.0, session, "PP_C_TASK_PROCESS_MODE", &id)`（无会话 401 / 非属主且非 admin 403；空归属 → 只允许 admin）。id 解析移到门之前，`pool.get()` 移到门之后 |
| 验证（本批代码变更） | — | ppsurface `612 passed / 0 failed / 9 ignored`（4 条钉 500 的表面契约臂改 401，行为真变）；新臂 `surface_gate_tests::surface_and_mode_writes_require_gate`（五把无会话 401 + 表面四把非 admin 403）与真库臂 `mode_save_owner_gate_lands`（造 `PP_C_TASK_PROCESS_MODE` 一行，非属主 403 不落笔 / 属主放行 / 收口零残留）双绿。尺子：**5 条 R7 → R5**，R7 994→**989**、含写 298→293、ungated −5、只读挂写不变（这些 handler 体内本就有写 SQL，非 crud 转调型）；EXIT 0、判据见证缺口 0 |

**注**：`x_process_surface` 行因 `create_surface` 长期硬编码 `creator='system'`（`creator_person` 为 NULL）**无真实属主**，故按「ownerless 系统数据写 = admin」（F5 / R4-3 / R4-5 族）取 admin 档，与本 crate `require_admin_opt` 的既有两档口径一致。同族读 `list_surfaces` / `mode_list` 属「只读挂写方法」族（754 条），不在本批。

### 三.39 第三十九批（尺子缺陷 27：跨 crate 通用落笔 `shared::crud_*` 纳入写 SQL 作证 + 顺手修 `face_create` 残留）

| 项 | 位置 | 处置与口径 |
| --- | --- | --- |
| 尺子缺陷 27（修尺） | `scripts/audit_write_gating.mjs::writeEvidence` + 新增 `CROSS_CRATE_WRITE_HELPERS` | `shared::crud_create/save/delete(_owned)` 的写 SQL 在 `crates/shared/src/crud.rs` 的**函数体**里；`callHelperBody` 按**模块路径**解析（`crates/shared/src/crud_create.rs` / `crud_create/mod.rs`）必然落空 ⇒ 全部 crud 型 handler 被读成「只读挂写方法」，R6/R7 的「含写 SQL」计数对 crud 型写面**系统性低估**。跨 crate 限定名不在注册表里，故按**具名白名单**作证（与缺陷 11 的限定符路径同族：跨 crate 只能点名，不能靠解析） |
| 缺陷 27 的即时产出 | `write_routes_readonly_on_writemethod.tsv` −2 | 修后当场把 2 条「只读挂写」纠正为写面：`personal_extend POST /api/personal/face/create`（face_create）、`program_center POST /api/program_center/collect/create`（collect_create） |
| 顺手修 A3-N9 残留：`face_create` | `crates/personal_extend/src/personal.rs` | 缺陷 27 暴露 `face_create` 仍是「无会话无门 + `crud_create` 直接落 `x_person_face`」——可**为任意 `person_id` 建人脸生物特征行**（批 34 只迁了 `face_save`/`face_delete`，漏了 `create`）。修：`Option<Extension<Session>>`（无会话 401）+ 非 admin **强制 `personId` = 会话登录人**（忽略请求体里的他人 id）；admin 保留其 personId |
| 验证（本批代码变更） | — | personal_extend `19 passed / 0 failed / 1 ignored`（`face_writes_require_owner` 扩臂：`face_create` 离线 401 + 真库自限定「非 admin 携他人 personId，落库归属仍被强制为本人」）。尺子：`face_create` R7→R5（R7 989→**988**）、R6 119（含写 105→**106**、只读挂写 14→**13**）、只读挂写表 −2、ungated −1；EXIT 0、判据见证缺口 0 |
| 残留（记账，不修） | — | `program_center collect_create` 仍 R6「读会话无门禁」——其 `creator` 取自会话（**自归属**），属已定性假阳（批 31 的 `SELF_SCOPE_SQL_RE` 有意不认 `creator`，避免把「`SET creator=$session`」的按 id 覆写读成有门）；本批只把它从「只读挂写」纠正为「含写 SQL」 |

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

第八批（§三.8）+ 第九批（§三.9）读数：

- **同尺对账**（这把尺子改了三处才算数，所以基准也必须用**改后的尺**重跑，否则「搬家」会被读成「失踪」）：
  把 HEAD 的干净副本放在 `/d/tmp/r48base/oa4rust/backend`，只把工作树里修好的
  `scripts/audit_write_gating.mjs` 复制过去跑：
  **R1 145 / R2 3 / R4 14 / R5 491 / R6 315 / R7 1268 / R8 1**（写路由 2237、全路由 4806）；
  工作树（两批加门之后）读 **R1 145 / R2 3 / R4 14 / R5 592 / R6 315 / R7 1167 / R8 1**。
  净迁移 **101 条 R7→R5**，按 crate 逐格可点名：`portal_assemble_designer` 37、`cms_assemble_control` 27、
  `processplatform_service_processing` 27（评审 3 + 字典深度 24）、`correlation_service_processing` 10。
  按 `(crate, method, path, handler)` 配对做键集对账：**keyDiff=0**（两侧 2233 个键完全同号）——
  所以这 101 条是**换桶**，不是消失，也不是我改了分母。
- **尺子自身的读数链**（与交付分开记，因为它每改一次都会整体搬家）：
  缺陷 11 修后（工作树代码、第八批进行中）R5 524 / R6 280 / R7 1271；
  缺陷 12 首版把 `!body` 排在 exempt 判据之前 → **R4 14→12、R8 100**（实测吞掉 2 条真匿名面，改回顺序后 R8 98）；
  缺陷 13 宏模板取证（98 条 R8 读得见，逐格去向：R5 +38、R6 +35、R7 +24、R8 仅剩 1）
  → R5 568 / R6 315 / R7 1191 / R8 1。
  **R7 反而 +24 不是坏消息**：那 24 条正是被「看不见」掩盖的真洞（pp 字典深度写族），
  加门之后才回到 1167。**宏模板读出的门禁必须逐条点名**，产物 `write_routes_macro_resolved.tsv`（121 行）。
- **缺陷 13 的注入自证**（`/d/tmp/inject_d13.mjs`，字节级注入 + `finally` 复原 + 哈希断言）：
  摘掉 `dict_data_fns!` 模板里的两行 `require_admin_opt`（命中行号 339 / 348，断言恰好 2 处），
  尺子当场读回 **R5 592→568、R6 315→339，R7 1167 一字不动**，24 条 `service_processing/applicationdict/*/data`
  逐格从 R5 变 R6（不是 R7——模板里 `session` 抽取器还在，正是「读会话但无门」的定义）；
  复原后 sha256 `d4fec82bbfe5…` 与进入时逐字节一致，读数回到 592 / 315 / 1167。
  同一臂顺带量出一件新事：**同路径形状的 `processplatform_assemble_surface` applicationdict 族在两态下都读 R7**
  （注入不动它 ⇒ 尺子没在自我实现），已列 §五 第十批选题。
- **测试读数**（一律 grep `test result`，不信管道退出码）：
  `correlation_service_processing` **61 passed / 0 failed / 2 ignored**，`-- --ignored` **2 passed / 0 failed / 无 `skip:`**；
  `processplatform_service_processing` **66 passed / 0 failed / 13 ignored**，`-- --ignored` **13 passed / 0 failed**；
  `cms_assemble_control` **334 passed / 0 failed**、`portal_assemble_designer` **26 passed / 0 failed**；
  `parity` **4143 passed / 0 failed**——两批加门未破路由存在性对账。
  按名复核：`cms -- security_gate` 2 支（`cms_metadata_writes_forbidden_for_non_admin`、
  `gate_is_what_rejects_not_the_missing_database`）、`portal -- security_gate` 2 支、
  `pp -- security_gate` 命中 **24 支**（12 在跑 / 12 DB 臂在册，含 `dict_depth_writes_reject_without_admin` 离线臂 +
  `dict_depth_write_lands_only_for_admin` DB 臂）、`correlation -- security_gate` 3 支，全部在册。
- **一处「按名复核」反噬自己的取证纪律**（必须留档）：上一轮我用 `-- --ignored <名字>` 跑 pp 的 DB 臂，
  得到「13 passed / 0 failed」就以为 correlation 也安全；实际 correlation 整族 `-- --ignored` 不带过滤跑出来是
  **1 passed / 2 failed**。过滤跑法只用于**取证某支**，不能当收口判据——收口必须整族跑一遍，
  否则「我改了这个 crate」与「这个 crate 的旧断言还绿不绿」是两件事。红的那两条见下。
- **两处旧契约断言按新姿态改写（不是削门）**：
  1. `tests::test_unlink_service_route_exists`（原断言「匿名打通报 200」）改为注入一支**非 admin 会话**断言 **403**。
     403 而不是 404 ⇒ 路由仍在注册表可匹配；403 而不是 500 ⇒ `is_admin` 在池连不上时 fail-closed
     且门禁排在 `pool.get()` 之前——**因此这支不需要真库，摘掉 `#[ignore]` 常驻在跑**。
  2. `tests::test_unlink_service_route_ok` 改成 admin 端到端 DB 臂：先经 SQL 建一行，再用 seeded admin
     打真路由解除，最后数行 **1→0**。判据不看状态码，因为 `unlink_service` 未命中行时同样返回 200
     （`ActionResult::error`），只看 200 会把「门夹死了放行侧」读成绿。
  3. pp 的 `u2_dict_path_set_then_delete_roundtrip`（PUT/DELETE 往返）两条腿改走 `seed_u2_admin()` 会话并断言 200，
     中间那腿「非 admin 403」原样保留——加门之后旧往返用例的**语义**变了，改断言而不是改门槛。
- **helper 上提**：`ensure_schema` / `app_with` / `seed_admin_session` 从 `mod u2_contract` 上提到父 `mod tests`
  （`tests.rs:16-118`，附为什么上提的注释），legacy 写路由臂与 u2 契约臂共用同一套「最小 DDL + 按 `is_admin`
  真实口径造 admin + 带会话层的 Router」，复制两份必然漂移；`u2_contract` 经 `use super::*` 引用，调用点零改动。
- **新发现但本批未修的策略不一致（列 §五）**：本批 4 个 crate 用了**两族**门禁入口——
  强制 `Extension<Session>`（correlation / cms / portal）与 `Option<Extension<Session>>` + `require_admin_opt`（pp）。
  前者在**没有会话层的裸 router** 上由 axum 0.6 回 `MissingExtension` ⇒ **HTTP 500**，后者给出 **401**。
  生产路径有 auth 中间件兜底，匿名根本打不到 handler，所以不是可利用漏洞；但它让**契约臂的姿态不可比**
  （同一件事：「匿名进不去」——两个 crate 测出两个码），也让 §四 那条 272 处 `assert_eq!(status, 500)`
  断言族更难收。统一法要么全用 `Option` + `require_*_opt`（401 显式），要么在 crate 级测试里统一注入会话层。



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



第十批（§三.10）读数：

- **同尺对账**（本批动了尺子两处：缺陷 14 写 SQL 判据、缺陷 15 间接门禁，所以基准也必须用
  **改后的尺**重跑）。HEAD（第九批收口态）产物矩阵与工作树矩阵按 `(crate, method, path, handler)`
  配对做键集对账：**两侧 2233 键同号、keyDiff=0**，写路由分母 2237、全路由 4806 一字未动。
  桶读数为
  **R1 145 / R2 3 / R4 14 / R5 592 / R6 315 / R7 1167 / R8 1**（HEAD）→
  **R1 145 / R2 3 / R4 14 / R5 643 / R6 310 / R7 1121 / R8 1**（工作树）。
  净变化 **51 行换桶**，逐行可点名、无一行失踪：
  ① **46 行 R7→R5** = tranche 2 的 35 条 `data/work` 别名注册点 + tranche 1 的 9 条 `applicationdict`
  写方法注册点 + tranche 3 的 2 条 ppsvc 软删；
  ② **5 行 R6→R5** = 缺陷 15 的间接门禁作证（`attendance` `v2_record_import{,/daily}`、
  `cms_express` `view_{publish,unpublish}`、`meeting` `u2_attachment_delete`），
  **这 5 行没有任何代码改动**，纯粹是尺子此前把「门在 helper 里」读成无门禁。
  分开记这两类是纪律：修尺 ≠ 修洞。
- **GET 写族（csrf / `requires_admin` 双盲区）**：体内无门禁 **169 → 150**
  （K1 匿名 1 / K2 读会话 15 / K3 无会话 134）。−19 的构成 = `data/work` 别名族里 10 条 GET 收门
  + 缺陷 15 认出 4 条 GET 的门 + 前序 tranche 的 5 条搬家。`write_routes_get_writes.tsv` 150 行逐条在册。
- **行动面分母**（本批新增的三向拆分，闭合断言 `写 + 只读 + 不可解析 == 桶总数` 才出数）：
  R7 1121 = **真落笔 318** + 只读挂写方法 803 + 不可解析 0；R6 310 = 123 + 187；R4 14 = 3 + 9 + 2。
  999 条只读挂写方法落 `write_routes_readonly_on_writemethod.tsv`。
  R7+R4 真落笔按 crate 集中度（下一轮按这张表排；这张表按「非只读」筛，因此**含 R4 那 2 条体内不可解析**，
  合计 323 = R7 318 + R4 3 + 2 不可解析，与上句的 318/3 并不矛盾）：
  `processplatform_assemble_surface` 115 → **80**、`query_assemble_designer` 42、`message_assemble_communicate` 30、
  `query_assemble_surface` 28、`processplatform_service_processing` 23 → **21**、`cms_assemble_control` 20、
  `attendance_assemble_control` 18、`program_center` 12、`meeting_assemble_control` 10、`jpush_assemble_control` 9。
- **尺子自身的两处自证**：
  缺陷 14 的判据自证段现读「含转义引号表名写法的 **15 条路由全部判为写面，0 条漏量**」（全仓该写法 25 处）；
  缺陷 15 现读「**间接门禁作证 12 条**」并按 helper 名逐族计数落盘。
  两把守卫各配一支字节级注入臂（`/d/tmp/inject_defect14_guard.mjs`、`/d/tmp/inject_indirect_gate.mjs`）：
  把判据/作证分支摘掉，尺子当场不再点名（不是恒真）；复原后 sha256 与进入时逐字节一致。
- **加门臂的注入取证**（证「测试能红」+ 量「门不存在时的真实后果」）：
  `surface` 摘 `applicationdict` 1 道门 → 该 handler 的 GET 注册点匿名实测 **200 OK**（真物理删），
  两支离线臂红并点名该 URI（`/d/tmp/inject_batch10_gate.mjs`）；
  摘 `data_work_id_path0_path1_mockdeletetoget` 1 道门 → 该 handler 两条注册（`DELETE …/{path0}/{path1}`
  与 `GET …/mockdeletetoget`）**双双 200 OK**，`surface_data_work_alias_writes_reject_anonymous` 红
  （`/d/tmp/inject_batch10b_gate.mjs`）；
  摘 ppsvc `data_work_id_delete` 1 道门 → 离线臂读到 `Err(Internal)`（会话都不要求，直接触库）、
  DB 臂读到 **`Ok(())`**（他人 work 的 `deleted_at` 被写进去），两支各自点名（`/d/tmp/inject_batch10b_ppsvc.mjs`）。
  三处复原 sha256 均与进入时一致，无注入残迹。
- **测试读数**（一律 grep `test result`，不信管道退出码；`--ignored` 一律整族跑，不按时长猜）：
  `processplatform_assemble_surface --lib` **608 passed / 0 failed / 4 ignored**，
  `-- --ignored` **4 passed / 0 failed / 无 `skip:`**（含本批 2 支 DB 臂，0.14 s 量级即本机 PG 在跑的真实读数）；
  `processplatform_service_processing --lib` **67 passed / 0 failed / 14 ignored**，
  `-- --ignored` **14 passed / 0 failed**；
  `parity` 整族 **4143 passed / 0 failed**（过滤 `processplatform_assemble_surface` 亦 945 / 0）——
  三 tranche 加门未破路由存在性对账。
  DB 臂的落库凭据读数：`mockputtopost` 族放行后 `xupdateTime` = `2026-10-14 17:08:25.481249+00`（字典）
  与 `2026-10-14 17:08:25.514461+00`（work），均为写进 TEXT 列的时间文本（见上「被否证的类型猜测」）。
- **尺子退出码** 0；五张产物同批刷新（`write_routes_matrix` 2238 行 / `write_routes_ungated` 1136 行 =
  表头 + R4 14 + R7 1121 共 1135 条 / `write_routes_get_writes` 151 /
  `write_routes_readonly_on_writemethod` 1000 / `write_routes_indirect_gated` 12）。
  本项初稿把 `write_routes_ungated` 记成 1138，与同段自述的 R4 14 + R7 1121 相加不自洽，
  那 2 行差来自 tranche 2/3 尚未全部落盘时的中间态计数；提交前按重跑读数订正。

第十一批（§三.11）读数：

- **三跑对账**（本批同时动了代码和尺子两处，所以基准必须拆成两条轴各跑一次；跑法是
  `git worktree add --detach` 出一个 HEAD 临时工作树，分别用 HEAD 尺与新版尺各跑一遍，
  产物按 `(crate, method, path, handler)` 配对，跑完 `git worktree remove` 收掉）：

  | 跑次 | 代码态 | 尺子态 | R5 | R6 | R7 | R9 | GET 写 | 真落笔行动面 | 间接门禁 |
  | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
  | A | HEAD（第十批收口） | HEAD | 643 | 310 | 1121 | — | 150 | 444 | 12 |
  | B | HEAD | 本批（16/17/18） | 645 | 301 | 1118 | 10 | 228 | 647 | 37 |
  | C | 工作树（本批加门） | 本批 | 671 | 301−0=301 | 1092 | 10 | 227 | **621** | 37 |

  **A 复跑与第十批在册读数一字不差**（R5 643 / R6 310 / R7 1121 / GET 150 / 间接门禁 12），
  说明这把尺在同一份代码上是可复算的，不是每次跑出一个新数。
  三跑之间矩阵**键数 2233 / 2233 / 2233，两侧 keyDiff=0**（矩阵 2237 行 − 4 组重注关键：
  `ai_assemble_control` 3 条 + `bbs` `POST /api/bbs/subject/create` 各注册两处，按四元组配对时并成一键，
  故按键统计比按行统计少 4；这条差异是既有形状，不是本批引入）。
- **修尺 ≠ 修洞，两轴分开记**：
  ① **尺子轴 A→B：14 行换桶，零代码改动**。其中 10 行进新桶 R9（缺陷 18 认出死档：
  `file_assemble_control` 5 条写方法档、`portal_assemble_surface` 3 条、`program_center` 2 条），
  另 4 行 R6→R5（缺陷 17 认出 cms `data_document_{create,update,delete,mockputtopost}` 的门在
  helper 的第二层里且拒判经尾位置 `.await` 传播）。
  ② **代码轴 B→C：26 行 R7→R5**，恰好是本批加门的 27 条活注册点里挂在写方法上的 26 条
  （9 job POST + 9 workcompleted POST + 8 workcompleted PUT），另 1 条是
  `GET /data/workcompleted/mockputtopost/{id}`，它落在 GET 段：GET 写 228 → **227**，−1 就是它。
  GET 段的 A→B 净 +78 由两笔反向的改动合成——缺陷 16／17 让 GET 段第一次看见 helper 内的写（正向），
  缺陷 18 把 5 条 GET 死档出列（负向，实测：临时把 `isDeadAlias` 恒假，同码同尺读回 232）；
  正向那一笔的精确值本批没有单独拆量，只保证净差可复算。
- **行动面分母（选题用）**：三向拆分的闭合断言仍成立（`写 + 只读 + 不可解析 == 桶总数`）。
  A：**444** = R4 3 + R6 123 + R7 318；B：647 = 5 + 272 + 370；C：**621** = R4 5 + R6 272 + R7 344。
  也就是**本批把洞修掉 26 条，但尺子新暴露出 203 条此前看不见的落笔面**——这个数字涨不是变坏，
  是「原来量不到」。只读挂写方法同步从 999 收成 **784**（R7 748 + R6 29 + R4 7）。
  按 crate 排的真落笔无门禁面（矩阵行计 623，去重后 621，多的 2 行是那 4 组重注关键里的写面）：
  `cms_assemble_control` 86、`processplatform_assemble_surface` 83、`program_center` 53、
  `query_assemble_designer` 44、`processplatform_service_processing` 43、`attendance_assemble_control` 39、
  `query_assemble_surface` 38、`message_assemble_communicate` 37、`meeting_assemble_control` 26、
  `file_assemble_control` 21（余 12 个 crate 长尾，见 §五）。
- **匿名写行动面 A1 从 3 条涨到 9 条并已钉成点名名单**（缺陷 16 的副产品）：新增 6 条全部是
  「写 SQL 在 helper 里、handler 本体只转发」的匿名入口——OAuth 三方登录 4 条经
  `provider_login` / `oauth_code_store`，密码重置 2 条经 `apply_reset`。**它们是否算洞本批未定拍**
  （三方登录回写与匿名重置按 o2server 语义本就要求匿名可达），名单里逐条在册，进 §五。
  尺子的 A1 断言现读「实测 9 条 / 名单 9 条」，多一条少一条都红。
- **判据见证（本批新增的尺子自证段）**：现读「缺陷 16 3 条 / 缺陷 17 3 条 / 缺陷 18 正向 6 条 +
  反向 4 条，**缺口 0 处**」；不齐即 `process.exitCode = 1`。
  字节级注入臂逐臂点名（`/d/tmp/arms_ruler.mjs`，每臂一进程、跑完逐字节复原并断言 sha256 与进入时一致）：
  E16 摘 `awaitCall` → 缺口 4 处、尺子退 1；E17 把递归压回 0 层 → 缺口 3 处；
  E18 正向把 `isDeadAlias` 恒假 → 死档 0 条、缺口 6 处；
  **E18 反向把尾逗号算成一段 → 死档 152 条**、反向见证当场点名 correlation `unlink` 被误判；
  对照臂（无害注释）仍绿、缺口 0。四臂全部「注入即红」，证明这三处修的是判据而不是读数。
- **加门臂的注入取证**（`/d/tmp/arms_code.mjs`，同样逐臂复原 + sha256 相等）：
  G1 摘 job 族 9 把门 → 离线臂点名 9 条 POST 档全部 **200 OK**（本机 PG 在跑，即匿名真落笔）；
  G2 摘 workcompleted 族 9 把门 → 点名 18 条（9 POST + 8 PUT + 1 GET）全部 200；
  G3/G4 走 DB 臂 → `assertion left == right failed: 他人会话 touch 他人 job|workcompleted 必须 403，left: 200`，
  即门一摘，他人归属校验连带失效（时间戳被改写）；对照臂仍绿。
  复原后整族复跑：注入残迹检查 `grep -c "let _ = session; /* 注入臂"` = **0**，门计数 9 + 9 复原。
- **测试读数**（一律看 `test result` 行，不信管道退出码；`--ignored` 整族跑）：
  `processplatform_assemble_surface --lib` **609 passed / 0 failed / 7 ignored**，
  `-- --ignored` **7 passed / 0 failed / 无 `skip:`**（本批 +2 支离线臂引用、+4 支臂，其中 3 支 DB）；
  `parity`（过滤本 crate 路由存在性族）**945 passed / 0 failed**——加门未破路由存在性对账；
  `openapi` 整族**跑不了**：`cargo test -p openapi` 在本机撞 `error calling dlltool 'dlltool.exe': program not found`
  （`windows-sys` 构建依赖，非本批改动引起），该 crate 的契约面本批**未取得读数**，不记为通过。
  尺子退出码 0。DB 臂放行侧落库读数：job `xupdateTime` = `2026-10-14 18:16:55.00026+00`、
  workcompleted = `…55.000202+00`（对照 work 族 `…55.000297+00`），仍是写进 TEXT 列的时间文本（第十批「被否证的类型猜测」同型）。
- **产物刷新八张**（行数含表头）：`write_routes_matrix` 2238（= 2237 条）/ `write_routes_ungated` 1107
  （= R4 14 + R7 1092 + 表头，闭合）/ `write_routes_get_writes` 228 / `write_routes_readonly_on_writemethod` 785 /
  `write_routes_indirect_gated` 38 / `write_routes_indirect_writes` 320 / **新** `routes_dead_alias` 25 /
  `anonymous_write_routes` 173。
  落盘顺序按第十批教训排：间接写、间接门禁、死档三段都在所有分桶段之后，GET 段往里追加的条目不会再丢。

- **第十二批读数（尺子轴三跑：C / D / G·H）**——本批**零产品码改动**，故只有尺子一条轴，
  但仍按前三批的规矩取三态，让「谁贡献了多少」可复算：

  | 跑 | 码 | 尺 | R5 | R6 | 只读挂写 | GET 写无门禁 | 间接门禁作证 | 间接写 SQL | 真落笔 | 见证缺口 |
  | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
  | C | 工作树码（= HEAD） | HEAD（缺陷 16/17/18） | 671 | 301 | 784 | 227 | 37 | 319 | **621** | 0 |
  | D | 同上 | + 缺陷 19、20 | 798 | 174 | 780 | 185 | 208 | 212 | **498** | 段未加 |
  | G·H | 同上 | + 缺陷 21 + 全部见证段 + 反证尺 | 798 | 174 | 780 | **193** | 216 | 196 | **498** | **0** |

  C→D 的 **−123 全部在 R6 含写格**（272→149）：缺陷 20 认出「门在 helper 体内且不写 `AppError::Forbidden` 字面量」这一族之后，
  **127 行 R6→R5**（其中 123 含写、4 只读），R7 一律未动。D→G 的净差只在 GET 面（**185→193，+8**）与间接名单归属，
  真落笔计数不变——缺陷 21 影响的是 GET 写面与「谁给谁作证」，不改 R4/R6/R7 的落笔判定。
  **记账缺口（不粉饰）**：D 那一跑是在缺陷 19、20 都改完之后才跑的，**19 与 20 各自的净贡献本批未拆开**，
  只能给合并差 −123；两臂的点名红（下方 T19a、T20a）证明两处都承重，但「摘掉 19 只留 20 会读成多少」没有读数。
- **注入臂取证（通道 B 字节级，`/d/tmp/inject_arm.mjs`；每臂一进程、跑完逐字节复原并断言 sha256 与进入时一致）**：
  `基线读数 → 逐臂点名红 → 复原后基线复绿` 三段齐：
  - **T19a** 把 `/^\s*\.await\s*\}/` 改成永不相交的 `/^\s*\.await\s*@@\}/` → 尺子退 1，**缺口 10 处**，
    点名 4 条「缺陷 19 见证缺失（尾表达式转调未被 `appdict_data_write` 作证）」（cms `surface_appdict` 的 DELETE/PUT/POST/DELETE）+ 6 处级联。
  - **T20a** 摘掉 `helperCanReject` 的新分支（只留字面量判据）→ 尺子退 1，**缺口恰 4 处**、全部标缺陷 20：
    `attendanceadmin_delete → delete_admin_record` 与 `attendancedetail_delete → delete_owned_record` 各两条（见证 + 间接门禁名单）。
  - **T20c** 把 `helperCanReject` 整体放宽成 `return true`（恒真放松）→ 尺子退 1，
    **13 条「缺陷 20 独立反证：… 该 helper 无拒绝能力」**逐条点名（`u2_is_admin` 族、`attachment_u2b_*` 族、`u2_att_update_meta/text`），
    即恒真放松被那支不调用被测函数的第二把尺抓住——这格是 T20a 给不了的，点名式反向见证在自然语料里量不到（实测 0 条）。
  - **T21a** 摘掉切段（回落整张模板）→ 尺子退 1，点名 **2 条「宏模板读档仍被写档 `appdict_data_write` 借证」**
    （`anonymous_surface_appdict_…_path0_data`、`surface_appdict_…_path0_data`）。
  - **CTRL**（无害改动）→ 退 0、仍绿、缺口 0。四臂全部「注入即红」，对照臂不红，证明这三处修的是判据而不是读数。
  - **一次假成功已纠**：T20c 首次跑用 `for` 循环 + `|` 作字段分隔传参，`--find` 里的 `||` 被切坏，注入后尺子报
    `SyntaxError: Unexpected token ')'`、退出码 1，被注入器读成「臂成立」。红因不对即判废，改用单条命令 + 单引号字面量重跑才有上方读数。
    **教训**：注入臂的「退 1」不等于取证成立，必须读日志确认红是**见证点名**还是**语法炸**——前者才是判据承重。
- **复原后基线（run G·H，干净工作树）**：尺子 EXIT=0，桶计数 R1 145 / R2 3 / R4 14 / R5 798 / R6 174 / R7 1092 / R8 1 / R9 10 = 2237，
  GET 写无门禁 193（K1 5 / K2 11 / K3 177）、只读挂写 780、真落笔 **498**（R4 5 + R6 149 + R7 344）、宏模板 121（切段 62 + 整张 59）、
  死档 24、A1 实测 9 / 名单 9、`判据见证 … 缺口 0 处`。**十五张在册产物与 run E 存档逐张 md5 全等**，注入零残留
  （本批再次踩到第十一批发过的那条纪律：注入跑会重写产物，提交前必须干净重跑再逐张比哈希）。
- **本批未跑 cargo 族**：改动全部落在 `scripts/audit_write_gating.mjs` 一张尺上，零产品码、零测试码，
  没有可验的 Rust 增量；尺子的可验面由上方五支注入臂承担。下一批动码时必须补 `--lib` 与 `-- --ignored` 读数。
- **按 crate 排的真落笔无门禁面（run H，去重后 497 / 矩阵计 498，多的 1 行是同一 `crate+method+path` 的重注关键）**：
  `processplatform_assemble_surface` **79**、`cms_assemble_control` **48**、`query_assemble_designer` 44、
  `processplatform_service_processing` 43、`message_assemble_communicate` 37、`program_center` **29**、
  `query_assemble_surface` 28、`file_assemble_control` 20、`meeting_assemble_control` 20、`attendance_assemble_control` **19**，
  其后 `cms_core_entity` 15、`processplatform_assemble_designer` 15、`calendar_assemble_control` 13、`mind_assemble_control` 13、
  `general_assemble_control` 11，余 22 个 crate 共 63 条（长尾首列 `bbs` 9、`jpush` 9、`ai_assemble_control` 6）。
  对照 §五 第十一批那张 621 的表：**cms 86→48、program_center 53→29、attendance 39→19**，
  `query_assemble_designer` / `ppsvc` / `message` 三族一字未动——它们是下一批的活，不是尺子能修掉的。

- **第十三批读数（尺子轴，run J/K——同一把尺两次跑，注入臂之后复验）**：
  写路由 2237 / 全路由 4806 未变，桶计数 R1 145 / R2 3 / R4 14 / R5 798 / R6 174 / R7 1092 / R8 1 / R9 10 **一格未动**
  （缺陷 22 只改「含写 SQL / 只读」这一刀，不改门禁与会话判定）。净差：

  | 判据 | run H（缺陷 22 前） | run K（缺陷 22 后） | 差 |
  | --- | --- | --- | --- |
  | 真落笔（R4+R6+R7 含写） | 498 = 5 + 149 + 344 | **512** = 5 + 151 + 356 | **+14** |
  | 只读挂写方法 | 780 | 766 | −14 |
  | GET 写面无门禁 | 193（K1 5 / K2 11 / K3 177） | **203**（K1 5 / K2 11 / K3 187） | **+10** |
  | 间接写 SQL | 196 | 221 | +25（`soft_delete_by_id` 14、`cms_entity_soft_delete` 等） |
  | 间接门禁作证 | 216 | 217 | +1 |
  | 写 SQL 判据自证 | 转义引号 15 条 / 0 漏量 | 加动态表名 **14 个函数 / 0 处漏量** | 新刀口 |

  按 crate 的真落笔同步刷新：`processplatform_assemble_surface` 79（不动）、**cms 48 → 52**、
  `query_assemble_designer` 44、`processplatform_service_processing` 43、`message_assemble_communicate` 37、
  `program_center` 29；去重后 511（矩阵计 512，多 1 行是同一 `crate+method+path` 的重注关键）。
- **注入臂取证（通道 B，五臂有落盘日志）**：
  - **T22a** 把表名位的 `|\{` 分支摘掉 → 尺子退 1，红因点名 `Error: 写 SQL 判据漏量 4 处（format! 动态表名被判成只读）`
    （helper 层自证先 throw，所以这一臂的量是「4 处被当场揪出」而非见证缺口数）。
  - **T22b** 把 `UPDATE\s+(…|{)` 整支压宽成裸 `UPDATE` → 尺子退 1，**但红因不是缺陷 22 的三条反向见证**：
    三条反向档体内根本没有 `update` 这个词，过宽放松把它们放过去了；真正喊红的是 **A1 真匿名写点名名单**
    （过宽后匿名面多出成员，集合相等断言当场红）。**如实记账**：缺陷 22 的反向见证只对「门禁侧退化」敏感，
    对「写判据过宽」不敏感——那一侧的兜底目前挂在 A1 名单上，是巧合承重，不是设计承重。
  - **CTRL22** 首跑**判废**：`--replace` 传的注释文本被 shell 吃掉行首一个 `/`，尺子 `SyntaxError: Invalid regular expression`，
    注入器按 `--expect green` 读到退 1 报「对照臂意外变红」（exit 4）。这是注入器该有的表现，不是假成功。
    换掉锚点重跑 **CTRL22b**（在 `const kRoutes = …` 后加一句无害注释）→ 退 0、仍绿、缺口 0。
  - **复原后基线**：run K 干净重跑 EXIT=0，`判据见证 … 缺口 0 处`，**十五张在册产物与 run J 存档逐张 md5 全等**。
- **本批未跑 cargo 族**：改动仍在尺子上，零产品码；T22b 顺手暴露的那批新档（cms 18 把 GET 别名写档）
  正是下一批代码轴的活，届时必须补 `--lib` 与 `-- --ignored` 读数。

- **第十四批读数（代码轴 + 尺子轴，run K → L → M → N/P）**：
  - 基线（HEAD，第十三批那把尺，run K 存档 `/d/tmp/b14_get_before.tsv` + `b14_matrix_head.tsv`）：
    真落笔 **512**、GET 无门禁 **203**（K1 5 / K2 11 / K3 187）、只读挂写 766、间接写 221、间接门禁 217。
  - run L（**只改产品码**，`/d/tmp/b14_L_ruler.log`）：尺子 **EXIT=1、缺口 2 处**——红因是缺陷 22 的见证过时
    （`appinfo` / `comment` 两把别名修好后出了 K 桶），按上面那格重钉；同跑 GET 面 181、R6 合计 175（含写 152）、
    真落笔 511、间接门禁 225、间接写 232。**这一跑不改尺子的读数是无效的**，只用来把「尺子轴」和「代码轴」分开。
  - run M（代码 + 缺陷 23，`/d/tmp/b14_M_ruler.log`）＝ run N/P（注入全部复原后干净重跑）：
    真落笔 **494**（R4 5 / R6 135 / R7 354）、GET 无门禁 **178**（K1 5 / K2 11 / K3 162）、只读挂写 **766 不变**、
    间接门禁 **234**、间接写 **223**、`判据见证 … 缺口 0 处`、EXIT=0。
    在册产物 15 张里 **6 张**变了（`write_routes_get_writes` / `write_routes_indirect_gated` /
    `write_routes_indirect_writes` / `write_routes_macro_resolved` / `write_routes_matrix` / `write_routes_ungated`），
    两次干净跑逐张 md5 全等（`/d/tmp/b14_md5_N1.txt` = `/d/tmp/b14_md5_P.txt`）。
  - **两轴分开归因，逐条点名不是估的**：拿 `git show HEAD~1:…write_routes_matrix.tsv`（含表头 2 238 行 = 数据
    2 237 条）与当前矩阵（同样 2 237 条）做键级
    diff ⇒ **0 条消失、0 条新增、18 条迁移**，而真落笔正好 −18（512→494）。这 18 条里
    **16 条 R6→R5 属尺子轴**——除 `categoryinfo_u2_delete`（本批只把它手写的 gate SQL 抽成
    `U2_CATEGORY_OWNER_SQL`，门语义不变）之外，**15 条函数体逐字节未变**（`viewfieldconfig_u2_delete` /
    `categoryinfo_u2_create` / `viewfieldconfig_u2_create` / `viewfieldconfig_u2_update` /
    `categoryinfo_ext_content_save_u3` / `correlation_u2_doc_delete` / `fileinfo_u2_copy_to_doc` /
    `fileinfo_u2_replace_to_doc` / `script_post_nested_u3` + `u3_permission_save_handler!` 生成的 6 把
    `permission_save_*`，它们在 HEAD 就带门，宏体里读 `u2_gate_by_sql`）
    ⇒ **HEAD 那 512 里有 16 条是假阳**，即「真落笔」一直把这 16 把算成没门；
    另 **2 条 R7→R5 属代码轴**（`appinfo_erase_app_id`、`categoryinfo_erase_category_id`）。
    GET 面 203→178 = −25 同样逐条点名：22 把 `mockdeletetoget` 别名 + `file_copy_u3` +
    `fileinfo_id_doc_docId_change_seqnumber_seqNumber` + `permission_management_refresh_category_categoryId`，
    **0 条桶迁移、0 条新增进面**。其中 `categoryinfo_id` / `categoryinfo_erase_category_id` /
    `viewfieldconfig_id` 三把别名必须「代码加门 **且** 尺子认门」才出列——单轴时它们只是 relabel 成 K2、
    仍留在 GET 面上（run L 的现场）。
- **门禁测试（`crates/cms_assemble_control/src/security_gate_tests.rs`，170 行 / 2 臂 → 6 臂）**：
  离线两臂＝`soft_delete_after_gate` 三态真值表（Forbidden→403、NotFound→200+error+回显文案、
  Allowed→**必须 `Internal`**，这一格就是「403 来自门、不来自缺库」的反证）+ admin 档 3 把逐把点名 403
  （`is_admin` 无库判假，而路径上任何一次真落笔都会先变成 500 ⇒ 「403 而非 500」本身就是门在写之前的顺序证明）。
  真库四臂一律 `#[ignore]`（与 surface 的 R4-8 / R4-10 / R4-11 同法，不用运行时守卫——那会把「没起库」的静默跳过
  伪装成通过）：owner/派生档 **14 把**（12 别名 + 2 正档）逐把跑「他人会话 403 **且行未被软删** → 属主会话成功
  **且 `deleted_at` 真落库**」两半，宏 `$mockdel` **8 把**逐把跑同一形状（判据换成字段行按 `doc_id` 存活/消失），
  `file_copy_u3` 按 `app_id` 计数增量（先钉 0，否则增量断言恒真），`fileinfo_…_seqnumber` 按 `create_time::text`
  逐字节相等钉住「403 之后没落笔」。
  读数：`cargo test --lib -p cms_assemble_control` **336 passed / 0 failed / 4 ignored**；
  `-- --ignored` 整族 **4 passed / 0 failed**（两族都不按时长猜，一律读 `test result` 行）。
  真库臂**第二轮才可信**：第一轮绿是因为库里恰好没残留，第二轮直接撞 `x_cms_data_document_field` 主键——
  `wipe_alias_rows` 按 `id` 回擦擦不到挂在 `doc_id` 上的字段行。补 `DELETE … WHERE doc_id = $1` 之后连跑两遍幂等全绿。
- **注入臂取证（通道 B，本机无 Python，用 Node 版注入器 `D:/tmp/inject_arm.mjs`；八臂日志 `D:/tmp/b14_L_*.log`）**：
  - **E1b** `comment_id_mockdeletetoget` 的门恒放行（实参 `gate` → `U2Gate::Allowed`）→ `--ignored` 退 **101**，
    **恰好一支红**并点名 `cms_get_alias_owner_arms_forbid_stranger_and_land_for_owner`，失败信息
    `comment_id_mockdeletetoget：他人会话未被 403`。
  - **E2** `gate_doc_writer` 改成恒 `Ok(U2Gate::Allowed)` → **双红**：
    `cms_data_document_path_alias_gate_lands` + `cms_fileinfo_seqnumber_alias_gate_lands`（这两把的门只存在于共享
    helper 里，此臂证的是 helper 作证链没断）。
  - **E3** `u2_require_admin` 里 `is_admin(...)` 短路成 `true || is_admin(...)` → admin 档离线臂红，
    **并连带第八批两臂**（`cms_metadata_writes_forbidden_for_non_admin`、
    `gate_is_what_rejects_not_the_missing_database`）一起红 ⇒ 新 3 把与旧 17 把共用同一道门，不是各写一份。
  - **E4** 把形状 helper 的 NotFound 格改成 `Err(AppError::Forbidden)` → 三态真值表臂红
    （`soft_delete_after_gate_three_states_are_distinct`）⇒ 「NotFound 不许伪装成 403」这一格有人看。
  - **E8** 只摘 `appinfo_erase_app_id` **正档**的门（拿带注释行的长锚点把命中数从 3 收成 1）→ 退 101 且点名
    `appinfo_erase_app_id：他人会话未被 403` ⇒ 新增那 2 条正档格子不是搭别名便车。
  - **E5（尺子轴）** 从 `GATE_RE` 摘掉 `u2_gate_by_sql|` → 尺子退 **1**、**缺口 3 处**，逐条点名缺陷 23 的三条正向
    见证（两条 DELETE 落回 `R6-已认证+读会话无门禁`、一条别名落回 `K`）⇒ 词表这一格有牙。
  - **E7（如实记成一失败臂）** 从 `GATE_RE` 摘掉 `u2_check_owner|` → **注入后仍绿、缺口 0，注入器按 `--expect red`
    判为恒真（退 4）**。读法：`u2_check_owner` 在词表里只是**快捷路径**，摘掉后 `gateEvidence` 走递归分支
    （helper 体内 `u2_gate_by_sql` 命中词表）照样作证 ⇒ 桶读数不变。所以这个词**没有**独立见证，本批也不给它补：
    要钉的命题是「这两条入口至少认一条」，E5 已证明另一条被摘时会红。**单独点名，不混进通过里**。
  - **E6（无害对照）** 只改 `soft_delete_after_gate` 的 doc 注释 → 4 passed / 仍绿 ⇒ 上面几支红是注入造成的，
    不是环境本来就不稳。
  - 每臂 `finally` 逐字节复原并断言读回哈希等于进入时哈希；复原后 run N/P 干净重跑 EXIT=0、
    6 张变化产物与注入前存档逐张 md5 全等；`git status --porcelain` 只剩本批 4 类在册改动
    （`lib.rs` / `security_gate_tests.rs` / `audit_write_gating.mjs` / 6 张产物）。


- **第十五批读数（纯代码轴，run R → S）**：
  - 基线（HEAD＝第十四批 run N/P 存档 `/d/tmp/b14_md5_Q15.txt`）：真落笔 **494**、GET 无门禁 **178**
    （K1 5 / K2 11 / K3 162）、只读挂写 766、间接写 223、间接门禁 234。
  - run R（只改产品码）＝ run S（七臂注入全部复原后干净重跑，`/d/tmp/b15_S_ruler.log`）：
    真落笔 **472**（R4 5 / R6 135 / R7 332）、GET 无门禁 **156**（K1 5 / K2 11 / **K3 140**）、
    只读挂写 **766 不变**、间接写 **223 不变**、间接门禁 **234 不变**、`判据见证 … 缺口 0 处`、EXIT=0。
    15 张在册产物只有 **3 张**变（`write_routes_get_writes` / `write_routes_matrix` / `write_routes_ungated`），
    `md5sum -c` 逐张 **15/15 OK** 等于 run R 存档。
  - **单轴归因，逐条点名**：拿 `git show HEAD~1:…write_routes_matrix.tsv`（含表头 2 238 行＝数据 2 237 条）
    与当前矩阵做键（crate+method+path）级 diff ⇒ **0 消失、0 新增、22 迁移，且 22 条全部 R7→R5**
    （逐条名单见 `/d/tmp/b15_diff2.mjs` 输出，22 条注册点全在 POST 侧键上）。GET 面 −22 同向等量：
    `write_routes_get_writes.tsv` 179→157 行（−22、0 新增）、`write_routes_ungated.tsv` 1105→1083 行（−22）。
    **本批没有第十三/十四批那种「尺子轴的假阳回收」混进来**——尺子一字未动，所以 −22 全是真收口的洞，
    引用行动面时也不必注明代次差（仍是第十四批那把尺）。
  - 22 把 × 2 条注册点＝**44 格**全部有测试点名（`MOCK_ALIAS_ARMS` 由 `D:/tmp/b15_emit_arms.mjs` 机械生成自
    `routes.rs`，臂①另把 `regs == 44` 与 `len == 22` 两枚计数钉死：少一格删档、多一格长档都会先红在计数上）。
- **门禁测试（`crates/processplatform_assemble_surface/src/security_gate_tests.rs`，1 014 行 / 9 臂 → 1 312 行 / 12 臂）**：
  离线臂 `surface_mock_alias_writes_reject_anonymous`＝44 格无会话一律 401（门在 `pool.get()` 之前，缺库也证得了）；
  真库臂 `surface_mock_alias_owner_gate_lands`＝逐把跑「他人 403 **且未落笔** → 放行侧成功 **且真落笔**」两半，
  放行凭据按形状分两族：`del` 族认**行是否还在**、`touch` 族认 `xupdateTime` 是否脱离 `'r51-before'` 哨兵
  （handler 未命中行也回 200，状态码在放行侧不是凭据）；record 档另加一格证伪——**被 seed 的那个人也 403 且未落笔**，
  因为 `PP_C_RECORD` 行上根本没有可认的属主列，「改成归属档」在此表不可表达；
  真库臂 `surface_mock_alias_gate_covers_both_verbs`＝他人会话打满 44 格（含 POST 侧）一律 403，钉「门不挑动词」。
  读数：`cargo test --lib -p processplatform_assemble_surface` **610 passed / 0 failed / 9 ignored**；
  `-- --ignored` 整族 **9 passed / 0 failed**；两支新真库臂**连跑两遍幂等**（`/d/tmp/b15_run1.log` = `b15_run2.log`，
  都 2 passed）；注入复原后再各跑一遍复绿（`/d/tmp/b15_full_after.log` / `b15_ignored_after.log`）。
  **第一轮红的不是门是 seed**：`seed_mock_row` 起初按「admin 档也把值写进 `xcreatorPerson`」设计，
  PostgreSQL 报 **42703**（`column "xcreatorPerson" of relation "pp_c_record" does not exist`，中文即「该列不存在」）——
  兜底建表的列假设来自同族表类比，而探针早就给出 record 无此列。改成 `owner_col` 为空就**只插 id/xid/xupdateTime**，
  并把「列缺失」这条实测事实升成上文的证伪格（若有人把 record 改回归属档，门会去读不存在的列 ⇒ 500，臂②点名红）。
- **注入臂取证（通道 B，本机无 Python，Node 注入器 `D:/tmp/inject_arm.mjs`；七臂日志 `D:/tmp/b15_E*.log` / `b15_R*.log`）**：
  - **E1** 摘掉 `draft_id_mockdeletetoget` 一格的门 → 离线臂退 101，失败信息**逐格点名**
    `draft_id_mockdeletetoget POST/… 期望 401，实际 200 OK` 与 GET 侧同档 ⇒ 单把门被摘会当场红，不是靠别的档兜。
    首跑**判废并如实记**：`--cmd` 里带了 `| tail -20`，管道把退出码变成 tail 的 0，注入器按 `--expect red` 误报「恒真」（退 4）——
    「不信管道退出码」这条纪律在注入器上同样成立，改成 `--quiet` 让 cargo 自己定退出码才是有效臂。
  - **E2** `u4_require_ppc_person` 的归属判定改成恒 `Ok(())`（会话门仍留首位）→ 两支真库臂**同时红**、离线臂不受影响
    ⇒ 「401 侧与 403 侧各由不同格盯住」得到证明。锚点首跑命中 2 处被注入器中止（`u4_require_ppc_actor` 体内同形），
    补上前一行的 `xperson` 读取才收成唯一命中——锚点唯一性在这里是**防串味**的，不是形式主义。
  - **E3** record 档由 `require_admin_opt` 改成归属档（`u4_require_ppc_actor(…, "PP_C_RECORD", …)`）→
    两支真库臂红（`security_gate_tests.rs:1290` / `:1320`）⇒ §三.15 那格「record 只能 admin」有牙。
  - **E4** helper 的属主列认错（`SELECT xperson` → `SELECT xcreatorPerson`）→ 两支真库臂红 ⇒ 列名选择是承重的。
  - **E0（无害对照）** 只在两个 helper 各插一句注释（`--hits 2`）→ 仍绿 ⇒ 上面的红是注入造成的。
  - **R1（尺子轴）** 从 `GATE_RE` 摘掉 `u4_require` → 真落笔 **472→513**、GET 面 **156→172**、间接门禁 **234→291**
    ⇒ 本批 20 把（person/creator/work）的门确实靠这个词作证，词有独立见证。
  - **R3（尺子轴）** 摘掉 `require_admin` → 尺子**非零退出**且自证先喊红：`判据见证 … 缺口 3 处`，逐条点名
    缺陷 20 的 `delete_admin_record` 两条 + 缺陷 22 的反向见证（organization `person/{flag}/mockdeletetoget`）；
    同跑读数 R6 含写 135→225、GET 面 156→184 ⇒ record 那 2 把所在的 admin 档由这条词表承重。
  - **R2（如实记成恒真臂，单独点名不混进通过里）** 摘掉 `require_actor` → **六格读数与 run R 全等、缺口 0**。
    读法：摘掉这个词后**全仓没有任何一档掉进无门禁面**，说明它在词表里只是一条**快捷路径**——
    现存的档全部还被别的词作证，所以这个词**没有独立见证**，与第十四批 E7（`u2_check_owner`）同族。
    （机制侧未逐条查证：到底是哪几个词覆盖了它们，本批不下结论，只认「摘了不红」这个读数事实。）
    本批不给它补见证、也不删词：要钉的命题是「几条入口至少认一条」，R1/R3 已各自证明被摘时会红；
    真要收这格，得把词表改成「两条入口都不许同时摘」的合取见证（记进 §五 下一批方法学缺口）。
  - 每臂 `finally` 逐字节复原并断言读回哈希等于进入时哈希；复原后 run S 干净重跑 EXIT=0、缺口 0、
    15 张产物与注入前存档逐张 md5 全等；`git status --porcelain` 只剩本批 3 类在册改动
    （`lib.rs` / `security_gate_tests.rs` / 3 张产物）。

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

### 第八、九批收口后的刷新（§三.8 / §三.9 / §四 两批读数）

- **已修并验证**：**第八批 77 把**（`cms_assemble_control` 元数据写 17、`portal_assemble_designer` 设计器写 26、
  `correlation_service_processing` 身份关联写 10 + 新增 `require_admin`、`processplatform_service_processing`
  评审链活路由 3）与**第九批 24 把**（`dict_data_fns!` 宏族 `dict_set_1..8` / `dict_del_1..8` 深度路径写收 admin），
  同尺净迁移 R7 1268 → R7 1167 / R5 491 → R5 592，键集零差；测试见 §四 两批读数。
  顺带收掉两个非加门缺陷：`correlation` 里 `r#"…"#` 中的孤立反斜杠（写路径必 500，全仓同类扫描归零）、
  以及 pp 同名**死 handler** 造成的「handler 存在 ≠ 路由可达」误判（尺子缺陷 11 由它顶出）。
- **尺子三处缺陷的账**（修尺 ≠ 修洞，两条线分记账）：**11 限定符被剥掉**（双向错，能把已修报成漏洞、
  也能把漏洞报成已修）、**12 宏生成 handler 读成空 body**（新 R8 桶「本尺不背书」+ `write_routes_unparsable.tsv`）、
  **13 宏模板取证**（`write_routes_macro_resolved.tsv` 121 行逐条点名）。
  归桶顺序教训：**R1 与 R2-R4 必须先于 R8**，否则匿名可达面被「看不见」桶吞掉（实测 R4 14→12）。
  附带效果：GET 写面无门禁读数 **165 → 169**（K1 1 / K2 15→16 中的宏族被认出后 K2 16→15、K3 148→153），
  因为宏模板同样让 GET-写探测第一次看得见那 4 条——引用旧数请按本节为准。
- **R4-2 余量重称（覆盖 §五 旧的「81 条 / 68 名」）**：`processplatform_service_processing` 的 R7 现读 **54 条**
  （两批共收 27 条：评审 3 + 字典深度 24）。全仓 R7 集中度前四改为
  `processplatform_assemble_surface` **351**、`cms_assemble_control` 96、`query_assemble_designer` 67、
  `query_assemble_surface` 58——下一轮选题按这张表排。
- **第十批首选（本批注入臂顺手量到的真洞，已回源码复核）**：
  `processplatform_assemble_surface` 的 applicationdict 深度路径族两态恒读 R7，逐 handler 定性分三类
  （合计 **33 条写路由 + 7 条 GET 写**）：
  ①`applicationdict_…_path{0..7}_data` 共 **24 条**（8 档 arity × POST/PUT/DELETE），但 handler 本体
  （`lib.rs:1241`）只有一句 `SELECT … FROM PP_E_APPLICATIONDICT`——**同一把只读 handler 挂在三个写方法上**。
  这不是越权洞，是「假阳性形状」：收法是把读挂回 GET、写方法显式拒绝，而不是给它加门。
  ②`…_mockdeletetoget` **物理删面 8 档**：1 档注册在 DELETE（`lib.rs:1287`，`path3`），其余 7 档注册在 **GET**
  （落 `write_routes_get_writes.tsv` 的 K3，正是 §五「GET 写禁令」那一族）；handler 体内
  `DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1` **且同一条删两次**，无 Session、无任何门禁——
  登录即可（GET 那 7 档连写方法都不需要）按任意 xid 物理删系统元数据，**属真洞、优先级最高**。
  ③`…_mockputtopost`（**8 条**，`lib.rs:1357` 起）体内 `UPDATE … SET "xupdateTime" = NOW()`，同为无会话无门写。
  该 crate `routes.rs` 里 `mockdeletetoget` 字样出现 93 次，即②的形状是全仓一族，不是孤例；
  修法沿用 §五「建议统一修法」：字典/元数据写走 admin 门，`mock*` 别名族与正式路由同锁。
- **新开两格策略性待修**：①**门禁入口两族并存**——强制 `Extension<Session>`（裸 router 回 500）与
  `Option<Extension<Session>>` + `require_admin_opt`（401）在同一 worktree 并存，生产有中间件兜底故非漏洞，
  但契约臂姿态不可比、也与 §五 那 272 处 `assert_eq!(status, 500)` 断言族纠缠；收法建议统一走 `Option` + `require_*_opt`。
  ②**correlation 读侧 IDOR 未动**：`list_correlations`（`/list/{personId}`）、`get_link`、`get_correlation`
  仍是「任一登录用户按 id 读他人身份关联映射」，写侧本批收了 admin，读侧没收——身份关联表的可读性本身就是
  情报面，下一批与②同题收。
- **产物卫生**：`write_handlers.tsv`（函数级旧产物）与本批矩阵有 215 行双向差、来源不可追，已删除；
  本批新增 3 张尺子产物（`route_handler_unresolved_refs.tsv` / `write_routes_unparsable.tsv` /
  `write_routes_macro_resolved.tsv`）与矩阵、ungated、get_writes 一并入册，均为 `node scripts/audit_write_gating.mjs`
  的确定性输出（同码重跑逐字节一致，注入臂的复原步已顺带验证过一次）。
- 提交落点：`618ef840a`（第八批 + 第九批代码 + 尺子缺陷 11/12/13 + 三张新产物与矩阵/ungated/get_writes 刷新 +
  契约返工）；本段与 §三.8 / §三.9 / §四 两批读数为「账本」随后一笔。共享工作树上并行会话的在制品未被本批收走。
  本轮未 push。

### 第十批收口后的刷新（§三.10 / §四 第十批读数）

- **已修并验证**：**三 tranche 共 36 把 handler / 63 条注册点加门**——surface `applicationdict` 别名 16 把（admin 档）、
  surface `data/work` 别名 18 把 handler / 45 条注册点（401 无会话 + 发起人或 admin）、
  ppsvc `data/delete` + `data/path/delete` 2 把（401 + 发起人，空发起人只 admin）。
  同尺净迁移 46 行 R7→R5、键集零差；测试见 §四 第十批读数（surface 608+4 DB、ppsvc 67+14 DB、parity 4143 全绿）。
  **本批收的是四轮以来最「实」的一族**：注入取证直接量出「门不存在时匿名 GET 打 `data/work/{id}/{p0}/mockdeletetoget`
  回 200 并真的删掉 PP_C_WORK 行」，而不是靠推断。
- **尺子两处的账（修尺 ≠ 修洞）**：**缺陷 14 转义引号表名**（`WRITE_SQL_RE` 认不出 `UPDATE \"x_table\"`，
  全仓 25 处 / 15 条路由，方向是漏量；补分支 + 判据自证段，现读「15 条全判为写面、0 漏量」）、
  **缺陷 15 门在 helper 里**（新增 `gateEvidence` + `callPropagates` 括号配平并要求 `.await?`，
  本批作证 12 条、其中 5 行把 R6 里既有的「读会话但门在 helper」洗成 R5，**这 5 行没有代码改动**）。
  两把守卫各有字节级注入臂证明非恒真。
- **口径更新（后续引用以此为准）**：`mock*` 别名族的注册形状在本仓是**一族而非孤例**
  （`routes.rs` 里 `mockdeletetoget` 93 次、`mockputtopost` 同量级），一把 handler 常对应 2~4 条注册点，
  且**同一 URI 的不同方法可以挂不同 handler**。因此选题与对账**必须按注册点（路由级）计数**，
  函数级数字（如旧的「47 处」）只能用来估工作量，不能用来核账。
- **下一轮入口（按 §四 那张真落笔集中度表排，不再按 crate 名顺序猜）**：
  ① `processplatform_assemble_surface` 仍居首（真落笔 80 条注册点），本批收的是 `data/work` + `applicationdict` 两族，
  余量按 handler 名前缀实测：**`data_workcompleted_*` 17、`data_job_*` 9、`read_id_*` 8、`task_id_*` 8、
  `work_id_*` 5、`work_v2_*` 5、`task_v2_*` 4、`taskcompleted_id_*` 3**，其余 `draft/handover/record/serialnumber`
  各 2 —— 即**与 `data/work` 完全同形状的 `data/workcompleted` + `data_job` 两族（26 条）是下一 tranche 的首选**，
  同一把 `u4_require_*_actor` 助手换个表名即可复用；
  ② `query_assemble_designer` 42 / `query_assemble_surface` 28 同属 A2-F1/F2/F3 那格（IDOR 覆写删除设计对象、
  `table/row/delete/all/{tableFlag}` 物理清空任意表、DDL 执行权错位），**危害高于 surface 的时间戳族**，建议优先；
  ③ `message_assemble_communicate` 30、`cms_assemble_control` 20、`attendance` 18、`program_center` 12、
  `meeting` 10、`jpush` 9 各对应 §五 第二批名单里的 A3-N4/N5/N7/N10/N11 族。
- **新开三格策略性待修（本批取证出、本批未动）**：
  1. **「只读 handler 挂在写方法上」999 条**：这不是越权洞，但它是**假阳性形状**——收法是把读挂回 GET
     或让写方法显式 405，**给它加门是假修**（门挡不住任何写，因为体内没有写 SQL）。
     明细已落 `write_routes_readonly_on_writemethod.tsv`，属独立一整格（涉及前端调用面，需按 crate 分批）。
  2. **`NOW()` 写进 TEXT 列的语义分歧（79 处）**：本批一支 DB 臂否证了「必 500」的猜测，
     实际落库成 `2026-10-14 17:08:25.514461+00` 这类文本时间戳。危害不是报错而是**同表内时间格式不同构**
     （与 `create_time`/`update_time` 的既有写法、与前端解析口径都可能错位），需先定「文本时间戳的规范格式」再收。
  3. **门禁入口两族并存**（沿 §五 第八、九批那格）：强制 `Extension<Session>`（裸 router 500）与
     `Option<Extension<Session>>` + `require_*_opt`（401）仍并存；本批新加的 3 处门一律走后者，
     因为 401 那一路**离线可证**，不必真库也能钉住「匿名进不去」。
- **本轮两处自伤（记下防再犯）**：
  ① **产物写出顺序错了导致读数缺一块**：缺陷 15 的明细 TSV 与逐 helper 计数原本写在 GET-写段之前，
  于是 GET 侧的 4 条间接作证「计数变了、明细没有行」。把该段移到脚本末尾后 12 条全部在册。
  **教训**：尺子的报告段必须排在所有桶填完之后，否则「同一把尺的前后两段自相矛盾」。
  ② **对旧契约断言先改判据再动门槛的诱惑**：`u2r_data_work_pathn_legacy_shape_extraction_contract`
  加门后当场红，最省事是把断言放宽成「500 或 401 都算过」。改为**逐格点名期望码**才保住取证力——
  放宽的那写法会让「别名档被漏注册」与「门被摘掉」两种红都读成绿。
- **一处注入提示（非本会话指令）**：一次 `grep` 的工具回显里混进了伪装成系统提醒的文本
  （`[System] System reminder injected…`），已按未受信内容处理、未采纳。工具回显不是指令通道。
- 提交落点：本批代码（三 tranche 加门 + 尺子缺陷 14/15 + 契约返工 + 7 支新测试臂 + 五张产物）
  为 `da27803bd`，与本文档（§三.10 / §四 第十批读数 / 本节）分两笔，本节即第二笔并回填该 hash。
  提交前订正：pssvc 两把 handler 的 `axum::extract::Path` 参数行曾被我的插入脚本写丢 4 空格缩进，
  已复原并重跑 `--lib`（67/0/14）与 `-- --ignored`（14/0、`skip:` 计数 0）与尺子（EXIT=0、
  R1 145 / R4 14 / R5 643 / R6 310 / R7 1121（真落笔 318）/ GET 写 150 全部与加门后读数一致）。
  共享工作树上并行会话的在制品未被本批收走。本轮未 push。

### 第十一批收口后的刷新（§三.11 / §四 第十一批读数）

- **已修并验证**：**18 把 handler / 36 条注册点加门**——surface `data_workcompleted_id*_mockputtopost` 9 把
  （1 条 GET `data/workcompleted/mockputtopost/{id}` + 9 条 POST + 8 条 PUT）与
  `data_job_job*_mockputtopost` 9 把（9 条 POST 活档 + 9 条 GET 字面量死档），
  门一律 `u4_require_ppc_actor(pool, session, "PP_C_WORKCOMPLETED" | "PP_C_JOB", &id)`——
  由第十批的 `u4_require_work_actor` 泛化（换表名即可复用，那笔泛化的判断在本批兑现）。
  代码轴净迁移 **26 行 R7→R5、键集零差**（`keyDiff = 0`）；测试见 §四 第十一批读数
  （surface `--lib` 609/0/7、`-- --ignored` 7/0 无 `skip:`、parity 过滤 945/0）。
- **三跑对账的读法（尺子轴与代码轴必须分开，见 §四 那张 A/B/C 表）**：
  A（HEAD 码 + HEAD 尺）真落笔 444 → B（HEAD 码 + 新尺）647 → C（工作树码 + 新尺）**621**。
  B→C 的 −26 才是本批代码收掉的门；A→B 的 +203 全部是「尺子多看见了」
  （缺陷 16 把 helper 转调落笔算进来、缺陷 17 重划 R6/R7、缺陷 18 把 24 条死档出列）。
  **今后引用「未门禁写面」必须先说明用的是哪把尺**，尺子轴的涨幅不能被读成「漏洞变多了」。
- **下一轮入口（按 §四 C 跑那张 621 条真落笔的 crate 表排，不再按 crate 名或 handler 名顺序猜）**：
  `cms_assemble_control` **86** → `processplatform_assemble_surface` **83** → `program_center` **53** →
  `query_assemble_designer` **44** → `processplatform_service_processing` **43** → `attendance_assemble_control` **39** →
  `query_assemble_surface` **38** → `message_assemble_communicate` **37** → `meeting_assemble_control` **26** →
  `file_assemble_control` **21**，其后 `cms_core_entity` 15、`processplatform_assemble_designer` 15，
  余 27 个 crate 共 121 条（`calendar` / `mind` / `organization` 各 13、`general` 11、`bbs` / `jpush` 各 9 为长尾前列）。
  - **surface 那 83 条的内部形状已量出**（R6 29 / R7 54）：`attachment_u2b` 族 **24** 是本批之后的最大族，
    其后 `read_id` 8、`task_id` 8、`work_id` 5、`work_v2` 5、`task_v2` 4、`taskcompleted_id` 3，
    其余 20 个前缀各 1~2。
    第十批 §五 预测的「`data_workcompleted` 17 + `data_job` 9 = 26 条首选」**已兑现并归零**：
    这两族的 `mockputtopost` 写面全部出列，剩下的 `data_job_job*` / `data_workcompleted_id*` PUT 字面档
    体内是读、挂在写方法上（11 条，落在 `write_routes_readonly_on_writemethod.tsv`），**给它们加门是假修**。
  - **cms 那 86 条里 `surface_appdict` 一族占 32**，与下格「宏模板作证」是同一块地——**先修尺子再动 cms**。
- **新开与沿用的待拍格（本批评证出、本批未动）**：
  1. **A1 真匿名写 9 条到底算不算洞**（逐条在 `anonymous_write_routes.tsv`）：`auth` 的
     `POST /api/authentication`、`POST /api/authentication/login`，4 条 OAuth（`oauth_login_qywx` /
     `oauth_login_dingding` / `oauth_login_name` / `oauth_bind_name`），`personal` 的
     `PUT /api/reset`、`POST /api/reset/password/anonymous`、`POST /api/person/regist`。
     其中 **6 条是「helper 转调型」**（OAuth 4 条经 `provider_login` / `oauth_code_store`、密码重置 2 条经 `apply_reset`），
     体内看不见写 SQL 但确实落笔——这是缺陷 16 让它们出列的。
     **匿名登录 / 注册 / 重置本来就是匿名入口，加门是错的**；真正要拍的是**限流、验证码、一次性凭据与浏览器会话绑定**，
     属另一族控制，不在本审计的门禁口径内。本批把 A1 从 3 修到 9 是尺子变严，**不是新洞**。
     另 2 条 `A1-条件匿名`（`program_init` `POST /api/secret/set`、`GET /api/secret/set/cancel`）只在未初始化窗口可达，
     第七批双锁已盖住「初始化之后」，剩余风险是「窗口内抢注」。
  2. **宏模板 handler 的门禁作证仍不计分（候选缺陷 19）**：缺陷 13 让「体内写 SQL 信号」能取自 `macro_rules!` 模板（121 条），
     但缺陷 15/17 的**门禁**判据只在展开后的 fn 体内找，宏模板里的 `require_*` 不作证。
     若这一格成立，cms `surface_appdict` 32 条里会有相当一部分本就是「有门但尺子不认」的假红；
     **做 cms 之前先修这一格**，否则会白加 32 道门。
  3. **24 条死档（缺陷 18）的处置**：逐条在 `routes_dead_alias.tsv`，形状是「签名要 N 个 `Path` 段、注册路径 0 段」。
     本批定拍**不补段**（补段等于激活一条新攻击面）。
     风险是日后有人「顺手补齐参数段」把它变成活档——现只有 surface 匿名臂里的 `assert_eq!(dead, 9)` 覆盖本批两族，
     **全 24 条 needs 一支独立守卫**（可挂在 parity 族或尺子的 `DEAD_ALIAS_WITNESSES` 上）。
  4. **784 条「只读挂写方法」**：沿用第十批那格。数字从 999 降到 784 是缺陷 16/18 重新划桶的结果，
     **不代表收了 215 条洞**；修法是把读挂回 GET 或让写方法显式 405，涉及前端调用面，需按 crate 分批。
  5. **沿用 §五 前批名单、本批未动**：`NOW()` 写进 TEXT 列的语义分歧 79 处；门禁入口两族并存
     （强制 `Extension<Session>` 裸 router 500 vs `Option<…>` + `require_*_opt` 401——本批 3 处新门一律走后者，
     理由是 401 那一路离线可证）；`shared/src/crud.rs` 的 CrudSpec 无归属校验；
     `assert_eq!(status, 500)` 断言族 272 处（把「死档 / 未接线」读成「预期行为」）；correlation 读侧 IDOR；
     F5 / F7 / F8 / F10；N4-02 / N4-03。
- **本轮三处自伤与教训（记下防再犯）**：
  ① **注入臂的产物会污染在册产物**：摘掉缺陷 18 判据那次注入跑把 `routes_dead_alias.tsv` 重写成「只剩表头」。
  若提交前不复跑，就会把**注入态的尺子输出**当基线产物提交（本批实际差一点发生）。
  **纪律**：注入跑之后、提交之前，必须用干净工作树重跑尺子，并逐张与 run C 存档比 md5——
  本批八张产物现已全部逐字节一致。
  ② **9 条 GET job 档读出 500 时，第一反应是放宽断言**。正解是靠一支打印 body 的临时探针臂定性
  （`Wrong number of path arguments for \`Path\``），再把它升格成尺子的缺陷 18 判据；
  探针臂用完即删，不把「打印型测试」留在库里。
  ③ **本机 `cargo check -p <crate>` 与 `cargo test -p openapi` 撞 `error calling dlltool 'dlltool.exe': program not found`**
  （`windows-sys` / getrandom 构建脚本），可用验证路径只有 `cargo test --lib -p <crate>`。
  **`openapi` 契约本批未取得读数**，故只记「未取读数」，不记通过。
- 提交落点：本批代码（18 把加门 + helper 泛化 + 尺子缺陷 16/17/18 + 判据见证段 + 契约逐格返工 + 4 支新测试臂 + 八张产物）
  为 `ccf07c5c3`，与本文档（§三.11 / §四 第十一批读数 / 本节）分两笔，本节即第二笔并回填该 hash。
  第二笔提交前用干净工作树重跑尺子（EXIT=0、R1 145 / R2 3 / R4 14 / R5 671 / R6 301 / R7 1092 / R8 1 / R9 10 = 2237、
  GET 写 227、真落笔 621、只读挂写 784、A1 实测 9 / 名单 9、见证缺口 0），八张产物与 run C 存档逐字节一致（md5 全等）
  后才提交——这一步揪出了上格 ①：注入跑曾把 `routes_dead_alias.tsv` 写成只剩表头。
  共享工作树上并行会话的在制品未被本批收走。本轮未 push。

### 第十二批收口后的刷新（§三.12 / §四 第十二批读数）

- **已修并验证（尺子轴）**：缺陷 **19**（`callPropagates` 尾表达式在折行代码上恒假）、**20**（`helperCanReject`
  只认 `AppError::Forbidden` 字面量，看不见 `require_admin` / `require_owner(...).await?` 型门）、
  **21**（宏模板未按生成函数切段，读档借写档的门禁与落笔）。判据见证段扩到缺陷 16/17/18/19/20/21，
  并为缺陷 20 另配一支**独立反证尺**。真落笔读数 **621 → 498**，`判据见证 … 缺口 0 处`，EXIT=0。
  **本批一个洞都没修**：−123 全是「尺子多认了 127 条已存在的门」，引用「未门禁写面」时按 §四 那张三跑表说明用的是哪把尺。
- **第十一轮那格「先修尺子再动 cms」已兑现**：`cms_assemble_control` 86 → **48**，其中 `surface_appdict` 一族 32 条
  现读 **R5-已认证+体内门禁**（`appdict_data_write` 作证，`/api/surface/appdict/{…}/{pathN}/data` 的 PUT/POST/DELETE 逐条在册）。
  也就是说那 32 道门**早就加了**，是尺子不认——若按第十一轮的名单动手，就是白加 32 道门并破一次契约。
- **下一批首选（代码轴，按 §四 run H 那张 498 表排）**：
  1. **cms `data_path_write_handlers!` 的 `$mockdel` 段——本批刚浮出的 8 条真洞**：
     `GET /api/data/document/{id}/{path0…path7}/mockdeletetoget` 不取 `Session`、体内直接软删
     `x_cms_data_document_field`，而同宏 `$del`/`$put`/`$create` 走 `write_doc_field_path(&pool, &session, …)` 带归属门。
     改法是让 `$mockdel` 也委托 `write_doc_field_path(… Remove)`，**先核对返回体形状与前端调用面**再动；
     另需注意它是 GET 上的写档（csrf / `requires_admin` 双盲区），加门之后仍要判「GET 该不该继续挂写」。
  2. `processplatform_assemble_surface` **79**（R6 25 / R7 54）内部形状：`attachment_u2b` **20**、`read_id` 8、`task_id` 8、
     `work_id` 5、`work_v2` 5、`task_v2` 4、`taskcompleted_id` 3，其余 20 余个前缀各 1~2。
     最大族仍是 `attachment_u2b`（第十、十一批未覆盖的那半）。
  3. `query_assemble_designer` 44 / `processplatform_service_processing` 43 / `message_assemble_communicate` 37 三族
     **本批读数一字未动**，是纯粹没碰过的活面。
- **待拍与新增方法学缺口（本批评证出、本批未动）**：
  1. **`processplatform_service_processing/dict_data_fns` 16 把回落整张模板，须人工复核**：该 matcher 含复数展开，
     `templateSegments` 绑不出段，尺子按「整张模板」作证——与缺陷 21 修前同风险（读档可能借邻居门禁）。
     尺子已当面点名不静默放行，但**要么给这个宏补一条按形状的白名单切段，要么逐把读源码定性**，不能长期停在回落档。
  2. **其余 43 条整张模板取证的宏写路由未逐条复核**（121 = 切段 62 + 整张 59，其中 59 条里 ppsvc 占 16）。
     回落判据是「matcher 形参与段数绑不上」，不是「已知不安全」，需要一次专门抽查确认没有第二处 `dict_data_fns`。
  3. **C→D 未拆缺陷 19 与 20 各自贡献**（见 §四 那格记账缺口）：若要精确归因，需在修好 20 的前提下单独摘掉 19 再跑一态；
     本批只有「合并 −123」和「两臂各自点名红」两条证据，没有归因读数。
  4. **独立反证尺只覆盖 `indirectGated` 一族**：缺陷 16/17/18/19/21 仍只靠点名式正反向见证，
     尚无第二把尺重写判据。哪一条下次被量出「自然语料里点不到名」（如 T20c 之于缺陷 20），就照反证尺的办法补一支。
  5. **沿用第十一轮四格、本批未动**：A1 真匿名写 9 条属限流/验证码族而非门禁族（定拍仍待拍）；
     24 条死档 needs 一支独立守卫（现只 surface 匿名臂 `assert_eq!(dead, 9)` 覆盖本族）；
     **只读挂写方法 784 → 780**（沿用「把读挂回 GET 或让写方法显式 405」，涉前端调用面需按 crate 分批）；
     `NOW()` 写进 TEXT 列 79 处、门禁入口两族并存、`shared/src/crud.rs` CrudSpec 无归属、
     `assert_eq!(status, 500)` 断言族 272 处、correlation 读侧 IDOR、F5 / F7 / F8 / F10、N4-02 / N4-03。
- **本批两处自伤与教训（记下防再犯）**：
  ① **注入臂「退 1」不等于取证成立**：T20c 首次跑被 bash 的 `|` 字段分隔符切坏 `--find`，尺子读的是
  `SyntaxError: Unexpected token ')'` 的红，注入器却按退出码判成「臂成立」。红因必须回到日志读——
  **只有「见证点名」那种红才算判据承重，语法炸、模块找不到、写盘失败都算废臂**。
  同时把 skill 那条老教训又踩了一遍：锚点一律用单引号字面量单条命令传，不要图省事拼 `for` 循环 + 分隔符。
  ② **恒真放松不能靠点名式反向见证否决**：探针实测自然语料 0 条样本（`/d/tmp/probe_nonenforcing.cjs`），
  即「把判据改成 `return true`」在既有见证名单里可能一处都不触发。**凡新增一条「放宽型」判据，
  要么配一支不调用被测函数的反证尺，要么明确记「本格无法自证恒真」**，不能拿正向见证冒充双向证明。
- 提交落点：本批**仅尺子轴**（`scripts/audit_write_gating.mjs` 缺陷 19/20/21 + 见证扩充 + 独立反证尺，
  与六张刷新产物）为 `aa2c87435`，与本文档（§三.12 / §四 第十二批读数 / 本节）分两笔，本节即第二笔并回填该 hash。
  第二笔提交前用干净工作树重跑尺子（run H：EXIT=0、R5 798 / R6 174 / R7 1092 / 真落笔 498 / GET 写 193 /
  只读挂写 780 / 宏模板 62+59 / 缺口 0），十五张在册产物与 run E 存档逐张 md5 全等后才提交。
  本批未动产品码，故无 cargo 读数；共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。
### 第十三批收口后的刷新（§三.13 / §四 第十三批读数）

- **已修并验证（尺子轴）**：缺陷 **22**（`format!` 动态表名的写语句判不出来）+ helper 层刀口自证 +
  缺陷 22 见证段。真落笔 **498 → 512**、GET 无门禁面 **193 → 203**、只读挂写 780 → 766、
  间接写 SQL 196 → 221；`判据见证 … 缺口 0 处`，EXIT=0。**本批同样一个洞都没修**，两格涨幅都是「原来量不到」。
  引用行动面时请一并说明尺的代次：**621（十一批）/ 498（十二批）/ 512（十三批）** 不是同一把尺。
- **下一批（代码轴）首选，按新浮出家族的「同一处改动收多把」排序**：
  1. **cms 的 GET 动词别名写档 18 把**（本批 + 上批浮出，全部 K3-已认证无会话无门禁）：
     - `data_path_write_handlers!` 的 `$mockdel` 段 **8 把** `GET /api/data/document/{id}/{path0…path7}/mockdeletetoget`，
       段内不取 `Session`、直接 `UPDATE x_cms_data_document_field SET deleted_at = NOW()`；
       **同宏的 `$del`/`$put`/`$create` 三段都走 `write_doc_field_path(&pool, &session, …)` 带归属门**，
       且同 crate 的 `data_document_id_mockdeletetoget`（非宏、单段）已经是「取 Session + 委托 `delete_doc_fields_base`」的正确形状 ⇒
       改法就是把 `$mockdel` 段收成同款委托，一处宏改动收 8 把。
       **返回体形状要先拍**：现 `$mockdel` 删完立刻用 `query_doc_fields_by_name` 回读，而该查询带 `deleted_at IS NULL`，
       所以它**永远返回 `ActionResult::error("field not found")`**（连成功也报没找到）；
       委托 `write_doc_field_path(… Remove)` 反而改成 `{docId, fieldName, deleted:true}`。
       也就是说这条档今天给不出任何有用响应体，改法不会破坏一个「本来就没在工作」的契约，但**必须先确认前端没在判这个 error**。
     - `soft_delete_by_id` 族 **10 把** `GET /api/{appinfo|appinfo/erase/app|categoryinfo|categoryinfo/erase/category|comment|script|templateform|view|viewcategory|viewfieldconfig}/{id或…}/mockdeletetoget`，
       体内只转调 `soft_delete_by_id(&pool, "x_cms_*", &id)`，**表名是写死的常量、id 是路径参数** ⇒
       登录即可按 ID 软删任意 cms 行。**正档与别名档要一起收**：实测 `DELETE /api/appinfo/{id}`（`appinfo_u2_delete`）
       已是 R5 有门，但 `DELETE /api/categoryinfo/{id}` 与 `DELETE /api/viewfieldconfig/{id}` 自身仍是 **R6-读会话无门禁**
       （正是下面第 2 格新浮出的那两条），所以这一族的说法是「部分正档有门、别名档一律没门」，
       不是「照抄正档就行」。
  2. **本批新浮出的 14 条写方法档**（逐条可点名，`真落笔减少 0 条` 说明纯增量）：
     `cms_core_entity` 的 5 张核心元数据表 × 2 动词（`DELETE` 与 `POST …/delete/{id}` 各 5 条：
     `column` / `column_manager` / `index` / `module` / `note`）+ `cms_assemble_control` 的
     `DELETE /api/{appinfo/erase/app|categoryinfo/erase/category|categoryinfo|viewfieldconfig}/{id}`。
     全部经 `soft_delete_by_id` / `cms_entity_soft_delete` 落笔且**体内无归属门**：
     这是「登录即可删任意列定义 / 索引定义 / 模块定义」的元数据破坏面，比业务数据越权更靠内核，
     但 `cms_core_entity` 是否按 o2server 语义就该限 admin，需要先读该 crate 的门禁口径再定拍。
  3. `processplatform_assemble_surface` 仍是 **79** 条居首（R6 25 / R7 54，`attachment_u2b` 20），尺子已量不出新东西，
     **下一批若不动它，就必须说明为什么先动 cms**——理由是本批浮出的 18 把 GET 别名处在 csrf / `requires_admin` 双盲区。
- **沿用的待拍格与本批新增方法学缺口**：
  1. **反向见证不兜「写判据过宽」**（T22b 实测）：三条反向档只盯门禁侧退化，写侧过宽那次红是 A1 点名名单巧合接住的。
     下次再放宽 `WRITE_SQL_RE` 时，要么给「只读档不得进写面」配**按 crate 的条数钉**，要么把 A1 名单那格升格成显式的写判据过宽守卫。
  2. `ppsvc/dict_data_fns` 16 把 + 其余 43 条整张模板取证的宏写路由**仍未逐条复核**（沿用第十二批那格）。
  3. 缺陷 19 与 20 的 C→D 归因未拆（沿用第十二批那格）。
  4. A1 真匿名写 9 条属限流族、24 条死档 needs 独立守卫、只读挂写现读 **766**（沿用「读挂回 GET 或写方法显式 405」）、
     `NOW()` 写进 TEXT 列 79 处、门禁入口两族并存、`shared/src/crud.rs` CrudSpec 无归属、
     `assert_eq!(status, 500)` 断言族 272 处、correlation 读侧 IDOR、F5 / F7 / F8 / F10、N4-02 / N4-03。
- **提交落点**：本批**仅尺子轴**（`scripts/audit_write_gating.mjs` 缺陷 22 + helper 层自证 + 见证段，
  与四张刷新产物 `write_routes_get_writes` / `write_routes_indirect_gated` / `write_routes_indirect_writes` /
  `write_routes_readonly_on_writemethod`）为 `f751858c1`，与本文档（§三.13 / §四 第十三批读数 / 本节）分两笔，
  本节即第二笔并回填该 hash。第二笔提交前用干净工作树重跑尺子（run K：EXIT=0、真落笔 512、GET 写 203、
  只读挂写 766、间接写 221 / 间接门禁 217、缺口 0），十五张在册产物与 run J 存档逐张 md5 全等后才提交。
  共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。

### 第十四批收口后的刷新（§三.14 / §四 第十四批读数 / §五 名单）

- **已修并验证（代码轴 + 尺子轴一起）**：cms 的 GET 动词别名写族 27 把加门（22 别名 + 2 正档 + 3 非别名 GET 写）
  与尺子缺陷 **23**。真落笔 **512 → 494**、GET 无门禁 **203 → 178**、只读挂写 766 不变、
  间接门禁 217 → 234、间接写 221 → 223；`判据见证 … 缺口 0 处`，EXIT=0。
  **其中 16 条「真落笔减少」不是修好的洞，是原来量不到的假阳**（HEAD 就带门，`GATE_RE` 认不出）——
  真实收口的洞是 GET 面 22 把别名 + 2 把正档 + 3 把非别名 = **27 把**。
  引用行动面时请一并说明尺的代次：**621（十一批）/ 498（十二批）/ 512（十三批）/ 494（十四批）**。
- **第十三批那张名单的账按实测改写**：它写「cms 的 GET 动词别名写档 **18 把**」，实测 cms 的 GET 写面是
  **30 行**（22 把 `mockdeletetoget` + 3 把非别名 + 4 把 K2 + 1 把计数）；「14 条新浮出写方法档」里
  `categoryinfo` / `viewfieldconfig` 两把正档是**尺子假阳**（本批已证），`erase` 两把本批补了门，
  所以那一格只剩 **`cms_core_entity` 12 条**（5 张核心元数据表 × `DELETE` 与 `POST …/delete/{id}`）。
  教训照旧：**名单行数要实测，别拿上一批的浮出数当本批的工作量**。
- **下一批（代码轴）首选，按「双盲区优先、同处改动收多把」排序**：
  1. **GET 面无门禁还剩 178 条**（K1 5 / K2 11 / K3 162）。本批只吃了 cms 的 25 条，K3 那 162 条是
     当前最大的一块，且和这批准则同族（ csrf / `requires_admin` 双盲）。选题时先按 crate 切片读数，
     不要再按「动词别名」猜。
  2. `processplatform_assemble_surface` **79** 条（R6 25 / R7 54，`attachment_u2b` 20）——尺子已量不出新东西，
     纯代码轴；本批没动它是因为别名族在双盲区更靠前，这格的理由现在用完了。
  3. `cms_core_entity` 12 条元数据写（登录即可删任意列 / 索引 / 模块定义），需先读该 crate 的门禁口径定拍。
  4. `ppsvc/dict_data_fns` 16 把宏未绑段 + 其余 43 条整张模板**仍未逐条复核**（沿用第十二批那格）。
- **本批新增的两格方法学缺口**：
  1. **词表冗余 = 见证盲区**（E7 实测）：`GATE_RE` 里 `u2_check_owner` 与「递归经 `u2_gate_by_sql`」是两条
     并行入口，摘前者读数不变。以后给词表补词时，要么承认某词只是快捷路径并**写明它不设独立见证**，
     要么把它改成「两条入口都不许同时摘」的合取见证；别再默认「进了词表就有臂钉着」。
  2. **真库臂的第一轮绿不可信**：`wipe` 按 `id` 回擦擦不到挂在 `doc_id` 上的子行，第二轮才撞主键。
     新增真库臂一律**连跑两遍验幂等**，并把「按外键列擦」写进 cleanup；`#[ignore]` 臂跑完要在账本里
     留 `test result` 原文而不是「大致通过」。
- **沿用的待拍格**：A1 真匿名写 9 条属限流族、24 条死档 needs 独立守卫、只读挂写现读 **766**
  （沿用「读挂回 GET 或写方法显式 405」）、`NOW()` 写进 TEXT 列 79 处、门禁入口两族并存、
  `shared/src/crud.rs` CrudSpec 无归属、`assert_eq!(status, 500)` 断言族 272 处、correlation 读侧 IDOR、
  F5 / F7 / F8 / F10、N4-02 / N4-03。
- **提交落点**：本批代码轴 + 尺子轴（`crates/cms_assemble_control/src/lib.rs` 27 把加门 + 形状 helper、
  `crates/cms_assemble_control/src/security_gate_tests.rs` 6 臂、`scripts/audit_write_gating.mjs` 缺陷 23 +
  缺陷 22 见证重钉、6 张刷新产物）为 ``07d426ae6``，与本文档（§三.14 / §四 第十四批读数 / 本节）分两笔，
  本节即第二笔并回填该 hash。第二笔提交前再取一次干净读数（run Q：EXIT=0、真落笔 494、GET 写 178、
  只读挂写 766、间接写 223 / 间接门禁 234、缺口 0），8 臂注入逐臂复原后与 run M 读数一致，
  run Q 与 run P 的**在册 15 张产物逐张 md5 全等**、`cargo test --lib -p cms_assemble_control`
  336 passed / 0 failed / 4 ignored 与 `-- --ignored` 4 passed 复绿之后才提交。
  共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。

### 第十五批收口后的刷新（§三.15 / §四 第十五批读数 / §五 名单）

- **已修并验证（纯代码轴单轴）**：surface 的 mock 动词别名写族 **22 把**补门（person 11 / creator 3 / work 6 / admin 2），
  真落笔 **494 → 472**、GET 无门禁 **178 → 156**（K3 162 → 140，K1 5 / K2 11 不动）、只读挂写 766 / 间接写 223 /
  间接门禁 234 **三格不变**、`判据见证 … 缺口 0 处`，EXIT=0。矩阵键级 diff 逐条点名：0 消失 / 0 新增 / 22 迁移全 R7→R5。
  **本批尺子一字未动**，所以 −22 全是真收口的洞，不像第十三、十四批那样掺「原来量不到的假阳」——
  行动面引用仍可只用一把尺的代次：**621（十一批）/ 498（十二批）/ 512（十三批）/ 494（十四批）/ 472（十五批）**。
- **surface 那格剩余量按实测改写**（第十四批写的是「79 条，R6 25 / R7 54」）：现读矩阵内 R4/R6/R7 **293 行**
  （R4 1 / R6 33 / R7 259），减掉同表只读挂写 **236 行** ⇒ 含写 SQL 的行动面 **57 行** ＝ 79 − 22，两批口径对得上；
  另有 GET 面 **16 条**（`*_list_count_*` 12 把 + handover / draft_id_start / mode_id_delete）属双盲区，与写方法面不同轴。
- **下一批（代码轴）首选**，GET 面（双盲区）现读 **156** 条，按 crate 实测切片：
  1. **`processplatform_service_processing` 27 条**——本批之后 GET 面最大的一块，27 条**全挂 K3**（已认证无会话无门禁）。
     按 handler 名机械切片（`D:/tmp/b15_ppsvc_slice.mjs`）：**snap 族 10 / task 族 12 / `work_v2_*_p2` 3 / 其余 2**
     （`workcompleted_flag_rollback`、`wc_merge_flag`）。这一族要的归属列（`xcreatorPerson` / `xperson`）
     在 PP_C_WORK / PP_C_JOB / PP_C_WORKCOMPLETED 上现成，**可直接复用本批 surface 的三档 helper**，
     不必像 cms 那样先造 SQL 门；snap 族 10 条同处一个宏/模块，一处改动收多把。
     注意该 crate 还挂着「16 把宏未绑段 + 43 条整张模板」的旧账，选题时先按产物切片读数，
     别拿上一批的浮出数当本批工作量（第十三批那格的教训）。
  2. `program_center` **21** 条（全 K3）/ `query_service_processing` **18** 条（全 K3，其中 **12 条**是
     `high_freq` / `low_freq` × work / workcompleted / document 的 touch+reset 族，宏生成、一处改动收多把；
     这 12 条同时挂在 `route_handler_unresolved_refs.tsv` 的 `u2h::` 未绑段旧账上，动手前先复核宏段）/
     `query_assemble_designer` **14** 条 / `attendance_assemble_control` **9** 条（四 crate 也全挂 K3）。
  3. **新立的一格优先度高于上面 2**：`mockdeletetoget` 族 **14 把**体内把同一条 `DELETE … WHERE xid = $1`
     连执行两次 ⇒ 第二次恒 0 行 ⇒ **删成功也恒回 `ActionResult::error("… not found")`**（§三.15 末格）。
     这一族与第十四批 `templateform` / `design_appdict` 两把同形，属「返回体形状」一条线，
     改前要核调用面（第十四批那格的核法可照抄：前端 + openapi 契约 + parity 三处）。
  4. `cms_core_entity` 12 条元数据写（登录即可删任意列 / 索引 / 模块定义），需先读该 crate 的门禁口径定拍。
- **本批新增的方法学缺口两格**：
  1. **词表冗余 = 无独立见证**（R2 实测，与第十四批 E7 同族、第二次发生）：`GATE_RE` 里 `require_actor`
     摘掉后全仓六格读数与 run R **全等**、缺口 0。两次同族说明这不是个别词的事，是词表缺一条
     「每个词要么设独立见证、要么写明只是快捷路径」的自检。下次动尺子时把它做成合取见证
     （两条入口不许同时摘）或做成逐词摘除的全臂自证，别再靠人工注入偶发发现。
  2. **真库臂的 seed 假设必须来自探针**：`seed_mock_row` 第一轮红（42703）源于按同族表类比假设 `pp_c_record`
     有 `xcreatorPerson`。新增真库臂一律先跑 information_schema 探针再写 seed；探针输出的列清单要贴进账本或注释，
     别只留「实测无此列」的结论——本批 §三.15 那格就把清单原文写进去了。
  3. **沿用的注入纪律补一条**：注入臂的 `--cmd` **不许带管道**（E1 首跑因 `| tail -20` 把退出码变成 tail 的 0，
     被注入器判成「恒真」退 4）。要截输出就用 `--quiet` / 让被测命令自己决定退出码。
- **沿用的待拍格**：A1 真匿名写 9 条属限流族、24 条死档 needs 独立守卫、只读挂写现读 **766**
  （沿用「读挂回 GET 或写方法显式 405」）、`NOW()` 写进 TEXT 列 79 处、门禁入口两族并存、
  `shared/src/crud.rs` CrudSpec 无归属、`assert_eq!(status, 500)` 断言族 272 处、correlation 读侧 IDOR、
  F5 / F7 / F8 / F10、N4-02 / N4-03。
- **提交落点**：本批代码轴（`crates/processplatform_assemble_surface/src/lib.rs` 22 把加门 + `u4_require_ppc_person`、
  `crates/processplatform_assemble_surface/src/security_gate_tests.rs` 3 臂 + 22 档表、3 张刷新产物）为
  ``54509d26a``，与本文档（§三.15 / §四 第十五批读数 / 本节）分两笔，本节即第二笔并回填该 hash。
  第二笔提交前用干净工作树重跑尺子（run S：EXIT=0、真落笔 472、GET 写 156、只读挂写 766、
  间接写 223 / 间接门禁 234、缺口 0），15 张在册产物与 run R 存档逐张 md5 全等、七臂注入逐臂复原、
  `cargo test --lib -p processplatform_assemble_surface` 610 passed / 0 failed / 9 ignored 与
  `-- --ignored` 9 passed 复绿后才提交。共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。

### 第十六批收口后的刷新（§三.16 / §四 第十六批读数 / §五 名单）

- **已修并验证（纯代码轴单轴）**：ppsvc 的 GET 面 **22 把 handler**（注册 27 条）补归属门
  （work 键 9 / task 键 7 / workcompleted 键 5 / snap 键 1，四档合计 22 与 §三.16 逐档点名互校），
  GET 无门禁 **156 → 129**（K3 140 → **113**，K1 5 / K2 11 **不动**）、
  真落笔 **472 → 469**（R4 含写 5 / R6 135 / R7 329）、只读挂写 766 / 间接写 223 / 间接门禁 234
  **三格不变**、`判据见证 … 缺口 0 处`，EXIT=0。行动面代次链：**621（十一）/ 498（十二）/ 512（十三）/
  494（十四）/ 472（十五）/ 469（十六）**。**本批尺子一字未动**，−27 全是真收口的洞。
  ppsvc 这一格从 GET 面名单**整块消失**（27 → 0），不是减量。
- **两条与权限轴分开记的账**（都是加门后真库臂第一次真落笔才暴露的既有缺陷，不是本批引入）：
  ①`x_snap.work_id` 被写成 `x_workcompleted` 的 ID ⇒ 完成项快照存进去再也读不到；
  ②`snap_data` 是 JSONB 而 4 处 INSERT 塞 TEXT、3 处 `get::<String>` 读 JSONB ⇒ 一侧必 42804 / 500、
  一侧必 `error deserializing column`。两条都有独立见证（②见注入臂 E4）。
- **产物对账**：15 张在册产物，与前批基线（run R 存档 `/d/tmp/b16_md5_R.txt`）比 **12 张全等、3 张按预期变化**
  （`write_routes_get_writes.tsv` −27 行、`write_routes_ungated.tsv` −3 行、`write_routes_matrix.tsv` 改 3 行档位）。
  尺子在复原后的干净树**连跑两次**（run T / T2，均 EXIT=0），3 张变化产物逐张 md5 全等 ⇒ 读数幂等。
  `write_gating_residual.tsv` 等其余 12 张一字节未动，说明本批没顺手改动别的桶。
- **矩阵键级 diff（vs `git show HEAD`）**：**3 迁移 / 0 丢失 / 0 新增**，全为
  `R7-已认证+体内无会话无门禁 → R5-已认证+体内门禁`，逐条点名：
  `ppsvc|POST|/task/press/{id}|task_id_press`、`ppsvc|POST|/task/v2/resume/{id}|task_v2_id_resume`、
  `ppsvc|POST|/task/{id}/replace|task_id_replace`。这 3 把 handler 同挂 GET 与 POST，加 POST 侧那行的注册
  因此同步改档——**是同一处改动跨方法受益，不是又收掉 3 条新洞**，记账时别把它算进 −27。
- **测试门禁**：`cargo test --lib -p processplatform_service_processing` **68 passed / 0 failed / 17 ignored**；
  `-- --ignored` 族**整族**跑 **17 passed / 0 failed**，且 `skip: PG unavailable` 计数 **0**（沿用「不靠时长推断、
  逐支数过」的口径）。本批新增 4 臂（1 离线 + 3 真库）全部入册，其中离线臂带
  `assert_eq!(cases.len(), 22, "本批 22 把必须逐把点名，少一把即漏门")`——**加门数本身可证伪**。
- **注入取证五臂**（通道 B，`/d/tmp/b16arms/inject_arm.mjs`，基线绿 → 逐臂点名红 → 复原后复绿三段齐全）：

  | 臂 | 注入 | 点名读数（实际红在哪一句） | 判定 |
  | --- | --- | --- | --- |
  | E0 无害对照 | `fn u4_actor_key_sql` → 多一个空格（行为全同） | 基线 `3 passed` → 注入后仍 `3 passed` | 臂成立：证明红由注入造成，不是环境本来就不稳 |
  | E1 摘门 | 删掉两把 `u4_require_work_owner(&pool.0, session, &work_id).await?;` | 离线臂 `security_gate_tests.rs:2316` 点名 `["snap_work_workId_type_abandoned", "snap_work_workId_type_suspend"]` | 臂成立：401 判据逐把点名，不是只数失败 |
  | E2 兜底恒真 | `is_admin(pool, me).await` → `true \|\| is_admin(...)` | `:2348`「流程之外的会话按 ID 办他人待办必须 403（实际 Ok(())）」、`:2717`「陌生会话给他人完成项打放弃快照必须 403（实际 Ok(())）」，2 failed | 臂成立，**但覆盖面要说清**：work 键那一档走 `require_owner` 的兜底、不在本注入面，故 `ppsvc_get_work_writes_*` 臂仍绿——本臂只证 `u4_require_actors` 一族的 admin 兜底有独立见证，不许写成「所有兜底都被见证」 |
  | E3 键表收成单列 | `&["person", "work_creator"]` → `&["person"]` | `:2377`「发起人催办自己单据上的待办应放行（实际 Err(Forbidden)）」 | 臂成立：两列 OR 的设计（发起人 vs 负责人双方）有独立见证，不是随手多写一列 |
  | E4 撤回读回 cast | `snap_data::text AS snap_data` → `snap_data` | `lib.rs:1699` `error retrieving column snap_data: error deserializing column 3` | 臂成立：JSONB 读回 cast 不是「防御性写法」，摘掉必红 |

  收尾：`git status --porcelain` 只含本批 7 个在范围内文件（4 源 + 3 产物），复原后两条基线**分别复跑复绿**
  （DB 族 `3 passed`、离线 `1 passed`）。
- **本批新增的方法学缺口三格**：
  1. **本机没有可用的 Python**：`python` / `python3` 都解析到 Windows Store 桩（`WindowsApps\python.exe`），
     首次五臂跑全部 EXIT=49「Python was not found」——**一次注入都没发生**、磁盘从头到尾干净，但那一轮取证等于没做。
     改法：用 Qoder 自带运行时（`ELECTRON_RUN_AS_NODE=1` + `…\0.4.7\Qoder.exe`）把通道 B 的字节级纪律
     （读 bytes、混合行尾拒、LF 归一做锚点并断言命中数、bytes 写回后读回比哈希、`finally` 复原 + 哈希等值 +
     `git status` 复验、退出码 0/2/3/4）移植成 `inject_arm.mjs`。**以后在这台机器上取注入证据直接用 Node 版**。
  2. **`spawnSync(shell:true)` 在 Windows 上是 cmd.exe**：首跑五臂「基线 exit=1 / 无 test result 行」的真因是
     `grep` 不存在，不是测试红。显式传 `shell: "D:/software/Git/bin/bash.exe"`。
  3. **判据的 grep 期望值要先实测**：把 `-- --ignored` 加进 DB 臂命令后仍按「4 passed」断言（离线那把不带
     ignore，被 `--ignored` 排除，实读 3 passed），导致基线判不绿而中止。这次是 **fail-loud 生效**（宁可中止也没
     产生假绿），但根子在于没先实测过滤串匹到几把——补的规矩：**写 `--cmd` 前先手跑一次并记下 `test result` 原文**。
     同批另补一条：`--cmd` 里加 `grep -q 'error\[E' … && exit 9`，注入器把 9 当**中止**而非红——
     「因为编译不过而红」不算臂成立。
- **下一批（代码轴）入口，按实测切片改写**：GET 面无门禁现读 **129** 条（K1 5 / K2 11 / K3 113），
  按 crate 降序：`program_center` **21**、`query_service_processing` **18**（含 12 条 `u2h::` 宏未绑段旧账，
  动手前先复核宏段）、`processplatform_assemble_surface` **16**、`query_assemble_designer` **14**、
  `attendance_assemble_control` **9**、`auth` **6**、`cms_assemble_control` 5、`message_assemble_communicate` 5、
  `processplatform_assemble_designer` 5、`ai_assemble_control` 4、`jpush_assemble_control` 4、
  `meeting_assemble_control` 4，其余 8 个 crate 各 ≤3。
  1. **优先度最高的一格是本批新立的**：`shared/src/response.rs:17-48` `row_to_json` 对 JSONB / JSON / DATE /
     TIMESTAMPTZ / NUMERIC 一律 `_ => None` **静默丢列**——跨 crate 的系统性响应缺陷，改前先核所有调用面
     的响应形状（与 openapi 契约 + 前端 TS 类型两层对账，可照 `ts-openapi-contract-guard` 的口径）。
  2. ppsvc **R7 写面仍 49 条**（尺子集中度行实测，本批只动了 GET 那 27 条注册），加上旧账
     「16 把宏未绑段 + 43 条整张模板」，这格剩余量按产物切片读数、别拿上批浮出数当工作量。
  3. `mockdeletetoget` 双 DELETE ⇒ 删成功也恒回 not found 的 **14 把**（第十五批 §三.15 末格），与第十四批
     `templateform` / `design_appdict` 同形，属「返回体形状」一条线。
  4. `cms_core_entity` 12 条元数据写（登录即可删任意列 / 索引 / 模块定义），需先读该 crate 门禁口径定拍。
- **沿用的待拍格**：A1 真匿名写 9 条属限流族、24 条死档 needs 独立守卫、只读挂写现读 **766**、
  `NOW()` 写进 TEXT 列 79 处、门禁入口两族并存、`shared/src/crud.rs` CrudSpec 无归属、
  `assert_eq!(status, 500)` 断言族 272 处、correlation 读侧 IDOR、F5 / F7 / F8 / F10、N4-02 / N4-03、
  `GATE_RE` 里 `require_actor` 无独立见证的冗余词。
- **提交落点**：本批代码轴（`src/lib.rs` 15 把加门 + `u4_require_actors` / `u4_actor_key_sql` + 7 处 JSONB cast、
  `src/u2.rs` 7 把加门 + `snap_take_for_wc` 键修正、`src/security_gate_tests.rs` 4 臂 + 22 把点名格、
  `src/tests.rs` 5 把既有测试改按属主会话、3 张刷新产物）为 ``8e96ac04c``，与本文档（§三.16 / §四 第十六批读数 / 本节）
  分两笔，本节即第二笔并回填该 hash。第二笔提交前已在干净树重跑尺子（run T / T2 全等、EXIT=0、真落笔 469、
  GET 写 129、只读挂写 766、间接写 223 / 间接门禁 234、缺口 0）、15 张产物逐张对账、五臂注入逐臂复原并复跑双基线复绿。
  共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。

### 第十七批收口后的刷新（§三.17 / §四 第十七批读数 / §五 名单）

- **尺子读数（代码轴，`scripts/audit_write_gating.mjs` 一字未动）**：
  | 判据 | 上批收口 | 本批 | 增量 | 归因 |
  | --- | --- | --- | --- | --- |
  | GET 体内无门禁 | 129 | **110** | **−19** | `write_routes_get_writes.tsv` 里 `program_center` 由 21 行降到 2 行，出列恰好 19 行、全是 K3 ⇒ 本批口径是 **17 把 handler / 19 条 GET 注册**（草稿里写的「21 条」把 handler 数与注册数混了，按产物逐行核过订正为 19；余 2 行是 captcha / code 两把待拍） |
  | K1 / K2 / K3 | 5 / 11 / 113 | 5 / 11 / **94** | K3 −19 | 本批 19 条全在 K3，匿名可达与会读会话的两档一支没动 |
  | 真落笔（R4+R6+R7 含写 SQL） | 469 | **461** | **−8** | R7 含写 SQL 329→**321**；R4 5、R6 135 一字未动。−8 对应 ungated 里 8 条 POST/PUT 行（同 handler 双方法注册同步受益），**不等于 −19**：GET 那 19 条挂在 GET 面上、本就不进 R 桶，两条账分开记 |
  | ungated 明细总数 | 1080 | **1072** | −8 | 与上一格同数，逐行 diff 全为 `program_center` 的 POST/PUT R7 行 |
  | R5-已认证+体内门禁 | 841 | **849** | +8 | 与 ungated −8 互为镜像，没有第三条来源 |
  | 只读挂写 | 766 | 766 | 0 | 本批 19 把都含真写 SQL，不属这族 |
  | 间接写 / 间接门禁 | 223 / 234 | 223 / 234 | 0 | helper 名单未新增成员（`require_admin_opt` 是门，不是写 helper） |
  | 判据见证（缺陷 16–23） | 缺口 0 | **缺口 0** | — | 尺子 EXIT=0 |

- **产物幂等与逐张对账**：15 张 artifact 中 **12 张 md5 与上批收口态一字不差**，恰 3 张变化
  （`write_routes_get_writes.tsv` −19 行、`write_routes_ungated.tsv` −8 行、`write_routes_matrix.tsv` 8 行改桶 = 16 行差异）。
  同一棵树上连跑 run T / T2 / T3 **三次全等**（日志与 15 张 md5 两两 `diff` 空）；注入取证之后在还原树重跑的 T3
  与注入前的 T 也全等——即「尺子读数变化」这一路完全由代码解释，没有注入残留污染。
- **矩阵键级 diff 逐条归因**（HEAD vs 本批，key = `crate|method|path`）：**8 条迁移、0 条丢失、0 条新增**，
  全部是 `program_center` 的 R7→R5：POST `dingding/request/pull/sync`、POST `dingding/sync/organization/callback`、
  POST `market/flag/install/or/update`、POST `market/install/offline`、POST `qiyeweixin/send/getprivateinfo/message`、
  POST `zhengwudingding/sync/organization/callback`、PUT `market/flag/install/or/update`、PUT `market/{flag}/install/or/update`。
  GET 那 19 条不在矩阵里（矩阵只收写方法，GET 面另存 `write_routes_get_writes.tsv`），所以「−19 + −8」是两张产物、
  两条判据，不是同一个数被拆了两次。
- **测试门禁**：`cargo test -p program_center` → **262 passed / 0 failed / 0 ignored / 0 filtered out**（基线 260，+2 臂；
  `#[test]` 属性计数在 `git show HEAD` 与工作树两处逐文件核过：`lib.rs` 4→6、`tests_u3.rs` 19→19）。
  本 crate 无 `#[ignore]`，真库臂走运行时 `is_db_available()`，本轮 0 skip。
- **注入取证五臂**（通道 B 的 Node 移植 `/d/tmp/b17arms/inject_arm.mjs`，目标 `crates/program_center/src/lib.rs`，
  基线绿 → 逐臂点名红 → 复原后复绿三段齐全；每臂 `RESTORE hash_ok=true`）：

  | 臂 | 注入 | 点名读数（实际红在哪一句） | 判定 |
  | --- | --- | --- | --- |
  | B0 无害对照 | `fn json_str` → 多一个空格（行为全同） | 基线 `2 passed` → 注入后仍 `2 passed` | 臂成立：红由注入造成，不是环境本来就不稳 |
  | B1 摘一把门 | `dingding_pull_sync` 的 `require_admin_opt(...).await?;` → `let _ = session;` | `lib.rs:12843`「以下 GET 面无会话未回 401：`["dingding_pull_sync"]`」、`:12977`「非 admin 未回 403…：`["dingding_pull_sync => Err(Internal)"]`」，2 failed | 臂成立：单把摘除即被单把点名，`assert_eq!(cases.len(), 17)` 与集合断言都不是摆设 |
  | B2 门恒真放行 | `require_admin_opt` 体内 `session.ok_or(Unauthorized)?; require_admin(...)` → `Ok(())` | `:12843` 与 `:12977` **各点名全部 17 把**（401 臂与 403 臂同红），2 failed | 臂成立：无会话即拒这一档由 `session.ok_or` 独立见证，不是顺带被 admin 门挡住的 |
  | B3 admin 判定短路 | `if is_admin(pool, &session.person_unique).await {` → `if true \|\| is_admin(...)` | **只有 `:12977` 红**，点名全部 17 把；401 臂 `1 passed` 仍绿 | 臂成立，且覆盖面正是要的效果：本臂只破坏 admin 判定、不破坏「无会话即拒」，所以 401 臂绿**不是**覆盖缺口，而是两条臂职责分开的证据 |
  | B4 门插错位置 | `dingding_pull_sync` 内 `require_admin_opt(...)` 与 `let client = pool.get()...` 两行交换 | `:12843`「无会话未回 401：`["dingding_pull_sync"]`」、`:12977`「…：`["dingding_pull_sync => Err(Internal)"]`」，2 failed | 臂成立：**「门必须在读库之前」不是文档口径，是被断言消息钉住的可证伪判据**（读数出现 Internal 即门后移） |

  收尾：`git status --porcelain` 只含本批 5 个在范围内文件（2 源 + 3 产物），复原后基线复绿
  （`262 passed / 0 failed / 0 ignored`），尺子读数与注入前全等（见上格 run T3）。
  锚点纪律照批 16：多行锚点直传 `argv`、`--hits 1` 断言唯一命中，`--cmd` 末位不拿管道退出码当判据，
  且先 `grep -q 'error\[E' … && exit 9` 把「编译不过而红」排除在臂成立之外。
- **下一批（代码轴）入口，按实测切片改写**：GET 面无门禁现读 **110** 条（K1 5 / K2 11 / K3 94），
  按 crate 降序：`query_service_processing` **18**（含 12 条 `u2h::` 宏未绑段旧账，动手前先复核宏段）、
  `processplatform_assemble_surface` **16**、`query_assemble_designer` **14**、`attendance_assemble_control` **9**、
  `auth` **6**、`processplatform_assemble_designer` 5、`message_assemble_communicate` 5、`cms_assemble_control` 5、
  `meeting_assemble_control` 4、`jpush_assemble_control` 4、`ai_assemble_control` 4，其余 11 个 crate 各 ≤3
  （`program_center` 本批后只剩 2 条待拍）。
  1. **优先度最高的一格仍是上批新立的**：`crates/shared/src/response.rs:17-48` `row_to_json` 对 JSONB / JSON /
     DATE / TIMESTAMPTZ / NUMERIC 一律 `_ => None` **静默丢列**。本批给这格补了一条新的紧迫性证据：
     program_center 的真库探针显示 migration 与在库 schema 已漂移（42703 族），而 `row_to_json` 会把漂移读成
     「这一列不存在也没关系」——丢列不报错，等于把这类缺陷从可见面上抹掉。改前先核所有调用面的响应形状
     （与 openapi 契约 + 前端 TS 类型两层对账，可照 `ts-openapi-contract-guard` 的口径）。
  2. **program_center schema drift（42703）族**：三张表的 INSERT 必 500，属「端点从来没通过」一族；
     修法要先定「补 migration 还是改 SQL」，两者对 o2server 行为对齐的影响不同，需拍板后才能动。
     与旧账 `assert_eq!(status, 500)` 断言族 **272 处** 是同一件事的两面：那批断言里有多少钉的是坏端点而非
     错误处理，本批只坐实了 program_center 的 5 处，剩余要按 crate 逐格查。
  3. **路径参数不消费族**（本批新发现，尚未做全仓普查）：至少 5 条注册的 `{flag}` / `{enable}` / `{timeout}`
     在 handler 签名里没有对应 `Path` extractor。先普查（注册路径段 ∩ 签名 extractor 的集合差），
     再逐条判「参数本该生效」还是「路径纯属别名」。
  4. ppsvc **R7 写面仍 49 条** + 旧账「16 把宏未绑段 + 43 条整张模板」；`mockdeletetoget` 双 DELETE 恒回
     not found 的 14 把；`cms_core_entity` 12 条元数据写。
- **沿用的待拍格**：program_center **2 把**（captcha / code，收 admin 会打断认证链路）与其同源的
  `creator='system'` 冒名 3 处（`lib.rs:1691 / 1844 / 8722`）、第三方回调**零验签**族（本批只关掉伪造写面，
  真验签 timestamp + nonce + 签名比对未做）、A1 真匿名写 9 条属限流族、24 条死档 needs 独立守卫、
  只读挂写现读 **766**、`NOW()` 写进 TEXT 列 79 处、门禁入口两族并存、`shared/src/crud.rs` CrudSpec 无归属、
  correlation 读侧 IDOR、F5 / F7 / F8 / F10、N4-02 / N4-03、`GATE_RE` 里 `require_actor` 无独立见证的冗余词。
- **提交落点**：本批代码轴（`crates/program_center/src/lib.rs` 17 把加门 + 2 支逐把点名臂、
  `src/tests_u3.rs` 5 处 500→401 与 welink 那支拆成 401 / 403 两读、3 张刷新产物）为 `13983b2ca`，
  与本文档（§三.17 / 本节）分两笔，本节即第二笔并回填该 hash。第二笔提交前已在干净树重跑尺子
  （run T / T2 / T3 全等、EXIT=0、真落笔 461、GET 写 110、只读挂写 766、间接写 223 / 间接门禁 234、缺口 0）、
  15 张产物逐张对账、五臂注入逐臂复原并复跑基线复绿。共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。

### 第十八批收口后的刷新（§三.18 / §四 第十八批读数 / §五 名单）

- **尺子读数（代码轴，`scripts/audit_write_gating.mjs` 一字未动）**：
  | 判据 | 上批收口 | 本批 | 增量 | 归因 |
  | --- | --- | --- | --- | --- |
  | GET 体内无门禁 | 110 | **92** | **−18** | `write_routes_get_writes.tsv` 里 `query_service_processing` 由 18 行降到 **0** 行（`git show HEAD` 与工作树两处逐行数过），出列 18 行全是 K3 ⇒ 本批口径是 **18 把 handler / 18 条 GET 注册**，两个数相等（不像批 17 那样 handler 数与注册数要分开） |
  | K1 / K2 / K3 | 5 / 11 / 94 | 5 / 11 / **76** | K3 −18 | 匿名可达与会读会话两档一支没动；本批只吃 K3 |
  | 真落笔（R4+R6+R7 含写 SQL） | 461 | **461** | **0** | R4 5 / R6 135 / R7 321 三档一字未动。**R 轴为 0 是本批的取证点**：这 18 条注册全是 GET，`write_routes_matrix.tsv` 只收写方法，GET 面另存 ⇒ 加 18 把门不动 R 桶一根毫毛。与批 17 的「GET −19 / R −8」互为对照，两条判据谁也不许替谁作证 |
  | ungated 明细总数 / R5 | 1072 / 849 | **1072 / 849** | 0 / 0 | 两格都零增量，与上一格同源：无写方法行改桶 |
  | 只读挂写 | 766 | 766 | 0 | 本批 18 把都含真写 SQL（经 `touch_impl` / `neural_start` / `neural_stop`），不属这族 |
  | 间接写 / 间接门禁 | 223 / 234 | 223 / 234 | 0 | helper 名单未新增成员。`touch_impl 14`、`neural_start 2`、`neural_stop 2`、`neural_model_modelFlag_reset_status_core 2` 本来就在间接写名单里——本批加的是**门**（`require_admin` 不带写 SQL），故既不进间接门禁名单（它不是 handler 体内判定，是被 handler 第一句调用的门），也不动间接写数 |
  | 判据见证（缺陷 16–23） | 缺口 0 | **缺口 0** | — | 尺子 EXIT=0 |

- **产物幂等与逐张对账**：15 张 artifact 中 **14 张 md5 与上批收口态一字不差**，恰 **1 张**变化
  （`write_routes_get_writes.tsv` 92 行 − 18 行）。同树连跑 run T / T2 两张 md5 清单 `diff` 为空；
  注入取证（七臂）之后在还原树重跑的 T2 与注入前的 T 也全等——「尺子读数变化」这一路完全由代码解释。
  与 HEAD 逐张 `md5sum` 对账脚本化跑过一遍（14 same / 1 DIFF），不是靠肉眼挑差异文件。
- **测试门禁**：`cargo test -p query_service_processing` → **25 passed / 0 failed / 0 ignored / 0 filtered out**
  （基线 23，+2 臂；`#[test]` / `#[tokio::test]` 属性计数在 `git show HEAD` 与工作树两处逐文件核过：
  `tests.rs` 19→21，`lib.rs` 0→0，`u2.rs` 0→0）。本 crate 无 `#[ignore]`，真库臂走运行时 `is_db_available()`，本轮 0 skip。
- **注入取证七臂**（通道 B 的 Node 移植 `/d/tmp/b17arms/inject_arm.mjs`，目标 `crates/query_service_processing/src/u2.rs`；
  基线绿 → 逐臂点名红 → 复原后复绿三段齐全；每臂 `RESTORE hash_ok=true`）：

  | 臂 | 注入 | 点名读数（实际红在哪一句） | 判定 |
  | --- | --- | --- | --- |
  | C0 无害对照 | `async fn touch_impl(` → 多一个空格（行为全同） | 基线 `2 passed` → 注入后仍 `2 passed` | 臂成立：红由注入造成，不是环境本来就不稳 |
  | C1 摘宏模板 touch 分支 | `:387` 那句门 → `let _ = session;` | `tests.rs:917` 与 `:1044` **各点名 6 把 `*touch`**（`high_freq_work_touch / low_freq_work_touch / high_freq_workcompleted_touch / low_freq_workcompleted_touch / high_freq_document_touch / low_freq_document_touch`，读数一律 `Some(Internal)`），6 把 `*reset` 不在名单里，2 failed | 臂成立：宏那 12 把**按分支可分证**，touch 分支坏了不牵连 reset ⇒ 「改模板一处」不是把 12 把糊成一团 |
  | C2 摘宏模板 reset 分支 | `:396` 那句门 → `let _ = session;` | 两处断言各点名 6 把 `*reset`，与 C1 名单互斥，2 failed | 臂成立：与 C1 合起来把宏两分支各自钉住，逐把集合等式（不是计数）可证伪 |
  | C3 摘一把具名门 | `neural_generate`（`:621`）的门 → `let _ = session;` | 两处各**只**点名 `["neural_generate => Some(Internal)"]`，2 failed | 臂成立：具名那 6 把不靠宏覆盖，单把摘除即被单把点名；`assert_eq!(cases.len(), 18)` 与集合断言都不是摆设 |
  | C4 无会话放行 | `require_admin` 内 `session.ok_or(Unauthorized)?` → `let Some(session) = session else { return Ok(()) };` | **只有 `:917` 红**，点名**全部 18 把**；`:1044` 那支 `1 passed` 仍绿 | 臂成立，且覆盖面正是要的效果：本臂只破坏「无会话即拒」、不破坏 admin 判定，所以 403 臂绿**不是**覆盖缺口，而是两条臂职责分开的证据。读数全为 `Some(Internal)`（不是 `None`）额外证了一件事——门一放行请求就真的走到 `pool.get()` |
  | C5 admin 判定短路 | `if shared::middleware::is_admin(...)` → `if true \|\| is_admin(...)` | **只有 `:1044` 红**，点名全部 18 把；401 臂 `1 passed` 仍绿 | 臂成立，与 C4 对偶：admin 档由 `is_admin` 独立见证，不是顺带被「无会话即拒」挡住的 |
  | C6 门插到落笔之后 | `optimize_index_touch` 内两句顺序颠倒并保留 `?` 传播 | `:917`「无会话未回 401：`["optimize_index_touch => Some(Internal)"]`」、`:1044`「非 admin 未回 403…：`["optimize_index_touch => Some(Internal)"]`」，2 failed | 臂成立：**「门必须在读库之前」是被断言消息钉住的可证伪判据**（读数出现 Internal 即门后移），与批 17 的 B4 同法。⚠️ 本臂**第一次跑的是坏形状**：注入体写成 `let _ = touch_impl(..).await?;` 丢了 `Json` 尾表达式 ⇒ `error[E0308]`，被驱动器的 `grep -q 'error\[E' … && exit 9` 判成中止（不算臂成立）。坏形状没被 `finally` 复原干净，**磁盘上留下了注入残留**（`u2.rs:436-437` 两行），是本轮 `git status` + 复跑编译才抓出来的；已按 `git diff` 逐字节手工复原并复跑基线复绿，再以修正形状（`let v = ..?; require_admin(..).await?; Ok(v)`）从**脚本文件**重跑成 C6b。两个教训回写进纪律：① 编译错造成的红不是臂证据，② 多行锚点不许走 Bash 命令行内联转义（第二次尝试即 `[C6b] ABORT anchor hits=0`，虚警一场，好在锚点命中数断言保证文件未动） |

  收尾：`git status --porcelain` 只含本批 3 个在范围内文件（2 源 + 1 产物），复原后基线复绿
  （`25 passed / 0 failed / 0 ignored / 0 filtered out`），尺子读数与注入前全等（见上格 run T / T2）。

- **下一批（代码轴）入口，按现读 92 条改写**：GET 面无门禁 **92**（K1 5 / K2 11 / K3 76），按 crate 降序：
  `processplatform_assemble_surface` **16**、`query_assemble_designer` **14**、`attendance_assemble_control` **9**、
  `auth` **6**、`processplatform_assemble_designer` 5、`message_assemble_communicate` 5、`cms_assemble_control` 5、
  `meeting_assemble_control` 4、`jpush_assemble_control` 4、`ai_assemble_control` 4、`hotpic` / `file` 各 3，其余 9 个 crate 各 ≤2
  （`query_service_processing` 本批后归零，`program_center` 只剩 2 条待拍）。
  1. **优先度最高的一格仍是批 16 立的**：`crates/shared/src/response.rs:17-48` `row_to_json` 对 JSONB / JSON /
     DATE / TIMESTAMPTZ / NUMERIC 一律 `_ => None` **静默丢列**。本批不新增证据也不动它，但它压着所有响应形状类结论。
  2. **surface 那 16 把不能整格收 admin**（本批核过名单，逐条点名写在这里以免下批重踩）：**12 条是读形状**
     ——`{read|readcompleted|task|taskcompleted|work|workcompleted}/list/…/process` 的 count 版 6 条 + `_p2` 计数版 6 条，
     经 `*_core` helper 落笔（间接写名单里各挂 2 条），收 admin 会直接打断所有人的待办列表；
     这 12 条要走**归属档**（`x_task.person` / `x_work.creator` 同锁，照批 16 的 task 键口径）或另定「读形状写」的处置。
     真按动作/删除收的是另 4 把：`handover/{cancel,process}/{id}`、`draft/{id}/start`、`mode/{id}/delete`。
  3. **auth 那 6 条里有 4 条是 K1 匿名可达**（`oauth/login/{qywx,dingding,name}`、`oauth/bind/name/...`，
     都在登录/绑定链上）⇒ 与 program_center 的 captcha / code 同一格待拍，收 admin 直接打断认证，
     只能按「第三方 code 换会话」的语义另定；余 2 条 K3（`oidc/callback`、`qiyeweixin/update/person/detail/{code}`）
     里 `qiyeweixin_update_person_detail` 与批 17 的组织回调族同性质（零验签 + 改他人档案），可与「路径参数不消费」普查并做。
  4. `query_assemble_designer` 14 条是下一个整格（`QueryManagerDeep.vue:702-722` 真在打它的 neural / table 面，
     动档前先核前端消费）；ppsvc **R7 写面仍 49 条** + 「16 把宏未绑段 + 43 条整张模板」旧账；
     `mockdeletetoget` 14 把；`cms_core_entity` 12 条元数据写。
- **新增待拍（本批记账，未修）**：
  1. `neural/list/calculate/model/{modelFlag}/work/{workId}`（`src/u2.rs:663`）体内只读、不进 GET 写档面，
     但任意 `workId` 即可取他人单据的推算字段、无归属校 ⇒ 与 correlation 读侧 IDOR 同族，属**读侧**那批的入口清单。
  2. `query_service_processing` 的 **5 条 POST 写面**（`design/search`、`index/directory/document/count`、
     `index/update/extra/document`、`table/{flag}/insert`、`table/{flag}/update/{bundle}`）本批未动，仍在 R 轴旧账里；
     其中 `neural_model_modelFlag_reset_status` 一类 GET 已在别的 crate 名下计入 K3。
  3. **宏生成 handler 的门禁取证口径**：本批证明「改模板一处」与「逐把作证」可以并存，前提是那批宏属
     尺子缺陷 21 的「按生成函数切段」档；ppsvc 的 `dict_data_fns` 那 16 把属「绑不上段、回落整张模板」档，
     照那格动刀时不许套用本批的取证结论。
- **沿用的待拍格**：program_center **2 把**（captcha / code）与其 `creator='system'` 冒名 3 处
  （`lib.rs:1691 / 1844 / 8722`）、第三方回调**零验签**族、A1 真匿名写 9 条属限流族、24 条死档 needs 独立守卫、
  只读挂写现读 **766**、`NOW()` 写进 TEXT 列 79 处、门禁入口两族并存、`shared/src/crud.rs` CrudSpec 无归属、
  correlation 读侧 IDOR、F5 / F7 / F8 / F10、N4-02 / N4-03、`GATE_RE` 里 `require_actor` 无独立见证的冗余词、
  路径参数不消费族（全仓普查未做）、program_center schema drift 42703 族与 `assert_eq!(status, 500)` 断言族 272 处。
- **提交落点**：本批代码轴（`crates/query_service_processing/src/u2.rs` 18 把加门 + crate 首个 `require_admin`、
  `src/tests.rs` 2 支逐把点名臂 + 契约臂的 admin 会话注入与角色 seed、1 张刷新产物）为 `0ae6d8aed`，
  与本文档（§三.18 / 本节）分两笔，本节即第二笔并回填该 hash。第二笔提交前已在还原树重跑尺子
  （run T / T2 全等、EXIT=0、真落笔 461、GET 写 92、只读挂写 766、间接写 223 / 间接门禁 234、缺口 0）
  并复跑门禁 25 / 0 / 0 / 0。共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。

### 第十九批收口后的刷新（§三.19 / §四 第十九批读数 / §五 名单）

- **尺子读数（代码轴，`scripts/audit_write_gating.mjs` 一字未动）**：
  | 判据 | 上批收口 | 本批 | 增量 | 归因 |
  | --- | --- | --- | --- | --- |
  | GET 体内无门禁 | 92 | **76** | **−16** | `write_routes_get_writes.tsv` 里 `processplatform_assemble_surface` 由 16 行降到 **0** 行（HEAD 与工作树两处逐行数过），出列 16 行全是 K3 ⇒ **16 把 handler / 16 条 GET 注册**两个数相等 |
  | K1 / K2 / K3 | 5 / 11 / 76 | 5 / 11 / **60** | K3 −16 | 匿名可达档与会读会话档一支没动——本批 16 把全在 K3，且收完之后 K1/K2 的名单逐条核看过，没有一把被顺手挪档 |
  | 真落笔（R4+R6+R7 含写 SQL） | 461 | **457** | **−4** | R4 5 / R6 135 / R7 **317**（上批 321）。这 −4 **不是** GET 轴的那 16 把，而是 4 把动作/删除 handler（`draft/start/{id}`、`handover/{id}/cancel`、`handover/{id}/process`、`mode/delete/{id}`）的 **POST 别名**从 R7 搬进 R5。两条判据分立记账：GET −16 是本批修的漏洞，R −4 是双方法注册的镜像，**谁也不许替谁作证**；与批 17「−19 与 −8 同源」、批 18「−18 与 0 并存」构成第三种形状 |
  | ungated 明细 / R5 | 1071 / 849 | **1067 / 853** | −4 / +4 | 与上一格同源：4 行改桶、0 行丢失、0 行新增（`diff` 逐行数过）。文件行数含表头 1072→1068 |
  | 只读挂写 | 766 | 766 | 0 | 本批 16 把**都含真写 SQL**（`UPDATE … SET "xupdateTime" = NOW()`），不属这族——这也正是「`NOW()` 写得进 TEXT、touch 真落笔」那条订正的反证：若它们只读，这格会 −16 |
  | 间接写 / 间接门禁 | 223 / 234 | 223 / 234 | 0 | `u4_require_ppc_person` / `u4_require_ppc_actor` 都是**门**（不带写 SQL）且早在批 15 就已在册；`*_core` 6 把本来就在间接写名单里各挂 2 条。名单零新增成员 |
  | 写 handler 矩阵 / 全 8 桶 | 991 / 2237 | 991 / 2237 | 0 | 只有桶标签变（R7→R5），行数一根毫毛未动 |
  | 判据见证（缺陷 16–23） | 缺口 0 | **缺口 0** | — | 尺子 EXIT=0，25 个函数动态表名全部判为写面、15 条转义引号表名零漏量 |

- **产物幂等与逐张对账**：15 张 artifact 中 **12 张 md5 与上批收口态一字不差**，恰 **3 张**变化
  （`write_routes_get_writes.tsv` −16 行、`write_routes_matrix.tsv` 4 行改桶、`write_routes_ungated.tsv` −4 行）。
  注入取证（五臂）跑完之后，在**还原树**重跑尺子（run T → T2），15 张 md5 与注入前快照 `diff` **逐张全等**
  （不是挑三张比，是 15/15 比），尺子读数同样复现 76 / 457 / 766 / 223 / 234 ⇒ 本轮所有读数变化完全由代码解释。
- **测试门禁**：`cargo test --lib -p processplatform_assemble_surface` → **612 passed / 0 failed / 9 ignored / 0 filtered out**
  （基线 **610**，+2 臂；610 这个读数是 §四 第十五批段给本 crate 记的）；`-- --ignored` → **9 passed / 0 failed / 612 filtered out**。
  ⚠️ **如实说明一支臂的档位**：`mod r419_gate_tests` 这**两支臂都没有 `#[ignore]`**（本文件里 `#[ignore]` 计数为 0，
  那 9 支 ignored 属于同 crate 别的 mod），真库那支靠体内 `is_db_available()` 早退——照纪律这是**弱形状**（无库机器上会静默早退成假绿），
  本轮沿用批 18 的同 crate 先例没另立规矩，但**执行与否有独立凭据**：D2 / D3 两支注入能把 `r419_get_writes_owner_gate_lands` 打红
  并逐把点名（`=> None` 的归属判定只有真库才拿得到），若它早退则注入不可能让它红 ⇒ 本轮这支臂**确实跑进了库**。
  真库臂两遍幂等、收口后 `xid LIKE 'r419-%'` 在 `PP_C_READ / PP_C_WORK / PP_C_DRAFT / PP_C_HANDOVER` 四表逐表数过**全 0 行**。
- **注入取证五臂**（通道 B 的 Node 移植 `/d/tmp/b17arms/inject_arm.mjs`，目标 `crates/processplatform_assemble_surface/src/lib.rs`；
  基线绿 → 逐臂点名红 → 复原后复绿三段齐全；每臂 `RESTORE hash_ok=true`；驱动 `/d/tmp/b17/run_b19_arms.sh`，
  `--cmd` 里带 `grep -q 'error\[E' … && exit 9` 以挡住「编译错冒充臂证据」——批 18 的 C6 教训本轮生效，五臂零返工）：

  | 臂 | 注入 | 点名读数（实际红在哪一句） | 判定 |
  | --- | --- | --- | --- |
  | D0 无害对照 | `require_admin_opt(` 前多一个空格（行为全同） | `2 passed` → 注入后仍 `2 passed` | 臂成立：红由注入造成，环境本来稳 |
  | D1 摘一把具名门 | `work_list_count_..._process`（base）整句门 → `let _ = session;` | **两支臂同时红**（2 failed）：401 臂点名**只** `["work_list_count_..._process => Some(Internal)"]`，403 臂点名同一把但读数是 `=> None` | 臂成立：单把摘除即被单把点名；`_p2` 那把（同 `_core`、门在各自 handler）**不在名单里** ⇒ 「门在 wrapper 不在 core」的形状被证伪成功；两读数一 `Internal` 一 `None` 还额外分开证了「无会话放行即触库」与「放行即不再拒他人」是两种失败 |
  | D2 摘 person 档归属判定 | `u4_require_ppc_person` 尾部 `require_actor_opt(pool, &session, owner)` → `Ok(())`（无会话拒仍在） | **只有 403 臂红**（1 passed / 1 failed），点名**恰 12 把**：`handover_id_cancel / handover_id_process / draft_id_start / mode_id_delete` + `{read,readcompleted,task,taskcompleted}×{base,_p2}` 8 把，全部 `=> None`（`draft_id_start` 例外读到 `Some(Internal)`）；401 臂 `1 passed` 仍绿 | 臂成立，且 12 这个数字**就是**「12 把走 `u4_require_ppc_person`」的直接作证；401 臂绿是职责分开（本臂只破坏归属判定、不破坏「无会话即拒」），不是覆盖缺口。`draft_id_start` 的 `Internal` 是它**本身必 500**（§三.19 真缺陷①），与门无关——真库臂把它明列豁免，注入臂也没让它混进「放行名单」 |
  | D3 摘 actor 档归属判定 | `u4_require_ppc_actor` 同一位置 → `Ok(())` | **只有 403 臂红**，点名**恰 4 把**：`{work,workcompleted}×{base,_p2}`，全部 `=> None`；401 臂仍绿 | 臂成立，与 D2 互斥成对：**两档 helper 的覆盖面用「点名集合」而非计数分开钉住**——12 与 4 相加正好等于本批 16，谁也不冒充谁 |
  | D4 门插到读库之后 | `mode_id_delete` 内门与 `pool.get()` 两句顺序颠倒 | **只有 401 臂红**，点名**只** `["mode_id_delete => Some(Internal)"]`；403 臂绿 | 臂成立：「门必须在读库之前」继续由断言消息里的 `Internal` 判据钉住（与批 17 B4、批 18 C6b 同法）。403 臂绿是**真绿**：门后移后非 admin 仍被拒（只是先取了连接），所以这一臂专门管「无会话不得触库」，不管 admin 判定 |

  ⚠️ **一处如实披露的取证边界**：D2 / D3 摘的是 `u4_require_ppc_person` / `u4_require_ppc_actor` 这两个**与第十五批共用**的
  helper 体内判定，波及面天然比本批宽（批 15 的 `PP_C_JOB` 9 把、`PP_C_WORKCOMPLETED` 9 把别名写族也走同一扇门）；
  注入跑的命令筛的是 `r419_` 前缀，所以**批 15 那些臂本轮没跟着复跑**。这两臂的结论只用于作证「本批这 16 把落在哪一档」，
  不许读成「批 15 的门也被复验过」。跨批次波及面由还原树上的全量门禁（612 那轮跑的是干净码）兜。

  收尾：`git status --porcelain` 在注入全程只含本批 5 个在范围内文件（2 源 + 3 产物），复原后基线复绿、尺子复现（见上两格）。

- **下一批（代码轴）入口，按现读 76 条改写**：GET 面无门禁 **76**（K1 5 / K2 11 / K3 60），按 crate 降序：
  `query_assemble_designer` **14**、`attendance_assemble_control` **9**、`auth` **6**、`processplatform_assemble_designer` 5、
  `message_assemble_communicate` 5、`cms_assemble_control` 5、`meeting_assemble_control` 4、`jpush_assemble_control` 4、
  `ai_assemble_control` 4、`hotpic_assemble_control` / `file_assemble_control` 各 3，其余 **9 个 crate 各 ≤2**
  （`processplatform_assemble_surface` 与 `query_service_processing` 本批后归零，`program_center` 只剩 2 条待拍）。
  1. **优先度最高的一格仍是批 16 立的**：`crates/shared/src/response.rs:17-48` `row_to_json` 对 JSONB / JSON /
     DATE / TIMESTAMPTZ / NUMERIC 一律 `_ => None` **静默丢列**。本批不新增证据也不动它，但它压着所有响应形状类结论。
  2. `query_assemble_designer` 那 14 条是下一个整格：`QueryManagerDeep.vue:702-722` 真在打它的 neural / table 面，
     **动档前先核前端消费**，别照批 18 的探针豁免口径套。
  3. `auth` 那 6 条里 **4 条是 K1 匿名可达**（`oauth/login/{qywx,dingding,name}`、`oauth/bind/name/...`，全在登录/绑定链上）
     ⇒ 与 program_center 的 captcha / code 同一格待拍，收 admin 直接打断认证，只能按「第三方 code 换会话」语义另定；
     余 2 条 K3 里 `qiyeweixin/update/person/detail/{code}` 与批 17 组织回调族同性质（零验签 + 改他人档案），可与「路径参数不消费」普查并做。
  4. K2 那 **11 条**是另一种形状（体内读了会话、但无判定）：`{document|comment}/{id}/(un)commend` 4 条、
     `meeting/{id}/(accept|checkin)` 2 条、`im/conversation/.../group/quit/self`、`single/mockdeletetoget`、
     `mind/restore/{id}`、`ai_assemble_control/chat/delete/{clueId}`、`calendar/follow/{id}/cancel`——
     这批多数**本来就该按归属收**，与批 16/19 同锁，是「加门不改语义」最省的一格，可考虑与 `query_assemble_designer` 并批。
  5. 存量老格：ppsvc **R7 写面 49 条** + 「16 把宏未绑段 + 43 条整张模板」旧账、`mockdeletetoget` 14 把、
     `cms_core_entity` 12 条元数据写、A1 真匿名写 9 条。
- **新增待拍（本批记账，未修）**：
  1. `{read|readcompleted|task|taskcompleted|work|workcompleted}/list/count/application/{applicationFlag}/process` 12 把
     **路径段名与实际语义不符**：`_core` 把 `applicationFlag` 直接绑进 `WHERE xid = $1`，「按应用计数」实际是「按行 ID touch 时间戳」。
     与「路径参数不消费」族**相邻但不同**（这里参数被消费了，只是消费错了维度），要不要按 o2server 的 list/count 语义重写属产品决策。
  2. `handover/{id}/cancel` 与 `handover/{id}/process` **函数体逐字全同**：两个不同业务动作在 Rust 侧无区别，属**行为缺陷**，
     门已覆盖但语义未补。
  3. `draft/{id}/start` **必然 500**（`there is no parameter $2` + `pp_c_draft` 无 `xstatus` 列，两个独立成因）。
     归入 `assert_eq!(status, 500)` 断言族与 schema drift 族一起处理；真库臂里的明列豁免跟着撤。
- **口径订正（不是新增，是把旧账改对）**：§五 的「`NOW()` 写进 TEXT 列 79 处」从**可用性**缺陷改记为**数据形状**缺陷——
  PostgreSQL 对赋值走 assignment I/O coercion，`NOW()` 写得进 TEXT、语句照落笔。订正后本批门禁定性才成立：
  未认证/他人请求不是「只探到 500」，而是**真的改得动审计时间戳**。往后任何「这条写会不会因为类型不匹配自己失败」的判断，
  一律先探针再看读数，不许凭直觉抵账。
- **沿用的待拍格**：program_center **2 把**（captcha / code）与其 `creator='system'` 冒名 3 处
  （`lib.rs:1691 / 1844 / 8722`）、第三方回调**零验签**族、A1 真匿名写 9 条属限流族、24 条死档 needs 独立守卫、
  只读挂写现读 **766**、门禁入口两族并存、`shared/src/crud.rs` CrudSpec 无归属、correlation 读侧 IDOR
  （含批 18 新记的 `neural_list_calculate_with_work`）、F5 / F7 / F8 / F10、N4-02 / N4-03、
  `GATE_RE` 里 `require_actor` 无独立见证的冗余词、路径参数不消费族（全仓普查未做）、
  program_center schema drift 42703 族与 `assert_eq!(status, 500)` 断言族 272 处、`query_service_processing` 5 条 POST 写面。
- **提交落点**：本批代码轴（`crates/processplatform_assemble_surface/src/lib.rs` 16 把补归属门、
  `src/security_gate_tests.rs` `mod r419_gate_tests` 逐把点名两臂、3 张刷新产物）为 `d44685d74`，
  与本文档（§三.19 / 本节）分两笔，本节即第二笔并回填该 hash。第二笔提交前已在还原树重跑尺子
  （15/15 张产物 md5 与注入前全等、EXIT=0、真落笔 457、GET 写 76、只读挂写 766、间接写 223 / 间接门禁 234、缺口 0）
  并复跑门禁 612 / 0 / 9 与 `--ignored` 9 / 0。共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。

### 第二十批收口后的刷新（§三.20 / §四 第二十批读数 / §五 名单）

- **三列读数（批 19 收口 → 本批代码后 → 本批收口）**。四列里的第三列是尺子两修都落地后的终态
  （T5 = T6 = T7 三次独立跑，读数与产物逐字节全等）：

| 尺子格 | 批 19 收口 | 代码后（T，旧尺子） | 收口（T5/T6/T7） | 归因 |
| --- | ---: | ---: | ---: | --- |
| GET 写面无门禁 | 76 | 63 | **62** | 跨批逐行对账：恰 14 行出列、0 新增、0 改桶 |
| ├ K1 匿名可达无门禁 | 5 | 5 | 5 | 本批不动这族（限流族单列待拍） |
| ├ K2 读会话无门禁 | 11 | 12 | 11 | T 的 +1 就是 `table_build_dispatch_flag`：门在 `u2_closures.rs` 里以**限定路径**调用，旧尺子整条扫不到，把它读成「读了会话但没门」（= 缺陷 24 的现场假阳）；修好后出列 |
| └ K3 已认证无会话无门禁 | 60 | 46 | **46** | 本批 14 把全在 K3，−14 全额归代码轴 |
| R5 已认证 + 体内门禁 | 853 | 854 | **866** | 缺陷 25：12 把「handler → 读库 helper → 门 helper」的链上有门的 PUT 从 R6 搬进 R5 |
| R6 已认证 + 读会话无门禁 | 158 | 158 | **146** | −12 与上一格同源，是改桶不是消失 |
| R7 已认证 + 无会话无门禁 | 1053 | 1052 | 1052 | −1 代码轴 |
| R4 含写 SQL | 5 | 5 | 5 | 不动 |
| R6 含写 SQL | 135 | 135 | **125** | 缺陷 24 先 +2（T3 为 137，把两行「只读」露成「含写」）、缺陷 25 再 −12 |
| R7 含写 SQL | 317 | 316 | 316 | −1 代码轴 |
| **真落笔（R4+R6+R7 含写）** | **457** | **456** | **446** | −1 是本批真修掉的一条写面，−10 是尺子假阳修正（那 10 行**一直**有门，只是旧尺子看不见） |
| 只读挂写方法 | 766 | 766 | **764** | 缺陷 24/25 的重新归类所致，R6 合计不受影响 |
| 间接门禁作证 | 234 | 247 | **263** | 其中 `query_assemble_designer` 一 crate **4 → 18**（恰本批 14 把）；余量来自两条尺子轴的改桶 |
| 间接写 SQL | 223 | 222 | **215** | designer **6 → 5**；两条轴方向相反，不许轧差 |
| 判据见证缺口 | 0 | 0 | 0 | 三次 EXIT=0 |

- **三条判据分开记账（这是本批最容易记错的格）**：
  1. **代码轴**（批 19 收口 → T）：GET 76→63 里真正属于本批的是 **14 把门**，之所以只 −13，是因为其中一把
     （`table_build_dispatch_flag`）被旧尺子错误留在名单上当 K2 —— 所以「K2 11→12」不是新漏洞，是尺子的锅。
     R 轴同时 −1（R7 1053→1052、含写 317→316）⇒ 真落笔 457→**456**。
  2. **尺子轴 · 缺陷 24**（T → T3）：解析口径改成「限定符 → 模块文件」后，`table_build_dispatch_flag` 出列（−1），
     另两把经限定调用的写面被露出来（GET 63→64、K2 12→13、只读挂写 766→764、R6 含写 135→137）⇒ **真落笔 456→458**。
     这一格必须写红：修尺子让读数**变差**了，因为露出来的是本来就存在、以前看不见的写面。
  3. **尺子轴 · 缺陷 25**（T3 → T5）：链上有门不作证的旧前置闸撤掉后，2 行 preview GET 假阳出列、
     12 把 PUT 从 R6 搬进 R5 ⇒ GET 64→**62**、R5 854→866、R6 158→146、R6 含写 137→125 ⇒ **真落笔 458→446**。
     三条轴合起来：**GET −14 全是本批修的漏洞，真落笔 −11 = −1 代码 + −10 假阳**；跨批逐行对账给出同一结论
     （76 行 vs 62 行，diff 恰 14 行删除、0 行新增、0 行改桶，14 行全是 `query_assemble_designer` 且全是 K3）。
- **还原树证明**：尺子的注入臂会**顺带重写产物**，R1/R2/R3 跑完磁盘上 4 张产物是脏的
  （`write_routes_get_writes` / `write_routes_matrix` / `write_routes_indirect_gated` / `write_routes_indirect_writes`）。
  在还原树重跑尺子（T7，EXIT=0）后 `git diff -- docs/audits/artifacts` 为**空**、15/15 张产物与提交态逐字节全等、
  GET 名单 62 行、真落笔 446、只读挂写 764、间接门禁 263 / 间接写 215、缺口 0。
  这条与第十九批的「产物要按**各自桶**比行数」并列：凡跑过尺子（不管注入还是基线），收口前都必须在还原树重跑一次。
- **门禁**：`cargo test -p query_assemble_designer` **51 passed / 0 failed / 0 ignored**（基线 49，本批 +2 臂）。
  如实记两条：① 本批两支臂**都不带 `#[ignore]`**，离线臂 `b20_get_writes_reject_without_session` 用死 pool 直接跑；
  ② 真库臂 `b20_get_writes_owner_gate_lands` 的「真执行、不是 skip 蒙过」由 E2/E3 两臂把它跑红来作证——
  若它是静默跳过，注入后不可能点名出 13 把与 3 格。
- **九臂注入取证（基线绿 → 逐臂点名红 → 还原复绿；D0–E6 用 51 臂组，R1–R3 用尺子）**：

| 臂 | 注入 | 读数 | 判定 |
| --- | --- | --- | --- |
| D0 | 只改 `tests.rs` 注释 | 51 / 0 两跑皆绿 | **成立（无害对照）**：红不是环境本来就不稳 |
| E1 | 摘 `u2_closures.rs:2051` 的限定路径门 | 无会话臂点名 `table_build_dispatch_flag`，403 臂同红（exit 101） | **成立** |
| E2 | 归属 helper 收成 `Ok(())` | 403 臂逐把点名 **13 把全 `=> None`** | **成立** |
| E3 | 空归属 helper 收成 `Ok(())` | 恰点名 3 格：`neural_empty_owner / table_empty_owner / table_missing_row`（含「他人 / admin」双列读数） | **成立** |
| E4 | 摘 `table_reload_dynamic` 的 admin 门 | 两臂红，无会话臂点名 `table_reload_dynamic => Some(Internal)` | **成立** |
| E5 | 摘单把裸名门（`table_flag_status_build`） | 两臂红，无会话臂恰点名该把 | **成立**：证明逐把点名，不是整臂兜底 |
| E6 | 把门挪到 `pool.get()` 之后 | 无会话臂点名同 handler `=> Some(Internal)`，403 臂绿 | **成立**：离线 ordering 哨兵有效（`Internal` 出现 = 门插在读库之后） |
| R1 | 摘掉限定调用点门后跑尺子 | 该行回到 `write_routes_get_writes.tsv` | **成立**：门是真被作证，不是恒列 |
| R2 | 限定调用换成解析不到 / 无拒绝能力的 helper 名 | 尺子不背书该行（回名单），且不作证 | **成立（防恒真）**：作证要求「解析得到 + 链上有拒绝能力」两格都中 |
| R3 | 把缺陷 25 的修法退回旧前置闸（要求本跳自己有拒绝能力） | 判据见证喊红，尺子 EXIT≠0 | **成立**：新 witness 不是摆设，退步即红 |
| — | 臂后磁盘 | 每臂 `hash_ok=true`，收尾只剩本批 4 个在 scope 文件；R1–R3 本轮在**已提交树**重跑一次，三臂仍全成立 | 残留：无 |

- **§五 下一批入口（按现读逐 crate 点名，共 62 条 = K1 5 / K2 11 / K3 46）**：
  `attendance_assemble_control` 9、`auth` 6、`cms_assemble_control` 5、`message_assemble_communicate` 5、
  `processplatform_assemble_designer` 5、`ai_assemble_control` 4、`jpush_assemble_control` 4、`meeting_assemble_control` 4、
  `file_assemble_control` 3、`hotpic_assemble_control` 3、`calendar_assemble_control` 2、
  `organization_assemble_authentication` 2、`organization_assemble_control` 2、`program_center` 2、`query_assemble_surface` 2、
  `general_assemble_control` 1、`message` 1、`mind_assemble_control` 1、`program_init` 1（明细 `write_routes_get_writes.tsv`）。
  集中度最高的三族仍是 attendance（签到/请假 touch 族）、auth（OAuth 回调 GET 写，属零验签族）、
  cms 与 pp designer（动词别名 GET 写档）。
- **新增待修（本批反向见证量出来的真洞，未修）**：`attachment_u2b_html_to_pdf` / `attachment_u2b_html_to_image`
  （POST，`surface/src/lib.rs:26178` 起，体内 `workHtml` → 渲染 → `general_file_store`
  INSERT 进 `x_general_assemble_general_file`，**全程无归属/无 admin 门**）⇒ 任何已认证用户可批量落库任意文件，
  既是存储面投毒也是无界 CPU（渲染）。与同族的 `attachment_u2b_preview_pdf` / `_preview_image_page` 只差一步：
  后者经 `read_attachment_blob` → `u2_gate_att_or_business_error` 有门，前者自己不收门。下一批连门带臂一起做。
- **新增待拍（本批记账，未修）**：
  1. designer 这 14 把取**归属档**是否与 o2server 权威档位一致，尚未逐把核（批 19 已把 surface 的档位拍成归属，
     但 designer 侧的 java 原义未查证）；`table_reload_dynamic` 是全批唯一的 admin 例外，理由是**无键全表写**。
  2. 尺子的 `super::` 限定符仍不解析（`route_handler_unresolved_refs.tsv` 里 process_designer / process_express /
     process_surface 共 13 行点名）。本批的修法只保证「`crate::` / 模块路径」这一族，`super::` 的**自然样本量未普查**，
     不知道还有多少行被这一欠账读成无门禁。
  3. 空归属（`creator` 与 `creator_person` 皆空）行的放行口径按 admin 收，属**数据缺陷不洗成权限放行**的临时口径，
     与批 17/18 的 admin 档是否合并成一把 helper 待拍。
- **沿用的待拍格**（读数按本批终态更新）：`row_to_json` JSONB/时间盲口仍是最高优先；program_center **2 把**（captcha / code）
  与 `creator='system'` 冒名 3 处（`lib.rs:1691 / 1844 / 8722`）、第三方回调**零验签**族、A1 真匿名写 9 条属限流族、
  24 条死档 needs 独立守卫、只读挂写现读 **764**、门禁入口两族并存、`shared/src/crud.rs` CrudSpec 无归属、
  correlation 读侧 IDOR（含 `neural_list_calculate_with_work`）、F5 / F7 / F8 / F10、N4-02 / N4-03、
  `GATE_RE` 里 `require_actor` 无独立见证的冗余词、路径参数不消费族（全仓普查未做）、
  program_center schema drift 42703 族与 `assert_eq!(status, 500)` 断言族 272 处、`query_service_processing` 5 条 POST 写面、
  ppsvc R7 49 + 宏绑不上段 16 把、批 19 三条待拍（list/count 路径段语义、handover cancel/process 全同、`draft/{id}/start` 必然 500）。
- **提交落点**：本批两条轴合成一笔 `29277fe46`——代码轴（`crates/query_assemble_designer/src/lib.rs` 13 把门（12 归属 + 1 admin）
  + `src/u2_closures.rs` 那把走限定路径的归属门，合计 14 把、`src/tests.rs` `b20_*` 两臂与 seed/unseed）+ 尺子轴
  （`scripts/audit_write_gating.mjs` 的 `callHelperBody`、缺陷 24/25 两处判据与正反向见证）+ 7 张刷新产物。
  与本文档（§三.20 / 本节）分两笔，本节即第二笔并回填该 hash。第二笔提交前后各在还原树跑一次尺子
  （T6 / T7 均 EXIT=0，15/15 产物 md5 与提交态全等），门禁 51 / 0 / 0，真库臂连跑两遍读数一致、
  收口后按 `id LIKE 'b20-%'` 逐表数过 0 行残留。共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。

### 第二十一批收口后的刷新（§三.21 / §四 第二十一批读数 / §五 名单）

- **两列读数（批 20 收口 → 本批收口）**。本批**尺子一字未动**，所以没有批 20 那种第三列：

| 尺子格 | 批 20 收口 | 本批收口 | 归因 |
| --- | ---: | ---: | --- |
| GET 写面无门禁 | 62 | **53** | 代码轴 −9，全额归本批 9 把门；跨批逐行对账：恰 9 行出列、**0 新增**、9 行全属 attendance |
| ├ K1 匿名可达无门禁 | 5 | 5 | 本批不动这族（限流族单列待拍） |
| ├ K2 读会话无门禁 | 11 | 11 | 不动 |
| └ K3 已认证无会话无门禁 | 46 | **37** | −9 与本批 9 把门逐把对得上（`attendance_assemble_control` 出列后该 crate 现 **0 行**） |
| R5 已认证 + 体内门禁 | 866 | **872** | +6：**同一批 handler 的 POST 别名**跟着体内门一起进门档 |
| R6 已认证 + 读会话无门禁 | 146 | 146 | 不动 |
| R7 已认证 + 无会话无门禁 | 1052 | **1046** | −6 与 R5 那 +6 同源，**改桶不是消失** |
| R1 / R2 / R4 / R8 / R9 | 145 / 3 / 14 / 1 / 10 | 145 / 3 / 14 / 1 / 10 | 五格全不动 |
| R4 含写 SQL | 5 | 5 | 不动 |
| R6 含写 SQL | 125 | 125 | 不动 |
| R7 含写 SQL | 316 | **310** | −6 全部是上一格改桶的镜像 |
| **真落笔（R4+R6+R7 含写）** | **446** | **440** | −6 = 6 条 POST 别名同步进门（**同一动作在 GET 表与 R 表上的两次记账，不与 GET −9 相加**） |
| 只读挂写方法 | 764 | 764 | 不动 |
| 间接门禁作证 | 263 | **263** | **本批 +0**（与批 20 的 +14 形状相反）：`require_admin_opt` / `require_owner` 命中 `GATE_RE` 的前缀词表，9 把门按**体内直证**进 R5，不经 helper 转调那层，故 `write_routes_indirect_gated.tsv` 一行不加 |
| 间接写 SQL | 215 | 215 | 不动 |
| 判据见证缺口 | 0 | 0 | 三次 EXIT=0（T2 注入前基线、T2/T3 改写说明文前后） |

- **两条判据分开记账（本批只有一条轴）**：
  1. **代码轴**：GET 62→**53** 是本批修的 9 条真漏洞；R 轴 −6 / 真落笔 446→**440** 是同一批 handler
     双方法注册的**镜像读数**，形状与批 19（GET −16 / R −4）、批 20（GET −14 / R −1）同族，是第四种读法：
     **GET −9 与 R −6 不可相加**，相加会把 3 把 GET-only 的门（钉钉、企微、考勤组刷新——它们没有 POST 别名）重算一遍。
  2. **尺子轴：本批零改动**。缺陷 26 只抓到、只取证、只记账，修复留下一批；本批唯一碰尺子侧的动作是
     改写 lib.rs 里两处说明文（去掉字面门名），**改写前后读数与产物逐字节全等**（T2 = T3，15/15 md5 相同）
     ⇒ 证明这 53 行的读数里**没有任何一格是靠注释撑住的**。
- **还原树证明**：注入的三把尺子臂（W1R2 / W1R3 与 D0 之外的所有臂）跑完都会重写产物，故收口前在还原树重跑
  `scripts/audit_write_gating.mjs`：EXIT=0，`git status --porcelain -- docs/audits/artifacts` 为**空**，
  15/15 张产物与提交态逐字节全等，GET 名单 53 行、真落笔 440、只读挂写 764、间接门禁 263 / 间接写 215、缺口 0。
- **门禁**：`cargo test -p attendance_assemble_control` **43 passed / 0 failed / 15 ignored**（基线 41，本批 +2 臂），
  doc-test 0 条。`--ignored` 族**整族**跑：12 passed / 3 failed，三条失败逐条点名且**全部先于本批存在**
  （本批未碰这三条代码路径，孤立重跑同样红）：
  1. `rule/list` 断言 500、现返 200；
  2. `rule/{id}/toggle` 同上——两格都是既有「契约臂钉 500 而环境已能真跑」漂移，属 §五 沿用名单里 `assert_eq!(status, 500)` 断言族 272 处；
  3. `import_checkin_rows` 臂红在 `Forbidden`：服务端 `normalize_key` 把 userId `trim().to_lowercase()` 之后，
     与**原值** `session.person_unique` 直接比 ⇒ 大小写不一致时**本人导入自己被判 403**（真缺陷，见「新增待修」5）。
  另如实记一条**未成功**的收尾动作：`cargo fmt -p attendance_assemble_control` 跑不了
  （`error: 'cargo-fmt.exe' is not installed for the toolchain 'stable-x86_64-pc-windows-gnu'`），
  本批代码是手工按 100 列排版，未经 rustfmt 认证。
- **真库臂的证伪**：`b21_get_writes_gate_lands` 连跑两遍读数一致、收口后按 `id LIKE 'b21-%'` + 两个 person 造数 +
  同步记录 `start_date` 逐表数过 **0 行残留**；它「真执行而不是静默 skip」由 W4 / W5 两臂跑红来作证——
  若它跳过，注入后不可能分别点名出 `Some(Forbidden)` 与 `None` 两格。
- **八臂注入取证（基线绿 → 逐臂点名红 → 还原复绿；D0–W5 用 cargo 单臂，W1R2 / W1R3 用尺子）**：

| 臂 | 注入 | 读数 | 判定 |
| --- | --- | --- | --- |
| D0 | 只改 `security_gate_tests.rs` 注释 | 离线臂 `1 passed / 0 failed` | **成立（无害对照）**：红不是环境本来就不稳 |
| W1 | 摘 `dingding_sync_start` 的 admin 门（体内摘，注释仍含门名） | 离线臂点名 `["dingding_sync_start => Some(Internal)"]`（exit 101） | **成立**：无会话臂逐把有效 |
| W1R2 | 摘 `attendancesetting_enable_type` 的门（该把注释**不含**门名）后跑尺子 | attendance GET 行数 0→**1**，该行回到 `write_routes_get_writes.tsv` | **成立**：门是真被作证，不是恒列 |
| W1R3 | 摘 `dingding_sync_start` 的门后跑尺子（现场：注释含门名） | attendance GET 行数仍 **0**，读数与产物一字不变 | **成立（尺子缺陷 26 取证）**：摘门在尺子侧**完全隐形**——与 W1R2 同形状两臂相反，坐实「`gateEvidence` 不剥注释」 |
| W2 | 摘归属档的会话前置（无会话不再 401） | 离线臂点名 `["group_refresh_participate => Some(Internal)"]` | **成立** |
| W3 | 把 `setting_enable_type` 的门挪到 `pool.get()` **之后** | 离线臂点名 `["setting_enable_type => Some(Internal)"]` | **成立**：离线 ordering 哨兵有效（`Internal` 出现 = 门插在读库之后） |
| W4 | 归属档 helper 收成「一律拒」 | 403 臂点名 `["group_refresh_participate => Some(Forbidden)"]`（本人段被误拒） | **成立**：owner **放行**侧有牙 |
| W5 | 归属档 helper 收成 `Ok(())`（放行） | 403 臂点名 `["group_refresh_participate => None"]`（他人段放行） | **成立**：**拒**侧有牙，不是恒 403 |
| — | 臂后磁盘 | 每臂 `hash_ok=true`；仓库根 `git status --porcelain` 收尾**为空**；harness 内那路 `git status` 本轮因 cwd 不在仓库根而打出 fatal、**实际失效**；首轮 W1 判定路径 `exit(4)` 跳过 `finally` 造成一次残留（见 §三.21），已重插门并整套重跑 | 残留：无（复验来源如实降级为「字节哈希 + 仓库级 status」两路） |

- **§五 下一批入口（按现读逐 crate 点名，共 53 条 = K1 5 / K2 11 / K3 37）**：
  `auth` 6、`cms_assemble_control` 5、`message_assemble_communicate` 5、`processplatform_assemble_designer` 5、
  `ai_assemble_control` 4、`jpush_assemble_control` 4、`meeting_assemble_control` 4、`file_assemble_control` 3、
  `hotpic_assemble_control` 3、`calendar_assemble_control` 2、`organization_assemble_authentication` 2、
  `organization_assemble_control` 2、`program_center` 2、`query_assemble_surface` 2、`general_assemble_control` 1、
  `message` 1、`mind_assemble_control` 1、`program_init` 1；`attendance_assemble_control` **0**（本批清空，明细同表）。
  下一格首选仍是 **auth 那 6 条**：OAuth 回调 GET 写，与「第三方回调零验签」族正面重叠——
  加门只是将写面收口，验签缺失仍在，两件事要分开记。其次是 cms / pp designer 的动词别名 GET 写档
  （批 14 / 批 20 已各收过一族，余量口径可复用）。
- **新增待修（本批点名，未修）**：
  1. **尺子缺陷 26 修复轴**：`gateEvidence` 判 `GATE_RE` 前须剥 `//` 行注释与块注释；修后要重量暴露面
     （已知约 120 行注释提到门名、缩进态约 40 行，**未普查**），并顺带收批 20 待拍 2 的 `super::` 解析欠账。
  2. **注入 harness 的 git 复验**：把 `git status --porcelain` 的 cwd 固定到仓库根并断言 exit=0，
     否则 RESTORE 行只会把 fatal 文本打进日志（`finally` 无条件复原那一半本批已改）。
  3. **三张考勤表无主键**（`migrations/032_create_assemble_control_tables.sql`：`x_attendance_appeal_info` /
     `x_attendance_detail` / `x_attendance_setting` 均 `"id" TEXT` 无 PK）⇒ 按 id `UPDATE` 无法保证单行，
     重复 id 时「归档某一条」写成「归档若干条」；测试侧已绕（42P10），产品侧要补 PK 迁移或改按唯一键写。
  4. **42883 那把**（`attendancedetail_analyse` 族的 `cycle_year` / `cycle_month` integer ↔ handler 绑 `String` 路径参数）：
     修它时必须同步撤销测试里对该把的**明列豁免**，否则类型修好、臂照样绿，等于把修复吞进豁免里。
  5. **`import_checkin_rows` 的归一化 ↔ 权限比较大小写不一致**（`lib.rs:8556-8588` 附近：`normalize_key` =
     `trim().to_lowercase()`，判权限时与原值 `person_unique` 直接比）⇒ 本人被判 403；属可用性 + 权限口径双缺陷。
  6. R 侧同族下一格：`attendancedetail_analyse`（POST，body `personId`，仍未收门）与
     `attendanceappealInfo_appeal_id` / `workflow_appeal_id` 两把申诉写档。
- **新增待拍（本批记账，未修）**：
  1. **`x_attendance_detail` 一表两档**：既有 `delete` 走**归属**档，本批 `archive` / `analyse` 4 把走 **admin** 档。
     同表同族动作两档是真分歧，不是巧合——要么拍成「一张表一把 helper、按动作分档」的显式约定，
     要么逐把回 o2server 原义对齐（与批 20 待拍 1 同族，合并处理更省）。
  2. 本批 8 把 admin 档**逐把未与 o2server 权威档位核对**（批 17/18 收 admin 有 java 侧依据，本批只有当场理由）。
  3. 钉钉 / 企微 `sync/start` 拉起的是**外部副作用**（第三方 API + 批量任务），除档位之外是否要加防抖 / 配额待拍。
  4. **归属列口径两族并存**：本批 `v2_group_refresh_participate` 只读 `creator_person`，批 20 designer 用
     `coalesce(nullif(trim(creator_person),''), creator, '')`。两者对「空归属行」的效果都是 admin-only，
     但一个是列缺失的**副产品**、一个是显式收口。要么全仓定一把 helper（含 trim / 回退 / 空值三档语义），
     要么逐 crate 记下各自口径——现在的状态是「同一条禁令，两把尺」，与批 20 待拍 3 的合并问题是一对。
- **沿用的待拍格**（读数按本批终态更新）：~~`row_to_json` JSONB/时间盲口~~ **已于批 35 收口**（JSONB / JSON / DATE / TIMESTAMPTZ / TIME / UUID 三族补齐，`shared` 152/0 真库臂取证；**NUMERIC 残留**——`postgres-types` 0.2.14 无 decimal 特性，须调用面 `::float8`/`::text` 转写）；只读挂写现读 **764**；
  program_center **2 把**（captcha / code）与 `creator='system'` 冒名 3 处（`lib.rs:1691 / 1844 / 8722`）、
  第三方回调**零验签**族（正压在下一批入口 auth 那 6 条头上）、A1 真匿名写 9 条属限流族
  （surface 的 `html_to_pdf` / `html_to_image` 两把经本批定性订正后**并入这一族**，不再是「无界 CPU」）、
  24 条死档 needs 独立守卫、门禁入口两族并存、~~`shared/src/crud.rs` CrudSpec 无归属~~ **已于批 34+36+37 收口**
  （归属版 `crud_save_owned/crud_delete_owned` + 全仓 crud 调用面逐 crate 加门，A3-N9 结）、
  correlation 读侧 IDOR（含 `neural_list_calculate_with_work`）、F5 / F7 / F8 / F10、N4-02 / N4-03、
  `GATE_RE` 里 `require_actor` 无独立见证的冗余词、路径参数不消费族（全仓普查未做）、
  program_center schema drift 42703 族与 `assert_eq!(status, 500)` 断言族 272 处（本批 ignored 族那两格红正属此族）、
  `query_service_processing` 5 条 POST 写面、ppsvc R7 49 + 宏绑不上段 16 把、
  批 19 三条待拍（list/count 路径段语义、handover cancel/process 全同、`draft/{id}/start` 必然 500）、
  批 20 三条待拍（designer 归属档权威、`super::` 欠账、空归属放行口径合并）。
- **提交落点**：本批**只有一条轴**，代码一笔 `49fb86cc3`——`crates/attendance_assemble_control/src/lib.rs`
  （8 把 admin 门 + 1 把归属门 + `require_admin_opt` helper + 两处说明文改写掉字面门名，共 65 行插入 / 0 删除）
  + 新增 `src/security_gate_tests.rs`（550 行，`b21_get_writes_reject_without_session` 与 `b21_get_writes_gate_lands`
  两臂、seed/unseed/untouched/residue）+ `src/tests_u2.rs` / `src/tests_u2_legacy.rs` 四支契约臂改逐条点名 expect
  + 3 张刷新产物（`write_routes_get_writes.tsv` −9、`write_routes_matrix.tsv` 6 进 6 出、`write_routes_ungated.tsv` −6）。
  与本文档（§三.21 / 本节）分两笔，本节即第二笔并回填该 hash。第二笔提交前后各在还原树跑一次尺子
  （均 EXIT=0，15/15 产物 md5 与提交态全等），门禁 43 / 0 / 15，真库臂连跑两遍读数一致、
  收口后按 `id LIKE 'b21-%'` 逐表数过 0 行残留。共享工作树上并行会话的在制品未被本批收走。**本轮未 push**。

