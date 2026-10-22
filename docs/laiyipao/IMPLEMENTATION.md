# 来一炮 · 已实现内容与验证状态

> **2026-10-20 自 [`laiyipao/README.md`](../../laiyipao/README.md) 拆出。**
> 定位：当前实现规模、平衡实测数据、验证状态与测试方法论复盘。
> 本文档的数字有测量工具，**不要凭印象改** —— 改完请重测。
>
> **引用约定**：本仓代码注释里的「README 已实现内容」即本文；
> `laiyipao/README.md` 的「已实现内容」小节是本文的转发锚。
> 相关文档：工程约束见 [`ENGINEERING.md`](./ENGINEERING.md)，
> 未做事项见 [`KNOWN_LIMITS.md`](./KNOWN_LIMITS.md)。

---

## 已实现内容

### 内容规模

| 类别 | 数量 |
|---|---|
| 敌人 | 21（含 105 条五维抗性） |
| 技能 | 24 基础 + 18 合成 = 42 |
| 合成配方 | 18 |
| 装备 / 宝石 / 皮肤 | 18 / 8 / 6 |
| 专精节点 | 96（8 系 × 3 层 × 4 选 2） |
| 关卡 / 波次 | 100 / 800 |
| 地形 | 5 类，25 关使用（`terrain.test.ts` 31 例逐类断言可触发） |
| 签到 / 任务 / 商城 | 7 天 / 12 个 / 8 项 |

难度全局单调 1000‰ → 4200‰，无章节断崖。所有 BOSS 抗性矩阵**必有解**（如白垩母皇冰+500/电+500，但毒−500/动能−500）。

### 平衡基准（改动数值前后必跑 `balance.probe.test.ts` 对照）

内容表缩放 `EnemyHpScale = 8` / `SkillProjectileScale = 4` 之后，第 1 关实测：

| 指标 | 缩放前 | 缩放后 |
|---|---|---|
| 命中率 `hits/shots` | 15% | **82%** |
| 通关时长 | 338s（且失败） | **194s** |
| 元素反应次数 | 29 | **100** |
| 1/2/3 技能构筑 | 全败 | 全败（1/2/3 技能都打不过） |
| 4 技能构筑 | 勉强赢 | 194s 通关 3 星 |

技能数梯度是**清晰**的：1/2/3 技能都打不过，4 技能才赢 ——
说明热量机制（I-2）重新成为真实约束，而不是摆设。

跨关卡（`difficulty.test.ts`）：

| 关卡 | 1 | 10 | 25 | 50 | 75 | 100 |
|---|---|---|---|---|---|---|
| 耗时 | 194s | 181s | 228s | 285s | 318s | **594s** |
| 结局 | won | won | won | won | won | won |
| 命中率 | 82% | 86% | 88% | 85% | 87% | 91% |

第 100 关 / 第 1 关 = **3.06 倍**。

> ⚠️ **刻意不**断言"逐关严格递增"：第 1 关 194s、第 10 关 181s（关 10 敌人更少但更快），
> 局部非单调是正常的。守的是全程趋势 + 新手关能赢这两条粗边界。
> 逐关调优未做，见 [`KNOWN_LIMITS.md`](./KNOWN_LIMITS.md) 第 6 条。

### 攻方加成对关卡的实际影响（全 100 关，三个档位）

uild 快照下发完整攻方属性后，量了一下「升级到底带来什么」：

| 指标 | 裸装 | 中配 | 满配（全部封顶） |
|---|---|---|---|
| 通关时长中位数 | 215s | 184s | **158s** |
| 逐关掉分 | — | — | 无（容差 2%） |
| 逐关掉星 | — | — | **无** |

各成长方向的实测效应（100 关、可比口径）：

| 方向 | 反应次数 | 分数 | 结论 |
|---|---|---|---|
| 攻击力 +3000‰ | **-25.1%** | +0.6% | **买不到分数却压制反应** ⇒ 封顶砍到 1000‰ |
| 元素系数 +1200‰ | -6.2% | +0.1% | 中性点，封顶定在这里 |
| 热量上限 +1000‰ | +2.0% | -1.5% | 有利（单位修正后） |
| 防线护甲 +750‰ | 0% | 0% | 对漏怪**数量**无影响（减的是伤害不是数量） |
| 机制卡强度 +1000‰ | 0% | 0% | 需真的取牌才可观测 |

