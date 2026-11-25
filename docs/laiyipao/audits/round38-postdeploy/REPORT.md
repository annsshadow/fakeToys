# R38 部署后独立回归报告（round38-postdeploy）

- 任务：t16 R38 发布（合成配方 Wiki 四列 + hash 安全 CTA），发布负责人 release-ops-wb
- 基线：HEAD `abaac8aa2c1aff3ece6bdb5bc7d777cdf3b2087c`；部署前 B4 `3d390e88c47621420744777216e7222e1c89456d`
- 总判定：**PASS**（所有硬门禁满足；SKIP/假阳性/显示层问题逐项如实标注）

## 一、发布批次与提交纪律

| 批次 | 提交 | 说明 |
|---|---|---|
| B1 | `7c4e88e9ea4d1a38c9a7ed28da3356191c59dbfa` | feat(r38)：`recipeMechanicsText.ts` + `WikiView.vue` |
| B2 | `37873d8ed68cc2a173abd9aede47c76c8d66bfb0` | test(r38)：`WikiRecipeMechanics.test.ts` 契约守卫 |
| B3 | `bde3719c87bfe13aebcdf8bc7013f102647d7cf9` | docs(r38)：`SPEC.md` + `ROUND38_DESIGN_REVIEW.md` |
| B4 | `3d390e88c47621420744777216e7222e1c89456d` | test(r38)：QA 审计脚本 + REPORT + 21 图（排除 data/ 与 auth-init） |
| B5 | 本报告 + 脚本 + 截图（提交哈希由 `git rev-parse HEAD` 读取，报告内不自引用） | postdeploy(r38)：部署后独立回归证据 |

- 每批精确 `git add` 显式路径，禁 `-A`/`.`；`LEAK_LINES=0`、`git diff --cached --check` RC=0。
- B4/B5 `DATA_IN_STAGE=0`、`TOKEN_IN_STAGE=0`、`LOGIN_IN_STAGE=0`（除本报告报告区明确排除清单外，真值均未入库）。
- `.gitignore` 行数 431（相对 abaac8aa2 仅追加 R38 live/postdeploy data/ 与 admin-auth-init.js 规则 425–431，历史规则未截断）；`zero-secret-scan.sh`：`BAD_COUNT=0`、`SYNTAX_RC=0`、`AUTH_BEFORE=YES`，`check-ignore` 命中 .gitignore:430。
- 推送命令：`git -c http.proxy=http://127.0.0.1:12450 -c https.proxy=http://127.0.0.1:12450 push origin HEAD`；终态 HEAD/REMOTE 一致、AB=0/0、tracked clean（推送结果以最终 git 状态为准，见第九节）。

## 二、部署完整性

- 部署前 QA：`CFG_SHA=9f433571d016ce8eb6e450ce7bcfc8a2ce994769beac462f12bb5ae751439e94`，QA REPORT sha256 `64fdf7020eb9eb165bccb0c49d2f455bcfcbf7d0637578928f98bdff7d3b8d1d`（61 行，总判定 PASS）。
- 部署后（服务重启后 config 重新生成，同源 version）：`CFG_SHA=ea4b8d96e5be420b597840f24c5cb79e56ee26e8f2911a751a6601840216a298`、`VERSION=6237e0b623219b8b`；`recipes=18`、二阶 11 / 三阶 7。
- 后端产物 sha256 `3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6`（R21 沿用，未回退）；H5 产物 `index-DCBbR3t9.js` sha256 `aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b`（R30 沿用，未回退）。
- 仅重建 admin：bundle 含 R38 标识 `WikiView-Bdd0TH_A.js` / `WikiView-CiYRqlHc.css`（2 文件），Vite served markers=3；healthz/readyz/config/H5/admin 200。
- 临时 JWT：`mint-token.sh` 通过 POST `/api/v1/admin/login` 获取，`TOKEN_LEN=229`、`MINTED=YES`；finalize 后删除。

## 三、三视口逐字段对账（公开 config，无 shim 真 Chromium）

证据：`data/reconcile-summary.txt`（GBK 控制台编码，乱码仅显示层）、截图 `screenshots/r38-390-recipes.png`、`r38-768-recipes.png`、`r38-1280-recipes.png`。

