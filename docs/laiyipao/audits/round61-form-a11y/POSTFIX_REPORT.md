# R61 表单输入语义与错误播报 — 修复后真浏览器重查报告（POSTFIX）

- 任务：t113 / attempt `cfd1fce5-7bf7-47b8-9402-971d99c76580`；QA：same-route-qa；日期：2026-11-26
- 工作树：`D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248`，起点 HEAD `eead131e6`；只读业务源码、仅写审计报告、零真值入库。
- 背景：t110（REPORT.md）判定 **FAIL** —— 17 处 `aria-label` 静态契约 4/4 过，但 uni-app H5 把 `aria-label` 编译到外层 `<uni-input>`，原生 `<input class="uni-input-input">` 无名称，真 Chromium AX 树 11 页 textbox 全裸名。t111（规范 RED）+ t112（实施）以 `v-aria-native` 指令在 `mounted/updated` 时把 label 桥接到 native input。本报告复查该修复。

## 0. 结论

**PASS（修复生效）**。真 Chromium 三视口 37 组探针证实：所有可见 native `<input>` 的 DOM 直接带 `aria-label`，AX 树 `textbox`（`verify` 页为 `spinbutton`）逐字带 SPEC §2 名称，空值与有值两态均带名；对比 t110 `data/ancestry-login.json` 证实桥接生效。6 处反馈节点中 5 处真浏览器坐实，第 6 处（me `passwordMsg`）因本会话为游客态（guest_token，服务端未开 `PLAYER_LOGIN_REQUIRED`）无法进入登录态改密表单，按诚实边界披露。R60/R43–R48/R55 与令牌三锚点无回退，零横向溢出；契约 11/11、全量 2025 passed|2 skipped、vue-tsc/build:h5/diff-check 全绿。

## 1. 环境与配方

- API：`docs/laiyipao/audits/round15-reaction-timeline/start-api.sh` 后台启动，端口 8080；`/api/v1/config` = 200，version `6237e0b623219b8b`；根路径 404 属正常。
- H5：在 `laiyipao/miniapp` 下 `pnpm run build:h5`（仓库根无此脚本），job `bash-b20f044d66d34422` RC=0 `DONE Build complete`；bundled python `http.server 5201 --bind 127.0.0.1 --directory laiyipao/miniapp/dist/build/h5` 后台 job `bash-d1867d41bbcb4bfd`，根 = 200。端口 5174 为旧 302 服务，弃用。
- 认证：`users.id=10` 的 guest_token 经 `POST /api/v1/auth/guest` 换取 token，`gen-alias-auth-init.sh` 生成 `data/auth-init.js`（517B），仅会话侧 `--init-script` 注入，不改 DB／服务端。凭据不回显、不入库；`.gitignore:553` 覆盖审计 `data/`。
- QA 库 `laiyipao` / `battle_records` 基线 `count|max = 104|477`（收尾复核不变）。
- 浏览器驱动：`AB(){ agent-browser --session "$1" "${@:2}"; }` 必须转发全部参数。探针在审计目录：`r61-postfix-probe.js`（native aria-label / accName / 反馈 / overflow）、`r61-postfix-fill.js`（写值 + dispatch input/change）、`r61-postfix-ax-driver.sh`（37 组，每组 open→viewport→probe→snapshot --json→fill→probe→snapshot→screenshot）。

## 2. 17 处 input 三视口两态 AX 逐页对账（37 组）

下表来自 `data/analyze-postfix.js .` 重跑输出；`domNoName`=native input 缺 aria-label 数（空／有值两态），`AXBare`=AX 树裸控件数。三视口名称一致，逐视口列出。

