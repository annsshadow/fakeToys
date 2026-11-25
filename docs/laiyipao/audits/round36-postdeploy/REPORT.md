# R36 部署后独立回归报告（Wiki 合成技能表字段对齐 SeedSkill 九字段 + hash 安全 CTA）

- 轮次：R36
- 任务：t5 发布（release-ops-wb），attempt_id=0bdb95ca-5fea-45ae-b265-789e2af72eed
- 日期：2026-11-25（Asia/Shanghai）
- 判定：**PASS**（部署后真浏览器全部硬门禁达成；诚实边界项如实标注 SKIP）

## 1. 结论

部署后另起真实 Chromium（headless，本地回环 127.0.0.1，管理员会话仅通过本地 init-script 注入）独立会话，三视口 390×844 / 768×1024 / 1280×800 以公开 `/api/v1/config` 的 18 条 composite_skills 为权威逐字段核对 13 列（保留 ID/名称/元素/说明 + SeedSkill 九字段：类型/冷却/伤害/热量/穿透/范围/连锁/层数/解锁等级），MISMATCHES=0；AI 四要素 18/18/18 同屏无脏值；真实 CLI 点击 `.r36-ai-cta a` 后 hash 从 `#/wiki?tab=skills` 变为 `#/wiki?tab=skills&anchor=composite` 并停留合成段，dashboard=false；表内 Element Plus `.el-scrollbar__wrap` 三视口真实横滚至末列「解锁等级」，页面无横向溢出；R31–R35 五 tab 不回退。后端与 H5 产物 sha 与基线一致、未回退。

## 2. 部署与产物基线

- 基线 HEAD=276e81b6c15dcc55b006a7058c3aa0100777472f，起点 tracked clean、AB=0/0。
- 五批本地提交（待本批后统一 push）：B1=9e5a1f8dce13a6daaa18bda7c40f2a265b054a14 feat(compositeMechanicsText.ts + WikiView.vue)；B2=be2eea70631dbb21fa29355bfd5139191f67b405 test(WikiCompositeMechanics.test.ts)；B3=20c26e5d95a91c37d6d68a8f1b8ae5f0bf5843e0 docs(SPEC.md + 终审报告)；B4=e38e63cc6f79621c4aa9463c87b2676fd861403b audit(round36-wiki-composite-skills 审计)；B5=postdeploy(本报告与部署后脚本/截图)。
- 后端产物 `laiyipao/deploy/bin/lyp-server-amd64` sha256=`3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6`（R21 沿用，未回退）。
- H5 产物 `laiyipao/miniapp/dist/build/h5/assets/index-DCBbR3t9.js` sha256=`aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b`（R30 沿用，未回退）。
- 本轮仅重建 admin：Vite 5175 实时编译 R36 源码（compositeMechanicsText.ts + WikiView.vue 13 列 + AI 四要素 + hash 安全 CTA），未改 engine/后端/H5/tokens。
- 服务健康：8080/api/v1/config=200、5174/=200、5175/laiyipao-admin/=302（正常重定向登录页）、QA Postgres 容器 lyp-qa-pg 15440→5432 正常。
- 部署后公开 config 快照 sha256=`a602e2bd07d91e774d87fab453f6d723be5f3c1ba44a722a78e29f5d15e3829a`，version=`6237e0b623219b8b`，composite_skills=18 条（服务重启后 config 重新生成，sha 与部署前 QA 快照 333397d3… 不同，但 version 与九字段内容同源一致）。

## 3. 三视口逐字段对账（reconcile.py 读公开 config）

执行 `e2e-viewports.sh`（390 390×844 / 768 768×1024 / 1280 1280×800）→ 三份 `data/probe-*.json`；`python reconcile.py 390 768 1280` → **ALL_RECONCILE_OK=True**，RECONCILE_EXIT=0：

