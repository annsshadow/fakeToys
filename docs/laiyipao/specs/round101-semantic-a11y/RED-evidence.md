# R101 契约 RED 逐字证据

- 命令：`cd laiyipao/miniapp && npx vitest run src/styles/r101SemanticA11y.contract.test.ts --reporter=verbose > r101-red.log 2>&1; echo RED_EXIT=$?`
- 执行目录：`laiyipao/miniapp`（vitest v4.1.12）
- 结果：`RED_EXIT=1`、`Test Files 1 failed (1)`、`Tests 4 failed | 5 passed (9)`、Duration 203ms。
- 4 failed = 4 处真实语义缺口；5 passed = r61 NODES 守卫（规范轮已完成）+ 4 个不变量全绿；无 PARSE_ERROR / 模块解析错误。
- 说明：首轮 RED 曾因不变量用 `@click` handler 作 needle 误定位 `.rxn-name`（同 handler），已改为 class 稳定锚点后复跑得到下述干净 RED；业务源码始终零改动。

## 逐字 stdout（r101-red.log）

```text

 RUN  v4.1.12 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp

 × src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > index.vue .ann-item 开标签含 :aria-expanded 且与展开态同源 6ms
   → 须补 :aria-expanded="String(expanded.has(a.id))": expected '<view\n          v-for="(a, i) in vis…' to contain ':aria-expanded="String(expanded.has(a…'
 × src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > enemies.vue .resist-toggle 开标签含 :aria-expanded 且与展开态同源 1ms
   → 须补 :aria-expanded="String(expandedEnemy === en.id)": expected '<text class="resist-toggle" role="but…' to contain ':aria-expanded="String(expandedEnemy …'
 × src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > battles.vue .battle-row 开标签含 :aria-expanded 且与展开态同源 1ms
   → 须补 :aria-expanded="String(expandedId === it.id)": expected '<view\n          v-for="it in items"\…' to contain ':aria-expanded="String(expandedId ===…'
 × src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > me.vue nicknameMsg 补 role=status + aria-live=polite（与 passwordMsg 同构） 0ms
   → 须补 role="status": expected '<text\n              v-if="nicknameMs…' to contain 'role="status"'
 ✓ src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > r61 NODES 增补 nicknameMsg 第 7 处语义守卫 0ms
 ✓ src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > 不变量：三处展开控件键盘三件套与原 @click 入口保留 0ms
 ✓ src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > 不变量：R61/R100 契约本体断言保留 0ms
 ✓ src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > 不变量：令牌三锚点不动且业务零 --r101- 1ms
 ✓ src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > 不变量：排除项不被误改（导航/选择态/dialog/link 详情/双入口） 1ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 4 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > index.vue .ann-item 开标签含 :aria-expanded 且与展开态同源
AssertionError: 须补 :aria-expanded="String(expanded.has(a.id))": expected '<view\n          v-for="(a, i) in vis…' to contain ':aria-expanded="String(expanded.has(a…'

- Expected
+ Received

- :aria-expanded="String(expanded.has(a.id))"
+ <view
+           v-for="(a, i) in visibleAnnouncements"
+           :key="a.id"
+           class="ann-item"
+           @click="toggleAnn(a.id)"
+           role="button"
+           tabindex="0"
+           @keydown.enter="toggleAnn(a.id)"
+         >

 ❯ src/styles/r101SemanticA11y.contract.test.ts:42:8
     40|     expect(b, '须能定位 .ann-item 开标签').not.toBeNull()
     41|     expect(b || '', '须补 :aria-expanded="String(expanded.has(a.id))"')
     42|       .toContain(':aria-expanded="String(expanded.has(a.id))"')
       |        ^
     43|     expect(indexSrc, '展开态须与 .ann-toggle 同源'
     44|       ).toContain(':class="{ open: expanded.has(a.id) }"')

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/4]⎯

 FAIL  src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > enemies.vue .resist-toggle 开标签含 :aria-expanded 且与展开态同源
AssertionError: 须补 :aria-expanded="String(expandedEnemy === en.id)": expected '<text class="resist-toggle" role="but…' to contain ':aria-expanded="String(expandedEnemy …'

Expected: ":aria-expanded="String(expandedEnemy === en.id)""
Received: "<text class="resist-toggle" role="button" tabindex="0" @click="toggleEnemy(en.id)" @keydown.enter="toggleEnemy(en.id)">"

 ❯ src/styles/r101SemanticA11y.contract.test.ts:51:8
     49|     expect(b, '须能定位 .resist-toggle 开标签').not.toBeNull()
     50|     expect(b || '', '须补 :aria-expanded="String(expandedEnemy === en.id…
     51|       .toContain(':aria-expanded="String(expandedEnemy === en.id)"')
       |        ^
     52|     expect(enemiesSrc, '展开态须与抗性图文案同源').toContain('expandedEnemy === en…
     53|   })

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/4]⎯

 FAIL  src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > battles.vue .battle-row 开标签含 :aria-expanded 且与展开态同源
AssertionError: 须补 :aria-expanded="String(expandedId === it.id)": expected '<view\n          v-for="it in items"\…' to contain ':aria-expanded="String(expandedId ===…'

- Expected
+ Received

- :aria-expanded="String(expandedId === it.id)"
+ <view
+           v-for="it in items"
+           :key="it.id"
+           class="battle-row tap"
+           :class="[it.result === 'win' ? 'win' : 'lose', { expanded: expandedId === it.id }]"
+           @click="toggleDetail(it)"
+           role="button"
+           tabindex="0"
+           @keydown.enter="toggleDetail(it)"
+         >

 ❯ src/styles/r101SemanticA11y.contract.test.ts:59:8
     57|     expect(b, '须能定位 .battle-row 开标签').not.toBeNull()
     58|     expect(b || '', '须补 :aria-expanded="String(expandedId === it.id)"')
     59|       .toContain(':aria-expanded="String(expandedId === it.id)"')
       |        ^
     60|     expect(battlesSrc, '展开态须与详情 v-if 同源').toContain('v-if="expandedId …
     61|   })

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[3/4]⎯

 FAIL  src/styles/r101SemanticA11y.contract.test.ts > R101 语义可访问性补全契约 > me.vue nicknameMsg 补 role=status + aria-live=polite（与 passwordMsg 同构）
AssertionError: 须补 role="status": expected '<text\n              v-if="nicknameMs…' to contain 'role="status"'

- Expected
+ Received

- role="status"
+ <text
+               v-if="nicknameMsg"
+               class="dim"
+               style="display: block; margin-top: 4rpx"
+               :style="{ color: nicknameErr ? 'var(--bad)' : 'var(--ok)' }"
+             >
+               {{ nicknameMsg }}
+             </text>

 ❯ src/styles/r101SemanticA11y.contract.test.ts:66:41
     64|     const b = tagBefore(meSrc, '{{ nicknameMsg }}', '<text')
     65|     expect(b, '须能定位 nicknameMsg text 开标签').not.toBeNull()
     66|     expect(b || '', '须补 role="status"').toContain('role="status"')
       |                                         ^
     67|     expect(b || '', '须补 aria-live="polite"').toContain('aria-live="pol…
     68|   })

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[4/4]⎯


 Test Files  1 failed (1)
      Tests  4 failed | 5 passed (9)
   Start at  03:31:40
   Duration  203ms (transform 22ms, setup 0ms, import 34ms, tests 12ms, environment 0ms)


```

## 结论

- RED 符合预期：4 个失败与队长裁定的 4 处业务缺口一一对应，非解析/模块错误。
- 5 个通过项证明契约守卫（r61 NODES 增补）与全部不变量在规范轮已就位，实施轮只需补 4 处属性即可转 GREEN。
