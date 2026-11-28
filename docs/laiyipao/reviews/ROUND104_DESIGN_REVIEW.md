# R104 独立终审报告 — 10 页加载态 a11y 语义补齐

## 元信息

- 任务：t285（kind=review，reviewedTaskId=t284 QA，实施 t283）
- attempt_id：3a65a46f-d38f-4062-9a82-13a80756f341
- 基线 HEAD：b4e0194ca46de1bfe022315b289ee5fc9e6a0a9c（全程未变，实施未提交）
- 审查对象：10 页加载根补 a11y 语义 —— A 类 7 页（v-if=loading / class=card 补 role=status + aria-busy=true + aria-label=数据加载中）；B 类 3 页（equipment/reactions/skills 的 v-if=r6Loading / class=r6-skeleton r6-enter 去 aria-hidden=true 并补同三属性）
- 全链：t280 侦察 → t282 规范+RED → t283 实施 → t284 QA PASS → 本轮终审
- **verdict = pass**（六维全绿，无 blocker/high/medium，7 条 low 观察不阻断，同意分批发布）

## 维度 1 · test-first 真实性 —— PASS

- SPEC：docs/laiyipao/specs/round104-loading-a11y/SPEC.md（72 行）§0–§5 齐备（四条件全中裁定、权威口径 MDN aria-busy + 同仓 admin 骨架范式、逐页改点清单、静态写死等价性论证、不变量）。
- RED 逐字证据：docs/laiyipao/specs/round104-loading-a11y/RED-evidence.md 记命令 `cd laiyipao/miniapp && pnpm exec vitest run src/styles/r104LoadingA11y.contract.test.ts --reporter=verbose`，vitest v4.1.12，exit=1，Test Files 1 failed，Tests 10 failed | 3 passed (13)，Duration 490ms。10 失败全为缺口断言（7 处 role=status 缺失 + 3 处 aria-hidden 未移除）且附 Received 原文坐实；3 条不变量断言通过；无 PARSE_ERROR。RED-raw.log:191-192 独立复核逐字一致。
- 契约：laiyipao/miniapp/src/styles/r104LoadingA11y.contract.test.ts（116 行 / 13 it）以 import.meta.url + readFileSync 真读 10 个 vue 与 tokens.css；tagBefore 按 class+v-if 唯一属性串锚点向前找开标签、向后找闭标签，跨行容错且不依赖行号。A 类锚点 v-if="loading" class="card"，B 类锚点 v-if="r6Loading" class="r6-skeleton r6-enter"。
- GREEN：本轮门禁 G1 复跑 13/13 RC0。契约断言能随业务语义变化而失败（RED 真实失败），符合测试验证意图。

## 维度 2 · 实施外科性 —— PASS

- git diff 业务 10 页各精确 1+/1-（numstat 全 1+/1-，合计 10+/10-），.gitignore 另 +5 行审计目录忽略（沿袭 round99 防真值入库）。
- A 类 7 处：announcements.vue:43 / bag.vue:36 / battles.vue:164 / mastery.vue:45 / rank.vue:62 / shop.vue:140 / signin.vue:36，各在开标签补 role=status + aria-busy=true + aria-label=数据加载中。
- B 类 3 处：equipment.vue:45 / reactions.vue:62 / skills.vue:63，各去 aria-hidden=true 并补同三属性。
- v-if 与页面文案逐字不动，每处一行，与 SPEC §2 逐处吻合；无 handler/逻辑/DOM 结构改动。

## 维度 3 · QA 可信度 —— PASS

- QA 报告：docs/laiyipao/audits/round104-loading-a11y/REPORT.md（101 行）。三视口 390×844 / 768×1024 / 1280×800 × 10 页，加载期 role=status + aria-busy=true + aria-label=数据加载中（B 类 hidden=null 缺席）全部坐实；卸载后 busyCount=0、skelCount=0；零横溢 iw=cw=sw。
- DOM 探针原始输出已核（data/w-390.txt）：announcements 加载态 busyRoots=[{tag=UNI-VIEW, cls=card, role=status, busy=true, label=数据加载中, text=加载中…}]，lives=1；卸载后 busyCount=0。bag 等同构。
- AX 完整 snapshot（data/ax-full-out.txt）在 equipment 与 announcements 两页坐实 `- status "数据加载中"` 节点，与 DOM 一致。
- 截图 30 张 sha256sum 本轮独立重算 30/30 与报告逐字一致，UNIQUE 30/30（ann-390=f1306525 / ann-768=64585f73 / ann-1280=8739b23e / bag-390=156778bb / bag-768=d6fd3af0 / bag-1280=742f27b6 / battles-390=081a1ccd / battles-768=f837f053 / battles-1280=9742c94c / mastery-390=8a5b233d / mastery-768=bd7a1e91 / mastery-1280=09a954b8 / rank-390=e0a79dae / rank-768=5fca71de / rank-1280=b0969c31 / shop-390=75a1cecb / shop-768=fe2b3bba / shop-1280=42399fba / signin-390=0e1ea9cb / signin-768=2c046dcf / signin-1280=61246485 / equip-390=6d050297 / equip-768=31a03c0a / equip-1280=a7491c5f / reactions-390=77b32bd9 / reactions-768=5087cda6 / reactions-1280=befe79bf / skills-390=39789bac / skills-768=bb7f478f / skills-1280=ce09dc26）。
- 亲读 2 张非登录页：r104-load-announcements-768.png（sha 64585f73，768×1024 / 116922B，公告加载中文字卡 + 三行骨架 + 公告与版本标题）与 r104-load-equipment-1280.png（sha a7491c5f，1280×800 / 89826B，装备图鉴紫框骨架屏 + AI 因果链空态），均非空态且与 DOM/AX 一致。
- 诚实边界已披露：后端 8080 未起、数据经 init-script 合成非真值；加载态经 Vue 实例 __r104Find/__r104Set 注入（bag 页源码 loading=false 且无 loading.value=true，必须注入才能观察加载态）；AX 完整快照仅 equip/ann 两页；亲读 2/30；改前基线引用 t280 未自建改前浏览器对照。

