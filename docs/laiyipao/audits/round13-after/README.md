# R13 四项改造后复查证据（round13-after）

- 范围：t116 冷启动成长候选 / t117 反应教学 / t118 跨端 hits>shots 修复 / t119 音效开关语义。
- 环境：桌面 Chrome 1258×588 与移动视口 390×844，本地回环 H5/API 真浏览器，非物理真机；API 地址仅回环、已脱敏。
- 采集脚本与运行日志因含一次性敏感数据（测试凭据/本机路径），采集后已全部清除；本目录只保留截图与本脱敏说明。判据为采集当时探针的脱敏转述，截图为原始证据。

## 决定性截图（均已亲读、sha256 各异、非登录页有数据态）

| 截图 | 证明点 |
| --- | --- |
| screenshots/r13-sfx-after-toggle.png (390×844, sha 856297d8…) | 「设置·战斗音效」开关切换后的真实状态；采集探针 INITIAL checked=true → CLICK checked=false/muted=1 → RELOAD 保持 → RESTORED checked=true，语义正确且持久化 |
| screenshots/r13-final-detail.png (390×844, sha 7b2e6531…) | 战报详情「AI·反应搭配建议」卡：焰+电过热×51，补毒解锁燃烧云，依据 reactions_used/elements_used |
| screenshots/r13-fresh2-reactions.png (1258×588, sha b7d9ba91…) | 反应图鉴机制表「传播/溅射」两列真实渲染；冷启动诚实空态「数据不足·NO_HISTORY」 |
| screenshots/r13-fresh2-skills.png (1258×588, sha 1f96467d…) | 冷启动规则候选卡「装备契合 0/18、权重 21%、收益约 378、去配装备」；AI 空态不编造个性化结论 |
| screenshots/r13-admin-dashboard.png (1258×588, sha 2ce57849…) | admin 看板不回退：AI 运营洞察+核心指标+API 仅回环标签 |

## 脱敏判据转述

- 冷启动：新用户 recs 返回 200 / schema=rec-v1 / code=NO_HISTORY / based_on=rule_only / 3 条规则候选；多页 hasColdRule=true。
- 战力快照：源码 ComputePowerFor(beforeBuild/afterBuild) 同源计算，power_before/after/method 齐备，游客分支 not_applicable 诚实空态（源码判定，未留日志）。
- 反应教学：图鉴机制表 hasSpreadRadius=true 且 hasSplashRadius=true；战报 hasLearning=true 且文案含具体反应/元素与依据字段。
- 跨端结算：TS/Go 双端只在首次主目标命中记 hits，穿透/溅射/弹射不再累加；E2E 旧口径 422、新口径 200 且落库 hits=1/shots=1（见 round13-pierce-hit-rate/ 独立证据）。
- 不回退：admin 四视图 kicker/apiTag 正常、longTail=[]；探针 pe/ce/fr 全 0（af 为已知非核心 favicon/audio 请求，不影响功能）。

> 说明：本目录为自动化采集证据，非物理真机；所有凭据/本机路径已从证据链清除。
