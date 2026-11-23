# R391 PC QueryViewApp 写链 200+type:error 误报成功 P1 修复

## 根因
- R390 审计坐实：QueryViewApp.vue 四个有真实 @click 入口的写函数 qvWrite / qvMore / qvRows / qvDesignerRows 裸 await 后无条件 toast.success（queryview 操作已提交 / 数据表行操作已提交 / 设计器表行已更新等），catch 仅覆盖非 2xx。
- 后端 query 系列写端点业务失败时返回 HTTP200 + type:error（如假 ID / 空 key），SDK api.ts 只对非 2xx 抛错，2xx+error 原样 resolve，故四写链误报成功、不刷新不落态。

## 修复（仅 PC 前端单页）
- 新增 `queryViewWriteSucceeded(resp:any):boolean`：type:error 或 success:false 判失败；type:success 或 type 缺省且 success 非 false 判成功；非对象 / undefined / null 判失败。不强求 bool（后端 query 系列成功信封多为 type:success + json 对象，无统一权威 bool，照搬严格 bool 会假阴性）。
- 四写函数在共享成功提示前读入响应并先判：qvWrite / qvMore / qvRows 用 try 内 let lastResp 承接各分支 await 响应，写前统一守卫；qvDesignerRows 六个分支各自 await 后插入同型守卫。失败 toast.error 透传 resp.message || 操作未生效，请刷新后重试并 return，不弹成功、不刷新、不落态。
- 端点路径 / payload / method / prompt 取参 / confirm 守卫 / 非 2xx catch 文案 / 只读加载 / 死代码 / 后端 crates / SDK api.ts / 其它页 / migrations 全零改。
- 修正 attempt=1 遗留 qvMore 未初始化 lastResp 的 ReferenceError（try 内补 let lastResp:any），契约失败用例同步补 errorText 断言防假阳性。

## 验证
- 契约（frontend/apps/desktop/src/views/QueryViewApp.writeEnvelope.contract.test.ts）：源码级提取真实函数体 + new Function/new AsyncFunction 真执行，注入 api Proxy / toast / confirmMsg / prompt 桩，参数带 :string 防 TS7006，20 it。真父 3040032cc 打契约 RED=10failed/10passed，修复版 20/20 GREEN。
- 根 vitest 定点契约：20/20 passed，exit 0。
- 根 vitest 全量：288 files / 2589 tests，0 failed，exit 0。
- desktop tsc --noEmit：exit 0。
- PC 构建入口：index-DozTb3D7.js（整目录原子部署到测试服）。

## 真浏览器对照（非物理真机）
- Playwright Chromium headless 1440x900 against deployed test server; NOT a physical device。
- 14 断言全 true、failed=[]、pageErrors=[]：qvWrite / qvMore / qvRows / qvDesignerRows 四组 type:error 均红 toast 透传后端 message 且无成功提示；HTTP 500 / 403 正确错误；confirm 取消零写；trueSuccess 仍保留原成功提示。
- 高危真写未执行：全部走 route 注入 + 假 ID 探针。
- 证据（gitignored）：test-results/r391-pc-queryview-write-fix/（evidence.json + 6 张 1440x900 截图）。
- 测试服地址、凭据与端口均为 <YOUR_TEST_HOST> / <YOUR_TEST_DOMAIN> 占位，真值不入库。