## 维度 4 · 语义正确性 —— PASS

- 静态写死 aria-busy=true 与 aria-busy=false 等价的依据：v-if 节点加载完成后从 DOM 卸载，卸载即天然消失，不存在 aria-busy 停留在 true 的误导状态（SPEC §2C，QA 卸载探针 busyCount=0 坐实）。
- B 类骨架 bar 为空 view（仅 style=width，无文字），去 aria-hidden 后读屏只读 status label，无噪音。
- 权威口径：MDN aria-busy + 同仓 admin/src/components/PageSkeleton.vue:10 `role=status aria-busy=true aria-label=数据加载中`、AiInsightCard.vue:19 `:aria-busy=loading?'true':'false'`，本轮 ops_grep 已确认。与 ROUND89_B（四态迁移布局无候选）属不同维度，不重复立轮。

## 维度 5 · 门禁独立复跑 —— PASS

- 自建只读门禁 docs/laiyipao/recon/.r104-review-gates.sh，后台 job bash-32f3644a2fc44aa0 已完成，输出 .r104-review-gates.out 末行 REVIEW_GATES_DONE。逐门结果：
- G1 r104 契约 13/13 RC0；G2 关联契约 r101/r60/r61 20/20 RC0；G3 页面定向 11 files passed RC0；G4 全量 236 files passed | 1 skipped (237)、2175 tests passed | 2 skipped RC0；G5 vue-tsc type-check RC0；G6 build:h5 RC0；G7 git diff --check RC0。
- QA 自报 gates-out.txt G1–G7 全 RC0，与本轮逐项一致。

## 维度 6 · 不变量与保护面 —— PASS

- 令牌三锚点在 tokens.css:14-16（--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6）；业务 pages 零 --r104-（契约断言文本命中除外）。
- 基线 HEAD 未变；git status 非 ?? 项仅 .gitignore + 10 个预期业务 vue（t283 写域）；git diff --check RC0。
- auth 五路径（.env / admin .env.local.keys / deploy .local.keys / server .local.keys / miniapp .env.local）经 git check-ignore 全 IGNORED；laiyipao 下 .pem/.key = 0；报告零真值。
- R60/R95/R98/R99/R100/R101/R102/R103 既有键盘与语义范式零回退；本轮只加加载态语义属性，不动 handler/令牌/既有 a11y 节点。

## 低风险观察（low，不阻断，供后续轮参考）

1. L1：QA 后端未起，公告/装备等数据经 init-script 合成非真值；语义属性验证不依赖数据真值，不影响结论。
2. L2：加载态经 Vue 实例注入观察（bag 页源码无 loading.value=true），非真实网络加载时序；静态写死等价性由 SPEC §2C + 卸载探针保证。
3. L3：AX 完整快照仅 equip/ann 两页，其余 8 页由 DOM 探针 + 契约静态保证，未逐一 AX 快照。
4. L4：截图亲读 2/30，其余 28 张以 sha256sum + DOM 探针坐实。
5. L5：battles 骨架浏览器加载态未单独坐实（归属 B 类外的 A 类，DOM 探针已覆盖），以契约静态保证。
6. L6：G7 git diff --check 输出含既有 LF→CRLF 行尾提示但 RC0，非本轮引入。
7. L7：改前基线引用 t280 侦察报告，未自建改前浏览器 A/B 对照；RED 原始证据已独立坐实缺口。

## 结论

六维证据链完整、门禁全绿、保护面零回退，无 blocker/high/medium。**verdict = pass**，同意 t283 实施随批次发布。
