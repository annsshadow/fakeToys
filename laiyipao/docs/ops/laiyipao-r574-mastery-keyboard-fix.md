# R574 laiyipao/miniapp 专精页 mastery.vue 3 类裸 @click 伪按钮键盘可达修复

## 1. 背景与缺陷
- 缺陷：laiyipao/miniapp 专精页 mastery.vue 全文件仅 3 处裸 @click 文本伪按钮（重铸 / 家族 tab / 专精节点）无 role/tabindex/aria-label/@keydown，键盘用户无法 Tab 到达，AT 读出 generic/name=null，违反 WCAG 2.1 SC 2.1.1 A（关联 4.1.2 A）。
- 可达性：首页一跳 navigateTo 可达、零等价入口（R573 甄别）。
- 来源：R573 甄别文档 laiyipao/docs/ops/laiyipao-r573-mastery-keyboard-audit.md 判 P2 真 bug。
- 修复纪律：按真 bug 的 RED→GREEN，先写契约在真父跑 RED，再修复跑 GREEN。

## 2. 方案取舍（Fail Loud）
- 方案A（全局增强器）：排除。理由：全局注入越界他页，动态 aria-label 无法从 DOM 可靠派生易误伤；R537 增强器仅覆盖宿主表单控件，不覆盖小程序自定义元素。
- 方案B（逐页显式补）：采用。3 处开标签就地补 role=button + tabindex=0 + 静态/动态 aria-label + @keydown.enter/.space.prevent 复用原 handler，@click 全保留，业务零改。先例 R561–R572 全绿。
- 家族 tab 形态：选 role=button + aria-pressed + tabindex=0 + 动态 aria-label=f.name，拒绝 role=tab + tablist。理由：验收要求两枚家族 tab 均 Tab 可达，WAI-ARIA Tabs 的 roving tabindex 只让选中项入 Tab 序列、其余需方向键导航，而原逻辑仅内联赋值 curFamily=f.family 无方向键逻辑，半吊子 tablist 反伤 AT；与 R564 rank.vue / R572 signin.vue 先例一致。契约守卫钉死无 tablist/tab/aria-selected。
- link-off 禁用外观：甄别已证 link-off 仅为外观 class、无 disabled 语义，重铸按钮在 link-off 态仍保留键盘可达（本次 E2E empty 态专精点不足时 aria=重铸、focused role=button 达标，据实披露）。

## 3. 写域与零改
- 写域精确 3 文件：
  - M laiyipao/miniapp/src/pages/mastery/mastery.vue（3 处开标签纯 a11y 属性追加）
  - A laiyipao/miniapp/src/pages/R574.masteryKeyboardA11y.contract.test.ts
  - A laiyipao/docs/ops/laiyipao-r574-mastery-keyboard-fix.md（本文件）
- 零改：resetMastery/toggle/load 函数体、内联 curFamily 赋值、api、样式、他页、main.ts、既有 a11y 契约、battles/rank/stage 正面对照、3 条 @click 全保留。

## 4. 源码修复表（3 处，git diff 16 insertions 纯属性追加）
| # | 位置 | 元素 | 追加属性 | 复用 handler |
|---|---|---|---|---|
| 1 | :26-33 | 重铸按钮 text.link | role=button tabindex=0 aria-label=重铸/重铸中（动态） | resetMastery |
| 2 | :42-52 | 家族 tab（v-for families）view.tab-chip | role=button tabindex=0 aria-label=f.name aria-pressed=curFamily===f.family | 内联 curFamily = f.family |
| 3 | :106-124 | 专精节点（v-for nodesInLayer）view.node | role=button tabindex=0 aria-label=节点名+值+点亮态（动态） | toggle(n) |
- 3 处均为多行开标签，ops_edit_file 原子 3 处、dryRun 先验证唯一匹配后一次写入，@click 全部保留。
- 修复版 sha=548032599f850ebafb9d0f2c5b4eeec617160b609d193f182fd6b547d96ace89，真父 sha=164f9a1049909f040dd3cf494e61ae221d1f052300a68409603bd7b9a8d24428（见 §9）。

