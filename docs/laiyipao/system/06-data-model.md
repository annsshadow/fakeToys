# 06 · 数据模型

> PostgreSQL 15+，**50 张表**，19 个 goose 迁移（`server/migrations/00001…00019`）。
> 迁移通过 `embed.go` 嵌入二进制，运行时从内存读，不依赖部署目录里的 `.sql` 文件。

---

## 1. 分域总览

| 域 | 表数 | 表 |
|---|---|---|
| 账号与鉴权 | 4 | `users` `refresh_tokens` `user_wallets` `game_configs` |
| 内容配置 | 12 | `levels` `level_waves` `enemies` `enemy_element_resist` `skills` `skill_recipes` `equipment` `gems` `gem_affixes` `skins` `mastery_trees` `mastery_nodes` |
| 玩家养成 | 8 | `user_progress` `user_skills` `user_skill_slots` `user_equipment` `user_gems` `user_skins` `user_mastery_nodes` `user_presets` |
| 战斗与验真 | 4 | `battle_tokens` `battle_records` `level_stars` `replay_verifications` |
| 防线 | 4 | `defenses` `defense_challenges` `user_daily_challenges` `defense_works` |
| 经济与运营 | 13 | `wallet_flows` `sign_in_calendar` `user_sign_ins` `tasks` `user_tasks` `shop_items` `user_purchases` `orders` `ad_rewards` `redeem_codes` `redeem_usages` `announcements` `offline_rewards` |
| 后台 | 3 | `admin_users` `admin_tokens` `admin_audit_logs` |
| 道具 | 1 | `user_tokens` |
| 重算审计 | 1 | `settle_recomputes` |

合计 50。

---

## 2. 账号与鉴权

### 2.1 `users`

| 列 | 类型 | 说明 |
|---|---|---|
| `id` | BIGSERIAL PK | |
| `guest_token` | TEXT NOT NULL **UNIQUE** | 游客设备凭证；微信登录后保留（用于数据合并） |
| `openid` / `unionid` | TEXT UNIQUE / TEXT | 微信登录回填 |
| `username` / `password_hash` | TEXT / TEXT | 玩家账号登录（迁移 00016，均可空；NULL = 不能用口令登录） |
| `nickname` | TEXT NOT NULL DEFAULT '' | 上限 64 **字**（服务层校验，不静默截断） |
| `avatar_url` | TEXT | |
| `is_guest` | BOOLEAN | 微信登录后置 false |
| `status` | SMALLINT | 1 正常 / 2 封禁 |
| `ban_reason` | TEXT | |
| `created_at` / `last_login_at` | TIMESTAMPTZ | |

索引：`idx_users_openid (openid) WHERE openid IS NOT NULL`、
`idx_users_username (username) WHERE username IS NOT NULL`（00016，部分唯一索引）。

不变量：

- `guest_token` 唯一且**登出时轮换**。它是凭证，等价于口令。
- `username` 非空则唯一（部分索引）—— 游客号没有用户名，多个 NULL 不冲突。
- 封禁即时生效：`ResolveUser` 每次请求都查 `status`，不依赖 JWT 过期。

### 2.2 `refresh_tokens`

| 列 | 说明 |
|---|---|
| `user_id` | FK → users，ON DELETE CASCADE |
| `token_hash` | TEXT UNIQUE（只存 SHA-256 摘要） |
| `expires_at` / `revoked_at` | 过期 / 吊销时刻 |

刷新是**轮换**：条件 UPDATE（`revoked_at IS NULL AND expires_at > now()`）
+ `RETURNING user_id`，并发下恰好一个成功。

索引：`idx_refresh_tokens_user (user_id)`。

### 2.3 `user_wallets`

| 列 | 类型 | 说明 |
|---|---|---|
| `user_id` | BIGINT PK | |
| `coin` | BIGINT | 基础升级 |
| `gem` | INTEGER | 专精洗练 / 商城 |
| `energy` | INTEGER | 进关 |
| `keys` | INTEGER | 门票 |
| `energy_updated_at` | TIMESTAMPTZ | 体力恢复的**整格推进**基准 |
| `updated_at` | TIMESTAMPTZ | |

