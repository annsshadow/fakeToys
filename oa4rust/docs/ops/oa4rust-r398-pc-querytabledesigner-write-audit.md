# R398 PC 表格设计器写链失败信封误报成功甄别

- 任务：t227 / R398（只读甄别，不改码）
- 页面：`frontend/apps/desktop/src/views/QueryTableDesignerApp.vue`（「表格设计器」，路由 `main.ts:266-268`，重定向 `main.ts:527`）
- 环境：Playwright Chromium headless 1440x900 对部署测试服实测；**非物理真机**，截图为 headless 浏览器画面
- 基线提交：`2f79823321ba58d40c64755c224cf5611ef807cb`（分支 opt1000/oa4rust-pc-ai-refactor）
- 测试服地址：`<YOUR_TEST_HOST>`（占位，真值不入库）；API 走相对路径 `/oa4rust-api`

## 结论：1 个 P1 真缺陷（用户可达，三全满足）

三条真实 UI 写链在后端返回 HTTP 200 + `type:"error"`（业务失败）时，前端不读信封 type，无条件弹出绿色成功 toast。真缺陷三要素全部满足：

1. **真 UI 可复现**：页面三个写按钮（保存定义 / 编辑结构 / 执行建表加列）真实渲染可点，无 EndpointFold 门控（源码 grep=0，DOM 审计 `fold=0`）。
2. **真请求响应**：真 UI 点击、写响应不拦截走真后端，捕获到真 `200 {type:"error",message:"table not found"}` 响应体，同时弹出成功 toast。
3. **源码定位**：见下方逐函数行号。

### 缺陷函数清单

| 函数 | 行号 | 请求 | 问题 |
| --- | --- | --- | --- |
| `saveTable` | :122-138 | :127-129 PUT `designer/table/{flag}` 或 POST `designer/table` | :130 读 `response.data?.tableFlag` 仅用于回填 activeFlag，未判 type；:131 无条件 `toast.success('表定义已保存为草稿')` |
| `editTableStructure` | :140-153 | :145 裸 `api.put` `designer/table/edit/{flag}` | :146 无条件 `toast.success('表结构编辑已提交')` |
| `executeTable` | :154-167 | :158 `api.post` `designer/table/{flag}/execute` | :159 虽把 `response.data` 赋给 `executeResult`（结果区展示真响应），但 :160 仍无条件 `toast.success('受限 DDL 已执行')` |

三处 catch（:133-134 / :148-149 / :162-163）仅处理非 2xx，200 + type:error 原样 resolve 落入成功分支。SDK `api.ts:82-88` 的 `parseJsonEnvelope` 不查 type，:275-296 仅非 2xx 抛错。

## 后端信封铁证（query_assemble_designer crate）

- `create_table_definition` `lib.rs:225-266`：缺 name→`AppError::BadRequest` 400（:229-231）；重名→`Ok(Json(ActionResult::error("table name already exists")))` **HTTP 200 + type:error**（:242-243）。
- `update_table_definition` `lib.rs:269-297`：假 flag / UPDATE 0 行→`ActionResult::error("table not found")` **HTTP 200 + type:error**（:289-291）。
- `execute_table_definition` `lib.rs:300+`：查不到表→`ActionResult::error("table not found")` **HTTP 200 + type:error**（:315-317）。
- `shared/response.rs:188` `ActionResult::error` 把 type 置为 `error` 且 HTTP 状态仍为 200（o2server 兼容式设计，R390-R397 已多次确证）。

## E2E 实测（1440x900 headless，非物理真机）

