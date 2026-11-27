# R598 会话 list/my 身份盲 IDOR 验证型甄别审计（t159）

## 结论
判定 1 项真实可达缺陷，严重度 P2：`im_conversation_list_my` 身份盲，任意已登录用户可读取全量会话元数据（会话名、最后消息预览、时间），跨 person 私有会话无隔离。

## 立项性质与范围
- 验证型甄别：零源码改动，唯一交付物为本审计文档；证据日志与夹具留 gitignored 证据目录，夹具用后即删。
- 真父 HEAD=0a7f08552d8b03a0b75e611958de201e00cbba11。
- 台账 `docs/ops/current-state.md` grep R59 系列与会话/calendar/IDOR/身份盲关键词零命中，本缺陷从未专项甄别。

## 机制锚点（逐行核实）
- `crates/message_assemble_communicate/src/lib.rs:638-688`：`im_conversation_list_my(pool: Extension<Pool>)` 签名无 Session / person 参数；`:644` SQL `SELECT id,name,type,last_message,create_time FROM x_message_conversation ORDER BY update_time DESC` 绑定空数组、无 WHERE，无归属过滤；`:665-669` 把 `last_message` 作为 `lastMessage` 返回。
- `crates/message_assemble_communicate/src/routes.rs:67`：挂载 GET list/my 路由。
- `crates/shared/src/middleware/token.rs:135-181`：全局 `auth_middleware` 只做 extract_authentication 到 validate_session 再到 request.extensions_mut().insert(session)，不做资源归属过滤，坐实只保登录、不保隔离。
- 对照兄弟端点：同文件 `:737-750` `im_conversation_mockputtopost` 先 require session 再 is_conversation_member 门禁；成员表 `x_message_conversation_member` 在 `:571-576` 会话创建时双写，证明归属数据齐备而本读端点忽略。

## 方法（真测试）
- 临时 PG：独立容器 + 高位端口，真 migration 032 建表（x_message_conversation 含 last_message，避免 mock 表缺列误判）；seed person A/B 各建私有会话 + 各自成员行。
- oneshot 直调 pub router(pool)，分别 layer 注入 A/B 的 Session 扩展，请求 list/my，断言应只含调用者本人会话。
- 采用 `--test-threads=1` 规避两测试并行争用同名 scratch 库（E23505 已作为 Fail Loud 披露）。

## 决定性实测
| 观测 | 结果 |
| --- | --- |
| status_a / status_b | 200 OK / 200 OK |
| 注入 A、B 不同 Session 的返回 | 完全相同（same=true） |
| A 身份返回会话 id | r598-conv-B、r598-conv-A（A、B 全表） |
| lastMessage 预览 | B私密最后消息、A私密最后消息（B 私有预览泄露） |
| a_seen / b_seen / b_preview_seen | true / true / true |
| 兄弟端点成员门禁对照 | a_in_a=1、a_in_b=0（A 无法改 B 会话） |
| 测试汇总 | 2 passed，0 failed |

身份盲、跨 person 泄露、读端点无归属过滤三项决定性成立；兄弟端点证明同 crate 内归属门禁可实施，本端点属遗漏。

## 三分裁决
- 可达 P2：全局 auth 要求登录，泄露面为任意已登录用户而非匿名（据实披露）；读端点仅泄露会话元数据（会话名/最后消息预览/时间），非凭据或写权限（据实披露）。故定为 P2 而非 P1/P0。
- 次候选 `im_conversation_id`（`:768-813` 按 id 裸读无归属）、`im_conversation_business_businessId`（`:593-635`）、`im_conversation_list_with_person`（`:691-734`）同族无归属，仅据实记录为同族暴露面，不硬凑为独立 P2 项。
- 正常归属路径对照：兄弟端点门禁有效，说明归属数据齐备，非系统设计如此。

## Fail Loud
- 首次运行因 workspace `[workspace.lints.rust] warnings=deny` 编译失败（未用变量 person），已精确修复，未采信。
- 两测试并行争用同名 scratch 库报 E23505（duplicate key oa_r598_idor），已改单线程重跑并通过；此前并行失败运行未采信。
- 临时 PG 为隔离实例，仅用 <TEST_HOST>:<PORT> 占位，无真值入库。

## 证据 sha256（runner 紧凑 JSON 与文件 sha256sum 分列）
- runnerSha=sha256(JSON.stringify(meta))=c3a0c92a81af91ad379248ad8159f39e1c6b3ab448b9bfac45cd43ea89adc360
- 证据日志 r598-test.log 文件 sha256sum=c3b45c7cc361609ede7bba1f61ea253548cc40473ef1369fdb76bcd23a1cb3db
- 夹具 r598_conversation_idor_audit.rs 文件 sha256sum=3790d55c0a9d3464867b0df86dc7e3b59080487d28d84db6a0391338a0b77fc1
- 源码 lib.rs=4a173cad1e424743b3ac9ee227aaef1312da5fb6be2894960b5086dff7bc2e8c；routes.rs=ba3befdcfa5986756ff7e2ddccfec8711ee0f51fe26a3e23e8692c15e4034be9
- 中间件 token.rs=b76f9d38ced1a83e6733386c2f293655f5967e9870c68e4d82f4f38ad97bd707；backend lib.rs=fff5ae0dea271fd66291a346e1a00350afe57f2998fb526ab355cdd236ef077b；migration 032=722c2899500c201245ffc3b612ae00fd8c05c21aa8512bab2453b57266428d9e

## 修复方向（建议，不实施）
参照兄弟端点：为 im_conversation_list_my 增加 Session 参数，SQL 以 x_message_conversation_member.person_id 等于 sess.person_unique 过滤，仅返回本人会话；同族裸读端点一并按成员表收敛。