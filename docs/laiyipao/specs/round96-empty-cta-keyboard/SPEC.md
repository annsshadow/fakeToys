# R96 规范：六页空态 CTA 键盘可达补漏

## 0. 判定与依据
- 来源：docs/laiyipao/recon/ROUND96_A_EMPTY_CTA_KEYBOARD_RECON.md（t226，真 Chromium 三视口坐实 battles 裸 CTA role=null / tabIndex=-1 / Tab×8 不命中 / AX 树仅 generic clickable，注入 tabindex 后 Enter 可达）。
- 权威证据：6 处 `<view class="r3-empty-action" @click="X()">文案</view>`（bag:43、battles:177、enemies:27、me:72、rank:74、skins:32）缺 role/tabindex/@keydown.enter；同组件类其余 10 处已是完整四件套，同构对照坐实结构漂移（WCAG 2.1.1 Keyboard）。
- 本轮性质：规范 + test-first 契约（RED）；不实施修复，修复由下一轮实施任务完成。

## 1. 缺口机制（真实、可复现）
- R60 契约 r60A11yKeyboard.contract.test.ts:41-53 只扫已带 role="button" 的 text/view，这 6 处压根无 role，属结构性盲区，R60 修 48 处未覆盖。
- 键盘用户 Tab 不命中、AX 树不暴露 button、Enter 无效；触屏可点。battles 三视口自然空态实测，bag 首次探索同型坐实。
- 后端未起时其余 4 页本地兜底数据先到达掩盖空态，真浏览器未自然复现，已如实披露（侦察报告 §5），不伪造证据。

## 2. 权威范式（同项目现有代码）
- R60/R95 四件套：`role="button" @click tabindex="0" @keydown.enter`；见 announcements.vue:71、equipment.vue:74/133、skills.vue:92，以及 me.vue:11（R95 修复返回按钮）。
- 属性顺序固定 role → @click → tabindex → @keydown.enter；common.css 已有 `[role="button"] cursor:pointer` 与 `[tabindex]:focus-visible`，无需新增样式或令牌。

## 3. 唯一改点
- 6 行模板各补三属性，保留原 class / @click / 文案，一行一处：
  - bag/bag.vue:43 `<view class="r3-empty-action" role="button" @click="r4GoBag()" tabindex="0" @keydown.enter="r4GoBag()">去关卡页 →</view>`
  - battles/battles.vue:177 `<view class="r3-empty-action" role="button" @click="r4GoBattle()" tabindex="0" @keydown.enter="r4GoBattle()">去关卡页挑一关开打 →</view>`
  - enemies/enemies.vue:27 `<view class="r3-empty-action" role="button" @click="loadPage()" tabindex="0" @keydown.enter="loadPage()">重新加载配置</view>`
  - me/me.vue:72 `<view class="r3-empty-action" role="button" @click="r4GoMe()" tabindex="0" @keydown.enter="r4GoMe()">去专精页 →</view>`
  - rank/rank.vue:74 `<view class="r3-empty-action" role="button" @click="r4GoRank()" tabindex="0" @keydown.enter="r4GoRank()">去冲榜关卡 →</view>`
  - skins/skins.vue:32 `<view class="r3-empty-action" role="button" @click="loadPage()" tabindex="0" @keydown.enter="loadPage()">重新加载配置</view>`
- 不改其他元素、不改样式、不改脚本、不改焦点函数实现。

## 4. 不变量（不可触碰）
- 六处焦点函数声明保留：r4GoBag@1377 / r4GoBattle@877 / loadPage(enemies)@342 / r4GoMe@973 / r4GoRank@260 / loadPage(skins)@323。
- 空态四段槽位顺序 r3-empty-slot < r3-empty-title < r3-empty-reason < r3-empty-action 不变；class / @click 绑定 / 文案逐字不变（契约 r4SecondaryPages.contract.test.ts:47-50 与 r5H5Pages.contract.test.ts:77-79 断言空态槽位与 @click，不锁标签名，补三属性不破坏）。
- 令牌三锚点不变：tokens.css:14 `--lyp-token-version: lyp-tokens-v1` / :15 `--lyp-r3-version: lyp-tokens-v3-r3` / :16 `--lyp-r6-version: lyp-tokens-r6`。
- 业务源码零 `--r96-`；不新增任何 R96 专用令牌。
- R60 既有 role+tabindex+@keydown 范式不回退（契约抽查至少 2 处）。

## 5. 契约结构（laiyipao/miniapp/src/styles/r96EmptyCtaKeyboard.contract.test.ts，8it）
- 真实 fs.readFileSync（import.meta.url + fileURLToPath + dirname/join）读 6 页源码，禁内嵌实现副本。
- 三条件锁定唯一模板行（class="r3-empty-action" + @click 原函数 + 原文案，expect hits.length=1）。
- it1-4 缺口断言（RED fail）：role="button" / tabindex="0" / @keydown.enter="X()" / 四属性顺序。
- it5-8 不变量（RED pass）：焦点函数声明 / 空态四段槽位顺序 / 令牌三锚点+零 --r96- / R60 范式抽查≥2处。
- 断言非弱化：定位唯一行 + 逐字属性校验，避免误判。

## 6. RED 门禁实测（test-first）
- 命令：`cd laiyipao/miniapp && pnpm exec vitest run src/styles/r96EmptyCtaKeyboard.contract.test.ts --reporter=verbose > ../../docs/laiyipao/specs/round96-empty-cta-keyboard/RED.txt 2>&1; echo RED_EXIT=$?`
- vitest v4.1.12，RED_EXIT=1（重定向日志后 echo，无管道掩盖）。
- Test Files 1 failed(1)；Tests 4 failed | 4 passed (8)；Duration 318ms，无解析错误 / 模块缺失 / 超时。
- 失败 4 项全为缺口断言（it1 bag 页 Received 为裸 <view class="r3-empty-action" @click="r4GoBag()">去关卡页 →</view>，缺 role/tabindex/@keydown.enter；it2/it3/it4 同型原始失败见 RED-evidence.md）；不变量 4 项（it5-8）全通过。
- RED 完全来自真实缺口，无假阳性。原始日志脱敏存档：docs/laiyipao/specs/round96-empty-cta-keyboard/RED-evidence.md。
- 独立复跑确认：同命令二次执行 RERUN_EXIT=1，Test Files 1 failed(1) / Tests 4 failed | 4 passed (8)，Duration 284ms，RED 稳定可复现。

## 7. 写域白名单与交接
- 本轮产出（UNTRACKED）：本 SPEC、RED-evidence.md、上述契约文件；不写业务源码 laiyipao/miniapp/src/pages/**/*.vue、不写 .gitignore、不写真值、不写 git。
- 实施轮仅改 6 行模板；完成后 8it 应全绿（it1-4 转绿，it5-8 恒绿）。
- 真浏览器复查建议：三视口 Tab 命中 6 页空态 CTA、Enter 触发对应函数、焦点环可见（复用 common.css [tabindex]:focus-visible）。
- 观察项（本轮不纳入）：index/index.vue:40/53/78/92 与 battle/battle.vue:462/465/469 裸 view @click 同型盲区，建议单列 a11y 轮。
- 本规范与契约不含任何密钥 / 令牌 / 真实服务器地址。