| 页面 | 视口 | native input 数 | domNoName 空/值 | AX 名称（空值与有值同） | AXBare 空/值 | hOverflow | 判定 |
|---|---|---|---|---|---|---|---|
| announcements | 390 | 1 | 0 / 0 | 搜索公告标题 / 正文 | 0 / 0 | 0 | PASS |
| announcements | 768 | 1 | 0 / 0 | 搜索公告标题 / 正文 | 0 / 0 | 0 | PASS |
| announcements | 1280 | 1 | 0 / 0 | 搜索公告标题 / 正文 | 0 / 0 | 0 | PASS |
| login | 390 | 2 | 0 / 0 | 用户名 / 口令 | 0 / 0 | 0 | PASS |
| login | 768 | 2 | 0 / 0 | 用户名 / 口令 | 0 / 0 | 0 | PASS |
| login | 1280 | 2 | 0 / 0 | 用户名 / 口令 | 0 / 0 | 0 | PASS |
| verify | 390 | 1 | 0 / 0 | battle_id（spinbutton） | 0 / 0 | 0 | PASS |
| verify | 768 | 1 | 0 / 0 | battle_id（spinbutton） | 0 / 0 | 0 | PASS |
| verify | 1280 | 1 | 0 / 0 | battle_id（spinbutton） | 0 / 0 | 0 | PASS |
| signin | 390 | 1 | 0 / 0 | 输入兑换码 | 0 / 0 | 0 | PASS |
| signin | 768 | 1 | 0 / 0 | 输入兑换码 | 0 / 0 | 0 | PASS |
| signin | 1280 | 1 | 0 / 0 | 输入兑换码 | 0 / 0 | 0 | PASS |
| me（游客态，0 input） | 390 | 0 | 0 / 0 | —（条件渲染，见 768-me-expanded） | 0 / 0 | 0 | PASS |
| me（游客态，0 input） | 768 | 0 | 0 / 0 | —（条件渲染，见 768-me-expanded） | 0 / 0 | 0 | PASS |
| me（游客态，0 input） | 1280 | 0 | 0 / 0 | —（条件渲染，见 768-me-expanded） | 0 / 0 | 0 | PASS |
| bag | 390 | 1 | 0 / 0 | 预设名（最多 20 字） | 0 / 0 | 0 | PASS |
| bag | 768 | 1 | 0 / 0 | 预设名（最多 20 字） | 0 / 0 | 0 | PASS |
| bag | 1280 | 1 | 0 / 0 | 预设名（最多 20 字） | 0 / 0 | 0 | PASS |
| equipment | 390 | 1 | 0 / 0 | 搜索装备名称 / 部位 / 元素 | 0 / 0 | 0 | PASS |
| equipment | 768 | 1 | 0 / 0 | 搜索装备名称 / 部位 / 元素 | 0 / 0 | 0 | PASS |
| equipment | 1280 | 1 | 0 / 0 | 搜索装备名称 / 部位 / 元素 | 0 / 0 | 0 | PASS |
| enemies | 390 | 1 | 0 / 0 | 搜索敌人名称 / 类别（如 Boss、飞行） | 0 / 0 | 0 | PASS |
| enemies | 768 | 1 | 0 / 0 | 搜索敌人名称 / 类别（如 Boss、飞行） | 0 / 0 | 0 | PASS |
| enemies | 1280 | 1 | 0 / 0 | 搜索敌人名称 / 类别（如 Boss、飞行） | 0 / 0 | 0 | PASS |
| gems | 390 | 1 | 0 / 0 | 搜索宝石名称 / 属性 | 0 / 0 | 0 | PASS |
| gems | 768 | 1 | 0 / 0 | 搜索宝石名称 / 属性 | 0 / 0 | 0 | PASS |
| gems | 1280 | 1 | 0 / 0 | 搜索宝石名称 / 属性 | 0 / 0 | 0 | PASS |
| reactions | 390 | 1 | 0 / 0 | 搜索反应名称 | 0 / 0 | 0 | PASS |
| reactions | 768 | 1 | 0 / 0 | 搜索反应名称 | 0 / 0 | 0 | PASS |
| reactions | 1280 | 1 | 0 / 0 | 搜索反应名称 | 0 / 0 | 0 | PASS |
| skills | 390 | 1 | 0 / 0 | 搜索技能名称 / 元素 / 主动被动 | 0 / 0 | 0 | PASS |
| skills | 768 | 1 | 0 / 0 | 搜索技能名称 / 元素 / 主动被动 | 0 / 0 | 0 | PASS |
| skills | 1280 | 1 | 0 / 0 | 搜索技能名称 / 元素 / 主动被动 | 0 / 0 | 0 | PASS |
| skins | 390 | 1 | 0 / 0 | 搜索皮肤名称 / 稀有度 | 0 / 0 | 0 | PASS |
| skins | 768 | 1 | 0 / 0 | 搜索皮肤名称 / 稀有度 | 0 / 0 | 0 | PASS |
| skins | 1280 | 1 | 0 / 0 | 搜索皮肤名称 / 稀有度 | 0 / 0 | 0 | PASS |
| me-expanded（768） | 768 | 2 | 0 / 0 | 用户名 / 密码（至少 10 位） | 0 / 0 | 0 | PASS |

