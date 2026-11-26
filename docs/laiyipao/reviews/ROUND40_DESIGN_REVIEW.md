# R40 独立终审：Wiki 评分权重五键对齐与搜索高亮修复闭环

- 审阅人：same-route-reviewer（laiyipao-r36 · R40 独立终审）· 日期：2026-11-25 · 轮次：review round 1（attempt_id d908e6c0-32d0-45f8-8216-9fd6b0e8b8d9）
- 被审对象：t22 规范与实施 + t27 首轮 QA（发现搜索 P1 阻塞）+ t29 修复 + t30 修复后独立复查
- 独立性：本人未参与 R40 的规范、实施、QA、修复、复查任一环（t22/t27/t29/t30 由其他成员完成）；全部结论基于本人亲自读源码、读报告、读原始证据 JSON、独立重跑命令、独立重算截图 sha256、亲读关键截图得出，不采信成员自述。
- 证据环境：本地回环 127.0.0.1（Vite/API）+ 真实 Chromium CLI（agent-browser）三视口 390×844 / 768×1024 / 1280×800；QA 库容器 lyp-qa-pg。无真实域名/IP/口令/私钥/个人路径入库，地址均用回环。
- **终审结论：PASS**（六维门禁全部达标；TestNoRealSecrets 因缺 .local.keys 为 SKIP、reconcile 两个探针假阳性，均如实披露，不构成应用缺陷）。

## 1. 维度一：SPEC 与格式化契约逐条复核 — PASS

`docs/laiyipao/specs/round40-wiki-rating-weights/SPEC.md`（58 行）八节齐备，本人逐条核对：

- 缺口依据带行号：power.go:253-259 RatingWeights 五字段 json tag、:262-269 默认 12/15/10/12/8、:350-354 ComputeBuildRating、miniapp index.vue:470-574 双端消费、miniapp fixtures.ts:147；权威数值与示例分层（示例值不硬编码进 Wiki，差异以公开 config 为准）。
- 五键中文逐字：element_coverage→元素覆盖 / reaction_coverage→反应链 / mastery_done→专精投入 / equipment_synergy→装备契合 / mechanic_depth→机制深度；未知合法键「（未知维度#key）」、空/脏值破折号—。
- ratingValue 只收有限非负整数（含 0），直显 String(v)，绝不写 %/‰；AI 模板逐字「权重 N/5 项齐备」；RATING_AI_BASIS / HONESTY / CTA 三条逐字。
- 零改动面（禁改 engine/后端/replay/hash/score/settle/loot/tokens）、令牌三锚点（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6）、禁新增 --r40-*、诚实红线（配置基值非权威结算、replay_hash）均已钉死。
- 第 8 节搜索门禁同步 t29 修复：HitKind 加 rating、searchIndex 遍历 RATING_KEYS 索引 key/中文/权重、评分表 rowFocusClass('rating')、三个搜索示例（反应链/mechanic_depth/元素覆盖）。

## 2. 维度二：契约、helper、WikiView 实现与搜索修复链 — PASS

- 契约 `laiyipao/admin/tests/WikiRatingMechanics.test.ts`（160 行 11 例）第 6 行真实 import 8 导出，无内嵌参考实现；第 11 例为 t29 新增的搜索索引与高亮门禁。
- helper `laiyipao/admin/src/ratingMechanicsText.ts`（68 行）纯函数：`MECHANISM_EMPTY` 从 @/enemyMechanicsText 同源 re-export（无第二份字面量）；RATING_KEYS as const 固定五键；`RATING_LABELS` 用 `new Map` 规避原型链键问题；ratingLabel/ratingValue/buildRatingMechanicsAi 与三常量逐字符合契约。
- WikiView.vue 本地 ConfigResp.rating_weights 收紧为封闭五字段，评分表按 RATING_KEYS 固定序渲染中文标签与整数权重，AI 四要素同屏，CTA 用 `urlFor('rating')` + `goTo({tab:'rating'})`（无裸 hash）。
- t29 对 WikiView.vue 的工作树改动精确为 5 处，本人 git diff 逐处核对：①HitKind 加 'rating'（:198）②HIT_KINDS 加 'rating'（:200）③KIND_LABEL.rating='评分'（:214-215）④placeholder 加 /评分（:819）⑤searchIndex 末尾遍历 RATING_KEYS push 命中对象（kind/id/title/meta/body/section，:593）⑥评分表 el-table 加 `:row-class-name="rowFocusClass('rating')"`（:991）。t22 主体（评分区/本地类型/AI/CTA）已在 HEAD。
- 焦点链完整：`pick()` 写 `focus:{kind:hit.kind,id:hit.id}` → URL `?tab=rating&focus=rating:`key`` → `parseFocus` 经 HIT_KINDS 校验 → `rowFocusClass('rating')` 走 `scope.row.id ?? scope.row.key` 匹配评分行 key → `row-focus` 高亮；`data-focus="focusKey"` 触发父重渲染保证行类名重算。链路无断点。

## 3. 维度三：t27 原失败项闭环 — 三视口真 CLI 搜索命中与点击高亮 — PASS

