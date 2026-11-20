# OA4Rust 双前端统一设计规范与令牌（Design Tokens & UI Spec）

> 适用范围：桌面端 `oa4rust/frontend/apps/desktop`（Vue3 + UnoCSS + naive-ui）与移动端 `oa4rust/mobile`（uni-app，H5 / 微信小程序）。
> 令牌实现：`oa4rust/frontend/packages/ui/src/theme/tokens.css`（由 `@oa4rust/ui` 入口导入，随 `style.css` 产出）。
> 权威来源：色彩类令牌以 `packages/ui/src/theme/dark.css` 为唯一来源（暗/亮双主题 + WCAG 2.1 AA 契约测试）；本文件与 tokens.css 只做增补，不重复定义色彩值。
> 约束：不引入新依赖；不改动两端业务代码即可使用（桌面端可直接用 UnoCSS 快捷类，移动端按映射表消费 CSS 变量）。

## 1. 现状审计（2026-11-19）

| 维度 | 桌面端 desktop | 移动端 mobile |
|---|---|---|
| 技术栈 | Vue 3.5 + Vite 8 + UnoCSS 66 + naive-ui 2.40 + vue-query/pinia/router | uni-app（Vue3 编译，H5 + 小程序），无组件库 |
| 主题体系 | 完整双主题 CSS 变量：暗色霓虹青 `#00d4ff` / 亮色蓝 `#2563eb`；玻璃拟态 + 霓虹辉光 | 单一浅色主题：`#2d8cf0` 企业蓝；变量散落在 `App.vue` 的 `page{}` 与 `uni.scss`（2 个变量） |
| 令牌覆盖 | 色彩/背景/边框/文字/阴影/圆角/过渡/尺寸（sidebar/topbar） | 仅色彩别名 20 个，无间距/字号/圆角/阴影/动效/断点令牌 |
| 无障碍 | 已有契约测试守护：文字对比度 ≥4.5:1、非文本 ≥3:1、focus-visible 焦点环、skip-link、reduced-motion（130+ 处动效收敛） | 部分文字色做了 4.5:1 注释，未成体系；无全局 focus/reduced-motion 守卫 |
| 性能瓶颈 | UnoCSS 快捷类大量手写色值/间距（598 处色值扫描命中），无统一 shortcut 导致重复原子类；`dist` 手工分包已有（vue/naive/echarts/codemirror），chunk 警告阈值 1000KB；Google Fonts 外链在离网环境会阻塞 | 每页独立 `.vue` 内嵌样式，同一卡片/按钮样式重复；`dist` 产物 313 个文件含大量重复 CSS（每页一份 wxss），无统一 token 复用 |
| 结论 | 体系成熟，作为统一基准 | 命名与桌面端分裂，缺暗色模式与结构化令牌，需按本规范逐步收敛 |

**关键冲突与取舍**：桌面端"深色科幻霓虹"与移动端"浅色企业蓝"是两套视觉语言。本次决策选择**桌面端双主题令牌体系为统一基准**，原因：① 它已有暗/亮双主题与无障碍契约测试，成熟度最高；② 移动端浅色色值可在不破坏可读性的前提下，逐步映射为桌面端 light 主题色（接近的蓝/白/灰）；③ 保留移动端现有色值作为过渡期别名，避免一次性大改业务代码。

## 2. 设计决策（架构）

