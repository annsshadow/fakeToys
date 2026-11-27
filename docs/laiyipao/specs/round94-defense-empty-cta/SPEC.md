# R94 规范：DefensesView 空态 CTA 与 reason 一一对应（RED）

## §0 背景
承接 t215 只读侦察「docs/laiyipao/recon/ROUND94_B_ADMIN_EMPTY_RECON.md」：DefensesView.vue:436 空态 reason 写「当前筛选条件下没有防线；可清空筛选或刷新后重试。」，但 :437 唯一 CTA「刷新防线」绑定的 handler 为「((page = 1), load())」——只重置页码并重拉当前筛选，不清空 keyword / ownerId / shieldedFilter。真浏览器三视口探针与 fetch URL hook 坐实：点 CTA 后输入框仍保留 NO_SUCH_DEFENSE_zzz，请求 URL 仍带 keyword 参数。对照 BattlesView:135-143 的 resetFilters() 真实清零并重载，四条件全部成立，队长裁定立 R94 修复轮，采法 A（行为对齐），与 Battles / Levels / Skills / Users 四视图同构。

## §1 权威口径
- docs/laiyipao/design/AI_VISUAL_SYSTEM.md:405：「空态卡高度与正常态一致；文案结构固定为『缺什么数据 + 下一步动作』，CTA 与结论一一对应」。
- 同文件 :115：「文案以『数据不足的原因 + 下一步动作』为结构」。
- 对照范式：BattlesView.vue:135-143 的 resetFilters()（userId/levelId/result/onlyMismatched/dateRange 清零 + page=1 + load()）；LevelsView:56、SkillsView:21、UsersView:27 同构。
- 侦察报告裁定表（四条件全成立）与 CTA 行为验证（before/after sha 相同，证明点后界面无变化）。

## §2 精确改点（唯一写域 laiyipao/admin/src/views/DefensesView.vue）
a) 在 load() 之后新增函数（变量名以本文件实际源码为准：keyword / ownerId / shieldedFilter / page）：
   function resetFilters() {
     keyword.value = ''
     ownerId.value = undefined
     shieldedFilter.value = undefined
     page.value = 1
     load()
   }
b) 空态 CTA（现 :437）：文案「刷新防线」改为「清空筛选」，@click 由「((page = 1), load())」改为「resetFilters」。
c) reason（现 :436）推荐保留原文案「当前筛选条件下没有防线；可清空筛选或刷新后重试。」——CTA 已真实兑现「清空筛选」分支，reason 与 CTA 语义一致；若微调也不得丢失与 CTA 的一一对应。

## §3 不变量（严禁改动）
- loadSeq 请求序号守卫：:192 let loadSeq = 0、:195 const seq = ++loadSeq、:203 / :211 两处 seq !== loadSeq return、:214 seq === loadSeq 才清 loading。
- load() 的 fetchDefenses(PAGE_SIZE, (page.value - 1) * PAGE_SIZE, { keyword / owner_id / shielded }) 参数与调用语义。
- 删除防线二次确认（onDeleteDefense 与 ElMessageBox.confirm）、防线挑战记录 el-dialog、KPI 与各列渲染、emptyText、分页、AI 洞察卡均不改。
- 令牌三锚点（laiyipao/admin/src/styles/tokens.css:14-16）：--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 原样保留。
- DefensesView 业务源码零「--r94-」；其它视图、组件、样式、契约、SPEC（本文件外）、.gitignore 零改动；不写真值、不写 git。

