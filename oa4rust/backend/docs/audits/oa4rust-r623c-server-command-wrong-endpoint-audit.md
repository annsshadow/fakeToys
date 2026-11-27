# R623c ServerApp 命令执行面板错端点甄别（t259）

## 结论

- 裁定：P2（功能可见错误 / 命令执行面板输入无效），不改判安全漏洞。
- 主端点 `executeCommand` 打到 `GET /api/server/execute?command=...`，后端 `server_execute` 根本没有 command 参数，恒返回 `{"value": true}`；用户输入命令从未被执行。
- 同一页面 `serverConsoleOps("cmdExecute")` 走真契约 `POST /api/console/command/execute`，后端 `execute_command` 真执行命令——说明正确端点在仓库内已存在，主面板属错接。
- 用户输入完全不生效反而降低注入面，故不构成本次引入的安全漏洞；命令面板生产可达属实（`/app/admin/server`）。

## 证据锚点（sha256）

- `oa4rust/frontend/apps/desktop/src/views/ServerApp.vue`（417 行，sha256 `dd6baae5f94349cf6aa74cf07667b37bc0f6a77ec00d2bdeadf94862fa7465ca`）：:67 输入框 `v-model="command"` + `@keydown.enter="executeCommand"`；:68 按钮绑定；:98 `const command = ref('')`；:212–226 `executeCommand`，:218 注释“后端 server/execute 仅 GET”，:219 `api.get(\`/api/server/execute?command=${encodeURIComponent(command.value)}\`)`，:220 直接 `JSON.stringify(r.data, null, 2)`；:36 `serverConsoleOps('cmdExecute')`，:259–263 分支 `api.post('/api/console/command/execute', { command: c })` + `consoleWriteSucceeded(r)`。
- `oa4rust/backend/crates/program_init/src/lib.rs`（sha256 `3b3acb4f01c33840e0eaeb7235c4b72ca3b7497fc530d8c0c3aa761fc353f241`）：:644 `server_execute(pool: Extension<Pool>, session: Extension<Session>)` 无 `Query`、无 `Json(payload)`、无 command 参数；:648 `require_admin`；:651–676 两表 UPDATE + 硬编码 `INSERT command='execute'`；:689 固定 `ActionResult::success(json!({ "value": true }))`。
- `oa4rust/backend/crates/console/src/lib.rs`（sha256 `07c303cba9c5e2f629629cde7b819629884c6781ceb814eb5346d3d9255b85ef`）：:275 `execute_command(pool, session, Json(payload): Json<ExecuteCommandRequest>)`；:280–283 `.command.as_deref()` + `command is required`；:287 admin 门；:292 `validate_command_and_args`；:296–299 `std::process::Command::new(&program).args().output()` 真执行。
- `oa4rust/frontend/apps/desktop/src/main.ts`（sha256 `e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0`）：:182–185 `path: 'server', name: ServerApp`；:531 `/app/admin/server` 重定向，生产可达。
- 台账：`oa4rust/backend/docs/audits` 对 `executeCommand|server/execute` 零命中；R386 仅将 executeCommand 列为对比组未甄别，R387 只修 serverConsoleOps 五分支，本次为首次甄别。
- P3 候选（不立案）：`oa4rust/frontend/apps/desktop/src/views/QueryManagerDeep.vue:1076` 文案 `viewRows.length` 缺 `.value`（sha256 `9270c8534098fa0283198716ba8da0097ff352a27c2dd8069fe57f91e65ce327`），仅提示文字行数失真，非安全/功能阻断，仅在本审计据实披露。

## 真测试

- 临时 vitest+jsdom 探针 `oa4rust/tests/r623c_probe.test.ts`（4 用例不 ignore）用 `node node_modules/vitest/vitest.mjs run tests/r623c_probe.test.ts --reporter=verbose` 真执行：EXIT 0，4 passed（vitest v5.0.5）。
- 坐实：①主面板错端点（GET query 带命令、无结果分支）；②后端 server_execute 签名无 command + 硬编码 execute + 恒 `value:true`；③兄弟 console/command/execute 真契约（JSON body、admin 门、validate、`Command::new(...).output()`）；④路由可达 + P3 文案存在。
- 日志落盘 `<PROBE_LOG_PATH>`，sha256 `d5259230e5f1b2902111e0832387b92ebb151f855c0d0304e8b76f48a965c514`；探针是临时甄别工具，已按写域约束删除，零源码改。

## 三分判级

- 可达性：命令面板无 devTools 门控（门控在分区导航），但端点需会话 + `require_admin`，故非匿名面；生产路由可达。
- 危害：功能可见错误（用户输入命令无效、输出恒 true），不越权、不写用户可控数据、不崩溃、无敏感信息回显；用户输入不生效反而更安全。
- L1/0 排除：不是空白页、按钮全失效或数据破坏；命令执行入口在兄弟 `serverConsoleOps` 仍可用，主面板功能局部失效。
- P0/P1 排除：无安全边界破坏、无提权、无数据丢失或可用性大面积受损。
- 裁定 P2：局部功能错误，修法为 `executeCommand` 对齐兄弟真契约（或移除误导面板），但本轮仅甄别、零源码改。

## 据实披露

- 未跑浏览器 GUI；证据为源码逐行复核 + vitest 真执行，面板实际渲染行为以真实浏览器为准。
- `tsc --noEmit` 不解析 `.vue`，本缺陷不会被 typecheck 捕获（据实）。
- 审计文档 sha256（磁盘文件值）与 runnerSha（紧凑 JSON meta 的 sha256）分列；runner meta 不含本文件 sha，避免自指。
- runnerSha: `9d131e12ac404ba551ad6c98cec1b281eb63d3701673f3241a5ee883344aef50`

文件 sha256 见交付报告（磁盘文件值，本文件不含自身 sha）。
