# R38 独立终审：Wiki 合成配方（recipes）字段对齐与 hash 安全 CTA

- 轮次：R38 / 覆盖 t12 规范、t13 实施、t14 真浏览器复查 / 角色 design-reviewer-wb（独立终审）
- 判定对象：t13（Wiki 合成配方表实施）
- 工作目录：`D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248`，分支 `opt1000/laiyipao-ai-refactor`，HEAD `abaac8aa2`
- 权威口径：公开 `/api/v1/config` 的 `recipes`（与服务端 `SeedRecipe`/`SeedRecipes` 同源），非 SQL 直连、非手填
- 复跑时间：2026-11-25 19:56–20:03（Asia/Shanghai）

## 终审结论

**verdict = pass**。六维门禁逐项独立复核全绿，证据均可复现。独立复跑未依赖任何成员口头报告：亲读 SPEC/契约/helper/WikiView diff/REPORT、亲读 6 张关键截图、亲跑全部测试/构建/守卫、亲算 sha256/DB 计数/保护面。

| 维度 | 结论 | 一句话证据 |
|------|------|-----------|
| 1 规范/契约 | PASS | SPEC 52 行钉死七导出与七口径；契约真实 import `../src/recipeMechanicsText`，无内嵌实现 |
| 2 实施 | PASS | helper 91 行满足七口径、MECHANISM_EMPTY 同源复用无第二份；WikiView diff 补齐四列/映射/横滚/AI/CTA/searchIndex |
| 3 QA 真浏览器 | PASS | REPORT 61 行总判定 PASS；三视口 18×4 零不符，6 张截图亲读非登录页、sha 逐行一致互异 |
| 4 测试/构建门禁 | PASS | 契约 120/120、全量 856/856、type-check/build exit0、Go TestNoInfra PASS、diff --check RC0 |
| 5 诚实边界 | PASS | TestNoRealSecrets 如实 SKIP 非 PASS；flake/SEARCH_LIMIT/1280 免滚/探针粒度均如实登记 |
| 6 收尾/保护面 | PASS | 零真值 BAD_COUNT=0、auth-init ABSENT、QA 库 104/477、无 --r38-、NO_PROTECTED_CHANGES |

## 维度 1：规范与契约（亲读）

- `docs/laiyipao/specs/round38-wiki-recipes/SPEC.md`（52 行）逐节亲读：§0 缺口依据带行号（`service.go:1518/1633`、`content.go:59-64/474-495`、`types.gen.ts:743-748`、`bag.vue:101-145/732-767`）；§1 零改动面与诚实红线；§2 七导出逐字口径；§3 UI/选择器；§4 AI 四判据；§5 三视口 QA；§6 RED 门禁；§7 分工。
- `laiyipao/admin/tests/WikiRecipeMechanics.test.ts`（126 行）第 7 行真实 `import { MECHANISM_EMPTY, recipeMaterialsText, tierText, buildRecipeMechanicsAi, RECIPE_AI_BASIS, RECIPE_AI_HONESTY, RECIPE_AI_CTA } from "../src/recipeMechanicsText"`——七导出齐全、无内嵌参考实现；模块缺失会整 suite RED。
- 契约覆盖核对：权威四字段证据（33/35/36 行）、Map/Record/函数映射与未知 id 逐字（含 id=999→「（未知技能#999）」）、tierText 正整数/非法边界、AI 确定性聚合（t2/t3 严格=2/3）、unknown 全边界（0/负数/小数/NaN/±Infinity/null/undefined/非数字）、UI 四列与真实调用、AI 四要素、hash 安全 CTA（`:href=urlFor(...)` + `@click.prevent=goTo(...)`、`not.toContain('href="#')`）、searchIndex 配方命中、诚实红线与令牌三锚点。
- 判定：规范与契约维度 PASS。

## 维度 2：实施（亲读 helper + WikiView diff）

- `laiyipao/admin/src/recipeMechanicsText.ts`（91 行）逐行亲读，七导出全部落地：
  - 第 4–5 行 `export { MECHANISM_EMPTY } from '@/enemyMechanicsText'` + 同源 import——**无第二份字面量声明**；全 src grep `const/let MECHANISM_EMPTY` 无结果，唯一真值源 `enemyMechanicsText.ts:9`。
  - `recipeMaterialsText(r, skillNameById)`：`unknown` 入参；a/b 必须正整数否则空态；`skillNameOf` 同时支持 Map/Record/函数三种映射；映射抛错也只回退「（未知技能#N）」不泄漏异常；输出「{材料A} + {材料B}」。
  - `tierText` 仅有限正整数→「N 阶」，否则空态；`buildRecipeMechanicsAi` 只读数组、output/a/b/out_tier 四项全合法才计数、t2/t3 严格等于 2/3、skills/compositeSkills 仅 `void` 不参与计数、同输入严格相等；三条逐字常量与 SPEC 完全吻合。
