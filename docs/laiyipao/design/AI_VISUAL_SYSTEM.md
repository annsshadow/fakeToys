# 来一炮 · AI 风格视觉设计规范（V1 · 唯一真源）

> **拍板日**：2026-11-21。**状态**：生效，作为 laiyipao 全端视觉改造的**唯一依据**。
>
> **地位**：本规范定义 laiyipao（H5/miniapp + PC/admin）的视觉语言与令牌契约。
> 任何视觉改动——组件、页面、图表、动效——都必须能追溯到本文的某一条；
> 与本文冲突的既有样式按本文收口（见 §10 反模式清单）。
> 令牌的**代码真源**是 `laiyipao/miniapp/src/styles/` 与 `laiyipao/admin/src/styles/`；
> 本文是它们的**设计契约与裁决依据**，两者必须逐项对齐（见 §8）。
>
> **依据**：本文所有色值、对比度、类名均来自 2026-11-21 对真实代码与页面的实测，
> 不是凭空设定。对比度按 WCAG 2.1 sRGB 相对亮度公式实算（与
> `laiyipao/admin/tests/textContrast.test.ts` 同算法），数值可复算。
>
> **保密**：文档中的部署地址一律用 `<YOUR_TEST_DOMAIN>` 占位；真值只从
> `laiyipao/deploy/.local.keys` 与环境变量读取，不入库。
>
> **R3 追加**：§1B 定义 16 个次级页面的六组件模式与肉眼可辨 AI 感规则；
> 令牌包标记 `lyp-tokens-v3-r3`（`--lyp-r3-version`），旧 `lyp-tokens-v1` 仍为兼容锚点。
>
> **R4 追加**：§1C 完成 H5 十页与 admin 四视图的组件映射与轻量 AI 规则；不新增令牌，v1/v3 双锚点保持。

---

## 现量复核（2026-11-21）

> 本规范为新增文档，无历史正文；下表是拍板时的代码基线，供后续漂移比对。

| 项 | 实测现量 | 依据 |
|---|---|---|
| miniapp 主题真源 | `miniapp/src/styles/tokens.css`（令牌唯一真源），`common.css` 只保留跨页复用样式 | t4 已落地 `lyp-tokens-v1` |
| admin 主题真源 | `admin/src/styles/tokens.css`（令牌唯一真源），`main.css` 只保留 Element Plus 映射与共享类 | t4 已落地 `lyp-tokens-v1` |
| 五元素主题色 | **火 fire / 冰 ice / 雷 lightning / 腐蚀 corrosion / 动能 kinetic**（对应 I-1 元素反应机制） | `common.css:40-51`、`main.css` 元素段 |
| 双端元素色是否一致 | 色值一致（#ff6b35/#58a6ff/#ffd33d/#7ee787/#c9a7ff），但**前缀不同**（mini 无前缀 / admin `--el-`） | 同上 |
| 对比度守卫 | 双端均有（文本四阶 ≥4.5:1、焦点环、层级递减）+ 双端令牌契约 | `admin/tests/textContrast.test.ts`、`focusVisible.contract.test.ts`、`designTokens.contract.test.ts`、`miniapp/src/styles/tokens.contract.test.ts` |
| 玻璃拟态 | t4 已在双端令牌层定义 `--glass-*` / `--lyp-glass-*`（panel 底 + 提亮文字）；基础类由下游落地 | 两端 tokens.css |
| 命令面板 | 双方均无 | 全量 grep 无 command/palette/Ctrl+K 实现 |
| 骨架屏 | mini 有 `.skeleton-row/.skeleton-bar`（shimmer 动画）；admin 用 `v-loading` 遮罩 | `common.css`「X 批 R32」段 |

---

## 0. 为什么要统一

现状不是「没有设计」，而是「有两套各自成立、彼此不通的设计」：miniapp 用 `--fire`，
admin 用 `--el-fire`；t4 前 miniapp 的 dim 是 `#8b949e`（panel-3 上 4.56:1），
admin 是 `#96a0ad`（5.41:1）。t4 已把双端 dim 统一到 `#96a0ad`、muted 统一到 `#9aa5b2`，
并把共同契约固化为双端 `tokens.css` + 自动守卫。新增元素色必须双端登记并跑契约测试。

本规范的目标：**一个游戏，一套视觉语言，一处定义，两端消费**。

---

## 1. 设计支柱（八条，按优先级）

| # | 支柱 | 一句话 | 落地位置 |
|---|---|---|---|
| P1 | **深色科幻底** | 近黑蓝底 + 分层表面，靠明度差和微光建立纵深，不靠亮色块 | §2.1 |
| P2 | **五元素主题色** | 火/冰/雷/腐蚀/动能五个高辨识色，是全局唯一的元素语义 | §2.2 |
| P3 | **克制玻璃拟态** | 只在浮层/命令面板/顶栏用半透明+模糊，正文卡片保持实色 | §3 |
| P4 | **微交互** | 按压、聚焦、入场、结果反馈短促且可关闭，反馈先于装饰 | §4 |
| P5 | **命令面板** | PC 端 Ctrl/Cmd+K 全局跳转，H5 端不模仿物理键盘操作 | §5 |
| P6 | **骨架屏** | 加载态用形状占位而非孤零零的「加载中」，先给结构再给数据 | §6 |
| P7 | **PC/H5 响应式** | 同一令牌、两套版式；PC 宽屏多列，H5 限宽单列 + 安全区 | §7 |
| P8 | **可见 AI 感**（R2 新增） | 首屏必须出现带 AI 标识、有数据依据、可点击的智能洞察，不做假聊天 | §1A |

**优先级规则**：可访问性（§2.4）> 功能可发现性（P5/P6）> 视觉表现（P1/P2/P3）。
当「酷」与「可读」冲突时，永远让可读赢——这是硬规则，不是偏好。

## 1A. AI 感支柱（P8 · R2 新增，F-10-2 整改唯一依据）

> 依据：t10 终审 F-10-2【high】——12 张改造后真浏览器截图无任何 AI 助手入口、智能洞察/构筑建议卡、
> 生成式视觉或 AI 语义交互；原 P1–P7 无一条定义 AI 感。本章把「AI 风格」从形容词变成可验收的组件规格。

### 1A.1 「可见 AI 元素」的判定（四条同时满足才算）

1. **肉眼可辨**：首屏（H5 390×844 视口 / admin 1440×900 视口）不滚动即可看到，且含可阅读的「AI」汉字标识；只有渐变光晕、粒子或抽象装饰**不算**。
2. **有据可查**：结论必须由当前页面已取到的真实数据经确定性规则推出，并显示「依据 + 数据时间/范围」；禁止写死文案冒充智能。
3. **可行动**：带一个明确 CTA（跳转补强页 / 战报 / 关卡），点击后到达与结论对应的页面；不能只是静态标语。
4. **诚实标注**：标注「规则引擎 · 基于本地/服务端数据」，不宣称 GPT/大模型/实时学习，不编造置信度或数值。

优先级：可访问性（§2.4/§9）> 真实数据 > 视觉表现。AI 卡不得挤压核心玩法信息，也不得用假数据填充空态。

### 1A.2 H5 首页：AI 战术洞察卡（t13 必须落地）

- **位置**：`miniapp/src/pages/index/index.vue`，资源顶栏之后、公告卡之前；首屏第一张内容卡。
- **尺寸**：整卡高 ≥180rpx（约 90 CSS px），宽度撑满 `.pad` 内容区；左侧 6rpx 竖向 `--ai-accent` 色条 + `--ai-glow` 微光，卡底 `--panel-2`、边框 `--border`。
- **结构（自上而下）**：
  1. 标识行：带边框的「AI」文字徽章（不是纯图标）+「AI 战术洞察」标题 +「规则引擎 · 基于当前构筑」副标（`--muted`）。
  2. 结论行（1 行，`--text`，≥`--fs-base`）：由 `store.buildRating.dims` 中最弱一维 + `marginalGain` 推出，例如「{{短板}}仅 {{pct}}%（权重 {{weightPct}}%）：优先补它，收益 {{gain}}」。数值必须来自现有 store，不得写死。
  3. 依据行（`--text-2`）：「依据：五维评分 / 补分边际收益 · 更新于本次进入」；可折叠，但没有数据时也要显示智能空态（§1A.4）。
  4. CTA：`去补强 →`，绑定该维度已有的 `hint` 路由；触控区 ≥88rpx × 88rpx，`:focus-visible` 用全局焦点环。
- **数据不足时**：显示「AI 暂时没有足够构筑数据，先打一场战斗吧」+「去战斗」CTA，不显示旧结论、不显示空卡。
- **一致性**：只消费 `store` 已有字段，不新增后端接口；卡片文字不重复 radar 图的数值堆砌，只讲一个结论。

### 1A.3 admin 看板：AI 运营洞察卡（t14 必须落地）

