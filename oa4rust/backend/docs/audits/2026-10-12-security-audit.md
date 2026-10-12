# 2026-10-12 全量安全审计（依赖 + 代码级）

审计范围：oa4rust 全栈（Rust 后端 100+ crate、Vue 前端 desktop/mobile、MCP 服务）。
方法：`cargo audit`（固定库 b50980a + 最新 main 双跑）、`pnpm audit` + 逐条可达性核实、
代码级模式审计（注入/穿越/越权/会话/密码学/出站）。所有修复附回归测试，
`cargo test --workspace --lib` 与前端 `pnpm test`/构建验证。

## 一、依赖漏洞

### Rust（cargo audit 0.22.2，双库对跑一致）

| 项 | 状态 |
| --- | --- |
| `lopdf 0.26.0` RUSTSEC-2026-0187（栈溢出，high） | 已有例外（`security/rustsec-exceptions.toml`，2026-10-28 到期）：genpdf 无补丁链，lopdf 载入路径不可达，PDF 解析/签名走 crates/signature 的 lopdf 0.42 |
| `rsa 0.9.10` RUSTSEC-2023-0071（Marvin，medium） | 已有例外（2026-11-08 到期）：仅 dev-dependency MySQL 测试驱动可达，生产 OIDC/PDF 签名用 ring |
| ttf-parser / adler / lzw / rusttype 等 unmaintained | 信息级（`audit.toml` informational_warnings），随既有依赖链，记录在案 |

### 前端（pnpm audit）

修复（`pnpm-workspace.yaml` overrides，沿既有安全覆盖约定）：
- `source-map-js 1.2.1 → 1.2.2`（event-loop DoS，high）
- `vue/@vue/* 3.4.21 → 3.5.42` 全家族（`@vue/server-renderer` XSS 修复版，high）——
  mobile uni-app 工具链 pin 的 3.4.21 家族整体抬到树中已存在的 3.5.42；
  验证：mobile H5 / mp-weixin 双平台构建 + 121 测试全绿。

保留 3 条（上游修复受阻 / 纯构建工具链，非运行时）：
- `braces <=3.0.3`（无修复版本；chokidar dev 工具，glob 输入非用户可控）
- `file-type 16.5.4`（jimp 硬 pin ^16，uni-mp-weixin 构建期图片探测）
- `postcss-selector-parser 6.1.3`（uni-h5 硬 pin 6.x，7.x 破坏 API；已抬到 6.x 末版 6.1.3）

## 二、代码级发现与修复

### V1（critical）未认证批量删除：appstyle 头像资源 ×7

`/api/program_center/appstyle/image/*` 前缀在 `AUTH_EXEMPT` + rbac `Public`，
其中 7 个 `appstyle_image_*_erase`（DELETE）handler 只取 `pool`，直接
`DELETE FROM x_program_deploy_resource WHERE resource_type='…'`（全类型删除）——
**匿名网络调用即可清空部署资源**。

修复（`crates/program_center/src/lib.rs`）：
- 7 个 erase handler 增加 `session: Option<Extension<Session>>` + 新增
  `require_admin_opt`（无会话 401 / 非 admin 403）。
- `/api/program_center/appstyle` 移出 `AUTH_EXEMPT_PATHS/PREFIXES` + rbac 由
  `Public` 降 `Authenticated`（登录页无未认证消费——已核实 appstyle 仅管理控制台
  调用）；顺带激活 3 个因豁免而 500 死掉的 admin 端点
  （PUT 上传 / GET erase 变体，原必选 `Extension<Session>` 在豁免路径永不注入）。
- 回归：`security_gate_tests`（无会话 401 / 非 admin 403，无 DB 依赖）。

### V2（high）未认证删库：processplatform touch/mergeitem

`/api/processplatform/service/processing/touch/mergeitem` 公开 + 豁免 +
handler 无会话 → 匿名 `DELETE x_attachment`（同名同工作去重删行）。

修复（`crates/processplatform_service_processing/src/u2.rs`）：
- `touch_merge_item` 加 `Option<Session>` + admin 门禁（`require_admin_opt`），
  并移出豁免清单与 rbac Public。
- 同族 7 个维护作业（cleanevent / deletedraft / handoverjob / loglongdetained /
  delay / urge / merge）原为任意登录用户可触发（非公开）——加**仅会话**门禁
  （`require_session_opt`），保持 o2server 对齐不越权升级 admin，并适配既有
  DB 契约测试（`send_with_session` 注入会话）。
- 回归：`security_gate_tests`（8 个 handler 无会话 401、mergeitem 非 admin 403）。

### V3（medium）未认证日志写：jest clear cache ×2

`jest_clear_cache_source` / `u3_jest_clear_cache_source` 无会话 + 公开豁免 →
匿名 `INSERT x_program_sync_log`。修复：Option 会话 + admin 门禁；移出豁免与
Public。适配既有 `tests_u3`（500 → 401 断言）。

