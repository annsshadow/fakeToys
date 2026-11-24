# R466 WCAG 2.2.2 Pause/Stop/Hide 动画暂停机制甄别报告

- 任务：t325 [R466]（attempt a779c856-82ee-41d1-81de-12831f4ac035）
- 类型：只读甄别，不改任何入库源码/契约/台账
- 标准：WCAG 2.2.2 Pause, Stop, Hide（A 级）——持续超过 5 秒且与其他内容并列呈现的自动移动/闪烁/滚动信息，须提供暂停/停止/隐藏机制
- 测试来源：Playwright Chromium headless 真浏览器实测（非物理真机）；PC 1440x900 DPR1 双主题、H5 390x844 DPR2 isMobile/hasTouch
- 证据目录（gitignored）：oa4rust/test-results/r466-pause-stop-hide/；域名/凭据真值仅存在于 gitignored runner，不入库、不回显

## 1. 审计范围与判据

WCAG 2.2.2 仅适用于**自动启动、持续超过 5 秒、且与其他内容并列**的移动/闪烁/滚动动画。加载骨架屏、提交 spinner 等在数据到达后即消失的短期动画通常不满足"持续 5 秒"，不触发本条；常驻动画（如通知未读徽章永久 pulse）若承载信息且无页面内暂停控件，才可能构成可达缺陷。

判据三分：
- **规范预期**：动画 <5s 临时加载态 / 受 reduced-motion 守卫 / 纯装饰不改变信息 / 条件渲染未激活；
- **真实退化**：常驻 >5s、与其他内容并列、承载信息、无页面内暂停控件 → 可达缺陷（标 P 级）；
- **不可达 L1**：源码存在但因业务条件恒不满足而无法在真浏览器触发。

已覆盖维度严禁重复：对比度 R458–R464、骨架样式 R453/R456（本轮只测动画时长非样式）、焦点/标题层级/表格/响应式；reduced-motion（2.3.3）本轮仅作区分讨论，不重复判缺陷。

## 2. 静态盘点（动画名 / 周期 / 触发 / 暂停控件）

全库 infinite 动画命中：PC 107 处、H5 2 处。按性质归类：

| 动画 | 周期 | 定义位置 | 触发/生命周期 | 2.2.2 适用性 |
| --- | --- | --- | --- | --- |
| ai-shimmer / shimmer / pulse（骨架） | 1.2–1.6s | ai-tokens.css:129、dark.css:209-221、各视图 .skel/.skeleton-row | v-if=loading，数据到达即卸载（<5s） | 不触发（临时加载态） |
| spin（登录按钮 spinner） | 0.6s | LoginScreen.vue:538-542 | 仅提交中显示，登录完成即消失 | 不触发（短加载态） |
| spin（OAuth 回调 spinner） | 0.8s | OAuthCallback.vue:72-77 | 回调处理中，跳转完成即消失 | 不触发（短加载态） |
| badge-pulse（通知未读徽章） | 2s | AppShell.vue:802-812 | v-if="unreadCount > 0"，实测 unreadCount 恒为 ref(0) | 不可达 L1（未激活）；即便激活数值文字承载信息、pulse 仅 box-shadow 光晕装饰 |
| typing-indicator（IM 输入中） | 1.2s | IMChat.vue:1136 | 仅"对方输入中"短暂态 | 不触发（临时态，非并列常驻） |

**暂停控件盘点**：全库 21 组实测页面扫描 button/[role=button]/checkbox/[aria-label]，命中"暂停/停止/pause/stop/hide/关闭动画"文案控件 **0 个**。因实测无满足 2.2.2 适用条件的常驻动画，缺少暂停控件不构成缺陷。

reduced-motion 守卫（dark.css:196-204）：

  @media (prefers-reduced-motion: reduce) { * { animation-duration: 0.01ms !important; animation-iteration-count: 1 !important; ... } }

该守卫服务 WCAG 2.3.3（系统偏好降动效），**不能直接豁免 2.2.2 的页面内主动暂停机制**——两者判据不同。本报告据实区分：本库所有适用 2.2.2 的候选动画均为 <5s 临时态或不可达，无需依赖 reduced-motion 守卫即可判定不触发 2.2.2，故不存在"用 2.3.3 冒充 2.2.2 合规"的情形。

## 3. 双探针真浏览器实测

### 3.1 稳定态探针（主 E2E，21 组）

PC 8 页 × 双主题（dashboard/org/file/bbs/calendar/meeting/attendance/common）+ H5 5 页（index/mine/message/calendar/collect），每组等待 2.5s 进入稳定态后采集 document.getAnimations()，间隔 2.5s 再采一次。

- 结果：**21/21 组首采与 2.5s 后复采均为 0 个运行中动画**（count 0/0）；暂停控件 0；pageerror 0；reducedMotion=false（默认无系统偏好）。
- 解读：进入稳定态后页面无任何自动动画在跑，骨架/spinner 均已随数据加载完成卸载，badge 因 unreadCount=0 未渲染。

