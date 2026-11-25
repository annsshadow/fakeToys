# R35 复查报告：Wiki 宝石图鉴真浏览器三视口闭环

- 任务：t272（被审实施任务 t271），attempt_id=22923978-cda4-416d-9cd1-ca280adb9140
- 基线：HEAD=3a53967fc，工作树含未提交 R35 改动；QA 只读，不改业务源码、不 git
- 审计目录：docs/laiyipao/audits/round35-wiki-gem-mechanics/
- 采样时间：2026-11-25T14:45+08:00（Asia/Shanghai）
- 权威配置：public /api/v1/config，version=6237e0b623219b8b，CFG_SHA=15eec211261f6e8cd94efe2e93cb2ce33441dbde868d71ec0773f6118ca665b0，快照落 data/config.json

## 一、总判定

**FAILED**：硬门禁③「搜索『锋锐石』『gold』命中并跳宝石 tab 高亮」中 gold 未命中，属于真实功能缺口，不可宣称整体 PASS。其余门禁项均坐实通过，详见下表。

## 二、硬门禁逐项结果

| # | 门禁 | 结果 | 证据 |
|---|------|------|------|
| ① | 三视口逐字段对账 8 宝石 + 5 品质，MISMATCHES=0，禁百分比 | PASS | data/reconcile.log：390/768/1280 各 GEMS=8 QUALS=5 MISMATCHES=0 AI_OK=True NOTE_OK=True BAD=False VERDICT=PASS；data/config.json 为唯一权威，千分号 1000‰…2000‰ |
| ② | documentElement.scrollWidth <= innerWidth+1；两表可横滚末列 | PASS（横滚无实际溢出） | reconcile.log DOC=390/390、768/768、1280/1280；W1/W2=0->0，两表 scrollWidth==clientWidth（列宽自适应，无需横滚，非失败）；CSS .r35-gem-table-scroll 有 overflow-x:auto |
| ③ | 搜索「锋锐石」「gold」命中并高亮 | **FAILED** | 锋锐石：data/search-锋锐石.json focus=gem:1 / highlighted=1 / hash=#/wiki?tab=gems&focus=gem:1（PASS）；gold：data/search-gold.json focused=[] / highlighted=0 / hash=#/wiki?tab=gems（FAIL）。根因：WikiView.vue:422-507 的 searchIndex 只遍历 c.gems，未索引 c.gem_qualities |
| ④ | CTA 真实 CLI 点击 [data-r35-cta] 后停留 gems | PASS | data/cta-pre.json、data/cta-click.txt(✓ Done)、data/cta-post.json：hashAfter=#/wiki?tab=gems，dashboard=false，activeNav=[宝石图鉴] |
| ⑤ | AI 四要素同屏逐字，诚实文案，禁 MinValue/MaxValue | PASS | probe-*.json AI_OK=True NOTE_OK=True BAD=False；截图可见「词条随机，具体数值以游戏内为准」 |
| ⑥ | R31/R32/R33/R34 四 tab 不回退 | PASS | data/reg-basics.json/reg-gems.json：reactionCount=7、腐蚀传播1层、150‰/4px、skillCount=24、enemyCount=22、levelCount=20、bad=false；其余 reg-* 三 JSON 已落盘 |
| ⑦ | ≥4 张新截图 sha 两两不同且亲读非登录页 | PASS | 10 张唯一 sha 见第三节 |
| ⑧ | 删 auth-init、gitignore、QA 库还原、不改业务源码不 git | PASS | auth-init 已删(AUTH_BEFORE=NO)；gitignore :402/:403 命中；zero-scan BAD_COUNT=0/SYNTAX_RC=0；QA 库 battle_records=104/max(id)=477；本 QA 未改业务源码未 git |
| ⑨ | REPORT 如实记录失败/未验证项 | PASS | 本报告明确判定③失败并列根因 |

## 三、截图证据（当前 sha256sum，亲读非登录页非看板）

