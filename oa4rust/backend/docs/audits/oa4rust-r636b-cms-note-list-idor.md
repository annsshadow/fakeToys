# R636b CMS 笔记列表身份盲读 IDOR 验证型甄别审计（t315）

## 结论
判定 1 项真实可达缺陷，严重度 **P2**：`cms_entity_note_list` 身份盲读，任意已登录用户在 `/api/cms/core/entity/note/list` 可读取**全量** `x_cms_note`（含他人笔记标题与正文全文），无 `person_id` 归属过滤、无 admin 分流；而同族写端点（create/save/delete）却强制 `require_admin_opt`，读写权限倒挂，证明是遗漏而非产品口径。

## 立项性质与范围
- 验证型甄别：**零源码改动**，唯一交付物为本审计文档；oneshot 夹具与日志留本地证据目录，均不入库，夹具用后即删。
- 真父 HEAD=9bd06a53a422f85feecc80839f1576d1fe3df9d4（派发基线；工作期间并发推进至 0c4df314，本甄别基于前者源码，SHA 见下）。
- 台账 `docs/ops/current-state.md:4372-4373` 仅记 R310「笔记删除不进回收站=产品口径」，与越权读无关；note/list、x_cms_note 无专项甄别。

## 机制锚点（逐行核实）
- `crates/cms_core_entity/src/lib.rs:386-429` `list_cms_entity_table(pool: &Pool, table: &str)`：`:407-411` x_cms_note 分支 SQL `SELECT id,title,content,person_id AS personId,... FROM x_cms_note WHERE deleted_at IS NULL ORDER BY create_time DESC,id`；`:418-421` `client.query(sql, &[])` 空绑定，无身份参数、无归属谓词、无 admin 分流；`:422` `row_to_json` 整行序列化；`:424-428` `legacy_success(Value::Array(data),count,0)`。
- `:463-469` `cms_entity_note_list(pool: Extension<Pool>)` 签名**无 Session**；`:876` 注册 `.route("/api/cms/core/entity/note/list",get(cms_entity_note_list))`；`backend/src/lib.rs:178` 合并进主路由。
- 写端点反证：`:801/:818/:836` 三写端点首行 `:806/:824/:841` 均 `require_admin_opt(&pool.0,session).await?`——写需 admin、读裸读，权限倒挂。
- 全局鉴权 `shared/src/middleware/token.rs:135-181`：`auth_middleware` 仅验证会话并 `insert(session)`（`:174-180`），不做资源归属过滤；无认证头/失效会话 → 401（`:157-159/:179`）。
- 前端 `NoteApp.vue:82` `listEp='/api/cms/core/entity/note/list'`；`:96-107` `useQuery` 调 `api.get(listEp)` 无身份参数；`:28-32` 表格 `:30 {{ item.content||'—' }}` **正文直接渲染**。`main.ts:114` `{path:'note',...meta:{title:'笔记管理'}}` 路由可达。
- migration `093_create_cms_core_entity_tables.sql:55-65` 建表含 `content TEXT`、`person_id TEXT`，归属数据齐备而被忽略。

## 方法（真测试）
- 独占 PG 容器 `<TEST_HOST>:<TEST_PORT>`（避开他组 15440/15534），真 migration 全量建表，scratch 库 `oa_r636b_*` 用后即清。
- `oneshot` 直调 `cms_core_entity_router(pool)`：`app.clone().layer(Extension(test_session("r636b-alice")))` 注入 alice 会话请求 note/list；seed 一条 bob 私有笔记（title/content/person_id=bob）。
- 匿名反臂：另建挂 `auth_middleware` 的 Router，无 token 请求必须 401。
- 源码守卫：`#[test]` 读 `src/lib.rs` 断言 list handler 无 Session、SQL 空绑定无 `person_id = $`、三写端点各含 `require_admin_opt(`。

## 决定性实测
| 观测 | 结果 |
| --- | --- |
| alice 请求 note/list status | 200 OK |
| 返回行数 | 1（全表含 bob 笔记） |
| title / content / personid | r636b-bob-private-title / r636b-bob-private-body-secret-note / r636b-bob |
| content 是否 bob 正文全文 | 是（逐字断言相等） |
| 写端点 admin 守卫 | 3 处 `require_admin_opt(` |
| 匿名无 token | 401 Unauthorized |
| 测试汇总 | 3 passed; 0 failed（TEST_EXIT=0） |

## 三分判级
- 泄露性质：笔记**正文全文**（业务数据），**非凭据**（无 token/密码/密钥）——不升 P1。
- 前置条件：**需有效登录会话**（匿名 401 反臂坐实）——利用门槛存在，不升 P1。
- 写操作：**无落库写**（纯读端点）——不改变数据完整性。
- 对照 R598（会话列表盲读判 P2）：同性质身份盲读；本例正文全文 + 前端直接渲染，危害更直观，故在 P2 内属偏高；但受「需会话/非凭据/无写」三项约束，**不硬凑 P1**。
- 综合判定 **P2**：跨身份全表泄露私有业务正文，属真实可达越权读，但非凭据泄露、不需认证绕过、不写库。

## 行为现状 / 可复现
任意登录用户访问「笔记管理」页即经 `api.get(listEp)` 拉回全量笔记，前端表格逐行渲染他人 `content`；无需特殊参数。复现见上「方法」段夹具步骤。

## Fail Loud
- 首轮夹具自身三处假设错误：①`AS personId` 未加引号被 PostgreSQL 折叠为 `personid`，断言键名错误；②源码区间 `f_end` 从头查找导致 `f_end < f_start`；③`client.query` 实际换行书写，字面量匹配失败。均已修正，**未采信失败运行**。
- Windows 下 cargo 未感知 mtime 变化、复用旧编译产物（报错行号与旧代码吻合），已 `touch` 源文件 + 删除旧 exe 强制重编译后重跑通过。
- 临时 PG 为隔离实例，仅用 `<TEST_HOST>:<TEST_PORT>` 占位，无真值入库；scratch 库用后即清、容器用后即删。

## 证据 sha256（据实分列）
- docSha（本文件除本行外正文 sha256，避免自指） = 7e3cc7740fe3ab1a7aebf1a747d2475aa30f8ae91c4bb8653623c7085e394d19
- runnerSha（夹具文件 sha256sum，可复算） = 3b8c4855d1c7c0ea4aca984484cae6a360cd8c5f5183344465e49564777002a1
- 证据日志 r636b-probe.log sha256sum = 291c280bb6457ef5dac3fe6aa4a24f1021c7b0fa38366e2ffa8786e720dbed48
- 源码 cms_core_entity/src/lib.rs = a95bf0b09f2f9168df8dec399bd205b652b4708cfae2c961283dc1318cc24902
- 前端 NoteApp.vue = a5bf5f60edcf72d60d74898b315f626460176ed9d50eca1e1c749d038455f7fa；main.ts = e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0
- token.rs = b76f9d38ced1a83e6733386c2f293655f5967e9870c68e4d82f4f38ad97bd707；backend/src/lib.rs = fff5ae0dea271fd66291a346e1a00350afe57f2998fb526ab355cdd236ef077b；migration 093 = 9879e64a7404eb191306dad3a333d67d2793bd5c8c4ba8ca17593fb84dd34e57

## 修复方向（建议，不实施）
为 `cms_entity_note_list` 增加 Session 参数，SQL 以 `person_id = $1`（`$1` 取 `session.person_unique`，admin 放行全量）收敛；同族 column/index/module/column_manager 裸读端点一并按归属/角色收敛，与写端点 `require_admin_opt` 对齐。
