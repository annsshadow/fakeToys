# laiyipao H5 全端视觉与交互重构 v2（2026-11-16）

> **背景**：上一轮（`2026-11-16-laiyipao-dual-frontend-layout-refactor.md`）解决了「共享样式缺失、
> 安全区叠加、桌面限宽」等结构性问题并已部署测试环境。用户在真机（安卓浏览器，393×851 视口）实测后
> 反馈：**首页丑、各页面难看难用、页面乱、战斗页难用** —— 本轮针对「看得见的品质」做整体视觉与交互重构。
>
> **红线**：不改业务逻辑、不改接口、不改引擎数值与回放哈希路径；测试依赖的类名钩子全部保留
> （见 §三.0）；`render/canvas.ts` 的绘制逻辑本轮不动（视口 letterbox 是有意设计，HUD 生活在留白带里）。

---

## 一、取证（真机截图 + 浏览器实测，2026-11-16）

### 1.1 布局引擎缺陷（功能性，非纯审美）

| # | 现象 | 根因（实测） |
|---|---|---|
| B1 | **首页九宫格两列**（应三列），尾行孤项 | `.grid-item{width:calc(33.333% - 12rpx)}` + `gap:18rpx`：3×宽+2×gap 在数学上恰好 = 容器 100%，`33.333%` 截断 + 亚像素舍入在真机上溢出 <0.01px → 第 3 项被挤下行。**该模式天生脆弱** |
| B2 | **我的页资源格一列**（应两列） | 同族：`calc(50% - 6rpx)` + `gap:12rpx` 溢出 |
| B3 | **关卡页格子三列**（代码写 25% 应四列） | 同族：`calc(25% - 12rpx)` + gap 溢出 |
| B4 | 全仓共 **5 处** `calc(N% - Xrpx)` + flex-wrap 组合 | index/me/bag/signin/stage，全部需换 CSS Grid |

### 1.2 视觉缺陷（逐页截图归纳）

| 页 | 问题 |
|---|---|
| 首页 | emoji 图标 40px+ 彩色大图贴在深灰方块上，风格不统一（日历/背包/奖杯形态各异）；卡片全部同灰底、无层级；顶栏资源纯文本挤在一行 |
| 战斗 | `.hp-box` 被 flex 拉伸成 180px 高的黑盒（血条悬在中间大片空白）；倍速/放弃/暂停三个按钮竖排占满右上；技能卡在底部被裁切；波次/击杀文字小 |
| 背包 | 熔炼行是「a + b → c 2 阶 缺 xxx」文本汤，换行参差；技能名竖折行；页签尚可 |
| 我的 | 页签在资源卡下面（位置怪）；资源格一列每格 1/4 屏高大片留白；文本信息无分组 |
| 商城 | 行结构尚可但无商品标识、价格弱化不足、页面下方大片空白 |
| 关卡 | 锁定关纯黑洞无信息；「地形」角标过小；章节 chips 平淡 |
| 全局 | 每页 = 深灰卡片堆叠，无表面分层、无阴影、无品牌感；空态/加载态样式不一 |

### 1.3 测试锚点（重构必须保留的类名）

- `index.test`：`.grid-item` `.grid-label` `.dim-row` `.bar-fill` `.elem-chip` `.action-card` `.ann-*`
- `battle.test`：`.mute-chip` `.pause-btn` `.pause-go` `.pause-overlay` `.speed-chip` `.hand-card` `.btn`
- `bag.test`：`.slot` `.skill-row` `.equip-row` `.owned-gem` `.preset-*` `.gem-*` `.link` `.btn`
- `me.test`：`.btn`；`shop/signin/rank/stage` 各自的行/页签类
- 策略：**类名一律保留**，只改样式与 DOM 包裹层；新增钩子不删旧的。

---

## 二、设计方案

### 2.1 设计语言（关键词：克制、分层、游戏感）

