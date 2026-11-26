# R566 · stage.vue 裸 @click 伪按钮键盘可达修复

- 范围：laiyipao/miniapp 关卡页 stage.vue 3 类伪按钮（章节切换 chip / 关卡格 / 开始战斗按钮）键盘不可达
- 标准：WCAG 2.1 SC 2.1.1 A（键盘可达），关联 4.1.2 A（角色、名称、状态可编程判定）
- 任务：t97（implementation）｜真父：22eeaa8936d231790a275965b2406368f3d2eb1e｜R565 甄别判 P2 真 bug，按纪律 RED → GREEN 优先

## 方案取舍（Fail Loud）

- 方案 A（全局增强器 / MutationObserver 自动补属性）：**排除**。越界改动他页与动态文案（章节名 / 关卡名 / startLabel 四分支）无法从 DOM 可靠派生，且 main.ts 仅 createSSRApp + createPinia，引入全局增强器会破坏零改约束。
- 方案 B（逐页显式补 a11y 属性）：**采用**。就地追加 role="button" / tabindex="0" / aria-label / @keydown.enter / @keydown.space.prevent 复用原 handler，保留全部 @click，与既有 R561/R562/R564 先例一致。
- 拒绝 tablist / tab / aria-selected 半吊子实现：任务只要求键盘可达，不要求方向键导航；roving tabindex 反而会让部分 tab 脱离 Tab 序列，故以普通 button 语义 + aria-pressed 类似范式处理。

## 写域（精确 3 文件）

- M laiyipao/miniapp/src/pages/stage/stage.vue（15 insertions 纯属性追加）
- A laiyipao/miniapp/src/pages/R566.stageKeyboardA11y.contract.test.ts
- A laiyipao/docs/ops/laiyipao-r566-stage-keyboard-fix.md（本文）

零改：switchChapter / enter / startBattle 函数体 / pages.json / store / api / 样式 / 他页 / main.ts / 已有 a11y 契约 / battles+rank 正面对照 / @click 全保留。

## 源码修复（3 处）

| 位置 | 候选 | 追加属性 | 复用 handler |
|---|---|---|---|
| :16-22 | chapter-chip（v-for ch in chapters） | role=button, tabindex=0, :aria-label=ch.name+当前章节/点击切换, @keydown.enter / .space.prevent | switchChapter(ch.id) |
| :104-116 | level-cell（v-for lv in chapterLevels） | role=button, tabindex=0, :aria-label=第 N 关 + lv.name, @keydown.enter / .space.prevent | enter(lv.id) |
| :218-224 | start-btn | role=button, tabindex=0, :aria-label=startLabel, @keydown.enter / .space.prevent | startBattle |

- 真父 stage.vue sha256 = 0d3a1225b366829bf4a28555feb81168ee39a148ae4f2e7e9a3bfae0e6621705
- 修复版 stage.vue sha256 = d2fc3aacbf669746e573a29b7d0644a565c56c0ddc13a13fb61fad9bbe1d0a57（git diff 15 insertions，纯属性追加，业务零改）

## 契约与 RED → GREEN

- 契约 7 用例：3 核心（blockFrom 定位开标签 + role/tabindex/aria-label/keydown 复用 + @click 保留 + @click 总数 3）+ 4 守卫（业务函数体 / api+store+样式零改 / main.ts 零增强器 / battles+rank 正面对照 + svg 关卡难度曲线零改 + 无 tablist/tab/aria-selected）
- 契约 sha256 = d3d9ba72a5a8f1464a1193e7cc328f07a4c39e2d126677d387993269497c9994
- RED（git show 真父覆盖）：EXIT=1，3 failed | 4 passed，日志 sha256 = da032ae3e53b138586e1b291192f38d83c18b0336a1d22ec337164743bc7f1c2
- GREEN（修复版恢复）：EXIT=0，7 passed，日志 sha256 = 7326f3e5525238790fc96438a8686adc14c4f1061d86db70d967ea5d4632f6ab

## 三 verify（workdir = laiyipao/miniapp，字面命令）

1. pnpm test src/pages/R566.stageKeyboardA11y.contract.test.ts → EXIT=0，7 passed，日志 sha256 = ea9fb6c5a3c936ad0f34462ab5d33fb51d96b9cda6e9ecac17029babd7a68c74
2. pnpm test → EXIT=0，160 files passed | 1 skipped（161 files），1551 tests passed | 2 skipped（1553 tests），日志 sha256 = ef3fe66f4958fceefc37ca905d2eebe632d769eab5bb4d568df64536e7a00c2b
3. pnpm type-check → EXIT=0，vue-tsc 无错，日志 sha256 = 37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f

## 真浏览器 CDP 决定性判据

- 环境：自起 laiyipao H5（<TEST_HOST>:<PORT>，端口占位脱敏），Playwright 390x844 DPR2 isMobile/hasTouch
- 工具：CDP DOM.getAttributes 读 DOM 属性 + Accessibility.getFullAXTree 取 focused=true 非 RootWebArea 节点；focusProbe 用 page.evaluate el.focus() + wait 200ms
- 4 态：withData / empty × light / dark
- runner sha256 = 12d93b999300aa482463773e79869854c313d560ea25484410a6b25fee38acdc
- 报告 sha256 = a02c451466eb6cfcd7bc691a2f206e37676b28fe1e27c69ee10397cacd8c9be1
- E2E 日志 sha256 = ff83f66a444d88c38f39f112fb6ffb1803c19953a7a849a8a0792317ca82fd4d
- 汇总 sha256 = 6ab62bd1cecd13c76fd4cc0c40a666e9bd8057203c81502c3a8e248e35f0ac49，cases=4，fatals=0，pageErrors=2（音频 play 被 pause 打断提示，非缺陷）

