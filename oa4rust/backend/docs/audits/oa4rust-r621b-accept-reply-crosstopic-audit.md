# R621b BBS 采纳回复跨主题污染甄别报告

## 结论与真父
- 结论：可达真缺陷 1 项（P2，认证后主题属主可用其它主题的回复 id 采纳，污染采纳关联）；不可达 L1 0；正常项 4（裸会话门、属主门、属主本人正常采纳、两写同事务原子性）。
- 真父：e78561a995dfcf9c9ed08aa71ea1379f71442ed5（R621 立项时 HEAD）。甄别期间 HEAD 前移到 c09ebf1a3（R621c OrgDirectory 只读搜索审计文档，t249），对本次锚点源码零改动；本次只读甄别、零源码改。
- 范围：bbs_assemble_control u2 域两个采纳入口 u2_subject_accept_reply（GET）与 u2_user_reply_accept（PUT）。

## 立项与机制锚点
- 台账 docs/ops/current-state.md 全文 git grep accept_reply 命中 0，该族从未专项甄别。
- u2.rs:747 u2_subject_accept_reply 经 gate_topic_owner(:753) 验主题 owner；gate_topic_owner(:96-104) 仅按主题 id 查 creator 与会话人/admin 比对，不校验 reply_id 所属主题。
- u2.rs:763 UPDATE x_bbs_topic SET accept_reply_id=$1 WHERE id=$2 只按主题 id；:769 UPDATE x_bbs_reply SET accepted=true WHERE id=$1 AND deleted_at IS NULL 只按 reply_id，无任何 topic_id/subject_id 绑定谓词；两写在同一事务(:759-774)。
- u2.rs:1324 u2_user_reply_accept（PUT 版）同缺陷：:1345 置 reply.accepted、:1351 置 topic.accept_reply_id，同样无跨主题绑定。
- 路由：routes.rs:334-335 GET user/subject/acceptreply/{id}/{replyId}，:268 PUT user/reply/accept。前端 BBSForum.vue:1252-1253 以 prompt('回复 ID:') 输入拼 URL，可输入任意 reply id；:1161 为 PUT 版入口。
- 基础数据：x_bbs_reply 建表带 subject_id（007_bbs_tables.sql:34-40）后补 topic_id（033:32），创建回帖 INSERT 写入 topic_id（u2.rs:1307-1309）；关联事实在数据模型里存在，采纳侧却未使用。

## 兄弟铁证与既定惯例
- u2_subject_unaccept_reply（:788-842）先 SELECT 旧 accept_reply_id，再同事务清 topic 指针 + 清 reply.accepted=false（:827），证明「跨表不变量要原子 + 绑定」在本模块是既定惯例。
- 采纳侧只做一半：两写原子但缺 reply↔topic 绑定谓词，且采纳时不清理旧 reply 的 accepted 标记（取消路径才清），构成双采纳隐患。

## 决定性实测（独占 PostgreSQL oneshot scratch，真迁移真表直调 handler）
命令：DATABASE_URL 由环境变量注入（连接串占位），cargo test -p bbs_assemble_control --test r621b_accept_reply_audit -- --nocapture，2 passed / 0 failed（exit0）。

| 臂 | 操作 | 实测 |
|---|---|---|
| ① GET 跨主题 | 主题 A owner 传主题 B 的 reply id（tA / rB） | success；tA.accept_reply_id=rB 且 rB.accepted=true，双向落库同事务 |
| ② 裸会话 | 受保护采纳路由无凭据过真实 auth_middleware | 401 Unauthorized（R621B_ARM2_BARE_SESSION_401=PASS） |
| ③ 非属主 | tB owner 用 rA 调 PUT 采纳 tA | Err(AppError::Forbidden)→403，零落库 |
| ④ 属主本人 | tA owner 用 rA 调 PUT 采纳 tA | success；tA.accept_reply_id=rA、rA.accepted=true |
| ⑤ PUT 跨主题 | tA owner 传 rB 调 PUT 采纳 tA | success；tA.accept_reply_id=rB、rB.accepted=true（两入口同缺陷） |
| ⑥ 旧标记 | 臂④/⑤ 后再查 rB.accepted | 仍为 true——采纳侧不清理旧标记，据实披露（非可达缺陷，属既定惯例缺口） |

- 事务与双向断言：臂①/④/⑤ 均同时断言 topic.accept_reply_id 与 reply.accepted，值一致；0 pageerror/0 DB error（除已记录的调试期失败见下）。
- 反臂有效性：臂③ 403 与臂② 401 证明门控生效，排除桩假阳性；臂④ 证明正常路径未被破坏。

## 三分裁决
- 可达真缺陷 1 项（P2）：两个采纳端点缺 reply↔topic 绑定，认证后主题属主可把其它主题回复设为本人主题采纳，污染采纳关联字段并误标他人回帖 accepted=true。
  - 可达性：需主题 owner/admin 前置 + 需知道其它主题的 reply id（前端 prompt 可输入任意 id，非不可达）；GET 与 PUT 两入口均可达。
  - 危害：限于采纳关联字段污染（accept_reply_id / accepted），不读写正文、不涉身份冒名、不跨用户越权（只能改本人主题指针），故定 P2 而非 P1。
  - 限制：非属主 403、裸会话 401，攻击面限于有权限的主题属主/管理员。
- 不可达 L1 0。
- 正常项：裸会话 401、非属主 403、属主本人正常采纳 200、两写同事务原子性；unaccept 取消路径清旧标记惯例保留。

## Fail Loud 与脱敏
- 调试期失败如实记录：首轮未用 helper 触发 -D dead-code 编译错；seed 用了不存在的 creator_person 列（属主门只读 creator）报 42703；均修正后重跑通过，无跳过。
- 全文档 0 真值：连接信息用 <TEST_HOST>:<PORT> 占位，无真实域名/IP/口令/容器名/个人路径。

## SHA 清单（runnerSha 与文件字节 sha 分列）
- runnerSha=c061e4089a14d7f5311dda129af6c56782ca1f77680413398039afe6b732f47c（sha256(JSON.stringify(meta))）。
- oneshot 夹具字节 sha256=34a9e53201e3bbf92cd7d4e7d66efda02eb26b4e10759377f7072f91999fd339；oneshot 日志 sha256=cb71cdb2e52fbb2215cbfbb6299d193f2971fa153ffafabe4d5285a5998599bc。
- 证据入 gitignored oa4rust/test-results/r621b-impersonation/（.gitignore:28）；临时夹具已从源码树删除，工作树零源码残留。

## 修复方向
- 采纳两写统一加绑定谓词：回复 UPDATE 改为 WHERE id=$1 AND (topic_id=$2 OR subject_id=$2) AND deleted_at IS NULL，并在 affected=0 时返回明确错误/回滚，避免主题悬空指向。
- 采纳侧补旧标记清理（照 u2_subject_unaccept_reply 惯例），将 accept_reply_id 与 reply.accepted 作为同一不变量一次性维护。
