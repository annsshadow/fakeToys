# 05 · 接口参考

> 服务端共 **66 个 HTTP 端点**：玩家侧 37（含 2 个健康检查）+ 管理端 29。
> 本文是 `server/internal/httpapi/routes.go` 的对照表，字段名以 Go 的 `json` tag 为准。

---

## 1. 通用约定

| 项 | 约定 |
|---|---|
| Base path | `/api/v1`（健康检查在根路径 `/healthz`、`/readyz`） |
| 认证 | `Authorization: Bearer <access_token>` |
| 请求体 | `Content-Type: application/json`，上限 1MB |
| 成功响应 | 业务 JSON，无统一包裹 |
| 错误响应 | `{"error":{"code":"…","message":"…"}}` |
| 时间 | RFC3339（`2026-10-20T12:00:00Z`） |
| **int64 字段** | 种子、battle_id 等可能超 2^53 的值**一律以字符串下发**（`seed`、`level.seed_str`） |
| 空集合 | 服务端保证是 `{}` / `[]`，不是 `null`（jsonb `null` 会让聚合 SQL 静默漏行或报错） |
| 尾斜杠 | Fiber 非严格路由，`/me` 与 `/me/` 等价 |

### 1.1 错误码

| `code` | HTTP | 含义 | 客户端建议 |
|---|---|---|---|
| `bad_json` | 400 | 请求体不是合法 JSON | 检查序列化 |
| `bad_input` | 400 | 参数非法（含 id 非正整数、余额不足、未拥有、已满级） | 提示具体原因 |
| `bad_id` / `bad_body` | 400 | 路径 id / 请求体形状不对 | 同上 |
| `unauthorized` | 401 | 缺 Authorization / 令牌无效或过期 | 刷新令牌后重试一次 |
| `forbidden` | 403 | 无权 / 被封禁 / 关卡未解锁 / 微信登录未启用 | 提示，不重试 |
| `not_found` | 404 | 资源不存在（含「不是你的」——刻意不区分） | 提示 |
| `settle_rejected` | 422 | 上报被判定不可信（见 §6） | 不要重试 |
| `role_not_writable` | 403 | 管理端当前角色不可写 | 提示联系管理员 |
| `rate_limited` | 429 | 触发限流 | 退避重试 |
| `db_unavailable` | 503 | `/readyz` 探测失败 | 提示维护中 |
| `internal_error` | 500 | 服务内部错误（真实原因只在日志） | 上报 |

---

## 2. 端点总表

### 2.1 公开（无需令牌）

| # | 方法 | 路径 | 限流 | 说明 |
|---|---|---|---|---|
| 1 | GET | `/healthz` | — | 存活探针，返回 `{ok, time}` |
| 2 | GET | `/readyz` | — | 就绪探针（Ping 数据库），失败 503 |
| 3 | POST | `/api/v1/auth/guest` | 60/min IP | 游客登录/恢复 |
| 4 | POST | `/api/v1/auth/wechat` | 60/min IP | 微信 code 登录（未配置时 403） |
| 5 | POST | `/api/v1/auth/refresh` | 60/min IP | 刷新令牌（轮换） |
| 6 | POST | `/api/v1/auth/login` | 10/min IP | 玩家账号登录（用户名 + 口令） |
| 7 | GET | `/api/v1/config` | — | 全量配置（支持 `?if_version=` → 304） |
| 8 | GET | `/api/v1/leaderboard` | — | 排行榜（`?type=power\|stage\|efficiency`） |
| 9 | POST | `/api/v1/admin/login` | 5/min IP | 管理员登录 |

### 2.2 玩家（`requireUser`）

| # | 方法 | 路径 | 限流 | 说明 |
|---|---|---|---|---|
| 9 | GET | `/me` | — | 身份 + 构筑 + 评分 + 战力 + 最高关 |
| 10 | GET | `/me/loadout` | — | 出战槽位 |
| 11 | PUT | `/me/loadout` | — | 保存出战槽位 |
| 12 | GET | `/me/stars` | — | 各关历史最好星级 |
| 13 | POST | `/me/skills/:id/upgrade` | — | 技能升一级 |
| 14 | POST | `/me/skills/synthesize` | — | 技能熔炼 |
| 15 | GET | `/me/gems` | — | 持有宝石列表 |
| 16 | POST | `/me/gems/:id/wash` | — | 洗练宝石 |
| 17 | POST | `/me/logout` | — | 登出（吊销全部 refresh + 轮换 guest_token） |
| 18 | GET | `/wallet` | — | 钱包（含体力恢复与道具余额） |
| 19 | GET | `/levels/:id` | — | 单关完整数据（含 waves） |
| 20 | POST | `/battle/token` | — | 开局凭证 |
| 21 | POST | `/battle/settle` | 60/min 用户 | 结算 |
| 22 | POST | `/battle/verify` | 20/min 用户 | 验真 |
| 23 | GET | `/battle/:id/replay` | — | 取回重放信息（限本人） |
| 24 | GET | `/mastery` | — | 专精状态 |
| 25 | POST | `/mastery/allocate` | — | 分配专精点 |
| 26 | POST | `/mastery/reset` | — | 重铸（消耗道具） |
| 27 | GET | `/tasks` | — | 任务列表（`?scope=daily\|weekly\|achievement`） |
| 28 | POST | `/tasks/:id/claim` | — | 领取任务奖励 |
| 29 | GET | `/shop` | — | 商城（含今日已购次数） |
| 30 | POST | `/shop/:id/buy` | — | 购买 |
| 31 | POST | `/signin` | — | 每日签到 |
| 32 | GET | `/signin/calendar` | — | 七日奖励表 |
| 33 | GET | `/signin/status` | — | 签到状态 |
| 34 | POST | `/redeem` | 5/min IP + 20/min 用户 | 兑换码 |
| 35 | GET | `/diagnose` | — | 卡关诊断 |
| 36 | GET | `/defenses` | — | 防线（我的 + 候选） |
| 37 | POST | `/defenses/save` | — | 保存防线快照 |
| 38 | POST | `/defenses/:id/challenge` | — | 挑战防线 |

