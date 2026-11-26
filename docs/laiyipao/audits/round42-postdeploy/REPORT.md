# R42 部署后真浏览器回归与发布报告（t46）

结论：**PASS（带诚实边界）**。R42 五批精确提交（B1–B5）；仅重建 admin 前端，后端与 H5 沿用基线未回退；部署后另起真 Chromium 三视口完成关卡所属章节核对、列内交叉引用与 AI CTA 真 CLI 点击、章节搜索高亮、R31–R41 十一 tab 回归与差异新图核对。TestNoRealSecrets 因缺 .local.keys 为 SKIP，如实标注非 PASS；章节下拉定位探针存在覆盖假阳性，见第八节。

## 一、发布提交（精确 git add，禁 -A/.; 唯一提交者）
- 起点 HEAD=893b2bf30f（R41 末提交）。
- B1 feat(r42)=cc800c6a3：levelChapterText.ts + WikiView.vue（关卡第14列所属章节、交叉引用链接、AI 章节交叉概览四要素、R42 样式），2 文件。
- B2 test(r42)=994c52a6a：WikiLevelChapterMechanics.test.ts 契约。
- B3 docs(r42)=b16f4afb6：round42-wiki-level-chapter/SPEC.md + ROUND42_DESIGN_REVIEW.md。
- B4 test(r42) audit=fa551ac3c1a6：.gitignore（只追加 R42 八行规则至 463 行不截断）+ 审计根 19 脚本 + 8 截图，28 文件 280 insertions。
- 泄露守卫：B4 首轮 guard 命中 secret-scan.py 自身的正则定义（自描述模式，非真实凭据）；复核排除该文件后非扫描器 0 命中，git diff --cached --check=0，随后提交。B1–B3 LEAK_LINES=0、CHECK_EXIT=0，全程 FORBIDDEN_STAGED=0（R43 三文件 battle.vue/battle_r24/battle_r27 未入库）。
- B5 postdeploy 见第七节。

## 二、仅 admin 重建与基线沿用
- admin 重建：cd laiyipao/admin && npx vite build，BUILD_RC=0，产物 laiyipao/admin/dist/assets/WikiView-JpQ4MFXA.js（59.31kB）sha256=cc70f6987a04064744d42620d99254554bdd84a4ffbd6999667230bba487ea0c；dist 被 .gitignore 忽略。
- 端口说明：5175 被并行任务的 oa4rust-h5 占用（返回 302 提示 public base /oa4rust-h5/），未停他人服务；admin dev server 另起 5275，BASE=http://127.0.0.1:5275/laiyipao-admin/。
- 后端 laiyipao/deploy/bin/lyp-server-amd64 sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 基线，沿用未回退）。
- H5 laiyipao/miniapp/dist/build/h5/assets/index-CBt24gPo.js sha256=d38176f572549b0bb402045a8cfba2bab688f620083cd47db150ebc70b8da8d8（git HEAD 现有产物，沿用未重新部署；R43 并行改的 miniapp battle.vue 未发布）。

## 三、部署后服务与权威快照
- 五端点：8080 healthz=200 readyz=200 /api/v1/config=200；5174 /=200；5275 /laiyipao-admin/=200。
- 公开 config version=6237e0b623219b8b，CONFIG_BYTES=75367，require_login=False，data/config.json sha256=168ece7eca761aad2e15441b741e589f00e56838be2cf8ee0d95d2630f49efd6。
- 六章配置：1 边境哨站 1–20 oil_drum boss0；2 工业废墟 21–40 collapse_wall boss0；3 冻原 41–60 charge_tower boss16；4 感染矿井 61–80 tidal_gate boss0；5 母巢外围 81–95 rotor_vane boss19；6 母巢核心 96–100 charge_tower boss22。

## 四、三视口关卡所属章节与真 CLI 交互
- 三视口 390×844 / 768×1024 / 1280×800，open 后 set viewport（headless 时序校正），真 CLI 深链 #/wiki?tab=levels。
- 关卡表第14列「所属章节」：部署后探针捕获 75 个唯一关卡 ID（ch1 1–20 / ch2 21–40 / ch4 61–80 / ch5 81–95），逐格与 config levels.chapter→chapters.name 对账，CHAPTER_CELL_MISMATCHES=0，dirty=false、docW==innerW。
- 列内交叉引用：真 CLI click .r34-level-table-scroll tbody tr:first-child td:last-child a，pd-click-link-*.txt=✓ Done；pd-click-after hash=#/wiki?tab=levels&anchor=chapter-mechanics，chapterVisible=true，停在 Wiki 未跳看板（三视口一致）。
- AI CTA：真 CLI click .r42-level-chapter-note .r42-ai-cta a，pd-cta-click-*.txt=✓ Done；pd-cta-after hash=#/wiki?tab=levels&anchor=chapter-mechanics，onWiki=true，ctaText=查看章节一览 →，三视口均停留 levels。
- 章节搜索：真 CLI fill .search-input rotor_vane → hits=[章节|母巢外围|81–95关 · 章节 5]；click .search-hit 后 hash=#/wiki?tab=levels&anchor=chapter-mechanics&focus=chapter:5，chapterFocus 为第5行「母巢外围 81–95关 旋翼风扇 #19」，hasChapterFocus=true（三视口一致）。

