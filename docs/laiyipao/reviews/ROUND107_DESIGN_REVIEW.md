# R107 独立终审报告 · 表单校验错误 a11y 语义

- 角色：same-route-reviewer（R40 独立终审，继承队长路由）
- 团队：laiyipao-r36；目标：R36 Wiki 合成技能表字段对齐权威 SeedSkill
- 审阅对象：t305 实施、t304 规范/契约、t306 QA（R107 表单校验错误 a11y 语义补全）
- 基线 HEAD：`528e6477d3f22d160bcbca21a65e3f1ecf773383`（全程未变）
- verdict：**pass** —— 六维全绿，无 blocker / high / medium；8 条 low 不阻断，同意分批发布

## 0. 审阅范围与写域纪律

本报告只读审阅，不改业务 vue / 契约 / 令牌 / .gitignore，唯一产出为本文件（UNTRACKED）。链路：t302 R107-A 侦察（四条件全中立案）→ t303 自由探查（无候选）→ t304 规范 SPEC + test-first RED → t305 实施 → t306 QA → 本轮终审。审阅对象是「错误态输入控件缺少 aria-invalid / aria-describedby 语义关联」这一**表单校验错误 a11y**维度，与 R101（aria-expanded 下拉）、R104（aria-busy 在途）分属不同属性维度，不重复。

## 1. 维度① RED 先行真实性 —— 通过

- 契约：`laiyipao/miniapp/src/styles/r107FormErrorA11y.contract.test.ts`（156 行）真实 `fs.readFileSync` 读 `login.vue` / `verify.vue` / `signin.vue` / `main.ts` / `tokens.css`；`inputBlock()` 按 `v-model` 锚点向前找 `<input`（跨度 ≤400 字符）跨行容错、`tagBefore()` 按文案锚点向前找 `<text>/<view>`、`attrValue()` 取属性值，**均不依赖行号**。
- 13 it = 9 缺口（4 input 语义 + 3 错误节点 id + 2 桥接属性）+ 4 不变量；缺口断言 `aria-invalid` 含错误条件关键串且 `.not.toBe('true')`（不静态写死），满足「测试编码意图」而非编码实现。
- RED 铁证：`docs/laiyipao/specs/round107-form-error-a11y/RED-raw.log:165` = `Tests  9 failed | 4 passed (13)`，`:164` = `Test Files  1 failed (1)`，`:167` = Duration 431ms；9 FAIL 逐条对应真实缺口（4 input / 3 节点 id / 2 桥接），4 PASS 为不变量。RED 期 exit=1，先行成立，无 PARSE_ERROR、无「假红」。
- 独立复跑 GREEN：见维度⑤ G1 = 13 passed / 13，RC=0。RED→GREEN 反转由同一契约驱动，真实有效。

## 2. 维度② 实施精确性 —— 通过

4 文件 diff 逐行与 SPEC §2 逐字吻合，零多余改动：

| 文件 | diff | 改动 | 与 SPEC 吻合 |
|---|---|---|---|
| `main.ts` | 9+/1- | `applyAriaNative` 内保留 R61 `setAttribute('aria-label', name)`，新增从外层 `el` 读 `aria-invalid` / `aria-describedby` 转发到原生 `input.uni-input-input`，无值则 `removeAttribute` | §2.C 逐字符合 |
| `login.vue` | 5+/1- | username、password 各加 `:aria-invalid="!!errMsg"` + `:aria-describedby="'login-err'"`；errMsg `<text>` 加 `id="login-err"` 保 `role="alert"` | §2.A/B |
| `verify.vue` | 3+/1- | battleId 加 `:aria-invalid="!!errMsg"` + `:aria-describedby="'verify-battle-err'"`；errMsg `<text>` 加 `id="verify-battle-err"` 保 `role="alert"` | §2.A/B |
| `signin.vue` | 3+/1- | code 加 `:aria-invalid="!!redeemMsg && !redeemMsg.startsWith('兑换成功')"` + `:aria-describedby="'signin-redeem-err'"`；redeemMsg `<text>` 加 `id="signin-redeem-err"` 保 `role="status" aria-live="polite"` | §2.A/B |

- 全部 **零新增 CSS**、零令牌改动；原 `aria-label` / `v-aria-native` / `@confirm` / `:class="input-error"` 全部保留。
- `.gitignore` +6 行 hygiene 增量（round107-a / round107-form-error-a11y / round107-form-error-a11y-postdeploy 的 data/ 与 admin-auth-init.js），属合理真值防护，非业务改动。

## 3. 维度③ QA 真实性 —— 通过

