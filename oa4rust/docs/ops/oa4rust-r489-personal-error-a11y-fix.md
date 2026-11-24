# R489 Personal 修改密码错误态无障碍语义修复（WCAG 3.3.1）

## 1. 概况

- 轮次：R489（台账 §484 据实立项），真父提交 `a3a485e268a25f387a93df87eefe6a139193f245`（R488 甄别 commit）；当前 HEAD 为台账 commit `a1ffdbfc`，Personal.vue blob 与真父一致（均 `05df49ab`），已 git rev-parse 核实。
- 缺陷：R488 甄别（文档 `oa4rust-r488-error-identification-audit.md` 138 行）判定 1 项 P2——Personal 修改密码错误态纯视觉：`.error-msg` 错误文本 `role=generic`/axName 空、无 aria-live，三密码框 aria-invalid/describedby 全空，读屏用户无法感知校验错误（请填写完整密码 / 两次密码不一致双主题 4 组实证）。
- 修复：在错误 div 上新增 `data-oa-field-error` 标记，纳入已有全局增强器 `installFormErrorA11y`（`main.ts`:10 导入、:614 调用；`formErrorA11y.ts`:83 扫描标记、:86 补 role=alert、:113-114 补控件 aria-invalid/aria-describedby、:98-101 错误移除时 restore）。
- 测试服：受控测试环境（域名 `<YOUR_TEST_DOMAIN>`、主机 `<TEST_HOST>` 占位）；凭据运行时从受控凭据源提取注入，真值不入库。

## 2. 根因与修复方案

- Personal.vue:50 原为 `<div v-if="pwdError" class="error-msg">{{ pwdError }}</div>`，纯赋值（:155 / :159 / :141 / :149 / :172 / :180 各分支）无任何语义角色，增强器只覆盖带 `data-oa-field-error` 标记的文本（全库唯一真实消费在 XformRuntime.vue:61），Personal 未纳入。
- 修复选择方案 B（复用全局增强器）：错误 div 加 `data-oa-field-error`，由增强器自动补 role=alert + 稳定 id（`oa-field-error-NNNNNN`）+ 从错误文本向上找表单控件容器并补 aria-invalid=true / aria-describedby；错误解除自动 restore。相比最小方案 A 仅加 role=alert，方案 B 同时解决三密码框的 aria-invalid/describedby 关联，覆盖验收全部维度。
- 硬纪律：仅新增 1 个属性标记；`savePassword` 业务校验逻辑与全部文案、三密码框 autocomplete（current-password/new-password/new-password）、按钮、样式、data-testid、class、v-model 零改；不改 formErrorA11y.ts / main.ts 全局设施。

## 3. 契约 RED→GREEN

- 契约：`oa4rust/frontend/apps/desktop/src/views/R489.personalErrorA11y.contract.test.ts`（源码字符串断言，readFileSync + `errorDiv` 片段提取）。
- 5 it：①错误 div 带 data-oa-field-error（核心）②错误 div 的 v-if/class/插值文本零改 ③ savePassword 校验文案与 submit 绑定零改 ④ 三密码框 autocomplete/v-model/data-testid 零改 ⑤ 保存按钮与 error-msg 样式零改。
- 真父 a3a485e2 定点 RED：Tests 1 failed | 4 passed（5），失败正为核心标记断言，4 项业务守卫全通过，非桩错；修复版 GREEN：Tests 5 passed（5）。

## 4. 三 verify（全 exit 0）

- V1 定点契约：`cd oa4rust && pnpm exec vitest run frontend/apps/desktop/src/views/R489.personalErrorA11y.contract.test.ts` → 1 passed (5 tests)，exit 0。
- V2 全量测试：`cd oa4rust && pnpm test` → 325 files / 3173 tests passed，0 failed，exit 0。
- V3 类型检查：`cd oa4rust && pnpm typecheck` → 6 项目全 Done（含 frontend/apps/desktop），exit 0。
- 构建：`cd oa4rust && pnpm run build` exit 0，入口 `index-CT7ZSrpR.js` / CSS `index-COmAqEyf.css`。

## 5. 部署与真浏览器 E2E

- 部署脚本 `oa4rust/test-results/deploy-pc-r489.sh`（gitignored）：本地 tar -czf → scp → 同一归档 sha256 比对（`SHA_OK=ff1477b0b46dfa15ffecfa7f985ca073a963b7d18760b0323a89dd8356976b96`）→ 远端解压 web.incoming-r489 → grep 入口 → 原子替换 → `DEPLOY_R489_PC_FULL_OK`。
- E2E `oa4rust/test-results/r489-personal-error-a11y-check.cjs` + runner（gitignored），Playwright Chromium headless 非物理真机 PC 1440×900 DPR1，light/dark 双主题。真实 UI 流程：登录 → 进入个人中心 → 点击「保存修改」（空表单触发校验错误）→ CDP DOM + Accessibility.getPartialAXTree 采集 → 截图 → 重新进入个人中心验证复位 → dashboard 对照。
- 终轮 `R489_DONE checks=22 passed=22 failed=[] shots=2 errors=0`，0 pageerror。
- AXTree 实证（双主题一致）：错误文本节点 `axRole=alert`、axIgnored=false，子树 StaticText 含「请填写完整密码」可播报（role=alert 可访问名按规范为空，内容经子树文本节点传达）；增强器补稳定 id `oa-field-error-000001`；三密码框 `axRole=textbox`、axIgnored=false、axInvalid=true，DOM aria-invalid=true、aria-describedby 指向该 id。
- 复位口径（据实声明）：错误态经重新导航回个人中心后验证错误文本消失、三密码框 aria-invalid/aria-describedby 复位为空、基线无错误态不被增强器施加属性；动态 restore 逻辑由 formErrorA11y.ts restore()/:98-101 源码保证，本 E2E 以导航后基线态作为复位代理，未在真实 UI 中触发成功改密以免修改测试账号密码。
- 业务零改实证：三密码框 autocomplete 全程保持 current-password / new-password / new-password；dashboard 对照页按钮可达，0 pageerror；2 图 read_image 目视真 OA4RUST admin 个人中心页 1440×900，light/dark 表单、错误红字、按钮布局完整，非物理真机。

## 6. 证据与 sha256（命令行 sha256sum 原始值）

- pc-personal-error-light-r489.png 59f8d4054245ad2bde2f7e81d6364fd3be06b40d2a02e521df7f0e48c77ff746
- pc-personal-error-dark-r489.png 4e8da018138913b68bcd2b9958a0aaeb55809b8fc19efbc6fe5022d916121996
- report.json a636dfd22f3c69ca41223a58096f1bdd3507f924b938001ff6862fb23e7af9c6
- 证据目录（gitignored）：`oa4rust/test-results/r489-personal-error-a11y/`（2 PNG + report.json + 脚本）。

## 7. 真值纪律

- 测试域名/IP/口令/SSH 端口/个人路径等真值一律不入库，域名用 `<YOUR_TEST_DOMAIN>`、主机用 `<TEST_HOST>` 占位。
- 凭据在 runner 运行时从受控凭据源提取后注入进程环境，真值仅存在于 gitignored 脚本，不回显、不入库。
- 源码 diff 仅 1 行新增属性；业务校验逻辑/三密码框/按钮/样式/data-testid/全局设施零改。
