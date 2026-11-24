# R453 H5 骨架屏与品牌渐变态真浏览器实证核查

- 任务：t309（design-lead，Kind=work，只读甄别、不改源码）
- attempt_id：6152ca4f-fee6-456f-a3f5-83a04b649f72
- 基线：分支 opt1000/oa4rust-pc-ai-refactor，HEAD=UP=eff9ff989c3dcf9336ef90d5870f59fd2816074f（任务描述的 c60ad3cdb 已过时，以 git 实况为准），工作树干净
- 测试入口：https://<YOUR_TEST_DOMAIN>/oa4rust-h5/#/pages/...（<TEST_HOST> 真值仅存在于 gitignored 运行脚本，不入库）
- 证据目录：oa4rust/test-results/r453-h5-skeleton/（10 PNG + report.json，均 gitignored）

## 1 方法与真实性声明

- 真浏览器：Playwright Chromium headless，viewport 390x844、DPR2、isMobile、hasTouch。
- 来源标注：测试服真实页面（NOT a physical device，非物理真机），全程未注入伪造业务数据。
- 凭据：ssh oa4rust-test 运行时从 /usr/local/oa4rust/.admin-credentials 正则提取，注入 OA_TEST_USER/OA_TEST_PASS；域名真值只在 gitignored runner，仓库内 0 真值。
- 骨架屏触发：用 page.route 对真实业务 API 延迟 6000ms 再 continue，令 loading 态真实出现；显式放行 /api/authentication/who，避免 auth-guard 的 ensureAuthenticated 被延迟阻塞导致 onShow 挂起（首轮全页 observed=false 的根因，已修正重测）。
- 采集：computedStyle（backgroundImage/backgroundSize/animationName/Duration/TimingFunction/IterationCount/display/gap）、getAnimations（playState/currentTime/timing）、CDP CSS.getMatchedStylesForNode 规则文本、截图 sha256。
- 结果：loggedIn=true，errors=[]，apiFailures=11 条全为被延迟的真实业务 API（延迟触发预期内）。

## 2 骨架屏实测（10 页，7 页真实触发）

7 页 observed=true：attendance、browse、calendar、collect、doc、process-start、recycle。每页实测值一致：

- .ai-skeleton-list：display=flex，flex-direction=column，gap=12px，padding=16px 0px
- .ai-skeleton-line（含 .short）：height=12px，border-radius=6px，backgroundImage=linear-gradient(90deg, rgb(240, 242, 245) 25%, rgba(15, 23, 42, 0.08) 50%, rgb(240, 242, 245) 75%)，backgroundSize=200% 100%
- animationName=ai-shimmer，animationDuration=1.6s，animationTimingFunction=cubic-bezier(0.16, 1, 0.3, 1)，animationIterationCount=infinite
- aria-hidden=true；getAnimations 首条 playState=running，timing.duration=1600ms，currentTime 介于 16~34ms（采样瞬间），证明动画在运行
- CDP：匹配规则含 .ai-skeleton-line { height:12px;background:linear-gradient(90deg,var(--ai-skeleton-base) 25%,var(--ai-skeleton-shine) 50%,var(--ai-skeleton-base) 75%);background-size:200% 100%;animation:ai-shimmer 1.6s var(--ai-ease-spring) infinite } 与 .ai-skeleton-list { display:flex;flex-direction:column;gap:12px;padding:16px 0 }

逐页 DOM（lineCount 均为 4 = 3 行 + 1 行 .short，artifact 与模板 v-for i in 3 + .short 一致）：

| 页 | 路由 | 触发条件 | observed | lineCount | anim 状态 |
| --- | --- | --- | --- | --- | --- |
| attendance | pages/attendance/attendance | loading 且未预加载 | true | 4 | running |
| browse | pages/browse/browse | loading | true | 4 | running |
| calendar | pages/calendar/calendar | eventsView === loading | true | 4 | running |
| collect | pages/collect/collect | loading 且未 loaded | true | 4 | running |
| doc | pages/doc/doc | loading 且未 loaded | true | 4 | running |
| process-start | pages/process/start | loading | true | 4 | running |
| recycle | pages/recycle/recycle | listView === loading | true | 4 | running |

