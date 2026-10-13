# 2026-10-13 全量安全审计 · 二轮复核（越权/属主 + 注入 DoS + 前端外泄）

目标：全面找出 oa4rust 所有漏洞并修复验证（`/goal`）。本轮回在 `2026-10-12-security-audit.md`
（V1–V9 + 审计后 5 个安全提交 35a35c53a / ef3885cea / 5fe82f1e7 / 0160df064 / b2cf4b728）
之上，对**当前代码树**做全库复扫。方法：4 个只读审计子代理（authz / 注入 / 前端，
各自先读一轮审计避免重报）+ 人工逐格取证 + 全部修复附回归测试。

结论：新发现 7 族真实漏洞（1 匿名写、4 越权/IDOR 族、1 死端点族、1 缓存 DoS）
+ 前端 1 低危 + 2 信息级；全部已修复并验证。另收口 1 处既有编译破损
（control crate `verify_password` 5 参签名回归）与 1 处既有 chrono 弃用警告。

## 一、依赖复扫（无新增漏洞面）

- **Rust**：`cargo audit` 需固定库（本机未装 cargo-audit 二进制）；核对
  `Cargo.lock` 自一轮审计后仅新增 `aes-gcm`（SSO AES-GCM-256 迁移，标准密码学
  crate，无已知 CVE）、bbs 移除 `md5`/`bcrypt`（并入 auth 主链）。两条
  `security/rustsec-exceptions.toml` 例外（lopdf 0.26.0 RUSTSEC-2026-0187、
  rsa 0.9.10 RUSTSEC-2023-0071）均在有效期内且 `scripts/validate_rustsec_exceptions.py`
  校验通过。无新漏洞面。
- **前端**：`pnpm audit --prod` 报 3 条（1 high braces / 2 moderate file-type、
  postcss-selector-parser），**逐条对应**一轮审计「保留 3 条」记录
  （braces 无修复版、file-type 被 jimp 硬 pin、postcss-selector-parser uni 工具链
  硬 pin 6.x）；`pnpm why` 实测版本一致。vue/@vue 家族双端均 3.5.42（含
  @vue/server-renderer XSS 修复版）。无新增 CVE。

## 二、代码级发现与修复（本轮新增，均附回归测试）

### S1（中-高，匿名写）meeting 系统配置匿名 UPDATE

`POST /api/meeting/assemble/control/config/system/config/manage`
（`meeting_assemble_control::config_system_config_manage`）：前缀
`/config/system` 在 AUTH_EXEMPT_PREFIXES + rbac Public（o2server 对齐的未登录读），
但写 handler 无会话/无 admin → 匿名 `UPDATE x_meeting_config`（会议模块全局配置）。
修复：handler 加 `Option<Extension<Session>>` + 新增 `u2_require_admin_opt`
（未认证 401 / 非 admin 403），前缀保留豁免供只读；回归 `security_gate_tests`
（401/403 各一，无 DB 依赖）。

### S2（高，越权族）general_assemble_control 全 crate 写 0 门禁

`general_assemble_control` 此前 52 处写 SQL、0 处门禁（rbac 兜底 Authenticated）：
- **系统配置/参考数据**（维护模式、注册开关、考勤范围、地区、安全等级、全局控制行）
  `update_general_control_status` / `general_control_create|save|delete` /
  `attendscope_*` / `area_*` / `securityclearance_*` → 加 crate-local `require_admin`
  （任一登录用户即可全局启停维护模式 / 改安全等级 = 可用性 + 鉴权面）。
- **属主资源**（invoice / qrcode，有 creator 列）`invoice_update_id` /
  `invoice_delete_id`（物理删）/ `invoice_update_apply_status_id` / `qrcode_delete`
  → 新增 `owner_gate`（命中行按 creator `require_owner`，admin 旁路；行不存在则放行
  保持 not-found 语义）。
- **creator 会话化**（原取请求体，可冒名归属）：`invoice_create`、`qrcode_create`、
  `excel_upload`、`excel_upload_with_url`、`generalfile_create` → `session.person_unique`。
- 回归 `security_gate_tests`（12 个管理写 handler 非 admin 403，无 DB 依赖）。