## 5. 契约 RED→GREEN
- 契约 7 用例 = 3 核心（blockFrom 起始锚点向下扫到 trimEnd().endsWith(>)，断言 role/tabindex/aria-label/@keydown 复用原 handler、无 tablist/tab/aria-selected）+ 4 守卫（handler 函数体零改、api/样式/main.ts 零改无全局增强器、battles/rank/stage 正面对照零改、@click 总数恒 3 无原生 button）。
- 踩坑：初版守卫 7 含 a11y 数量断言致真父 RED 4 failed|3 passed（守卫也 fail），不符核心 fail 守卫 pass 纪律 → 将数量断言并入核心 2，守卫 7 改纯守卫断言。
- RED（真父 git show 覆盖）：EXIT=1，3 failed | 4 passed，HEAD 未变。
- GREEN（修复版）：EXIT=0，7 passed。
- 恢复：cp 修复版备份回工作树，sha=548032599f850ebafb9d0f2c5b4eeec617160b609d193f182fd6b547d96ace89 坐实，HEAD 保持真父。

## 6. 三 verify（workdir laiyipao/miniapp，字面命令全 exit 0）
- V1：pnpm vitest run --config vitest.config.ts src/pages/R574.masteryKeyboardA11y.contract.test.ts → 7 passed，EXIT=0
- V2：pnpm test → 164 files passed | 1 skipped（165 files），1588 tests passed | 2 skipped（1590），EXIT=0
- V3：pnpm run type-check → vue-tsc 无错，EXIT=0

## 7. 真浏览器 CDP 决定性判据
- 环境：Playwright chromium headless 390x844 DPR2 isMobile/hasTouch，自起 laiyipao/miniapp H5，CDP getFullAXTree prop(n,focused)===true 非 RootWebArea。
- 判据口径：document.activeElement + getFullAXTree focused 非 root 节点的 AX role/name；严禁宿主 wrapper 假阳性（R535 教训）；UNI-VIEW/UNI-TEXT 身份独立确认。
- 6 组 = light/dark × withData/empty（4 组基线截图）+ func + mouse 探针，cases=6 fatals=0 pe=0。
- 决定性判据全绿：
  - 4 组双主题 5 类 focused 节点：重铸 tag=UNI-TEXT、家族 tab/专精节点 tag=UNI-VIEW，isTarget=true isBody=false 非宿主 wrapper；axFocused=[role=button]，name=重铸/焰系/霜系/flame-L1N1…，+10，已点亮（empty 态为可点亮）。
  - DOM.getAttributes role=button/tabindex=0/aria-label 达标；getFullAXTree 映射 backendDOMNodeId 读 role/name 与 partial 一致。
  - Tab 命中（withData/empty 各同）：reset 2、tab0 2、tab1 2、node0 1、node1 1，distinct 16。
  - 注入对照：给 5 类 data-r574 控件重设 role=button/tabindex=0/aria-label=R574-inj-*，AX role/name 全部达标（自包含验 AX 通路，规避跨页假阴性）。
  - 家族 tab 选中态程序化可读：初始 tabClasses=[tab-chip active, tab-chip]，Enter 切霜系后 [tab-chip, tab-chip active]、树标题焰系→霜系、节点集随之切换；Space 回焰系。
  - 功能真命中：nodeEnter 选可点亮节点（node4）→ writeCalls=[allocate:104]、专精点 5→4、节点点亮 class 转移；resetEnter → writeCalls=[reset]、节点全清、resetClass=link link-off。
  - 鼠标零回归：click tab1 切霜系 + click reset 写 reset。
  - 0 pageerror / 0 fatal。
- battles 正面对照：battles.vue 已修 6 类控件（role=button/aria-label/tabindex=0）由契约守卫钉死零改，源码 grep 复核存在；E2E 侧以自包含注入原生 button 验 AX 通路，未虚构跨页 E2E 对照（据实披露）。

