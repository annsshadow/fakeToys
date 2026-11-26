# R6 admin 五视图补 AI 与数据驱动推荐 — 真浏览器审计

## 环境标注
- 非物理真机：Windows Chrome headless（agent-browser 0.41.1）
- 本地回环：http://127.0.0.1:5175/laiyipao-admin/（dev server），后端 127.0.0.1:8080（仓库公开 dev 默认凭证，非真值）
- 桌面视口：1258x588（headless 默认）
- 截图方式：before 为注入 CSS 隐藏 AI 元素后截取（本轮唯一新增可见内容即 AI 卡/提示，其余结构与改造后一致），after 为真实改造后页面。before 是近似基线，非改前源码快照。

## 三重确认（每页 after 探针）
| 视图 | hash | title | AI卡 | KPI带 | 表格行 |
|---|---|---|---|---|---|
| content | #/admin/content | 公告与兑换码 | 1 | 1 | 88 |
| defenses | #/admin/defenses | 防线值守 | 1 | 1 | 0（暂无设置，空态可见） |
| equipment | #/admin/equipment | 装备宝石皮肤 | 1 | 1 | 32 |
| wiki | #/wiki | 玩家玩法文档站 | 1 | 0（文档站无KPI带，符合预期） | 124 |
| login | #/admin/login | 登录 | role=note AI提示可见 | - | - |

## 肉眼差异
- content/defenses/equipment/wiki：after 在 page-header 下、KPI 带上方新增紫色微光 AI 洞察卡（§1E），含可见「AI 运营洞察」徽章、规则引擎标注、真实数据结论、依据行、CTA 按钮；before 无此卡。
- login：after 表单下方新增 role=note 的 AI 提示（登录后可在各业务页查看规则引擎洞察；本页不收集玩家数据）；before 无。
- 数据真实性：content 结论「当前配置版本 6237e0b6…8b8b，反应条目 7 类」来自 AdminDashboard 配置字段；equipment「玩家平均战力 1，平均关卡 0.0」来自 Progression；均为确定性本地规则，无编造置信度。

## AI 四判据
1. 可见汉字 AI：卡片 AI 徽章 + 「AI 运营洞察」标题 ✓
2. 当前页数据经确定性规则结论：buildLocalRecommendation(content/defenses/equipment/wiki) 从真实 Dashboard 字段派生 ✓
3. 依据：卡片 evidence 行复用真实字段名与数值 ✓
4. 可点 CTA：router-link 跳真实路由（/admin/content|defenses|equipment|levels 等）✓
- 接口优先：fetchAdminRecommendation(scope) 命中后端 GET /admin/recommendations/<scope>（soft:true），失败回退本地规则引擎并标注「本地规则引擎」，不用 0.93 之类编造置信度。

## 门禁
- npm test: 57 files / 673 tests all passed
- npx vue-tsc --noEmit: EXIT 0
- npm run build: EXIT 0（built in 1.38s）
- 既有 r6PersonalizedRec.contract.test.ts (7例) 未改，随全量通过

## 截图 hash (sha256)
- before-login.png: 0a04c499d51d5a57d0446bf605acca657e7ea552580dac0e4dcd7f78d0d7d12c
- after-login.png: 1f7901a350e3854d8f319d9140759d51f776c9c8e4c6faadd73f1998e7843eaf
- before-content.png: 02c20efdba18e96db95c1a52a3b2a541c7e760c2240cf18bed3edad675af7af5
- after-content.png: 282a556388d1cf467235942d1ac7eecb2850703f5f1ba559ec0671e1c73f8d4b
- before-defenses.png: 3fb54da8b0e9d01ab36a432f71b184627112a6e3f954d47583336fb267c042e5
- after-defenses.png: 5e6a83644c00d9ed8a5abcfcbac1f876fbf0e01b00b2491352e8fb0282ea3d47
- before-equipment.png: 1af9c2c2395ac17eb0637670dbaedc8c1c09c709ece806da67a31bfcc947f8f2
- after-equipment.png: e2062df0c9fd4d45f25ab3f3fff0c058bea29cc0c2624d03d5d2bade66d34a08
- before-wiki.png: d92b055335acdb966832382803f4fea1d0a1d2b7ca6380f2f7b88292da1de23f
- after-wiki.png: 7310499bc097e4a76c852cc65addc09d00d79203a68c90a5d5388c184a0bd57a

## 安全
- 截图与文档仅含回环地址，无真域名/IP/口令/私钥；API 标签由 t33 脱敏逻辑渲染为回环豁免值。