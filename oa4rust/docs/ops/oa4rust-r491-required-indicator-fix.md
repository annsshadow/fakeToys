# R491 ThreeMemberApp 姓名必填语义修复（WCAG 2.1 AA 3.3.2）

## 1. 概况

- 轮次：R491（台账 §486 据实立项），真父提交 `854062243b4316e3f57c21fee435f2533974ff35`（R490 甄别 commit）；工作基于当前 HEAD `d7eb50bee9fe90ea1a40927fea50d17c115a0d59`，ThreeMemberApp.vue blob 与真父一致（均 `7f2a6fbd1d8130e3d47e695b17cd7d85c590dbb1`），已 git rev-parse 核实。
- 缺陷：R490 甄别（文档 `oa4rust-r490-instructions-audit.md` 86 行）判定 1 项 P2——ThreeMemberApp 新建成员弹窗姓名字段事实必填（:46 disabled=!form.name?.trim() 坐实），但无 required/aria-required、无可见必填标识（星号或必填文字），保存按钮旁无说明，CDP AXTree required=false 双主题实证；读屏与视觉用户均无法在提交前预知必填，违反 WCAG 3.3.2。
- 修复：姓名 label 加可见星号 `<span class="required" aria-hidden="true">*</span>`（满足视觉用户）+ 姓名 input 加原生 `required`（满足读屏 AX required=true）+ scoped 样式补 `.required{color:var(--color-error)}`。范式参照 XformRuntime.vue:12-14/20 既有实现。
- 测试服：受控测试环境（域名 `<YOUR_TEST_DOMAIN>`、主机 `<TEST_HOST>` 占位）；凭据运行时从受控凭据源提取注入，真值不入库。

## 2. 根因与修复方案

- ThreeMemberApp.vue:39 原为 `<div class="form-group"><label>姓名</label><input v-model="form.name" placeholder="姓名" class="form-input" /></div>`；:46 保存按钮 `:disabled="!form.name?.trim()||saving"` 坐实姓名事实上必填。XformRuntime.vue:13 已有 `<span v-if="module.required" class="required" aria-hidden="true">*</span>` + :20 `:required` 规范范式，ThreeMemberApp 未采用。
- 硬纪律：仅改姓名字段 1 行模板（加星号 span + required 属性）+ 新增 1 行 .required 样式；不动 :46 disabled 业务校验、手机/邮箱/单位/状态四个可选字段、v-model/placeholder/class/data-testid/按钮/弹窗行为；不改 XformRuntime.vue/formErrorA11y.ts/main.ts 全局设施。
- aria-hidden=true 星号：按 XformRuntime 既有范式，星号仅为视觉标识，必填语义由原生 required 提供 AX 播报，避免星号被读屏重复念出。

## 3. 契约 RED→GREEN

- 契约：`oa4rust/frontend/apps/desktop/src/views/R491.requiredIndicator.contract.test.ts`（源码字符串断言，readFileSync + nameGroup 片段提取，5 it）。
- 覆盖：①姓名 label 带 `span.required aria-hidden=true` 且 input 带 required（核心）②保存按钮 disabled 校验零改 ③其它四字段 label/v-model 保留且不加 required ④姓名 input 的 v-model/placeholder/class 与 data-testid 零改 ⑤ .required 颜色样式存在。
- 真父 85406224 定点 RED：Tests 2 failed | 3 passed（5），失败正是核心 required+星号断言与 .required 样式断言，3 项业务守卫父版全通过，非桩错；修复版 GREEN：Tests 5 passed（5）。

## 4. 三 verify（全 exit 0）

- V1 定点契约：`cd oa4rust && pnpm exec vitest run frontend/apps/desktop/src/views/R491.requiredIndicator.contract.test.ts` → 1 passed (5 tests)，exit 0。
- V2 全量测试：`cd oa4rust && pnpm test` → 326 files / 3178 tests passed，0 failed，exit 0。
- V3 类型检查：`cd oa4rust && pnpm typecheck` → 6 项目全 Done（含 frontend/apps/desktop），exit 0。
- 构建：`cd oa4rust && pnpm run build` exit 0，入口 `index-DCZNeE2p.js` / CSS `index-COmAqEyf.css`。

## 5. 部署与真浏览器 E2E

- 部署脚本 `oa4rust/test-results/deploy-pc-r491.sh`（gitignored）：本地 tar -czf → scp → 同一归档 sha256 比对（`SHA_OK=12980b33f85ddd089d7d2bd0f66888ab027f9371a563339df700e6b8b797a41c`）→ 远端解压 web.incoming-r491 → grep 入口 `index-DCZNeE2p.js` → 原子替换 → `DEPLOY_R491_PC_FULL_OK`。
- E2E `oa4rust/test-results/r491-required-indicator-check.cjs` + runner（gitignored），Playwright Chromium headless 非物理真机 PC 1440×900 DPR1，light/dark 双主题。真实 UI 流程：登录 → 进入 admin/three-member → 点「+ 新建」打开弹窗 → DOM 取 label 星号/required 属性 + CDP Accessibility.getPartialAXTree 取 axRequired → 截图。
- 终轮 `R491_DONE checks=14 passed=14 failed=[] shots=2 errors=0`，0 pageerror。
- AXTree 实证（双主题一致，路由 `/oa4rust/app/admin/three-member/`）：姓名字段 labelText=姓名*，星号 `starFound=true/starText=*/starAriaHidden=true/starDisplay=inline`，星号颜色 light rgb(153,27,27)、dark rgb(255,128,128)（var(--color-error) 主题解析值）；姓名 input `hasRequired=true/requiredProp=true/AX axRole=textbox/axRequired=true/axIgnored=false`。
- 对照零回归：手机/邮箱/所属单位/状态四字段 starFound=false、hasRequired=false、AX axRequired=false（状态为 combobox 无 required），保持可选语义；保存按钮 :46 disabled 校验与弹窗行为未动。
- 2 图 read_image 目视（1440×900）：admin 三方成员管理页新建弹窗，姓名标签带红色星号、其它字段无星号、五字段与按钮布局完整，light/dark 双主题，非物理真机，名实相符。

## 6. 证据与 sha256（命令行 sha256sum 原始值）

- pc-three-member-required-light-r491.png ff377c1e770121939371b2d4cf4688ba39b671a68e85aebfa41f9274421af74a
- pc-three-member-required-dark-r491.png 6e808a0048b32d3d507742d4ac574ea6f1d6e7bf169ac357b4aeb7f535d2d0b0
- report.json 55986d6ff41b22acd97b500d02a9a5a9d45837685cfdc723d9f344da3203514f
- 证据目录（gitignored）：`oa4rust/test-results/r491-required-indicator/`（2 PNG + report.json + 脚本）。

## 7. 真值纪律

- 测试域名/IP/口令/SSH 端口/个人路径等真值一律不入库，域名用 `<YOUR_TEST_DOMAIN>`、主机用 `<TEST_HOST>` 占位。
- 凭据在 runner 运行时从受控凭据源提取后注入进程环境，真值仅存在于 gitignored 脚本，不回显、不入库。
- 源码 diff 仅模板 1 行改写（加星号 span + required）+ 1 行样式新增；:46 业务校验/其它四字段/v-model/placeholder/class/data-testid/按钮/弹窗/全局设施零改。
