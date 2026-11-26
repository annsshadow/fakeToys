# R61 规范：表单输入语义与错误播报（WCAG 4.1.2 / 3.3.1）

- 轮次：R61
- 任务：t108（kind=implementation，attempt=1，attempt_id=6aba88f3-9e59-49dc-ad6b-6587ab7ca486）
- 角色：same-route-lead（规范+契约，test-first RED）
- 来源：docs/laiyipao/recon/ROUND61_A_FORM_A11Y_RECON.md（t106 真 Chromium 36 组合铁证）
- 工作根：本工作树根目录
- 日期：2026-11-26
- 口径：只写规范与契约，不改业务源码、不 git 写、零真值

## 0. 判定与红线

真实缺口（t106 坐实）：H5 编译后每个 `<input class=uni-input-input>` 的 aria-label / aria-labelledby / labels 全空，placeholder 属性也为 null（占位文本由兄弟节点 DIV.uni-input-placeholder.ph 承载），无障碍树显示裸 textbox；触发校验后 errMsg / redeemMsg / passwordMsg / form-hint 等即时反馈节点 role=null、aria-live=null，读屏器不播报。
标准依据：WCAG 4.1.2 Name/Role/Value（每个可聚焦控件须有可访问名）；WCAG 3.3.1 Error Identification（错误须以文本标识并可被感知）。
红线：
- 只可改 laiyipao/miniapp/src/pages 下相关 .vue 的模板展示层属性与文本节点语义。
- 禁碰 engine/replay/hash/score/settle/loot/tokens、后端、权威 SeedSkill 数值口径。
- 禁改 CTA 文案与路由（R43/R44/R46/R47/R48 言行一致性已收口）。
- 禁回退 R60 键盘维度：role=button 的 tabindex/@keydown、common.css focus-visible 与 [role=button] 规则不得删除。
- 禁回退 R55 占位惯例：input 的 placeholder-class="ph" 必须保留（对比度 6.0）。
- 令牌三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 不变；禁新增 --r61- 令牌。
- 零真值：不引入密钥、token、真实域名/IP、个人路径。

## 1. 缺口机制（为何是局部遗漏）

- 全 src aria-label 命中 29 处，但全在容器/view/svg（bag.vue:25、index.vue:61/68/81/142/243、me.vue:27/36/125），无一在 input。
- 同仓库已有 60+ 处 role="status" aria-live="polite"（battles.vue:283/285/310/340、equipment.vue:70/86/129/241、gems.vue:43/57/72、reactions.vue:87/239/241/271、skills.vue:28/83/99/274、announcements.vue:67、me.vue:68），用于空态/结果播报——表单即时反馈未接入该惯例，属局部遗漏而非平台限制。
- axe-core 4.12.1 label 维度 4 页 violations=0 与裸 textbox 不矛盾：uni-app 编译后原生 input 无 placeholder/id/label 关联时 axe 判 inapplicable；以 DOM/AX 客观量测为准。

## 1A. t110 真浏览器复查暴露的根因与修复口径（运行时桥接）

根因（t110 报告 docs/laiyipao/audits/round61-form-a11y/REPORT.md 坐实，结论 FAIL/reject）：
- 静态 aria-label 写在 `<input>` 模板标签上，uni-app H5 编译后把它落在外层自定义元素 `<uni-input class="inp" aria-label="用户名">`；原生 `<input class="uni-input-input">` 的属性来自硬编码白名单（value/onInput/disabled/type/maxlength/step/enterkeyhint/pattern/class/style/autocomplete/onKeyup/inputmode，见 node_modules 内 @dcloudio/uni-h5.es.js:10856-10886），aria-label 不在白名单内被丢弃。铁证 data/ancestry-login.json：native input attributes 仅 type/maxlength/step/enterkeyhint/class/autocomplete/inputmode，无 aria-label。
- 结果：11 个渲染页 AX 树 textbox 全裸名，axe-core 4.12.1 violations 含 label（impact=critical）。t109 静态契约 4/4 GREEN 只证明源码带属性，不证明运行时原生 input 带名——源码层与运行时层必须分开验证。
- 对照实验：真浏览器对 native input 执行 setAttribute 写 aria-label 后 AX 树立即带名，证明运行时桥接可行。
- 反馈节点真浏览器坐实 4/6（login form-hint/errMsg、verify errMsg、signin redeemMsg）；me passwordMsg 与 bag presetSaving 未捕获，不虚构已验证。

