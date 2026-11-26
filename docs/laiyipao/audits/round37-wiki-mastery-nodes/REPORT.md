# R37 真浏览器复查报告：Wiki 专精树节点字段、诚实口径与 hash 安全 CTA

- 轮次：R37
- 任务：t8 真浏览器复查（qa-e2e-wb）
- 尝试：attempt_id=5769983e-6fb9-4ea3-8c9f-14d119731fea
- 日期：2026-11-25（Asia/Shanghai）
- 判定：**PASS**（本 QA 范围内的 8 项硬门禁全部达成；失败/跳过项如实登记见 §5–§6）

## 1. 验证方法与权威口径

- 真 Chromium：`agent-browser` 三视口 390×844 / 768×1024 / 1280×800；全部浏览器命令以后台 job 启动并用 `job_output` 控时，未用 `timeout.exe` 包装 `.cmd`。
- 页面：admin Wiki 专精 tab `#/wiki?tab=mastery`，`.r37-mastery-mechanics` 可见，nav 命中「专精树」。
- 权威：公开 `GET /api/v1/config` 的 `mastery_families`（未用 SQL/DB 直连作字段权威）；快照 `data/config.json`，sha256=`ff87a8db361394de7dcf588c8ea6ea4c03cb17061fcc6b6bc01112f677ea7a69`，`version=6237e0b623219b8b`。
- 对账：`reconcile.py` 从公开 config 按 SPEC 口径生成 8 系×12 节点、七列（系/层/槽/节点/效果/数值/前置）期望值，与三视口浏览器 DOM 探针逐行逐字段比较；摘要 `data/reconcile-summary.txt` 为 1498 字节、sha256=`7d83204ecd6fd592e60350961ec65745492bee901e580f40e2d9c454cb025b0e`，`ALL_RECONCILE_OK=True`。
- 环境：API 8080（QA Postgres 15440，`SETTLE_RECOMPUTE_MODE=shadow`）与 admin Vite 5175 为测试环境；临时令牌 init-script 写在 gitignore 的 `data/admin-auth-init.js`，收尾已删除。

## 2. 硬门禁逐项结果

