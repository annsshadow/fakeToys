# R326 D1 会议搜索/筛选不生效修复

## 1. 现象与根因

PC 会议管理 /app/meeting「我的会议」搜索框输入无匹配关键词并按「搜索」后，列表仍显示所有会议，无空态；状态下拉筛选同样无效。

双端根因（R326 审计已坐实）：

- 前端 frontend/apps/desktop/src/views/MeetingApp.vue:208-212 的 loadMeetings() 构造了 const p = {} / if (searchKey) p.key = ... / if (statusFilter) p.status = ...，但 :212 调用固定 URL /api/meeting/assemble/control/meeting/list/apply/1/size/50，p 是死变量，key/status 从未进入请求。
- 后端 backend/crates/meeting_assemble_control/src/lib.rs:1054-1079（旧）meeting_list_apply_page_size_size(pool, session, Path((page,size))) 只接受 Path，SQL FROM x_meeting WHERE applied=true AND (归属谓词) ORDER BY create_time DESC LIMIT $1 OFFSET $2，无 key/status 过滤能力。

## 2. 方案偏差台账（据实记录）

- 队长初版建议 SQL status = $5 等值匹配。真实探测（测试服 API，脚本 gitignored）发现：前端 select 传值为 0/1/2，而 x_meeting.status 实际为 NULL（新建会议 R309 时 status 默认 NULL），历史值域含英文串 wait/processing/completed 等；直接等值会漏掉全部现存会议。
- 队长两次裁决（m03527、m03544）同意改为 tone 兼容映射，与 frontend/.../meetingStatus.ts:12-15 的 statusTone 同源：0=pending（NULL/未识别）、1=active（1/processing/accepted/confirmed）、2=ended（2/completed/denied/rejected）；非法值不过滤，保证向后兼容。
- inScope 由 5 文件修订为 6 文件（队长 m03544 批准），新增 backend/crates/meeting_assemble_control/src/security_gate_tests.rs，仅因 handler 新增 Query 参数后唯一一处直调需补空 Query，未改任何断言/种子/归属谓词语义。

## 3. 修复实现

前端 loadMeetings()：用 URLSearchParams 拼 key/status 到真实列表 URL，其余逻辑不变。

后端 meeting_assemble_control/src/lib.rs：

- 新增纯函数 meeting_list_filter_clause(key, status) 返回 (extra_sql, key_pattern, status_value)，key 用 ILIKE 参数绑定并对百分号、下划线、反斜杠三种 LIKE 通配符转义；状态 tone 映射用 IN/IS NULL 组合；$4::text IS NULL / $5::text IS NULL 显式类型标注规避 PG 42P08「could not determine data type of parameter」；SQL 文本不含任何用户输入。
- handler 新增第 4 参 axum::extract::Query<HashMap<String,String>>，SQL 追加过滤片段并绑定 $4/$5。applied=true、归属谓词 APPLIED_CREATOR_READ_PREDICATE($3)、分页 $1/$2、R309 两修复均保持不动；SELECT 投影与卡片字段未改。
- routes.rs:42 GET 与 :131 POST 自动注入 Query，无需改路由。

## 4. RED → GREEN 证据

- 前端契约 frontend/apps/desktop/src/views/MeetingApp.meetingSearch.contract.test.ts（5 it，读源码 new Function 真执行 loadMeetings + api.get stub 收集 URL）：修复前 3 failed / 2 passed（key/status 无法进入 URL），修复后 5/5 passed。
- 后端 backend/crates/meeting_assemble_control/src/tests_generated.rs 末尾 5 个 r326_filter_* 单测（key 转义、统一片段、pending 显式 NULL、防注入、空值 noop）：修复前 5/5 failed，修复后 5/5 passed。
- 修复过程中 cargo test -p meeting_assemble_control --lib 曾因新 SQL 的 $4 类型不确定返回 500（tests_u2.rs:775），临时诊断日志定位为 PostgreSQL SQLSTATE 42P08，加 ::text 显式标注后消除，临时日志已移除。
- 最终回归：cargo test -p meeting_assemble_control --lib 66 passed / 0 failed / 30 ignored；pnpm exec vitest run --reporter=dot 267 files / 2152 tests，0 failed；pnpm run typecheck exit 0。

## 5. 测试服真浏览器复验（非物理真机）

入口：测试域占位 <YOUR_TEST_DOMAIN> 下的 PC /app/meeting，Playwright Chromium headless，viewport 1440x900；凭据经部署机远端脚本运行期读取，未入仓、未回显。

脚本 test-results/r326-search-fix-check.cjs（gitignored）依次执行：登录 → 初始列表 3 条 → 输入 ____NO_SUCH_MEETING____ 搜索 → 0 条并显示空态 → 输入 R326审计 → 1 条命中且标题正确 → 状态 0（未开始）→ 3 条（NULL 归入 pending，兼容存量）→ 清空筛选 → 恢复 3 条。监听 pageerror 与 /oa4rust-api/api/ 4xx/5xx。

9 项断言全部通过：noMatchZero、emptyStateShown、keywordMatch、keyParamSent、statusParamSent、pendingIncludesNull、resetRestores、noPageErrors、noApiFailures；pageErrors=[]、apiFailures=[]。

独立 API 探针（gitignored）结果：无参 count=3；key=____NO_SUCH_MEETING____ 得 count=0；key=R326 得 count=1；status=0 得 count=3；status=9（非法值兜底不过滤）得 count=3。证明过滤真实由后端 SQL 生效，而非前端隐藏。

截图（均 1440x900，来源 Playwright Chromium headless against deployed test server; not a physical device，存放 gitignored test-results/r326-search-fix/）：

- 01-no-match-empty.png sha256 9956cbaae9ff53a01e1aedfe3b326bf289f4059bf2abaa046f41593eaab0dd3a —— 无匹配关键词后空态。
- 02-keyword-match.png sha256 4704e76ddbbecac3000cd65f804a32e1da1d74ec037bef17c932f00eeb76e4a2 —— 关键词 R326审计 命中 1 条会议卡。
- 03-reset.png sha256 11b0e5171c2bcb85616159a10e0ea3da715fbf5bf0308c7d9e175db47456c036 —— 清空筛选恢复列表。

## 6. 结论

R326 D1（搜索/筛选不生效）已修复并部署复验通过；R309 既有修复未回退，构建/测试/真浏览器全绿。本轮非终态，后续轮次继续。
