# R43 部署后独立回归报告（postdeploy）

## 一、任务与范围
R43「战斗 HUD CTA 文案与行为错配修复」发布后独立回归。唯一发布者 release-ops，worker same-route-rel。
起点 HEAD=51f0243d918d（R43 审计末批），分支 opt1000/laiyipao-ai-refactor，author=annsshadow/annsshadow@users.noreply.github.com。
本轮必须重建部署 H5；后端与 admin 沿用基线不回退。严格写域，排除并行 R44 写域与历史噪声。

## 二、五批提交（精确 git add，禁 -A/.）
- B1 feat(r43)：989e4fdc4a9a824a4a819f9eb013dc637a19986b —— battle.vue 四处（删 r27/r24 模板 @click=openHelp、删 CSS pointer-events:auto）。
- B2 test：23d189b75bac71854759c41f34c600b14014701a —— 两契约 battle_r24_element_hud.test.ts / battle_r27_wave_progress.contract.test.ts。
- B3 docs：5b52ddd7ee4e9e8e4de5677c59482c1639f2036e —— SPEC.md + ROUND43_DESIGN_REVIEW.md。
- B4 test(r43) audit：51f0243d918dfe5616b0ed75814c0dca100c024e —— .gitignore + 10 根脚本 + 审计 REPORT + 15 截图（27 文件 290+）。
- B5 postdeploy(r43)：本报告 + postdeploy-e2e.sh + postdeploy-help.sh + 5 张 pd-* 新截图（详见第九节提交清单）。
每批 LEAK=0、git diff --cached --check=0。B4 首轮 forbidden 规则误伤合法 gen-auth-init 脚本与登录初始截图，收窄为精确真值路径后重跑 FORBIDDEN=0；gen-auth-init 从 stdin 运行取令牌、无硬编码真值。

## 三、H5 重建与产物核验
cd laiyipao/miniapp && pnpm build:h5 成功（BUILD_RC=0，Compiler 5.24 vue3，仅 Appid 提示）。
- 新入口 index-BQ39M86b.js sha256=a90137484fcd6467946174cc3d2eae88c5c13e8d683425bc1271f61c4bd032b4（index.html 已引用新入口）。
- 新战斗分块 pages-battle-battle.BM4KKStv.js sha256=1c5f38986e9f1129c1c2591b74acd586cbfb86661fc677233cd3233afc9b83f4。
- 旧基线 index-CBt24gPo.js sha256=d38176f572549b0bb402045a8cfba2bab688f620083cd47db150ebc70b8da8d8（R42 部署产物，已被替换）。
- grep 新 bundle：r27-ai-cta / r24-ai-cta 各 1 处（文案保留为纯展示），r27-ai-cta 邻近 openHelp 与 r24-ai-cta 邻近 openHelp 均 0 命中 —— 旧 CTA @click 绑定已消失。
- 后端 lyp-server-amd64 sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（沿用，未回退）。
- admin WikiView-JpQ4MFXA.js sha256=cc70f6987a04064744d42620d99254554bdd84a4ffbd6999667230bba487ea0c（沿用，未回退、未重建）。

## 四、服务与端点
8080 API 用 round15-reaction-timeline/start-api.sh 后台启动；5174 被并行 oa4rust-h5 占用（未停他人服务），另起 5274 用 http-server 静态部署新 H5。
E2E 采集时五端点均 200：8080/healthz、8080/readyz、8080/api/v1/config、5274/、5274 battle 深链。
收尾有界 kill 后台 job 后端口回 000（服务随 job 生命周期退出，属预期，非部署缺陷），证据已在 E2E 时固定。

