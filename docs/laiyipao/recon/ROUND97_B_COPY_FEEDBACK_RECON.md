# R97-B 复制/分享功能失败反馈与降级一致性 只读侦察报告

- 轮次：R97-B；执行：same-route-reviewer；attempt_id：596907b0-27e1-491b-a3d0-1409bdd3a5ed
- 基线 HEAD：586f77c227642bce4f69385d1f9ec7be64053e06（全程未变）
- 范围：laiyipao/miniapp/src 全部复制/分享 handler + laiyipao/admin/src 复制 handler
- 性质：纯只读侦察，不改业务源码/契约/规范/令牌/测试，不写真值，不写 git

## 一、结论摘要

**列 1 个推荐候选（四条件全中）**：miniapp 两处复制 handler 仅有 `success` 回调、缺 `fail` 回调，剪贴板 API 失败时静默吞错，用户点击「复制」后无任何反馈。真 Chromium 三视口（390/768/1280）×成功/失败两态坐实：成功有 toast「已复制用户 ID」，失败零 toast。同仓 admin 12 处复制统一 `try/catch + ElMessage.error("复制失败，请手动选择复制")` 提供权威可对齐口径；改动面为两处各补一个 `fail` 回调，不碰令牌三锚点/业务语义/测试不变量。

分享（onShareAppMessage）为纯函数返回分享卡片配置，走微信原生菜单转发，无成功/失败 toast 环节，不属静默吞错范畴，不列候选。

## 二、静态审计表

| 位置 | handler | 成功反馈 | 失败反馈 | 判定 |
|---|---|---|---|---|
| me.vue:714-721 | copyUserId | success→toast「已复制用户 ID」(:718) | **无 fail 回调** | 缺陷 |
| battles.vue:896-904 | copyHash | success→toast「已复制回放哈希」(:901) | **无 fail 回调** | 缺陷 |
| me.vue:110-111 | 模板入口 @click="copyUserId" | — | — | 触发点 |
| battles.vue:312-313 | 模板入口 @click="copyHash(detail.replay_hash)" | — | — | 触发点（需战报数据） |
| admin 12 处 | copyReplayHash/copyBattleId/copyAnnBody/copyRedeemCode/copyAuditDetail/copyAuditTarget/copyDefenseId/copyChallengeId/copyOrderNo/copyShopId/copyUserId 等 | ElMessage.success | **catch→ElMessage.error("复制失败，请手动选择复制")** | 权威范式 |
| share.ts | indexShare/rankShare/battleShare 纯函数 | 无（微信原生菜单） | 无（同上） | 非缺陷 |

admin 12 处精确行号：BattlesView.vue:162 copyReplayHash / :175 copyBattleId；ContentView.vue:393 copyAnnBody / :628 copyRedeemCode / :689 copyAuditDetail / :777 copyAuditTarget；DefensesView.vue:29 copyDefenseId / :42 copyChallengeId；EconomyView.vue:239 copyOrderNo / :252 copyShopId；RecomputesView.vue:301 copyBattleId；UsersView.vue:464 copyUserId（try 在 :463，ElMessage.success 在 :465，catch→ElMessage.error 在 :466-467）。

分享调用点：index.vue:719 onShareAppMessage(()=>indexShare())；rank.vue:333-339 rankShare；battle.vue:1767-1781 battleShare（以 settled: !!settleResult.value && !settleResult.value.error 防未结算假数据）。share.ts 注释充分防假数据/路径脱节。

## 三、真浏览器三视口数据表

环境：H5 常驻 127.0.0.1:5174；API 本地回环占位库 127.0.0.1:15440（go run ./cmd/api）；guest token 经 base64 注入 localStorage（真值运行时不回显不落盘）。通过覆盖 uni.setClipboardData 注入 success/fail 回调模拟成功/失败两态。

| 视口 | 失败路径 clip 探针 | 失败路径 toast | 成功路径 toast |
|---|---|---|---|
| 390×844 | CLIP:hasSuccess=true:hasFail=false | **[]（静默）** | TOAST:已复制用户 ID |
| 768×1024 | CLIP:hasSuccess=true:hasFail=false | **[]（静默）** | TOAST:已复制用户 ID |
| 1280×800 | CLIP:hasSuccess=true:hasFail=false | **[]（静默）** | TOAST:已复制用户 ID |

对照诊断（job bash-785260d1f9bf47f5）：直接调 uni.showToast 可被 hook 记录（证明 hook 有效）；点击复制后记录 CLIPCALL:hasSuccess=true:hasFail=false（handler 只传 success 不传 fail）；FAIL_CB_FIRED:setClipboardData:fail（H5 复制确实失败）；失败后无任何 SHOWMODAL/TOAST 记录——静默吞坐实。

