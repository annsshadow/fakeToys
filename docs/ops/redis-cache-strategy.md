# OA4Rust Redis 缓存策略研究

**日期：** 2026-08-08
**状态：** 研究文档

---

## 1. 缓存目标分析

### 当前瓶颈

| 模块 | 问题 | 频率 |
|------|------|------|
| `is_admin` | 每次请求 1-3 次 DB 查询 | 每请求 |
| `person_has_role` | 每次请求 1 次 DB 查询 | 每请求 |
| `person_has_group` | 每次请求 1 次 DB 查询 | 每请求 |
| `check_permission` | 聚合查询（role + group + admin） | 每请求 |

### 缓存收益预估

- 当前：每请求 3-5 次 DB 查询（auth + rbac）
- 目标：每请求 0-1 次 DB 查询（缓存命中后）
- 预期 QPS 提升：2-5x（取决于缓存命中率）

---

## 2. 缓存策略设计

### 2.1 缓存键设计

```
admin:{person_unique}           → bool (TTL: 60s)
role:{person_unique}:{role}     → bool (TTL: 60s)
group:{person_unique}:{group_id}→ bool (TTL: 60s)
```

### 2.2 TTL 策略

| 缓存项 | TTL | 理由 |
|--------|-----|------|
| admin 角色 | 60s | 角色变更不频繁，60s 可接受 |
| 角色 membership | 60s | 同上 |
| 组成员 | 60s | 同上 |
| Session | 持久化 | 与 SessionManager 一致 |

### 2.3 缓存失效策略

**主动失效（推荐）：**
```rust
// 当用户角色/组成员变更时，清除相关缓存
pub async fn invalidate_user_cache(pool: &Pool, person_unique: &str) {
    let redis = get_redis_client().await;
    let patterns = vec![
        format!("admin:{}", person_unique),
        format!("role:{}:*", person_unique),
        format!("group:{}:*", person_unique),
    ];
    for pattern in patterns {
        if let Ok(keys) = redis.keys(&pattern).await {
            for key in keys {
                redis.del(key).await.ok();
            }
        }
    }
}
```

**被动失效：**
- TTL 到期自动失效
- DB 宕机时使用内存降级（当前 AdminCache 已部分实现）

### 2.4 降级策略

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Redis     │────▶│  AdminCache │────▶│     DB      │
│  (L1 Cache) │     │ (L2 Cache)  │     │ (Source of  │
│  TTL: 60s   │     │  per-request│     │  truth)     │
└─────────────┘     └─────────────┘     └─────────────┘
     命中              未命中               未命中
   (1ms)           (1ms + DB)          (10-50ms)
```

---

## 3. 实现方案

### 3.1 依赖添加

```toml
# Cargo.toml (shared crate)
redis = { version = "0.25", features = ["tokio-comp"] }
```

### 3.2 缓存层结构

```
shared/src/
├── cache/
│   ├── mod.rs           # 公共接口
│   ├── redis_client.rs  # Redis 客户端封装
│   └── admin_cache.rs   # 管理员缓存（结合现有 AdminCache）
├── middleware/
│   ├── auth_middleware.rs
│   ├── rbac_middleware.rs
│   └── rate_limit.rs
└── middleware.rs        # 统一导出
```

### 3.3 核心接口

```rust
pub trait Cache: Send + Sync + 'static {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: &str, ttl: Duration);
    async fn del(&self, key: &str);
    async fn invalidate_pattern(&self, pattern: &str);
}

pub struct RedisCache {
    client: redis::Client,
}

pub struct MemoryCache {
    store: Arc<RwLock<HashMap<String, CachedValue>>>,
}

#[derive(Clone)]
pub struct CachedValue {
    pub data: String,
    pub expires_at: Instant,
}
```

### 3.4 集成到中间件

```rust
// 修改 is_admin 函数
pub(crate) async fn is_admin(pool: &Pool, person_unique: &str, cache: &Arc<dyn Cache>) -> bool {
    // 1. 检查请求级缓存
    if let Some(cached) = AdminCache::get(person_unique) {
        return cached;
    }
    
    // 2. 检查 Redis 缓存
    let cache_key = format!("admin:{}", person_unique);
    if let Some(cached) = cache.get(&cache_key).await {
        let result = cached == "1";
        AdminCache::set(person_unique, result);
        return result;
    }
    
    // 3. 查询 DB
    let result = query_admin_from_db(pool, person_unique).await;
    
    // 4. 写入 Redis（60s TTL）
    if result {
        cache.set(&cache_key, "1", Duration::from_secs(60)).await;
    }
    
    // 5. 写入请求级缓存
    AdminCache::set(person_unique, result);
    
    result
}
```

---

## 4. 部署建议

### 4.1 环境配置

```bash
# .env
REDIS_URL=redis://localhost:6379
CACHE_TTL_SECONDS=60
CACHE_MAX_MEMORY=256mb
```

### 4.2 启动检查

```rust
pub async fn init_cache_pool(redis_url: &str) -> Result<Arc<dyn Cache>, AppError> {
    if redis_url.is_empty() {
        // Redis 未配置，使用内存缓存
        Ok(Arc::new(MemoryCache::new()))
    } else {
        let client = redis::Client::open(redis_url)?;
        // 健康检查
        client.get_async_connection().await
            .map_err(|e| AppError::Database(e.into()))?;
        Ok(Arc::new(RedisCache::new(client)))
    }
}
```

### 4.3 回滚策略

- Redis 不可用时自动降级到内存缓存
- 内存缓存不可用时降级到纯 DB 查询（当前行为）
- 不丢失任何功能，仅影响性能

---

## 5. 性能预期

| 场景 | 当前延迟 | 预期延迟 | 提升 |
|------|----------|----------|------|
| 缓存命中 | 10-50ms (DB) | 1-2ms (Redis) | 5-25x |
| 缓存未命中 | 10-50ms | 11-52ms | - |
| 无 Redis | 10-50ms | 10-50ms | - |

---

## 6. 实施优先级

1. **Phase 1:** 实现 `MemoryCache` + 请求级缓存（已部分实现）
2. **Phase 2:** 实现 `RedisCache` 接口
3. **Phase 3:** 集成到 `is_admin` / `person_has_role` / `person_has_group`
4. **Phase 4:** 添加缓存失效机制
5. **Phase 5:** 生产环境配置和监控

---

## 7. 风险评估

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| Redis 宕机 | 性能降级 | 自动降级到内存缓存 |
| 缓存穿透 | 数据不一致 | Bloom filter 或 null 值缓存 |
| 缓存雪崩 | DB 压力 | 随机 TTL 偏移 |
| 数据一致性 | 权限错误 | TTL 60s + 主动失效 |
