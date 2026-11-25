# R530 程序中心详情抽屉焦点遮挡（WCAG 2.4.11）只读甄别

- 任务：R530（台账 §521，真父 `86e8bde8f918ed46539a439fe4079fb574dc86a6`）
- 性质：只读甄别，不改源码、不提交源码
- 环境：本地 Vite dev server + Playwright chromium headless，1440×900、DPR 1、light/dark 双主题
- 测试地址：`<TEST_HOST>`（占位；运行时使用本地回环，真值不入文档）
- 判定：**可达真缺陷 1 项** — WCAG 2.2 SC 2.4.11 Focus Not Obscured (Minimum)（AA）

## 一、结论

程序中心详情抽屉打开后不移动焦点、也不建立焦点陷阱：键盘用户按 Tab 时，焦点依次落在抽屉后方被完全覆盖（overlap=1，fullyCovered=true）的「禁用/执行/编辑/删除」操作按钮上，焦点不可见，违反 SC 2.4.11（AA）。抽屉本身具备 role=dialog 与可读 aria-label，可访问性名称维度（SC 4.1.2）通过，不重复计缺陷。

## 二、源码逐行锚点

| 文件 | 行 | 事实 |
|---|---|---|
| `frontend/apps/desktop/src/views/ProgramCenterApp.vue` | 41 | `<button class="detail-link" @click="openDetail(...)">` 详情入口，同族见 62/79/101 |
| 同 | 384–390 | `<div v-if="selectedDetail" class="detail-drawer" role="dialog" :aria-label="...">`，含 387 关闭按钮 |
| 同 | 423–425 | `selectedDetail` ref，`openDetail` 仅赋值，**不移焦、无焦点陷阱** |
| 同 | 441/497–510/962 | tab 默认 agent；`loadAgents` api.get agent 列表；`watch(tab, ..., {immediate:true})` 触发 → 非死代码 |
| 同 | 2519 | `.detail-drawer { position:fixed; right:0; width:min(460px,90vw); z-index:120 }` 覆盖右侧表格操作列 |
| `frontend/apps/desktop/src/utils/modalFocus.ts` | 10 | `OVERLAY_SELECTOR` = `.modal-overlay,.upload-overlay,.paging-overlay,.modal-mask,.preview-overlay,...`，**不含 `.detail-drawer`** |
| 同 | 93/121/143–159/174–204 | `installViewModalManager` 注入 role=dialog / aria-modal=true / aria-labelledby / 移焦 / Tab 陷阱 / 关闭焦点回归 —— 框架已有机制但抽屉未接入 |
| `frontend/apps/desktop/src/router/main.ts` | 170–173 | path:program → ProgramCenterApp（admin 守卫） |

## 三、判定依据（CDP/DOM/键盘实证，light/dark 双主题一致）

- DOM：详情链接 linkCount=2；首个链接 AX `role=button`、`name=R530智能体甲`、`focusable=true`。
- 聚焦详情链接后按 Enter 开抽屉：`activeElement` 仍是该链接（drawer `role=dialog`、`aria-label=Agent·R530智能体甲详情`、`ariaModal=null`、`z-index=120`，rect x959→1419 / y165→753）；焦点链接 rect x563→977.656，仅右端 18.656px 与抽屉重叠（coverage=0.045），**触发元素未被完全遮挡**。
- 抽屉开启后连续 Tab 遍历：第 1–4 次落到同行「禁用/执行/编辑/删除」按钮，第 5 次为第 2 行详情链接（coverage=0.045），第 6–8 次为第 2 行操作按钮，`inDrawer=false`、`overlap=1`、`fullyCovered=true`——焦点 100% 被 z-index 120 抽屉覆盖、完全不可见；第 9 次才进入抽屉内「关闭」按钮。
- 关闭抽屉后：`drawerCount=0`，但焦点落至 BODY（`bodyFocus=true`、`isDetailLink=false`）——开抽屉未移焦、关抽屉未回归，属 SC 2.4.3 焦点顺序辅助证据，**不另计缺陷**。
- 正面对照：同页注入 `.modal-overlay` 后，`modalFocus.ts` 自动注入 `role=dialog`、`aria-modal=true`、`aria-labelledby`，焦点入内（`marker=true`、`focusInside=true`），AX `modal=true`；证明框架焦点约束机制可用，仅 `.detail-drawer` 未纳入 `OVERLAY_SELECTOR`。

## 四、证据清单（sha256）

| 证据 | sha256 | 备注 |
|---|---|---|
| `oa4rust/test-results/r530/r530-focus-report.json` | `5c142009c70cb06ed2bbc5ab8b86dcdcda27260b975a1f4f4dbfb5fc4ff6af0e` | 双主题主探针报告 |
| `oa4rust/test-results/r530/r530-tab-report.json` | `a590e2204916f9dc19a9b7353ae86e838aac5ffa80b9ab66ff683611b65ac539` | 双主题 Tab 遍历遮挡报告 |
| `oa4rust/test-results/r530/r530-detail-light.png` | `3b719a984927e8fea86b44443aeb0b76dbb54291412f411676aef3676f399f7b` | 1440×900，155902 B |
| `oa4rust/test-results/r530/r530-detail-dark.png` | `a393d933b506b245ac0f99e3d4e36109520b07b62dd71c149bc6827deccd7f9f` | 1440×900，178335 B |
| `oa4rust/test-results/r530/r530-tab-light.png` | `b91118e2d9b6d644f19683813b63b54aa13d79c5a188087175862a1648e610bd` | 1440×900，156182 B |
| `oa4rust/test-results/r530/r530-tab-dark.png` | `f970022320d4ddee8eb3325192c9154781deda34a82b7b78b0a7784a5a3d4e2a` | 1440×900，178622 B |

截图目视：light/dark 均见右侧抽屉覆盖表格操作列，后方「禁用/执行/编辑/删除」按钮不可见，与几何 overlap=1 一致。

## 五、去伪与覆盖甄别

- 死代码排除：列表 2 条记录为 stub 真实可达；`watch(tab, ..., immediate)` + `loadAgents` 坐实入口可达，非 R525 式死代码。
- 排除误报：图标 title-only（R477 已覆盖）、1.4.13 两 tooltip 死代码、1.3.1 多表头 0 命中、3.2.6 无帮助、1.4.12 过泛 —— 均不并入本报告。
- 4.1.2 维度：抽屉 role=dialog / aria-label 可读、详情链接 role/name/focusable 齐备 —— 通过，不重复计数。
- 2.4.3 维度：关闭后焦点落 BODY 属焦点顺序问题，作辅助证据不另计缺陷。
- 主判唯一缺陷：SC 2.4.11（AA）焦点被 z-index 120 抽屉完全遮挡；触发元素仅 4.5% 边缘重叠不作为「完全遮挡」主判依据，决定性证据是 Tab 后方的操作按钮 100% 覆盖。

## 六、修复建议（不在本次改动范围）

将 `.detail-drawer` 纳入 `modalFocus.ts` 的 `OVERLAY_SELECTOR`，使 `installViewModalManager` 为其注入 aria-modal、移焦、Tab 陷阱与关闭焦点回归；或在 `openDetail` 中显式移焦至抽屉内关闭按钮并在关闭时回归触发链接。

## 七、脱敏自检

文档仅含占位域名 `<TEST_HOST>`；无真实域名、IP、SSH 端口、口令、个人目录或环境路径。证据探针用本地回环与占位 stub 数据，真值均不入仓。