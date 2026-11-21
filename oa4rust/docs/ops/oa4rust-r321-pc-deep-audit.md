# R321 PC 深度功能审计第二轮（未覆盖模块）

## §1 来源与边界

- 任务：t105 [R321 PC 深度功能审计第二轮（未覆盖模块）]，kind=work（只读审计），分支 opt1000/oa4rust-pc-ai-refactor（禁 main）。
- 入口：PC /oa4rust/app/，Playwright Chromium headless 1440x900，真实 UI 登录后逐模块走真实 DOM 入口，监听 /oa4rust-api/api/ 记录 HTTP 状态与页面回显。
- 测试服地址/凭据：运行期从 gitignored 部署脚本模板与 .local.keys 读取，不入库、不回显；文档统一 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，0 真值。
- 证据：oa4rust/test-results/r321-pc-deep-audit/（gitignored），evidence.json 的 source 明确标注 Playwright Chromium headless against deployed test server; not a physical device（本地无头浏览器渲染，非物理真机）。
- 纪律：每条发现区分①真缺陷②取证限制③产品语义差异，必须真 UI 可复现 + 真请求/响应 + 页面回显 + 源码定位；不臆造，不重复报 R300/R307/R309/R311/R314/R315/R316/R318 已闭环项。

## §2 逐模块真实浏览器结论

| 模块 | 真实 UI 流程 | 真请求/响应 | 页面回显 | 判定 |
|---|---|---|---|---|
| 通知公告 | /app/dashboard 通知区，读取列表并点「刷新通知」 | GET instant/list/currentperson/count/20/desc 200（刷新前后各一次） | 刷新前/后均 1 条 test 11/17 18:17，refresh 入口存在 | 通过（能力可用；无未读流转数据，R316 L2 已列取证限制，不重复） |
| 文件管理（新建→删除） | 点「新建可清理文件」→原生 prompt 填名 →可清理列表点删除→自定义确认弹窗确认 | POST /api/file/core/entity/file 200；DELETE /api/file/assemble/control/file/{id} 200 | 可清理列表出现 R321审计文件502913；删除后列表清空、toast「文件已删除」；回收站列表仍空 | 真缺陷候选，见 §3 D1 |
| 文件管理（上传入口） | 点「上传」弹窗检查 input type=file multiple | 本轮未提交真实文件（不伪造上传） | 上传弹窗与文件选择器入口存在（FileManager.vue:149-163） | 入口存在；上传闭环本轮未取证，非缺陷 |
| CMS 笔记 | /app/note 新建→填标题/正文→保存→查列表 | GET note/list 200；POST note/create 200；GET note/list 200 | 新建 R321审计笔记08566 出现在列表，rowCount=3 | 通过 |
| 论坛回复 | /app/bbs 真实 DOM 查找主题帖→开详情→回复 | PUT subject/index/list/page/1/count/20 200；GET section/list 200 | 当前干净数据主题列表为空（bbs-topic-empty），无 bbs-topic-{id} 可点；脚本初次误选版块项，已用精准选择器复测确认 topics=[] | 取证限制：无主题帖可回复，非缺陷 |
| 组织通讯录 | /app/directory 搜索 a、翻页、部门树 | POST person/list/filter/1/size/20 200（×2）；GET departments/tree 200（×2） | 共 21 人 · 第 1 / 2 页，5 人 + 2 部门节点，无空态/错误态 | 通过 |
| 组织架构 | /app/org 查看组织树 | PUT group/list/like 200 | h1=组织架构，树节点 8 | 通过 |
| 个人中心 | /app/personal 查看资料/签名/密码/委托入口 | GET signature/list 200；GET empower/list/currentperson 200 | h1=个人中心；personal-password/personal-signature 存在；签名计数「个人签名 0/200 保存签名」 | 通过（未改真数据：改密码/签名为写操作，非本轮目标） |
| 系统设置 | /app/settings 切到「安全设置」分组 | GET config/system 200 | h1=系统设置，navCount=4，切到第 2 项显示「当前设置分组：安全设置」，面板含密码最小长度/登录失败锁定/会话超时/允许注册/保存设置 | 通过 |

全程 pageErrors=[]、所有被监听的 /oa4rust-api/api/ 响应无 >=400；截图 10 张落证据目录，均标注非物理真机。

## §3 真缺陷

### D1（P2）文件管理「新建可清理文件」删除后不进回收站，页面承诺与真实行为不符