修复口径（静态保留 + 运行时桥接，二者缺一不可）：
- 保留第 2 节 17 处 `<input>` 上逐字静态 aria-label（it1 不回退；源码层语义对其余端与 SSR 仍有意义）。
- 新增一个全局自定义指令 v-aria-native：mounted / updated 钩子里从 el（uni-input 根元素）用 querySelector('input.uni-input-input') 找到原生 input，再以 setAttribute('aria-label', binding.value || el.getAttribute('aria-label')) 把可访问名写上去；binding.value 可显式覆盖，未传时回退读取 el 上静态名。
- 实施轮给 17 处 input 模板块挂 v-aria-native，并在 laiyipao/miniapp/src/main.ts 注册 app.directive('aria-native', { mounted, updated })。
- 桥接红线：只设可访问名，不改输入值、事件、class、placeholder、路由；不引入真值；不碰 engine/replay/hash/score/settle/loot/tokens 与后端。

## 2. 17 处 input 逐字 aria-label 清单（文案取现有可见占位 / label）

| # | 文件:行 | 现有 placeholder | 应补 aria-label |
|---|---|---|---|
| 1 | announcements/announcements.vue:87 | 搜索公告标题 / 正文 | aria-label="搜索公告标题 / 正文" |
| 2 | bag/bag.vue:405 | 预设名（最多 20 字） | aria-label="预设名（最多 20 字）" |
| 3 | equipment/equipment.vue:147 | 搜索装备名称 / 部位 / 元素 | aria-label="搜索装备名称 / 部位 / 元素" |
| 4 | enemies/enemies.vue:34 | 搜索敌人名称 / 类别（如 Boss、飞行） | aria-label="搜索敌人名称 / 类别（如 Boss、飞行）" |
| 5 | gems/gems.vue:83 | 搜索宝石名称 / 属性 | aria-label="搜索宝石名称 / 属性" |
| 6 | login/login.vue:33 | 用户名 | aria-label="用户名" |
| 7 | login/login.vue:43 | 口令 | aria-label="口令" |
| 8 | me/me.vue:86 | 新昵称（≤64 字） | aria-label="新昵称（≤64 字）" |
| 9 | me/me.vue:568 | 原口令 | aria-label="原口令" |
| 10 | me/me.vue:577 | 新口令（至少 10 位） | aria-label="新口令（至少 10 位）" |
| 11 | me/me.vue:615 | 用户名 | aria-label="用户名" |
| 12 | me/me.vue:623 | 密码（至少 10 位） | aria-label="密码（至少 10 位）" |
| 13 | reactions/reactions.vue:185 | 搜索反应名称 | aria-label="搜索反应名称" |
| 14 | skills/skills.vue:154 | 搜索技能名称 / 元素 / 主动被动 | aria-label="搜索技能名称 / 元素 / 主动被动" |
| 15 | signin/signin.vue:203 | 输入兑换码 | aria-label="输入兑换码" |
| 16 | skins/skins.vue:39 | 搜索皮肤名称 / 稀有度 | aria-label="搜索皮肤名称 / 稀有度" |
| 17 | verify/verify.vue:53 | 例如 1（占位为示例值，语义取可见 label） | aria-label="battle_id" |

