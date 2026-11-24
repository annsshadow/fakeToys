# R488 WCAG 2.1 AA 3.3.1 Error Identification 错误识别甄别报告

- 轮次：R488（台账 §483 据实立项）
- 结论速览：**可达真缺陷 1 项（P2）**；不可达 L1 0；正常/规范预期若干
- 性质：只读甄别，本轮不改业务代码；缺陷修复建议供后续轮次实施
- 真值声明：本报告不含任何真实域名、IP、主机、SSH 端口或凭据。测试地址以 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` 占位；截图所载运行时 URL 在证据中已脱敏。

## 1. 测试环境与方法（如实标注）

- 浏览器：Playwright Chromium **headless**，视口 **1440×900**，DPR=1
- **非物理真机**：以下截图均为无头浏览器渲染的测试服页面截图，**不是物理设备真机截图**，不冒充真机
- 主题：light / dark 双主题各跑一轮
- 判定手段：真实 UI 流程触发校验错误态（空提交 / 两次密码不一致 / 必填缺失 / 错误凭据 / 强制移除原生 required 触发业务校验），再用 CDP `Accessibility.getPartialAXTree` 取可访问树角色与名称，配合 DOM `computedStyle` / `getAttribute` 读取 `role` / `aria-live` / `aria-invalid` / `aria-describedby` / 可见文本
- 硬纪律：严禁仅凭源码出现 `aria-invalid` 0 命中就预判缺陷；错误态必须真实出现（避免 R466 式误判）；toast 语义播报的错误态判正常，不因"有视觉红框"误判
- 一致性纪律（针对 R487 覆辙）：报告结论与 `report.json` / `supplement.json` / `focus.json` 逐项 programmatic 字段一致，判据在采集脚本内计算并回写 JSON 的 `probe.class`，本报告不复算、不改口径

## 2. 判据（三分判定）

| 分类 | 判据 |
| --- | --- |
| 可达真缺陷 | 校验错误已真实出现，但错误提示仅视觉呈现（无 role=alert/status、无 aria-live、关联控件无 aria-invalid/aria-describedby），读屏用户无法程序化识别错误 |
| 不可达 L1 | 错误态不可达，或对 WCAG 3.3.1 不适用（如预防性 disabled 阻止提交、原生 required 拦截使业务错误未产生） |
| 正常 | 错误以 toast role=alert + aria-live=assertive 或专用 role=alert 文本播报；或由全局增强器补全语义；或未触发错误态（仅路由播报）——判 0 |

## 3. 静态侦察（决定性结构，勿重复查）

- `frontend/apps/desktop/src/utils/toast.ts:23-24` `getContainer` 设 `role=status` + `aria-live=polite`；`:60-61` error 类型 `el.setAttribute(role, alert)` + `aria-live=assertive`，图标 ✗ 色 `#ef4444`。toast 错误确实语义播报。
- `frontend/apps/desktop/src/utils/formErrorA11y.ts`（R188）`installFormErrorA11y(root=document)` 只处理带 `data-oa-field-error` 标记的错误文本：补 `role=alert` + 稳定 id（`oa-field-error-NNNNNN`）+ 同字段控件 `aria-invalid=true` / `aria-describedby` 指向错误 id，MutationObserver 驱动。`main.ts:10` 导入 + `:614` 调用。
- 全库 `data-oa-field-error` 仅 1 处真实消费：`frontend/apps/desktop/src/components/XformRuntime.vue:61` `<p v-if="errors[fieldKey(module)]" class="xform-error" data-oa-field-error>`。**增强器只覆盖工作流 XformRuntime，不覆盖 Personal / Calendar / ThreeMember 的 .error-msg。**
- `Personal.vue:33-51` `pwdForm/pwdError`；`savePassword()`：空值 → `pwdError=请填写完整密码`（纯赋值无 toast），两次不一致 → `pwdError=两次密码不一致`（纯赋值无 toast）。模板 `:50` `<div v-if="pwdError" class="error-msg">{{ pwdError }}</div>`（无 role / 无 aria-live）。CSS `:785` `.error-msg{color:var(--color-error);font-size:13px;padding:8px 0}`。
- `LoginScreen.vue:81` `role="alert" aria-live="assertive"` 语义正确。
- `CalendarApp.vue:131-136` 表单 `title/startTime/endTime` 带原生 `required`；`:470-474` 缺失时 `toast.error(标题与起止时间必填)`；日历 chip 选择器 `.cal-chip`。
- `ThreeMemberApp.vue:39-41` 新建弹窗姓名/手机/邮箱 input；`:46` 保存按钮 `:disabled=!form.name?.trim()||saving`（预防性）；`:172/:198` onError `toast.error`，无前端格式校验。