### V4（low）未认证控制表写：surface touch ×3

`touch_expire|passexpired|touchdetained`（更新 `pp_c_touch_*` 时间戳，流程中心
「触发」按钮对任意登录用户开放）→ 仅会话门禁（`u2_require_touch_auth`）；
移出豁免与 Public，未认证 401、登录用户功能不变。

### V5（激活既有门禁）4 个「豁免 + 必选会话」死端点

`batch/delete/manage`、`adminlogin`、`cachedispatch`、`ai/chat/delete`：
handler 必选 `Extension<Session>` + require_admin/require_owner，但挂在公开豁免
前缀下 → 会话永不注入 → 所有人 500（功能死、语义错位）。从
`AUTH_EXEMPT_PATHS/PREFIXES` 与 rbac Public 移除后：未认证 401、admin/owner
可正常工作。

### V6（medium）CSP `connect-src` 通配外泄通道

`security_headers_middleware` 的 CSP 原含 `wss://* ws:*`——任何脚本可对任意
wss 端点发起 WebSocket（XSS 时的数据外泄通道）。前端 realtime 仅连同域
（`sdk/websocket.ts` 用 `window.location.host`），收紧为
`'self' ws://localhost:* ws://127.0.0.1:*`。

### V7（low-medium）FORCE_HTTPS 开放重定向

`https_redirect` 直接拼请求 `Host` 头构造 307 Location：攻击者 `Host: evil.com`
+ `X-Forwarded-Proto: http` 可把受害者浏览器重定向到攻击者站点。
修复：生产（配置了 https 的 `APP_PUBLIC_ORIGIN`）固定用配置源 host，
开发环境回退请求 Host。

### V8（high）MCP 独立服务无认证 + 0.0.0.0

`mcp_server` HTTP 模式绑定 `0.0.0.0:3001`，`/mcp` 无任何鉴权，ToolBridge 直通
后端 DB（工具可触发后端操作）。修复（`crates/mcp_server/src/main.rs`）：
- 默认绑定改 `127.0.0.1`；`MCP_BIND_ADDR` 显式改绑非回环且未设 key 时告警。
- 新增 `MCP_API_KEY`：设置后 `/mcp` 强制 `X-API-Key` 匹配（常量时间比较，
  401 拒绝）。主二进制内的 `/mcp`（`--http` 挂载）本就在 auth/authorize
  中间件链内，不受影响。

### V9（low，潜在）preview crate 路径穿越

`LibreOfficePreview`（独立 crate，未挂进主服务）：`temp_dir.join(file_name)`
可被 `../` 逃出写任意位置、`download` 可经 `file://` 读任意文件、
`--convert-to {target}` 拼进输出路径。修复：`safe_base_name`（取末段、拒
`.`/`..`/NUL）、download 限定沙箱内且拒 `ParentDir` 成分、target 限
ASCII 字母数字。附 `security_preview_tests`。

## 三、评估后不改（记录在案）

- **SQL**：动态表名均为 `&'static str` 编译期常量 + `$N` 参数化（FI-002 既有
  方法沉淀）；`format!` SQL 命中点逐一核实无请求输入插值。
- **命令**：仅 console（白名单 + 参数字符校验 + admin）与 preview
  （libreoffice，参数化、无 shell）。
- **SSRF**：ai crate 出站 URL 来自 env（`AI_API_BASE`/`AI_API_KEY`），非请求输入。
- **会话/cookie**：HttpOnly + Secure（生产强制）+ SameSite=Lax + host-only，
  CSRF 精确 Origin 校验，refresh 轮换；生产配置缺失即拒绝启动。
- **重置码**：6 位数字码 + 5 次尝试 + 5 分钟 TTL + 需先经发放端点
  （`/api/reset` 30 次/分/IP 限流）——暴力面 ~0.05%/次，接受现状。
- **`/openapi.json` 公开**：API 面清单信息泄露（低），属文档用途有意为之。
- **express 批量查询匿名读**：o2server 行为对齐（R24/R401J200），枚举面已限流。

## 四、验证

- 新增回归测试：`shared`（豁免清单/rbac 4 项）、`program_center`（3）、
  `processplatform_service_processing`（3）、`preview`（1）全部通过。
- **本次改动的每个 crate 单独跑均全绿**：shared 151、program_center 259、
  processplatform_service_processing 57、processplatform_assemble_surface 602、
  parity 4143、mcp_server 19、preview 5、bbs_assemble_control（修 `$2` 复用于
  两列类型后）全绿；`cargo check --workspace` 通过。
- 前端：root vitest 1024 + mobile 121 + contracts 19 全绿；mobile H5 /
  mp-weixin 双平台构建 + desktop 生产构建通过（vue 3.5.42 override 后）。
- 既有断言适配 2 处：`tests_u3`（jest 500→401）、`tests.rs`（cleanevent 注入会话）。