约束：`chk_wallet_nonneg CHECK (coin>=0 AND gem>=0 AND energy>=0 AND keys>=0)`。

> 四种货币各只有一个主要用途，这是反通胀的结构性设计（见 `GAME_DESIGN` I-3）。
> 道具（`revive_token` 等）**不在**这张表，见 §8。

### 2.4 `game_configs`

`key` / `version` / `value JSONB` / `updated_at`。

当前实现里 `/config` 由 `domain` 静态生成，这张表是**预留的运营配置位**
（例如将来放活动开关）。读取路径目前不依赖它。

---

## 3. 内容配置

这一组表是**运营与分析的读面**，不是玩家路径的输入源
（`DECISIONS D1`：玩家侧关卡权威 = 生成器）。写入由 `cmd/seed` 完成，全部 upsert。

### 3.1 `levels`（100 行）

| 列 | 说明 |
|---|---|
| `id` | INTEGER PK，1..100 |
| `chapter` | 1..6 |
| `name` / `seed` | 名称 / 生成种子（BIGINT） |
| `base_hp` | 防线血量 |
| `wave_count` | 波数 |
| `difficulty` | 难度系数（千分比） |
| `energy_cost` | 进关体力（缺省 5） |
| `star_targets` | JSONB `[一星, 二星, 三星]` 分数门槛 |
| `terrain_config` | JSONB `[{kind,x,y,param}]` |
| `is_boss` / `enabled` | |

### 3.2 `level_waves`

PK `(level_id, wave_index)`；`spawns JSONB`：

```json
[ { "enemy_id": 1, "count": 6, "interval": 800, "delay": 0 } ]
```

`interval` / `delay` 单位是**毫秒**（曾经错成 tick，差 50 倍）。

### 3.3 `enemies` / `enemy_element_resist`

`enemies`：`id` / `code` UNIQUE / `name` / `category`
（normal/ranged/flying/special/elite/boss）/ `hp` / `speed`（定点 ×1000）/
`armor` / `shield_hp` / `attack` / `attack_range` / `attack_interval`（ms）/
`fly_height` / `burrow` / `is_boss`。

> 落库的是**缩放后**的 `hp`（×8）与 `shield_hp`，与客户端 `/config` 一致。

`enemy_element_resist`：PK `(enemy_id, element)`，`resist_pct` 千分比 `-500..500`
（CHECK 约束）。BOSS 必须同时有高抗与负抗。

### 3.4 `skills` / `skill_recipes`

`skills`：`id` / `code` UNIQUE / `name` / `family`（8 系）/ `element` / `kind`
（active/passive）/ `descr` / `base_damage` / `heat_cost` / `cooldown_ms` /
`pierce` / `aoe_radius` / `apply_element` / `apply_stacks` / `projectile_speed`
（定点 ×1000）/ `chain` / `unlock_level`。

`skill_recipes`：`output_skill_id` / `a_skill_id` / `b_skill_id` / `output_tier`，
`UNIQUE (a_skill_id, b_skill_id)`。

> 合成是**无序**的：服务端用 `LEAST/GREATEST` 抹平 (a,b) 与 (b,a)。

### 3.5 `equipment` / `gems` / `gem_affixes` / `skins`

`equipment`：6 部位（weapon/helmet/armor/gloves/boots/charm）× 3 阶；
`element` 是**契合标签**（与技能同系给大加成，异系给不足 1/3 的小额）。

`gems`：8 属性目录（`code` / `name` / `attr` / `descr`）。
**品质不是目录属性** —— 蓝宝石与金宝石是同一个 `gem_id` 的两个实例，
品质存在 `user_gems.quality`。

`gem_affixes`：洗练词条池 `(gem_id, affix, min_value, max_value)`，
`UNIQUE (gem_id, affix)`（迁移 00014 补）。

`skins`：`code` / `name` / `rarity` / `passive`（机制型词条）/ `unlock_type`。

### 3.6 `mastery_trees` / `mastery_nodes`

`mastery_trees`：`family` PK（flame/frost/volt/kinetic/light/blight/drone/engineering）。

`mastery_nodes`：`id` / `family` / `layer`（1..3）/ `slot`（0..3）/ `name` /
`kind`（element_cap/reaction_mult/heat_cap/extra_slot/mechanic/skill_damage/crit/armor）/
`value` / `prereq_family` / `prereq_layer`，`UNIQUE (family, layer, slot)`。