> 「攻击力买到分数却压制反应」这一条是本轮最反直觉的发现：
> 击杀数几乎不随攻击力变（5262 → 5275），说明**战斗瓶颈是热量/冷却周期而非伤害**，
> 而 I-1 的核心创新是元素反应 —— 堆面板伤害与设计意图直接冲突。

### I-4 地形在真实对局里的生效率

单元测试只证明"机制能工作"，**不证明"它在真实对局里会被触发"**。
探针跑全 100 关，按**可观测的实际效果**统计：

| 地形 | 放置 | 生效 | 生效率 |
|---|---|---|---|
| 油桶 oil_drum | 6 | 2 | 33% |
| 潮汐闸 tidal_gate | 10 | 10 | **100%** |
| 旋转风障 rotor_vane | 6 | 6 | **100%** |
| 崩塌掩体 collapse_wall | 9 | 9 | **100%** |
| 蓄能塔 charge_tower | 15 | 15 | **100%** |

这一节的血泪是**度量口径换了五次，前四次都是观测错了** ——
详见 [`ENGINEERING.md`](./ENGINEERING.md) 约束 11。修掉的三个真实缺陷：

1. **地形 y 范围与敌人走廊几乎不重叠**（levelgen）：
   地形 y ∈ [380,660] vs 敌人 y ∈ [600,940]，重叠仅 16%
   → 火焰弹丸瞄着敌人飞，而油桶在弹道之外，充能恒为 0
2. **风障作用半径 140 太小**：弹道很窄，6 个风障的作用范围内
   **一次弹丸都没进过**（偏转计数恒为 0）→ 半径改 260
3. **油桶阈值够不着**：y 修正后充能峰值 11~32 而 param 是 22~68
   → 阈值降到 8~23，生效率 17% → 33%

### 端

- **server**：config / domain / battle / power / seeder / httpapi / service / apitypes / tygen / store 全部实现，`gofmt` 无差异，`go vet` 干净
- **miniapp**：12 页面（首页/关卡/战斗/背包/专精/签到/排行/验真/我的/商城/shop/反应图鉴/reactions/装备相关）+ TS 战斗内核 + Canvas 渲染层；战斗页 1/2/4/8× 倍速档位（2026-10-22，只加速墙钟、不改 `TICK_MS`，见 KNOWN_LIMITS 第 17 条）
- **admin**：12 视图（Dashboard / Levels / Skills / Equipment / Defenses / Economy / Battles / Users / Wiki / Recomputes / Content / Login）+ 玩法文档站（目录 / 全站搜索 / URL 深链 `?tab=&anchor=&focus=`）；登录守卫、审计日志、货币净流入监控、胜率异常告警

### 这一轮的复盘：「存在」不等于「有意义」，而「有意义」必须能被机器查出来（第 150 轮）

这一轮把「消费面」从**人肉记账**变成**可执行不变量**。起因是一次穷尽扫描：

| 层 | 缺口 | 后果 |
|---|---|---|
| **A 端点** | `POST /mastery/reset`（第 74 轮就有）、`GET /levels/:id` | 前者让**点亮的专精节点永远无法取消**（`AllocateMastery` 只插不删），后者零调用 |
| **B 字段** | `/config` 的 17 个字段里 **5 个零消费**：`version` / `recipes` / `gem_qualities` / `skins` / `rating_weights` | 运营改了它们，游戏里**看不到任何反应**，且不报任何错 |
| **B 字段** | `TokenBalance` 这个 service 方法**没有任何 HTTP 端点读它** | `mastery_reset` 买得到、看不见、也点不动 |
| **C 道具** | `revive_token` / `gem_wash_token` 买得到但无消费端点 | 商城里两件「买了不知道干嘛」的商品 |

