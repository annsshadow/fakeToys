# R380 PC 组织通讯录写链误报成功 P1 修复

## 问题
OrgDirectoryApp.vue 的唯一真实 @click 写函数 createUnitFromForm（模板「新建部门 → 提交创建」）在后端返回 HTTP 200 + `type:error`（重名 `unit already exists`）时，await 后未接响应、无条件 `toast.success('部门已创建')`，误报成功。后端 organization_assemble_control 的 unit_create 成功返回 `ok({id,name})`（必带权威 id），失败 `err` 经 ActionResult::error 仍为 HTTP 200，SDK 传输层只按 `resp.ok` 抛错，不检查 `type`，前端必须自行判定业务结果。

## 修复
- 新增 helper `writeSucceeded(resp:any):boolean`：`type==='error'` 或 `success===false` 判失败；`type==='success'` 或 type 缺省且 success 非 false、且 `resp.data.id` 存在才判成功。本端点无 bool 语义分化，采用稳定权威 id，不一刀切 `bool===true`（R367 教训）。
- createUnitFromForm 接住 `api.post` 响应先判 helper：失败 `toast.error(r?.message||'部门未创建，请重试')` 并 return，保留表单、不关闭弹窗、不刷新列表；成功才清表单、关弹窗、`toast.success` 与 `void loadDeptTree()`。
- 空 name 校验、端点路径 / payload / method、catch 非 2xx、死代码群（manage/orgAuth/runXxx）、只读加载函数全部保持原样。

## 验证
- 契约 `OrgDirectoryApp.writeEnvelope.contract.test.ts` 源码级提取 + stripTs + new Function 真执行注入 api.post/toast/withTimeout/loadDeptTree/表单 ref 桩：父版 RED 5 failed / 7 passed，修复版 12/12 GREEN。
- 根 vitest 284 files / 2504 tests 全绿；desktop tsc --noEmit exit 0。
- PC 构建入口 `index-BrwejHRJ.js`，整目录原子部署测试服。
- 真浏览器对照（Playwright Chromium headless 1440x900，**非物理真机**）：type:error（重名）→ 红 toast 透传后端 message、表单保留、0 次部门树刷新；success 无 id → 红 toast 不误报；HTTP 500/403 → 正确错误 toast；空 name 零写；type:success 带 id → 原成功提示、表单关闭、部门树刷新；0 pageerror。证据见 `test-results/r380-pc-orgdirectory-write-fix/evidence.json` 与 2 张 1440x900 截图。
- 取证约束：测试服已有 probe2 副作用测试部门行，本轮全程 route 注入场景，未执行破坏性真建部门写。

## 部署与访问
- 部署脚本：`test-results/deploy-pc-r380.sh`（相对路径执行）。
- 测试服地址从部署环境注入，不在本文件记录真值，统一用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位。