> 96 行由 `domain.AllMasteryFamilies()` 生成，ID = `familyIndex*100 + layer*10 + slot + 1`。

---

## 4. 玩家养成

| 表 | 主键 | 关键列 | 说明 |
|---|---|---|---|
| `user_progress` | `user_id` | `max_stage` / `level_exp` / `mastery_points`（初始 3，每 10 级 +1）/ `total_battles` / `total_kills` | 关卡解锁与战力榜的依据 |
| `user_skills` | `(user_id, skill_id)` | `level` | 技能等级；等级已烘进底伤 |
| `user_skill_slots` | `(user_id, slot)` | `skill_id` | 0..3 主动槽 + 4 被动槽 |
| `user_equipment` | `(user_id, equipment_id)` | `slot` / `equipped` / `level` / `star` | 部分索引 `WHERE equipped` |
| `user_gems` | `id` BIGSERIAL | `gem_id` / `quality` / `affixes JSONB` / `equipped_slot` | 实例，不是目录 |
| `user_skins` | `(user_id, skin_id)` | `equipped` | |
| `user_mastery_nodes` | `(user_id, node_id)` | — | 已点亮节点 |
| `user_presets` | `id` | `slots JSONB` / `is_current` | 阵型预设；部分唯一索引保证一人一个 current |

`user_gems.affixes` 结构：

```json
[ { "affix": "crit", "value": 42 } ]
```

`user_presets.slots` 结构：

```json
{ "skills": [1,4,7,0], "equipment": [1,10], "gems": [3], "works": ["slow_belt"] }
```

---

## 5. 战斗与验真

### 5.1 `battle_tokens`

| 列 | 说明 |
|---|---|
| `id` | BIGSERIAL PK |
| `user_id` / `level_id` | FK |
| `seed` | BIGINT，服务端生成 |
| `issued_at` / `expires_at` | TTL 30min |
| `used_at` | 结算时置位（一次性凭证的原子判据） |
| `revived` | 本局是否消耗了复活币 |

索引：`idx_battle_tokens_user (user_id, issued_at DESC)`。

### 5.2 `battle_records`

| 列 | 类型 | 说明 |
|---|---|---|
| `id` | BIGSERIAL PK | |
| `user_id` / `level_id` | | `level_id` **不建外键**（关卡是生成物） |
| `battle_token_id` | FK，ON DELETE SET NULL | |
| `result` | TEXT | win / lose |
| `stars` / `score` | | 服务端重算/裁剪后的值 |
| `power_after` | BIGINT | |
| `kills` / `leaked` / `hp_left` / `wave_reached` | INTEGER | |
| `duration_ms` / `shots` / `hits` / `reactions` / `heat_max` | INTEGER | |
| `elements_used` / `reactions_used` | JSONB | `{fire:120}` / `{steam_burst:12}` |
| `terrain_used` | JSONB | `["oil_drum"]` |
| `build_snapshot` | JSONB | 结算时的构筑 + `settle_input` 原文 |
| `replay_hash` | TEXT | 16 位 hex |
| `loot` | JSONB | `{coin:1200,gem:3}` |
| `card_picks` | TEXT | 逗号分隔，`-1` 表示跳过（迁移 00008） |
| `created_at` | TIMESTAMPTZ | |

索引：`(user_id, created_at DESC)`、`(level_id, created_at DESC)`、`(replay_hash)`。

`build_snapshot` 结构（同时也是 `/battle/:id/replay` 的 `build`）：

```json
{
  "skills": { "1": { "id": 1, "name": "燃烧弹", "element": "fire", "kind": "active",
                     "level": 2, "slot": 0, "base_damage": 120, "apply_stacks": 1,
                     "heat_cost": 20 } },
  "elements": ["fire", "ice", "lightning"],
  "equipment": [1, 10],
  "mastery_nodes": [101, 111],
  "gems": [ … ],
  "active_slots": 5,
  "attacker": { "attack": 120, "crit_permille": 50, "…": "…" },
  "settle_input": { … }        // ⚠️ 玩家侧响应里被剥离
}
```