> 编号 1–37 是玩家侧，38 是防线挑战；总数 37 个玩家端点 + 29 个管理端点 = 66。
> （上表把健康检查与公开端点计入玩家侧。）

### 2.3 管理端（`requireAdmin`，写操作另挂 `requireWritable`）

| # | 方法 | 路径 | 可写角色 | 说明 |
|---|---|---|---|---|
| 39 | POST | `/admin/logout` | 全部 | 吊销本管理员全部会话 |
| 40 | GET | `/admin/me` | — | 当前管理员 |
| 41 | GET | `/admin/dashboard` | — | 看板聚合 |
| 42 | GET | `/admin/levels` | — | 关卡列表（`?chapter=&keyword=`） |
| 43 | GET | `/admin/levels/:id/waves` | — | 关卡波次 |
| 44 | PUT | `/admin/levels/:id` | admin/ops | 改关卡 |
| 45 | POST | `/admin/levels/regenerate` | admin/ops | 重生成 100 关 |
| 46 | GET | `/admin/skills` | — | 技能 + 配方 |
| 47 | PUT | `/admin/skills/:id` | admin/ops | 改技能 |
| 48 | GET | `/admin/equipment` | — | 装备/宝石/皮肤 |
| 49 | GET | `/admin/reactions` | — | 反应表（含中文名与说明） |
| 50 | GET | `/admin/users` | — | 用户列表 |
| 51 | POST | `/admin/users/:id/ban` | admin/ops | 封禁 |
| 52 | POST | `/admin/users/:id/unban` | admin/ops | 解封 |
| 53 | POST | `/admin/users/:id/grant` | admin/ops | 发币/回收 |
| 54 | GET | `/admin/battles` | — | 战报列表 |
| 55 | GET | `/admin/battles/:id` | — | 战报详情 |
| 56 | POST | `/admin/battles/:id/verify` | admin/ops | 运营验真 |
| 57 | GET | `/admin/recomputes` | — | 重算台账 |
| 58 | GET | `/admin/recomputes/stats` | — | 重算总账 + 分布 + 闸门 |
| 59 | GET | `/admin/defenses` | — | 防线列表 |
| 60 | GET | `/admin/economy` | — | 货币流水 + 商城 |
| 61 | PUT | `/admin/shop/:id` | admin/ops | 改商品 |
| 62 | GET | `/admin/announcements` | — | 公告列表 |
| 63 | POST | `/admin/announcements` | admin/ops | 创建公告 |
| 64 | GET | `/admin/redeem-codes` | — | 兑换码列表 |
| 65 | POST | `/admin/redeem-codes` | admin/ops | 创建兑换码 |
| 66 | GET | `/admin/audit-logs` | — | 审计日志 |

---

## 3. 玩家端详解

### 3.1 认证

#### `POST /api/v1/auth/guest`

```json
// 请求
{ "guest_token": "", "nickname": "" }
// 响应 200
{
  "access_token": "eyJ…",
  "refresh_token": "…",
  "expires_at": "2026-10-20T14:00:00Z",
  "user": { "id": 1, "nickname": "玩家1234", "avatar_url": "", "is_guest": true, "status": 1 }
}
```

- `guest_token` 为空 → 服务端生成新的（新号）；非空且存在 → 恢复该账号并只更新 `last_login_at`。
- `nickname` 为空 → 生成「玩家 + 4 位数字」；长度 > 64 → 400（不静默截断）。
- 新号初始化：coin 2000 / gem 50 / energy 30 / keys 1，2 件装备，3 个技能（1/4/7），3 个出战槽位。
- ⚠️ `PLAYER_LOGIN_REQUIRED=true` 时本端点 **403 `login_required`**（且**不建号**）。
  服务端这一道是权威判据 —— 只在客户端挡的话，浏览器里缓存着旧 bundle 的
  测试者仍会调它，于是「每个浏览器一个号、测试数据散在各处」的老问题
  会在**看起来已经修好**之后继续存在。

#### `POST /api/v1/auth/login`

```json
// 请求
{ "username": "admin", "password": "…" }
// 响应 200 —— 与 /auth/guest 完全同构
{
  "access_token": "eyJ…",
  "refresh_token": "…",
  "expires_at": "2026-10-21T14:00:00Z",
  "user": { "id": 15, "nickname": "admin", "avatar_url": "", "is_guest": false, "status": 1 }
}
```

- 账号登录与游客登录拿到的是**同一种令牌**（`kind=user`），权限面完全一样；
  差别只在「用哪把钥匙进门」。
- **用户不存在与口令错误返回同一条错误**（401 `用户名或密码错误`）——
  分开报等于白送一个用户名枚举器。
- 账号被封禁（`status=2`）→ 403 `forbidden`；但**口令错**时仍返回
  「用户名或密码错误」，不泄露封禁状态（先验口令再查状态）。
- 限流 **10/min/IP**（比游客建号严）：它做 bcrypt（cost 10），
  且测试环境下这个口令与运营超管相同 —— 它同时是超管口令的爆破面。
- 用途：测试环境要求全体测试者共用一个账号，否则数据散在无数个游客号里。

#### `POST /api/v1/auth/wechat`

