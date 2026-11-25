# R53 首页 AI 结论整数百分比 · 真浏览器复核报告

- 任务：t80 R53 QA：首页AI结论整数百分比三视口真浏览器复核
- attempt_id：37e352b8-4267-4416-9975-44c3a25fc326
- 结论：**PASS（带诚实边界）**
- 工作树：D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248（HEAD=6d0c54c81）
- 范围：只写本审计目录；未改业务源码、未做 git 写操作。

## 1. 门禁逐条结论

| # | 门禁 | 结论 | 证据 |
|---|------|------|------|
| 1 | 三视口 AI 结论整数百分比无长浮点 | PASS | 390×844 / 768×1024 / 1280×800 三视口 case1/2/3 probe：conclusion 逐字「装备契合仅 44%（权重 21%）：优先补它，收益 210」，longFloat=false；innerW==docW（390/768/1280），无溢出 |
| 2 | 五维进度条宽度与雷达几何 | PASS | dims 宽度 element100%/reaction43%/mastery100%/equipment44%/mechanic100%（Math.round）；雷达 viewBox=0 0 200 180、rings=3、axes=5、五中文标签不溢出、shape 多边形可见；空维 pct===0 判定未变（empty case 五维 width 0%、引导「→ 去背包配元素」） |
| 3 | 回归 R12 四页 / R51 me / R48 自指防护 | PASS | skills/equipment/gems/bag 均判同短板装备契合（r12Text「装备契合 8/18（44%，权重 21%）…优先补它」）；equipment 页 CTA「在本页强化装备契合 →」保持自指，其余三页「去配装备 →」；me 页整数 44% + 雷达 viewBox/rings/axes/honesty 全部保留；element 短板 case（元素覆盖 0/5）skills CTA「去背包配元素 →」与权威 helper 一致 |
| 4 | 代码门禁 | PASS | R53 契约 r53HomePct.contract.test.ts 4/4 RC=0；miniapp 全量 214 files passed\|1 skipped，2006 tests passed\|2 skipped RC=0；type-check RC=0；build:h5 RC=0；git diff --check RC=0 |
| 5 | Go 守卫 | PASS（含 SKIP 标注） | TestNoInfraIdentifiersInRepo PASS(10.44s)；TestNoRealSecretsInRepo **SKIP 非 PASS**，原文见 §5 |
| 6 | 零真值 / 清理 | PASS | auth 注入仅在 gitignore data/ 目录，收尾已删；QA 库 battle_records count\|max=104\|477；审计目录 secret scan 0 命中；截图 sha 互异且亲读非登录页 |

## 2. 浏览器证据（真 Chromium 390×844 / 768×1024 / 1280×800）

- 服务：API 8080（start-api.sh 后台）、H5 静态 5201（bundled Python http.server），打开前均验证精确端口 HTTP 200；端口 5174 是旧 302 服务，未使用。
- 认证：QA 库 user10 活 alias → POST /api/v1/auth/guest → localStorage lyp_token 等（无任何真值回显）；build_rating 与 rating_weights 通过浏览器 network route 会话侧注入（allow-origin=页面源 http://127.0.0.1:5201）。
- 三视口 case1/2/3 href 均为 http://127.0.0.1:5201/#/pages/index/index，结论逐字「装备契合仅 44%（权重 21%）：优先补它，收益 210」，无 44.44444444444444% 长浮点；收益条 mg=[装备契合(还差10)收益210, 反应链(还差4)收益104, 元素覆盖(还差0)收益0]。
- 雷达 shape（case1/3）：`100,20 128.62680114048413,80.69858846931409 141.14496766047313,146.63118960624632 81.89621422939183,114.91772342674838 33.426043859339245,68.36881039375369`；case2 首点 100.0,63.2（element 2/5 映射）。几何按 value/cap(5/7/8/18/8) 映射，无 NaN / 负宽。
- 回归六页 reg4 driver：skills / equipment / gems / bag / me / element（element 短板 mock），全部 href 合法 deep link、innerW=docW=390，字段结论见 §1 第 3 项。

## 3. 截图 SHA256（互异、亲读非登录页）

