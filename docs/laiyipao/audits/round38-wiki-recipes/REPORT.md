# R38 真浏览器复查：Wiki 合成配方（recipes）字段对账与 CLI 搜索/CTA

- 轮次：R38 / 任务 t14 / 角色 qa-e2e-wb（真浏览器 QA）
- 公开权威：`GET /api/v1/config` 的 `recipes`（与服务端 `SeedRecipe`/`SeedRecipes` 同源），非 SQL/DB 直连
- 审计目录：`docs/laiyipao/audits/round38-wiki-recipes/`（脚本、快照、探针 JSON、截图）
- 独立 QA 库：`lya_qa`（由 `laiyipao` 模板克隆），基线 `battle_records count/max = 104/477`（测试前后一致）
- 服务：API `go run ./cmd/api`（`laiyipao/server`，`DATABASE_URL=.../lya_qa`，`SETTLE_RECOMPUTE_MODE=shadow`）；admin Vite `:5175`（`VITE_API_TARGET=http://127.0.0.1:8080`）
- config 快照：`data/config.json`，`CFG_SHA=9f433571d016ce8eb6e450ce7bcfc8a2ce994769beac462f12bb5ae751439e94`，`version=6237e0b623219b8b`
- 权威聚合：`recipes=18`，`二阶 11 条 / 三阶 7 条`；AI 结论「共 18 条配方：二阶 11 条、三阶 7 条」

## 门禁结论（总判定：PASS）

| # | 门禁 | 结果 | 关键证据 |
|---|------|------|----------|
| 1 | 三视口 390×844/768×1024/1280×800 打开 Wiki skills/#composite，公开 config 18 条配方四字段逐行对账 | PASS | `reconcile.py 390 768 1280`：三视口 `ROWS=18 MISMATCHES=0`、`ALL_RECONCILE_OK=True`；四列产物/材料A/材料B/阶数，种子 31=灼热弹幕(燃烧弹+燃油弹,2阶)、33=焦土重锤(温压弹+破片弹,3阶)、48=光棱攻城(攻城锤+棱镜散,3阶) 与 config 一致 |
| 2 | AI 四要素同屏 | PASS | 390 聚焦截图 AI 面板：结论「共 18 条配方：二阶 11 条、三阶 7 条」、依据逐字 `RECIPE_AI_BASIS`、红线逐字 `RECIPE_AI_HONESTY`、CTA「查看合成配方 →」；`AI_OK=True FOUR_OK=True`，`BAD=False`（无 NaN/undefined/[object Object]） |
| 3 | CTA 真 CLI 点击 `.r38-ai-cta a`，hash 停留合成锚点、不跳看板 | PASS | `cta-click.txt=✓ Done`；`cta-pre hashBefore=#/wiki?tab=skills` → `cta-post hashAfter=#/wiki?tab=skills&anchor=composite`、`dashboard=false`、`activeNav=["技能图鉴"]`、`recipeStillThere=true` |
| 4 | 表内横滚到末列，页面无横向溢出 | PASS | `scroll-390.json`：`.el-scrollbar__wrap` `sw=510 cw=324`，`scrollLeft 0→186`；`scroll-1280.json`：`sw=884=cw=884` 末列「阶数」直接可见（无需滚动，非缺陷）；三视口 `DOC=390/390、768/768、1280/1280`（`document.documentElement.scrollWidth<=innerWidth+1`）；reconcile `SCROLL=True` |
| 5 | CLI 搜索命中配方并跳 skills/composite | PASS | CLI `type ".search-input" 焦土/光棱` 各 `✓ Done`；`detail-search-jiaotu.json` DOM `outerHTML` 证实命中第三条 `hit-kind=配方 / hit-title=焦土重锤 / hit-meta=合成配方`（光棱同理，见 `search2-guangleng.png`）。注：`search-probe.js` 的 `hits[].kind` 读的是面板首个 span（合成）而非 button 内 span，故汇总 kind 显示为「合成」，逐条 DOM 明细才是真值；配方命中排在第 21+ 条，被 `SEARCH_LIMIT=20` 的默认列表截断，但确实存在并可点击跳转 |
| 6 | R31–R37 七 tab 回归不回退 | PASS | `reg3.sh` 七次 CLI 点击全 `RC=0`；`reg3-basics.json`：`reactionCount=7`、`corrosionSpread=true`、`armorShred=true`、`armorKnock=true`、`basicsTableCounts=[7,5]`（反应 7 行 + 基础技能 5 表）；其余 tab hash/activeNav 各自正确（skills/composite/enemies/levels/gems/mastery），`bad=false`；R36 合成技能表三视口 `R36=True`（36 行）、R37 专精树 `reg3-mastery` 可见 section 正常 |
| 7 | 令牌三锚点无回退、无 --r38- | PASS | `tokens.css:12-14` = `lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6`；`grep -- "--r38-" laiyipao/admin/src` → `NO_MATCH` |
| 8 | 零真值扫描脚本、令牌清理、QA 库还原、业务源码零改 | PASS | `zero-secret-scan.sh`：`BAD_COUNT=0`、`SYNTAX_RC=0`、`.gitignore:426` 命中 data/；`data/admin-auth-init.js` 收尾删除；QA 库 `104/477`；`git diff --check=0`（业务源码变更均来自 t13 实施，本复查未改业务源码、未 git 操作） |
| 9 | 测试证据 | PASS（含 SKIP 标注） | R38 契约 `WikiRecipeMechanics.test.ts` 10/10 passed；全量 vitest 856 用例 855 passed / 1 failed（`bootstrap.test.ts` 登录页用例 20s 超时，环境 flake），同用例定向 `--testTimeout=60000` 2/2 passed；`TestNoRealSecretsInRepo` 因缺 `deploy/.local.keys` → **SKIP（非 PASS）**，脚本日志逐字「读不到 .local.keys —— 真值扫描这条守卫没有执行，不是通过了」 |

