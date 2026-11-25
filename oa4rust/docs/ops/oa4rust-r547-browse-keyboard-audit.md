# R547 H5 浏览页伪按钮键盘可达性甄别（WCAG 2.1 SC 2.1.1 A 关联 SC 4.1.2 A）
- 轮次：R547（audit，t59），真父 `88ee74708040aa70b1dc0eee19dc899c94f869b0`（台账，已 push），attempt `9951eca5-fd45-45f2-9a8d-57a4d5331fbf`。
- 性质：只读甄别，不改源码、不提交业务代码。与 R543（process.vue:259 页签、列表项）同构先例；维度覆盖 ≠ 实例覆盖，静态线索严禁预判，全部据真浏览器 CDP 取证。
- 环境：自起 uni H5（`http://<TEST_HOST>:<PORT>/oa4rust-h5/` 占位），Playwright chromium headless 390×844 DPR2，page.route 独立 stub 造数（认证/三数据源列表/详情/HTTP500 错误态）；H5 App.vue 无主题机制，light/dark 截图 sha 相同。0 真值入库。
- 判据口径：决定性问题为 `document.activeElement` 与 `Accessibility.getFullAXTree` 中属性 `focused=true` 的非 RootWebArea/document/BODY 节点（R535 教训：严禁探宿主 wrapper 假阳性）；对每个候选读 DOM role/tabindex/aria-label 与 `getPartialAXTree`/`getFullAXTree` 按 backendDOMNodeId 映射比对；注入对照 + calendar:295 跨页正面对照双向去伪。

## 一、结论（三分判定）
**可达真缺陷 1 项 P2**：browse 浏览页 7 类 <view>/<text> @tap 伪按钮在键盘通道完全不可达且 AX 语义不可读——
① 页签 v-for（:124 @tap=switchTab，常驻，3 个：门户/查询/栏目）；② 空态重试（:135 @tap=load）；③ 空态刷新（:142 @tap=load）；④ 空态去搜索（:143 @tap=goSearch）；⑤ 有数据错误态重试（:150 @tap=load）；⑥ 列表卡片 v-for（:152 @tap=open）；⑦ 详情关闭（:165 @tap=detail=null）。
违反 SC 2.1.1 Keyboard（A）：功能只能鼠标/触控操作，键盘用户无路径；关联 SC 4.1.2 Name, Role, Value（A）：role/tabindex/aria 全空，AX role=generic name=null。
三分明细：上述 7 类均为「可达真缺陷」（造数下点击可切页签、打开/关闭详情、错误态重试条真实呈现，非死代码）；空态无列表项、不另计伪按钮缺陷；原生 <button> 在本 H5 页 grep=0，无 uni-button 实例需排除；已修 index/message/process/calendar:295 实例、PC 侧已覆盖实例不重复立案。

## 二、源码逐行锚点
- :18-22 三页签 tabs（portal 门户/query 查询/cms 栏目）；:26 tab=ref(portal)；:64-67 onShow 经 ensureAuthenticated 后 load()，非死代码。
- :69-74 switchTab(key)：切 tab 后 load([])；:76-105 open(r)：按页签取详情（portalApi.pageDetail / queryviewApi.execute / cmsApi.documentSearch），置 detail.value；:115-117 goSearch() navigateTo 搜索页。
- :124 <view v-for=t class=tab @tap=switchTab(t.key)>（无 role/tabindex/@keydown）；:135 <view class=retry-btn @tap=load>重试（rows=0 且 loadError）；:142/:143 <view class=empty-guide-link @tap=load / @tap=goSearch>（rows=0 空态）；:150 <view class=retry-btn @tap=load>（有数据 + loadError 横幅）；:152 <view v-for=r class=card @tap=open(r)>；:165 <text class=detail-close @tap=detail=null>关闭。
- services/index.ts:469-505：portalApi.pageList→/api/portal/assemble/surface/page/list/portal/default、pageDetail→/api/portal/assemble/surface/page/v2/{id}；queryviewApi.viewList→/api/queryview/view/list/query/all、execute→/api/queryview/execute/{view}/{id}；cmsApi.columnList→/api/cms/core/entity/column/list、documentSearch→/api/cms_assemble_control/document/search。
- utils/loadList.ts:32-37 三分支 listState（loading→error→empty→list，错误优先空态）；browseFeed.ts:21-26 loadBrowseRows 复用统一口径（label=加载失败）。错误文案实测为「加载失败：HTTP 500」。
- main.ts:16-67 installH5NativeInputNameA11y 只处理 input/textarea/picker 名称透传，不覆盖 @tap 伪按钮键盘语义；全局无 installA11y 键盘增强器（与 desktop main.ts 差异）。

