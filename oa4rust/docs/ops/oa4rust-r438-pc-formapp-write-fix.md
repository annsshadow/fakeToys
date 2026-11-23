# R438 PC 表单管理写链吞错修复

- 模块：PC 表单管理（FormApp.vue）
- 缺陷：saveForm 的 PUT/POST 与 deleteForm 的 DELETE 在 HTTP 200 + type:error 信封下不读 type，静默关弹窗/刷新、乐观移除行，用户以为成功。

## 根因

后端 form_u2_update / form_u2_delete 对不存在的表单返回 HTTP 200 + type:error, message:"form not found"；SDK 仅在网络非 2xx 时抛错，await api.put/post/delete 正常 resolve。原代码不接响应、不检查业务信封，直接进入成功分支。

## 修复

- 新增 formWriteSucceeded(resp: any): boolean —— type=error 或 success=false 失败；type=success 或 type 缺省且 success 非 false 成功；非对象/null/undefined 失败；不强求权威 bool（后端成功字段为 saved/deleted:true，无统一 bool 名）。
- saveForm 两分支用 lastResp 承接响应，失败 toast.error 透传 message 并 return，不关 modal 不刷新；成功才 showCreate=false + loadList。
- deleteForm 用 lastResp 承接 DELETE 响应，失败 toast.error 透传并 return，不 filter 移除行；确认守卫、catch 非 2xx 文案、saving 在途守卫原样保留。
- doImport 与新建分支后端恒 success 列 L1，本轮零改；端点/payload/method/模板/其它页面/后端零改。

## 验证

- 契约 RED 真父 ad41a5f8b：10 failed / 7 passed（全为真实断言，0 桩错误）。
- 修复后 GREEN 17/17；根 vitest 307 files / 2954 tests 0 failed；desktop tsc --noEmit exit 0。
- 构建入口 index-CsZpn-iB.js，原子部署到测试服；1440x900 Chromium headless 非物理真机 E2E 20 断言全 true、0 pageerror，证据截图位于 gitignored 目录 test-results/r438-pc-formapp-write-fix/。
- 关键截图 sha256：
  - 01-save-typeerror.png = 9b4baa69de2129a531d9f784fa91463cd6daa816f4b1604af1868b3ef9310333（红 ✗ form not found，编辑 modal 保留）
  - 03-save-success.png = cc647bb0f8154c1bc35b0b20d555e0f07a930400ed70f4f84dbec171edb6483f（modal 关闭）
  - 06-del-typeerror.png = 3b5995864e2352e9f4246eb5d193c9b72b885d2bbd0aaf800f1c7e62affb67a0（红 ✗ form not found，行保留）

测试环境地址由 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，真值仅存在于 gitignored 证据文件与运行环境，未入库。