- 方法：真 Chromium（agent-browser 0.41.1）三视口 390×844 / 768×1024 / 1280×800；后端 8080 未起，全量 init-script 合成响应，占位令牌 `r107-qa-placeholder-token` 仅 localStorage 内存注入零真值；改前 baseline（t302 recon）坐实 verify 错误态视觉生效（红框红字）但原生 `ariaInvalid=null` / `ariaDescribedBy=null` / alert `id=null`、AX 中 spinbutton 与 alert 孤立。
- §2 三视口 9 组错误态全部坐实：原生 `aria-invalid` 由 false→true 字符串、`aria-describedby` 恒指唯一 id、3 错误节点 id 编译保留、零横溢、均非登录页。§3 768 verify 清空 battle_id 点「取复现信息」后原生 `ariaInvalid false→true` / `ariaDescribedBy=verify-battle-err` / alert `id=verify-battle-err`，AX 快照三视口一致控件与错误文本同时可见。
- 9 图 sha256 用 `ops_stat revision` 重算，与 REPORT §6 逐字 **MATCH 9/9、UNIQUE 9/9**（三张 verify 图 sha 与改前 recon 同名一致，纯 a11y 零视觉变化符合预期）。路径 `docs/laiyipao/audits/round107-form-error-a11y/screenshots/`。
- 亲读 2 张非登录页（本轮独立复核）：`r107-verify-768.png`（sha de0e9b2d…，768×1024，战绩验真页，AI 复核提示 + battle_id 红框错误态）；`r107-signin-1280.png`（sha c9a81c91…，1280×800，签到页兑换码 INVALID-CODE 红框 + 红字「兑换码无效」）。两图非空态，与 DOM/AX 探针一致。

## 4. 维度④ 不变量与边界 —— 通过

- 令牌三锚点 `tokens.css:14-16`（`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`）不动；业务零 `--r107-`（`laiyipao/miniapp/src` 全仓仅契约自身 3 处命中：注释 1 + 断言 2）。
- 契约 4 条不变量 it 逐条锁死并通过：R61 aria-label 桥接保留 + 4 input 仍有 `aria-label` + `v-aria-native`；R60 me back-btn 键盘三件套（role/tabindex/@keydown.enter）；R101 `:aria-expanded="String(expanded.has(a.id))"` 与 R104 `aria-busy="true" role="status"`；令牌三锚点 + 零 `--r107-`。
- CTA / 焦点环 / 触摸目标等前轮成果未在 diff 中被触碰。QA 诚实边界充分（见重点③⑤）。

## 5. 维度⑤ 独立门禁复跑 —— 通过

本轮自建 `.t307-gates.sh` 后台独立复跑（不复用 QA 的 gates-final.sh）：

| 门禁 | 命令 | 结果 | RC |
|---|---|---|---|
| G1 | `npx vitest run src/styles/r107FormErrorA11y.contract.test.ts` | 1 passed / 13 tests | 0 |
| G2 | `npx vitest run src/styles/r61FormA11y.contract.test.ts` | 1 passed / 5 tests | 0 |
| G3 | 全量 `npx vitest run --config vitest.config.ts` | 239 passed \| 1 skipped (240) files；2209 passed \| 2 skipped (2211) tests | 0 |
| G4 | `npm run type-check`（vue-tsc） | 无错误 | 0 |
| G5 | `npm run build:h5`（uni build） | DONE Build complete | 0 |
| G6 | `git diff --check` | 仅 .gitignore LF→CRLF 提示（非错误） | 0 |
| G7 | `go test ./internal/domain/ -run TestNoRealSecretsInRepo` | SKIP（`.local.keys` 不存在，守卫第 1 条按设计跳过；第 2 条定位信息规则仍执行），PASS | 0 |

G1–G7 全 RC=0，与 QA §7 门禁结果一致。G3 中 `[Vue warn] Failed to resolve directive: aria-native` 与 `build.attacker` 缺失为测试既有 stderr 告警，非本轮引入。

## 6. 维度⑥ 卫生 —— 通过

- HEAD = `528e6477d3f22d160bcbca21a65e3f1ecf773383` 未变。
- `git status` 非 `??` 项仅：`M .gitignore` + `M main.ts` + `M login.vue` + `M signin.vue` + `M verify.vue`（全为预期 t305；其余大量 `??` 为历史遗留 `.rXX-*.sh` 与各 round audits，非本轮产出）。
- auth 五路径（.env 与 admin / deploy / server 的 .local.keys、miniapp 的 .env.local）全 IGNORED；`.pem/.key` = 0；报告零真值（命中仅回环 127.0.0.1 与占位令牌名）。
- 自启 job `.t307-gates.sh`（`bash-6fac86d66d2a4bce`）已 completed；临时脚本收尾删除。

## 7. 五大重点独立判断

**① main.ts 桥接机制与 R61 一致性、removeAttribute 回归风险** —— 判定安全，无回归。机制完全沿用 R61 的 `Directive<HTMLElement, string>` 在 `mounted/updated` 双钩子调用 `applyAriaNative`，定位原生 `input.uni-input-input` 的方式（`el.tagName==='INPUT' && classList.contains('uni-input-input') ? el : el.querySelector(...)`）未改，`setAttribute('aria-label', name)` 必须保留的 R61 语义逐字未动（G1/G2 不变量 it 通过）。新增的两属性转发用「外层 `el.getAttribute()` 有值则 set、无值则 removeAttribute」的对称写法：`aria-invalid` / `aria-describedby` 在 uni-app H5 属性白名单里本就被外层丢弃、靠本指令桥接，外层无值时清除原生同名属性可避免 updated 阶段残留过时属性（例如错误消除后 `aria-invalid` 应回到 false/移除而非停停留留 true）——这是与 R61「只改名不碰值/事件/class」一致的对称设计。QA §3 已实测错误态 false→true 与错误消除后的属性值，行为正确，无回归。

