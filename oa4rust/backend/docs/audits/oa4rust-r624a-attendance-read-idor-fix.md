# R624a 考勤三读端点身份盲读真修复审计

## 1. 结论

- 判级：P1。三个读端点此前仅按请求体中的 personId / person 过滤，非空即查他人、空串即查全员；任意持有效会话的非管理员可空 body 逐页拉取全员考勤明细（含 personId、日期、状态、fileId 等 PII），亦可拉取全员聚合计数与导出标记。
- 本轮为真修复：三端点统一接收会话身份并按会话人钳制，管理员保留按 body 跨人查询能力。契约先 RED 坐实缺陷、修复后 GREEN；未改动写域外任何文件。
- 数据面口径：写端点身份绑定另案；本轮只处理读侧身份盲读，不扩大到考勤其他读接口。

## 2. 锚点与根因

- oa4rust/backend/crates/attendance_assemble_control/src/lib.rs：v2_detail_list_page_size 原签名仅 pool:Extension<Pool> + Path + Option<Json<Value>>，无会话参数；personId 与 status 完全由 body 自证，COUNT 与数据 SQL 谓词均为 ($1 = 空串 OR person_id = $1)，空 body 即全员放行。
- v2_detail_statistic_filter 与 v2_detail_statistic_export_filter 原签名同样无会话；共用 detail_statistic_aggregate 的 SQL 谓词 ($1 = 空串 OR person_id = $1)，空 person 即全员聚合；export 端点还返回可导出标记。
- 决定性兄弟范式：v2_leave_list_page_size 接收 session:Extension<shared::session::Session>，先 is_admin，非管理员把 filter_person 钳制为 session.person_unique。管理员以外不得用 body 跨人查询。
- 根因：三端点把本应由服务端从会话推导的授权主体下放给请求体自证，缺少与兄弟端点一致的钳制；路由注册的全局认证只证明会话存在，不参与查询主体判定。

## 3. RED → GREEN 真执行

- 契约：oa4rust/backend/crates/attendance_assemble_control/tests/r624a_attendance_read_idor_fix_contract.rs（2 用例）。夹具在独占临时 PG 上 DROP/CREATE 后跑全量迁移，播种三名占位人员、admin 角色及 alice/bob 明细各若干条；连接串仅运行期本地回环占位，不入仓。
- RED（真父版）：0 passed / 2 failed，退出码 101。非管理员 alice 空 body 读到他人明细条目，坐实身份盲读；源码守卫判定 v2_detail_list_page_size 未接收会话身份。
- 修复：三端点签名加 session:Extension<shared::session::Session>；统一先 shared::middleware::is_admin(&pool, &session.person_unique).await；非管理员将 filter_person 强制置为 session.person_unique.clone()，管理员才采纳 body 中的 personId / person。零新增依赖。
- 裸会话反臂：契约自建挂载 auth_middleware 的 Router，无 token 请求 detail/list 得到 401，证明修复未绕过认证。注意 oneshot 直调业务 router 不经过外层全局认证，故反臂单独构建认证栈。
- GREEN：touch 强制重编译后定点 2 passed / 0 failed，退出码 0。源码守卫用函数体精确提取，避免兄弟端点同范式误命中。

## 4. 验证矩阵

- V1 定点：RED 101 → GREEN 0，2 passed；格式化后再次 touch 复跑仍 2 passed。
- V2 包级：cargo test -p attendance_assemble_control --all-targets 在已迁移库上 56 passed / 1 failed / 15 ignored。唯一失败为写域外既有断言 u2_v2_detail_list_and_my_statistic_reachable（src/tests_u2.rs:235），旧断言期望 503，实际 500；因本轮为 detail/list 增加 Session 提取，mock_pool 裸请求缺 Session 返回 500，与兄弟 leave/list 同构（同文件 :173 断言 500）。跳过该项后全绿。未迁移的默认根库会出现 E42P01，属环境而非回归。
- V3：cargo fmt -p attendance_assemble_control -- --check 退出码 0（格式化仅触及本契约长行，git status 未越界）；cargo clippy -p attendance_assemble_control --all-targets --no-deps -- -D warnings 退出码 101，两条告警均来自写域外既有 tests/r607a_attendance_recive_owner_fix.rs:184/:189 的 clippy::bool_assert_comparison，与本轮零源码改无关，据实 Fail Loud，不修。

## 5. 行为变化、兼容与限制

- 行为变化：非管理员无论 body 传空、传他人 personId 或传他人 person，均只能读本人明细与本人聚合；管理员行为保持原样，可跨人查询与导出。
- 路由、Cargo、migrations、前端与其他端点零改；签名变化只影响 axum 提取器，路由表无需变动。
- 限制：仍要求有效会话（401 反臂已证）；聚合端点本轮钳制后出现计数口径变化（非管理员只能看到本人），属预期收紧；契约夹具使用占位口令与本地回环，真实口令、域名、IP、个人路径均未入库。
- 区间内队友提交仅涉及文档与前端其他文件，git merge-base 确认真父为祖先，lib.rs revision 未变；据实披露，未回滚队友工作。

## 6. SHA 分列

- lib.rs（修复后）：68d147add0972f0e7d08859a83d1e1e42c4e0f28268d2f3457fb1a4fe777fa09
- 契约（格式化后实际 GREEN 版本）：8bb9ac3457d8c4b52151451e86618088941c8a5644fcb6d9e0ababde0d6c1742
- runnerSha（甄别元数据 JSON 摘要，无文件 IO）：ee26551238bba5334c749d956f52566216a79f3783a1c0b41ac1542a4ac23c4a
- 文档自身 sha256 以提交后 ops_stat 结果为准，不在正文自引用。
