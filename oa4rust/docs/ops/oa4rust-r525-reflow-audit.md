# R525 弹窗窄屏 Reflow（WCAG 1.4.10）只读甄别

- 类型：只读甄别（不改码、不提交源码）
- 目标：WCAG 2.1 SC 1.4.10 Reflow (AA) —— DesignCenterApp.vue「全部设计器清单」弹窗在 320px 等效视口下的重排与横向溢出
- 真父：7b88bd4cb849a1d8d36c3005c25f323b784e4cda（台账 §516）
- 测试域：<TEST_HOST>（占位；运行时凭据经远端注入，真值不入库）
- 视口：320×256 DPR1（等价 1280px 宽 400% 缩放），light/dark 双主题，headless Chromium
- 探针：oa4rust/test-results/r525-e2e.cjs + oa4rust/test-results/r525-run.sh（均被 .gitignore:28 忽略）
- 源码 SHA-256：4e7089786f61225fc46e9fd4b096d9ad3954c56ecdb42e65983062534a0c31b3（DesignCenterApp.vue）
- 报告 SHA-256：ca8a9876b4f0c2e63fca2365841bb89407bdf53221a6a362e3583e04186c7651

## 一、结论（三分判定）

`正常：0 个可达 WCAG 1.4.10 Reflow 缺陷。`

首选候选的静态线索（:55/:295 硬宽度 width:700px、全文件 0 处 @media、无 max-width）成立，但真浏览器实证推翻了「320px 居中溢出负侧不可滚」的预判，去伪链有两道独立否决：

1. 该弹窗为死代码：`showAllList` 全域仅 3 处（:54 v-if、:56 关闭、:77 定义），无任何置 true 的触发入口；自然路径遍历设计中心页全部可见按钮后 `.modal-overlay` 始终 count=0，用户无法打开。
2. 即便运行时 DOM 同构构造该 700px 弹窗（scoped hash `data-v-d6c1a7b7` 与源码一致），flex 容器 `align-items/justify-content:center` 使 flex item 默认 `flex-shrink:1` 将其收缩至视口宽度 320px（rect.left=0、right=320、width=320），无负侧溢出；computed `overflow-x:auto` 提供横向滚动兜底；`documentElement.scrollWidth=320=clientWidth`，页面无横向滚动；关闭按钮 rect.left=288.48、right=299 处于视口内可见可达。

既非可达真缺陷，也非「不可达 L1」（它是构建产物中存在但无触发入口的残留死代码，不是被登录/权限/环境阻断的主流程），据实判正常 0 缺陷，不臆造。

## 二、源码逐行甄别

- `:54` `"div v-if="showAllList" class="modal-overlay" @click.self="showAllList=false""`
- `:55` `"div class="modal glass-card" style="width:700px;max-height:80vh;overflow-y:auto""` —— 内联硬宽度 700px，仅 `overflow-y:auto`（纵向），无 max-width、无 overflow-x 显式声明。
- `:56` modal-header 内 `button.btn-close`，`aria-label="关闭设计器清单"`，`@click="showAllList=false"`。
- `:57-64` `designer-list` 遍历 `allDesigners`。
- `:77` `const showAllList = ref(false)` —— 唯一的 ref 定义；grep 全域 `showAllList` 只命中 :54/:56/:77，无置 true 语句。
- `:6` AdminPageHeader title=设计中心；`:8` button @click=refreshAll；`:9` button @click=showStats=true —— 页面可见操作只有刷新与统计，无「查看全部设计器」类入口。
- `:294` `.modal-overlay{position:fixed;inset:0;background:var(--bg-overlay);display:flex;align-items:center;justify-content:center;z-index:200}`
- `:295` `.modal{...width:700px;max-height:80vh;overflow-y:auto;display:flex;flex-direction:column;gap:12px}`
- 全文件 @media 命中 0 处；未声明 max-width。
- 全局兜底：grep `"\.modal[^a-zA-Z-]|max-width:\s*90vw"` 于 oa4rust/frontend 所有 *.css 无命中；`.glass-card` 仅定义于 `packages/ui/src/theme/dark.css:226`（未含 max-width）。无全局 scoped 之外的 max-width 兜底。

正面对照（同仓既有约定）：33 处 `max-width:90vw` 兜底，例如 `BBSForum.vue:1496` `.modal width:560px max-width:90vw`、`ConfigDesignerApp.vue:477`、`CmsColumnApp.vue:220`、`PortalDesigner.vue:433`、`QueryStatementDesigner.vue:2307` 等。本弹窗是唯一缺 max-width 的 outlier——但实证显示 flex shrink 已等价兜住，未构成失败。

