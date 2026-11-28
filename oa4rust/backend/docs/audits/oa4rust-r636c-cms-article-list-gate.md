# R636c CMS 文章列表绕过 read_gate 身份盲读甄别审计

## 结论
- 真父：9bd06a53a422f85feecc80839f1576d1fe3df9d4（观察 HEAD=6a2f84c6c0，队友并行提交；锚点源码真父..HEAD 零 diff）。
- 纯甄别：零源码改动；临时 oneshot 夹具测后即删，仓库无残留（tests 目录残留为队友 R636b 契约测试，未触碰）。
- 三分判定：可达真缺陷 1 项（P2，认证后列表端点绕过 read_gate 泄露他人未发布草稿正文）；不可达 L1 0 项；正常项 3 类。
- 判级说明：需有效会话 + 前端未直接渲染正文 + limit(20) 为减级因素；但泄露的是未发布草稿全文且与 article_read_gate（同一 id 单读 403）形成明确访问控制矛盾，属认证后身份盲读族，与 R625b/R629c/R623b 同档 P2。不预支 P1，不降 L1。

## 立项范围与机制锚点
- 目标：oa4rust/backend/crates/cms_core_entity/src/lib.rs:227-272 article_list（GET /api/cms/article/list）。
- :227 签名仅 db:Extension<DatabaseConnection>，无 session 参数；:230-236 Entity::find().filter(DeletedAt.is_null()).order_by_desc(CreateTime).limit(20).all() 无 status 过滤、无 author 谓词。
- :248-250 content 正文原样回传、:251 authorId、:252 status、:267-271 legacy_success 全量返回。
- 对照 :275-285 article_get 含 session 且首行 article_read_gate；:55-59 ARTICLE_READ_ALLOWED_SQL 仅 published/属主/admin 放行；:63-90 article_read_gate 对他人 draft 返回 Forbidden。
- 路由 lib.rs:851 cms_core_entity_router、:856 /api/cms/article/list get(article_list)；主路由 backend/src/lib.rs:178 合并；全局认证中间件只验登录不验归属。
- 前端 CmsIndexApp.vue:212/251/278 api.get('/api/cms/article/list')，:215-216 只取条数、:243-266 取 firstId 拼 title，未直接渲染 content。

## 决定性 oneshot 真测（独占临时 PG，DATABASE_URL 由环境变量注入）
- 夹具 crates/cms_core_entity/tests/r636c_cms_article_list_gate_audit.rs；cargo test -p cms_core_entity --test r636c_cms_article_list_gate_audit -- --nocapture：exit 0，2 passed / 0 failed。
- 臂1 alice 会话调 article/list：响应含 bob 的 draft 文章（status=draft、authorId=r636c-bob、content=R636C_BOB_DRAFT_CONTENT_CANARY 全文）——列表泄露坐实。
- 臂2 同 id 经 article_get（alice）：Err(Forbidden)——单读被 read_gate 拒绝，与臂1 矛盾坐实。
- 臂2b 属主本人读自己 draft / admin 跨人读：均 Allowed（正常臂未破坏）。
- 臂3 published 文章列表可读且字段正常。
- 反臂 自建 SecurityState + router.layer(auth_middleware) + oneshot 裸 GET /api/cms/article/list → 401。
- 源码守卫：article_list 签名无 session/无 status 过滤；article_get 含 article_read_gate（含串断言）。

## 三分裁决
- 可达真缺陷 1（P2）：article_list 绕过 article_read_gate，任何认证用户可读他人未发布草稿正文 + 作者 ID；单读 403 而列表 200 全量，访问控制被绕过。
- 不可达 L1 0：无死代码误报，锚点经真测可达。
- 正常 3 类：匿名 401 门、属主/admin 正常读、published 内容公开符合设计。

## Fail Loud / 脱敏 / SHA
- 未跳过任何步骤；夹具首轮编译曾因 find_row<'a> 写坏（-D warnings）修正重跑，非业务失败。
- 0 真值：连接信息用 <TEST_HOST>:<PORT> 占位，无密钥/真实域名/IP/个人路径。
- runnerSha=6119b3235f4ace6cf32eaca78cbddc0d22586793cad56c7f34fb9d6f1b4b12bb（sha256(JSON.stringify(meta))）。
- 文件字节 sha256：夹具=705863c410a4a7c0c33d7b337ff4eb8ad163b43c8c8a30dbec24f4980409d5ef、oneshot.log=53e6dfc8ddd571f05fb85ac9a69be6de298eab28766d2b14a3fd0782c85e5000。
- 证据目录 oa4rust/test-results/r636c-cms/（.gitignore:28 已忽略）。

## 修复方向（不在本次范围）
- article_list 增 session 参数，按 article_read_gate 同语义对每行过滤：仅 published、author_id=本人、或 admin 可读；未授权行不返回正文。或复用 ARTICLE_READ_ALLOWED_SQL 谓词下推到查询。
