# R551 H5 会议页伪按钮键盘可达性甄别（WCAG 2.1 SC 2.1.1 A 关联 SC 4.1.2 A）
- 轮次：R551（audit，t67），真父 `4bc75440b494fac6126e1f4c2ebd22a963500b68`（台账§541入账后），attempt `860881ba-0a53-4ba3-98e0-2e5f53b404f3`。
- 性质：只读甄别，零源码改动、仅提交唯一文档。同缺陷类不同页先例 R539(index)/R541(message)/R543(process)/R545(contacts)/R547(browse)/R549(calendar)；维度覆盖 ≠ 实例覆盖，静态仅备料，全部据真浏览器 CDP 取证，不预判 P2 也不因"同族已覆盖"判 0。
- 环境：自起 uni H5（`http://<TEST_HOST>:<PORT>/oa4rust-h5/` 占位），Playwright chromium headless 390×844 DPR2，page.route 独立 stub 造数（认证/待接受/待确认/我的会议分页/HTTP500）；H5 App.vue 无主题机制，light/dark 截图 sha 相同。0 真值入库。
- 判据口径：决定性证据为 `document.activeElement` 与 `Accessibility.getFullAXTree` 中属性 focused=true 的非 RootWebArea/document/BODY 节点（R535 教训：严禁探宿主 wrapper 假阳性）；逐候选读 DOM role/tabindex/aria 与 `getPartialAXTree`/`getFullAXTree` 按 backendDOMNodeId 映射比对；Tab 遍历命中数、鼠标 click 对照、运行时注入对照与跨页 :295 语义对照去伪。

## 一、结论（三分判定）
**可达真缺陷 1 项 P2**：meeting.vue 6 类 `<view @tap>` 伪按钮在键盘通道完全不可达且 AX 语义不可读——
① 常驻三页签（:164 v-for @tap=switchTab，待接受/待确认/我的会议）；② 列表错误态重试（:175 @tap=load）；③ 空态刷新（:182 @tap=load）；④ 空态换页签（:183 @tap=switchTab）；⑤ 我的会议 footer 错误重试（:211 @tap=loadMore）；⑥ 我的会议加载更多（:214 @tap=loadMore）。
违反 SC 2.1.1 Keyboard（A）：功能只能鼠标/触控操作，键盘用户无路径；关联 SC 4.1.2 Name, Role, Value（A）：role/tabindex/aria 全空，AX role=generic name=null。
三分明细：上述 6 类均为「可达真缺陷」——常驻页签/空态零造数可达，错误态经 HTTP500 拦截可达，footer 重试与加载更多经 50 条分页造数可达（鼠标点击实证）；空态无列表卡片不计独立缺陷；页内原生 `<button>`（:195/:196/:199/:200/:203/:204/:222）在 uni-app H5 编译为 uni-button 自定义元素，属另一族按 R545 契约排除、不混入本轮 6 类；已修 index/message/process/contacts/browse/calendar 实例与 PC 侧已覆盖实例不重复立案。

## 二、源码逐行锚点
- oa4rust/mobile/src/pages/meeting/meeting.vue（sha256 `260bb7dfbdefa8985472453a8c63143517c71308aa28f3d5812b1ef29f2625c2`）：:20-24 TABS=[待接受 wait/待确认 confirm/我的会议 mine]；:26 tab=ref(wait)；:36-38 listView computed(listState)；:40-61 load() 按 targetTab 调 meetingApi.invitedWait/waitConfirm/applied(1,50)，loadList label=会议加载失败，hasMore 仅 mine 且 result.ok；:63-88 loadMore() applied(next) label=更多会议加载失败，失败保留已加载行；:90-93 onShow 经 ensureAuthenticated 后 load()；:95-100 switchTab(key) 切 tab 后 load([])；:102-126 act/accept/reject/allow/deny/checkin。
- 模板锚点：:158 `<view class=tabs>`；:164 `<view v-for=t in TABS class=tab :class={on} @tap=switchTab(t.key)>`（无 role/tabindex/@keydown）；:175 `<view class=retry-btn @tap=load()>`；:182 `<view class=empty-guide-link @tap=load()>`；:183 `<view class=empty-guide-link primary @tap=switchTab(...) >`；:211 `<view class=retry-btn @tap=loadMore>`；:214 `<view class=load-more @tap=loadMore>`。
- 入口 oa4rust/mobile/src/pages/index/index.vue:151 会议 cell 已武装 `role=button tabindex=0 aria-label=会议 @tap/@keydown.enter/@keydown.space.prevent=goPage(/pages/meeting/meeting)`，一跳直达本页（非死代码）。
- main.ts（sha256 `b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b`）:16-67 installH5NativeInputNameA11y 只处理 input/textarea/picker 名称透传，不覆盖 @tap 伪按钮键盘语义；全局无 installA11y 键盘增强器。meetingApi 端点 services/index.ts:390-416（invited/wait、wait/confirm、apply/{page}/size/{size}、accept/reject/confirm/checkin）。

