# R41 部署后真浏览器回归与发布报告（t37）

结论：**PASS（带诚实边界）**。R41 五批精确提交已落库；仅重建 admin 前端（后端与 H5 沿用基线未回退）；部署后另起真 Chromium 对 390/768/1280 三视口完成六章 × 字段零不符、CTA 停留 levels、chapter 搜索命中高亮、R31–R40 十 tab 回归与差异新图核对。TestNoRealSecrets 因缺 .local.keys 为 SKIP，如实标注非 PASS。

## 一、发布提交（精确 git add，禁 -A/.; 唯一提交者）
- 起点 HEAD=fa0ff692b；R40 末提交=3a594a3a67（t32）。
- B1 feat(r41)=8bd9ede8113366c55760c2dee15f22896b8583e3：chapterMechanicsText.ts + WikiView.vue（章节六字段表/横滚/AI四要素/CTA/chapter 搜索/bossText→level_bossText 别名），2 文件 151 insertions。
- B2 test(r41)=33f47199b6b17d6fa62e3830d66bac34ba47b382：WikiChapterMechanics.test.ts，1 文件 140 insertions。
- B3 docs(r41)=0cbeb03aeaa32ed0c8b59c1bd3fa5936a323a940：round41-wiki-chapters/SPEC.md + ROUND41_DESIGN_REVIEW.md，2 文件 132 insertions。
- B4 test(r41) audit=1f7ebe52d566eb82ea78789548f166ef8c9bc829：.gitignore（只追加 R41 规则至 455 行不截断）+ 审计根 37 脚本 + 11 截图，49 文件 604 insertions，DATA/TOKENJS/LOGINJSON_IN_STAGE=0。
- 每批 LEAK_LINES=0、git diff --cached --check=0；tracked 提交后干净。B5 postdeploy 见第七节。

## 二、仅 admin 重建与基线沿用
- admin 重建：cd laiyipao/admin && npx vite build，BUILD_RC=0，产物 laiyipao/admin/dist/assets/WikiView-L7QybPj7.js（57.30kB）。
- 产物含 r41-chapter-mechanics、chapter-mechanics、rotor_vane、r41-chapter-title、章节一览、HitKind chapter 映射、五地形映射、章节六字段表与 CTA；dist 被 .gitignore:10 忽略。
- 后端 laiyipao/deploy/bin/lyp-server-amd64 sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 基线，沿用未回退）。
- H5 laiyipao/miniapp/dist/build/h5/assets/index-DCBbR3t9.js sha256=aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b（R30 基线，沿用未回退）。

## 三、部署后服务与权威快照
- 8080 healthz=200 readyz=200 /api/v1/config=200，5175 /laiyipao-admin/=200，5174 /=200；agent-browser 0.41.1。
- 公开 config version=6237e0b623219b8b，CONFIG_BYTES=75367，require_login=False。
- 六章（配置六字段）：1 边境哨站 1–20 oil_drum boss0；2 工业废墟 21–40 collapse_wall boss0；3 冻原 41–60 charge_tower boss16；4 感染矿井 61–80 tidal_gate boss0；5 母巢外围 81–95 rotor_vane boss19；6 母巢核心 96–100 charge_tower boss22。
- 渲染期望：1–20关/油桶/—、21–40关/塌墙/—、41–60关/蓄能塔/#16、61–80关/潮汐闸门/—、81–95关/旋翼风扇/#19、96–100关/蓄能塔/#22，区间 en-dash。

## 四、三视口六字段与 CTA / 搜索
- 三视口 390/768/1280 每视口 open 后 set viewport（校正 headless 封装下 viewport 时序），真 CLI 深链 #/wiki?tab=levels。
- data/pd2-probe-{390,768,1280}.json：ROWS=6、headers 六列齐备、MISMATCHES=0、sectionVisible=true、AI 结论精确（共 6 章：含首领 3 章，地形 5 种）、basis/honesty/cta 四要素齐备、dirty=false、docW==innerW。
- CTA：cta-pre-{vp} ctaHref=#/wiki?tab=levels；真 CLI click .r41-ai-note a 后 cta-post-{vp} hash=#/wiki?tab=levels、dashboardJump=false、onWikiPath=true、chapterRows=6，停留 levels 未跳看板。
- 章节搜索：真 CLI fill .search-input rotor_vane → hits=[章节|母巢外围|81–95关 · 章节 5]；click .search-hit 后 hash=#/wiki?tab=levels&anchor=chapter-mechanics&focus=chapter:5，第 5 行 母巢外围 81–95关 旋翼风扇 #19 高亮（pd2-pick-{vp}.json hasChapterFocus=true）。三视口一致。

