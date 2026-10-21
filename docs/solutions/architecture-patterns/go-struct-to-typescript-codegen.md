---
title: "用 Go struct 生成 TypeScript：把「契约漂移」从运行时问题变成编译期问题"
date: 2026-10-21
last_refreshed: 2026-10-21
category: architecture-patterns
module: laiyipao（server + admin）
problem_type: architecture_decision
component: api_contract
severity: high
symptoms:
  - "后台表格/抽屉的列「看起来有数据、其实恒空」：战报详情 14/16 字段空、防线属主/快照哈希两列空、宝石「效果」列显示的是说明文字"
  - "每一处都有测试，而且测试全绿 —— 因为前端 fixture 按前端自己编的形状写，与线上跑的服务端形状一致地错"
  - "前端用 Record<string, unknown> 承接服务端 JSON，字段名拼错不报错"
  - "服务端 15 个端点的响应是手写的 map[string]any，形状由裸 SQL 决定，前后端两侧都没有事实源"
---

# 用 Go struct 生成 TypeScript：把「契约漂移」从运行时问题变成编译期问题

## 症状

后台有 4 处「看起来有数据、其实是空的」：

| 位置 | 症状 | 真实原因 |
|---|---|---|
| 战报详情抽屉 | 14/16 字段恒空 | 服务端返回 `ReplayInfo`，前端按 `AdminBattle` 读 |
| 防线「属主」列 | 恒空 | 前端读 `user_id`，服务端给 `owner_id` |
| 防线「快照哈希」列 | 恒空 | 服务端**根本没返回**这一列 |
| 宝石「效果」列 | 显示的是说明文字 | 两列同绑 `descr`，真实字段是 `attr` |

**共同的隐蔽之处**：每一处都有测试，而且测试**全绿**。

## 根因（为什么绿测试抓不到）

三层叠加，缺一不可：

1. **服务端没有类型**：15 个后台端点的响应是手写的 `map[string]any`，
   形状由裸 SQL 的 `SELECT` 列表和 map 字面量共同决定。
   → 形状是**运行时的**，编译期无从校验。
2. **前端用 `Record<string, unknown>` 承接**：
   → 字段名拼错**不报错**，`row.user_id` 静默变成 `undefined`。
3. **测试 fixture 按前端形状写**：
   → 测试断言的是「前端自己编的形状」，与线上那个形状一致地错。

第 3 条是关键：**fixture 让缺陷变成「自洽的」**。
改前端字段名时测试会红（说明它测的是前端形状），
但线上早就坏了 —— 因为线上跑的是**服务端**的形状。

## 做法

### 1. Go 侧：响应形状变成类型

```go
// server/internal/service/adminviews.go
type AdminDefenseView struct {
    ID          int64  `json:"id"`
    OwnerID     int64  `json:"owner_id"`   // ← 前端曾读 user_id
    OwnerName   string `json:"owner_name"`
    SnapshotHash string `json:"snapshot_hash"` // ← 此前根本不返回
    ...
}
```

**关键：连「信封」也要有类型**。视图行有类型还不够 ——
handler 还要在外面包一层 `{"items": [...], "total": n}`，
那层包装此前也是手写的，而前端写的**正是这层**。

### 2. 反射式生成器（`internal/tygen`）

用 `reflect` 而不是手写解析器：Go 的 `encoding/json` 用的就是
「json tag 优先、`-` 跳过、匿名嵌入 struct 平铺」这套规则，
所以反射的结果**与 json.Marshal 的行为一致** ——
而这正是我们要的：TS 类型必须描述**线上真实的 JSON**。

类型映射要点：

| Go | TS | 备注 |
|---|---|---|
| `*T` | `T \| null` | 漏成 `T` 会让前端在 null 上崩 |
| `map[string]T` | `{ [key: string]: T }` | |
| `any` | `unknown` | 猜成 `any` 等于放弃检查 |
| `[]byte` | `string` | Go 的 json 把它编码成 base64 |
| `time.Time` | `string` | RFC3339 |
| 具名 struct | 引用注册表里的名字 | 不内联展开 |

### 3. 守卫：**生成物逐字节比对**

```go
// server/internal/tygen/tygen_test.go
want, _ := apitypes.AdminRegistry.Generate(apitypes.Header)
got, _ := os.ReadFile(paths.AdminTypes)
if string(got) != want { t.Errorf("生成物已过期…") }
```

**为什么必须入库而不是构建时生成**：CI 里前端只做 `type-check`，
生成物入库才能让「类型漂移」在 PR 上**立刻可见**。
CI 再加一步 `go run ./cmd/tygen -check`（非零退出 = 过期）。

### 4. 注册表要「点名」

```go
// server/internal/apitypes/apitypes.go
var AdminRegistry = tygen.Registry{
    {Name: "AdminLevelView", Go: service.AdminLevelView{}},
    ...
}
```

新增响应 struct 而忘了登记 → 守卫红。
**光有注册表不够**：还要有一张「哪些类型必须被登记」的清单，
否则「忘了登记」这个动作本身没有守卫（它表现为前端缺一个类型，
而前端可以自己编一个 `Record<string, unknown>`，于是漂移悄悄回来）。

## 效果

- 改一个 json tag → **两端同时红**（Go 编译期 + 前端 type-check）。
- 「恒空列」这类缺陷**结构上不可能再发生**：
  前端读的字段名由生成器从服务端的 tag 派生，两者永远一致。

## 踩过的坑

### 坑 1：只类型化「行」，忘了「信封」

行有类型、handler 还在手写 `fiber.Map{"items": items, "total": total}` →
前端写的**是信封**，所以信封才是真正要类型化的那层。
→ 两个端点（列表/详情）如果各包各的，形状必然漂移。

### 坑 2：生成物的路径用相对路径

`../../admin/src/api/types.gen.ts` 在 `go run` / `go test` / IDE 里
工作目录各不相同 —— 实测把文件写到了**仓库根下面多出一个 `admin/` 目录**。

正确做法：**从 `go.mod` 反推**。向上找含 `module github.com/laiyipao/server`
的 `go.mod`，admin 就在它旁边。

### 坑 3：把 `map[string]any` 换成 struct 时，忘了 JSON 键名要逐字对齐

Go struct 的 json tag 就是线上 JSON 的键名，
**不许在 handler 里再包一层别的名字**。
改名前后必须逐字比对（`strconv.FormatInt(seed, 10)` 那类
「字符串 vs 数字」的约定也要保留 —— seed 超过 2^53 会让 JS 失真）。

### 坑 4：测试 fixture 必须改成**服务端真实形状**

这是最容易漏的一步。不改 fixture 的话：
新加的守卫会**全绿**（因为 fixture 仍然是前端形状），
而线上仍然是坏的 —— 守卫成了摆设。

判据：**fixture 是契约，不是测试的自便**。

## 推广

同一套做法适用于任何「Go 后端 + TS 前端、字段名漂移过」的项目。
关键不在生成器本身（那部分很短），而在**三件事同时做**：

1. 服务端响应变成**具名类型**（含信封）；
2. 前端**不允许**手写响应类型（改成 import 生成物）；
3. fixture 改成**服务端形状**（否则前两件白做）。