## 三、决定性 CDP 证据矩阵（12 组 light/dark × withData/error/empty/mineMore/mineError/errorConfirm + 4 组补充，全 fatals=0 pageErrors=0）
| 候选 | 触发态/count | DOM role/tabindex/aria | partial/full AX | DOM.focus 后 activeElement | getFullAXTree focused 非 root | Tab30 命中 | 鼠标对照 |
|---|---|---|---|---|---|---|---|
| 三页签 :164（3 个） | 常驻 count=3 | 全 null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 | 点待确认：待接受→待确认 changed=true |
| 错误重试 :175 | error count=1，错误文案「会议加载失败：HTTP 500」 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | HTTP500 拦截造态真实呈现 |
| 空态刷新 :182 | empty count=2 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 空态零造数可达 |
| 空态换页签 :183 | empty count=2 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 点击：待接受→我的会议 changed=true |
| footer 重试 :211 | mineError 点击加载更多后 count=1，错误文案「更多会议加载失败：HTTP 500」 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 50 条首屏 + 第2页 HTTP500 造态可达 |
| 加载更多 :214 | mineMore 我的会议 50 条 count=1 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 点击加载更多后 cards=50 |

主探针 `tab164/retry/emptyLink/loadMore/BUTTON` 五类 Tab30 命中在全部 12 组均 0；全部候选 focus 后 activeElement=BODY（isTarget=false）、axFocused=[]；注入首个页签 role=button + tabindex=0 + aria-label=R551-injected-tab 后 partial/full AX role=button / name=R551-injected-tab（focusable=true），Tab30 命中 15 次，聚焦后 Enter 成功触发（injectEnter=true，light/dark 一致）。

## 四、反预判与去伪
1. **跨页语义对照**：calendar.vue:295 `<text class=calendar-create-btn role=button aria-label=新建日历>` 同节点 DOM role=button / aria-label=新建日历，partial AX role=button / name=新建日历 可读——证明 H5 CDP AX 通道正常、generic/null 是候选源码缺语义属性所致非工具盲点。据实披露 :295 DOM tabindex=null 也不进 Tab，仅取其为 AX 语义可命名对照，不混入本轮 6 类候选。
2. **运行时注入对照**：对 6 类锚点分别注入 role=button + tabindex=0 + aria-label 后（页签 R551-injected-tab / 空态 R551-inj-empty / 错误重试 R551-inj-retry / 加载更多 R551-inj-more），partial AX 均 role=button / name=可读，Tab30 命中（页签 15 次、其余均命中）——证明补语义+键盘即可进入 Tab 并等价触发，键盘不可达可修复且非环境限制。
3. **非死代码**：造数下鼠标点页签切待接受→待确认（changed=true）、空态点换页签到我的会议（changed=true）、mine 页签 50 条卡片分页 + 加载更多出现（loadMore=1）、第2页 HTTP500 后 footer 重试出现（footerRetry=1，错误文案可读）；各控件真实可达，非空态伪影。
4. **等价键盘入口逐项排除**：本页无搜索/过滤输入框可替代上述控件；6 类均只有 @tap 处理器无 @keydown；tabBar 仅页面导航不豁免页内操作；Tab30 全页遍历候选命中 0 且聚焦节点非 root 为 0；空态无列表卡片不计独立缺陷；uni-button 按 R545 契约另族排除。
5. **已覆盖实例排除**：index/message/process/contacts/browse/calendar 同缺陷实例已在各自轮次立案，本轮仅 meeting.vue 6 类锚点；PC 侧覆盖维度不覆盖 H5 实例（先例 R492–R496/R509/R515/R532）。

## 五、最小修复方向（P2，业务零改）
类比 R544/R546/R550/R552 逐页显式补口径：对 meeting.vue:164/:175/:182/:183/:211/:214 各 `<view @tap>` 补 `role=button tabindex=0`、动态 `aria-label`（页签用 t.label、重试/刷新/加载更多用固定中文），并补 `@keydown.enter` / `@keydown.space.prevent` 复用原 handler；原 @tap 与业务函数不动。

## 六、证据 SHA-256 表
| 项 | sha256 |
|---|---|
| 源码 meeting.vue | `260bb7dfbdefa8985472453a8c63143517c71308aa28f3d5812b1ef29f2625c2` |
| 源码 index.vue（入口） | `892d75341e36d115bf61b21b53aaf62d5e53b7f94c06b57b382955cd8408cd40` |
| 源码 main.ts | `b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b` |
| 源码 pages.json | `fc21d8e8a8a2f9f80d2cf192db965ec506e894a1b358685ebf2731169e41a466` |
| 主报告 r551-report.json（12 组） | `741310ddf34e5844ba7e30b60bd690c426f178537e228bb2091020ade3f992f2` |
| 补充报告 r551-detail-report.json（4 组） | `b10056e85abef1a28bffcb6b764de3f9a952bfdedb7072828693f58cc1bae319` |
| 截图 withData（light=dark） | `c3797adce3bdd4b21debe43aa3fa6b92581872422c8be1ae3dd59da0a8b6c5d4` |
| 截图 error（light=dark） | `18d04366228b014697477748d369bad97f4e5d3cf866708b10d05a6c7d08255c` |
| 截图 empty（light=dark） | `63e6f0a688521d2a444495eff895b4933cad8ee26640e61f32b0fd219c1e7ee7` |
| 证据目录 | `oa4rust/test-results/r551/`（`.gitignore` 第28行忽略，不入库） |
