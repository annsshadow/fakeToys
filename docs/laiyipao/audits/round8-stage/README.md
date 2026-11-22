# R8 关卡页星级口径与机制拐点（t83）

- 日期：2026-11-22
- 环境：本地回环 H5（http://127.0.0.1:5174/#/pages/stage/stage?level=98）+ 本地 API（http://127.0.0.1:8080）
- 声明：截图来自真浏览器自动化的 H5 页面，**非物理真机**；390×844 移动视口；均为本地回环环境，未包含真实域名/IP、口令或密钥。
- 写域：laiyipao/miniapp/src/pages/stage/stage.vue 与 stage.test.ts，以及本审计文档；未触碰 battle.vue / game / 后端 / admin；未执行 git。

## 1. 缺口① 星级口径诚实解释

R8 真浏览器审计（round8-gameplay）发现 100 关中 1★ 门槛下降 47 次、max_score 下降 44 次，L97 39132/65220 → L98 20187/33646，玩家会误以为高关更容易。

关卡详情卡上方新增 .star-rule-card：同时展示配置下发的「本关满分 max_score」「1★门槛 star_targets[0]（约满分的 N%）」以及难度 ‰、波次、元素上限；说明文案写明「按本关满分百分比给星；关卡难度仍逐级上升，星级门槛按本关满分缩放，因此不同关卡的绝对分数线会升降，不代表高关更容易」。N 由 star_targets[0] / max_score 计算，只用于解释，不参与星级判定；无任何硬编码具体关卡数字（如 11941/20187/33646）。L98 实测显示「本关满分 33646 / 1★门槛 20187（约满分的60%）/ 难度 4135‰ / 波次 11 / 元素上限 8」。

## 2. 缺口② 难度曲线机制拐点

难度曲线从纯折线升级为「机制拐点」视图（after-L98-detail.png 肉眼可见）：

- 曲线点按 is_boss / 当前关敌种 id / terrain 分别着色；图上以 ◆/▲/■ 三种形状标注首领、本关敌种、地形变化，配「为什么这里变难」文字：首领机制、飞行/潜地/远程威胁与克制建议、地形机制说明。图例为「◆ 首领 ▲ 本关敌种 ■ 地形」。
- 诚实口径修正：服务端 /config 不下发每关 waves，detailWaves 只属于当前选中关。首版误把当前关所有敌种伪造成整章每关的「新敌种首现」（截图暴露十几个重复标注在 L96），这是新的假洞察；已修正为只在当前关标注真实敌种（去重），文字写「本关敌种」而非「首现」，并加回归测试断言敌种标记只出现在当前关。
- 地形机制取 TERRAIN_DESCR；无机制拐点时显示「本章难度平稳上升」。
- r5AiStage 不再把首个并列 +33‰ 说成「本章最大跳变」；改为优先找当前关之后的机制拐点，结论写明拐点类型与原因，依据行写真实字段（关卡曲线/地形/星数 + is_boss / 敌种 id / 地形 id），CTA 指向对应关卡或装备成长页；完全没有机制拐点时诚实写平稳上升。

## 3. test-first RED→GREEN

新增 stage.test.ts 用例：星级口径说明卡从配置读取数值并解释非单调；机制拐点标注与 r5AiStage 禁止假洞察；敌种标记不得跨关伪造。先 RED：2 failed / 21 passed（缺 .star-rule-card、缺 .diff-milestone），实现后 GREEN 23/23。

t79 的两条守卫测试完整保留并通过：R8 地形关索引同类地形按关卡去重、敌人组成预览标注飞行/潜地/远程/首领威胁。

## 4. 真浏览器 DOM 探针证据

在 L98 页面内执行 DOM 断言（curve-dom-probe.json，eval --stdin）：

| 探针 | 实测值 |
| --- | --- |
| .diff-milestone 数量 | 17（>0，机制拐点确实渲染） |
| 图例 .diff-legend-item | ["◆ 首领","▲ 本关敌种","■ 地形"] |
| 形状 .diff-shape | ["◆","▲","■"] |
| 首条标记 | ◆ 第 96 关 · 首领关 / 首领血量与技能机制改变打法 |
| 地形标记 | ■ 第 98 关 · 地形变化 蓄能塔 / 蓄满后给全场敌人上同种元素（翻盘机制） |
| 敌种标记归属 | 10 条 ▲ 本关敌种 全部在第 98 关，无跨关伪造 |

## 5. 验证与截图

- 定向：pnpm vitest run --config vitest.config.ts src/pages/stage/stage.test.ts → 23 passed。
- 全量：pnpm test → 176 files passed | 1 skipped，1683 tests passed | 2 skipped。
- 类型：pnpm type-check（vue-tsc --noEmit）exit 0。
- 构建：pnpm build:h5 → DONE Build complete。
- 浏览器控制台：errors.json / errors-l98.json errors=[]。

截图（390×844，非物理真机，本地回环；三张场景图内容不同、sha256 各异）：

| 文件 | 说明 | sha256 |
| --- | --- | --- |
| after-L98-chapter1.png | L1 首屏：AI 推进建议改为机制拐点说明 + 星级口径卡露出 | 30194e609dc6a2dc480db23e42c517ef6e41c9c2ba96595984b170817b03a8d6 |
| after-L98-top.png | L98 首屏：第 98 关 hero + AI 机制拐点建议 + 星级口径卡顶部 | 50f3f746e301b8f9dd31f50de01185b6ecead0e51aceac119c6640bd121b779a |
| after-L98-detail.png | L98 难度曲线卡：◆/▲/■ 图例、折线标记、机制拐点文字列表、星级口径卡完整数值 | 13fda0826eeb360f161e2f17c3fda8daf0fc4e178a0e7cc4c1e313cb9d633a5b |
| after-L98-curve.png | L98 修复后首屏：AI 建议已改为「本关敌种」口径，不再伪造新敌种首现 | 91f197c6aa4d144c83d7bd95cb7707c6e9d53b9c1e4e70563aa08cc6ba9b215c |
| after-L98-scrollend.png | L98 详情底部：敌人组成威胁标签 + 星级门槛（t79 守卫仍生效） | 025c7f1fb42db70b209390b2a7ff97e882a97243721f229c5f6c8cc0db7b190a |

before 对照：round8-gameplay README 已记录同一 L98 页面只有 1★20187/max33646 裸门槛和 +33‰ 假洞察；本次 after 截图可见口径卡、机制拐点折线图与图例、威胁标签，肉眼可辨。
