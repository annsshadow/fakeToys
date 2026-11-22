# R387 PC ServerApp 控制台写链误报成功 P1 修复

## 根因
- P1-A：后端 console 四个写端点（clear_cache/send_message/server_deploy_save/server_deploy_delete）失败或未生效时均返回 HTTP200 + type:success + 权威 bool:false，execute_command 靠非 2xx 400。前端 \`serverConsoleOps\` 五个分支裸 await 后无条件写 \`execOutput='控制台操作已提交'\`，误报成功。
- P1-B：源码原仅 import ref / toast，却在 stopServer/cacheClear/deployDelete 三处调用 confirmMsg 而未导入，运行时 ReferenceError 使三处高危确认守卫失效。

## 修复（仅 PC 前端单页）
- 新增 \`consoleWriteSucceeded(resp:any, flagKey?:string):boolean\`：type:error 或 success:false 判失败；type:success（或 type 缺省且 success 非 false）为成功前提；传 flagKey 时额外要求 data[flagKey] 或顶层 [flagKey] 严格 === true；非对象/undefined/null 判失败。type 判定为主，不强求 bool。
- serverConsoleOps 五分支 await 响应先判守卫：cacheClear 传 cleared、sendMessage 传 sent、deploySave 传 saved、deployDelete 传 deleted、cmdExecute 不传；失败 \`execError=后端 message || '操作未生效，请刷新后重试'\` 并 return，不写成功。
- 补 \`import { toast, confirmMsg } from '../utils/toast'\`，保留三处 confirmMsg 高危守卫。
- 端点路径 / payload / method / prompt 输入 / 非 2xx catch 文案 / sysConfig 只读分支 / 其它页面零改；后端 crates、SDK api.ts、migrations 零改。

## 验证
- 契约（frontend/apps/desktop/src/views/ServerApp.writeEnvelope.contract.test.ts）：源码函数体真执行注入桩，25 it。父版真父 RED=13failed/12passed，修复版 25/25 GREEN。
- 根 vitest 全量：287 files / 2569 tests，0 failed，exit 0。
- desktop tsc --noEmit：exit 0。
- PC 构建入口：index-BiDDFgkn.js（整目录原子部署到测试服）。

## 真浏览器对照（非物理真机）
- Playwright Chromium headless 1440x900  Against deployed test server; NOT a physical device。
- 11 断言全 true、failed=[]、pageErrors=[]：cleared/sent/saved/deleted:false 与 type:error 均红 execError 透传后端 message、不写成功；confirm 取消零写；cmdExecute HTTP400 正确错误；权威 true 仍写「控制台操作已提交」。
- 高危真写未执行：全部走 route 注入 + 假 ID 探针。
- 证据（gitignored）：test-results/r387-pc-server-console-write-fix/（evidence.json + 5 张 1440x900 截图）。
- 测试服地址、凭据与端口均为 <YOUR_TEST_HOST> / <YOUR_TEST_DOMAIN> 占位，真值不入库。
