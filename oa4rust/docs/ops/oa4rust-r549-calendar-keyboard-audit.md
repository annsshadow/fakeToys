# R549 H5 日历页伪按钮键盘可达性甄别（WCAG 2.1 SC 2.1.1 A 关联 SC 4.1.2 A）
- 轮次：R549（audit，t63），真父 `404f7d7f66cd97f4530f1e907d76cc773d6a702a`（台账§539入账后），attempt `470e8aa4-f267-4157-9e7b-dee47ef5954f`。
- 性质：只读甄别，不改源码、不提交业务代码。同缺陷类不同页先例 R539(index)/R541(message)/R543(process)/R545(contacts)/R547(browse)；维度覆盖 ≠ 实例覆盖，静态仅备料，全部据真浏览器 CDP 取证。
- 环境：自起 uni H5（`http://<TEST_HOST>:<PORT>/oa4rust-h5/` 占位），Playwright chromium headless 390×844 DPR2，page.route 独立 stub 造数（认证/日历/事件/详情/HTTP500 五态）；H5 App.vue 无主题机制，light/dark 截图 sha 相同。0 真值入库。
- 判据口径：决定性证据为 `document.activeElement` 与 `Accessibility.getFullAXTree` 中属性 `focused=true` 的非 RootWebArea/document/BODY 节点（R535 教训：严禁探宿主 wrapper 假阳性）；逐候选读 DOM role/tabindex/aria 与 `getPartialAXTree`/`getFullAXTree` 按 backendDOMNodeId 映射比对；Tab 遍历命中数、鼠标 click 对照、运行时注入对照与同页 :295 语义对照双向去伪。

## 一、结论（三分判定）
**可达真缺陷 1 项 P2**：calendar.vue 9 类 <view>/<text> @tap 伪按钮在键盘通道完全不可达且 AX 语义不可读——
① 范围页签 v-for（:287 @tap=switchRange，常驻，今日/本周 2 个）；② 新建事件（:296 @tap=toggleCompose，常驻）；③ 创建日历（:302 @tap=submitCalendar，展开表单后可达）；④ 保存事件（:318 @tap=submitEvent，展开撰写表单后可达）；⑤ 错误态重试（:327 @tap=load，rows=0+loadError）；⑥ 错误 banner 重试（:332 @tap=load，有日历但过滤请求失败）；⑦ 事件卡片 v-for（:335 @tap=openEvent）；⑧ 详情关闭（:351 @tap=detail=null）；⑨ 删除事件（:356 @tap=removeEvent）。
违反 SC 2.1.1 Keyboard（A）：功能只能鼠标/触控操作，键盘用户无路径；关联 SC 4.1.2 Name, Role, Value（A）：role/tabindex/aria 全空，AX role=generic name=null。
三分明细：上述 9 类均为「可达真缺陷」（造数下鼠标点击可切页签、展开表单、打开/关闭详情、错误态重试条真实呈现，非死代码）；空产品线（noCal）无事件卡不计独立缺陷；本页除 :295 语义按钮外无原生 button 参与候选；已修 index/message/process/contacts/browse 实例与 PC 侧已覆盖实例不重复立案。

