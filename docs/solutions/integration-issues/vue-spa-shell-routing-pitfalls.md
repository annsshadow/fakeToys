---
title: "Vue 3 SPA 应用壳三大坑：空路径子路由无限重定向、v-for/v-if 同元素、TanStack Query v5 签名"
date: 2026-09-10
last_refreshed: 2026-10-14
category: integration-issues
module: oa4rust/frontend/apps/desktop
problem_type: bug_pattern
component: frontend_routing
severity: medium
symptoms:
  - "所有 /app/* 深链进来 #o2-app-root 空白，控制台 RangeError: Maximum call stack size exceeded"
  - "AppShell 导航项全部不渲染"
  - "Dashboard setup 崩溃：client.defaultQueryOptions is not a function"
root_cause: vue3_essential_rule_violations
resolution_type: code_fix
tags: [vue-router, tanstack-query, pinia, vue3, spa-shell, infinite-redirect]
applies_when:
  - "Vue 3 + Vue Router 4 history 模式的应用壳设计"
  - "迁移 Pinia / TanStack Query v5 到现有 Vue 3 工程"
  - "排查「生产空白但开发正常」类缺陷"
related_components:
  - oa4rust/frontend/apps/desktop/src/main.ts
  - oa4rust/frontend/apps/desktop/src/views/AppShell.vue
  - oa4rust/frontend/apps/desktop/src/views/Dashboard.vue
---

# Vue 3 SPA 应用壳三大坑

## Problem

三个独立缺陷叠加导致「未认证深链跳登录正常、登录后应用壳空白」——每层单看都像配置问题，叠在一起没有任何一层能渲染。

## Resolution

### 坑 1：参数化父路由 + 空路径子路由 = 无限重定向

```ts
// 错误：父声明为参数化
{ path: '/app/:appId', children: [{ path: '', redirect: '/app/dashboard' }] }
```

子匹配器正则 `^/app/([^/]+?)\/?$` 会先匹配到 `/app/dashboard` 本身（把 dashboard 当 appId），于是 `/app/dashboard` → redirect → 又匹配 → `pushWithRedirect` 无限递归，RangeError，根节点空白。**任何深链都触发**，开发时若不测深链只在首页打转则完全不可见。

修复：父改静态 `/app`，子路由解析为 `/app/dashboard`、`/app/org`…；空路径子路由保留但命名（`AppShellHome`）消除 vue-router 空路径警告。

### 坑 2：v-for 与 v-if 同元素（Vue 3 Essential Rule B2）

`v-if` 在 Vue 3 中**无法访问 v-for 的别名**（优先级分离），同元素写两者时条件恒按"无别名"求值——导航项全部静默不渲染，无警告（取决于构建配置）。修复：先 `computed` 过滤出 `enabledNavItems` / `sectionNavItems` 再 v-for。

### 坑 3：TanStack Query v5 的 useQuery 单参数签名

```ts
// 错误：第二个参数（options 对象）遮蔽了注入的 QueryClient
useQuery(options, queryClient)
```

v5 是单参数 `useQuery(options & {queryKey, queryFn})`；多传的对象会被当 QueryClient，setup 崩溃 `client.defaultQueryOptions is not a function`。合并进 options 包即可。同类隐性坑：Pinia 的 `store.state` 是 ref，模板外访问用 `session.state.user` 而非 `session.state.value?.user`（对象字面量 getter 快照恒 false）。

## 验证方法

浏览器黑盒（Chromium）：未认证深链落 `/login?redirect=/app/dashboard`（无 RangeError）；登录渲染壳 + dashboard；`/app/org` 导航；logout 回登录且 who 为 anonymous。**四个断言缺一不可**——只测"登录能进首页"会漏掉全部三个坑。

## Why This Matters

这三个都是"升级框架大版本"类改动的典型残留：路由从 hash 改 history、状态库迁 Pinia、请求层迁 TanStack v5。单个看都是一行修复，叠加后是"应用完全不可用且无任何报错指向真因"。排障顺序：控制台 RangeError → 路由表；setup 崩溃 → 库签名；元素不渲染 → 模板规则。

## When to Apply

- Vue 3 应用壳/导航设计
- 框架大版本迁移后的冒烟清单
- 排查"生产空白、开发正常"类缺陷

## Related

- 计划：`docs/plans/2026-09-09-001-...-biome-2-5-11-plan.md`「/app/* blank-shell defect fixed」节
- HttpOnly Cookie 迁移同计划的 Pinia 解包坑：`../security-issues/browser-session-cookie-httponly-migration.md`
