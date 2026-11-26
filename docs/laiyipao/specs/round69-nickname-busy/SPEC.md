# R69 规范：昵称提交在途反馈与防连点（test-first RED）

来源：t135 R68-A 操作反馈三态与重复提交防护只读侦察 docs/laiyipao/recon/ROUND68_A_FEEDBACK_RECON.md 候选 A。

## §0 判定与红线
本轮只修「修改昵称」这一处展示层操作反馈缺口：在途必须有可见 loading、保存按钮与输入框必须禁重复提交、请求结束后必须复位。判据：慢网点击后 1 秒内按钮显示「提交中…」且无第二个可点保存入口；连续点击只发一次 PUT /me/nickname；失败时编辑态保留、错误信息可见且按钮复位可重试。
红线：禁改 engine/replay/hash/score/settle/loot/tokens 与后端；禁改 api.updateNickname 与 store.refreshProfile 语义；禁动 R61 无障碍、R64 竞态、R66 缓存兜底既有产物；禁新增视觉令牌或 --r69- 业务标记；禁写 git；本轮不改任何业务源码。

## §1 问题描述与根因
文件 laiyipao/miniapp/src/pages/me/me.vue:1244-1270 submitNickname()：
- :1245-1261 仅有空昵称与码点 64 长度校验（该校验正确，须保留）。
- :1262-1269 try/catch 中 await api.updateNickname(name) 与 await store.refreshProfile()，catch 置 nicknameErr/nicknameMsg，但全函数无 busy ref、无入口 if 守卫、无 finally 复位。
- 模板 :96 `<view class="btn btn-mini btn-primary" @click="submitNickname">保存</view>` 无 :class 禁用态、无「提交中…」条件文案；输入框 :86-95 只有 v-model、v-aria-native、aria-label，无 :disabled。
R68-A 真 Chromium 证据：慢网 XHR patch 延迟 3000ms，点击后 1 秒 saveLabel=[保存,保存当前构筑为防线]、spinners=0、disabledBtns=0、errText=false；正常连点两次 PUT_COUNT=2（相同昵称，幂等但多余）；失败 route abort 有「无法连接服务器：request:fail」且编辑态保留可重试。
根因：与其它 75 处已收口异步点不同，该函数缺少共享的在途状态与 finally 复位；用户看不到在途反馈，也无法靠 UI 阻止连点。

## §2 修复口径（实施轮照此改，本轮不改业务源码）
采用同页 submitPassword（me.vue:1135-1158）既有范式，最小 4-6 行改动：
- 在 ref 声明区（me.vue:1226-1229 附近）新增 `const nicknameSubmitting = ref(false)`。
- submitNickname 开头（首个 await 前）若 `nicknameSubmitting.value` 为 true 立即 return；保留 trim/码点 64 校验，校验通过后在 try 前置 `nicknameSubmitting.value = true`。
- try 成功分支保持 api.updateNickname → store.refreshProfile → editingNickname=false；catch 保持 nicknameErr/nicknameMsg 展示；新增 finally { nicknameSubmitting.value = false } 保证成功与失败都复位。
- 模板保存按钮 :96 改为 `:class="{ btn-disabled: nicknameSubmitting }"`，文案改为 `{{ nicknameSubmitting ? '提交中…' : '保存' }}`；昵称输入框 :86-95 加 `:disabled="nicknameSubmitting"`。
- 入口 @confirm 与 @click 均指向同一 submitNickname，守卫覆盖两条路径；不新增第三套文案或令牌。
结果：慢网在途可见「提交中…」且按钮与输入框不可再触发；成功退出编辑态；失败保留编辑态、错误可见、按钮复位可重试。

## §3 写域边界
- 本轮（规范+契约）只写本 SPEC 与 laiyipao/miniapp/src/styles/r69NicknameBusy.contract.test.ts。
- 实施轮仅改 laiyipao/miniapp/src/pages/me/me.vue 的 ref 声明区、submitNickname 函数体与昵称编辑模板块。
- 禁止改 api/client.ts、store、helper、tokens.css、其它页面与后端代码。

## §4 不变量（必须逐字保持）
- 校验语义不变：`nicknameDraft.value.trim()`、`[...name].length > 64`、空值/超长文案「昵称不能为空」「昵称最长 64 个字符」不变。
- 权威调用不变：`await api.updateNickname(name)`、`await store.refreshProfile()`、成功 `editingNickname.value = false`、catch 的 `nicknameErr.value = true` 与 `nicknameMsg.value = (e as Error)?.message || '修改失败，请稍后再试'` 不变。
- 正面对照范式不变：submitPassword 的 passwordSubmitting 守卫与模板 :593-599 的 :class/:disabled/「提交中…」写法保持原样；R61 的 v-aria-native/aria-label 保持。
- 模板结构不变：startEditNickname/cancelEditNickname/@confirm/@click 均仍指向既有函数；取消按钮不受在途守卫阻塞之外的改动。
- 令牌三锚点 styles/tokens.css:14-16 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6；无 --r69- 业务令牌。

## §5 RED → GREEN 步骤
1. 运行 `cd laiyipao/miniapp && pnpm exec vitest run src/styles/r69NicknameBusy.contract.test.ts --reporter=verbose`。
2. 预期 RED：exit=1；it1（busy ref / if 守卫 / finally）、it2（保存按钮禁用态与「提交中…」文案）、it3（输入框 :disabled）失败；it4 不变量与 it5 入口绑定通过。失败必须来自缺口断言，不得是解析错误、模块缺失或超时。
3. 实施轮按 §2 改 me.vue 后重跑，预期 GREEN：5/5 通过。

## §6 诚实边界
本轮不启动真浏览器、不重跑 PUT 计数；R68-A 的三视口探针数据为历史实测证据，规范仅静态引用，不伪造新截图。缺口由源码静态判据与既有实测报告共同坐实。

## §7 交接
规范+契约 RED 交付后，由实施轮按 §2 修复 me.vue 并跑 GREEN，再由 QA 真浏览器按慢网/失败/连点三条件复验。本轮不改业务源码。