> `settle_input` 只存在于表里，`GetReplay` 会 `delete` 掉它 ——
> 否则读走别人的完整构筑 + 结算报文就是一份「高星战报蓝本」。

### 5.3 `level_stars`

PK `(user_id, level_id)`；`stars` / `best_score` / `clears` / `min_power_clear`。

写入语义（结算事务内）：

- `stars = GREATEST(旧, 新)`
- `best_score = GREATEST(旧, 新)`
- `clears = 旧 + 新`
- `min_power_clear`：新值为 0（失败局）时保持旧值；两侧 > 0 取 `LEAST`

部分索引 `(level_id, min_power_clear) WHERE min_power_clear > 0` 服务效率榜。

> `min_power_clear` 曾长期恒为 0，导致效率榜 `WHERE > 0` 恒零行 ——
> 整个榜单自出生起就是死代码。

### 5.4 `replay_verifications`

| 列 | 说明 |
|---|---|
| `battle_id` | FK → battle_records |
| `verifier_id` | BIGINT **NULL**（玩家侧） |
| `admin_verifier_id` | BIGINT NULL（运营侧，FK → admin_users） |
| `expected_hash` / `actual_hash` / `matched` | |

约束 `replay_verifications_one_verifier CHECK (num_nonnulls(verifier_id, admin_verifier_id) = 1)`。
唯一索引 `idx_replay_verifications_unique
(battle_id, COALESCE(verifier_id, admin_verifier_id), actual_hash)`（迁移 00017）——
同一场战报 + 同一个验真者 + 同一个哈希只留一条；建索引前先删重复行（保留最早）。
建在 `COALESCE(...)` 上是为了让玩家侧与运营侧的验真落在同一个口径里
（理由见迁移 00017 文件头注释）。

> 两个都允许为空会产生「这条验真没有验真人」的行，而这一列的全部价值
> 就是回答「谁验的」。

---

## 6. 防线（I-5）

### 6.1 `defenses`

| 列 | 说明 |
|---|---|
| `id` | BIGSERIAL PK |
| `owner_id` | **UNIQUE**（一人一条，历史快照不保留） |
| `name` / `power` / `element_coverage` / `mastery_done` | |
| `snapshot` | JSONB `{skills,equipment,gems,mastery,works}` |
| `snapshot_hash` | FNV-1a 64（16 位 hex） |
| `wins` / `losses` | |
| `shielded_until` | 24h 护盾 |

索引：`(power DESC)`、UNIQUE `(owner_id)`。

### 6.2 `defense_challenges`

`defense_id` / `challenger_id` / `seed` / `won` / `duration_ms` /
`hp_left_pct`（0..100）/ `replay_hash` / `loot` / `settled`（幂等）。

唯一约束 `(defense_id, challenger_id, created_at)`。

### 6.3 `user_daily_challenges`

PK `(user_id, challenge_date)`；`attempts`（上限 3）/ `stolen_times`（上限 2）。

> `challenge_date` 由 **Go** 传入日期字面量，不用 `CURRENT_DATE`。

### 6.4 `defense_works`

`user_id` / `code`（slow_belt/block_wall/tesla_grid）/ `level` / `slot`（0..2），
唯一索引 `(user_id, slot)`。

---

## 7. 经济与运营

### 7.1 `wallet_flows`

| 列 | 说明 |
|---|---|
| `user_id` | FK |
| `currency` | `coin/gem/energy/keys` 或道具名 |
| `delta` | 正负 |
| `balance` | **变动后**余额 |
| `reason` | `battle_reward` / `battle_start` / `upgrade` / `signin` / `shop` / `redeem` / `admin_grant` / … |
| `ref_id` | 关联对象（battle_id / item_id …） |

索引：`(user_id, created_at DESC)`、`(reason, created_at DESC)`。

> 货币与道具走**同一张流水**，这样运营看板按 `currency` 聚合时
> 「进了多少 / 花了多少」才对称。

### 7.2 签到

`sign_in_calendar`：`day_index` PK（1..7），`reward JSONB`。
种子：`coin = 1000 × day`；第 3/7 天额外 `gem = 20 × day`；第 7 天 `energy = 50`。

