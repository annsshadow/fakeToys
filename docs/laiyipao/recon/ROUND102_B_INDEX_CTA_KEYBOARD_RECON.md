# R102-B 首页 CTA 键盘可达性侦察报告

- 轮次：R102-B（只读侦察）
- 视角：小程序 H5 首页（index.vue）CTA / 入口元素的键盘可达性
- 基线 HEAD：5ec099e5508b853a4cc9b8936d09d76ec1020589（全程未变）
- 交付：docs/laiyipao/recon/ROUND102_B_INDEX_CTA_KEYBOARD_RECON.md

## §0 摘要结论

列 1 个推荐候选（四条件全中）：index.vue 首页 5 处用户可感知的导航/恢复入口仍为裸 view / text，缺 role=button + tabindex=0 + @keydown.enter 键盘三件套，键盘用户（仅 Tab/Enter）完全无法触达：

1. :40 错误态「重试」按钮（登录失败时唯一恢复路径，优先级最高）
2. :53 新手引导卡 first-run-guide（新玩家首屏目标）
3. :78 AI 洞察「去补强 →」CTA
4. :92 AI 洞察「去战斗 →」CTA
5. :109 公告「全部 N 条 →」入口 ann-all

不重复立轮：:189-192 空维补导 go(d.hint) 已由 R99 契约 r99LinkKeyboard.contract.test.ts:53 CASES「index 空维补导」明列覆盖，源码已含三件套，不在本候选内。

## §1 逐处盘点表

| 行号 | 元素 | handler | role/tabindex/@keydown | 现有契约覆盖 | 裁定 |
|---|---|---|---|---|---|
| 40 | view.btn.btn-primary 重试 | retry | 全缺 | 无 | 候选（最高） |
| 53 | view.card.first-run-guide.tap | goStage | 全缺 | 无 | 候选 |
| 78 | view.btn.ai-cta 去补强 | go(aiInsight.hint) | 全缺 | R60 仅守 @click 存在，不守 role | 候选 |
| 92 | view.btn.ai-cta 去战斗 | goStage | 全缺 | R60 仅守 @click 存在，不守 role | 候选 |
| 109 | text.ann-all 全部 N 条 | goAnnouncements | 全缺 | R99 19 处 CASES 全文无 ann-all / goAnnouncements | 候选 |
| 189-192 | text 空维补导 | go(d.hint) | 已补三件套 | R99 CASES:53 明列 | 已覆盖，不立轮 |
| 115-119 | text.ann-item | toggleAnn | 已补三件套+aria-expanded | R100/R101 覆盖 | 已覆盖，不立轮 |

handler 定义（index.vue:686-701）均为真实导航：go(path) 到 uni.navigateTo、goAnnouncements() 到公告页、goStage() 到关卡页、retry() 到 await store.login()；非假入口。store/game.ts:384 在 doLogin 失败时置 loginError，触发 :40 错误态渲染。

## §2 权威口径

- 仓内同构范式（R96）：r3-empty-action 在 6+ 页统一补 role=button + tabindex=0 + @keydown.enter 同 handler（common.css:1346 定义，announcements/bag/battles/enemies/equipment/gems/mastery/me/rank/reactions/shop 均已修）。
- R99：r99LinkKeyboard.contract.test.ts 19 处 link 补同款三件套，含 :192 go(d.hint)。
- R100/R101：可点击行/展开控件继续沿用三件套并补 aria-expanded/语义播报。
- 焦点环：common.css:914 :where(a,button,input,textarea,select,[tabindex]):focus-visible outline 3rpx solid var(--accent)，:918 裸 :focus-visible；[role=button] cursor:pointer。
- 契约现状：r60A11yKeyboard.contract.test.ts:29-54 的 roleButtonLines 只逐行扫描已含 role=button 的元素，裸 view 不在其覆盖；:79-80 仅守 @click=go(aiInsight.hint)/goStage 存在，不守 role/tabindex/keydown。全仓契约 grep 确认无一处守 retry/ann-all/first-run-guide/ai-cta 的三件套。
- WAI-ARIA：交互入口应暴露为 button 角色并可键盘操作（APG keyboard-accessible 原则）。

## §3 真浏览器证据（agent-browser 0.41.1，真 Chromium，H5 常驻 5174）