## 4. Real UI 实测逐组 programmatic 明细

### 4.1 主轮（`report.json`，10 组，errors=0，0 pageerror）

| 主题/场景 | probe.class | 可见错误文本 | DOM role / aria-live | 三密码框 aria-invalid | AX 语义 |
| --- | --- | --- | --- | --- | --- |
| light/login-badcred | SEMANTIC | Login did not create a session | `alert` / `assertive` | — | alert（axSemantic=1） |
| light/personal-empty | **VISUAL_ONLY** | 请填写完整密码 | `-` / `-` | `["","",""]` | generic，axName 空（axSemantic=0） |
| light/personal-mismatch | **VISUAL_ONLY** | 两次密码不一致 | `-` / `-` | `["","",""]` | generic，axName 空（axSemantic=0） |
| light/calendar-empty | NO_ERROR | 无 | — | — | 仅路由播报 |
| light/threemember-invalid | NO_ERROR | 无 | — | — | 仅路由播报 |
| dark/login-badcred | NO_ERROR | 无（复用登录态） | — | — | 仅路由播报 |
| dark/personal-empty | **VISUAL_ONLY** | 请填写完整密码 | `-` / `-` | `["","",""]` | generic，axName 空（axSemantic=0） |
| dark/personal-mismatch | **VISUAL_ONLY** | 两次密码不一致 | `-` / `-` | `["","",""]` | generic，axName 空（axSemantic=0） |
| dark/calendar-empty | NO_ERROR | 无 | — | — | 仅路由播报 |
| dark/threemember-invalid | NO_ERROR | 无 | — | — | 仅路由播报 |

汇总：`semantic=1, visualOnly=4, noError=5`；visualGroups=`[light/personal-empty, light/personal-mismatch, dark/personal-empty, dark/personal-mismatch]`。

### 4.2 补采（`supplement.json`，5 组，errors=0，0 pageerror）

| 主题/场景 | probe.class | 实测事实 |
| --- | --- | --- |
| light/calendar-real | NO_ERROR | 选中日历 chip 后弹表单，但空提交被浏览器原生 required 拦截，业务 `toast.error` 未执行 |
| light/threemember-modal | NO_ERROR | 新建弹窗空姓名时 `saveDisabled=true`（预防性 disabled），错误态不可达 |
| dark/calendar-real | NO_ERROR | 同 light/calendar-real |
| dark/threemember-modal | NO_ERROR | 同 light/threemember-modal |
| dark/login-dark-wrong | NO_ERROR | 复用既有登录态，未产生错误 |

### 4.3 focus 探针（`focus.json`，5 组，errors=0，0 pageerror）

| 主题/场景 | probe.class | 可见错误文本 | DOM role / aria-live | AX |
| --- | --- | --- | --- | --- |
| light/calendar-forced | SEMANTIC | ✗标题与起止时间必填（auto-hide） | toast `alert` / `assertive` | role=alert |
| dark/calendar-forced | SEMANTIC | ✗标题与起止时间必填（auto-hide） | toast `alert` / `assertive` | role=alert |
| light/personal-semantics | NO_ERROR | 无（空表单未提交） | — | 三密码框 required=false / aria-required="" / aria-invalid="" / aria-describedby="" |
| dark/personal-semantics | NO_ERROR | 无（空表单未提交） | — | 同上 |
| dark/login-dark-fresh | SEMANTIC | Login did not create a session | `alert` / `assertive` | role=alert（全新 context 错密，停在 /login） |

