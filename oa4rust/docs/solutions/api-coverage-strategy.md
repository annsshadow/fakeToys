---
module: oa4rust-test-coverage
tags: [testing, coverage, decision]
problem_type: documentation
---

# API 包覆盖率策略

## 决策

采用 **方案 A：接受声明式层的覆盖上限**。

`packages/apis/src/index.ts` 作为业务 API 映射层，其覆盖门禁按可测试的声明式配置处理，不再强行要求业务覆盖率 95%。

## 原因

- 该层是自动生成的 HTTP 路由映射
- 在单元测试中主要依赖 `@oa4rust/sdk` mock，V8 难以统计真实调用覆盖
- 实际质量保证由契约守卫测试承担

## 落地

- vitest 对该文件保留现实阈值
- 该文件的可执行分支辅助函数继续保留
- 该文件的正确性由 endpoint contract tests 保证