修完之后加了两条守卫，让这件事**不再依赖谁记得看一眼**。
A 口径（服务端侧，每条路由都要有客户端调用方）、B 口径（客户端侧，
`/config` 每个字段都要有消费方，或有写明理由的豁免）与 C 口径
（每种上架道具必须要么有 `spendToken` 扣减点、要么登记为待接通并写明前置条件）分别是：

```bash
go test ./internal/httpapi/ -run TestEveryRouteHasClientCaller
go test ./internal/service/ -run TestEveryWalletTokenIsSpendableOrRegistered
pnpm vitest run src/test/contract_coverage.test.ts
```

两条守卫的设计要点，都是被变异测试逼出来的：

- **豁免必须写具体理由**（≥40 字，且锚点可核对）。写「暂时没空」等于把缺口永久合法化，
  而没有任何测试会再提醒 —— `TestEveryExemptionIsJustified` 专治这一条。
- **匹配必须带终止符**。第一版只判断「路径片段出现在客户端源码里」，
  于是 `/redeem` 的调用方 `'/redeem'` 会**顺带**满足 `/redeem/never-called`：
  **把路由改成任何更深的路径，守卫照样绿**。变异实测确认后才补上终止符。
  那是这条守卫最坏的失效方式：看起来在守着，实际什么也没守。
- **扫源码而不是扫测试**。测试里的 URL 拼装不算「有人用这个端点」——
  那恰好是「写了测试但线上没人调」最常见的伪装。

#### 顺带拆掉的性能债：配置包砍掉 56%

实测 `/config` 未压缩 167,558 字节，其中 `levels` 占 123,712（74%），
而 `waves` 占 `levels` 的 **75.4%**（93,338 字节）—— 整个配置包 **56%**
是「100 关的刷怪排布」。

逐个消费点核过，客户端**只有一处**真的读它：防线挑战用第 1 关当标准战场
（`me.vue` 的 `challengeLevel()`）。其余全部只读元数据：
战斗页的关卡来自 `/battle/token`，验真页来自 `/battle/:id/replay`。

于是列表走摘要（`domain.LevelSummary`），波次按需取 `GET /levels/:id` ——
那个端点本来就存在（与 `/config` 同一生成器输出），此前零消费。

**为什么是「单独的类型」而不是 `json:"waves,omitempty"`**：
`omitempty` 会影响**所有**下发路径，包括 `/battle/token` 与 `/battle/:id/replay`
—— 那两处恰恰必须有 waves。手写摘要类型会有字段漂移风险，用
`TestLevelSummaryCoversEveryGeneratedLevelField`（反射逐字段对拍）钉死。
变异实测：往 `GeneratedLevel` 单向加一个字段，守卫当场红。

客户端侧同样拆成 `LevelSummary` / `GeneratedLevel extends LevelSummary` 两个类型。
**写成一个 `waves?:` 可选字段是不行的** —— 那样引擎里三处 `cfg.level.waves`
的读取会因「忘了判空」在运行时炸；类型层面分开，**编译期**就会把
「谁需要完整关卡」逼出来（`stage.vue` 就是这样被立刻指出来的）。

### 同一轮的复盘：「测试全绿」与「测试有效」之间的距离

19. **每周一必红的既有断言** —— `periodStart(now,"weekly")` 与
    `periodStart(now,"daily")` 在**周一完全相等**，而
    `weekly_task_concurrency_test.go` 断言「daily 那一行不存在」，
    而它自己插入的 weekly 行正好被那条 SQL 命中。于是 `dailyRows != 0`
    是**必然成立**的，与代码有没有写死 daily 无关。
    修法不是删断言，而是**只在两者不等时**判定（周一那条确实观察不到），
    并新增 `TestPeriodStartWeeklyNeverCollapsesToDaily` 逐日钉住这个前提。
    README 原先记录的「全绿」实测日 10-10 是周六。