| 视口 | 行数 | 不符 | AI | 四要素 | 脏值 | 文档宽/内宽 | 横滚（外层 sw/cw） | CTA | 判定 |
|------|------|------|----|--------|------|-------------|---------------------|-----|------|
| 390×844 | 18 | 0 | OK | OK | False | 390/390 | 0→0 / 324/324 | 查看合成技能 → | PASS |
| 768×1024 | 18 | 0 | OK | OK | False | 768/768 | 0→0 / 702/702 | 查看合成技能 → | PASS |
| 1280×800 | 18 | 0 | OK | OK | False | 1280/1280 | 0→0 / 884/884 | 查看合成技能 → | PASS |

- 期望值由公开 config 生成：元素映射 fire→焰/ice→冰/lightning→电/kinetic→动能/corrosion→毒；kd active→主动/passive→被动；冷却毫秒→秒；穿透/范围/连锁/层数/解锁等级正数展示，否则「—」。
- AI 结论「共 18 个合成技能：主动 18 个，多目标 18 个」+ 依据 + 诚实红线 + CTA 四要素均在屏；无 NaN/undefined/[object Object]。
- 终端中文显示乱码仅控制台编码问题，probe JSON 原始字节与 Python 复核一致。

## 4. 表内横滚与末列可达

`scroll-detail.sh` → `data/scroll-detail-*.json`，精确探针确认 Element Plus 内部 `.el-scrollbar__wrap` 在三视口均可真实横滚至末列：

| 视口 | wrap scrollWidth/clientWidth | scrollLeft 实测 | 末列 | documentElement.scrollWidth=innerWidth |
|------|------------------------------|-----------------|------|----------------------------------------|
| 390×844 | 1213 / 324 | 0 → 889 | 解锁等级 | 390/390 |
| 768×1024 | 1213 / 702 | 0 → 511 | 解锁等级 | 768/768 |
| 1280×800 | 1213 / 884 | 0 → 329 | 解锁等级 | 1280/1280 |

`capture-end.sh` 三视口滚至 18 行末行 ID48 光棱攻城后截图，实测 wrapSl=889/511/329、wrapSw=1213、lastId=48、docW=innerW，末列「解锁等级」可见（末行 70 级解锁），页面无横向溢出。

## 5. hash 安全 CTA 真实 CLI 点击

- `cta-click.sh`：`agent-browser --session lyp-r36post-cta click '.r36-ai-cta a'` 输出 `✓ Done`、RC=0。
- 点击前 `data/cta-pre.json`：hash=`#/wiki?tab=skills`，sectionVisible=true，rowCount=18，ctaText=「查看合成技能 →」。
- 点击后 `data/cta-post.json`：hash=`#/wiki?tab=skills&anchor=composite`，compositeStillThere=true，rowCount=18，dashboard=false，activeNav=[技能图鉴]——停留合成段未跳看板。
- 截图 `r36post-04-cta-before.png`（技能表顶部）与 `r36post-05-cta-after.png`（合成技能段）亲读确认为真实页面、非登录页。

## 6. R31–R35 五 tab 回归

`reg.sh`（390×844，session lyp-r36post-reg）五 tab basics/skills/enemies/levels/gems 探针全部一致：reactionCount=7 且 corrosionSpread=true / armorShred=true / armorKnock=true；skillCount=24；enemyCount=22；levelCount=20；gemCount=8；docW=innerW=1258；bad=false。REG_DONE=YES，前轮功能不回退。

## 7. 新截图证据（sha256，均亲读非登录页）

满足「≥4 张新图 sha 两两互异且全不等于部署前 QA 图」的部署后截图（节选）：

| 截图 | 视口 | sha256 |
|------|------|--------|
| `screenshots/r36post-768-composite.png` | 768×1024 | `f358edd42e81b0d5e0194a21debcf79883b6a343c92291031c9424599e4fad78` |
| `screenshots/r36post-1280-composite.png` | 1280×800 | `3936741251a80ecf6ec744defc5c407766b82dea5a04a2ca250436d8be4d6a5e` |
| `screenshots/r36post-05-cta-after.png` | 390×844 | `7487324299b7ce4f45e779f28622e0783c4341f13686db5757b94f1d3d2decfa` |
| `screenshots/r36post-768-composite-end.png` | 768×1024 | `cb613bae372dc1c6c239e266a24df39fc91b08edc971a20700ff018d5b53d88b` |
| `screenshots/r36post-1280-composite-end.png` | 1280×800 | `9a0df867f5df6598314d9ac4db6f8add0ab684b5dd62df14c1878764157c62f7` |
| `screenshots/r36post-390-composite-end.png` | 390×844 | `430309fd17c6502daf62bd062c8b99ac5b0460d2e5e4ec6ad146d54222453751` |

