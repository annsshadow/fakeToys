# R451 H5 端 19 页 AI 组件类真浏览器呈现甄别

- 任务：t306（Attempt2），视觉总设计师只读甄别；真父 commit c60ad3cdba273c0708e9c28babea0740dcacbc89。
- 取证来源：Playwright Chromium headless，390x844、DPR2、isMobile/hasTouch，访问 <YOUR_TEST_DOMAIN> 测试服真实页面（NOT a physical device，非物理真机）。
- 证据目录（gitignored）：oa4rust/test-results/r451-h5-ai-visual/（19 png + report.json + deep.json）；E2E 脚本与凭据运行时提取器亦在 gitignored oa4rust/test-results/，真域名/凭据不入库。
- 本轮只读甄别，未改任何源码、未执行页面真写（发帖/审批/上传等）；判定均据真浏览器 computedStyle + CDP CSSOM matched rules + 截图目视，不采信静态预判为最终结论。

## 1. 实测口径与覆盖面

- 19 页逐页采集根容器 .ai-surface-gradient 的 computedStyle（backgroundImage/backgroundColor）、.ai-brand-fill 元素 computedStyle、.ai-panel/.ai-gradient-text/.ai-pressable 命中与状态；6 页做 CDP CSS.getMatchedStylesForNode 深挖（deep.json）。
- 全 19 页 surfaceCount=1（每页恰一个根容器 AI 渐变元素）；errors=[]；登录态 loggedIn=true。
- 令牌在元素层可解析（生效页 backgroundImage 为真实 linear-gradient），documentElement 层读取 tokens 为空串属探针取点差异，不作为失效证据。

## 2. A 项：根容器 .ai-surface-gradient 渐变生效性

令牌定义真值：oa4rust/mobile/src/App.vue:318-320 = .ai-surface-gradient { background: var(--ai-gradient-surface); }。

生效 6 页（computedStyle backgroundImage=linear-gradient(160deg, rgba(79, 70, 229, 0.06) 0%, rgba(8, 145, 178, 0.02) 100%)，backgroundColor=rgba(0, 0, 0, 0)）：

- login（pages/login/login）、index（工作台）、bbs（论坛）、bbs-topic（主题详情）、bbs-new（发布主题）、collect（我的收藏）。

失效 13 页（backgroundImage=none，backgroundColor=rgb(245, 247, 250)，渐变被页面 scoped 背景覆盖、退化为纯色）：

- message（消息）、chat（即时通讯）、process（审批）、process-start（发起流程）、attendance（考勤打卡）、doc（文档）、contacts（通讯录）、meeting（会议）、calendar（日历）、browse（浏览）、search（搜索）、recycle（回收站）、mine（我的）。

CDP 决定性对照（deep.json）：

- index 生效页：全局 .ai-surface-gradient { background:var(--ai-gradient-surface) } 在前，页 scoped .page[data-v-d7a65309] { padding:1rem } 在后但不设 background，不冲突，渐变生效。
- message/process/mine 失效页：全局 .ai-surface-gradient 在前，scoped .page[data-v-f5cd44d6] / .page[data-v-47e2a8e8] / .page[data-v-61ba5fea] { min-height:100vh;background:var(--color-bg-page) } 在后；scoped 选择器 (0,2,0) 高于全局 (0,1,0)，覆盖渐变，computedStyle 实测 none。
- attendance 根容器同链中可见 bgImage=none、bgColor=rgb(245,247,250)，属同一 13 页失效集合。

A 判定：可达真缺陷。13/19 页用户可见的背景渐变失效（退化为纯色），根因是全局类特异性未提权，与 App.vue:299 注释预警的机制一致；App.vue:300-303 已为 .ai-brand-fill 做重复类提权，而 .ai-surface-gradient 仅单点定义、未做同类提权。

## 3. B 项：.ai-brand-fill 品牌渐变生效性

- 有该元素的页面，绝大多数 computedStyle backgroundImage=linear-gradient(135deg, rgb(79, 70, 229) 0%, rgb(8, 145, 178) 100%)，提权规则 uni-page-body .ai-brand-fill.ai-brand-fill, body .ai-brand-fill.ai-brand-fill（App.vue:300-303）生效。
- brandCount=0 页面：message / doc / bbs-topic —— 页面本身无该元素，非缺陷。
- 灰显元素：attendance/meeting/calendar/browse 存在 disabled 态 brand 按钮，computedStyle backgroundImage=none、backgroundColor=rgb(192, 196, 204)（占位灰）。CDP（attendance）：.btn[data-v-e7d83744] { background:var(--color-primary) } 与 .btn[disabled][data-v-e7d83744] { background:var(--color-placeholder) } 特异性 (0,3,0) 压过提权 (0,2,1)，且元素 disabled=true；同页非 disabled 的 .btn.leave-btn.ai-brand-fill 渐变正常。

B 判定：非缺陷。品牌渐变提权总体有效；disabled 灰显属业务语义禁用态的正确呈现，不应按 AI 类失效计缺陷。

