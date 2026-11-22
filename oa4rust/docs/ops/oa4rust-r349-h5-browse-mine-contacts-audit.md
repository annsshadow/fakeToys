# R349 H5 未审计页批量甄别：browse / mine / contacts

> 审计来源：2026-11-22 测试服真浏览器 E2E（Playwright Chromium 390x844 headless，isMobile/hasTouch/DPR2），**非物理真机**；证据截图均为 headless 浏览器真实渲染，非设备截图。
> 域名/凭据脱敏：仅使用占位符 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，0 真值入库。
> 本轮为只读审计：未改任何源码/契约/后端，仅提取缺陷与最小修复方向。

## 1. 范围与甄别

- 页面：`oa4rust/mobile/src/pages/browse/browse.vue`（浏览）、`oa4rust/mobile/src/pages/mine/mine.vue`（我的）、`oa4rust/mobile/src/pages/contacts/contacts.vue`（通讯录）。
- H5 入口 `/oa4rust-h5/`，API 前缀 `/oa4rust-api`（`main.ts:25`：VITE_OA_API_BASE || /oa4rust-api）。
- 文档甄别：`docs/ops` 对三页名 0 命中（无独立审计覆盖）；R296/R306/R315/R332 等既有轮次只点名视觉/搜索信封，未覆盖三页写路径。
- E2E 共 3 轮脚本：首轮探针（裸路径误报 404/405，已纠正）、v2（前缀修正 + browse 三 tab + contacts 只读 + P1 HTTP 探针）、v3（密码框 DOM 探测 + 错误旧密码 UI 分支）。0 pageerror（各轮）。

## 2. 真缺陷（1 个 P1）

### P1：我的页修改密码在服务端拒绝时误报「密码修改成功」（R327 D1 同型）

- 源码链路：
  - `oa4rust/mobile/src/pages/mine/mine.vue:62-85` `changePassword()`：`await authApi.changePassword(...)` 后无条件 `uni.showToast({title:"密码修改成功", icon:"success"})`（:78），catch 才显示错误（:80-81），成功后清空表单（:79）。
  - `oa4rust/mobile/src/services/index.ts:39-40` `changePassword` = `mapi.put("/api/person/password", data, { discardResponse: true })`。
  - `oa4rust/mobile/src/services/http.ts:186-192`：HTTP 2xx 一律 `resolve`；`discardResponse` 直接 `resolve({success:true,data:undefined})`，从不解析也不检查响应体 `type:"error"`。
- 实测证据（E2E v2 pwdProbe）：错误旧密码 PUT `/oa4rust-api/api/person/password` → **HTTP 200**，body `{type:"error", message:"old password mismatch"}`，`prompt:"com.x.base.core.project.exception.ExceptionEntityNotExist"`。UI 分支（v3）提交后确实发出该请求 `calls=[{s:200,p:/person/password}]`。
- 影响：用户输入错误旧密码（或任何后端以 200+error 信封返回的拒绝）时，页面弹「密码修改成功」并清空表单，用户误以为密码已改，实际未改——安全风险 + 误导。
- 最小修复方向（本轮不改）：`http.ts` 传输层统一识别 2xx 响应体 `{type:"error"}` 并抛 `ApiError(message)`（对齐桌面 ApiClient 语义）；或至少 `changePassword` 去掉 `discardResponse` 并在 `mine.vue` 判定返回信封 `type==="success"` 后再报成功。

## 3. 通过页（未发现真缺陷）

### browse（浏览）三 tab — 裸数组解析正确、空态可区分

- 逐 tab 带 `/oa4rust-api` 前缀 fetch 探针（v2 browse）：portal `GET /api/portal/assemble/surface/page/list/portal/default` → 200 裸数组 n=0；query `GET /api/queryview/view/list/query/all` → 200 裸数组 n=0；cms `GET /api/cms/core/entity/column/list` → 200 裸数组 n=2。dataIsArray=true / innerIsArray=false / count=null（无顶层 count 双层信封）。
- DOM：portal/query 卡片 0 + 空态「暂无浏览数据，换个页签或去搜索找找看」（empty=1）；cms 卡片 2 + 真实栏目名可见（empty=0）。列表渲染与空态一致，无 R340/R347 双层信封致恒空缺陷。

### contacts（通讯录）— 只读列表裸数组正常

- 只读探针（v2 contacts）：`GET /api/organization/assemble/control/threemember/list` → 200 裸数组 n=22；`GET /api/unit/list/all` → 200 裸数组 n=2；`GET /api/organization/assemble/control/group/list/like` → 415（probe 仅 GET 未带 body，如实记录、非页面缺陷）。
- DOM：body 显示「通讯录 搜索姓名/手机号/邮箱 人员 单位 群组」及真实人员行（R347V1827554-ThreeMember 等可见）。页面为只读查看 + 搜索 + 聊天入口，未发现写路径缺陷。

## 4. 取证限制与产品语义差异

- 取证限制：改密 UI 分支用错误旧密码（`__WRONG_R349__`）避免改掉测试账号真实口令；toast 为 uni.showToast 约 2s 自动消失，v3 在 2.6s 采样未捕获文案，但已由源码链路 + HTTP 响应体双重确证缺陷，且未执行破坏性成功改密。
- 取证限制：contacts 的 group/like 探针 415 因 probe 仅 GET 未带参数，未验证真实群组搜索交互；属只读查看页，不影响判定。
- 产品语义差异：browse 的 portal/query 当前测试环境无数据（空态），cms 有 2 条；不同 tab 数据量差异属环境数据，非缺陷。

## 5. 证据与命令

- 截图（gitignored）：`oa4rust/test-results/r349-h5-browse-mine-contacts/` 下各页 `*.png`，均为 390x844 isMobile/hasTouch DPR2 headless 真浏览器全页截图（image 实际 1960x4428 降采样展示），**非物理真机**。
- evidence：`evidence.json`（首轮）、`evidence-v2.json`（前缀修正+browse/contacts+P1探针）、`evidence-v3.json`（密码框 DOM+错误分支）。
- 脚本（gitignored）：`r349-h5-browse-mine-contacts.cjs` / `-v2.cjs` / `-v3.cjs` 及对应 `*-run.sh`。

## 6. 结论

- 本批 3 页：1 个 P1 真缺陷（mine 改密误报成功，R327 D1 同型）、0 个 P2、browse/contacts 2 页通过、0 个新增 P0。0 pageerror。
- 项目仍在持续迭代（≥1000 轮），本轮非终态。
