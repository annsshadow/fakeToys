# R61 真浏览器 QA 报告：表单输入语义与错误播报

- 任务：t110（kind=verification，attempt=1，attempt_id=afa2a3af-fc59-4a0e-9ee4-5c52666ddefd）
- 执行：same-route-qa（只读走查，不改业务源码、不 git 写、零真值）
- 工作根：本工作树根目录（HEAD=eead131e6）
- 日期：2026-11-26
- 结论：**FAIL / reject**。契约 4/4 过、静态源码与构建门禁全绿，但真 Chromium 实测核心修复未生效——17 处 `aria-label` 编译后落在 `<uni-input>` 外层自定义元素而非原生 `<input class=uni-input-input>`，AX 树 textbox 全部裸名，硬门禁①不满足。反馈节点 4/6 真浏览器坐实，2/6 本会话条件下未捕获（如实披露）。

## 1. 范围与方法

- 走查对象：12 页 × 三视口（390×844 / 768×1024 / 1280×800）= 36 组合。页：announcements/login/verify/signin/me/bag/equipment/enemies/gems/reactions/skills/skins。
- 工具：真 Chromium（agent-browser 0.41.1）；API 8080 + 静态 H5 5201（本地回环）；鉴权用 users.id=10 guest_token 换 token，会话侧 init-script 注入，不改 DB/服务端，凭据落 gitignored data/，不回显不入库。
- 客观量测：eval 抓 native input 的 outerHTML/attributes/AX 可访问名回退链、`[role=alert],[role=status],[aria-live]` 节点、role=button tabindex、横向溢出；agent-browser `snapshot` 读 AX 树；axe-core 4.12.1 `a11y --tags wcag2a --json` 独立审计；真实 click/fill 触发反馈。
- 证据落盘：docs/laiyipao/audits/round61-form-a11y/{data,screenshots}/（data/ 已在 .gitignore:553 预置忽略）。

## 2. 硬门禁逐条判定

| 门禁 | 结果 | 证据 |
|---|---|---|
| ①17 input 两态 AX textbox 带名 | **FAIL** | 11 个渲染页全部裸 textbox；对照实验证实 aria-label 落点错误（§3） |
| ②6 反馈节点 role=alert/status 文本 | PARTIAL(4/6) | login form-hint/errMsg、verify errMsg、signin redeemMsg 坐实；me passwordMsg、bag presetSaving 未捕获运行时（§4） |
| ③R60 键盘/R43-R48 CTA/R55 占位/令牌不回退 | PASS | 48 tabindex="0" + 48 @keydown.enter 在源码；focus-visible css:908 在；tokens 三锚点在、业务 0 处 --r61-；零溢出 |
| ④零横向溢出/CTA 越界 | PASS | 36/36 innerW==docW、overflowCount=0 |
| ⑤门禁全绿 | PASS | 契约4/4、全量2024 passed|2 skipped、type-check、build:h5、git diff --check RC=0；Go TestNoInfra PASS；TestNoRealSecrets SKIP(非PASS) |
| ⑥截图互异/亲读/清理诚实 | PASS | 36/36 sha 互异、亲读非登录页；auth 收尾删；诚实边界见 §7 |

## 3. 核心阻塞：aria-label 编译落点错误（门禁① FAIL）

源码逐字检查：`laiyipao/miniapp/src/pages/login/login.vue:33-55` 两个 `<input>` 均带 `aria-label="用户名"` / `aria-label="口令"`；契约 `r61FormA11y.contract.test.ts` 17 处断言全过；编译产物 `dist/build/h5/assets/pages-login-login.Cr1lHvbg.js` 含 aria-label 字符串。但真浏览器：

- DOM（`data/ancestry-login.json`）：native `<input class="uni-input-input">` 的 attributes 仅 `type/maxlength/step/enterkeyhint/class/autocomplete/inputmode`，**无 aria-label**；aria-label 实际落在外层 `<uni-input class="inp" aria-label="用户名">` 自定义元素上（uni-app H5 编译器把 `<input>` 包成 `<uni-input>` wrapper + native input）。
- AX 树（`data/ax-login-768.txt`）：`textbox [ref=e5]: admin`、`textbox [ref=e6]`——名称来自 value 回显，空值框裸名；11 个渲染页（announcements/login/verify/signin/bag/equipment/enemies/gems/reactions/skills/skins）snapshot 全部裸 textbox，无一处带 SPEC §2 的 aria-label 文案。
- 对照实验（本会话 eval）：对 native `<input>` 直接 `setAttribute("aria-label","QA测试口令名")` 后，snapshot 立即变为 `textbox "QA测试口令名" [ref=e8]`——证明 AX 可访问名只读 native input 属性，外层 uni-input 的 aria-label 无效。
- axe-core 4.12.1 `a11y --tags wcag2a --json`：`violations` 含 `label`(impact=critical, html=`<input ... class="uni-input-input">`, failure="Element does not have ... aria-label")；`incomplete` 含 `aria-prohibited-attr`：`aria-label attribute is not well supported on a uni-input with no valid role attribute`。这是独立于自研探针的第三方坐实，排除假阳性。
- 修复建议（供实施轮，QA 不改码）：uni-app H5 下须让可访问名落到 native input 本身（如以 `:aria-label` 透传至原生节点、wrapper 加合法 role、或改用 aria-labelledby 指向可见 label 并确认编译后 id 关联到 native input），并以真浏览器 AX textbox 带名为准，而非仅静态源码属性。