```json
{ "code": "wx_code" }
```

- `WECHAT_APP_ID` / `WECHAT_APP_SECRET` 未配置 → **403** `forbidden`，message 明确写「微信登录未启用」。
- 不静默降级成游客登录（那会让「登录失败」变成「登录成功但数据不对应」）。

#### `POST /api/v1/auth/refresh`

```json
{ "refresh_token": "…" }
```

- 服务端用条件 UPDATE 吊销旧 token 并签发新的（**轮换**）。
- 无效/已用/过期 → 401。

#### `POST /api/v1/me/logout`

```json
// 响应
{ "revoked": 3, "guest_token": "新的游客凭证" }
```

- 吊销该用户**全部**未过期 refresh token，并在同一事务里**轮换** `guest_token`。
- 幂等：没有可吊销的令牌时返回 `revoked: 0`。
- ⚠️ 客户端必须保存新 `guest_token`，否则下次冷启动会建新号。

### 3.2 配置与排行榜

#### `GET /api/v1/config`

| 查询参数 | 说明 |
|---|---|
| `if_version` | 客户端已有版本；与服务端内容哈希相同 → **304**（零载荷） |

响应字段（`service.GameConfig`）：

| 字段 | 类型 | 说明 |
|---|---|---|
| `version` | string | 内容哈希（SHA-256 前 16 位 hex），只在内容变化时变 |
| `levels` | `LevelSummary[]` | **摘要**，无 `waves` |
| `enemies` | `SeedEnemy[]` | 已应用血量 ×8 缩放 |
| `skills` / `composite_skills` | `SeedSkill[]` | 基础 24 + 合成 18 |
| `recipes` | `{output,a,b,out_tier}[]` | 18 条 |
| `equipment` / `gems` / `skins` | array | 目录 |
| `gem_qualities` | `{name,affix_count,mult}[]` | 品质 → 词条数/倍率 |
| `mastery_families` | `{family,name,nodes[]}[]` | 8 系 × 12 节点 |
| `reactions` | `ReactionSpec[]` | 7 条，含 `attack_weight_pct` |
| `chapters` | `{id,name,start_level,end_level,terrain_kind,boss_enemy_id}[]` | 6 章 |
| `rating_weights` | object | 五维权重 |
| `score_rules` | `{per_damage_unit,on_kill_normal,on_kill_boss,star_target_ratio[3],score_full_at_sec}` | **必须由服务端下发** |
| `skill_rules` | `{max_level,coef_permille,base_cost}` | 技能升级规则 |
| `server_time` | RFC3339 | 每次请求不同，用于校准客户端时钟 |

> 实测未压缩 167,124 B，gzip 后 23,964 B（14.3%）。`levels` 占 74%。

#### `GET /api/v1/leaderboard`

| 参数 | 取值 | 缺省 |
|---|---|---|
| `type` | `power` / `stage` / `efficiency` | `power` |
| `limit` | 1..200 | 50 |

```json
{ "type": "power", "items": [ { "rank": 1, "user_id": 3, "nickname": "…", "avatar_url": "", "score": 12345, "detail": "…" } ] }
```

- 未知 `type` → **400**（不回落到战力榜）。`?type=`（显式空）同样 400。
- 响应里的 `type` 是**规范化后**的值。
- 三种榜：战力（`level_exp` 降序）、关卡（最高通关关号）、效率（`min_power_clear` 升序）。

### 3.3 玩家档案

#### `GET /api/v1/me`

```json
{
  "user_id": 1,
  "build": {
    "skills": { "1": { "id": 1, "slot": 0, "level": 2, "base_damage": 120, "element": "fire", "…": "…" } },
    "elements": ["fire", "ice", "lightning"],
    "equipment": [1, 10],
    "mastery_nodes": [101, 111],
    "active_slots": 5,
    "attacker": {
      "attack": 120, "crit_permille": 50, "crit_multiplier_permille": 1500,
      "reaction_mult_permille": 0, "element_cap": 2, "reaction_tier": 1,
      "element_coef_permille": 1000, "heat_cap_permille": 0,
      "armor_permille": 0, "mechanic_permille": 0
    },
    "…": "…"
  },
  "build_rating": {
    "element_coverage": 3, "reaction_coverage": 4, "mastery_done": 2,
    "mastery_picked": 4, "equipment_synergy": 1, "mechanic_depth": 0,
    "total": 78, "reaction_mult_bonus": 400, "crit_bonus": 0,
    "element_cap_bonus": 1, "armor_bonus": 0, "reaction_mult_nodes": 2,
    "weaknesses": ["…"]
  },
  "power": 4567,
  "max_stage": 12
}
```

- `build.attacker` 是**唯一**可信的攻方属性来源（客户端不得本地推算）。
- `max_stage` 是服务端权威进度，客户端据此恢复解锁态。

#### `GET /api/v1/me/stars`

```json
{ "stars": { "1": 3, "2": 2 } }
```

- 从未结算的关卡**不出现**在 map 里（客户端按 0 处理）。

#### `GET|PUT /api/v1/me/loadout`

```json
// GET → { "skill_ids": [1, 4, 7, 0] }
// PUT 请求 → { "skill_ids": [1, 4, 7, 0] }
// PUT 响应 → { "skill_ids": [1, 4, 7, 0] }
```

- 0 表示空槽；槽位参与回放哈希，必须落服务端。
- 服务端校验：技能已拥有、槽位不超预算（基础 5 + 专精额外插槽）、无重复。

#### `POST /api/v1/me/skills/:id/upgrade`

```json
{ "skill_id": 1, "level": 2, "wallet": { "coin": 1800, "gem": 50, "energy": 30, "keys": 1, "energy_updated_at": "…", "tokens": {} } }
```

