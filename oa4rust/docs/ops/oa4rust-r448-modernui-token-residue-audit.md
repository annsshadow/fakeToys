# R448 ModernUI 错误令牌残留与令牌体系一致性只读甄别（t301）

- 来源标注：Playwright Chromium headless 1440x900 到测试服真实页面（NOT a physical device，非物理真机）。站点域名用 <YOUR_TEST_DOMAIN> 占位；口令仅在运行时经 ssh 从远程凭据文件提取注入环境变量，未入库、未回显。
- 范围：只读甄别，未改任何源码/样式/契约；本轮唯一入库产物为本甄别文档。真父 f5dd2067490ad2961d5516dfcbb6e262a8ba075f（分支 opt1000/oa4rust-pc-ai-refactor）。
- 结论提纲：A ModernUI 错误令牌残留为真实代码缺陷但页面不可达，判 L1（不可达低优先）；B 令牌解析决定性对照坐实未定义令牌致声明被丢弃；C CalendarApp 新建日历表单存在同类残留且用户可达，判可达真缺陷；D 系统性反查共 3 个未定义令牌引用，建议后续独立任务修复。

## 1. A — ModernUI 可达性与错误令牌残留甄别

- 真实代码残留 1 处：oa4rust/frontend/apps/desktop/src/components/admin/ModernUI.vue 第 10 行 background: var(--ai-surface-gradient)；根容器第 4 行 class 为 modern-page ai-page-b5 ai-b5-surface；第 9 行 scoped .ai-b5-surface，是 R300 B5 契约共享原语之一（AdminZone.aiStyle.B5.contract.test.ts 第 18 行）。
- 令牌定义反证：oa4rust/frontend/packages/ui/src/theme/ai-tokens.css 全文 139 行只定义 --ai-gradient-surface（暗色第 31 行、亮色第 74 行），全文无 --ai-surface-gradient 定义（词序颠倒）。R447 修复后全库 --ai-surface-gradient 共 10 命中：1 处真代码 + 6 处 R446/R447 文档说明 + 3 处 R447 契约断言，无其它代码残留。
- 可达性穷尽（全 frontend grep）：ModernUI / modern-ui / modernUI 共 12 命中——barrel components/admin/index.ts 第 13 行 export、若干契约测试、FileManager.vue 第 1486 行与 Personal.vue 第 759 行注释；无组件 import、无 resolveComponent、无 app.component 全局注册、无别名导入。.ai-b5-surface 仅 4 命中（ModernUI.vue 第 4/9 行 + 契约第 59/60 行）。真浏览器 6 路由实测 .ai-b5-surface count 全为 0（见第 5 节）。
- 判定：不可达 L1（C2）。ModernUI.vue 组件无真实渲染入口，错误令牌虽真实存在但当前不影响任何可见页面；不得臆造为可见视觉缺陷。FileManager.vue 第 1488 行与 Personal.vue 第 762 行各自 scoped 块使用正确 var(--ai-gradient-surface)，未复制该错误，属另一自洽路径。

## 2. B — computedStyle 令牌解析决定性对照

