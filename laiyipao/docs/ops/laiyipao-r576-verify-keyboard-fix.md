# R576 laiyipao 验真页 verify.vue 4 类裸 @click 伪按钮键盘可达修复

## 1. 背景与判定
R575 甄别判可达真缺陷 1 项 P2：\`laiyipao/miniapp/src/pages/verify/verify.vue\` 全文件仅 4 处裸 \`@click\`（取复现信息 / 本地重放并提交验真 / 提交伪造哈希 / 验真记录回填行），键盘完全不可达且 AX 为 generic / name=null。首页一跳可达、零等价入口，与已修七页零重叠。适用 WCAG 2.1 SC 2.1.1 (A) 关联 SC 4.1.2 (A)。

## 2. 方案取舍（Fail Loud）
采用方案 B 逐页显式补，沿用 R561–R575 同族范式：每处开标签补 \`role="button"\` + \`tabindex="0"\` + 静态或动态 \`aria-label\` + \`@keydown.enter\` / \`@keydown.space.prevent\` 复用原 handler。全局增强器方案因越界他页 Fail Loud 排除。外观 class（btn-disabled / falsifying 等）不替代可访问语义，甄别已证 link-off 类无 disabled 语义仍需键盘可达，本次 4 类一律补齐。4 条 \`@click\` 全保留，业务 handler 函数体 / api / 样式 / 他页 / main.ts 零改。

## 3. 写域精确 3 文件
- M \`laiyipao/miniapp/src/pages/verify/verify.vue\`：4 处开标签纯 a11y 属性追加（16 insertions / 1 deletion，末处单行整行替换）。
- A \`laiyipao/miniapp/src/pages/R576.verifyKeyboardA11y.contract.test.ts\`：9 用例改为 8 用例（4 核心 + 4 守卫；详见 §5）。
- A \`laiyipao/docs/ops/laiyipao-r576-verify-keyboard-fix.md\`：本文档，脱敏 0 真值。

修复后 \`verify.vue\` sha256 = \`f475d9c8722393cf5d384db1d66f53c3e7d95bc1872b3cf53a0638ea711b7c53\`，真父 sha256 = \`b6e8dba0050b7d4d4dfd36f8ffa8e1aacb4d0158b46ba9bf7c8c46eaee9810c7\`，真父 HEAD = \`987157e47b233a4d86aba3a382641cecad611f86\`。

## 4. 4 类修复明细
| # | 锚点 | handler | aria-label |
|---|---|---|---|
| 1 | view.btn 取复现信息（:47-59，多行开标签） | fetchReplay | :aria-label="fetching ? '获取中' : '取复现信息'" |
| 2 | view.btn.btn-primary 本地重放并提交验真（:130 单行开标签） | doReplay | aria-label="本地重放并提交验真" |
| 3 | view.btn 提交伪造哈希（:134 附近，多行开标签） | doFalsify | :aria-label="falsifying ? '提交中' : '提交伪造哈希'" |
| 4 | view.kv.vh-row.tap 验真记录回填（v-for verifyHistory） | refillBattleId(h.id) | :aria-label="'回填战报 #' + h.id + (h.matched ? '一致' : '不一致') + '，' + h.created_at" |

每处均追加 \`@keydown.enter="handler"\` 与 \`@keydown.space.prevent="handler"\`，\`@click\` 原样保留。

## 5. 契约 RED → GREEN
- \`R576.verifyKeyboardA11y.contract.test.ts\` 共 8 用例：4 核心（每类 role/tabindex/aria-label/keydown 复用 + @click 保留）+ 4 守卫（handler 函数体零改 / 数据源样式 main.ts 零改无全局增强器 / 无 tablist 语义且 @click 恰 4 处 / 无原生 button 标签）。
- 踩坑（Fail Loud）：初版守卫 8 含「role=button 数量恒 4」断言，真父 RED 时守卫亦 fail（4fail+4pass 变成 5fail+3pass），违反「核心 fail 守卫 pass」纪律；已将数量断言并入核心 4，守卫 8 改为纯结构守卫。
- RED replay：真父源码覆盖 → EXIT=1，4 failed | 4 passed（4 核心 fail / 4 守卫 pass）；恢复修复版 sha256 复核一致，FINAL HEAD 未变。GREEN：EXIT=0，8 passed。

## 6. 三 verify（cd laiyipao/miniapp，全 exit 0）
1. \`pnpm vitest run --config vitest.config.ts src/pages/R576.verifyKeyboardA11y.contract.test.ts\` → 8 passed
2. \`pnpm test\` → 165 files passed | 1 skipped (166)，1596 tests passed | 2 skipped (1598)
3. \`pnpm run type-check\`（vue-tsc --noEmit -p tsconfig.json）→ 无错

## 7. 真浏览器 CDP 决定性判据
自起 laiyipao miniapp H5（dev:h5，实际端口以探测为准，占位 \`<TEST_HOST>:<PORT>\`），Playwright chromium headless 390x844 DPR2。runner：\`oa4rust/test-results/r576/r576-e2e.cjs\`；报告 \`oa4rust/test-results/r576/r576-report.json\`。cases=6，fatals=0，pageErrors=0。

判据实现：CDP \`DOM.getAttributes\` + \`Accessibility.getPartialAXTree\` / \`getFullAXTree\`，按 backendDOMNodeId 映射读 role/name；\`document.activeElement\` 读 \`getFullAXTree focused=true\` 非 RootWebArea 节点；\`UNI-VIEW\` 身份独立确认，严禁宿主 wrapper 假阳性（R535 教训）。

### 7.1 静态判据（withData / empty × light / dark）
- 4 类（withData）focused：tag=UNI-VIEW，isTarget=true，isBody=false；AX full role=button，name=取复现信息 / 本地重放并提交验真 / 提交伪造哈希 / 回填战报 #575，不一致，<时间>；partial 与 full 双通道一致，ignored=false。
- DOM 属性：role="button" / tabindex="0" / aria-label=对应文案（动态回填行含 battle_id + 一致/不一致 + 时间）。
- Tab 命中：withData fetch/replay/falsify/history 各 5（30 次 Tab 内）；empty 仅 fetch 命中 10（其余 3 类随 replayInfo 不存在，据实披露）。
- empty 态：replayCard=false / historyCount=0 / battleId=""（零造数态仅常驻取复现信息按钮）。

### 7.2 功能真命中（func case）
- 注入对照：stripA11y 后 role=null / AX generic / name=null；restore 后 role=button / name=取复现信息 全恢复；4 类注入 aria=R576-inj-* 且 AX full role=button（自包含验 AX 通路）。
- fetch Enter（空 battle_id）：errMsg="请输入有效的 battle_id" + input-error=true + shake 类，:51 三重反馈真命中。
- replay Enter：clamp 显示"无法重放：构筑快照里没有任何已装备技能（全部 slot < 0），无法重放"——stub REPLAY 的 build.skills 为空触发业务守卫，证明 handler 确被键盘触发（据实披露：未产生 verifyCalls 是业务前置校验所致，非键盘通路缺陷）。
- falsify Space：verifyCalls delta=1，verdicts=验真失败 — 分数被判定为不可复现，POST /battle/verify 真命中。
- history Enter：replayCard 保持 true，battleId=575，回填通路可达（该条已是当前 battleId 故视觉无变化，据实披露）。
- 正面对照（Fail Loud 披露）：本轮用被测页自包含注入原生 \`<button aria-label="R576正面对照控件">\`，AX full role=button / name 达标，等价验证辅助技术通路正常。未跨页另起 signin/mastery（R575 模板的跨页对照在该 H5 单页上下文不便复用）；该取舍不影响被测 4 类判据的独立性。
- 鼠标零回归（mouse case）：点击 replay → falsify，verifyCalls=["verify","verify"]，状态与键盘路径一致。

## 8. 截图
- \`r576-withData-light.png\` sha256=da0f9f4cb8ff83cbd8076341c80f1864e7152d45d99cd9f413e1f60f3572c7e7（780x2690）
- \`r576-withData-dark.png\` sha256=a47eb11a148edbd67444edc371b639e18cdde39c3f88aab44d8bfac693efbd56（780x2690）
- \`r576-empty-light.png\` sha256=ceb203b643c8dcad0a529909797195db7f195a67be4a247bca9cbcb41963a485（780x1776）
- \`r576-empty-dark.png\` sha256=ceb203b643c8dcad0a529909797195db7f195a67be4a247bca9cbcb41963a485（780x1776，与 light 同 sha）
4 图已 \`read_image\` 目视名实相符：withData 显示取复现信息 / 复现信息卡 / 本地重放 / 提交伪造哈希 / 服务端裁定（验真失败）/ 验真记录；empty 显示战绩验真说明 / 输入 battle_id / 取复现信息按钮。H5 无主题切换机制，light/dark 视觉一致，sha 微差来自时间戳等动态文本，据实披露。

## 9. sha256 证据表
### A 组：runner 紧凑 JSON（crypto.sha256(JSON.stringify(report))）
- E2E 汇总 \`R576_DONE\` = \`0eaa6f099e9e5661fde6b2ef99af1bbd872b14e2e6e7086838bc79a421a908be\`

### B 组：文件 sha256sum（逐项命令行核对）
| 文件 | sha256 |
|---|---|
| laiyipao/miniapp/src/pages/verify/verify.vue | f475d9c8722393cf5d384db1d66f53c3e7d95bc1872b3cf53a0638ea711b7c53 |
| laiyipao/miniapp/src/pages/R576.verifyKeyboardA11y.contract.test.ts | 252453816ccdf4ec5e69722b30eff85d8a6eed1d05dd150a1af9c6948bf72451 |
| oa4rust/test-results/r576/r576-e2e.cjs | 5c475a8e4b06787be32a893baa85885869f0b4ad023af7086ea32db816ea07dc |
| oa4rust/test-results/r576/r576-summary.cjs | 69f0391c095aad1a5b66d00abcef7af7cb2b455fe9fc3ff71510e03aa013dc04 |
| oa4rust/test-results/r576/r576-report.json | 155f624ddc7ef38daa12645eb7f924509dcaf55d7872c45d67ca52f0beff1d15 |
| oa4rust/test-results/r576/r576-withData-light.png | da0f9f4cb8ff83cbd8076341c80f1864e7152d45d99cd9f413e1f60f3572c7e7 |
| oa4rust/test-results/r576/r576-withData-dark.png | a47eb11a148edbd67444edc371b639e18cdde39c3f88aab44d8bfac693efbd56 |
| oa4rust/test-results/r576/r576-empty-light.png | ceb203b643c8dcad0a529909797195db7f195a67be4a247bca9cbcb41963a485 |
| oa4rust/test-results/r576/r576-empty-dark.png | ceb203b643c8dcad0a529909797195db7f195a67be4a247bca9cbcb41963a485 |
| oa4rust/test-results/r576/verify-parent.bak.vue | b6e8dba0050b7d4d4dfd36f8ffa8e1aacb4d0158b46ba9bf7c8c46eaee9810c7 |
| oa4rust/test-results/r576/verify-fixed.bak.vue | f475d9c8722393cf5d384db1d66f53c3e7d95bc1872b3cf53a0638ea711b7c53 |
| /tmp/r576-green.log | f5efaa204505b1046e7914aa12dc493c3a6164d96e89cf0d2c25706b977eaca9 |
| /tmp/r576-red.log | 74ff4b7a76d89bb2245d37ddeb2d45a3b2ceddc2b97eb25802594b51ffb41cd9 |
| /tmp/r576-v1.log | 1802182df2c5529ee9dd65b0ba6c0e18154b0364969c73cff31ae91c44059568 |
| /tmp/r576-v2.log | ea036704fc6086fca73212825df4b992fe5dfc8949d36078f6b7f03fe9c0043d |
| /tmp/r576-v3.log | 37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f |
| /tmp/r576-e2e.log | 1187807810aabd18439ab3e6d5668798194fd560a75e7aceec540f200770bbab |
| /tmp/r576-summary.log | 840d85cc6f6937c892233cb644c5515db7323bdbf52ffd0664e4cb7b2901b417 |

## 10. 脱敏
本文件与证据均仅使用 \`<TEST_HOST>:<PORT>\` 占位符，0 真值（无域名 / IP / token / 个人路径）。证据全部在 gitignored \`oa4rust/test-results/r576/\`，不入库。