## 3 未触发 3 页根因（源码坐实，非缺陷）

- bbs-topic（pages/bbs/topic）：observed=false，bodyText=主题详情 主题不存在或已被删除。源码 topic.vue:70-77 onLoad 要求 options.id，本轮直达页无 id 参数，topicId.value 为空即 return，未调用 load()，loading 始终 false；模板 :91 显示业务空态。属正常业务态，骨架屏未进入是因无加载动作。
- chat（pages/chat/chat）：observed=false，bodyText=即时通讯 会话 暂无消息 发消息… 发送。源码 chat.vue:140-148 onLoad 要求 options.id 才 loadHistory()；无会话 id 时不触发加载，属正常入口态。
- search（pages/search/search）：observed=false，bodyText=搜索 搜索文档、主题、数据… 搜索。源码 search.vue:22-24 onShow 仅 ensureAuthenticated，loading 只在 doSearch()（:26-29）内置位；本轮无关键词输入，未发起搜索，属正常待输入态。

三页 observed=false 均因真实业务条件（参数/交互）未满足，非 .ai-skeleton-* 样式失效；按三分判定归入「正常/非缺陷」。

## 4 品牌渐变与按压反馈（8 页复核）

- index：brandCount=2，两元素 bgImage=linear-gradient(135deg, rgb(79, 70, 229) 0%, rgb(8, 145, 178) 100%)，color=rgb(255,255,255)，transition=transform 0.14s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.14s cubic-bezier(0.16, 1, 0.3, 1)；pressCount=15。CDP 含全局 .ai-brand-fill 与提权规则 uni-page-body .ai-brand-fill.ai-brand-fill, body .ai-brand-fill.ai-brand-fill。
- login：brandCount=2，渐变/颜色/transition 与 index 一致，pressCount=15，提权规则同上。
- browse：brandCount=4，3 元素渐变生效；第 2 个 .tab.ai-brand-fill.on 的 bgImage=none/bgColor=rgb(45,140,240) 为选中态业务色覆盖（tab 选中高亮语义），pressCount=2。
- collect：brandCount=1，渐变生效，pressCount=2；.ai-pressable transition 同上。
- attendance：brandCount=2。第 1 个 .btn.ai-brand-fill bgImage=none/bgColor=rgb(192,196,204) 为业务禁用灰显；CDP 显示 uni-button[disabled] 与 .btn[disabled][data-v] { background:var(--color-placeholder) } 特异性压过渐变提权，属业务语义禁用态。第 2 个 .btn.leave-btn.ai-brand-fill 渐变正常。
- message：brandCount=0，pressCount=0 —— 复核 R451 结论一致，DOM 无 .ai-brand-fill 元素。
- doc：brandCount=0，pressCount=1（press transition 生效）；复核 R451 一致。
- bbs-topic：brandCount=0，pressCount=0；复核 R451 一致。

品牌提权结论：凡真实渲染的 .ai-brand-fill（index/login/browse/collect/attendance 提交按钮）均解析为品牌渐变；disabled/选中态等业务色覆盖属正常语义，非 AI 类缺陷。message/doc/bbs-topic 的 brandCount=0 是 DOM 实际无该元素（条件渲染未出），与 R451 判定一致，不构成新缺陷。

## 5 App.vue 样式定义精确行（只读引用，未改）