### 3.2 早期采样探针（API 延迟，决定性时长证据）

为捕获加载期骨架动画，对 /oa4rust/app/admin/common/ 以 page.route 延迟业务 API 4s（放行认证/权限），在 150/600/1200/2400/4200ms 五时点采集 getAnimations() 与骨架节点数：

| 时点 | 运行中动画 | 动画名 | duration | iterations | playState | 骨架节点数 |
| --- | --- | --- | --- | --- | --- | --- |
| 150ms | 5 | ai-shimmer-6d6bff86 | 1600ms | infinite | running | 5 |
| 600ms | 5 | ai-shimmer-6d6bff86 | 1600ms | infinite | running | 5 |
| 1200ms | 5 | ai-shimmer-6d6bff86 | 1600ms | infinite | running | 5 |
| 2400ms | 5 | ai-shimmer-6d6bff86 | 1600ms | infinite | running | 5 |
| 4200ms | 0 | — | — | — | — | 0 |

**决定性结论**：骨架 shimmer 虽迭代无限（infinite），但在真实网络下随数据到达（4200ms 采样前）即随骨架节点一同卸载，生命周期 **<5s**，属 WCAG 2.2.2 明确排除的临时加载态。5s 以上的 API 延迟是人为构造的极端场景；即便该极端下骨架常驻，其信息由加载完成后的真实内容承载、shimmer 纯视觉提示，且属"加载过程"而非"与其他内容并列的信息动画"，仍不构成 2.2.2 缺陷。errors=0。

**证据补正（t326）**：早期探针原截图 early-skeleton-with-api-delay.png 经独立复核发现实为"公共组件库"加载完成态（sha 与 pc-common-light-r466.png 相同），不能佐证骨架运行态。已用补采脚本 r466-early-fixed.cjs 在 page.route 延迟业务 API 6s 期间、骨架节点数=5 且 5 条 ai-shimmer playState=running 的 600ms 时点真实抓取 early-skeleton-running-fixed.png（sha256 83681e49…，目视 5 条灰色骨架占位条 + "加载中..." 清晰可见），名实相符；WAAPI 时间序列测量与上表数值不变。

## 4. CDP / WAAPI 根因坐实

- Web Animations API（document.getAnimations()）在第一手采样中直接读出动画 name/duration/iterations/playState，均来自浏览器渲染引擎真实状态，非源码推断。
- CDP CSS.getMatchedStylesForNode（DOM.enable + CSS.enable 先行）交叉核对：稳定态 .badge/.spinner/.skeleton/.skel/[class*="pulse"] 均无匹配节点（页面未渲染这些元素），与 getAnimations()=0 一致。
- 源码定位：badge-pulse 由 AppShell.vue:802 定义，模板 :112 为 v-if="unreadCount > 0"，:256 const unreadCount = ref(0) 且全文件无赋值语句 → 常驻徽章动画在本库当前业务下不可达。

## 5. 三分判定

- **可达真缺陷：0**。所有实测自动动画均为 <5s 临时加载态（骨架 shimmer、登录/OAuth spinner、IM 输入指示）或不可达（badge-pulse），无任何"持续 >5s、与其他内容并列、承载信息、无暂停控件"的常驻动画。据实判 0，不臆造。
- **不可达 L1：1**。badge-pulse（AppShell.vue:802-812）——unreadCount 恒为 0，徽章条件渲染永不激活；即便激活，未读数由文字承载、pulse 仅 box-shadow 装饰，仍不满足 2.2.2 缺陷定义。
- **正常-规范预期：若干**。骨架 shimmer 临时加载态（<5s）、spinner 短加载态、reduced-motion 守卫（服务 2.3.3，与 2.2.2 判据分离）、稳定态 0 动画 0 暂停控件。

## 6. 截图证据（sha256 命令行原始值）

source：Playwright Chromium headless 真浏览器（非物理真机）。原稳定态 21 张 + 原早期骨架 1 张，另 t326 补采 6 张（骨架运行态 1 + H5 五页各 1），合计 28 张，全部位于 gitignored 证据目录。原 h5-*-r466.png 五张因导航 URL 双 # 缺陷停留同一视图（sha 全同）、原 early-skeleton-with-api-delay.png 实为加载完成态，均已由补正图取代，详见 6.1。

