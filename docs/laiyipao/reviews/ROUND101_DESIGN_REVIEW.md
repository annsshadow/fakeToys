# R101 语义可访问性补全 — 独立终审报告

- 轮次：R101（kind=review，reviewedTaskId=t264 实施）
- 终审人：same-route-reviewer
- 基线 HEAD：`187aeb0eeaba475f9e2eb4bf5e9b74175699973b`（全程未变，实施未提交）
- 审查链路：t261 ROUND101_A 侦察 → t262 ROUND101_B 侦察 → t263 规范（SPEC+契约+RED）→ t264 实施 GREEN → t265 QA PASS → 本轮终审
- 审查对象：3 处 `aria-expanded`（battles/index/enemies）+ me `nicknameMsg` role=status + r61 第 7 处 NODES 守卫

## 判定总览

**verdict = pass**。六维全绿，无 blocker / high / medium finding；7 条 low 观察如实列出不阻断。同意分批发布。

---

## 维度 1 — test-first 真实性：PASS

- SPEC `docs/laiyipao/specs/round101-semantic-a11y/SPEC.md` §0–§5 齐备：§0 目标、§1 四条件裁定（A 3 处 + B 1 处均成立）、§2 权威口径（WAI-ARIA 1.2 #aria-expanded / APG Alert-Status Pattern）、§3 改点表、§4 排除项、§5 写域。
- RED 权威原始证据 `RED-evidence.md`：逐字 `RED_EXIT=1`、`Tests 4 failed | 5 passed (9)`、v4.1.12、Duration 203ms。4 failed 全部为真实语义缺口（index `.ann-item` / enemies `.resist-toggle` / battles `.battle-row` 缺 `:aria-expanded`、me `nicknameMsg` 缺 `role="status"`），Received 原文坐实各开标签原样；5 passed = r61 NODES 守卫（规范轮已完成）+ 4 不变量（键盘三件套保留 / R61-R100 契约本体 / 令牌三锚点零 --r101- / 排除项不误改）。无 PARSE_ERROR。
- 契约 `r101SemanticA11y.contract.test.ts`（9 it）真 `readFileSync` 读 4 页 vue + r61/r100 契约源码，`tagBefore()` 以稳定锚点向前找 `<tag`、向后找 `>`，跨行开标签容错、不依赖行号；同源断言分别钉 `:class="{ open: expanded.has(a.id) }"` / `expandedEnemy === en.id` / `v-if="expandedId === it.id"`，防硬编码漂移。
- 我独立复跑契约 GREEN 9/9 RC0（门禁 G1）。
- L1：QA 报告 §1 写「RED 1 failed|5 passed」与权威 RED-evidence「4 failed|5 passed」不一致，属 QA 转述笔误，以原始逐字证据为准（见 findings L1）。

## 维度 2 — 实施外科性与写域：PASS

- git diff 精确 5 处业务/契约改点，每处一行：battles.vue:190 `:aria-expanded="String(expandedId === it.id)"`；enemies.vue:102 同开标签末追加 `:aria-expanded="String(expandedEnemy === en.id)"`（2+/1-）；index.vue:119 `:aria-expanded="String(expanded.has(a.id))"`；me.vue:103-104 `role="status"` + `aria-live="polite"`；r61FormA11y.contract.test.ts:71 NODES 增补第 7 项。另 .gitignore +5 行审计目录卫生（沿袭 round99/100 防真值入库）。
- **特别核验——同源无硬编码**：三处 aria-expanded 表达式与各自展开态响应式变量逐字同源（expanded.has(a.id) / expandedEnemy === en.id / expandedId === it.id），与契约同源断言一致，QA 三视口 before→open→close false→true→false 同步佐证，无写死 true/false。
- **特别核验——me role=status 同构**：nicknameMsg（me.vue:100-107）补的两属性与同页 passwordMsg（me.vue:605-608，`v-if="passwordMsg"` + `role="status"` + `aria-live="polite"`）逐字同构，R61 范式一致。
- **特别核验——r61 第 7 处纳入守卫**：`[pages/me/me.vue, {{ nicknameMsg }}, state, me nicknameMsg]`（r61:71）已入 NODES，G2 5/5 通过。
- 键盘三件套 `role="button"` / `tabindex="0"` / `@keydown.enter` 与原 `@click` 入口逐处保留（多行标签属性落首行满足 R60 扫描）；排除项（导航/选择态/dialog/link 详情/双入口）未被误改，契约 it9 静态守。

## 维度 3 — QA 可信度：PASS

