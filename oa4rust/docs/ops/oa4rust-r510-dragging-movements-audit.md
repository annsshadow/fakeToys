# R510 WCAG 2.1 AA 2.5.7 Dragging Movements 门户组件重排甄别

## 1. 依据与判据

- 轮次 R510，真父 429605082f48b816ebe36f9753ba728e8ff98dbe（台账 §504 已 push，HEAD=UP AB=0/0）。
- WCAG 2.1 AA 2.5.7 Dragging Movements：若功能可通过单指针拖拽操作完成，须提供不依赖拖拽的替代方式（除非拖拽本身是必要或由用户代理/应用决定），以确保动作可用单一指针或键盘完成。门户设计器的模块重排若仅能通过鼠标 HTML5 拖拽完成、检查器与键盘均无序入口，则命中本条。
- 反预判铁律：须真浏览器构造 ≥2 模块、真实触发拖拽重排、监听 dragstart/dragover/drop 事件坐实顺序变化；键盘逐键尝试确认无替代；区分“新增模块”（palette 点击已是替代，不算缺陷）与“重排模块”（缺替代才可能判缺陷）；不把全库 draggable 一律判缺陷（FormDesigner 是正确样本）；0 缺陷据实判 0。
- 证据来源：Playwright Chromium headless，PC 1440x900 DPR1 双主题 + H5 390x844 DPR2 移动视口（非物理真机，属响应式 H5 视图）；CDP AXTree + DOM 事件探针；0 pageerror。
- 测试服域名与凭据仅存在于 gitignored 本地 runner，文档仅用占位符 <YOUR_TEST_DOMAIN> / <TEST_HOST>，0 真值入库。

## 2. 方法与自洽

- 静态锚点（源码 sha256：PortalDesigner.vue=bdc792068426296fed9ba5b53a5bf4844a0c92e1077f76640aa2ec3fabf53ff6；FormDesigner.vue=4903c8b3772fb2c2978b6f7b7f81f962fd696190ebe6e2126cfe948387549435）：
  - oa4rust/frontend/apps/desktop/src/views/PortalDesigner.vue:59-72：画布 article.widget 带 draggable=true，@dragstart=dragIndex=index、@dragover.prevent、@drop.stop=dropWidget(index)、@click=selectedId=widget.id；:331-334 dropWidget→moveItem 实现重排。
  - 同文件 :74-89 检查器：仅标题 input、宽度 select、按类型的文本/地址/数据源、删除模块按钮，无上移/下移/排序入口；全局无重排快捷键实现。
  - 正面对照 FormDesigner.vue:70-81：字段行 draggable 拖拽同时提供 button title=上移 @click=moveField(index,-1) 与 title=下移，行内可键盘操作。
- 真浏览器流程：登录 → /oa4rust/app/admin/portal-designer/ → 设主题 → 真实“新建设计”创建独立设计 → 点击 palette 新增 2 个模块（文本、指标）→ 监听 canvas 上 dragstart/dragover/drop → dragTo 真实拖拽首模块到次位置 → 采顺序与事件 → 聚焦 widget 后逐键尝试 ArrowUp/Down、Alt+Arrow、Ctrl+Arrow → CDP 采 AXTree 检查器控件 → 截图。补采对 FormDesigner 新建表单加 2 字段并点击“上移”作正面对照；H5 视口重复门户模块构造并采检查器按钮与 widget 属性。
- 主轮报告 test-results/r510-dragging/report.json（sha256=495b89a6f0d81e7b3d25a449db52d568fc5b4d10d344854a6c7249b8e882a954）；补采 test-results/r510-dragging/supplement.json（sha256=88f9500922a5fce24e1b01c6b66f2547b9025a43bb50b13aa0c1b7410c9f947c）。两轮共 6 组，均 0 fatal / 0 pageerror。
- 首跑曾因“创建”按钮选择器误用 button[type=submit]（实际为无 type 的 btn primary）导致新建/加模块/拖拽全超时、before 为空；据实披露并修正选择器后重跑成功，未把首跑超时当作“拖拽不可达”或缺陷证据。

