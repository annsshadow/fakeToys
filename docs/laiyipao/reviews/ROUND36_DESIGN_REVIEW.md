# R36 独立终审：Wiki 合成技能表字段对齐权威 SeedSkill + hash 安全 CTA

- 轮次：R36
- 任务：t4 独立终审（design-reviewer-wb），覆盖 t1 规范 / t2 实施 / t3 真浏览器复查
- 尝试：attempt_id=91e031a2-f313-4c85-9e34-8f70e3e0de9f
- 日期：2026-11-25（Asia/Shanghai）
- 评审人：design-reviewer-wb（独立终审，仅采信本人亲读源码 / 亲跑门禁 / 亲读截图）
- **verdict = pass**（六维门禁全绿；唯一未执行项 TestNoRealSecretsInRepo 如实标注 SKIP，不计 PASS）
- 报告落盘：docs/laiyipao/reviews/ROUND36_DESIGN_REVIEW.md

## 一、判定对象与方法

R36 目标：把 Wiki 合成技能表从漂移的四字段（id/name/element/descr）对齐权威 SeedSkill 九字段（Kind/BaseDamage/HeatCost/CooldownMs/Pierce/AoeRadius/ApplyStacks/Chain/UnlockLevel），修复本地类型漂移，并加 AI 四要素同屏与 hash-router 安全 CTA。本轮不采信 t1/t2/t3 的自报结论，独立从源码亲读、契约与测试复跑、真值守卫、截图亲读、git 保护面五条路径交叉核验。

## 二、六维门禁逐条结论

### ① 规范与契约（t1）— PASS
亲读 docs/laiyipao/specs/round36-wiki-composite-skills/SPEC.md（9894 B）：§0 钉死权威来源 = 公开 /api/v1/config 的 composite_skills（service.go:1517 json tag、:1631-1632 ScaleAllSkills/ScaleAllCompositeSkills 同源），content.go:43-56 SeedSkill 九字段 snake_case、:413-473 SeedCompositeSkills 18 条（ID31~48），engine.ts:1468/1505-1509/1656-1657 双端真实消费；§1 零改动面；§2 复用 @/skillMechanicsText 不写第二套；§3 十三列选择器；§4 AI 四判据；§5 三视口 QA；§6 RED 门禁。
亲读 laiyipao/admin/tests/WikiCompositeMechanics.test.ts：真实 import "../src/compositeMechanicsText"（无内嵌参考实现），8 个 it 覆盖 §2 格式 unknown 边界、§0 权威字段/种子/引擎、§3 类型与列与真实调用、§4 四判据与 hash 安全 CTA、§1/§5 令牌三锚点与禁用 --r36-。t1 保留的 RED 证据（模块解析失败 exit=1）与本轮 GREEN 形成 test-first 闭环。

### ② 实施源码亲读（t2）— PASS
- laiyipao/admin/src/compositeMechanicsText.ts（新增，41 行）：从 @/skillMechanicsText 只导入 MECHANISM_EMPTY / kindText / SkillMechanicsRow 并 re-export MECHANISM_EMPTY，CompositeMechanicsRow = SkillMechanicsRow；三个格式化函数（kindText/cooldownText/pierceText/aoeRadiusText/chainText）全部复用 R32，无第二套。compositeUnlockText 只对有限正整数返回 N 级解锁，其余归 MECHANISM_EMPTY；buildCompositeMechanicsAi 按 kindText==='主动' 与 positiveInt(pierce/aoe_radius/chain) 确定性聚合；三条逐字常量与 SPEC 一致。
- WikiView.vue diff（git diff 亲读）：ConfigResp.composite_skills 本地类型由 4 字段补为 13 字段（含 family?、九权威字段），修复漂移；从 @/compositeMechanicsText 真实导入 5 项并在模板真实调用；id="composite" 整段包 .r36-composite-mechanics、表格包 .r36-composite-table-scroll（overflow-x:auto;max-width:100%，仅作用于这两个选择器，未改全局 .el-table）；13 列（ID/名称/元素/说明 + 类型/冷却/伤害/热量/穿透/范围/连锁/层数/解锁等级），层数用 typeof+Number.isInteger+>0 合法展示否则 MECHANISM_EMPTY；AI 四要素同屏，CTA href=urlFor('skills','composite') + @click.prevent="goTo({tab:'skills',anchor:'composite'})"，无裸 href="#composite"。
- .gitignore diff：410/411 新增 R36 审计 data/ 与 auth-init 忽略规则（含部署后回归目录预留），无越界。

