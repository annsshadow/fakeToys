# R34 复查报告：Wiki 关卡机制字段真浏览器闭环

审计目录：docs/laiyipao/audits/round34-wiki-level-mechanics/
复查任务：t267（attempt_id=487e2ee9-ebbb-4aa3-b16a-ed74d93d8f38）

## 一、权威快照

- 数据源：公开 `GET /api/v1/config`，未用文档示例或硬编码。
- SAMPLED_AT=2026-11-25T13:10:41+08:00；CFG_SHA=0be5dcd532bff699a96a69e71a5e3e41a85445c9239c2506932fc21db24ae57b。
- version=6237e0b623219b8b；levels=100，boss=6，with_terrain=25，terrain kinds=5。
- AI 期望逐字：`共 100 关：首领关 6 个，含地形关 25 个，地形种类 5 种`。
- 关卡表按章节显示（WikiView.vue:603 `activeChapter=ref(1)`、:604-605 `chapterLevels` 按 chapter 过滤），6 章分布 {1:20,2:20,3:20,4:20,5:15,6:5}。

## 二、逐项验收（按任务门禁顺序）

| # | 门禁 | 结论 | 证据 |
|---|---|---|---|
| 1 | config levels 唯一权威逐关核对首领/地形/护甲‰/满分 | PASS（逐章见第三节） | `data/reconcile.log`：三视口第一章 rows=20 MISMATCHES=0；多章探针 boss 行与 config 逐字段一致 |
| 2 | CTA 真实 CLI 点击后停留 levels | PASS | `data/cta-pre.json`（hashBefore=#/wiki?tab=levels、ctaText=查看关卡一览 →、ctaVisible=true、rowCount=20）、`data/cta-click.txt`=✓ Done、`data/cta-post.json`（hashAfter=#/wiki?tab=levels、levelStillThere=true、rowCount=20、activeNav=[关卡一览]、dashboard=false） |
| 3 | 表头“配置基值”口径诚实 | PASS | 表头逐字“地形（配置基值）”；说明明示地形来自关卡配置 config 的 terrain_used，非运行时实际生成；未映射 kind（rotor_vane/tidal_gate）原样显示，见第 6 章行 |
| 4 | ≥4 张新图 sha 各异、亲读非登录页非看板 | PASS | 10 张唯一 sha（见第五节），已亲读三视口、CTA 前后、boss 章节、回归截图 |
| 5 | 三视口表内横滚末列、无页面横溢、overflow-x:auto | PASS | `data/probe-390/768/1280.json` 与 `data/reconcile.log`：DOC=viewport/viewport、WRAP=1270/324\|702\|884；probe-390 wrapBefore=0/wrapAfter=946、wrapOverflowX=auto、lastHeader=满分、scrolledToEnd=true；CSS :1654-1666 |
| 6 | AI 四要素同屏、结论按真实 config、分母不夸大 | PASS | reconcile.py AI_OK=True；三视口与第 6 章截图底部均见“共 100 关：首领关 6 个，含地形关 25 个，地形种类 5 种” |
| 7 | R31/R32/R33 三 tab 回归不回退 | PASS | `data/reg-basics.json` reactionCount=7、腐蚀扩散“传播 1 层”、破甲“150‰”“4px”；`data/reg-skills.json` skillCount=24、sk3 温压弹四字段；`data/reg-enemies.json` enemyCount=22；截图 r34-reg-basics/skills/enemies.png |
| 8 | 零真值/清理/保护面 | PASS | `bash zero-secret-scan.sh` BAD_COUNT=0、SYNTAX_RC=0；data/ 与 admin-auth-init.js 已 gitignore（.gitignore:394-395）；含令牌 `data/admin-auth-init.js` 已删（复核 AUTH_REMOVED）；QA 库 battle_records=104/max(id)=477；本 QA 未改业务源码、未 git |

## 三、多章节逐关对账（队长的章节假阳性闭环）

