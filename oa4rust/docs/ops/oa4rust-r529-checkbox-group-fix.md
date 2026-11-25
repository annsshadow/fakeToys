# R529 修复：FormDesigner 预览多选组分组语义（WCAG 2.1 AA / 1.3.1）

- 轮次：R529（修复），对应甄别 R528 判定的 1 项可达 P2。
- 真父：`3c065730a0a91c40b62400a468920d97c0fa0d6d`（台账 §520）；其父 `e1ef44ff428d1069b493f9ddfdaf353a86722fea`。
- 写域：精确 3 文件（1 改 + 2 增），业务零改。
- 工作区：`D:/WORKSPACE/fakeToys`（文档内所有主机/域名均以占位符表示，0 真值）。

## 1. 缺陷与判定

FormDesigner 预览态 `FormDesigner.vue:149-153` 的多选组容器 `.pv-cb-group` 仅有 class，无 `role=group` / `aria-label` / `aria-labelledby`；字段标题裸 label（如"多选"）未与组程序化关联。R528 以 CDP `Accessibility.getPartialAXTree` + `getFullAXTree` 双通道坐实容器 `role=none` / `ignored=true` / name 空，浏览器未推断分组语义。违反 WCAG 2.1 SC 1.3.1 Info and Relationships（Level A）：辅助技术无法获知三个复选框归属于"多选"这一组。

正面对照：`XformRuntime.vue:38` 多选容器已用 `role="group"`；R225 契约 `XformRuntime.requiredA11y.contract.test.ts` 覆盖必填语义。

## 2. 修复（仅模板标记，业务零改）

仅两行模板标记改动，diff 逐字如下：

1. `FormDesigner.vue:128` 字段标题 label 用 v-for 索引 i 生成唯一 id：
   `<label v-if="!['row_start','row_end'].includes(field.type)" :id="'pv-field-label-' + i">`
2. `FormDesigner.vue:149` 多选组容器声明分组角色与关联：
   `<div v-else-if="field.type==='checkbox_group'" class="pv-cb-group" role="group" :aria-labelledby="'pv-field-label-' + i">`

`row_start` / `row_end` 分支的 v-if 不渲染该 label，不受 :id 影响。未改 v-model / class / option 列表 / parseOptions / 预览开关 togglePreview / @submit / 按钮 / 样式 / 业务函数 / 他页；三个复选框自身 name A/B/C（4.1.2 已通过）与键盘 Tab（2.1.1 已通过）逐字未动。相对 `XformRuntime.vue:38` 采用 aria-labelledby（指向真实标题节点）而非硬编码 aria-label，避免标题文案重复。

## 3. 契约 RED → GREEN

契约 `R529.checkboxGroupA11y.contract.test.ts` 共 11 用例：3 核心断言（唯一 id / role=group / aria-labelledby 指向 id）+ 8 守卫断言（option 列表、复选框 value/disabled/class、row_start 分支、必填星号、预览开关、@submit 与提交按钮、v-for 索引、parseOptions 函数）。

- 真父（`e1ef44ff4` 版本 FormDesigner.vue，当前工作树即该版本）RED：`Test Files 1 failed (1)；Tests 3 failed | 8 passed (11)`，退出码 1。失败为 3 条核心断言，8 条守卫全 pass（证明业务零改基线成立）。
- 修复后 GREEN：`Test Files 1 passed (1)；Tests 11 passed (11)`，退出码 0。
- RED 日志留档于 gitignored `oa4rust/test-results/r529-red.log`。
- 说明：首轮 RED 曾因契约源码中转义字符导致 PARSE_ERROR（作者脚本 bug，非产品缺陷），已重写契约（regex 字面量 + 单引号 needle）后重跑，取得上述真实 RED。

## 4. 三 verify（全 exit 0）

- 定点：`pnpm exec vitest run src/views/R529.checkboxGroupA11y.contract.test.ts` → 11 passed，exit 0。
- 全量：`pnpm test` → Test Files 336 passed (336)、Tests 3238 passed (3238)，exit 0。
- 类型：`pnpm typecheck` → 6 项目（desktop / apis / ui / sdk / locales / mobile）全 Done，exit 0。