20. **孤儿测试库把整套 DB 用例拖垮（累计性故障）** —— `docs/laiyipao/audits/coverage-2026-09-28.md`
    §6 记过「曾残留 1049 个孤儿库」，当时只做了一次性清理，**没留下机制**，于是必然复发。
    本轮实测 406 个残留 → `CREATE DATABASE` 超时 → 种子 `INSERT` 撞主键（SQLSTATE 23505）
    → 几十个用例红。根因有三条，每条都在代码里留了痕：

    | 根因 | 旧写法 | 症状 |
    |---|---|---|
    | 库名每秒重复 | `time.Now().UnixNano()%1_000_000_000` —— 取的是**最后 9 位**（秒以下），每秒绕回一次 | 两个用例拿到同名库，一个的 `Cleanup` 把另一个正在用的库删掉 |
    | DROP 超时留孤儿 | `t.Cleanup` 里 10 秒预算，库一多就超时 | 孤儿累积 → 下次跑测在资源耗尽处失败 |
    | 建库失败静默跳过 | 任何 CREATE 失败都 `t.Skipf("无权创建测试库")` | **「测试全绿」与「一半用例根本没跑」无法区分** |

    修法：抽出 `internal/testutil/scratchdb`（原先 5 个包各抄一份），
    库名用 `前缀 + PID + 进程内计数器`，各包 `TestMain` 在跑测试前按**自己的前缀**清理
    （只清自己是因为 `go test ./...` 并行跑多个包，跨前缀清理会删掉别人正在用的库），
    CREATE 失败**分类**处理 —— 确实没权限才 Skip，其余一律 `Fatal`。
    另有三条结构性守卫：`TestNoScratchNameUsesTimeModulo`（扫源码禁掉时间取模）、
    `TestEveryScratchPrefixIsRegistered`（漏登记就在本轮红）、
    `TestNameIsUniqueWithinProcess`。

    > 这条与「消费面」同源：**它坏的方式都是「让测试看起来比实际更可信」**。

### 同一轮的复盘：`version` 从「每秒都变的时间戳」改成内容哈希

`/config` 的 `version` 曾经是 `int(time.Now().Unix() % 100000)`，于是：

1. 它**从不稳定**，客户端无法据此判断「内容变没变」；
2. 客户端**根本不读它** —— `loadConfig()` 只有 `if (config.value) return` 这个
   内存内缓存，`config.version` 在全仓库**零引用**；
3. 配置里还有 `server_time: time.Now()`，两次响应本来就不相同。

也就是说「版本没变就不用刷新」这个判断**从来没被实现过**。
代码注释里早就写明了要哪三件事，但只记了事实、没做实现 ——
而**只做内容哈希这一件是「只写不读」**，还会给最热的端点加上「marshal 167KB + 哈希」的每请求 CPU。

现在三件一起做了：

- `version` = 静态内容的 SHA-256 前 8 字节（易变字段取零值，天然被排除）；
- **静态部分进程内只组装一次**（原来每个请求都重跑 `GenerateAllLevels()`，纯浪费）；
- 客户端把配置**持久化**到 storage，冷启动先用本地副本点亮界面，
  再带 `?if_version=` 问服务端 —— 内容没变时只回 **304，零载荷**。

实测：`/api/v1/config` 无参 200 / **167,558 字节**；带同 version **304 / 0 字节**。

#### 为什么用查询参数而不是 `If-None-Match` / ETag

两条都是真实原因：

1. **自定义请求头会触发 CORS 预检**。H5 端的 cors 中间件目前只放行
   `Content-Type,Authorization`；加一个头就得同步改预检白名单，
   而忘改的症状是「整批请求发不出去」，很难和网络故障区分。
2. **ETag 会让浏览器自作主张缓存**。响应一旦带上 ETag，H5 的 XHR 会拿到
   浏览器缓存里的旧响应体 —— 里面的 `server_time` 是上一次拉取时的时间，
   而 `estimateServerNowMs()` 会把它当成「服务端当前时间」。
   不设 ETag 则两端行为一致：每次都真的走到服务端。

304 没有响应体，但客户端仍要校准本地时钟 —— 用 HTTP 标准 `Date` 响应头。
**不能「304 就不校准」**：那会让时钟偏移随着缓存存在时间一起漂移，
症状是**过期的护盾显示为生效中**（或反过来），且只在老玩家身上复现。

### 验证状态