## 4. C 项：其余 AI 类

- .ai-panel（App.vue:304-311）：19 页 panelCount 全为 0 —— App.vue 有定义但页面模板未消费，不计缺陷。
- .ai-gradient-text（App.vue:312-317）：19 页 gradientTextCount 全为 0 —— 同上，未消费不计缺陷。
- .ai-pressable（App.vue:343-349）：部分页命中（index=15、meeting=2、doc=1、collect=2、browse=2、recycle=2 等）；实测 transition=transform 0.14s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.14s cubic-bezier(0.16, 1, 0.3, 1)，按压动效生效，未被覆盖。
- 未见深浅色对比度异常或文字不可读的据实问题；空态（如 collect/index）文案与动作按钮正常呈现。

## 5. D 项：三分判定汇总

- ①可达真缺陷：.ai-surface-gradient 在 13 页被页面 scoped .page[data-v] 背景覆盖，用户可见渐变失效（第 2 节证据）。
- ②注释风险部分成立：.ai-brand-fill 提权有效（第 3 节），.ai-surface-gradient 未做同等提权导致 ①；attendance 等 disabled 灰显属业务语义，非缺陷。
- ③其它样式问题：无据实可报的对比度/可读性缺陷；ai-panel、ai-gradient-text 为零消费而非失效。
- 修复建议（后续独立任务，本轮不改码）：仿 .ai-brand-fill 为 .ai-surface-gradient 增加容器前缀+重复类提权，使全局渐变压过页面 scoped .page[data-v] 背景；改后须回归本 19 页 A 项 computedStyle。

## 6. 截图目视核实（read_image，390x844 DPR2 视口）

- index.png：工作台渐变顶部与两枚品牌渐变统计卡清晰可见；.ai-surface-gradient 生效页的典型呈现。
- message.png：会话列表背景为纯色，顶部无柔和渐变，肉眼印证 A 项失效（scoped 覆盖）。
- attendance.png：打卡按钮 disabled 灰显、提交假申请按钮品牌渐变正常，印证 B 项 disabled 语义非缺陷。
- collect.png：收藏空态大渐变头图与“去浏览内容/刷新”动作正常，.ai-surface-gradient 生效页的典型呈现。

## 7. 19 截图 sha256（sha256sum 命令行原始值）

| 页面 | sha256 |
| --- | --- |
| attendance.png | `4321051379c92e0d27bb27d4bdf89017cf09a6b53727d70f936a341748d92fb6` |
| bbs-new.png | `f03dee35dddd696f7bedbba3fb349f4988b4ef91431db8e76cb02d5a6f8c66f0` |
| bbs-topic.png | `5f1daef87720e4bfc87dba2cf044e2c03cb4271196fa60985dd91d8c7e1366e9` |
| bbs.png | `0851b49ab35568b09871f6dda497b8853cac1396a9e802bf7872a2cad4c8f1c9` |
| browse.png | `fce1933d56abb16412f47687848ad0de4532db647b13be8ebe9f31e48834a398` |
| calendar.png | `af1346441718686bc102971e5040a7db9bb1069a4dab5b63b5d33dc5b09e7af1` |
| chat.png | `0def5210baccaf9446e4f63a070ec0d5ac5ab4faacac0a539d85a4fb66909c4d` |
| collect.png | `cbae5e3d08c81c9b2276a9f3c103d6b0094494f6dbda229b8a1f66a2276f0ed6` |
| contacts.png | `e81c4b1c721ac0eddfd3fb4a9901c28d9e05a78d2aac6422e8cc3bdd18fd3a99` |
| doc.png | `8a0ca450f06bb848ff271d67564e4f174d9c143d25ccc5de4ff088dd908b4603` |
| index.png | `c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96` |
| login.png | `a78c95b886e43687e880a7d6215b4e9029ae12e4cfef7318d624711dfd06abd8` |
| meeting.png | `5a4fd7c384b84e89584b40bde2d87189525760240a13e86682769ca6a7e6ef1d` |
| message.png | `f351d74b585e8934f8cd373fdd935f22793145dd5dd23777f42cf50db02c61fd` |
| mine.png | `59c001c77151c95efb2288d7bc98dab7dfa23cefe5c05e6a9148824d6359023b` |
| process-start.png | `2958fcfc809c62c523fc2b3c9bf51f2e54c69d447498f9a4be8ecb0f9208a90a` |
| process.png | `40e609dd02db06b62c8362b2a96ef07b72bd8d1845a9b5ca0ffac2fa02fc5b1f` |
| recycle.png | `363943d0233f1afd7ea2511c6e1d60e944285446c31634500abd0ccdf1859429` |
| search.png | `fc2ef0655408c7c4b7171dd95f0d27e28594ffee5b4c1222c6b43605361fef8c` |

## 8. 真值纪律

- 文档用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位；口令、私钥、真实域名/IP/凭据、测试会话数据不入库；截图与临时脚本留在 gitignored 证据目录。
- ≥1000 轮持续迭代中的一环，非终态；甄别结论供后续独立修复任务使用。