### S3（高，越权族）organization_assemble_control 组织结构写 0 门禁

`organization_assemble_control` 此前 119 处写 SQL、0 门禁。提权/破坏路径：
- **组成员增删**（`group_flag_add|delete_member` + mockputtopost 变体，4 处）：
  任一登录用户可把**任何人**加进/踢出任意组 → 借群组-角色体系提权。
- **结构软删族**（group / identity / permissionsetting / personattribute /
  personcard(=person) / role / unitattribute / unitduty 的 `mockdeletetoget`，8 处）：
  任意登录用户可软删群组/角色/权限配置/人员。
- **职务排序/删除**（`unitduty_flag_mockputtopost` / `unitduty_flag_mockdeletetoget`）。
- **人员 qr_code / card_code 写**（`personcard_createQR_cardId` /
  `personcard_createCode_cardId`）：对任意人员记录写列。
- **身份排序**（`identity_flag_order_before_followFlag`）。
修复：以上 15 个 handler 统一 `u2_helpers::require_admin`（复用 crate 既有 helper）；
回归 `security_gate_tests`（10 个结构写非 admin 403，无 DB 依赖）。

### S4（中，越权族）meeting_core_entity 会议/会议室物理删改 0 门禁

`meeting_core_entity`（assembly 侧双胞胎已在「优化二轮 11/12」加属主门，core 侧漏掉）：
- **会议**（x_meeting 有 creator）`update_meeting` / `delete_meeting`（物理删）→
  新增 `meeting_require_owner`（查 creator + `require_owner`，行不存在放行保持
  not-found）；`create_meeting` 的 `organizerId` 改取 `session.person_unique`
  （原取请求体，可冒名）。
- **会议室**（x_meeting_room 无属主列）`create_room` / `update_room` / `delete_room`
  → `require_admin`。
- 回归 `security_gate_tests`（会议室 3 写非 admin 403 无 DB 断言；会议属主门
  DB 门控种真实行断言非属主 403 / 属主放行）。

### S5（中-低，越权族）mind / message / hotpic / calendar / jpush / portal / component /
process_designer 属主与 admin 门

- **mind**（mind_base_info / mind_folder_info 有 creator）`update_mind` /
  `delete_mind` / `update_folder` / `delete_folder` → `mind_owner_gate`；
  `create_mind` / `create_folder` / `create_version` 的 creator 会话化；create_version
  额外按所挂 mind 的 creator 校验。
- **message**（x_msg_message 全员可见系统通知）`custom_create` → `require_admin`
  （原任一登录用户可伪造系统消息）。
- **hotpic**（轮播图 = 管理台内容，creator 恒写 "system"）`create_hotpic` /
  `save_hotpic` / `delete_hotpic` / `update_control_config` → `require_admin`，
  create 的 creator 会话化。
- **calendar**（CAL_CALENDAR / CAL_EVENT 有 createor）`calendar_update` /
  `calendar_remove` / `event_update` / `event_remove` → 按 createor `require_owner`；
  `calendar_create` / `event_create` 的 createor 会话化。
- **jpush**（x_jpush_device）`device_create` 的 `userId` 改取 `session.person_unique`
  （原取请求体，替他人注册推送设备 = 推送劫持面）。
- **portal_assemble_surface**（门户内容）`create_surface` / `publish_surface` /
  `surface_publish` / `page_id_delete` / `create_layout` / `save_layout` /
  `delete_layout` / `dict_…_mockputtopost` → `require_admin`（/api/portal 已在
  ADMIN_WRITE_DEPRECATED_PREFIXES，中间件不再兜底 admin）；surface/layout create
  的 creator 会话化。
- **component_assemble_control**（x_component + 控制配置）`update_control_config` /
  `create_component` / `save_component` / `delete_component` / `component_delete_all`
  （全表软删）→ `require_admin`，create 的 creator 会话化。
- **process_designer**（PROCESS_APPLICATION 无属主列 = 系统级流程定义）
  `application_create` / `application_update` / `application_remove` →
  `require_admin`（ownerless 系统数据裁决，见 idor 档案）。
- 各 crate 附 `security_gate_tests`（无 DB fail-closed 403 / DB 门控属主 403）。

