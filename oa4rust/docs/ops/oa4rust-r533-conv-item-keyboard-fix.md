# R533 IMChat 会话项键盘可达修复（WCAG 2.1 SC 2.1.1）

- 轮次：R533（implementation，t31）
- 真父：521600fa6d1b0a325df7f306efad38a984332fd0（台账 §524；R532 判定 P2 真缺陷后由甄别轮改为修复轮，原定表头甄别顺延）
- 缺陷：IMChat.vue 会话列表 .conv-item（role=listitem）仅有 @click，无 tabindex/键盘处理器，键盘用户无法切换到会话——违反 SC 2.1.1（Keyboard）。
- 修复：仅给 .conv-item 开标签追加 tabindex=0、@keydown.enter=selectConversation(conv)、@keydown.space.prevent=selectConversation(conv)，复用既有 selectConversation。业务零改。

## 一、写域（精确 3 文件）

| 文件 | 变更 |
| --- | --- |
| oa4rust/frontend/apps/desktop/src/views/IMChat.vue | M：:18 .conv-item 开标签加键盘属性一行 |
| oa4rust/frontend/apps/desktop/src/views/R533.convItemKeyboardA11y.contract.test.ts | A：9 用例契约（3 核心 + 6 守卫） |
| oa4rust/docs/ops/oa4rust-r533-conv-item-keyboard-fix.md | A：本文档 |

未改：selectConversation 函数体、业务 API（imChatApi.ts）、搜索框、textarea 消息输入绑定、样式、其他页面、原有 @click 与 role=list/listitem 语义。无关未跟踪 augmentor/web/pnpm-lock.yaml 未入库。

## 二、修复取舍（Fail Loud）

保留 role=listitem + 容器 role=list，仅追加 tabindex=0 与 keydown。理由：会话列表是既有 list/listitem 语义，改成 button/listitem 混排会破坏列表 AX 结构并牵动样式；tabindex=0 使 listitem 可聚焦，Enter/Space 调 selectConversation 与鼠标等价，是满足 SC 2.1.1 的最小改动。AX 中该项仍报 role=listitem（真实、非缺陷项），键盘可达由 tabindex + keydown 提供。契约守卫逐字钉死 role=list、role=listitem、@click、selectConversation 函数体 6 token 零改。

## 三、契约 RED → GREEN

- 契约 9 用例：核心 3（tabindex=0、@keydown.enter、@keydown.space.prevent 均在 .conv-item 开标签且复用 selectConversation(conv)）；守卫 6（@click 保留、role=list/listitem 保留、selectConversation 函数体零改、业务 API 零改、搜索框零改、textarea 零改）。
- 真父态 RED：RED_EXIT=1，3 failed（三项核心属性缺失）| 6 passed（守卫全过）。日志 oa4rust/test-results/r533-red.log。
- 施加修复 GREEN：GREEN_EXIT=0，9 passed。日志 oa4rust/test-results/r533-green.log。

## 四、三 verify（全 exit 0，cd oa4rust）

| # | 命令 | 结果 | 日志 |
| --- | --- | --- | --- |
| V1 | pnpm exec vitest run frontend/apps/desktop/src/views/R533.convItemKeyboardA11y.contract.test.ts | exit 0，9 passed | r533-verify1.log |
| V2 | pnpm test（全量） | exit 0，全绿 | r533-verify2.log |
| V3 | pnpm typecheck（6 项目） | exit 0，全 Done | r533-verify3.log |

## 五、真浏览器双主题 CDP 实证（1440x900 DPR1，headless）