1. **单一来源，分层增补**：色彩令牌只在 `dark.css`（暗/亮双主题）定义；新增 `tokens.css` 仅补充间距、字号、圆角补充值、阴影补充值、动效补充值、层级、控件尺寸、响应式断点。这样既有 WCAG 契约测试（读取 dark.css）不受影响。
2. **CSS 变量为跨端桥梁**：H5 与小程序均支持 CSS 自定义属性；桌面端 UnoCSS 主题引用 `var(--token)`，移动端 uni-app 页面直接 `var(--token)`。无需运行时 JS，无新依赖。
3. **命名对齐**：以桌面端命名（`--color-primary` / `--bg-base` / `--text-secondary` 等）为准。移动端现有变量名通过第 5 节映射表逐步迁移，过渡期可双写。
4. **组件类名契约**：统一 `.o2-*` 前缀（o2-card / o2-btn / o2-input / o2-tag / o2-divider / o2-skeleton），桌面端用 UnoCSS shortcut，移动端用同名 class（uni-app 支持全局 class）。组件内禁止硬编码色值和间距。
5. **无障碍基线端到端统一**：focus-visible 焦点环、reduced-motion 守卫、skip-link、触控目标 ≥44px（WCAG 2.5.5）为双端共同底线。桌面端已由 dark.css 全局提供；移动端需在 App.vue 增加同等守卫（后续移动端成员落地）。

## 3. 令牌清单（tokens.css 权威定义）

### 3.1 间距（4px 栅格）
`--space-0/1/2/3/4/5/6/8/10/12/16` = 0/4/8/12/16/20/24/32/40/48/64px；
语义别名：`--space-card-padding`(16) / `--space-section-gap`(24) / `--space-page-gutter`(16，随断点增至 20/24)。

### 3.2 字号与行高
`--font-size-xs/sm/base/md/lg/xl/2xl/3xl` = 12/13/14/16/18/20/24/30px；
`--line-height-tight/normal/relaxed` = 1.25/1.5/1.75；字重 `400/500/600/700`。

### 3.3 圆角
`--radius-none`(0) / `--radius-sm`(6) / `--radius-md`(10) / `--radius-lg`(16) / `--radius-xl`(24) / `--radius-full`(9999)。（sm~xl 源自 dark.css）

### 3.4 阴影
`--shadow-card`(卡片) / `--shadow-glow`(辉光) / `--shadow-panel`(面板) / `--shadow-inset`(内阴影) / `--shadow-dropdown`(下拉) / `--shadow-card-mobile`(移动端轻卡片)。

### 3.5 动效
`--transition-fast/normal/slow`（dark.css：150/250/400ms）；补充 `--duration-instant/fast/normal/slow` = 100/150/250/400ms；缓动 `--ease-out`(0.16,1,0.3,1) / `--ease-in-out`。**必须**遵守 reduced-motion 守卫。

### 3.6 层级 z-index
`--z-base`(0) / `--z-dropdown`(100) / `--z-sticky`(200) / `--z-modal`(1000) / `--z-toast`(1100) / `--z-skiplink`(1200)。

### 3.7 控件尺寸
`--control-height-sm/md/lg` = 32/40/48px；`--touch-target-min` = 44px（移动端可点击区域下限）。

### 3.8 响应式断点
xs<576 / sm≥576 / md≥768 / lg≥992 / xl≥1200 / 2xl≥1536；CSS 变量 `--breakpoint-*`。页面边距 `--space-page-gutter` 随 sm/lg 断点递增。小程序端媒体查询不生效时可忽略。

### 3.9 色彩（dark.css 权威，不在 tokens.css 重定义）
| 语义 | 暗色（dark） | 亮色（light） | 移动端现状（过渡期别名） |
|---|---|---|---|
| 主色 | `#00d4ff` | `#2563eb` | `#2d8cf0` → 建议映射 `--color-primary` |
| 成功 | `#10b981` | `#166534` | `#67c23a` → `--color-success` |
| 警告 | `#f59e0b` | `#92400e` | `#ff9900` → `--color-warning` |
| 错误 | `#ff8080` | `#991b1b` | `#c0392b` → `--color-error` |
| 页面底 | `#060913` | `#f0f4f8` | `#f5f7fa` → `--bg-base` / `--color-bg-page` |
| 卡片面 | `#0d1525` | `#ffffff` | `#ffffff` → `--bg-surface` |
| 主文字 | `#f0f6fc` | `#0f172a` | `#263238` → `--text-primary` |