- 费用与上限由服务端裁定（`skill_rules`）；余额不足/未拥有/已满级 → 400。
- 回执带 `wallet`，避免客户端再发一次 `/wallet` 看到中间态。

#### `POST /api/v1/me/skills/synthesize`

```json
// 请求 { "a": 1, "b": 4 }
// 响应 { "output": 31, "build": { … } }
```

- 顺序无关（服务端用 LEAST/GREATEST 抹平）。
- 失败：无配方 / 未达等级 / 材料不全 / 已拥有产物 → 400（不泄漏配方细节）。
- 材料会被消耗，`build` 立即失效所以一并回传。

#### `GET /api/v1/me/gems` / `POST /api/v1/me/gems/:id/wash`

```json
{
  "gems": [
    { "id": 7, "gem_id": 3, "code": "gem_crit", "name": "锐击石", "descr": "…",
      "quality": "purple", "affixes": [ { "affix": "crit", "value": 42 } ], "equipped_slot": "" }
  ]
}
```

- 洗练消耗 1 枚 `gem_wash_token`，品质不变，词条重 roll。
- 余额不足 → 400；宝石不属于本人 → 404。

### 3.4 钱包与关卡

#### `GET /api/v1/wallet`

```json
{
  "coin": 2000, "gem": 50, "energy": 30, "keys": 1,
  "energy_updated_at": "2026-10-20T12:00:00Z",
  "tokens": { "gem_wash_token": 0, "mastery_reset": 1, "revive_token": 0 }
}
```

- 读取时**顺带做体力恢复**：每 6 分钟 +1，上限 120，按整格推进（不吞余数）。
- `tokens` 是道具余额，未持有过的补 0（客户端靠「键在不在」区分「0 个」与「缺字段」）。

#### `GET /api/v1/levels/:id`

返回 `GeneratedLevel`（含 `waves`、`terrain`、`star_targets`、`max_score`、`element_cap`、
`armor_permille`、`max_reaction_tier`）。

- `id ∈ [1,100]`，否则 400。
- 这是 `/config` 摘要的补充：只有防线挑战需要完整波次。

### 3.5 战斗

#### `POST /api/v1/battle/token`

```json
// 请求 { "level_id": 1 }
// 响应
{
  "token_id": 123,
  "seed": "8123456789012345678",
  "expires_at": "2026-10-20T12:30:00Z",
  "level": { "id": 1, "chapter": 1, "name": "…", "base_hp": 1000, "waves": [ … ], "seed_str": "…", "…": "…" },
  "revived": false,
  "extra_base_hp": 0,
  "build": { … }
}
```

| 失败 | 码 | 说明 |
|---|---|---|
| 关卡不存在 | 400 | `level_id ∉ [1,100]` |
| 未解锁 | 403 | `level_id > max_stage + 1` |
| 体力不足 | 400 | 扣 `energy_cost`（缺省 5） |
| 未装备技能 | — | 服务端不拦，但客户端会提示（开局后无技能可用） |

副作用：插 `battle_tokens`、扣体力、若有 `revive_token` 则扣 1 并给额外血量。

#### `POST /api/v1/battle/settle`

```json
{
  "token_id": 123,
  "result": "win",
  "stars": 3,
  "score": 18000,
  "kills": 42,
  "leaked": 1,
  "hp_left": 700,
  "wave_reached": 5,
  "duration_ms": 95000,
  "shots": 320,
  "hits": 280,
  "reactions": 26,
  "heat_max": 88,
  "elements_used": { "fire": 120, "ice": 80 },
  "reactions_used": { "steam_burst": 12 },
  "terrain_used": ["oil_drum"],
  "replay_hash": "0123456789abcdef",
  "replay_skills": "0:1:120:2:20,1:4:100:1:18",
  "card_picks": [0, -1, 1, 2, -5]
}
```

响应：

```json
{
  "battle_id": 456, "result": "win", "win": true,
  "score": 18000, "stars": 3,
  "clamped": false, "clamp_note": "",
  "loot": { "coin": 1200, "gem": 3 },
  "wallet": { … }, "power": 4567,
  "build_rating": { … }, "new_max_stage": 5
}
```

要点：

- `stars` 与 `score` 由服务端**重算/裁剪**，上报值只作为对照。
- `token_id` 一次性：重复提交 → 422 `settle_rejected`（`战斗凭证已被使用`）。
- `replay_skills` 为空串**放行**（老客户端兼容）；非空则逐字比对。
- 被裁剪时 `clamped=true` + `clamp_note` 说明原因。

#### `POST /api/v1/battle/verify`

```json
// 请求 { "battle_id": 456, "replay_hash": "0123456789abcdef" }
// 响应
{
  "battle_id": 456, "expected_hash": "…", "actual_hash": "…",
  "matched": true, "level_id": 5, "seed": "8123…", "recorded_at": "2026-10-20T12:01:00Z"
}
```

- 只比对「提交值」与「结算时记录值」，**不重算**。
- 只能验自己的战报（他人战报与不存在返回同一个 404）。
- `replay_hash` 长度上限 64。

#### `GET /api/v1/battle/:id/replay`

```json
{
  "battle_id": 456, "level_id": 5, "seed": "8123…",
  "level": { … 含 waves … }, "expected_hash": "…", "created_at": "…",
  "build": { "skills": { … }, "attacker": { … }, "active_slots": 5, "…": "…" },
  "card_picks": [0, -1, 1]
}
```

- 限本人；`build.settle_input` 已被剥离（防止被当作「高星战报蓝本」）。

### 3.6 专精

#### `GET /api/v1/mastery`