## 三、决定性 CDP 证据矩阵（10 组 light/dark × withData/error/empty/queryError/cmsError + 2 组补充态，全 fatals=0 pageErrors=0）
| 候选 | DOM role/tabindex/aria | partial AX | full AX | DOM.focus 后 activeElement | getFullAXTree focused 非 root | Tab 30 次命中 |
|---|---|---|---|---|---|---|
| 页签 ×3（:124） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 空态重试（:135，error 态 count=1） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 空态刷新/去搜索（:142/:143，empty 态 count=2/1） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 有数据错误态重试（:150，切换后 count=1） | 全 null | role=generic name=null（name 空串） | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 列表卡片（:152，withData count=3） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 详情关闭（:165，详情态 count=1） | 全 null | role=generic name=null（name 空串） | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |

## 四、反预判与去伪
1. **AX 通道正常（注入对照）**：对首个页签运行时注入 role=button + tabindex=0 + aria-label=R547-injected-tab 后，同节点 partial AX role=button / name=R547-injected-tab 可读，Tab 30 次命中该注入节点 15 次（双主题一致）——证明 generic/null 是源码缺属性所致，非 CDP 通道失灵。
2. **跨页正面对照**：calendar:295 按钮 partial/full AX 均 role=button / name=新建日历 可读，证伪「本页 AX 全不可读」的环境性辩解。
3. **非死代码（page.route 注入据实记录）**：withData 造 3 条门户行，点击卡片 → detailOpened=true，点关闭 → detailClosed=true；error 态注入 HTTP500 首屏错误条 count=1 且 errorText=加载失败：HTTP 500；empty 态注入空集得刷新+去搜索 count=2；queryError/cmsError 态注入对应页签 HTTP500，点击页签切至查询/栏目后 errorText=加载失败：HTTP 500、retry count=1、cards=0（on 类切换坐实非死代码）。
4. **等价键盘入口逐项核查**：browse 页无搜索/过滤输入框、无原生 button、无 keydown 处理器，@tap 之外无任何键盘可达替代；goSearch 虽导航至搜索页，但不替代当前页页签/卡片/重试/关闭控件；tabBar 仅 5 项页面导航不豁免本页内操作；空态无列表项不计独立缺陷；tabHits（tab/card/retry/emptyLink/detailClose/BUTTON）全为 0。
5. **另一族与已覆盖排除**：本页 grep 原生 <button>=0，无 uni-button 编译层实例需排除；index/message/process/calendar:295 已修实例、PC 侧已覆盖的 2.1.1 实例不重复立案；3.3.8 验证码不属本族。
6. **截图名实相符**：详情态截图显示门户 3 行 + 详情浮层（含关闭文字），查询错误态显示「查询」页签选中 + 加载失败：HTTP 500 + 重试按钮，空态显示暂无数据 + 刷新/去搜索，均与 AX 断言逐项对应。

## 五、最小修复方向（供后续 repair，业务零改）
按 R544/R546 逐页显式补口径（全局增强器已 Fail Loud 排除，不新增全局逻辑）：为上述 7 类 <view>/<text> @tap 控件追加 role=button + tabindex=0 + 动态 aria-label（如「门户页签」「查询页签」「重试加载浏览数据」「打开 R547portal0 详情」「关闭详情」），并追加 `@keydown.enter.prevent` 与 `@keydown.space.prevent` 复用原 @tap handler；保留 @tap 不动，鼠标/触控行为零改。

## 六、证据 SHA-256（全在 gitignored oa4rust/test-results/r547/，未入库）
- 源码：browse.vue=`16ea81b75a320b795122cb35a3a16c8089e474da477f744b33a816ff1920077d`；browseFeed.ts=`4c5e5e283bf2be4f7d1032fa17a68ba742e9507648f7860e45d483aa6bced76c`；main.ts=`b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b`；pages.json=`fc21d8e8a8a2f9f80d2cf192db965ec506e894a1b358685ebf2731169e41a466`。
- 报告：r547-report.json=`a0b8b04167f642ba013bfdc7a097578a1a75ca9a904eab121e6778b36e00a03e`（10 组）；r547-detail-report.json=`a1fbd739fe8783c8a689d8c981301901c5f2dfc7d12c30b2ef84d73742bbbfe6`（2 组补充态）。
- 截图（780×1688，light/dark 同 sha 因 H5 无主题机制）：首屏 withData=`656ad76127eda320a6494ce25588c03f1003462c5827f503ec226fbc642a01e6`；首屏 HTTP500 错误态=`fe6dea6f655d5b9efd183f55ad17aae490f5028ea47369f2ba5826ec50db0397`；空态=`363005b7c95eb18b1929f4615b76ab20fb25251b41e0480e76735faf16315294`；详情态=`ce41db47816826a47e2ba7fc253b6eae0dd38e8949cfa38ccd64ecff5fdfd71f`；切换后查询错误态=`b3e4ee68a2bf920ac99e60d4769c3030482c4ced3863042693efc9f3ffc721fa`；切换后栏目错误态=`4b7789eb20c181ad184523c45e68b0a804fced586455ab1788ee9f917a73df62`。
- 探针脚本：r547-e2e.cjs / r547-parse.cjs / r547-detail.cjs 与报告/摘要/截图均在 gitignored 目录（.gitignore:28）；域名/端口用 <TEST_HOST>/<PORT> 占位，0 真值。