### S6（低，DoS）auth 密码校验缓存无上限 + 缓存失败结果

`auth/password.rs` `VERIFY_CACHE`（`HashMap`）无容量、无 TTL、缓存**失败**结果：
匿名 `/api/login` 可用唯一明文洪泛 → 缓慢 OOM；且缓存失败使重复错误密码跳过
bcrypt 慢哈希，放大暴力破解吞吐。修复：仅缓存**成功**结果 + 容量上限
（`VERIFY_CACHE_CAP=4096`，满则整体清空重建）；回归 `verify_cache_bounded_and_success_only`
钉死两条语义。

### S7（低，前端外泄）BBS 帖子图片 URL 跨源跟踪像素 + Referrer 外泄

`frontend/apps/desktop/src/views/BBSForum.vue` `topicPics` 来自帖子正文抽取的
用户可控 URL，`<img :src>` 直渲染 → 攻击者把 `http://attacker/pixel` 写进正文，
受害者打开详情即跨源抓取（跟踪 + Referrer 外泄）。修复：新增 `isSafePicUrl`
白名单（同源相对 / 本机回环 http(s)，拒绝 javascript:/data:/vbscript:/file:/
异源 http(s)），赋值处 `.filter(isSafePicUrl)`，`<img>` 加
`referrerpolicy="no-referrer"`。

### 信息级（记档，已随本轮处置）

- **SW message origin 守卫**（`frontend/apps/desktop/public/sw.js`）：SKIP_WAITING
  监听未校验来源；加 `event.origin !== self.location.origin` 守卫（纵深防御）。
- **sandbox.ts `bootstrapSandboxFrame` 死 API 误导注释**：注释称「called internally
  via srcdoc」但生产走 `iframe.src = sandbox.html`；已更正注释（函数仅测试引用，
  保留导出；sandbox 属性在 srcdoc 下仍生效，无新增风险面）。

## 三、既有破损收口（非本轮安全改动引入，但阻塞 `--lib` 全绿）

- **control crate 编译破损（E0061）**：审计后提交 `35a35c53a` 把
  `verify_password` 从 4 参改为 5 参（加 `legacy_compat`），但
  `crates/control/src/tests.rs` 的 `test_password_hash_and_verify` 仍是 4 参调用 →
  `cargo test --workspace --lib` 整体编译失败。已改为 5 参（bcrypt 路径传 `false`，
  遗留 MD5 路径传 `true` 验证兼容开关、`false` 验证 fail-closed），并新增
  「关闭开关时 MD5 必须拒绝」断言。
- **program_center chrono 弃用警告**：`security_gate_tests::build_session` 用
  弃用的 `NaiveDateTime::from_timestamp`；改走 `shared::testing::test_session`
  （新增的可复用测试会话构造器，内部用 `parse_from_str`）。
- **processplatform_assemble_bam 2 个 state delta 测试卫生收口**（一轮审计「留档
  未动」项）：`state_summary` / `state_running` 两测试对 x_work/x_task 全表计数做
  before/after 差值断言 + 固定种子 id（`r25-*`/`r26-*`）无前置清理——默认并发下
  互相污染计数窗口，失败运行残留撞唯一键。已加：① 进程内互斥锁
  （`STATE_DELTA_LOCK`）串行化两个 delta 测试；② 开跑前置 DELETE 各自种子行（幂等）。
  收口后默认并发 30/30 全绿（此前仅清库 + 串行 30/30）。

## 四、已核对无问题（避免重报 / 避免误改）

- **CSRF / 会话固定 / 角色即时生效**：`csrf_middleware` 覆盖全部写方法
  （POST/PUT/PATCH/DELETE，无 any/all 通配），cookie 认证写请求强制 Origin 精确
  等于 `APP_PUBLIC_ORIGIN` 或回环镜像；所有登录路径 `create_session` 发全新
  UUID token（无会话固定面），`refresh` 轮换；`is_admin` / `person_has_role` /
  `person_has_group` 每请求实时查库，角色变更下一请求即时生效。
