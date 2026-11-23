# R406 PC SQL语句设计器 按Flag保存写链误报成功修复

## 根因
QueryStatementDesigner.vue saveByFlag 在后端 HTTP 200 + type:error(statement not found)/success:false 失败信封下，await api.put 响应后未判 type/success，无条件 toast.success(语句已按 flag 保存) 并 queryClient.invalidateQueries 刷新列表；SDK 仅非 2xx 抛错，2xx 失败信封原样 resolve。

## 修复
新增 queryStatementWriteSucceeded(resp:any):boolean：type:error 或 success:false 失败；type:success 或 type 缺省且 success 非 false 成功；非对象/null/undefined 失败；不强求 bool。saveByFlag 用 const lastResp:any 承接响应，守卫失败时 toast.error 透传 message 并 return，不弹成功、不刷新列表；成功路径保留原 toast.success + invalidateQueries。端点/payload/method/prompt 取参/catch 非2xx文案/只读加载/后端/SDK 零改。

## 验证
- 契约真执行注入桩：真父 bc2af84de RED 6 failed/9 passed → 修复版 15/15 GREEN。
- 根 vitest 全量：295 files / 2737 tests，0 failed。
- desktop tsc --noEmit：exit 0。

## 真浏览器实测
Playwright Chromium headless 1440×900 对已部署测试服真实页面注入对照测试，非物理真机。覆盖 type:error statement not found / success:false / HTTP 403 / HTTP 500 / trueSuccess；12 断言全 true，0 pageerror。失败信封场景红 toast 透传后端 message、无成功 toast、列表不刷新(listRefreshes=0)；trueSuccess 场景绿 toast 且列表刷新一次(listRefreshes=1)。证据位于被 gitignore 的 test-results/r406-pc-querystatement-write-fix/。测试服地址与凭据统一用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，不写真值。

## 构建部署
PC 构建入口 index-B3-kpZY7.js，整目录 tar + scp + sha256 校验 + 远端原子替换部署，DEPLOY_R406_PC_FULL_OK。