上述三命令在 workspace 根 `oa4rust/` 下执行。任务 contract 中字面给出的 `oa4rust/frontend/apps/desktop && pnpm test` 因该子包无 test script、`oa4rust/frontend && pnpm typecheck` 因该目录无 package.json 而不可直接运行；据实改为在根 workspace 执行等价命令并取得全绿（Fail Loud：如实披露路径差异，未隐瞒跳过）。全量测试含 `XformRuntime.requiredA11y.contract.test.ts`，全绿即证明 XformRuntime 正面对照零回归；源码 diff 仅 FormDesigner.vue 一处，XformRuntime.vue 未触碰。

## 5. 真浏览器 CDP 双主题实证

本地 Vite dev server（`127.0.0.1:5199`，加载含修复的工作区源码）+ Playwright chromium headless，`1440x900`、`deviceScaleFactor=1`，light/dark 双主题；API 以 stub 信封注入管理员会话（占位账号，0 真值），无真实写库。流程：`/app/admin/form-designer` → 新建 → 选"多选"字段 → 预览 → CDP DOM + AXTree 采样。

结果（双主题完全一致，`fatals=0`、`pageErrors=[]`）：

- DOM：容器 `role="group"`、`aria-labelledby="pv-field-label-0"`、label `id="pv-field-label-0"`（精确互指），字段标题 "多选"，复选框值 A/B/C。
- AX `getPartialAXTree`：容器 `role=group`、`name="多选"`、`ignored=false`。
- AX `getFullAXTree`：容器 `role=group`、`name="多选"`、`ignored=false`（与 partial 双通道一致）。
- 三复选框 AX name 依次为 A / B / C；键盘从第一个复选框 Tab 可达第二个（reachedSecond=true）。
- 注入式 `role="group"` + `aria-label` 对照节点 AX `role=group`，证明 CDP 通道正常、非假阴性。
- 两主题截图目视：预览模态框"表单预览 / 多选 / A、B、C / 提交按钮"正常渲染，B 处于键盘焦点，主题切换无异常。

报告：`oa4rust/test-results/r529-checkbox-group-fix/r529-ax-report.json`（base 记为 `<TEST_HOST>` 占位）。

## 6. SHA-256 证据（脱敏，0 真值）

| 文件 | SHA-256 |
| --- | --- |
| r529-ax-report.json | `12db82d485bcc3a4375bc45685683ca30d32982aec29622175dd33f515ffe63f` |
| FormDesigner.vue（修复后） | `7bb613705e5ccb0f59c043f0c7b0721b2554b0ca22690015f02a1c1a111e5efc` |
| R529.checkboxGroupA11y.contract.test.ts | `bf38309cba43cab738f0d1d793742a92a45721bdc098cf90ba2c7dbd14ceb461` |
| r529-preview-light.png | `173c9331bd00fa40fc584429055037785a22d801e868a4fe9ef80129e1c48e6b` |
| r529-preview-dark.png | `15b8ed643cad03958e76e648533121301a68b6f0ab149bc8fecedef2300a0926` |

runner 脚本、日志、报告与截图均位于 gitignored `oa4rust/test-results/r529-*/`（.gitignore 第 28 行 `oa4rust/test-results/`），不入库。

## 7. 脱敏与 Fail Loud

文档与入库 3 文件仅含占位符（`<TEST_HOST>` / `<YOUR_TEST_DOMAIN>` / stub 账号），无真实域名、IP、主机名、口令或个人路径；截图为本地 Vite stub 环境，无真值。已如实披露：①契约首轮 PARSE_ERROR 为作者 bug 已修正；②contract 字面 verify 路径不可直接运行，改用 workspace 根等价命令全绿。提交纪律：精确 git add 仅 3 文件，禁 -A / .，单提交 push；push 失败 FAIL-LOUD 交队长补推。
