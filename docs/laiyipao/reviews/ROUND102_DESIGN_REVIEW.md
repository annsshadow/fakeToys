# R102 独立终审报告（首页 CTA 键盘可达补漏）

- **审查角色**：same-route-reviewer（独立终审）
- **审查对象**：t271 实施（reviewedTaskId）
- **基线 HEAD**：`5ec099e5508b853a4cc9b8936d09d76ec1020589`（全程未变，实施未提交）
- **候选来源**：t269 R102-B 侦察 `docs/laiyipao/recon/ROUND102_B_INDEX_CTA_KEYBOARD_RECON.md`
- **verdict**：**pass**（六维全绿，无 blocker / high / medium；8 条 low 观察不阻断）
- **发布建议**：同意进入分批发布

## 审查范围与写域

唯一写域 = 本报告。Out of scope：业务源码 / 契约 / QA 报告 / SPEC / 令牌 / `.gitignore`——本轮均为只读审查，未改动。
实测非 `??` 改动仅 `.gitignore` 与 `laiyipao/miniapp/src/pages/index/index.vue`，均在 t271 预期写域内。

## 维度一：test-first 真实性 —— PASS

- **SPEC**：`docs/laiyipao/specs/round102-index-cta-keyboard/SPEC.md` §0 候选来源与四条件全中裁定、§1 权威口径（R60/R95/R96/R99 + WAI-ARIA）、§2 精确改点 5 处、§3 不变量、§4 契约清单、§5 RED 预期、§6 验证命令与诚实边界，齐备。
- **RED 原始证据**：`RED-evidence.md` 逐字记录命令 `cd laiyipao/miniapp && pnpm exec vitest run src/styles/r102IndexCtaKeyboard.contract.test.ts --reporter=verbose`，`exit=1`，`Tests 5 failed | 4 passed (9)`，vitest v4.1.12，Duration 268ms。5 处失败全部断 `role="button"`，Received 原文坐实五处开标签缺三属性；4 处不变量通过（R99 `go(d.hint)` / R100-R101 `ann-item` / 四处 `@click` 保留 / 令牌三锚点且零 `--r102-`）；无 PARSE_ERROR。
- **契约**：`laiyipao/miniapp/src/styles/r102IndexCtaKeyboard.contract.test.ts`（4306B）用 `import.meta.url` + `fileURLToPath` + `readFileSync` 真读 `index.vue` / `tokens.css`；`tagBefore` 按唯一属性串向前找 `<tag`、向后找 `>`，跨行容错不依赖行号；CASES 5 处（it1-5 各验 role / tabindex / `@click` 保留 / `@keydown.enter` 同名 handler）+ it6-9 四不变量，共 9 it。
- **GREEN**：本轮独立复跑 R1 = 9/9（见维度五）。RED→GREEN 对照成立。

## 维度二：实施正确性 —— PASS

- `git diff` 精确 5 处，每处仅改一行（`index.vue` 业务净 +5/-5 标签）：
  - `:40` `view.btn.btn-primary`（重试）→ 补 `role="button" tabindex="0" @keydown.enter="retry"`，`@click="retry"` 保留；
  - `:53` `view.card.first-run-guide`（新手引导）→ 补三件套，`@click="goStage"` 保留；
  - `:78` `view.btn.ai-cta`（去补强）→ 补三件套，`@click="go(aiInsight.hint)"` 保留；
  - `:92` `view.btn.ai-cta`（去战斗）→ 补三件套，`@click="goStage"` 保留；
  - `:109` `text.ann-all`（全部公告）→ 补三件套，`@click="goAnnouncements"` 保留。
- 每处三属性落同一开标签行，满足 R60 逐行扫描；`@keydown.enter` 与 `@click` 同名 handler；`@click` 与文案逐字未动；与 SPEC §2 逐处吻合。
- `.gitignore` +5 行为 round102 审计目录忽略（沿袭 round99 防真值入库），卫生配置非业务改动。

## 维度三：不变量与写域 —— PASS

- 令牌三锚点 `laiyipao/miniapp/src/styles/tokens.css:14/15/16` 在（`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`）。
- 业务零 `--r102-`：pages 下 NO_MATCH，仅契约断言字符串命中，无业务令牌漂移。
- 写域仅 `.gitignore` + `index.vue`，未触碰业务其它页 / 契约 / QA / SPEC / 令牌。

## 维度四：QA 可信度 —— PASS

