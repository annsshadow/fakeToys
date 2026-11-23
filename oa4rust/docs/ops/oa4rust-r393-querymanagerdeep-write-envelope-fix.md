# R393 PC QueryManagerDeep 写链误报成功 P1 修复

## 根因
QueryManagerDeep.vue 九个真实 @click 写函数（qdCreate/qdEdit/qdDelete/qdPerm/qdMisc/qdWrite2/qdDeleteDesigner/qdInput/qdTableRow）在 await 写端点后不读响应信封，无条件 toast.success。SDK api.ts 仅对非 2xx 抛错；后端 200 + type:error（或 success:false）被原样 resolve，导致失败也弹成功绿 toast。

## 修复
新增 queryManagerWriteSucceeded(resp: any): boolean：type:error 或 success:false 判失败；type:success 或（type 缺省且 success 非 false）判成功；非对象/null/undefined 判失败。不强求权威 bool——后端部分写端点幂等成功返回 deleted:0 等合法值，一律严格 bool===true 会假阴性。九个写函数 try 内以 lastResp 承接 await 响应，共享/分支成功提示前先判守卫，失败 toast.error 透传后端 message 并 return，不弹成功、不刷新、不落态；端点路径/payload/method/prompt 取参/confirm 守卫/catch 非 2xx 文案/只读加载零改。

## RED → GREEN
契约 QueryManagerDeep.writeEnvelope.contract.test.ts（真执行注入桩 new Function，函数参数名注入，模板串在字符串拼接层不提前插值）。真父 b6b9a7335d06d1ecea36390c518e81ce6c1a68ab 跑契约 = 24 failed / 23 passed；修复版 = 47 passed / 47。

## 验证
- 定点契约：pnpm exec vitest run frontend/apps/desktop/src/views/QueryManagerDeep.writeEnvelope.contract.test.ts → 47 passed。
- 根全量：pnpm exec vitest run → 289 files / 2636 tests，0 failed。
- 桌面类型检查：frontend/apps/desktop 下 pnpm exec tsc --noEmit → exit 0。
- PC 构建入口 index-CETPQFwT.js，整目录原子部署到 <YOUR_TEST_HOST>（域名 <YOUR_TEST_DOMAIN>，值从环境变量/部署环境读取，不入库）。
- 真浏览器对照：Playwright Chromium headless 1440x900（非物理真机）注入 type:error / success:false / 幂等 deleted:0 / trueSuccess / HTTP 500 / HTTP 403 / confirm 取消，15 断言全 true、0 pageerror。证据落 gitignored test-results/r393-pc-querymanagerdeep-write-fix/（evidence.json + 12 张 1440x900 截图）。破坏性真写未执行，全程假 ID + route 注入。

## 边界
仅改 QueryManagerDeep.vue 写链判定与契约/文档三文件；后端 crates、SDK api.ts 传输层、其它视图、migrations 零改。