判据（4 态全绿）：

1. 3 类焦点节点 AX role=button，name 分别 = "R566第一章，当前章节" / "R566第二章，点击切换" / "第 1 关 R566第一关"（start：withData="体力不足"、empty="未选择关卡"）
2. document.activeElement = UNI-VIEW[role=button]，isTarget=true、isBody=false，**非宿主 wrapper**（R535 教训）
3. DOM.getAttributes：role=button / tabindex=0 / aria-label 达标；empty 态无关卡格（level0 attr=null）据实披露
4. Tab 命中：withData chip10 / level10 / start5；empty chip15 / level0 / start7
5. 注入对照：removed aria=null → restored "R566-injected-tab"，证明 AX 通路有效
6. 键盘等价：chip1 Enter 使 activeChapter 0→1、levelCount 2→1；level0 Enter 后 currentLevel 切换、detailTitle="第 3 关 · R566第三关"；start Space 跳 hash=#/pages/battle/battle?level=2
7. 鼠标零回归：dispatchEvent click chip 回第 0 章
8. 正面对照：被测页注入原生 button aria-label="R566正面对照控件"，AX full role=button / name 达标（自包含验 AX 通路，规避 R564 跨页 svg 该态未渲染的假阴性）
9. 0 fatal

### 截图取证瑕疵披露

- withData-light / withData-dark 两张截图拍在 start Space 跳转之后，显示 battle 页"无法开始战斗"错误态；empty-light / empty-dark 正常展示 stage 页（章节 chip + "本章节还没有关卡" + "未选择关卡"按钮）
- 这恰好佐证 start Space 跳转判据生效；决定性判据（AX role/name、activeElement、DOM 属性、键盘等价、注入对照）均在跳转前采集且全绿，不依赖截图
- 4 图 SHA256：
  - withData-light = 5497c25f004f3b3187272f4f0fc44c85e40fa993594402f950ddf860bd0f19f4
  - withData-dark = 75bbdf868b7f49af5b1997397baa37e75243dafdb46249c0490cbb04653938c2
  - empty-light = f5dd3e2833d4aaf596f87d6bd95bc2db75f0e3551d7a81c7cdd5f42de7139f3e
  - empty-dark = 7842d0bc0b410439cb4ad1c5657989875a3b3c3257ea66c7fd8287966d4e4283
- H5 无主题机制，light/dark 本应同 sha；本次因截图时窗口状态差异而不同，据实披露

## SHA 汇总表

| 项 | SHA256 |
|---|---|
| 真父 stage.vue | 0d3a1225b366829bf4a28555feb81168ee39a148ae4f2e7e9a3bfae0e6621705 |
| 修复版 stage.vue | d2fc3aacbf669746e573a29b7d0644a565c56c0ddc13a13fb61fad9bbe1d0a57 |
| 契约 | d3d9ba72a5a8f1464a1193e7cc328f07a4c39e2d126677d387993269497c9994 |
| RED 日志 | da032ae3e53b138586e1b291192f38d83c18b0336a1d22ec337164743bc7f1c2 |
| GREEN 日志 | 7326f3e5525238790fc96438a8686adc14c4f1061d86db70d967ea5d4632f6ab |
| V1 定点日志 | ea9fb6c5a3c936ad0f34462ab5d33fb51d96b9cda6e9ecac17029babd7a68c74 |
| V2 全量日志 | ef3fe66f4958fceefc37ca905d2eebe632d769eab5bb4d568df64536e7a00c2b |
| V3 typecheck 日志 | 37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f |
| E2E runner | 12d93b999300aa482463773e79869854c313d560ea25484410a6b25fee38acdc |
| E2E 汇总 | 6ab62bd1cecd13c76fd4cc0c40a666e9bd8057203c81502c3a8e248e35f0ac49 |
| E2E report | a02c451466eb6cfcd7bc691a2f206e37676b28fe1e27c69ee10397cacd8c9be1 |
| E2E 日志 | ff83f66a444d88c38f39f112fb6ffb1803c19953a7a849a8a0792317ca82fd4d |
| withData-light 截图 | 5497c25f004f3b3187272f4f0fc44c85e40fa993594402f950ddf860bd0f19f4 |
| withData-dark 截图 | 75bbdf868b7f49af5b1997397baa37e75243dafdb46249c0490cbb04653938c2 |
| empty-light 截图 | f5dd3e2833d4aaf596f87d6bd95bc2db75f0e3551d7a81c7cdd5f42de7139f3e |
| empty-dark 截图 | 7842d0bc0b410439cb4ad1c5657989875a3b3c3257ea66c7fd8287966d4e4283 |

脱敏：全文仅用 <TEST_HOST>:<PORT> 占位，无真实域名 / IP / 端口 / 凭据 / 个人路径。