## §4 契约设计
契约文件：laiyipao/admin/tests/R94DefenseEmptyCta.contract.test.ts，用 import.meta.url + dirname + join + fs.readFileSync 真实读取 laiyipao/admin/src/views/DefensesView.vue / BattlesView.vue / laiyipao/admin/src/styles/tokens.css，禁止内嵌源码副本。共 6 it：
- it1（缺口）：resetFilters 函数存在，函数体含 keyword.value = ''、ownerId.value = undefined、shieldedFilter.value = undefined、page.value = 1、load()。
- it2（缺口）：空态 CTA 行含「清空筛选」与 @click="resetFilters"，且不含旧文案「刷新防线」与旧 inline handler「((page = 1), load())」。
- it3（缺口）：reason 行含「清空筛选」承诺，且 CTA 行同样含「清空筛选」（一一对应）。
- it4（不变量）：loadSeq 守卫仍在（let loadSeq = 0 / const seq = ++loadSeq / 至少 3 处 seq !== loadSeq / 至少 1 处 seq === loadSeq）、fetchDefenses 参数形态保留。
- it5（不变量）：令牌三锚点逐字保留，DefensesView 零「--r94-」。
- it6（不变量）：对照 BattlesView 的 resetFilters 范式仍存在（函数体含 page.value = 1 与 load()）。

## §5 RED 定义（实测）
命令（cwd=laiyipao/admin）：
\`\`\`
pnpm exec vitest run tests/R94DefenseEmptyCta.contract.test.ts --reporter=verbose
\`\`\`
exit=1。原始输出末尾逐字（含 failed/passed 计数）：
\`\`\`
 × tests/R94DefenseEmptyCta.contract.test.ts > R94 DefensesView 空态 CTA 与 reason 一一对应契约 > it1 resetFilters 函数存在且含四个清零赋值 + page=1 + load() 4ms
   → 必须新增 function resetFilters(): expected '' not to be '' // Object.is equality
 × tests/R94DefenseEmptyCta.contract.test.ts > R94 DefensesView 空态 CTA 与 reason 一一对应契约 > it2 空态 CTA 文案含「清空筛选」且 @click=resetFilters 2ms
   → CTA 文案必须改为「清空筛选」: expected '            <el-button class="empty-s…' to contain '清空筛选'
 × tests/R94DefenseEmptyCta.contract.test.ts > R94 DefensesView 空态 CTA 与 reason 一一对应契约 > it3 reason 含「清空筛选」且 CTA 兑现承诺（一一对应） 1ms
   → CTA 必须兑现「清空筛选」: expected '            <el-button class="empty-s…' to contain '清空筛选'
 ✓ tests/R94DefenseEmptyCta.contract.test.ts > R94 DefensesView 空态 CTA 与 reason 一一对应契约 > it4 loadSeq 竞态守卫与 fetchDefenses 参数不变量保留 0ms
 ✓ tests/R94DefenseEmptyCta.contract.test.ts > R94 DefensesView 空态 CTA 与 reason 一一对应契约 > it5 令牌三锚点保留 + DefensesView 零 --r94-（不变量） 0ms
 ✓ tests/R94DefenseEmptyCta.contract.test.ts > R94 DefensesView 空态 CTA 与 reason 一一对应契约 > it6 对照 BattlesView resetFilters 范式未回退（不变量） 0ms

 Test Files  1 failed (1)
      Tests  3 failed | 3 passed (6)
   Start at  18:10:54
   Duration  729ms (environment 90%, transform 5%, import 2%, tests 1%, setup 1%, worker 1%)
\`\`\`
判读：it1/it2/it3 为真缺口失败，it4/it5/it6 不变量通过，无解析 / 模块 / 超时错误。另以 \`pnpm exec tsc --noEmit tests/R94DefenseEmptyCta.contract.test.ts --module esnext --moduleResolution bundler --target es2022 --types node,vitest/globals --skipLibCheck\` 独立校验语法，RC=0。

## §6 验收
- RED：cd laiyipao/admin && pnpm exec vitest run tests/R94DefenseEmptyCta.contract.test.ts --reporter=verbose → exit=1，it1 / it2 / it3 failed（真缺口），it4 / it5 / it6 passed（不变量），无解析 / 模块 / 超时错误。
- 实施轮 GREEN 后应为 6/6；且 PageSkeleton / R4FourViews 等前序契约与全量测试不回退。
- hygiene：HEAD 未变 b4bedf83944d41ce142cd2dfd5191ef516610540；非 ?? 项白名单仅 .gitignore（队长预置）；auth 四路径 ABSENT。
