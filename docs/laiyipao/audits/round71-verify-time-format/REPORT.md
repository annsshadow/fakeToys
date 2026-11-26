# R71 验真页记录时间本地化 — 真浏览器 QA 报告

- 任务：t147 / attempt 08a60526-3681-4458-8bf4-ba8f65d3685b
- 工作树：D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248，HEAD=a0e1ead08
- 执行日期：2026-11-27（Asia/Shanghai，UTC+8）
- 结论：**PASS**。R71 修复（verify.vue 新增 fmtVerifyTime，模板 :101 由裸 UTC RFC3339 改为本地 y-mm-dd hh:mi）在真 Chromium 三视口生效，非法 ISO 回退原串，不变量与回归无回退，门禁全绿。

## 0 方法与诚实边界
- 真浏览器：agent-browser 0.41.1（Chromium），XHR patch（data/r71-init.js）mock GET /api/v1/battle/424242/replay，返回合成 created_at（**非真值**，仅用于语义验证）；经 gitignored data/auth-init.js 注入游客态凭据（不回显、不入库）。
- 验真链路为**真点击**：open 空页带 init-script → navigate 不带 battle_id 的 verify 页（避免 onLoad 自动取数）→ 用 r71-click.js 填 number input=424242 并点「取复现信息」按钮。探针 r71-probe.js 读 DOM .kv/.hash/.bs-item/.r5-note-text/[role=alert] 与 __qaHits。
- 样本仅覆盖候选页（verify 页）本身；后端 4 处 createdAt.UTC().Format(time.RFC3339) 未改，属 R71 范围外。
- 截图 sha 相同者系记录时间行在 390/768 视口首屏以下（截图仅截可视区）；DOM 探针才是时间行差异证据，已逐组落盘。

## 1 三视口 × 状态实测（12 组，data/{normal,bad,double,fail}-{V}-probe.txt）
### 1.1 合法 ISO（mock created_at=2026-11-27T01:23:45Z）
- 390 / 768 / 1280 三组均：recordTime="记录时间2026-11-27 09:23"、recordRaw="2026-11-27 09:23"（UTC+8 本地化）、hasT=false、hasZ=false、errText=null、hOverflow=0（innerW=docW=clientW）、hitCount=1、snapshotCount=5、hashLine="服务端记录的哈希ABCD EF01 2345 6789"（pretty 分组）、r5Conclusion 为齐备提示。
- 对照 R70-A 改前：DOM 曾为「记录时间2026-11-27T01:23:45Z」裸 ISO；本次已不含 T/Z。
### 1.2 非法 ISO 回退（mock created_at=not-a-date）
- 390 / 768 / 1280 三组均：recordTime="记录时间not-a-date"、recordRaw="not-a-date"（原串诚实回退，无崩溃）、errText=null、hOverflow=0、snapshotCount=5。bad-1280 截图肉眼可见「记录时间 not-a-date」。
### 1.3 连点守卫（normal 会话内连点两次 r71-click.js）
- 390 / 768 / 1280 三组 hitCount=1（第二击被 fetching 守卫拦截，仅一次 getReplay），时间与哈希渲染正常。
### 1.4 失败态清结论（mock status=500 + 错误 JSON）
- 390 / 768 / 1280 三组均：errText="请求失败 HTTP 500"、kvList=[]（复现信息清空）、hashLine=null、snapshotCount=0、r5Conclusion="复现信息尚未到位：先填 battle_id 取信息，再本地重放比对哈希。"（结论已清）、hOverflow=0、hitCount=1。fail-768 首轮探针曾因会话未就绪 hitCount=0（数据作废），重跑后与上表一致（data/fail-768-probe.txt）。

## 2 回归与不变量（静态 + 真浏览器）
- 令牌三锚点：laiyipao/miniapp/src/styles/tokens.css:14 --lyp-token-version: lyp-tokens-v1 / :15 --lyp-r3-version: lyp-tokens-v3-r3 / :16 --lyp-r6-version: lyp-tokens-r6 逐字在；全 src --r71- 仅 r71VerifyTimeFormat.contract.test.ts:79 断言 1 处，业务 0 命中。
- R60 键盘：src/pages 下 tabindex="0" 48 处、@keydown.enter 48 处；R61 aria：verify.vue:55 v-aria-native="'battle_id'"、:56 aria-label="battle_id"；R55：placeholder-class="ph" 全 src 17 处、verify.vue:61 在。
- verify.vue 不变量：:26 r5AiVerify.conclusion、:95 buildSnapshotCard v-for、:104 pretty(replayInfo.expected_hash)、fetchReplay if(fetching.value)return / finally fetching=false / catch 清 outcome（:451-461）保持。
- 写域：git diff --name-only HEAD 仅 .gitignore + laiyipao/miniapp/src/pages/verify/verify.vue；engine/replay/hash/score/settle/loot/tokens/后端零改。

