# R613a bbs_core_entity 写端点身份盲 IDOR 甄别（P1）

## 结论
判 P1：forum/section/subject 三组 update/delete 写端点只验「已登录」，
不验记录归属、不验 admin；任意登录用户凭已知 id 即可跨用户改写/物理删除他人
论坛/版块/主题数据，破坏数据完整性与管理面隔离。零源码改，仅留本甄别文档。

## 根因
require_session 只从 Option<Extension<Session>> 取 Session，不校验 person_unique
与目标记录归属；各写端点以 `require_session(session)?` 丢弃返回值后直接按主键
find_by_id + ActiveModel::update/delete 落库。同文件已有的 resolve_author/is_admin_db
仅被 create_subject/create_reply 调用，update/delete 完全绕过。

## 精确锚点（crates/bbs_core_entity/src/lib.rs）
- :27-31 require_session 只 map 取 Session → Unauthorized，不校归属。
- :310-353 update_forum：:316 丢弃会话 + :350 .update()。
- :369-390 delete_forum：:374 丢弃 + :387 .delete()（物理删）。
- :448-503 update_section：:454 丢弃；:524-545 delete_section：:529 丢弃 + :543 .delete()。
- :603-662 update_subject：:609 丢弃 + :610 find_by_id + :649 原样保留 author_id + :659 .update()。
- :686-707 delete_subject：:691 丢弃。
- :829-855 bbs_core_entity_router 真实注册上述 POST/DELETE，:855 .layer(Extension(db))。
- :54-77 resolve_author（非 admin 写他人→403）与 :34-50 is_admin_db 只护 create。

## 真测试（临时 PG + oneshot 真路由，1 passed / 0 failed）
夹具 seed 受害 bob / 攻击 alice 各一组 forum+section+subject，经 crate router 直发请求：
| 场景 | 期望 | 实测 |
|---|---|---|
| 无会话删 bob forum | 401 且数据未动 | 401，行仍在 |
| alice 凭 bob id update forum/section/subject | 200 + 真改 | 3×200，DB 三行均被改写 |
| alice 凭 bob id delete forum/section/subject | 200 + 真删 | 3×200，DB 三行均消失 |
| subject 盲写后 author_id | 原样保留 | 仍为 bob，无归属校验 |
| alice 本人 update / admin delete | 200（放行） | 200，admin 删除真落库 |
| 兄弟 bbs_assemble_control delete_forum 冒名 | 403 且未软删 | 403，行未被软删 |

兄弟对照 bbs_assemble_control/src/lib.rs:931/:977/:1009 delete_forum/delete_subject/
delete_forum_body 均有 :938/:982/:1014 is_admin→403（注释 :936「任何登录人可调=越权
管理面」），路由 evidence 于 routes.rs:73-94，证明正确 admin 门存在且本 task 不误伤。

## 三分判定
- P1 成立：写端点可达且确已落库破坏数据；subject 有明确 author_id 归属；管理面
  角色隔离被绕过。需有效会话 + 需知目标 id，故不夸大 P0（非匿名任意写）。
- L1（认证前可达）排除：9 把端点均经 require_session，无会话返回 401。
- L0（匿名任意读写）排除：必须有有效 Session，非匿名。

## 据实披露
- 测试锁定 security_gate_tests_b79.rs:23-101 只测「无会话 401」，把「只需登录即可写」
  固化成预期；:104-115 仅纯单元。b79 注释 :12-14 称 router 不提供 Extension 致 500
  的旧判断已被 w3 与本次 oneshot 真路由实测推翻。
- security_gate_tests_w3.rs 只覆盖 create 冒名（403），未测 update/delete，故本族漏网。
- 前端消费点 BBSForum.vue:1194-1195/1206-1220 弹窗输入任意 id 且无角色门禁；本次仅
  后端甄别，未定位/改动该 vue 组件，不影响后端可达性结论。
- 零源码改：git diff <true-parent> -- crates/bbs_core_entity/ 为空。
- 临时 PG/夹具/日志均为甄别脚手架，用后即删，不入库。

## Fail Loud
DB 不可用则夹具 SKIP 并显式告警，绝不把跳过冒充 pass；本次未 SKIP（1 passed）。

## SHA 清单（runnerSha 与源文件 sha256 分列）
- runnerSha(sha256(JSON.stringify(meta))): 9b5cfd8fc8fe884a3c86662473014e29077bbd320a096b7ca2ab395dbc8acdb8
- bbs_core_entity/src/lib.rs sha256: e45ba7a71db7bfb102837bb38448f770d76deafe87293d0f678aab4fb97c8da8
- bbs_core_entity/src/routes.rs sha256: ff5c74dbb8c91c1902e1abd99aa49d3aeffe2c090b572ce6959d8edb45c951b5
- 夹具 r613a_bbs_write_idor_audit.rs sha256: cac635f5a2600abaaee56f43ffe4e150866f71b23e1bb86940fa30a3abfc8975
- 日志 r613a-run.log sha256: 91e61fdcd8a6ed35e491bcfb5d0b10e7760cab6bb306b2401be87635b9841d26
- 真父 HEAD: c0947b0c1591645f5bf58c72b4b451048435251c
