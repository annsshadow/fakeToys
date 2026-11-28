# R634a hotpic filter_list_page 身份盲读甄别审计

## 1. 结论
hotpic_assemble_control 的 user_hotpic_filter_list_page_page_count_count 列表端点存在身份盲读：handler 签名无 session 参数、SQL 无可见范围谓词，任何调用者都能读到全表 x_hotpic 的 title / imageUrl / creator。判级 P2（需登录非匿名、回传 URL 危害低于 base64 同族，不夸 P1）。本轮为验证型甄别，零源码/零测试入库/零配置改。

## 2. 锚点与根因
审计对象：oa4rust/backend/crates/hotpic_assemble_control/src/lib.rs（968 行）。
- 目标 handler 位于 lib.rs:750-796，签名为 pool: Extension<Pool> + Path((page,count))，无 session 提取。
- lib.rs:758-760 SQL：SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT/OFFSET，仅过滤软删，无任何可见范围或 admin 谓词。
- lib.rs:765-788 将 image_url 映射为 imageUrl、creator 原样回传，故跨身份私有热图对调用者可见。
- 对照：同 crate 写端点 user_hotpic_changeTitle（lib.rs:662-720）及 user_hotpic_id DELETE 已有 owner 门；兄弟 crate hotpic_core_entity 的 list（lib.rs:72-93）已按本人+全局空键+admin 语义过滤。目标列表端点未同步该语义。
- 路由：routes.rs:111-114 GET /api/hotpic/assemble/control/user/hotpic/filter/list/page/{page}/count/{count} 指向该 handler，真实可达。

## 3. 真执行证据
起独占本地回环 PG trust 实例（临时端口，避开他组占用），进程内注入测试连接串（本地回环+占位口令，不入库）。写临时 oneshot 契约 r634a_hotpic_filter_scope_contract.rs（验证型用后删），fresh_scratch 建库并跑全量迁移后 seed 三方数据：bob 私有热图行、alice 私有热图行、全局空键热图行。真跑命令：cargo test -p hotpic_assemble_control --test r634a_hotpic_filter_scope_contract -- --nocapture。结果：running 2 tests / 2 passed / 0 failed / EXIT0。
坐实四臂：① 无会话直调 handler 读到全表，含 bob 私有行的 title / imageUrl / creator；② 与 R626b 可见范围语义对照，私有行本应被过滤却仍回传；③ 经 router oneshot 不挂 session 层仍 200 且含 bob 私有行，坐实 handler 自身无身份门；④ 源码守卫断言 handler 无 session 提取且 SQL 无可见范围谓词，全部命中。
据实披露：生产路由经全局 auth_middleware，外部匿名请求仍需登录；本轮坐实的是 handler 自身不区分身份，登录后任一账号可读全表。

## 4. 三分据实
- 可达性：路由真实注册、handler 真被执行、DB 真读到跨身份行，非死代码。
- 危害面：回传 imageUrl（URL 引用）而非 base64 二进制，敏感度低于同族 base64 盲读；且需有效登录会话，故判 P2 不夸 P1/P0。
- 不适用 P3：语义明确为个人热图列表（user/hotpic/filter），跨身份读取私有行违背最小权限，非设计预期。

## 5. 修复参考（仅建议，不实施）
可复用 R626b 可见范围语义（见 hotpic_core_entity/src/lib.rs:72-93）：handler 加 Option<Extension<Session>> 并 require_session，SQL 追加可见范围谓词 creator=actor OR (application='' AND info_id='') OR admin EXISTS。本轮不落地修复，留后续 implementation 轮次决策。

## 6. SHA 与披露
- runnerSha（执行证据 meta JSON 摘要）：f9ff7413fbd770b4bcd58c2bdb1e9a5397006276404a3a087acb3e37d4369ba4
- 审计对象文件 sha256：defe10589ec51ce239e01ac2ee26e919401f00f59879a77ce3f4e567159bb23b（lib.rs，968 行，只读未改）
- docSha：文档自身 sha256，由提交后 ops_stat 读取，与 runnerSha 分列。
- HEAD 披露：派发时真父 856e56ff；审计与提交时 HEAD=145930de（队友并行提交前移）。审计对象 lib.rs 全文 968 行未变，本轮未触碰队友改动。
- 临时 oneshot 契约与独占 PG 用后即清；既有 untracked 目录未纳入。