## 3 门禁（全绿）
- G1 R71 契约 r71VerifyTimeFormat.contract.test.ts 5/5 passed RC0。
- G2 全量 vitest：222 files passed | 1 skipped（223）、2044 tests passed | 2 skipped（2046）RC0。
- G3 vue-tsc --noEmit RC0。
- G4 pnpm run build:h5 RC0（Build complete，Compiler 5.24 vue3）。
- G5 git diff --check RC0（仅 .gitignore LF→CRLF warning）。
- Go（laiyipao/server 下）：TestNoInfraIdentifiersInRepo PASS 21.33s RC0；TestNoRealSecretsInRepo **SKIP 非 PASS**，原文「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」，GO_SEC_RC=0。

## 4 截图 sha256（sha256sum 实测，报告逐字一致）
| 文件 | sha256 |
|---|---|
| r71-normal-390.png | 536909cc6aae182117f77e1039c02925eda98f8a011f5d3cd05786a2d40e2ea0 |
| r71-normal-768.png | abf3427740de0a581fd09611fc2d0144a7d20819b64dbdda6c26c566165a38db |
| r71-normal-1280.png | b1c9aa378a407f29706c1e68bd46232726c6be79eda2881f2762046930d48f6c |
| r71-bad-390.png | 536909cc6aae182117f77e1039c02925eda98f8a011f5d3cd05786a2d40e2ea0 |
| r71-bad-768.png | abf3427740de0a581fd09611fc2d0144a7d20819b64dbdda6c26c566165a38db |
| r71-bad-1280.png | 01f6369b2bbc6ebd240131e400c5c5325e33403cb124725ddb3a63728f66c41b |
| r71-double-390.png | 536909cc6aae182117f77e1039c02925eda98f8a011f5d3cd05786a2d40e2ea0 |
| r71-double-768.png | abf3427740de0a581fd09611fc2d0144a7d20819b64dbdda6c26c566165a38db |
| r71-double-1280.png | b1c9aa378a407f29706c1e68bd46232726c6be79eda2881f2762046930d48f6c |
| r71-fail-390.png | 04b8e5daf9f67162c888087528fda6a47296a31b14db50d8aaaff87aa2c32c1b |
| r71-fail-768.png | 4a21d2d1619c58b4c014198e06109222da2a88b75db4fe68df278a13e781a02f |
| r71-fail-1280.png | 279f56a1b599fbd3ef8f63ab63011cd659909e57f82a32eb14106759060ee8ca |

- 12 张主走查截图 UNIQUE=6/12：normal/bad/double 在 390/768 同视口共享 sha（记录时间行在首屏以下，可视区一致），1280 视口可见时间行故 normal/double(b1c9aa37) 与 bad(01f6369b) 区分；fail 三视口全不同。诚实披露：非全互异。
- 诊断附加截图：trial-768.png=abf3427740de0a581fd09611fc2d0144a7d20819b64dbdda6c26c566165a38db、trial5-768.png=abf3427740de0a581fd09611fc2d0144a7d20819b64dbdda6c26c566165a38db（同 768 可视区，诊断用不计入主表）。
- 亲读截图：r71-normal-768.png（战绩验真页，AI 复核提示齐备态）、r71-fail-768.png（同页错误态、r5 回到空态）、r71-bad-1280.png（同页「记录时间 not-a-date」肉眼可见）；均为非登录页，无横向溢出视觉异常。

## 5 清理
- data/auth-init.js 用 ops_stat 取 revision 后 ops_remove 删除，AUTH_AFTER=ABSENT（tracked 无 auth 文件）；QA 库只读，battle_records count|max=104|477 复核；自启 job 全 kill；零真值入库（mock 合成 battle_id/created_at 为占位，无 token/口令/域名/IP 回显）。