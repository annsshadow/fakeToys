# R96 独立设计终审：六页空态 CTA 键盘可达补漏

> 轮次 R96 ｜ 任务 t231 ｜ attempt_id=d76a5fc6-1f61-4866-84b8-878569f93736 ｜ 角色 same-route-reviewer（R40 独立终审，继承队长路由）
> 基线 HEAD=85d5b0d91f384fdb0bda1625d9893680a9d3dfbf（R96-B 收官 clean，全程未变）｜ 审阅对象 t229 实施 + t230 QA ｜ 报告 UNTRACKED ｜ 中文零真值
> reviewedTaskId=t229 ｜ 候选来源 t226 / 规范 t228 / 实施 t229 / QA t230

## 0. 终审判定

**verdict = pass（六维全绿，无 blocker / high / medium）。** 同意进入后续分批发布流程。

R96 补的是 R60/R95 权威范式的结构性盲区：六页空态 CTA 原为裸 `<view class="r3-empty-action" @click="X()">`，无 `role="button"` / `tabindex="0"` / `@keydown.enter`，键盘 Tab 不命中、AX 树不暴露 button、Enter 无效（WCAG 2.1.1）。实施以每页 1 行（共 6 行 +1/-1）补齐四件套，与同仓其余 10 处 r3-empty-action 范式逐字一致。独立复跑确认：RED 4 failed | 4 passed → GREEN 8/8；三视口 × 六页 18 组全绿；门禁 G1–G7 全 RC0。5 项 low 观察均非阻断，已逐条判定。

## 1. 维度一：test-first 真实性 —— PASS

证据：
- 规范 t228 的 `docs/laiyipao/specs/round96-empty-cta-keyboard/SPEC.md`（53 行）§0–§7 齐备：来源、缺口机制、权威范式、唯一改点 6 行、不变量、契约结构（8it）、RED 门禁实测、写域白名单。
- 契约 `laiyipao/miniapp/src/styles/r96EmptyCtaKeyboard.contract.test.ts` 135 行 / 8 it：以 `import.meta.url` + `fileURLToPath` + `dirname/join` 真 `readFileSync` 读 6 页源码，禁内嵌实现副本。it1–it4 为缺口断言（role / tabindex / @keydown.enter / 四属性顺序），it5–it8 为不变量（焦点函数声明 / 空态四段槽位顺序 / 令牌三锚点+零 --r96- / R60 范式抽查≥2 处）。`ctaLine()` 用 class + @click 原函数 + 原文案三条件锁定唯一行并断言 `hits.length=1`，断言非弱化。
- RED 存档 `docs/laiyipao/specs/round96-empty-cta-keyboard/RED-evidence.md`（94 行）：RED_EXIT=1，Test Files 1 failed(1)，Tests **4 failed | 4 passed (8)**，4 项失败全为缺口断言（it1–it4，Received 逐字为裸 `<view class="r3-empty-action" @click="r4GoBag()">去关卡页 →</view>`），4 项不变量（it5–it8）全通过。仅将工作树绝对路径替换为 `<WORKTREE>` 占位符，断言原文未改，脱敏合规。SPEC §6 另载独立复跑 RERUN_EXIT=1 同型稳定。
- 我独立复跑 GREEN 契约：`pnpm exec vitest run src/styles/r96EmptyCtaKeyboard.contract.test.ts --reporter=verbose` → G1_RC=0，8/8 全通过（报告 `review-gates2.txt:1-19`）。
判定：RED 完全来自真实缺口、无假阳性；GREEN 独立复跑 8/8；test-first 链真实可追溯。

## 2. 维度二：实施外科性 —— PASS

逐页 `git diff` 全文核对（六页各 +1/-1，numstat 全为 `1 1`）：
- `laiyipao/miniapp/src/pages/bag/bag.vue:43`：`<view class="r3-empty-action" role="button" @click="r4GoBag()" tabindex="0" @keydown.enter="r4GoBag()">去关卡页 →</view>`
- `laiyipao/miniapp/src/pages/battles/battles.vue:177`：同型，`r4GoBattle()` / 「去关卡页挑一关开打 →」
- `laiyipao/miniapp/src/pages/enemies/enemies.vue:27`：同型，`loadPage()` / 「重新加载配置」
- `laiyipao/miniapp/src/pages/me/me.vue:72`：同型，`r4GoMe()` / 「去专精页 →」
- `laiyipao/miniapp/src/pages/rank/rank.vue:74`：同型，`r4GoRank()` / 「去冲榜关卡 →」
- `laiyipao/miniapp/src/pages/skins/skins.vue:32`：同型，`loadPage()` / 「重新加载配置」

