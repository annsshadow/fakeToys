# R484 WCAG 2.4.1 Bypass Blocks（绕过块）甄别（只读）

## 1. 概况

- 轮次：R484（台账 §479 据实立项），任务 t359，design-lead 甄别，只读不改码。
- 真父提交：efdbbb9915fd5cfbfe40aa78b14746d5bf1614fc；甄别前基线 HEAD=UP 同值，AB=0 0，工作树干净，目标文档为新建。
- 分派前提：队长消息称全库无 skip-link、需据实甄别是否可以合理绕过重复导航块。该前提经本轮源码、部署产物、真浏览器三重证据反证，实际已实现 skip-link，详见 §2 与 §4。
- 真机声明：Playwright Chromium headless，PC 1440×900 DPR1，light/dark 双主题，非物理真机；真域名仅存在于 gitignored runner，本文档使用占位符 <TEST_HOST> / <YOUR_TEST_DOMAIN>。
- 证据目录：oa4rust/test-results/r484-bypass-blocks/（26 PNG + report.json + skip-report.json + focus-report.json + 分析/探针脚本，全部 gitignored）。

## 2. 静态侦察：分派前提被源码与部署反证

- oa4rust/frontend/apps/desktop/src/utils/skipLink.ts:4：TARGET_ID=main-content；:5：LINK_CLASS=skip-link；:17：export function installSkipLink()。
- 同文件 :20-23：createElement(a)、href=#main-content、textContent=跳转到主要内容；:25-29：插入到挂载点父节点之前，确保成为 body 第一个元素；:32-37：prepareTarget 给 main 设 tabindex=-1；:39-46：onClick 调 target.focus()；:48-50：MutationObserver 重挂。
- oa4rust/frontend/apps/desktop/src/main.ts:19 导入 installSkipLink，:611 注释 R187，:612 调用（位于 installTableA11y 之前）。
- 契约 oa4rust/frontend/apps/desktop/src/utils/skipLink.contract.test.ts:39 起 describe、:53-90 三 it 钉住首焦点、焦点入 main、异步挂载。
- oa4rust/frontend/packages/ui/src/theme/dark.css:291 起、:297 .skip-link：position:fixed;top:-1000px;z-index:1000，:focus{top:12px}；main.content:focus:not(:focus-visible){outline:none}——离屏隐藏、聚焦可见。
- 受控部署环境核实：当前部署构建产物含字符串“跳转到主要内容”，说明 skip-link 已进入真实构建，而非仅存在于源码。
- 第二条绕过：oa4rust/frontend/apps/desktop/src/utils/routeAnnouncer.ts:22 MAIN_ID=main-content，:52-72 tryFocus 设 tabindex=-1 并 main.focus()（R231 SPA 路由导航后移焦）。
- 第三条结构与入口：AppShell.vue:15 nav aria-label=主导航（约 15 个可聚焦项），:72 面包屑 nav，:145 页签 nav，:156 main#main-content，:166 移动端 tab-bar；:88-105 搜索框 aria-label=全局搜索 + data-testid=command-palette-trigger，:184-209 命令面板 role=dialog aria-modal=true aria-label=命令面板。
- 静态结论只作候选；本轮全部判定以真浏览器焦点序列、CDP AXTree 与 DOM 探针为准，不凭源码预判。

## 3. 方法

- 主脚本 r484-e2e.cjs：4 页（dashboard/process-work/attendance/file）× 双主题 = 8 组；登录并注入主题后采 counts（navFocusables/skipExists/skipHref/skipText/skipTabIndex/skipPosition/skipTop/mainExists/mainTabIndex/landmarks）、CDP DOM.describeNode + Accessibility.getPartialAXTree（.skip-link 与 #main-content）、28 次 Tab 焦点序列、skip-link 激活探针、⌘K 探针。
- 确定性探针 r484-skip.cjs：直接 page.locator(.skip-link).focus()，采 before/onFocus/activated/next，避免主脚本 blur 后起点受 routeAnnouncer 干扰。
- 清洁焦点探针 r484-focus.cjs：reload 后不 blur，从 document 起始连续 Tab，遇 main 即停；另写 r484-diag.cjs 记录 reload 后 activeElement 及随后 12 次 Tab 的 landmark。
- 命令面板探针 r484-palette.cjs：分别测 body blur 后 Control+k 与点击 data-testid=command-palette-trigger 两条路径，并测 Escape 关闭。
- runner r484-run.sh 等从已验证范式复制仅改脚本名；凭据在 ssh 运行时从受控凭据源提取后注入进程环境，真域名只存在于 gitignored runner，不回显、不入库。

## 4. 实测读数

### 4.1 主 E2E：8 组一致读数

