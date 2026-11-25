# R537 H5 表单控件可访问名透传到焦点节点修复（WCAG 2.1 SC 4.1.2 / 3.3.2）

- 轮次：R537（repair round 2，t37），修复 t36 终审 needs_revision 的 F1-CORE-NAME-WRONG-NODE（blocker）+ F2-VERIFY-PROBE-LAYER（high）。
- 真父：68db97466895e422e088694eb39a56e54ac03bf0（t35 HEAD，已 push）。
- 决定性缺陷：t35 把 aria-label 加在源码控件上，uni-app H5 编译为 <uni-input aria-label=X><div class=uni-input-wrapper><input class=uni-input-input></div></uni-input>；aria-label 只落宿主 UNI-INPUT（AX role=generic, focusable=false, name=X），AT 真正聚焦播报的内层 input（AX role=textbox, focusable=true, focused=true）name 仍为 null。t35 报告以宿主层 name 非空为通过依据，属验证口径错配的假阳性。

## 一、写域（实际改动 3 文件）

| 文件 | 变更 |
| --- | --- |
| oa4rust/mobile/src/main.ts | M：新增 installH5NativeInputNameA11y() 全局 H5 无障碍增强器（MutationObserver 把宿主 aria-label 透传到内层可聚焦原生控件），随 createApp 在 window 守卫内安装，与 desktop main.ts 全局增强器同模式 |
| oa4rust/mobile/src/pages/R537.h5NativeInputName.contract.test.ts | A：8 用例源码级契约（3 核心增强器存在/透传逻辑/安装点 + 5 守卫业务零改） |
| oa4rust/docs/ops/oa4rust-r537-h5-native-input-name-fix.md | A：本文档 |

calendar.vue 与 bbs/new.vue 本轮零改动：t35 已加的宿主层 aria-label 保留作透传 label 来源（契约守卫钉死 9 处）。业务零改：v-model / @tap / @change / placeholder / maxlength / handler 函数 / 模板结构 / 样式 / :295 正面对照按钮 / 他页逐字未改；main.ts 原有 pinia/SSR/api/errorHandler/unhandledrejection 逐字保留。

## 二、修复方案

增强器三步：①初次扫描 document.querySelectorAll 对现有宿主与原生控件补 aria-label；②MutationObserver(childList+subtree+attributes[aria-label]) 监听 v-if 异步插入与 label 变化；③labelHost（宿主→内层 querySelectorAll(input,textarea)）与 labelFromNative（内层→closest 宿主）双向透传。仅写 aria-label 属性，不触碰任何业务属性或事件。一处覆盖所有 H5 表单。

写域内可修复的决定性探针（oa4rust/test-results/t37-focus-probe.cjs）：真父态 focused textbox name=null，脚本给内层 input.setAttribute("aria-label","日历名称") 后 focused textbox name 变 "日历名称"，证明框架硬编码仅挡模板层透传，H5 运行时脚本可写内层。

## 三、RED → GREEN

- 契约定点：真父 main.ts（sha 508492d1…）覆盖 → RED_EXIT=1，3 failed（3 核心：增强器定义/透传逻辑/安装点全缺）| 5 passed（守卫全过）；恢复修复版 main.ts（sha b2b627d7…）→ GREEN_EXIT=0，8 passed。日志 r537-red.log / r537-green.log。
- E2E 决定性 RED：真父态自起 H5（<TEST_HOST>）+ Playwright 390x844 DPR2，document.activeElement=INPUT.uni-input-input aria=null，getFullAXTree focused=true textbox name=null → R537_RED_VERDICT=DEFECT_REPRODUCED；恢复修复态后 9 个焦点节点 name 全部非空。

## 四、三 verify（全 exit 0，workdir=oa4rust/mobile）

| # | 命令 | 结果 | 日志 |
| --- | --- | --- | --- |
| V1 | pnpm test src/pages/R537.h5NativeInputName.contract.test.ts | exit 0，8 passed | r537-verify1.log |
| V2 | pnpm test | exit 0，79 files / 522 tests | r537-verify2.log |
| V3 | pnpm typecheck | exit 0，tsc --noEmit | r537-verify3.log |

