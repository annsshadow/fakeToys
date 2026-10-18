---
title: "REST API 路径白名单与写操作鉴权（路径穿越/限流未注册/numpy 序列化潜伏炸弹）"
date: 2026-09-21
last_refreshed: 2026-10-14
category: security-issues
module: augmentor/api
problem_type: security_issue
component: rest_api
severity: critical
symptoms:
  - "POST /api/quality/evaluate {\"input_file\": \"C:\\\\Users\\\\<user>\\\\OUTSIDE_PROJECT_SECRET.json\"} 返回 200 且 total_samples=7——项目目录外任意文件被读"
  - "POST /api/data/export、POST /api/augment/start 可任意写；GET/PUT/DELETE /api/data/load/{filename} 绝对路径逃逸"
  - "RateLimitMiddleware 完整实现且有测试，但 app.user_middleware 里根本没有它——44 路由无限流"
  - "全部 44 路由无认证：POST /api/config 可改运行时配置，DELETE /api/data/delete/{filename} 可删数据"
  - "QualityScore.passed 声明 bool 实为 np.bool_，json.dumps 直接 TypeError（潜伏，聚合路径未引爆）"
  - "ModelBackend.__del__ 在构造失败时抛 AttributeError，污染解释器退出"
root_cause: missing_path_validation_and_authz
resolution_type: code_fix
tags: [path-traversal, authz, rate-limit, fastapi, numpy-bool, destructor-safety]
applies_when:
  - "REST API 接受文件路径参数"
  - "内部工具从「零配置开箱」向「暴露网络」演进"
  - "用 numpy/pandas 的数据结构进 API 响应"
related_components:
  - augmentor/api/deps.py
  - augmentor/api/routes/data.py
  - augmentor/api/main.py
  - augmentor/augmentor/models/base.py
  - augmentor/augmentor/quality.py
---

# REST API 路径白名单与写操作鉴权

## Problem

三个独立缺陷叠加成「任意文件读写 + 无认证 + 无限流」的内部工具暴露面：路径入参只判存在性、白名单校验只拦 `..` 不限定基目录；限流中间件写了没注册；写路由逐个手写 `Depends(verify_api_key)` 容易漏挂。另有类型正确性潜伏炸弹（np.bool_）与析构安全（__del__）。

## Resolution

### 路径白名单：唯一入口 + 分层解析

`api/deps.py` 收敛三个层级，全部路由/CLI 共用：

| 函数 | 语义 | 用途 |
|------|------|------|
| `resolve_data_path(name, *, for_write=False)` | 必须存在 + 白名单内 | 读写数据文件 |
| `resolve_within_roots(name)` | 只校验边界、不要求存在 | 多模态降级路径、导出目标 |
| `resolve_data_dir(name)` | 目录级校验 | 批量导出 |

- 基目录用 `Path(base).resolve()` + `candidate.is_relative_to(base)`；**拒绝绝对路径、非 `.json` 扩展名、符号链接**三种逃逸；`web.data_roots` 配置 + `AUGMENTOR_DATA_ROOTS` 环境变量可配多根。
- `api/routes/data.py:_safe_data_path` 改为委托同一入口，消灭第二套校验口径。
- 前端调用方先统计实际传入路径形态再启用白名单（内部工具前端传绝对路径是真实风险，先修前端或配置 data_roots）。

### 写操作鉴权：从"逐个挂载"到"元测试固化"

- `verify_api_key`（`X-API-Key` 比对 `AUGMENTOR_API_KEY`），写操作强制、读操作可选（`web.auth.require_for_writes`）。
- **"生产未配置密钥则启动失败"与"零配置开箱可用"冲突时，显式选 WARNING 放行并在架构文档写明边界**——不要假装两个目标都满足。
- 防漏挂：`tests/integration/test_api_security.py::TestWriteRouteAuthCoverage` 把全部 42 条路由的鉴权状态固化为元测试（11 保护 / 31 只读白名单），**新增路由必须显式分类，漏挂即红**；已验证临时摘 Depends 后测试点名 `('POST', '/api/config')` 失败。

### 限流注册 + 配置化

`RateLimitMiddleware` 注册进 `app.user_middleware`，参数读 `web.rate_limit`（max_requests / window_seconds / exempt_paths:["/api/health"]）；集成测试断言第 N+1 次请求 429 + `Retry-After`。"已实现未注册"类缺陷用 `assert any(isinstance(m, RateLimitMiddleware) for m in app.user_middleware)` 防回归。

### 类型与析构安全

- `passed=bool(total_score >= self.threshold)`：构造点收敛 np.bool_ → Python bool（np.float64 是 float 子类无需转）。只要有人把 `QualityScore` 直接放进响应/落盘就 500——潜伏炸弹要在构造点拆，不在序列化层兜底。
- `close()` 用 `session = getattr(self, "_session", None)` + try/except 兜底；`__del__` 中抛异常会污染解释器退出流程（PytestUnraisableExceptionWarning 就是信号）；全仓扫描同类 `__del__`。

## Why This Matters

"内部工具"是权限模型最容易腐烂的地方：零配置约定会阻止启动 fail-closed，手写鉴权会漏挂，路径校验会长出第二套口径。白名单唯一入口 + 元测试固化 + 限流注册断言，把这三类"写了但没生效"变成不可能静默存在。

## When to Apply

- 任何接受文件路径的 FastAPI/Flask 路由
- 内部工具要暴露到网络（哪怕是内网）
- 审查 numpy/pandas 类型流入 JSON 响应的边界

## Related

- 计划：`docs/archived-plans/2026-09-21-001-audit-augmentor-fullstack-optimization-plan.md`（F1-F5、F12、F13、Phase 0 执行状态与偏差登记）
- 测试卫生：`../best-practices/test-hygiene-ast-triage-and-coverage-traps.md`
