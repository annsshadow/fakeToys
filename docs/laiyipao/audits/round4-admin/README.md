# t36 审计：R4 admin 四视图 §1C.2 标杆级升级

## 环境与口径

- 环境：**非物理真机**（Windows Chrome headless via agent-browser 0.41.1），桌面视口 1258x588，本地回环 127.0.0.1:5175。
- 后端：go run ./cmd/api，本地回环 :8080（dev 默认 admin 账号，仓库公开默认值，非真实密钥）。
- 登录成功校验：URL 目标视图 + document.title 含「来一炮 · 运营后台」+ 结构探针非 0（防登录页假阳性）。
- 无真实域名/IP/口令/私钥入库；文档按仓库口径用 <YOUR_TEST_DOMAIN> 占位。

## 依据与设计规范

docs/laiyipao/design/AI_VISUAL_SYSTEM.md §1C.2：ContentView/EconomyView/DefensesView/EquipmentView 四视图按 SectionKicker → KpiStrip → FilterBar → DataGrid → EmptyState 顺序升级，不新增令牌（v1/v3 双锚点保持）。

## 改造内容

- 四视图均加：中文眉题+英文小标（section-kicker）、两级指标带（kpi-strip 主 KPI 2-4 张 + 次卡 1-2 张）、filter-bar + result-count（aria-live=polite）、data-grid 表容器（tabular-nums 数字）、empty-state 智能空态（role=status + 原因说明 + 动作按钮）。
- EconomyView 加可见 AI 洞察卡：AI 徽章 + 「AI 回收压力诊断 · 规则引擎」+ 基于当前页 flows delta 的净流入结论（如「金币净流入 +3,750，回收压力上升」）+ 依据行（delta 汇总口径）+ CTA「查看订单筛选」（只读角色可用）；无流水走诚实空态。
- 复用 R3 已落地的共享样式类，未引入第二套体系；低透明度底纹/辉光不计入 AI 感。

## AI 四判据（EconomyView 实测）

- 可见汉字「AI」：badge=AI，标题含 AI。
- 真实数据结论：headline=金币净流入 +3,750，回收压力上升，建议关注回收渠道（来自当前页 flows.delta 汇总）。
- 依据：evidence=当前页货币流水的 delta 汇总（总量列）；净流入为正意味着该货币在增发。
- 可行动 CTA：button「查看订单筛选」→ 滚动定位订单筛选区。

## 真浏览器实测（非物理真机）

- 改造后 DOM 探针四页全非 0：kicker=1 / kickerEn=1 / kpi=1 / kpiMain=2~4 / kpiSub=1~2 / filterBar=1 / resultCount=1 / grid=1~3；economy.ai=1；defenses rows=0 时 empty=1（诚实空态）；errors=[]。
- 肉眼差异：before 为标题 + 拥挤 toolbar + 等权密集表的传统 EP 后台；after 为眉题分区 + 紫光 AI 卡 + 主次 KPI 带 + 分层筛选，信息层级明确可辨。

## 截图（sha256）

| 文件 | sha256 |
| --- | --- |
| screenshots/before-content.png | d749e816178a57c61c5438de29357727a760530cafa6a975f2499f82dbe28607 |
| screenshots/after-content.png | 9b111d61e0fb8dd3db8a5575e37227d1a00ac029b3a74c59282e90544acfe18f |
| screenshots/before-economy.png | 4a94a6a24203240ff14deffc37db58f3c817bfae5fe3590f265b6111dc33eca6 |
| screenshots/after-economy.png | 444ffdfe32db2150eca8b8a92c149fe8ad7cb857180f87d69abd5f745c663087 |
| screenshots/before-defenses.png | 45dbbbeda2a08960af9908cb8ba20c379abe2bce532ac4cfd1110affff4956db |
| screenshots/after-defenses.png | d109c10a37068151327b5e9ab029c870f99109822811d2374b5cbba722ac4804 |
| screenshots/before-equipment.png | f0d3bd4d370911b4ccf60118fdbbeed3c449ce0d97dbb8fc457a7e04033c41b4 |
| screenshots/after-equipment.png | 9f82b7655db651433eabc72927b854f70f63a11e38f903ed5298346eba19506a |

## 测试与门禁

- RED：tests/R4FourViews.contract.test.ts 新建，22 failed（源码级 20 + DOM AI 2）；GREEN：23/23 通过。
- 四视图既有回归 105/105；全量 npm test 51 files / 619 tests 全绿；npx vue-tsc --noEmit EXIT 0；npm run build EXIT 0。
- 反模式守卫：filter-bar 源码禁裸 color:var(--accent)（对比度红线）。

## 写域

- laiyipao/admin/src/views/{ContentView,EconomyView,DefensesView,EquipmentView}.vue
- laiyipao/admin/tests/R4FourViews.contract.test.ts
- docs/laiyipao/audits/round4-admin/**