- `laiyipao/admin/src/views/WikiView.vue` diff 亲读（相对 HEAD）：
  - 导入别名 `MECHANISM_EMPTY as RECIPE_EMPTY`、`recipeMaterialsText as recipeTextFromHelper`（批量导入第 43 行）；注释明确单参包装只在模板出现。
  - `ConfigResp` 补 `recipes: Array<{ output: number; a: number; b: number; out_tier: number }>`（snake_case，无 `outTier` 漂移）。
  - `HitKind`/`HIT_KINDS`/`KIND_LABEL` 新增 `recipe`→「配方」；`searchIndex` 遍历 `c.recipes ?? []` 生成命中：`id=String(r.output)`、`title=产物名`、`body` 同时含材料A/材料B/阶数、`section='skills'`/`anchor='composite'`——产物名与材料名均可搜到。
  - `nameById` computed 合并 `skills`+`composite_skills` 的 id→name Map；`recipeOutputText` 产物列复用同映射、未知 id 逐字；`recipeMaterialsText` 包装调用底层二参纯函数。
  - `.r38-recipe-mechanics` 区位于 R36 合成技能表之后、同一 #composite 锚点内，四列「产物/材料A/材料B/阶数」+ `el-empty` 空态；`.r38-recipe-table-scroll { overflow-x:auto; max-width:100% }` 仅表内横滚。
  - AI 四要素同屏：结论 `buildRecipeMechanicsAi(cfg?.recipes ?? [], cfg?.skills ?? [], cfg?.composite_skills ?? [])`、依据 `RECIPE_AI_BASIS`、红线 `RECIPE_AI_HONESTY`、CTA `RECIPE_AI_CTA` 用 `:href=urlFor('skills','composite')` + `@click.prevent=goTo({ tab:'skills', anchor:'composite' })`；全文件无 `href="#"` 裸hash。
- 令牌锚点三处无漂移、无 `--r38-`（tokens.css:12-14 实测 `lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6`）；材料列显示真实技能名而非 #id。
- 判定：实施维度 PASS。

## 维度 3：QA 真浏览器（亲读 REPORT + 6 图 + 证据快照）

- `docs/laiyipao/audits/round38-wiki-recipes/REPORT.md`（61 行）亲读，总判定 PASS，9 门禁逐项有证据。
- 权威 config 独立解析（`data/config.json`，UTF-8）：`recipes=18`、`t2=11`、`t3=7`；种子 31={1,2,2}、33={3,11,3}、48={12,15,3} 与 REPORT 一致；`skills=24`、`composite_skills=18`。CFG_SHA 亲算 `9f433571d016ce8eb6e450ce7bcfc8a2ce994769beac462f12bb5ae751439e94` 与 REPORT §3 一致。
- `data/reconcile-summary.txt`（GBK 编码，1127 B）亲读：390/768/1280 三视口均 `ROWS=18 MISMATCHES=0 AI_OK=True FOUR_OK=True BAD=False DOC=vp/vp SCROLL=True R36=True`、`ALL_RECONCILE_OK=True`；三视口四列逐字段对公开 config 零不符。
- CTA 真 CLI 点击证据：`cta-pre.json` `hashBefore=#/wiki?tab=skills`→`cta-post.json` `hashAfter=#/wiki?tab=skills&anchor=composite`、`dashboard=false`、`activeNav=["技能图鉴"]`、`recipeStillThere=true`、`rowCount=18`。
- CLI 搜索证据：`detail-search-jiaotu.json` DOM `outerHTML` 逐条含 `hit-kind=配方 / hit-title=焦土重锤 / hit-meta=合成配方`；`r38-search2-*.png` 亲读见「配方 焦土重锤」「配方 光棱攻城」命中行。
- 横滚证据：`scroll-390.json` `.el-scrollbar__wrap` `sw=510 cw=324`、`sl 0→186`（真横滚）、`docW=innerW=390`；`scroll-1280.json` `sw=cw=884` 末列「阶数」可见免滚。
- 七 tab 回归：`reg3-basics.json` `reactionCount=7`、`corrosionSpread/armorShred/armorKnock=true`、`basicsTableCounts=[7,5]`、`compositeCount=36`、`recipeCount=18`、`bad=false`。
- 6 张关键截图逐张亲读（非登录页）：`r38-focus-390`（390×844，四列表+AI 四要素）、`r38-focus-768`（768×1024）、`r38-focus-1280`（1280×800，末列可见）、`r38-05-cta-after`（390×844，停在合成技能区）、`r38-search2-jiaotu`、`r38-search2-guangleng`（均 1280×800，配方命中行可见）。
- sha256sum 独立亲算：6 图与 REPORT §3 逐行完全一致，6 个哈希互异（distinct=6）。
- 判定：QA 维度 PASS。

## 维度 4：测试/构建门禁（独立复跑，命令与结果）

