---
module: oa4rust-test-coverage
tags: [testing, coverage, plan, roadmap]
problem_type: planning
---

# OA4Rust 95%+ 测试覆盖率达成计划

## 当前状态

| 指标 | 当前值 | 目标值 | 差距 |
|------|--------|--------|------|
| 总体语句覆盖 | 42.97% | 95%+ | -52% |
| API包覆盖 | 16.26% | 95%+ | -79% |
| 测试数量 | 870 | 2000+ | 63% |

## 障碍分析

### 1. API包覆盖不足（关键瓶颈）

**问题**：
- API文件 `/packages/apis/src/index.ts` 738个语句，仅120个被覆盖
- 638个API函数，仅34个被覆盖
- 自动生成的REST API映射代码困难覆盖

**原因**：
- 当前测试仅验证函数存在性
- 未实际调用API函数执行代码路径
- 需要后端响应才能验证完整路径

### 2. E2E测试受限

**问题**：
- 依赖真实后端server
- 当前环境无可用后端
- 无法收集main.ts等入口文件覆盖

## 达成95%目标的路线图

### 阶段1：启动后端环境（前置条件）

**动作**：
```bash
# 需要先启动o2server或oa4rust后端
# 查看文档：https://oa4rust.org/docs/deployment

# 本地开发启动示例：
cd o2server
cargo run --release
# 或启动o2server Java版本
```

**验证**：
- `curl http://localhost:8080/api/authentication/who` 返回用户信息

### 阶段2：扩大API单元测试覆盖（可立即实施）

**动作**：
为每个API函数编写实际调用测试：

```typescript
// 示例：覆盖processApi.processService
it('processService.taskList calls POST with correct params', async () => {
  const { processServiceApi } = await import('./index.ts')
  await processServiceApi.taskList(1, 10)
  expect(mockPost).toHaveBeenCalledWith(
    '/api/processplatform/service/processing/task/list/paging/1/size/10',
    {}
  )
})
```

**工作量**：
- 估计需要：约50小时
- 目标：为638个API函数编写测试
- 产出：从16%提升至80%+

### 阶段3：E2E覆盖测试（后端可用后）

**动作**：
```bash
# 设置环境变量
export BASE_URL=http://localhost:8080
export COLLECT_COVERAGE=true

# 运行E2E覆盖测试
npx playwright test e2e/app-coverage.spec.ts
```

**预期覆盖**：
- main.ts：从0%提升至80%+
- registerSW.ts：从0%提升至60%+
- 路由系统：90%+

### 阶段4：分支测试完善

**动作**：
为关键分支添加测试：

```typescript
// API错误处理分支
it('auth.login with invalid credentials handles error', async () => {
  mockPost.mockRejectedValueOnce(new Error('Invalid'))
  const { authApi } = await import('./index.ts')
  await expect(authApi.login({ credential: 'bad', password: 'wrong' }))
    .rejects.toThrow()
})

// 参数边界测试
it('processApi.workStart validates input', async () => {
  const { processApi } = await import('./index.ts')
  // 测试空参数、边界值等
})
```

## 立即可行的改进（不依赖后端）

### 1. 优化mock策略

修改`packages/apis/src/apis-integration.test.ts`，让mock更真实：

```typescript
// 当前mock
const mockGet = vi.fn().mockResolvedValue({ success: true, data: {} })

// 优化后
const mockGet = vi.fn().mockImplementation(async (url) => {
  // 根据URL返回不同响应
  if (url.includes('/list')) return { success: true, data: [] }
  if (url.includes('/detail')) return { success: true, data: { id: '1' } }
  return { success: true, data: null }
})
```

### 2. 添加更多API函数调用

在`apis-integration.test.ts`中继续添加函数调用：

```typescript
// 当前：约100个函数测试
// 目标：覆盖70%+的API函数

describe('API comprehensive coverage', () => {
  // 按照模块分组添加测试
  // 每个模块至少5-10个函数测试
})
```

### 3. 集成测试框架

创建`scripts/test-api-coverage.js`：
- 自动扫描index.ts中的所有API函数
- 生成对应的test模板
- 可快速扩展测试覆盖

## 里程碑

| 日期 | 里程碑 | 期望覆盖 |
|------|--------|----------|
| 2026-09-18 | 启动后端环境 | 后端可用 |
| 2026-09-20 | API单元测试50%覆盖 | API 80%+ |
| 2026-09-25 | E2E测试上线 | 60%+ |
| 2026-10-01 | 完整分支测试 | 85%+ |
| 2026-10-15 | 达成95%目标 | 95%+ |

## 资源需求

1. **后端开发环境**：o2server或oa4rust Rust后端
2. **测试工程师**：约1人，2周
3. **CI/CD支持**：GitHub Actions或GitLab CI配置

## 总结

要达成95%覆盖率目标，**必须先启动后端环境**。当前阶段：

✅ 已完成：测试框架搭建，870个测试通过，SDK/Contract覆盖良好
🕒 进行中：API单元测试扩充（从16%提升至80%+）
⏳ 待完成：E2E覆盖测试，需要后端支持

## 文档引用

- [OA4Rust 测试覆盖率总结](test-coverage-summary.md)
- [OA4Rust API 测试生成器](../../scripts/generate-api-tests.ts)
- [E2E 测试指南](../e2e/app-coverage.spec.ts)