- 37 组全部 `docW == innerW`（hOverflow=0）、`hrefOk=true`、无 chrome-error；native input `nativeAria` 逐字等于 SPEC §2 名称；空态探针值与有态探针（`r61-postfix-fill.js` 写入 `QA填写0号` 等并 dispatch input/change）均带名。
- **假阳性更正**：`analyze-postfix.js` 输出 `FAILURES=["390-verify","768-verify","1280-verify"]`，根因是脚本 `getTB()` 仅匹配 `role="textbox"`；verify 的 `<input type="number">` 在 AX 树为 `spinbutton "battle_id"`（`data/pf-ax-empty-768-verify.json` refs `e21: {name:"battle_id", role:"spinbutton"}`），且 DOM 探针 `nativeAria="battle_id"`。按 spinbutton 口径判 **PASS**，非缺陷。
- me 基础页游客态 0 input（条件渲染，R61 范围外）；以 `768-me-expanded` 点击「设置账号密码」展开表单补证 2 处（用户名 / 密码（至少 10 位））。

## 3. 对照 t110：桥接生效坐实

- t110 `data/ancestry-login.json`：native `<input class="uni-input-input">` outerHTML 仅 `type/maxlength/step/enterkeyhint/class/autocomplete/inputmode`，无 `aria-label`；`aria-label` 落在外层 `<uni-input>`（uni-app 原生属性白名单不含该属性，`@dcloudio/uni-h5.es.js:10856-10886`）。
- 本次 `data/pf-empty-768-login.json`：`inputs[].nativeAria` 分别为 `"用户名"`、`"口令"`，`domAcc.src="aria-label"`。同源同页面从「裸」到「带名」，证明 `main.ts` 的 `v-aria-native` 指令（`applyAriaNative` → `native.setAttribute("aria-label", name)`，mounted/updated 钩子）真正在 H5 运行时生效。
- 实现 diff（队长资产，QA 只读）：`git diff --stat` 14 files / 68+ / 6-（`.gitignore`、`src/main.ts`、12 页面）；`src/main.ts:9-21` 定义 `applyAriaNative`，`:26` `app.directive("aria-native", ariaNative)`；全仓 17 处 `v-aria-native`，零删除。

## 4. 反馈节点播报（6 处：5 坐实 + 1 环境限制披露）

| # | 页面 | 节点 | AX role / live | 文本 | 证据 |
|---|---|---|---|---|---|
| 1 | login | form-hint | status / polite | 请填写用户名与口令后可登录 | `pf-fb-login-err.json` |
| 2 | login | errMsg | alert | 请填写用户名与口令 | `pf-fb-login-err.json` |
| 3 | verify | errMsg | alert | 请输入有效的 battle_id | `pf-fb-verify-err.json` |
| 4 | signin | redeemMsg | status / polite | 输入非法: 兑换码无效或已用尽 | `pf-fb-signin-redeem.json` |
| 5 | bag | presetSaving | status / polite | 保存中… | `pf-fb-bag-mo.json`（MutationObserver 捕获瞬时节点） |
| 6 | me | passwordMsg | status / polite（源码 me.vue:601-610） | 原口令与新口令都不能为空 / 新口令至少 10 位 | 源码确认；真浏览器未捕获（见下） |

- bag `presetSaving`（`bag.vue:415`，`SAVING_PRESET_TEXT="保存中…"` `bag.vue:1263`）在 `doSavePreset` 中 `true → await api.saveCurrentPreset → finally=false`，服务端写入过快，普通轮询 10 次全 `count=0`；改用 `pf-mo-install.js` 在 body subtree 上挂 MutationObserver，触发保存后捕获到 `{role:"status",live:"polite",cls:"dim",text:"保存中…"}`（`data/pf-fb-bag-mo.json`），坐实第 5 节点。
- me `passwordMsg` 属登录态改密表单（`me.vue:563` `v-if="!changingPassword"` 的「修改密码」按钮仅登录态显示）。本 QA 会话为 guest_token 游客态（`me.vue:614-661` 游客分支改为「设置账号密码」/ `upgradeMsg`，该节点源码未标注 role 且不属 R61 六节点）。尝试 `POST /api/v1/auth/login` 使用文档默认凭据返回 `{"code":"unauthorized"}`，服务端需 `PLAYER_LOGIN_REQUIRED` 环境；游客 upgrade 会改写 QA 库用户态故不执行。第 6 节点真浏览器未捕获，如实披露；源码 `me.vue:601-610` 确认 `role="status" aria-live="polite"` 已标注、静态契约覆盖。