- **位置**：`admin/src/views/DashboardView.vue`，KPI 卡片行之前、页面标题之后；宽屏占满内容区，≤900px 窄屏叠为单列。
- **尺寸**：高 88–112px；两栏（约 `2fr : 1fr`）：左结论 + 依据，右 CTA 与数据范围。
- **结构**：
  1. 标识行：「AI」文字徽章 +「AI 运营洞察」标题 +「规则引擎 · 数据截至 {{最近一天日期}}」。
  2. 结论：从 `data.stage_funnel` 最大跌幅关卡 / `data.reaction_usage` 最高频反应 / `data.daily_active` 首尾变化中按确定性规则选**一条**最重要结论，例如「第 {{N}} 关流失最大（尝试 {{a}} → 通关 {{b}}），建议优先复核难度」。
  3. 依据行（`--lyp-muted`）：「依据：关卡漏斗 / 反应分布 / 近 14 日活跃」，附可展开的数据表（复用已有 `describeDashboardChart` 等价文本，WCAG 1.1.1）。
  4. CTA：`查看战报 →` 或 `复核该关卡 →`，路由必须真实存在；空数据时给智能空态而不是空卡。
- **加载态**：用已有 `.skeleton-row` / `.skeleton-bar` 保持卡片高度，避免 CLS；`aria-busy="true"`。
- **ECharts 令牌化协同**：图表色改从 `getComputedStyle` 读 `--lyp-dim` / `--lyp-border`（F-10-4），AI 卡与图表必须同色阶，不得各写一套硬编码。

### 1A.4 智能空态文案（双端共用的写法）

| 场景 | 禁止 | 必须写 |
|---|---|---|
| 无足够数据 | 「暂无数据」 | 「数据不足：AI 洞察需要至少 3 天活跃记录，当前 {{n}} 天」+ 说明缺什么 |
| 卸载/未登录 | 「加载失败」 | 「AI 洞察需要登录后读取你的构筑，登录后即可生成」+ 登录 CTA |
| 规则无结论 | 空白卡片 | 「当前构筑五维均衡，没有明显短板；继续保持」+「查看构筑」CTA |

空态卡高度与正常态一致；文案以「数据不足的原因 + 下一步动作」为结构，不得用「AI 思考中…」掩盖没有数据。

### 1A.5 语义命令入口（admin 专属，H5 不模仿键盘）

- 命令面板（`Ctrl/Cmd+K`，§5）在 `groups` 单一源中新增 `AI 洞察` 分组，命令项：`查看今日运营洞察`、`复核流失最大关卡`、`跳转到元素反应分布`；关键词支持「洞察 / 智能 / 分析」语义词，不新增一套硬编码路由表。
- 选中洞察命令后关闭面板、滚动到 AI 卡并给卡内 CTA 焦点；面板仍需焦点陷阱 + Esc + 焦点归还（§9.6）。
- H5 无物理键盘：首页顶栏放一个文字「AI 洞察」胶囊，锚点到洞察卡；不做假的聊天输入框、不伪装成可对话模型。

### 1A.6 令牌与视觉规格（唯一真源：双端 `tokens.css`）

| 语义 | miniapp | admin | 值 / 用途 |
|---|---|---|---|
| AI 标识色 | `--ai-accent` | `--lyp-ai-accent` | `#c9a7ff`（动能紫），仅徽章边框、左侧色条、图标描边 |
| AI 文字色 | `--ai-ink` | `--lyp-ai-ink` | `#d7bfff`，叠 18% kinetic soft + panel-3 仍 5.90 / 6.04:1；soft 底上的 AI 文字必须用此令牌 |
| AI 微光底 | `--ai-halo` | `--lyp-ai-halo` | `rgba(201,167,255,0.14)`，仅装饰光斑，禁止作为承载正文的背景 |
| AI 光晕 | `--ai-glow` | `--lyp-ai-glow` | `0 0 24rpx / 24px rgba(201,167,255,0.28)`，只给卡片外缘，`prefers-reduced-motion` 下静止 |

文字始终使用文本四阶（`--text/-text-2/-muted/-dim`）；`--ai-accent` 只做非文字装饰或边框，
**不得**把元素 soft 叠 panel-3 当 AI 卡背景（§2.4 已实测 fire 3.89 / ice 4.04 不达标）。

### 1A.7 可访问性与验证门禁

- 对比度：标题/结论 ≥4.5:1；徽章「AI」是文字而非仅颜色（WCAG 1.4.1）；CTA 键盘可达且焦点可见（2.4.7）；H5 触控 ≥88rpx（2.5.5）。
- 动态：光晕最多 1 次入场（`--motion-base`），不循环「思考中」动画；`prefers-reduced-motion: reduce` 下全部静止；骨架屏结束后立即替换为真实结论。
- 截图门禁（F-10-2 复审硬条件）：H5 首页与 admin 看板各交 1 张真浏览器截图，AI 卡须位于首屏、标识文字在 100% 缩放下可辨；截图文件标注「非物理真机」，并附所用视口尺寸。
- 回归门禁：双端令牌契约测试与对比度守卫保持全绿；AI 令牌只加一端、图表硬编码旧色回潮、空态写成「暂无数据」均视为不合格。

---

## 1B. 次级页面规范（R3 · 16 页结构性升级唯一依据）

> 依据：R2 终审（docs/laiyipao/reviews/ROUND2_DESIGN_REVIEW.md）确认 H5 仅首页/关卡/战斗三页完成结构性升级，
> 其余 16 页仍是基线简陋态（专精页纯规则文字+灰条、商城页文本标签堆砌、宝石图鉴近乎空白）；用户诉求原文「H5端也是各个页面都太简陋了，都可以改」。
> R1 失败教训必须记住：`.page-root:before` 的 rgba(255,255,255,.022) 网格 + rgba(...,.06) 辉光在 390×844 真浏览器截图上肉眼不可辨。
> **本轮禁止把低透明度底纹、微弱渐变或换色当作改造成果或 AI 感**；改造成效以结构性组件 + RED 契约 + 真浏览器截图为准。

### 1B.1 范围与页面—模式映射

适用 19 页中除 index / stage / battle 外的 16 页。每页至少落地「一个分区眉题或指标带」+「一个数据容器（DataGrid / ProgressTree）」+「一个智能空态或 AI 线索」。
只改颜色、只加底纹、只调字号不算完成。

| 页面（`miniapp/src/pages/`） | 必备组件模式 | AI 线索（必须取现有数据） |
|---|---|---|
| `mastery.vue` 专精 | SectionKicker + KpiStrip + ProgressTree + EmptyState | AI 推荐节点：依据已点亮节点与剩余专精点，给一条「先补哪一系」结论 + 跳转 CTA |
| `shop.vue` 商城 | KpiStrip + FilterBar + DataGrid + EmptyState | AI 性价比提示：依据持有货币与价格给一条推荐商品 + 购买 CTA |
| `gems.vue` 宝石图鉴 | SectionKicker + KpiStrip + ProgressTree + EmptyState | AI 收集缺口：依据已拥有/总数指出最缺的一颗 + 获取 CTA |
| `bag.vue` 背包 | KpiStrip + FilterBar + DataGrid + EmptyState | AI 整理建议：依据道具数量与过期/稀缺状态给一条动作 |
| `battles.vue` 战报 | KpiStrip + FilterBar + DataGrid + EmptyState | AI 战报诊断：依据近 N 场胜率给一条改进结论 |
| `equipment.vue` 装备 | KpiStrip + FilterBar + DataGrid + EmptyState | AI 搭配建议：依据已装备部位与契合度给一条动作 |
| `rank.vue` 排行 | KpiStrip + FilterBar + DataGrid + EmptyState | AI 冲榜建议：依据当前排名与分差给一条动作 |
| `reactions.vue` 反应图鉴 | SectionKicker + FilterBar + DataGrid | AI 反应推荐：依据当前构筑元素给一条克制建议 |
| `skills.vue` 技能 | SectionKicker + FilterBar + DataGrid | AI 构筑建议：依据已选技能与热量给一条取舍建议 |
| `enemies.vue` 敌人图鉴 | SectionKicker + FilterBar + DataGrid | AI 克制提示：依据元素分布给一条上阵建议 |
| `announcements.vue` 公告 | SectionKicker + DataGrid + EmptyState | AI 摘要：取最新一条公告标题做一句话摘要，不编造内容 |
| `signin.vue` 签到 | SectionKicker + KpiStrip + DataGrid + EmptyState | AI 提醒：依据连续签到与缺口给一条领取动作 |
| `skins.vue` 皮肤 | KpiStrip + FilterBar + DataGrid + EmptyState | AI 换装建议：依据已拥有皮肤给一条装备动作 |
| `verify.vue` 验真 | KpiStrip + DataGrid + EmptyState | AI 异常提示：依据异常/守恒计数给一条复核动作 |
| `me.vue` 我的 | KpiStrip + DataGrid + EmptyState | AI 成长诊断：依据关卡与构筑评分给一条补强动作 |
| `login.vue` 登录（例外） | SectionKicker + EmptyState | **不强制 AI**：登录页无用户数据，编造即违反 §1A.1「有据可查」；只要求空态文案与焦点环，登录后落地页必须满足 AI 判据 |

### 1B.2 六个可复用组件模式

命名统一用 BEM 风格 class；H5 在 `.card` / `.chip-tabs` / `.list-row` / `.state-block` 上追加 modifier，
admin 在 `.section-title` / `.toolbar` / `.kpi-*` / `.glass-surface` 上追加 modifier，不另起平行样式体系。