```json
{
  "points": 3, "nodes": [101, 111],
  "layer_limit": 2, "base_slots": 5, "total_slots": 5,
  "families": [ { "family": "flame", "name": "焰", "nodes": [ … ] } ]
}
```

- `total_slots = base_slots + 专精额外插槽`，客户端**必须**用它而不是编译期常量。

#### `POST /api/v1/mastery/allocate`

```json
// 请求 { "node_id": 101 }
// 响应 = GET /mastery 的形状
```

失败：点数不足 / 同层已选满 2 / 前置层未点亮 / 节点不存在 → 400。

#### `POST /api/v1/mastery/reset`

```json
{
  "reset_cleared": 4, "points": 3, "nodes": [],
  "layer_limit": 2, "base_slots": 5, "total_slots": 5
}
```

- 消耗 1 件 `mastery_reset`；点数预算不变。
- 道具不足 → 400。

### 3.7 任务 / 商城 / 签到 / 兑换

#### `GET /api/v1/tasks?scope=daily`

```json
{
  "scope": "daily",
  "items": [
    { "id": 1, "code": "daily_kills_30", "name": "今日击杀 30 个敌人",
      "scope": "daily", "target": 30, "metric": "kills",
      "progress": 12, "reward": { "coin": 800 }, "claimed": false, "done": false }
  ]
}
```

- `scope` 白名单 `daily/weekly/achievement`；非法值 → 400（不返回空列表）。
- `progress` 语义：daily/weekly 为本周期累计；achievement 为**历史最大值**（终身）。

#### `POST /api/v1/tasks/:id/claim`

```json
{ "reward": { "coin": 800 } }
```

未完成 / 已领取 → 400。

#### `GET /api/v1/shop`

```json
{
  "items": [
    { "id": 1, "code": "shop_coin_small", "name": "金币袋", "category": "currency",
      "price": { "gem": 10 }, "payload": { "coin": 20000 },
      "limit_per_day": 3, "bought_today": 0 }
  ]
}
```

#### `POST /api/v1/shop/:id/buy`

```json
{ "granted": { "coin": 20000 } }
```

失败：余额不足 / 今日已达限购 / 商品下架 → 400。

#### `POST /api/v1/signin`

```json
{
  "already": false, "day_index": 2,
  "reward": { "coin": 2000 },
  "wallet": { … }
}
```

七天签完后再次调用返回 `already: true`。

#### `GET /api/v1/signin/calendar`

```json
{ "days": [ { "day_index": 1, "reward": { "coin": 1000 } }, … ] }
```

#### `GET /api/v1/signin/status`

```json
{ "claimed_count": 2, "signed_today": true, "can_sign": false }
```

#### `POST /api/v1/redeem`

```json
// 请求 { "code": "LAIYIPAO" }
// 响应 { "reward": { "coin": 30000, "gem": 50 } }
```

失败：码不存在 / 已过期 / 已停用 / 已用完 / 本人已用过 → 400。

### 3.8 诊断

#### `GET /api/v1/diagnose?level_id=5&failed_times=2`

```json
{
  "Stage": "no_reaction",
  "Title": "你的构筑几乎打不出元素反应",
  "Detail": "已装载元素：fire；本关敌人……",
  "Suggestions": ["再带一种不同元素的技能", "……"],
  "Confidence": 75
}
```

> ⚠️ **已知不一致（文档如实记录）**：`domain.Diagnose` 没有 `json` tag，
> 因此实际下发的是 **PascalCase**（`Stage` / `Title` / `Detail` / `Suggestions` / `Confidence`）。
> 而 `miniapp/src/pages/stage/stage.vue` 读的是 `d.stage` / `d.confidence` / `d.title` /
> `d.detail` / `d.suggestions`（小写），因此诊断卡会渲染成空白（置信度后面没有数字）。
> 修法是给该结构补 snake_case tag（并同步客户端或直接改客户端），
> 这属于 `ENGINEERING.md` 第 4 条「跨端字段名显式约定」的典型违例。

### 3.9 防线

#### `GET /api/v1/defenses`

```json
{
  "mine": { "id": 3, "owner_id": 1, "owner_name": "…", "name": "我的防线",
            "power": 4567, "element_coverage": 3, "mastery_done": 2,
            "wins": 5, "losses": 2, "shielded_until": "2026-10-21T12:00:00Z",
            "snapshot": { … }, "snapshot_hash": "…",
            "challenged_today": 0, "my_attempts_today": 1, "my_stolen_today": 0,
            "attempt_limit": 3, "can_challenge": true, "challenge_blocked": "" },
  "candidates": [ … 最多 20 条，按战力降序，排除自己 … ],
  "attempt_limit": 3,
  "stolen_limit": 2
}
```

- 护盾期内或次数用尽时 `can_challenge=false` + `challenge_blocked` 给出人话原因。
- 这是**只读**端点（不写 `user_daily_challenges`）。

#### `POST /api/v1/defenses/save`

```json
{
  "name": "我的防线",
  "skills": [1, 4, 7],
  "equipment": [1, 10],
  "mastery_nodes": [101, 111],
  "works": ["slow_belt", "block_wall"],
  "shield_hours": 24
}
```

响应 `{ "defense": DefenseView }`。

校验：名称 ≤ 32 字；`works` 白名单三种；技能/装备/专精必须真实属于本人；
技能数量 ≤ 12。

#### `POST /api/v1/defenses/:id/challenge`

```json
{
  "seed": "8123456789012345678",
  "won": true,
  "duration_ms": 62000,
  "hp_left_pct": 45,
  "replay_hash": "0123456789abcdef",
  "replay_skills": "0:1:120:2:20"
}
```

