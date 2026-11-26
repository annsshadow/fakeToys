# Round8 H5 实测审计：推荐一键落地 · 收益对比与两步确认

## 采集口径

- 环境：本地回环 http://127.0.0.1:5174（H5 dev）+ http://127.0.0.1:8080（Go API），真实浏览器 agent-browser 390×844 移动视口。
- 标注：**非物理真机**（桌面浏览器模拟 H5 视口），不是 iOS/Android 真机。
- 页面：skills（技能图鉴）、equipment（装备图鉴）。

## before / after 截图差异

| 页面 | before | after | 可见变化 |
| --- | --- | --- | --- |
| skills | before-skills.png | after-skills.png / after-skills-modal.png | 新增 R7「真实收益预估 · 当前 vs 推荐」四区卡（当前 / 推荐双条对比、历史三星率说明、依据与规则引擎标注）、两步确认弹层（取消 / 确认替换）；数据缺失时显示诚实空态「收益数据不可用」，无虚构数字。 |
| equipment | before-equipment.png | after-equipment.png / after-equipment-modal.png | 同上；护甲 ‰ 与每波伤害双条对比，替换仅对当前已装备且槽位匹配的装备生效。 |

## 接口驱动与诚实空态

- 收益来自真实接口 GET /api/v1/me/recommendations/impact?kind=...&id=...&replace=...；rec-v1 envelope；空历史返回 empty_reason.code=NO_HISTORY / NOT_ENOUGH_DATA / CONFIG_MISSING。
- 三星率说明明确写「历史三星率（stars=3 场数占比，不是胜率）」；star_probability_estimate 恒 null，不推测概率。
- 实测 lyp_token 下接口返回 recStatus=200、insights=[]、NO_HISTORY、battle_count=0；页面呈现合法空态与「收益数据不可用」文案，不用本地兜底冒充真实预估。
- 本地规则兜底 status='local' 时也显示「本地规则引擎没有真实推荐，不调用收益预估，不展示虚构数值」。

## 当前 vs 推荐 · 两步确认

1. 首击「应用推荐 →」仅打开对比弹层（r7OpenCompare），不写出战配置。
2. 弹层展示「当前 → 推荐」逐项差异；只有点击「确认替换」（r7ConfirmApply）才写：技能用 store.persistLoadout，装备用 applyEquipmentImpact 整组替换并刷新。
3. 失败保留原配置并显示真实错误与「重试」。

## 契约结果

- r7ImpactApply.contract.test.ts 15/15 绿；r7GameplayFeedback.contract.test.ts 5/5 绿（合计 20/20）。
- 全量 miniapp test：175 files passed | 1 skipped，1670 tests passed | 2 skipped；type-check exit 0；build:h5 DONE。

## 截图 sha256

- before-skills.png cf41f88c21b9ca770b1e8c93ed8d01e9b5a9c0682e8e07de3a471392f963285d
- after-skills.png 2b69c98072add9bad7c213c59263dfda0a8289fb7fe4fa245b63bd8f3a70b011
- after-skills-modal.png 11dc310383d8bdbba4d8b8e55e40a19105686f8fb2c2e3bb976d15419829d837
- before-equipment.png 07ef05e871fc77242f3a9eee9eb5221c2f2217cdbdc9bf5b03cb01df03e93648
- after-equipment.png ffa7c5fd46f191e4164a3687b19a26c65268dcc00f43f25e71faee8c305191a3
- after-equipment-modal.png ffa7c5fd46f191e4164a3687b19a26c65268dcc00f43f25e71faee8c305191a3