## 五、决定性 CDP 取证（t36 F2 核心判据：focused=true 节点 name 非空）

- 报告：oa4rust/test-results/r537/r537-ax-report.json，R537_E2E_DONE records=9 / namesBad=[] / fatals=[] / pageErrors=[]，probeLayer=getFullAXTree focused=true non-root node（AT 实际播报节点），宿主层仅辅助。
- 9 个焦点节点 AX name 均非空且等于预期：日历名称 / 事件标题 / 开始时间 / 结束时间 / 地点 / 日历选择 / 版块 / 标题 / 内容。其中 7 个原生 input/textarea（document.activeElement=INPUT/TEXTAREA.uni-input-input，role=textbox, focused=true, editable=plaintext，aria-label 由增强器透传）；2 个 picker（见 Fail Loud）。
- 正面对照：.calendar-create-btn role=button name=新建日历 ignored=false，零回归。
- 业务零改（全 true）：calendarFormOpened / composeOpened / composeInputCount=4 / placeholderIntact / submitBtnIntact / bbsPageReached / bbsPlaceholderIntact / bbsSubmitIntact / bbsFieldCount=3；0 pageerror。
- H5 无主题切换机制，据实单组取证不伪造暗色差异（与 R534/R535 口径一致）。

### Fail Loud：picker 焦点节点口径说明

uni-picker 无内层原生控件，宿主自身即焦点节点（document.activeElement=UNI-PICKER，AX role=generic, focusable=true, focused=true, name=日历选择/版块 非空）。这与 t36 F1 批评的"输入框宿主 name 非空但内层 textbox name=null"口径错配不同：picker 的宿主就是被聚焦、AT 播报的节点，其 aria-label 直接构成焦点节点 name，无需透传。取证时为让 uni-picker 进入焦点序列，脚本临时注入 tabindex=0 并 focus（仅测试页临时 DOM，不落库、不改源码、不影响业务）。增强器保留对 uni-picker 宿主的处理以覆盖未来可能的嵌套原生控件。

## 六、证据 sha256

| 产物 | sha256 |
| --- | --- |
| main.ts（修复后） | b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b |
| R537 契约 | b1a1e915008ac507740e95d18acef56c52729d8b6bfda6d6e4ec8bbe5b77b3e2 |
| r537-e2e.cjs | 84d1c93034c8d6d676195c36b4ef3fce8c393e92ee3cbd62d88e1f9aa04f1591 |
| r537-ax-report.json | c6ba10d66ed9ff9968257d5b2658aca6b2050dc148da92b5adccda7b2912834c |
| r537-red.log | 9520711a1987f620472de3db1530e125298dd7ef620d54066331a177b26efded |
| r537-green.log | 93eb12be00bd0b5e1dc06b4ec572c162bbf91edbf53ace9ff88d2e53d4655e55 |
| r537-verify1.log | 6a4a6e9c667781560173cb0d34ca9c19ccfa4ac80573a7b56ffd78d23598177d |
| r537-verify2.log | aabfd55e461929e5b6517989363fe70a5d8b65a6b27f3fdbf20fbe5801154606 |
| r537-verify3.log | 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92 |
| r537-calendar.png | 53aeb01ba3a15b146c828f3595172dca3deffa8c8f4ad06bee180a54dc32f947（780x1687，112015B） |
| r537-bbs.png | f03dee35dddd696f7bedbba3fb349f4988b4ef91431db8e76cb02d5a6f8c66f0（780x1687，112027B） |

2 张 780x1687 移动视口截图均目视正常：日历页表单（日历名称/日历选择/创建日历/事件标题/开始 2026-09-22 09:00/结束 2026-09-22 10:00/地点/保存事件）与发帖页（版块/标题/内容/发布主题）渲染完整。runner/日志/截图/报告位于 .gitignore 排除的 oa4rust/test-results/，不入库。

## 七、脱敏

全文无密钥/口令/真实域名/IP/主机名/SSH 端口/本机绝对路径/个人目录；服务地址以 <TEST_HOST> 占位，API stub 为占位数据，0 真值入库。