| # | 门禁 | 判定 | 实测证据 |
|---|------|------|----------|
| ① | 三视口 8 系×12 节点=96 行 × 七列对账 | PASS | 390/768/1280 均 `ROWS=96 MISMATCHES=0`，`ALL_RECONCILE_OK=True`；`families` 实测焰/冰/电/物理/光/毒/无人机/工程各 12 行；七列表头逐字「系/层/槽/节点/效果/数值/前置」 |
| ② | 效果口径与前置列逐字 | PASS | 热上限 `+10‰/+20‰/+30‰`（heat_cap 配置值 10/20/30 直接显示，无二次乘 10、无百分比）；反应倍率/技能伤害/暴击/护甲/机制改造 `+N‰`；元素上限 `+10层/+20层`；额外插槽 `+1 槽`；种子样本：焰|1|2|热量上限|热量上限|+10‰|无前置，焰|2|1|反应倍率|反应倍率|+20‰|—，焰|3|0|额外插槽|额外插槽|+1 槽|—。**前置列诚实说明**：公开 config 全部节点 `prereq_family` 为空串、`prereq_layer=layer-1`，按 SPEC `prereqText` 严格口径第 1 层显示「无前置」、第 2/3 层显示「—」（实测分布 32+64=96）；因此页面没有任何「前置：{族} 第{N}层」文案，这不是实施缺陷，也绝非逐字展示了该文案。AI 结论「含前置 64 个」来自把非「无前置」（含「—」）计入的聚合口径，与页面前置列分布一致。若要求第 2/3 层显示完整族名前置，需改规范/种子配置，本 QA 不改源码。 |
| ③ | AI 四要素同屏 | PASS | 三视口 `AI_OK=True FOUR_OK=True BAD=False`；结论「共 8 系：96 个节点，含前置 64 个，额外插槽 8 个」；依据逐字「依据：公开 /api/v1/config 的 mastery_families（与服务端 MasteryFamily / MasteryNode 同源）；效果按 power.go 口径展示，实际以引擎结算为准」；红线逐字「辅助展示配置基值，非权威战斗数据；实际效果以引擎结算为准，权威回放以 replay_hash 为准」；CTA「查看专精树 →」；无 NaN/undefined/[object Object] |
| ④ | CTA 真实 CLI 点击 | PASS | `agent-browser --session lyp-r37-cta click ".r37-ai-cta a"` 输出 `✓ Done`、RC=0（`data/cta-click.txt`）；点击前 hash `#/wiki?tab=mastery`、rowCount=96，点击后 hash 仍 `#/wiki?tab=mastery`，`masteryStillThere=true`、`dashboard=false`、`activeNav=["专精树"]`；CTA href `#/wiki?tab=mastery`，可复制新标签打开；截图为专精树段 |
| ⑤ | 表内横滚与页面无溢出 | PASS | 三视口 `documentElement.scrollWidth=innerWidth`（390/390、768/768、1280/1280）；真实横滚发生在 Element Plus 内部 `.el-scrollbar__wrap`：390 sw=740/cw=294→scrollLeft 0→446，768 sw=740/cw=672→0→68，均 8 个 family-block 可滚到末列「前置」；1280 sw=854=cw=854，`lastInView=true`、moved=[]，末列已直接可见，故无滚动需求而非缺陷；已按队长复核修正判据为「sw<=cw 或 after>before」并重算三视口 `SCROLL=True`。外层 `.r37-mastery-table-scroll` overflowX=auto；末列可见截图 `r37-06-lastcol-1280.png` |
| ⑥ | R31–R36 六 tab 不回退 | PASS | 改用 CSS 选择器 `.nav-item[href=...]`/`.nav-sub-item[href=...]` 的 `reg3.sh` 真实 CLI click 七次均 `✓ Done`（`data/reg3-click-*.txt`），每次切换后抓 `data/reg3-*.json`：basics=#/wiki、activeNav=核心规则、`reactionCount=7`、`corrosionSpread=true`、`armorShred=true`、`armorKnock=true`、basicsTableCounts=[7,5]；skills/composite/enemies/levels/gems/mastery hash 依次正确，visibleSections=1，compositeCount=18、masteryCount=96，全部 `bad=false`、`docW=innerW=1258`。**探针边界**：section 用 `v-show` 全量在 DOM，`skillsCount` 选择器对技能表返回 0，该字段不作行数证据；R32 24 行、R33 22 行、R34 20 行、R35 8 行沿用原 `reg-*.json` 全 DOM 计数与 R36 既有回归基线，仅作无回退旁证，不作为「当前 tab 可见行数」的独立证据；当前 tab 可见性与反应链传播以 reg3 为准。 |
| ⑦ | 截图、脚本真值、QA 库还原 | PASS | 见 §3 截图清单（≥4 张新图 sha256 两两互异、全非登录页）；`zero-secret-scan.sh` 输出 `BAD_COUNT=0`、`SYNTAX_RC=0`，脚本自身无本机绝对路径/真值（`NO_ABS_OR_TRUTH`）；`.gitignore:418/:419` 命中审计 data/ 与 auth-init；QA 库 `battle_records count=104, max(id)=477`；未改业务源码、未执行 git 写操作 |
| ⑧ | 令牌锚点无 R37 漂移 | PASS | `laiyipao/admin/src/styles/tokens.css:12-14` 锚点 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`；全 admin src `grep -- "--r37-"` 无匹配（`NO_R37_TOKEN`） |

## 3. 截图证据（sha256）

全部截图位于 `screenshots/`，已人工查看，内容为专精树段或回归 tab，不是登录页；以下为本报告引用的最终有效截图。

| 截图 | 视口 | sha256 | 用途 |
|------|------|--------|------|
| `screenshots/r37-390-mastery.png` | 390×844 | `026b5ca3a5f9dd3b10bdfc9c5533cab56c330deb4c90aede2442b0dac19e7d4e` | 专精树首屏 7 列前 4 列 |
| `screenshots/r37-768-mastery.png` | 768×1024 | `e1d9e4109969af271c3052ce02e63e21f78cf1e57ac715750ed1d90f7e843490` | 专精树 8 系与末列前置可见 |
| `screenshots/r37-1280-mastery.png` | 1280×800 | `2442413fbe495a34a2cd6fd9b8ff175603a0c3c5842ea89bfffc7692c424eee0` | 桌面专精树含末列前置 |
| `screenshots/r37-04-cta-before.png` | 390×844 | `026b5ca3a5f9dd3b10bdfc9c5533cab56c330deb4c90aede2442b0dac19e7d4e` | CTA 点击前 |
| `screenshots/r37-05-cta-after.png` | 390×844 | `41d330071c789e96d4dcf6969c61f3bcccae61f1ee25f53548b18248a218bdd1` | CLI 点击后 hash 未跳 dashboard |
| `screenshots/r37-reg3-basics.png` | 1258×588 | `daa0cafad3096636ecc7a3b5fe7f49a8fd9454534d4066fd3ce3a94df52075b8` | CLI 点击后核心规则反应链 tab |
| `screenshots/r37-reg3-composite.png` | 1258×588 | `a262a6edcc084632cd73aa4f272f96321ad1b6b7002fd1b5a641ae6fa5dcb35f` | CLI 点击后合成技能段 ID31–40 |
| `screenshots/r37-reg3-mastery.png` | 1258×588 | `a8c346d4ea2fdcad1e663d45366b59063ba2622aa9aeb5b2e18d0a9a4875df4c` | CLI 点击后专精树段 |

注：`r37-06-lastcol-1280.png` 与 `r37-1280-mastery.png` 为同字节（sha 同为 `2442413f…`），`r37-390-mastery.png` 与 `r37-04-cta-before.png` 同字节（sha 同为 `026b5ca3…`）；两者均保留作不同用途证据，不冒充互异新图。上表互异 sha 至少 6 个，满足 ≥4 张新图两两互异。

## 4. 测试与命令

- 三视口真浏览器对账：`bash e2e-viewports.sh` → `data/probe-390/768/1280.json`；`python reconcile.py 390 768 1280` → `ALL_RECONCILE_OK=True`（`data/reconcile-summary.txt`）。
- 表内横滚精确复测：`bash scroll-detail.sh` → `data/scroll-detail-390/768/1280.json`；队长复核后修正判据（sw≤cw 也视为末列可见），三视口 `SCROLL=True`。
- CTA 真 CLI 点击：`bash cta-click.sh` → `data/cta-click.txt` 输出 `✓ Done`，`data/cta-pre.json`/`data/cta-post.json` 记录 hash 与 dashboard=false。
- 七 tab CLI 回归：`bash reg3.sh` → `.nav-item[href=...]` CSS 选择器七次 click 全部 `✓ Done`，`data/reg3-*.json` 与截图为切换后实时抓取；早期 `text=` 选择器脚本 `reg2.sh` 全部 ✗ Element not found，已废弃并 kill 其 job，不作为证据。
- 1280 末列截图：`bash lastcol-1280.sh` → `screenshots/r37-06-lastcol-1280.png` 与 `data/lastcol-1280.json`（8 个 family-block 的 `.el-scrollbar__wrap` sw=854=cw=854、末列表头「前置」lastInView=true）。
- 契约单测：`pnpm exec vitest run tests/WikiMasteryMechanics.test.ts --testTimeout=60000` → 11/11 passed（exit 0）。
- 安全扫描：`bash zero-secret-scan.sh` → `BAD_COUNT=0`、`SYNTAX_RC=0`，`.gitignore:418/:419` 生效；脚本自身及业务源码 `grep` 无本机绝对路径/令牌真值。
- 令牌守卫：`grep` 三锚点值及 `NO_R37_TOKEN`。
- 真值守卫：`go test -run TestNoRealSecretsInRepo -v ./...` → **SKIP**（`secrets_leak_test.go:145` 读不到 `laiyipao/deploy/.local.keys`，工作树确实无该文件），该守卫未执行，**不是 PASS**，见 §5。

## 5. 诚实边界与失败/跳过项

- **TestNoRealSecretsInRepo = SKIP，非 PASS**：缺 `.local.keys`，未执行真值子串比对，不声称真值扫描通过；脚本真值扫描只覆盖本审计脚本自身，结论 `BAD_COUNT=0`，两者不能互相替代。
- **AI「含前置 64 个」与页面前置列不一致的口径**：公开 config 未提供 `prereq_family`（全部空串），故无「前置：{族} 第{N}层」逐字展示；页面显示「无前置」32 +「—」64 是 `prereqText` 严格契约的诚实结果。AI 以非「无前置」计入 64，与页面「—」数量一致。本 QA 如实登记，不谎称逐字展示完整前置文案；是否调整规范/种子由队长裁决。
- **R32–R35 行数证据边界**：`reg3-probe.js` 的 `skillsCount` 选择器返回 0、全 DOM 计数受 `v-show` 影响，故六 tab 回归以真实 CLI click 后的 hash/activeNav/可见 section/反应链传播为准；24/22/20/8 行数沿用旧基线旁证，不作为「当前 tab 可见行数」的独立强证据。
- 早期 `text=` 选择器回归脚本 `reg2.sh` 全部失败（✗ Element not found），已废弃并清理；对应截图/JSON 不纳入本报告证据集。
- 本 QA 只复查 Wiki 专精树展示与导航，不验证战斗结算；表格为配置基值，实际效果以引擎结算为准。
- 权威仅公开 `/api/v1/config`；数据库仅用于 QA 库还原基线确认（104/477），不作字段真值来源。
- 临时 auth-init 在 gitignore 审计目录，收尾已删除；未改业务源码、未执行 git 写操作。

## 6. 收尾状态

- 临时令牌 `data/admin-auth-init.js` 已删除（收尾复核 `AFTER=DELETED`）；QA 库复核 `battle_records count=104, max(id)=477`。
- 所有 QA 后台 job 已 `job_kill` 清理；浏览器会话已停止，未发现本次启动的遗留 agent-browser/Chrome 进程。
- 报告所列截图 sha256 与当前 `sha256sum screenshots/*.png` 对引用文件一致；早期失败探针与重复截图已从证据目录剔除。