# R5 admin 四视图结构性升级审计（t48）

## 环境标注
- 截图来源：**非物理真机**（Windows Chrome headless via agent-browser 0.41.1）
- 视口：1258×588 桌面；访问地址：本地回环 127.0.0.1:5175（回环豁免，非真实域名）
- 会话 lyp-t48；登录 admin/admin12345（仓库公开开发默认值，非私密凭据）
- 采集方式：逐页 open → DOM 结构探针 + screenshot；errors=[] 全页无 pageerror/console.error

## 改造依据
ROUND5 设计审计 §1D 逐页映射：Skills / Users / Recomputes / Battles 四页此前无眉题分区、无双级 KPI、无智能空态、无可见 AI 元素（基线探针 kicker=0/kpiStrip=0/ai=0）。本轮四页统一落地六组件模式 + 可见 AI 洞察卡（规则引擎，非编造模型）。

## 前后肉眼差异
| 视图 | 改造前 | 改造后 |
|---|---|---|
| Skills 技能与配方 | 标题+说明→直接 toolbar+等权表 | AI 卡(侦察无人机热量效率 0.0 DPS)+眉题+2主2次 KPI+filter-bar+data-grid |
| Users 用户管理 | 拥挤 toolbar+9 列等权表 | AI 卡(无异常账号诚实结论)+眉题+KPI 带+分层筛选+数据网格 |
| Recomputes 结算审计 | 档位卡+告警+stat-row | AI 卡(第 1 关不符率 100%)+眉题+双级 KPI+filter-bar |
| Battles 战斗记录 | 本页速览摘要条 + 单 toolbar + 等权表 | AI 卡(1 场验真异常)+眉题+单一 KPI 指标带(去重)+两层筛选+data-grid |

## AI 卡四判据（四页全部满足）
- 可见汉字「AI」徽章 + 「AI 运营洞察」标题（ai-badge/ai-title）
- 结论来自当前页数据，确定性规则引擎推导（buildAdminAiInsight，无随机/无编造置信度）
- 依据行（ai-evidence）+ 数据截至说明；空数据走诚实空态「数据不足」
- 可点击 CTA（ai-cta，router-link 跳真实路由）
- 示例（battles）：headline「本页发现 1 场验真异常、0 场守恒可疑，建议优先复核」；evidence「验真异常＝重放哈希不符；守恒可疑＝击杀+漏怪超过该关总怪数」

## 实测证据（改造后 DOM 探针）
- skills：kicker=1/kpiStrip=1/kpiMain=2/kpiSub=2/filterBar=1/resultCount=1/dataGrid=1/ai=1，rows=42
- users：同结构全部=1/2/2，ai=1，rows=50
- recomputes：同结构，ai=1（第 1 关不符率 100%），rows=3
- battles：同结构，ai=1（1 场验真异常），rows=50，data-test=battle-summary 挂在 kpi-strip（无重复指标带）
- errors=[]（全四页）

## 发现并修复的真实缺陷
1. 指标带重复：BattlesView 原有 battle-summary 摘要条与新增 kpi-strip 显示同样 4 指标 → test-first RED（benchmark 新断言 kpi-strip 带 data-test=battle-summary 且无独立 battle-summary 容器）→ GREEN：合并到 kpi-strip、删旧块与孤立样式。55/55 相关测试通过。
2. t33 顶栏 API 脱敏未触碰、不回退（截图顶部显示回环地址，属回环豁免）。

## 门禁
- 全量 npm test：54 files / 650 tests 全绿
- npx vue-tsc --noEmit：EXIT 0
- npm run build：EXIT 0
- 新增契约：tests/R5FourViews.contract.test.ts（18）、tests/adminAiInsight.test.ts（5）

## 截图 sha256
- before-skills.png 2bd007555945158f160b0b77d20936670da4b579fd22930cc788206949020f07
- after-skills.png 71f8bc8bb719862fe0b281c38efc04b44d7abe39ac5da13fbdb4d9e1738b67dc
- before-users.png baa64d5d945cad714738e2681e35ee91173fe9d3ab932b78bf56979b10191d0b
- after-users.png 908f2a74d6fed08dcb46e723c81cacd69eefe81e255c11c4b68590085efda3e5
- before-recomputes.png 90340f79a6a75f1bb1f04da75738225d423c439cd19c69c088f6f43d138d8533
- after-recomputes.png fd847a4febc8669b7b543140cf4c919b2ee937aadf26c200694d77279a7538b2
- before-battles.png fa4ceafe7986d895c36254f532f9736f97722a9c2a34e16cc1be3a85962c5049
- after-battles.png 349a39a916c5ed5aa201f6599a7d63b0a844603714e2944b11181c8253d5c102