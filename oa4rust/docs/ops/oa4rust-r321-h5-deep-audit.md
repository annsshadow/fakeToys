# R321 H5 深度功能审计第二轮（未覆盖模块 + T1 甄别）

## 来源与边界

- 执行人：qa-e2e（本地无头浏览器渲染，**非物理真机**）；分支 opt1000/oa4rust-pc-ai-refactor；审计起始 HEAD 2c6863da，收尾 HEAD 以提交记录为准（队友并行提交使 HEAD 前移，本任务只读审计未改源码/契约/配置）。
- 测试服：站点 <YOUR_TEST_DOMAIN>，H5 入口 /oa4rust-h5/，构建 index-BUQSTyeU.js（沿用既有产物，本轮未重新 build/部署）。凭据仅运行期经 SSH 读取远端凭据文件，不入库、不回显。
- 浏览器：Playwright Chromium headless，390x844，isMobile/hasTouch/DPR2；所有截图为**本地无头浏览器渲染，非物理真机**。
- 方法：真实 UI 登录（账号/密码输入 + 提交按钮）后逐条走真实 DOM 入口，监听 /api/ 响应记录真实 HTTP 状态与响应体；H5 fetch 统一前缀 /oa4rust-api（源码 oa4rust/mobile/src/http.ts:25）。失败项严格区分真缺陷、取证限制（无 UI 入口/无头 picker 不兼容）、产品语义差异（不同 API base/表族不同源）与脚本选择器问题。
- 证据目录（gitignored）：oa4rust/test-results/r321-h5-deep/，含 main-evidence.json、t1-evidence.json、bbs-evidence.json、supplement-evidence.json、contacts-evidence.json、final-evidence.json、recycle-evidence.json/recycle2-evidence.json/recycle3-evidence.json 与 20+ PNG。

## 七模块 + T1 闭环判定表

| 编号 | 闭环 | 判定 | 真请求/真响应/页面回显 | 证据 |
|---|---|---|---|---|
| M1 | 通知公告 | PASS | GET /oa4rust-api/api/ai/assemble/control/ann/list 200；安全区渲染、数据真 | M1-notice.png / main-evidence.json |
| M2 | 收藏 | PASS | GET /oa4rust-api/api/.../collect/list 200；空态真实（当前账号无收藏） | M2-mine.png / main-evidence.json |
| M3 | 个人中心与设置 | PASS | 会话用户显示 true；改密入口真实存在（3 个 password 型 uni-input-input + 保存按钮）；退出 true（补测排除脚本选择器误判） | M2-mine.png / S2-mine-pwd.png / supplement-evidence.json |
| M4 | 通讯录/组织 | PASS | GET /oa4rust-api/api/.../person/list/like 200；搜索回车触发生成 42 行、含 E2E001 等人员；成员详情弹窗可打开 | M4-contacts.png / S3-contacts-searched.png / contacts-evidence.json |
| M5 | 文档/文件 | PASS | GET /oa4rust-api/api/file/.../file/list/admin 200 + attachment/list/editor/admin 200；附件区块渲染 | M5-doc.png / main-evidence.json |
| M6 | 论坛回帖 | PASS | GET /oa4rust-api/api/bbs/.../mobile/view/all 200（列表 20 行）；POST /oa4rust-api/api/bbs/.../reply/create 200 提交成功；刷新后回贴数 0→1、persisted=true、countGrew=true | M6-bbs-list.png / M6-reply-sent.png / M6-reply-after-reload.png / bbs-evidence.json |
| M7 | 回收站除 | PASS | GET /oa4rust-api/api/recycle/list 200（3 条含 R321 审计文件）；Playwright .tap() 真实派发删除按钮 → 确认 modal 确定 → DELETE /oa4rust-api/api/recycle/delete/{id} 200；刷新后 3→2 | M7-recycle-list.png / M7-recycle-final.png / recycle3-evidence.json |
| T1 | 日历/组织/文件两族端点甄别 | 非同源不同业务语义（不判 bug） | 见下节详证 | T1-calendar.png / T1-contacts.png / T1-doc.png / t1-evidence.json |

## T1 两族端点甄别结论（日历/组织/文件）

- 日历：control 族 GET /api/calendar/.../myCalendars 200 count=2、event/list/filter 200 count=11（样本 R311-E2E-事件）；core 族 GET /api/calendar/event/list/{id} 在同一 calId（3348efb9-81fa-4663-ad49-72a5b4d68c9a，日历数 7）下 200 count=3（样本 R317H5日程_1795241668724/R316evt_1795240874397）。两族返回样本完全不同 → 非同源：control 族 event/list/filter 为全量过滤视图，core 族按日历 ID 精确归属。与源码 oa4rust/mobile/src/services/index.ts:445 注释「create 与 myCalendars 同源（均写/读 cal_calendar）」不矛盾——该注释讲的是日历容器，非事件表。
- 组织：control 族 GET /api/.../person/list/like 200 count=21（E2E001 等）；core 族 GET /api/.../unit 200 count=2、person/by/unit 200 count=0（当前查询单元无人员）。两族读不同表族，属不同业务语义。control 族 group 端点裸 GET 400 系缺参（契约 backend-route-methods.fixture.ts:1575 为 GET+PUT），非业务路径问题；真 UI 群组 Tab 已渲染内容（groupApi=[]，G1-group-tab.png）。
- 文件：control 族 GET /api/file/.../list 200 count=0（x_file 空）；core 族 GET /api/file/.../attachment 200 count=13（FILE_FILE，r301-c301-JSON-A.txt 等）。两族读不同表族，属不同业务语义。
- 依任务纪律：上述三域两族为「非同源、不同业务语义」，**不判 bug**。

## 取证限制（不计业务缺陷）

1. 通知公告点击开弹窗：无头 Chromium 对 uni-app 组件点击派发未命中（G2-notice-dialog clicked 为空、shown=false）；M1 已由 ann/list 200 + 安全区渲染证实数据真实，开弹窗交互受限如实标注。
2. 通讯录「发起会话」按钮选择器未命中（chatStarted=false，S3-chat.png）；会话发起能力已在 R314/R317 审计验证，本轮不重复判缺陷。
3. 回收站前两轮 DOM .click() 无法触发 uni-app @tap（R317 同类问题）；第三轮改用 Playwright .tap() 真实派发后删除闭环 PASS，限制已解除。

## 真实缺陷

- 本轮七模块 + T1 甄别未发现新增真实 P0/P1 缺陷。M3/M4 的初判 FAIL 均经补测排除为脚本选择器问题；M7 删除经三轮攻坚以真实 tap 派发闭环 PASS。

## 结论

R321 第二轮 H5 深度审计：通知公告、收藏、个人中心与设置、通讯录/组织、文档/文件、论坛回帖、回收站删除七模块均以真实 UI + 真请求/真响应/页面回显验证 PASS；T1 三域两族端点甄别定性为「非同源、不同业务语义」，不判 bug。取证限制 2 项（无头弹窗点击、会话按钮选择器）如实标注，不构成业务缺陷。项目持续迭代非终态。