说明：calendar-forced 为移除原生 required 后触发的真实业务校验错误；personal-semantics 证明空表单未提交时密码框无残留错误语义，DEFECT 判定只基于 4.1 中真实出现错误的 VISUAL_ONLY 组。

## 5. 截图 SHA-256（20 张，命令行核验 mismatch=0）

> 全部为 1440×900 headless 测试服截图，**非物理真机**；同页 light/dark 或不同组复用相同 sha 属正常（页面本身未变）。

| 组 | 截图 | SHA-256 |
| --- | --- | --- |
| light/login-badcred | pc-login-badcred-light-r488.png | `f1cce92fe70ba40b12f656b56c986849b3a3d05f0fbc6b88006dbbf97738d7bd` |
| light/personal-empty | pc-personal-empty-light-r488.png | `59f8d4054245ad2bde2f7e81d6364fd3be06b40d2a02e521df7f0e48c77ff746` |
| light/personal-mismatch | pc-personal-mismatch-light-r488.png | `7af278ac98270a1e3c8210e2f97df00ccb5e229e7259ab29fee5ce77d5faffcd` |
| light/calendar-empty | pc-calendar-empty-light-r488.png | `8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad` |
| light/threemember-invalid | pc-threemember-invalid-light-r488.png | `3879ca4f0e2b97706c71525a181500ca04f19eff50eac685a74b27d02a30fff3` |
| dark/login-badcred | pc-login-badcred-dark-r488.png | `6481d04b43638fd711b95f1b64a9950c0ae84983f8dd40a579808396054f4c70` |
| dark/personal-empty | pc-personal-empty-dark-r488.png | `4e8da018138913b68bcd2b9958a0aaeb55809b8fc19efbc6fe5022d916121996` |
| dark/personal-mismatch | pc-personal-mismatch-dark-r488.png | `f3e1b7b98d068667bed85c0aa5ee513ac1690d1e65cddd3fc2144cd0b2f301bc` |
| dark/calendar-empty | pc-calendar-empty-dark-r488.png | `0075d87d57dc86a5bc3963fa095ecbabaa06562542fbaea5922b2234fe9f6c4b` |
| dark/threemember-invalid | pc-threemember-invalid-dark-r488.png | `2aa86202c2e94604ee8a6bd557b9b9be5d8d293c4d56e916e24a8313f0e2997b` |
| light/calendar-real | pc-calendar-real-light-r488.png | `8131e083ec5af2b8468787ed04030b7a84a3174341b7d4ce50aae7c4070ae870` |
| light/threemember-modal | pc-threemember-modal-light-r488.png | `99ac76222918b0dbf9dc3fa112db775be01f4ed75ad0fbb04bb5c6ab740b1b29` |
| dark/calendar-real | pc-calendar-real-dark-r488.png | `e8d54232b55ad0c0f956bbaabaf69165150f3095cf2d8969f8331bb026b9dbe8` |
| dark/threemember-modal | pc-threemember-modal-dark-r488.png | `c34c494b0c0f6015cbb2fec680b81a8c469901c2c6f3cc0bee1e8d162366237e` |
| dark/login-dark-wrong | pc-login-dark-wrong-r488.png | `b1a796172c726df1367eff654d884b8cb15fee81f0eb5ae06a06f75baea0a8cf` |
| light/calendar-forced | pc-calendar-forced-light-r488.png | `ae5520d5be876b7e25aadf556ed113031a29b10ca85d39c6d15ec01f1c156535` |
| dark/calendar-forced | pc-calendar-forced-dark-r488.png | `3b1e53256bb616b2210ff483c3b49a902a3c52374568a10fd44dd62749f649ce` |
| light/personal-semantics | pc-personal-sem-light-r488.png | `59f8d4054245ad2bde2f7e81d6364fd3be06b40d2a02e521df7f0e48c77ff746` |
| dark/personal-semantics | pc-personal-sem-dark-r488.png | `4e8da018138913b68bcd2b9958a0aaeb55809b8fc19efbc6fe5022d916121996` |
| dark/login-dark-fresh | pc-login-dark-fresh-r488.png | `c431d11b2267c0394c61e45ee4153cbc3683ceb906798cbd7dc8b95fbec07ede` |