六行均保持 `class` / `@click` 绑定 / 文案逐字不变；属性顺序严格 `role="button"` → `@click` → `tabindex="0"` → `@keydown.enter`，与 SPEC §3 及同仓 R60/R95 范式一致。diff 仅此六行，不动脚本、样式、令牌、焦点函数实现，不动 .gitignore（.gitignore 的 M 系队长预置 R96 数据目录忽略规则，无真值）。`git diff --check` 复跑 RC0（仅 .gitignore LF→CRLF warning）。
判定：改动面最小、无越界、外科性成立。

## 3. 维度三：不变量零回退 —— PASS

- 令牌三锚点独立 grep + 直读 `laiyipao/miniapp/src/styles/tokens.css:14/15/16`：`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 全在。
- 业务源码零 `--r96-`：对 `laiyipao/miniapp/src` 的 *.vue/*.ts/*.css 独立 grep 命中 0（契约文件断言文本中的 `--r96-` 不算业务源码，且被契约 walk 主动排除 `.test.`）。
- 空态四段槽位顺序 `r3-empty-slot < title < reason < action` 六页未变（契约 it6 GREEN + 六页 diff 行上下文可见）。
- 六处焦点函数声明保留：`r4GoBag` / `r4GoBattle` / `loadPage`(enemies) / `r4GoMe` / `r4GoRank` / `loadPage`(skins)（契约 it5 GREEN）。
- R60/R95 范式不回退：契约文件 `laiyipao/miniapp/src/styles/r60A11yKeyboard.contract.test.ts` 与 `r95BackKeyboard.contract.test.ts` 独立 glob 确认存在；R96 契约 it8 GREEN（announcements/equipment/skills 抽查≥2 处 role+tabindex+@keydown.enter 仍在）。另独立复跑 `me.test` G2_RC=0、29/29（含 R95 返回按钮键盘用例）。
- auth 四路径独立 glob 全 ABSENT：`laiyipao/miniapp/.env.local`、`.env.local.keys`、`laiyipao/admin/.env.local.keys`、`laiyipao/deploy/.local.keys`。零真值写入。
判定：全部不变量保持，无回退。

## 4. 维度四：QA 可信度与诚实边界 —— PASS

QA 报告 `docs/laiyipao/audits/round96-empty-cta-keyboard/REPORT.md`（89 行 / 10599B）三视口 390×844 / 768×1024 / 1280×800 × 六页 18 组全绿：每组 `role=button`、`tabindex="0"`、`tabIndex=0`、`cursor=pointer`，AX 树可见 button 角色，Tab 命中 CTA（me 页因先命中 R95 返回按钮需 2 次），`activeIsCta=true`，焦点环 `solid / rgb(255,107,53)`，Enter 事件 `target.isCta=true` 送达 CTA handler，`iw=cw=sw` 零横溢。改前 R96-A 基线 battles 三视口 `role=null` / `tabIndex=-1` / Tab×8 全停 BODY，18 组全部修复。
独立复核：18 张截图 SHA 我抽 4 张 `sha256sum` 与报告逐字一致（battles-768=0636b865…、bag-390=cde82e5c…、me-1280=f465c905…、skins-768=85fc705f…）；队长 ops_stat 已核 18/18 一致。我亲读 `r96-battles-768.png`（768×1024）：清晰显示「我的战报」空态卡片与 CTA「去关卡页挑一关开打 →」带橙色焦点环，坐实 QA 观测。

诚实边界逐条判定（均未构成阻断）：
- **enemies/skins `loadPage()` 幂等 no-op、me `r4GoMe()` 空态 `r4AiMe=null` 提前 return**：三页 × 三视口 Enter 事件均 `target.isCta=true` 送达 handler，但无 DOM/导航变化。R96 验证目标是「键盘可聚焦可聚焦可激活」，键盘绑定链完整，无变化系既有业务守卫而非绑定失效。判定：**非阻断**（low，建议后续业务轮单独验证 reload 语义）。
- **焦点环 390/1280 为 1px、768 为 3px**：系 `3rpx` 不同 DPR/视口下渲染差异，颜色与样式一致（`solid` / `rgb(255,107,53)`）。判定：**非阻断**（low，视觉等效）。
- **三个作废脚本版本披露**：首版 8 次 Tab 偏离 CTA、转义 JSON 匹配失败、未注入 Enter observer，均已重写，仅最终版 18 组数据计入结论。判定：**非阻断**（low，披露充分）。
- **截图亲读 2/18**：其余 16 张仅列 SHA-256。我本轮亲读 battles-768 1 张 + QA 已亲读 battles-768 与 me-1280，DOM 探针 18/18 坐实全部指标，SHA 抽查一致。判定：**非阻断**（low）。
- **后端未启动、空态来自本地兜底**：六页均自然复现空态 CTA（`ctaCount=1`），零 mock、零人为构造。判定：**非阻断**（说明充分）。

## 5. 维度五：门禁独立复跑 —— PASS

我独立前台/后台复跑全套门禁（`docs/laiyipao/audits/round96-empty-cta-keyboard/review-gates2.txt`，job bash-bdfb85cce5874014 exit 0），逐项 RC：
- G1 R96 契约：`8/8 passed`，G1_RC=0（:19）
- G2 `me.test`：`29/29 passed`，G2_RC=0（:1088）
- G3 全量 vitest：`228 files passed | 1 skipped (229)`、`2081 tests passed | 2 skipped (2083)`，G3_RC=0（:8768）
- G4 `vue-tsc type-check`：RC0（:8771）
- G5 `build:h5`：RC0（:8782）
- G6 `git diff --check`：RC0（:8785，仅 .gitignore LF→CRLF warning）
- G7 Go 守卫（`go test ./internal/domain/ -run "TestNoInfra…|TestNoRealSecrets…" -v`）：G7_RC=0（:8794）
  - `TestNoInfraIdentifiersInRepo` **PASS** 8.22s（:8791）
  - `TestNoRealSecretsInRepo` **SKIP 非 PASS**（:8789），逐字「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」。如实标注，**不把它计入「通过」项**。
判定：前端 G1–G6 全 RC0；Go 一项 PASS 一项 SKIP，SKIP 已如实披露。门禁可信。

## 6. 维度六：保护面零回退与诚实边界充分性 —— PASS

- 写域精确：`git status --porcelain` 非 `??` 项仅 `M .gitignore` + 六页业务 vue，与 t229 声明一致，无越界改动。
- 真值零入库：文本、报告、契约、截图均无密钥/token/口令/私钥/真实域名与 IP；RED 存档用 `<WORKTREE>` 占位；auth 四路径 ABSENT；`.gitignore` 仅新增 R96 数据目录忽略规则。
- HEAD 全程未变 85d5b0d9；QA 库只读（前序记录 `users` id=477 存在、104 无记录，本轮未写）；自启门禁 job 已 completed，无遗留服务。
- 诚实边界覆盖完整（SKIP 非 PASS、三页 Enter 无 DOM 变化、DPR 焦点环差异、作废脚本披露、亲读 2/18、后端未起本地兜底），无夸大、无掩盖。
判定：保护面无回退，诚实边界充分。

## 7. Findings（5 项 low，均非阻断）

| id | severity | problem | requiredFix |
|---|---|---|---|
| R96-LOW-1 | low | enemies/skins/me 三页 Enter 送达 handler 后因既有业务守卫无 DOM/导航变化 | 后续业务轮单独验证 `loadPage` 幂等与 `r4GoMe` 空态 return 语义，不属 R96 键盘绑定范围 |
| R96-LOW-2 | low | 焦点环宽度 390/1280 为 1px、768 为 3px（3rpx DPR 差异） | 视觉等效，无需修改；若要求像素级一致另开视觉轮 |
| R96-LOW-3 | low | QA 首版三脚本作废，仅最终版数据计入结论 | 已充分披露，保留最终版即可 |
| R96-LOW-4 | low | 截图亲读 2/18，其余仅 SHA 校验 | 我本轮补亲读 battles-768 + DOM 探针 18/18 坐实，无需补读 |
| R96-LOW-5 | low | Go `TestNoRealSecretsInRepo` 因缺 .local.keys SKIP 非 PASS | 保持如实标注；真值扫描需部署环境注入 keys 后另行验证 |

无 blocker / high / medium。

## 8. 终审结论

六维全绿，verdict=**pass**。R96 六页空态 CTA 键盘可达补漏链（侦察四条件全中 → 规范 RED → 实施 6 行 GREEN → QA 三视口 18 组 PASS）完整、真实、可追溯；不变量与保护面零回退；诚实边界充分。同意进入后续分批发布。

> 本报告为独立终审，未修改任何业务源码、契约、规范、QA 审计文件、.gitignore 或 git 状态。
