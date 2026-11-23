# R410 PC 流程设计器 saveProcess 失败信封误报保存成功修复

## 根因
ProcessDesigner.vue saveProcess(:5857-5869) :5861 await api.post(.../designer/save/{id}) 不接响应不读 type，:5864 无条件写回 currentProcess.value.definition，:5865 无条件 toast.info(保存成功)；后端 save_flow UPDATE 0 行 → ActionResult::error(process definition not found) HTTP 200 + type:error，SDK 仅非 2xx 抛错，2xx 失败信封原样 resolve，前端误报成功。

## 修复
新增 processDesignerWriteSucceeded(resp:any):boolean：type:error 或 success:false 失败；type:success 或 type 缺省且 success 非 false 成功；非对象/null/undefined 失败；不强求 bool（后端成功 success({id,saved:true,updatedAt}) 权威字段 saved 非统一 bool 名，严格 bool 致假阴性，对齐 R402/R404/R406/R408）。saveProcess 用 const lastResp:any 承接 api.post 响应，仅守卫成功时才写回 definition 并 toast.info(保存成功)（info 仅视觉样式非缺陷，文案保留）；失败 toast.error 透传 lastResp?.message 并 return，不写回不提示成功。catch 仅非 2xx 保持；端点/payload/其它 CRUD/只读加载/后端/SDK 零改。

## 验证
- 契约真执行注入桩：真父 9e58cc525(真父 5944a54e4 同代码版本) RED 6 failed/11 passed → 修复版 17/17 GREEN。
- 定点契约 vitest：17/17 通过；根 pnpm test 全量：297 files / 2772 tests，0 failed。
- desktop pnpm run typecheck（tsc --noEmit）：exit 0。

## 真浏览器实测
Playwright Chromium headless 1440×900 对已部署测试服真实页面注入对照测试，非物理真机。覆盖 type:error process definition not found / success:false / HTTP 403 / HTTP 500 / trueSuccess；9 断言全 true，0 pageerror。证据位于被 gitignore 的 test-results/r410-pc-processdesigner-write-fix/。测试服地址与凭据统一用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，不写真值。

## 构建部署
PC 构建入口 index-CDtZIhqz.js，整目录 tar + scp + sha256 校验 + 远端原子替换部署，DEPLOY_R410_PC_FULL_OK。