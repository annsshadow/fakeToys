# R631a x_org_custom 四端点身份盲 IDOR 甄别审计（验证型，零源码改）

## 1. 结论
- 写三端点判 P1（据实，不夸大 P0）：custom_create/update/delete 仅验会话存在、丢弃返回值，任意登录者凭他人 identityId 或 record id 即可挂字段/改名/软删，且数据库真改，属身份盲写，与同仓 message 已读、mind 保存同型。需有效会话且知目标 id（非匿名、非遍历），故不夸大到 P0。
- custom_list 判 P2：无会话裸读任意 identityId 的自定义字段值，敏感值泄露；非匿名门槛更低但仅读、不改数据。
- 四端点合并一案据实陈述，不硬凑同级判级：写三端点 P1 与读一端点 P2 分别标注。

## 2. 锚点与根因
- `oa4rust/backend/crates/organization_core_entity/src/lib.rs:32-36` `require_session` 仅 `map(|Extension(s)| s).ok_or(Unauthorized)`，调用方若丢弃返回值则只验"会话存在"。
- `:337-378` custom_list 签名仅 `db: Extension<DatabaseConnection>` + `Path(identity_id)`，无 Session，裸 `org_custom::Entity::find()` 按 identityId 全量回传 fieldName/fieldValue。
- `:858-899` custom_create：`:863` `require_session(session)?` 丢弃返回值；`identity_id` 直接取 body 入 ActiveModel insert——任意登录者往他人身份挂字段。
- `:902-942` custom_update：`:908` 同样丢弃；`find_by_id(&id).one()` 取他人记录后 `active.update`——凭 id 跨人改名/改值。
- `:945-965` custom_delete：`:950` 同样丢弃；取他人记录后置 `deleted_at=Some(now)` 软删——凭 id 跨人软删。
- `:1134-1143` 三路由 /api/organization/custom（list/{identityId} get、根 post、{id} put/delete）真实注册，`:1164` router 可达。
- DDL `migrations/029_create_missing_core_entity_tables.sql:256-263` x_org_custom 无 creator/person 归属列，无法按行判归属。
- 兄弟门对照：`:423-444` definition_create 绑定 `creator=actor`；`:682-704` identity_update 用 `CREATOR_OWNER_PREDICATE`；`:970-1000` bind_create 先校验归属——同 crate 写端点已有身份绑定口径，四个 custom 端点独缺。

## 3. 真执行证据（oneshot + 独占临时 PG）
- 起 trust 模式独占临时 PG（本地回环独占端口，用后即清，避开他组容器），`shared::migrate::run_migrations` 建表，四 handler 经 `router(db)` oneshot 直调。
- 命令：`DATABASE_URL=<本地回环占位连接串> cargo test -p organization_core_entity --test r631a_org_custom_idor_contract -- --nocapture`（在 oa4rust/backend 下）。
- 结果：running 1 test / 1 passed / 0 failed / EXIT=0；该用例内五个维度断言全部通过（真父行为坐实）。
- 维度1：custom_list 无会话 GET 他人 identityId → 200 且响应含他人字段值（裸读泄露坐实）。
- 维度2：alice 会话 POST 挂到 bob identityId → 200 且 SELECT 确认新字段真落 bob 名下。
- 维度3：alice 会话 PUT bob record id → 200 且 SELECT 确认字段值真改。
- 维度4：alice 会话 DELETE bob record id → 200 且 SELECT 确认 deleted_at 真置位。
- 反臂A：三写端点无会话直调 → 401（缺陷需有效会话，非匿名）。反臂B：bob 本人改自己 → 200 且真改（合法路径保留）。反臂C：admin 跨人删 → 200 且真置位（合法 admin 路径保留）。
- 临时契约 oneshot 取证用，验证后删除，不入库；被测源码零改。

## 4. 三分据实
- 可达真缺陷 P1（写）/P2（读）：端点真实注册、router 可达；有效会话 + 目标 id 即可跨人写，数据库真改。
- 非 P0：需有效会话且知 id，非匿名批量遍历；软删可恢复。
- 不适用 P3：真实鉴权/归属缺失，非文案或色值瑕疵。
- 前端消费点据实披露：前端批量探针面板以空 body/id=0 形式调用，非真实业务数据操作，削弱"用户可在 UI 直接触发"主张；但后端 PUT/DELETE/list 端点确已注册可达，API 层缺陷成立。

## 5. 修复方向（未实施，零源码改）
custom_create/update/delete 保留 `require_session` 返回值作 actor，按 identity_id/record 归属校验（x_org_custom 加 creator 列或经 x_org_identity.creator 反查），非本人需 admin 放行（复用 `CREATOR_OWNER_PREDICATE`/`actor_is_admin` 与同 crate 写端点口径），门置于首个 DB 访问前；custom_list 补 session 门并按归属过滤。属 implementation 后续轮次，本甄别不改源码。

## 6. SHA 与范围声明（分列）
- 真父 HEAD：`c1879f8868a0374d4444f5b0367a8fece20a0ebf`；审计时 HEAD 一致，无队友前移。
- runnerSha（本次 meta JSON 的 sha256，非文件哈希）：`788552ae08346184b86b2a6173d8c403e4460102eed0b2484e8ccb2dfd56133c`。
- 审计对象文件 sha256（organization_core_entity/src/lib.rs，只读未改）：`6e7081c2e10353fdf2e4c33afcaf5f76984cfa9b7534c55e4c34e8bbafa3d0df`（40862 bytes / 1166 行）。
- 本次零源码改、零测试入库、零配置改；唯一入库物为本审计文档。
- 0 真值：无真实域名/IP/口令/私钥/个人路径/真实端口；连接串与夹具值均以占位或描述性措辞表达。
- Fail Loud：队友 t631b 新增 `crates/meeting/tests/r631b_participant_idor_audit.rs` 为他人工作，未纳入本次提交、未触碰。
