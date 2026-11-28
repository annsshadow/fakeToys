# R635a hotpic filter_list_page 身份盲读真修复审计

## 1. 结论
判级 P2（沿用 R634a 甄别 t305 结论，非新判级）。user_hotpic_filter_list_page_page_count_count 的 GET/PUT 共用 handler 原先无会话校验、SQL 无可见范围谓词，任何调用者可全表读取他人私有热图的 title/imageUrl/creator。修复后：匿名 401、非本人私有行不可见、本人/全局空键行/admin 可见。

## 2. 锚点根因
- 目标 handler 位于 hotpic_assemble_control/src/lib.rs，原签名仅 pool + Path(page,count)，SQL WHERE 仅 deleted_at IS NULL，无任何归属或可见范围谓词。
- 路由 GET/PUT 同路径共用此 handler（routes.rs），一处修复全覆盖，路由零改。
- 兄弟读口径 hotpic_core_entity::list 已有可见范围语义（本人 creator OR 全局空键行 OR admin），本修复对齐该语义。

## 3. RED→GREEN 证据
- RED（真父未改源码）：oneshot 契约 0 passed / 2 failed / EXIT101；匿名臂断言 left=401 right=200（真父匿名 200 坐实缺陷），source_guard 断言“handler 必须提取会话”失败。
- GREEN：加 session 提取 + require_session 前置（首个 DB acquire 前）+ SQL 可见范围谓词（creator=$3 OR (application='' AND info_id='') OR admin EXISTS，参数 $3=person_unique，保留 $1=offset/$2=count）后，2 passed / 0 failed / EXIT0。

## 4. 三 verify
- V1 定点：2 passed / EXIT0。
- V2 包级：首跑 18 passed / 5 failed / EXIT101，5 项为写域外既有基线（security_gate_tests* 依赖 auth_role / x_hotpic 表，目标库无迁移表，code 42P01），与本改动无关；skip 这两组后 18+2+0 passed / EXIT0，Fail Loud 披露跳过。
- V3a fmt --check：EXIT0；V3b clippy --all-targets --no-deps -D warnings：EXIT0 零告警。

## 5. 行为变化
- 匿名调用该 handler 由 200 变 401；非本人私有行由可见变不可见。
- 保留本人/全局空键行/admin 的 200 合法路径，分页与回传形状不变。
- 未跑 GUI 手工验证；写域外基线 5 项按上述 skip 披露。

## 6. SHA 与披露
- runnerSha（meta JSON sha256）：adb0c8433051215a7cf0adb0af4b648e800142e506cddd52cbb5825dff951056
- lib.rs 文件 sha256：ca2046542ccd0b51b8662608436d7b82846d886d9f3b44b3499543caaeae9d35（35732 bytes / 976 行）
- 契约文件 sha256：8c06444e7bec771c94f818657245c54585e6f850bda306d3607d8f41e1ce34ea（7116 bytes / 165 行）
- 审计时 HEAD：4eecb982ab4b0dc8ad47457b4d3d25f6dab81506（与派发真父一致，无队友前移）
- 连接串仅本地回环 + 占位口令，无真域名 / IP / 凭据入库。
