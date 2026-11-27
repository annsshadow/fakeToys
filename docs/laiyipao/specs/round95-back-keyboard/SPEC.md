# R95 规范：我的页返回按钮键盘可达

## 0. 判定与依据

- 来源：docs/laiyipao/recon/ROUND94_A_NAV_BACK_RECON.md（t214，真 Chromium 三视口×6 路径坐实返回目标）。
- 权威证据：laiyipao/miniapp/src/pages/me/me.vue:11 `<text class="back-btn tap" @click="goBack">‹ 返回</text>` 实测 role=null / tabindex=-1，AX 树仅 StaticText，Tab 6 次不命中；R60 同维度遗漏（WCAG 2.1.1 Keyboard）。
- R60 权威范式（同项目现有代码）：`role="button" @click tabindex="0" @keydown.enter` 四件套；见 announcements.vue:36/71、equipment.vue:64/74、battles.vue:151/282。
- 本轮性质：规范 + test-first 契约（RED）；**不实施修复**，修复由下一轮实施任务完成。

## 1. 缺口机制（真实、可复现）

- me.vue 显式「‹ 返回」入口在 `v-if="canGoBack"` 的 `view.me-back-row` 内，仅有 @click="goBack"。
- 键盘用户无法 Tab 到该按钮，Enter 也不能触发返回；触屏可点、键盘失效。
- 系统原生返回箭头 `uni-page-head-btn` 属 uni-app 框架编译产物，不可局部修（t214 诚实排除项）。

## 2. 唯一改点

- me.vue:11 单行模板补齐三个属性，保留原 class / @click / 文案：
  `<text class="back-btn tap" role="button" @click="goBack" tabindex="0" @keydown.enter="goBack">‹ 返回</text>`
- 不改其他元素、不改样式、不改脚本；一行模板修复。

## 3. 不变量（不可触碰）

- canGoBack 判定保持 `getCurrentPages().length > 1`（me.vue:755-761），try/catch 兜底不变。
- goBack 保持 `uni.navigateBack({ delta: 1 })`（me.vue:763-765），行为语义不变。
- back-btn 样式（me.vue:1764-1779：flex-shrink/min-height/padding/color/font-size + :active 左移降透明度）不变。
- 令牌三锚点不变：laiyipao/miniapp/src/styles/tokens.css:14-16 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`。
- 业务源码零 `--r95-`；不新增任何 R95 专用令牌。
- R60 既有 48 处 role=button 键盘 CTA 范式不回退（契约抽查至少 2 处仍具 role+tabindex+@keydown.enter）。

## 4. 契约结构（laiyipao/miniapp/src/styles/r95BackKeyboard.contract.test.ts，6it）

- 真实 fs.readFileSync 读 me.vue（import.meta.url + dirname/join），禁内嵌实现副本。
- it1 = back-btn 行含 role="button"；it2 = 含 tabindex="0"；it3 = 含 @keydown.enter="goBack"（三缺口断言，RED 时 fail）。
- it4 = canGoBack 的 getCurrentPages().length>1 与 goBack 的 navigateBack({delta:1}) 保持；it5 = R60 键盘范式抽查≥2处；it6 = 令牌三锚点 + 业务源码零 --r95-（三不变量，RED 时 pass）。
- 断言非弱化：逐字/结构校验，定位唯一模板行（back-btn + @click="goBack" + 「‹ 返回」同时命中），避免误判。

## 5. RED 门禁实测（test-first）

- 命令：`cd laiyipao/miniapp && pnpm exec vitest run src/styles/r95BackKeyboard.contract.test.ts --reporter=verbose`，vitest v4.1.12。
- 原始退出码 exit=1（无管道掩盖：重定向日志后 `echo RED_EXIT=$code`）。
- Test Files 1 failed(1)，Tests 3 failed | 3 passed (6)，Duration 355ms，无解析错误/模块缺失/超时。
- 三条失败均为真实缺口断言（it1/it2/it3：Received 为 `<text class="back-btn tap" @click="goBack">‹ 返回</text>`，缺 role/tabindex/@keydown.enter）；三条不变量（it4/it5/it6）通过。
- RED 完全来自三个真实缺口，无假阳性。

## 6. 写域白名单与交接

- 本轮产出（UNTRACKED）：本 SPEC 与上述契约文件；**不写业务源码 me.vue**、不写真值、不写 git。
- 实施轮仅改 me.vue:11 单行；完成后 6it 应全绿（it1/2/3 转绿，it4/5/6 恒绿）。
- 真浏览器复查建议：三视口 Tab 能聚焦返回按钮且 Enter 触发返回；焦点环可见（复用 common.css [tabindex]:focus-visible）。
- 本规范与契约不含任何密钥/令牌/真实服务器地址。
