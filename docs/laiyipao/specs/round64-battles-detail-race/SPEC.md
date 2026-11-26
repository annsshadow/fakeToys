# R64 战报详情辅助面板竞态守卫规范（test-first RED）

## 0. 判定与范围

- 目标：为 R63 候选 A1 立修复规范；本轮只写规范与契约，不改业务源码、不 git 写、零真值、无 --r64- 业务令牌。
- 唯一改动对象（实施轮）：laiyipao/miniapp/src/pages/battles/battles.vue 脚本层，约 4-6 行。
- 红线：不动 details 缓存、:472 computed 主详情、:827 幂等、applyAdviceAction 协议、engine/replay/hash/score/settle/loot/tokens 与后端。

## 1. 根因坐实（源码铁证）

- growthCompare：:491 `const growthCompare = ref(...)`，写入 :504 `growthCompare.value = baseline ? ... : ...`；写入前经两次 await（:497 `await api.myBattleHistory(20)`、:500 `await api.myBattleDetail(priorId)`）。
- r14Advice：:761 `const r14Advice = ref(null)`，写入 :769（空态清除）与 :771 `r14Advice.value = await fetchAdviceAction(...)`。
- 触发链：:816-823 `toggleDetail` 先置 `expandedId.value = it.id` 再 `await loadDetail(it)`；:826-839 `loadDetail` 中 :832 `await loadGrowthCompare(d)`、:833 `void loadR14Advice(d)`；:827 仅按 `details.value[it.id]` 幂等（同局一次），不同战报之间无互斥。
- 主详情无竞态（排除误报）：:472-474 `const detail = computed(() => expandedId.value ? details.value[expandedId.value] : undefined)` — 按 id 缓存，与主详情一一对应。问题只在两个辅助面板的单一 ref。
- 同页已有同构 guard 范式：:429-438 `reload` 的 `const seq = ++listReqSeq` / `if (seq !== listReqSeq) return`；:903-919 `loadMore` 同款（注释「筛选已切换/已有更新的 reload，丢弃陈旧响应」）。

## 2. 触发序列与后果

- 序列：先展开战报甲（未缓存）→ :830 取详情、:831 写 details[甲]、:832 进入 loadGrowthCompare 停在 :497/:500 的 await；此时点击战报乙（另一 id、未缓存）→ expandedId=乙、loadDetail(乙) 并行并先完成 :504/:771 写入；随后甲的 :504/:771 晚到覆盖 → 当前展开乙详情（computed 正确），但「AI · 养成前后」（:296-311）与「AI · 可执行建议」（:276-288）显示甲的数据。
- 后果升级：:282 CTA「一键替换为 {{ r14Advice.candidateName }} →」→ `r14OpenCompare()`（:774-779）→ `r14ConfirmApply()`（:786-796）依 r14Advice 调 `applyAdviceAction` 改出战配置；错局陈旧建议可被应用到当前构筑。

## 3. 修复口径（仅展示层）

1. 新增模块级序号变量（如 `let detailPanelSeq = 0`），与当前展开目标绑定。
2. `loadDetail` 开头（首个 await 之前）取号 `const seq = ++detailPanelSeq`，并把 seq 传入 `loadGrowthCompare(d, seq)` 与 `loadR14Advice(d, seq)`。
3. `loadGrowthCompare` 在 :504 写 `growthCompare.value` 前、`loadR14Advice` 在 :769/:771 写 `r14Advice.value` 前，校验 `if (seq !== detailPanelSeq) return`（或等价形式），过期直接丢弃，不写 ref。
4. 保持 :827 按 id 幂等、:831 details 缓存、:838 按 id 清标记、:472 computed 主详情逐字不动。
5. 变量名不限定（如 panelSeq/detailSeq 亦可），但必须可辨识为「序号」且写 ref 前校验相等。

## 4. 保护面与不变量

- details 按 id 缓存 `details.value[it.id] = d` 保留；computed `detail` 读 `details[expandedId.value]` 保留。
- `applyAdviceAction(a, store)` 调用保留，CTA 文案与路由不改。
- 令牌三锚点 lyp-tokens-v1 / v3-r3 / r6 不变；无 --r64- 样式令牌。
- 不碰 engine/replay/hash/score/settle/loot/tokens 与后端；零真值。

## 5. 契约与 RED 门禁

- 契约 laiyipao/miniapp/src/styles/r64BattlesRace.contract.test.ts：真实 readFileSync 读 battles.vue，结构断言，无内嵌实现副本。
- 5 it：①growthCompare.value 写入前有序号守卫；②r14Advice.value 主写入前有序号守卫；③loadDetail 开头首个 await 前有序号递增；④不变量 details 缓存 + computed detail；⑤不变量 applyAdviceAction 调用。
- 期望 RED：修复前 ①②③ 失败（缺口断言），④⑤ 通过（不变量）；无解析/模块/超时错误。

## 6. RED 实测记录

- 命令：cd laiyipao/miniapp && npx vitest run src/styles/r64BattlesRace.contract.test.ts --reporter=verbose 2>&1
- 结果：exit=1，RUN v4.1.12，Test Files 1 failed (1)，Tests 3 failed | 2 passed (5)，Duration 338ms（transform 28ms，tests 10ms）。
- 三条失败全部为缺口断言（逐字）：
  1. :56「growthCompare.value 写入前须校验展开序号相等才写入，过期直接丢弃（同页范式 seq !== listReqSeq）; 源码写入点前未见序号守卫: expected false to be true」
  2. :67「r14Advice.value 主写入前须校验展开序号相等才写入，过期直接丢弃; 源码写入点前未见序号守卫: expected false to be true」
  3. :77「loadDetail 开头（首个 await 前）须递增序号（如 const seq = ++detailPanelSeq）并传入两个辅助加载函数; 源码开头未见序号递增: expected false to be true」
- 两条通过（不变量）：details 按 id 缓存 + computed detail 绑定保留；applyAdviceAction 权威调用保留。
- 无 Transform failed / 模块缺失 / 超时错误；RED 完全来自三处真实缺口，无假阳性。

## 7. 交接

- 实施轮按第 3 节加序号守卫，跑契约 GREEN 后交真浏览器复查（快速切换两场未缓存战报确认面板不再错配）。