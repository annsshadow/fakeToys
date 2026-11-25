# R36 真浏览器复查报告：Wiki 合成技能表字段对齐权威 SeedSkill + hash 安全 CTA

- 轮次：R36
- 任务：t3 真浏览器复查（qa-e2e-wb）
- 尝试：attempt_id=62232132-81e2-45b4-9c07-76edf295f279
- 日期：2026-11-25（Asia/Shanghai）
- 判定：**PASS**（本 QA 范围内的 7 项硬门禁全部达成）

## 1. 验证方法与权威口径

- 真 Chromium：`agent-browser` 三视口 390×844 / 768×1024 / 1280×800；全部浏览器命令以后台 job 启动并用 `job_output` 控时，未使用 `timeout.exe` 包装 `.cmd`。
- 页面：H5 admin Wiki 技能 tab，锚点 `#/wiki?tab=skills&anchor=composite`，探针实测 `#composite` 可见、nav 命中「技能图鉴」。
- 权威：公开 `GET /api/v1/config` 的 `composite_skills`；快照 `data/config.json`，sha256=`333397d30acb2b81d77de95f52b1e72db9662fa2f0739f0275a309174584f5a0`，`version=6237e0b623219b8b`，共 18 条；未使用 SQL/DB 直连作为字段权威来源。
- 对账脚本：`reconcile.py` 以公开 config 生成 13 列期望值，与浏览器探针逐行逐字段比较；元素列按前端展示字典映射（fire→焰、ice→冰、lightning→电、kinetic→动能、corrosion→毒），映射仅解决展示中文名，九字段本身严格按 config 期望。
- 环境：API 8080（QA Postgres 15440，`SETTLE_RECOMPUTE_MODE=shadow`）与 admin Vite 5175 为测试环境；临时令牌 init-script 写在忽略目录 `data/admin-auth-init.js`，收尾已删。

## 2. 硬门禁逐项结果

