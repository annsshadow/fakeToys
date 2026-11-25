# R42 真浏览器复查：Wiki 关卡表所属章节列与交叉引用（t41）

结论：**PASS（带诚实边界）**。三视口真浏览器按公开 /api/v1/config 逐关核对新增第 14 列「所属章节」，MISMATCHES=0；章节下拉切换、列内交叉引用点击、AI CTA 点击均由真实 Chromium CLI 验证并留在关卡 tab；表内横滚、页面无横向溢出、R31–R41 十一 tab 回归、测试门禁均通过。以下如实标注探针假阳性、SKIP 守卫、截图 sha 漂移与临时凭据文件处置，不把它们伪装成通过。

## 权威快照与核对口径
- 公开 API 快照：`docs/laiyipao/audits/round42-wiki-level-chapter/data/config.json`，sha256=`467b58ff64a1cea33be3bc58309341a92e64f2b64f94296edaf1da4ce2ab09ea`，version=`6237e0b623219b8b`。
- 权威 chapters：1 边境哨站 1–20；2 工业废墟 21–40；3 冻原 41–60；4 感染矿井 61–80；5 母巢外围 81–95；6 母巢核心 96–100。levels 共 100 关，chapter 合法去重 6。
- 渲染期望：第 1–20 关末列为「边境哨站」，41–60 为「冻原」，96–100 为「母巢核心」；列内链接 href=`#/wiki?tab=levels&anchor=chapter-mechanics`；AI 结论 `AI 章节交叉概览：共100关，覆盖6章`。
- 红线：chapter 是配置归属，不是 terrain_used，不参与 replay_hash；AI 依据/诚实/CTA 四要素逐字来自 `laiyipao/admin/src/levelChapterText.ts`。

## 真浏览器 DOM 与逐关对账
- 三视口证据：`data/vp-{390,768,1280}.json`（job `bash-0d79013c95f341e9`）。每视口表头 14 列，末列「所属章节」；可见关卡行 20（默认章 1），逐关 chapter→章节名核对 MISMATCHES=0；首行 1 边境哨站·1 → 边境哨站，末行 20 边境哨站·20 → 边境哨站。
- AI 四要素：`data/vp-*.json` 的 ai=`AI 章节交叉概览：共100关，覆盖6章`；basis / honesty 与常量逐字一致；CTA 文本「查看章节一览 →」，href=`#/wiki?tab=levels`；dirty=false（无 NaN / undefined / [object Object]）。
- 严格 DOM 结构：`data/dom-structure.json`（job `bash-fab8cf9bd8cc44c2`）thCount=14，表头依次为 关/名称/难度/防线血量/波次/体力/层数上限/反应阶/星级门槛/首领/地形（配置基值）/护甲‰/满分/所属章节；el-table 带 `el-table--scrollable-x`。
- 横滚与页面溢出：`data/ss-state-*.json`（job `bash-35dce7cd22934623`）表内 wrap scrollWidth=1390，clientWidth=324/702/884，scrollLeft 实际置到 1066/688/506；三视口 document.scrollWidth==innerWidth==390/768/1280，无页面横向溢出。

## 真实 CLI 交互（禁 DOM eval 作最终点击证据）
- 列内交叉引用：job `bash-447e10be4672451a`，`data/click-link.txt=✓ Done`。点击前 hash=`#/wiki?tab=levels`、chapterTop=364；点击后 hash=`#/wiki?tab=levels&anchor=chapter-mechanics`、章节表可见 chapterTop=0、未跳数据看板、docW==innerW==1280。
- AI CTA：同一 job，`data/cta-click.txt=✓ Done`；`data/cta-before.json`→`cta-after.json`：hash 从 `#/wiki?tab=levels` 变为 `#/wiki?tab=levels&anchor=chapter-mechanics`，onWiki=true，章节表可见 chapterTop=0，未跳看板。
- 章节下拉：job `bash-a3b0ee4aa9fa4e26` / `bash-22b4f02ac4794f1e`，`ch3-click/down/enter.txt` 与 `switch-click1.txt` 均 `✓ Done`。第 3 章（两次 ArrowDown）`data/ch3-probe.json`：20 行，首行 41 冻原·41 → 冻原，末行 60 冻原·60 → 冻原；第 6 章（四次 ArrowDown）`data/switch-ch6.json`：5 行，首 96 母巢核心·96 → 母巢核心，末 100 母巢核心·100 → 母巢核心。中间一次 `switch-ch3.json` 实际落在第 2 章（20 行 21–40 工业废墟），已由正确 ch3 重跑覆盖；如实保留为过程探针，不作为第 3 章结论。