| 截图 | 视口 | 内容 | sha256 |
|------|------|------|--------|
| r53-case1.png | 390×844 | 首页 AI 结论 44% 整数 | 028a4520f60de3513f64bcfdc53e84bd7dd3517915c6947e34b875c10cb06417 |
| r53-case2.png | 768×1024 | 首页 AI 结论 44% 整数 | e05869677508c8e0e3eb79f80e2fa63da60c627debfe128f5f497befcf5f68fd |
| r53-case3.png | 1280×800 | 首页 AI 结论 44% 整数 | f744ad3e470d107a904ed0a8b2040b5d5a663628065ad3b807c13fc75ec832e6 |
| r53-empty.png | 390×844 | 首页空 rating（0% 兜底） | dc9b9c1247c4bd92648f26914db839b0da7d5076a48d5483baea5bb5ef0a4719 |
| r53-reg4-equipment.png | 390×844 | 装备图鉴 · R48 自指 CTA | edfa76305e2fc1a99e5323262114e6a57cd8006e07cadafab977c8681b66c2f4 |
| r53-reg4-me.png | 390×844 | me 页 · 整数 44% + 五维雷达 | db55d757690e7ae28426dd12f224d59073dcf6ae9f0a12e3b5a818021871d01e |
| r53-reg4-element.png | 390×844 | 技能图鉴 · 元素覆盖短板 | dd7eb60c9cfae15a8c62d1bf6a026a018421c0dc74a9aeade6d87881c2c8845c |
| r53-reg4-skills.png | 390×844 | 技能图鉴 · 装备契合短板 | e23de2aac9306ed6357baf701ebcdbb6d566f138504210e2d60ec57da3d3eab3 |

## 4. 门禁命令记录

- `pnpm exec vitest run src/styles/r53HomePct.contract.test.ts`：4 passed，RC=0
- `pnpm test`（miniapp 全量）：214 files passed | 1 skipped，2006 tests passed | 2 skipped，RC=0
- `pnpm run type-check`：RC=0
- `pnpm run build:h5`：DONE Build complete，RC=0
- `git diff --check`：RC=0（仅 .gitignore LF→CRLF warning）
- `go test ./internal/domain -run TestNoInfraIdentifiersInRepo`：PASS(10.44s)
- `go test ./internal/domain -run TestNoRealSecretsInRepo`：**SKIP**，原文见 §5

## 5. 诚实边界

1. **TestNoRealSecretsInRepo SKIP 非 PASS**：原文「读不到 <WORKTREE_ROOT>/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」。本报告不将其记为通过。
2. **build_rating / config 为会话侧 mock**：通过 agent-browser network route 在浏览器侧替换 GET /api/v1/me 与 /api/v1/config 的响应，仅作用于该浏览器会话，未改 DB 或服务端；权重值与服务端权威一致 {12,15,10,12,8}。
3. **空 rating 分支未触发**：mock build_rating:null 时首页仍由 `?? 0` 兜底产出 aiInsight（「装备契合仅 0%…收益 378」、五维 width 0%），SPEC §4 期望的「AI 暂时没有足够构筑数据」文案分支未出现；如实披露，不伪称触发。
4. **早期 driver 缺陷已弃用**：首版 r53-driver.sh 用 NAME 同时作输出名与 mock 名导致 case2/3 写成 case1 探针（ENOENT），修 OUT/BODYNAME 分离后重跑有效；回归首版 skills/ 尾斜杠 URL 非法、driver2 的 `$4` 未传导致 unbound、driver3 的 echo 被 JS 模板提前展开，均已修复为 driver4，旧无效探针与截图剔除。
5. **重复截图剔除**：跨 driver 尝试的同内容截图（如 reg4-me 与旧 R51 case1 sha 相同、reg4-bag/gems 与旧 driver 相同）只在本报告列一次最终 reg4 集合。
6. **端口纪律**：5174 旧 302 服务不用；静态 5201 与 API 8080 在打开浏览器前均验证精确端口 HTTP 200，避免 port 漂移产生 chrome-error 错误页证据。
7. **首页 aiInsight 长浮点修复验证口径**：三视口只断言结论字符串不含长浮点；权重份额已 Math.round 为 21%，与服务端 12/57≈21.05% 的整数表达一致。

## 6. 收尾

- 已删除 data/auth-init.js 等真值注入文件；git check-ignore 命中 .gitignore:506 round53 data/ 规则。
- QA 库 battle_records count|max=104|477（与基线一致）。
- 审计目录 secret scan（115 文件）仅命中报告自身 2 处审计描述文字（门禁表与本节中的「secret scan」字样），凭据真值 0 命中；data/ 临时真值文件已全部删除。
- 自启 API / H5 / 浏览器 jobs 全部 killed 或 completed。