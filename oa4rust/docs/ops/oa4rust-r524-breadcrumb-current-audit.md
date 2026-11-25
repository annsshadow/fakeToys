# R524 面包屑当前项 aria-current 状态属性只读甄别

- 类型：只读甄别（不改码、不提交源码）
- 目标：WCAG 2.1 SC 2.4.8 Location (AAA) / SC 4.1.2 State Properties (A) —— 文件内面包屑当前层级项
- 真父：08770e230d57cfb81c0bf3401cb3ed889ccdccda（台账 §515）
- 测试域：<TEST_HOST>（占位；运行时凭据经远端注入，真值不入库）
- 视口：PC 1440×900、DPR 1、light/dark 双主题、headless Chromium
- 探针：`oa4rust/test-results/r524-e2e.cjs` + `oa4rust/test-results/r524-run.sh`（均被 .gitignore:28 忽略）

## 一、结论（三分判定）

`可达真缺陷：1 个。`

文件管理页「我的文件」区内面包屑（`div.breadcrumb.glass-card`）的当前层级项 `span.bc-item.active` 仅通过 CSS class 视觉标注，未设置 `aria-current`，CDP 确认二段面包屑在 light/dark 两主题下末段 `aria-current=null`、AX 语义通道无 current 属性，辅助技术无法编程获知用户当前所处的文件夹层级。归属 SC 2.4.8 Location (AAA)；aria-current 作为状态属性缺失，关联 SC 4.1.2 State Properties (A)。属构建产物真实 DOM 可达，非不可达 L1，也非正常既有约定。

## 二、源码逐行甄别

- `oa4rust/frontend/apps/desktop/src/views/FileManager.vue`（SHA-256 `832fc7d5f22637d383325d9bfafd189f22b7d68d6f384e486111bbe2bf38020b`）
  - `:30` `<div class="breadcrumb glass-card">` —— 文件内面包屑容器（无 role、无 aria-label）。
  - `:31` `<span v-for="(seg,i) in breadcrumbs" class="bc-seg">`
  - `:32` `<span class="bc-item" :class="{ active: i === breadcrumbs.length - 1 }" @click="navigateTo(i)">{{ seg }}</span>` —— 唯一断言点：当前项仅 `:class` 视觉加 `active`，无 `:aria-current`，且 `@click` 挂在 span 上（键盘可达缺失属另一缺陷族，本轮不混报）。
  - `:204` `const breadcrumbs = ref<string[]>(['根目录'])`；`:820-825` `openItem` 在进入文件夹时 `breadcrumbs.value = [...breadcrumbs.value, f.name]` 追加层级；`:828-832` `navigateTo` 截断层级。面包屑确为 ≥2 段的层级导航。
- 正面对照（仓库既有约定）：
  - `oa4rust/frontend/apps/desktop/src/components/admin/AdminLayout.vue`（SHA `d0167e0da874710bca3a2c72f8cd9e3065c0b16b6a0047e95212e648db9398cf`）`:22` `:aria-current="isActive(item.path) ? 'page' : undefined"`,契约 `AdminLayout.contract.test.ts:22-28` 钉住该写法且禁止静态写死。
  - `oa4rust/frontend/packages/ui/src/components/AppShell.vue`（SHA `4244a7b3c121515d9c50f9dc077e9ab8531d33a656caa1931507ff62c5597d6b`）`:78` `<span class="breadcrumb-current" :aria-current="isAdminArea ? undefined : 'page'">{{ currentTitle }}</span>` —— 顶栏面包屑当前项已有 aria-current 正确做法，唯独文件内层级面包屑遗漏。

## 三、契约与判据

- SC 2.4.8 Location (AAA)：导航层级中的当前位置必须能被用户代理确定并编程式呈现给辅助技术。
- SC 4.1.2 (A)：组件的状态属性必须可被编程化确定；`aria-current` 为 WAI-ARIA 当前项状态。
- 判据链：① DOM 属性层 `aria-current` 有无；② AX 语义层 current 属性/role；③ 正面对照同仓 `aria-current=page` 的可读性。三项均须真浏览器 CDP 实证，静态线索不预判。

## 四、决定性 CDP 证据矩阵（真浏览器，双主题一致）

