# R307 组织域数据真实性根因审计（PC /app/org 与 /app/directory）

## 来源与取证声明
- 执行者：qa-e2e（只读审计，不改任何源码/契约测试/后端/生产）。
- 运行环境：部署测试服（通过 `<YOUR_TEST_DOMAIN>` 占位访问），PC 入口产物 `index-BiyhjNmp.js`，服务 active。
- 浏览器证据：Playwright Chromium headless 1440x900 真浏览器、真 UI 登录（口令运行时经部署宿主读取，不入库、不回显）；截图为**本地无头浏览器渲染，非物理真机**。
- 截图与结构化证据：`oa4rust/test-results/r307-org-data/`（gitignored），含 `{org,directory,hotpic,appinfo}-before.png`、`directory-after-refill.png`、`org-after-refill.png`、`probe.json`、`refill.json`、`org-retry.json`。
- 敏感信息：本文所有域名用 `<YOUR_TEST_DOMAIN>` 占位；0 真值（无口令、私钥、会话、真实姓名/手机/邮箱）；接口响应仅结构脱敏（数组记长度、对象记 keys）。

## 一、结论（根因）
PC 组织域数据为空的**根因是后端组织组与部门数据真空**，不是前端解析 bug，也不是页面卡死：
- `PUT /api/organization/assemble/control/group/list/like` 与 `GET /api/departments/tree` 均 **HTTP 200 / `type=success` / `data:[]`**（空数组）。
- 与之对照，人员接口 `POST /api/organization/assemble/control/person/list/filter/1/size/20` 返回**真实 21 条人员数据**（`count=21`，信封 `data` 为 `{count,data,page,size}`）。
- 因此：`/app/org` 左侧树空态「暂无组织架构数据，请点击「重试加载」」是真实空数据的正确呈现；`/app/directory` 人员列表有 21 人、部门筛选 chip（公司总部/产品部/研发部/市场部/人事行政部/财务部）来自人员侧数据，但内嵌「部门树」区因 `departments/tree` 返回空而显示空态。
- 真 UI 补数不可行：点击「加载部门树」「重试加载」后两个空接口仍返回 `data:[]`；`/app/org` 空态下 `data-testid=org-tree-retry` 按钮未渲染（`v-if` 条件不满足），接口仅随页面加载自动触发且依旧空。

## 二、接口取证（HTTP 状态 / 信封结构 / 数据条数）
| 页面 | 方法与接口（脱敏路径） | HTTP/业务状态 | data 结构 | 结论 |
|---|---|---|---|---|
| /app/org | `PUT .../organization/assemble/control/group/list/like` | 200 / success | array，长度 0 | 组织组数据真空 |
| /app/directory | `GET .../departments/tree` | 200 / success | array，长度 0 | 部门树数据真空（点击后 2 次复验仍空） |
| /app/directory | `POST .../organization/assemble/control/person/list/filter/1/size/20` | 200 / success | object `{count,data,page,size}`，`data` 为人员数组 | 真实 21 条人员数据，证明组织域并非全空 |

说明：响应摘要仅记录长度与 keys，未记录任何人员明细字段（截图中的 `E2E001..E2E020` 为测试人员编号，不属于个人敏感信息，仅在证据 PNG 中保留）。

## 三、DOM 与截图取证（真浏览器）
- `/app/org`：bodyLen=221，`data-testid=org-tree` 节点数 0，可见「暂无组织架构数据，请点击「重试加载」」与「选择节点查看详情」；截图 `org-before.png` 目视确认空态、无报错、非卡死。
- `/app/directory`：bodyLen=715，部门 chip 6 个、人员卡片 20 张（共 21 人 · 第 1/2 页），内嵌「部门树」区空态「暂无部门树数据，请点击「加载部门树」」；截图 `directory-before.png` 目视确认人员数据真实渲染。
- `/app/admin/hotpic`、`/app/admin/appinfo`：探针正则 `loadingText=true` 系误匹配统计卡标签「加载中」，目视 `hotpic-before.png`（189199 B）/`appinfo-before.png`（188577 B）确认统计卡「1 总计 / 1 已启用」与表格「Test Hotpic」「应用列表」均已正常渲染，**非加载卡死，无缺陷**。
- 补数后复验：`directory-after-refill.png` 部门树区仍空；`org-after-refill.png` 与 `org-before.png` 字节哈希一致（`90ca589d…`），确认点击未改变页面数据。