## 二、源码逐行锚点
- :39-42 ranges=[今日,本周]；:44 range=ref(today)；:51-56 eventsView 计算属性（loading→error→empty→list，error 优先）。
- :68-93 toggleCalendarForm/submitCalendar（create POST calendar，body name/type=person）；:94-99 toggleCompose；:100-140 submitEvent（coreEventCreate POST /api/calendar/event/create）；:141-163 removeEvent（showModal 确认后 coreEventRemove POST /api/calendar/event/remove）。
- :204-233 load()：myCalendars() → normalizeCalendarRows → 并行 eventsFilter({calendarId}) → allSettled 单日历失败不阻断 → 客户端按 start_time 时间窗过滤排序；:235-238 onShow 经 ensureAuthenticated 后 load()；:240-244 switchRange；:249-263 openEvent → eventDetail(id)，失败回落本地事件行。
- :287 <view v-for=r in ranges class=tab :class={on} @tap=switchRange(r.key)>（无 role/tabindex/@keydown）；:296 <text class=compose-btn @tap=toggleCompose>（无语义）；:302 <view class=c-submit @tap=submitCalendar>；:318 <view class=c-submit @tap=submitEvent>；:327 <view class=retry-btn @tap=load()>；:332 <view class=retry-btn @tap=load()>（error-banner）；:335 <view v-for=e in events class=card @tap=openEvent(e)>；:351 <text class=detail-close @tap=detail=null>；:356 <view class=c-remove @tap=removeEvent(detail)>。
- 同页 :295 <text class=calendar-create-btn role=button aria-label=新建日历 @tap=toggleCalendarForm>：语义属性齐备，作本页 A/B 通道对照。
- main.ts:16-67 installH5NativeInputNameA11y 只处理 input/textarea/picker 名称透传，不覆盖 @tap 伪按钮键盘语义；全局无 installA11y 键盘增强器。calendarApi 端点见 services/index.ts:436-462（myCalendars / event/list/filter PUT / event/{id} / event/create / event/remove）。

## 三、决定性 CDP 证据矩阵（12 组 light/dark × withData/error/empty/errorBanner/detail/noCal + 2 组详情补充，全 fatals=0 pageErrors=0）
| 候选 | 触发态/count | DOM role/tabindex/aria | partial/full AX | DOM.focus 后 activeElement | getFullAXTree focused 非 root | Tab30 命中 | 鼠标对照 |
|---|---|---|---|---|---|---|---|
| 范围页签 :287（2 个） | 常驻 count=2 | 全 null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 | 点本周：今日→本周 changed=true |
| 新建事件 :296 | 常驻 count=1 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 点击展开撰写表单=true |
| 创建日历 :302 | 鼠标展开表单 count=1 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 点击 :295 展开 .calendar-create-form=true |
| 保存事件 :318 | 鼠标展开撰写 count=1 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 点击 :296 展开 .compose=true |
| 错误态重试 :327 | error count=1，错误文案「日程加载失败：HTTP 500」 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 造数状态真实呈现 |
| 错误 banner 重试 :332 | errorBanner count=1，错误文案「部分日历日程加载失败：HTTP 500」 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 造数状态真实呈现 |
| 事件卡片 :335 | withData/detail count=3 | 全 null | role=generic name=null（补充探针精确选择器坐实） | BODY | axFocused=[] | 0 | 点击卡片 detailOpened=true |
| 详情关闭 :351 | 详情态 count=1 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 点击 detailClosed=true |
| 删除事件 :356 | 详情态 count=1 | 全 null | role=generic name=null | BODY | axFocused=[] | 0 | 详情浮层渲染有效（截图） |

## 四、反预判与去伪
1. **同页语义对照（决定性优势）**：:295 同节点 DOM role=button / aria-label=新建日历，partial/full AX 均 role=button / name=新建日历 可读——证明本页 CDP AX 通道正常、generic/null 是候选源码缺语义属性所致，非工具盲点。据实说明：:295 自身 DOM tabindex=null 也不进 Tab（tabCreate=0），本轮仅取其为「AX 语义可命名」对照，其键盘可达缺口不属本轮 9 类 @tap 候选，避免夸大或混报。
2. **运行时注入对照**：对首个页签注入 role=button + tabindex=0 + aria-label=R549-injected-tab 后，partial/full AX 均 role=button / name=R549-injected-tab 可读，Tab 30 次命中该节点 5 次，聚焦后按 Enter 成功触发（injectEnter=true）——证明「补语义+键盘即可进入 Tab 并等价触发」，键盘不可达可修复且非环境限制。
3. **非死代码**：造数下鼠标点击 :295 展开创建日历表单、点击 :296 展开撰写表单、点击页签切今日→本周（changed=true）、点击卡片打开详情（detailOpened=true）再关闭（detailClosed=true）；error/errorBanner 态造 HTTP500 得错误条 count=1 及对应错误文案；各控件均真实可达。
4. **等价键盘入口逐项核查**：本页无搜索/过滤输入框可作为上述控件的替代入口；新建日历/事件/切范围/开关详情/删除均只有 @tap 处理器、无 @keydown；tabBar 仅 5 项页面导航不豁免页内操作；Tab 30 次遍历全页候选命中 0（tab/calCreate/compose/card/retry/detailClose/remove/BUTTON 均 0），getFullAXTree focused=true 非 root 节点为 0；空产品线（noCal）无事件卡不另计缺陷。
5. **另一族与已覆盖排除**：本页无原生 <button> 参与候选（:295 为 <text role=button>，Tab 不入另记）；uni-app <uni-button> 编译层观察项属另一族，按契约不混入 @tap 伪按钮族；已修 index/message/process/contacts/browse 与 PC 侧已覆盖实例不重复立案。
6. **截图名实相符**：详情态截图显示今日/本周页签、新建日历/新建按钮、3 条 R549event 事件卡、详情浮层（含关闭与删除事件），与 AX 断言逐项对应。