## 8. 造数与据实披露
- stub 端点：/config（mastery_families 两系各 3 层×4 节点，含 rating_weights/drop_rates 全字段）、/mastery（points/nodes/layer_limit/base_slots/total_slots）、/mastery/reset、/mastery/allocate、/wallet、/me、/me/stars、/auth/*。
- 造数分层：家族 tab 常驻零业务造数（纯前端 curFamily 切换，三重反馈=.tab-chip.active + 树标题 + 节点集）；重铸需 resetTokens>0 且有已点亮节点（POST /mastery/reset）；专精节点需 points>0 且未锁定（POST /mastery/allocate）。
- runner 踩坑 4 处（均属取证脚本，非产品缺陷）：① chromium.launchContext 不存在 → 改 chromium.launch().then(b=>b.newContext())；② 首版 stub config 字段不全致应用渲染报 Cannot read properties of undefined (reading 'length')、5 类焦点探针全 null → 改用 R573 完整字段模板重建；③ func case 家族 tab Enter 切系后 v-for 重建致 node 的 data-r574 标记丢失、null.focus fatal → 每步操作前重跑 mark+inject；④ nodeEnter 初选 node2 为不可点亮节点无写请求 → 改为动态选首个 selectable 节点（node4）后 writeCalls 命中 allocate:104。
- 端口披露：默认开发端口被他组 worktree 占用，uni-app 硬编码该端口忽略 --port 传参，服务自动顺延到备用端口后 HTTP=200 取证；真地址以 <TEST_HOST>:<PORT> 占位，不入库。
- 截图 4 张 780 宽目视名实相符：withData=专精点 5 + 焰系 +2 + 已点亮 2 节点 + 可点亮 4；empty=专精点 3 + 重铸道具 0 + 重铸 link-off + 未点亮。H5 无主题机制，light/dark 视觉一致：empty 两主题同 sha；withData 两主题 sha 微差来自截图窗口状态，据实披露。
- 决定性判据均在截图/功能探针前采集，不依赖截图。

## 9. sha256 证据表（逐项核对）
分两组：A 组为 runner 紧凑 JSON 汇总值，B 组为文件 sha256sum 实测值。

A. runner 汇总/汇总日志
| 项 | sha256 |
|---|---|
| R574_DONE 汇总 | c93974d9385c51e958f2d0461fb09632c3ac6d5a0d292ef2a7ff4744513f8c8a |
| E2E 日志 | 1cf38285ed946d7ba4e2b884279ca2b30f9c05815dcc225ead3169b844a9b101 |

B. 文件 sha256sum（逐一实测）
| 文件 | sha256 |
|---|---|
| mastery.vue（修复版） | 548032599f850ebafb9d0f2c5b4eeec617160b609d193f182fd6b547d96ace89 |
| mastery.vue（真父） | 164f9a1049909f040dd3cf494e61ae221d1f052300a68409603bd7b9a8d24428 |
| R574.masteryKeyboardA11y.contract.test.ts | 2a44d1a54352f67bb10203b8b91e017c3e5c69d3a2a235717c7f530d2b8d74a5 |
| r574-e2e.cjs | 9d62d60cf68da5e498b784e3fb55903d3bb8e24714df7287f82609cd314b7afc |
| r574-report.json | e9af64d7eb5019acfc89e32408e23626aeca2dc5d71bd5f86e48fe6537f641d5 |
| /tmp/r574-red.log | 8d1115b220b366af4000cee7f695bb6a98788c9146200419303e452b11737fe8 |
| /tmp/r574-green.log | d8e89d0f89aed2dc5972206750131d36b5b4677974c110c113cd75482a8bab89 |
| /tmp/r574-v1.log | ad68ffa1575f404ffa8787271cbc12779591c779b85520466e9fc9550f07a8f7 |
| /tmp/r574-v2.log | 829eef162def888e9aaea1f5a48df8325dad17d7ff3cfad8d43ecbe7d9ffd1ec |
| /tmp/r574-v3.log | 37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f |
| r574-withData-light.png | 4f893ca932a395fad46481058455514926e01a7e70a71ef5fb3616cc5b4db98b |
| r574-withData-dark.png | e826057dd82bf6a3f7237e497ec72fb0752bcb007e99fe3a4f13e7e9b44d79ed |
| r574-empty-light.png | 14c3176e1e5e783b81b79f3e942cc77d4e51932c89abcb832ee1dc6e5fcc7e1f |
| r574-empty-dark.png | 14c3176e1e5e783b81b79f3e942cc77d4e51932c89abcb832ee1dc6e5fcc7e1f |

## 10. 脱敏检查
- 全文 0 真值：无真实域名/IP/端口/用户名/绝对路径/token，仅 <TEST_HOST>:<PORT> 占位。
- 证据（runner/report/截图/日志/备份）均位于 gitignored test-results 或 /tmp，未 track。
- 精确 git add 三文件单提交 push FAIL-LOUD。