| 项 | 状态 |
|---|---|
| Go 测试 | **13 个含测试包全绿**（2026-10-21 三端优化收尾复测：Go 1.26.9 + PostgreSQL 17；此前 2026-10-13 实测为 12 包 / Go 1.26.3 / PostgreSQL 18.6，当时快照保留）。`internal/service` 全套约 163s；孤儿库残留 **0 个**；开发库测试不再写入 |
| TS 测试 | miniapp **1092** 通过 + 2 跳过（`i6.e2e.test.ts` 需 `LYP_E2E=1` + 运行中的服务端）· admin **321** 全绿（详见 `docs/plans/2026-10-21-laiyipao-three-end-optimization.md` 收尾核验；数字为 2026-10-22 加倍速档位 + 文档站目录/搜索/深链后的复测值，`--no-file-parallelism` 串行跑 —— 并行跑在 Windows 上偶发丢测试文件，只报少不报红） |
| `go vet` / `gofmt -l` | 干净 |
| `vue-tsc`（admin + miniapp） | 0 error |
| `e2e.ps1` 19 步 | 全通过 |
| **I-6 闭环端到端** | **通过** —— 引擎重放哈希 == 服务端记录；篡改种子必被证伪 |
| 构建 | miniapp H5 ✓ / mp-weixin ✓ / admin ✓ |
| 工具链 pin 守卫 | `miniapp/src/test/toolchain_pin.test.ts` —— vite 必须锁 `@dcloudio/vite-plugin-uni` 的 peer 硬锁 `5.2.8`、vitest 留 `^2.1.9`。dependabot 两次自动升级（→6.4.3→8.3.4）曾把 `npm install`（ERESOLVE）/ `type-check`（TS2307）/ `build:h5`（Rolldown 解析不了 vue-demi）同时打坏，此守卫让漂移直接变红 |
| **全 100 关** | **won 100 / lost 0，零死锁、零难度回归**，平均 280s、最长 594s |

> ⚠️ **这一行原本写着「2026-10-10 实测：621 个测试函数全部执行、零跳过」。**
> 2026-10-12 复查时发现该结论**只在非周一成立** ——
> `weekly_task_concurrency_test.go` 有一条按星期几相关的断言，
> 周一必然红，与代码对错无关（详见 [`ENGINEERING.md`](./ENGINEERING.md) 约束 19）。10-10 是周六，所以那天确实是绿的。
> 这正是「测试全绿」与「测试有效」之间的距离：**绿的那天和红的那些天可能都是对的**。

验证 I-6 闭环（需先起服务）：

```bash
cd server && go run ./cmd/api
cd miniapp && LYP_E2E=1 pnpm vitest run --config vitest.config.ts src/game/i6.e2e.test.ts
```

### 跨端契约文件由生成器产出，不要手工编辑

```bash
cd server && go run ./cmd/vectors          # 重新生成
cd server && go run ./cmd/vectors -check   # 只校验是否最新（CI 用）
```

产出三个文件：

| 文件 | 内容 | 谁断言 |
|---|---|---|
| `testdata/reaction_specs.json` | 反应链数值表（7 条） | 客户端 `reaction_contract.test.ts` |
| `testdata/smoke_levels.json` | **全部 100 关**真实关卡 + 用到的敌人 + 全部技能 | 客户端 `engine.smoke.test.ts`、`difficulty.test.ts` |
| `testdata/level_seeds.json` | 100 关种子表 | 客户端 `elements.test.ts`（对字面量断言） |

三条规则：

1. **只导数据，不导期望值。** 期望值写在 `formula_vectors.json` 里人工推导 —— 用实现生成期望值就是自证，测试会永远绿。
2. **Go 侧不读回自己的导出做自证。** 跨端锁在客户端：改了 TS 侧副本，客户端测试红；改了 Go 侧，`-check` 红。
3. **不写 UTF-8 BOM。** `encoding/json` 拒绝 BOM 而 Vite 会吃掉它 —— 同一个文件在两端被解析成不同内容，是最难查的一类故障。（真实发生过：所有 Go 一致性测试突然集体失败。）

### 端到端冒烟测试为什么必须用真实内容表

