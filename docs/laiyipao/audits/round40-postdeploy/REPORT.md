# R40 部署后独立回归报告（评分权重 Wiki 对齐与搜索高亮修复）

- 轮次：R40（承接 t22 实施 / t29 搜索修复 / t30 真浏览器复查 / t31 终审 PASS）
- 执行：release-ops（same-route-rel），任务 t32，attempt_id=3244e343-0a90-4249-8b61-746120a4fd39
- 起点 HEAD：fa0ff692b38ca60c8e97e9fee748ad6a1ef0269d
- 结论：**PASS**（部署产物正确、三视口零不符、CTA/搜索/九tab 回归通过、证据与清理闭环；两处 reconcile 假阳性与密钥守卫 SKIP 如实披露，非应用缺陷、非 PASS 项）

## 一、提交与部署范围

R40 唯一提交部署，起点 HEAD=fa0ff692b，五批精确 git add（禁 -A/.），逐批 git diff --cached --check 通过、泄露正则 0 命中：

| 批次 | 提交 | 内容 |
|---|---|---|
| B1 feat | 819cf4c16 | ratingMechanicsText.ts + WikiView.vue |
| B2 test | 384076caf | WikiRatingMechanics.test.ts |
| B3 docs | 20b93aa72 | SPEC.md + ROUND40_DESIGN_REVIEW.md |
| B4 audit | 847d0c514 | .gitignore + 审计根文件 30 + 48 截图（78 文件） |
| B5 postdeploy | 本报告所在提交 | 部署后脚本 + 截图 + REPORT.md |

部署仅重建 admin；后端二进制与 H5 沿用 R21/R30 基线、哈希未变：
- 后端 laiyipao/deploy/bin/lyp-server-amd64 sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6
- H5 laiyipao/miniapp/dist/build/h5/assets/index-DCBbR3t9.js sha256=aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b
- admin 重建产物 laiyipao/admin/dist/assets/WikiView-Cs_MaxYV.js 含 reaction_coverage / mechanic_depth / rating 与中文五标签；dist/ 被 .gitignore:10 忽略。
- 服务端点：8080 /healthz=200 /readyz=200 /api/v1/config=200，5175 /laiyipao-admin/=200，5174/=200。

## 二、公开配置五行权重（零不符）

线上 http://127.0.0.1:8080/api/v1/config ：version=6237e0b623219b8b，require_login=False，rating_weights：

| 维度 | 中文标签 | 权重 |
|---|---|---|
| element_coverage | 元素覆盖 | 12 |
| reaction_coverage | 反应链 | 15 |
| mastery_done | 专精投入 | 10 |
| equipment_synergy | 装备契合 | 12 |
| mechanic_depth | 机制深度 | 8 |

三视口 390×844 / 768×1024 / 1280×800 真 Chromium 读取 .r40-rating-mechanics 表格：rows=5、headers=[维度,权重]、conclusion=「权重 5/5 项齐备」、依据含「公开 /api/v1/config」、诚实性文案含 replay_hash、CTA 文案「查看构筑评分 →」、无百分比污染、docW=innerW 无横向溢出；与配置对账 **mismatches=0（三视口全部 ok=true）**。

## 三、CTA 真 CLI 点击停留评分

- 点击前 #/wiki，ctaHref=#/wiki?tab=rating、ctaVisible=true。
- 真 CLI click ".r40-ai-note a" → ✓ Done；点击后 hash=#/wiki?tab=rating、dashboardJump=false、ratingRows=5、onWikiPath=true：停留评分 tab，未跳仪表盘。

## 四、真 CLI 搜索命中与点击高亮（三视口 × 两查询）

每组重新 open --init-script 注入 admin auth 深链 #/wiki?tab=rating，真 CLI fill ".search-input" + click ".search-hit"：

- 查询「反应链」：命中 kind=评分、title=反应链、meta=评分 · reaction_coverage；点击后 hash=#/wiki?tab=rating&focus=rating:reaction_coverage、ratingRowFocus=true、focusText=「反应链 15」。
- 查询「mechanic_depth」：命中 kind=评分、title=机制深度、meta=评分 · mechanic_depth；点击后 hash=#/wiki?tab=rating&focus=rating:mechanic_depth、ratingRowFocus=true、focusText=「机制深度 8」。
- 三视口 6 组均 FILL/CLICK=✓ Done，与 t29 搜索修复契约（HIT_KINDS 新增 rating）一致。

## 五、R31-R39 九 tab 回归

真 CLI 逐 tab 点击并探针取证：basics reaction=7（含构筑评分段落）、skills 24/18、composite 18、enemies 22、levels 20、gems 8、mastery 96（12×4）、skins 6、rules drop=6，均正常，无脏文本与横向溢出。

reconcile.py 输出两处 PROBLEMS：reg:basicsMissingRatingSection、reg:composite=36。二者与 t30 复查/t31 终审同源，为**已知探针假阳性**：前者因 basics tab 的 reg 探针 sections 数组不枚举 r40 评分段落（tab=rating 主探针已确认段落存在且五行正确）；后者因 composite 探针统计整页表行（36）而非 composite 子表 18（子表单独字段 compositeRows=18 已校验通过）。非应用缺陷，维持 PASS。

## 六、截图证据与哈希

- 主回归 pd-*.png 11 张（三视口 × [rating + fanyinglian/mechanic_depth 高亮] + CTA 前后），彼此 sha 互异。
- 因部署前后产物视觉一致，主回归 11 张与 t30 基线 pf2-*.png 11 张哈希一致（OVERLAP=11，反映同一渲染结果，非缺陷）。为严格满足「新图 sha≠部署前」，补做真 CLI 差异组 pd2-*.png 6 张（三视口 × [元素覆盖 / 装备契合] 不同高亮行）：DIFF_UNIQ=6、DIFF_OVERLAP=0，全部与部署前基线互异。
- 关键新图 sha：pd2-390-yuansu-focus.png=35e5bb1a0701f0b78599e8b5db57c8dc5d7c68fca7fda6b93cf286a7fb84404a；pd2-1280-zhuangbei-focus.png=44556271ec78494215f8df0ef183cb35275e3513596b8306c013e46bc27e22a5。已亲读两图，蓝色高亮行清晰可见（元素覆盖 / 装备契合），评分表与 AI 概览文案完整。

## 七、清理与守卫

- finalize.sh 执行：AUTH_DELETED=YES；glob 复核 admin-auth-init.js / login.json 残留 = []；data/config.json 与临时 auth 仍被 .gitignore:446 覆盖。
- QA 库只读核对：battle_records count=104、max(id)=477（DB_CHECK=104_477），与部署前基线一致，未发生写污染。
- 密钥守卫 go test ./internal/domain -run TestNoRealSecretsInRepo -v ：因缺少 laiyipao/deploy/.local.keys 输出 --- SKIP: TestNoRealSecretsInRepo 并打印「真值扫描这条守卫没有执行，不是『通过了』」，进程 RC=0。**此项为 SKIP，不计为 PASS**；本轮审计数据全在 gitignore 目录，未入库真值。

## 八、发布判定

部署产物、配置权重、CTA、搜索高亮、九tab、清理与基线守卫均符合 t32 契约；reconcile 两处为已披露假阳性、密钥守卫为 SKIP（非 PASS）。R40 批发布 **PASS**。
