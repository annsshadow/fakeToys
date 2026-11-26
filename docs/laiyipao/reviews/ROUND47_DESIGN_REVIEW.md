# R47 独立设计终审：R16 验真 CTA 言行一致性

> 结论：**verdict = pass（可发布）**。本轮终审由 same-route-reviewer 独立执行，未修改任何业务源码，未执行 git 写操作；唯一允许落盘的文件为本报告。

## 独立性声明与范围
- 审阅对象：R47 完整链 t58（规范）→ t60（实施）→ t62（真浏览器 QA），reviewed task=t60。
- 写域边界：仅 `docs/laiyipao/reviews/ROUND47_DESIGN_REVIEW.md`；明确不触碰 `laiyipao/admin/src/views/BattlesView.vue`、`laiyipao/admin/tests/**`、`laiyipao/server/**`。
- 环境：工作根 `D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248`；DSH 文件策略 danger-full-access；AGENTS.md 要求中文回复、禁入库真值。

## 维度一：SPEC/契约逐条复核
- `docs/laiyipao/specs/round47-r16-verify-cta/SPEC.md`（63 行）：§0 引用 round16 SPEC.md:39 逐字承诺『4. **可执行语义**：给出 CTA 指向验真——例如「发起验真 →」定位到同抽屉已有验真区…让运营看完逐次归因即可比对 replay_hash；不新造路由。』；如实记录原实现 BattlesView.vue:679 为无 @click 的死 div、:741 CSS 无 cursor:pointer。
- 修复判据六条齐备：CTA 带 @click、方法体含 scrollIntoView/focus 引用验真区、CSS 含 cursor:pointer/pointer-events:auto、文案逐字『发起验真 →』、禁裸 hash、submitVerify/验真结论/replay_hash 不改只定位。
- 契约 `laiyipao/admin/tests/R16ReactionTimeline.render.test.ts`（R47 describe 从 189 行起）：真实 import node:fs readFileSync / node:path join，viewPath 与 r16SpecPath 均指向真实磁盘源码；ctaLine/cssRule/clickHandler 三个辅助函数 + 4 个 it 覆盖可点击/定位/CSS 光标/round16 SPEC 承诺，无内嵌参考实现。
- RED 曾真：t58 规范阶段契约对原实现 exit=1、3 failed（死 div 无 @click / CSS 无 cursor 等新增断言），无解析错误/模块缺失/超时；原始 RED 日志未单独留存，该项依队长复核记录采信并如实标注。本轮亲自复跑当前实现为 GREEN 13/13。

## 维度二：实施 diff 严格三处
- `git diff -- laiyipao/admin/src/views/BattlesView.vue` 精确三 hunk：① :280 后新增 `scrollToVerify()`，仅 `document.querySelector<HTMLElement>('.verify-row')` + `el.scrollIntoView({behavior:'smooth',block:'center'})`，附注释『只定位到同抽屉已有验真区，不自动提交/不自动填哈希』；② :685 模板死 div 加 `@click="scrollToVerify"`，文案逐字不变；③ :747 CSS 仅加 `cursor: pointer;`。
- 亲读 :280-295 确认 scrollToVerify 无提交、无填 hash；submitVerify / 验真结论 / replay_hash / score / settle / loot / 后端 / tokens 零改。

## 维度三：QA 证据独立判断
- `docs/laiyipao/audits/round47-r16-verify-cta/REPORT.md`（39 行，t62）总判定 PASS。本人独立读 cta-before.json / cta-after.json / probe2.json 及 open2/mark/detail-click/cta-scroll/cta-move/cta-down/cta-up 共 7 份 TXT，逐字段对账一致。
- 点击前：href `#/admin/battles`、isLogin=false、drawerOpen=true、ctaText『发起验真 →』、ctaCursor=pointer、verifyRows top117/133『种子 5621489295744462231』、hashes=[8ddc5cc01e787ec9,5621489295744462231]、hash 输入框 value 空、verifyResultCount=0、bodyLen=7634。
- 点击后（真实鼠标 CTA_MOVE=1043,386 move/down/up 全 ✓ Done）：href / isLogin / drawerOpen / hashes / inputs / verifyResultCount / bodyLen 全部不变，verifyRows top117→378、ctaRect.y 378→639，证明已平滑滚动定位到验真区；未提交、未填 hash、未跳页、未改 replay_hash。
- 正对照：open2 非登录页、mark marked=1、detail-click ✓Done；probe2 verifyRowTop=562 与滚动位移一致（平滑滚动下 scrollTop=0 属采样时机，以 getBoundingClientRect 位移为准）。