- 真浏览器页面根元素 getComputedStyle(document.documentElement).getPropertyValue(...) 原始值：
  - --ai-surface-gradient（WRONG）→ 空字符串（未定义）；
  - --ai-gradient-surface（REAL）→ linear-gradient(160deg, #4f46e50f 0%, #0891b205 100%)（实测有效）；
  - --ai-text-primary / --ai-text-secondary（WRONG）→ 空字符串 / 空字符串；
  - --color-text-primary / --color-text-secondary（候选别名）→ 空字符串 / 空字符串（该两别名在根元素未定义，另见第 4 节说明）。
- 机制：CSS 声明中未定义 var() 令该条 declaration 无效并被浏览器丢弃，元素回退到继承色/默认背景，不是值为空这么简单。
- 契约宽松根因：ModernUI.contract.test.ts 第 46-50 行仅检查样式块用 var(...)、第 55-68 行仅检查 .ai-b5-surface 存在加 scoped 不碰全局；AdminZone.aiStyle.B5.contract.test.ts 第 43-48 行用宽松正则匹配任意 --ai- 前缀令牌，均无法识别令牌名拼错。

## 3. C — CalendarApp 新建日历表单：可达真缺陷

- 可达性：oa4rust/frontend/apps/desktop/src/main.ts 第 82-84 行路由 /oa4rust/app/calendar（meta.title 日历）；真浏览器实测 title 为 日历 · OA4Rust、bodyLen=536，新建日历入口 count=1，点击后表单可见（cal-create-label=2、cal-create-color=1、cal-create-cancel=1）。
- 残留：oa4rust/frontend/apps/desktop/src/views/CalendarApp.vue 第 816 行 color: var(--ai-text-secondary)、第 818 行 color: var(--ai-text-primary)、第 819 行 color: var(--ai-text-secondary)（scoped .ai-page .cal-create- 规则）引用未定义令牌。
- 真浏览器决定性证据（CSSOM 探针 form-probe.json）：三个元素 computedStyle color 均为 rgb(15, 23, 42)（继承色）；遍历全部 document.styleSheets 的 cal-create-label/color/cancel 规则集合 rules 为空，元素 matchedRules 为空——声明被浏览器整条丢弃。截图 calendar-form-open.png 目视标签/取消按钮呈深色，与设计意图的 secondary 浅色不符。
- 判定：可达真缺陷（C1），建议后续独立任务按实际生效的文本令牌名修复；本轮只读不改码。注意：--color-text-primary/secondary 在根元素实测亦为空（可能定义于 body 等后代层级或主题作用域），修复时须以实际生效令牌为准并补 computedStyle 复验，不得凭名称猜测。

## 4. D — 系统性反查：未定义令牌引用差集

- 方法：tools.grep 全 frontend 提取 --ai- 定义集（29 个）与 var 引用集（29 个）求差集。
- 差集 3 项：
  1. ai-surface-gradient：ModernUI.vue 第 10 行（L1 不可达，见第 1 节）；
  2. ai-text-secondary：CalendarApp.vue 第 816、819 行（可达，见第 3 节）；
  3. ai-text-primary：CalendarApp.vue 第 818 行（可达，见第 3 节）。
- 除上述外无其它未定义 --ai- 引用；R446/R447 已修复 6 页 R300 B5 的同名错误后，残留面收敛到这 3 处。

## 5. 真浏览器实测证据（6 路由加新建表单、0 pageerror）

- 运行：test-results/r448-e2e.cjs 加凭据运行时注入 runner；证据目录 oa4rust/test-results/r448-modernui-token-audit/（7 png + report.json + form-probe.json，gitignored），errors 为空。
- 6 路由摘要：dashboard（title 工作台 · OA4Rust，aiB5Surface=0）、calendar（日历 · OA4Rust，0）、file（文件管理 · OA4Rust，modern-page=1、aiB5Surface=0）、personal（个人中心 · OA4Rust，modern-page=1、0）、admin-role（角色管理 · OA4Rust，modern-page=1、0）、admin-config（配置设计器 · OA4Rust，0）——无任一页面渲染 .ai-b5-surface，坐实 ModernUI 不可达。
- 截图 sha256（命令行 sha256sum 原始值）：
  - admin-config.png bef1b41df1a111486ec9a0da3f83498ddcfbad5948ba10f9ced3aeabdaa34384
  - admin-role.png 97033ee231fa8e7e00be4c49c40b1285506d7516fe7729513c5291327e5be5f5
  - calendar-form-open.png 08895065fc6b19f2033e35cb8be98968a3a05c438ab6732caed4f622c0d3d859
  - calendar.png 88e87f3762f036b3455ef4b03ddc97a8ee22044ff41e99a8d886f0038a20da0d
  - dashboard.png 5772e33570f33d2ebe8184ae74c2231d35527b4ddfd17f9cfcf65bc0f06aac87
  - file.png 4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf
  - personal.png fe2eb5c95c2cf44c779990133675083b6fe074bc5c502cddd0b3c87cb8f22eb1

## 6. 三分判定汇总与后续建议

- A / ModernUI.vue 第 10 行：不可达 L1。组件无 import/注册/渲染；6 路由 .ai-b5-surface=0；错误令牌真实但不可见。
- C / CalendarApp.vue 第 816/818/819 行：可达真缺陷。日历路由可达、新建表单可展开；computedStyle 回退继承色 rgb(15,23,42)、CSSOM 规则被丢弃。
- D / 其它 --ai- 引用：无新增缺陷。差集仅上述 3 项；R446/R447 同名修复后收敛。
- 不建议本轮扩大改动：本任务只读甄别；修复 A/C 与契约收紧（把宽松 --ai- 正则改为精确令牌名白名单校验）应另立后续任务，按真 bug test-first RED→GREEN 执行。
- 真值纪律：文档域名占位、无口令/私钥/会话数据；截图与临时脚本留在 gitignored 证据目录。