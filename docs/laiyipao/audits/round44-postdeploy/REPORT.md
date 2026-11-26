# R44 部署后独立回归报告：R22 只读 CTA 文案上线与真实点击无副作用

- 任务：t55（attempt c0d8c186-2704-44f2-8dad-ca7a1566e5bf）
- 审计目录：docs/laiyipao/audits/round44-postdeploy/
- 发布对象：R44（t49 侦察 / t50 规范 / t51 实施 / t52 真浏览器复查 / t54 终审 verdict=pass 全闭环）唯一提交部署。
- 起点 HEAD：d8a41a83847cf9c8ad3be4bff1a1bfef03e36ef8（R43 末）；五批提交 B1..B5 见第七节。

## 一、结论

**PASS（带诚实边界）**：仅重建 H5、后端与 admin 产物沿袭不回退。部署后另起真实 Chromium（390×844）进入 L5 选卡协同预判阶段，真实底层鼠标点击 R22 新 CTA「↑ 点上方卡牌即可装备它 →」后，href、手牌、预判结论/依据/诚实标注、clock、bodyHead、HUD 共存逐字/逐字段不变，帮助层/时间线/暂停全空；正对照真实点击手牌卡牌正常 takeCard 推进到波次 2、clock 139→125 且换卡。4 张有效新图 sha 互异且与 QA 基线 17 张零重叠。密钥真值扫描守卫因缺 .local.keys 为 SKIP，如实标注非 PASS。

## 二、范围与产物 sha

- 仅重建 H5：cd laiyipao/miniapp && pnpm build:h5（Compiler 5.24 vue3，Build complete；仅 Appid 提示）。index.html 仍引用 index-BQ39M86b.js，确定性构建产物 sha 不变：
  - index-BQ39M86b.js = a90137484fcd6467946174cc3d2eae88c5c13e8d683425bc1271f61c4bd032b4
  - pages-battle-battle.BM4KKStv.js = 1c5f38986e9f1129c1c2591b74acd586cbfb86661fc677233cd3233afc9b83f4
- 新 bundle 核对：新 CTA 文案命中 1，旧 CTA「去装备/选它」命中 0，r22-ai-cta.{0,120}openHelp 命中 0，r27/r24-ai-cta 邻近 openHelp 命中 0。
- 后端沿用 laiyipao/deploy/bin/lyp-server-amd64 = 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 基线，未回退）。
- admin 沿用 laiyipao/admin/dist/assets/WikiView-JpQ4MFXA.js = cc70f6987a04064744d42620d99254554bdd84a4ffbd6999667230bba487ea0c（R42 基线，未回退）。
- 源码锚点只读核对：battle.vue:364 新 CTA 文本无 @click；:58 / :95 的 r27/r24 CTA 仍为只读文本（R43 不回退）；:116 .help-chip 的 @click=openHelp 保留（帮助入口正对照）。

## 三、环境与服务

- API :8080 用 round15-reaction-timeline/start-api.sh 后台启动；H5 静态 :5274（5174 被并行 oa4rust-h5 占用，避免端口漂移）；五端点 healthz/readyz/config/H5 根与深链均 200。
- curl 默认代理 env 使 127.0.0.1 显示 000，统一用 --noproxy * / NO_PROXY=127.0.0.1,localhost 后为真值。live config version=6237e0b623219b8b。
- 会话未设 QA_ADMIN_USER/PASS；经 QA 库未过期 guest_token_aliases（user_id=10，max_stage=60）在回环 /api/v1/auth/guest 换 user kind 令牌，open 阶段 --init-script 注入 lyp_token / lyp_token_exp / lyp_profile，令牌真值不入报告与提交。
- H5 在 5274 全程稳定，无 chrome-error 错误页；任何 open 失败轮次均重来且不作证据。

## 四、进入真实战斗态

- 视口 390×844（先 open 再 set viewport），深链 #/pages/battle/battle?level=5。R22 预判仅在 engine.phase=card_select 且有已装备元素与带元素手牌时出现：初始 clock=143、hasR22cta=false 属未到选卡阶段，不作为证据。
- 轮询等待 .sheet/.hand-card/.r22-ai-cta 三者同时出现（CARD_SELECT_READY=1）后采集 before：clock=139、cardCount=3、r22CtaRect={x:21,y:776,w:351,h:14}、hudCoexist={r22,r24,r26,r27,r28=true,r29=false}。

