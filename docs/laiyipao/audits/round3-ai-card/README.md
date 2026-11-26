# t31 审计：AI 洞察卡零次反应诚实文案

## 环境与口径

- 环境：**非物理真机**（Windows Chrome headless via agent-browser 0.41.1），本地回环 127.0.0.1:5176。
- 后端：go run ./cmd/api，本地回环 :8080（dev 默认 admin 账号，仓库公开默认值，非真实密钥）。
- 零数据态构造：用 agent-browser network route 拦截 dashboard 响应，注入 reaction_usage 全部 count=0、stage_funnel 空数组、daily_active 3 天的 Dashboard，触发 buildAiInsight 规则2 零次分支。
- 无任何真实域名/IP/口令/私钥入库；文档按仓库口径用 <YOUR_TEST_DOMAIN> 占位。

## 缺陷（R3 终审 t24 low backlog）

buildAiInsight 规则2 在 reaction_usage 非空但所有 count=0 时，仍输出「最高频元素反应（触发 0 次）」——没有任何触发就谈不上「最高频」，是自相矛盾的误导文案。

## 修复（test-first RED → GREEN）

- RED：tests/aiInsight.test.ts 新增零次场景断言（kind=reaction / 不含「最高频」「触发 0 次」/ 含「无触发记录」与埋点建议 / CTA=/admin/skills）+ 非零不退化守卫。运行得 1 failed / 8 passed，失败原文 expected 最高频文案 not to contain 最高频。
- GREEN：src/utils/aiInsight.ts 规则2 加 top.count<=0 守卫，headline 改为「当前无触发记录，建议检查埋点或引导玩家使用反应」，保留 evidence / ctaPath=/admin/skills / asOf。复跑 9/9 passed。

## 真浏览器实测（非物理真机）

- 零次数据态：.ai-insight-card 存在，headline=当前无触发记录，建议检查埋点或引导玩家使用反应；highfreq=false、zero=false；source=规则引擎 · 数据截至 2026-01-03；CTA「查看反应配方」保留；errors=[]。
- 对照：修复前同数据态 headline 为「蒸汽爆发是最高频元素反应（触发 0 次）」（误导），修复后改为诚实表述。

## 截图

| 文件 | sha256 | 说明 |
| --- | --- | --- |
| screenshots/zero-count-honest.png | 3db56cca45fff4e0c21889baf60fa69666bb91bf0f74c87d9f9f3ad6932548ed | 零次数据态 AI 卡诚实文案（1258x588） |

## 门禁

- npm test：48 files / 582 tests 全绿。
- npx vue-tsc --noEmit：EXIT 0。
- npm run build：EXIT 0（built in 1.17s）。

## 写域

- laiyipao/admin/src/utils/aiInsight.ts（修复）
- laiyipao/admin/tests/aiInsight.test.ts（RED 测试）
- docs/laiyipao/audits/round3-ai-card/**（审计与截图）