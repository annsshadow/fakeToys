# R6-BUG-1 修复审计：admin 推荐合法空态误回退本地兜底

## 环境标注
- 非物理真机：Windows Chrome headless（agent-browser 0.41.1）
- 本地回环：http://127.0.0.1:5175/laiyipao-admin/；后端 127.0.0.1:8080（仓库公开 dev 默认凭证，非真值）
- 桌面视口：1258x588

## 缺陷与根因
ContentView/DefensesView/EquipmentView 三视图在推荐接口返回 HTTP 200 / rec-v1 / empty_reason.code=NO_HISTORY / insights=[] 的合法空态时，旧 insightFromRecommendation() 仅凭 insights.length===0 判定为无数据，回退到 buildLocalRecommendation 并显示「本地规则引擎」——把后端的诚实空态误报成规则引擎兜底。

## 修复（test-first RED→GREEN）
- RED: tests/recommendation.contract.test.ts 追加3例（NO_HISTORY evidence 不含本地规则引擎/含 NO_HISTORY 与 message；空态文案与 H5 口径一致；真网络失败 null 才标注本地且合法有数据不误伤）。首轮 2 failed，失败原文 AssertionError: expected '本地规则引擎' not to contain '本地规则引擎'。
- GREEN: src/utils/recommendation.ts insightFromRecommendation() 顶部先判 resp?.empty_reason?.code，命中已知 code（NO_HISTORY/NOT_ENOUGH_DATA/CONFIG_MISSING，文案与 H5 r6Recommendation.ts 对齐）即返回 {kind:'empty',headline:'AI 数据不足：暂时没有足够数据给出建议',evidence:'数据不足 · <code>；缺什么：<detail>；下一步：…',asOf}，ctaLabel/ctaPath 留空；仅在真正失败（resp=null）时回退本地。12/12 passed。

## 真浏览器验证（fetch patch 注入响应，三重确认 hash/title/AI卡）
| 视图 | 合法空态(headline) | 真网络失败(回退,evidence含本地规则引擎) |
|---|---|---|
| content #/admin/content 公告与兑换码 | AI 数据不足：暂时没有足够数据给出建议 | 已发布 38 条公告…（本地规则引擎 · 公告 38 条 / 兑换码 50 张）|
| defenses #/admin/defenses 防线值守 | AI 数据不足：暂时没有足够数据给出建议 | 数据不足：暂无防线记录，无法定位薄弱防线（本地规则诚实地说明数据缺失）|
| equipment #/admin/equipment 装备宝石皮肤 | AI 数据不足：暂时没有足够数据给出建议 | 装备覆盖 5/5 元素…（本地规则引擎 · 装备 18 件 / 宝石 8 颗 / 皮肤 6 款）|
- 截图：empty-*.png 三张（合法空态）与 fallback-*.png 三张（真失败回退对照）。合法空态卡无 CTA、无「本地规则引擎」标注；真失败回退卡保留完整洞察与诚实标注。
- 注入方式：eval 注入 window.fetch patch，拦截 /api/v1/admin/recommendations/*；mode=empty 返回 HTTP200 + rec-v1 + empty_reason + insights=[]；mode=fail 返回 rejected Promise 模拟真网络失败。切 hash 触发各视图 load()（keep-alive onActivated 亦可）。

## 门禁
- npm test: 57 files / 676 tests all passed（含 t54 的 4 视图 AI 四判据与 r6PersonalizedRec.contract.test.ts 7例）
- npx vue-tsc --noEmit: EXIT 0
- npm run build: EXIT 0（built in 1.04s）
- t33 顶栏脱敏未回退；未碰 H5/后端（H5 仅作为文案口径参照）。

## 截图 hash (sha256)
- empty-content.png: 87a818aede8ac7a132b20bbb438417f03cf3f34562d7ec77597bef87e11752e3
- empty-defenses.png: 3c7a847415e772b7c268e6d3aa9851ca2248e498fae28aac5817b426a09abbfa
- empty-equipment.png: 39dd94694ec05249fe883f778022977c334e1c7409b6145e568ac610caa973ed
- fallback-content.png: 34d276fbc3982dbbb1322ad8712e0f7e452a7ad24d434ecb7c244d14c53ef4f4
- fallback-defenses.png: 5378a91bbba618683a3ba2fe0e78ba33dc9773ff17c9a9e3ccc0bca738de6bd8
- fallback-equipment.png: a3f37308822bdf7037555da0cd9db1aaa25fe623003e33a74ec32b65fd8d7a65

## 安全
- 截图/文档仅含回环地址与仓库公开 dev 默认凭证，无真域名/IP/口令/私钥。