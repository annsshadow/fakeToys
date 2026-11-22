# 来一炮 · 设计令牌速查表（配套 AI_VISUAL_SYSTEM.md）

> 本文是 [AI_VISUAL_SYSTEM.md](./AI_VISUAL_SYSTEM.md) 的**按名查阅版**，
> 供工程师实现时快速对照；条款裁决以主规范为准。
> 令牌**代码真源**：`laiyipao/miniapp/src/styles/tokens.css` 与 `laiyipao/admin/src/styles/tokens.css`（t4 起，版本 `lyp-tokens-v1`）；
> `common.css` / `main.css` 仅 `@import` 并保留跨页样式与组件映射。
> 新增令牌必须双端登记（主规范 §8.1）。
> R3 追加（t20）：包标记 `--lyp-r3-version: lyp-tokens-v3-r3` 双端同值；旧 `lyp-tokens-v1` 为兼容锚点。
> R4 追加（t34）：十页 H5 + 四视图 admin 的组件映射见主规范 §1C；**不新增令牌**，v1/v3 双锚点保持。

---

## 1. 表面与文本（对比度已实测达标，见主规范 §2.4）

| 语义 | miniapp | 值 | admin | 值 |
|---|---|---|---|---|
| 页面底 | `--bg` | `#070a10` | `--lyp-bg` | `#0b0f14` |
| 卡片 | `--panel` | `#121826` | `--lyp-panel` | `#141a21` |
| 浮起 2 | `--panel-2` | `#1a2233` | `--lyp-panel-2` | `#1a222c` |
| 浮起 3 | `--panel-3` | `#222c3d` | `--lyp-panel-3` | `#212b37` |
| 描边 | `--border` / `--border-soft` | `#2a3547` / rgba(.55) | `--lyp-border` / `-soft` | `#2b3540` / `#222b35` |
| 主文 | `--text` | `#e6edf3` | `--lyp-text` | `#e6edf3` |
| 次文 | `--text-2` | `#c2ccd6` | `--lyp-text-2` | `#c2ccd6` |
| 弱化 | `--muted` | `#9aa5b2` | `--lyp-muted` | `#9aa5b2` |
| 更弱 | `--dim` | `#96a0ad` | `--lyp-dim` | `#96a0ad` |

> ✅ t4 已把 miniapp 的 muted/dim 提到双端同值：dim 在 panel-3 上由 4.56:1 提升到 5.30:1；
> `.dim` 工具类已改走 `--dim`（不再借用 `--muted`）。见主规范 §8.2 步骤 1。

## 2. 五元素主题色（主规范 §2.2）

| 元素 key | 主色 | miniapp | admin | soft（miniapp） | 在 panel-3 上纯文字对比度 |
|---|---|---|---|---|---|
| `fire` | `#ff6b35` | `--fire` | `--el-fire` | `--fire-soft` | 4.95 / 5.05（mini/admin）|
| `ice` | `#58a6ff` | `--ice` | `--el-ice` | `--ice-soft` | 5.56 / 5.67 |
| `lightning` | `#ffd33d` | `--lightning` | `--el-lightning` | `--lightning-soft` | 9.79 / 9.99 |
| `corrosion` | `#7ee787` | `--corrosion` | `--el-corrosion` | `--corrosion-soft` | 9.14 / 9.33 |
| `kinetic` | `#c9a7ff` | `--kinetic` | `--el-kinetic` | `--kinetic-soft` | 6.99 / 7.13 |

**禁止组合**：元素色字 + 18% soft 背景 + panel-3 底 → fire 3.89:1 / ice 4.04:1（不达标）。
soft 背景只允许叠 panel / panel-2；必须叠 soft+panel-3 时，文字改用下表 ink 令牌。

| 元素 | `--*-ink`（mini）/ `--el-*-ink`（admin） | 叠自身 soft+panel-3 |
|---|---|---|
| fire | `#ff8659` | 4.62 / 4.70 |
| ice | `#6fb2ff` | 4.62 / 4.71 |
| lightning | `#ffd33d` | 6.26 / 6.37 |
| corrosion | `#7ee787` | 5.90 / 6.00 |
| kinetic | `#c9a7ff` | 4.82 / 4.91 |

## 3. 语义色（主规范 §2.3）

