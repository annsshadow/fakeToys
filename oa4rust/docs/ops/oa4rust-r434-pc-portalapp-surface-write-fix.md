# R434 PC 门户管理 surfaceOps 写链误报成功 P1 修复

> 证据来源：Playwright Chromium headless 1440x900 对已部署测试服的真实页面实测，**非物理真机**；截图为该 headless 浏览器真实截屏。

## 一、根因与判定

- 目标：`frontend/apps/desktop/src/views/PortalApp.vue` 的 `surfaceOps`（:428-476），八个操作按钮（建门户/发布门户/存字典数据/删字典数据/按名建脚本等）真实可达，无门控。
- 五写分支（create / publish / dictSet / dictDel / scriptByName）`await api.*` 均不接响应信封，:472 无条件 `toast.success("门户表面操作已提交")`；后端在假 ID 场景返回 HTTP 200 + `type:error`（surface not found / dict not found / script already exists），SDK 仅非 2xx 抛错，误报成功成立（R433 甄别三条代表性分支坐实）。

## 二、修复

- 新增 `portalSurfaceWriteSucceeded(resp:any):boolean`：type 为主不强求 bool——`type:error`/`success:false` 失败；`type:success` 或 type 缺省且 success 非 false 成功；非对象/null/未知 type 失败；不强求权威 bool。
- 五写分支各改 `const lastResp:any = await api.*(...)` + 守卫 `if(!portalSurfaceWriteSucceeded(lastResp)){toast.error(lastResp?.message||"操作未生效，请刷新后重试");return}`；失败即 return 不弹成功，五分支全部成功才执行 :472 `toast.success`。
- 三只读 GET 分支（portalPage / pageById / dictData）零改；`dictDel` 的 `confirmMsg` 确认守卫与非 2xx `catch` 原文案保留；loadPortalTwin/Twin2 吞错探针、designerUpdate/Delete/Misc（R415 已修）、deleteMutation/deletePage、端点/payload/method/模板/后端/SDK/其它页零改。
- 完整 diff：helper 7 行 + 五写分支各 1 行响应承接+1 行守卫，零外溢。

## 三、契约 RED→GREEN 与验证

- 契约 `frontend/apps/desktop/src/views/PortalApp.surfaceWrite.contract.test.ts`：@vitest-environment node，27 it；readFileSync 源码 + bodyOf 花括号配对 + stripTs + new Function 真执行注入 op/api.post/delete/get/prompt/confirmMsg/toast/helper；覆盖 helper 判定、五写分支各 type:error/success:false/type:success、非 2xx catch、三只读 GET、confirm 取消、端点校验。
- 真父 HEAD 6e9d0f57f（无 helper、五写分支裸 await）：RED 13 failed / 14 passed（全为真实断言，0 桩错误；日志 test-results/r434-red.log）。
- 修复版：GREEN 27/27；根全量 `pnpm test` 305 files / 2921 tests 0 failed；desktop `pnpm exec tsc --noEmit -p frontend/apps/desktop/tsconfig.json` exit 0。
- 构建入口 `index-CYQ6OCte.js`，整目录 tar + sha256 校验原子部署测试服 `DEPLOY_R434_PC_FULL_OK`。

## 四、真浏览器实测（注入对照，非物理真机）

- 1440x900 headless，路由 `/app/admin/portal`，五个 surfaceOps 按钮均可见；prompt 对话框注入默认值。
- 11 断言全 true / failed=[] / totalWrites=7 / pageErrors=[]：
  - publish / dictSet / scriptByName 三代表性写分支 200+type:error → 红 toast 透传 `surface not found`、无绿「门户表面操作已提交」、各写 1 次；
  - success:false → 红 toast 透传 `R434 denied`、无绿成功；
  - type:success → 绿 toast `门户表面操作已提交`；
  - confirm 取消（dictDel）→ 零写；HTTP500 → HTTP 500；403 → 没有权限。
- 证据目录（gitignored）：`test-results/r434-pc-portalapp-surface-write-fix/`（evidence.json + 9 张 1440x900 截图）。
- 测试服地址按脱敏占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 表示，本文不含任何真实域名/IP/主机/凭据。

## 五、截图 sha256（headless 1440x900，非物理真机）

- 00-dom-audit.png = `a341179a720bc35d7eed1f934e0688711120a8ddfa30e0acb0da6f6eb88dffb7`
- 01-publish-typeerror.png = `5684660ef19b69f182064563cf39166420741b874cffbe48e6dc6514e256d9b7`（红 ✗surface not found，无绿成功）
- 02-dictset-typeerror.png / 03-script-typeerror.png = 同 `5684660ef19b69f182064563cf39166420741b874cffbe48e6dc6514e256d9b7`（同场景同响应，截图字节一致）
- 04-success-false.png = `3a7d7c8fab678f6925087ab087f35fe10077ee22b5bbbb1ca0658d6ccb2575a2`（红 ✗R434 denied）
- 05-true-success.png = `767812dd180b7bea05e73f15643a0284fb95fc395b5ac30ed95ff9ad1ef23618`（绿 ✓门户表面操作已提交）
- 06-http500.png = `972f166aa423f3be4649066e72ea66d9e093e2935a72c10f68336dd23b7d20d3` / 07-http403.png = `310abbf27aaf9ff72952d4578264c58f7f02d170c0470a2448e3012d7db18bdc` / 08-confirm-cancel.png = `f8ea8ad552558cfaeb4375332f13a358a793048623fca453d33dac2673ba5172`
