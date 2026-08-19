# Performance Baseline & Capacity Report

## 1. 测试目的

建立 oa4rust（Rust / Axum）核心链路的性能基线，并与 o2server（Java）同等场景进行对比，为后续容量规划、灰度切换决策提供数据依据。

## 2. 测试环境

| 项目 | 值 |
|------|-----|
| 压测工具 | aiohttp + asyncio（wrk / ab 未就绪时使用） |
| Rust 后端 | oa4rust，端口 3000 |
| Java 后端 | o2server，端口 8080（或 20020） |
| 数据库 | PostgreSQL 14 |
| 并发模型 | 异步 HTTP 客户端，信号量控制并发度 |
| 测试数据 | 预热阶段预创建；压测阶段动态生成 ID |

## 3. 压测场景

| 场景 | 端点 | 认证 | 说明 |
|------|------|------|------|
| 登录 | `POST /jaxrs/authentication/login` | 无需 | 每次请求携带 credential + password |
| 流程发起 | `POST /jaxrs/work/{id}/start` | 需要 | 预热时批量创建 pending work，压测时直接启动 |
| CMS 发布 | `POST /jaxrs/cms_assemble_control/data/document/create` | 需要 | 每次生成唯一 document ID |

## 4. 如何运行压测

### 4.1 单场景压测

```bash
# 登录
python scripts/benchmark.py --base-url http://localhost:3000 \
    --concurrency 50 --requests 1000 --scenario login

# 流程发起
python scripts/benchmark.py --base-url http://localhost:3000 \
    --concurrency 50 --requests 1000 --scenario work-start

# CMS 发布
python scripts/benchmark.py --base-url http://localhost:3000 \
    --concurrency 50 --requests 1000 --scenario cms
```

### 4.2 全场景压测

```bash
python scripts/benchmark.py --base-url http://localhost:3000 \
    --concurrency 50 --requests 500 --all
```

### 4.3 时长压测

```bash
python scripts/benchmark.py --base-url http://localhost:3000 \
    --concurrency 50 --duration 30 --scenario login
```

### 4.4 对比压测

```bash
python scripts/compare_o2server.py \
    --rust-url http://localhost:3000 \
    --java-url http://localhost:8080 \
    --concurrency 50 --requests 500
```

## 5. 输出指标

每个场景输出：

```
=== {scenario} ===
  Total requests : {total}
  Success        : {success}
  Errors         : {errors}
  Error rate     : {error_rate}%
  QPS            : {qps}
  Latency P50    : {p50} ms
  Latency P95    : {p95} ms
  Latency P99    : {p99} ms
```

JSON 输出文件（`benchmark-results.json`）结构：

```json
[
  {
    "scenario": "login",
    "total": 1000,
    "success": 1000,
    "errors": 0,
    "error_rate_pct": 0.0,
    "qps": 1250.50,
    "p50_ms": 3.21,
    "p95_ms": 6.45,
    "p99_ms": 8.90
  }
]
```

## 6. 当前基线（示例 / 待实际运行填充）

> **NOTE**: 以下数据为占位示例。请在实际运行压测后替换为真实数据。

### 6.1 Rust (oa4rust) 基线

| Scenario | Total | Success | Errors | Error Rate | QPS | P50 (ms) | P95 (ms) | P99 (ms) |
|----------|-------|---------|--------|------------|-----|-----------|-----------|-----------|
| login | 1000 | 1000 | 0 | 0.00% | TBD | TBD | TBD | TBD |
| work-start | 1000 | 1000 | 0 | 0.00% | TBD | TBD | TBD | TBD |
| cms | 1000 | 1000 | 0 | 0.00% | TBD | TBD | TBD | TBD |

### 6.2 Java (o2server) 基线

| Scenario | Total | Success | Errors | Error Rate | QPS | P50 (ms) | P95 (ms) | P99 (ms) |
|----------|-------|---------|--------|------------|-----|-----------|-----------|-----------|
| login | 1000 | 1000 | 0 | 0.00% | TBD | TBD | TBD | TBD |
| work-start | 1000 | 1000 | 0 | 0.00% | TBD | TBD | TBD | TBD |
| cms | 1000 | 1000 | 0 | 0.00% | TBD | TBD | TBD | TBD |

### 6.3 对比结果

| Scenario | Rust QPS | Java QPS | Rust P95 | Java P95 | Rust P99 | Java P99 | Winner (QPS) |
|----------|----------|----------|----------|----------|----------|----------|--------------|
| login | TBD | TBD | TBD | TBD | TBD | TBD | TBD |
| work-start | TBD | TBD | TBD | TBD | TBD | TBD | TBD |
| cms | TBD | TBD | TBD | TBD | TBD | TBD | TBD |

## 7. 容量结论与后续动作

- [ ] 完成实际压测并填充上述表格
- [ ] 确定服务端单实例 QPS 上限（如 2000 QPS @ P95 < 10ms）
- [ ] 确定数据库连接池合理大小
- [ ] 根据 QPS 目标计算所需实例数
- [ ] 将压测步骤加入 CI（可选）

## 8. 故障排查

- **服务无法启动**：确认 PostgreSQL 已运行且 `.env` 配置正确
- **登录失败**：确认 `auth_person` 表中存在测试账户（默认 `testadmin` / `testadmin`）
- **work-start 报 400**：预热阶段 work 创建不足或 work 状态非 pending
- **Java 不可达**：确认 o2server 已部署且端口正确（默认 8080 或 20020）