响应：

```json
{
  "won": true,
  "stolen": { "coin": 300 },
  "owner_name": "…",
  "owner_power": 4567,
  "my_attempts_today": 2,
  "owner_stolen_today": 1
}
```

边界（事务外校验）：`seed` 十进制且 ∈ [0, 2^63-1]；`duration_ms ∈ (0, 24h]`；
`hp_left_pct ∈ [0,100]`；`replay_hash ≤ 64`；`replay_skills ≤ 1024`。

业务闸门：每日 3 次；对方每日被偷 ≤ 2；护盾期内拒绝；胜利按 10% 偷取并受掉落封顶。

---

## 4. 管理端详解

### 4.1 登录与会话

#### `POST /api/v1/admin/login`

```json
// 请求 { "username": "admin", "password": "…" }
// 响应
{ "access_token": "eyJ…", "expires_at": "2026-10-20T14:00:00Z",
  "admin": { "id": 1, "username": "admin", "role": "admin" } }
```

- bcrypt cost 10；用户名不存在与密码错误返回**同一个** 401。
- 限流 5/min/IP（这是全站最贵的未认证请求）。

#### `POST /api/v1/admin/logout`

```json
{ "revoked_sessions": 2 }
```

吊销该管理员全部 `admin_tokens`（幂等）。不挂 `requireWritable`。

#### `GET /api/v1/admin/me`

```json
{ "admin": { "id": 1, "username": "admin", "role": "ops" } }
```

### 4.2 看板

#### `GET /api/v1/admin/dashboard`

```json
{
  "users": { "total": 120, "guests": 118, "wechat": 2, "banned": 1, "new_today": 5 },
  "battles": { "total": 900, "today": 40, "wins": 610, "avg_duration_ms": 92000 },
  "progression": { "avg_max_stage": 14, "avg_power": 3800, "stage_100_clears": 0 },
  "economy": { "coin_in": 1000000, "coin_out": 800000, "gem_in": 500, "gem_out": 300,
               "orders_paid": 0, "orders_pending": 0 },
  "verification": { "checked": 30, "matched": 28, "mismatched": 2 },
  "reaction_usage": [ { "reaction": "steam_burst", "count": 120 } ],
  "daily_active": [ { "date": "2026-10-20", "dau": 30 } ],
  "stage_funnel": [ { "level_id": 1, "attempts": 50, "clears": 40 } ]
}
```

### 4.3 关卡

#### `GET /api/v1/admin/levels?chapter=2&keyword=冻原`

```json
{
  "items": [
    { "id": 41, "chapter": 3, "name": "冻原·41", "seed": "…", "base_hp": 2600,
      "wave_count": 7, "difficulty": 1200, "energy_cost": 5, "is_boss": false,
      "enabled": true, "terrain_config": [ … ], "star_targets": [ … ],
      "player_authority": "generator", "applied_to_players": false,
      "clear_rate": 35, "avg_wave": 4 }
  ],
  "total": 100
}
```

- `clear_rate` **已是百分数**。
- `player_authority="generator"` 表示玩家侧关卡由生成器产出，本页编辑不影响线上。

#### `GET /api/v1/admin/levels/:id/waves`

```json
{ "waves": [ { "wave_index": 0, "spawns": [ { "enemy_id": 1, "count": 6, "interval": 800, "delay": 0 } ] } ] }
```

#### `PUT /api/v1/admin/levels/:id`

请求为 `Partial<AdminLevel>` 的字段子集；响应 `{ "level": AdminLevel }`。
写 `admin_audit_logs`（`update_level`）。

#### `POST /api/v1/admin/levels/regenerate`

```json
{ "generated": 100 }
```

重跑生成器并 upsert `levels` / `level_waves`。同样**不影响玩家路径**。

### 4.4 技能 / 装备 / 反应

#### `GET /api/v1/admin/skills`

```json
{ "items": [ { "id": 1, "code": "ember", "name": "燃烧弹", "family": "flame", "element": "fire",
               "kind": "active", "base_damage": 100, "heat_cost": 20, "cooldown_ms": 800,
               "pierce": 0, "aoe_radius": 0, "apply_element": "fire", "apply_stacks": 1,
               "unlock_level": 1, "descr": "…" } ],
  "recipes": [ { "output": 31, "a": 1, "b": 4, "out_tier": 2 } ] }
```

#### `PUT /api/v1/admin/skills/:id`

字段白名单更新，响应 `{ "skill": AdminSkill }`，写审计（`update_skill`）。

#### `GET /api/v1/admin/equipment`

```json
{ "equipment": [ … ], "gems": [ … ], "skins": [ … ] }
```

#### `GET /api/v1/admin/reactions`

```json
{ "reactions": [ { "key": "steam_burst", "name": "蒸汽爆发", "base_coef": 60,
                   "attack_weight_pct": 300, "status_duration_ms": 0,
                   "aoe_radius": 120, "spread_stacks": 0, "dispel_shield": true,
                   "amplify_pct": 0, "armor_shred_permille": 0, "knockback": 0,
                   "descr": "焰后冰触发：范围伤害并驱散护盾。" } ] }
```

| 字段 | 说明 |
|---|---|
| `base_coef` | 反应基础系数（千分比） |
| `attack_weight_pct` | 反应伤害中攻击力的占比（千分比），**结构性上限 300（30%）** |
| `status_duration_ms` | 附加状态时长（0 = 无） |
| `aoe_radius` | 溅射半径（像素，0 = 单体） |
| `spread_stacks` | 元素层数传播量（仅 `corrosion_spread`） |
| `dispel_shield` | 是否驱散护盾 |
| `amplify_pct` | 受击伤害放大（仅 `superconduct`） |
| `armor_shred_permille` | 削甲量（仅 `armor_break`） |
| `knockback` | 击退位移（仅 `armor_break`） |
| `descr` | 面向玩家的说明 |