## 维度四：截图 sha256 重算与亲读
- 本人 sha256sum 亲算 3 张，与 REPORT §三逐行一致且两两互异：r47-detail-open.png=95acdd9ac1c901117ca07f6c9a285bac6e92d720157fea9cb29af5936f726565；r47-cta-before.png=745ddcca5beb36b9fb2eb6b4f94fc88598e7b0ab7959d7d72a547fd2491a82bd；r47-cta-after.png=af42c501cf5b5fac33ba5f4aeaf1625ed5bc69088dcda75fba006a06e7ce68bd。
- read_image 亲读 3 张均 1280×720 非登录页：detail-open 为战报详情抽屉 id477；cta-before 抽屉内 AI·反应时间线（64 次）+ 红色 CTA 在下方、验真区在顶部；cta-after 抽屉已滚动、验真区（种子/空 hash 框/提交比对）上移视口、AI 块下移——视觉确认 CTA 仅滚动定位。

## 维度五：独立复跑门禁（全绿）
| 命令 | 结果 |
| --- | --- |
| `pnpm exec vitest run tests/R16ReactionTimeline.render.test.ts --reporter=dot` | exit0，1 file / 13 tests passed，15.61s；stderr 仅内存路由 warn |
| admin 全量 `pnpm exec vitest run` | exit0（后台 job bash-c55ee524dfdd4ff6）；ContentView jsdom 导航告警非失败 |
| `pnpm run type-check && pnpm run build` | exit0（后台 job bash-43dbb91e29b84ca0）；✓ built in 7.42s |
| `go test ./internal/domain/ -run 'TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo' -v` | exit0；TestNoInfraIdentifiersInRepo PASS(5.07s)；TestNoRealSecretsInRepo **SKIP** |
| `git diff --check` | exit0；两处告警来自入场前既有越界文件（.gitignore LF→CRLF、R16ReactionTimeline.render.test.ts:287 EOF 空行），非本次改动 |
- TestNoRealSecretsInRepo SKIP 逐字原因：『读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」』。严格标注为未执行，不计 PASS。

## 维度六：保护面与零真值
- auth 四路径全 ABSENT：审计 data/admin-auth-init.js、data/login.json、根 admin-auth-init.js、login.json。
- `git check-ignore -v` 命中 `.gitignore:482:docs/laiyipao/audits/round47-r16-verify-cta/data/`。
- 令牌三锚点 tokens.css:12-14 原样（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6）；grep `--r47-`/`--r48-` 源码与测试零新增（仅否定断言）。
- QA 库 `docker exec lyp-qa-pg psql -U postgres -d laiyipao` 查 battle_records=104 / max(id)=477。
- 保护面 engine/replay/hash/score/settle/loot/tokens/后端零改；tracked diff 仅 BattlesView.vue + 契约测试（本终审未改源码）。

## 诚实标注（非阻塞）
1. t58 RED 原始日志未单独留存，3 failed 依队长复核记录采信；本轮 GREEN 亲自复跑。
2. TestNoRealSecretsInRepo 因缺 .local.keys 为 SKIP（未执行），不是 PASS。
3. 平滑滚动下 probe2 scrollTop=0 属采样时机，以 getBoundingClientRect 位移判定定位成立。
4. admin 全量与契约测试的 Vue Router / jsdom 导航告警属测试内存路由，非失败。
5. git diff --check 两处告警来自入场前既有越界文件，非 R47 改动。
6. 本轮只读复跑，未写 QA 库；QA 库 104/477 为只读查询结果。

## 结论
六维门禁全部通过，R16 验真 CTA 兑现『发起验真 →』的可执行定位语义且不触碰保护面，判定 **verdict=pass，可发布**。