- **SQL / 命令 / 路径穿越 / SSRF / 弱加密**（注入族复扫）：动态表名均
  `&'static str` 编译期常量 + `$N` 参数化；`Command` 仅 console（白名单 + admin）
  与 preview（libreoffice，参数化）；netguard 覆盖 2 处用户可控 URL 出站；
  无密码/token/secret 进 JSON 响应或 tracing 日志。均与一轮结论一致，无新项。
- **V5 死端点族漏网 8 条**（attendance ×4、document/batch、file referencetype、
  meeting config、script/list/manager）：`exempt + 必选 Extension<Session>` 组合 →
  会话永不注入 → 全员 500（功能死、语义错位）。已按一轮 V5 先例从
  `AUTH_EXEMPT_PATHS` 移除（含 R401J200 只读块里误放的 4 条 attendance 写路径）
  并在 `security.rs` `destructive_write_endpoints_require_session` 钉死「不得匿名可达」。

## 五、验证

- **各修复 crate 单独 `cargo test -p <crate> --lib` 全绿**：shared、
  meeting_assemble_control、general_assemble_control、organization_assemble_control、
  meeting_core_entity、mind、message、hotpic_assemble_control、calendar、jpush、
  portal_assemble_surface、component_assemble_control、process_designer、control、auth、
  program_center。新增回归 `security_gate_tests` 共 15+ 组。
- **`cargo test --workspace --lib`（本地共享 PG 就绪时）全绿**；`parity` crate
  4143/4143（PG 就绪时）。⚠ 注：共享 PG（laiyipao-pg 容器）停摆期间 41 个
  DB 依赖的 parity behavior 测试会 500 失败，PG 恢复即绿——属环境而非代码回归。
- **前端**：root vitest 1028/1028、mobile 121/121、desktop 生产构建通过
  （含本轮 BBS/sw.js/sandbox.ts 改动）。
- **端到端佐证**：`program_center_app_crud_lifecycle`（真实 `create_app` + 会话 +
  `Extension<Pool>` 的 200 往返）通过，钉死了「handler 内 `Extension<Pool>`/
  `Extension<Session>` 门禁在生产路由上真实生效」（非仅裸 router 测试）。

### 既有失败根因取证（非本轮安全改动因果，留档）

- **`integration_runner -- --ignored` 的 `integration_scenarios` /
  `data_integrity_scenarios` 不稳定（seed 碰撞 + 共享 OnceLock 库 + 并发）**：
  `seed_test_data` 用固定 `person-it-admin` 种 admin；`org_person_meeting` 走
  `TEST_DB`（共享 OnceLock `oa4rust_test_<pid>` 库）而其它场景走
  `init_isolated`（独立库）；并发跑时固定 id 撞 `auth_person_pkey` 唯一键、
  共享库连接被并行测试 drop → “insert admin person failed (duplicate key)” /
  “connection closed”。种 admin 不完整时，`u2_room_create` 的 admin 门禁对会话
  返回 403（即前序观测到的 org_person_meeting 403 的真正来源）。**根因是集成
  测试基础设施卫生（固定种子 id + 共享库 + 默认并发），非安全漏洞，且非本轮
  改动因果**（本轮未触碰 `u2_room_create` / `seed_test_data` / `is_admin` /
  场景文件）。收口建议（未并入安全提交）：种子 id 随机化 + 每场景独立库 +
  `--test-threads=1` 串行，或改 `ON CONFLICT` 幂等 + 串行。

## 六、不改（记录在案）

- **express 批量查询匿名读**（o2server 对齐 R401J200，枚举面已限流）——维持。
- **`/openapi.json` 公开**（API 面清单，文档用途有意为之）——维持。
- **重置码暴力面**（6 位 + 5 次 + 5min TTL + 30/min/IP 限流，~0.05%/次）——接受。
- **前端保留 3 条依赖项**（braces / file-type / postcss-selector-parser）——构建
  期工具链，上游修复受阻或 API 不兼容，记录在案。
- **org `x_org_export` 导出作业 INSERT**（用户自触发自己的导出，非越权面）——维持。
- **general `area_create` 的 creator 仍取请求体**：该族已 admin 门控，admin 代建
  时 creator 取值有业务意义，仅加 admin 门禁不改归属语义（外科最小化）。
