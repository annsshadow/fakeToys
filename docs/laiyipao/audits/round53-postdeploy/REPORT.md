# R53 部署后真浏览器复核报告（postdeploy）

- 任务：t82 R53发布：首页pct取整五批提交与H5重建部署
- attempt_id：0850f41d-d706-47a0-baee-53308c59aae4
- 结论：**PASS（带诚实边界）**
- 起点 HEAD：6d0c54c8101799f0b29ac84cd993cd0d0322ce74（R51末）
- 发布分支：opt1000/laiyipao-ai-refactor；唯一发布者 release-ops；零真值入库。

## 1. 部署产物（重建 H5，其余沿用不回退）

| 产物 | sha256 | 状态 |
|------|--------|------|
| 后端 lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | R21 沿用不回退 |
| admin WikiView-ST1PbAt2.js | f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b | R47 沿用不回退 |
| H5 index-B1xav1GR.js | 1e77b23bf76159a47bbd6b914cd25860653180b2669060cf33372eb00a4b058e | 本轮 cd laiyipao/miniapp && pnpm build:h5 重建，RC=0 |

- 重建为确定性构建：index.html 引用 index-B1xav1GR.js，产物 sha 与重建前一致；bundle 内长浮点字面量 44.44444444444444 命中 0 处、Math.round 命中 7 处。
- 服务：API 8080（start-api.sh 后台，config 端点 200）、H5 静态 5201（python http.server 127.0.0.1，页面源 200），打开浏览器前均验证精确端口 200。

## 2. 五批精确提交（每批 git diff --cached --check=0、LEAK=0）

| 批次 | hash | 内容 |
|------|------|------|
| B1 feat(r53) | 50921e9df042578c7a8d6553d463e0be667e0cb7 | index.vue dims 五维 pct 加 Math.round（5 处） |
| B2 test(r53) | 7a40c845a78b95845321f0c334e1943e4d4f3b75 | r53HomePct.contract.test.ts（真实源码 + 权威 helper） |
| B3 docs(r53) | 85991977c00f5ff750e5d9e1f30d02804d334194 | SPEC.md + ROUND53_DESIGN_REVIEW.md |
| B4 test(r53) audit | b9fa1895ec648b5566e63283cc21ef81c800f76d | .gitignore（仅 R53 四行）+ REPORT + 9 根脚本 + 8 截图，共 18 文件 |
| B5 postdeploy(r53) | 见下方提交后 HEAD | 本报告 + r53-probe.js + postdrive.sh + 3 张 pd 截图 |

- 提交后至 B4：HEAD=b9fa1895ec648b5566e63283cc21ef81c800f76d，AB=4/0，工作树 tracked clean。
- 精确 add：禁 git add -A/点号；审计 data/ 含令牌文件全部排除，git check-ignore 命中 .gitignore:506（审计 data/）与 510（postdeploy data/）。

## 3. 真浏览器证据（Chromium 390×844 / 768×1024 / 1280×800）

- 认证：QA 库 user10 未过期 alias 经 POST /api/v1/auth/guest 换 user token，注入 localStorage（auth-init.js 仅含 lyp_token/refresh/token_exp/guest/profile，存放于 gitignored data/，收尾删除）。
- build_rating 与 rating_weights 通过 agent-browser network route 在浏览器会话侧注入（allow-origin=页面源 http://127.0.0.1:5201），不改 DB 与服务端。
- case1（390×844，pd-case1.json）：hash=#/pages/index/index，innerW=docW=390，AI 结论逐字「装备契合仅 44%（权重 21%）：优先补它，收益 210」，longFloat=false；五维宽度 element100%/reaction43%/mastery100%/equipment44%/mechanic100%（全整数）；雷达 viewBox=0 0 200 180、rings=3、axes=5、radarVisible=true、shape 多边形可见；收益序列 mg=[装备契合收益 210, 反应链 104, 元素覆盖 0]。
- empty（768×1024，pd-empty.json）：build_rating 置空后 ?? 0 兜底，结论「装备契合仅 0%（权重 21%）：优先补它，收益 378」，longFloat=false；五维宽度全 0%、雷达 shape 收于中心 100,90；innerW=docW=768 无溢出。
- case2（1280×800，pd-case2.json）：元素覆盖 2/5 另一存档，结论仍为整数 44%、装备契合 width 44%、元素覆盖 width 40%，longFloat=false；innerW=docW=1280，雷达 rings3/axes5 可见。
- R4/R12/R48/R51 不回退：QA 阶段已回归四页同短板装备契合、equipment 自指 CTA、me 页整数 44% + 雷达；本轮仅改首页五条 pct 表达式，未触碰这些页面（见 SPEC §2 不变量与终审维度二 diff 亲读）。

## 4. 部署后截图 SHA256（互异，亲读非登录页，≠QA 八图）

| 截图 | 视口 | 内容 | sha256 |
|------|------|------|--------|
| pd-case1.png | 390×844 | 首页整数 44% + 雷达 + 100% 进度条 | 56bed657009c5a9661e9a93b12fce657bde0498d668e28b9b53d837614a3a0f8 |
| pd-empty.png | 768×1024 | 首页 0% 空态兜底 + 0% 进度条 | 35259eb3deb17fc49b293d4d5731d9e180fe594b0a4d5896859fecd2178445d6 |
| pd-case2.png | 1280×800 | 首页整数 44% + 元素 2/5 40% 进度条 + 雷达 | 7e9b84c6aabf7a67a49db58275aab1244f9113e49dc5f8cf0f438d1f29505382 |

- 三张 pd 图 sha 两两互异，且均不在 QA 八图 sha 集合（028a4520/e0586967/f744ad3e/dc9b9c12/edfa7630/db55d757/dd7eb60c/e23de2aa）内。
- 三张均 read_image 亲读，为游戏首页（玩家 9100、AI 战术洞察、装备契合 44% 或 0%、去补强 CTA、构筑评分雷达与进度条），非登录页 / 非 chrome-error 页。

## 5. 门禁与收尾

- Go 守卫：cd laiyipao/server && go test ./internal/domain -run 'TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo' -v，GATES_EXIT=0；TestNoInfraIdentifiersInRepo PASS(5.07s)；TestNoRealSecretsInRepo **SKIP 非 PASS**，原文「读不到 <WORKTREE_ROOT>/laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是「通过了」」。
- QA 库：docker exec lyp-qa-pg 查 battle_records count|max = 104|477，与基线一致。
- finalize：删除 postdeploy/data/auth-init.js 等令牌注入文件；data/ 受 .gitignore:510 保护。
- 推送前一致性核对：tracked clean、staged 空、HEAD=b9fa1895、AB=4/0、remote=6d0c54c81，无历史残留误入。

## 6. 诚实边界

1. TestNoRealSecretsInRepo SKIP 非 PASS：缺 .local.keys，真值扫描守卫未执行，本报告不记为通过。
2. build_rating / config 为浏览器会话侧 network route mock，仅作用于该浏览器会话，未改 DB 与服务端；权重与服务端权威一致 {12,15,10,12,8}。
3. 空 rating 分支以 ?? 0 兜底产出 AI 结论（0%），SPEC §4 期望的「AI 暂时没有足够构筑数据」文案分支未出现；如实披露，不伪称触发。
4. 产物为确定性构建，H5 sha 与重建前一致， 真正的修复体现在源码五条 Math.round 与 bundle 无长浮点字面量，非哈希漂移。
5. 端口纪律：5174 旧 302 服务不用；静态 5201 与 API 8080 打开前均验 200。