## 4. 基础组件规范

| 类名 | 语义 | 强制消费的令牌 |
|---|---|---|
| `.o2-card` | 内容容器 | `--bg-surface` + `--radius-lg` + `--shadow-card` + `--space-4` |
| `.o2-btn` | 主/次按钮 | `--color-primary` + `--control-height-md` + `--radius-md` + `--transition-fast`；焦点走全局 focus-visible 环 |
| `.o2-input` | 输入框 | `--border-subtle` + `--radius-md` + `--bg-base`；聚焦用 `--border-active` |
| `.o2-tag` | 状态标签 | `--color-primary-soft` 底 + `--color-primary` 文字 + `--radius-full` |
| `.o2-divider` | 分隔线 | 1px `--border-subtle` |
| `.o2-skeleton` | 加载占位 | 复用 dark.css 的 `.skeleton` shimmer 动画 |

规则：① 组件样式只允许引用令牌，不得出现十六进制字面量；② 交互态（hover/focus/active/disabled）必须覆盖；③ 移动端按钮/可点击行最小 44px；④ 所有动效在 `prefers-reduced-motion` 下收敛（沿用 dark.css 的 0.01ms 策略）。

## 5. 两端落地映射（供桌面端 / 移动端成员实现时参考）

**桌面端成员**：
- `@oa4rust/ui/style.css` 已包含 dark.css + tokens.css；入口 `src/index.ts` 已导入。
- UnoCSS 主题已扩展 `spacing/sm/md/lg/xl/2xl`、`fontSize`、`lineHeight`、`zIndex`、`borderRadius(none/full)` 映射，新增 shortcuts：`o2-card/o2-btn/o2-input/o2-tag/o2-divider`。
- 新页面优先使用 `o2-*` shortcut 与 `var(--token)`，逐步替换硬编码色值（598 处扫描命中）；保留 naive-ui 组件，通过 `n-config-provider` 主题覆盖接入令牌（后续迭代）。

**移动端成员**：
- 在 `App.vue` 的 `page{}` 中引入 `tokens.css`（H5 可 `@import`，小程序可将变量块复制进 `App.vue`），保留现有色彩变量作为过渡别名。
- 逐步把 `#2d8cf0` 等硬编码色值替换为 `var(--color-primary)` 等统一命名；同步在 `uni.scss` 增加 `$space-*`/"$font-size-*" Sass 变量镜像（编译期常量，供 scss 使用）。
- 新增 `page{}` 全局守卫：`focus-visible` 焦点环 + `prefers-reduced-motion` 收敛；按钮/列表项加 `min-height: var(--touch-target-min)`。
- pages.json 的 tabBar 色值需与 `--color-primary` 保持一致（原生 tabBar 不支持 CSS 变量，需在配置中同步字面量，建议写在注释标明来源令牌）。

## 6. 性能与一致性检查清单

- [ ] 新增样式零硬编码色值/间距（全部 `var(--token)` 或 UnoCSS 令牌类）。
- [ ] 桌面端 LCM：`pnpm --filter @oa4rust/desktop build` 通过，chunk 警告不新增。
- [ ] 移动端 H5/小程序编译通过，产物 CSS 无重复 token 定义。
- [ ] 暗/亮主题切换后对比度契约测试仍全绿（desktop 的 `*Contrast.contract.test.ts` 系列）。
- [ ] reduced-motion / focus-visible / skip-link 守卫在两端均生效。
- [ ] 触控目标 ≥44px（移动端）。

## 7. 变更记录

| 日期 | 变更 | 文件 |
|---|---|---|
| 2026-11-19 | 初版：审计双端现状，建立统一令牌层、UnoCSS 映射、组件规范文档 | `packages/ui/src/theme/tokens.css`（新增）、`packages/ui/src/unocss.config.ts`（扩展主题与快捷类）、`packages/ui/src/index.ts`（导入令牌）、本文档（新增） |