#### 1B.2.1 SectionKicker（分区眉题）

- 结构：中文眉题 + 可选英文小标 + 可选说明；`.section-kicker--ai` 变体追加文字徽章「AI」。
- H5：中文 24rpx/600 `--muted`，英文 20rpx `--dim` + `letter-spacing: .12em`；上边距 24rpx、下边距 12rpx；左侧 4rpx `--accent` 竖条。
- admin：中文 15px/600 `--lyp-muted`，英文 11px `--lyp-dim` + `.12em`；上边距 20px、下边距 8px。
- 可访问性：中文眉题是主标题文本，英文只作装饰补充；AI 变体的「AI」必须是可朗读汉字，不是纯图标或伪元素。

#### 1B.2.2 KpiStrip（主/次两级指标带）

- 每页**最多 4 张主 KPI**；主卡带 6rpx/3px 左强调条（`--accent`），次卡无强调条。禁止 R1 的「8 张等权卡平铺」。
- H5：2 列 grid，gap 12rpx；主卡跨 2 列、最小高 96rpx，次卡最小高 80rpx；数值 `--fs-xl` + `font-variant-numeric: tabular-nums`；标签 `--muted`、副文 `--dim`。
- admin：4 列 grid（≤900px 降为 2 列），主/次各一组，卡高 88px；数值 `--lyp-fs-xl`。
- 加载态用 `.skeleton-row` 占位且保持同高（`aria-busy="true"`），骨架结束立即换真实数值，避免 CLS。

#### 1B.2.3 EmptyState（空数据态）

- 固定结构：图形槽 + 标题 + 原因 + 下一步动作按钮。H5 最小高 240rpx、admin 180px，与正常态等高。
- 文案强制套 §1A.4：写清「缺什么数据/为什么空 + 下一步做什么」。通用写法：「还没有 {对象}：{原因}，{动作}」。
- **禁止**裸「暂无数据」「加载失败」「-」；AI 空态禁止「AI 思考中…」伪装计算。
- AI 变体：图形槽用 `--ai-halo` 作底 + `--ai-accent` 1px 边框 + 汉字「AI」标签（不得只靠 halo 表示 AI）；按钮复用页面已有路由。
- 触控按钮 H5 ≥88rpx、admin ≥44px；焦点环走全局 `:focus-visible`。

#### 1B.2.4 DataGrid（表格/列表）

- H5 ≤560px：表格转成卡片行（`.list-row` + 标签/值成对），表头信息用 `aria-label` 保留，不做横向滚动的窄表。
- admin：真 `<table>`，表头 `--panel-2` + 1px `--border-soft`，行 hover `--panel-3`（muted 文字对该底 5.30:1 仍达标）。
- 数字列 `tabular-nums`；每行至少一个文字标识（WCAG 1.4.1）；空数据必须切到 EmptyState，不留空表头。
- 加载态给 3 行 `.skeleton-row`，高度与真实行一致。

#### 1B.2.5 FilterBar（筛选区）

- H5：横向滚动 chip，`min-height: 88rpx`、gap 12rpx；admin：`.toolbar` 最多两行，窄屏自动折行。
- 活动态：`background: var(--accent-soft)` + `color: var(--accent-ink)`。实算：accent-ink 在 soft+panel-2 为 5.20/5.23:1，soft+panel-3 为 4.63/4.67:1。
- **禁止** `color: var(--accent)` 叠 `--accent-soft`：实算仅 4.37/4.40:1，未过 4.5 红线（禁止组合扫描要覆盖这一条）。
- 结果数用 `.result-count` 文本实时更新并加 `aria-live="polite"`；筛选后焦点不得丢失。

#### 1B.2.6 ProgressTree（专精树/收集进度）

- 状态机：`locked / available / selected / ai-recommended`，每态必须带文字标签「未解锁 / 可选 / 已点亮 / AI 推荐」，不得只换颜色。
- 连接线 `--border`，节点底 `--panel-2`，selected 边框 `--accent`，ai-recommended 边框 `--ai-accent` + 汉字「AI」+ 一行依据文案。
- 专精遵循 8 系 × 3 层 × 4 节点（每层 4 选 2，GAME_DESIGN §I-3）；H5 按系做纵向手风琴（一次展开一层），admin 4 列网格。
- 进度条轨道用 `--progress-track` + 1px `--border`，旁边必须有数字文本「已点亮 n/12」；颜色不是唯一信息载体。
- 节点触控区 H5 ≥88rpx；点击后所给反馈（点亮/不可点原因）用文字而非仅动效。

### 1B.3 PC / H5 双端适配规则

| 规则 | H5（390×844，限宽 560px） | admin（1440×900，≤900px 折行） | 双端硬约束 |
|---|---|---|---|
| 信息密度 | 单列卡片，主 KPI 跨 2 列 | 4 列 KPI + 表格 | 同一令牌、同一文案结构 |
| 交互载体 | chip / 手风琴 / 卡片行 | toolbar / 表格 / 网格 | 不用 hover 承载唯一信息 |
| 触控/焦点 | 按钮与节点 ≥88rpx；全局 3rpx 焦点环 | 控件 ≥44px；2px 焦点环 | 焦点环都带 offset 且键盘可达 |
| AI 标识 | 汉字「AI」徽章 ≥22rpx | 汉字「AI」徽章 ≥11px | 文案同源，不写死两套结论 |
| 动效 | 入场 240ms，reduced-motion 静止 | 同左 | 不循环「思考中」动画 |

### 1B.4 AI 感落地（肉眼可辨，不是低透明度底纹）

1. 每个内容页至少一处满足 §1A.1 四判据的 AI 线索：可见「AI」汉字 + 由当前页已有数据经确定性规则得出的结论 + 依据 + 可点击 CTA；无数据时走 §1A.4 智能空态。
2. 允许载体：`.section-kicker--ai`、KpiStrip 的结论副行、`.empty-state--ai`、`.progress-node--ai`。只有一个 kicker 时，同页 KpiStrip/EmptyState 必须提供对应 CTA。
3. 不得计入 AI 感：`.page-root:before` 的 .022 网格、.06 辉光、渐变光斑、纯图标、需要 hover 才出现的提示、只有英文「AI」没有结论、写死文案。
4. 视觉下限：AI 徽章描边用 `--ai-accent`，徽章文字用 `--ai-ink`（叠 kinetic soft + panel-3 为 5.90/6.04:1）；正文仍用文本四阶，soft 不得叠 panel-3 承载 muted/dim。
5. 诚实标注：写「规则引擎 · 基于当前数据」；不宣称 GPT/大模型/实时学习，不编造置信度。

### 1B.5 test-first 契约断言方式（RED → GREEN）

本规范不代替测试。页面改造前先由 h5-ui（H5 页面）与 qa-e2e（双端契约）在各自写域新增以下合约并执行到 **RED**；
页面代码改动后重跑到 **GREEN**。禁止为了让测试变绿而删断言或放宽阈值。

```ts
// 双端令牌对齐（admin/tests/r3Tokens.contract.test.ts 示意）
expect(M["--lyp-r3-version"]).toBe("lyp-tokens-v3-r3");
expect(A["--lyp-r3-version"]).toBe("lyp-tokens-v3-r3");
expect(M["--accent-soft"]).toBe(A["--lyp-accent-soft"]);
expect(M["--progress-track"]).toBe(A["--lyp-progress-track"]);
```

```ts
// 页面结构（miniapp/src/styles/r3SecondaryPages.contract.test.ts 示意）
const SECONDARY = ["announcements","bag","battles","enemies","equipment","gems",
  "login","mastery","me","rank","reactions","shop","signin","skills","skins","verify"];
for (const page of SECONDARY) {
  const src = read(join(pagesDir, page + ".vue"));
  expect(src, page + " 必须有分区眉题或指标带").toMatch(/section-kicker|kpi-strip/);
  expect(src, page + " 必须有数据容器").toMatch(/data-grid|progress-tree|list-row/);
  expect(src, page + " 不得裸空态文案").not.toMatch(/暂无数据|加载失败/);
  if (page !== "login") expect(src, page + " 必须有可见 AI 线索").toMatch(/AI[\s\S]{0,40}(依据|建议|提示|诊断)/);
}
```

补充断言：FilterBar 活动态源码不得出现 `color: var(--accent)` 叠 `--accent-soft`（改用 `--accent-ink`）；
ProgressTree 必须同时出现 `--progress-track` 与数字标签模板；EmptyState 必须含动作按钮与 `aria-live`/`role=status` 之一。
截图门禁：h5-ui 至少交专精、商城、宝石图鉴、公告 4 张 390×844 真浏览器截图（标注「非物理真机」），由 design-reviewer 按 §1B.4 肉眼复核。

### 1B.6 R3 令牌（唯一真源：双端 tokens.css）

| 语义 | miniapp | admin | 值 / 用途 |
|---|---|---|---|
| R3 规范包标记 | `--lyp-r3-version` | `--lyp-r3-version` | 双端同值 `lyp-tokens-v3-r3`；旧 `--lyp-token-version` 仍为 `lyp-tokens-v1` 兼容锚点 |
| 筛选活动底 | `--accent-soft` | `--lyp-accent-soft` | `rgba(255,107,53,.18)`；仅叠 panel/panel-2，文字必须 `--accent-ink` |
| 进度轨道 | `--progress-track` | `--lyp-progress-track` | `#39465a`；双端同值，状态必须另配数字文本 |