- oa4rust/mobile/src/App.vue:296-303 .ai-brand-fill 背景与重复类提权（含 :299 特异性注释）
- App.vue:318-326 .ai-surface-gradient 背景与 R452 提权
- App.vue:327-333 .ai-skeleton-line：height 12px、linear-gradient(90deg, --ai-skeleton-base 25%, --ai-skeleton-shine 50%, --ai-skeleton-base 75%)、background-size 200% 100%、animation ai-shimmer 1.6s var(--ai-ease-spring) infinite
- App.vue:334-337 @keyframes ai-shimmer：0% background-position 200% 0 / 100% -200% 0
- App.vue:339-344 .ai-skeleton-list：display flex、column、gap 12px、padding 16px 0；:345-347 .short width 58%
- App.vue:349-355 .ai-pressable transition 与 :active scale(0.97)/opacity .82；:356-361 prefers-reduced-motion 关闭
- 令牌（App.vue:60 区）：--ai-skeleton-base=var(--color-divider)、--ai-skeleton-shine=rgba(15,23,42,0.08)、--ai-ease-spring=cubic-bezier(0.16,1,0.3,1)。实测 rgb(240,242,245)=--color-divider 亮色值，rgba(15,23,42,0.08)=--ai-skeleton-shine，均解析成功。

## 6 三分判定

- ① 可达真缺陷：0 项。骨架屏 7 个真实加载页全部呈现预期渐变灰条与 1.6s 无限流光动画；品牌渐变、按压 transition 均生效；无未定义令牌导致的声明丢弃。
- ② 不可达 L1：0 项。本轮无错误令牌残留（R448 的 CalendarApp --ai-text-* 已由 t302 修复；R451 的 .ai-surface-gradient 已被 R452 提权修复，本轮不再复现）。
- ③ 正常/非缺陷：3 页骨架屏未触发（无参数/无交互）；attendance 禁用按钮灰显、browse 选中 tab 业务色、brandCount=0 三页元素未渲染——均据实记录，不臆造为缺陷。

## 7 截图 sha256（sha256sum 命令行原始值）

- skeleton-attendance.png 88eb4c2210d3b721d4b1c8ccf32af0048387b08cc9649a0603f400b94d834751
- skeleton-bbs-topic.png 5f1daef87720e4bfc87dba2cf044e2c03cb4271196fa60985dd91d8c7e1366e9
- skeleton-browse.png eb2b3264e1c90c4723e6e6556d2fd331f28983661bbffd43b14a1327388d012e
- skeleton-calendar.png a218ea5a8efbe1db387be1f2ea65ff5ced98972c665e35ef8c7a0f616311ba70
- skeleton-chat.png 8baa9b0c304216ce4836a60e5440e5c7cdd47de24793762c64096a1784227d87
- skeleton-collect.png 927f2e4585c6d9434614f844f6b5f347017f1d6ecb328a43834bb24aa30b7504
- skeleton-doc.png 4b9c6d376d73e27319d44111f03a37edd664ab2e14835ffcb2d4a3ac81f5583f
- skeleton-process-start.png b41322dc10091315ecf3359fd43fc8388e60ed19ede59e3d8ad8406263152214
- skeleton-recycle.png 59e9fce8274d8412642e200f6dd1ed5922c9bf60d2573e2305365cfcaf3522f7
- skeleton-search.png 0f76fbd9b8f7e24fb3cd97d57eb23422d9da9f4aacc4e8de1054714c66b01569

目视抽样：attendance（骨架灰条 + 底部导航）、browse（品牌 tabs 渐变 + 骨架灰条）、doc（拍照/上传按钮 + 骨架灰条）三页视觉正常；bbs-topic 显示主题不存在的业务空态，与 DOM 判定一致。

## 8 真值纪律与边界

- 文档内测试域用 <YOUR_TEST_DOMAIN>、<TEST_HOST> 占位，0 真值；无密钥、口令、IP、个人路径。
- 全程只读甄别，未修改任何源码/样式/契约；未执行真实写操作；截图全部来自测试服真实页面并标注非物理真机。
- 结论：R453 骨架屏与品牌渐变态在真实浏览器下均正常呈现，无可达 AI 风格缺陷，无需追加修复；本轮为 ≥1000 轮迭代中的一环，非终态。