> 数据源是 `domain.AllReactionSpecs()`，与 `testdata/reaction_specs.json` 双向锁定。
> 客户端 `elements.ts` 的 `REACTIONS` 是**手工同步的副本** ——
> 改动任何一侧都必须同步另一侧并重生成契约文件。

### 4.5 用户

#### `GET /api/v1/admin/users?keyword=&limit=50&offset=0`

```json
{
  "items": [
    { "id": 1, "nickname": "玩家1234", "is_guest": true, "status": 1,
      "max_stage": 12, "power": 4567, "coin": 3000, "gem": 50,
      "last_login_at": "…", "created_at": "…",
      "verify_checked": 3, "verify_mismatched": 0 }
  ],
  "total": 120
}
```

> `verify_checked=0 && verify_mismatched=0` 表示**从没被验真过（未知）**，
> 不是「干净」。两个字段必须一起看。

#### `POST /api/v1/admin/users/:id/ban`

```json
// 请求 { "reason": "使用外挂" }
// 响应 { "user": AdminUser }
```

副作用：`status=2` + `ban_reason`，同事务吊销该用户全部 refresh token。
封禁**即时生效**（`ResolveUser` 每次请求查 status）。

#### `POST /api/v1/admin/users/:id/unban`

```json
{ "user": AdminUser }
```

#### `POST /api/v1/admin/users/:id/grant`

```json
// 请求 { "currency": "coin", "amount": 5000 }   // 负数表示回收
// 响应 { "wallet": { "coin": 8000, "gem": 50, "energy": 30, "keys": 1 } }
```

`currency` 白名单：`coin/gem/energy/keys` + `revive_token/mastery_reset/gem_wash_token`。
审计事件名按符号区分：`grant_currency` / `revoke_currency`。

### 4.6 战报与验真

#### `GET /api/v1/admin/battles?user_id=&level_id=&only_mismatched=1&limit=50`

```json
{
  "items": [
    { "id": 456, "user_id": 1, "level_id": 5, "result": "win", "stars": 3,
      "score": 18000, "kills": 42, "leaked": 1, "wave_reached": 5,
      "duration_ms": 95000, "reactions": 26, "heat_max": 88,
      "replay_hash": "…", "created_at": "…",
      "total_enemies": 45, "verify_checked": 1, "verify_mismatched": 0 }
  ],
  "total": 900
}
```

- `user_id` / `level_id` 缺省 0 表示**不过滤**；`?user_id=abc` 或 `?user_id=` → 400
  （不能静默变成「不过滤」）。
- `total_enemies` 用于「守恒」列判断击杀数是否可能。

#### `GET /api/v1/admin/battles/:id`

```json
{ "battle": { … AdminBattle 全字段 + build_snapshot 等 … } }
```

#### `POST /api/v1/admin/battles/:id/verify`

```json
// 请求 { "replay_hash": "0123456789abcdef" }
// 响应 = VerifyResult
```

- **不重算**；调用方需自行在能跑引擎的地方重放后提交 hash。
- 写 `replay_verifications`（`admin_verifier_id`）与 `admin_audit_logs`（`battle_verify`）。

### 4.7 结算重算审计

#### `GET /api/v1/admin/recomputes?user_id=&level_id=&only_mismatch=1&limit=50&offset=0`

```json
{
  "items": [
    { "id": 9, "battle_id": 0, "user_id": 1, "nickname": "…", "level_id": 5,
      "reported": { "score": 18000, "kills": 42, "duration_ms": 95000, "hash": "…" },
      "recomputed": null,
      "mismatched_fields": ["replay_skills"],
      "mode": "enforce", "rejected": true,
      "engine_version": "skills-only", "created_at": "…" }
  ],
  "total": 12
}
```

> `battle_id = 0` 表示**没有战报**（S 段拒绝发生在写战报之前），不是「战报 #0」。
> `recomputed = null` 表示**没算**，不是「重算成 0 分」。

#### `GET /api/v1/admin/recomputes/stats?hours=24`

```json
{
  "mode": "shadow", "configured_mode": "enforce", "demoted": true,
  "demoted_reason": "不符率 12.5% 超过阈值 5%（样本 240）",
  "since": "2026-10-19T12:00:00Z",
  "breaker_rate_permille": 50, "breaker_min_samples": 100,
  "totals": { "total": 240, "mismatched": 30, "rejected": 0, "rate_permille": 125 },
  "levels": [ { "level_id": 96, "total": 40, "mismatched": 20, "rate_permille": 500,
                "max_user_mismatches": 2 } ],
  "engine": {
    "verified": 200, "mismatch": 30, "cannot_run": 5, "skipped": 5,
    "settled_total": 240, "verified_permille": 833,
    "status": "校验正常运行（83.3% 走到重算）"
  }
}
```

- `demoted=true` 表示 `enforce` 已被熔断降级 —— **本该拒发的奖励正在放行**。
- `engine` 与 `totals` 口径不同：前者含所有走到重算的局（进程内计数，从启动算起），
  后者只含台账里的（台账只记不符）。混用会把「校验没跑」误判成「没问题」。

### 4.8 防线 / 经济 / 内容 / 审计

#### `GET /api/v1/admin/defenses?limit=50`

```json
{ "items": [ { "id": 3, "owner_id": 1, "name": "我的防线", "power": 4567,
               "wins": 5, "losses": 2, "shielded_until": "…", "created_at": "…" } ],
  "total": 20 }
```

