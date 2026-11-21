# 来一炮 · 设计令牌速查表（配套 AI_VISUAL_SYSTEM.md）

> 本文是 [AI_VISUAL_SYSTEM.md](./AI_VISUAL_SYSTEM.md) 的**按名查阅版**，
> 供工程师实现时快速对照；条款裁决以主规范为准。
> 令牌**代码真源**：`laiyipao/miniapp/src/styles/tokens.css` 与 `laiyipao/admin/src/styles/tokens.css`（t4 起，版本 `lyp-tokens-v1`）；
> `common.css` / `main.css` 仅 `@import` 并保留跨页样式与组件映射。
> 新增令牌必须双端登记（主规范 §8.1）。
> R3 追加（t20）：包标记 `--lyp-r3-version: lyp-tokens-v3-r3` 双端同值；旧 `lyp-tokens-v1` 为兼容锚点。

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