- Runner：oa4rust/test-results/r533-e2e.cjs（本地 Vite dev 加载仓库源码；API stub 占位数据，无真实凭据/真实写库；BASE=<TEST_HOST>）。
- 报告：oa4rust/test-results/r533/r533-ax-report.json，R533_E2E_DONE groups=2 fatals=0，两主题一致：
  - .conv-item CDP AX：role=listitem、attrs.tabindex=0（可聚焦），itemCount=2。
  - Tab：从搜索框连按 Tab，tabHitIndex=0 命中会话项（tabReachesConv=true），对照修复前不可达。
  - Enter：聚焦首项按 Enter → activeIdx=0、hasChatContent=true（enterSwitched=true）。
  - Space：聚焦次项按 Space → activeIdx=1、hasChatContent=true（spaceSwitched=true）；spaceScroll before=0 after=0，@keydown.space.prevent 防页面滚动生效。
  - 鼠标对照：点击次项 → activeIdx=1、chatName 含「R533会话乙」（mouseSwitched=true），@click 行为零回归。
  - 搜索框仅过滤：填「乙」→ itemCount=1、未选中会话（searchSwitched=false），行为不变。
  - pageErrors=[]、fatal=null。
- textarea 零回归补充探针 r533-textarea-probe.cjs：E2E 中 textarea.found=false 系检查时机在未选中会话的占位态（输入区未渲染），非回归；补探针在点击选中会话后验证——found=true（tag=TEXTAREA，placeholder/aria-label/rows 正常）、v-model 绑定生效（填入后发送按钮 disabled=false）、Shift+Enter 换行生效（hasNewline=true）、0 pageerror。IMChat.vue:96-99 textarea 绑定逐字未改，契约守卫已源码级钉死。

## 六、证据 sha256

| 产物 | sha256 |
| --- | --- |
| IMChat.vue（修复后） | fe5b08a1d317771b5afcdd31515c2d873d10c360f6304401be7e5548ac2df3ed |
| R533 契约 | 12f8a8e0e4800f501a26406f3feb4ae5648170c4794d754198a042434d616ff9 |
| r533-red.log | d35c6ac58e710469da3dd5c2b686936363b304bfa4cb825f18249cb7e5daac3d |
| r533-green.log | 15d7177cc6bf805ae35f904647837450f951f453eaf06e7524c37a3eefdc54ac |
| r533-verify1.log | ab0f306a20f3c03b797868bde262ebb342b31d839a36f2dadbe8da6e5349a409 |
| r533-verify2.log | e35a8cd649c11e5de6ce2834d7230f29476186977c4421f340752dd2990643b7 |
| r533-verify3.log | e3b52a586c51cfd3b99e27c7a152a6c6113bd95a10bcb4087e3251901744fdff |
| r533-e2e.cjs | 7bd15cd3cef2fafd4184b8396cb6d75a32921553ac122a83ad0542e53dd6e650 |
| r533-ax-report.json | d6503ed4fab88fc7eb01e57bbeead712367ae5e258b313d40808ad55c53a8804 |
| r533-textarea-probe.cjs | f1c6fee397965231a82ed6e0b9dd820ecda9a73f3b015972d3b24035161131d0 |
| r533-textarea-probe.json | 94771ab72174856bed601d5fe0de06ea738042127b9b82bb60da5c08658a6960 |
| r533-im-list-light.png | e1c0ad81f7768817f777d052d76bad5a783d329964bca3979e14dc83451000b4（1440x900，177616B） |
| r533-im-list-dark.png | 650143cf75477aab73cb4842d4b9979a988814e5b468a03d8395a4e0140c0140（1440x900，231438B） |
| r533-after-keyboard-light.png | 4652bf5df1516c9998a4255b6ca812817d9ae6c55b4e1c6ba01046553465b6a4（1440x900，177587B） |
| r533-after-keyboard-dark.png | 8c33354d855744567a96997514022c1809b3718a60aacbe807e53d9d831cda33（1440x900，231317B） |

4 张截图均 1440x900 目视正常：列表双主题显示「R533会话甲/乙」两条会话；键盘后截图为搜索框操作后的占位态，渲染无异常。全部 runner/日志/截图/报告位于 .gitignore 排除的 oa4rust/test-results/，不入库。

## 七、脱敏

全文无密钥/口令/真实域名/IP/主机名/SSH 端口/本机绝对路径/个人目录；服务地址以 <TEST_HOST> 占位，API stub 为占位数据，无真实凭据。