逐点最小改动：在每个 `<input ...>` 标签内加一个 aria-label 属性；不改 v-model、placeholder、placeholder-class、class、:class、@confirm、:disabled 等既有属性。第 17 处占位「例如 1」是示例值非语义名，取同卡片可见标题「输入 battle_id」的语义名 battle_id。

## 3. 错误 / 状态节点播报清单

| # | 文件:行 | 节点 | 推荐播报属性 | 判定口径（满足其一） |
|---|---|---|---|---|
| 1 | login/login.vue:76 | form-hint「请填写用户名与口令后可登录」 | role="status" aria-live="polite" | role="status" 且 aria-live="polite" |
| 2 | login/login.vue:85-87 | errMsg | role="alert" | role="alert" 或（role="status" 且 aria-live="polite"） |
| 3 | verify/verify.vue:72-74 | errMsg | role="alert" | 同上 |
| 4 | signin/signin.vue:212-214 | redeemMsg（成功/失败两态，中性播报） | role="status" aria-live="polite" | role="status" 且 aria-live="polite" |
| 5 | me/me.vue:595-602 | passwordMsg（成功/失败两态） | role="status" aria-live="polite" | 同上 |
| 6 | bag/bag.vue:413 | presetSaving「保存中…」状态 | role="status" aria-live="polite" | 同上 |

说明：纯错误（errMsg）优先 role="alert"；含成功/失败两态的反馈（redeemMsg / passwordMsg / form-hint / presetSaving）用 role="status" aria-live="polite"，沿用仓库既有惯例。契约对错误节点接受 role="alert" 或 role="status"+aria-live="polite" 二者其一，对状态节点只接受 role="status"+aria-live="polite"。

## 4. 修复判据（供实施轮）

1. 第 2 节 17 处 input 每处标签块内出现与「应补 aria-label」列逐字一致的属性。
2. 第 3 节 6 个错误/状态节点按「判定口径」列带播报属性。
3. 不删任何既有 v-if / :key / class / :style / @click / @confirm 绑定，不改 CTA 文案与路由。
4. 不新增全局样式，但允许新增一个 a11y 全局指令 v-aria-native 用于把可访问名桥接到原生 input；focus-visible 复用 common.css:907 既有规则，不重复造。

## 5. 不变量与保护面

- role="button" 键盘维度：R60 契约 r60A11yKeyboard.contract.test.ts 所钉 tabindex/@keydown/common.css [role="button"] cursor:pointer 与 focus-visible 覆盖 [tabindex] 的规则不得因本轮修复被删。
- 占位惯例：17 处 input 的 placeholder-class="ph" 全部保留。
- 数值权威：不触碰 engine/replay/hash/score/settle/loot/tokens 与后端，不改 SeedSkill 字段与任何权威数值。
- 令牌三锚点不变、无 --r61-。

## 6. QA 验证口径（供后续真浏览器复查轮）

- 契约层：新契约 exit=0，17 处 aria-label 与 6 处播报属性全绿。
- 真浏览器：每个 input 在空值/有值两态下 computed name 非空（AX 树 textbox 带名称，不再是裸 textbox）；触发空字段校验后 AX 树出现 role=alert 或 role=status 节点且文本匹配。
- 三视口 390×844 / 768×1024 / 1280×800 复测，确认零横向溢出、零 CTA 越界（不回退 R54/R56/R57/R58 视觉结论）。

## 7. RED 实测记录