- QA 报告 `docs/laiyipao/audits/round101-semantic-a11y/REPORT.md`（100 行）三视口 390×844 / 768×1024 / 1280×800 × 4 改点：aria-expanded 由改前计数 0 / AX expanded 0 变为计数 2(index)/1(enemies)/1(battles) / AX expanded=true，false→true→false 与展开态同步；me 错误文本由仅 StaticText 变为 `- status` live region。改前基线对照引用 t261/t262 侦察，不重造。
- 12 张截图 sha256 我独立重算 12/12 与报告逐字一致，UNIQUE 12/12。亲读 2 张非登录页：`r101-ann-768.png`（sha 54cf742b…，首页 QA 玩家游客态，公告「QA公告一」展开态⌃、显示完整正文）与 `r101-me-1280.png`（sha fb4b0f29…，指挥官档案页昵称编辑框内红色「昵称不能为空」可见），均与 AX 数据一致、非空态。
- 诚实边界充分：后端 8080 未起，页面数据经 init-script 注入合成（公告 2 条/敌兵 1 条/战报 1 条/profile 均合成非真值），占位令牌 `r101-qa-placeholder-token` 仅内存注入无真值入库；QA 库 DSN 未暴露；me 失败态走本地校验无网络依赖；键盘未重复 Tab/Enter 动态用例（R100 已坐实且未改 handler）；截图亲读 2/12 如实标注。

## 维度 4 — 门禁独立复跑：PASS

- 自建 `docs/laiyipao/recon/.r101-review-gates.sh`（复刻 QA gates101.sh 命令）后台 job 独立复跑，输出 `.r101-review-gates.out`，REVIEW_GATES_DONE，逐项 RC=0：
- G1 r101 契约 9/9；G2 r61 5/5；G3 R60 6/6；G4 R100 12/12；G5 定向 4 文件（index/battles/enemies_fresh_load/me）71/71；G6 全量 233 passed|1 skipped(234)、2141 passed|2 skipped(2143)；G7 `pnpm run type-check` RC0；G8 `pnpm run build:h5` DONE Build complete RC0；G9 `git diff --check` RC0。
- 全量输出含 `[Vue warn]: Failed to resolve directive: aria-native` 等既有告警（enemies_fresh_load/me 测试），非本轮引入，结论以 RC=0 与 passed 计数为准。

## 维度 5 — 保护面零回退：PASS

- 令牌三锚点 tokens.css:14/15/16（`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`）在；业务 vue 零 `--r101-`（NO_MATCH）。
- R95 me.vue:11 返回键、common.css:914/918 焦点环、R98 @confirm 七处、R99/R100 link 键盘范式均未回退；aria-expanded 三处仅追加属性，未触及既有 role/tabindex/handler。
- 写域仅 .gitignore + battles/enemies/index/me.vue + r61 契约，无业务外溢；HEAD 未变。

## 维度 6 — 诚实边界：PASS

- mock 合成数据非真值、占位口令零真值、QA 库未写、截图亲读 2/12、键盘未重复动态用例、编译结构（role=status 落外层 UNI-TEXT 非内层 SPAN，已由 probe-status + AX 双证）、既有 Vue 告警——均已如实披露，无隐瞒跳步。

---

## Findings（全部 low，不阻断发布）

- **L1（low）**：QA REPORT.md §1 转述「RED 1 failed|5 passed」与权威 `RED-evidence.md`「4 failed|5 passed (9)」不一致，系笔误；终审采信原始逐字证据。建议 QA 报告后续更正该行，不影响契约真实性。
- **L2（low）**：12 张截图亲读 2 张（ann-768 / me-1280），其余 10 张以 sha + 探针/AX 数据为准。
- **L3（low）**：真浏览器后端 8080 未起，页面数据为 init-script 合成，坐实的是「属性存在+AX 语义生效」的客户端因果，非服务端真实数据下的交互结果。
- **L4（low）**：键盘 Tab/Enter 动态用例本轮未重跑，三件套未回退由探针回读 role/tab + R60 契约静态保证（R100 已坐实动态）。
- **L5（low）**：me `role=status`/`aria-live` 落 Vue 编译后外层 UNI-TEXT，内层 SPAN 无语义属性；已由 probe-status + AX `- status` 双证，非缺陷。
- **L6（low）**：.gitignore +5 行审计目录卫生配置，属防真值入库的正当配套，非业务改动。
- **L7（low）**：定向/全量 vitest 含既有 `Failed to resolve directive: aria-native` 等告警，非本轮引入。

## 终审结论

R101 语义可访问性补全 test-first 链路真实、实施外科、QA 可信、门禁全绿、保护面零回退、诚实边界充分。4 处缺口均关闭，特别核验四项（同源无硬编码 / me role=status 同构 / r61 第 7 处纳守卫 / 令牌与零 --r101-）全部成立。**verdict = pass，同意分批发布。**