| 语义 | miniapp | admin | 值 |
|---|---|---|---|
| 主行动 | `--accent` / `--accent-grad` | `--lyp-accent` / `-grad` | `#ff6b35` → `#ff3d71` |
| 成功 | `--ok` | `--lyp-success` | `#7ee787` |
| 警告 | `--warn` | `--lyp-warning` | `#ffd33d` |
| 危险 | `--bad` | `--lyp-danger` | `#ff7b72` |
| 信息 | `（--ice）` | `--lyp-info` | `#58a6ff` |

## 3A. AI 感令牌（主规范 §1A，t12 新增）

| 语义 | miniapp | admin | 值 / 用途 |
|---|---|---|---|
| AI 标识色 | `--ai-accent` | `--lyp-ai-accent` | `#c9a7ff`，仅徽章边框/左色条/图标描边 |
| AI 文字色 | `--ai-ink` | `--lyp-ai-ink` | `#d7bfff`，18% kinetic soft + panel-3 上 5.90 / 6.04:1 |
| AI 微光底 | `--ai-halo` | `--lyp-ai-halo` | `rgba(201,167,255,.14)`，仅装饰，不承载正文 |
| AI 光晕 | `--ai-glow` | `--lyp-ai-glow` | `0 0 24rpx/24px rgba(201,167,255,.28)`，reduced-motion 下静止 |