兼容策略说明：现有双端契约把 `--lyp-token-version` 钉在 `lyp-tokens-v1`，本任务写域不含测试文件，
因此新增独立 R3 标记而不改旧字段，避免既有守卫误红；待测试 owner 统一迁移后再提升旧字段。

---

## 1C. R4 剩余页面设计规范与组件映射

> 依据：R3（§1B）已覆盖专精/商城/宝石图鉴等 16 个次级页的结构性升级；R4 聚焦此前仍未升级的 H5 十页与 admin 四视图。
> 本节不另起视觉语言：组件模式沿用 §1B 的 SectionKicker / KpiStrip / EmptyState / DataGrid / FilterBar / ProgressTree，
> AI 判据沿用 §1B.4 四条——可见汉字「AI」+ 真实数据结论 + 依据 + 可行动 CTA。
> R1 教训继续生效：`.page-root:before` 的 rgba(255,255,255,.022) 网格与 rgba(...,.06) 辉光肉眼不可辨，
> 低透明度网格/光斑/渐变一律**不算** AI 感，也不得当作改造成果。本节不新增令牌，双端 `lyp-tokens-v1` / `lyp-tokens-v3-r3` 双锚点保持不变。

### 1C.1 H5 十页组件映射

展示顺序 = 组件模式顺序；每页至少一处满足 AI 四判据的线索，空数据走 §1A.4 智能空态。
真实数据源均来自各页现有 store / 接口 / 配置，不得编造、不得写死结论。

| 页面 | 组件模式（按顺序） | AI 线索（数据源 → 结论 → 依据） | CTA |
|---|---|---|---|
| `pages/announcements/announcements.vue` | SectionKicker → KpiStrip → FilterBar → DataGrid → EmptyState | 取 `items.length` 与最新一条标题，给「当前共 N 条公告，最新一条是 X」；加载失败走独立空态，不把失败说成空数据 | 查看最新一条 |
| `pages/bag/bag.vue` | KpiStrip → FilterBar → DataGrid → EmptyState | 取 4 个出战槽 `selected` 的元素种类数：<2 时提示「当前只带 1 种元素，多数反应链需要 2 种以上」（依据为页内既有说明） | 去技能槽 |
| `pages/battles/battles.vue` | KpiStrip → FilterBar → DataGrid → EmptyState | 取 `battleTotals.kills/leaked/avgScore` 与 `streak`，计算漏杀比并用文字分级，给一条改进结论；不宣称模型预测 | 看战报详情 |
| `pages/enemies/enemies.vue` | SectionKicker → FilterBar → DataGrid → EmptyState | 取服务端 /config 的克制排行与当前构筑元素，指出「该元素下最易伤的敌人类别」并列出抗性数值 | 切换对应元素 |
| `pages/login/login.vue` | SectionKicker → EmptyState（轻量 AI，见 §1C.3） | 读本机已缓存的最近战绩或空态提示，不读服务端、不新增账号步骤 | 复用已有登录按钮 |
| `pages/me/me.vue` | SectionKicker → KpiStrip → DataGrid → EmptyState | 取 `store.nickname`、当前关卡与构筑评分，给「优先补强哪一项」的结论与依据 | 去专精 / 商城 |
| `pages/rank/rank.vue` | KpiStrip → FilterBar → DataGrid → EmptyState | 取 `scoreDistribution` / `scoreBuckets` 与本人分数，算出与上一名的分差并文字说明 | 去冲榜关卡 |
| `pages/signin/signin.vue` | SectionKicker → KpiStrip → ProgressTree → DataGrid → EmptyState | 取 `claimedCount`、`cycleDays`、下一档奖励与 `offlineReward`，提示「下一档值得等几天 / 今天可先领什么」 | 领取今日奖励 |
| `pages/skins/skins.vue` | KpiStrip → FilterBar → DataGrid → EmptyState | 取 `rarityDonut`、`passiveElementMap` 与当前构筑元素，推荐一款被动流派并说明匹配理由 | 切换皮肤 |
| `pages/verify/verify.vue` | SectionKicker → DataGrid（轻量 AI，见 §1C.3） | 仅在 `replayInfo` 到位后给出三要素齐备检查与哈希口径说明；未到位不显示 AI 文案 | 复用本地重放按钮 |

### 1C.2 admin 四视图组件映射

admin 沿用 `.section-title` / `.kpi-*` / `.toolbar` / 真 `<table>` / `.glass-surface`，窄屏 ≤900px 折行，不引入第二套样式体系。

| 视图 | 组件模式（按顺序） | AI 线索（数据源 → 结论 → 依据） | CTA |
|---|---|---|---|
| `views/ContentView.vue` | SectionKicker → KpiStrip → FilterBar → DataGrid → EmptyState | 取 `annTotal/codeTotal/logTotal` 与审计操作类型计数，提示「近期发放/回收笔数异常」并列出计数；空表写明为什么空，不用裸「暂无记录」 | 切到审计 tab |
| `views/EconomyView.vue` | SectionKicker → KpiStrip → FilterBar → DataGrid → EmptyState | 取流水 `delta`（净流入）与订单筛选结果，净流入为正时提示「回收压力上升」并给出数值与口径；readonly 角色只给结论不显写入口 | 切到订单筛选 |
| `views/DefensesView.vue` | SectionKicker → KpiStrip → FilterBar → DataGrid → EmptyState | 取防线总数与挑战记录胜率/被窃取数，指出「最需要巡检的防线」并给出挑战次数 | 查看挑战记录 / 用户详情 |
| `views/EquipmentView.vue` | SectionKicker → KpiStrip → FilterBar → DataGrid → EmptyState | 取 equipment/gems/skins 三个目录的款数与 `ELEMENT_LABEL`/`SLOT_LABEL`，指出元素或部位覆盖缺口及具体款数 | 切到对应 tab |

### 1C.3 登录页与验真页的轻量 AI（不打断流程）

- `pages/login/login.vue`：轻量 AI = 登录卡片底部一行 `role="note"` 语义的确定性提示（role=note），高度 ≤56rpx，排在主按钮之后；不打断登录流程。
  - 有本机最近战绩缓存：「AI 提示：本机上次停在第 X 关 / 得分 Y，登录后到战报可复核。」
  - 无缓存：「AI 等待数据：登录后首页会按你的构筑给建议。」——不编造、不伪装成已分析。
  - 硬约束：不新增输入框、不弹窗、不遮罩、不改提交顺序、不插入验证码或二次确认；`aria-live="off"`，无循环思考动画；登录按钮仍是唯一主操作。
- `pages/verify/verify.vue`：轻量 AI = 仅在 `replayInfo` 到位后，结果卡下方单行 `role="note"`，不打断三步验真流程。
  - 文案：「AI 复核：关卡 / 种子 / 构筑三要素齐备；本地哈希与服务端一致才可作为证伪依据。」缺项时逐项列出缺什么。
  - 硬约束：未取到信息时不渲染 AI 文案；不伪造「思考中」或置信度；不插入步骤、不阻塞输入与「本地重放」按钮；CTA 复用已有按钮。
- 两页共同下限：徽章文字用 `--ai-ink`、边框用 `--ai-accent`（admin 用 `--lyp-ai-*`），正文仍用文本四阶；AI 行不得用 `--ai-halo` 单独承载信息。

### 1C.4 与 §1B 的复用边界与反模式

- 六组件只加 modifier，不新建平行样式体系；§1B.2 的尺寸、焦点环、数字伴随文本规则在本节全部生效。
- 反模式（一律不算改造成果）：把 rgba(255,255,255,.022) 网格、rgba(...,.06) 辉光、渐变光斑调亮；纯图标或英文 AI；只有结论没有数据源；写死文案；用低透明度底纹替代组件结构。
- 令牌：本节**不新增**变量。复用 `--ai-accent/--ai-ink/--ai-halo/--ai-glow`、`--accent-soft`、`--progress-track`（admin 加 `--lyp-` 前缀）；双端 `--lyp-token-version: lyp-tokens-v1` 与 `--lyp-r3-version: lyp-tokens-v3-r3` 必须同时保留。

### 1C.5 test-first 契约与验收

- 规范映射由两端契约测试钉住：`laiyipao/admin/tests/r4Mapping.contract.test.ts`（十四项映射全覆盖 + 双端令牌锚点）与 `laiyipao/miniapp/src/styles/r4Mapping.contract.test.ts`（十页映射 + 登录/验真轻量规则 + H5 锚点）。先 RED 再随本节落盘转 GREEN。
- 断言范围：§1C 五个子条款存在；十页 H5 与四视图 admin 均出现；六组件与 AI 四判据齐全；login/verify 轻量与不打断；低透明度反模式被明文禁止；v1/v3 双锚点存在且无 v4 误升。
- **不弱化既有断言**：不修改 `designTokens.contract.test.ts`、`textContrast.test.ts`、`focusVisible.contract.test.ts`、`tokens.contract.test.ts` 的任何判据；新契约只增不减。
- 页面实现归属：h5-ui / admin-ui 在各自写域补页面级 class 与数据断言；本节契约只守规范映射，不替页面实现背书。
- 截图门禁：登录页与验真页各交 1 张 390×844、ContentView 与 EconomyView 各交 1 张 1440×900 真浏览器截图（标注「非物理真机」），由 design-reviewer 按 §1B.4 肉眼复核 AI 是否可见、是否打断流程。

