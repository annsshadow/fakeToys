# R100 RED 证据（逐字 stdout）

- 命令：`cd laiyipao/miniapp && npx vitest run src/styles/r100ClickableView.contract.test.ts --reporter=verbose`
- 退出码：exit=1（vitest 1 failed）；wrapper RED_RC=1
- 摘要：Test Files 1 failed (1)；Tests 8 failed | 4 passed (12)；v4.1.12；Duration 256ms
- 说明：8 失败均为真实缺口（断 `role="button"`），4 通过为不变量（@click 保留、令牌三锚点、零 --r100-、排除项防误改）。无 PARSE_ERROR / 模块解析错误。

```text

 RUN  v4.1.12 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp

 × src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > battles 战报展开行 须 role=button + tabindex=0 + @keydown.enter 同名 handler 5ms
   → 须补 role="button": expected '<view\n          v-for="it in items"\…' to contain 'role="button"'
 × src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 公告展开行 须 role=button + tabindex=0 + @keydown.enter 同名 handler 1ms
   → 须补 role="button": expected '<view\n          v-for="(a, i) in vis…' to contain 'role="button"'
 × src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > enemies 抗性图展开 须 role=button + tabindex=0 + @keydown.enter 同名 handler 1ms
   → 须补 role="button": expected '<text class="resist-toggle" @click="t…' to contain 'role="button"'
 × src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 构筑卡片 须 role=button + tabindex=0 + @keydown.enter 同名 handler 0ms
   → 须补 role="button": expected '<view class="action-card card-hero ta…' to contain 'role="button"'
 × src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 玩法/图鉴卡片 须 role=button + tabindex=0 + @keydown.enter 同名 handler 0ms
   → 须补 role="button": expected '<view v-for="m in playMenus" :key="m.…' to contain 'role="button"'
 × src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 边际收益行 须 role=button + tabindex=0 + @keydown.enter 同名 handler 0ms
   → 须补 role="button": expected '<view\n          v-for="(m, i) in mar…' to contain 'role="button"'
 × src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 元素 chip 须 role=button + tabindex=0 + @keydown.enter 同名 handler 0ms
   → 须补 role="button": expected '<view\n            v-for="e in elemen…' to contain 'role="button"'
 × src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > stage 章节切换 chip 须 role=button + tabindex=0 + @keydown.enter 同名 handler 0ms
   → 须补 role="button": expected '<view\n            v-for="ch in chapt…' to contain 'role="button"'
 ✓ src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > 8 处原 @click 入口全部保留（不变量） 0ms
 ✓ src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > 令牌三锚点不动且无 --r100-（不变量） 0ms
 ✓ src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > 业务源码零 --r100- 令牌（不变量） 0ms
 ✓ src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > 排除项不被误改：mastery .node 锁定态 + enemies .rxn-name 防双入口（不变量） 1ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 8 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > battles 战报展开行 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<view\n          v-for="it in items"\…' to contain 'role="button"'

- Expected
+ Received

- role="button"
+ <view
+           v-for="it in items"
+           :key="it.id"
+           class="battle-row tap"
+           :class="[it.result === 'win' ? 'win' : 'lose', { expanded: expandedId === it.id }]"
+           @click="toggleDetail(it)"
+         >

 ❯ src/styles/r100ClickableView.contract.test.ts:72:39
     70|       expect(blocks.length, '必须能定位 ' + c.file + ' 的 ' + c.cls + ' / ' …
     71|       for (const b of blocks) {
     72|         expect(b, '须补 role="button"').toContain('role="button"')
       |                                       ^
     73|         expect(b, '须补 tabindex="0"').toContain('tabindex="0"')
     74|         expect(b, '须补 @keydown.enter="' + c.handler + '"（同名 handler）')

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/8]⎯

 FAIL  src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 公告展开行 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<view\n          v-for="(a, i) in vis…' to contain 'role="button"'

- Expected
+ Received

- role="button"
+ <view
+           v-for="(a, i) in visibleAnnouncements"
+           :key="a.id"
+           class="ann-item"
+           @click="toggleAnn(a.id)"
+         >

 ❯ src/styles/r100ClickableView.contract.test.ts:72:39
     70|       expect(blocks.length, '必须能定位 ' + c.file + ' 的 ' + c.cls + ' / ' …
     71|       for (const b of blocks) {
     72|         expect(b, '须补 role="button"').toContain('role="button"')
       |                                       ^
     73|         expect(b, '须补 tabindex="0"').toContain('tabindex="0"')
     74|         expect(b, '须补 @keydown.enter="' + c.handler + '"（同名 handler）')

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/8]⎯

 FAIL  src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > enemies 抗性图展开 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<text class="resist-toggle" @click="t…' to contain 'role="button"'

Expected: "role="button""
Received: "<text class="resist-toggle" @click="toggleEnemy(en.id)">"

 ❯ src/styles/r100ClickableView.contract.test.ts:72:39
     70|       expect(blocks.length, '必须能定位 ' + c.file + ' 的 ' + c.cls + ' / ' …
     71|       for (const b of blocks) {
     72|         expect(b, '须补 role="button"').toContain('role="button"')
       |                                       ^
     73|         expect(b, '须补 tabindex="0"').toContain('tabindex="0"')
     74|         expect(b, '须补 @keydown.enter="' + c.handler + '"（同名 handler）')

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[3/8]⎯

 FAIL  src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 构筑卡片 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<view class="action-card card-hero ta…' to contain 'role="button"'

Expected: "role="button""
Received: "<view class="action-card card-hero tap" @click="goStage">"

 ❯ src/styles/r100ClickableView.contract.test.ts:72:39
     70|       expect(blocks.length, '必须能定位 ' + c.file + ' 的 ' + c.cls + ' / ' …
     71|       for (const b of blocks) {
     72|         expect(b, '须补 role="button"').toContain('role="button"')
       |                                       ^
     73|         expect(b, '须补 tabindex="0"').toContain('tabindex="0"')
     74|         expect(b, '须补 @keydown.enter="' + c.handler + '"（同名 handler）')

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[4/8]⎯

 FAIL  src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 玩法/图鉴卡片 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<view v-for="m in playMenus" :key="m.…' to contain 'role="button"'

Expected: "role="button""
Received: "<view v-for="m in playMenus" :key="m.path" class="grid-item tap" @click="go(m.path)">"

 ❯ src/styles/r100ClickableView.contract.test.ts:72:39
     70|       expect(blocks.length, '必须能定位 ' + c.file + ' 的 ' + c.cls + ' / ' …
     71|       for (const b of blocks) {
     72|         expect(b, '须补 role="button"').toContain('role="button"')
       |                                       ^
     73|         expect(b, '须补 tabindex="0"').toContain('tabindex="0"')
     74|         expect(b, '须补 @keydown.enter="' + c.handler + '"（同名 handler）')

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[5/8]⎯

 FAIL  src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 边际收益行 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<view\n          v-for="(m, i) in mar…' to contain 'role="button"'

- Expected
+ Received

- role="button"
+ <view
+           v-for="(m, i) in marginalGain.slice(0, 3)"
+           :key="m.label"
+           class="mg-row tap"
+           @click="m.hint && go(m.hint)"
+         >

 ❯ src/styles/r100ClickableView.contract.test.ts:72:39
     70|       expect(blocks.length, '必须能定位 ' + c.file + ' 的 ' + c.cls + ' / ' …
     71|       for (const b of blocks) {
     72|         expect(b, '须补 role="button"').toContain('role="button"')
       |                                       ^
     73|         expect(b, '须补 tabindex="0"').toContain('tabindex="0"')
     74|         expect(b, '须补 @keydown.enter="' + c.handler + '"（同名 handler）')

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[6/8]⎯

 FAIL  src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > index 元素 chip 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<view\n            v-for="e in elemen…' to contain 'role="button"'

- Expected
+ Received

- role="button"
+ <view
+             v-for="e in elements"
+             :key="e.key"
+             class="elem-chip tap"
+             @click="go('/pages/reactions/reactions')"
+           >

 ❯ src/styles/r100ClickableView.contract.test.ts:72:39
     70|       expect(blocks.length, '必须能定位 ' + c.file + ' 的 ' + c.cls + ' / ' …
     71|       for (const b of blocks) {
     72|         expect(b, '须补 role="button"').toContain('role="button"')
       |                                       ^
     73|         expect(b, '须补 tabindex="0"').toContain('tabindex="0"')
     74|         expect(b, '须补 @keydown.enter="' + c.handler + '"（同名 handler）')

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[7/8]⎯

 FAIL  src/styles/r100ClickableView.contract.test.ts > R100 可点击交互行键盘可达性契约 > stage 章节切换 chip 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<view\n            v-for="ch in chapt…' to contain 'role="button"'

- Expected
+ Received

- role="button"
+ <view
+             v-for="ch in chapters"
+             :key="ch.id"
+             class="chapter-chip"
+             :class="{ active: curChapter === ch.id }"
+             @click="switchChapter(ch.id)"
+           >

 ❯ src/styles/r100ClickableView.contract.test.ts:72:39
     70|       expect(blocks.length, '必须能定位 ' + c.file + ' 的 ' + c.cls + ' / ' …
     71|       for (const b of blocks) {
     72|         expect(b, '须补 role="button"').toContain('role="button"')
       |                                       ^
     73|         expect(b, '须补 tabindex="0"').toContain('tabindex="0"')
     74|         expect(b, '须补 @keydown.enter="' + c.handler + '"（同名 handler）')

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[8/8]⎯


 Test Files  1 failed (1)
      Tests  8 failed | 4 passed (12)
   Start at  01:36:24
   Duration  256ms (transform 22ms, setup 0ms, import 35ms, tests 13ms, environment 0ms)

```