- **QA 报告**：`docs/laiyipao/audits/round102-b-index-cta/REPORT.md`，三视口 390×844 / 768×1024 / 1280×800 真 Chromium 坐实五处 CTA `role=button` / `tabIndex=0` / `cursor=pointer` / `vis=true` / AX `button` / 真 Tab 命中 / Enter 送达 `isCta=true`（#39），零横溢。后段 8080 未起，`--init-script` 合成零真值数据，错误态 guest 返回 500 合成。
- **12 图 SHA 独立重算 12/12 一致且 UNIQUE**：
  - `r102-index-390.png` = `4a5f89beb08530201949b9531199224afb9149995e8106eca214c4260946bf7d`（76233B）
  - `r102-index-768.png` = `c8f0865db5c2d649f7b0d864906ea4f84f5e4b5a37456ef3db261e88f4a81bca`（99499B）
  - `r102-index-1280.png` = `b72320fbd95b3fc0e0daedfd08720cc6a75034b463a9c403fca77d7681bd8d12`（81733B）
  - `r102-err-390.png` = `8b05db7d2fd28c7132b091c56ddd1ff4e20b47107d396b7777fc09be12ee129a`（40846B）
  - `r102-err-768.png` = `257fc37c9d13a0abeedbd540e88ce29c49cb805c8627e1f59be6709477cd5d4f`（92207B）
  - `r102-err-1280.png` = `9b19436fb0459d826c2bde75aa33a63c2670510fb84e1e11fcb2813215c1a4bb`（55564B）
  - 注：QA 目录另含 `r102-explore-*` 与 `r102-error-*` 等改前侦察遗留图，不计入本轮 12 图。
- **亲读 2 张**（原图 SHA 与上一致）：`r102-index-768.png` 见 QA 玩家资源栏、构筑评分 D30 雷达图、「→ 去背包配元素」「→ 看反应搭配」CTA，非空态；`r102-err-1280.png` 见「无法连接服务器 / 请求失败 HTTP 500」+ 橙红「重试」大按钮，非空态。
- **诚实边界**（QA 已披露，终审采信）：mock 合成非真值；焦点环探针 `outlineStyle=none` 系 JS focus 不触发 `:focus-visible`，由 `common.css:914-919` 静态保证；Enter 用 stub 未做跳转后 DOM 断言；`ann-item` 未做展开态 DOM 断言；亲读 2/6。

## 维度五：门禁独立复跑 —— PASS

自建 `docs/laiyipao/recon/.r102-review-gates.sh`，后台 job `bash-b57575058b614861` 全 RC0，`REVIEW_GATES_DONE`：

- R1 契约 9/9；R2 r60 6/6；R3 r99 24/24；R4 r100 12/12；R5 r101 9/9；R6 `index.test.ts` 27/27；
- R7 全量 234 files passed (1 skipped) / 2150 tests passed (2 skipped)；R8 type-check；R9 build:h5；R10 git diff --check。
- 另：Go 守卫 `cd laiyipao/server && go test ./internal/domain/... -run "TestNoInfra|TestNoRealSecrets" -count=1` PASS（9.531s RC0）。

## 维度六：保护面零回退 —— PASS

- R99 `go(d.hint)`：`index.vue:189-192` 三件套在；
- R101 `ann-item`：`index.vue:114-119` 三件套 + `:aria-expanded="String(expanded.has(a.id))"` 在；
- R95 `me.vue:11` 返回键 `role/tabindex/@keydown.enter` 在，`me.vue` 多处 link 范式在；
- 令牌三锚点在（见维度三）；`index.vue` `@keydown.enter` 共 12 处（新增 5 + 既有 7，无减少）；
- HEAD 未变 `5ec099e5...`，实施未提交。

## Low 观察（不阻断）

- **L1**：QA 后端 8080 未起，数据用 `--init-script` 合成（零真值），坐实客户端因果而非服务端真实结果。
- **L2**：12 图亲读 2/6（报告涉及的核心 6 图），其余由 SHA 独立重算一致保证。
- **L3**：焦点环探针 `outlineStyle=none`，因 JS focus 不触发 `:focus-visible`，由全局样式静态保证。
- **L4**：Enter 用 stub 验证事件送达，未做跳转后 DOM 断言。
- **L5**：`ann-item` 未做展开态 DOM 断言，由 R101 契约静态保证。
- **L6**：`.gitignore` +5 行卫生配置（round102 审计目录忽略），沿袭 round99 防真值入库。
- **L7**：QA 目录含改前侦察遗留图（`r102-explore-*` / `r102-error-*`），不计入本轮 12 图。
- **L8**：全量 vitest 有 1 file / 2 tests skipped，属仓库既存 skip，非本轮引入。

## hygiene

- HEAD 未变 `5ec099e5508b853a4cc9b8936d09d76ec1020589`；
- 非 `??` 改动仅 `.gitignore` + `index.vue`（t271 预期写域）；
- `git diff --check` RC0；
- auth 五路径（`.env` / `.local.keys` / `remote-config.env` / `laiyipao/deploy/.local.keys` / `laiyipao/server/.env`）经 `git check-ignore` 全部 IGNORED，`laiyipao` 下 `.pem`/`.key` = 0；
- 报告无真值；自启 job 全 completed，无 running。

**结论**：六维全绿，verdict = **pass**，同意进入分批发布。