## 1D. 剩余页面与「可见 AI 感」补齐规范（R5 · 逐页可执行映射）

> 依据：R2–R4 真浏览器截图归档与 2026-11-22 源码审计（截图统一标注「非物理真机」，不等同物理设备实测）。§1B/§1C 已把 19 页 H5 与四视图 admin 纳入结构规范，但下列页面首屏仍缺「肉眼可辨的 AI 结论」：H5 announcements / login / signin / stage / verify 仍是基线或只有作战简报；index / equipment / reactions 已有 R2/R3 AI 卡。admin Skills / Users / Recomputes / Battles 已有六组件但无 AI 行，Levels 已有 `.ai-insight` 达标。本节只钉住映射与验收下限，不替页面实现背书。

### 1D.1 范围与升级状态（以当前代码为准）

| 端 | 页面 / 视图 | 状态 | 本轮动作 |
|---|---|---|---|
| H5 | `announcements.vue` | 待升级 | 补 SectionKicker + AI 公告摘要行 |
| H5 | `equipment.vue` | 已达标（R3） | 保持，不重复改造 |
| H5 | `index.vue` | 已达标（R2） | 保持首屏 AI 战术洞察卡 |
| H5 | `login.vue` | 待升级 | 落地 §1C.3 轻量 AI 行 |
| H5 | `reactions.vue` | 已达标（R3） | 保持，不重复改造 |
| H5 | `signin.vue` | 待升级 | 补 KPI + AI 领取提醒 |
| H5 | `stage.vue` | 待升级 | 在作战简报后补 AI 推进建议 |
| H5 | `verify.vue` | 待升级 | 落地 §1C.3 轻量 AI 复核行 |
| admin | `LevelsView.vue` | 已达标 | 保留 `.ai-insight`，仅维持 |
| admin | `SkillsView.vue` | 待升级 | 补 AI 技能/热量结论 |
| admin | `UsersView.vue` | 待升级 | 补 AI 异常账号结论 |
| admin | `RecomputesView.vue` | 待升级 | 补 AI 复算异常结论 |
| admin | `BattlesView.vue` | 待升级 | 补 AI 战报结论 |

### 1D.2 AI 四判据是硬下限（沿用 §1A.1，缺一不算）

1. **肉眼可辨**：首屏（H5 390×844 / admin 1440×900）不滚动即可看到带**可见汉字「AI」**的文字徽章 + 中文标题；纯图标、英文 AI、光晕或渐变**不算**。
2. **有据可查**：结论必须是**真实数据结论**，由页面已取到的字段经确定性规则推出，并在同卡显示**依据 + 数据时间/范围**；写死文案冒充智能不算。
3. **可行动**：每条结论带一个 **CTA**，指向与结论对应的真实页面或真实按钮；没有可跳转目标时宁可不渲染 AI 行。
4. **诚实标注**：标注「规则引擎 · 基于本地/服务端数据」，**不宣称 GPT**/大模型/实时学习，不编造置信度或数值。

四条缺一不算 AI 感。空数据按 §1A.4 给智能空态（说明缺什么 + 下一步动作），禁止「AI 思考中…」循环动画；R1 教训照旧生效：`rgba(255,255,255,.022)` 网格 + `rgba(...,.06)` 辉光在真截图上肉眼不可辨，**不算**改造，也**不算** AI 感。

### 1D.3 H5 五页待升级逐页映射

以下每行都是可执行映射：眉题 / KPI / FilterBar / DataGrid / EmptyState / AI 结论来源 / CTA 必须一次给全；`已达标` 三页不重复改造，保持 §1A/§1B 对应行。

| 页面 | 眉题（SectionKicker） | KPI（KpiStrip） | FilterBar / DataGrid / EmptyState | AI 结论来源（真实数据，必须显示依据与时间/范围） | CTA 指向 |
|---|---|---|---|---|---|
| `announcements.vue` | 「公告与版本」 | `items.length` 条公告 / 近 7 日新增 / 未读（无字段写 0 并注明） | FilterBar 用 `annFilters`（all/7d/30d）；DataGrid 按 `groupedByMonth` 分组；EmptyState「近 30 天没有公告，去看看战报」 | AI 结论：从 `items` 最新一条 + `groupedByMonth` 密度推出「本月更新 N 条，最新：{title}」；依据「公告列表 · 近 30 天」 | CTA：有详情链接则打开该公告；无链接则跳 `battles.vue` 查看变更影响 |
| `login.vue` | 例外页：SectionKicker「欢迎回来」 | 不编造 KPI（无用户数据） | 不新增 FilterBar/DataGrid；EmptyState 只用于加载失败提示，焦点环保留 | AI 结论（轻量）：沿用 §1C.3 的 `role="note"` 单行；有本机缓存才说「上次停在第 X 关 / 得分 Y」，无缓存说「登录后首页会按构筑给建议」；必须不新增输入、不弹窗、不打断提交；依据「本机缓存 · 上次登录」 | CTA：不新增按钮，登录按钮仍是唯一主操作；登录后由首页 AI 卡承接 |
| `signin.vue` | 「每日签到」 | `claimedCount` / `cycleDays` 周期 / 离线补偿 `offlineReward.coin` | DataGrid 渲染 `tasks[].progress/target`；EmptyState「今日任务已领完，明天再来」；FilterBar 按 SCOPES | AI 结论：由 `claimedCount`、`calendarDayCount/cycleDays`、`offlineReward.coin/cap_hours` 与 `tasks[]` 缺口推出「还差 N 天满 cycles，先领 X 币离线补偿 / 先做任务 Y」；依据「签到记录 · 本周期」 | CTA：跳到未完成任务或缺席补签按钮；无可领项则指向 `index.vue` |
| `stage.vue` | 「作战推进」 | `chapterStars/chapterTotal` 星数 / 可挑战章节 / 体力 `energyBudget` | DataGrid 列 `chapterOverview`；EmptyState「章节数据同步中」；FilterBar 按章节 | AI 结论：由 `difficultyCurve`、`terrainIndex`、`bossLevels`、`chapterStars` 与 `activeLevelId` 推出「第 N 关坡度最大（星数 X/Y、地形 Z），优先补该关」；依据「关卡曲线/地形/星数 · 当前存档」 | CTA：直达推荐关卡（`store.unlockedLevel` 已解锁时）或 `equipment.vue` 补强 |
| `verify.vue` | 「战斗验真」 | `replayInfo` 三要素齐备数 / `verifyHistory` 次数 / 匹配状态 | DataGrid 列 `verifyHistory` 与 `replayInfo`；EmptyState「暂无可复核的复现信息」 | AI 结论（轻量）：仅在 `replayInfo` 到位后显示「关卡/种子/构筑齐备；`result.matched` 与 `expected_hash` 一致才可作为证伪依据」；缺项逐项列出；依据「复现信息 · 本次验真」；不伪造思考或置信度，不插入步骤 | CTA：复用已有「本地重放」按钮，验真完成后跳 `battles.vue` 对照战报 |

### 1D.4 admin 五视图逐页映射（四页待升级，一页已达标）

| 页面 | 眉题（SectionKicker） | KPI（KpiStrip） | FilterBar / DataGrid / EmptyState | AI 结论来源（真实数据，必须显示依据与时间/范围） | CTA 指向 |
|---|---|---|---|---|---|
| `LevelsView.vue` | 已达标：现有 SectionKicker | 已达标：kpi-strip | 已达标：toolbar/data-grid/empty-state | AI 结论已存在：`clear_rate` 最弱关；保持 `.ai-insight`，不在本节重复改造 | CTA 保留复核该关卡 |
| `SkillsView.vue` | 「技能与配方」 | `skills.length` / 可合成配方数 / 平均热量 | FilterBar 按 `ELEMENT_LABEL`；DataGrid 列 `skills` + `recipes`；EmptyState「暂无技能数据」 | AI 结论：由 `dpsPerHeat(s)`（`base_damage*1000/cooldown_ms/heat_cost`）与 `recipes` 缺口推出「{技能} 每热量 DPS 最高但未入配方，优先替换」；依据「技能/配方表 · 当前种子版本」 | CTA：切到该技能详情或配方页 |
| `UsersView.vue` | 「用户与验真」 | `users.length` / 总量 `total` / 可疑账号数 | FilterBar 按验真三态；DataGrid 列 `users`；EmptyState「暂无用户」 | AI 结论：按 `suspicious(u) = power>5000 && max_stage<=1` 从 `users` 推出「N 个账号战力与关卡不符，优先验真」；依据「用户列表 · 当前筛选」 | CTA：打开对应用户详情抽屉并定位验真三态 |
| `RecomputesView.vue` | 「复算一致性」 | `summary.totals.total` / `summary.totals.mismatched` / `rate_permille` | Toolbar 按 `engine.status`；DataGrid 列 `summary.levels[].mismatched/max_user_mismatches`；EmptyState「暂无复算批次」 | AI 结论：由 `rate_permille`、`breaker_rate_permille` 与 `summary.levels` 最高不符关卡推出「第 N 关不符率最高，先复核」；依据「复算汇总 · 当前批次」；`shapeHint()` 仅作辅助文案 | CTA：定位该关卡复算明细 |
| `BattlesView.vue` | 「战报概览」 | `battleSummary.count` / `winRate` / `mismatched` / `suspicious` | FilterBar 按结果/时间；DataGrid 列本页 battles；EmptyState「暂无战斗记录」 | AI 结论：由 `battleSummary` 推出「胜率 {winRate}，异常 {suspicious} 场，先复核最近一场不符」；依据「本页战报 · 当前筛选范围」 | CTA：打开对应战报详情或跳 `RecomputesView.vue` |

