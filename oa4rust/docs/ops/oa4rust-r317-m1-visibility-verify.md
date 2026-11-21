# oa4rust R317 M1 消息 sender 缺省 普通用户可见性复验

## 1 来源与边界
- 任务 t95 [R317][A6]，只读复验：不改源码/契约/后端/配置，仅创建必要测试数据。
- 背景：t92 报告 oa4rust/docs/ops/oa4rust-r317-message-candidate-verify.md 中 M1 判为部分坐实/取证限制，需非 admin 账号复验。本轮坐实或排除，禁臆造。
- 测试服用现有部署产物 PC index-yrBKdt7I.js（含 t93 M2 修复），未重新 build/部署。
- 真浏览器 Playwright Chromium headless，本地无头浏览器渲染，非物理真机。
- 真域名/口令/会话数据零入库，统一占位 <YOUR_TEST_DOMAIN>；临时测试账号唯一标识不写入本文。

## 2 判定结论
- M1 等于潜伏缺陷 P2：后端读归属谓词缺口真实存在且已坐实；但前端 customCreate 为死代码、生产 bundle 不含该端点，无真实用户触发路径，实际影响为 0。
- 本轮用真实非 admin 账号完成可见性复验，t92 遗留的取证限制已解除。

## 3 源码定位四件套
- frontend/apps/desktop/src/views/IMChat.vue:794/834/863/878：imMore/imMore2/imMore3/imMore4 四函数仅定义无调用点（全文件 grep 仅 4 处定义）。
- IMChat.vue:821：op 等于 customCreate 分支发出空请求体，位于死函数 imMore 内，永不可达。
- 测试服 bundle index-yrBKdt7I.js：grep -c message/custom/create 结果为 0，端点字符串被 tree-shake，UI 无真实入口。
- backend/crates/message_assemble_communicate/src/lib.rs:2812：写端点 message_custom_create，2833-2836 行 sender 取 req.sender 缺省 system。
- 同文件 2861-2866 行：MESSAGE_SENDER_OWNER_PREDICATE 等于 sender=$3 或 admin 角色，3 为会话人 unique。
- 同文件 2869/2882 行：读端点 message_list_paging_page_size_size，SQL WHERE 套该谓词。
- routes.rs:138：POST /api/message/assemble/communicate/message/custom/create 真实注册。

## 4 真浏览器证据四件套
4.1 死代码旁证：测试服 bundle grep -c message/custom/create 等于 0；前端全仓 grep 仅命中 IMChat.vue:821 一处。
4.2 第一轮 r317-m1-verify.cjs：admin 建临时非 admin 账号 HTTP200 success；该账号登录 who 返回 roleList 为空、isAdmin=false；建会话 HTTP200；缺省 sender 与随机显式 sender 消息均 created=true；message/list/paging 返回 data 空、matched 空，verdict=INCONCLUSIVE（对照 sender 非本人，不能定性）。
4.3 第二轮 r317-m1-verify2.cjs：新建临时非 admin 账号 roleList 为空、isAdmin=false；建会话 HTTP200；缺省 sender 消息落库 system 且 created=true；本人 unique 作 sender 消息 created=true；message/list/paging 返回 allRows=1，matched 仅含 SELF 行且 sender 为本人 unique，selfVisible=true、defaultVisible=false，verdict=CONFIRMED_PREDICATE_GAP。
4.4 截图 03-nonadmin-contrast.png 1440x900：非 admin 账号工作台正常渲染，证明会话有效；可见性差异来自后端谓词非前端渲染。全程 0 pageerror、0 非预期 API 失败（空请求体 not a conversation member 为门禁正确拒绝）。

## 5 判定依据
- 谓词缺口坐实：非 admin 会话人 3 等于本人 unique，sender=system 缺省既不本人又无 admin 放行分支则不可见；本人 unique 作 sender 可见（同会话 allRows=1 仅 SELF 行）。
- 死代码坐实：imMore 系列无调用加 bundle 不含端点字符串，无真实 UI 路径触发 customCreate。
- 综合：真实后端语义缺口，但当前无用户可达路径，实际影响为 0，判潜伏 P2，优先级低于有真实入口缺陷。

## 6 修复方向若另派
- message_custom_create 以 sess.person_unique 作为 sender 事实源替代 req.sender 缺省 system，或新增 creator 列并让列表谓词改用 creator，使写读同源。
- 契约 RED→GREEN：非 admin 账号创建缺省 sender 消息后，本人 message/list/paging 必须可见。

## 7 验证方式
- 契约测试固定缺省 sender 等于本人 unique，断言写读同源。
- E2E 复用 r317-m1-verify2.cjs，非 admin 建缺省消息后 defaultVisible=true 即修复成功。

## 8 证据索引均 gitignored
- oa4rust/test-results/r317-m1-verify/evidence.json 与 evidence2.json。
- 截图 01-nonadmin-logged.png、02-nonadmin-after-create.png、03-nonadmin-contrast.png。
- 脚本 r317-m1-verify.cjs、r317-m1-verify2.cjs；DB 只读统计 r317-m1-accounts-probe.py、r317-m1-user-probe.py、r317-m1-pw-probe.py。

## 9 结论
- M1 复验完成：后端谓词缺口已坐实、前端入口死代码已坐实，综合判潜伏缺陷 P2（真实但无实际影响）。
- 项目至少 1000 轮持续迭代，本轮非终态。