`engine.smoke.test.ts` 跑的是**从 Go 真相源导出的真实第 1 关**。这不是为了"更真实"，是因为历史上所有引擎测试都用手工构造的关卡，结果两个致命缺陷一路活到端到端试玩才暴露：

| 缺陷 | 手工夹具为什么测不到 | 真实数据下的表现 |
|---|---|---|
| 热量吸收态（衰减被整数除法截断为 0 + 死区无兜底） | 夹具只装 1 个技能时 5 发正好到 100 触发过热，缺陷不可见 | 4 技能（20/18/22/20，总和 80 < 100）永远到不了 cap，战斗停摆 |
| 刷怪 `delay`/`interval` 是毫秒却被当 tick | 夹具的 delay 是个位数，差 50 倍也看不出 | 第 1 关第一波 1012ms 变成 50.6 秒，全关 18.7 分钟 |

两者都只在"真实参数 + 长时间运行"下暴露。修复后实测：默认构筑第 1 关 **6806 tick（5.7 分钟）通关，score 16048，拿到 3 星**（3 星门槛 14656）。修复前是 385 秒 0 杀 0 星。

这些断言都经**变异测试**验证过有效性 —— 把缺陷改回去，确认测试真的会红：

| 变异 | 结果 |
|---|---|
| 衰减改回 `(10n*50n)/1000n` | `heatmap.test.ts` 5 条红 |
| 衰减截断 + 移除死区兜底（即修复前状态） | `engine.smoke.test.ts` 2 条红（`3000 tick 只开 3 发`） |
| `atTick` 改回直接加毫秒 | `engine.smoke.test.ts` 3 条红 |
| `star_targets` 还原成 `[78000, 93600, 109200]` | `engine.smoke.test.ts` 3 条红 |
| 护甲加成加回敌人 | `engine.smoke.test.ts` 1 条红 |
| 客户端反应表某项漂移 | `reaction_contract.test.ts` 1 条红 |
| `applyArmor` 完全失效 | `armor_test.go` 11 处红 |
| `MAX_ARMOR` 750 → 100 | `armor.test.ts` 4 条红 |
| Go 侧 PHI 末位 +1 | `-check` 报 2 个文件过期 + 客户端 1 条红 |
| 平衡 flag 改 false | `consistency.test.ts` 1 条红 |
| `Buy` 去掉 `FOR UPDATE` | `concurrency_test.go` 稳定 `成功 5 次 > 限购 3 次` |
| `SignIn` 恒返回错误 | `concurrency_test.go` 夹具自检捕获 |
| `reactionAttackCap` 去掉 `/M` 收紧 | `anti_inflation.test.ts` **5 条红** |
| 技能卡空槽查找改回 `findIndex` 语义 | `cards.test.ts` 1 条红 |
| `absorbOverheat` 不清热量 | `cards.test.ts` 1 条红（每 tick 白烧护盾） |
| 旋转风障查表换回 `Math.cos` | `terrain.test.ts` 1 条红（打桩捕获） |
| `spawnProgress` 换回浮点 `+=0.08` | `determinism.test.ts` 2 条红 |
| 地形判定改回 `(seed>>8) % 4` | `terrain_ratio_test.go` **2 条红**（占比 37% 超上限、第 5 章 13/15 超上界） |
| TS `reactionElement` 改回 `??` | `anti_inflation.test.ts` 2 条红 |
| Go `reactionElement` 去掉空串回退 | `damage_test.go` 1 条红 |

### 变异测试里三次"注入失败 ≠ 守卫有效"

这类误读值得单独记，因为它会让人以为守卫已经够强：

| 变异 | 现象 | 真实原因 |
|---|---|---|
| `Math.cos` 换回查表 | 30 个地形用例**全绿** | 守卫断言的是**表本身**；"表是对的"与"运行时用的是表"是两件事 |
| 去掉雪崩只留 `levelID × 奇数` | 均匀度测试**全绿** | 这个变异确实无害 —— 连续整数乘奇数 mod 8 天然均匀。如实记录，不粉饰 |
| Go 侧 `reactionElement` 变异 | 连续三次**注入失败** | 正则漏了闭合括号 / `if false` 触发变量未使用；改用不可能匹配的字面量才成功 |

