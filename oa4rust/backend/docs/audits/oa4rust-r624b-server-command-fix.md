# R624b ServerApp 命令执行错端点真修复审计

## 元信息
- 任务: t264 (implementation)；轮次: R624b
- 判级: P2（功能可见错误，非安全漏洞）
- 真父 HEAD: d5b4629a1e614ca04bd0824768b58355c2c83d0c
- 写域: ServerApp.vue（仅 executeCommand）+ 新契约 + 本文档；其余零改
- runnerSha: 2abb9f9113b445b840bbab9b6b7095f0dcc48a2d1b72c8796f50b1b5a1ad2b57
- 文档文件 sha256 见文末；与 runnerSha 分列，两者均不含自身 sha

## 背景
甄别轮 R623c（t259）已判 ServerApp 命令面板调用错端点。真父中 executeCommand 把用户输入命令拼进 GET query 打 /api/server/execute?command=...；后端 server_execute 不读该参数，恒返回 value:true。命令输入实际无效。

## 源码复核（零改，sha256 采集）
- ServerApp.vue 真父 sha dd6baae5f94349cf6aa74cf07667b37bc0f6a77ec00d2bdeadf94862fa7465ca（417 行）
  - :98 command = ref('')；:90-96 consoleWriteSucceeded 失败信封校验（type=error 或 success=false 判失败）
  - :212-226 executeCommand：:219 api.get 错端点、:220 JSON.stringify(r.data)
  - :259-263 serverConsoleOps(cmdExecute) 分支：api.post('/api/console/command/execute', { command: c }) + 失败信封校验（正确范例）
- program_init/src/lib.rs sha 3b3acb4f01c33840e0eaeb7235c4b72ca3b7497fc530d8c0c3aa761fc353f241
  - :644 server_execute(pool, session) 无 Query 无 command；:648 require_admin；:679 INSERT command='execute' 硬编码；:689 固定返回 value:true
- console/src/lib.rs sha 07c303cba9c5e2f629629cde7b819629884c6781ceb814eb5346d3d9255b85ef
  - :275 execute_command(pool, session, Json(payload))；:280-283 command 必填校验；:287 is_admin 门；:292 validate_command_and_args；:296 std::process::Command::new(&program).args().output() 真执行；:309-323 返回 command/args/output/stderr/exitCode
- 结论：真端点是 console/command/execute（POST body.command）；用户输入此前从未被读取执行。

## 缺陷与影响
- 命令面板（不受 devTools 门控）输入命令后输出恒为 value:true，无任何真实执行。
- 需会话且 admin 才能到达真端点；非 admin 调用真端点返回 403。
- 用户输入不生效，不构成越权/写库/崩溃，故非 P0/P1/L1/0。

## 修复（已落盘）
- executeCommand 改为 api.post('/api/console/command/execute', { command: command.value })，复用 consoleWriteSucceeded(r) 校验失败信封；失败写 execError 并 return；成功 execOutput = JSON.stringify(r.data, null, 2)。
- 与同文件 serverConsoleOps(cmdExecute) 分支对齐；成功返回结构零改，其它代码零改。
- 修复后 ServerApp.vue sha 54713174428b5940871d112766e8509b0caeb662d97bdf3350e611f8cf73f290（changed=1）。

## RED → GREEN
- 契约 ServerApp.command.test.ts sha 0015ad9e7b1aeb643722438c8e5ebda55f498d9252dd2a6695ac0bfd5437bc05（node:fs 读源码断言真端点 POST/body.command/失败信封校验/输出，且不含错端点）。
- 真父覆盖 → RED EXIT=1（1 failed；expected body to contain api.post('/api/console/command/execute' 收到 api.get），日志 sha 881f4d3a17d6d5bb3e7c20311f6d612ed231a8027c906c5f2b841c6f13ae2dec。
- 修复版 → GREEN EXIT=0（1 passed），日志 sha a7e8072c3f9e7ecce06c0a44d13e1bc3a07314dd0d935cbc524284eb28748fb0。

## 验证（三条，全 EXIT0）
- V1 定点 vitest：EXIT0，1 passed。
- V2 pnpm typecheck（tsc --noEmit）：EXIT0；据实披露 tsc 不解析 .vue，对模板/脚本内字符串属盲区，由 V1 契约兜底。
- V3 biome check 新契约：EXIT0（Checked 1 file）。

## 判级理由（P2）
- 功能可见错误：命令输入无效，需主动使用命令面板才触发。
- 非安全漏洞：用户输入不生效反而更安全；仅 admin 可真执行；不越权、不写库、不崩溃。
- 排除 P0/P1/L1/0。

## 据实披露
- 未跑浏览器 GUI 端到端验证（环境限制）。
- typecheck 不解析 .vue（见 V2）。
- 真端点仅 admin 可真执行，非 admin 403；修复后用户输入才生效，属预期行为变化。
- 临时日志落 <PROBE_LOG_PATH>（gitignored），不入仓；后端 program_init/console、路由、Cargo、migrations 及其它文件零改。

## 结论
错端点已修复，RED→GREEN 坐实，三 verify 全绿，判 P2 成立。

文件 sha256 由 sha256sum 独立采集，本文档不含自身 sha。
