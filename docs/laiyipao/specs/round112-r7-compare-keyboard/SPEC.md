# R112 规范：R7 对比弹层键盘闭环（test-first RED）

## §0 立轮依据

- 侦察 A 报告：docs/laiyipao/recon/ROUND112_A_INTERACTION_RECON.md（四条件全中，判立候选 1 项）。
- 基线 HEAD=5bc72e4c012494aaa843f3bb501ad3b2603195db（未提交，全程未变）。
- 权威唯一真源：docs/laiyipao/design/AI_VISUAL_SYSTEM.md:120/825/831/929（dialog + aria-modal + Esc + 焦点陷阱 + 焦点归还）；同仓正面范式 r73LegendFocus.contract.test.ts 与 r103TimelineKeyboard.contract.test.ts；无反向权威。
- 独立对象：R100/R102/R109 是裸 view CTA 三件套、R73/R103 是 battle.vue 两特定弹层、R107 是表单错误语义；本轮是 skills/equipment 的 R7「写入确认」弹层，不重复。

## §1 缺陷陈述

- skills.vue:264-288 / equipment.vue:226-250 的外层 `<view v-if="r7CompareOpen" class="r7-compare-mask" @click="r7CompareOpen = false">` 无 role="dialog"、aria-modal、aria-label、ref、@keydown.esc、@keydown.tab。
- r7OpenCompare()（skills:630-638 / equipment:532-540）仅设 r7CompareOpen.value=true，不记录触发源、不 nextTick 移入焦点。
- 关闭为内联 r7CompareOpen=false（遮罩/取消/完成），无统一 close、无焦点归还。
- r7ImpactApply.contract.test.ts:165-174 假守卫仅断言 common.css 含 .r7-compare-mask 类名，不查语义/键盘闭环——令牌样式存在即绿。
- 真 Chromium 6 组合（2 页×390/768/1280）坐实：弹层出现但 role=null、aria-modal=null、Escape 后弹层不关、activeElement=BODY、AX 无 dialog 节点。

## §2 修复目标（供实施轮，本轮只写契约+SPEC）

两页结构对称，各六处（照 R73/R103 口径）：
1. mask 开标签补 role="dialog" aria-modal="true" aria-label="当前 vs 推荐·确认替换" ref="r7CompareRef" @keydown.esc="closeR7Compare" @keydown.tab="trapR7CompareTab"，保留 v-if；遮罩 @click 改调 closeR7Compare。
2. setup 增 r7CompareRef / r7TriggerEl 两个 ref(null)。
3. r7OpenCompare 记录 document.activeElement 到 r7TriggerEl，nextTick 后 focus 弹层根 view（r7CompareRef.value）。
4. 新增 closeR7Compare()：r7CompareOpen.value=false 后 focus 归还 r7TriggerEl.value。
5. 遮罩/取消/完成三元素的 @click 与 @keydown.enter 全部改调 closeR7Compare（模板不留内联 r7CompareOpen = false）。
6. 新增 trapR7CompareTab(e)，复刻 R103 trapTimelineTab：以 r7CompareRef.value 取根、querySelectorAll 查 [tabindex="0"]/[role="button"] 首尾循环。
不变量：两步确认（open 只预览、confirm 才写入 persistLoadout/applyEquipmentImpact）、写入失败保留原配置+「请重试」、令牌三锚点 tokens.css:14-16、零 --r112-、R73/R103 闭环不动、内层 @click.stop 保留。

## §3 焦点可行性边界（队长裁定，务必落契约与 QA）

- 触发 CTA 与弹层按钮均为 text（UNI-TEXT），H5 下 focus() 不生效（R103 已坐实）。
- 焦点移入优先 focus 弹层根 view（UNI-VIEW 可聚焦，真浏览器可坐实）——故契约 G3 锁定 r7CompareRef.value 而非 close 按钮。
- 焦点归还目标 CTA 是 text，若落 BODY，按 R103 先例由契约静态守源码 closeR7Compare 含 focus 归还调用；QA 须如实披露，不冒充真浏览器 PASS。
- dialog 语义 / ESC 关闭 / Tab 陷阱属 DOM 层，必须真浏览器坐实，不以静态通过替代。

## §4 契约结构（r112R7CompareKeyboard.contract.test.ts，18 it）

- 真实 fs.readFileSync 读 skills.vue / equipment.vue / tokens.css / r7ImpactApply.contract.test.ts / battle.vue；tagBefore 按唯一 class 锚点、blockAfter 按函数名括号配对，均不依赖行号。
- 12 缺口 it（两页各 6，改前必须失败）：G1 mask dialog+ref+esc+tab；G2 两 setup ref；G3 open 触发源记录+nextTick+focus 根 view；G4 close 存在且 focus 归还；G5 模板内联关闭统一改调；G6 trapR7CompareTab 存在。
- 6 不变量 it（改前全通过）：I1 两步确认；I2 写入失败保留原配置+请重试；I3 令牌三锚点+零 --r112-；I4 R73/R103 闭环不动；I5 r7ImpactApply 假守卫保留记录且语义检查由本契约补齐；I6 mask v-if 与内层 @click.stop 保留。

## §5 RED 验证口径

- 命令：cd laiyipao/miniapp && npx vitest run src/styles/r112R7CompareKeyboard.contract.test.ts --reporter=verbose
- RED 预期：exit 1，12 failed | 6 passed (18)；12 失败全为真实缺口，6 不变量全绿，无 PARSE_ERROR。
- GREEN 预期（实施轮）：exit 0，18 passed (18)。
- 逐字日志 docs/laiyipao/specs/round112-r7-compare-keyboard/RED-raw.log；摘要 RED-evidence.md。

## §6 写域与 hygiene

- 唯一写域：docs/laiyipao/specs/round112-r7-compare-keyboard/{SPEC.md,RED-evidence.md,RED-raw.log} + laiyipao/miniapp/src/styles/r112R7CompareKeyboard.contract.test.ts。
- 禁改业务 vue/.gitignore/令牌/既有契约/admin；零真值；HEAD 未变；自启 job 回收。
