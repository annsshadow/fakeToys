# R449 PC CalendarApp 新建日历表单文本令牌修复（t302）

- 来源标注：Playwright Chromium headless 1440x900 到测试服真实页面（NOT a physical device，非物理真机）。站点域名用 <YOUR_TEST_DOMAIN> 占位、测试主机用 <TEST_HOST> 占位；口令仅在运行时经 ssh 从远程凭据文件提取注入环境变量，未入库、未回显。
- 真父：f7171990d96a6ae6beaaead02d9037ad91389a13（当前 HEAD，工作树干净，AB=0/0）。
- 范围：R448 甄别（t301）坐实的用户可达真缺陷；只改 3 行 scoped 颜色令牌 + 新增契约 + 本文档，业务/接口/data-testid/端点/后端/SDK/main.ts/adminNav.ts/其它 view/构建配置零改。

## 1. 缺陷与根因

CalendarApp.vue 新建日历表单（路由 /oa4rust/app/calendar，点「+新建日历」展开）scoped 块 3 处引用未定义令牌 --ai-text-secondary / --ai-text-primary，全 frontend 无任何 ai-text-* 定义。浏览器丢弃整条 color 声明，元素回退继承色；甄别期 form-probe.json 实测 label/color/cancel 三元素 color=rgb(15,23,42)（继承色）、CSSOM matchedRules 为空。

## 2. 精确修复（各 1 行）

- frontend/apps/desktop/src/views/CalendarApp.vue:816  .ai-page .cal-create-label  color: var(--ai-text-secondary) → var(--text-secondary)
- frontend/apps/desktop/src/views/CalendarApp.vue:818  .ai-page .cal-create-input/.cal-create-color  color: var(--ai-text-primary) → var(--text-primary)
- frontend/apps/desktop/src/views/CalendarApp.vue:819  .ai-page .cal-create-cancel  color: var(--ai-text-secondary) → var(--text-secondary)

正确令牌依据：同页 :734/738/761/777/783/786/790 已大量使用 --text-primary/--text-secondary；定义于 packages/ui/src/theme/dark.css:51(--text-primary)/:52(--text-secondary) 与浅色区 :114/:115。未臆造其它令牌名。

## 3. 契约 RED → GREEN

新增 CalendarApp.textToken.contract.test.ts（@vitest-environment node）：readFileSync 真读 CalendarApp.vue，正则抽取全部 <style> 块，真断言 .cal-create-label/.cal-create-color/.cal-create-cancel 的 color 用 --text-secondary/--text-primary/--text-primary 且不含 ai-text-*（非桩）。真父 f7171990d 版 RED = 7 failed / 0 passed（7 条全为真实令牌断言，0 桩错误）；修复版 GREEN = 7/7。

## 4. 三 verify（全 exit 0）

1. cd oa4rust && pnpm exec vitest run frontend/apps/desktop/src/views/CalendarApp.textToken.contract.test.ts → 1 file / 7 tests passed。
2. cd oa4rust && pnpm test → 315 files / 3111 tests passed，0 failed。
3. cd oa4rust && pnpm exec tsc -p frontend/apps/desktop/tsconfig.json --noEmit → exit 0。

## 5. 构建与部署

pnpm run build（workdir oa4rust）成功，入口 index-Bk7Zfwmt.js；test-results/deploy-pc-r449.sh 整目录 tar → scp → 本地/远端 sha256 比对一致 → 远端 /usr/local/oa4rust 建 web.incoming-r449 → grep 入口 → mv 原子替换，DEPLOY_R449_PC_FULL_OK。

## 6. 真浏览器决定性对照（三证据合一）

运行 test-results/r449-pc-calendar-run.sh（1440x900 headless isMobile=false，非物理真机）：登录测试服 → 日历路由 → 点「+新建日历」展开表单 → 采令牌解析 + CDP CSS.getMatchedStylesForNode 规则 + computedStyle 颜色。终轮 R449_DONE：17 断言全 true / failed=[] / 0 pageerror。

- 证据① 令牌解析：根元素 --text-primary=#0f172a、--text-secondary=#475569（均非空）；--ai-text-primary/--ai-text-secondary 均为空串（未定义）。
- 证据② CSSOM/CDP 规则：cal-create-label → .ai-page .cal-create-label[data-v-c3018281] color=var(--text-secondary)；cal-create-color → .ai-page .cal-create-input/.cal-create-color[data-v-c3018281] color=var(--text-primary)；cal-create-cancel → .ai-page .cal-create-cancel[data-v-c3018281] color=var(--text-secondary)（origin=regular）。父版 matchedRules/rules 均为 []。
- 证据③ 颜色：label/cancel 由继承色 rgb(15,23,42) → rgb(71,85,105)（secondary 生效）；color 保持 rgb(15,23,42)。★浅色 --text-primary=#0f172a=rgb(15,23,42) 与继承色巧合相同，:818 primary 修复前后颜色数值不变属正常，靠证据①②坐实。

说明：document.styleSheets 遍历在该 SPA 下取不到 scoped 规则（4 张表均无 cal-create-* 选择器）；改用 CDP CSS.getMatchedStylesForNode 直接读已匹配规则，父版为空、修复版三条规则均含真实令牌，构成决定性对照。

## 7. 截图与证据

- gitignored 证据目录：oa4rust/test-results/r449-pc-calendar-token-fix/（report.json + calendar-form-open.png）。
- calendar-form-open.png sha256（命令行 sha256sum 原始值）：7a147f1e31901355f374201cc271f64ed862e56c655d23217ff54a75cb7a384e（1440x900，真 OA4RUST admin 日历页，新建表单已展开）；已 read_image 目视确认 label/取消按钮为浅色 secondary 文本、color 输入框深色。

## 8. 写域与零改声明

- 写域精确 3 文件：CalendarApp.vue、CalendarApp.textToken.contract.test.ts、本文档；单 commit。
- 未夹带 ModernUI.vue:10 不可达 L1 与宽松契约收紧（均另案）；未动第 4 个文件。文档 0 真值（域名/主机占位），无口令/私钥/真实服务器地址入库。