**审计后补记（同轮收口）**：

- 穷尽扫查补齐：对全部 crate 的注入（format!/拼接 SQL、命令执行）、文件穿越、
  SSRF、弱加密四族 pattern 逐一人工过完（3 个子代理因配额失败后手动完成）——
  **未发现新的真实安全漏洞**。所有 `format!` SQL 位均为静态字面表名/列名、
  参数走 bind；`Command` 仅 shell/console 展示；SSRF 面仅管理员配置的
  AI/MCP URL 与环境变量；遗留 md5/DES 校验路径仅 o2server 兼容 + 登录即升级
  bcrypt；wechat SHA1 为协议要求；session token 为 HMAC-SHA256 签名 UUID。
- 4 个既有失败的精确根因取证（见上节）：qsp 输入契约与 org person_detail
  SQL 拼接为**真实功能缺陷**，已在审计后以独立提交修复；bam 2 个 state
  测试为测试卫生（非幂等 + 并发 delta 干扰），留档未动。
- 补修后复验：`query_service_processing` 23/23、`organization_assemble_express`
  42/42 全绿；`processplatform_assemble_bam` 干净库 + 串行 30/30 全绿（并发
  `--test-threads` 默认下 2 个 state 测试仍会因互相干扰挂，属既有测试卫生，
  非本次回归）。

### 与本次改动无关的既有失败（已根因取证：2 个真实功能缺陷已修，2 个为测试卫生）

初版审计将 `cargo test --workspace --lib`（本地共享 PG）暴露的 4 个失败笼统记为
「种子残留 / schema 漂移」。后续用一次性本地探针（`examples/db_security_probe.rs`，
跑完即删，不进 git）做了**精确根因取证**，结论修正如下——4 个失败**均与本次
安全改动无因果**（全在未触碰 crate，走裸 router、不经 auth/rbac 中间件），但根因
比初版记录更精确，其中 2 个是**真实的功能缺陷**（已在审计后修复提交中修掉）：

1. **`organization_assemble_express::u2_person_detail_flag_resolves_by_id_and_name`
   —— 真实 SQL 字符串 bug（已修）**：`endpoints_person2.rs` 的 `person_detail_flag`
   群组 CTE 中 `… AND m.person_id = $1\` 行尾反斜杠（Rust 字符串换行续接符）吃掉了
   换行与下一行缩进，拼出 `…$1UNION`。任何**存在的** person 走到该 CTE 必触发
   PG `E42601: trailing junk after parameter at or near "$1UNION"` → 端点恒 500。
   探针端点级复现（存在 person → 500，不存在 → 200 早退）+ 临时在 8 处
   `map_err` 打点抓到确切 SQL 错误后定位。修复：`$1` 后补空格。全库扫描
   「行尾 `\` 前为非空格字符」同类隐患仅此 1 处。修复后该 crate 42/42 全绿。
2. **`query_service_processing::u2_batch_process_prefetches_and_preserves_order`
   —— 真实输入契约 bug（已修）**：`QueryRequest` 按 snake_case `query_type`
   反序列化，但同一端点的响应字段与 o2server Java 契约均为 camelCase `queryType`。
   请求体里的 `"queryType"` 全部落空 → batch 每项都返回 `query_type is required`
   → 测试断言 `results[0]["id"]` 得 `Null`。探针端点级复现确认（5 项全落空分支）。
   修复：`#[serde(rename = "queryType")]`（`params`/`options` 单词不受影响；两个
   mock 池 500 断言测试与字段命名无关，不受影响）。修复后该 crate 23/23 全绿。
3. **`processplatform_assemble_bam` 的 `state_summary|state_running_filter_buckets_
   match_seeded_deltas`（2 个）—— 测试卫生问题（未动，留档）**，非种子残留那么简单：
   - **非幂等**：固定种子 id（`r25-*`/`r26-*`）无前置 DELETE，失败路径不清理
     （清理只在成功尾段），前序失败运行留下残留 → 后续 `INSERT` 撞
     `x_work_pkey`/`x_task_pkey` 唯一键。清库后单跑即通过。
   - **并发不互斥**：两测试共享 `x_work`/`x_task` 且都按「before/after 计数差」
     断言；`cargo test` 默认多线程并发跑时互相往对方计数窗口里种行 → 差值断言
     必挂（复现签名 `left: 2, right: 1`）。干净库 + `--test-threads=1` 串行
     30/30 全绿。

处置与边界：#1 #2 是**既有功能缺陷**（可用性/契约层面，非可利用的安全漏洞），
已在审计后以独立提交修复并全绿；#3 属测试卫生（共享库 delta 断言 + 固定种子），
与安全无关，未并入安全提交，留档备查——如需收口可单独立项（种子改随机 id +
前置 DELETE + 两测试加进程内互斥或改串行）。CI 用独立 DB 实例时，#3 在
`--test-threads=1` 或独立库下同样可绿。