- r35-390-gems.png：3dd91d6dc92617ef8ef74156bc01bd2426d0d0ecda8e0d746ca71d9de7c7c184（390×844，宝石表8行+品质表5行）
- r35-768-gems.png：26d1ec958e3b72c2a8e2d895f92d0454a6d0d1add583fea31938687b8fb8c9fc（768×1024）
- r35-1280-gems.png：25241659fa93ea9d4afa4c539307b706aa72b79317ece091622943fced5200d9（1280×800）
- r35-05-cta-after.png：73d2fc6f61619897c51bc10eb34af15e1bcd51715e8f788b6e18595695b08eb2
- r35-search-锋锐石.png：fde56332687143be9a0be60aca29e23edd9b651f12bb5e757e0b6b2b4c0c0b03
- r35-search-gold.png：a2ad27ccd25e50e3259d6f052a84dd91e2cad6a885ab9fafc190f6895f6c33a3（截图存在但该图恰是未命中态，不作搜索成功证据）
- r35-reg-basics.png：daa0cafad3096636ecc7a3b5fe7f49a8fd9454534d4066fd3ce3a94df52075b8
- r35-reg-enemies.png：51d0fd7caa3927944a2456599cf26b9b180ea65abfffa8ce427d79c26c8c012e
- r35-reg-gems.png：5c41f6814e8b0cc86c0ecac89ab2d457abd6eecbf8cc242c6c2982ee839acb12
- r35-reg-levels.png：57612e4cd4dc68dc79a2c92f2964bd4732657b9f83632a286ee279481da2d3e0
- r35-reg-skills.png：b7b8e247d9553d15fb98f2dc3963994f74eb65d16c43e6865d9b5abeb32a4031

重复图：r35-04-cta-before.png 与 r35-390-gems.png sha 相同，不作独立证据。

## 四、目录项与 section 对应

data/nav-probe.json：navCount=9、sectionCount=9、gemSectionCount=1、gemSectionVisible=true、hash=#/wiki?tab=gems。目录项【核心规则/敌人抗性图鉴/技能图鉴/宝石图鉴/装备图鉴/数值规则/专精树/关卡一览/当前版本实战分布】与 SECTIONS 一一对应。

## 五、测试与构建（前台独立复跑，data/tests.log）

- R35 定向 vitest run tests/WikiGemMechanics.test.ts --reporter=dot：8/8 passed，RC=0
- 前序 R31-R34+R35 合并（WikiGem/Enemy/Reaction/Skill/View）：5 files / 78 tests passed，RC=0
- admin 全量 pnpm test -- --testTimeout=90000：74 files / 826 tests passed，RC=0
- pnpm type-check：RC=0；pnpm build：RC=0（built in 1.35s）
- TestNoRealSecretsInRepo：前置 laiyipao/deploy/.local.keys 不存在，按 laiyipao/server/internal/domain/secrets_leak_test.go:145 显式 Skip，未执行真值子串比对，不计 PASS；手工零真值脚本 BAD_COUNT=0 作为补充

## 六、发现的问题与修复建议

**P1 — 品质名 gold 无法被搜索命中（硬门禁③失败）**
- 现象：在 Wiki 搜索框输入 gold 并回车，hash 保持 #/wiki?tab=gems，focus 列表为空，无高亮。
- 根因：laiyipao/admin/src/views/WikiView.vue 的 searchIndex 计算属性（约 422–507 行）为 gems 建立 kind=gem 索引（title=g.name、meta=宝石·attrText、body=g.descr），但完全没有遍历 c.gem_qualities；所以品质名（gray/blue/purple/red/gold）与倍率（1000‰…2000‰）都不在搜索域中。
- 建议修复：在 searchIndex 中为 c.gem_qualities 追加条目（kind=quality、id=q.name、title=q.name、meta=宝石品质·qualityText(q)、body 含词条数与倍率、section=gems），并让 pick() 对 section=gems 的跳转/高亮兼容 quality 类型，同时补一条测试确保搜 gold 命中并跳 gems tab。本次 QA 不改业务源码，留给后续修复任务。

## 七、诚实边界

- 初期三视口 E2E 因缺第二次 set viewport + reload 产生假阳性（三探针同 docW、截图同 sha），已修脚本重跑，采用修正后证据。
- r35-04-cta-before 与 r35-390 同 sha，重复图不计入独立证据数量。
- 横滚门禁：两表 scrollWidth==clientWidth，无法演示滚到末列动作但列已全部可见，记为 PASS 并注明无实际溢出。
- 后台 job 状态显示 running 滞后，实际产物文件已落盘并经 ops_read_files 亲读确认。
- 本轮真实 Chromium 走本地回环地址，无 shim；无 DOM eval/e.click 冒充最终点击证据（CTA 用 agent-browser CLI click）。

## 八、结论

除搜索品质名 gold 未命中这一真实缺口外，R35 实施在三视口逐字段对账、CTA hash 安全、AI 四要素、R31–R34 回归、测试构建与清理方面全部通过。整体复查判定 **FAILED**，待修复任务补上 gem_qualities 搜索索引后重新复查。