## 4. 6 处反馈节点播报验证

| 节点 | 真浏览器结果 | 证据 |
|---|---|---|
| login form-hint | PASS | role=status aria-live=polite，文本「请填写用户名与口令后可登录」；AX `status` 节点 |
| login errMsg | PASS | 空字段点登录触发 role=alert，文本「请填写用户名与口令」；AX `alert` 节点 |
| verify errMsg | PASS | 空 battle_id 点取复现触发 role=alert，文本「请输入有效的 battle_id」 |
| signin redeemMsg | PASS | 空码/非法码点兑换触发 role=status aria-live=polite，文本「输入非法: 兑换码无效或已用尽」 |
| me passwordMsg | 未捕获 | 本会话 user10 为登录态走 `!isGuest` 分支（修改密码），非游客升级分支；点击「修改密码」后提交按钮文案「确认修改」eval 未命中，`trig-me-submit` 返回 NO_BTN。源码 me.vue:599-607 有 role=status aria-live=polite，静态契约过，不虚构运行时截图 |
| bag presetSaving | 未捕获 | 「保存中…」为请求在途瞬时状态（bag.vue:414 role=status aria-live=polite，:1280-1293 置位/finally 复位）；本会话轮询 10×0.2s 未捕获（fill/click 触发方式受限），静态契约过，不虚构运行时快照 |

## 5. 36 组探针结果（摘要，完整 JSON 见 data/probe-*.json）

全部 36 组 hrefOK=Y、innerW==docW、overflowCount=0、err=0。渲染 input 数与可访问名：announcements/login(2)/verify/signin/bag/equipment/enemies/gems/reactions/skills/skins 各 1（login 2），**全部 noName=inputCount**；me 页 0 input（条件渲染）。反馈节点：login=2（status+alert）、gems=2、reactions=3，其余按页 0~1。role=button tabindex 实测 `tabindex="0"` 属性存在（`tabIndex` 计算值 0），源码 48+48 锚点在；早期汇总表的 roleButtonNoTab 非零系探针把字符串 `"0"` 与数字 `0` 严格比较的口径 bug，已用直接 DOM 检查纠正（详见 §3/§6 诚实边界）。

## 6. 门禁命令与结果

- `npx vitest run src/styles/r61FormA11y.contract.test.ts --reporter=verbose` → 4 passed(4)，RC=0。
- `pnpm test`（laiyipao/miniapp）→ 218 files passed|1 skipped；2024 tests passed|2 skipped；RC=0。
- `pnpm run type-check`（vue-tsc --noEmit）→ RC=0。
- `pnpm run build:h5` → DONE Build complete，RC=0。
- `git diff --check` → RC=0（仅 .gitignore LF→CRLF warning，无空白错误）；`git diff --check -- laiyipao/miniapp/src` → RC=0。
- Go `go test ./internal/domain -run TestNoInfraIdentifiersInRepo -count=1` → ok 8.533s，RC=0。
- Go `go test ./internal/domain -run TestNoRealSecretsInRepo -count=1 -v` → **SKIP 非 PASS**，逐字：「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」，Go 命令 RC=0 但测试未执行。

## 7. 诚实边界与非候选披露

- **本轮判定 FAIL**：核心门禁①（真实 AX textbox 带名）未通过，原因不是探针假阳性而是第三方 axe 亦报 label critical + aria-prohibited-attr；实施层虽满足静态契约，但真浏览器语义无效。按 QA 契约如实 fail，交实施轮修复后复验。
- 早期汇总表 roleButtonNoTab 非零是字符串/数字比较 bug，已以直接 DOM 检查（tabindex 属性 "0"、tabIndex 属性 0）纠正；R60 键盘不回退判定以源码 48+48 锚点与本次 DOM 真值为准。
- me passwordMsg、bag presetSaving 运行时未捕获，不虚构；以源码 role=status aria-live=polite 和静态契约通过为据，标注为未验证项而非通过项。
- login 页截图是**被测对象本身**（登录页为 12 页之一），不是误抓占位；其余 35 张均非登录页。
- 36 张截图 sha256 全互异（36/36 unique）；本轮亲读 r61-768-announcements、r61-768-login（均真实业务渲染），完整 sha 表见 data/ 与报告附录。
- 令牌三锚点 `lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6` 逐字在 tokens.css:14-16；业务 src 0 处 `--r61-`（仅契约断言）。
- 本轮为只读验证：未改业务源码、未 git 写；鉴权经 gitignored data/ 注入，凭据不回显不入库；收尾删除 auth-init.js、QA 库回 104|477、自启 job 全 kill。

## 8. 截图 sha256 表（36 张）

