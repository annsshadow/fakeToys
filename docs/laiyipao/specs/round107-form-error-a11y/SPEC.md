# R107 规范：表单校验错误 a11y 语义契约补全

## §0 轮次定位与范围

- 基线 HEAD=`528e6477d3f22d160bcbca21a65e3f1ecf773383`（规范产出时核实未变）。
- 上游：t302 R107-A 只读侦察（报告 `docs/laiyipao/recon/ROUND107_A_FORM_ERROR_A11Y_RECON.md`，97 行）四条件全中裁定立轮；t303 自由探查无候选。
- 与 R101（展开态 aria-expanded）、R104（加载态 aria-busy）为不同维度：本轮专治**表单校验错误与输入控件的语义关联**，不重复立案。
- 唯一写域：`docs/laiyipao/specs/round107-form-error-a11y/`（SPEC.md / RED-evidence.md / RED-raw.log）与 `laiyipao/miniapp/src/styles/r107FormErrorA11y.contract.test.ts`。禁改业务 `.vue` / .gitignore / 令牌 / 其他契约。

## §1 问题与四条件裁定

静态决定性：miniapp 全仓 `aria-invalid` / `aria-describedby` / `aria-errormessage` / `aria-required` 命中 **0**。错误文本已有 role=alert/status 播报节点，但与输入控件无任何语义关联——读屏用户聚焦输入框时听不到错误内容，须跳出控件才能遇到孤立 alert。真 Chromium 三视口坐实：verify 错误态视觉生效（红框红字）但原生 input ariaInvalid=null / ariaDescribedBy=null、alert id=null、AX 中 spinbutton 与 alert 彼此孤立。

| 条件 | 判定 | 依据 |
|---|---|---|
| ① 真实可感知缺陷 | 成立 | DOM 属性 + AX 双证：错误文本播报但与输入控件无关联 |
| ② 有权威/同仓口径 | 成立 | MDN aria-invalid + aria-describedby；AI_VISUAL_SYSTEM §2.4/§922；admin 正面范式；无反向权威 |
| ③ 改动面小且局部 | 成立 | 3 页 4 个 input 各补 2 属性 + 3 个错误节点补 id + main.ts 桥接扩展，零新增 CSS |
| ④ 不破坏现有范式 | 成立 | 不动 R60/R61/R101/R104 与令牌三锚点；业务零 --r107-；契约设不变量 it 锁死 |

裁定：立轮，按下列队长口径固化契约（RED 先行）。

## §2 修复口径（逐字锁定）

**A. 3 页 4 个错误态 input**，各补 `:aria-invalid` 失败态绑定 + `:aria-describedby` 指向错误节点唯一 id（保留原 `aria-label` / `v-aria-native` / `@confirm` / `input-error` 绑定）：

- `login.vue:33` username：`v-model="username"`、错误条件 `!!errMsg`、describedby=`login-err`
- `login.vue:45` password：同上（与 username 共用 `errMsg` 节点）
- `verify.vue:53` battleId：`v-model="battleId"`、错误条件 `!!errMsg`、describedby=`verify-battle-err`
- `signin.vue:203` code：`v-model="code"`、错误条件 `!!redeemMsg && !redeemMsg.startsWith('兑换成功')`（**仅失败态标 true**）、describedby=`signin-redeem-err`

**B. 3 个错误播报节点补唯一 id**（保留既有 role/live 锚点）：

- `login.vue:89` errMsg text 补 `id="login-err"`（保留 `role="alert"`）
- `verify.vue:75` errMsg text 补 `id="verify-battle-err"`（保留 `role="alert"`）
- `signin.vue:215` redeemMsg text 补 `id="signin-redeem-err"`（保留 `role="status" aria-live="polite"`）

**C. 桥接扩展（main.ts:9-21 applyAriaNative）**：现只把 aria-label setAttribute 到原生 `input.uni-input-input`；uni-app H5 属性白名单同样丢弃 aria-invalid / aria-describedby，须扩展指令在 mounted/updated 把外层 uni-input 上编译保留的这两个属性一并 setAttribute 到原生 input。**不改 `binding.value: string` 签名**；`setAttribute('aria-label', name)` 必须保留不回退（R61 契约守卫）。

**D. bag 明确排除**：`doSavePreset` 错误走 `uni.showToast`（`bag.vue:1278-1280`），无页内错误节点，补语义须新增节点 + 业务逻辑，超出属性补漏范畴，记待办不在本轮。`me.vue` nicknameMsg 属状态反馈非校验错误，本轮不纳入。

**E. 不变量**：不动 R60 键盘三件套（`role=button` + `tabindex=0` + `@keydown.enter`）、R61 aria-label 与 17 input v-aria-native、R101 aria-expanded、R104 aria-busy、令牌三锚点 `tokens.css:14-16`（`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`）、业务零 `--r107-`。

## §3 契约设计

- 文件：`laiyipao/miniapp/src/styles/r107FormErrorA11y.contract.test.ts`，真实 `fs.readFileSync` 读 login/verify/signin.vue + main.ts + tokens.css。
- 定位：`inputBlock(src, vmodel)` 按 `v-model="xxx"` 向前找最近 `<input`、向后找最近 `>`（跨行容错）；`tagBefore(src, needle)` 按文案/唯一锚点向前找最近 `<text` / `<view`。**均不依赖行号**。
- 13 it = **9 缺口**（4 input 语义 + 3 错误节点 id + 2 桥接属性；实施前须失败）+ **4 不变量**（R61 label 桥接 + 4 input aria-label/v-aria-native；R60 键盘三件套 me back-btn；R101 aria-expanded + R104 aria-busy；令牌三锚点 + 零 --r107-）。
- 缺口断言：`:aria-invalid` 绑定值须含错误条件关键串（且不静态写死 `true`）；`:aria-describedby` 须含目标 id；错误节点须含 `id="xxx"` 且保留既有 role/live 锚点；main.ts 须有含 `aria-invalid` / `aria-describedby` 的 setAttribute 行。

## §4 验证命令

```bash
cd laiyipao/miniapp
npx vitest run src/styles/r107FormErrorA11y.contract.test.ts --reporter=verbose
```

RED 预期：exit=1，9 failed | 4 passed (13)；失败 it 全为缺口断言，4 不变量 it 通过。GREEN 预期（实施轮）：13/13；并复跑 `r61FormA11y.contract.test.ts` 确保未回退。

## §5 诚实边界

- 规范轮只写 SPEC + 契约 + RED 证据，不改任何业务 `.vue`；实施与真浏览器复查另轮。
- signin 兑换错误态在 R107-A 真浏览器未触发（按钮未命中），该处缺口以源码决定性。
- 属性桥接为非可见机制：实施轮复查须在真浏览器坐实原生 input 上的 aria-invalid/describedby 与错误节点 id 关联。
