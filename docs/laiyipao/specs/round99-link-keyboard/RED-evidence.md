# R99 契约 RED 逐字证据

- 命令（在 `laiyipao/miniapp` 下执行）：
  `npx vitest run src/styles/r99LinkKeyboard.contract.test.ts --reporter=verbose`
- 执行时间：2026-11-28 00:12:01 +08:00
- **exit code = 1**（预期 RED）
- 完整原始 stdout 另存：`docs/laiyipao/specs/round99-link-keyboard/RED-raw.log`（417 行，逐字未改）

## 逐字摘要行（RED-raw.log:413-416）

```text
 Test Files  1 failed (1)
      Tests  19 failed | 5 passed (24)
   Start at  00:12:01
   Duration  319ms (transform 24ms, setup 0ms, import 38ms, tests 19ms, environment 0ms)
```

- vitest 版本（RED-raw.log 头部）：`RUN  v4.1.12 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp`
- 19 failed 全部为「须补 role=button / tabindex=0 / @keydown.enter 同名 handler」的**真实缺口断言**，无 PARSE_ERROR、无模块解析错误、无 setup 错误。
- 5 passed 全部为不变量：①19 处原 @click 入口保留 ②令牌三锚点不动且无 --r99- ③业务源码零 --r99- ④禁用态 2 处保持无 role ⑤common.css .link 与全局键盘焦点样式保留。

## 失败断言逐字摘录（示例 1：announcements 清除搜索，RED-raw.log:50-55）

```text
 FAIL  src/styles/r99LinkKeyboard.contract.test.ts > R99 可点 link 键盘可达性契约 > announcements 清除搜索 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<text class="link" @click="annKeyword…' to contain 'role="button"'

Expected: "role="button""
Received: "<text class="link" @click="annKeyword = ''">"
```

## 失败断言逐字摘录（示例 2：bag 去商城买一颗，RED-raw.log:98-102）

```text
 FAIL  src/styles/r99LinkKeyboard.contract.test.ts > R99 可点 link 键盘可达性契约 > bag 去商城买一颗 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<text class="link" @click="goShop">' to contain 'role="button"'

Expected: "role="button""
Received: "<text class="link" @click="goShop">"
```

## 失败断言逐字摘录（示例 3：me 复制，RED-raw.log:146-150）

```text
 FAIL  src/styles/r99LinkKeyboard.contract.test.ts > R99 可点 link 键盘可达性契约 > me 复制 须 role=button + tabindex=0 + @keydown.enter 同名 handler
AssertionError: 须补 role="button": expected '<text class="link" style="font-size: …' to contain 'role="button"'

Expected: "role="button""
Received: "<text class="link" style="font-size: 22rpx" @click="copyUserId">"
```

## 通过的不变量逐字摘录（RED-raw.log 头部 verbose 标记）

```text
 ✓ src/styles/r99LinkKeyboard.contract.test.ts > R99 可点 link 键盘可达性契约 > 19 处原 @click 入口全部保留（不变量）
 ✓ src/styles/r99LinkKeyboard.contract.test.ts > R99 可点 link 键盘可达性契约 > 令牌三锚点不动且无 --r99-（不变量）
 ✓ src/styles/r99LinkKeyboard.contract.test.ts > R99 可点 link 键盘可达性契约 > 业务源码零 --r99- 令牌（不变量）
 ✓ src/styles/r99LinkKeyboard.contract.test.ts > R99 可点 link 键盘可达性契约 > 禁用态 2 处保持无 role（防误改，不变量）
 ✓ src/styles/r99LinkKeyboard.contract.test.ts > R99 可点 link 键盘可达性契约 > common.css .link 与全局键盘焦点样式保留（不变量）
```

## 判定

RED 成立：19 个缺口断言全部因真实缺少三属性而失败，5 个不变量全绿；实施轮补齐 19 处三属性后应同一命令 exit=0、Tests 24 passed (24)，并复跑 R60 契约全绿。