> 「可见 AI 元素」四条判据与组件规格见主规范 [§1A](./AI_VISUAL_SYSTEM.md#1a-ai-感支柱p8--r2-新增f-10-2-整改唯一依据)：
> 首屏可见 + 汉字「AI」标识 + 真实数据依据 + 可行动 CTA；禁止假聊天框/写死结论。正文一律用文本四阶。

## 4. 几何与字阶

| 语义 | miniapp | admin |
|---|---|---|
| 小圆角 | `--radius-sm: 10rpx` | `--lyp-radius-sm: 8px` |
| 圆角 | `--radius: 16rpx` | `--lyp-radius: 10px` |
| 大圆角 | `--radius-lg: 18rpx` | — |
| 阴影 1 / 2 | `--shadow-1` / `--shadow-2` | `--lyp-shadow` |
| 字阶 xs→xl | 22/24/26/28/32/38 rpx | 11/12/13/15/20/26 px |
| 间距 | `--sp-1..4`：8/12/16/24 rpx | `--lyp-gap-1..4`：6/10/14/20 px |

## 5. 现成动效类（miniapp，主规范 §4）

| 类 | 用途 | 时长 |
|---|---|---|
| `.page-root` → `page-in` | 页面入场 | 240ms |
| `.tap:active` / `.btn:active` | 按压 | 120ms / 100ms |
| `.filter-chip.active` → `chip-pop` | 筛选切换回弹 | 280ms |
| `.pop-in` | 成功弹入 | 320ms |
| `.claim-pop` | 领奖高亮 | 450ms |
| `.shake` | 错误抖动 | 360ms |
| `.stagger-in` | 列表错峰 | 340ms + 40ms/项 |
| `.skeleton-bar` → `skeleton-shimmer` | 骨架扫光 | 1.2s |
| `.spinner` | 加载转圈 | 0.7s/圈 |
| `.empty-glyph` → `empty-float` | 空态浮动 | 2.6s |

**全局焦点环**（两端必须有）：
`:where(a,button,input,textarea,select,[tabindex]):focus-visible { outline: 2px solid var(--lyp-accent) !important; outline-offset: 2px }`（admin）
`:where(...):focus-visible { outline: 3rpx solid var(--accent); outline-offset: 2rpx }`（miniapp）
动效令牌（双端同值）：`--motion-fast 120ms / --motion-base 240ms / --motion-slow 320ms / --motion-ease cubic-bezier(.2,.85,.3,1.15)`。

## 6. 玻璃浮层参数（主规范 §3）

```css
/* 仅命令面板 / 顶栏 / 弹窗面板使用；底必须是 panel 或 panel-2，不得叠 panel-3 承载 muted/dim */
.glass-surface {
  background: var(--glass-bg); /* admin: var(--lyp-glass-bg)，均为 panel 基色 */
  backdrop-filter: var(--glass-blur); /* admin: var(--lyp-glass-blur) */
  border: 1px solid var(--glass-border); /* admin: var(--lyp-glass-border) */
  box-shadow: var(--shadow-2); /* admin: var(--lyp-shadow) */
}
.glass-surface .muted { color: var(--glass-text-muted); } /* admin: --lyp-glass-text-muted */
.glass-surface .dim { color: var(--glass-text-dim); } /* admin: --lyp-glass-text-dim */
@supports not (backdrop-filter: blur(1px)) {
  .glass-surface { background: var(--panel); } /* 降级实色 */
}
```

## 7. 令牌命名增删流程

1. 先在 [AI_VISUAL_SYSTEM.md](./AI_VISUAL_SYSTEM.md) §8.1 表里登记语义与两端名称。
2. 同时写入两端 `styles/tokens.css`（保持现有前缀约定：mini 无前缀 / admin `--lyp-` / 元素 `--el-`）。
3. 补/更新对比度守卫（主规范 §8.3），确认色值测过 §2.4 的每一种表面。
4. 在改动说明里写明主规范对应条款号。

---

## 7A. R3 次级页面组件速查（主规范 §1B）

**包标记**：`--lyp-r3-version: lyp-tokens-v3-r3`（双端同值）。旧 `--lyp-token-version: lyp-tokens-v1` 由现有守卫钉住，暂不提升。

| 语义 | miniapp | admin | 值 / 用途 |
|---|---|---|---|
| R3 包标记 | `--lyp-r3-version` | `--lyp-r3-version` | `lyp-tokens-v3-r3` |
| 筛选活动底 | `--accent-soft` | `--lyp-accent-soft` | `rgba(255,107,53,.18)`；文字必须 `--accent-ink`，禁止 `--accent` 主色（4.37/4.40:1 未过线） |
| 进度轨道 | `--progress-track` | `--lyp-progress-track` | `#39465a`；必须配「n/总数」数字文本，不得只靠颜色传状态 |

**六组件模式**（详细规格见主规范 §1B.2）：

| 模式 | class 锚点（H5 / admin） | 硬约束 |
|---|---|---|
| SectionKicker | `.section-kicker` / `.section-title` | AI 变体 `.section-kicker--ai` 必须有汉字「AI」 |
| KpiStrip | `.kpi-strip` / `.kpi-*` | 主 KPI ≤4；主卡 6rpx/3px 左强调条；加载态骨架同高 |
| EmptyState | `.empty-state` / `.state-block` | 图形+标题+原因+动作按钮；禁「暂无数据」「加载失败」裸文案 |
| DataGrid | `.list-row` / `<table>` | H5 卡片行、admin 真表格；数字 `tabular-nums`；空态切换 EmptyState |
| FilterBar | `.chip-tabs` / `.toolbar` | 活动态 `--accent-soft` + `--accent-ink`；结果数 `aria-live="polite"` |
| ProgressTree | `.progress-node` / `.progress-grid` | 四态都有文字标签；轨道 `--progress-track` + 数字；节点 ≥88rpx |

**AI 感下限**（§1B.4）：`.022` 网格、`.06` 辉光、渐变光斑、纯图标都不算 AI；必须同时有可见汉字「AI」+ 真实数据结论 + 依据 + CTA。
徽章文字用 `--ai-ink`（soft+panel-3 上 5.90/6.04:1），正文仍用文本四阶。诚实标注「规则引擎 · 基于当前数据」。

**test-first**（§1B.5）：先补双端令牌对齐 + 16 页结构扫描 + 禁止裸空态/禁止 `--accent` 叠 soft 的契约，跑到 RED 再改页面到 GREEN。
截图门禁：专精 / 商城 / 宝石图鉴 / 公告 4 张 390×844 真浏览器截图（标注「非物理真机」）。

---

## 7B. R4 剩余页面映射速查（主规范 §1C）

- 十页 H5：announcements / bag / battles / enemies / login / me / rank / signin / skins / verify；四视图 admin：ContentView / Economy / Defenses / Equipment。
- 组件仍用 §7A 六件套；AI 仍需「可见汉字「AI」+ 真实数据结论 + 依据 + CTA」。
- 登录页：主按钮后一行 `role="note"`，≤56rpx，不新增输入、不弹窗、不打断登录；无本地战绩时写「AI 等待数据」，不编造。
- 验真页：仅在 `replayInfo` 到位后显示一行复核说明，不伪造「思考中」，不阻塞三步流程。
- 低透明度 .022 网格 / .06 光斑不算 AI；令牌零新增，v1/v3 双锚点不动。契约：`admin/tests/r4Mapping.contract.test.ts`、`miniapp/src/styles/r4Mapping.contract.test.ts`。
---

## 7C. R6 真实数据驱动推荐速查（主规范 §1E）

- 包标记：`--lyp-r6-version: lyp-tokens-r6`（双端同值）；`lyp-tokens-v1` / `lyp-tokens-v3-r3` 保留，禁止 `lyp-tokens-v4`。**不新增视觉令牌**，仍用 `--ai-accent` / `--ai-ink` / `--ai-halo` / `--ai-glow`（admin `--lyp-ai-*`）。
- 审计：H5 18/19 有结构标记（battle.vue 为实时战斗例外），login / verify / reactions / skills / equipment 为浅升级；admin 7/12 有 AI 结论卡，ContentView / DefensesView / EquipmentView / LoginView / WikiView 无 AI 卡。
- 推荐接口（本轮只规范不实现）：`GET /api/v1/me/recommendations?context=<...>` 与 `GET /api/v1/admin/recommendations/<scope>`；输入必须来自用户画像 + 战斗历史 + 配置版本。响应写 `schema_version` / `generated_at` / `context` / `evidence[]` / `model` / `confidence` / `cta` / `empty_reason`；`confidence` 只写 `null` / `rule_based` / `statistical`，禁止假数字与 GPT 宣称；缺数据用 `NO_HISTORY` 等诚实空态。
- 玩法三类：元素反应链构筑、装备/宝石搭配、关卡难度与 BOSS 机制引导；每条结论必须带真实字段依据与可点击 CTA。
- 推荐卡四区：AI 徽章 + 一句结论 → 数据条 → 「为什么推荐」 → 主 CTA；加载/空态/错误态等高，错误态标注「本地规则引擎」。契约：`admin/tests/r6PersonalizedRec.contract.test.ts`、`miniapp/src/styles/r6PersonalizedRec.contract.test.ts`。

## 7D. R7 玩法体验速查（主规范 §1F）

- 依据：`docs/laiyipao/audits/round7-gameplay/README.md`（非物理真机）。三缺口 → 三动作：复盘归因（19726 → 10909 / 74s 上限 / 11941 门槛）→ 结算与战报写「为什么没拿星」；战斗中无目标感 → 「战斗实时星级差额」HUD；成长推荐无法落地 → 「当前 vs 推荐」+「确认替换」。
- 复盘只解释服务端权威字段：`settleResult.score/stars/leaked/reactions/clamped/clamp_note/replay_hash` + `star_targets` / `max_score` / `score_full_at_sec`；不得重算分数或星级。优先级：时长额度 → 漏怪 → 反应链（30% 口径，禁止「关系不大」）；缺字段走诚实空态。
- HUD：`gap = star_targets[下一个未达门槛] - score`，门槛来自服务端配置，不得硬编码 11941；回合结束前可见、非阻塞、不遮挡塔位与技能卡，reduce 下静止。
- 推荐落地：`/me/recommendations?context=skills|equipment` 的 `evidence[]` 解释因果，`expected_gain` 以 t65 接口为准；两步 CTA「应用推荐 → 当前 vs 推荐 → 确认替换 / 取消」，禁止无确认写出战配置，失败回退标注「本地规则引擎」。
- 令牌与守卫：不新增视觉令牌（v1 / v3-r3 / r6 锚点保留，无 v4），契约 `admin/tests/r7GameplayFeedback.contract.test.ts` 与 `miniapp/src/styles/r7GameplayFeedback.contract.test.ts` 保持 GREEN。
## 7E. R8 关卡规范速查（主规范 §1G）

- 依据：docs/laiyipao/audits/round8-gameplay/README.md（非物理真机）。星级非单调：1★降47次/max降44次、约0.60；关卡页并列显示 `max_score` 与 `star_targets`，解释「按本关满分百分比给星」，禁止「高关更容易」误读与硬编码11941。
- 难度曲线：标注 boss 关 56/60/91/95/96/100、新敌种首现、地形变化；拒绝 r5AiStage 首个并列 +33‰ 当「本章最大跳变」。
- 时间预算 HUD：「剩余保分窗口：{N}s」来自 `score_full_at_sec` 或等价字段，临近截止即时提示，不遮挡不硬编码，配置缺失走诚实空态。
- AI 四判据：可见汉字「AI」+真实数据结论+依据+CTA，低透明度底纹不算；不回退 R1-R7 诚实文案与可访问性。
- 令牌与守卫：不新增视觉令牌（v1/v3-r3/r6 与 v1E/v1F 标记保留，无 v4、无 `--r8-*`）；双端契约 r8LevelCurve.contract.test.ts 保持 GREEN。
## 7F. R9 战斗反馈与选卡因果速查（主规范 §1H）

- 依据：docs/laiyipao/audits/round9-gameplay/README.md（非物理真机）。三关全程跳过卡牌仍 C/C/B；钳分 19684→8992（61s）/19208→10130（71s）/16297→11521（76s）；D1 已做反应 HUD（过热 -216 等）与下一关 CTA。
- 反应因果：`recentReactions` 每条含 REACTIONS 名称 + 触发元素 + `damage` + 抗性标记，保留 3 条、每局清空、aria-live、pointer-events:none；补一句「焰+电 → 过热，克制密集波」式 contextual hint，缺元素字段写「元素组合未上报」。
- 选卡反馈：权威 `card_picks` 统计取/弃/跳与反应数/伤害占比，卡面相对收益只来自当前构筑，禁止数值碾压与「不拿就输」；全程跳过通关写真实陈述与可行动建议，不羞辱玩家。
- 实时换算：钳分倒计时与星级差额共用 `score_full_at_sec` + 战斗时钟 + `star_targets`；「上限→实得→差额」提前到 HUD；锚点前只说「钳分中·距满分 Xs / 即将免钳分」，禁止「保分窗口」与硬编码。
- AI 四判据：可见汉字「AI」+真实数据结论+依据+CTA，低透明度底纹不算；不回退 R1-R8 诚实文案、30% 口径、25% AoE、顶栏脱敏、focus-visible/reduced-motion、玻璃 soft 禁用组合。
- 令牌与守卫：不新增视觉令牌（v1/v3-r3/r6 与 v1E/v1F/r8 标记保留，无 v4、无 `--r9-*`）；双端契约 r9BattleFeedback.contract.test.ts 保持 GREEN。
## 7G. R10 养成因果链与收益可解释速查（主规范 §1I）

- 依据：docs/laiyipao/audits/round10-gameplay/README.md（非物理真机）。燃烧弹 100→105 后同关 10614→12088 分、C→B、反应 92→93，hash `501a22c7`→`1c36fcc2`；D1 写入成功可见、D2 宝石图鉴双 CTA 已修。
- 推荐收益：`RecCTA.params.candidate/replace` 必须由服务端 `recImpactCTA` 给出（配置存在/已解锁/已拥有/同槽或同主战/数值更优），无参数或不可执行走诚实空态；客户端不得伪造、不得猜文案；两步确认后才写入，成功后 `.apply-done` 可见。
- 养成因果：战报并列养成前/后同一关可信差值（分数/评级/星级/反应数/上限→实得→差额），来自两次真实结算并各带 hash；缺一局显示诚实空态，禁止估算冒充。
- 宝石三段：图鉴「去商城找宝石」+「去背包镶嵌 →」最短路径；佩戴收益只认服务端属性差分（attack/crit/element_coef/reaction_mult/heat_cap），未上报写「佩戴收益未上报」，不虚构。
- AI 四判据：可见汉字「AI」+真实数据结论+依据+CTA，低透明度底纹不算；不回退 R1-R9 诚实文案、30% 口径、25% AoE、顶栏脱敏、focus-visible/reduced-motion、玻璃 soft 禁用组合、锚点前非保分窗口、跳过选卡不羞辱。
- 令牌与守卫：不新增视觉令牌（v1/v3-r3/r6 与 v1E/v1F/r8/r9 标记保留，无 v4、无 `--r10-*`）；双端契约 r10GrowthCausal.contract.test.ts 保持 GREEN。


