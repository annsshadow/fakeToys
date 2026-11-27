# R622b BBS 采纳回复跨主题污染真修复报告

## 结论与真父
- 结论：R621b 判定的 P2 缺陷已真修复并验证通过。两采纳入口在事务内任何写之前先校验回帖属于被采纳主题，跨主题传参零写返回 error；正常采纳、属主门、裸会话门、旧标记清理全部保持。
- 真父：56476ec5e5069e7201f3334c7c257c6d9bff9dff（HEAD=真父）。甄别期间工作树存在其他任务并行改动，本次仅改本任务三文件，对其余改动零触碰。
- 性质：真修复（非甄别），零配置/路由/migrations/前端改动。

## 立项与缺陷机制（承接 R621b）
- R621b 已证：u2_subject_accept_reply（GET）与 u2_user_reply_accept（PUT）经 gate_topic_owner 验主题属主后，两写（topic.accept_reply_id / reply.accepted）均无 reply↔topic 绑定谓词，属主可用他人主题回复 id 污染采纳关联。
- 修复锚点：u2.rs GET 事务开启后（原 :760）与 PUT 事务开启后（原 :1355）各插入同一校验块。

## 修复方案（两入口对称）
- 事务内、任何写之前先查：`SELECT 1 FROM x_bbs_reply WHERE id = $1 AND topic_id = $2 AND deleted_at IS NULL`，参数为（reply_id, 主题 id）。
- 不存在（is_none）则 `tx.commit()` 后返回 `ActionResult::error("reply not found in subject")`，后续两写均不执行，零污染落库。
- 存在则原两写逻辑与返回结构原样保留；查询错误经 `AppError::internal_with_source` 透出，不吞错。
- 改动量：u2.rs 净增 24 行（GET/PUT 各 12 行），无其他源码改动。

## 决定性实测（独占 PostgreSQL 临时库 oneshot，真迁移真表直调 handler）
连接串由环境变量注入（<TEST_HOST>:<PORT> 占位，不入库）；契约测试自管 scratch 库真迁移真表。

| 阶段 | 命令 | 结果 |
|---|---|---|
| RED（真父源码） | cargo test -p bbs_assemble_control --test r622b_accept_reply_fix -- --nocapture | exit=101；跨主题臂返回 success 触发断言失败，缺陷坐实 |
| GREEN（修复后） | 同上（touch + cargo clean -p 强制重编译） | exit=0，2 passed / 0 failed |
| V2 包级全量 | cargo test -p bbs_assemble_control --all-targets | exit=0，lib 51 + 契约 2 全 passed |
| V3 格式 | cargo fmt -p bbs_assemble_control -- --check | exit=0 |
| V3 静态 | cargo clippy -p bbs_assemble_control --all-targets --no-deps -- -D warnings | exit=0 |

GREEN 契约 2 测试逐臂：①GET 跨主题（tA owner 传 rB）→ error + tA.accept_reply_id=None + rB.accepted=false（双向零写）；①b PUT 跨主题同效；④同主题本人 GET/PUT → success 且两写落库；③非属主 → Forbidden；unaccept 清标记；裸会话过真实 auth_middleware → 401。

## 验证裁决
- RED→GREEN 成立：同一夹具真父失败、修复后通过，反臂（401/403/本人 200）证明门控与正常路径未被破坏，排除桩假阳性。
- 零回归：V2 包级 51+2 全绿；V3 fmt/clippy exit=0；无跳过、无忽略。
- 未扩大写域：路由/其他端点/Cargo.toml/migrations/前端均零改。

## Fail Loud 与脱敏
- 如实记录调试期问题：ops_edit_file 保留原 mtime 致 cargo 链接旧 rlib 假失败（日志 Finished in 0.92s 无 Compiling），改用 touch + cargo clean -p 后重编译才得真实 GREEN。
- V2 首次 exit=101 的 11 个失败全因 DATABASE_URL 指向被契约测试 drop 的 scratch 空库（E42P01 relation does not exist），非本次改动；建持久迁移库指向后全绿。
- 全文档 0 真值：连接信息用 <TEST_HOST>:<PORT> 占位，无真实域名/IP/口令/容器名/个人路径。

## SHA 清单（runnerSha 与文件字节 sha 分列）
- runnerSha=4601b3ae78dec6d450e07e412428f8b06bdcda180ec48d36e1da6b1fd517faf1（sha256(JSON.stringify(meta))）。
- u2.rs 字节 sha256=987fe634c23d451da796f464ad08ca25a57dc40574bc21112c898b8b52c138a6。
- 契约测试字节 sha256=1db7b94fa6d534eda6b8ba8e50bc58658abeb080173b623e3942591ab33aa2ad（源码树与证据副本一致）。
- 证据日志 sha256：red=646a6f15…、green=76b3a1e5…、v2-all-targets=1cb37161…、v3-fmt-check=e3b0c442…(空输出)、v3-clippy=2c057f0a…、migrate-main=a02d4e7c…。
- 证据入 gitignored oa4rust/test-results/r622b-fix/（.gitignore:28）；临时迁移引导测试已删除，源码树零残留。

## 修复后残留（据实披露，非本次范围）
- 采纳侧仍不清理旧 reply 的 accepted 标记（取消路径 unaccept 才清），属 R621b 已记录的既定惯例缺口，本次未扩范围处理。