### ③ 真浏览器证据亲读（t3）— PASS
- 三视口 18 条 × 九字段对账：独立重跑 python docs/laiyipao/audits/round36-wiki-composite-skills/reconcile.py 390 768 1280（仓库根执行）→ CFG_SHA=333397d3…（与报告一致），VP=390/768/1280 均 ROWS=18 MISMATCHES=0 AI_OK=True FOUR_OK=True BAD=False DOC=宽/宽（无页面级溢出），ALL_RECONCILE_OK=True（exit 0）。注：Windows 控制台 GBK 编码使中文输出呈乱码，但数值字段清晰可读、不影响判定。
- 种子行（data/ai-and-seeds.txt）：ID31 灼热弹幕=主动/1.9秒/260/46/—/范围190/连锁1/3/30级解锁；ID38 雷暴链=主动/3.2秒/400/68/—/—/连锁6/4/62级解锁；ID48 光棱攻城=主动/3.4秒/520/72/穿透3/范围300/连锁5/4/70级解锁，与权威 config 逐字一致。
- AI 聚合：结论「共 18 个合成技能：主动 18 个，多目标 18 个」=18/18/18，依据/红线/CTA 逐字同屏，无 NaN/undefined/[object Object]。
- CTA hash 安全（data/cta-pre.json → cta-post.json）：hashBefore=#/wiki?tab=skills → hashAfter=#/wiki?tab=skills&anchor=composite，compositeStillThere=true、rowCount=18、dashboard=false、activeNav=[技能图鉴]；cta-click.txt=✓ Done。
- 表内横滚：data/scroll-detail-*.json 三视口 .el-scrollbar__wrap sw=1213/cw=324→after=889（390）、cw=702→511（768）、cw=884→329（1280），真实可滚末列。
- R31–R35 回归（data/reg-*.json 五份）：reactionCount=7、corrosionSpread/armorShred/armorKnock=true、skillCount=24、enemyCount=22、levelCount=20、gems=8、bad=false，与 R35 报告基线一致，无回退。
- 截图亲读（4+ 张，均非登录页）：r36-390-composite.png（390×844，ID31–46 合成表）、r36-fixed-768-composite.png（768×1024，18 行全 + AI 结论「共 18…主动 18…多目标 18」）、r36-fixed-1280-composite.png（1280×800，前 8 列含穿透列真实值 + 目录「技能图鉴」高亮）、r36-05-cta-after.png（CTA 点击后合成表）、r36-reg-gems.png（1258×588，R35 宝石 tab 真实页 + AI 运营卡片）。sha256sum 实测与 REPORT §3 所列一致。

### ④ 独立门禁复跑 — PASS
| # | 命令 | 结果 |
|---|------|------|
| 1 | pnpm exec vitest run tests/WikiCompositeMechanics.test.ts --reporter=dot（cd laiyipao/admin） | exit 0，Test Files 1 passed / Tests 8 passed (8)（定向 8/8） |
| 2 | pnpm exec vitest run R31–R35 五个 WikiMechanics 契约 --reporter=dot | exit 0，5 files / 61 passed（R31–R35 相关契约） |
| 3 | pnpm exec vitest run --testTimeout=90000 --reporter=dot（admin 全量） | exit 0，75 files / 835 passed（无失败；QA 报告曾记录一次 834/1 bootstrap flake，本轮以 90s 超时独立复跑全绿） |
| 4 | pnpm run type-check（vue-tsc --noEmit） | exit 0 |
| 5 | pnpm run build（vue-tsc --noEmit && vite build） | exit 0，built in 3.79s，WikiView chunk 40.78 kB |
| 6 | go test -run TestNoInfraIdentifiersInRepo -v ./internal/domain -count=1（cd laiyipao/server） | exit 0，PASS（2.45s） |
| 7 | git diff --check | DIFF_CHECK_RC=0 |
| 8 | python .../reconcile.py 390 768 1280（仓库根） | exit 0，ALL_RECONCILE_OK=True，CFG_SHA=333397d3… |
| 9 | bash .../zero-secret-scan.sh | BAD_COUNT=0、SYNTAX_RC=0、.gitignore:410/:411 三条命中、AUTH_BEFORE=NO |
| 10 | docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc "SELECT count()||'|'||max(id) FROM battle_records" | 104/477（与 R35 基线一致，QA 库已还原） |
| 11 | test -e .../data/admin-auth-init.js | AUTH_ABSENT（临时令牌已删） |

