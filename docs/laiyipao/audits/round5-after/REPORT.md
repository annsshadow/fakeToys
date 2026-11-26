# t49 · Round5 真浏览器复查与部署准备报告

- 任务：t49「R5真浏览器复查与部署准备」（QA，只读复查，依赖 t46 玩法实测 / t47 H5五页 / t48 admin四视图）
- 时间：2026-11-22（Asia/Shanghai）
- 环境：本地回环 QA（admin `http://127.0.0.1:5175/laiyipao-admin/#/`、H5 `http://127.0.0.1:5174/#/`、API `http://127.0.0.1:8080` healthz/readyz=200）；**Round5 尚未部署测试环境，本报告为部署前本地复查，非线上回归**。
- 页面性质：真实浏览器自动化截取，**非物理真机**（Chrome headless，admin 1258×588 / H5 390×844 视口）。
- 真值纪律：本地回环 QA 无真实域名/口令入库；admin 顶栏 `API 127.0.0.1:8080` 为回环地址（t33 设计的回环可见分支，非回环显示纯 API），扫描本地 keys 文件的域名/口令 0 命中（NO_LEAK）。

## 0. 结论速览

| 验收锚点 | 结论 |
|---|---|
| ① H5 五页（announcements/login/signin/stage/verify）+ admin 四视图（Skills/Users/Recomputes/Battles）URL/title/结构三重确认 + 探针全0 | **PASS** |
| ② AI 四判据逐页核对 + 诚实空态 §1A.4 | **PASS** — 有数据的页四判据齐；空态页给诚实「空态+下一步引导」 |
| ③ index 攻击力诚实文案 + 0星文案 + t41 AoE 25% 不回退 | **PASS** |
| ④ 11 张前后/当前截图 + 报告落盘、标注非物理真机 | **PASS** |
| ⑤ 真值扫描 0 命中 | **PASS** |
| ⑥ 不改源码 / 不 git | **PASS** |

**本轮未发现新真实缺陷。** 3 项采集口径假阳性已在 §4 澄清。

## 1. H5 五页（§1D 升级页，DOM 结构证据）

| 页 | title | 登录态 | 眉题 | KPI | AI 令牌 | 空态/内容 | 探针 |
|---|---|---|---|---|---|---|---|
| announcements | 公告 | 游客✓非login | 1「公告与版本」 | 0（空态页） | AI | 📢暂无公告 + AI「去看战报→」 | 0/0/0/0 |
| login | 来一炮 | 登录页本身 | 1「欢迎回来」 | 0 | AI | 轻量 role=note | 0/0/0/0 |
| signin | 签到 | ✓非login | 1「每日签到」 | 1 | AI | AI领取提醒+去任务CTA+签到网格 | 0/0/0/0 |
| stage | 关卡 | ✓非login | 1「作战推进」 | 1 | AI | AI推进建议+难度曲线+开始战斗 | 0/0/0/0 |
| verify | 战绩验真 | ✓非login | 1「战斗验真」 | 0 | AI | 轻量 role=note | 0/0/0/0 |

- 五页 `probe{pe:0,ce:0,af:0,fr:0}`，URL/title/眉题三重确认（login 页 isLogin=true 是页面性质本身，非假阳性缺陷）。
- 诚实空态（§1A.4）：announcements 显示「近30天没有公告，无法从版本记录推导更新影响→去看战报」，不伪造内容；截图目视确认。
- 读图：`r5-h5-announcements.png`、`r5-h5-signin.png`、`r5-h5-stage.png` 肉眼可见眉题+KPI带+AI卡+数据容器，无白屏。

## 2. admin 四视图（§1D 升级页）

| 视图 | title | 登录态 | 眉题 | KPI带/卡 | FilterBar | 行 | AI 卡四判据 | 探针 |
|---|---|---|---|---|---|---|---|---|
| skills | 技能与配方 | ✓非login | 技能配置·元素与配方 | 1/4 | 1 | 42 | AI审计洞察：侦察无人机热量效率最低+依据42技能/18配方+复核技能数值CTA | 0/0/0/0 |
| users | 用户管理 | ✓非login | 用户管理·玩家档案 | 1/4 | 1 | 7 | 无高战力低关卡异常账号+依据本页7名用户+查看用户CTA | 0/0/0/0 |
| recomputes | 结算重算审计 | ✓非login | 结算审计·重算与不符 | 1/4 | 1 | 28 | 无关卡级不符统计+依据总账关卡分布空+查看审计CTA | 0/0/0/0 |
| battles | 战斗记录与验真 | ✓非login | 战斗记录·验真与守恒审查 | 1/4 | 3 | 0(空态) | 数据不足无法识别异常对局+依据需至少1条战报+查看战报CTA | 0/0/0/0 |

