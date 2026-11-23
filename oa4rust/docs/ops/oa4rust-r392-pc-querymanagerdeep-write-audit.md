# R392 PC 查询管理深化写操作链失败信封甄别报告

- 轮次：R392（qa-e2e 只读甄别，不改码）
- 甄别对象：PC 端 \`QueryManagerDeep.vue\` 查询管理深化页建/改/删/权限/表行/执行写链在后端 HTTP 200 业务失败信封下的 UI 反馈一致性
- 实测环境：Playwright Chromium headless，视口 1440x900，针对已部署测试服；**非物理真机**。测试域名/主机均为占位符 \`<YOUR_TEST_DOMAIN>\` / \`<YOUR_TEST_HOST>\`，真值不入库。
- git 基线：\`bb9b70a702ecd98b69130ab63857254a7187f5e1\`（HEAD 与上游一致，工作树干净）

## 一、任务与范围

甄别查询管理深化页写链是否存在与 R366/R368/R370/R373/R381/R390/R391 同型的缺陷：后端返回 HTTP 200 + type:error 业务失败时，前端 await 后无条件弹成功 toast，误导用户。判真缺陷需三全：真 UI 可复现 + 真请求响应 + 源码定位。破坏性真状态翻转写（建/改/删真数据、deleteAll、批量执行）未授权不执行，用假 ID + 写响应不拦截走真后端替代。

覆盖甄别：docs/ops 命中 r342-query-deep-audit（写端点群**只读深度审计**，静态枚举 63 写端点可达性与真浏览器端点实测，未测失败信封下 UI 反馈）、r343-querykey-fix（queryKey 写后回显修复），均非失败信封 UI 反馈专项；相邻页 QueryViewApp 的 R390/R391 口径可参照但不可直接套用结论。本轮首次专测失败信封 UI 反馈，有效增量。

## 二、源码甄别（真实入口 / 只读探测 / 门控三分）

文件相对路径 \`frontend/apps/desktop/src/views/QueryManagerDeep.vue\`（约 1600 行）；路由 \`main.ts:235-240\` path=query-manager-deep/name=QueryManagerDeep，页面标题「查询管理深化」。无 EndpointFold / devToolsEnabled 门控（与 R372/R388 门控型不同），写按钮在测试服生产构建真渲染：DOM 审计 70 个按钮，含建查询/改查询/删查询/表行更新/改统计/设计器检索/删查询设计器各命中，fold=0。

**裸成功提示且有真实 @click 入口的 10 个写函数（模板 :15-78，均 prompt 取参 + await 后无条件 toast.success，不读 type / 权威 bool；catch 仅覆盖非 2xx）**：
- qdCreate（:576-589，:585「\${kind} 已创建」，POST designer/query|statement|stat|importmodel|neural/model）
- qdEdit（:590-607，:603「\${kind} 已更新」，PUT designer/query/\${eid} 等）
- qdDelete（:608-628，:624「\${kind} 已删除」，:613 confirmMsg 守卫，DELETE designer/query|statement|stat|view|table|importmodel|neural/model|importer/delete/\${eid}）
- qdPerm（:629-644，:640「\${kind} 权限已设置」，POST .../\${eid}/permission）
- qdMisc（:646-673，:669「查询设计器操作已提交」，search/searchV2/tableExecute/statSimulate/queryIcon/iconSet/outputSelect/statementList）
- qdWrite2（:675-723，:719「查询设计器写操作已提交」，statEdit/statDelete/importRun/importEdit/importDelete/viewDelete/tableDelete/stmtPermission/stmtExecute/neuralUpdate/neuralDelete/neuralReset，多分支 confirmMsg 守卫）
- qdDeleteDesigner（:766-776，:772「查询设计器已删除」，:769 confirmMsg 守卫，POST designer/delete/\${id}）
- qdInput（:777-788，:784「输入 \${op} 已提交」，PUT designer/input/compare|cover|create|prepare/cover|prepare/create）
- qdTableRow（:789-806，:802「表行 \${op} 已提交」，insert/save/deleteAll/update/delete，POST/DELETE designer/table/\${ef}/row...）

read 类 qdReads2（:725-745，:740 toast 读命中）/ qdReads3（:746-765，:761 读命中）与 :497-569 load 系列（s() swallow error）为只读探测，不判缺陷。13 处 toast.success 中 :999/:1044「导入完成」属执行结果反馈，非本节写的建改删链。

SDK 铁证：frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 仅 JSON.parse 不查 type；:275-296 仅 !resp.ok 抛错（非 2xx 进 catch），2xx + type:error 原样 resolve → 上述函数落入无条件成功路径。

## 三、真后端信封实测（假 ID / 缺参探针，不注入）

在测试服真浏览器内 fetch 直打（API base \`/oa4rust-api\`），10 个端点信封形状全部坐实：
- query 改 / 删 / 权限假 ID → 200 + type:error「query not found」
- stat 改假 ID → 200 + type:error「stat not found」
- view 删假 ID → 200 + type:error「view not found」
- table 删假 ID → 200 + type:error「table not found」
- 表行改假 ID → 200 + type:error「row not found」
- 删查询设计器假 ID → 200 + type:error「query design not found or already deleted」
- designer/search → 200 + data 数组（无 type 字段，只读检索正常）

产品语义差异：表行 deleteAll 假 ID → 200 + type:success + data.count=0 / deleted=0（幂等删除口径，非失败信封，不构成误报前提）；designer/search 返回 data 数组属正常只读成功。shared/response.rs ActionResult::error 用 HTTP 200 是 o2server 兼容既有设计（非本轮改）。

## 四、E2E 决定性对照（写响应不拦截，走真后端）

1440x900 headless，5 个真实 UI 点击 + prompt 填假 ID，写响应完全走真后端不拦截，逐个捕获真请求响应体：
1. 「改查询」填 R392-NOSUCH-Q → 真 PUT designer/query/假 = 200 type:error「query not found」→ UI 弹「✓query 已更新」（误报）
2. 「删查询」填假 ID + confirm 确认 → 真 DELETE designer/query/假 = 200 type:error → UI 弹「✓query 已删除」（误报）
3. 「改统计」填假 ID → 真 PUT designer/stat/假 = 200 type:error「stat not found」→ UI 弹「✓stat 已更新」（误报）
4. 「表行更新」填假表 flag + 假行 ID → 真 PUT designer/table/假/row/假 = 200 type:error「row not found」→ UI 弹「✓表行 update 已提交」（误报）
5. 「设计器检索」→ 真 POST designer/search = 200 data 数组（正常成功）→ UI 弹「✓查询设计器操作已提交」（此例响应成功，仅证明入口可达）

每次 writesDelta=1，newWrites 响应体为铁证；截图 qm-decisive-edit-query.png / qm-decisive-delete-query.png / qm-decisive-row-update.png 目视确认绿 toast 与 200+error 并存（已排除人造注入伪造质疑）。

## 五、通过项 / 取证限制 / 产品语义差异

通过项：confirm 取消守卫（删查询填值后点取消）writesDelta=0、无 toast、零写；非 2xx 注入 500→「✗更新 query 失败: HTTP 500: Internal Server Error」、403→「✗更新 query 失败: 没有权限执行该操作」正确 toast.error；空态「暂无查询定义」可见；0 pageerror；totalWrites=17。

取证限制：破坏性真状态翻转写（建/改/删真数据、deleteAll、批量执行）未授权未执行，用假 ID + 不拦截真后端写响应替代；失败态列表 500 注入未坐实（failState.fail=false、retry=0，据实记录，与 R388 同型）；全量约 60+ 写按钮按代表性覆盖（5 个真 UI 点击 + 10 个探针 + 源码行号），未逐一 UI 触发。

产品语义差异：ActionResult::error 用 HTTP 200 是 o2server 兼容既有设计；表行 deleteAll 假 ID success + deleted:0 属幂等删除；designer/search data 数组无 type 属正常只读成功——三者均不构成误报前提，据实记录不虚报缺陷。

## 六、三分判定与最小修复方向

判定：**1 个 P1 真缺陷**——10 个有真实 @click 入口的写函数在后端 HTTP 200 + type:error 业务失败下无条件弹成功 toast，三全满足（真 UI 可复现 + 真请求响应 + 源码定位）。

最小修复方向（对齐 R363 方案 B，本轮不改码）：新增 queryManagerWriteSucceeded(resp) 判 type 为 success（或 type 缺省且 success 非 false）为主、对带权威 bool/flag 的端点附加校验；10 个写函数 await 后先判响应再提示，失败时 toast.error 透传 resp.message，不再无条件 toast.success。

## 七、交付证据

- 文档：\`docs/ops/oa4rust-r392-pc-querymanagerdeep-write-audit.md\`（本文件），脱敏占位 0 真值、相对路径、非物理真机标注。
- E2E 脚本与证据（gitignored）：\`test-results/r392-pc-querymanagerdeep-write.cjs\`、\`test-results/r392-pc-querymanagerdeep-write/evidence.json\` 与截图 qm-dom-audit / qm-probe-done / qm-decisive-edit-query / qm-decisive-delete-query / qm-decisive-stat-edit / qm-decisive-row-update / qm-decisive-search / qm-cancel-guard / qm-http500 / qm-http403 / qm-fail-state / qm-empty-state.png。
