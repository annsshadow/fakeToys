# R100 规范：可点击交互行键盘可达补漏（test-first RED）

## §0 四条件裁定

| 条件 | 判定 | 依据（t254 R100-A 侦察报告） |
| --- | --- | --- |
| ① 真实可复现用户可见缺陷 | 成立 | 真 Chromium 三视口坐实公告/抗性图/卡片跳转 role=null/tabIndex=-1/AX 无 button；战报行源码决定性（0 战报未浏览器坐实） |
| ② 展示层小改 | 成立 | 每处开标签 +role=button +tabindex=0 +@keydown.enter 同名 handler，无逻辑/样式/数据改动 |
| ③ 权威口径明确 | 成立 | R60 r60A11yKeyboard.contract.test.ts §41-53 / R95 me.vue:11 / R99 text.link 19 处同范式 |
| ④ 改动面可控 | 成立 | 8 处候选（3 核心交互行 + 5 精选扩展），limit 到交互行/导航 chip，不触碰在途态/表单/遮罩 |

队长裁定：采纳 A 方向（键盘三件套补漏）；B 方向（aria-expanded 语义）记 R101 待办，本轮不纳入。

## §1 权威口径

- 形态：R60 钉死的 role="button" + tabindex="0" + @keydown.enter 同名 handler 四件套（@click 原入口保留）。
- 先例：R95 me.vue:11 返回键、R99 可点 text.link 19 处均真实 fs.readFileSync 读源码 + handler 表达式稳健定位；本契约沿用同款，不用内嵌实现副本。
- 定位：以 @click handler 表达式反查开标签，再用静态 class 精确区分同 handler 多元素（toggleEnemy 有 .rxn-name / .resist-toggle；go(m.path) 有 playMenus/codexMenus 两处）；不硬编码行号，容忍上游插行漂移。
- handler 同源：@keydown.enter 必须与 @click 逐字同名表达式（含 m.hint && 守卫），防行为漂移。

## §2 精确改点表（8 处）

| # | 文件 | class 锚点 | @click handler（同名 @keydown.enter） | 分类 |
| --- | --- | --- | --- | --- |
| 1 | pages/battles/battles.vue | battle-row | toggleDetail(it) | 核心：战报展开行 |
| 2 | pages/index/index.vue | ann-item | toggleAnn(a.id) | 核心：公告展开行 |
| 3 | pages/enemies/enemies.vue | resist-toggle | toggleEnemy(en.id) | 核心：抗性图展开（二选一，优先 .resist-toggle） |
| 4 | pages/index/index.vue | action-card | goStage | 扩展：构筑卡片 |
| 5 | pages/index/index.vue | grid-item（2 处） | go(m.path) | 扩展：玩法/图鉴卡片 |
| 6 | pages/index/index.vue | mg-row | m.hint && go(m.hint) | 扩展：边际收益行（守卫同名） |
| 7 | pages/index/index.vue | elem-chip | go('/pages/reactions/reactions') | 扩展：元素 chip |
| 8 | pages/stage/stage.vue | chapter-chip | switchChapter(ch.id) | 扩展：章节切换 chip |

每处开标签补：role="button"、tabindex="0"、@keydown.enter="<上表 handler>"。@click 原样保留。

## §3 不变量（须同批通过）

1. 8 处原 @click 入口全部保留（含守卫表达式 m.hint && go(m.hint)）。
2. 令牌三锚点不动：--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6。
3. tokens.css 与业务源码零 --r100- 令牌。
4. 排除项不被误改：mastery.vue .node（toggle(n)，含 isLocked 视觉态，产品裁定本轮排除）保持无 role/tabindex/@keydown.enter；enemies.vue .rxn-name（同 toggleEnemy(en.id)）保持无 role/@keydown.enter，避免与 .resist-toggle 双 Tab 停靠。

## §4 验证口径

- RED 命令（laiyipao/miniapp）：npx vitest run src/styles/r100ClickableView.contract.test.ts --reporter=verbose
- RED 实测：exit=1、Test Files 1 failed、Tests 8 failed | 4 passed (12)、v4.1.12、Duration 256ms；8 失败全为真实缺口（role="button" 缺失），4 不变量全通过，无 PARSE_ERROR / 模块解析错误。
- 逐字日志：docs/laiyipao/specs/round100-clickable-view/RED-evidence.md。
- GREEN 预期：12/12 全绿；实施后须复跑 R60 r60A11yKeyboard.contract.test.ts 与 R99 r99LinkKeyboard.contract.test.ts 无回退。
- 队长独立复跑：须 exit1 且『8 failed | 4 passed』缺口真失败，不变量通过。

## §5 诚实边界

- B 方向（aria-expanded 语义）队长记 R101 待办，本轮不纳入契约。
- 排除项：mastery .node 锁定态（isLocked）需确认键盘绕过守卫，产品裁定本轮排除；在途态/表单按钮 16 处（R96/R97-A 已记录）、遮罩/装饰 13 处、已修 9 处、view 版 AI CTA（需逐页核对）均不纳入。
- 行号定位：契约按 handler 表达式 + class 定位； SPEC 表中行号仅侦察快照（battles:186/index:115,209,233,274,287,303, enemies:102, stage:93），上游插行时以 handler 为准。
- 战报展开行浏览器未坐实（游客 0 战报 + mock 未注入），其缺陷判定为源码决定性，不冒充浏览器 PASS。
- 本规范仅交付契约与 RED，不含实施；未改任何业务源码，零真值入库。