| 视口 | ROWS | MISMATCHES | AI_OK | FOUR_OK | BAD | DOC/innerW | SCROLL | R36 | VERDICT |
|---|---|---|---|---|---|---|---|---|---|
| 390×844 | 18 | 0 | True | True | False | 390/390 | True | True | PASS |
| 768×1024 | 18 | 0 | True | True | False | 768/768 | True | True | PASS |
| 1280×800 | 18 | 0 | True | True | False | 1280/1280 | True | True | PASS |

- `ALL_RECONCILE_OK=True`；CTA 文案 `查看合成配方 →`；AI 四要素同屏 `共 18 条配方：二阶 11 条、三阶 7 条`，三视口均 AI_OK/FOUR_OK=True。
- 抽样行 ROW0/ROW2/ROW17 四列（名称/类别/材料/阶数）与公开 config 一致；中文抽样在 GBK 控制台乱码，JSON/脚本层为零不符。

## 四、hash 安全 CTA（真实 CLI 点击）

证据：`data/cta-pre.json`、`data/cta-post.json`、`data/cta-click.txt`、`screenshots/r38-04-cta-before.png`、`r38-05-cta-after.png`。

- 点击前：`hashBefore=#/wiki?tab=skills`、`sectionVisible=true`、`rowCount=18`、`ctaHref=#/wiki?tab=skills&anchor=composite`。
- CLI 命令 `agent-browser click '.r38-ai-cta a'`：`✓ Done`、RC=0（无界 `timeout.exe` 包装）。
- 点击后：`hashAfter=#/wiki?tab=skills&anchor=composite`、`recipeStillThere=true`、`rowCount=18`、`dashboard=false`、`activeNav=['技能图鉴']`。
- `r38-05-cta-after.png` 与部署前 QA 同名图字节一致（确定性像素、非登录页；见第七节诚实标注）。

## 五、表内横滚、页面无溢出与末列可见

证据：`data/probe-390.json`/`probe-768.json`/`probe-1280.json`、`data/scroll-390.json`、`screenshots/r38-07-scroll-390.png`。

| 视口 | wrap scrollWidth/clientWidth | before→after | docW/innerW | 末列 |
|---|---|---|---|---|
| 390 | 510/324 | 0→186（`el-scrollbar__wrap` 与 `el-table__header-wrapper` 同步） | 390/390 | 真实横滚后可见 |
| 768 | 702/702 | 0→0 | 768/768 | 直接可见 |
| 1280 | 884/884 | 0→0 | 1280/1280 | 直接可见（免滚，非失败） |

- 页面无横向溢出：三视口 `docW=innerW`；`scroll-detail.js` `moved=[{i:3,el-scrollbar__wrap,0→186},{i:4,el-table__header-wrapper,0→186}]`。

## 六、搜索命中配方与 R31–R37 七 tab 回归

- 搜索：`data/search2-jiaotu.json`/`search2-guangleng.json` 两 hit 分别为「焦土重锤」「光棱攻城」，`panelVisible=true`；CLI 输入脚本 `search-recipe.sh` 通过。DOM 明细 `data/detail-search-jiaotu.json` 的 `span.hit-kind` 第二 hit 为「配方」（`hit-meta=合成配方`），搜索面板 `hits[].kind` 探针粒度缺陷（两 hit 均显示「合成」）如实标注为探针粒度限制，不作项目缺陷，不作 PASS 依据。GBK 乱码仅显示层。
- 七 tab CLI 回归：`data/reg3-*.json` 与 `reg3-click-*.txt` 全 ✓ Done；`reg3-basics`：reactionCount=7、corrosionSpread=true、armorShred=true、armorKnock=true、basicsTableCounts=[7,5]、compositeCount=36、recipeCount=18、bad=false、docW=innerW=1258。
- `reg3-composite`：`hash=#/wiki?tab=skills&anchor=composite`、activeNav=['技能图鉴']、bad=false；`enemies/levels/gems/mastery/skills`：hash 分别 `#/wiki?tab=enemies/levels/gems/mastery/skills`、activeNav 对应正确、bad=false；R31–R37 既有 tab 无回退。
- 截图 `r38-reg3-basics.png`/`composite.png`/`enemies.png`/`levels.png`/`gems.png`/`mastery.png`/`skills.png` 七张各异。

## 七、截图哈希与诚实标注

证据目录：`docs/laiyipao/audits/round38-postdeploy/screenshots/`（18 张）。

