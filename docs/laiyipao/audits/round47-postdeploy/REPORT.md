# R47 部署后独立回归报告（round47-postdeploy）

## 一、结论

PASS。R47（R16 验真 CTA 从「发起验真 →」只读展示改为可点击定位到验真区，只定位、不提交、不改 hash）已完成唯一提交部署与部署后真浏览器独立回归。战报 #477 详情抽屉内点击 CTA 后，verify-row 滚动居中（种子行 top 117→378），hash 文本、输入框、验真结论、body 长度、路由 href 全部保持不变，符合契约「只定位不提交」。

## 二、发布范围与五批提交

- 起点 HEAD：073a2072686e42adf8658c59d4844ba153912519（R44 末，AB=0/0）；分支 opt1000/laiyipao-ai-refactor。
- B1 feat(r47)：b8c45fc9bf21 — laiyipao/admin/src/views/BattlesView.vue（scrollToVerify 定位函数 + CTA @click + CSS cursor:pointer）。
- B2 test：e4a4145e32cd — laiyipao/admin/tests/R16ReactionTimeline.render.test.ts（新增 describe 4 it）。
- B3 docs：d774a4a9d2c8 — R47 SPEC.md + docs/laiyipao/reviews/ROUND47_DESIGN_REVIEW.md。
- B4 test(r47) audit：3b3623faa699 — .gitignore（仅 R47 四行，排除 R48）+ R47 审计目录 10 根脚本/REPORT/3 截图，共 14 文件。
- B5 postdeploy(r47)：本报告 + postdeploy 脚本/探针/有效截图 + .gitignore 追加 R47-postdeploy 两行（见第八节）。
- 每批 LEAK=0、git diff --cached --check=0；B4_FORBIDDEN=0（未含 R48 任何文件）。R48 写域（miniapp 三 vue、r12GrowthPriority.ts 及契约）与 R43 已发布文件均未进入五批。

## 三、产物核对（仅重建 admin）

- admin：cd laiyipao/admin && npx vite build，BUILD_RC=0；产物 BattlesView-B7Ue5u0O.js，sha256=70e18ea2e6f73e7a3795134c5054a3c53e510cb76f716232e97307d68c0c6872。
- bundle 核对：含 function bt(){document.querySelector(".verify-row").scrollIntoView(...)} 与 onClick:bt、发起验真 →、verify-row、cursor:pointer；CTA 无提交绑定（保持只读语义）。
- 不回退：WikiView-ST1PbAt2.js=f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b（R46 沿用）；H5 index-BM_sn-oz.js=5b3914547c65a0d0d9e83ba331c51c839cfcb09951ca3ef938b15f77d86e155a；后端 lyp-server-amd64=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6。三者均未改动，H5 未重建。
- dist/ 由 .gitignore:10 忽略，产物不入库。

## 四、部署后真浏览器验证（1280×720 真 Chromium）

- 服务端点：API 8080 healthz/readyz/config=200；admin 由 laiyipao/admin 下 npx vite --host 127.0.0.1 --port 5191 --strictPort 提供（http-server dist 方案在该环境不可用），5191/laiyipao-admin/=200，5191/.../BattlesView-B7Ue5u0O.js=200。curl 需 --noproxy "*"。
- 鉴权：用 QA 引导环境注入 admin 令牌（脚本生成，令牌值不入库、不回显），session lyp-r47pd3 打开深链 #/admin/battles，set viewport 1280 720，mark 后真鼠标点击战报 #477 行详情。
- CTA 真点击：num.py 取整得 ctaRect 中心 (1043,386)，agent-browser mouse move/down/up left → CTA_MOVE=1043,386 ✓ Done。
- 定位证据（cta-before.json → cta-after.json）：verifyRows[0].top 117→378（种子 5621489295744462231 行居中，verifyRowInViewport=true）、ctaRect.y 378.03125→639.03125；CTA 由视口上方移动到验真区附近，证明 scrollIntoView 真正生效。
- 不变证据：hashTexts=［8ddc5cc01e787ec9, 5621489295744462231］前后一致、verifyInputs[0].value 为空串、resultText 为空串（无验真结论）、bodyLen=7634 不变、href 仍为 #/admin/battles、isLogin=false、drawerOpen=true、ctaText=发起验真 →、ctaCursor=pointer、ctaClickAttr=false（无提交绑定）。判定：只定位，不提交、不改 hash、不跳转。

## 五、截图证据与确定性同 sha 说明

- 有效截图 sha256：pd-list=f39de0b70afb8772d8764c7e71b6f3784050362863f85e2b197ec1c37e0b8dfa、pd-detail-open=95acdd9ac1c901117ca07f6c9a285bac6e92d720157fea9cb29af5936f726565、pd-cta-before=745ddcca5beb36b9fb2eb6b4f94fc88598e7b0ab7959d7d72a547fd2491a82bd、pd-cta-after=af42c501cf5b5fac33ba5f4aeaf1625ed5bc69088dcda75fba006a06e7ce68bd。
- 已亲读 pd-cta-after.png（1280×720）：验真区（种子/回放哈希/粘贴比对）居中显示、AI 反应时间线在下、输入框空、无验真结论。
- 诚实标注：本轮四张有效截图与 t62 QA 基线确定性同 sha（静态同产物、同视口、同数据 → 确定性像素），pd-cta-after 亦与 QA 的 r47-cta-after.png 同 sha。这是确定性构建的正常结果，不作为「新图互异」证据；点击生效以第四节 cta-before/after.json 的 verifyRows 位移为准，而非图 sha 差异。
- 1366×768 补拍失败（如实披露）：CTA cy≈831 超出 768 视口，mouse move 未命中，pd2-detail-1366 / pd2-cta-before-1366 / pd2-cta-after-1366 三张同 sha 14c07c929… 属无效证据，不计入有效图；pd2-list-1366.png=b1013109c01dbb51e661e5314a89c0b1d0e0d3b8c311e31b628daea0c5b451f8 仅作列表参考。

## 六、守卫与数据库核对

- go test ./internal/domain -run "TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo" -count=1 -v：exit=0；TestNoInfraIdentifiersInRepo PASS（4.20s）。
- QA 库只读核对：select count(*),max(id) from battle_records → 104|477（发布前后未变）。

## 七、诚实边界

- TestNoRealSecretsInRepo 因读不到 laiyipao/deploy/.local.keys 而 SKIP，原文「真值扫描这条守卫没有执行，不是『通过了』」——如实标注为未执行（非 PASS）。
- 本轮截图确定性同 sha，未产出与 QA 基线互异的新图；有效期内真点击一次成功（lyp-r47pd3），首轮 lyp-r47pd2 因浮点坐标导致 mouse move 报 Missing arguments 未命中，已修正取整后成功，过程如实披露。
- 1366×768 补拍未命中，三张抽屉图同 sha 作废。
- 令牌经本地 QA 引导环境注入脚本写入 data/admin-auth-init.js，属临时真值，finalize 已删除，不入库。

## 八、收尾

- finalize：删除 data/admin-auth-init.js，QA 库保持 104|477；.gitignore 追加 R47-postdeploy 的 data/ 与 admin-auth-init.js 两行（仅此两行，排除 R48），git check-ignore 命中。
- 五批经命令级代理推送（清代理 env 后 git -c http.proxy -c https.proxy push），提交信息无真值。