## 3. 逐项程序化证据（与 report.json / supplement.json 一致）

| 主题/视口 | 场景 | 操作前 | 操作后 | 事件/替代 | 判定依据 |
|---|---|---|---|---|---|
| light PC | PortalDesigner 模块重排 | [文本, 指标] | 拖拽后 [指标, 文本] | dragstart:widget span-2 / dragover:widget span-2 selected / drop:widget span-2 selected；dragReordered=true | 拖拽真实生效 |
| dark PC | PortalDesigner 模块重排 | [文本, 指标] | 拖拽后 [指标, 文本] | 同上三事件；dragReordered=true | 双主题一致 |
| light PC | 键盘替代尝试 | [指标, 文本] | ArrowUp/Down、Alt+Arrow、Ctrl+Arrow 共 6 键后仍 [指标, 文本] | keyResults 全 changed=false；keyboardReordered=false | 无键盘重排 |
| dark PC | 键盘替代尝试 | [指标, 文本] | 同上 6 键后不变 | 全 changed=false | 无键盘重排 |
| light PC | 检查器 AXTree | article×2（widget，axName 空）、标题 textbox=标题、宽度 combobox=宽度、数据源 textbox=数据源、button=删除模块 | 无上移/下移/排序控件 | ax 6 节点如上；inspector HTML 仅 3 label + 1 button | 无重排替代控件 |
| dark PC | 检查器 AXTree | 同 light | 同 light | 同 light | 无重排替代控件 |
| light H5 | PortalDesigner 模块 | 构造出 [文本, 指标] 2 模块 | inspectorButtons=[删除模块]；widget draggable=true、tabindex 空、role 空 | bodyW=390=innerW 无横向溢出 | 移动视口同样无重排替代 |
| dark H5 | PortalDesigner 模块 | 同 light | 同 light | 同 light | 移动视口同样无重排替代 |
| light PC | FormDesigner 正面对照 | [文本, 多行文本] | 点第 2 行“上移”后 [多行文本, 文本] | upReordered=true；按钮 title=上移/下移/删除，首尾行 disabled 正确 | 工程内已有合规重排范式 |
| dark PC | FormDesigner 正面对照 | [文本, 多行文本] | 同上 | 同上 | 双主题一致 |

程序化自洽链：DOM 事件坐实拖拽是真实重排路径（顺序改变）→ 键盘 6 键全无变化 → AXTree 检查器仅配置/删除无任何排序控件 → H5 视口同样只有删除按钮、widget 无 tabindex/role → 正面对照 FormDesigner 的上移按钮真实可改变顺序。故 PortalDesigner 模块重排仅能靠指针拖拽，无单指针/键盘替代。

## 4. 三分判定

- 可达真缺陷：1 项（P2）。PortalDesigner.vue:59-72 画布模块重排仅能通过 HTML5 指针拖拽完成；检查器（:74-89）无上移/下移/排序控件，键盘 6 种按键尝试均不能重排，H5 视口同样无替代。双主题 PC + H5 共 6 组实证一致。属 2.5.7 要求的可达单指针操作缺替代，非破坏性操作，P2 边界据实不夸大。
- 不可达 / L1：0 类。真实新建设计并构造 2 模块，拖拽重排成功（before/after 顺序变化 + 三事件），非数据不可达；0 pageerror / 0 fatal。
- 正常规范预期：3 类。① 新增模块已有 palette 点击替代（拖拽不是新增的唯一方式，据实不计入缺陷）；② 正面对照 FormDesigner 提供上移/下移按钮且实证有效——证明工程内已有合规范式，本项是单点遗漏；③ 检查器的标题/宽度/数据源等配置控件 AX 名称正确（标题、宽度、数据源），关闭/删除语义清晰，不夸大至 4.1.2。
- 反预判边界：仅抽测 PortalDesigner 模块重排与 FormDesigner 正面对照，未把全库 draggable 元素（如 palette 项、字段行）一律判缺陷；H5 为同一响应式页面在移动视口的呈现（URL 仍为 /app/admin/portal-designer/），据实披露而非独立 H5 原生应用。

