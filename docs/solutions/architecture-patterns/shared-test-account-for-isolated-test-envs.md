---
title: "测试环境的共享账号登录：让全体测试者共用一个玩家身份"
date: 2026-10-21
last_refreshed: 2026-10-21
category: architecture-patterns
module: laiyipao/server + laiyipao/miniapp
problem_type: architecture_pattern
component: authentication
severity: medium
symptoms:
  - "玩家侧只有游客身份 ⇒ 每个浏览器/设备一个号，测试数据（金币/关卡进度/专精/战报）散在无数个 users 行里"
  - "换浏览器或清一次缓存就是一个新号 ⇒ 「我刚打的关卡呢」变成一个无法解释的 bug"
  - "多人同时测时互相看不见对方的进度，运营无法复现「玩家卡在第 27 关」"
root_cause: wrong_identity_model_for_shared_environment
resolution_type: feature_with_server_side_gate
tags: [test-env, shared-account, guest-login, login-gate, rate-limit, secret-hygiene]
applies_when:
  - "一个环境要供多人反复测试，且需要共享同一份账号数据"
  - "游客/匿名身份导致数据分散、进度无法延续"
  - "同一套后端既要支持游客，又要支持固定账号"
related_components:
  - laiyipao/server/internal/service/player_account.go
  - laiyipao/server/internal/httpapi/routes.go
  - laiyipao/miniapp/src/pages/login/login.vue
  - laiyipao/miniapp/src/store/game.ts
  - docs/laiyipao/OPERATIONS.md
---

# 测试环境的共享账号登录

## Problem

玩家侧原本只有两种身份：

| 身份 | 载体 | 特性 |
|---|---|---|
| 游客 | `users.guest_token`（客户端 localStorage） | **每设备一个**，换设备即换号 |
| 微信 | `users.openid` | 需要 AppID/Secret（本仓未接通） |

游客通道对「随手玩两下」是合适的，对**共享测试环境**不合适：
数据散落、进度不连续、互相看不见。

## Resolution

### 三层

1. **身份**：玩家账号（`users.username` + `users.password_hash`，均可空 ——
   存量游客号没有用户名）。令牌与游客登录**同构**（`kind=user`），权限面完全一样。
2. **开关**：服务端 `PLAYER_LOGIN_REQUIRED`（缺省关闭）。打开后
   `POST /api/v1/auth/guest` 一律 **403 `login_required`**，且**不建号**；
   启动时引导一个**与运营超管同名同口令**的共享测试账号。
3. **客户端闸门**：H5 出登录页；冷启动若本地没有可用令牌，**绝不再打
   `/auth/guest`**（那会建出一个新的游客号）。

### 三条硬规矩

| 规矩 | 为什么 |
|---|---|
| **服务端那道 403 才是权威判据** | 只在客户端挡（弹登录页）不够：浏览器里缓存着旧 bundle 的测试者仍会调 `/auth/guest` 建新号，老问题会在**看起来已经修好**之后继续存在 |
| **口令绝不进前端产物** | H5 产物是公开可下载的；口令进去等于把超管口令公开（它同时是测试账号的口令）。只回填**用户名**（不是秘密，且省一次输入） |
| **共享账号已存在时只同步口令，绝不重建** | 重建 = 清空钱包/进度/专精/战报，而「测试数据不要丢」正是这个功能要解决的事 |

### 两条安全取舍（明说，不藏）

- 这个登录端点同时是**超管口令的爆破面**（口令与超管相同）→
  限流 **10/min/IP**（游客建号是 60），且 bcrypt cost 10；
- 用户不存在与口令错误返回**同一条**错误（防用户名枚举），且**先验口令再查状态**
  —— 反过来会让「账号已停用」成为枚举信号。

### 代价与取舍

- 首次访问控制台会有一条 403：客户端先试 `/auth/guest` 撞到 403 才跳登录页。
  **没有**改成「先读本地配置缓存判断」—— 那会让「服务端关掉开关后，老客户端
  卡在登录页出不来」（缓存副本过期且没人能刷新它）。
- 共用账号意味着**共用钱包**：多人同时测会互相看到体力/金币消耗 —— 这正是
  「数据聚在一处」的代价，也是它的收益。

## When to Apply

- 共享测试/演示环境，需要「全体测试者 = 一个身份、一份数据」；
- 同一套后端既要支持游客，又要支持固定账号（用开关而不是二选一）；
- 想让「数据散落」变成「数据聚在一处」。

## Related

- 部署与运维：`docs/laiyipao/OPERATIONS.md`「`PLAYER_LOGIN_REQUIRED` 怎么用」
- 接口契约：`docs/laiyipao/system/05-api-reference.md`「`POST /api/v1/auth/login`」
- 实现：`laiyipao/server/internal/service/player_account.go`