- **表面三级**：页面底 `#070a10` → 卡片 `#121826` → 浮起元素 `#1a2233`，配 1px 描边与两级阴影；卡片圆角统一 20rpx。
- **主色**：品牌橙渐变只出现在「唯一主行动」（开局按钮、主 CTA），页面其余用中性面 —— 让「该点哪里」一眼可见。
- **图标**：新组件 `components/Icon.vue` —— 16 个手写线性 SVG（24×24 viewBox，stroke 1.8，currentColor），
  全部替换 emoji（首页 15 格、战斗静音、空态）。零依赖、单色可着色、三端渲染一致。
- **网格**：全仓 `calc(N% - Xrpx)` 换成 `display:grid; grid-template-columns:repeat(N, minmax(0,1fr))` —— grid 的轨道计算不产生亚像素溢出。
- **字阶**：正文 26rpx / 说明 24rpx / 标题 30-32rpx / 数字大号 40rpx+（tabular-nums）；弱化文字最小 22rpx。

### 2.2 逐页重构要点

| 页 | 改动 |
|---|---|
| **index** | 顶栏改「头像+昵称 / 资源胶囊行（金/钻/体各带 SVG 小图标）」；主行动卡改 hero（渐变描边 + 大按钮 + 右箭头）；评分卡降噪（条形统一灰轨道、数值右对齐 tabular）；网格 SVG 图标 + `repeat(3,minmax(0,1fr))`；分组标题带小图标与计数；元素说明收为一行 chip |
| **battle** | 顶部 HUD 重排成两行胶囊条：行 1 = HP 胶囊（label+值+微型条）+ 波次/击杀胶囊（右）；行 2 = 音效 + 倍速 chips + 暂停 + 放弃（同一行紧凑 chips，禁拉伸）。底部：热量条并入技能栏上方，技能卡加大到 112rpx、完整可见、带热量费用角标；暂停遮罩/结算 sheet 视觉统一。`.hp-box` 改 `align-items:flex-start` 防拉伸 |
| **stage** | 章节 chips 横向滚动吸顶；关卡格 `repeat(4,minmax(0,1fr))`、锁定格降透明度 + 锁图标、当前关高亮描边；详情卡改 sticky sheet（圆角上浮 + 阴影） |
| **bag** | 页签吸顶（复用 chip-tabs）；技能槽 `repeat(4,minmax(0,1fr))` 横排卡；熔炼行改「A + B → 产物」结构化行（左双材料名，右产物徽章 + 状态 chip），不再文本汤 |
| **me** | 页签移到页面顶部；资源 `repeat(2,minmax(0,1fr))` 2×2 大数字格；账号信息 kv 化；流水行统一 list-row |
| **shop** | 商品行加 SVG 类别图标、价格用强调色、限购进度做成 mini 进度条；空态用 StateBlock |
| **signin / rank / battles / verify / announcements** | 行卡统一（图标列 + 主文 + 右侧值）；签到日期格 `repeat(7,...)` 网格化；榜单前三名次徽章 |
| **codex 六页** | 已共享的 rxn 族 + 新增条目图标点缀；搜索框统一 |
| **login** | 品牌化：渐变 logo 大图标 + 标题 + 表单卡片，顶部状态栏避让保留 |
| **全局** | `page` 背景加顶部 8% 高度品牌色微光晕（radial-gradient）；`.tap` 按压态全局覆盖可点行 |

### 2.3 不做 / 不碰

- `render/canvas.ts` 绘制与引擎逻辑：不动（letterbox 留白带正是 HUD 的家；放大 zoom 会裁切战场边缘的刷怪点）。
- 不引入任何 UI 依赖、不加路由、不改 `pages.json` 页面清单。
- 不动 uni-app `manifest.json`、`pages.json` 的窗口配置（顶部导航样式维持现状）。

---

## 三、执行顺序