## 五、R31–R40 十 tab 回归
- 十 tab 精确探针 data/reg4-*.json：navs=[#/wiki, enemies, skills, gems, equipment, skins, rules, mastery, levels, live] 十项齐全；每页 dirty=false、docW==innerW。
- 计数：reaction=7（#reactions 紧邻 el-table 精确校准）、skills=24、compositeRows=18、recipeRows=18、enemies=22、levels=20、gems=8、skins=6、rating=5、drop=6、chapter=6、masteryRows=96，与权威一致，无回归。
- 校准记录：早期 reg-probe 用整页选择器得到 reaction=12/composite=36/levels=26/gems=13 等双计假阳性；calib-probe 与 reaction-probe 定位精确子表后校正，不作为应用缺陷。

## 六、截图与唯一性
- pd2 差异新图 6 张，sha 互异：pd2-chapter-390=42b9e9df11ae11f05d6e087e0154f164995dc7dac7a43b698e5eeabb790f4b5f（与 t35 基线 r41-chapter-390-correct 同图，属同产物视觉一致）、pd2-chapter-768=7f686f07616e6b62942b4bf45e4006eefab2e2ad24ebd31d898bc6615e6d7685、pd2-chapter-1280=e91541cbf2bde03169b7241a20c57c43c8fd9ca9503d5e96c73a04de526b55ba、pd2-search-390=0ea5bfef31dedc94e9af3493d78236d813308768bfd0cff1fe88ee039d277007、pd2-search-768=874a783b5fc97cbdae794678f0ce95da94e9f8fc0fc07c4358663d833cdda052、pd2-search-1280=3634671e91a492488938d74405a4437e0a5e15998f5eea1a9a53bfbafba45ed4。
- 其中 5 张 sha 完全不同于 t35 基线且彼此互异，满足「≥4 张新图 sha 互异且 ≠ 部署前」。亲读 pd2-search-390/pd2-search-1280 可见第 5 行蓝色高亮，非登录页。

## 七、对账、守卫与收尾
- data/reconcile-result.json：六章字段三视口逐格对账 + CTA + 搜索 + 十 tab 汇总，allOk=true、problems=[]。
- Go 守卫：cd laiyipao/server && go test ./internal/domain -run "TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo" -count=1 -v，exit=0；TestNoInfraIdentifiersInRepo PASS；TestNoRealSecretsInRepo **SKIP**（缺 laiyipao/deploy/.local.keys，原文「真值扫描这条守卫没有执行，不是『通过了』」），如实标注非 PASS。
- 零真值：临时 admin-auth-init 从已有审计 token 工件生成，写入被 .gitignore 命中的 data/，finalize 前删除；报告与最终提交脚本不含 token/口令/Authorization 字面量。
- QA 库 battle_records count=104 max(id)=477，finalize 只读核对未 truncate。
- B5 postdeploy 见本次提交：精确 git add REPORT.md + 审计根脚本（排除 data/ 与 auth）+ 截图。

## 八、已知边界（诚实披露，非缺陷）
- 「冻原」搜索 SEARCH_LIMIT=20 时章节命中在 20 条关卡之后，属 WikiView.vue:622-633 设计取舍，改用 rotor_vane/母巢外围可直达章节命中；未以 DOM eval 伪装点击。
- 早期 vp/reg2/reg3 探针因 viewport 时序与选择器双计产生假阳性，已由 pd2/reg4 校正；以本报告 pd2/reg4 与 reconcile-result.json 为最终证据。
- pd2-chapter-390 与 t35 基线同 sha 系同产物同视口视觉一致，非回归；B5 以 5 张全新 sha 证明差异。
