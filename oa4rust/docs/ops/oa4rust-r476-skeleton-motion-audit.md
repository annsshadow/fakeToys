# R476 AI骨架屏加载动效一致性甄别（PC双主题·真浏览器据实·禁预判）

- 任务：t343，attempt 347407ac-7c4d-4b21-a103-8b53b006b4da
- 维度：PC端骨架屏两种加载视觉语言并存——opacity脉冲（pulse纯色呼吸）vs AI三色流光（ai-shimmer背景位移）
- 真父：687f2407e9f2871f88b8fc1e64d6b7b54f9eafb1（HEAD=UP AB=0/0 工作树干净）
- 真值纪律：真域名/端口/凭据仅存在于 gitignored runner，本文一律用 <YOUR_TEST_DOMAIN>/<TEST_HOST> 占位，0 真值入库

## 1. 审计概况

- 环境：Playwright Chromium headless，PC 1440x900 DPR1，light/dark 双主题，非物理真机。
- 选页：8 个真实可达页 × 双主题 = 16 组。pulse 候选 5 页（collect/cms-column/cms-index/deployment/note）；shimmer 候选 3 页（fileinfo/filemgr/meeting）。
- R306C 硬排除页（FindDesignerApp/CommonApp/PdfViewerApp/ServiceInvoke 等）未纳入实测，据实不判 L1。
- 已有覆盖：R453 判 H5 骨架 0 缺陷、R456/R457 已修 PC 部分纯色骨架、令牌差集已穷尽、R475 判 glass 0 可达。本轮聚焦两套加载语言在同功能域是否可见割裂。

## 2. 方法（运行时取证，规避 R466 名实不符陷阱）

- 先登录并注入主题（localStorage oa4rust_theme + data-theme），再对 page.route 放行认证/权限/菜单/主题 API，仅延迟业务 API 4500ms 触发 loading 态。
- 导航后在 500/900/1400/2000/2800ms 时点轮询骨架元素数量；当 count>=3 立即以 getComputedStyle + document.getAnimations() 取证并截图。
- 决定性：16/16 组在 atMs=500 即捕获骨架运行态（count 5-6，running 动画至少 5 个），全部早于 4500ms 延迟解除，非加载完成态，证据名实相符。
- CDP：先 DOM.enable + CSS.enable，再 CSS.getMatchedStylesForNode 取命中规则来源；getAnimations 提供 playState=running 实证。
- 结果：R476_DONE groups=16 observed=16 errors=0，0 pageerror。

## 3. 逐页实测表（16组）

说明：animName 后缀为 Vue scoped 哈希；pulse=纯色透明度呼吸，shimmer=三色渐变背景位移。opacity 列为取证瞬时值（呼吸中）。

| 页 | 主题 | animation-name | 时长 | background-image | background-color | 元素数 | 归类 |
|---|---|---|---|---|---|---|---|
| collect | light | pulse-a65b0f1d | 1.5s | none | rgb(248,250,252) | 5 | pulse纯色 |
| collect | dark | pulse-a65b0f1d | 1.5s | none | rgb(19,30,51) | 5 | pulse纯色 |
| cms-column | light | pulse-c60a9a6b | 1.5s | none | rgb(248,250,252) | 5 | pulse纯色 |
| cms-column | dark | pulse-c60a9a6b | 1.5s | none | rgb(19,30,51) | 5 | pulse纯色 |
| cms-index | light | pulse-35a8f62a | 1.5s | none | rgb(248,250,252) | 5 | pulse纯色 |
| cms-index | dark | pulse-35a8f62a | 1.5s | none | rgb(19,30,51) | 5 | pulse纯色 |
| deployment | light | pulse-82381ca8 | 1.5s | none | rgb(248,250,252) | 5 | pulse纯色 |
| deployment | dark | pulse-82381ca8 | 1.5s | none | rgb(19,30,51) | 5 | pulse纯色 |
| note | light | pulse-d967f126 | 1.5s | none | rgb(248,250,252) | 5 | pulse纯色(见第5节) |
| note | dark | pulse-d967f126 | 1.5s | none | rgb(19,30,51) | 5 | pulse纯色(见第5节) |
| fileinfo | light | ai-shimmer-4e0a598b | 1.6s | linear-gradient(90deg, rgb(248,250,252) 25%, rgba(15,23,42,0.08) 50%, rgb(248,250,252) 75%) | rgba(0,0,0,0) | 6 | shimmer流光 |
| fileinfo | dark | ai-shimmer-4e0a598b | 1.6s | linear-gradient(90deg, rgb(19,30,51) 25%, rgba(148,163,184,0.18) 50%, rgb(19,30,51) 75%) | rgba(0,0,0,0) | 6 | shimmer流光 |
| filemgr | light | ai-shimmer | 1.6s | linear-gradient(90deg, rgb(248,250,252) 25%, rgba(15,23,42,0.08) 50%, rgb(248,250,252) 75%) | rgba(0,0,0,0) | 6 | shimmer流光 |
| filemgr | dark | ai-shimmer | 1.6s | linear-gradient(90deg, rgb(19,30,51) 25%, rgba(148,163,184,0.18) 50%, rgb(19,30,51) 75%) | rgba(0,0,0,0) | 6 | shimmer流光 |
| meeting | light | ai-shimmer-ef93c602 | 1.4s | linear-gradient(90deg, rgb(248,250,252) 25%, rgba(15,23,42,0.08) 50%, rgb(248,250,252) 75%) | rgba(0,0,0,0) | 5 | shimmer流光 |
| meeting | dark | ai-shimmer-ef93c602 | 1.4s | linear-gradient(90deg, rgb(19,30,51) 25%, rgba(148,163,184,0.18) 50%, rgb(19,30,51) 75%) | rgba(0,0,0,0) | 5 | shimmer流光 |

