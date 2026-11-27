# R619b 组织属性/授权日志/消息投递 冒名族甄别报告

## 结论与真父
- 结论：可达真缺陷 7 项（P2，认证后属主伪造身份写入）；不可达 L1 0；正常项=裸 401 门 7/7。
- 真父：9776f82363f2fde0e3d3da079ffd30b8cc90e70d（本次只读甄别，零源码改）。
- 范围：组织属性/授权日志 5 写端点 + 消息投递 2 写端点；均为已认证会话下 body 身份与会话身份解绑。

## 立项与范围
- 任务链：R5 门控矩阵将七端点标为"已认证+体内门禁"，R619 立项冒名族专项，本次 oneshot 实证。
- 靶点：organization_assemble_express/src/endpoints_attr.rs:225/238/383/395/455；message_assemble_communicate/src/lib.rs:2989/3066。
- 认证门：两 crate 的 require_session 只校验会话存在（org lib.rs:21-25 / msg lib.rs:42-46），丢弃会话身份。

## 机制锚点
- 属性写 attr_write_values（endpoints_attr.rs:131-210）：owner=body.person 或 body.unit（:138），归属子查询按 body 姓名/ID 定位（:150-196），与会话身份无关，故任意已认证者可写他人属性。
- 授权日志 empowerlog_create（:455-492）：fromIdentity/toIdentity 直接取 body 落库（:474-488）。
- connector_create（msg lib.rs:2989-3060）：person=body.person（:3002），x_message_instant.person 与 x_message.sender 均写该值（:3020/:3044）。
- ws_create（msg lib.rs:3066-3113）：person=body.person（:3074），sender=body.sender 缺省 system（:3079），命中他人在线 ws 后以伪造 sender 落 x_message_consume（:3102）。
- 自认注释：endpoints_attr.rs:230/243/388/400/460 明写"冒名族，另记待修"；message_assemble_communicate/src/security_gate_tests.rs:425-433/575-589 明记 connector/ws 身份冒名不在射程内。

## 兄弟铁证与契约意图独立裁定
- 项目内兄弟口径：general_core_entity/src/lib.rs:66、general_assemble_control/src/lib.rs:57、mind_core_entity/src/lib.rs:53 的 resolve_creator 一律"creator 缺省会话人、写他人仅 admin"， corroborates 会话身份绑定是本项目现行规范。
- 门控矩阵 docs/audits/artifacts/write_routes_matrix.tsv 七行标"R5-已认证+体内门禁"；实证表明体内门禁只挡匿名，不挡冒名，矩阵对身份绑定维度失明。
- 裁定：o2server Wi 契约 body 携带身份字段属兼容历史遗留，但源码自认待修 + 兄弟已改会话绑定，故不构成有意安全设计；按认证后越权改写（P2）据实判缺陷，非越权读、非匿名、非数据泄露，不定 P1。

## 决定性实测（独立 PG16 oneshot scratch，真表真迁移直调 handler）
| 端点 | 会话人→body身份 | 实测结果 |
|---|---|---|
| personattr_set_person_name | attacker→victim(姓名) | success；victim 行落下 FORGED-SET，attacker 行空 |
| personattr_append_person_name | attacker→victim | success；victim 追加 FORGED-APPEND |
| unitattr_set_unit_name | attacker→victim-unit | success |
| unitattr_append_unit_name | attacker→victim-unit | success；UNIT-SET/UNIT-APPEND 均落 victim unit |
| empowerlog_create | attacker→body身份 | success；FORGED-FROM/FORGED-TO 日志行落库 |
| connector_create | attacker→victim | success；instant.person=dispatch.sender=victim |
| ws_create | attacker→victim+sender=attacker | success；consume.sender=attacker 落库 |
| 裸 401 反臂（7/7） | 无会话 | 两 crate 七 handler 全 Unauthorized→401，零写入 |
- 命令：DATABASE_URL 由环境变量注入（连接串含占位口令），cargo test -p 各 crate --test 临时夹具 -- --nocapture，均 exit0（org 1 passed / msg 1 passed）。
- 真值纪律：所有连接信息用 <TEST_HOST>:<PORT> 占位；scratch 库用后 DROP；临时夹具从源码树删除，工作树零源码残留。

## 三分裁决
- 可达真缺陷 7（P2，逐端点如上表）：认证后任意会话人可冒名写他人组织属性、伪造授权日志主体、冒名投递/消费消息。
- 不可达 L1 0。
- 正常项：裸会话门 7/7 有效（401），前端 UI（OrgViewer.vue:671-672/425-431、IMChat.vue:900-916）以 session.user.unique 填 person，但 API 面无服务端绑定仍可冒名。

## Fail Loud 与脱敏
- 实测中 msg 夹具曾因 SQL 字符串内嵌双引号编译失败（E0425/E0061），改位置参数后通过；org 夹具一次通过；均如实记录无跳过。
- 全文档 0 真值：无真实域名/IP/口令/个人路径，连接信息占位。

## SHA 清单（runnerSha 与文件字节 sha 分列）
- runnerSha=ab4b2285dcc6d1fa475fe9af290973ad7d78fe53c773128e24dbf7a4ca1ca211（sha256(JSON.stringify(meta))）。
- meta.json=75ad20cb964ca6d7704d3c3f3949ce43b16d6d07b459226a78970c27b7c5891e；org 夹具字节=e0ea058aa5321a0ca606c1531b9531629cef383e53c1ee36cc494438dc0243ea；msg 夹具字节=ef39669ac3e32902743a10a9e58d140529c628e00f3fd3f066a8e973a460648a。
- org-oneshot.log=fe18b8abc3d67e8a97e49d0ca3a068cd630319ea71e7c3831af9d5b11eb1eda6；msg-oneshot.log=c08977d988aa905ac77c6379ed1ec3f115fff093cad0669279cd0968d1a0a5d8。

## 修复方向
- 七写端点统一接入 resolve_creator 口径：身份取会话人，写他人需 admin；fromIdentity/toIdentity 等代理字段需显式授权校验；消息 sender/person 不许 body 自证。