## 5. 回归与不变量（无回退）

- **R60 键盘**：契约 `r60A11yKeyboard` 6/6 RC=0；`role=button` `tabindex="0"` + `@keydown` 未被本轮 17 处 input 改动破坏。
- **R43–R48 CTA**：`battles.vue:151` `r20-ai-cta`、index「去配装备 →」、equipment/bag/gems/skills `r12-priority-cta` 保持；本轮未触碰。
- **R55 placeholder**：`placeholder-class="ph"` 在 12 文件 17 处仍在（含本轮 12 页全部 input）。
- **令牌三锚点**：`tokens.css:14-16` `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 逐字在；`--r61-` 仅契约断言 2 处（`r61FormA11y.contract.test.ts:128/:133`），业务源码 0 命中。
- **零溢出**：37 组探针 `docW == innerW`、`overflowCount=0`；AX 快照中 CTA 无越界。

## 6. 门禁（job `bash-295ed57ad251460f` 汇总）

| 门禁 | 结果 | RC |
|---|---|---|
| R61 契约 `r61FormA11y.contract.test.ts`（5 it） | 5 passed | 0 |
| R60 契约 `r60A11yKeyboard.contract.test.ts`（6 it） | 6 passed | 0 |
| `pnpm test` 全量 | 218 files passed\|1 skipped；2025 tests passed\|2 skipped | 0 |
| `vue-tsc --noEmit` | passed | 0 |
| `pnpm run build:h5` | DONE Build complete | 0 |
| `git diff --check` | 0（仅 `.gitignore` LF→CRLF warning） | 0 |
| Go `TestNoInfraIdentifiersInRepo` | ok 8.684s | 0 |
| Go `TestNoRealSecretsInRepo` | **SKIP（非 PASS）**：「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」 | 0 |

## 7. 截图 SHA-256（37 张全互异，亲读非登录页）

- 完整表在 `data/pf-sha256.txt`（`TOTAL=37 UNIQUE=37`）。
- `7cd071fd4efbc259652b3a3ef1ee8fc5b1d047fedee68144e1f9dc06603662a7  pf-1280-announcements.png`
- `f278b28f3e694a568bd4aa2e694ab73d72470a2707f00a00c5ecc01660311eb2  pf-1280-bag.png`
- `5a9092d30407f50292b13b367314471248bddb3124524a4c54593392eecf305e  pf-1280-enemies.png`
- `731891d974567d4dcf7b554abc7f4bb5e734ebf1042a2612c4ab98185cdf053f  pf-1280-equipment.png`
- `30bbfdacacaf0ebeda737a270e6e22fb7eb33e2670e54256fb439c02628bd098  pf-1280-gems.png`
- `35d9475d13d5222163daed6d3f63adaf448c01981deea3ad2bc79ca45b82bc32  pf-1280-login.png`
- `7d258ed3fc8f9c764781d8d156e11e9afae2d079054e5d81f72081da322cfec1  pf-1280-me.png`
- `8d1c062887947c48ad9c5b4769ae017da2bd8d3b83a3241cdf7d7766f79b146a  pf-1280-reactions.png`
- `e36a672a62c6e4ccb493ed9640d6b399e74c3e9d3a08921c91d3c0b4f61d025a  pf-1280-signin.png`
- `abd614143411ce514bc62c74b78c067a1ef754804366326741059aa3e1b71b97  pf-1280-skills.png`
- `6864d2daeb6c4d4871560ebaa804cbef520f18b337012b08ac4834ff1903079e  pf-1280-skins.png`
- `0e91c7cf51e3a348441e725da9df5a160a5c876b05bec7d4b916addc1e36e3bf  pf-1280-verify.png`
- `050097bb8ece1dbab04f2ce99783a28b56acda6d729711e457d2a4f39010e45f  pf-390-announcements.png`
- `aaa06fdea504787d314a8d25f36e876da498c5f5d6f04a8a8177a4aca1531e2e  pf-390-bag.png`
- `eaaaa2707afce617a1119f016642145345643ac8ac371e2c9896001265cddff6  pf-390-enemies.png`
- `944046e4672545de24364abf21f398628c58e79e2c9b69c62630bfd4e2403a2e  pf-390-equipment.png`
- `c21f2520d67ca31fd25a633e55e6fc4de79e746df64f0e9182b1c5e6e252ca09  pf-390-gems.png`
- `1be216809a778244e38597ea3dffd94aaa840b85731edf61f6353147dcaebd29  pf-390-login.png`
- `c376b765f5f9a7d866f7c93e28fb4f3a67fbb45ad0f15a45479e1d9554935d00  pf-390-me.png`
- `de5abd4bea2bed06cc15409749c052898ecfdcdd5e5d99833cdac770320e76f3  pf-390-reactions.png`
- `c9587ab354dbc529a770d6010ed6adf6f8bbc17ef0b455206c35ebc0e395674f  pf-390-signin.png`
- `111627423c2d57a2819b4d21d765b469826109ffebc991da9901fb0310e23ac9  pf-390-skills.png`
- `7ea971b7799f2c66e50c4974edaa6ceee5ef86dba0e694209370b61aeff158e9  pf-390-skins.png`
- `db8e5ee6bd29f2ceb11447379fd4e119030e583d787401d85c47e147aa8941b0  pf-390-verify.png`
- `84003005ba01e3f4c50b04d893f4c99f55f75f227909b670d5a9c03f610ebc3d  pf-768-announcements.png`
- `8416d3ad6a82a3b038f4e8e6294b79dbc08121862628dbdc394af0b9ffd73df6  pf-768-bag.png`
- `3485cd64a1156ec5a447603f464a3c5a652b87e87340fbc177da5435cc911f90  pf-768-enemies.png`
- `82bb4a8c473c252be9d89a1a2196d821648337f5fa00cd3918559ba204f7e153  pf-768-equipment.png`
- `516ca85520a368c231ef478022357f48b9f6794e74da0abf322b431fa4c88e1e  pf-768-gems.png`
- `b147dc6223b8f83b0df1f80966061f37d5cf80cc968d5ff9dbec92b14eebeffa  pf-768-login.png`
- `fc7d7041da30a99a0f1ef0a1399caac48707e8710d1d4492b1104544d7c9fc4e  pf-768-me-expanded.png`
- `bbe9c88c810911010fd306d06194337b2f9fd9db10709622225f19b29abf08b5  pf-768-me.png`
- `1faa8c5090a120b4ec8777fca2d51d2f30835bde8f33bc80986c373b4c3d91d5  pf-768-reactions.png`
- `c1859d812906749ac785898f15a5ca76adb757e9de48148ebdcc06415e720487  pf-768-signin.png`
- `9c10c1cffa1095344d8e7e99fa918a56143206d19fa1d076926d474c3ec798d6  pf-768-skills.png`
- `1a6134f96c77ec97b8f8561f64757193266e0e2773427dc6bebb0800d3ce1b0e  pf-768-skins.png`
- `c66b0d054b5547c603be53bde52b50b1e184dc4396fd83d7204d6a616b3f8ee8  pf-768-verify.png`

- 亲读（read_image，均非登录页）：`pf-768-announcements.png`（sha `84003005…`，公告 AI 摘要 + CTA 正常）、`pf-768-login.png`（sha `b147dc62…`，账号登录表单区）、`pf-768-me-expanded.png`（sha `fc7d7041…`，指挥官档案 + 展开表单）。截图为真实业务页，非 chrome-error。

## 8. 诚实边界

- `analyze-postfix.js` 三处 verify FAIL 为 spinbutton 假阳性，已按 DOM `nativeAria="battle_id"` + AX `spinbutton "battle_id"` 更正为 PASS（§2）。
- 6 处反馈节点真浏览器坐实 5 处；me `passwordMsg` 因游客态环境与登录凭据限制未捕获，源码确认标注、静态契约覆盖（§4）。
- `TestNoRealSecretsInRepo` 为 SKIP，不冒充 PASS（§6）。
- me 基础页游客态 0 input，以 768-me-expanded 补证；这不是隐藏失败。
- 认证经 gitignored `data/auth-init.js` 会话侧注入，凭据不回显不入库；端口 5174 旧 302 服务已弃用。

## 9. 收尾

- 删除 `data/auth-init.js` 并验证 ABSENT；QA 库复核 `104|477`；`git check-ignore` 命中 `.gitignore:553`；自启 job（API 8080 / H5 5201 / 各浏览器 driver）全部 `job_kill`。
