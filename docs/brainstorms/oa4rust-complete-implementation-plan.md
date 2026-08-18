# O2OA Rust 完整复刻实施计划

## 现状盘点

| 维度 | 数量 |
|------|------|
| Rust 已实现业务路由 | 275 |
| Java 去重后接口路径（53 个核心模块） | 2,019 |
| 缺口 | 1,744 |
| 已有 crate 但路由极少的模块 | 14 |
| 完全缺失路由的 crate | 0（所有模块都有基础路由） |

## 三阶段实施计划

### 阶段一：Stub 路由全量补齐（第 1 周）
**目标**：前端调用 Rust 后端不再 404，所有 Java 接口路径在 Rust 中至少返回 200。

**执行策略**：
1. 按 `scripts/final_gap2.py` 提取的 1,744 个缺失路由，批量生成 stub handler
2. 每个 stub 返回 `ActionResult::success(null)`，HTTP 200
3. 按模块分组，每组合成为一个 commit

**优先级**：
- P0（Week 1 前半）：processplatform_*（工作流引擎，~400 路由）
- P0（Week 1 中半）：form_*（表单引擎，~300 路由）
- P1（Week 1 后半）：portal_*、attendance_*、calendar/meeting_*、file_*（~500 路由）
- P2（Week 1 收尾）：其余模块（~544 路由）

**交付物**：
- 所有缺失路由的 stub 实现
- `cargo test --workspace` 全绿
- 端到端冒烟测试通过（curl 调用每个 stub 返回 200）

---

### 阶段二：逻辑真实化（第 2-5 周）
**目标**：将阶段一的 stub 替换为真实业务逻辑，对接 PostgreSQL。

**执行策略**：
1. 按 Java Action 类 1:1 翻译为 Rust handler
2. 每个模块按 `list → get → create → update → delete` 顺序实现
3. 优先实现高频路径（登录、组织查询、流程发起、表单提交）

**周计划**：
- Week 2：processplatform 工作流引擎真实化
- Week 3：form 表单引擎 + organization 组织架构
- Week 4：portal 门户 + attendance 考勤
- Week 5：calendar/meeting + file + 其余 P1 模块

**交付物**：
- 每个模块的单元测试 + 集成测试
- 端到端联调通过（Java 前端 ↔ Rust 后端）

---

### 阶段三：生产就绪（第 6-7 周）
**目标**：达到生产部署标准。

| 项 | 内容 | 周次 |
|----|------|------|
| 结构化日志 | tracing + OpenTelemetry + trace_id | Week 6 |
| 健康检查 | `/health/live` + `/health/ready` | Week 6 |
| 配置管理 | 环境变量 + `.env` + 敏感配置加密 | Week 6 |
| 监控指标 | prometheus `/metrics` | Week 6 |
| Docker | `Dockerfile` + `docker-compose.yml` | Week 7 |
| 压测基线 | wrk/k6 核心路径 P99 延迟 | Week 7 |
| CI/CD | GitHub Actions | Week 7 |

---

## 立即执行：阶段一 Stub 生成

### 步骤 1：生成 processplatform 模块 stubs（最高优先级）

**模块清单**：
- `x_processplatform_assemble_designer` → `processplatform_assemble_designer`
- `x_processplatform_assemble_surface` → `processplatform_assemble_surface`
- `x_processplatform_assemble_bam` → `processplatform_assemble_bam`
- `x_processplatform_core_entity` → `processplatform_core_entity`
- `x_processplatform_core_express` → `processplatform_core_express`
- `x_processplatform_service_processing` → `processplatform_service_processing`
- `x_process_designer` → `process_designer`
- `x_process_express` → `process_express`
- `x_process_surface` → `process_surface`
- `x_process_bam` → `process_bam`

### 步骤 2：批量生成其余模块 stubs

按缺失路由数量排序，逐个模块生成。

### 步骤 3：注册到 main.rs

每个新 crate 在 `src/main.rs` 中：
1. 添加 `use` 语句
2. 添加 `.merge(crate::router(pool.clone()))`

### 步骤 4：编译验证

```bash
cargo check --workspace
cargo test --workspace
```

---

## 风险与缓解

| 风险 | 缓解措施 |
|------|----------|
| 1,744 个 stub 代码量大 | 脚本自动生成，无需手写 |
| 某些 Java 路径语义复杂 | Stub 阶段不关心逻辑，只返回 200 |
| 路径参数提取困难 | 先用 `Path<String>` 接收，后续细化 |
| 前端期望特定响应格式 | Stub 返回 `ActionResult::success(null)`，前端需兼容 |

---

## 下一步行动

1. **立即**：运行 `scripts/generate_stubs.py --all` 生成所有 stub
2. **今天**：手动将 stub 代码合并到各 crate 的 `lib.rs`
3. **明天**：更新 `main.rs` 注册新路由
4. **本周内**：`cargo check --workspace` 全绿 + 端到端冒烟

---

*计划生成时间：2026-08-05*
