# R102-B 规范契约 RED 证据（逐字日志）

- 轮次：R102-B（t270 / attempt_id=899cfeb0-3455-4a8e-9d52-39e96ca2ef0e）
- 命令：`cd laiyipao/miniapp && pnpm exec vitest run src/styles/r102IndexCtaKeyboard.contract.test.ts --reporter=verbose`
- 执行时间：2026-11-28 04:35（Asia/Shanghai）；vitest v4.1.12
- 实测 exit=1，Test Files 1 failed (1)，Tests 5 failed | 4 passed (9)，Duration 268ms。
- 5 失败全为 §2 五处 CTA 的 `role="button"` 真实缺口断言；4 不变量（R99 go(d.hint)、R100/R101 ann-item、四 @click handler、令牌三锚点+零 --r102-）全通过；无解析 / 模块 / 超时错误。
- 原始 stdout 同步落 `docs/laiyipao/audits/round102-b-index-cta/RED-raw.log`。

## 逐字 stdout

```

 RUN  v4.1.12 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp

 × src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 错误态重试 retry 补键盘三件套且 @keydown.enter 复用 @click 5ms
   → 错误态重试 retry 须有 role="button": expected '<view class="btn btn-primary" style="…' to contain 'role="button"'
 × src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 新手引导卡 first-run-guide 补键盘三件套且 @keydown.enter 复用 @click 1ms
   → 新手引导卡 first-run-guide 须有 role="button": expected '<view v-if="store.maxStage === 0" cla…' to contain 'role="button"'
 × src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > AI 洞察去补强 补键盘三件套且 @keydown.enter 复用 @click 1ms
   → AI 洞察去补强 须有 role="button": expected '<view class="btn ai-cta" @click="go(a…' to contain 'role="button"'
 × src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > AI 洞察去战斗 补键盘三件套且 @keydown.enter 复用 @click 0ms
   → AI 洞察去战斗 须有 role="button": expected '<view class="btn ai-cta" @click="goSt…' to contain 'role="button"'
 × src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 公告全部 ann-all 补键盘三件套且 @keydown.enter 复用 @click 0ms
   → 公告全部 ann-all 须有 role="button": expected '<text class="ann-all" @click="goAnnou…' to contain 'role="button"'
 ✓ src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 不变量：R99 空维补导 go(d.hint) 三件套不回退 0ms
 ✓ src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 不变量：R100/R101 ann-item 三件套与 aria-expanded 不回退 0ms
 ✓ src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 不变量：四个 @click handler 在 index.vue 保留 0ms
 ✓ src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 不变量：令牌三锚点不动且业务源码零 --r102- 0ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 5 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 错误态重试 retry 补键盘三件套且 @keydown.enter 复用 @click
AssertionError: 错误态重试 retry 须有 role="button": expected '<view class="btn btn-primary" style="…' to contain 'role="button"'

Expected: "role="button""
Received: "<view class="btn btn-primary" style="margin-top: 20rpx" @click="retry">"

 ❯ src/styles/r102IndexCtaKeyboard.contract.test.ts:54:49
     52|       expect(b, '须能定位 ' + c.name + ' 开标签').not.toBeNull()
     53|       const tag = b || ''
     54|       expect(tag, c.name + ' 须有 role="button"').toContain('role="butto…
       |                                                 ^
     55|       expect(tag, c.name + ' 须有 tabindex="0"').toContain('tabindex="0"…
     56|       expect(tag, c.name + ' 须保留 @click="' + c.handler + '"').toContain('@click="' + c.handler + '"…

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/5]⎯

 FAIL  src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 新手引导卡 first-run-guide 补键盘三件套且 @keydown.enter 复用 @click
AssertionError: 新手引导卡 first-run-guide 须有 role="button": expected '<view v-if="store.maxStage === 0" cla…' to contain 'role="button"'

Expected: "role="button""
Received: "<view v-if="store.maxStage === 0" class="card first-run-guide tap" @click="goStage">"

 ❯ src/styles/r102IndexCtaKeyboard.contract.test.ts:54:49
     52|       expect(b, '须能定位 ' + c.name + ' 开标签').not.toBeNull()
     53|       const tag = b || ''
     54|       expect(tag, c.name + ' 须有 role="button"').toContain('role="butto…
       |                                                 ^
     55|       expect(tag, c.name + ' 须有 tabindex="0"').toContain('tabindex="0"…
     56|       expect(tag, c.name + ' 须保留 @click="' + c.handler + '"').toContain('@click="' + c.handler + '"…

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/5]⎯

 FAIL  src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > AI 洞察去补强 补键盘三件套且 @keydown.enter 复用 @click
AssertionError: AI 洞察去补强 须有 role="button": expected '<view class="btn ai-cta" @click="go(a…' to contain 'role="button"'

Expected: "role="button""
Received: "<view class="btn ai-cta" @click="go(aiInsight.hint)">"

 ❯ src/styles/r102IndexCtaKeyboard.contract.test.ts:54:49
     52|       expect(b, '须能定位 ' + c.name + ' 开标签').not.toBeNull()
     53|       const tag = b || ''
     54|       expect(tag, c.name + ' 须有 role="button"').toContain('role="butto…
       |                                                 ^
     55|       expect(tag, c.name + ' 须有 tabindex="0"').toContain('tabindex="0"…
     56|       expect(tag, c.name + ' 须保留 @click="' + c.handler + '"').toContain('@click="' + c.handler + '"…

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[3/5]⎯

 FAIL  src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > AI 洞察去战斗 补键盘三件套且 @keydown.enter 复用 @click
AssertionError: AI 洞察去战斗 须有 role="button": expected '<view class="btn ai-cta" @click="goSt…' to contain 'role="button"'

Expected: "role="button""
Received: "<view class="btn ai-cta" @click="goStage">"

 ❯ src/styles/r102IndexCtaKeyboard.contract.test.ts:54:49
     52|       expect(b, '须能定位 ' + c.name + ' 开标签').not.toBeNull()
     53|       const tag = b || ''
     54|       expect(tag, c.name + ' 须有 role="button"').toContain('role="butto…
       |                                                 ^
     55|       expect(tag, c.name + ' 须有 tabindex="0"').toContain('tabindex="0"…
     56|       expect(tag, c.name + ' 须保留 @click="' + c.handler + '"').toContain('@click="' + c.handler + '"…

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[4/5]⎯

 FAIL  src/styles/r102IndexCtaKeyboard.contract.test.ts > R102-B 首页 CTA 键盘可达性契约 > 公告全部 ann-all 补键盘三件套且 @keydown.enter 复用 @click
AssertionError: 公告全部 ann-all 须有 role="button": expected '<text class="ann-all" @click="goAnnou…' to contain 'role="button"'

Expected: "role="button""
Received: "<text class="ann-all" @click="goAnnouncements">"

 ❯ src/styles/r102IndexCtaKeyboard.contract.test.ts:54:49
     52|       expect(b, '须能定位 ' + c.name + ' 开标签').not.toBeNull()
     53|       const tag = b || ''
     54|       expect(tag, c.name + ' 须有 role="button"').toContain('role="butto…
       |                                                 ^
     55|       expect(tag, c.name + ' 须有 tabindex="0"').toContain('tabindex="0"…
     56|       expect(tag, c.name + ' 须保留 @click="' + c.handler + '"').toContain('@click="' + c.handler + '"…

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[5/5]⎯


 Test Files  1 failed (1)
      Tests  5 failed | 4 passed (9)
   Start at  04:35:14
   Duration  268ms (transform 26ms, setup 0ms, import 39ms, tests 8ms, environment 0ms)
```

## 收尾卫生

- HEAD=5ec099e5508b853a4cc9b8936d09d76ec1020589（未变）；`git diff --check` RC=0。
- `git status --porcelain -- laiyipao/miniapp/src/pages/index/index.vue laiyipao/miniapp/src/styles/tokens.css` 空 —— 未改业务源码与令牌。
- tracked diff 仅 `.gitignore`（队长预置上游既存，非本任务改动）。
- auth 两路径 `laiyipao/deploy/.local.keys` / `laiyipao/deploy/remote-config.env` ABSENT；产物零真值。