#### `GET /api/v1/admin/economy`

```json
{ "flows": [ { "id": 1, "user_id": 1, "currency": "coin", "delta": 1200,
               "balance": 3200, "reason": "battle_reward", "ref_id": 456, "created_at": "…" } ],
  "shop": [ { "id": 1, "code": "shop_coin_small", "name": "金币袋",
              "price": { "gem": 10 }, "payload": { "coin": 20000 },
              "limit_per_day": 3, "enabled": true } ] }
```

#### `PUT /api/v1/admin/shop/:id`

字段白名单：`name` / `price` / `payload` / `limit_per_day` / `enabled` / `sort_order`。
响应 `{ "item": … }`，写审计（`update_shop_item`）。

#### `GET /api/v1/admin/announcements` · `POST /api/v1/admin/announcements`

```json
// POST 请求
{ "title": "维护公告", "body": "今晚 23:00 维护", "published": true }
// 响应
{ "item": { "id": 1, "title": "…", "body": "…", "published": true, "published_at": "…" } }
```

#### `GET /api/v1/admin/redeem-codes` · `POST /api/v1/admin/redeem-codes`

```json
// POST 请求
{ "code": "NEWYEAR", "reward": { "coin": 10000, "gem": 20 }, "max_uses": 100, "expires_at": "2026-12-31T00:00:00Z" }
// 响应
{ "item": { "id": 4, "code": "NEWYEAR", "reward": { … }, "max_uses": 100, "used_count": 0 } }
```

#### `GET /api/v1/admin/audit-logs?limit=100`

```json
{ "items": [ { "id": 1, "admin_id": 1, "username": "admin", "action": "update_level",
               "target": "5", "detail": { … }, "ip": "1.2.3.4", "created_at": "…" } ] }
```

审计动作全集：`logout` / `update_level` / `regenerate_levels` / `update_skill` /
`ban_user` / `unban_user` / `grant_currency` / `revoke_currency` / `battle_verify` /
`update_shop_item` / `create_announcement` / `create_redeem_code`。

---

## 5. 限流

| 端点 | 维度 | 额度 | 理由 |
|---|---|---|---|
| `POST /auth/guest` | IP | 60/min | 冷启动集中；NAT 后可能很多玩家 |
| `POST /auth/wechat` | IP | 60/min | 同上 |
| `POST /auth/refresh` | IP | 60/min | 一次冷启动 1 次 |
| `POST /auth/login` | IP | 10/min | bcrypt 最贵；**且测试环境口令与超管相同**（爆破面），但共享账号要照顾整个测试组 |
| `POST /admin/login` | IP | 5/min | bcrypt 最贵；运营人数个位数 |
| `POST /redeem` | IP | 5/min | 挡兑换码爆破 |
| `POST /redeem` | 用户 | 20/min | 挡「换 IP 继续试」 |
| `POST /battle/settle` | 用户 | 60/min | 写放大路径 |
| `POST /battle/verify` | 用户 | 20/min | 写放大路径（无幂等键） |

超限返回 429 + `rate_limited`。**限流是进程内计数**：多副本部署时实际额度是
`N × 配置值`，上多副本前必须换共享计数器。

---

## 6. 422 结算拒绝清单

以下错误统一映射为 `422 settle_rejected`（`isSettleRejection`）：

| 错误 | 触发条件 |
|---|---|
| `ErrTokenNotFound` | token 不存在或不属于本人（404 分支在此之前） |
| `ErrTokenUsed` | 同一 token 重复结算 |
| `ErrTokenExpired` | 超过 `BATTLE_TOKEN_TTL`（30min） |
| `ErrLevelMismatch` | 上报关卡与 token 不符 |
| `ErrTooManyKills` | 击杀数 > 关卡总怪数 |
| `ErrTooManyLeaked` | 漏怪数 > 总怪数 |
| `ErrTooShort` | 时长 < `MIN_BATTLE_DURATION`（15s） |
| `ErrScoreRateExceeded` | 分数超过按关卡推导的速率上限 |
| `ErrInvalidHitRate` | `hits > shots` 或命中率越界 |
| `ErrWaveExceeded` | `wave_reached` > 总波数 |
| `ErrTooManyReactions` | 反应次数 > 理论上限 |
| `ErrTooManyShots` | 发射数 > 时长物理上限 |
| `ErrInvalidField` | 字段非法（集合溢出、时长越界、hash 过长等） |
| `ErrHPLeftExceedsBase` | 剩余血量 > 关卡初始血量 |
| `ErrSlotBudgetExceeded` | 装备件数超过槽位预算 |
| `ErrReplaySkillsMismatch` | S 段与构筑不符（仅 enforce） |

新增 domain 结算错误时必须同步这份清单，否则会掉进 500，
而 500 会让「你作弊了」和「我们坏了」在客户端无法区分。

---

## 7. 兼容性说明

| 场景 | 行为 |
|---|---|
| 老客户端不上报 `replay_skills` | 空串放行（不校验，不拒绝） |
| 老客户端不上报 `card_picks` | 允许，但该战报无法被第三方完整重放 |
| 老战报缺 `build.active_slots` | 引擎缺省 4 槽（历史行为） |
| 客户端用旧 `version` 拉配置 | 服务端回 200 全量 |
| 配置内容未变 | 304（零载荷） |
| 客户端拿不到 `build.attacker` | 本地占位 + 控制台告警（哈希会不符） |

接口演进原则：**新增字段是兼容的，改字段名不是**。改字段名必须同时改
客户端、`cmd/vectors` 夹具与本文档，否则会出现「两端各自自洽、只在真机失败」
的漂移。
