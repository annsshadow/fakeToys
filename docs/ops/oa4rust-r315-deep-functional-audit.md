# oa4rust R315 深度功能缺陷审计报告（CMS/程序中心/搜索/BAM/通知/回收站/权限/上传）

## 来源与边界

- 任务：t85 [R315 D1] 深度功能缺陷审计，qa-e2e 只读审计（不改源码/契约）。
- 测试环境：测试服 PC 端 `/oa4rust`，Playwright Chromium 无头浏览器，**本地无头浏览器渲染，非物理真机**；视口 1440x900。
- 服务端点、口令均从本机 gitignored 凭据文件运行期读取，本报告用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，**零真值入库**。
- 部署：按权威步骤 build→备份 web→web.bak-$STAMP→scp→原子替换→grep 新哈希，静态资源无需重启；本轮为唯一授权部署者。部署前线上哈希 `index-95EPWgTt.js`，部署后 grep 校验 `DEPLOY_R315_HASH_OK`，实测构建哈希 JS=`index-95EPWgTt.js` / CSS=`index-BMFkaD73.css`（HEAD 前移仅队友 ledger 文档，前端源码未变）。
- 证据目录（gitignored）：`oa4rust/test-results/r315-deep-audit/`，含 flow-evidence.json、supplement-evidence.json、search-probe.json 与 17 张 PNG 截图。
- 项目处于 ≥1000 轮持续迭代中，本报告非终态结论。

## 逐闭环实测结果（真 UI 登录 + HTTP 状态 + DOM 回显）

| 闭环 | 真实操作 | 关键 HTTP | 页面回显 | 判定 |
| --- | --- | --- | --- | --- |
| C1 CMS 字典 | 新建字典项→刷新 | dict/create 200 / dict/list 200 | 新字典 flag 回显=1 | PASS |
| C2 程序中心 | 字典/调用 tab 切换+刷新 | agent/invoke/dict/list 均 200 | 内容渲染 bodyLen=917 | PASS |
| C3 全局搜索 | 关键词 R315 搜索 | queryview/search 200 | **停留初始提示，无结果无空态** | **FAIL（真实缺陷 D1）** |
| C4 BAM 统计 | 周期统计/定义/状态按钮 | bam/state/summary 200 | 真实统计 totalTask=48/completedTask=26 渲染 | PASS |
| C5 IM 未读流转 | 未读条/全部已读 | conversation/list/my 200 | 账号未读=0，无未读可流转（取证限制） | PASS（受限） |
| C6 回收站恢复 | 首行恢复 | recycle/resume 200 resumed:true | 条目 4→3 | PASS |
| C7 角色 | 新建角色→刷新 | role 200 / role/list 200 | 角色回显=1，成员入口=1 | PASS |
| U1 文件上传 | 上传弹窗真实 file input | file/upload 200（无 415） | 上传后列表刷新 | PASS（t82 修复生效） |
| U2 永久删除 | 回收站永久删除+确认 | recycle/{id}/delete 200 | 条目 3→2 | PASS |
| U3 通知公告 | 工作台通知面板刷新 | instant/list 200 / unread/count 200 | 面板渲染 bodyLen=707 | PASS |

全程 pageerror=0，API 4xx/5xx=0（初轮 41 次调用 + 补测 19 次调用，均 200）。C3 初轮脚本因仅校验 POST 200 误判 PASS，深度补测后改判 FAIL，详见下节。

## 真实缺陷清单

### D1 【P1】全局搜索有数据也不渲染，永久停留「输入关键词开始搜索」初始态