## 五、最小修复方向（供后续 repair，业务零改）
按 R544/R546/R548 逐页显式补口径（全局增强器已 Fail Loud 排除，不新增全局逻辑）：为上述 9 类 <view>/<text> @tap 控件追加 role=button + tabindex=0 + 动态 aria-label（如「今日页签」「本周页签」「新建事件」「创建日历」「保存事件」「重试加载日程」「打开 R549event0 详情」「关闭详情」「删除事件」），并追加 `@keydown.enter.prevent` 与 `@keydown.space.prevent` 复用原 @tap handler；保留 @tap 不动，鼠标/触控行为零改。

## 六、证据 SHA-256（全在 gitignored oa4rust/test-results/r549/，未入库）
- 源码：calendar.vue=`75c2c2bdbbb1f1e78b5259749e278158164105fcb8512f94208f2977983d4838`；calendarEnvelope.ts=`9ecfca3215c9261e35cb8adc9261291744590afd1cf631f2df910f145c42f315`；eventTime.ts=`1893b61d66b0655bb4ed3782b592f39e7a2dd1959dee350f9c6e31a3bd35e9bc`；services/index.ts=`6ddefbf6d65ebfe38cedf58982fbad66c925f7f065dbd38d629f2ae19f5d1655`；main.ts=`b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b`；pages.json=`fc21d8e8a8a2f9f80d2cf192db965ec506e894a1b358685ebf2731169e41a466`。
- 报告：r549-report.json=`6d30137d218349cd25564fd297870b4ff1d3afd83096030abc4e95aafa5e2fce`（12 组）；r549-digest.json=`44c7ff5a8fcb78ca03742267499388298dafc0ad0433a9b583d4f2caed6f611c`；r549-detail-report.json=`134c9fe7496b9b13768a085cf3df5dc134f59f127994ea6ba0b716f9c6af1b7f`（2 组详情补充）。
- 截图（780×1688，light/dark 同 sha 因 H5 无主题机制）：withData=`f838fcdb54b82e3d80cee0a10395f8a7a1de00c91d759079ecce62b358ad2dec`；error=`95a14f5930a54a3f0e18ff76082d835ec1c1e888f4f6154b290d317c9708ffa4`；empty=`adbc710e9186145b086c77582d32d7908d6acc07d91b68b4e6f159a6eb19dbc5`；errorBanner=`9ae0685e7990916c3085d73805da4b954a2c923ce967e29987e4db2a7fc696f1`；noCal=`9867930e4fe6aaa63bd8b50c8420a2fdc4478957cf54acdb385e547624b3da0f`；详情态=`e13f20a8e47a81a6b5f2b016f76cad7589713e33d8bf4bd4a4d39a0f2d1b176e`。
- 探针脚本：r549-e2e.cjs / r549-detail.cjs / r549-parse.cjs / r549-show.cjs / r549-detail-show.cjs 与报告/摘要/截图均在 gitignored 目录（.gitignore:28）；域名/端口用 <TEST_HOST>/<PORT> 占位，0 真值。