## 5. 修复建议（本轮只读不改源码）

- 在 PortalDesigner.vue:74-89 检查器内为上/下移动增加按钮（复用 FormDesigner 的 moveItem 语义），或在 article.widget（:59-68）行内增加上移/下移按钮并加 @click.stop；使重排可由单一指针点击完成，零业务排序逻辑改动。
- 或给 article.widget 增加 tabindex=0、role 与 @keydown（如 Alt+ArrowUp/Down）以支持键盘重排，与拖拽并存。
- 回归验证：真浏览器构造 ≥2 模块后，不拖拽、仅点击上移/下移（或按键）即可改变画布模块顺序；PC 双主题与 H5 视口均通过。

## 6. 证据与 sha256

| 证据 | sha256 |
|---|---|
| test-results/r510-dragging/report.json | 495b89a6f0d81e7b3d25a449db52d568fc5b4d10d344854a6c7249b8e882a954 |
| test-results/r510-dragging/supplement.json | 88f9500922a5fce24e1b01c6b66f2547b9025a43bb50b13aa0c1b7410c9f947c |
| pc-portal-light-r510.png | 2dccdff85dc8f4e6d8572e7530b1c6ecc8fb5f3e1b7bc46b6a2cbe0bde71ad9e |
| pc-portal-dark-r510.png | c7808b8fed71515f3fc9dbaefd0ede636c8ced9f0ff3ed652c63269d618f859b |
| pc-formdesigner-control-light-r510.png | 16e8d39015ef4e966b0405f183571f05e410d82bc18ee0984fa78aca61da5d15 |
| pc-formdesigner-control-dark-r510.png | c1ca2a78decfe1a5bd7233196b92ae22500d42f86c2b27a3909b408a3a34c729 |
| h5-portal-light-r510.png（780x1688，DPR2） | 163c17a424cf637b2cd32f3e0e6985ca04277794a8b9369cc0392287ac458a40 |
| h5-portal-dark-r510.png（780x1688，DPR2） | f03484fffc138557c9e4bb66363d215c7871adcffde4e08d9d9e9c1310f30f16 |
| frontend/apps/desktop/src/views/PortalDesigner.vue（锚点） | bdc792068426296fed9ba5b53a5bf4844a0c92e1077f76640aa2ec3fabf53ff6 |
| frontend/apps/desktop/src/views/FormDesigner.vue（正面对照） | 4903c8b3772fb2c2978b6f7b7f81f962fd696190ebe6e2126cfe948387549435 |

全部 sha 由命令行 sha256sum 对磁盘实存文件核验。截图均为 headless 非物理真机：PC 1440x900 DPR1，H5 390x844 DPR2（截图 780x1688）；双主题实际生效（DOM theme=light/dark 坐实）。

## 7. 脱敏边界

- 测试服真实域名、SSH 目标、登录凭据只存在于 gitignored 本地 runner 与环境变量，未写入本文档或任何入库文件；域名占位 <YOUR_TEST_DOMAIN> / <TEST_HOST>。
- 造数所建门户设计为测试服真实 UI 流程产物（名称含 R510 甄别标记），不含凭据；证据目录 test-results/r510-dragging/ 已 gitignored。

## 8. 结论

R510 据实甄别：可达真缺陷 1 项（P2，PortalDesigner 模块重排仅指针拖拽、无单指针/键盘替代）；不可达 / L1 0 类；正常规范预期 3 类（新增有 palette 点击替代、FormDesigner 上移/下移正面对照、检查器配置控件 AX 名称正确）。两轮真浏览器共 6 组 0 pageerror，反预判链完整，不泛化全库 draggable 元素。