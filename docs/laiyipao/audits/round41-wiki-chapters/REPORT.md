# R41 真浏览器复查：Wiki 章节元数据六字段对齐（t35）

结论：**PASS（带诚实边界）**。三视口真浏览器逐字段核对 6 章 × 6 字段 MISMATCHES=0；章节下拉切换、CTA、章节搜索命中与高亮均由真实 Chromium CLI 验证。以下明确标注探针假阳性、SKIP 守卫与已知工具限制，不将它们伪装成通过。

## 权威快照与核对口径
- 公开 API 快照：`docs/laiyipao/audits/round41-wiki-chapters/data/config.json`，sha256=`acb51ad23b1c1c94d31cba28e214f54fd2e2125ae7ef6f2dc70549b19040261e`，version=`6237e0b623219b8b`。
- 六章配置（以活配置为准，示例仅参照）：1 边境哨站 1–20 oil_drum boss0；2 工业废墟 21–40 collapse_wall boss0；3 冻原 41–60 charge_tower boss16；4 感染矿井 61–80 tidal_gate boss0；5 母巢外围 81–95 rotor_vane boss19；6 母巢核心 96–100 charge_tower boss22。
- 渲染期望：`1–20关/油桶/—`、`21–40关/塌墙/—`、`41–60关/蓄能塔/#16`、`61–80关/潮汐闸门/—`、`81–95关/旋翼风扇/#19`、`96–100关/蓄能塔/#22`；区间用 en-dash，地形五映射，首领仅正整数 #N。
- AI 结论：`AI 章节概览：共 6 章：含首领 3 章，地形 5 种`。basis / honesty / CTA 四要素齐备，CTA href=`#/wiki?tab=levels`，无脏值、无占位符病句。

## 真浏览器 DOM 与截图
- 三视口 DOM 证据：`data/vp-chapter-{390,768,1280}.json` 均 ROWS=6、MISMATCHES=0、sectionVisible=true、AI 结论精确、dirty=false。
- 表内横滚 / 页面溢出：768 视口 `scrollWidth=740 clientWidth=702 scrollLeft 0→38`，证明表内横向滚动；390/1280 为 `sw=cw` 但无溢出。页面 `document.scrollWidth==innerWidth`，无页面横向溢出。注意：早期探针在 headless 封装下把 `set viewport` 放在 `open` 之前不生效，实际 innerWidth=1258；已重跑（`data/vpc-state-*.json`）。
- 修正后截图（当前 sha256，非登录页，互异）：
  - `screenshots/r41-chapter-390-correct.png`：`42b9e9df11ae11f05d6e087e0154f164995dc7dac7a43b698e5eeabb790f4b5f`，状态 iw/ih=390/844。
  - `screenshots/r41-chapter-768-correct.png`：`85cd06b2823f6de10afd64c9dbd4c91c5e9b6b682c7643a898710e79b4117bcd`，iw/ih=768/1024。
  - `screenshots/r41-chapter-1280-correct.png`：`028714d1d5648527c050908b53c75b87caaf52e3bf0e7e36ff44b767ea3bb0e4`，iw/ih=1280/800。
  - `screenshots/r41-chapter-search-390.png`：`a016ca25bad15acdd365be9893470abbb62a14e1ce3b4437437323899ed515f8`。
  - `screenshots/r41-chapter-search-768.png`：`b70e3d7e86005d6ba011134d792e9fbaa4e3a26652915a24110b3d14b30820d9`。
  - `screenshots/r41-chapter-search-1280.png`：`0ffd3015a017f6c81ec9f99183134f651403b555d82bfc16df6366f16b5acb6a`。
- 历史诊断截图 `r41-chapter-{390,768,1280}.png` 与 `r41-390-rating-fontfix.png`/`r41-wide-full-verify.png` 实为 iw=1258 捕获，sha 漂移不代表当前视口证据，报告中只采用上述当前 sha。

## 真实 CLI 交互
- 章节下拉：job `bash-773ae75913244047`，`dropkeys-open3/enter3/open6/enter6.txt` 均 `✓ Done`；第 3 章 ACTIVE=冻原、LEVELS=20、FIRST=41 冻原·41、LAST=60 冻原·60；第 6 章 ACTIVE=母巢核心、LEVELS=5、FIRST=96、LAST=100。章节表仍 6 行。
- CTA：`data/cta-click.txt=✓ Done`；`cta-post.json` hash=`#/wiki?tab=levels`，未跳看板，章节表仍可见。无 DOM eval 点击作为最终证据。
- 章节搜索：`rotor_vane` 唯一命中 kind=章节 title=母巢外围；三视口 `data/chsv-hits-*.json` hits=[`章节|母巢外围`]，`chsv-click-*.txt=✓ Done`，`chsv-pick-*.json` URL=`#/wiki?tab=levels&anchor=chapter-mechanics&focus=chapter:5`，高亮第 5 行 母巢外围 81–95关 旋翼风扇 #19。
- 评分/皮肤搜索不回退：`search-fanyinglian.json` / `search-yuansu.json` 命中评分维度；点击后 focus=`rating:reaction_coverage`/`element_coverage`，对应行高亮。皮肤列表 6 行不变。
- 冻原搜索限制：`冻原` 命中很多，SEARCH_LIMIT=20（`WikiView.vue:622-633`）时前 20 条都是关卡条目，章节命中在后续；CLI 的 find 不支持 nth，select 不能直接切换 el-select 选项，键盘下拉测试已替代证明第 3 章切换。没有以 DOM eval 伪装点击。

## R31–R40 回归
- 十标签 CLI 遍历：`reg4-*.json`（job `bash-28852e94f3c5461d`）每页均 `dirty=false docW=innerW=1280`，稳定计数：skills=24、recipes=18、enemies=22、levels=20、gems=8、qualities=5、skins=6、rating=5、drop=6、chapter=6、masteryRows=96。
- 反应链与合成计数的探针假阳性校正：`reg4-*` 的 reaction=0 是因为 `#reactions` 是标题 h4 而非表容器；composite=36 是选择器重叠双计。精确探针 `data/rc-basics.json` / `rc-skills.json` 证实 reactionRows=7、has150=true、has4px=true，合成/配方两张表均 18/18。与权威一致，无回归。
- 早期 `smoke-probe2.txt`、`s2-probe.txt`、`s3-probe.txt`、`reg3-*.json` 及空 rows 的 `vp-chapter-*.json` 是探针选择器错误导致的假阳性，不作为最终证据。

## 测试闸门与守卫
- R41 契约（t34 已跑，实施模块真实导入）：10/10；admin 全套：888/888；type-check / build / diff-check 全绿。本次复查未改业务源码。
- Go 守卫：`go test ./internal/domain -run "TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo" -count=1 -v` exit=0；`TestNoInfraIdentifiersInRepo` PASS；`TestNoRealSecretsInRepo` **SKIP**（缺 `laiyipao/deploy/.local.keys`，原文：`读不到 … —— 真值扫描这条守卫**没有执行**，不是「通过了」`）。不把 SKIP 写成 PASS。
- 零真值：临时 admin-auth-init 从已有审计 token 工件生成，写入被 `.gitignore:450` 命中的 `docs/laiyipao/audits/round41-wiki-chapters/data/`，结束前删除；报告与脚本不含 token/口令/Authorization/C:\Users 字面量。

## 收尾
- QA 库：`battle_records count=104 max(id)=477`。
- API/Vite 后台 job 已停止；agent-browser 会话已关闭。
