# R634b 会议签到码裸读甄别报告

## 结论与真父
- 结论：可达真缺陷 1 项（P3，任何已认证用户凭会议 id 可读他人会议签到码）；不可达 L1 0；正常项 3（app 层裸 401、NULL 码/无行分支行为、签到写端点会话权威）。
- 真父：856e56ffe43dcde249caed8c590bb7e1c494077c（立项 HEAD）；甄别期间 HEAD 前移到 ca2f471180fcad689718c71d0251304a1e35a593（队友并行提交），锚点源码 meeting_assemble_control/src/lib.rs 与 routes.rs 真父至 HEAD 零 diff。
- 本次纯甄别、零源码改；临时夹具测后已从源码树删除，工作树零源码残留。

## 立项与机制锚点
- 台账 docs/ops/current-state.md git grep 签到码 / checkin/code / checkinCode 仅命中 R527 键盘可达性（current-state.md:5447/5449），无越权专项甄别。
- meeting_assemble_control/src/lib.rs:2753-2787 meeting_id_checkin_code：:2753-2756 签名仅 pool: Extension<Pool> + Path(id)，无 session/Extension<Session> 入参。
- :2761 SQL "SELECT checkin_code, to_char(expire_time, ...) FROM x_meeting_checkin_code WHERE meeting_id = $1 ORDER BY create_time DESC LIMIT 1" 只按会议 id，无任何调用者身份/归属谓词；:2772-2781 回传 checkinCode/expireTime；:2785 无行 → ActionResult::error("checkin code not found")。
- 路由 meeting_assemble_control/src/routes.rs:72 POST meeting/checkin/code/{id}、:115 GET meeting/{id}/checkin/code 均挂该 handler；全局 backend/src/lib.rs:161 .merge(meeting_assemble_control::router(...))，认证由 app 层 auth_middleware 提供（只验登录，不验归属）。
- 前端 MeetingApp.vue:66 签到码按钮 + :730-735 showCheckinCode() 真实 api.post 消费读取结果，非死代码。
- 表 x_meeting_checkin_code 见 migrations/036_create_additional_tables.sql:131-188（checkin_code VARCHAR / expire_time TIMESTAMPTZ / meeting_id VARCHAR / create_time TIMESTAMPTZ）。

## 决定性判级依据：读码→代签链路不成立
- 兄弟正面：同文件 :2721-2750 meeting_id_checkin（签到写端点）有 session + require_session(:2727)，且 :2731 以会话身份为权威 person = sess.person_unique.as_str()；INSERT (:2736) 只写 (id, meeting_id, person, checkin_time)，完全不含 checkin_code。
- 全仓 git grep checkin_code 在 *.rs 仅命中：读端点 :2761/:2774、security_gate_tests.rs:178 注释、generated_routes.rs mcp 生成描述、tests_generated.rs:1006 SKIPPED 注释；无任何写端点/校验逻辑消费签到码。
- 故攻击者即使读到他人会议签到码，也无法用它完成代签或绕过任何校验——签到以会话人为权威，码不具备可执行能力；后果限于会议内部信息泄露（码 + 过期时间），不越权写、不改数据、不冒名。这是判 P3 而非 P2 的核心理由。

## 决定性实测（独占 PostgreSQL oneshot，真迁移真表直调 pub handler）
命令：DATABASE_URL 环境变量注入（连接串占位），cargo test -p meeting_assemble_control --test r634b_checkin_code_read_audit -- --nocapture，2 passed / 0 failed（exit0）。

| 臂 | 操作 | 实测 |
|---|---|---|
| ① 跨人读 | 无 session 参数直调，凭 bob 会议 id 读 | success；checkinCode=bob 码 + expireTime（R634B_ARM1_CROSS_READ） |
| ② 本人读 | 凭 alice 会议 id 读 | success；checkinCode=alice 码（R634B_ARM2_OWN_READ，证明检测器有效） |
| ③a NULL 码行 | checkin_code 为 NULL 的行 | success + checkinCode=""（据实披露行为细节） |
| ③b 无行 | meeting_id 表中无记录 | type=error / "checkin code not found"（None 分支有效） |
| ④ 决定性源码守卫 | 读端点签名 + 写端点语义 | read_no_session=1 / write_ignore_payload=1 / write_session_authority=1 / write_no_code=1 |
| ⑤ 匿名反臂 | 裸请求过真实 auth_middleware（自建 SecurityState + router().layer） | 401 Unauthorized（R634B_UNAUTH_APP_LAYER=401） |

- 反臂有效性：⑤ 401 证明认证门生效，排除桩假阳性；② 证明读端点正常路径未被破坏；④ 从源码坐实判级推理非主观推断。

## 三分裁决
- 可达真缺陷 1 项（P3）：meeting_id_checkin_code 会话盲读，需有效会话（app 层 401）+ 知道目标 meeting id（前端列表可见/可枚举）；泄露限于签到码与过期时间；无后续可执行滥用链（④），故 P3。与 P2 等价越权写或高敏数据泄露区分：本条无写、不涉身份冒名、码不可转化为代签。
- 不可达 L1 0。
- 正常项 3：app 层裸 401（会话门）；NULL 码 success 空串与无行 error（分支自洽）；签到写端点以会话人为权威且不消费码（兄弟正面）。

## Fail Loud / 脱敏 / SHA（runnerSha 与文件字节 sha 分列）
- 调试失败据实记录：首轮"无码会议"臂断言 error，实测 NULL 码行返回 success+空串（:2774 unwrap_or_default），据实改判为 ③a；另补 ③b 无行臂验证 None 分支。决定性臂首轮因 split 先切到读端点尾部误含 checkin_code，改为先按下一函数签名切断后复验通过。无跳过。
- 0 真值：连接信息用 <TEST_HOST>:<PORT> 占位；无密钥/口令/真实域名/IP/个人路径；签到码为夹具自造测试标记，文档不写其字面值。泄露扫描 LEAK_HITS=0。
- runnerSha(sha256(JSON.stringify(meta)))=6e8b3a71d57f593434934f65c91f0cae5181f7f9e0177556811dee22f055fe99
- 夹具字节 sha256（源码树与证据副本一致）=d5094d78da33dfc71008e47e414fd928c2b87264090137c17730bed2f6f954f4
- oneshot.log 字节 sha256=4294b327704a69903dd1daa5c9e7fd248571776337379638d619999ab5240069
- 证据目录 oa4rust/test-results/r634b-checkin/（.gitignore 已忽略）；临时夹具已从源码树删除。

## 修复方向（不在本次范围）
- 为 meeting_id_checkin_code 增加 session: Extension<Session> 并复用会议属主门（参照同文件 accept/confirm_allow 的 u2_meeting_owner_gate），仅会议属主/admin 可读签到码；或收敛为只向签到写端点内部提供、不对通用客户端暴露。
