# R431 PC 查询设计器 QueryDesigner saveQuery 静默吞错 P1 修复

> 证据来源：Playwright Chromium headless 1440x900 对已部署测试服的真实页面实测，**非物理真机**；截图为该 headless 浏览器真实截屏。

## 一、根因与判定

- 目标：`frontend/apps/desktop/src/views/QueryDesigner.vue` 的 `saveQuery`（:594-627），模板 modal 保存按钮「更新/创建」（:124）真实可达，路由 `/app/admin/query-designer`，无门控（DOM fold=0）。
- 根因：原 :600/:602 两分支 `await api.put/post(...)` 不接响应，无条件 :604 `showModal.value=false` + :605 `loadQueries()`；后端 `save_designer`（lib.rs:607-645，:630-631 0 行 UPDATE→`ActionResult::error("query design not found")`）返回 HTTP 200+type:error，SDK 仅非 2xx 抛错，视觉成功实际未保存（R430 甄别坐实）。

## 二、修复

- 新增 `queryDesignerWriteSucceeded(resp:any):boolean`：`type:error` 或 `success:false` 判失败；`type:success` 或 type 缺省且 success 非 false 判成功；非对象/null 判失败；未知 type 判失败；不强求 named bool（后端成功 `success({id,saved,name,query})`，权威字段 saved 非统一 bool 名，严格 bool 会假阴性）。
- saveQuery 的 PUT(save/{id}) 与 POST(create) 两分支均 `const lastResp:any = await api.put/post(...)` 承接响应；守卫失败 `toast.error(lastResp?.message || '操作未生效，请刷新后重试')` 并 return——不关 modal、不 loadQueries、不写任何本地状态；成功才 `showModal.value=false + loadQueries()`。
- `catch` 非 2xx 原文案保留；`querySaving` 在途守卫与空名称校验不动。
- 严格零改：`deleteQuery`（:651-659，`confirmMsg` 未导入不可达属独立观察项，本轮严禁夹带补 import）、`runQuery`/`openEdit`/`loadQueries` 只读、SQL 编辑器 `saveSqlQuery`（独立于 saveQuery，未复用）、端点/payload/method/模板/后端/SDK/其它页/migrations。
- 完整 diff：helper 8 行 + 两分支守卫各 5 行，18 insertions / 2 deletions。

## 三、契约 RED→GREEN 与验证

- 契约 `frontend/apps/desktop/src/views/QueryDesigner.writeEnvelope.contract.test.ts`：@vitest-environment node，readFileSync 源码 + bodyOf 花括号配对 + stripTs + new Function 真执行注入 querySaving/mform/editingQuery/showModal/api.put+post/loadQueries/toast/helper；16 it（helper 6 + PUT 分支 6 + POST 分支 2 + 在途/校验 2）。
- 真父 HEAD d31dcba5a（无 helper）：RED 6 failed / 10 passed（helper 3 + PUT/POST type:error 失败守卫 3，全为真实断言，0 桩错误）。
- 修复版：GREEN 16/16；根全量 `pnpm test` 303 files / 2880 tests 0 failed；desktop `pnpm exec tsc --noEmit -p frontend/apps/desktop/tsconfig.json` exit 0。
- 构建入口 `index-M3QlHHgN.js`（chunk `QueryDesigner-CvVdZXyr.js`），整目录 tar + sha256 校验原子部署测试服 `DEPLOY_R431_PC_FULL_OK`。

## 四、真浏览器实测（注入对照，非物理真机）

- 1440x900 headless，路由 `/app/admin/query-designer`，DOM 审计：注入 1 行假查询、新建按钮 1、fold=0；编辑（PUT 更新）与新建（POST 创建）两条路径均覆盖。
- 17 断言全 true / failed=[] / totalWrites=6 / pageErrors=[]：
  - 编辑路径 200+type:error → 红 toast 透传 `query design not found`、无成功、**modal 保留（modalAfter=1）**、行仍在、写 1 次；
  - 新建路径 200+type:error → 同上 modal 保留；success:false → 红 toast `R431 denied`、modal 保留；
  - type:success → modal 关闭（modalAfter=0）+ 列表刷新；HTTP500 → HTTP 500 且 modal 保留；403 → 没有权限 且 modal 保留。
- 证据目录（gitignored）：`test-results/r431-pc-querydesigner-silent-write-fix/`（evidence.json + 7 张 1440x900 截图）。
- 测试服地址按脱敏占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 表示，本文不含任何真实域名/IP/主机/凭据。

## 五、截图 sha256（headless 1440x900，非物理真机）

- 00-dom-audit.png = `6b01f9c4c3df24f1ba02bcab4c493c88b8aa160957c171d12078e4ee4e9e5e2c`
- 01-typeerror-row-modal-kept.png = `99bf14a919d991c981f5ad4dad3174b6af1dfa9a2d179d2d0a32e6df250bc183`（红 ✗query design not found，modal 保留、行仍在）
- 02-typeerror-create-modal-kept.png = `17828b3b96be976bced4e318046292f1c1539574d764260d1d620380c84efb29`（新建路径同一红 toast、modal 保留）
- 03-success-false.png = `f61052591c82d062f632d128ad96d3807014f29e275ace8dc0ce9ad5e148c5ce`（红 ✗R431 denied、modal 保留）
- 04-true-success-modal-closed.png = `d1a9ad11fad7f259d55f66031cc51cc9594b623f1a178514b484e883d691c289`（成功 modal 关闭 + 列表刷新）
- 05-http500.png = `7ea6ef0003b5df0ebc76cd7967a649a971153594c42957786ad2e596c3ec1d75` / 06-http403.png = `fd2afb4cf9afa684d635b1b915e2970776e87f7013c85bc483d090080b12b286`（错误 toast 正确、modal 保留）