- R484_DONE groups=8 shots=16 errors=0，0 pageerror。
- 八组全部：navFocusables=15；skipExists=true；skipHref=#main-content；skipText=跳转到主要内容；skipTabIndex=0；skipPosition=fixed；skipTop=-1000px；mainExists=true；mainTabIndex=-1；mainId=main-content。
- landmarks 含 nav[主导航]、header、nav[面包屑]、main#main-content、header，结构完整。
- CDP：skipAx={found:true,axRole:link,axName:跳转到主要内容,axIgnored:false,attrs:[class,skip-link,href,#main-content]}；mainAx={axRole:main,axName:空,axIgnored:false,attrs 含 tabindex,-1}。
- 主脚本 blur 后 Tab 起点不一致（dashboard 首个焦点 quick-card 已在 main），部分组 afterActivate.isMain=false 属探针起点模糊，故以 §4.2 确定性探针为准。

### 4.2 skip-link 确定性探针：8 组全部一致

- R484_SKIP_DONE groups=8 errors=0。
- before={exists:true,isFirstBodyChild:true,tag:a,href:#main-content,text:跳转到主要内容,position:fixed,top:-1000px,tabIndex:0,isFirstFocusable:true}。
- onFocus={focused:true,focusVisible:true,top:12px,position:fixed,rect:{x:12,y:12,w:132,h:43}}。
- activated={activeTag:main,activeId:main-content,isMain:true,mainTabIndex:-1,hash:#main-content}；下一 Tab 落主内容或 header（attendance/process 的 input 在 header）。
- 决定性：skip-link 为首个可聚焦元素、聚焦时可见、激活后焦点进入主内容，完全满足 2.4.1 的绕过手段要求。

### 4.3 清洁焦点与路由后移焦

- R484_FOCUS_DONE groups=2 errors=0；light/dark 各 seq 仅 1 条：tag=a、text=组织架构、cls=quick-card、landmark=main、rect x281 y492 w131 h76、isSkip=false——reload 后 activeElement 已在 main。
- 诊断探针：reload 后 before={skipExists:true,skipIsFirst:true,activeTag:main,activeId:main-content,activeCls:content,hash:空}，证实 routeAnnouncer 已把焦点移到 main；随后 12 次 Tab 全在主内容区（quick-card 组织架构/工作审批/即时通讯/日历/会议/文件/论坛/AI助手 + 查看全部 + 去处理×3），isSkip 全 false，0 侧栏导航链接。
- 决定性：路由切换后键盘用户无需 Tab 过侧栏导航块，绕过需求在实际交互流中已被前置满足。

### 4.4 命令面板与据实披露

- 路径 A（body blur 后 Control+k）：open=false，activeTag=input、activeCls=search-input——该测试环境下快捷键未打开面板。
- 路径 B（点击 data-testid=command-palette-trigger）：open=true，role=dialog，label=命令面板，activeTag=input、activeCls=command-palette-input；Escape 后 closedAfterEsc=true。
- 据实披露：Ctrl+K 快捷键在 Playwright 键盘注入下未触发，但可见触发按钮点击路径可用；主绕过结论不依赖此快捷键，由 skip-link 与 routeAnnouncer 已充分支撑。

## 5. 三分判定

- 可达真缺陷：0。8/8 组 skip-link 有效、路由后焦点已在 main、landmark 完整、命令面板点击可达；分派前提“无 skip-link”被源码、部署、真浏览器三重反证，判不出 2.4.1 退化。
- 不可达 L1：0。目标元素全部可见可聚焦，无 aria-hidden/display:none 场景，无 R306C 硬排除页介入。
- 正常规范预期：若干（skip-link 实现与契约、routeAnnouncer 路由后移焦、nav/header/main landmark、命令面板点击入口）。
- 技术观察（非缺陷）：Ctrl+K 在本测试环境未由 page.keyboard.press 触发；属探针环境差异，不作为产品缺陷，也不影响绕过充分性。
- 据实声明：不臆造缺陷、不凑轮次；本轮只读，未改任何源码或契约。

## 6. 截图目视

- pc-dashboard-light-skipfocus-r484.png（1440×900）：左上角显著蓝色焦点环“跳转到主要内容”覆盖侧栏顶部，工作台内容完整、统计卡与待办列表清晰。
- pc-file-dark-skipfocus-r484.png（1440×900）：深色主题左上青色焦点环跳转链接，文件管理页头图、列表与右栏完整。
- pc-dashboard-light-r484.png（1440×900）：常规态跳转链接离屏不可见，工作台正常显示，证明隐藏策略生效。
- 三图均名实相符、非物理真机；来源为 Playwright Chromium headless 真实页面流程。

## 7. sha256 原始值（命令行 sha256sum 核验，与报告一致）

主 E2E 16 图：
- pc-dashboard-light-r484.png：4148c08faf9990daac223f86818bd17004c22c1f6d7eff00adcfd3a9114b2ad6
- pc-dashboard-light-skip-r484.png：29754632392e2bbb485a6600886d6c5436d1d98bb91d02e95a7b6270ca7bc08b
- pc-process-work-light-r484.png：e9fb63803e004acee4d79fbb5a6a41f14a55d20357a73b51d2459de31bc18c4e
- pc-process-work-light-skip-r484.png：f4d12ed337280d784f5da26a1fb7bd793265fcbb6e5e768ad5e7b8f80a8cacb7
- pc-attendance-light-r484.png：22feacdaa7d55c5882b312aea46b2b1545ca0b94c4dbf14a336de96d917e358f
- pc-attendance-light-skip-r484.png：3f5300ea2f9e35df04dad4f6cb5f09bc91091fa4f3cf3532856c9265ed2add68
- pc-file-light-r484.png：de92ff8f8e347de3ddba58e4a3d6f44e2715d06049813b818e751e7a08414e03
- pc-file-light-skip-r484.png：a8561185a2353eba0b8dcf34663c3ec077eae38c362c40d5469058067de42011
- pc-dashboard-dark-r484.png：3b41c11b7aab5b02f02d02db8dfbe3150d4e7cfeffb74cb9db5ab500bf1b1a05
- pc-dashboard-dark-skip-r484.png：94320006d8f001aa8e3503ba6762ffb31852b9c880b27d792d42ca5641ea5dbf
- pc-process-work-dark-r484.png：2cfe6cc443cf84958baf4597cac0b8313945ace5a91f7f15aaf07825ebae950a
- pc-process-work-dark-skip-r484.png：d55fe1fd9a1e56445129cd2bc246c1fa235cd2163798db1bc7933304cda0ad7d
- pc-attendance-dark-r484.png：cb344e9723086d3e0214129785dee92fb846a30b3dce96bffab6853f2cce8861
- pc-attendance-dark-skip-r484.png：dba679efc616100a2a1a8607cce84dd4d878f86fe15540783d13f93e1fee0a19
- pc-file-dark-r484.png：c0810143abeee5342049eab9a8a02c79a732335156ef4e5c1a6f0ad02cfd2d32
- pc-file-dark-skip-r484.png：1785301e6a585b068d13e51ef9f57cb3243227eb2369338d102924c1c7fe6835

skipfocus 8 图与 cleanfocus 2 图：
- pc-dashboard-light-skipfocus-r484.png：8f63831f581bc333021af158971f7ca81fb59d2ef3f8c1ef64bb5b7a98729b7b
- pc-process-light-skipfocus-r484.png：9fae7eb58c46dd45d5913f3c53036d0d36584190f44c94b52cc69c223d6212ad
- pc-attendance-light-skipfocus-r484.png：737f5b434e6c318490696b91e44b5f5de46b268bbf2aab10e5760b0bb3608b83
- pc-file-light-skipfocus-r484.png：cfea4caeca60dea25ba1b386a27c87b63ba3e2a2407896c1dfcf2b10f923d97e
- pc-dashboard-dark-skipfocus-r484.png：af09885f7618c9698bdf40022583a690d47d06a5e56f2aaf1a4abcda6f516286
- pc-process-dark-skipfocus-r484.png：45095748a5ed2847a54ab2a72aa76e6cbb1a13cca941e52e2ea524589de6f342
- pc-attendance-dark-skipfocus-r484.png：6d1f7acfe0153d7730784c3676a0aa206b906d128e2856781af283b2a19681d0
- pc-file-dark-skipfocus-r484.png：5b6ed9a1e95cb3da5c9c95e01b34fa1249d77732d237b0dd650ff97eb2900d04
- pc-dashboard-light-cleanfocus-r484.png：3e4c5902085631c301bdfd18ccd3c26c4c7a5c44f38bb99b803f5215fe314452
- pc-dashboard-dark-cleanfocus-r484.png：541861d5fad4e865ae4038f383f973c058863d2faaf9f95eb9cf9b1ea6e5e85f

焦点序列/AXTree 报告 3 份：
- report.json：28ec58e712e0d22d821810d6e18706e643f11155a6a389b1d3d10684bb2e1549
- skip-report.json：af78228bfb40b2ec5fa0b940cc53fe3983f7473185f028a2a25de1db10177214
- focus-report.json：3e5a2d09c2f0aaab75b9ce4dd5d2fcc0d16c76ca5077122500422b1edd9c948c

## 8. 真值纪律与写域

- 文档仅使用 <TEST_HOST> / <YOUR_TEST_DOMAIN> 占位，不含真实域名、IP、主机名、SSH 端口、口令、令牌、私钥或个人路径。
- 证据与凭据隔离：所有 runner、脚本、截图、JSON 位于 gitignored oa4rust/test-results/r484-bypass-blocks/，凭据运行时注入进程环境。
- 本轮唯一写域为本文；未改源码、未改契约、未改台账；提交时精确 git add 单文档，禁 -A 与点号。
- 截图来源如实标注为 Playwright Chromium headless 非物理真机，不冒充真机人工听感。