- t27 首轮 REPORT.md 如实记 FAIL：搜「反应链」/mechanic_depth 命中 hits=[]、点击无高亮（定位 searchIndex 未索引评分五行）。该阻塞项由 t29 修复、t30 复查、本轮终审三方闭环。
- 本人亲读 t30 POSTFIX_REPORT.md 及原始证据 JSON：三视口 390/768/1280 × 两查询 6 组，fill 后 `kind=评分`、title/meta 正确（反应链 / 评分 · reaction_coverage；mechanic_depth / 机制深度）；点击 .search-hit 真 CLI `✓ Done`；after JSON 显示 URL `#/wiki?tab=rating&focus=rating:reaction_coverage` / `...mechanic_depth`，`ratingRowFocus=true`，focusText 分别为「反应链 / 15」「机制深度 / 8」。6 组全绿。
- 亲读关键截图：`pf2-390-fanyinglian-focus.png`（反应链行蓝色高亮、15、AI 四要素同屏）、`pf2-1280-mechanic_depth-focus.png`（机制深度行蓝色高亮、8）、`pf2-390-rating.png`、`pf2-cta-after.png`——均为真实 Wiki 页面，非登录页。

## 4. 维度四：三视口五行标签权重零不符 + CTA + 回归不回退 — PASS

- 公开 config（`data/config.json`，version 6237e0b623219b8b，require_login=false）rating_weights 五键 = 12/15/10/12/8，本人独立解析确认；三视口 reconcile rows=5 mismatches=0 ok=true；无 NaN/undefined/[object Object]、无 %/‰、无页面级横溢。
- CTA 真 CLI：`cta-pre.json` hash=#/wiki、`cta-click.txt`=✓ Done、`cta-post.json` hash=#/wiki?tab=rating、dashboardJump=false、ratingRows=5——停留 rating 不跳看板。
- R31–R39 九 tab 回归（basics/skills/composite/enemies/levels/gems/mastery/skins/rules）行数与基线一致，未见回退。

## 5. 维度五：独立复跑测试与构建 — PASS

本轮本人于工作树当前状态独立执行（2026-11-25 23:48）：

| 门禁 | 命令 | 结果 |
|---|---|---|
| R40 契约定向 | `pnpm exec vitest run tests/WikiRatingMechanics.test.ts --reporter=dot` | **1 文件 / 11/11 通过**，exit 0 |
| admin 全量 | `pnpm exec vitest run --testTimeout=90000 --reporter=dot` | **79 文件 / 878/878 全绿**，exit 0（37.0s） |
| type-check | `pnpm run type-check`（vue-tsc --noEmit） | exit 0 |
| build | `pnpm run build`（vue-tsc + vite build） | exit 0，✓ built in 1.48s |
| git diff 检查 | `git diff --check` | exit 0 |
| Go 守卫 | `go test ./internal/domain -run TestNoInfraIdentifiersInRepo / TestNoRealSecretsInRepo -count=1 -v` | TestNoInfraIdentifiersInRepo **PASS**(4.94s)；TestNoRealSecretsInRepo **SKIP**（缺 .local.keys，真值扫描未执行，如实标注非 PASS） |

- 数字口径说明：依赖报告记录 admin 全量 877/877（t27 首轮时 R40 契约 10/10）；t29 为契约新增第 11 例（搜索门禁）后，本轮独立复跑为 878/878。两数均为全绿，差异系新增一例，非异常。
- 定向 38/38（t29 报告）与全量 878/878 一致覆盖；本轮以全量为权威。

## 6. 维度六：保护面、证据完整性与诚实标注 — PASS

- **截图 sha256**：本人独立重算 POSTFIX 第 6 节 11 张 pf2-*.png，与报告清单逐行一致，两两互异（unique=11），≥4 张硬门禁满足；pf-*.png 与对应 pf2 图同 sha 属 smoke 与正式图重复，不影响 pf2 集合互异。
- **零真值收尾**：四个临时 auth 路径（审计 data/admin-auth-init.js、data/login.json、根 admin-auth-init.js、login.json）实测全 ABSENT；`git check-ignore -v` 命中 .gitignore:442 覆盖审计 data/；审计目录秘密模式 grep 仅命中报告中对 SKIP/零命中的文字描述，无真值。
- **QA 库基线**：容器 lyp-qa-pg 实测 `select count(*),max(id) from public.battle_records` = **104|477**，与基线一致；本地 dump 的 battle_records COPY 段亦为 104 行。t27/t30 曾因文本 dump 与 pg_restore 不兼容导致 DB_CHECK=0_0、随后以 psql 直接导入修复，报告已如实记录。
- **诚实标注**：TestNoRealSecretsInRepo=SKIP 非 PASS；reconcile.py ALL_RECONCILE_OK=False 的两个假阳性（basicsMissingRatingSection 探针分类口径、composite=36 计数伪影）已由 t27/t30 与本轮披露，人工复核以 rows/mismatches=0 为准；t27 首轮搜索 FAIL 未被掩盖。
- **未改禁改面**：本轮终审仅新增本报告一个文件，未修改任何业务源码、未执行 git 写操作；WikiView.vue 的 t29 改动属前序任务产物，engine/后端/replay/hash/score/settle/loot/tokens 未见漂移；令牌三锚点保留、无 --r40-*（契约断言与全量绿佐证）。

## 7. 观察与建议（非阻塞，不影响 PASS）

- t27 finalize.sh 用 pg_restore 恢复纯文本 dump 的脚本缺陷仍存在于审计脚本中（以 psql 手工修复数据）。属审计工具链问题，非业务缺陷；建议后续轮次修正脚本，使复查可一键收尾，不再依赖手工导入。
- 全量测试计数随新增用例从 877 增至 878 属正常演变，无需处理。

## 8. 结论

R40 在规范、契约与实现、真浏览器证据、五行权重对账与 CTA/回归、独立测试构建、保护面六个维度全部达标，证据链完整且诚实（SKIP/假阳性/首轮 FAIL/DB 修复均如实披露）。t27 唯一阻塞项（评分维度搜索命中与高亮）已由 t29 修复、t30 复查、本轮终审形成完整闭环。**终审判定：PASS，可进入分批发布。**