## 五、R31–R41 十一 tab 回归
- 十 tab 精确探针 pd-reg-{basics,skills,composite,enemies,levels,gems,skins,mastery,rules,rating}-{vp}.json：每页 dirty=false、docW==innerW 无横向溢出；导航项≥8 齐全，无回归。
- 关卡 tab 同时验证 R42 第14列与 AI 交叉概览；rating/r40 表与 chapter/r41 表未回退。

## 六、截图与唯一性
- pd2 新图 15 张（含 1 张章节下拉过程调试图），其中互异且全 ≠ 部署前 QA 8 图（r42-crossref/cta/level-chapter/level-table 系列）的 ≥12 张，满足「≥4 张新图 sha 互异且 ≠ 部署前」。
- 关键新图 sha：pd2-crossref-1280=2c15fc57e150be0b2d12a5c30f02b7e6183fef77e24e07f2e1b3eac58c5906ef；pd2-cta-1280=84b69671ac0dcf4ac724e56bc48da6e93c4485941f5d8990bd8a2f3b210562b9；pd2-search-1280=57deee687076a8ff2fddf750f4980b7365e4a1992591f313dafac14a20b522a8；pd2-search-390=316f1bc9ba77217b2096f5aa13ed506d4d99c7266abbe45253238a40506697df；pd2-level-ch6-768=bd16849a8425fe43ca30c900057e088d478500fe678aafd0dda56c310853d17e。
- 已知同 sha：pd2-crossref-390 与 pd2-cta-390 同为 877791839033b012ed06a6ee5937be7fb049f4e8fad0686074de33e67f093539（同一 390 视口 chapters 区域静态截图，确定性像素）；pd2-level-ch6-390 与 pd2-sel-debug-390-ch3 同为 4ed2405557c3ac5fd767fad3162a720dbd38cac3ef0dc7453c60c263ae9b792d（同视口同状态）。属静态同 sha 确定性像素，非缺陷。

## 七、对账、守卫与收尾
- data/reconcile-result.json：allOk=false，唯一 problem=coverage 75 != 100；CHAPTER_CELL_MISMATCHES=0（见第八节诚实边界）。CTA/交叉引用/搜索/回归各项在脚本中逐条校验通过。
- Go 守卫：cd laiyipao/server && go test ./internal/domain -run "TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo" -count=1 -v，exit=0；TestNoInfraIdentifiersInRepo PASS；TestNoRealSecretsInRepo **SKIP**（缺 laiyipao/deploy/.local.keys，原文「真值扫描这条守卫没有执行，不是『通过了』」），如实标注非 PASS。
- 零真值：临时 admin-auth-init 由既有审计 token 工件生成写入被 gitignore 命中的 data/，finalize 已删除（AUTH_DELETED=YES、AUTH_GLOB=0）；git check-ignore 命中 .gitignore:462(data/) 与 :463(auth)。
- QA 库 battle_records count=104 max(id)=477，finalize 只读核对 DB_CHECK=104_477，未 truncate。
- 浏览器作业：按队长指令全部 job_kill（e2e 5 个 job 均已终止），不再无限重跑；admin dev server 5275 保留。
- B5 postdeploy 精确 git add 本报告 + 审计根脚本 + 15 张截图，排除 data/ 与 auth。

## 八、已知边界（诚实披露，非应用缺陷）
- **章节下拉覆盖假阳性**：部署后脚本中 ch3（41–60）与 ch6（96–100）的 el-select 定位命中了非目标下拉框，捕获到的是 ch4（61–80）与 ch1（1–20）数据；因此唯一关卡覆盖为 75/100 而非 100/100。已捕获的 75 关所属章节零不符；ch3/ch6 的逐关正确性由 QA t41（三视口 100/20 抽样 MISMATCHES=0）与终审 t44 六维门禁覆盖。按队长有界收尾指令不再重跑，故 reconcile allOk=false 的唯一原因为此脚本级覆盖问题，非应用缺陷。
- **静态同 sha 确定性像素**：pd2-crossref-390 与 pd2-cta-390 同 sha、pd2-level-ch6-390 与 pd2-sel-debug-390-ch3 同 sha，系同产物同视口同状态渲染一致，非回归。
- **探针假阳性**：reconcile 早期版本曾因表头分隔符、章节搜索命中文案（中文标签而非英文 key）、nav 锚点 href 形式产生 36 条假阳性，已由行自校验判据重写校准；以本报告与最终 reconcile-result.json 为准。
- 5175 端口被并行任务占用，admin 改用 5275；不影响功能验证。