### ⑤ 保护面 — PASS / NO_PROTECTED_CHANGES
- git diff --name-only 亲读：仅 .gitignore 与 laiyipao/admin/src/views/WikiView.vue 两个修改文件。
- 工作树 R36 预期新增：laiyipao/admin/src/compositeMechanicsText.ts、laiyipao/admin/tests/WikiCompositeMechanics.test.ts、docs/laiyipao/specs/round36-wiki-composite-skills/SPEC.md、docs/laiyipao/audits/round36-wiki-composite-skills/（报告/脚本/截图/对账数据）。
- 未改 engine.ts / 后端 Go / replay / hash / score / settle / loot / tokens.css 业务变量；无新增 --r36-* 令牌。
- 令牌三锚点（tokens.css:12-14）独立 grep：--lyp-token-version: lyp-tokens-v1、--lyp-r3-version: lyp-tokens-v3-r3、--lyp-r6-version: lyp-tokens-r6 均在；全 laiyipao/admin/src grep --r36- 零命中（NO_R36_TOKEN）。
- 结论：NO_PROTECTED_CHANGES。

### ⑥ 诚实边界与真值守卫 — PASS（含如实 SKIP）
- go test -run TestNoRealSecretsInRepo -v ./internal/domain -count=1 → exit 0 但输出 --- SKIP：secrets_leak_test.go:182（读不到 laiyipao/deploy/.local.keys，真值扫描这条守卫没有执行）。按 SKIP 如实标注，非 PASS，不声称真值泄露扫描通过。源码亲读确认 localSecrets 解析不到真值时 t.Skip，与被测行为一致。
- 脚本真值扫描（零注入面）：zero-secret-scan.sh 只覆盖本审计脚本自身 BAD_COUNT=0；data/ 下 config.json 与 auth-init 由 .gitignore:410 拦截、不入库。data/login.json 内有一次 QA 会话 token（已过期 2026-11-25 11:34Z），位于 gitignore 目录，不构成入库泄露；本终审未在对话中回显其值。
- 对账权威仅公开 /api/v1/config；QA 库 104/477 仅用于还原基线，不作字段真值。
- 截图均为真 Chromium 截取、非物理真机；r36-768-composite.png 与 r36-fixed-768-composite.png 内容相同（sha 均为 16ca02fb…），REPORT §3 只列后者，实测哈希与报告一致。
- 全量 vitest stderr 存在既有 jsdom teardown 噪声（not-implemented navigation / skipLink MutationObserver / Vue Router warn about /admin/content），属历史既有噪声，非 R36 功能回退；本轮全量 exit 0、835/835 通过。
- 本轮为只读终审：未改业务源码、未执行任何 git 写操作；唯一落盘为本文档。

## 三、结论
六维门禁（规范契约 / 实施源码 / 真浏览器证据 / 独立门禁复跑 / 保护面 / 诚实边界）全部满足，无阻塞性 finding；唯一未执行项 TestNoRealSecretsInRepo 明确 SKIP 非 PASS。R36 实施满足用户目标，合成技能 Wiki 九字段对齐权威 SeedSkill、四字段漂移修复、AI 四要素同屏与 hash 安全 CTA 均成立，保护面干净（NO_PROTECTED_CHANGES）。verdict = pass，可进入分批发布阶段。

附注：QA 报告称 r36-reg-gems.png 为 390×844，实测尺寸 1258×588（同为真实宝石 tab 页面，非登录页）；该元数据偏差不影响任何功能门禁，仅如实记录。长跑目标（≥1000 轮）不变。
