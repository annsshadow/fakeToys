# R98 RED 证据（表单 Enter 提交键盘入口契约）

- 轮次：R98（test-first 规范任务）
- 命令：`pnpm vitest run src/styles/r98FormEnter.contract.test.ts --config vitest.config.ts`（在 `laiyipao/miniapp` 下执行）
- **exit code = 1（RED，符合预期）**
- 计数：**Test Files 1 failed (1)；Tests 3 failed | 6 passed (9)**
- 失败 3 项：三处 input 缺 `@confirm` 键盘入口（signin 兑换码 / verify battleId / bag 预设名）——真实缺口断言
- 通过 6 项：原 `@click` 入口保留 ×3、r69 双入口范式、令牌三锚点、`--r98-` 零命中——不变量全部成立
- vitest 版本：v4.1.12；Duration 372ms；无解析错误、无模块解析失败（纯断言 RED）

## 逐字日志（原文）

```text

 RUN  v4.1.12 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp

 ❯ src/styles/r98FormEnter.contract.test.ts (9 tests | 3 failed) 14ms
     × signin 兑换码 input 须有 @confirm="doRedeem" 键盘入口 8ms
     × verify battleId input 须有 @confirm="fetchReplay" 键盘入口 2ms
     × bag 预设名 input 须有 @confirm="doSavePreset" 键盘入口 1ms
     ✓ signin 兑换码 原 @click 提交入口保留 0ms
     ✓ verify battleId 原 @click 提交入口保留 0ms
     ✓ bag 预设名 原 @click 提交入口保留 0ms
     ✓ r69 双入口范式仍成立（me 三处 + login 一处 @confirm） 0ms
     ✓ 令牌三锚点不动且无 --r98- 0ms
     ✓ 业务源码零 --r98- 令牌 0ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 3 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/styles/r98FormEnter.contract.test.ts > R98 表单 Enter 提交键盘入口契约 > signin 兑换码 input 须有 @confirm="doRedeem" 键盘入口
AssertionError: signin 兑换码 须补 @confirm="doRedeem": expected '<input\n            v-model="code"\n …' to contain '@confirm="doRedeem"'

- Expected
+ Received

- @confirm="doRedeem"
+ <input
+             v-model="code"
+             v-aria-native="'输入兑换码'"
+             aria-label="输入兑换码"
+             class="code-input"
+             :class="{ 'input-error': !!redeemMsg && !redeemMsg.startsWith('兑换成功') }"
+             placeholder="输入兑换码"
+             placeholder-class="ph"
+           />

 ❯ src/styles/r98FormEnter.contract.test.ts:50:67
     48|       const block = inputBlock(c.src, c.vmodel)
     49|       expect(block, '必须能定位 ' + c.vmodel + ' input 块').not.toBeNull()
     50|       expect(block!, c.name + ' 须补 @confirm="' + c.handler + '"').toCo…
       |                                                                   ^
     51|     })
     52|   }

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/3]⎯

 FAIL  src/styles/r98FormEnter.contract.test.ts > R98 表单 Enter 提交键盘入口契约 > verify battleId input 须有 @confirm="fetchReplay" 键盘入口
AssertionError: verify battleId 须补 @confirm="fetchReplay": expected '<input\n            v-model="battleId…' to contain '@confirm="fetchReplay"'

- Expected
+ Received

- @confirm="fetchReplay"
+ <input
+             v-model="battleId"
+             v-aria-native="'battle_id'"
+             aria-label="battle_id"
+             class="inp"
+             :class="{ 'input-error': !!errMsg }"
+             type="number"
+             placeholder="例如 1"
+             placeholder-class="ph"
+           />

 ❯ src/styles/r98FormEnter.contract.test.ts:50:67
     48|       const block = inputBlock(c.src, c.vmodel)
     49|       expect(block, '必须能定位 ' + c.vmodel + ' input 块').not.toBeNull()
     50|       expect(block!, c.name + ' 须补 @confirm="' + c.handler + '"').toCo…
       |                                                                   ^
     51|     })
     52|   }

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/3]⎯

 FAIL  src/styles/r98FormEnter.contract.test.ts > R98 表单 Enter 提交键盘入口契约 > bag 预设名 input 须有 @confirm="doSavePreset" 键盘入口
AssertionError: bag 预设名 须补 @confirm="doSavePreset": expected '<input\n            v-model="presetNa…' to contain '@confirm="doSavePreset"'

- Expected
+ Received

- @confirm="doSavePreset"
+ <input
+             v-model="presetName"
+             v-aria-native="'预设名（最多 20 字）'"
+             aria-label="预设名（最多 20 字）"
+             class="preset-input"
+             placeholder="预设名（最多 20 字）"
+             placeholder-class="ph"
+             maxlength="20"
+             :focus="presetFocus"
+           />

 ❯ src/styles/r98FormEnter.contract.test.ts:50:67
     48|       const block = inputBlock(c.src, c.vmodel)
     49|       expect(block, '必须能定位 ' + c.vmodel + ' input 块').not.toBeNull()
     50|       expect(block!, c.name + ' 须补 @confirm="' + c.handler + '"').toCo…
       |                                                                   ^
     51|     })
     52|   }

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[3/3]⎯


 Test Files  1 failed (1)
      Tests  3 failed | 6 passed (9)
   Start at  23:08:38
   Duration  372ms (transform 30ms, setup 0ms, import 50ms, tests 14ms, environment 0ms)


```

## 摘要

- exit=1：3 failed | 6 passed（共 9 项）。
- 三项失败均为真实缺口：三处 input 未含 @confirm 属性；Received 原文显示 input 块仅有 v-model/v-aria-native/aria-label/class/placeholder 等既有属性。
- 六项不变量通过：@click 原入口保留、r69 双入口范式、令牌三锚点不动、业务零 --r98-。
- 本轮未改任何业务 vue/令牌/其他契约；GREEN 由后续实施轮达成。
