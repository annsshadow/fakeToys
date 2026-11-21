# t28 · R3 admin 四核心视图标杆级升级（审计留档）

> 截图环境：**非物理真机** · Windows Chrome headless（agent-browser 0.41.1） · 桌面视口 1258×588 · 本地回环 127.0.0.1:5175。未使用真实域名 / 真机。
> 登录态已确认（URL=#/admin/<view>、title 正确、.page=1、结构探针非 0），非登录页假阳性。

## 改造依据

- 任务：t28「R3 admin 四核心视图标杆级升级」，attempt db3d2f34-47f7-4841-b0e7-c614e0fef901。
- 规范：`docs/laiyipao/design/AI_VISUAL_SYSTEM.md` §1B 六组件模式（SectionKicker / KpiStrip / EmptyState / DataGrid / FilterBar / ProgressTree）+ §1B.4 可见 AI 元素四判据。
- 契约：`laiyipao/admin/tests/R3SixComponents.contract.test.ts`（源码级 20 + DOM 级 5，RED→GREEN）。

## 四视图前后差异（肉眼评判）

| 视图 | 改造前 | 改造后 |
|---|---|---|
| 关卡配置 | 标题+说明+单行 toolbar+等权 12 列密集表 | 新增「关卡配置·关卡总览」眉题 + **紫光 AI 洞察卡**（真实结论「P107响应一致通关率0.0%当前页最低，建议复核波次难度」+依据+「查看该关波次」CTA）+ 2 主 2 次 KPI 带（关卡总数 100 / 章节 6…）+ filter-bar/result-count + data-grid + 智能空态 |
| 技能与配方 | 标题+说明+元素 radio 挤一行+等权 13 列表 | 眉题「技能配置·元素与配方」+ 2 主 2 次 KPI 带（技能数/配方数/元素数/当前筛选）+ filter-bar + data-grid + 智能空态 |
| 用户管理 | 标题+说明+9 控件挤两行+等权 9 列表 | 眉题「用户管理·玩家档案」+ KPI 带（总数 753/本页 50/页 1/16）+ 两层 filter-bar + result-count + data-grid + 智能空态 |
| 结算重算审计 | 档位卡+告警+标题+stat-row+等权表 | 眉题「结算审计·重算与不符」+ 2 主 2 次 KPI 带（窗口重算 2/不符数 2/不符率 100%/档位 enforce）+ filter-bar/result-count + data-grid + 智能空态 |

## 可见 AI 元素（§1B.4 四判据）

- 载体：关卡配置页 `.ai-insight` 卡（after-levels.png 可见紫色光晕 + AI 徽章）。
- ① 可见汉字 AI：徽章「AI」+ 标题「AI 关卡洞察」。
- ② 当前页数据经确定性规则结论：从 `levels` 中取 clear_rate 最低关，输出「{name}通关率 {rate}% 为当前页最低，建议优先复核该关波次难度」——真浏览器实测显示「P107响应一致通关率 0.0% 当前页最低」（真实数据驱动，非写死）。
- ③ 依据：`依据当前页 N 关的通关率分布推算 · 规则引擎 · 基于当前数据`。
- ④ 可点 CTA：「查看该关波次」→ 真浏览器点击实测打开 `.el-drawer`「P107响应一致 · 波次编排」（5 行波次）。
- 空数据走智能空态「数据不足：当前页没有关卡数据…」，不编造结论；诚实标注规则引擎，未宣称 GPT / 置信度。

## 交互实测（真浏览器）

- 空态：关卡页搜索 `__NO_SUCH_LEVEL__` → `.empty-state`（role=status, aria-live=polite, title「没有匹配的关卡」, action「清空筛选」）；点击清空筛选 → 恢复 rows=105，kw 清空。
- 结构探针（4 页均非 0）：section-kicker=1/kicker-en=1/kpi-strip=1/kpi-main=2/kpi-sub-card=2/filter-bar=1/result-count=1/data-grid=1。
- 控制台 `errors=[]`（agent-browser errors --json）。

## 门禁

- RED→GREEN：新契约 24 failed → 25 passed（RED 阶段仅对比度反模式守卫通过；GREEN 全绿）。
- `npm test`：48 files / 580 tests passed（含新增 25 用例，既有四视图 120 用例全绿）。
- `npx vue-tsc --noEmit`：EXIT 0。
- `npm run build`：EXIT 0（built in 1.17s）。
- 不回退：F-1 零战报兜底 / 命令面板 10 项 / 图表令牌化 / AI 洞察卡（DashboardView 既有测试全绿）。

## 截图清单（sha256）

| 文件 | sha256 |
|---|---|
| screenshots/before-levels.png | d879a4d0c75c510982ed0fd1c5d8d50f9208d4ea6227378422b32362bc5220fd |
| screenshots/after-levels.png | 3e22fb1d9709d46f09f68aba1fdbd32ae35948ceb9fa7a52beda5aebbefb6467 |
| screenshots/before-skills.png | 926be9fa12073258209d672185839d1814f60abbbac4a37ee0ef36f5351c46d8 |
| screenshots/after-skills.png | e23a3853b57019758869ff7bdddf862a959c75c905e8dafb4a0a11a842cb7bbc |
| screenshots/before-users.png | 7cc461cfa85ffb7b3dfe7f6660af4c80be6e389f8519ea54e33d69e8fa82d201 |
| screenshots/after-users.png | 81181d5ef3dc03aba444043a6504d523ff04ab25b40ab31dde936884d1038f2b |
| screenshots/before-recomputes.png | 41a3062e240293b89adc7eac80e0d797848fc4a089f6c02567604933e2255c02 |
| screenshots/after-recomputes.png | bd086ead4b531c4a9357138c558a6741b3dc673904541ab04e5291f4acb5a378 |

全程未执行 git；无真域名 / IP / 口令 / 私钥入库或日志（API 标签为本地回环开发默认值）。