### 1D.5 令牌、反模式与空态

- 本节**不新增视觉令牌**：复用 §1A.6 的 `--ai-accent` / `--ai-ink` / `--ai-halo` / `--ai-glow`（admin 为 `--lyp-ai-*`）与 §1B/§1C 的 `--accent-soft`、`--progress-track`。双端 `--lyp-token-version: lyp-tokens-v1` 与 `--lyp-r3-version: lyp-tokens-v3-r3` 必须同时保留，禁止误升 `lyp-tokens-v4`。
- AI 徽章文字用 `--ai-ink` / `--lyp-ai-ink`，边框用 `--ai-accent` / `--lyp-ai-accent`；`--ai-halo` 只作装饰，不能单独承载文字。正文仍走文本四阶，不把元素 18% soft 叠 panel-3 当卡底（已实测 fire 3.89 / ice 4.04 不达标）。
- 反模式一律**不算**改造成果：`rgba(255,255,255,.022)` 网格、`rgba(...,.06)` 辉光、只换色不加结构、只有英文 AI 或纯图标、只有结论没有依据、写死文案、空态写「暂无数据」。
- 空态卡高度与正常态一致；文案结构固定为「缺什么数据 + 下一步动作」，CTA 与结论一一对应。

### 1D.6 test-first 契约与验收

- 先 RED 后 GREEN：`laiyipao/admin/tests/r5Mapping.contract.test.ts` 与 `laiyipao/miniapp/src/styles/r5Mapping.contract.test.ts` 在 §1D 落盘前必须失败，落盘后转 GREEN；页面实现契约由 h5-ui / admin-ui 在各自写域补齐。
- 断言范围：六个子条款存在；八页 H5 与五视图 admin 均出现且区分已达标/待升级；五页 H5 与四视图 admin 映射行含真实数据字段、AI 结论与 CTA；四判据写死为下限；低透明度反模式被明文禁止；双端 v1/v3 锚点存在且无 v4 误升。
- **不弱化既有断言**：不修改 `designTokens.contract.test.ts`、`textContrast.test.ts`、`focusVisible.contract.test.ts`、`tokens.contract.test.ts`、`r4Mapping.contract.test.ts` 的任何判据；新契约只增不减。
- 验收门禁：双端全量测试绿；H5 五页各交 1 张 390×844、admin 四视图各交 1 张 1440×900 真浏览器截图，文件标注「非物理真机」，由 design-reviewer 按 §1A.7 / §1B.4 肉眼复核 AI 徽章是否可辨、CTA 是否可达、是否有数据依据。

---

## 2. 色彩系统

### 2.1 深色科幻底（表面分层）

表面按「页面底 → 卡片 → 浮起」三级明度递增。**层次靠明度差 + 微阴影，不靠堆叠描边**。

| 层 | miniapp 令牌 | miniapp 值 | admin 令牌 | admin 值 | 用途 |
|---|---|---|---|---|---|
| 页面底 | `--bg` | `#070a10` | `--lyp-bg` | `#0b0f14` | body / page 背景 |
| 卡片 | `--panel` | `#121826` | `--lyp-panel` | `#141a21` | card / 表格 / 侧栏 |
| 浮起 2 | `--panel-2` | `#1a2233` | `--lyp-panel-2` | `#1a222c` | 输入框 / 次级块 |
| 浮起 3 | `--panel-3` | `#222c3d` | `--lyp-panel-3` | `#212b37` | 弹层 / hover 行 |

**科幻底纹**：页面底允许一层极弱的品牌色径向光
（现量 `radial-gradient(120% 36% at 50% -6%, rgba(255,107,53,.10), transparent 60%)`），
**透明度不得超过 0.10**，不得铺满整屏、不得动画。这层是「氛围」不是「信息」，
任何文字都不得依赖它来传达含义。

> **收口动作**：两端底/面板色值允许保留各自的微小差异（H5 更冷、admin 更中性），
> 但**命名语义必须一致**（bg/panel/panel-2/panel-3），且**文本对比度必须同时满足 §2.4**。
> 长期目标是统一到同一组值（推荐 miniapp 向 admin 的 `#0b0f14` 系靠拢），
> 由后续令牌收敛任务执行，本文只裁决方向。

### 2.2 五元素主题色

**这是游戏机制的五个元素（I-1 元素反应），不是五行金木水火土。** 下游严禁另造别名。

| 元素 | key | 主色 | miniapp 令牌 | admin 令牌 | 语义 |
|---|---|---|---|---|---|
| 火 | `fire` | `#ff6b35` | `--fire` | `--el-fire` | 燃烧/爆发；同时是品牌主行动色 |
| 冰 | `ice` | `#58a6ff` | `--ice` | `--el-ice` | 减速/冻结；同时是稀有度与信息蓝 |
| 雷 | `lightning` | `#ffd33d` | `--lightning` | `--el-lightning` | 连锁/触发；同时是警告黄 |
| 腐蚀 | `corrosion` | `#7ee787` | `--corrosion` | `--el-corrosion` | 持续/侵蚀；同时是成功绿 |
| 动能 | `kinetic` | `#c9a7ff` | `--kinetic` | `--el-kinetic` | 弹射/物理；同时是史诗紫 |

每种元素必须有配套的 **soft 背景**（主色 18% 透明度叠在 panel 上），
现量令牌：`--fire-soft` … `--kinetic-soft`（miniapp），admin 按同语义补齐。

**使用规则**：
1. 元素色只能用于**该元素本身**或与之一致的状态（火=主行动、冰=信息/稀有、雷=警告、
   腐蚀=成功、动能=史诗）。跨语义复用必须先过 §2.4。
2. 一个视图里元素色出现 ≥3 种时，必须有文字标签或图标辅助区分——**颜色不是唯一线索**
   （WCAG 1.4.1）。
3. 元素色**不得**用作大面积背景（>25% 视口）；只允许软背景、描边、文字、小面积图标。

### 2.3 语义色

| 语义 | miniapp | admin | 值 | 说明 |
|---|---|---|---|---|
| 主行动 | `--accent` / `--accent-grad` | `--lyp-accent` / `-grad` | `#ff6b35` → `#ff3d71` | 与火同色系；**唯一**的「点这里」色 |
| 成功 | `--ok` | `--lyp-success` | `#7ee787` | 与腐蚀同色 |
| 警告 | `--warn` | `--lyp-warning` | `#ffd33d` | 与雷同色 |
| 危险 | `--bad` | `--lyp-danger` | `#ff7b72` | **独立于火**：火是主行动，红才是错误 |
| 信息 | `（用 --ice）` | `--lyp-info` | `#58a6ff` | 与冰同色 |

> **历史教训**：miniapp 曾让 `--bad` 与 `--accent` 同为 `#ff6b35`，
> 同一个橙既表示主按钮又表示错误，红色警示完全不醒目。已修复，严禁回退。

文本四阶：`text`（#e6edf3）> `text-2`（#c2ccd6）> `muted`（双端 #9aa5b2）
> `dim`（双端 #96a0ad）。t4 已对齐 miniapp，双端守卫强制 text-2 > muted > dim，
> 且 `.dim` 工具类走 `--dim` 令牌（不再借用 `--muted`）。

### 2.4 对比度红线（实测，硬约束）

所有承载信息的文字，在**它可能落到的每一种表面**上都必须 ≥4.5:1（WCAG AA 1.4.3）；
大字（≥18px/14pt 粗体）≥3:1。以下是 2026-11-21 实算结果：

**五元素色作为纯文字，落在最亮表面 panel-3 上**（全部合格，可作为安全基线）：

| 元素 | mini panel-3 | admin panel-3 |
|---|---|---|
| fire | 4.95 | 5.05 |
| ice | 5.56 | 5.67 |
| lightning | 9.79 | 9.99 |
| corrosion | 9.14 | 9.33 |
| kinetic | 6.99 | 7.13 |

**⚠️ 两类隐藏缺口（t4 已在令牌层修复，以下组合仍为禁止模式）**：