| 截图 | sha256 |
| --- | --- |
| pc-dashboard-light-r466.png | 9d8f5e7434e45b030d1c365434c4141d6f122952b3a0f03e1b93ba3fc781af2d |
| pc-dashboard-dark-r466.png | e79ab0a556f6b77132b3256f9e3e723e7ae6a4aaf5a7ad99e6e4f6afd1ec8b23 |
| pc-org-light-r466.png | e6660e9702b990795f1c5a852f7505011642073e3c429ec7ce6b63727dfb1e7b |
| pc-org-dark-r466.png | 887da0a8cbdb5e02ad1b6aebbbaa7b849f087bd14426b2a46b6ef05040f11738 |
| pc-file-light-r466.png | 066053df18c5f1c676bd606d31b2a046745444692e2e06ce106958f71fe7547f |
| pc-file-dark-r466.png | d7ebe750e0bf26efbca0896ab619ae7dae03e2af697ff92616fa43b1c35b9875 |
| pc-bbs-light-r466.png | 888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105 |
| pc-bbs-dark-r466.png | 6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c |
| pc-calendar-light-r466.png | 8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad |
| pc-calendar-dark-r466.png | 0075d87d57dc86a5bc3963fa095ecbabaa06562542fbaea5922b2234fe9f6c4b |
| pc-meeting-light-r466.png | 94c4840fbd266eeaae6c3287abe09f1d0e3b7e874f6e35ef8a358d73fd891b26 |
| pc-meeting-dark-r466.png | 4f8279150d32c028e5165c2b0636c59d76dfcda93e561bf3dae0af6a347c6b46 |
| pc-attendance-light-r466.png | 678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b |
| pc-attendance-dark-r466.png | 85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7 |
| pc-common-light-r466.png | 33f73040fdde480b96841e417ee4fee1d06884845ba16e4303365e3d9b0db073 |
| pc-common-dark-r466.png | afe79b29a25873e32fabc99ddcb7729ec6aab8faffcec317d88c0adfaca33424 |
| h5-index-r466-fixed.png | c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96 |
| h5-mine-r466-fixed.png | cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385 |
| h5-message-r466-fixed.png | 170482f3818cb89f3b3fd21adf4971b65a72ef834b2b270a600c1e736800b3d4 |
| h5-calendar-r466-fixed.png | 95af2b84a08af75e603a927b7e757e5cf132f6a848c2cd26b82b9fd1e6f6f1fc |
| h5-collect-r466-fixed.png | cbae5e3d08c81c9b2276a9f3c103d6b0094494f6dbda229b8a1f66a2276f0ed6 |
| early-skeleton-running-fixed.png | 83681e49761e73fdbfb5bdc8f5f2033128185232d4a7ec71132748876a6ec7b2 |

目视核对：pc-dashboard-light（浅色工作台，统计卡/待办完整、无可见动效残留）、pc-dashboard-dark（暗色工作台一致）、h5-index-r466-fixed（品牌渐变卡"8 待办审批/1 今日出勤"清晰）、early-skeleton-running-fixed（骨架 shimmer 运行态，5 条灰色骨架占位条 + 加载中提示清晰）。

### 6.1 t326 证据补正对照（补采前后 sha256）

队长独立复核发现原证据两处名实不符，已用真浏览器补采修正（脚本 r466-early-fixed.cjs / r466-h5-fixed.cjs，凭据运行时注入，gitignored）：

| 项目 | 补采前（名实不符） | 补采后（据实） | 依据 |
| --- | --- | --- | --- |
| 骨架运行态 | early-skeleton-with-api-delay.png = 33f73040…（与 pc-common-light 相同，实为加载完成态） | early-skeleton-running-fixed.png = 83681e49… | 600ms 时点 skel=5、5×ai-shimmer running，目视 5 条骨架占位条 |
| H5 index | c531e964…（工作台，本身正确） | h5-index-r466-fixed.png = c531e964…（不变） | bodyText=工作台/你好admin/8待办审批 |
| H5 mine | c531e964…（误停留工作台） | h5-mine-r466-fixed.png = cf939600… | bodyText=我的/admin/修改密码 |
| H5 message | c531e964…（误停留工作台） | h5-message-r466-fixed.png = 170482f3… | bodyText=消息/单聊会话列表 |
| H5 calendar | c531e964…（误停留工作台） | h5-calendar-r466-fixed.png = 95af2b84… | bodyText=日历/今日暂无日程 |
| H5 collect | c531e964…（误停留工作台） | h5-collect-r466-fixed.png = cbae5e3d… | bodyText=我的收藏/暂无收藏 |

根因：原 r466-e2e.cjs H5 段 route 已含 # 前缀，拼接时再补 # 形成双 #（##/pages/...），uni-app 路由失效停留工作台；补采脚本改用单 # 正确路由 #/pages/{page}，单 context 逐页导航，五页 bodyText 与 sha 均各异。核心结论（稳定态 21 组 count=0、骨架 <5s WAAPI 时间序列、badge-pulse 不可达）不因证据补正改变，仍为 0 可达真缺陷。

## 7. 真值纪律

本报告仅使用占位符 <YOUR_TEST_DOMAIN> / <TEST_HOST>；测试服域名、SSH 主机、端口、管理员口令真值只存在于 gitignored runner 与部署机凭据文件，未写入任何入库文档；截图来源如实标注为 Playwright Chromium headless 非物理真机，未冒充物理设备。

## 8. 结论

R466 WCAG 2.2.2 动画暂停机制甄别：**可达真缺陷 0 / 不可达 L1 1 / 正常-规范预期若干**。不臆造缺陷；≥1000 轮持续迭代中一环，非终态。