分类汇总：PULSE_SOLID=10 组、SHIMMER_GRADIENT=6 组。shimmer 组 background-size 均为 200% 100%（位移基础），pulse 组 background-size=auto。

## 4. CDP 规则来源与两套体系对照

- pulse 体系（collect/cms-column/cms-index/deployment/note）：命中页 scoped 规则 .skel[data-v-xxx] → 「background:var(--bg-elevated)」+「animation:pulse-xxx 1.5s infinite ease」，伴随 scoped @keyframes（opacity 1→0.4）。来源为组件内联 scoped 样式，非 AI 令牌；背景纯色、仅透明度呼吸，无渐变位移。
- shimmer/fileinfo：命中 .sk[data-v-4e0a598b] →「background:linear-gradient(90deg,var(--ai-skeleton-base) 25%,var(--ai-skeleton-shine) 50%,...)」+「animation:ai-shimmer-4e0a598b 1.6s var(--ai-ease-spring) infinite」+「background-size:200% 100%」。AI 令牌生效。
- shimmer/filemgr：双规则并存且同源——全局 .ai-skeleton-line（ai-tokens.css:124-129）+ scoped .skeleton-row[data-v-b72bbe1c]/.sk[data-v]，均为 shimmer 三色渐变，构成全局+scoped 双保险（R456/R457 修复成果）。
- shimmer/meeting：先命中基线 .sk[data-v-ef93c602]（纯色），再命中高特异性 .ai-page .sk[data-v-ef93c602] 的 shimmer 渐变（1.4s ease-in-out）后置救回——与 R455/R457 的救回模式一致，最终计算值为 shimmer。
- 令牌实证（从计算值反推，ai-tokens.css:90-91 light / :51-52 dark）：light base=rgb(248,250,252)、shine=rgba(15,23,42,0.08)；dark base=rgb(19,30,51)、shine=rgba(148,163,184,0.18)。两套体系下令牌解析均正确。

## 5. 技术观察（非缺陷）：NoteApp 同元素两套并存

- CDP 显示 note 的 .skel[data-v-d967f126] 同时命中两条规则：第一条 :184 shimmer 三色渐变+ai-shimmer 1.6s；第二条 :215 后置「background:var(--bg-elevated)」+「animation:pulse 1.5s infinite」。
- 后置 pulse 特异性相同而胜出，最终计算值 animName=pulse-d967f126、bgImage=none——AI 流光增强未在该页胜出。
- 据实定性：该页 pulse 骨架仍清晰可见（加载中文案+呼吸条），功能完整、无不可读、无布局崩坏，不构成可见退化，按队长判据不判 P 级。记为技术观察，供后续可选统一（删除 :215-216 遗留 pulse 或提升 :184 特异性），本轮只读不改码。

## 6. 按钮反馈与令牌一致性

- 16/16 组页面首个按钮（主题切换）transition=0.14s cubic-bezier(0.16, 1, 0.3, 1)，与 var(--ai-motion-fast)+var(--ai-ease-spring) 一致；disabled=false。
- 按钮 color：light=rgb(71,85,105)、dark=rgb(139,157,195)，随主题切换，与 AI 令牌语义一致。骨架体系差异不影响按钮反馈。

## 7. 目视证据

