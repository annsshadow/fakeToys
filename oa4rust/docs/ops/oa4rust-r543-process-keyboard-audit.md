# R543 H5 审批页伪按钮键盘可达性甄别（WCAG 2.1 SC 2.1.1 A 关联 SC 4.1.2 A）
- 轮次：R543（audit，t51），真父 `8b611dee11ac8fda75dcc6c4bdc4c278948c9206`（台账 §533，已 push），attempt `c4cbb258-5bee-42ed-aca3-a3f3b2ec936c`。
- 性质：只读甄别，不改源码、不提交业务代码。属 R539/R540（index 工作台）、R541/R542（message 消息页）已修同缺陷类不同页面实例（有 R509/R515/R532 先例，维度覆盖 ≠ 实例覆盖），静态线索严禁预判，全部据实 CDP 取证。
- 环境：自起 uni H5（`http://127.0.0.1:<PORT>/oa4rust-h5/`，<TEST_HOST> 占位），Playwright chromium headless，390×844 DPR2；认证/流程列表/详情数据以 stub 注入，0 真值。H5 App.vue 无主题机制，light/dark 截图 sha 相同。
- 判据口径：document.activeElement + Accessibility.getFullAXTree 中 focused=true 非 root 节点 AX role/name，getPartialAXTree 与 getFullAXTree 按 backendDOMNodeId 映射比对；不探宿主 uni-view/unibutton wrapper（R535 教训），注入对照 + calendar 正面对照双向去伪。
## 一、结论（三分判定）
**可达真缺陷 1 项 P2**：process 审批页 6 类 <view>/<text> @tap 伪按钮——3 个 tab 切换（:259）、列表项打开详情（:276，50 条造数坐实非空）、错误重试（:269/:292）、加载更多（:295）、详情关闭（:299 mask / :303 sheet-close）、发起流程 fab（:333）——DOM 无 role/tabindex/aria-label，CDP AX partial/full 均 role=generic name=null，Tab 22 次（详情态再 22 次）无一命中，focused=true 非 root 节点为 0。归一族（SC 2.1.1 Keyboard A 关联 SC 4.1.2 Name Role Value A）。
- 非 L1（不可达）：tab 常驻且合成 click 后 .on 类在三项间切换（changed=true）；列表项 stub 造数 50 条真实可达，详情浮层 detailOpened=true/关闭后 detailClosed=true；错误态/空态均为真实可触发状态。均非死代码。
- 非正常：注入 role=button tabindex=0 aria-label 后同节点 AX 变 button/focusable=true/name=注入值且进 Tab 序列 11 次；同页 calendar.vue:295 正面对照 role=button name=新建日历 可读，证明 AX 通道正常而非探针假阳性。
- 0 缺陷据实判 0、有缺陷据实判缺陷，未臆造。
## 二、源码锚点（逐行核实）
| 文件:行 | 代码 | 判定 ||---|---|---|
| process.vue:259 | <view v-for t in tabs class=tab ai-brand-fill @tap=switchTab(t.key)> | tab 切换伪按钮，缺陷 |
| process.vue:276 | <view v-for row in rows class=item o2-list-item @tap=openDetail(row,i)> | 列表项打开详情唯一入口，缺陷 |
| process.vue:269/:292/:311 | <view class=retry-btn @tap=loadFirst/loadMore/retryDetail>重试</view> | 重试伪按钮，缺陷 |
| process.vue:295 | <view v-else-if=hasMore class=load-more @tap=loadMore>加载更多</view> | 加载更多伪按钮，缺陷 |
| process.vue:299/:303 | <view class=mask @tap=closeDetail> / <text class=sheet-close @tap=closeDetail>✕</text> | 详情关闭伪按钮，缺陷 |
| process.vue:333 | <view class=fab @tap=goStart>发起流程</view> | 发起流程伪按钮，缺陷 |
| process.vue:280/:283 | <button size=mini ... @tap.stop=act(row,reject/approve)> | 原生 button，合规，明确排除，不混入伪按钮族 |
| process.vue:212-216 | switchTab(key)：active.value=key; loadFirst() | 业务 handler，鼠标可达键盘无入口 |
| process.vue:49-80 | openDetail(row,i)：detailOpen=true + Promise.all 三 API | 详情业务，造数可达 |
| calendar.vue:295 | role=button aria-label=新建日历 | 正面对照，AX 名可读 |
| main.ts:16-82 | installH5NativeInputNameA11y 仅表单名增强，无伪按钮补救 | 全局无兜底 |
## 三、决定性 CDP 证据（6 组 = light/dark × 列表/错误/空态，fatals=0 pageErrors=0）
| 状态 | 宿主 class | 数量 | DOM role/tabindex/aria | AX partial/full | AX name | Tab 命中 ||---|---|---|---|---|---|---|
| 列表(造数 50) | .tab | 3 | null/null/null | generic/generic | null | 0（22 次全 BODY） |
| 列表 | .item.o2-list-item | 50 | null/null/null | generic/generic | null | 0 |
| 列表 | .load-more | 1 | null/null/null | generic/generic | null | 0 |
| 列表 | .fab | 1 | null/null/null | generic/generic | null | 0 |
| 详情浮层 | .sheet-close / .mask | 各 1 | null/null/null | generic/generic | null | 0（详情态 22 次） |
| 错误态 | .retry-btn | 1 | null/null/null | generic/generic | null | 0 |
| 空态 | .fab | 1 | null/null/null | generic/generic | null | 0 |
- partial↔full backendDOMNodeId 映射一致（fullMatched=true），排除单一树假象。
- 补充探针：24 次 Tab 中 document.activeElement 全为 BODY，getFullAXTree 中 focused=true 非 root 节点数 0（anyPseudoFocused=0、anyButtonFocused=0）——键盘焦点从未进入任何伪按钮，也未进入原生按钮。
- 注入对照：运行时给 .tab 加 role=button/tabindex=0/aria-label 后，partial/full AX 均 role=button name=R543-injected-tab props=[focusable=true]，进 Tab 11 次。
- 正面对照：calendar.vue:295 编译后 role=button aria-label=新建日历，AX role=button name=新建日历，证明页面级 AX 播报链路有效。
- tab 切换功能实证：合成 click 第二项后 .on 类从第 1 项移到第 2 项（changed=true），坐实非死代码。
- 原生 <button> 合规明确排除（Fail Loud 据实披露）：源码 :280/:283 为原生 <button>，但 uni-app H5 编译后 DOM 为 <uni-button> 自定义元素（100 个，内部无原生 <button> 子节点，AX role=generic、focus() 后 activeElement=BODY），即原生按钮在 H5 端同样键盘不可达——属另一族（uni-app 编译层）观察项，契约明确要求排除，本轮不混入伪按钮缺陷族，仅据实记录供后续独立甄别。
- 等价入口核查：process 页无搜索/过滤输入框（grep 仅命中 @tap/role/键盘锚点，无 input/search/filter），无其它键盘路径完成 tab 切换/打开详情/重试/加载更多/关闭/发起流程；tabBar 审批项仅页面导航，不构成对页内控件豁免。
## 四、证据清单（sha256）
| 项 | 文件 | sha256 ||---|---|---|
| 源码 | mobile/src/pages/process/process.vue | `ecf15d78a5c1aecf74b3040a471eb0494d131b390549c7c5548a694b6f4449a3` |
| 源码 | mobile/src/main.ts | `b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b` |
| 源码 | mobile/src/pages/calendar/calendar.vue（正面对照） | `75c2c2bdbbb1f1e78b5259749e278158164105fcb8512f94208f2977983d4838` |
| 源码 | mobile/src/pages.json（tabBar 核查） | `fc21d8e8a8a2f9f80d2cf192db965ec506e894a1b358685ebf2731169e41a466` |
| 报告 | test-results/r543/r543-report.json | `789593c98c57d5bc2f0c09380a118e0d7bab2d4774ef7368afb501d83be74e53` |
| 报告 | test-results/r543/r543-supplement.json | `75cf6efa1663a9ea3e086a88d0ee7941f628da252f4eaf24e3d756406fbceb3f` |
| 报告 | test-results/r543/r543-nativebtn.json | `c2dbe6c539f779ae124117a8f91ec82d273b2beabb3b02bbe8a57fed8f0d78f0` |
| 截图 | r543-withData-light/dark.png | `9867930e4fe6aaa63bd8b50c8420a2fdc4478957cf54acdb385e547624b3da0f` |
| 截图 | r543-error-light/dark.png | `89933745548741712fd0bc25228aa98bd4bdbed4753523519eacd57999b76178` |
| 截图 | r543-empty-light/dark.png | `fc2cf51a43a8c0cb6c1b925289e92d5840f8c8b63714af0ed20be1d315fe2062` |
| 探针 | test-results/r543-e2e.cjs / r543-supp.cjs / r543-nativebtn.cjs / r543-parse.cjs | gitignored，运行时脚本 |
证据全部位于 gitignored 的 oa4rust/test-results/r543-*（.gitignore:28），未入库。
## 五、覆盖甄别与最小修复方向
- 与 R534-R538（表单控件可访问名）为不同维度；与 R539/R540、R541/R542 为同缺陷类不同页面实例，符合维度覆盖不等于实例覆盖先例。
- 去伪：不探 UNI-VIEW/UNI-BUTTON 宿主 wrapper（generic 假阳性），以 DOM 原生节点 + AX partial/full + Tab/focused 实测为准；注入对照 + calendar 正面对照排除 AX 通道失效辩解；造数/功能切换排除空态与死代码辩解；原生 button 按契约明确排除并 Fail Loud 记录。
- 最小修复方向（供后续 repair 任务，本轮不改）：对上述伪按钮追加 role=button tabindex=0 :aria-label（列表项用 titleOf(row,i) 动态名、tab 用 t.label），并补 @keydown.enter/@keydown.space.prevent 复用 switchTab/openDetail/loadFirst/loadMore/closeDetail/goStart；业务 handler 零改、原 @tap 鼠标行为保留。与 R540/R542 逐页显式补口径一致（全局增强器越界已 Fail Loud 排除）。
（文档脱敏：真域名/IP/凭据/个人路径均未入库，统一用 <TEST_HOST> / <PORT> 占位。）