| 禁止组合 | 实测对比度 | 后果 | 正确做法 |
|---|---|---|---|
| 元素色字 + 18% 元素软背景，且叠在 **panel-3** 上 | fire **3.89**、ice **4.04**（<4.5） | 图鉴/标签在浅色卡片上读不清 | 软背景只叠 panel/panel-2（fire 4.87、ice 5.13 ✓）；必须用 `--fire-ink`/`--ice-ink` 替代主色文字（双端 4.62/4.62:1） |
| 玻璃层（+8% 白）叠在 **panel-3** 上再放 muted/dim 文字 | muted **4.37**、dim **4.13**（<4.5） | 命令面板/浮层副标题发灰 | 玻璃浮层固定用 panel 底；`--glass-text-muted/dim` 已提亮（最坏玻璃底 4.65/4.66:1）；仍禁止叠 panel-3 |

> 这两条不是「建议」：它们是真实存在的可访问性缺陷。任何引入上述组合的改动，
> 都必须同时提供修复（换底、降透明度或提亮文字），否则不得落地。
> 落地时应把这两条写成自动守卫（见 §8「待补守卫」）。

---

## 3. 玻璃拟态（克制）

**原则：玻璃是「层的语言」，不是「主题的语言」。** 默认界面是实色、高对比的；
玻璃只在「临时浮在内容之上」的组件出现。

| 允许的载体 | 参数 | 禁止 |
|---|---|---|
| 命令面板 / 快速搜索浮层 | `background: rgba(18,24,38,.82)`（panel 基色 + 82% 不透明）；`backdrop-filter: blur(16px) saturate(1.2)`；1px `--border-soft` 描边 | 正文卡片、列表行、表格 |
| 顶部导航 / 吸顶栏 | 同上，blur 12px；滚动后才加玻璃，顶部透明 | 首屏就整屏毛玻璃 |
| 弹窗遮罩上的面板 | panel-2 基色 + blur；阴影 `--shadow-2` | 多层玻璃叠加（blur 叠加会糊且掉帧） |

**性能与降级**：
- `backdrop-filter` 只允许出现在 ≤3 个同时可见的浮层；列表项**逐个**加玻璃会导致滚动掉帧。
- 不支持 `backdrop-filter` 的运行时（部分安卓 WebView / 小程序基础库）必须有 `@supports`
  回退到实色 panel——**可读性优先于效果**。
- 系统开启 `prefers-reduced-motion` 时，玻璃位移/淡入动画关闭（保留玻璃本身）。
- H5 端小程序容器若 `backdrop-filter` 实测花屏，直接禁用玻璃，用实色 + 阴影代替。

---

## 4. 微交互

**判据：每个交互都要「按得下、看得见、回得来」。** 现有 miniapp 已有完整动效族
（`page-in` / `chip-pop` / `pop-in` / `shake` / `stagger-in` / `skeleton-shimmer` 等），
admin 大部分视图仍无入场/按压反馈——这是下一阶段的补齐重点。

| 场景 | 时长 | 曲线 | 现成实现 / 约定 |
|---|---|---|---|
| 按压反馈 | 80–120ms | ease-out | miniapp `.tap:active` / `.btn:active`（scale .97） |
| 焦点可见 | 即时 | — | 全局 `:focus-visible`：2px 主色描边 + 2px 偏移；admin 已有守卫，miniapp 必须补 |
| 页面入场 | 200–280ms | ease-out | miniapp `.page-root` `page-in` |
| 结果弹入（成功/领奖） | 320–450ms | 弹性 cubic-bezier(0.2,.85,.3,1.2) | miniapp `.pop-in` / `.claim-pop` |
| 错误反馈 | 360ms | ease-in-out | miniapp `.shake`（配合 `:key` 重播） |
| 列表错峰 | 每项 +40ms，最多 6 项 | ease-out | miniapp `.stagger-in` |
| 加载指示 | 700ms/圈 或 1.2s shimmer | linear | `.spinner` / `.skeleton-bar` |

**硬规则**：
1. 所有装饰动画必须包在 `@media (prefers-reduced-motion: reduce)` 里可关闭——
   admin 已全局处理，miniapp 目前只覆盖了部分类，需补全。
2. 动效不得延迟「可操作」：按钮点击后 ≤100ms 内必须有视觉反馈，
   数据加载用骨架屏而不是让按钮转圈到数据回来。
3. 不要用动画表达关键信息（如「成功」只靠绿色脉冲而无文字）——颜色与文案双通道。

---

## 5. 命令面板（PC 专属）

**范围：仅 admin（PC）。H5 端不实现命令面板**——触屏没有物理键盘，Ctrl/Cmd+K 无意义；
H5 的等价物是现有的搜索入口（图鉴搜索 / 顶栏），保持「搜索即导航」。

### 5.1 PC 命令面板规格

- **唤起**：`Ctrl+K`（Windows/Linux）/ `Cmd+K`（macOS）；Esc 关闭；点击遮罩关闭。
- **位置**：视口上方 15% 居中，最大宽 640px，最大高 60vh，内部可滚动。
- **外观**：§3 玻璃浮层；输入框用 `--lyp-panel-2`；结果行 hover 用 `--lyp-panel-3`。
- **内容**（按优先级）：① 导航跳转（复用 `AdminLayout.vue` 的 `groups` 路由表，
  **不得再抄一份路由**）；② 常用操作（导出、重算等）；③ 最近访问。
- **键盘**：↑↓ 选择、Enter 执行、Esc 关闭；焦点陷阱（Tab 不逃出面板）。
- **无障碍**：`role="dialog"` + `aria-modal="true"`，打开时焦点移入输入框、关闭时归还触发元素；
  结果用 `aria-activedescendant` 播报。
- **空/错态**：无匹配时显示「没有匹配的页面」+ 清空按钮，不是空白面板。

> **实现约束**：命令源必须来自单一路由表（`AdminLayout.vue` 的 `groups`）。
> 已有多次「硬编码一份标签表导致漂移」的教训（DashboardView 的 reactionLabel 曾硬编码，
> 见该文件注释）——命令面板严禁重蹈覆辙。

---

## 6. 骨架屏

**原则：先给结构，再给数据。** 加载态必须保留最终布局的形状，避免数据到达时的布局跳动（CLS）。

| 端 | 现状 | 规范 |
|---|---|---|
| miniapp | 已有 `.skeleton-row` + `.skeleton-bar`（shimmer） | 保留并推广到所有列表/详情；shimmer 1.2s，`prefers-reduced-motion` 下改为静态灰条 |
| admin | 目前 `v-loading` 遮罩，整块糊一层灰 | **新增**通用骨架组件：表格页显示表头 + 5–8 行占位条；KPI 显示数字块占位。`v-loading` 仅用于「按钮提交中」等局部短时场景 |

**骨架条参数**：高 12–14px（PC）/ 28rpx（H5），圆角 4px/8rpx，
底色 `--panel-2` → `--panel-3` → `--panel-2` 渐变扫光；
宽度按内容层级变化（主 70%、次 45%、末 25%），不要全部等宽——等宽的骨架看起来像进度条而非内容。

---

## 7. PC / H5 响应式

**同一套令牌，两套版式。** 不为 PC 和 H5 各做一套颜色。

| 维度 | PC / admin | H5 / miniapp |
|---|---|---|
| 字号基准 | 13px 正文（`--lyp-fs-base`） | 26rpx 正文（`--fs-base`）；正文最小 24rpx |
| 页面宽度 | 内容限宽 1600px 居中（现量 `.page`） | 限宽 560px 居中（现量 `uni-page-body`） |
| 布局 | 侧栏 + 多列 KPI/图表，≥900px 完整展现 | 单列卡片流；≤900px 的 admin 工具栏转 grid、表格横向滚动 |
| 安全区 | 无 | 底部 `env(safe-area-inset-bottom)`（`.safe-bottom`）；顶部由 `useStatusBarPad()` 内联处理 |
| 触控目标 | ≥32px | ≥88rpx（≈44px，WCAG 2.5.5） |
| 命令面板 | 有（§5） | 无，用搜索入口 |
| 玻璃拟态 | 命令面板/顶栏 | 仅弹层；容器不支持则实色降级 |

**窄屏 admin 不是「缩小版 PC」**：目标是「可读可用」——工具栏堆叠成 grid、
表格在卡片内横向滚动（现量 `main.css` 已有 @media max-width:900px 规则，保留）。
严禁在窄屏把 12 列表格压进 253px。

---

## 8. 令牌体系与落地路径（t4 已落地 v1）

### 8.1 令牌契约（v1 已落地 · R3 包 `lyp-tokens-v3-r3`）

两端令牌「语义同名、前缀不同」的现状予以保留（避免大范围改名破坏现有测试）。
唯一真源是 `miniapp/src/styles/tokens.css` 与 `admin/src/styles/tokens.css`，
`common.css` / `main.css` 通过 `@import` 消费；新增令牌必须同时登记两端并跑契约测试：

