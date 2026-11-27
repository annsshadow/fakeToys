# R97 RED 原始证据：两处复制失败反馈补漏契约

- 轮次：R97 规范（test-first RED）；执行：same-route-lead；attempt_id：88d35756-8e88-49eb-8d3b-31ef6e52a179
- 基线 HEAD：586f77c227642bce4f69385d1f9ec7be64053e06（本轮未改业务源码）。
- 契约：laiyipao/miniapp/src/styles/r97CopyFailure.contract.test.ts（6 it）。
- 命令：cd laiyipao/miniapp && NO_COLOR=1 pnpm exec vitest run src/styles/r97CopyFailure.contract.test.ts --reporter=verbose 2>&1; code=$?; echo RED_EXIT=$code
- vitest v4.1.12；下列为原始 stdout 逐字存档（仅将本机工作树绝对路径脱敏为 <WORKTREE>），未截断。
- 结果：exit=1，Test Files 1 failed (1)，Tests 3 failed | 3 passed (6)；失败项 it1/it2/it3 恰为三条缺口断言，通过项 it4/it5/it6 为不变量，无假阳性。

```text

 RUN  v4.1.12 <WORKTREE>/laiyipao/miniapp

 × src/styles/r97CopyFailure.contract.test.ts > R97 两处复制失败反馈补漏契约 > it1 copyUserId 的 setClipboardData 参数含 fail 回调（缺口，RED 失败） 5ms
   → copyUserId 须补 fail 回调: expected '{\n    data: String(store.userId),\n …' to contain 'fail:'
 × src/styles/r97CopyFailure.contract.test.ts > R97 两处复制失败反馈补漏契约 > it2 copyHash 的 setClipboardData 参数含 fail 回调（缺口，RED 失败） 1ms
   → copyHash 须补 fail 回调: expected '{\n    data: hash,\n    success: () =…' to contain 'fail:'
 × src/styles/r97CopyFailure.contract.test.ts > R97 两处复制失败反馈补漏契约 > it3 两处 fail 文案均为权威口径「复制失败，请手动选择复制」（缺口，RED 失败） 1ms
   → copyUserId fail 文案: expected '{\n    data: String(store.userId),\n …' to contain '复制失败，请手动选择复制'
 ✓ src/styles/r97CopyFailure.contract.test.ts > R97 两处复制失败反馈补漏契约 > it4 两处 success 路径文案与图标不变量保留（RED 通过） 1ms
 ✓ src/styles/r97CopyFailure.contract.test.ts > R97 两处复制失败反馈补漏契约 > it5 令牌三锚点不变量保留（RED 通过） 0ms
 ✓ src/styles/r97CopyFailure.contract.test.ts > R97 两处复制失败反馈补漏契约 > it6 业务源码零 --r97- 私有令牌且 share.ts 分享纯函数不动（RED 通过） 23ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 3 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/styles/r97CopyFailure.contract.test.ts > R97 两处复制失败反馈补漏契约 > it1 copyUserId 的 setClipboardData 参数含 fail 回调（缺口，RED 失败）
AssertionError: copyUserId 须补 fail 回调: expected '{\n    data: String(store.userId),\n …' to contain 'fail:'

- Expected
+ Received

- fail:
+ {
+     data: String(store.userId),
+     success: () => {
+       uni.showToast({ title: '已复制用户 ID', icon: 'success' })
+     },
+   }

 ❯ src/styles/r97CopyFailure.contract.test.ts:63:49
     61|
     62|   it('it1 copyUserId 的 setClipboardData 参数含 fail 回调（缺口，RED 失败）', () =>…
     63|     expect(meParams(), 'copyUserId 须补 fail 回调').toContain('fail:')
       |                                                 ^
     64|   })
     65|

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/3]⎯

 FAIL  src/styles/r97CopyFailure.contract.test.ts > R97 两处复制失败反馈补漏契约 > it2 copyHash 的 setClipboardData 参数含 fail 回调（缺口，RED 失败）
AssertionError: copyHash 须补 fail 回调: expected '{\n    data: hash,\n    success: () =…' to contain 'fail:'

- Expected
+ Received

- fail:
+ {
+     data: hash,
+     success: () => {
+       uni.showToast({ title: '已复制回放哈希', icon: 'success' })
+     },
+   }

 ❯ src/styles/r97CopyFailure.contract.test.ts:67:51
     65|
     66|   it('it2 copyHash 的 setClipboardData 参数含 fail 回调（缺口，RED 失败）', () => {
     67|     expect(battleParams(), 'copyHash 须补 fail 回调').toContain('fail:')
       |                                                   ^
     68|   })
     69|

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/3]⎯

 FAIL  src/styles/r97CopyFailure.contract.test.ts > R97 两处复制失败反馈补漏契约 > it3 两处 fail 文案均为权威口径「复制失败，请手动选择复制」（缺口，RED 失败）
AssertionError: copyUserId fail 文案: expected '{\n    data: String(store.userId),\n …' to contain '复制失败，请手动选择复制'

- Expected
+ Received

- 复制失败，请手动选择复制
+ {
+     data: String(store.userId),
+     success: () => {
+       uni.showToast({ title: '已复制用户 ID', icon: 'success' })
+     },
+   }

 ❯ src/styles/r97CopyFailure.contract.test.ts:72:46
     70|   it('it3 两处 fail 文案均为权威口径「复制失败，请手动选择复制」（缺口，RED 失败）', () => {
     71|     const FAIL = '复制失败，请手动选择复制'
     72|     expect(meParams(), 'copyUserId fail 文案').toContain(FAIL)
       |                                              ^
     73|     expect(battleParams(), 'copyHash fail 文案').toContain(FAIL)
     74|   })

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[3/3]⎯


 Test Files  1 failed (1)
      Tests  3 failed | 3 passed (6)
   Start at  21:36:45
   Duration  263ms (transform 23ms, setup 0ms, import 37ms, tests 31ms, environment 0ms)

RED_EXIT=1

```

复跑稳定性：本轮共独立复跑 3 次，每次均为 exit=1 / Tests 3 failed | 3 passed (6)，失败项与通过项集合完全一致，RED 稳定。

实施轮只改两处各补一个 fail 回调后，it1/it2/it3 应转绿、it4/it5/it6 恒绿，全套 6it 全绿即为 GREEN 门禁。