## R31–R41 十一 tab 回归
- job `bash-825bfdec6c1e4bc4`，`data/reg-{basics,rating,enemies,skills,gems,equipment,skins,rules,mastery,levels,live}.json`：每页 dirty=false、docW=innerW=1280；basics/rating 评分 5 行、反应链 150‰/4px 文案在；enemies 22、skills 24/合成 18/配方 18、gems 8、equipment 18、skins 6、rules drop 6、mastery 8×12=96、levels 默认 20 + chapters 6、live 0。
- 探针假阳性校正：`reg-*.json` 的 reactionRows=0 是把 h4 `#reactions` 当表容器；`data/rc-basics.json` / `rc-skills.json`（job `bash-329bb921a1bd4309`）精确探针反应链 7 行、has150=true、has4px=true，合成/配方两表 18/18 行，与权威一致，无回归。
- 令牌：`laiyipao/admin/src/styles/tokens.css:12-14` 三锚点 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 保留；grep `--r42-` 仅命中测试否定断言与规范禁用说明，无令牌定义、无新增色值令牌。

## 截图 sha（新增 8 张，均非登录页，互异）
- `screenshots/r42-level-table-390.png`：`6e3c2e964abbe60328eaedc49074ecd75a04e32b6d2a258bf067c19234bf968d`（390×844，已滚动到第 14 列与 AI 区，可见「所属章节」链接）。
- `screenshots/r42-level-table-768.png`：`899850bd6118d3d3c6e0664325e41bb2783734fc86e5b01537225f53bb024ad4`（768×1024，可见末列整列 20 行章节名与 AI 四要素）。
- `screenshots/r42-level-table-1280.png`：`d25f34bd53d756b1b92e73033719ed51037a011374d58be4ddfd193795682854`（1280×800）。
- `screenshots/r42-level-chapter-390.png`：`42b9e9df11ae11f05d6e087e0154f164995dc7dac7a43b698e5eeabb790f4b5f`。
- `screenshots/r42-level-chapter-768.png`：`85cd06b2823f6de10afd64c9dbd4c91c5e9b6b682c7643a898710e79b4117bcd`。
- `screenshots/r42-level-chapter-1280.png`：`028714d1d5648527c050908b53c75b87caaf52e3bf0e7e36ff44b767ea3bb0e4`。
- `screenshots/r42-crossref-after.png`：`804efe6049a8da29fa24fb75303af4bbd730f6e6ab67d7aa66438e762b6b94fd`。
- `screenshots/r42-cta-after.png`：`a7ddc6459d4cded7a6b988fe6230de727a1f7e5b5a9389c607e8cd9f5e7d3d41`。
- 诚实边界：`r42-level-chapter-{390,768,1280}.png` 的 sha 与 R41 修正后截图完全相同，原因是首屏内容（R41 章节表）未变、关卡表在下方未入镜；这不是伪造，也不作为 R42 新增列的视觉证据，R42 以滚动后 `r42-level-table-*.png` 三张为准。三视口均遵守 open 后再 set viewport（先设后开在 headless 封装里不生效）。

## 门禁、守卫与零真值
- R42 契约：`cd laiyipao/admin && pnpm exec vitest run tests/WikiLevelChapterMechanics.test.ts --reporter=dot` → 1 file / 11 tests passed，exit=0。
- admin 全量：81 files / 899 tests passed，exit=0；type-check 与 build 均 exit=0（build 2.01s）。
- Go 守卫：`go test ./internal/domain -run "TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo" -count=1 -v` exit=0；TestNoInfraIdentifiersInRepo PASS；TestNoRealSecretsInRepo **SKIP**（读不到 laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是通过了），不冒充 PASS。
- 字面量扫描：审计脚本/报告共 77 个文本文件，唯一命中是临时 `data/admin-auth-init.js` 内的 token 初始化行；该文件由环境变量401令牌派生的既有审计产物写入，已纳入 .gitignore:458，并在收尾删除；报告与脚本不写任何口令/令牌/绝对路径真值。
- QA 库收尾核对：battle_records count/max(id)=104/477；未在本次复查中修改业务源码、未执行 git 写操作。