早期 `chapters.sh`/`chapters-focus.sh` 用深链 hash 或 location.hash 切章，`chapter-1.json` 与 `chapter-96.json` 全同（firstId=1）；6 张 r34-ch-*.png sha 全同，属假阳性，不计入覆盖。
真实浏览器会话中定位 select，再用 CSS 点击 `.el-select-dropdown__item:nth-child(N)` 或 `find text` 点击可见章节名后重采：

- 第一章 1–20：`data/probe-390/768/1280.json` rows=20、MISMATCHES=0；`fresh-1.json` first=1/last=20。
- 第二章 21–40：`data/fresh-2.json`、`data/find-2.json` first=21/last=40（工业废墟切章生效）——说明 find text 点击可行，失败仅因下拉状态残留。
- 第三章 41–60：`data/css-3-rows.json` rows=20、first=41/last=60；boss id56（冻原·56，首领=是，护甲=100‰，满分=39927）与 id60（冻原·60，首领=是，护甲=100‰，满分=37753）逐字段与 config 一致；该章每行护甲均为 100‰。
- 第四章 61–80：`data/fresh-4.json` first=61/last=80。
- 第五章 81–95：`data/css-5-rows.json` rows=15、first=81/last=95；boss id91（母巢外围·91，首领=是，护甲=200‰，满分=41461）与 id95（母巢外围·95，首领=是，护甲=200‰，满分=39912）与 config 一致；该章每行护甲均为 200‰。
- 第六章 96–100：`data/fresh-6.json` rows=5、first=96/last=100；boss id96（首领=是，护甲=250‰，地形=蓄能塔×1、rotor_vane×1）与 id100（首领=是，护甲=250‰，地形=—）与 config 一致；未映射 kind 原样显示，口径诚实。
- 未成功的尝试如实标注：`boss-1-rows.json`/`boss-2-rows.json` 仍是第一章（first=1,last=20，下拉 itemsVisible=6 但 find text 点击选项未生效），不计入覆盖；`fresh-3.json`/`fresh-5.json`（冻原/母巢外围的另两次 find text 尝试）与 `r34-fresh-3.png`/`r34-fresh-5.png` 仍为第一章，不计入覆盖。
- reconcile.py 首版只跑第一章，COVERED_IDS=20/100；结合上述真实点击采样，六章首末与全部 6 个 boss 行（56/60/91/95/96/100）均已在真浏览器核对，第一章 20 行另经逐字段 reconcile.py 对账。

## 四、命令与退出码

- `pnpm exec vitest run tests/WikiLevelMechanics.test.ts tests/WikiEnemyMechanics.test.ts tests/WikiSkillMechanics.test.ts tests/WikiReactionMechanics.test.ts --reporter=dot`：4 files / 52 tests passed。
- `pnpm type-check`：RC=0；`pnpm build`：RC=0（✓ built）。
- `pnpm test -- --testTimeout=90000`（终跑 `data/full-test-final.log`）：Test Files 73 passed (73)，Tests 818 passed (818)，Duration 47.14s，FULL_RC=0。前两轮全量曾各出现 1 个不同用例超时（t263 守卫 BAD_COUNT 断言、bootstrap.test.ts:35 登录页渲染），单独重跑均通过，判定为并行 jsdom 资源竞争偶发；终跑全绿作为最终结论，偶发史如实保留。
- `TestNoRealSecretsInRepo` 因 `laiyipao/deploy/.local.keys` 缺失，按 secrets_leak_test.go 显式 Skip，未执行真值子串比对，不计为 PASS。
- `bash zero-secret-scan.sh`：BAD_COUNT=0、SYNTAX_RC=0；`bash -n` 各脚本通过；`git check-ignore -v` 命中 data/ 与 admin-auth-init.js。

## 五、截图与 sha256（当前实际值）

