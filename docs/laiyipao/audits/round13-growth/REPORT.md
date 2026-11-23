# R13 后端成长引导契约与验证记录

## 背景

R12 实测（docs/laiyipao/audits/round12-growth-guidance/README.md）暴露两个真实缺口：

1. 新号/低战报数时，rec-v1 只返回 `NO_HISTORY`，H5 四成长页只能显示诚实空态，没有可执行建议。
2. 升级响应只带成功状态，客户端无法展示即时战力变化。

本轮只改 Go 后端，不放宽任何校验、不编造收益、不写真实环境值。

## 接口契约

### 1. GET /api/v1/me/recommendations?context=<...>

沿用 rec-v1 envelope，不新增顶层必填字段；仅在**近 7 天战报数 < 3**时追加可选字段 `cold_start`：

```json
{
  "schema_version": "rec-v1",
  "empty_reason": {"code": "NO_HISTORY", "message": "..."},
  "cold_start": {
    "based_on": "rule_only",
    "label": "基于规则而非你的历史战报",
    "threshold": 3,
    "battle_count": 0,
    "config_version": "<当前配置版本>",
    "candidates": [
      {
        "id": "skill-upgrade-1",
        "headline": "...",
        "basis": ["..."],
        "cta": {"label": "...", "route": "/pages/...", "params": {"...": "..."}},
        "source": {
          "config_version": "<当前配置版本>",
          "rule": "lowest_level_owned_active_skill",
          "fields": ["build.skills[].level", "skill_rules.base_cost", "..."]
        }
      }
    ]
  }
}
```

- `battle_count` 来自 `battle_records` 实读；`threshold=3` 是固定规则阈值。
- `candidates` 只从**当前构筑快照 + /api/v1/config 的真实数值**推导；规则名与字段来源逐条落库在 `source`。
- 已有三条规则：`lowest_level_owned_active_skill`（升级已拥有未满级主战技能）、`lowest_bonus_equipped_item`（检查已装备最低加成项的同槽搭配）、`first_config_gem_attribute`（配置宝石属性引导，明确不承诺战力）。
- 达到 3 场后不再下发 `cold_start`，既有 rec-v1 insights 行为完全保留。
- `NO_HISTORY` 诚实空态保留；规则候选是补充，不伪装成战绩结论。

### 2. POST /api/v1/me/skills/{id}/upgrade

原有 `skill_id`/`level`/`wallet` 不变，新增：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `power_before` | number | 升级前由 `service.ComputePowerFor(build)` 计算 |
| `power_after` | number | 升级后由同一函数计算 |
| `power_method` | string | `service.ComputePowerFor(build) / domain.computePower` |
| `power_snapshot` | object | 兼容形状：`applicable=true`, `before`, `after`, `method` |

### 3. POST /api/v1/me/upgrade（游客转正）

账号面变更不改变构筑，新增字段全部诚实标注不适用：

```json
{"ok": true, "power_before": null, "power_after": null,
 "power_method": "not_applicable",
 "power_snapshot": {"applicable": false, "before": null, "after": null,
                    "note": "游客转正只改账号凭据，不改变技能与专精构筑，战力不适用"}}
```

## test-first 证据

新增 E2E：`laiyipao/server/internal/httpapi/routes_r13_growth_snapshot_e2e_test.go`（真实 Fiber app.Test + 真实 PostgreSQL）。

- RED 1：`响应缺少 cold_start 对象`（0 战报）；`升级响应缺少 power_snapshot`（技能升级）；`账号升级响应缺少 power_snapshot`（游客转正）。
- RED 2（对齐任务契约字段名）：`契约字段 power_before 必须与同一战力函数一致：<nil>`；`不适用分支的 power_method 必须标注 not_applicable：<nil>`。
- GREEN：三条用例全部 PASS；其中冷启动用例覆盖 0/2/3 场阈值与字段来源，技能升级用例对齐 `/api/v1/me/` 的战力值，游客转正则断言 null 与不适用说明。

## 不破坏性验证

- `go test ./internal/httpapi -run 'TestE2EColdStartGrowthCandidates|TestE2EUpgradeSkillPowerSnapshot|TestE2ECosmeticUpgradePowerNotApplicable' -count=1 -v`：PASS（0.830s）。
- 全仓串行 `go test ./... -count=1 -p 1 -parallel 1 -timeout 30m`：见下方「全量结果」。
- `gofmt -l internal/service/growth_cold_start.go internal/service/recommendations.go internal/httpapi/routes.go internal/httpapi/routes_r13_growth_snapshot_e2e_test.go`：无输出。
- `go vet ./internal/service ./internal/httpapi`：exit 0。
- 环境排障：首次全仓失败并非本轮代码所致。① key=value 形式 DSN 会让 scratchdb 的 `AdminDSN`/`swapDB` 失效，一次性库 DSN 仍指向原测试库；② 原测试库虽有 31 条 goose 版本记录，却缺 `redeem_codes` 等表（历史遗留的半迁移状态），导致 httpapi E2E 500。处理：建一张全新的只用于验证的库，执行 `go run ./cmd/migrate` 与 `go run ./cmd/seed`，并以 URL 形式 DSN 重跑全仓；未改任何生产迁移或业务代码。

## 全量结果

首次全仓串行因测试库环境失败（见「不破坏性验证」排障说明）：service/store 在修正 DSN 形式后已全绿，httpapi 因原测试库历史半迁移状态缺 `redeem_codes` 而失败。已新建完整迁移+种子的独立验证库并启动最终全仓串行重跑；本轮改动未触碰迁移与防作弊逻辑。

## 零真值声明

本文件与本次改动均不包含真实域名/IP/口令/私钥；测试数据库连接只从本地环境变量注入，输出经掩码处理。