补第一条的方式是加一条**行为**断言（把 `Math.cos` 打桩记账，跑 3000 tick 断言一次没调用），
它比任何源码文本扫描都强：不管实现怎么写，只看有没有真的调用。

⚠️ 在项目文件上做变异测试的两条纪律：**先备份到项目外**、**必带 `-timeout`**。
本项目已经栽过一次：把 `go func` 改成同步直调导致 `wg.Wait()` 永久阻塞、
命令超时，文件被留在变异状态，备份缺失时是手工改回去的。

### 测试空洞的四种形态

| 形态 | 实例 | 改坏了能绿吗 |
|---|---|---|
| **恒等式** | `expect(levelSeed(1)).toBe(levelSeed(1))` | 能。名字叫「与 Go 侧一致」，实际自己等于自己 |
| **本地复制** | 测试内抄一份 PHI 常量与公式 | 能。改 Go 侧完全不影响它 |
| **夹具落在保护之外** | 契约向量 `armor_permille` 全为 0，而 `applyArmor` 第一行就 `return` | 能。两端都改成 `return dmg`，140+ 测试全绿 |
| **数据文件当开关** | `if (!vectors.balance_invariants.xxx) return` | 能。把 flag 改 false，断言静默消失 |

共同点：**它们全都待在「看起来测了东西」的位置上**。所以补测试的判据不是
"覆盖到了吗"，而是**"改坏被测代码会不会红"** —— 这只能靠变异测试回答，
读代码回答不了。

---

## 后台构建体积

后台是运营每天要开的页面，首屏体积直接等于等待时间。**这一节的数字有测量工具，
不要凭印象改** —— 改完请重测。

```bash
cd admin && npm run build && npm run size        # 报告
cd admin && npm run build && npm run size:check  # 超预算则退出码 1
```

| | 数值 |
|---|---|
| **首屏 JS** | 160 KB（gzip 61 KB），3 个 chunk |
| **首屏 CSS** | 8 KB（gzip 2 KB），28 个 CSS 里只有 2 个进首屏 |
| **首屏合计** | **168 KB（gzip 64 KB）** |
| **预算** | **220 KB（gzip 80 KB）** —— 定在 `admin/scripts/size-report.mjs` 的 `BUDGET` |

> 预算留了约 37% 余量，正常加功能不会误报；而一次「整包注册」级别的回归
> （实测首屏会从 161 KB 涨到 1028 KB）会超出 4.7 倍，必红。
> `tests/bundle_size.test.ts` 有一条守卫专门核对**本节的数字与脚本里的 `BUDGET` 一致** ——
> 改预算忘了同步文档（或者反过来）会立刻被抓住。
| element-plus | **286 KB**（15 个按需 chunk） |
| echarts | 534 KB，**只服务 Dashboard 一个页面**，不进首屏 |

第 47 轮把 element-plus 从整包注册改成按需引入，首屏从 **1378 KB 降到 161 KB**
（gzip 386 → 62 KB）。依赖是 devDependencies 里**早就装了但一直没接进配置**的
`unplugin-vue-components`。

### 三个测量陷阱（都踩过，数字错了但不报错）

1. **别用 dist 总量。** 路由是代码分割的，dist 总量把每个懒加载页都算进去。
   用它会两头出错：**指错方向**（让人去优化只在 Dashboard 用的 echarts），
   **低估收益**（element-plus 那次，dist 总量只反映 −520 KB，首屏实际 −1217 KB）。
2. **别在陈旧 dist 上测。** `npm run size` 只读 dist、不构建，忘记 build 时它会
   拿上一次的产物给出一个**格式正常、退出码 0 的错误数字**。
   工具现在会比对 mtime，旧了直接退出码 2。
3. **首屏 CSS 只能从 `index.html` 的 `<link>` 取。** 把 `assets/` 下所有 CSS 加总
   会把 26 个懒加载页的样式也算进来（175 KB vs 实际的 8 KB）。

> 这三条的共同点：**数字错了但没有任何报错。**
> 一个能跑通、退出码 0、格式规整的报告比一个崩溃更危险，因为人会相信它。

