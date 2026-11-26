# R8 关卡系统与难度曲线深度实测（t79）

- 日期：2026-11-22
- 环境：本地回环 H5（http://127.0.0.1:5174/）+ 本地 API（http://127.0.0.1:8080），真浏览器自动化
- 声明：所有截图均来自本地回环 H5 真浏览器自动化，**非物理真机**；无生产域名、密钥或真实环境信息入仓。
- 任务：从新玩家视角逐关核查目标清晰度、难度曲线、敌人辨识度、卡关/碾压与首关时长，并用 test-first 修复写域内的真实缺陷。

## 1. 样本与方法

1. 真实 API 返回 **100 关**（任务假设的 8 关不成立）、6 章、22 种敌人。
2. 通过浏览器内带本地会话凭证的 fetch 分 10 块读取 /api/v1/config 与 /api/v1/levels/:id；一次性请求百关会触发解析/内存异常，分块后可重放。
3. 每章抽样 + 边界关（1/2/3/56/57/60/61/97/98/99/100）读 DOM，并在浏览器控制台断言关键字段。
4. 第 1 关从头到结算手动走完 8× 五波流程，记录游戏内时长与结算归因。

## 2. 关卡与难度曲线事实

- 章节：1 边境哨站（1–20，油桶）；2 工业废墟（21–40，崩塌掩体）；3 冻原（41–60，蓄能塔，boss 16）；4 感染矿井（61–80，潮汐闸）；5 母巢外围（81–95，旋转风障，boss 19）；6 母巢核心（96–100，蓄能塔，boss 22）。
- 难度从 1000‰ 线性升到 4200‰，同章每关 +33‰ 为主；波数 5→12，总敌人数 32→84，单关敌种 2→15；boss 关为 56/60/91/95/96/100。
- 边界样本：L98 防线 12000、11 波、难度 4135‰、体力 11、元素上限 8、蓄能塔、1★20187/2★28599/3★32973；L99 难度 4167‰、1★49124/max 81874；L100 boss、难度 4200‰、12 波、1★31386/max 52311。
- 星级/满分口径**非单调**：1★门槛下降 47 次、max_score 下降 44 次，例如 L97 39132/65220 → L98 20187/33646 → L99 49124/81874 → L100 31386/52311；1★/max 全程约 0.60。玩家会看到“难度更高、门槛反而更低”，UI 没有任何解释。

## 3. 真实缺陷与 RED→GREEN 修复

### 缺陷 1：地形关索引重复计数

- 证据：L98 页面 DOM 逐字出现「蓄能塔 第 22、34、41、42、43、53、57、59、59、62、82、96 关 等 14 关」「崩塌掩体 第 3、15、21、22、22、23、33、34、34、42、63、83 关」「潮汐闸 第 53、61、61、62、63、63、68、97 关」。
- 根因：浏览器内读配置证实同关可配两个同类地形（id22/id34=[collapse_wall,charge_tower,collapse_wall]、id59=[charge_tower,oil_drum,charge_tower]、id61/id63=[tidal_gate,rotor_vane,tidal_gate]、id82=[rotor_vane,charge_tower,rotor_vane]）；旧 terrainIndex 用 Map<string, number[]> 直接 push，重复关号虚高总数。
- RED：stage.test.ts 新测试逐字失败 "AssertionError: expected 崩塌掩体第 22、22 关 to contain 第 22 关"。
- GREEN：stage.vue:367 改为 Map<string, Set<number>>，ids.add(l.id) 后排序输出，总数按唯一关卡计。
- 复测 DOM：蓄能塔「第 22、34、41、42、43、53、57、59、62、82、96、97 关 等 13 关」，其余地形无重复。
- 截图：[r8-01-terrain-index-deduped.png](screenshots/r8-01-terrain-index-deduped.png)（1258×588，非物理真机/本地回环）。

### 缺陷 2：敌人组成预览看不出威胁类型

- 证据：L98 DOM 只有「电鳍兽 ×4」「铁罐炮 ×3」「浮空囊 ×6」「索敌眼 ×2」「蜂巢母 ×7」，玩家无法知道浮空囊=飞行、铁罐炮/蜂巢母=远程。
- 根因：stage.vue:252-256 只渲染 enemyName × count，enemyNameMap 只映射 id→name；而 game/types.ts:19 EnemyDef 已含 fly_height / burrow / attack_range / is_boss。
- RED：新测试先因漏 import 报 ReferenceError: makeEnemy is not defined，补 import 后业务失败 "AssertionError: expected false to be true"。
- GREEN：stage.vue import EnemyDef，新增 enemyDefMap 与 enemyThreatTag(id)（优先级 boss→飞行→潜地→远程），模板加 .threat-tag。
- 复测 DOM：出现「电鳍兽 ×4[远程]」「浮空囊 ×6[飞行]」「铁罐炮 ×2[远程]」「蜂巢母 ×7[远程]」等。
- 截图：[r8-02-enemy-threat-tags.png](screenshots/r8-02-enemy-threat-tags.png)（1258×588，非物理真机/本地回环）。

## 4. 第 1 关完整实战

- 入口：首页主行动卡→关卡页→「开始战斗（消耗 6 体力）」；8× chip 实测 speed-chip on。
- 每波选卡遮罩真实点击「全部不要了，直接下一波」；注意泛化 view/text/div/button 文本选择器会命中不可点击父元素，必须定位 .btn。
- 结算 DOM：通关/C、击杀 39、漏怪 0、反应 73、分数 11056、0 星；服务端钳分「75s 内得分上限 19708，实得 11056；距 1 星门槛 11941 还差 885」；金币 +1190、体力 +5、hash 7b2b6e051fc73f8c。
- 首关时长：游戏内 75s（服务端钳分日志给出），8× 下五波清完；真实自动化墙钟包含失败选择器循环，不作为首关时长依据。
- 截图：[r8-03-level1-result.png](screenshots/r8-03-level1-result.png)（1258×588，非物理真机/本地回环）。

## 5. 最值得优先升级的 3 个玩法缺口

1. **星级目标感与满分口径脱钩**：100 关中 1★门槛下降 47 次、max_score 下降 44 次，L97→L98 从 39132/65220 掉到 20187/33646。门槛像随满分缩放而非随难度提升；需要在关卡页解释“按本关满分百分比给星”或改为单调递增目标，否则玩家会觉得高关更容易拿星。
2. **难度曲线缺少机制拐点**：难度几乎固定 +33‰/关，阶段跳变全是 +33 并列，r5AiStage 选首个并列即当“本章最大跳变”。应在章节切换或特定关引入 boss/飞行/潜地/远程的机制拐点，并在难度曲线上标注，而不是只堆数值。
3. **时间预算与得分归因滞后**：第 1 关 8× 仍用 75s，满分 19708 被钳到 11056，玩家结算后才知道“75s 内上限”。R7 已做实时星级差额，但还没有实时时间预算/钳分预警；应在 HUD 显示“剩余 Xs 保分窗口”并在临近截止时给即时提示。

## 6. 验证

- 定向：stage.test.ts 21/21 通过；RED 两条均先失败后转绿。
- 全量：npm test exit 0，175 files passed | 1 skipped（176），1672 tests passed | 2 skipped（1674）。
- 类型：npm run type-check（vue-tsc）与 npm run type-check:test（tsc）均 exit 0。
- 浏览器：agent-browser errors --json 全程 errors=[]。
- 写域：仅 src/pages/stage/stage.vue、src/pages/stage/stage.test.ts 与本审计文档；未执行 git，未触碰 admin/后端。
