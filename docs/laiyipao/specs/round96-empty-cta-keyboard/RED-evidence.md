# R96 RED 门禁原始日志存档（脱敏）

> 命令：`cd laiyipao/miniapp && pnpm exec vitest run src/styles/r96EmptyCtaKeyboard.contract.test.ts --reporter=verbose > ../../docs/laiyipao/specs/round96-empty-cta-keyboard/RED.txt 2>&1; echo RED_EXIT=$?`
> 退出码 RED_EXIT=1（vitest v4.1.12）。以下逐字复制自 RED.txt（被 .gitignore `*.txt` 忽略），仅将工作树绝对路径替换为 `<WORKTREE>` 占位符，测试结果与断言原文未作任何改动。

```text

 RUN  v4.1.12 <WORKTREE>/laiyipao/miniapp

 × src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it1 六处空态 CTA 均含 role="button" 6ms
   → pages/bag/bag.vue 须声明 role=button 语义: expected '        <view class="r3-empty-action"…' to contain 'role="button"'
 × src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it2 六处空态 CTA 均含 tabindex="0" 2ms
   → pages/bag/bag.vue 须进入 Tab 序列（WCAG 2.1.1）: expected '        <view class="r3-empty-action"…' to contain 'tabindex="0"'
 × src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it3 六处空态 CTA 均含 @keydown.enter="X()" 1ms
   → pages/bag/bag.vue 须可被 Enter 键盘激活: expected '        <view class="r3-empty-action"…' to contain '@keydown.enter="r4GoBag()"'
 × src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it4 属性顺序 role -> @click -> tabindex -> @keydown.enter 1ms
   → pages/bag/bag.vue role 位: expected -1 to be greater than -1
 ✓ src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it5 六处焦点函数声明保留（不变量） 2ms
 ✓ src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it6 空态四段槽位顺序不变（不变量） 3ms
 ✓ src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it7 令牌三锚点不变且业务源码零 --r96-（不变量） 28ms
 ✓ src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it8 R60 既有键盘 CTA 范式不回退（抽查≥2处，不变量） 1ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 4 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it1 六处空态 CTA 均含 role="button"
AssertionError: pages/bag/bag.vue 须声明 role=button 语义: expected '        <view class="r3-empty-action"…' to contain 'role="button"'

Expected: "role="button""
Received: "        <view class="r3-empty-action" @click="r4GoBag()">去关卡页 →</view>"

 ❯ src/styles/r96EmptyCtaKeyboard.contract.test.ts:57:57
     55|   it('it1 六处空态 CTA 均含 role="button"', () => {
     56|     for (const s of SIX) {
     57|       expect(ctaLine(s), s.rel + ' 须声明 role=button 语义').toContain('rol…
       |                                                         ^
     58|     }
     59|   })

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/4]⎯

 FAIL  src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it2 六处空态 CTA 均含 tabindex="0"
AssertionError: pages/bag/bag.vue 须进入 Tab 序列（WCAG 2.1.1）: expected '        <view class="r3-empty-action"…' to contain 'tabindex="0"'

Expected: "tabindex="0""
Received: "        <view class="r3-empty-action" @click="r4GoBag()">去关卡页 →</view>"

 ❯ src/styles/r96EmptyCtaKeyboard.contract.test.ts:63:61
     61|   it('it2 六处空态 CTA 均含 tabindex="0"', () => {
     62|     for (const s of SIX) {
     63|       expect(ctaLine(s), s.rel + ' 须进入 Tab 序列（WCAG 2.1.1）').toContain(…
       |                                                             ^
     64|     }
     65|   })

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/4]⎯

 FAIL  src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it3 六处空态 CTA 均含 @keydown.enter="X()"
AssertionError: pages/bag/bag.vue 须可被 Enter 键盘激活: expected '        <view class="r3-empty-action"…' to contain '@keydown.enter="r4GoBag()"'

Expected: "@keydown.enter="r4GoBag()""
Received: "        <view class="r3-empty-action" @click="r4GoBag()">去关卡页 →</view>"

 ❯ src/styles/r96EmptyCtaKeyboard.contract.test.ts:69:53
     67|   it('it3 六处空态 CTA 均含 @keydown.enter="X()"', () => {
     68|     for (const s of SIX) {
     69|       expect(ctaLine(s), s.rel + ' 须可被 Enter 键盘激活').toContain('@keydow…
       |                                                     ^
     70|     }
     71|   })

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[3/4]⎯

 FAIL  src/styles/r96EmptyCtaKeyboard.contract.test.ts > R96 六页空态 CTA 键盘可达契约 > it4 属性顺序 role -> @click -> tabindex -> @keydown.enter
AssertionError: pages/bag/bag.vue role 位: expected -1 to be greater than -1
 ❯ src/styles/r96EmptyCtaKeyboard.contract.test.ts:80:40
     78|       const iTab = line.indexOf('tabindex="0"')
     79|       const iKey = line.indexOf('@keydown.enter="')
     80|       expect(iRole, s.rel + ' role 位').toBeGreaterThan(-1)
       |                                        ^
     81|       expect(iClick, s.rel + ' click 位').toBeGreaterThan(-1)
     82|       expect(iTab, s.rel + ' tabindex 位').toBeGreaterThan(-1)

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[4/4]⎯


 Test Files  1 failed (1)
      Tests  4 failed | 4 passed (8)
   Start at  20:12:39
   Duration  318ms (transform 26ms, setup 0ms, import 38ms, tests 46ms, environment 0ms)

```

## 独立复跑确认

同一命令二次执行：RERUN_EXIT=1，Test Files 1 failed(1) / Tests 4 failed | 4 passed (8)，Duration 284ms。RED 稳定可复现，缺口断言持续失败、不变量持续通过。