- **DOM 审计**：按钮 11 个；新建表 / 保存定义 / 执行建表 / 编辑结构 / 添加列 / 刷新各 1；`fold=0`；页面标题与注入行可见。
- **真后端假 ID 探针**：`tableCreateMissing`(空 name)=400 `type:error`（name is required）；`tableExecuteFake`(假 flag)=200 `type:error`（table not found）。另三条（tableCreateFake / tableSaveFake / tableEditFake）探针因脚本 evaluate 内 body 未随 fetch 送出返回 400「Failed to parse the request body as JSON: EOF」——此为探针脚本局限，已由下述决定性对照以真 UI 点击覆盖，据实记为取证限制。
- **决定性对照（写响应不拦截走真后端）**：先点注入的 `R398测试表 / R398-FAKE-FLAG` 行使 activeFlag 非空，再依次点击三个写按钮：
  - 编辑结构 → 真 `PUT designer/table/edit/R398-FAKE-FLAG` = `200 {type:"error",message:"table not found"}` → 弹「✓ 表结构编辑已提交」（截图 `dt-decisive-edit.png` 目视确认）。
  - 执行建表 → 真 `POST designer/table/R398-FAKE-FLAG/execute` = `200 {type:"error",message:"table not found"}` → 弹「✓ 受限 DDL 已执行」（截图 `dt-decisive-execute.png` 目视确认，两条绿 toast 并存）。
  - 保存定义 → 真 `PUT designer/table/R398-FAKE-FLAG` = `200 {type:"error",message:"table not found"}` → 弹「✓ 表定义已保存为草稿」。
  - `newWrites` 逐条捕获真响应体，排除注入伪造质疑。
- **人造 stub 对照（仅佐证前端不读 type，不单独判缺陷）**：注入 200 + type:error 后三处仍弹成功，证明前端成功判定与 type 无关。
- **非 2xx 正确报错**：编辑端点注入 500 →「✗ 结构编辑失败: HTTP 500: Internal Server Error」；403 →「✗ 结构编辑失败: 没有权限执行该操作」。
- **失败态 / 空态可区分**：列表 500 →「✗ 加载数据表失败: HTTP 500: Internal Server Error」+「暂无数据表」+ 刷新按钮；空 data →「暂无数据表」，无失败 toast。
- **0 pageerror**；`totalWrites=12`。

## 三分判定

- **真缺陷（P1，1 个）**：上述三条写链在 200 + type:error 下误报成功，三全满足。
- **通过项**：缺 name 走 400 非 2xx 正确错误；非 2xx 500/403 正确 toast.error；列表失败态与空态可区分；0 pageerror；`newTable` / `addColumn` / `removeColumn` 为本地状态无网络写，`openTable` 详情 GET 只读（catch 吞错）不判缺陷。
- **取证限制**：破坏性真写（真建表 / 真改表结构 / 真执行 DDL）未执行，用假 flag + 不拦截对照替代；三条探针因 body 未送达返回 400 EOF 据实记录，误报由决定性对照真 UI 点击坐实；「保存定义」在 activeFlag 为空走 POST 新建的分支未单独 UI 触发（同函数同型，按代表性覆盖）；本页三处写均无 confirm / prompt 守卫（直接执行），无取消零写可测。
- **产品语义差异**：`ActionResult::error` 走 HTTP 200 是 o2server 兼容既有设计；create 缺 name 走 400 而假 ID / 重名走 200 error，属既有混合口径差异。

## 最小修复建议（本轮不改码）

对齐 R390-R397 统一范式：新增 `queryTableWriteSucceeded(resp: any)`，以 `resp?.type === 'success'` 为主判定（不强求权威 bool，本批端点无权威 bool）；`saveTable` / `editTableStructure` / `executeTable` 接响应后先判成功再弹成功 toast，失败则 `toast.error` 透传 `message`，不再弹成功。`executeTable` 的 `executeResult.value` 保留展示真响应（含失败信封）本身无害，仅成功提示需按 type 判定。

## 证据

- 截图（gitignored）：`test-results/r398-pc-querytabledesigner-write/dt-dom-audit.png`、`dt-probe-done.png`、`dt-decisive-edit.png`、`dt-decisive-execute.png`、`dt-decisive-save.png`、`dt-stub-edit-exec.png`、`dt-http500.png`、`dt-http403.png`、`dt-fail-state.png`、`dt-empty-state.png`。
- 数据：`test-results/r398-pc-querytabledesigner-write/evidence.json`（含逐次写请求真响应体）。
- 脚本（gitignored）：`test-results/r398-pc-querytabledesigner-write.cjs`、`test-results/r398-pc-querytabledesigner-write/run.sh`。
