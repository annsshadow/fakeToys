# R111 RED 证据（--accent-ink 令牌真源缺失）

- 命令：cd laiyipao/miniapp && npx vitest run src/styles/r111AccentInkToken.contract.test.ts --reporter=verbose
- 原始逐字日志：同目录 RED-raw.log。
- 环境：vitest v4.1.12，Start at 19:51:33，Duration 292ms（transform 33ms, setup 0ms, import 48ms, tests 9ms）。
- 结果：exit=1（命令内 echo RED_EXIT=1），Test Files 1 failed (1)，Tests 2 failed | 6 passed (8)。

## 失败 it（真实缺口，改前必须失败）

1. G1 tokens.css :root 色段必须真实定义 --accent-ink: #ff8659
   - 断言：expect(rb).toMatch(/--accent-ink\s*:\s*#ff8659\s*;/i)
   - 失败：expected :root 块内容 to match /--accent-ink\s*:\s*#ff8659\s*;/i —— :root 内确实没有 --accent-ink 定义。
   - 源码定位：tokens.css:60-65 ink 色段只有 fire/ice/lightning/corrosion/kinetic 五个 ink，无 accent-ink。
2. G2 --accent-ink 必须与 --fire-ink 同值（双端同值口径）
   - 断言：expect(ink).not.toBeNull() → expect(ink).toBe(fireInk)
   - 失败：expected null not to be null（--accent-ink 未定义，tokenValue 返回 null）。
   - 前置 --fire-ink=#ff8659 断言通过，证明失败来自 accent-ink 缺失而非 fire-ink 漂移。

## 通过 it（守卫 + 不变量，改前应通过）

- 守卫 现有 r3 契约只查 var(--accent-ink) 字样不查定义，本契约补定义检查 —— 坐实假守卫盲区。
- 不变量① common.css 三消费点 var(--accent-ink) 仍在（.r3-row-value / .r3-filter .r3-chip.active / .r3-empty-action）。
- 不变量② tokens.css 三锚点 lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6 不变。
- 不变量③ --fire-ink: #ff8659 原定义唯一一处不变。
- 不变量④ admin --el-fire-ink: #ff8659 不被改。
- 不变量⑤ 业务样式零 --r111-。

## 结论

- RED 干净：2 失败均为真实缺口，6 通过均为守卫/不变量，无 PARSE_ERROR、无误报失败。
- 可交实施轮：只需在 tokens.css ink 色段补一行 --accent-ink: #ff8659; 即可转 GREEN（预期 8 passed）。
- 队长复核：可直接复跑上述命令或读 RED-raw.log 逐字日志。