- 四视图 `isLogin=false`、`kicker=1`、`kpiStrip=1`、`kpiCards=4`、`aiCard=1`，探针全0；台头顶栏仅「admin/API/登出信息」。
- AI 四判据（可见AI+真实数据结论+依据+CTA）四页齐全；空态页（battles/recomputes）给诚实结论+依据+引导，符合 §1A.4。
- t48 修复的 battles 指标带重复（battle-summary 与 kpi-strip 合并）复查：`kpiStrip=1` 单一指标带，未回退。
- 读图：`r5-admin-skills.png`、`r5-admin-users.png`、`r5-admin-battles.png` 肉眼可见眉题+双级KPI+FilterBar+AI卡，无白屏。

## 3. 诚实文案与不回退核验（锚点③）

- **index 攻击力口径**（t47 RED→GREEN）：`falseClaimNone=true`（无「与面板攻击力无关/无关攻击力」）、`honestContribution=true`，实读文案「反应伤害中元素侧是主力，面板攻击力也有贡献但占比受限（权重上限 30%，具体以 damage.ts 实测为准）——低养成靠元素搭配仍有翻盘空间」（`data/copy.log`）。源码 `index.vue:299` 与本轮 DOM 一致。
- **0 星结算文案**（t46 RED→GREEN）：源码 `battle.vue:395-411` 按服务端 `star_targets` 动态算下一星门槛与差额（如「还差 N 分就能拿 X 星（门槛 Y 分）」），仅 stars≥3 或门槛拿满才出「冲3星」；`battle.test.ts:460` 钉死「0星1★门槛11941不得空喊冲3星」。本轮源码核验未回退（t46 真浏览器复测证据见 docs/laiyipao/audits/round5-gameplay/）。
- **t41 AoE 25% 不回退**：真浏览器 reactions 页 `pct25=5`、`radius120=4`、蒸汽/过热反应文案均可见（`data/copy.log`）；截图 `r5-h5-reactions-aoe.png`。

## 4. 假阳性澄清（诚实记录）

1. H5 登录：采集首步游客态 `hasToken=false`（首页游客登录），后续页 token 建立；五页均正常渲染，非缺陷。
2. index 文案首轮 eval 报错「SyntaxError: Invalid regular expression: missing /」：脚本 heredoc 内正则 `\n` 转义问题，改写安全 JS 后重跑成功（`data/copy.log`），采集工具假阳性，非产品缺陷。
3. admin 顶栏 `API 127.0.0.1:8080`：本地回环 QA 的 t33 设计行为（仅非回环脱敏为纯 API），非真域名泄露。

## 5. 基线对照（队长补充要求）

- t47 的 before 基线因跨工作树认证/API 状态差异采到登录页而无效（已如实记录）。本轮采用当前态 DOM 结构证据 + `docs/laiyipao/audits/baseline-round1/screenshots/`（Round1 早期基线）+ 读图对照：
- H5 五页当前态均有 `section-kicker` 眉题（Round1 基线页面无此结构）、signin/stage 有 KPI 带与 AI 卡、空态页有 AI 线索——新增结构肉眼可辨。
- admin 四视图当前态均有 `section-kicker`+`kpi-strip`(4卡)+`ai-insight-card`，Round1 基线为旧版表格页——眉题/KPI/AI 卡肉眼可辨。

## 6. 产物清单（docs/laiyipao/audits/round5-after/）

- `REPORT.md`（本文件）、`probe-init.js`
- 脚本：`capture-h5.sh`、`capture-admin.sh`、`verify-copy.sh`、`capture-extra.sh`
- 数据：`data/{h5.log,admin.log,copy.log}`
- 截图 11 张：`screenshots/r5-h5-{announcements,login,signin,stage,verify,index-honest,reactions-aoe}.png` + `screenshots/r5-admin-{skills,users,recomputes,battles}.png`，均标注非物理真机。

（Round5 尚未部署测试环境；本任务不改源码/不 git，部署由 release-ops 后续负责。）