## 四、截图与亲读

截图目录 docs/laiyipao/audits/round97-b-copy-feedback/data/，7 张 SHA-256：
- me-390-success.png = bd8e97bd817a7e2dfb8b88066095d4d48282af7ded21d1060d4cb01f786a0e61
- me-390x844.png = 794f0ce4317931b8beb8374219f2eb02f93a9c3e481636d0e2ac5132353da1a0
- me-390x844-fail.png = 2ac4a6f552a067f8578e78f5c61e37cc6889d1b330d69dad0835ca1a2345e604
- me-768x1024.png = 734ab0443fc7191dac08aae968eb08c4c001a129569e4d97b9a537440b98e06c
- me-768x1024-fail.png = a487ab7fb698a9b8031a0975d6e5c3aed7fc92e40196abd83e8ce3de5f487f0b
- me-1280x800.png = 97eb42c5cc86d304fcb6bfc5db70b7acf4d175df4784b0de3cfad92e4f487f36
- me-1280x800-fail.png = 3e0d14d0e03d0f65b95095561151ac42c511f5806af3203e8ea08a5542f0fbca

亲读 2 张非登录页：me-768x1024.png（768 常态，档案卡片正常呈现）；me-1280x800-fail.png（1280 失败态，底部「ID 1530 复制」入口无任何 toast，与日志 toast=[] 一致）。UNIQUE 7/7。

## 五、四条件裁定表

| 条件 | 判定 | 证据 |
|---|---|---|
| ①真实可复现用户可见缺陷 | 成立 | 三视口失败路径零 toast；用户无权限时以为复制成功实则失败 |
| ②展示层可修有同仓权威口径 | 成立 | admin 12 处统一 catch→ElMessage.error("复制失败，请手动选择复制") |
| ③改动面小零风险 | 成立 | 两处各补 fail 回调，不碰令牌三锚点/业务语义/测试不变量 |
| ④真 Chromium 三视口坐实 | 成立 | 390/768/1280 × 成功/失败两态，clip 探针+toast 探针+截图 |

## 六、最小修复边界（供实施阶段参考，本轮不改）

- me.vue:714-721 copyUserId：在 uni.setClipboardData 参数对象补 fail: () => { uni.showToast({ title: "复制失败，请手动选择复制", icon: "none" }) }。
- battles.vue:896-904 copyHash：同上补 fail 回调，文案对齐「复制失败，请手动选择复制」。
- 文案直接复用 admin 权威口径；不引入新令牌、不改 success 路径、不动 share.ts。

## 七、诚实边界

1. battles 页 copyHash 在 guest 态无战报数据（bodyLen=195 空态），复制入口不可见，浏览器侧未能实际触发；其缺 fail 由静态审计确认，裁定成立但缺真浏览器两态坐实（该页单独标注）。
2. H5 复制失败通过覆盖 uni.setClipboardData 注入 fail 回调模拟，非真实系统剪贴板权限拒绝；但已证明 API 失败路径存在且无反馈，缺陷机制成立。
3. R87-B（docs/laiyipao/recon/ROUND87_B_WRITE_FEEDBACK_RECON.md）聚焦成功反馈与回显，将 copyUserId 列为「完整」，未审失败分支；本轮补齐该缺口，非推翻 R87-B 结论。
4. docs/laiyipao/design/AI_VISUAL_SYSTEM.md:519-521 规定错误态与成功态等高切换，无专门剪贴板失败文案规范；文案取自 admin 同仓权威口径而非设计文档。

## 八、hygiene

- HEAD：586f77c227642bce4f69385d1f9ec7be64053e06（全程未变）。
- tracked 改动：非 ?? 项 0；git diff --check RC=0。
- auth 四路径：ABSENT（.env/.local.keys/remote-config.env/laiyipao/deploy/.local.keys/laiyipao/server/.env 及 laiyipao 下 *.pem/*.key 均不存在）。
- 自启 job：API（bash-e678327577364243）、H5（bash-f0bc54fa2fda4793）、采集 driver（bash-a92428bb7b5e4ddc）等，收尾统一 job_kill 清理。
- QA 库：纯只读，未写库。

## 九、脚本资产（均 UNTRACKED）

docs/laiyipao/recon/ 下 .r97-probe.js / .r97-clean.sh / .r97-parse.py 等探针、driver、解析器及对应 .log；截图位于 docs/laiyipao/audits/round97-b-copy-feedback/data/。