`user_sign_ins`：PK `(user_id, sign_date)`，`day_index` / `reward`。
一天一次的保证来自主键，不是「先查后写」。

### 7.3 任务

`tasks`：`id` / `code` UNIQUE / `name` / `scope`（daily/weekly/achievement）/
`target` / `metric`（kills/clears/wins/reactions/signin/max_stage）/ `reward` / `enabled`。

`user_tasks`：PK `(user_id, task_id, task_date)`；`progress` / `claimed_at`。

**`task_date` 的三种语义**：

| scope | `task_date` |
|---|---|
| daily | 今天（`periodStart(now,"daily")`） |
| weekly | 本周一 |
| achievement | **`1970-01-01` 哨兵**（终身累计，迁移 00013 合并历史行） |

> 成就曾经也按天分行，于是可以每天重复领取；迁移把历史行合并进哨兵行，
> `progress` 取 MAX、`claimed_at` 取 MIN（「曾经领过」是既成事实）。

### 7.4 商城与订单

`shop_items`：`id` / `code` / `name` / `category`（currency/boost/keys/starter/gem）/
`price JSONB` / `payload JSONB` / `limit_per_day` / `sort_order` / `enabled` / `updated_at`。

```json
// price 例
{ "gem": 10 }
// payload 例
{ "coin": 20000 }          // 货币
{ "revive_token": 1 }      // 道具
{ "gem_instance": 1 }      // 特殊：铸造一颗带随机词条的宝石实例
```

> 种子刻意**不把 `enabled` 放进 upsert**：后台手动上下架必须能扛住重新灌种子。

`user_purchases`：`user_id` / `item_id` / `purchase_date` / `price`；
索引 `(user_id, item_id, purchase_date)` 服务限购统计。

`orders`：`order_no` UNIQUE / `sku` / `amount`（分）/ `status`
（pending/paid/delivered/refunded）/ `provider`（mock/wechat）/ `provider_txn`。

> 当前只有 mock 通道；接真实支付需商户号，状态机已按真实协议设计。

`ad_rewards`：`kind`（revive/double）/ `battle_id` / `reward`。当前 mock。

### 7.5 兑换码

`redeem_codes`：`id` **GENERATED BY DEFAULT AS IDENTITY**（迁移 00009）/
`code` UNIQUE / `reward JSONB` / `max_uses` / `used_count` /
`expires_at` / `enabled`；CHECK `used_count <= max_uses`。

`redeem_usages`：`(code_id, user_id)` UNIQUE，`created_at`。

> `id` 曾是裸 INTEGER PK，管理端创建兑换码必然 not-null violation → 500。
> 改成 IDENTITY 而非 BIGSERIAL 是为了让 seeder 仍可显式插固定 id。

### 7.6 公告与离线收益

`announcements`：`title` / `body` / `published` / `published_at` / `created_at`。

`offline_rewards`：`user_id` PK / `last_claim_at` / `last_login_at`。
按离线时长结算，领取时清零；上限由 `OFFLINE_CAP_HOURS`（默认 12）约束。

---

## 8. 道具（`user_tokens`）

| 列 | 说明 |
|---|---|
| `user_id` | |
| `token` | `revive_token` / `mastery_reset` / `gem_wash_token` |
| `balance` | CHECK `>= 0` |
| `updated_at` | |

PK `(user_id, token)`；索引 `(token)`。

为什么独立成表而不是给 `user_wallets` 加三列：

1. 语义不同：货币有单价、有通胀控制；道具只有「有没有」和「几个」；
2. 塞进同一张表会让 `SUM(coin)` 之类的聚合悄悄把它们算进去；
3. 新增道具不需要迁移（主键是 `(user_id, token)`）。

消费面（全部已接通）：

| 道具 | 消费端点 |
|---|---|
| `mastery_reset` | `POST /mastery/reset` |
| `revive_token` | `POST /battle/token`（开战扣，本关 +额外血量） |
| `gem_wash_token` | `POST /me/gems/:id/wash` |

发放面：三者的唯一途径都是商城 `shop_items`。

---

## 9. 后台

### 9.1 `admin_users`

`id` / `username` UNIQUE / `password_hash`（bcrypt）/ `role`（admin/ops/readonly）/
`status` / `created_at` / `last_login_at`。

