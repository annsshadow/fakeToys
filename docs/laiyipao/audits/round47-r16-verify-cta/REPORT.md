# R47 真浏览器 QA：验真 CTA 点击定位验真区

结论：**PASS（带诚实边界）**。真实 Chromium（1280x720）打开战报详情抽屉（id=477，含 reaction_timeline），出现 AI · 反应时间线与可执行 CTA「发起验真 →」；用真实低位鼠标 move/down/up 点击 CTA 后，验真区 .verify-row 被滚入视口居中，replay_hash / hash 输入框 / 验真结论均未改变，无页面跳转、无自动提交。

## 一、环境与数据来源
- 前端：pnpm exec vite --host 127.0.0.1 --port 5191 --strictPort，http://127.0.0.1:5191/laiyipao-admin/ HTTP 200（固定端口，避免漂移 chrome-error）。
- API：docs/laiyipao/audits/round15-reaction-timeline/start-api.sh，http://127.0.0.1:8080/api/v1/config HTTP 200。
- 鉴权：管理员令牌由 gen-fresh-admin-init.py 运行时从环境变量 BOOTSTRAP_ADMIN_PASS 登录 /api/v1/admin/login 取得，仅写入 gitignored 的 data/admin-auth-init.js（localStorage lyp_admin_token），脚本与日志从未回显口令/令牌。历史 login.json 令牌已过期（401），未复用。
- QA 库基线 battle_records count=104 / max(id)=477；含 reaction_timeline 的记录 id 477/378/341（列名为 reaction_timeline 本体，不在 payload 内）。本轮仅只读查询，未写库。

## 二、真实浏览器证据（会话 lyp-r47b）
- data/open2.txt / data/list-probe.json：href=http://127.0.0.1:5191/laiyipao-admin/#/admin/battles，isLogin=false（未跳登录页）。
- data/mark.txt = "marked=1"；data/detail-click.txt = ✓ Done（真实选择器点击 #r47-detail-477）。
- data/cta-scroll.txt = "scrolled"；data/cta-before.json：drawerOpen=true、ctaText="发起验真 →"（UTF-8 复核 verbatim 一致）、ctaCursor=pointer、ctaRect={x:862,y:378,w:361,h:16,cx:1043,cy:386}。
- 实鼠标 data/cta-move.txt / cta-down.txt / cta-up.txt 均 ✓ Done，CTA_MOVE=1043,386（CTA 文本中心）。
- data/cta-after.json（点击后）：href 不变、isLogin=false、drawerOpen=true、verifyRows 首行 top 由 117 → 378（视口 720 内居中）、ctaRect.y 由 378 → 639；hashes=[8ddc5cc01e787ec9, 5621489295744462231] 前后完全一致；hash 输入框 value 为空；verifyResultCount=0（无验真结论）；bodyLen=7634 不变；windowScrollY=0。证明 CTA 仅定位，未提交、未填 hash、未改 replay_hash、未跳页。
- AI 四要素可见：timelineText 含「AI · 反应时间线 / 本局记录 64 次逐次反应 / 依据：reaction_timeline.events（wave/tick/reaction/enemy_uid/damage）/ 展示性归因、非权威结算」；CTA 可执行语义符合 SPEC。
- data/probe2.json（UTF-8 渲染）：ctaText="发起验真 →"，verifyRowTop=562；滚动容器为抽屉内部（el-drawer__body h=2259/ch=2178），scrollTop=0 因 scrollIntoView 走浏览器平滑滚动/父容器定位 —— 以 getBoundingClientRect 位移为准，不虚构 scrollTop 变化。

## 三、截图 sha256（3 张互异、非登录页）
| 文件 | sha256 | 说明 |
|---|---|---|
| screenshots/r47-detail-open.png | 95acdd9ac1c901117ca07f6c9a285bac6e92d720157fea9cb29af5936f726565 | 抽屉打开、AI 时间线与 CTA 可见 |
| screenshots/r47-cta-before.png | 745ddcca5beb36b9fb2eb6b4f94fc88598e7b0ab7959d7d72a547fd2491a82bd | 点击前，验真区在抽屉顶部 |
| screenshots/r47-cta-after.png | af42c501cf5b5fac33ba5f4aeaf1625ed5bc69088dcda75fba006a06e7ce68bd | 点击后，验真区居中（视觉复核：种子/空 hash 输入框/提交比对按钮完好，AI 块在下方） |

## 四、门禁
- pnpm exec vitest run tests/R16ReactionTimeline.render.test.ts：13/13 通过，RC=0。
- pnpm exec vitest run --testTimeout=90000（admin 全量）：903/903 通过，RC=0。
- pnpm run type-check：RC=0；pnpm run build：RC=0（built in 1.72s）。
- Go 守卫：TestNoInfraIdentifiersInRepo PASS；TestNoRealSecretsInRepo **SKIP，非 PASS** —— 精确信息：「读不到 WORKTREE_ROOT/laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是「通过了」」。

## 五、诚实边界与假阳性
1. 第一次 driver job bash-7e21030a29bf4fcd 因令牌过期跳登录页、mark 脚本括号不配对，证据无效，未采用；第二次 bash-435901f4ef3d4af0 有效。
2. drawerScrollTop 始终为 0，定位证据以 .verify-row 的视口 top 位移（117→378）为准，不声称 scrollTop 变化。
3. data/admin-auth-init.js 含运行时令牌，被通用密钥扫描命中属预期；该文件已删除且被 .gitignore:482 覆盖（git check-ignore -v 命中），未入库。
4. 静态扫描 26 个审计文本文件，除临时 auth 文件外无 JWT/Bearer/带口令连接串真值。
5. 本轮三张截图均为本会话新采、互异、非登录页。
6. 源码仅只读核对：BattlesView.vue:286 scrollToVerify() 仅 scrollIntoView，submitVerify 未被 CTA 调用；契约由 t60 实施，本轮未改任何业务源码、未做 git 写操作。