## 五、闭环逐字段对账（最终有效证据）

| 字段 | CTA 点击前 | CTA 点击后 | 正对照点卡后 |
| --- | --- | --- | --- |
| href | .../battle?level=5 | 同左，未跳页 | 同左 |
| r22 CTA 文案 | ↑ 点上方卡牌即可装备它 → | 逐字不变 | 逐字不变 |
| 手牌 | 燃烧弹:焰 / 元素容器 / 快速装填 | 同左（完全一致） | 燃烧弹 / 锋锐训练 / 扩张装药（takeCard） |
| 结论/依据/诚实标注 | 当前手牌「燃烧弹」可强化已有【蒸汽爆发】等 | 三项均逐字不变 | 同左 |
| clock | 139s | 139s（card_select 阶段 tick 冻结） | 125s（推进到波次 2） |
| 波次/bodyHead | 波次 1/5、击杀 3 | 逐字不变 | 波次 2/5、击杀 8 |
| helpOverlay / hasTimeline | false / false | false / false | false / false |
| hasPause / hasResume / pauseLabels | 0 / 0 / [] | 0 / 0 / [] | 0 / 0 / [] |
| HUD 共存 | r22/r24/r26/r27/r28=true | 同左 | 同左 |

真实点击坐标：CTA 中心 (197,783)，手牌中心 (70,523)，均为 mouse move + mouse down left + mouse up left 低层事件，非 DOM eval 派发。CTA 点击确实无副作用；点卡主路径未被破坏。

## 六、有效新图与 sha 互异

- pd2-r22-before.png = 5a6e16be0059e9282b140b237f3e76b33ec5121abad4dc044050d52c3a4261bd
- pd2-r22-cta-after.png = ca6e33eba557f6c7f054873cad006b5cd0a9c73fd20f3f1a2bd469b8e68371de
- pd2-card-after.png = 8fdfea0fee88562d96e1e2e67522f6f1709cefbc7364931e2a7a7efe5e959136
- pd-r22-before.png = d2fcb4145e020fc51d963ac3a066ed386505be06b201d6a1fd433c7466ebeead
- 4 张互异（UNIQ_NEW4=4），与 QA 基线 17 张 r44-*.png sha 零重叠（OVERLAP_QA=0）。已亲读 before/cta-after/card-after 三图：前两图逐像素一致、第三图波次与手牌已变，与探针一致。

## 七、五批提交

- B1 72fa9499bbaf feat(r44) battle.vue 一行
- B2 1c5571f03283 test R22 两契约
- B3 3da9b17ee9c4 docs R44 SPEC + round22 SPEC + 终审报告
- B4 61ee976c4144 test(r44) audit .gitignore + 18 根文件 + 17 截图
- B5 postdeploy(r44) 报告 + 脚本 + 新图（见提交记录）
- 每批精确 git add（禁 -A/.），git diff --cached 泄露正则 0 命中、git diff --cached --check=0，B4/B5 排除 data/ 与 auth-init。

## 八、诚实边界与收尾

- TestNoRealSecretsInRepo 因仓库缺 .local.keys 输出 SKIP（原文「真值扫描这条守卫没有执行，不是『通过了』」），如实标注非 PASS；Go 测试进程 RC=0。
- 首轮 open 失败（--init-script 缺失致 about:blank）与首轮 eval 过早（clock=143 未到选卡阶段）两轮均重来，不作证据；最终证据仅取 CARD_SELECT_READY=1 后的有效闭环。
- finalize：删除 postdeploy/data/auth-init.js（令牌注入文件），QA 库 battle_records 保持 104|477，未做截断/恢复写操作；postdeploy data/ 与 admin-auth-init.js 命中 .gitignore（478/479 行）。
- 未修改业务源码（除已发布的 R44 一行）；未回退 R43/R42/R21 产物；远端 GitHub dependabot 告警非本任务范围。
