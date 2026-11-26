# R64 部署后真浏览器竞态守卫复核报告（postdeploy）

结论：**PASS（带诚实边界）**。五批精确提交已完成，H5 已重建并静态部署，真 Chromium 三视口复刻「A(378 第5关 10411) → B(41 第40关 19925) 快速切换 + history 延迟 3000ms」竞态时序：辅助面板 growthCompare / r14Advice 均归属 B 或保持空态，未出现 A 的 10411→8888 错配。

## 1. 五批提交链（起点 932106f19 = R61 末）

| 批次 | 完整 SHA | 主题 | 写域 |
| --- | --- | --- | --- |
| B1 | 8282c2521b3236c7bdf94929b0f4f1500ed57432 | feat(r64) 战报详情辅助面板竞态守卫 | battles.vue(+13/-5) |
| B2 | 90f85478e35eb4b1a60cf61a9d94680b66eded1c | test(r64) 竞态守卫契约 5 项语义判据 | r64BattlesRace.contract.test.ts(+103) |
| B3 | 3c1d974654113da77bba3e336cd0bd00c52ad485 | docs(r64) 规范与终审 | SPEC.md+ROUND64_DESIGN_REVIEW.md(+140) |
| B4 | 57b73a05f8b26f1d42faa98182b35ee569409308 | test(r64) audit 真浏览器竞态复查证据与 gitignore | .gitignore+REPORT+POSTFIX_REPORT+22脚本+5截图(32文件/+471) |
| B5 | 本批 | postdeploy(r64) 部署后真浏览器竞态复核证据 | REPORT.md+pd-driver.sh+3截图 |

每批提交前均过泄露正则守卫（带口令连接串/私钥/Bearer/Cookie）、`git diff --cached --check`=0、`data/` 暂存数=0。warning 仅 LF→CRLF（既有）。

## 2. 部署产物 sha（均未回退）

| 产物 | sha256 |
| --- | --- |
| 后端 lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 |
| admin WikiView-ST1PbAt2.js | f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b |
| H5 index-DcssZHXt.js | 6011e9cb2b0307686209eb32b811223ce2050397edbe0a875d0e046b4fbaf092 |

`cd laiyipao/miniapp && pnpm run build:h5` 退出 0（DONE Build complete）。index.html 引用 index-DcssZHXt.js。后端/admin 本轮未改，沿用前轮基线。

## 3. 部署后真浏览器复核（三视口竞态时序）

服务：API 8080（start-api.sh，/api/v1/config=200）、H5 静态 5201（/index.html=200）。鉴权由 gen-alias-auth-init.sh 从 QA 库 user10 guest_token 换 user token 写入 data/auth-init.js（517B，只含 lyp_token/refresh/token_exp/guest/profile），两处 data/ 均被 .gitignore(562/564) 忽略。

复刻口径：init 脚本注入 XHR 延迟规则（/me/battles?limit=20&offset=0 与 reaction=superconduct 各延迟 3000ms 一次），先点 B（10411）再 0.4s 内点第 5 行（10411 起始）触发切换，等待 4.2s 后 evaluations r64-probe.js。三视口探针结果一致：

- expandedHead=「胜 ★ 第 40 关 19925」（B 面板，非 A 的 10411）
- detailHash=da91c7bf08b63fa4（属 B，非 A）
- growthText=「AI · 养成前后… 还缺养成前那局战报，先打一局同关 · 构筑变更点未上报，不做估算。」；growthHashes=空 —— growthCompare 保持 B 空态，**无** A 的 10411→8888 错配
- r14Text 仅含 B 的 overheat 相关（「反应 过热 ×51、蒸汽爆发 ×46、超导 ×47」在 kv；r14 面板为「AI 暂时没有足够数据给出建议…」）
- raceLog 显示 /me/battles/477 中间请求仍发出（history 延迟恢复后），但响应被丢弃，未污染面板（见诚实边界 2）

三视口 414×896 / 1440×900 / 820×1180 各跑一组，探针 JSON 落 data/pd-race-*.txt（被 ignore，不入库）。三张截图亲读为「我的战报」列表页、非登录页；竞态判据以 DOM 探针为准。

## 4. 部署后截图 sha（互异且 ≠ QA 五图）

| 截图 | 视口 | sha256 |
| --- | --- | --- |
| pd-race-414.png | 414×896 | 58b89051251790eb4cd1e0711c699fb1f6ea7f5d51f7d951262381856c992a83 |
| pd-race-1440.png | 1440×900 | 83aad5a2cfb2e2efe3a49b7ba63114ab3d054d4c46f68799d73a8cb1d7148a60 |
| pd-race-820.png | 820×1180 | 3ee6eb17672522683bad34b86713be62456f85adafc6f4e175c3e4f9c86005f4 |

QA 五图 sha（race-view-390=1bfc1b52 / race-view-768=f7b44cb6 / race-view-1280=dd1ee470 / race-768-final 与 race-768-afterB 同 sha=d5482858），三张新图均不等于其中任一张。

## 5. 门禁与回归

- Go：`TestNoInfraIdentifiersInRepo` PASS(8.22s)；`TestNoRealSecretsInRepo` SKIP 非 PASS（缺 .local.keys，逐字「真值扫描这条守卫**没有执行**，不是「通过了」」）；GO_RC=0
- QA 库 battle_records count|max(id)=104|477（容器 lyp-qa-pg）
- 全仓 git diff --check RC=0；data/ 与 auth-init.js 未入库；源码无 --r64- 业务令牌；tokens.css 三锚点在（--lyp-token-version / --lyp-r3-version / --lyp-r6-version，本轮未改）
- 回归：R61 v-aria-native 桥接、R60 role=button 键盘可达、R55 视觉可读性、R53 pct 取整均未回退（本轮仅改 battles.vue 两处辅助加载函数的 await 后复核）

## 6. 诚实边界

1. `TestNoRealSecretsInRepo` 因缺 .local.keys SKIP，按规则不计为 PASS。
2. 477 中间请求残留：A 延迟 history 恢复后仍发 1 次 /me/battles/477，但响应在写 ref 前被 seq 守卫丢弃，不污染面板；彻底消除该请求为可选加固项，非本轮目标。
3. 主 bundle 中序号守卫标识符经压缩后不以 detailPanelSeq 字面出现，故「bundle 含序号语义」以真浏览器 DOM 行为探针坐实，不以 grep 字面命中判定。
4. 三张部署后截图为战报列表页顶部（未滚动至展开行），竞态四判据（归属/空态/无错配/仅 B 建议）以 DOM 探针 JSON 为准，非截图视觉。
5. 终审列为非阻塞观察的「RED 原始日志未留存」依 SPEC 记录 + 契约结构 + GREEN/回退对照采信；本轮发布不改判定。

综合：R64 目标「旧响应晚到不污染辅助面板」经部署后真浏览器三视口复核成立，可发布。