## 6. 三分判定

### 可达真缺陷：1 项（P2）

- **Personal 修改密码错误态纯视觉、无语义播报**
  - 实证：主轮 4 组（light/dark × empty/mismatch）错误文本真实出现（`请填写完整密码` / `两次密码不一致`），但 `.error-msg` `role=-` `aria-live=-`；CDP AXTree 为 `generic`、`axName` 空、`ignored=false`；三密码框 `oldPwd/newPwd/confirmPwd` `aria-invalid=""` `aria-describedby=""`。读屏用户无法程序化识别"发生了错误以及哪个字段出错"。
  - 根因：`Personal.vue:50` 的错误 div 无语义属性，且不在 `formErrorA11y` 增强器覆盖面（该增强器只吃 `data-oa-field-error` 标记）。
  - 影响：违反 WCAG 3.3.1（错误应以文本描述并以程序化方式呈现）。P2——核心账户安全流程、双主题复现、读数屏用户。

### 不可达 L1：0

### 正常 / 规范预期（判 0，不记缺陷）

- Login 错误：`role=alert` `aria-live=assertive`（主轮 light + focus dark 全新 context 两组实证）
- Calendar 业务校验错误：`toast.error` → `.oa4-toast` `role=alert` `aria-live=assertive`（calendar-forced 双主题实证）
- XformRuntime 工作流表单：`data-oa-field-error` + 全局增强器补全语义（静态坐实，本轮未触发错误态，不计缺陷）
- ThreeMember：空姓名 `saveDisabled=true` 预防性阻止（错误态不可达，不构成缺陷）
- 5 组 NO_ERROR（仅路由 announcer `aria-live=polite` `已切换到：XX` visible=false）：未触发校验错误，判 0；**不臆造为缺陷**

## 7. 修复建议（供后续轮次，本轮不改码）

1. 在 `frontend/apps/desktop/src/views/personal/Personal.vue:50` 的错误 div 增加 `role="alert"`（最小改动，使错误文本被即时播报）。
2. 更完整方案：给该错误 div 加 `data-oa-field-error` 标记，纳入既有 `installFormErrorA11y` 增强器，使其自动补 `role=alert` + `id` 并给关联控件补 `aria-invalid=true` / `aria-describedby`。若为多字段分别报错，应为每个错误文本绑定对应 input。
3. 不改动任何业务校验逻辑（空值/一致性判断保持原样），仅补语义层。

## 8. 实测脚本与证据索引（均 gitignored）

- `test-results/r488-e2e.cjs` + `r488-run.sh` → `r488-error-identification/report.json`（主轮 10 组）
- `test-results/r488-supplement.cjs` + `r488-sup-run.sh` → `r488-error-identification/supplement.json`（补采 5 组）
- `test-results/r488-focus.cjs` + `r488-focus-run.sh` → `r488-error-identification/focus.json`（focus 5 组）
- `test-results/r488-unified.cjs` → `UNIFIED.txt`；`r488-verify.cjs` → SHA_VERIFY total=20 mismatch=0
- 统一口径常量：`MAIN_VISUAL_ONLY=4`（全 Personal，P2）、`MAIN_SEMANTIC=1`、`FOCUS_SEMANTIC=3`、`PERSONAL_FLAW_GROUPS=4`

## 9. 结论

R488 WCAG 3.3.1 甄别：**可达真缺陷 1 项 P2**（Personal 修改密码错误态纯视觉），不可达 L1 0，正常若干。所有结论与 3 份 JSON 的 `probe.class` programmatic 字段逐项一致；20 张截图 SHA 命令行核验无偏差；测试地址与截图 URL 已脱敏占位，报告与提交均不含真值。建议下一轮按 §7 实施 RED→GREEN 修复。