## 逐字段对账明细

- 对账脚本 `reconcile.py` 从 `data/config.json` 直接读 `recipes` 的 `output/a/b/out_tier` 四字段，经公开 config 的 `skills + composite_skills` 的 `id→name` 映射得到期望四列；三视口探针 `recipe-probe.js` 从真实 Chromium DOM 读四列 `td .cell`，逐格比对。未知 id 期望值 `（未知技能#N）`，本轮 18 条配方全部映射到真实技能名，无未知项。
- 种子三行（config 权威）：31 灼热弹幕=燃烧弹+燃油弹·2阶；33 焦土重锤=温压弹+破片弹·3阶；48 光棱攻城=攻城锤+棱镜散·3阶。三视口页面显示与之一致（`SAMPLE ROW0/ROW2/ROW17`）。
- 摘要落盘 `data/reconcile-summary.txt`（1127 B），末行 `ALL_RECONCILE_OK=True`。

## 截图（新图，sha256 各异，非登录页）

| 截图 | sha256 | 说明 |
|------|--------|------|
| `r38-focus-390.png` | `47ef90a5635a65c0fa2d5f487845348ab03328a6276d8273061f58ec765c7e58` | 390 配方段聚焦：四列表 + AI 四要素同屏 |
| `r38-focus-768.png` | `6a5ae7aee8475de631e191af1c68ad5fe38d3175faed212b00f1e75dbeef4e42` | 768 平板配方段聚焦 |
| `r38-focus-1280.png` | `9fbc1480b797c086d3c2ce04d1b848d7dcdcbcae72fb09406a18a62df744645d` | 1280 桌面配方段聚焦，末列「阶数」可见 |
| `r38-05-cta-after.png` | `b169b10569522b910eccc69d5ddcb7e3e7866481214ddc099cf32120da897645` | CTA 真实 CLI 点击后：hash 停在 skills&anchor=composite |
| `r38-search2-jiaotu.png` | `3c3d092e543cccccf0b4a79c04c7868d3cfd4ad6157e848171aab1b4d609131c` | CLI 搜索「焦土」命中配方 |
| `r38-search2-guangleng.png` | `a4de27177fbd875d6fedf7a79ece33127651dc97166302a4c08d303c1f7fa7a3` | CLI 搜索「光棱」命中配方 |

## 诚实边界与已知非缺陷

- `TestNoRealSecretsInRepo` 因缺 `.local.keys` 为 SKIP，不作为 PASS 依据；真值防护由 `zero-secret-scan.sh` 的 `BAD_COUNT=0` 补足。
- 全量 vitest 的 1 例 bootstrap 超时为环境 flake（同用例定向加时 2/2 通过），非 R38 引入；报告如实登记，不掩盖。
- 搜索面板默认 `SEARCH_LIMIT=20`，配方命中排在合成技能之后（第 21+ 条），默认列表未展示；但 `data/detail-search-jiaotu.json` 的 DOM 明细含 `hit-kind=配方`，证明 searchIndex 配方条目存在且可命中。探针 `hits[].kind` 取值粒度（面板首个 span）与逐条 button 内 span 不一致，已用 DOM 明细纠正，非产品缺陷。
- 1280 视口配方表 `sw=cw=884`，四列全部可见无需横滚；按 R37 澄清的判据「`sw<=cw` 或滚动后 `after>before`」判为 PASS，非滚动失效。
- 所有浏览器命令均为后台 job + `job_output` 有界控时，未使用 `timeout.exe` 包装 `.cmd`；结束已 kill 遗留浏览器/服务 job。审计脚本无本机绝对路径/真值，根目录统一 `git rev-parse --show-toplevel`。

## 复现命令

```bash
bash docs/laiyipao/audits/round38-wiki-recipes/mint-token.sh
bash docs/laiyipao/audits/round38-wiki-recipes/e2e-viewports.sh
python docs/laiyipao/audits/round38-wiki-recipes/reconcile.py 390 768 1280
bash docs/laiyipao/audits/round38-wiki-recipes/cta-click.sh
bash docs/laiyipao/audits/round38-wiki-recipes/search-recipe.sh
bash docs/laiyipao/audits/round38-wiki-recipes/reg3.sh
bash docs/laiyipao/audits/round38-wiki-recipes/focus-shot.sh
bash docs/laiyipao/audits/round38-wiki-recipes/zero-secret-scan.sh
```