命令：cd 'D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp' && npx vitest run src/styles/r61FormA11y.contract.test.ts --reporter=verbose 2>&1
RC=1，Test Files 1 failed (1)，Tests 2 failed | 2 passed (4)，Duration 337ms。
失败 1（it1，r61FormA11y.contract.test.ts:86:69）逐字摘要：「以下 input 无可访问名，AX 树为裸 textbox: pages/announcements/announcements.vue v-model=annKeyword 缺 aria-label=\"搜索公告标题 / 正文\"; pages/bag/bag.vue v-model=presetName 缺 aria-label=\"预设名（最多 20 字）\"; …（17 处全列）… pages/verify/verify.vue v-model=battleId 缺 aria-label=\"battle_id\": expected [ …(17) ] to deeply equal []」。17 处全为真实缺口断言。
失败 2（it2，r61FormA11y.contract.test.ts:102:53）逐字摘要：「以下反馈节点读屏器不会播报: form-hint 空字段提示: 缺 role=alert 或 role=status+aria-live=polite; login errMsg: 缺 …; verify errMsg: 缺 …; signin redeemMsg: 缺 …; me passwordMsg: 缺 …; bag presetSaving: 缺 …: expected [ …(6) ] to deeply equal []」。6 处全为真实缺口断言。
通过 2 条：it3（R60 tabindex/@keydown 契约锚点、R43/R44/R48 CTA 锚点、R55 placeholder-class=ph 惯例 17 处全在）与 it4（令牌三锚点 lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6，无 --r61-）。
无 Transform failed / 模块缺失 / 超时；两条失败即两候选本身，假阳性零，RED 正当。

### t111 修复轮 RED 实测（追加）

命令（相对工作树根）：cd laiyipao/miniapp && npx vitest run src/styles/r61FormA11y.contract.test.ts --reporter=verbose 2>&1
RC=1，RUN v4.1.12，Test Files 1 failed (1)，Tests 1 failed | 4 passed (5)，Duration 314ms（transform 31ms, setup 0ms, import 46ms, tests 31ms, environment 0ms）。
唯一失败 = it5（r61FormA11y.contract.test.ts:147:91）AssertionError: 「以下 input 未挂运行时桥接指令 v-aria-native，编译后原生 input 仍无可访问名: pages/announcements/announcements.vue v-model=annKeyword 缺 v-aria-native（静态 aria-label 编译后落在 uni-input 外层，AX 树裸名）; …（17 处全列：announcements/bag/equipment/enemies/gems/login×2/me×5/reactions/skills/signin/skins/verify）… pages/verify/verify.vue v-model=battleId 缺 v-aria-native（静态 aria-label 编译后落在 uni-input 外层，AX 树裸名）: expected [ …(17) ] to deeply equal []」。17 处全为真实缺口断言。
通过 4 条：it1（17 处静态 aria-label）/ it2（6 处播报属性）/ it3（R60 键盘锚点 + CTA 锚点 + placeholder-class=ph 惯例）/ it4（令牌三锚点，无 --r61-）。
无 Transform failed / 模块缺失 / 超时；唯一失败即 it5 运行时桥接缺口本身，假阳性零，RED 正当。

## 8. 轨迹

- t106 侦察：真 Chromium 36 组合坐实输入无命名 + 反馈无播报，报告 docs/laiyipao/recon/ROUND61_A_FORM_A11Y_RECON.md。
- t108 规范轮：立规范与契约（test-first RED），静态 aria-label + 播报属性。
- t109 实施轮：在 12 个 .vue 给 17 处 input 加静态 aria-label、6 处反馈节点加 role/aria-live，静态契约 4/4 GREEN。
- t110 真浏览器复查：FAIL/reject。uni-app H5 把静态 aria-label 编译到外层 uni-input 自定义元素，原生 input.uni-input-input 属性白名单丢弃 aria-label，11 页 AX 树 textbox 全裸名，axe label(critical) violation；报告 docs/laiyipao/audits/round61-form-a11y/REPORT.md。
- t111 本轮（修复规范）：保留 17 处静态 aria-label，新增 v-aria-native 运行时桥接方案；SPEC 增 1A 根因/修复口径节并区分 §4 红线，契约新增 it5（RED）；不改 main.ts 与页面源码。
- 后续实施轮：给 17 处 input 挂 v-aria-native 并在 src/main.ts 注册 app.directive('aria-native', { mounted, updated })，跑契约 GREEN 后交真浏览器复查。