## 五、真浏览器 E2E（390×844 真战斗态）
认证链：游戏页需 user kind 的 lyp_token，注入 admin token 会跳登录页；新 guest 默认 max_stage=0 无法进 L5。
通过 QA 库已有解锁 L5 的 guest（max_stage=60）走 guest_token_aliases 别名换取 user token，并在全新 session 的 open 阶段注入 lyp_token + 可解析的未来 lyp_token_exp + lyp_profile，避免 session 残留 lyp_guest/lyp_refresh 自动登录覆盖。
结果：/me 返回 user_id=10、max_stage=60；深链 battle?level=5 进入真实战斗态（防线 1100/1100、波次 1/5）。
- r27 CTA 真鼠标坐标点击（196,168）：倒计时 138→137，helpOverlay=false、hasTimeline=false、hasPause=0、pauseLabels=[]，不打开图例、不暂停。
- r24 CTA 真鼠标坐标点击（196,667）：倒计时 136→135，同样 helpOverlay=false、hasPause=0、pauseLabels=[]。
- 共存探针 pd-coexist.json：r27-ai-cta / r28-feed-cta / r24-ai-cta / help-chip 四类 CTA 同页共存，hasWave=true，正对照入口仍在。

## 六、诚实边界与限制
- 帮助正对照 .help-chip 在本轮 postdeploy 多次点击未命中（选择器点落在视口外、session 未重新 open、帮助层中心随布局漂移导致坐标时序错位），postdeploy 的 help-open/help-closed 图不作为正对照证据。
- QA t45 已用真坐标覆盖正对照：点击 .help-chip 后 helpOverlay=false→true、倒计时冻结在 138s，.legend-close 恢复 137s，截图 r43-hc-help-open.png 可见「战场图例」弹层；本轮不无限重跑，按有界收尾处理。
- hasResume=0 / pauseLabels=[] 在帮助层打开时仍为假阴性（探针按精确文案匹配），可靠暂停判据是 helpOverlay + 计时冻结/恢复 + 截图。
- R28/R29 时间线入口真实点击本轮未单跑，按源码只读核对（r28-feed-cta 仍保留 @click.stop 与 pointer-events:auto）。
- Go 守卫：TestNoInfraIdentifiersInRepo PASS；TestNoRealSecretsInRepo SKIP —— 缺 laiyipao/deploy/.local.keys，守卫原话「真值扫描这条守卫没有执行，不是通过了」，如实标注为 SKIP 非 PASS。

## 七、新截图 sha256（互异且全 ≠ QA 基线）
- pd-r27-after.png = 929dc7090cd3aa65ed53d04f7f4ce1a1957dde2cfcf8fa576a3829e96c0a0e77
- pd-r24-after.png = c837049ee55aea431663285656c33bae605171fbd9494d74f335b3f6e6786095
- pd-help-open.png  = cc93318b71b6e2b9ff9aec20852f51906a2d226eb110f10ed960f5d357793db3
- pd-help-closed.png= 3d6f677e0b78c99307f731ea2b7cb688894ab2d5bd171c376c9e79e9752607d3
- pd-coexist.png    = 7bb0cd43975667c88cde2a4cfa5e8c4a7e818f81aa2442d656c4a4abeedcc121
5 张互异且均不在 QA t45 基线 14 种 sha 之列，满足「≥4 新图互异且 ≠ 部署前」要求。

## 八、收尾
- finalize：删除 postdeploy 下 admin-auth-init.js 与 data/auth-init.js、login.json；AUTH_LEFT=NONO；QA 库 battle_records=104|477。
- git check-ignore 命中 .gitignore:470（postdeploy/data/）与 :471（postdeploy/admin-auth-init.js）。
- 工作树并行 R44 改动（.gitignore、round22 SPEC、battle.vue、r22 两契约）属他人写域，未纳入本任务任何提交。
- 所有浏览器后台 job 已 job_kill；代理推送与 update_task 见任务回执。

## 九、判定
PASS（带诚实边界）。核心目标——H5 重建部署新 bundle、r27/r24 CTA 纯展示化、真战斗态坐标点击无暂停副作用、前序 HUD 共存、新图要求——均满足；帮助正对照由 QA t45 覆盖，SKIP/假阳性/探针限制已如实披露。