| 语义 | miniapp | admin | 必须一致 |
|---|---|---|---|
| 页面底 / 卡片 / 浮起 2 / 浮起 3 | `--bg/-panel/-panel-2/-panel-3` | `--lyp-bg/-panel/-panel-2/-panel-3` | 对比度（§2.4） |
| 主文 / 次文 / 弱化 / 更弱 | `--text/-text-2/-muted/-dim` | `--lyp-text/-text-2/-muted/-dim` | 层级 + 对比度 |
| 五元素主色 / soft | `--fire…--kinetic` / `--*-soft` | `--el-fire…--el-kinetic` | **色值** |
| 语义色 | `--accent/-ok/-warn/-bad` | `--lyp-accent/-success/-warning/-danger` | 色值 |
| 圆角 / 阴影 | `--radius* / --shadow-*` | `--lyp-radius* / --lyp-shadow` | 视觉量级（rpx↔px 换算） |
| on-soft 元素 ink | `--*-ink` | `--el-*-ink` | 同值 + soft+panel-3 ≥4.5:1 |
| 玻璃层 / 玻璃文本 | `--glass-bg/-border/-blur/-text-*` | `--lyp-glass-*` | 文本同值；底限定 panel |
| 动效 / 层级 | `--motion-* / --z-*` | `--lyp-motion-* / --lyp-z-*` | 同值 |
| AI 感标识 / 文字 / 光晕（§1A.6） | `--ai-accent / --ai-ink / --ai-halo / --ai-glow` | `--lyp-ai-*` | 色值同值；ink 对 soft+panel-3 ≥4.5:1 |
| R3 规范包标记（§1B.6） | `--lyp-r3-version` | `--lyp-r3-version` | 同值 `lyp-tokens-v3-r3`；旧 `--lyp-token-version` 保留 v1 |
| R3 筛选活动态（§1B.2.5） | `--accent-soft` | `--lyp-accent-soft` | 同值 rgba(255,107,53,.18)；文字必须 `--accent-ink` |
| R3 进度轨道（§1B.2.6） | `--progress-track` | `--lyp-progress-track` | 同值 `#39465a`；状态必须配数字文本 |

> 兼容说明：旧 `--lyp-token-version: lyp-tokens-v1` 由现有守卫钉住，本轮不提升；R3 以独立标记登记。R4 无新增令牌，v1/v3 双锚点继续有效。

### 8.2 落地顺序（t4 已完成 1–4；5–6 由下游继续）

1. ✅ **对齐对比度**（t4）：miniapp 的 dim 提到 `#96a0ad`、muted 提到 `#9aa5b2`，双端文本四阶守卫齐全。
2. ✅ **修 §2.4 两类禁止组合**（t4）：新增 `--*-ink`（叠 soft+panel-3 ≥4.5:1）与 `--glass-text-*`（最坏玻璃底 ≥4.5:1）；soft / glass 底限定 panel、panel-2，并有规则块扫描守卫。
3. ✅ **令牌文件归一**（t4）：双端 `tokens.css` 为唯一真源（版本 `lyp-tokens-v1`），`common.css` / `main.css` 通过 `@import` 消费，旧令牌名全部保留。
4. ✅ **玻璃/微交互基建**（t4）：双端补齐 reduced-motion 全局兜底与键盘焦点环；`main.css` 新增 `.glass-surface` / `.skeleton-row` / `.skeleton-bar` 基础类，t5/t7 只需在组件层复用。
5. **命令面板**：按 §5 在 admin 落地，路由源复用 `groups`。
6. **响应式回归**：两端各跑一轮真实浏览器截图核验（标注「非物理真机」）。
7. **R3 次级页面**：按 §1B 六组件模式改造 16 页；先 RED 契约再 GREEN，4 张代表页留真浏览器截图（标注「非物理真机」）。

### 8.3 已补守卫（自动测试，防止规范回退）

- miniapp：文本四阶对比度（对 bg/panel/panel-2/panel-3 四种表面均 ≥4.5:1）+ 层级递减 + 焦点环 + reduced-motion —— `miniapp/src/styles/tokens.contract.test.ts`。
- 双端：令牌版本 / 五元素色值 / ink / glass / 动效令牌对齐 —— `admin/tests/designTokens.contract.test.ts`。
- 双端：禁止模式扫描——元素 soft 背景不得与 panel-3 同规则块共现；玻璃层不得叠 panel-3 承载 muted/dim（同上测试）。
- admin 原有守卫：`tests/textContrast.test.ts`（改读 tokens.css）、`tests/focusVisible.contract.test.ts`。
- R3 待补守卫（h5-ui / qa-e2e 在各自写域新增，断言见 §1B.5）：16 页组件标记与 AI 文案扫描、双端 R3 令牌对齐、禁止裸「暂无数据」、禁止 `--accent` 文字叠 `--accent-soft`。
- R4 已补守卫：`admin/tests/r4Mapping.contract.test.ts`、`miniapp/src/styles/r4Mapping.contract.test.ts`（断言见 §1C.5），只钉规范映射与令牌锚点。

---

## 9. 无障碍底线（不可协商）

1. 信息**不得只靠颜色**（WCAG 1.4.1）——元素/状态必须有文字或图标副线索。
2. 承载信息的文字 ≥4.5:1（§2.4）；大字 ≥3:1。
3. 键盘可达且焦点可见（WCAG 2.4.7）——双端已有守卫与自动测试（t4）。
4. 触控目标 ≥44px（WCAG 2.5.5）——H5 ≥88rpx。
5. `prefers-reduced-motion` 必须关闭装饰动画。
6. 命令面板为模态对话框时必须焦点陷阱 + Esc 关闭 + 焦点归还。

---

## 10. 反模式清单（看到即修）

| 反模式 | 为什么错 | 正确做法 |
|---|---|---|
| 元素色大字铺满卡片背景 | 破坏深色底层次、文字对比度崩 | 软背景 18% 或更小面积 |
| 同一个橙既当主行动又当错误 | 历史真实故障（§2.3） | 危险用 `#ff7b72` |
| 列表每行加 `backdrop-filter` | 掉帧、糊 | 仅浮层用玻璃 |
| 元素色字叠 18% 软背景再叠 panel-3 | fire 3.89 / ice 4.04 不达标（§2.4） | 换 panel 底或降透明度 |
| 命令面板硬编码一份路由/标签 | 漂移（DashboardView 教训） | 复用 `groups` 单一真源 |
| H5 复制 PC 的 Ctrl+K | 触屏无键盘 | H5 用搜索入口 |
| 加载态只显示「加载中…」 | 布局跳动（CLS） | 骨架屏占位 |
| 硬编码真实域名/IP 进样式或文档 | 泄密（公开仓库） | `<YOUR_TEST_DOMAIN>` 占位 |
| 新令牌只加一端 | 双端漂移 | 按 §8.1 映射表双端登记 |
| 假 AI：写死结论、假聊天框、无数据还显示洞察 | 误导用户，F-10-2 明确指出 | 按 §1A.1 四判据：真实数据 + 规则 + 依据 + CTA |
| AI 卡用 soft 叠 panel-3 或渐变当正文底 | 对比度崩（§2.4 实测） | panel / panel-2 底 + `--ai-ink` 文字 |
| 用 `.page-root:before` 的 .022 网格 / .06 辉光充当 AI 感 | R1 截图肉眼不可辨 | §1B.4：可见「AI」汉字 + 真实数据结论 + CTA |
| 活动筛选芯片用 `--accent` 文字叠 `--accent-soft` | 实算 4.37 / 4.40:1 未过红线 | 改 `--accent-ink`（5.20 / 5.23:1） |
| 进度条只靠颜色表示完成 | 违反 §9.1 信息不只靠颜色 | `--progress-track` + 数字文本「n/总数」 |
| 次级页只换色/加底纹就宣布完成 | 用户点名的 16 页仍简陋 | §1B 六组件 + RED 契约 + 真浏览器截图 |

---

## 11. 验收清单（任何视觉改动合入前自检）

- [ ] 改动能追溯到本文某一条（在提交说明里写明条款号）。
- [ ] 新增/修改的令牌已按 §8.1 双端登记，色值与本文一致。
- [ ] 文字在它所有可能的表面上 ≥4.5:1（§2.4），未引入禁止组合。
- [ ] 信息不只是靠颜色区分（§9.1）。
- [ ] 按压/焦点反馈存在；`prefers-reduced-motion` 下动画可关。
- [ ] PC 与 H5 都经过真实浏览器截图核验（截图标注「非物理真机」）。
- [ ] 未引入任何真实域名/IP/密钥（`<YOUR_TEST_DOMAIN>` 占位）。
- [ ] 若改动涉及 AI 元素：满足 §1A.1 四判据，H5 首页 / admin 看板各有首屏截图（标注「非物理真机」）。
- [ ] 相关守卫测试通过（admin 现有 3 项 + §8.3 待补项）。
- [ ] 若涉及 H5 次级页：按 §1B.1 映射表六组件齐全，AI 线索肉眼可辨且非低透明度底纹。
- [ ] R3 令牌双端齐全：`--lyp-r3-version: lyp-tokens-v3-r3`、`--accent-soft`、`--progress-track`。
- [ ] 若涉及 R4 页面：按 §1C 映射齐全，登录/验真的轻量 AI 不打断流程；低透明度网格/光斑不计入 AI 感。
- [ ] 空态未出现裸「暂无数据」/「加载失败」；筛选活动态未用 `--accent` 文字叠 soft。

---

> **变更规则**：本文由设计负责人维护。任何条款修订必须写明「改了什么、依据是什么
> （真实页面实测或审计证据）、影响哪些端」，并同步更新 §8.1 令牌表与 §8.3 守卫。
> 本文不替代代码真源——令牌的最终值以两端 `styles/` 文件为准，但二者不符时
> 以本文为准修复代码，而不是反过来。