| 视口 | 页面 | sha256 |
|---|---|---|
| 390 | announcements | 6ab29fc32066bc9bf24d89331da6513646ad7db00a10aff955d316353b7fb10c |
| 390 | bag | d049a77f3c911548cf946b8be4029f4056c5c6b1d3de56be019c707c1e2322c9 |
| 390 | enemies | 28a81b5eaf4c028539b7de64b6f234095dab80eb60d4fb7e8f0a9308c3938fc7 |
| 390 | equipment | 02a102fb22b321b44d31507fc23b11fd711404ae962d64a54956737fcd258b43 |
| 390 | gems | 645545fb6f52d4bce0082674a7a64edc956ad195d63d0aac518941493500f752 |
| 390 | login | 23c101df48b49311bdb23ced2ed07a31fd6d536cf1522c06551e586ea6582564 |
| 390 | me | c376b765f5f9a7d866f7c93e28fb4f3a67fbb45ad0f15a45479e1d9554935d00 |
| 390 | reactions | 46ba749089e8ceb67d161824426138295de70a8c79a051bad9b213028e96e64e |
| 390 | signin | 98a367eecc080949de9b9582972aa4d898f710e524ebf1b45143f46c831abe25 |
| 390 | skills | deda2d38ffe617db7b7acf9bffd37a04efbf8e1a65d4fe8330fae5224d4ed947 |
| 390 | skins | 2b614718efcd720b27a5e570aad5308851378c447ac4c22af5af29668f8afc2f |
| 390 | verify | db8e5ee6bd29f2ceb11447379fd4e119030e583d787401d85c47e147aa8941b0 |
| 768 | announcements | 506038cca7088dcc695efb6b45ab7005bdb44fd9be4564e34abc223f45233523 |
| 768 | bag | dcfa4ce15494acd40544d900be5e8933f28a6a8532e1d965c2f6d904b948b7e7 |
| 768 | enemies | 035f2b863c60b025f44acbcdc960b0f38078715e9e54f132e48a0f28767ac1cc |
| 768 | equipment | 832afa3b928f413e3eaf876da02d222b9b3e2e36473816418651163f1af2082b |
| 768 | gems | d1cefcd56deca3211bcd1e8f9f1661112d2c87cda48176468146462e0a13b71a |
| 768 | login | 714330055e68d9535ce6df357a5ddaca7b6188f70f3b8a37264d2bbbb1052145 |
| 768 | me | bbe9c88c810911010fd306d06194337b2f9fd9db10709622225f19b29abf08b5 |
| 768 | reactions | fc429d4fe757a7810116dc3b2f82fc30ffa6c3c93739591b995237612244cef0 |
| 768 | signin | 9c8e14d81985f09f1f0d912bb1aeb9a31bb3d8d4c19090ed9096121af224e2c6 |
| 768 | skills | 67bc1bb4d47b8f461c0d62e0d75a7aa29bc5a2183bf7ee96f9e2199a8af13e25 |
| 768 | skins | 9a689d06aebec21f01d63bf5ea6dd7b0e87e79187b2502f16503838ab4a5cc70 |
| 768 | verify | c66b0d054b5547c603be53bde52b50b1e184dc4396fd83d7204d6a616b3f8ee8 |
| 1280 | announcements | 717c61c491af2d3a4555b43ce19df52f0a98de72789b767a29fc21ee7e8b3c35 |
| 1280 | bag | 339dbc2edb40193dadd61b790ba27ba7c0c3996ccfe287c6bb5ea808db93a9c2 |
| 1280 | enemies | 3917849b7ca3fb218d6e48f8e6165b6a87c66cfe7da40c45223614997bd3cf34 |
| 1280 | equipment | 268b042ce54fcbd94162c8c54a7c9bc2b84110041cbea8ebd7f9dacf27b1a81a |
| 1280 | gems | 590fd1133d54cf8eafdef2387cfa1e4d23fae48d12496287b9b45ba31c582e79 |
| 1280 | login | e0e2d87732df6627321e7d5b316d404875e600609bcbdba79cec29ea8063fe40 |
| 1280 | me | 7d258ed3fc8f9c764781d8d156e11e9afae2d079054e5d81f72081da322cfec1 |
| 1280 | reactions | b9552f492fe582841b100b1cd270151343b5287f5501ede16b8f5ec0a9b3678b |
| 1280 | signin | a43025f4faa07279d278453ee4696a342b3529d50acac8ed9a578c9e4acc9330 |
| 1280 | skills | 840f1ed139f275d3555c225d46d00f140c9aadcd60fe1ab423afe97bc4d68d16 |
| 1280 | skins | dcaa2cb0e0e27789e64283fba2fddf165b065f27d96e4c78472c25a32fc9a102 |
| 1280 | verify | 0e91c7cf51e3a348441e725da9df5a160a5c876b05bec7d4b916addc1e36e3bf |

## 9. 总体判定

**FAIL / reject**：契约静态层 4/4 与构建门禁全绿不构成真实可访问性通过。真浏览器 AX 裸 textbox + axe label critical violation 证明核心修复未生效，必须回实施轮修复 aria-label 落点后重新真浏览器复查。反馈播报 4/6 坐实、2/6 未捕获，不回退 R60/CTA/占位/令牌，零溢出；截图/清理/诚实边界均达标。