- 可复现路径：登录 → /app/file → 点「新建可清理文件」→ prompt 输入任意名称 → 可清理列表出现该项 → 点该项删除图标 → 自定义确认弹窗点确认 → 观察右上角 toast「文件已删除」、可清理列表移除该项、右侧「回收」区点「刷新回收站」→ 仍显示「回收站为空」。
- 真浏览器证据：test-results/r321-pc-deep-audit/evidence.json 的「文件管理」段：created.names=[R321审计文件502913]、deleteFlow=confirm-dialog-found、after.names=[]、after.recycle=[]、after.toast=true；API 链 POST /api/file/core/entity/file 200 → DELETE /api/file/assemble/control/file/{id} 200 → GET file/list/root 200；截图 02-file-created.png、03-file-after-delete.png（sha256 d6d48d03ca8d92cf6b526c17bb4779e2d0c8fe8e3e04705469a8c4e137a4ab57）。
- 源码定位：
  - 承诺文案 oa4rust/frontend/apps/desktop/src/views/FileManager.vue:85 title=删除（进入回收站），:210-211 新建成功 toast「文件已创建，可删除进入回收站」。
  - 删除实现 FileManager.vue:896-909 的 deleteFile()：deleteMutation 仅 api.delete(/api/file/assemble/control/file/{id})（:891），onSuccess 仅 recyclableFiles.value.filter 本地移除 + toast 成功（:900-902），从未调用 loadRecycleBin()（定义于 :878，GET /api/recycle/list）。
  - 后端软删存在：oa4rust/backend/crates/file_assemble_control/src/lib.rs:532 UPDATE x_file SET deleted_at = NOW() WHERE id = $1；回收站列表读 FILE_FILE WHERE deleted_at IS NOT NULL（同文件 recycle_list 约 :3995）。但新建走 /api/file/core/entity/file（file_core_entity 表族），删除走 /api/file/assemble/control/file/{id}（x_file 表族），回收站读的又是 FILE_FILE，因此即使后端软删成功也不会在该新建文件的回收站出现。
- 性质判定：真缺陷（页面明确承诺「删除后进入回收站」，真 UI 可复现，API 200 但回显与承诺不符）；不是接口失败，也不是 R314 已办 / R315 搜索 / R318 论坛版块桥接已闭环项。
- 修复方向（不在本只读任务内实施）：统一新建—删除—回收站三者的表族与记录 id（选定唯一文件实体表），删除成功后主动刷新回收站并按真实软删结果回显；若产品语义是不入回收站，则同步改页面文案与 title，消除误导。验证方式：新建后删除 → 回收站列表出现同名项 → 恢复后回到可清理列表 → 0 pageerror / 0 API 失败，并加契约锁定三链同源。

## §4 取证限制与产品语义差异（不计缺陷）

1. 论坛回复闭环：真实 bbs-topic-{id} 主题帖 0 条（列表仅 bbs-topic-list-title / bbs-topic-empty），因此 bbs-topic-detail / bbs-reply-input / bbs-reply-submit 无法进入；脚本初次按前缀选择器误选版块导航项，已用 BBSForum.vue:61-101 的准确结构复测确认。待有真实主题后再审计回复链，不臆造。
2. 通知已读流转：instant/list 列表 1 条且会话 unread=0，无未读徽标/「全部已读」入口可触发；R316 L2 已列同类取证限制。
3. 个人中心写操作：修改密码、保存签名、上传真实文件均为写数据动作，本轮仅验证入口与只读列表 200，不伪造提交，不算缺陷。
4. R318 论坛跨端不同源（PC section 与 H5 forum）已有桥接修复文档 oa4rust/docs/ops/oa4rust-r318-bbs-section-bridge-fix.md；本轮论坛无主题可发帖，未重复审计该链路。
5. R300 A2/A3、R307 组织树、R309 笔记、R311 建日历、R314 已办同步、R315 搜索信封等历史项均已在既有文档闭环，本轮只做回归观察，不重复报。

## §5 证据索引（均 gitignored，本地无头浏览器非物理真机）

- 目录：oa4rust/test-results/r321-pc-deep-audit/
- 脚本：r321-pc-deep-audit.cjs（真实 UI 登录 + 逐模块 DOM 流程 + API 监听 + 原生 prompt/自定义确认处理）
- 结构化结果：evidence.json（8 模块、pageErrors=[]、无 >=400 响应）
- 截图：01-dashboard-notice.png、02-file-created.png、03-file-after-delete.png、04-note-created.png、05-bbs-reply.png、05b-bbs-topic-detail.png、06-directory.png、07-org.png、08-personal.png、09-settings.png

## §6 结论

R321 PC 未覆盖模块第二轮真浏览器审计完成：7 个模块通过、0 pageerror、0 API 失败；发现 1 个新增真缺陷 D1（文件管理新建可清理文件删除不进回收站，承诺与事实不符，含可复现路径、真证据与源码定位），论坛回复与通知已读等归为取证限制。项目按 ≥1000 轮持续迭代，本轮为其中一环，非终态。