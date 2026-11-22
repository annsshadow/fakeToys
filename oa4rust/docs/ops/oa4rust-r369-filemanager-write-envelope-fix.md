# R369 PC FileManager.vue 写链失败信封误报成功 P1 修复

- 任务：t185 / R369。依赖 t184 / R368 只读甄别（文档 `oa4rust/docs/ops/oa4rust-r368-pc-filemanager-write-audit.md`）坐实 1 个 P1。
- 性质：真实 bug 修复，test-first RED→GREEN。
- 测试环境：Playwright Chromium headless **1440x900 非物理真机**；测试服域名与凭据运行期从环境变量/远端凭据文件读取，文档仅用占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，0 真值入库。
- 证据目录：`oa4rust/test-results/r369-pc-filemanager-write-fix/`（gitignored，不入库），含 `evidence.json` 与 5 张 1440x900 截图。

## 一、缺陷与裁决

**缺陷**：`createRecyclableFile()` / `deleteFile()` 两处真 `@click` 写函数在后端 HTTP 200 + 业务失败信封（`type:error`）时无条件 `toast.success`，用户误以为操作已生效。R368 真 DELETE 假 ID → 200 + `type:error`「file not found」仍弹「✓文件已删除」；create 失败还会用空 id 造出一条记录。

**队长裁决（type 判定为主，不照搬审计里严格 `bool===true`）**：后端 `file_create` 成功信封 `success({id,name,...})` 无权威 bool 字段，强制 bool 会假阴性。故采用：
1. 新增本地 helper `writeSucceeded(resp:any):boolean`：`type==='error'` 或 `success===false` 判失败；`type==='success'` 或（`type` 缺省且 `success!==false`）判成功。
2. `createRecyclableFile`：`await api.post` 后先判 `writeSucceeded` 且 `data.id` 非空才构造记录并成功提示，否则 `toast.error(resp?.message || '操作未生效，请刷新后重试')`，不造空 id 记录。
3. `deleteFile`：`deleteMutation.mutate` 的 `onSuccess(r:any)` 先判 `writeSucceeded(r)`，失败则错误提示并 return，不 filter、不刷新回收站、不成功提示；成功才落态。
4. 端点路径 / payload / `confirmMsg` 取消守卫 / `catch` 非 2xx 处理 / 其它函数一律不变。

## 二、源码改动

- `oa4rust/frontend/apps/desktop/src/views/FileManager.vue`：插入 `writeSucceeded` helper；`createRecyclableFile` 接响应并判 type + data.id；`deleteMutation.mutate` 的 `onSuccess` 接响应并判 type。外层 `deleteMutation` 自身 `onSuccess`（invalidateQueries + loadFiles）未改。
- 未改后端 / SDK `api.ts` 传输层 / 其它 views / migrations。

## 三、契约 RED→GREEN

- 新建 `oa4rust/frontend/apps/desktop/src/views/FileManager.writeEnvelope.contract.test.ts`：源码级 `bodyOf` 花括号配对提取 + `stripTs` 剥离 TS 标注 + `new Function`/`new AsyncFn` 真执行，注入 api/toast/deleteMutation 桩（参数带 `:string` 防 TS7006），非文本断言。16 it：helper 6 + create 5 + delete 5。
- RED（还原 t184 版源码）：9 failed / 7 passed，失败全为 helper 缺失与两函数无守卫，信号干净。
- GREEN：16/16 passed。
- 连带维护：`FileManager.recycleRefresh.contract.test.ts` 旧契约把 `onSuccess: () => {` 写死为锚点，合法签名改 `(r: any)` 后失效；同步该锚点后全量恢复通过（属契约断言同步，非业务逻辑回归）。

## 四、验证台账

- R369 单契约：`cd oa4rust && pnpm exec vitest run frontend/apps/desktop/src/views/FileManager.writeEnvelope.contract.test.ts` → 16/16，exit 0。
- 根全量：`cd oa4rust && pnpm exec vitest run` → 279 files / 2366 tests，0 failed，exit 0。
- 类型检查：`cd oa4rust/frontend/apps/desktop && pnpm exec tsc --noEmit` → exit 0。
- PC 构建：`pnpm exec vite build` → exit 0，主入口 `index-DERUwKuV.js`（FileManager 为懒加载 chunk）。
- 测试服 web 整目录 tar 原子替换部署（远端 `/usr/local/oa4rust/web`），LOCAL/REMOTE sha256 比对 + grep 入口一致后切目录。

## 五、真浏览器实测（1440x900 headless，非物理真机）

注入对照 8 断言全 true：
- create type:error → 显示后端 message「后端拒绝：R369 create rejected」，无成功 toast、可清理文件区仍「尚未新建文件」（未造空 id 记录）。
- create HTTP 500 → 正确「HTTP 500」错误提示、无成功 toast。
- create type:success → 仍成功提示且新增记录。
- delete confirm 点「取消」→ 零 DELETE 写、无成功提示。
- delete type:error → 显示后端 message「后端拒绝：R369 delete not found」、无「文件已删除」、注入行仍在。
- delete type:success → 仍成功提示并落态。
- 全程 0 pageerror。

截图（`test-results/r369-pc-filemanager-write-fix/`，1440x900，Playwright Chromium headless，**非物理真机**）：`01-create-typeerror-fixed.png`、`02-create-trueSuccess.png`、`03-delete-cancel-guard.png`、`04-delete-typeerror-fixed.png`、`05-delete-trueSuccess.png`；`evidence.json` 的 `source` 明标非物理真机。

## 六、取证限制与产品语义

- 破坏性真状态翻转（真实创建/删除真文件）未执行：写响应用页面真实 route 注入对照，非物理真机；后端 200+type:error 语义由 t184 真探针坐实。
- 后端 `ActionResult::error` 用 HTTP 200 + type:error 是 o2server 兼容既有设计，本轮不改。
- `confirmMsg` 自定义 DOM 弹窗（非原生 confirm）属既有设计，守卫保持不变。