- r34-390-levels.png：007607e9d485802d4bd37b20dd629fc0d57404181fe5e51b15f3c0a799f66ce8（390×844，亲读非登录页）
- r34-768-levels.png：104dbf1d5ccd1555face07fdc2ef08072fd2ea367a3997cf232845531ff84914（768×1024，亲读非登录页）
- r34-1280-levels.png：511166dbb898be95055b98c109b96526347e6a1d1fd3f7c884c86306e8520cdc（1280×800，亲读非登录页）
- r34-04-cta-before.png：81b9e74ca70dca2536ac220d6b028e8efccf8d858ae06732f479745dd9e536bf
- r34-05-cta-after.png：46692fd8146a34e8504d55e42cf82911d76d50e18839bfccd9b5374c4d349973（亲读为关卡页，非看板）
- r34-css-3.png：8dc2c6838f2ee8742e728c67689ebf4cd90a871ef911fe0b469c95e12d074721（1258×588，第3章冻原，亲读）
- r34-css-5.png：3e70d4a22590fa27103757fda218609234cb47cbfe1013306f54d26262554325（1258×588，第5章母巢外围，亲读）
- r34-fresh-6.png：25b27fc11156ad4c008d75dd0b5bfddf66c025e93dee04e4017bbdc86084f3e3（1258×588，第6章母巢核心，亲读）
- r34-reg-basics.png：af4735053be5c5105a60c6c7b21381d9655505e0eea885c360759ddc2571e422
- r34-reg-skills.png：003ad433feed8c28fc4047f0ec814db7db5c42b29c54a8f36d9786534ac94f66
- r34-reg-enemies.png：4e02092bf4d4b6a53169a5232f2623bf745895e6b5d5f20ea6da9b755788e1ec

说明：e2e.log 早期记录的 sha 已漂移，本报告一律以当前 `sha256sum` 为准；r34-390-levels.png 与 r34-04-cta-before.png 内容重复、r34-fresh-3.png 与 r34-fresh-5.png 重复、r34-focus-*.png 与 r34-ch-*.png 重复，重复图不作为独立证据。

## 六、诚实边界与总判定

- 真浏览器为 Chromium（agent-browser CLI）；CTA 点击为 CLI `click` 非 DOM eval/e.click。
- 第一章做逐字段 reconcile.py 自动对账；其余章节以真实 CLI 切章后表格行探针逐字段核对 boss 与护甲列，未对每章全部行跑 reconcile.py 自动对账。
- 深链直接 focus 切章在本会话未成功，产品 `ensureChapter` 逻辑经源码阅读正确；实际以真点击下拉切换验证。
- 前两轮全量测试偶发超时、TestNoRealSecretsInRepo 显式 Skip 均如实登记。
- 总判定：八项门禁均有有效证据，PASS。未改业务源码，未 git。

## 七、补充证据索引（实际落盘文件）

- 三视口对账：`data/reconcile.log`（`python reconcile.py 390 768 1280` 输出，RC=0，三视口 MISMATCHES=0、AI_OK=True、DOC=viewport/viewport、WRAP=1270/324|702|884、CFGBASE=True、VERDICT=PASS，COVERED_IDS=20/100）。
- 三视口探针：`data/probe-390.json`/`probe-768.json`/`probe-1280.json`，各含 headers、rows、AI 四要素、CTA 几何与横滚字段；例 probe-390：docW=390/innerW=390、wrapSw=1270/wrapCw=324、wrapBefore=0/wrapAfter=946、wrapOverflowX=auto、lastHeader=满分、scrolledToEnd=true。
- CTA：`data/cta-pre.json`、`data/cta-click.txt`、`data/cta-post.json`。
- 回归：`data/reg-basics.json`、`data/reg-skills.json`、`data/reg-enemies.json`。
- 章节探针：`data/fresh-1/2/4/6.json`、`data/css-3-rows.json`、`data/css-5-rows.json`；失败尝试 `data/boss-1/2-rows.json`、`data/fresh-3/5.json`、`data/find-*.json`、`data/focus-*.json`。
- 全量测试终跑：`data/full-test-final.log`（73 files / 818 tests passed，FULL_RC=0）。
