# R104 RED 证据（契约先行）

## 执行命令

```bash
cd laiyipao/miniapp
npx vitest run src/styles/r104LoadingA11y.contract.test.ts --reporter=verbose
```

- 实测退出码：`1`（RED 预期，缺口 it 失败）
- vitest 版本：v4.1.12
- 结果：`Test Files 1 failed (1)` / `Tests 10 failed | 3 passed (13)` / `Duration 490ms`
- 失败 it 全为缺口断言（A 类 7 处 role="status" 缺失 + B 类 3 处 aria-hidden="true" 未移除），3 个不变量 it 全通过，无 PARSE_ERROR / 模块加载错误。

## 逐字 stdout（RED-raw.log 同源）

```text

 RUN  v4.1.12 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp

 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > A announcements/announcements.vue 加载根补 status/busy/label 三属性 7ms
   → announcements/announcements.vue 加载根须含 role="status": expected '<view v-if="loading" class="card">' to contain 'role="status"'
 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > A bag/bag.vue 加载根补 status/busy/label 三属性 1ms
   → bag/bag.vue 加载根须含 role="status": expected '<view v-if="loading" class="card">' to contain 'role="status"'
 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > A battles/battles.vue 加载根补 status/busy/label 三属性 1ms
   → battles/battles.vue 加载根须含 role="status": expected '<view v-if="loading" class="card">' to contain 'role="status"'
 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > A mastery/mastery.vue 加载根补 status/busy/label 三属性 1ms
   → mastery/mastery.vue 加载根须含 role="status": expected '<view v-if="loading" class="card">' to contain 'role="status"'
 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > A rank/rank.vue 加载根补 status/busy/label 三属性 1ms
   → rank/rank.vue 加载根须含 role="status": expected '<view v-if="loading" class="card">' to contain 'role="status"'
 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > A shop/shop.vue 加载根补 status/busy/label 三属性 1ms
   → shop/shop.vue 加载根须含 role="status": expected '<view v-if="loading" class="card">' to contain 'role="status"'
 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > A signin/signin.vue 加载根补 status/busy/label 三属性 1ms
   → signin/signin.vue 加载根须含 role="status": expected '<view v-if="loading" class="card">' to contain 'role="status"'
 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > B equipment/equipment.vue 骨架根去 aria-hidden 并补 status/busy/label 三属性 1ms
   → equipment/equipment.vue 骨架根须去掉 aria-hidden="true": expected '<view v-if="r6Loading" class="r6-skel…' not to contain 'aria-hidden="true"'
 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > B reactions/reactions.vue 骨架根去 aria-hidden 并补 status/busy/label 三属性 1ms
   → reactions/reactions.vue 骨架根须去掉 aria-hidden="true": expected '<view v-if="r6Loading" class="r6-skel…' not to contain 'aria-hidden="true"'
 × src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > B skills/skills.vue 骨架根去 aria-hidden 并补 status/busy/label 三属性 1ms
   → skills/skills.vue 骨架根须去掉 aria-hidden="true": expected '<view v-if="r6Loading" class="r6-skel…' not to contain 'aria-hidden="true"'
 ✓ src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > 不变量：令牌三锚点不动且 10 页业务源码零 --r104- 6ms
 ✓ src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > 不变量：R60/R100 键盘三件套抽查（announcements CTA + bag 空态 + enemies resist-toggle）仍在 2ms
 ✓ src/styles/r104LoadingA11y.contract.test.ts > R104 骨架屏加载态 a11y 语义契约 > 不变量：R101 aria-expanded 与既有空态 aria-live/role=status 不被误改 2ms

 ⎯⎯⎯⎯⎯⎯ Failed Tests 10 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  ... A announcements/announcements.vue 加载根补 status/busy/label 三属性
 AssertionError: announcements/announcements.vue 加载根须含 role="status": expected '<view v-if="loading" class="card">' to contain 'role="status"'
 Expected: "role="status""   Received: "<view v-if="loading" class="card">"
 ❯ src/styles/r104LoadingA11y.contract.test.ts:62:54
 FAIL  ... A bag/bag.vue（同上 role="status" 缺失，Received: <view v-if="loading" class="card">，test.ts:62:54）
 FAIL  ... A battles/battles.vue（同上，test.ts:62:54）
 FAIL  ... A mastery/mastery.vue（同上，test.ts:62:54）
 FAIL  ... A rank/rank.vue（同上，test.ts:62:54）
 FAIL  ... A shop/shop.vue（同上，test.ts:62:54）
 FAIL  ... A signin/signin.vue（同上，test.ts:62:54）
 FAIL  ... B equipment/equipment.vue 骨架根须去掉 aria-hidden="true": expected '<view v-if="r6Loading" class="r6-skel…' not to contain 'aria-hidden="true"'
 Expected: "aria-hidden="true""   Received: "<view v-if="r6Loading" class="r6-skeleton r6-enter" aria-hidden="true">"
 ❯ src/styles/r104LoadingA11y.contract.test.ts:72:66
 FAIL  ... B reactions/reactions.vue（同上，test.ts:72:66）
 FAIL  ... B skills/skills.vue（同上，test.ts:72:66）

 Test Files  1 failed (1)
      Tests  10 failed | 3 passed (13)
   Start at  12:27:16
   Duration  490ms (transform 29ms, setup 0ms, import 45ms, tests 27ms, environment 0ms)
```

> 注：完整逐字输出（含每处 FAIL 的 Expected/Received 与代码帧）见同目录 `RED-raw.log`；上面为同源摘录，未改写任何断言文本。

## 失败 it 清单（10）

1. A announcements/announcements.vue 加载根补 status/busy/label 三属性
2. A bag/bag.vue 加载根补 status/busy/label 三属性
3. A battles/battles.vue 加载根补 status/busy/label 三属性
4. A mastery/mastery.vue 加载根补 status/busy/label 三属性
5. A rank/rank.vue 加载根补 status/busy/label 三属性
6. A shop/shop.vue 加载根补 status/busy/label 三属性
7. A signin/signin.vue 加载根补 status/busy/label 三属性
8. B equipment/equipment.vue 骨架根去 aria-hidden 并补 status/busy/label 三属性
9. B reactions/reactions.vue 骨架根去 aria-hidden 并补 status/busy/label 三属性
10. B skills/skills.vue 骨架根去 aria-hidden 并补 status/busy/label 三属性

## 通过 it（3 不变量）

- 令牌三锚点不动且 10 页业务源码零 --r104-
- R60/R100 键盘三件套抽查（announcements CTA + bag 空态 + enemies resist-toggle）仍在
- R101 aria-expanded 与既有空态 aria-live/role=status 不被误改