| # | 门禁 | 判定 | 实测证据 |
|---|------|------|----------|
| ① | 三视口合成技能 18 条 × 九字段对账 | PASS | 390/768/1280 均 `ROWS=18 MISMATCHES=0`；`ALL_RECONCILE_OK=True`；种子行 ID31 灼热弹幕=主动/1.9秒/260/46/—/范围190/连锁1/3/30级解锁，ID38 雷暴链=主动/3.2秒/400/68/—/—/连锁6/4/62级解锁，ID48 光棱攻城=主动/3.4秒/520/72/穿透3/范围300/连锁5/4/70级解锁，与公开 config 完全一致 |
| ② | AI 四要素同屏 | PASS | 390/768/1280 均 `AI_OK=True FOUR_OK=True BAD=False`；结论「共 18 个合成技能：主动 18 个，多目标 18 个」；依据逐字「依据：公开 /api/v1/config 的 composite_skills（与服务端 SeedSkill / SeedCompositeSkills 同源）；格式化仅展示，不改结算」；红线逐字「辅助展示非权威战斗数据，展示为配置基值，实际效果以引擎结算为准；权威回放以 replay_hash 为准」；CTA「查看合成技能 →」；无 NaN/undefined/[object Object] |
| ③ | hash 安全 CTA 真实 CLI 点击 | PASS | `agent-browser --session lyp-r36-cta click '.r36-ai-cta a'` 输出 `✓ Done`、RC=0；点击前 hash `#/wiki?tab=skills`，点击后 hash `#/wiki?tab=skills&anchor=composite`，`dashboard=false`，合成段仍在、`rowCount=18`；截图为合成技能段 |
| ④ | 表内横向滚动与页面无溢出 | PASS | 三视口 `documentElement.scrollWidth=innerWidth`（390/390、768/768、1280/1280），页面无横向溢出；精确探针确认 Element Plus 内部 `.el-scrollbar__wrap` 可真实横滚：sw=1213/cw=324→scrollLeft 0→889（390），sw=1213/cw=702→0→511（768），sw=1213/cw=884→0→329（1280）；外层 `.r36-composite-table-scroll` 与 `.el-table` overflowX=auto/max-width:100%；13 列（含末列「解锁等级」）表头可滚动到，种子行 30/62/70 级解锁可解析 |
| ⑤ | R31–R35 回归 | PASS | 五 tab 浏览器探针均 `reactionCount=7, corrosionSpread=true, armorShred=true, armorKnock=true, skillCount=24, enemyCount=22, levelCount=20, gems=8, bad=false`；与既有 R35 报告所列回归值一致 |
| ⑥ | 截图、脚本真值、QA 库还原 | PASS | 见 §3 截图清单（≥4 张新图 sha256 互异且非登录页）；`zero-secret-scan.sh`: `BAD_COUNT=0`, `SYNTAX_RC=0`；`.gitignore:410/:411` 命中审计 data/ 与 auth-init；QA 库 `battle_records count=104, max(id)=477`；未改业务源码、未 git 操作 |
| ⑦ | 令牌锚点无 R36 漂移 | PASS | `laiyipao/admin/src/styles/tokens.css:12-14` 锚点 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`；全 admin src `grep -- "--r36-"` 无匹配（`NO_R36_TOKEN`） |

## 3. 截图证据（sha256）

全部截图位于 `screenshots/`，均已人工查看，内容为合成技能段或回归 tab，不是登录页。

| 截图 | 视口 | sha256 | 用途 |
|------|------|--------|------|
| `screenshots/r36-390-composite.png` | 390×844 | `9058c5e33a7bf21bd8aa3963adbd77d450679179aee02fe1031a07e113cd8fab` | 合成技能段 ID31–46 |
| `screenshots/r36-fixed-768-composite.png` | 768×1024 | `16ca02fbfad4dabea6243fd331df52f9c99ae1614563bd9506f56d63667baf31` | 合成技能段全 18 行 + AI 结论 |
| `screenshots/r36-fixed-1280-composite.png` | 1280×800 | `73e5c450649121b54b4d27edfe72ed2e63d979b1fcf838c18a05647d4949e4c0` | 桌面合成表前 8 列 + 目录 |
| `screenshots/r36-04-cta-before.png` | 390×844 | `0d79cc7ca0ed1d76b06575df9f0ad88c98f3ba6226da00167b9f35e210f41253` | CTA 点击前合成表 |
| `screenshots/r36-05-cta-after.png` | 390×844 | `7487324299b7ce4f45e779f28622e0783c4341f13686db5757b94f1d3d2decfa` | CLI 点击后合成表与 hash |
| `screenshots/r36-reg-basics.png` | 390×844 | `0db7d33d45814700f87eb84020a3d6b4ea1155262aa6fc3a3e2416583f2c2aa2` | R31 反应 tab |
| `screenshots/r36-reg-skills.png` | 390×844 | `889f28fc9041f987d49e95445883d2b99a1174a364438bab79d923fbd821e7dd` | R32 技能 tab |
| `screenshots/r36-reg-enemies.png` | 390×844 | `e901c522564d3168d920ac3d935add7d4bde8be363db9fa1479f84182a5b11c7` | R33 敌人 tab |
| `screenshots/r36-reg-levels.png` | 390×844 | `b8f706af0a4cafcc81955e6e1382e9a6e30aa40280c8f83a51466e5f0cab3b7a` | R34 关卡 tab |
| `screenshots/r36-reg-gems.png` | 390×844 | `2500550050f930e3f8254a5cc7548d779cf766e98493cd5c2d2b3242a24bef60` | R35 宝石 tab |

## 4. 测试与命令

- 三视口真浏览器对账：`bash e2e-viewports.sh` → 三份 `data/probe-*.json`；`python reconcile.py 390 768 1280` → `ALL_RECONCILE_OK=True`（摘要 `data/reconcile-summary.txt`）。
- 表内横滚精确复测：`bash scroll-detail.sh` → `data/scroll-detail-*.json`，Element Plus `.el-scrollbar__wrap` 在三视口均可滚动至末列。
- 真实 CTA 点击：`bash cta-click.sh` → `data/cta-click.txt` 输出 `✓ Done`，`data/cta-pre.json`/`data/cta-post.json` 记录 hash 变化与 dashboard=false。
- 回归五 tab：`bash reg.sh` → `data/reg-*.json` 五份，字段数值一致。
- 安全扫描：`bash zero-secret-scan.sh` → `BAD_COUNT=0`, `SYNTAX_RC=0`，`.gitignore:410/:411` 生效。
- 令牌守卫：`grep` 三锚点值及 `NO_R36_TOKEN`。
- 前端测试：`pnpm test -- --run` 全量一次 834 passed / 1 failed（`tests/bootstrap.test.ts` 无令牌登录页用例 20s 超时，疑似并发环境影响），单独重跑 `pnpm exec vitest run tests/bootstrap.test.ts --testTimeout=60000` 2/2 passed，属环境性 flake 而非 R36 功能回归；仍有 stderr `document is not defined` 的 teardown 噪声（既有 skipLink MutationObserver 问题）。
- 真值守卫：`go test -run TestNoRealSecretsInRepo -v ./...` → **SKIP**，`secrets_leak_test.go:182` 明示读不到 `laiyipao/deploy/.local.keys`，该守卫未执行，**不是 PASS**。

## 5. 诚实边界

- 本 QA 只复查 Wiki 合成技能表展示与导航，不验证战斗结算；表格是配置基值，实际效果以引擎结算为准。
- `TestNoRealSecretsInRepo` 因缺 `.local.keys` 为 SKIP，不声称真值扫描通过；脚本真值扫描只覆盖本审计脚本自身，结论 `BAD_COUNT=0`。
- 对账权威仅公开 `/api/v1/config`；数据库只用于 QA 库还原基线确认（battle_records 104/477），不作为字段真值来源。
- 前端测试全量一次的 bootstrap 超时已单独复跑通过，报告同时保留失败事实，不掩盖。
- 临时 auth-init 与 config 快照均在 gitignore 审计目录，收尾删除 `data/admin-auth-init.js`；未改业务源码、未执行 git 写操作。

## 6. 收尾状态

- 临时令牌 `data/admin-auth-init.js` 已删除（收尾复核 `AFTER=DELETED`）；QA 库复核 `battle_records count=104, max(id)=477`。
- 所有 QA 后台 job 已发 `job_kill` 清理；浏览器会话已停止，未发现本次启动的遗留 agent-browser/Chrome 进程（系统自带 WebView/Tabbit 进程非本 QA 启动）。
- 报告保留的截图哈希与 `sha256sum screenshots/*.png` 实测一致。