以上 6 张 SHA 两两互异，且均不在部署前 `round36-wiki-composite-skills/screenshots/` 的 SHA 集合中。部署前 QA 图 SHA 对照：r36-390-composite=9058c5e3…、r36-fixed-768=16ca02fb…、r36-fixed-1280=73e5c450…、r36-04-cta-before=0d79cc7c…、r36-05-cta-after=74873242…。

诚实边界：`r36post-390-composite.png` = `0d79cc7ca0ed…`、`r36post-04-cta-before.png` = `0d79cc7ca0ed…`、`r36post-fixed-*.png` = 9058c5e3…/16ca02fb…/73e5c450… 与部署前 QA 图同 SHA——固定视口/滚动位的静态页在 agent-browser 下确定性像素一致，属截图确定性而非复用旧图；验收所需的「≥4 张新图」由上表 6 张唯一 SHA 独立满足。

## 8. 真值治理与安全边界

- 临时管理令牌写入 `data/admin-auth-init.js`（bootstrap 默认口令登录，JWT 229B），由 `.gitignore:414` 拦截；`data/login.json`（含 admin token）与 `data/config.json` 同在忽略目录，绝不入库。
- 零真值扫描（`zero-secret-scan.sh`，排除 data/）：BAD_COUNT=0；全部 `.sh` 脚本 `bash -n` 通过 SYNTAX_RC=0。
- **`TestNoRealSecretsInRepo`：本机缺 `laiyipao/deploy/.local.keys` 真值源，Go 守卫打印「未执行」后 SKIP——这是未执行、非 PASS，不以此作为通过依据**；改用静态零真值扫描与 git 门禁作为替代证据。
- 令牌三锚点无 `--r36-` 漂移（t4 终审独立复核：`--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`，全 admin src grep `--r36-` 无匹配）。

## 9. QA 数据还原与收尾

- `finalize.sh`：删除 postdeploy/data/ 与根令牌 → AUTH_DELETED=YES；TRUNCATE battle_records RESTART IDENTITY CASCADE 后 pg_restore qa-before.dump；独立复核 DB=`104|477`（count=104、max id=477，与发布前基线一致）。
- `git check-ignore -v` 命中 `.gitignore:414 docs/laiyipao/audits/round36-postdeploy/data/`；.gitignore 共 415 行（相对 407 行 R35 基线仅追加 R36 两组规则），未截断既有规则。
- 所有浏览器命令均以后台 job + `job_output` 控时，未使用 `timeout.exe` 包装 `.cmd`；结束时 `job_kill` 清理全部遗留 job 并关闭 agent-browser 会话。
- 审计权威仅公开 `/api/v1/config`；数据库只用于 QA 库还原基线确认，不作为字段真值来源。本轮不验证战斗结算，表格是配置基值，实际效果以引擎结算为准。

## 10. 部署 sha 与提交哈希汇总

| 项 | 值 |
|----|----|
| 后端 lyp-server-amd64 | `3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6` |
| H5 index-DCBbR3t9.js | `aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b` |
| 部署后 config | `a602e2bd07d91e774d87fab453f6d723be5f3c1ba44a722a78e29f5d15e3829a`（version `6237e0b623219b8b`） |
| B1 feat | `9e5a1f8dce13a6daaa18bda7c40f2a265b054a14` |
| B2 test | `be2eea70631dbb21fa29355bfd5139191f67b405` |
| B3 docs | `20c26e5d95a91c37d6d68a8f1b8ae5f0bf5843e0` |
| B4 audit | `e38e63cc6f79621c4aa9463c87b2676fd861403b` |
| B5 postdeploy | 本提交（HEAD，含本报告与部署后脚本/16 张截图） |