- 6 张代表图（light: collect/fileinfo/note/meeting；dark: collect/fileinfo）均 1440x900，名实相符：骨架条+「加载中...」文案清晰，渐变标题正常，无布局崩坏。
- pulse 页（collect）为纯色条呼吸+加载文案；shimmer 页（fileinfo/meeting）为三色流光条+完整控件；note 加载态标题「笔记管理」渐变正常（R472 修复零回归）。
- light 与 dark 同页加载态结构一致、仅配色随主题切换，无主题失衡。

## 8. 三分判定

- 可达真缺陷：0。两套体系都实现可见加载占位，双主题清晰无崩坏、无不可读；跨页差异分属不同功能域（内容管理域 pulse vs 文件/会议域 shimmer），属设计分工；NoteApp 同文件 pulse 覆盖 shimmer 无可见退化实证。据实判 0，不臆造、不凑轮次。
- 不可达 L1：0。8 页 loading 全部可达、骨架全部运行中捕获、选择器全部命中；已避开 R306C 硬排除页，无 L1 消耗。
- 正常-规范预期：若干（16组渲染+pulse/shimmer 两套令牌链+button 反馈一致）；附 1 项技术观察（NoteApp 遗留 pulse 覆盖 shimmer，非缺陷）。

## 9. 截图 sha256（命令行原始值，16张）

1623184af01ac5bc0a32d504791f7d8567cce64c4dc179c037d281c222fdb889  pc-collect-light-skeleton.png
8a90e3931f3f1e027ec901322bf5c9f19c7b038f181efe8856e0ae9ad8204e8c  pc-collect-dark-skeleton.png
bb096bb122ed8a007131b3d51f488734b0b028b6cd20ed9368c57efab3f67477  pc-cms-column-light-skeleton.png
d38564016a71b051ffd5535c6ed388d710f3386f8529c41da91ec8e2ef85c9d5  pc-cms-column-dark-skeleton.png
547971d23d079e7d7a50bf18b2e967bf4a1a4b1c66477b01eb3ac0eb5a8bbd2f  pc-cms-index-light-skeleton.png
84e448342a75e7c7a2177eb2ba92dbbd784345f8369242f05e4d921ac7947ff6  pc-cms-index-dark-skeleton.png
8c37983ad982af0fb74b77abb1aa1386dba770881aba7ced973fecc2889afde0  pc-deployment-light-skeleton.png
fc7c5082d8f6d2f85532f27c74ca0d611e1d77a89a42cd89265aeb2ecf54dbe5  pc-deployment-dark-skeleton.png
ac17ab091c0195e6af109a00c2a4d799d27f98c756e3ea4e289fc16e408ff0ee  pc-note-light-skeleton.png
c11463362c269dde580e0b37be8fd8f6ff5f605b44d8579f5a9a5f968340afc4  pc-note-dark-skeleton.png
5d6003874a973bb044d0ae5cac9c914ff61c7b0b1e4965fd993825d80c6f8b69  pc-fileinfo-light-skeleton.png
f72af4f24de36c567db6339060a5ae69925df20d5a9720a17dc2c07140ac8978  pc-fileinfo-dark-skeleton.png
bc6707ce03dd0a7d1b55540e8b348da4842a25fcbb03448c531f4f253a2da14a  pc-filemgr-light-skeleton.png
78d1c72b82f8e63c8e2b8a9f82ee078cbe8af0593a797a2989494cd5a5c64617  pc-filemgr-dark-skeleton.png
1ffe6b1137f21927b0338806d593d4113640e2d2284d2956a5e28c42c6d5a2f8  pc-meeting-light-skeleton.png
5cfe3bf24f27f297dc91568dc65e2ae244d4b6ebfa2465d62ac86c21a5066c78  pc-meeting-dark-skeleton.png

## 10. 真值纪律

- 文档仅含占位符 <YOUR_TEST_DOMAIN>/<TEST_HOST>；无真域名、IP、端口、口令、个人路径。
- 截图与报告入 gitignored 目录 oa4rust/test-results/r476-skeleton-motion/（.gitignore 拦截），真域名与凭据仅在 gitignored runner 运行时从受控凭据源提取注入进程环境，不回显、不入库。
- 截图标注来源：Playwright Chromium headless 非物理真机，未冒充物理设备。
- 证据路径：oa4rust/test-results/r476-skeleton-motion/（16 PNG + report.json + analysis.txt）；脚本 r476-e2e.cjs / r476-analyze.cjs / r476-run.sh 均 gitignored。
