# R81 规范：skills 收集进度条一致性美化

## §0 背景与决策

- 任务：t176 Attempt1，attempt_id=1ab6d8db-3835-46fd-9e7b-84df85a26ae6。基线 HEAD=74a8952236797a52104c248da21e229925bbe0ea。
- 性质：纯规范 + test-first RED，只写本 SPEC 与契约两文件；不改业务源码（skills.vue 留待实施轮），不写真值，不写 git，不碰令牌三锚点。
- 来源：R80-A 侦察报告 docs/laiyipao/recon/ROUND80_A_COLLECTION_PROGRESS_RECON.md 发现同构 KPI 图鉴页进度展示一致性差异——equipment/gems/reactions/mastery 均有 r3-progress-row 单条收集进度条，唯 skills 页 KPI 区后直接跳筛选行，无进度条。
- 队长裁定立 R81 一致性美化：skills 页补 r3-progress-row 收集进度条，复用同仓权威范式。理由：G66 覆盖矩阵（元素×类型）答「哪些类别缺」，进度条答「整体完成度%」，二者互补；改动极小零风险；符合美化玩法升级目标。

## §1 权威口径（队长已读源码坐实，逐字复刻勿自造）

- 权威模板 equipment.vue:34-38：
  <view class="r3-progress-row is-ai r3-enter"><text class="r3-progress-label">收集进度</text><view class="r3-progress"><view class="r3-progress-fill" :style="{ width: r3EquipPct + '%' }" /></view><text class="r3-progress-num">已拥有 {{ ownedCount }}/{{ items.length }}</text></view>
- skills 版差异：不用 is-ai（非 AI 专属），用 class="r3-progress-row r3-enter"；文案「收集进度」+「已解锁 {{ ownedCount }}/{{ skills.length }}」（skills 语义是解锁非拥有，与 skills.vue:45「已解锁 / 技能总数」一致）。
- 权威 computed equipment.vue:412：const r3EquipPct = computed(() => items.value.length ? Math.round(ownedCount.value / items.value.length * 100) : 0)。
- skills 版：const r3SkillPct = computed(() => skills.value.length ? Math.round(ownedCount.value / skills.value.length * 100) : 0)，与 ownedCount(skills.vue:486-488) / skills 同源。
- common.css:1308-1315 已有 .r3-progress / .r3-progress-fill / .r3-progress-row 全套样式，无需新增 CSS。
- 插入位置：skills.vue KPI 区 </view>（:51）之后、r3-filter（:52）之前。

## §2 精确改点（实施轮执行，本轮不改）

1. skills.vue KPI 区（:51 </view>）后插入一条进度行：r3-progress-row r3-enter + r3-progress-label「收集进度」+ r3-progress/r3-progress-fill 绑定 r3SkillPct 宽度 + r3-progress-num「已解锁 {{ ownedCount }}/{{ skills.length }}」。
2. script setup 加 r3SkillPct computed，与 r3EquipPct 同构（分母 skills.value.length、分子 ownedCount.value、Math.round、空列表 :0 兜底）。
3. 零 CSS 改动、零令牌改动、零 API/data 改动。

## §3 不变量（契约逐字守卫）

- G66 元素×类型覆盖矩阵（skills.vue:166-184 skill-matrix / covered: cell.owned > 0 / 每格 owned/total / 标题「元素 × 类型覆盖」）必须保留，不被进度条替代。
- ownedCount computed（:486-488）语义与实现不变（仍按 ownedSkillIds.value.has(s.id) 过滤）。
- :164「个已解锁（背包可查看拥有状态）」文案、筛选 chip（:52-56 r3Element/r3-chip active）、搜索占位（:159）、结果计数（:186）全部不动。
- 令牌三锚点（tokens.css:14 --lyp-token-version: lyp-tokens-v1 / :15 --lyp-r3-version: lyp-tokens-v3-r3 / :16 --lyp-r6-version: lyp-tokens-r6）不动。
- R61/R55 等既有测试不变量不动；业务语义零改（不改任何 computed 之外的逻辑、不改 API、不改 data）。
- 零令牌改动：不碰 tokens.css / common.css（样式已存在）。

## §4 契约清单

- 文件：laiyipao/miniapp/src/styles/r81SkillCollectionProgress.contract.test.ts（6 it，fs.readFileSync 真实读取 skills.vue / equipment.vue / tokens.css，禁内嵌源码副本，blockAfter 大括号配对提取函数块）。
- it1：r3SkillPct 存在且与 r3EquipPct 同构（computed / skills.value.length / ownedCount.value / Math.round / :0 兜底）。
- it2：模板 r3-progress-row 存在且绑定 r3SkillPct、含 r3-progress-fill、文案「收集进度」+「已解锁」+ skills.length、无 is-ai、含 r3-enter。
- it3：插入位置在 KPI 区之后与 r3-filter 之前（顺序断言）。
- it4：不变量 G66 skill-matrix / covered / cell.owned/total / 标题保留。
- it5：不变量 ownedCount computed 原样 + :164 解锁文案 + 筛选 + 搜索 + 结果计数保留。
- it6：令牌三锚点保留 + 零 r81 令牌 + 写域边界白名单。
- 正则纪律：不要求 JS 非法语法（连字符裸 key 允许可选引号），避免历史 t138 教训。

## §5 RED 验证结果

- 命令：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r81SkillCollectionProgress.contract.test.ts --reporter=verbose。
- 实测：exit=1（vitest v4.1.12），Test Files 1 failed (1)，Tests 3 failed | 3 passed (6)，Duration 339ms。
- 失败恰为缺口断言：
  1. it1「skills 必须定义 r3SkillPct computed: expected undefined to be truthy」（契约 :43）。
  2. it2「skills KPI 区后必须有 r3-progress-row: expected -1 to be greater than -1」（契约 :56）。
  3. it3「进度条必须位于 KPI 与筛选行之间: expected -1 to be greater than 2018」（契约 :74）。
- 通过为不变量断言：it4 G66 矩阵保留、it5 ownedCount/解锁文案/筛选/搜索/计数保留、it6 令牌三锚点/零 r81 令牌/写域边界。
- 无解析 / 模块 / 超时错误：缺口失败与预期完全一致，RED 有效。

## §6 诚实边界

- 本轮仅规范与契约，未改 skills.vue；进度条真实渲染与百分比填充留待实施轮及真浏览器复查验证。
- 无后端 API 环境，ownedCount 常为 0，进度条填充为 0% 属预期；非零拥有量下的百分比显示未验证。
- RED 日志落在 docs/laiyipao/red-r81.log（UNTRACKED），供实施轮对照。
- 契约对模板结构作文本断言而非运行时渲染断言，实施轮须真浏览器复核进度条可见且零横向溢出。