| 命令 | exit | 结果 |
|------|------|------|
| `pnpm exec vitest run tests/Wiki*.test.ts --reporter=dot` | 0 | 10 files / 120 tests passed（R31–R38 全 Wiki 契约） |
| `pnpm exec vitest run tests/WikiRecipeMechanics.test.ts ... --reporter=dot` | 0 | 5 files / 52 tests passed（R38 + 定向相关） |
| `pnpm exec vitest run --reporter=dot --testTimeout=90000` | 0 | 77 files / 856 tests passed（本轮全量全绿） |
| `pnpm type-check && pnpm build` | 0 | TYPECHECK_RC=0 / BUILD_RC=0；WikiView chunk 47.64 kB |
| `go test ./internal/domain -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo' -count=1 -v` | 0 | TestNoInfra PASS(3.27s)；TestNoRealSecrets **SKIP** |
| `git diff --check` | 0 | DIFF_CHECK_RC=0，无空白错误 |

- 说明：QA 报告记录其全量曾出现 1 例 `bootstrap.test.ts` 20s 超时 flake（定向加时 2/2 通过）；本轮终审独立以显式 `--testTimeout=90000` 复跑得到 856/856 全绿，未复现该 flake。两种结果均如实记录，不互相覆盖。stderr 中的 jsdom/Vue Router 警告为既有环境噪声，非 R38 回退。
- 判定：测试/构建门禁维度 PASS（TestNoRealSecrets 的 SKIP 见维度 5）。

## 维度 5：诚实边界（如实标注）

- **TestNoRealSecretsInRepo = SKIP（非 PASS）**：因缺 `laiyipao/deploy/.local.keys`，`secrets_leak_test.go:182` 输出 `--- SKIP`；真值防护由 `zero-secret-scan.sh` 的 `BAD_COUNT=0` 独立补足，但守卫本身未执行，不作为 PASS 依据。
- 全量 vitest 历史 flake（1 例 bootstrap 20s 超时）如实登记；本轮独立复跑未复现，不掩盖历史。
- `SEARCH_LIMIT=20` 默认列表截断：配方命中排在合成技能之后（第 21+ 条），默认视图未直接展示；但 DOM 明细证明条目存在可命中可跳转，非产品缺陷。
- 搜索汇总探针 `hits[].kind` 粒度缺陷：`search-probe.js` 读面板首个 span 显示「合成」，逐条 button 内 span 才是「配方」真值；已用 DOM 明细纠正。
- 1280 视口 `sw=cw=884`，四列全可见无需横滚，按「`sw<=cw` 或滚动后 `after>before`」判据为 PASS，非滚动失效。
- `reconcile-summary.txt` 为 GBK 编码（非 UTF-8），读取需指定编码；仅证据文件编码特性，不影响对账结论。
- 判定：诚实边界维度 PASS（所有非 PASS/非缺陷项均明确标注，无隐瞒）。

## 维度 6：收尾与保护面

- 零真值扫描 `zero-secret-scan.sh`：`BAD_COUNT=0`、`SYNTAX_RC=0`、`.gitignore:426` 命中 `docs/laiyipao/audits/round38-wiki-recipes/data/` 与 `data/config.json`/`admin-auth-init.js`、`AUTH_BEFORE=NO`。
- `laiyipao/admin/public/data/admin-auth-init.js`：ABSENT（收尾已删）。
- QA 库 `docker exec lyp-qa-pg psql ... battle_records`：`104|477`（测试前后基线一致）。
- 令牌三锚点保留（tokens.css:12-14），全 `laiyipao/admin/src` 无 `--r38-`（grep 无匹配）。
- 保护面：`git diff --name-only` 仅 `.gitignore` 与 `laiyipao/admin/src/views/WikiView.vue`；受保护路径（engine/server/replay/hash/score/settle/loot/tokens/miniapp）匹配数 = 0；R38 新增仅 `recipeMechanicsText.ts`、`SPEC.md`、`audits/round38-wiki-recipes/`。**NO_PROTECTED_CHANGES**。
- 终审自身未改任何业务源码、未执行任何 git 写操作（仅新增本报告 docs/laiyipao/reviews/ROUND38_DESIGN_REVIEW.md）。
- 判定：收尾/保护面维度 PASS。

## 证据链索引

- 规范：`docs/laiyipao/specs/round38-wiki-recipes/SPEC.md`
- 契约：`laiyipao/admin/tests/WikiRecipeMechanics.test.ts`
- helper：`laiyipao/admin/src/recipeMechanicsText.ts`
- 实施：`laiyipao/admin/src/views/WikiView.vue`（diff 相对 abaac8aa2）
- QA 报告：`docs/laiyipao/audits/round38-wiki-recipes/REPORT.md`
- 终审报告：`docs/laiyipao/reviews/ROUND38_DESIGN_REVIEW.md`（本文件）

## 最终判定

六维门禁全绿，**verdict = pass**，可进入分批发布。唯一非 PASS 项为 `TestNoRealSecretsInRepo` 的 SKIP（缺 `.local.keys`），已如实标注并由零真值扫描独立补足，不构成阻断。