探针以 DOM 克隆真实 `span.bc-item` 构造二段面包屑「根目录 › 子目录R524当前项」（生产构建未暴露 Vue setupState，故按与 `:32` 完全一致的 DOM 结构构造末段 active 项），再走 CDP `DOM.getAttributes` + `Accessibility.getPartialAXTree` + `Accessibility.getFullAXTree`。

| 观测项 | light | dark | 判定意义 |
|---|---|---|---|
| 二段面包屑构造 | `constructed=true, segCount=2` | 同左 | 前提成立，≥2 段层级有效 |
| 末段 `span.bc-item.active` `aria-current` | `null`（属性不存在） | `null` | 决定性：当前层级无状态属性 |
| 末段 AX `role` / `name` / `properties` | `generic` / `""` / `[]` | 同左 | AX 语义无 current、无可访问名 |
| 首段 `span.bc-item` | `aria-current=null`, `role=none` | 同左 | 非当前项亦无状态，符合预期 |
| 页面内 `[aria-current]` 清单 | 仅侧栏 `nav-item.active`(`page`) 与顶栏 `breadcrumb-current`(`page`) | 同左 | 文件内面包屑不在清单内 |
| 正面对照 admin 当前项 | DOM `aria-current="page"`, AX `role=link`, name=控制面板 | 同左 | 同仓 `aria-current=page` 确可被 AT 读取 |

说明：CDP `getFullAXTree` 返回的 `properties` 在 Chromium 中不携带 `current` 属性（admin 当前项该计数亦为 0），故 `axCurrentCount=0` 不用于区分两组；区分的决定性证据是 `DOM.getAttributes` 的 `aria-current` 属性有无（admin 当前项 `"page"`，文件内末段 `null`）与 partial AX 语义。据实记录该 API 限制，不夸大 AX 计数。

## 五、去伪与层级归属

- 去伪 1：`span@click` 无键盘可达属 WCAG 2.1.1 Keyboard 缺陷族，本轮只锚定 aria-current 唯一断言，不混报。
- 去伪 2：`admin-breadcrumb`、`ec-breadcrumb` 路径文本及顶栏 `breadcrumb-current` 均带页面级 `aria-current=page`,排除；它们表达的是「文件管理页」页面级位置，不表达「子目录R524」文件夹层级位置，不能据此辩称满足 SC 2.4.8。
- 去伪 3：单段「根目录」时也有 `active` 类但无 aria-current；二段构造后当前项仍无 aria-current，非单段特例。
- 层级归属：主判 SC 2.4.8 Location (AAA)（专指当前位置对 AT 的可编程可读性）；aria-current 作为状态属性缺失关联 SC 4.1.2 (A)。缺陷可达性：真实 DOM 在登录态文件管理页构造层级后稳定复现，非不可达 L1。

## 六、截图与证据哈希

- 截图：`oa4rust/test-results/r524-breadcrumb-current/breadcrumb-light.png` SHA-256 `4e99c4040fa5c7fe77d97a6c7f317a8d67767baf3e50c1a0a5fd9750ad54f98a`（1440×900，127321 B）；`breadcrumb-dark.png` SHA-256 `de2872ba99f312bb420d0d4afaf60ed3cb023c2a32f7e081f1026c1a342be223`（1440×900，129666 B）。两图均可见二段面包屑「根目录›子目录R524当前项」及末段视觉高亮。
- 报告：`oa4rust/test-results/r524-breadcrumb-current/report.json` SHA-256 `a348b19f8da66f978436f92a2e75c5106632633d82d2d2fba07a3732ea322ebe`（2 组，fatals=0）。
- 证据脚本/截图/报告均为 gitignored（`.gitignore:28 oa4rust/test-results/`），不入库；本文仅引用哈希与相对路径。

## 七、覆盖甄别与取证限制

- 覆盖：FileManager.vue 文件内面包屑当前项 aria-current；AdminLayout.vue 侧栏与 AppShell.vue 顶栏正面对照；light/dark 双主题。
- 限制：生产构建下 Vue `setupState` 不可从 DOM 访问，二段面包屑以同构 DOM 构造验证渲染后语义，未改源码；`getFullAXTree.properties` 不含 current，故以 DOM 属性 + partial AX 为决定性依据。
- 0 缺陷判定不适用：本轮坐实 1 个可达缺陷，据实判定，不臆造、不漏报。
- 脱敏：全文域名用 <TEST_HOST> 占位；无密钥、口令、真域名、IP、个人路径等真值。