- 部署前 QA 六图：`r38-05-cta-after=b169b10569522b910eccc69d5ddcb7e3e7866481214ddc099cf32120da897645`、`r38-focus-390=47ef90a5635a65c0fa2d5f487845348ab03328a6276d8273061f58ec765c7e58`、`r38-focus-768=6a5ae7aee8475de631e191af1c68ad5fe38d3175faed212b00f1e75dbeef4e42`、`r38-focus-1280=9fbc1480b797c086d3c2ce04d1b848d7dcdcbcae72fb09406a18a62df744645d`、`r38-search2-guangleng=a4de27177fbd875d6fedf7a79ece33127651dc97166302a4c08d303c1f7fa7a3`、`r38-search2-jiaotu=3c3d092e543cccccf0b4a79c04c7868d3cfd4ad6157e848171aab1b4d609131c`；部署后同名确定性像素字节一致，如实标注 IN_QA，不计入「新图」。
- 新图互异计数：`QUALIFY_NEW_DISTINCT=10`，满足 ≥4 张新图两两不同且全 ≠ 部署前 QA 六图。新图 sha（节选）：`r38-1280-recipes=3936741251a80ecf6ec744defc5c407766b82dea5a04a2ca250436d8be4d6a5e`、`r38-768-recipes=f358edd42e81b0d5e0194a21debcf79883b6a343c92291031c9424599e4fad78`、`r38-reg3-basics=0db7d33d45814700f87eb84020a3d6b4ea1155262aa6fc3a3e2416583f2c2aa2`、`r38-reg3-composite=a262a6edcc084632cd73aa4f272f96321ad1b6b7002fd1b5a641ae6fa5dcb35f`、`r38-reg3-enemies=e901c522564d3168d920ac3d935add7d4bde8be363db9fa1479f84182a5b11c7`、`r38-reg3-gems=2500550050f930e3f8254a5cc7548d779cf766e98493cd5c2d2b3242a24bef60`、`r38-reg3-levels=ae4d9da7818e681cb5b520a41f6efb9c71b343f6755df02f7982da928c1a3763`、`r38-reg3-mastery=1b953ea418a4957c22c0ced91122a2c4adff895fa6b115baa732443eac9ec456`、`r38-reg3-skills=889f28fc9041f987d49e95445883d2b99a1174a364438bab79d923fbd821e7dd`。
- 静态初始态同字节：`r38-04-cta-before`、`r38-07-scroll-390`、`r38-390-recipes` 均 `0d79cc7ca0ed1d76b06575df9f0ad88c98f3ba6226da00167b9f35e210f41253`（确定性截图，非伪造、非遗漏，如实标注；不计入互异新图）。
- 亲读图片：`r38-1280-recipes.png`（1280×800，非登录页，AI 横幅/技能表完整）、`r38-07-scroll-390.png`（390×844，非登录页，移动端表格）、`r38-05-cta-after.png`（390×844，非登录页，合成技能表）。

## 八、测试与安全守卫（如实标注）

- `go test ./internal/domain -run TestNoRealSecretsInRepo -v`：`--- SKIP`（读不到 `.local.keys`，真值扫描守卫**没有执行**），**非 PASS**，不得冒充。
- `zero-secret-scan.sh`：`BAD_COUNT=0`、`SYNTAX_RC=0`；`git diff --cached` 真值自检 `LEAK_LINES=0`；每批 `git diff --check` RC=0。
- GBK 控制台中文乱码、搜索面板 `hits[].kind` 探针粒度、1280 末列免滚、静态同 sha，均已逐项如实标注，不作假阳性 PASS 依据。

## 九、收尾（finalize）

- 临时令牌：`data/admin-auth-init.js`、`data/login.json` 已删除（`AUTH_DELETED=YES`）；data/ 目录整体删除、未入库。
- QA 库还原：`104/max477`（`DB=104|477`、`FINALIZE_DONE=YES`）。
- 浏览器会话全 close、Chrome 残留清零；所有后台 job（服务、三视口、CTA、搜索、七 tab、scroll、focus）`job_kill` 完毕，active=0。
- 终态：HEAD=REMOTE，AB=0/0，tracked clean；.gitignore 431 行、历史规则未截断。

## 十、结论

R38 部署后真 Chromium 独立回归 **PASS**：三视口 18×4 零不符、AI 四要素 18/18/18、CTA 安全停留 anchor=composite 不跳看板、表内横滚末列可见且页面无溢出、七 tab 无回退、≥4 新图互异且全≠QA 六图；TestNoRealSecretsInRepo SKIP 非 PASS 已如实标注。报告路径：`docs/laiyipao/audits/round38-postdeploy/REPORT.md`。