## 四、真 UI 补数验证（拒绝纸上推断）
按铁律「数据不足走真实 UI 流程添加」，执行两轮补数尝试：
1. `r307-org-refill.cjs`：真登录后点击 `/app/directory`「加载部门树」、尝试 `/app/org`「重试加载」，抓接口响应；结果：两次 `GET /departments/tree` 仍 `data:[]`，`PUT /group/list/like` 仍 `data:[]`，directory 空态未消除。
2. `r307-org-retry.cjs`：改用 `data-testid=org-tree-retry` 精准点击；结果 `buttonCount=0`（空态下按钮未渲染），页面加载自动触发的 `PUT /group/list/like` 仍 `dataLen=0/type=success`。
→ 前端 UI 层无有效补数入口，空数据来自后端存储/种子缺失，属于**需后端补种子或初始化组织数据**的问题，非前端可修复。

## 五、源码定位（前端调用点与空态来源）
- 路由：`oa4rust/frontend/apps/desktop/src/main.ts:51-54` → `/app/org` 映射 `views/OrgViewer.vue`（meta title 组织架构）；`main.ts:57-59` → `/app/directory` 映射 `views/OrgDirectoryApp.vue`。
- `OrgViewer.vue:1422-1439` `loadOrgTree()` 用 `api.put('/api/organization/assemble/control/group/list/like', {})`，`nodes.value = ((resp as any)?.data ?? []) as N[]`；空数组直接落到空态。模板 `OrgViewer.vue:130` `data-testid=org-tree-retry`，`:132` 空态文案「暂无组织架构数据，请点击「重试加载」」。另 `:708`/`:1540` 亦引用同一接口（初始化聚合与详情加载）。
- `OrgDirectoryApp.vue:216-236` `loadDeptTree()` 用 `api.get('/api/departments/tree')`，`take(resp)` 解包后 `deptTree.value = list`；空数组落到空态。模板 `:73` `data-testid=dept-tree-error-retry`、`:83` `org-load-retry`。`:237-260` `loadPersonFilter()` 用 `person/list/filter` 接口，`page.data` 数组给 `rows.value`，`page.count` 给 `personCount`——这正是人员列表有 21 人的来源，证明前端对信封解析正确。
- 说明：t28（commit `bd7031a9a`）已修复名册加载的错误态/重试/`onMounted`；t46 治理的是 W13 crud-view 清单口径；本轮确认剩余空态不在前端错误态处理，而在后端返回空数据。

## 六、建议（不在本任务实施，供后续轮次决策）
1. 后端补齐组织组与部门种子数据（或在测试服初始化脚本中创建部门/组织树），使 `group/list/like` 与 `departments/tree` 返回非空；这是消除空态的真正修复。
2. 若产品允许空组织，则前端空态应提供「新建部门」入口并配套后端新增接口，避免只有「重试加载」这种对空数据无效的操作；需先评估接口缺口再补。
3. `hotpic`/`appinfo` 的 `loadingText` 探针误报建议后续 E2E 脚本改用 `data-testid` 或骨架屏节点判定，避免把统计卡标签当加载态。

## 七、复核命令与证据索引
- 探针：`cd oa4rust && node test-results/r307-org-probe.cjs` → `R307_PROBE_DONE pages=4 responses=3 errors=0`。
- 补数：`cd oa4rust && node test-results/r307-org-refill.cjs` → `refill.json`。
- 精准重试：`cd oa4rust && node test-results/r307-org-retry.cjs` → `R307_ORG_RETRY_DONE {"buttonCount":0,"emptyAfter":true,"responses":[{"method":"PUT","api":"...group/list/like","dataType":"array","dataLen":0,"type":"success"}]}`。
- 所有脚本与截图在 gitignored `oa4rust/test-results/r307-org-data/`，不入库。
- 项目处于 ≥1000 轮持续迭代中，本文仅审计当前基线，不代表项目完成。
