# R66 规范：宝石图鉴缓存 JSON 解析兜底（test-first RED）

来源：t127 R66-B 只读侦察 docs/laiyipao/recon/ROUND66_B_ERROR_PERSIST_RECON.md 候选 A。

## §0 判定与红线
本轮只修「本地缓存 JSON 解析失败导致整页崩溃」这一处展示层健壮性缺口。判据：损坏的 localStorage 值不得让 setup 或 onShow 抛 SyntaxError；必须回退到既有诚实空态。
红线：禁改 engine/replay/hash/score/settle/loot/tokens 与后端；禁改 R64 战报竞态守卫与 R61 表单语义既有产物；禁新增视觉令牌；禁写 git。

## §1 问题描述与根因
文件 laiyipao/miniapp/src/pages/gems/gems.vue:191-203 loadGemGain()：
- :193-199 try 块只包住 localStorage 前缀遍历与 getItem，catch 仅处理该段。
- :200 gemGain.value=normalizeGemGain(raw ? JSON.parse(raw) : null) 中 JSON.parse(raw) 位于 try 之外，无兜底。
- :202 setup 同步调用、:203 onShow 再调用，两处均无 try 保护。
根因：缓存值一旦被外部改写、旧版本写入或截断（例如半写入、手工编辑、跨版本格式变化），JSON.parse 抛 SyntaxError，冒泡后整页 setup 初始化或 onShow 回调中断，模板 :48-59 的诚实空态分支无法渲染。
权威惯例对照：store/game.ts:43-53 readConfigCache 用 try/catch 包住读与 JSON.parse 并在 catch 返回 null；audio/sfx.ts:96-107 initSfx catch 回退默认；helper pages/_shared/r10GemGain.ts:18-41 normalizeGemGain(data:unknown) 对 null/undefined/非法对象安全返回诚实空态。
写入方 bag.vue:892 与 :916 各自 try localStorage.setItem(...) catch 只写合法 JSON，但缓存 key 形状 lyp:r10:gem-gain:<user_id>:<gem_id> 可预测，损坏来源无法在读取侧杜绝，故必须在读取侧兜底。

## §2 修复口径（实施轮照此改，本轮不改业务源码）
把 JSON.parse 纳入 try 作用域；catch 中把解析结果归零并回退到 normalizeGemGain(null) 等价的诚实空态。建议形态（约 3-4 行改动，语义等价即可）：
- 保留 :192 let raw: string | null = null 与 :193 try {。
- 把 :200 的解析移入 try：在 getItem 命中后于 try 内完成 parsed = JSON.parse(raw) 并赋给一个中间变量。
- catch 中同时清零 parsed（例如置 null），使后续 normalizeGemGain(parsed) 走与 null 相同的诚实空态。
- :200 统一为 gemGain.value = normalizeGemGain(parsed)，其中 parsed 保证为已解析对象或 null。
结果：合法缓存照常解析出权威差分；损坏/不可读缓存走既有诚实空态「佩戴收益未上报，先看词条描述」，不虚构收益、不改变 rows 结构。

## §3 写域边界
- 允许：实施轮仅改 laiyipao/miniapp/src/pages/gems/gems.vue 的 loadGemGain 函数体；本轮（规范+契约）只写本 SPEC 与 r66GemCacheGuard.contract.test.ts。
- 禁止：改 helper r10GemGain.ts、改 bag.vue 写入方、改 tokens.css、改任何 engine/replay/hash/score/settle/loot 与后端代码。

## §4 不变量（必须逐字保持）
- normalizeGemGain(null) 返回 available=false、text 含诚实文案、rows=[]、basis 说明需两次服务端快照。
- r10GemGainCacheKey 形状 lyp:r10:gem-gain:<user_id>:<gem_id>；bag.vue:892/:916 两处 setItem 写入方不变。
- gems.vue 模板 :48-59 诚实空态锚点（class r10-gem-gain、gemGain.available 分支、role=status 的 r10-gem-empty）不变。
- 令牌三锚点 tokens.css:14-16 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6；无 --r66- 业务令牌。
- :202 loadGemGain() 与 :203 onShow(() => { loadGemGain() }) 两处调用仍在。

## §5 RED → GREEN 步骤
1. 运行 cd laiyipao/miniapp && pnpm exec vitest run src/styles/r66GemCacheGuard.contract.test.ts --reporter=verbose。
2. 预期 RED：exit=1，it1（JSON.parse 未纳入 try / 无 catch 回退）失败；it2/it3/it4 通过。失败必须来自缺口断言，不得是解析错误、模块缺失或超时。
3. 实施轮按 §2 改 gems.vue 后重跑，预期 GREEN：4/4 通过。

## §6 诚实边界
本轮不启动真浏览器、不虚标崩溃复现；缺口由源码静态判据与既有测试盲区坐实。r10GrowthGaps.contract.test.ts（143 行 7 it）未覆盖损坏 JSON 路径，属于真实测试盲区。

## §7 交接
规范+契约 RED 交付后，由实施轮按 §2 修复 gems.vue 并跑 GREEN；本轮不改业务源码。