- 三视口 390×844 / 768×1024 / 1280×800 完全一致：BODYLEN=976；.first-run-guide/.ai-cta/.ann-all 三者 role=null、tabIndex=null、cursor=auto；对照组 .ann-item/.dim-hint/.mg-row/.action-card/.grid-item 全 role=button/tabIndex=0/cursor=pointer。
- 14 次真实 press Tab 序列全部命中已修元素（ann-item×3 / dim-hint×3 / mg-row×3 / action-card×1 / grid-item×4），完全跳过三处 CTA 盲区；AX_TARGETS=0（三处 CTA 在 AX 树缺席）。
- 错误态（init-script 拦截 guest 返回 500）单视口坐实：错误页 bodyLen=82，含「无法连接服务器 / 请求失败 HTTP 500 / 重试」；重试按钮 role=null、tabIndex=null、cursor=auto，同样不在键盘序列中。截图亲读确认红色大按钮非空态。
- 服务：API job bash-42a99cd454d04f41（工作目录 laiyipao/server，回环库 + 占位口令 + PLAYER_LOGIN_REQUIRED=false + SETTLE_RECOMPUTE_MODE=shadow）；H5 job bash-5a197f0d2fd94a85（工作目录 laiyipao/miniapp，pnpm dev:h5 --host 127.0.0.1）。guest token 运行时注入 localStorage，不回显/不落盘。init-script 公告为合成数据，零真值。

## §4 截图证据（UNIQUE 4/4，另错误态 1 张）

- r102-index-390.png sha 743c68127afd0479da6f3dff831669c734f1e53d1a05dd1c4150d5fcb9d6f9af（70784B，亲读：资源栏/攻方属性/当前关卡/玩法九宫格）
- r102-index-768.png sha 768aeae8c4967c224b509099a658474b91ad6f78b8fa51983d490095999ee222（69479B，亲读：玩法+图鉴网格，橙框焦点环落已修「专精」grid-item）
- r102-index-1280.png sha ffb89c0f73230b04402074532787217cce05ef80fcc14c295d7a6d3f70ddabdb（84189B）
- r102-explore-390.png sha 49dbbec9593420cd267006a851a5c7d29dfee1f230e253feda775a63f515bbf7
- r102-error-390.png sha 4bf2d50f3abbf12c2cab1775bcbfe8b9360c07a252f7a03c30b82198d3dbac73（40245B，亲读：错误态重试按钮）

## §5 四条件裁定

- ① 真实可复现用户可见缺陷：成立。三视口 DOM/AX/真 Tab 三重坐实 5 处入口键盘不可达（错误态重试为唯一恢复路径，危害最大）。
- ② 展示层小改：成立。每处各补一行三属性，零新增 DOM/逻辑，不动 handler。
- ③ 权威口径明确：成立。R96/R99/R100/R101 同仓范式 + WAI-ARIA。
- ④ 改动面可控：成立。仅 index.vue 单文件 5 处，不动令牌/测试不变量。

最小修复建议：index.vue:40/53/78/92/109 各补 role=button + tabindex=0 + @keydown.enter 指向同名 handler（与已有 @click 同 handler），并新增契约守卫防止回退。按可感知优先级：重试 > 新手引导 > 去补强/去战斗 > ann-all。

## §6 观察项（非候选）

- O1：:189-192 go(d.hint) 已被 R99 CASES:53 覆盖，不重复立轮。
- O2：:115-119 ann-item 已被 R100/R101 覆盖三件套+aria-expanded。
- O3：搜索类输入实时 computed 过滤无需 Enter 属设计取舍（R98-B 已裁定）。

## §7 诚实边界

- 错误态重试以 init-script 注入 guest 500 合成（非真实服务故障），坐实的是客户端 DOM 属性与键盘不可达因果。
- 错误态 Tab 用合成 KeyboardEvent 不触发浏览器默认焦点移动；键盘不可达结论依据 tabindex 缺失（天然不在 Tab 序列）与正常态 14 次真实 Tab 跳过模式。
- 截图 5 张亲读 3 张（index-390/index-768/error-390），其余 sha 独立重算 UNIQUE。
- 顶部 CTA 在截图时为 14 次 Tab 后已滚出视口的状态，DOM 探针坐实其存在。
- 焦点环像素在 768 为 3px、其余视口 1px（rpx 换算差异），不影响可达性结论。

## §8 hygiene

- 基线 HEAD 5ec099e5508b853a4cc9b8936d09d76ec1020589 全程未变；本轮只读侦察未改业务源码/契约/令牌。
- 审计产物落 docs/laiyipao/audits/round102-b-index-cta/（data/screenshots），零真值（guest token 运行时注入不回显，公告为合成）。
- 自启 job 收尾清理：API/H5 服务保留给后续轮次，采集/错误态 driver job 全部 kill/completed。