**② signin 错误条件 `!redeemMsg.startsWith('兑换成功')` 合理性** —— 判定合理。`redeemMsg` 既承载成功态（「兑换成功…」）也承载失败态（「兑换码无效」等），仅失败态标 `aria-invalid=true` 符合 WCAG：成功态不应把输入控件标为无效。该条件与同 input 的 `:class="{ 'input-error': !!redeemMsg && !redeemMsg.startsWith('兑换成功') }"`（改动前既有）完全同口径，语义一致、零新增判定逻辑；契约缺口断言仅要求含 `redeemMsg` 关键串且不写死 true，未锁死具体实现，保留合理空间。

**③ describedby 静态绑定在无错误态指向未渲染 id 的可接受性** —— 判定可接受，且已诚实披露。`:aria-describedby="'login-err'"` 等静态绑定恒指错误节点 id，无错误态时 `v-if errMsg/redeemMsg` 使错误节点未渲染，理论上形成「指向不存在 id」。但：(a) 这是 R107 契约与源码的同口径设计（契约要求 `:aria-describedby` 含目标 id，本就静态）；(b) 无错误态下不存在「错误播报」，辅助技术读到不存在的 referenced id 时不产生实际错误播报，不构成 WCAG 失败；(c) QA §4 与 §8 已明确披露、不冒充「无陈旧属性」全绿；(d) 错误态出现时错误节点同步渲染，`aria-describedby` 指向有效 id，AX 可见控件与错误文本同时出现（QA §3 三视口坐实）。属 low 观察项而非缺陷。

**④ 与 R101 / R104 是否不同维度** —— 判定不同维度，不重复。R101 处理下拉展开态 `aria-expanded`（index.vue），R104 处理写操作在途 `aria-busy` / `role=status`（equipment.vue），本轮处理表单校验错误 `aria-invalid` + `aria-describedby`（login/verify/signin.vue + main.ts 桥接）。三者属性、目标控件、错误/状态语义、修复文件与契约互不重叠；本轮 diff 未触碰 R101/R104 锚点，契约不变量 it 逐条锁死其不被误改（G1 通过）。

**⑤ QA signin 合成 HTTP500 诚实边界充分性** —— 判定充分。QA §8 明确披露 signin 兑换失败态由 mock 让 `/redeem` 返回 HTTP500 + error.message 触发 `client.ts` reject → `signin.vue:481 redeemMsg=兑换码无效`，并说明为何不能用 HTTP200 + ok:false（`request()` 对 200 直接 resolve 不抛错，故不能只返回 200+ok:false）——机制解释准确、可复现。后端 8080 未起、占位令牌内存注入均如实声明；改前 recon 该按钮未命中未触发、本轮经修正 mock 才坐实的曲折也已披露。真实后端可用时 `redeemMsg` 仍由同一 error.message 路径产生，故合成故障与真实错误态走同一展示逻辑，边界合理，不影响「错误态 a11y 语义生效」结论。

## 8. low 观察项（不阻断发布）

| id | 严重度 | 观察 |
|---|---|---|
| L1 | low | 后端 8080 未起，QA 全量 init-script 合成响应，非真实服务端交互；展示逻辑路径一致，不影响本轮 a11y 语义结论 |
| L2 | low | describedby 无错误态静态指向未渲染 id（重点③），属契约同口径设计，QA 已披露 |
| L3 | low | signin 兑换错误态由合成 HTTP500 触发（重点⑤），非真实服务端错误 |
| L4 | low | AX 快照未显式打印 described-by 引用标记，关联由原生 DOM 属性坐实，非直接可读 AX 引用 |
| L5 | low | 亲读 2/9 图，其余 7 张以 sha 重算 + DOM 探针佐证 |
| L6 | low | 三张 verify 截图与改前 recon 同名同 sha（纯 a11y 零视觉变化符合预期），语义差异以 DOM/AX 为准 |
| L7 | low | Go 真值守卫因 `.local.keys` 不存在 SKIP（CI 设计如此），本地未做真值子串比对；定位信息规则与 gitignore 检查仍执行 |
| L8 | low | G3 既有 `[Vue warn] aria-native` 与 `build.attacker` 缺失 stderr 告警，非本轮引入，不影响 RC=0 |

## 9. 结论

R107 表单校验错误 a11y 语义补全：RED 先行真实、实施与 SPEC 逐字精确、QA 三视口坐实、不变量与令牌未回退、独立门禁全绿、卫生合规；五大独立判断点均判定安全/合理。六维全绿，无阻断项。**verdict = pass，同意分批发布。**