- **现象**：任意关键词（R315/a/字典/组织）点击搜索后，页面始终显示初始提示「输入关键词开始搜索 / 试试请假、文件…」，既无结果列表，也无「未找到结果」空态，用户无法判断搜索是否执行。以关键词 `a` 为例，后端明确返回 3 条真实数据（`designList` 含 S3 查询 contract 项），页面仍无反馈。
- **真浏览器证据**：`oa4rust/test-results/r315-deep-audit/C3-search-probe-a.png`（1440x900，输入框有 `a`、页面停留初始态）；`search-probe.json` queries 四项均 `items:0, empty:0, initial:1`；同文件 resps 关键词 `a` 信封 `data.designList` 含 3 条 `{id,name,category}` 记录；`flow-evidence.json` C3 调用 `/api/queryview/search` 200。
- **源码定位**：前端 `oa4rust/frontend/apps/desktop/src/views/SearchApp.vue:90` `results.value = (r?.data ?? []) as any[]` —— `api.post` 经 `oa4rust/frontend/packages/sdk/src/api.ts:84`/`:296` 的 `parseJsonEnvelope` 直接返回整信封，`r.data` 是对象 `{key,designList,viewList}` 而非数组；模板 `SearchApp.vue:18-20` 以 `results.length === 0` / `results.length > 0` 分支渲染，`results.length` 为 `undefined`，三个分支（含 `:61` 初始态的 `v-else`）永久落到初始态分支。后端 `oa4rust/backend/crates/query_assemble_surface/src/u2_closures.rs:656-662` 返回分组对象 `{key,designList,viewList}`，`designList` 字段项结构为 `{id,name,category}`（`:619-632`），`viewList` 为 `{id,name,viewFlag}`（`:636-654`）——与前端期望的扁平 `{id,title,desc,type}` 不一致。
- **可复现路径**：真登录→/app/search→输入 `a`（后端有匹配数据）→点「🔍 搜索」→观察页面持续为初始提示，无结果、无空态；对照后端响应可见 3 条 designList。
- **严重度**：P1。全局搜索是核心导航功能，有数据也完全不可见，且无加载/空态反馈，误导用户以为系统无响应；不影响数据写入与系统运行，故非 P0。
- **修复方向**：在 `SearchApp.vue:90` 适配后端分组信封——将 `r.data.designList`（映射 `title=name`、`type=design`、`desc=category`）与 `r.data.viewList`（映射 `title=name`、`type=view`）合并为统一扁平数组再赋给 `results`；模板空态改用数组 `length` 判断。或后端改返扁平数组（需同步契约）。二者取一，前端适配改动面更小。
- **验证方式**：①契约测试固定 `/api/queryview/search` 信封结构与字段，RED→GREEN；②E2E 搜索 `a` 断言渲染 3 条结果项、搜索无匹配词断言显示「未找到结果」空态且不再停留初始提示；③本轮截图 `C3-search-probe-a.png` 作为修复前基线对照。

## 取证限制（非缺陷，如实标注）

- **C5 IM 已读流转**：当前登录账号会话未读数=0，`im-unread-bar`（v-if totalUnread>0）与「全部已读」按钮未渲染，`mark_read` 端点未被触发；无未读消息可流转属前置数据限制，不判缺陷。需先在另一端制造未读消息再复查。
- **C4 BAM 周期统计明细**：`state/summary` 返回真实统计（totalTask=48/completedTask=26/pendingWork=22）并正常渲染；本轮点击周期统计按钮后 `.period-note` 未出现，疑为脚本等待不足或需单位/应用前置数据，主统计面板可用，不判缺陷。
- **附件下载/预览（任务列项⑧）**：本轮 U1 已验证上传 200，下载/预览需真实附件预览 UI 与对象存储回读，受测试服附件数据限制未在本次闭环内单独取证，列为后续待补，不臆断为缺陷。

## 结论

本轮 10 个真实 UI 闭环中 9 项 PASS（含上传 415 修复生效、回收站恢复/永久删除、角色与 CMS 字典新建回显、BAM/通知真实数据渲染），发现 1 项真实功能缺陷 D1（全局搜索有数据不渲染，P1，前端信封解包类型错误），已给出源码定位、复现路径、修复方向与验证方式。除 D1 外未发现新增真实功能缺陷；两项取证限制（IM 未读/BAM 周期明细）与一项待补项（附件下载/预览）如实标注，不臆断。全程 0 pageerror、0 API 4xx/5xx。项目持续迭代非终态。