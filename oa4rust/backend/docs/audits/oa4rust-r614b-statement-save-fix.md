# R614b QueryStatementDesigner 保存按钮 saveStatement 死绑定真修复

- 任务：t216 / R614b（implementation，真修复 RED→GREEN）
- 真父：7f79cb7e0764e51e1207f20bc8952ee4305208a9（git rev-parse 核实，HEAD 全程未变）
- 裁决：P2 已修（主保存按钮静默 no-op；原缺陷存在替代路径，不越权、不写库、不丢数据）

## 根因与锚点
- oa4rust/frontend/apps/desktop/src/views/QueryStatementDesigner.vue:15 模板 @click="saveStatement" 绑定未定义函数；全 frontend+mobile grep 该名仅此一处，script setup 六种定义形式均无。
- 真值可达性：:929 currentStatement=ref(null)，:1024-1026 selectStatement(s) 选中后按钮启用，但点击无 handler。
- 替代路径（R613b 坐实）：:16/:1060 saveByFlag（prompt flag，语义不同）；:141/:1079 modalSave→saveM.mutate。
- 后端契约：PUT /api/query/assemble/designer/save/{id} 为按 id 覆写端点（saveM:1039-1047 已用）；:1050-1056 queryStatementWriteSucceeded 提供 envelope 守卫。

## 修复（surgical，仅写域 3 文件）
- QueryStatementDesigner.vue：在 queryStatementWriteSucceeded 之后、saveByFlag 之前新增 20 行 async function saveStatement()：
  取 currentStatement.value，空则零写；PUT save/{encodeURIComponent(id)} 带 {...item, sql: sql.value}；
  经 queryStatementWriteSucceeded 守卫，失败 toast.error(resp.message) 不 invalidate，成功 toast.success + invalidateQueries(['stmt','list'])，异常走 catch。
- 不复用 saveByFlag：其 prompt('目标 flag…') 为交互副作用且语义为「按标识覆写」，与「保存当前选中语句」不等价，据实披露。
- 未改其他按钮/函数；模板 :15 绑定零改。

## 契约与 RED→GREEN
- 契约 oa4rust/frontend/apps/desktop/src/views/statementSave.contract.test.ts（@vitest-environment node）：readFileSync 源码+括号配平抽函数体，new Function 注入 ctx（currentStatement/sql/api.put/toast/queryClient/envelope 守卫）动态执行。
- 8 用例：静态绑定+函数存在/按 id 端点+无 prompt/成功 PUT+payload+toast+invalidate/type:error 拦截/success:false 拦截/非对象响应拦截/HTTP 异常 catch/未选中零写。
- RED replay（本人亲跑）：CAS 删函数恢复真父源码 sha=c71dba6a… → pnpm vitest run 定点契约 EXIT=1，7 failed / 1 passed（8）。
- GREEN replay：CAS 重插函数恢复修复态 sha=8c00655c… → EXIT=0，8 passed（8）。HEAD=7f79cb7e 全程未变，未用 git checkout。

## 三 verify（全 EXIT0 项据实；全量 vitest 另述）
- V1 定点：cd oa4rust/frontend/apps/desktop && pnpm vitest run src/views/statementSave.contract.test.ts → 1 file / 8 tests passed，EXIT=0。
- V2 typecheck：同目录 pnpm typecheck → tsc --noEmit EXIT=0；盲区：tsc --noEmit 不解析 .vue 单文件组件，不能替代契约。
  V2 定点 vitest 同上 EXIT=0。契约首版曾因隐式 any 触发 TS7006/TS7034 与 TS2554，已补全类型注解后归零。
- V3 biome：cd oa4rust && node_modules/.bin/biome check frontend/apps/desktop/src/views/statementSave.contract.test.ts → EXIT=0（12 warnings 为 any 使用提示，非 error）。
  V3 全量 vitest：同目录 pnpm vitest run → EXIT=1，仅因 2 个既有基线 suite 加载失败（tabStateA11y.contract.test.mjs 无 test suite；TaskCompleted.R314.contract.test.ts 需 backend cwd 致 ENOENT）；2257 tests 全 passed、282 files passed / 2 files failed(284)。两基线文件 git status 干净、非本次引入，Fail Loud 据实披露，不冒充 pass。

## 三分判定（据实）
- P2 已修：默认主保存按钮静默失效属局部可感知交互缺陷，已补真函数；原缺陷不越权、不写库、不丢数据，且有替代保存路径。
- 排除 P0/L1/0：非全局崩溃、非认证前可达、非 0 优先级噪音；仅前端按钮绑定缺陷，无后端写权限放大。

## Fail Loud 与脱敏
- 未跑浏览器 GUI 真页面；用 node 环境源码提取+函数体求值等价覆盖 handler 绑定与 API/envelope 行为，据实披露。
- 未引入任何密钥/口令/真实域名 IP/个人路径；测试 id、flag、message 均为伪造占位符；文档 0 真值。
- 写域精确 3 文件，零其他 .vue/后端/文档改动；git add 用单引号指定路径，禁 -A/.。

## 校验标识（runnerSha 与文件 sha256 分列）
- runnerSha=3687d9c31f961216fd7df0964c4da3a8b4ddca51e4e25965eb8520eb2a90c576
- runnerSha 口径：sha256(JSON.stringify(meta)) 紧凑 JSON，meta 含 task/round/verdict/trueParent/red/green/fix/contract。
- 真父源码 sha256=c71dba6aa59f54ba576cfc3599f5fa39936b5e6ed133392f96d363c23575371b
- 修复态源码 sha256=8c00655cac1b532fe182d16ae251c3f1d7421e13fc1722763079fcf7a97be7f7
- 契约 sha256=bb93a10f4a5ef0bdaaf102edcc0e59fb5b8e22a06bf77771f128cbff2c466e72
- 本文件不含自身哈希。