### 9.2 `admin_tokens`

`admin_id` / `token_hash` UNIQUE（sid 的 SHA-256）/ `expires_at` / `revoked_at`。

> JWT 载荷里的 `sid` 是明文，这里存摘要。每次请求都查这一行，
> 所以「登出 / 过期」立刻生效，不必等 JWT TTL。

### 9.3 `admin_audit_logs`

`admin_id`（SET NULL）/ `username` / `action` / `target` / `detail JSONB` / `ip` / `created_at`。

索引：`(created_at DESC)`、`(action, created_at DESC)`。

> 「不可删除」是约定而非数据库权限：没有 DELETE 端点，也没有清理任务。

---

## 10. 重算审计（`settle_recomputes`）

| 列 | 说明 |
|---|---|
| `battle_id` | **不建外键**（战报被清理时不能连带删掉证据） |
| `user_id` / `level_id` | |
| `reported_score` / `reported_kills` / `reported_duration_ms` / `reported_hash` | 客户端上报 |
| `recomputed_hash` / `recomputed_score` / `recomputed_kills` / `recomputed_duration_ms` | 服务端重算（**可为 NULL**） |
| `mismatched_fields` | `TEXT[]`，取值 `score/kills/leaked/duration/hash/replay_skills` |
| `mode` | `shadow` / `enforce`（CHECK） |
| `rejected` | 本次是否真的拒发 |
| `engine_version` | 定位「哪次发布开始大量不符」 |

索引：

- `(user_id, created_at DESC)` —— 查「这个号是不是在作弊」
- `(level_id, created_at DESC) WHERE mismatched_fields <> '{}'` —— 区分「引擎某关算错」与「某玩家全关造假」
- `(created_at DESC)` —— 熔断与看板的时间窗统计

**刻意不建**「只索引不符行」的部分索引：熔断要算「不符数 / 总数」，
只有不符行就没有分母了。

`recomputed_*` 允许 NULL 的理由：引擎未接入时 shadow 记的就是「还没算」。
把两列设成 NOT NULL 会逼迫引擎一上线就得写满，而**恰恰是那段时间最需要它**。

---

## 11. 关键不变量

| 不变量 | 保证方式 |
|---|---|
| 钱包不为负 | `chk_wallet_nonneg` + 扣减用 `WHERE col >= need` |
| 道具不为负 | `chk_user_tokens_nonneg` + 条件 UPDATE |
| 兑换码不超发 | `chk_redeem_uses` + 条件 UPDATE 抢名额 |
| 一人一码 | `redeem_usages (code_id, user_id)` UNIQUE |
| 一天一签 | `user_sign_ins (user_id, sign_date)` PK |
| 一 token 一结算 | `battle_tokens.used_at IS NULL` 条件 UPDATE |
| 一人一防线 | `idx_defenses_owner` UNIQUE |
| 一人一 current 预设 | `idx_user_presets_current` 部分唯一 |
| 验真恰有一个验真人 | `num_nonnulls(verifier_id, admin_verifier_id) = 1` |
| 抗性范围 | `chk_resist_range -500..500` |
| 每层专精 ≤ 2 | 服务层校验（无法用简单约束表达） |
| JSONB 不为 `null` | 服务层 `orEmptyMap` / `orEmptySlice` |

最后一条值得单独说明：`json.Marshal(map[string]int(nil))` 返回 `"null"`，
存进 jsonb 列就是 jsonb `null`。它有两个实测危害：

1. `null <> '{}'::jsonb` 在 SQL 里求值为 **NULL 而不是 true**，
   于是 `WHERE reactions_used <> '{}'` 会静默排除这些行；
2. `jsonb_each_text(jsonb 'null')` 直接报错，任何 JSONB 聚合都会崩。

---

## 12. 迁移清单