## 三、契约与判据

- SC 1.4.10 Reflow (AA)：内容在 320 CSS 像素宽（等价 1280px 400% 缩放）下须可呈现而不丢失内容或功能，且不得要求二维滚动（纵向滚动允许）。
- 判据：`doc.scrollWidth <= viewport.clientWidth`（无横向二维滚动）；关键控件（关闭按钮）在视口内可达；弹窗 `rect.left >= 0`。
- 等效视口：320×256（1280×1024 的 400%），headless Chromium，DPR1，light/dark 双主题。

## 四、决定性 CDP 证据（双主题一致，fatals=0）

自然路径（design-center 页，点击全部可见按钮）：
- `overlayBefore=0`，`overlayAfter=0`；6 个按钮（含 ☰/🔔/🔄 刷新状态/📊 统计）点击后 `.modal-overlay` 计数始终 0；无可打开路径。
- light 组末尾因点击导航至 dashboard（非弹窗），dark 组停留 design-center；均无弹窗。

构造同构弹窗后几何测量（`width:700px;max-height:80vh;overflow-y:auto`）：
- scoped hash 取到 `data-v-d6c1a7b7`，构造节点带同源 hash，确保 :294/:295 样式生效。
- modal computed `width=320px`、`max-width=none`、`overflow-x=auto`、`overflow-y=auto`；rect left=0/right=320/width=320；`scrollWidth=318≈clientWidth=318`（内容未超容器）。
- overlay `display=flex`、`justify-content=center`、`overflow-x=visible`、rect 0..320。
- doc `clientWidth=320`、`scrollWidth=320`、`overflowX=hidden`；无横向滚动需求。
- close rect left=288.48/right=299，`aria-label="关闭设计器清单"`，处于视口内，截图确认可见。

对照（同弹窗追加 `max-width:90vw`）：computed `max-width=288px`、`width=288px`、rect left=16/right=304，居中留白，同样 `scrollWidth=320`，无溢出——两者均合规。

## 五、截图与取证限制

截图（均 320×256）：
- light：natural=`27256c0c09c70c8c28bdd5ef29b8d58bba0e478a37ae2f18822bef49b0b2ec8e`；hard=`5b2e14a99b3d3a69c29639e05c6543e6c65ef3c91d572030c4d9888ac33074bf`；control=`f86f9bf84f2cbd20106c906d32d4b76b038fe06685d60b1aa73c2d5e145981c5`。
- dark：natural=`5817063893bca58a62d24e922729e47e831ec532ab5b115680f5c50dde1ad964`；hard=`5bad47ceb34fa1368609827ea9e57a5978de967325a6cd99419392aa17aaf0e0`；control=`3b57f26ebc3edff52a038a78b42a0b314bb5c29dab5a9af70d840d209c6da1eb`。
- `hard-320-light.png` 视觉：弹窗满宽居中、标题「全部设计器清单」、列表项可见、右上角 × 关闭按钮可见，无负侧裁切。
- `natural-320-light.png` 视觉：仅系统管理页卡片与底部导航，无弹窗，印证死代码。
- 取证限制：生产构建未暴露 Vue setupState，无法从组件实例置 `showAllList=true`；改用同源 scoped hash 的 DOM 同构构造测量样式效果，属等价几何验证，非真实用户路径（真实用户路径已被证明不可达）。

## 六、三分判定

- 可达真缺陷：0 —— 自然路径无触发入口；构造路径几何实测无横向溢出、关闭可达。
- 不可达 L1：0 —— 弹窗存在于构建产物但属残留死代码，非登录/权限/环境阻断的主流程；不充作 L1。
- 正常：1 项（本候选）—— SC 1.4.10 未失败；缺 max-width 仅为代码风格 outlier，`flex-shrink` + `overflow-x:auto` 已提供等价兜底。

## 七、覆盖甄别

- 次强候选 `BBSForum.vue:1384-1401` 页头 flex 无 flex-wrap + `:1413` sidebar width:200px：320px 下 sidebar rect left=-280/right=0，为默认收起的响应式抽屉（需汉堡按钮展开），`doc.scrollWidth=320` 未溢出；grep 未见该 sidebar 引发横向滚动。去伪风险高，仅备案不主判。
- 关联的 65 个 overflowing 元素（sidebar 内 nav-group/nav-item 等 left=-280）均为收起抽屉内部，不计入 reflow 失败。
- 未改任何源码、未改探针外的仓库文件；证据与脚本全部位于 gitignored 的 `oa4rust/test-results/r525-*`。