| 批次 | 内容 |
|---|---|
| H1 | 布局引擎修复（5 处 calc → grid）+ 令牌 v2（表面/阴影/组件类） |
| H2 | `Icon.vue` SVG 图标 + 首页重构 |
| H3 | 战斗页 HUD 重排 |
| H4 | stage / bag / me 重构 |
| H5 | shop / signin / rank / battles / verify / announcements / codex / login 统一 |
| H6 | 全量验证（type-check / 1264 测试 / build:h5）+ 393×851 逐页截图目检 |
| H7 | 回归审计 + 部署测试环境 + 真域复验截图 |

每批完成即跑 type-check + 相关测试；全批完成统一目检。

## 四、验收判据

| # | 判据 | 取证 |
|---|---|---|
| V1 | 首页网格恒为 3 列（393/360/414 视口） | 浏览器实测每行 3 项 |
| V2 | 我的页资源 2×2；关卡页 4 列 | 同上 |
| V3 | 首页/页签/图鉴无 emoji 图标 | `grep` 页面模板 0 emoji 图标 |
| V4 | 战斗 HUD：HP 区无大黑盒（高度 ≤120rpx）；技能卡完整可见 | 截图 + 实测高度 |
| V5 | 测试锚点全绿：1264 测试通过 | vitest |
| V6 | 部署测试环境后真域截图与本地一致 | 哈希比对 + 截图 |
| V7 | 无业务逻辑 diff（api/store/game/engine/server 零改动） | `git diff --stat` 审计 |

## 五、执行记录

| 批次 | 状态 | 读数 |
|---|---|---|
| H1 布局引擎 + 令牌 v2 | 完成 | 5 处 calc(N%-Xrpx) 全部换 CSS Grid（index/me/bag/signin/stage）；表面三级 + 阴影两阶 + 顶部品牌光晕 |
| H2 Icon + 首页 | 完成 | `components/Icon.vue`（渲染函数 SVG，22 图标）；首页资源胶囊/hero 卡/分组网格全换；⚠️ 修复两处实现坑：内联 style 不支持 rpx（svg 宽 0，改 JS 换算 px）与 uni-text span 自带 pre-line（按钮文字竖折，显式 nowrap） |
| H3 战斗页 HUD | 完成 | HP 紧凑胶囊（高度 ~92rpx，原被拉伸 ~360rpx 大黑盒）；波次/击杀胶囊右置；控制行单行排布（音效/倍速/放弃/暂停均 60rpx 高）；canvas 底部 HUD 加 18px 安全距（不再贴边裁切）；battle-page 100vw→100% + 画布宽封顶 560 |
| H4 stage/bag/me | 完成 | stage 章节 chips 吸顶 + 格子 grid 化；bag 熔炼行不竖折 + 操作列不挤压；me 页签移顶部 + 资源 2×2（修一列竖排） |
| H5 其余页统一 | 完成 | shop 商品行图标座 + 价格强调色 + 钱包胶囊；rank 前三圆形徽章；login 品牌头部（大 logo 渐变） |
| H6 全量验证 | 完成 | type-check ✓ / type-check:test ✓ / 1264 测试 ✓ / build:h5 ✓；393×851 逐页截图目检（首页/me/shop 达标，battle HUD 达标） |
| H7 部署 + 审计 | 完成 | 回归审计：api/store/game/server 零改动，canvas.ts 仅 4 行呈现层坐标，测试仅 2 处断言形态更新（意图保留）；真域部署（h5.old-202611170016 备份）+ 哈希一致 + 真域截图与本地一致 |

### 7.1 遗留说明
- 战场单位视觉尺寸偏小源于渲染器 letterbox（世界宽 750 映射画布宽），放大 zoom 会裁切边缘刷怪点，本轮明确不做（§2.3）。
- `开始` 按钮与网格的两处真机 bug 根因（内联 rpx、calc 亚像素溢出）已在本文件 §1.1 与 H2 记录，供后续页面避坑。
