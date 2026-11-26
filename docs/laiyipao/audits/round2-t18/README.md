# t18 F-10-3 admin 标杆级重构 · 真浏览器前后对比

## 环境标注（合规）

- **非物理真机**：Windows Chrome headless（agent-browser 0.41.1），非真实移动设备。
- 访问地址：本地回环 `http://127.0.0.1:5175/laiyipao-admin/`（测试环境前端）；API 本地回环 `:8080`。
- 登录：使用仓库 dev 默认凭证（仓库公开默认值，非真实密钥）。
- 截图均为 1258×588 PNG，采集于 2026-11-21，浏览器控制台 errors=[]（无 pageerror / console.error）。

## 改造目标（F-10-3）

审计依据 `docs/laiyipao/reviews/ROUND1_DESIGN_REVIEW.md` F-10-3：admin 看板/战报为传统 Element Plus 观感，
要求 1-2 个标杆级重构（对标 Linear 信息密度 / Vercel 层次克制），附真浏览器前后对比图。

## 看板（dashboard）前后差异

| 维度 | 改造前 | 改造后 |
| --- | --- | --- |
| 信息层级 | 8 张等权灰卡平铺，无主次 | 3 段 eyebrow 分区（核心指标 / 经济与防线 / 趋势与分布）+ CSS Grid 层级栅格 |
| 核心 KPI | 与次级卡同字号同权重 | `.kpi-card--hero` 左侧橙强调条 + 30px 数值；次级卡 20px 收敛 |
| 图表区 | 3 张等权并排 | 主图 `.chart-primary` 满宽（活跃趋势），反应/漏斗并排 |
| AI 元素 | 无 | 保留 t14 AI 洞察卡（规则引擎，紫色 halo），位于 KPI 之前 |

DOM 断言：`kpiPrimary=4 / kpiSecondary=4 / sectionKicker=3 / chartPrimary=1 / charts=3 / aiCard=1`。

## 战报（battles）前后差异

| 维度 | 改造前 | 改造后 |
| --- | --- | --- |
| 打开第一眼 | 9 控件挤两行 toolbar + 16 列等权密集表 | 本页速览指标带（本页记录/胜率/验真异常/守恒可疑）+ 表格 |
| 筛选动线 | 单一 toolbar 混杂 | `.filter-bar` 两层 `.filter-row`：主筛选（谁/哪关/结果）与范围筛选（日期/验真/计数）间有分隔线 |
| 关键列 | 结果与 13 列等权 | 结果/一致性/验真加 `col-emphasis`（单元格字重 600） |
| 数据契约 | — | 速览从当前页 50 条确定性派生（胜率 72.0%），不新增接口；15 列全部保留 |

DOM 断言：`summary=4 / filterBar=1 / filterRows=2 / cols=15 / rows=50`。

## 截图清单（sha256）

| 文件 | sha256 |
| --- | --- |
| before-01-dashboard.png | b6dabf8a903383fa08c7e7337bba086d20eac2a6f2d224db3f23c1ca19e555c3 |
| after-01-dashboard.png | 144e0ec639fd5b553d622f9073dc1bae96438355ac9dcf6bc3e76e08ae396860 |
| before-02-battles.png | 53e96507f97d888bb1e6397daa6dbbea0a2386b8e69087869979080736e14828 |
| after-02-battles.png | 3551fbd7a89fb9f3f169a7d786b0ff3454b6610256971548baa57ddae4f18361 |

## 门禁

- `tests/DashboardView.benchmark.test.ts` 3/3、`tests/BattlesView.benchmark.test.ts` 3/3（RED→GREEN）。
- admin 全量 `npm test`：47 files / 555 tests 全绿；`vue-tsc --noEmit` EXIT 0；`npm run build` EXIT 0。
- F-1 零战报兜底 / 命令面板 10 项导航 / F-10-4 图表令牌化 / t14 AI 洞察卡均未回退（既有测试全绿佐证）。