| 文件 | 内容 | 是否有损回滚 |
|---|---|---|
| `00001_core.sql` | users / refresh_tokens / user_wallets / game_configs | 否 |
| `00002_game_config.sql` | 关卡 / 波次 / 敌人 / 抗性 / 技能 / 配方 / 装备 / 宝石 / 词条 / 皮肤 / 专精 | 否 |
| `00003_progression.sql` | 进度 / 技能 / 槽位 / 装备 / 宝石实例 / 皮肤 / 专精节点 / 预设 | 否 |
| `00004_battle.sql` | battle_tokens / battle_records / level_stars / replay_verifications | 否 |
| `00005_defense.sql` | defenses / defense_challenges / user_daily_challenges / defense_works | 否 |
| `00006_economy.sql` | 流水 / 签到 / 任务 / 商城 / 订单 / 广告 / 兑换码 / 公告 / 离线 | 否 |
| `00007_admin.sql` | admin_users / admin_tokens / admin_audit_logs | 否 |
| `00008_replay_card_picks.sql` | `battle_records.card_picks` | 否（丢选牌决策） |
| `00009_redeem_code_identity.sql` | `redeem_codes.id` → IDENTITY | 否 |
| `00010_jsonb_null_normalize.sql` | 把存量 jsonb `null` 归一为 `{}` / `[]` | 是（无法区分原本的 null） |
| `00011_verification_admin_verifier.sql` | `verifier_id` 可空 + `admin_verifier_id` + CHECK | **是**（删运营验真记录） |
| `00012_wallet_tokens.sql` | `user_tokens` | 否 |
| `00013_achievement_epoch.sql` | 成就合并到哨兵日期 | **是**（历史周期不可还原） |
| `00014_gem_affix_pool.sql` | `gem_affixes` 唯一约束 + `gems.descr` | **是**（丢已洗练词条） |
| `00015_settle_recompute_audit.sql` | `settle_recomputes` | 是（丢历史证据） |
| `00016_player_account.sql` | `users.username` / `password_hash` + 部分唯一索引 | **是**（共享账号退回登不上的游客号） |
| `00017_verification_idempotency.sql` | `replay_verifications` 幂等唯一索引（先删重复行） | 否（Up 删的是重复行，Down 只删索引） |
| `00018_verification_verifier_kind.sql` | 订正 00017 把「玩家 `users.id`」与「运营 `admin_users.id`」两个独立 ID 空间折叠进同一个 COALESCE 唯一索引的碰撞：先按各自空集合重新去重，再删旧索引、改建**两条部分唯一索引**（玩家 `WHERE admin_verifier_id IS NULL` / 运营 `WHERE verifier_id IS NULL`） | 是（Down 段恢复 00017 的 COALESCE 单索引；已被 00017 误删的重复行找不回来，且重 Up 前需先删重复行，否则部分索引建成即报冲突） |
| `00019_dashboard_indexes.sql` | 看板时间范围扫描的索引：`battle_records(created_at, user_id)`（14 天 DAU 走 index-only scan）+ `wallet_flows(created_at)`（货币净流量聚合） | 否（纯 `CREATE INDEX` / `DROP INDEX`，不动数据；非 CONCURRENTLY 是刻意的——本仓迁移在单事务里跑） |

有损回滚必须在迁移文件里写明「为什么不对称」—— 否则半年后
下一个人会以为只是漏写了 Down。

迁移执行：

```bash
# 应用全部
go run ./cmd/migrate

# 回滚一步（仅本地调试）
go run ./cmd/migrate -down
```

生产环境不建议 `-down`：先看 `OPERATIONS.md` §5.3 与 §6（备份恢复）。

---

## 13. 数据保留与清理

当前**没有**自动清理任务。几类会持续增长的表：

| 表 | 增长来源 | 建议 |
|---|---|---|
| `battle_records` | 每局一行 | 按 `created_at` 归档；注意 `settle_recomputes` 不建外键，归档不会连带删证据 |
| `replay_verifications` | 每次验真一行（00017 起同人同哈希幂等，重复行已在迁移中清理） | 长期可按 `created_at` 归档 |
| `wallet_flows` | 每笔货币变动 | 运营分析需要，建议保留 |
| `admin_audit_logs` | 每次后台写 | **不建议删**（审计） |
| `user_daily_challenges` | 每人每天一行 | 可归档 90 天前 |
| `settle_recomputes` | 每次不符一行 | 保留（回滚时的唯一证据） |

`battle_records` 被清理时不会影响 `settle_recomputes`（刻意不建外键），
但会让「按 battle_id 回查原始上报」失效 —— 归档前先确认台账已完整。
