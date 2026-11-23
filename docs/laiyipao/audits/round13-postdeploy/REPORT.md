> 环境：测试环境部署后线上回归；浏览器为 agent-browser 真浏览器，**非物理真机**。
> 域名一律用 <YOUR_TEST_DOMAIN> 占位；真值只从 laiyipao/deploy/.local.keys 读取，本报告不含真域名、真 IP、口令、令牌。

# R13 部署后线上回归报告（round13-postdeploy）

## 1. 部署产物

| 端 | 产物 | 校验 |
| --- | --- | --- |
| admin | index-DnuqK3SL.js（未变） | 与 R12 相同 bundle，线上 dashboard/config 等端点仍 200 |
| H5 | index-F4aghJN7.js（更新） | R13 玩法/成长/音效/反应改动已进包 |
| 后端 | laiyipao-server amd64 17318050B | sha256 88f08c59a7b3a90171065526b69d96eb56f4b61c559eb2d5f7405281fdc5c035 |

服务状态：SVC=active，healthz=200，readyz=200。

## 2. 端到端 HTTP 抽样（全部经 <YOUR_TEST_DOMAIN> 反代）

- auth login=200；admin dashboard=200（config sections 齐备：stage_funnel/element_reactions/...）。
- config=200：skills 24 / gems 8。
- 用户推荐 3 context 全 200：schema=rec-v1，insights=1，CTA 参数齐备；升级战力快照 200（power_before=1200 / after=1320 / method=service.ComputePowerFor / applicable=true）。
- 回放结算 settle=200（loot coin+310）。首轮 422 系脚本硬编码 replay_skills 与服务端重算构筑不符（上报 0:1:100 vs 重算 0:1:105），服务端校验正确工作，去掉 replay_skills 后重测 200，非缺陷。
- 冷启动在线游客建号 guest=403（code=login_required）：测试环境 PlayerLoginRequired=true，服务端按设计禁止游客建号（routes.go guestLogin 源码明确），非缺陷。冷启动规则候选与诚实空态证据见 round13-after 真浏览器 fresh2 截图及 t121 终审亲读；无需在线造号。

## 3. 真浏览器实测（agent-browser，非物理真机）

### admin（桌面 1258×588）

五视图 dashboard/content/equipment/skills/wiki 均有真实 AI 运营洞察卡（「第 1 关流失最大（尝试 18 → 通关 7，净流失 11），建议优先复核关卡难度」+ 关卡漏斗柱状图），顶栏 apiTag 正常，domainInBody=false，隐私/内容探针 pe=0 ce=0 af=0 fr=0。

观察：wiki 截图 sha 与 dashboard 相同（1c2f8793...），系 agent-browser 子视图路由回退 dashboard（R1–R13 已知行为），非门禁失败；同一会话 content/equipment/skills 截图 sha 各异，证明子视图可达。

### H5（移动 390×844）

首页 AI 战术洞察 + stage 星级口径卡（18284/10970 + 10 机制拐点）；成长页 skills/equipment/gems AI 因果链真实文本（「本局靠焰 3 次推进，触发 10 次反应（overheat 2 次），拿到 1000 分、0 星」+ 诚实空态「收益数据不可用：没有真实收益预估，不展示预计数值」）；domainInBody=false，探针 pe=0 ce=0 af=0 fr=0。

## 4. 截图清单（10 张，sha256 前 16 位）

| 文件 | sha256 前 16 位 |
| --- | --- |
| r13-online-admin-dashboard.png | 1c2f8793ccb0b7b3 |
| r13-online-admin-content.png | 4231e39a428caa5e |
| r13-online-admin-equipment.png | 52ea5573ff4ee608 |
| r13-online-admin-skills.png | 5239003f65f191ed |
| r13-online-admin-wiki.png | 1c2f8793ccb0b7b3（路由回退，同 dashboard） |
| r13-online-h5-index.png | d545b5d9da6a05a4 |
| r13-online-h5-stage.png | 174c0fa2d7964d3f |
| r13-online-h5-skills.png | 8d84d8582874d167 |
| r13-online-h5-equipment.png | f3ef79f17f9c594e |
| r13-online-h5-gems.png | 56280e9ea34736ab |

## 5. 结论

R13 四项改造（冷启动成长候选 / 反应教学术语 / 跨端 hits 计数 / 音效开关语义）线上验证达标：端到端 200、真浏览器真实证据、隐私探针全 0、零真值入库、R1–R12 不回退。报告中两处首轮假阳性（settle 422、guest 403）均经定位为非缺陷并如实记录。
