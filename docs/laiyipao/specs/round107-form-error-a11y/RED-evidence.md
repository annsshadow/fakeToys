# R107 RED 证据（契约先行）

## 执行命令

```bash
cd laiyipao/miniapp
npx vitest run src/styles/r107FormErrorA11y.contract.test.ts --reporter=verbose
```

- 实测退出码：`1`（RED 预期，缺口 it 失败）
- vitest 版本：v4.1.12
- 结果：`Test Files 1 failed (1)` / `Tests 9 failed | 4 passed (13)` / `Duration 431ms`（transform 27ms, setup 0ms, import 42ms, tests 21ms, environment 0ms）
- 9 个失败 it 全为缺口断言（4 input `:aria-invalid` 缺失 + 3 错误节点 id 缺失 + main.ts 桥接 aria-invalid/aria-describedby 缺失），4 个不变量 it 全通过，无 PARSE_ERROR / 模块加载错误。

## 失败 it 名（逐字，来自 RED-raw.log）

1. 缺口：pages/login/login.vue username 错误态语义（:aria-invalid 绑定失败态 + describedby 指向 login-err） → `pages/login/login.vue username 缺 :aria-invalid 绑定: expected null not to be null`
2. 缺口：pages/login/login.vue password 错误态语义（:aria-invalid 绑定失败态 + describedby 指向 login-err） → `pages/login/login.vue password 缺 :aria-invalid 绑定: expected null not to be null`
3. 缺口：pages/verify/verify.vue battleId 错误态语义（:aria-invalid 绑定失败态 + describedby 指向 verify-battle-err） → `pages/verify/verify.vue battleId 缺 :aria-invalid 绑定: expected null not to be null`
4. 缺口：pages/signin/signin.vue code 错误态语义（:aria-invalid 绑定失败态 + describedby 指向 signin-redeem-err） → `pages/signin/signin.vue code 缺 :aria-invalid 绑定: expected null not to be null`
5. 缺口：pages/login/login.vue 错误播报节点补唯一 id="login-err" 且保留 role="alert" → `错误节点缺 id="login-err"`
6. 缺口：pages/verify/verify.vue 错误播报节点补唯一 id="verify-battle-err" 且保留 role="alert" → `错误节点缺 id="verify-battle-err"`
7. 缺口：pages/signin/signin.vue 错误播报节点补唯一 id="signin-redeem-err" 且保留 aria-live="polite" → `错误节点缺 id="signin-redeem-err"`
8. 缺口：main.ts applyAriaNative 桥接 aria-invalid 到原生 input → `桥接未把 aria-invalid setAttribute 到原生 input: expected false to be true`
9. 缺口：main.ts applyAriaNative 桥接 aria-describedby 到原生 input → `桥接未把 aria-describedby setAttribute 到原生 input: expected false to be true`

## 通过的不变量 it（逐字，来自 RED-raw.log）

- ✓ 不变量：R61 原生 input aria-label 桥接保留且 4 个目标 input 仍有 aria-label + v-aria-native
- ✓ 不变量：R60 键盘三件套（me back-btn）未被误改
- ✓ 不变量：R101 aria-expanded 与 R104 aria-busy 未被误改
- ✓ 不变量：令牌三锚点不动且 3 页 + main.ts 零 --r107-

## 逐字 stdout 摘录（完整见同目录 RED-raw.log，168 行 / 12792B）

```text
RUN  v4.1.12 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp

× src/styles/r107FormErrorA11y.contract.test.ts > R107 表单校验错误 a11y 语义契约 > 缺口：pages/login/login.vue username 错误态语义 5ms
→ pages/login/login.vue username 缺 :aria-invalid 绑定: expected null not to be null
× ... password / verify battleId / signin code（同上 :aria-invalid 缺失）
× ... login/verify/signin 错误播报节点补唯一 id（缺 id="login-err" 等）
× ... main.ts applyAriaNative 桥接 aria-invalid / aria-describedby（expected false to be true）
✓ ... 不变量：R61 原生 input aria-label 桥接保留且 4 个目标 input 仍有 aria-label + v-aria-native 1ms
✓ ... 不变量：R60 键盘三件套（me back-btn）未被误改 1ms
✓ ... 不变量：R101 aria-expanded 与 R104 aria-busy 未被误改 1ms
✓ ... 不变量：令牌三锚点不动且 3 页 + main.ts 零 --r107- 1ms

Test Files  1 failed (1)
Tests  9 failed | 4 passed (13)
Duration  431ms
```

## 结论

RED 干净：9 个失败均为待实施缺口，4 个不变量在缺口态即通过，说明契约未误伤既有 R60/R61/R101/R104 与令牌体系。实施轮 GREEN 目标 13/13，并复跑 `r61FormA11y.contract.test.ts` 确保桥接扩展未回退 aria-label。
