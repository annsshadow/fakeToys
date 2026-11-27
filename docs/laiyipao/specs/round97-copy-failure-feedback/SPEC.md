# R97 规范：两处复制失败反馈补漏（test-first RED）

## 0. 背景与权威依据

- 来源：docs/laiyipao/recon/ROUND97_B_COPY_FEEDBACK_RECON.md（R97-B 只读侦察，队长读源码坐实并裁定四条件全中）。
- 缺口：laiyipao/miniapp/src/pages/me/me.vue:714-721 的 copyUserId 与 laiyipao/miniapp/src/pages/battles/battles.vue:896-904 的 copyHash，其 uni.setClipboardData 参数对象只有 success 回调、缺 fail 回调；剪贴板 API 失败时静默吞错，用户点「复制」后无任何反馈。真 Chromium 三视口（390/768/1280）失败态实测 toast=[]。
- 权威口径：同仓 admin UsersView.vue:462-469 的 try/catch → ElMessage.error('复制失败，请手动选择复制')；该文案在 admin 12 处复制 handler 中统一使用。
- 本轮性质：规范 + test-first 契约（RED）；**不实施修复**，修复由下一轮实施任务完成。

## 1. 权威范式原文引用

laiyipao/admin/src/views/UsersView.vue:462-469：

```ts
async function copyUserId(userId: number) {
  try {
    await navigator.clipboard.writeText(String(userId))
    ElMessage.success(`已复制用户 ID ${userId}`)
  } catch {
    ElMessage.error('复制失败，请手动选择复制')
  }
}
```

失败分支文案「复制失败，请手动选择复制」为同仓统一口径。miniapp 侧无 ElMessage，改用 uni.showToast({ title: '复制失败，请手动选择复制', icon: 'none' }) 表达同等语义，不引入新令牌。

## 2. 改造点逐处

- me.vue:714-721 copyUserId：在 uni.setClipboardData 参数对象内、success 回调之后补一个 fail 回调：
  fail: () => { uni.showToast({ title: '复制失败，请手动选择复制', icon: 'none' }) }
- battles.vue:896-904 copyHash：同上补一个 fail 回调，文案逐字对齐权威口径。
- 每处只补一个 fail 回调；不改 success 路径、不改 data 字段、不改模板入口（me.vue:111、battles.vue:313）、不动 share.ts。

## 3. 不变量清单

- success 路径文案与图标保留：copyUserId「已复制用户 ID」、copyHash「已复制回放哈希」，均 icon: 'success'。
- copyHash 的 if (!hash) return 守卫与 data: hash 保留；copyUserId 的 data: String(store.userId) 保留。
- 令牌三锚点不变：laiyipao/miniapp/src/styles/tokens.css:14-16 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6。
- 业务源码零 --r97- 私有令牌。
- share.ts 三个分享纯函数（indexShare/rankShare/battleShare）不动；分享走微信原生菜单、无失败 toast 环节，不属本轮缺陷。

## 4. test-first RED 预期

契约 laiyipao/miniapp/src/styles/r97CopyFailure.contract.test.ts 共 6 it，以 import.meta.url + fileURLToPath 真实 fs.readFileSync 读两页源码，用 blockAfter 做大括号配对解析 copyUserId/copyHash 内 uni.setClipboardData 的参数对象，禁止内嵌实现副本。

- it1：copyUserId 参数含 fail 回调 —— 缺口断言，RED 应失败。
- it2：copyHash 参数含 fail 回调 —— 缺口断言，RED 应失败。
- it3：两处 fail 文案均含「复制失败，请手动选择复制」 —— 缺口断言，RED 应失败。
- it4：两处 success 文案与图标不变量保留 —— 不变量，RED 应通过。
- it5：令牌三锚点保留 —— 不变量，RED 应通过。
- it6：业务源码零 --r97- 且 share.ts 三函数不动 —— 不变量，RED 应通过。
- 预期：exit=1，Tests 3 failed | 3 passed (6)；三条失败全部来自真实缺口，无假阳性。原始输出见 RED-evidence.md。

## 5. 验证命令

```bash
cd laiyipao/miniapp && pnpm exec vitest run src/styles/r97CopyFailure.contract.test.ts --reporter=verbose
```

vitest v4.1.12。三次独立复跑均 exit=1、Tests 3 failed | 3 passed (6)，RED 稳定；退出码以重定向后 echo RED_EXIT=$code 读取，无管道掩盖。

## 6. 写域白名单与交接

- 本轮产出（均 UNTRACKED）：本 SPEC、RED-evidence.md、上述契约文件。不改业务 vue、不写真值、不 git 提交、不新增 .gitignore 条目（R97 四行队长已预置 .gitignore:626-629）。
- 实施轮仅改两处各补一个 fail 回调；完成后 6it 应全绿（it1/2/3 转绿，it4/5/6 恒绿）。
- 真浏览器复查建议：失败态注入后两处均出现 toast「复制失败，请手动选择复制」，成功态文案不变；battles 页需有战报数据使复制入口可见。
- 本规范与契约不含任何密钥/令牌/真实服务器地址。
