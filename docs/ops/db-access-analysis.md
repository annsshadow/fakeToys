# Database Access Pattern Analysis — OA4Rust Migration

## 1. 分析范围

- 当前已真实化 6 个 crate（auth、control、personal、personal_extend、message、program_init）
- 双轨运行期间，Rust 和 Java 同时访问同一 PostgreSQL 实例

## 2. 事务隔离级别

当前使用 PostgreSQL 默认隔离级别：**READ COMMITTED**

### 风险评估
- 大多数 O2OA 操作为短事务（单条 INSERT/UPDATE/DELETE），READ COMMITTED 足够
- 流程引擎（process）的复杂工作流可能涉及多表事务，需关注幻读和不可重复读
- 考勤模块的批量操作（打卡记录写入 + 统计更新）可能产生写偏斜

### 建议
- 保持 READ COMMITTED，不在数据库级别修改隔离级别
- 对于复杂工作流，在应用层实现乐观锁（version 字段）
- 对于批量操作，使用 `SELECT ... FOR UPDATE` 锁定相关行

## 3. 并发写入风险分析

### 高风险表

| 表名 | 风险描述 | 影响模块 | 缓解措施 |
|------|----------|----------|----------|
| auth_session | 会话并发创建/删除 | auth | 使用 `INSERT ON CONFLICT` 幂等 |
| auth_person | 人员信息并发修改 | control | 乐观锁（version 字段） |
| attendance_record | 打卡记录批量写入 | attendance | 批量 INSERT + 错误忽略 |
| process_work | 流程工作项状态流转 | process | 乐观锁 + 状态机约束 |
| message | 消息状态并发更新 | message | 原子 UPDATE ... WHERE status = 'unread' |

### 中风险表

| 表名 | 风险描述 | 影响模块 | 缓解措施 |
|------|----------|----------|----------|
| file_entry | 文件元数据并发修改 | file | 乐观锁 |
| calendar_event | 日程并发编辑 | calendar | 乐观锁 |
| bbs_article | 文章发布/撤回 | bbs | 乐观锁 |

## 4. 双轨运行期间的数据校验策略

### 阶段 1：Java 为主，Rust 只读（当前状态）
- Rust 侧仅实现查询端点
- Java 侧保持全部写入
- 定期对比 Rust 和 Java 的查询结果

### 阶段 2：Rust 部分模块写入
- 先切分无依赖的边缘模块（hotpic、bbs）
- 对正在迁移的表实施数据校验：
  ```sql
  -- 校验示例：对比 Java 和 Rust 写入的行数
  SELECT
    'java' as source, COUNT(*) as count FROM auth_person WHERE updated_at > $1
  UNION ALL
  SELECT
    'rust' as source, COUNT(*) as count FROM auth_person_rust WHERE updated_at > $1;
  ```

### 阶段 3：Rust 为主，Java 只读
- 大部分模块已切到 Rust
- 禁用 Java 写入（应用级别，非数据库级别）
- 保留 Java 只读能力作为热备

## 5. 迁移期间的数据一致性保障

### 实时校验
- 对关键表（auth_person、auth_role）实施双写校验
- Rust 写入后立即读取验证
- 不一致时告警并自动回滚到 Java

### 定期全量校验
- 每天凌晨执行全量数据对比
- 对比策略：
  1. 行数对比
  2. 关键字段哈希对比
  3. 软删除记录对比

### 禁用 Java 写入的条件
- 连续 3 天全量校验无差异
- 核心业务流程（登录、人员 CRUD、消息）稳定运行 7 天
- 回滚计划已通过演练验证

## 6. 建议的数据库连接配置

```rust
// 连接池配置建议
deadpool_postgres::Config {
    pool: {
        max_size: 20,           // 最大连接数
        idle_timeout: Some(300), // 空闲连接超时（秒）
        max_lifetime: Some(1800), // 连接最大生命周期（秒）
    },
}
```

- 双轨运行期间适当增加连接池大小
- 设置 `idle_timeout` 避免连接泄漏
- 监控连接池使用率，峰值不超过 80%
