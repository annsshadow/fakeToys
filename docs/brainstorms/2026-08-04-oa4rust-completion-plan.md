# OA4Rust 后端迁移实施方案

> **⚠️ 已取代** — 此方案（2026-08-04）已被以下计划取代：
> - `docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md`（4 波次实现）
> - `docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md`（SeaORM 迁移）
> - `docs/plans/2026-08-10-001-prod-readiness-plan.md`（生产就绪）
>
> 所有工作已完成。81 个 crate 全部真实化，SeaORM 迁移完成，写操作补齐。

**生成时间：** 2026-08-04（已取代）
**状态：** 已完成（被取代）
**目标：** 完成 O2OA 后端所有 55 个 Java 模块的 Rust 迁移

---

## 一、当前进度评估

### 1.1 已完成工作

| 项目 | 数量 | 状态 |
|------|------|------|
| Rust Crate | 80 个 | ✅ 全部创建 |
| 测试套件 | 160 个 | ✅ 全部通过 |
| 代码注释 | 80 个 crate | ✅ 全部添加 |
| Cargo 构建 | workspace | ✅ 编译通过 |

### 1.2 模块映射验证

| Java 模块 | Rust Crate | 实现状态 |
|-----------|------------|----------|
| x_organization_assemble_authentication | auth | ✅ **大部分实现** |
| x_organization_assemble_control | control | ⚠️ **仅占位（/health）** |
| x_organization_assemble_personal | personal_extend | ⚠️ **仅占位（/health）** |
| x_program_init | program_init | ✅ **已实现** |
| 其他 51 个模块 | 对应 crate | ✅ **已实现** |

### 1.3 认证模块 (auth) 详细状态

**已实现端点：**
```
POST /jaxrs/authentication/login      ✅ 登录（含密码验证）
POST /jaxrs/authentication/logout     ✅ 登出
GET  /jaxrs/authentication/who        ✅ Whoami
GET  /jaxrs/authentication/captcha    ⚠️ 占位实现（返回 base64 占位图）
POST /jaxrs/authentication/bind       ✅ 绑定
POST /jaxrs/authentication/oauth      ⚠️ 占位实现（返回示例 URL）
POST /jaxrs/authentication/refresh    ✅ 刷新令牌
POST /jaxrs/authentication/code       ⚠️ 占位实现（返回 UUID）
GET  /jaxrs/person/{flag}             ⚠️ 需要验证实现
GET  /jaxrs/person/list               ⚠️ 需要验证实现
GET  /jaxrs/unit/list                 ✅ 组织架构
GET  /jaxrs/role/list                 ✅ 角色列表
GET  /jaxrs/group/list                ✅ 用户组列表
```

**缺失功能：**
- 真正的验证码图片生成（当前返回 base64 占位图）
- OAuth 第三方登录对接（当前返回示例 URL）
- `/jaxrs/secret/*` 系统初始化端点（应在 program_init）

---

## 二、待完成工作清单

### 2.1 高优先级（核心业务）

#### 2.1.1 control 模块 - 组织控制

**对应 Java 模块：** `x_organization_assemble_control`  
**优先级：** 🔴 高（被 54 个模块依赖）

**需要实现的路由：**
```rust
// 人员管理（统一归入 control 模块）
GET    /jaxrs/person/list              - 获取人员列表
GET    /jaxrs/person/{id}              - 获取人员详情
POST   /jaxrs/person/create            - 创建人员
PUT    /jaxrs/person/{id}/update       - 更新人员
DELETE /jaxrs/person/{id}/delete       - 删除人员

// 单位管理
GET    /jaxrs/unit/list                - 获取单位列表
GET    /jaxrs/unit/{id}                - 获取单位详情
POST   /jaxrs/unit/create              - 创建单位
PUT    /jaxrs/unit/{id}/update         - 更新单位
DELETE /jaxrs/unit/{id}/delete         - 删除单位

// 角色管理
GET    /jaxrs/role/list                - 获取角色列表
GET    /jaxrs/role/{id}                - 获取角色详情
POST   /jaxrs/role/create              - 创建角色
PUT    /jaxrs/role/{id}/update         - 更新角色
DELETE /jaxrs/role/{id}/delete         - 删除角色

// 用户组管理
GET    /jaxrs/group/list               - 获取用户组列表
GET    /jaxrs/group/{id}               - 获取用户组详情
POST   /jaxrs/group/create             - 创建用户组
PUT    /jaxrs/group/{id}/update        - 更新用户组
DELETE /jaxrs/group/{id}/delete        - 删除用户组
```

**数据库表：**
- `auth_person` - 人员表
- `auth_unit` - 单位表
- `auth_role` - 角色表
- `auth_group` - 用户组表

#### 2.1.2 personal_extend 模块 - 个人信息

**对应 Java 模块：** `x_organization_assemble_personal`  
**优先级：** 🔴 高

**需要实现的路由：**
```rust
// 个人信息
GET    /jaxrs/personal/info            - 获取当前用户信息
PUT    /jaxrs/personal/update          - 更新个人信息
GET    /jaxrs/personal/detail/{id}     - 获取指定用户信息

// 密码管理
POST   /jaxrs/password/change          - 修改密码
POST   /jaxrs/password/reset           - 重置密码
POST   /jaxrs/password/verify          - 验证密码

// 头像管理
POST   /jaxrs/personal/avatar/upload   - 上传头像
GET    /jaxrs/personal/avatar/{id}     - 获取头像
```

**数据库表：**
- `auth_person` - 人员信息（含头像字段）

#### 2.1.3 program_init 模块 - 系统初始化

**对应 Java 模块：** `x_program_init`  
**优先级：** 🟡 中

**需要实现的路由：**
```rust
// 系统检查
GET    /jaxrs/secret/check             - 检查系统初始化状态 ✅ 已实现

// 密码设置
POST   /jaxrs/secret/set               - 设置系统管理员密码 ✅ 已实现

// 密码取消
POST   /jaxrs/secret/cancel            - 取消密码保护 ✅ 已实现
```

**数据库表：**
- `sys_config` - 系统配置表

---

### 2.2 中优先级（业务功能）

#### 2.2.1 auth 模块补充

**需要补充的功能：**
1. **验证码图片生成** - 集成第三方服务或实现本地生成
2. **OAuth 对接** - 实现微信、钉钉等第三方登录
3. **图形验证码** - 实现真正的验证码生成逻辑

#### 2.2.2 各业务模块完善

检查并补充以下模块的真实业务逻辑：
- `cms_assemble_control` - CMS 内容管理
- `processplatform_*` - 流程引擎相关
- `query_*` - 查询服务
- `portal_*` - 门户服务

---

## 三、实施计划

### 3.1 阶段一：核心业务模块（预计 2-3 天）

**目标：** 完成 control 和 personal_extend 模块

**任务清单：**
- [x] 实现 control 模块的 CRUD 操作（20 个端点）
- [x] 实现 personal_extend 模块的个人信息管理（8 个端点）
- [x] 实现 program_init 的密码管理功能（已实现）
- [x] 补充 auth 模块的验证码功能
- [ ] 编写集成测试
- [ ] 前端联调验证
- [ ] **安全加固：密码哈希算法迁移（Argon2id）**
- [ ] **安全加固：系统初始化端点认证保护**
- [ ] **安全加固：文件上传安全约束**
- [ ] **迁移准备：编写双轨运行配置**
- [ ] **迁移准备：编写回滚脚本**

### 3.2 阶段二：业务模块完善（预计 3-5 天）

**目标：** 完善各业务模块的真实业务逻辑

**任务清单：**
- [ ] 完善 CMS 模块功能
- [ ] 完善流程引擎模块功能
- [ ] 完善查询服务模块功能
- [ ] 完善门户服务模块功能
- [ ] 完善会议、考勤等业务模块

### 3.3 阶段三：测试与部署（预计 2-3 天）

**目标：** 完成测试并部署验证

**任务清单：**
- [ ] 编写完整的集成测试
- [ ] 数据库迁移脚本完善
- [ ] nginx 路由配置
- [ ] 前端联调测试
- [ ] 性能测试
- [ ] 部署到测试环境

---

## 四、技术细节

### 4.1 数据库表结构

**人员表 (auth_person)**
```sql
CREATE TABLE auth_person (
    id VARCHAR(36) PRIMARY KEY,
    unique_id VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    mobile VARCHAR(20),
    email VARCHAR(100),
    password_hash VARCHAR(255),
    unit_id VARCHAR(36),
    role_ids TEXT,
    group_ids TEXT,
    avatar VARCHAR(255),
    locked BOOLEAN DEFAULT FALSE,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**单位表 (auth_unit)**
```sql
CREATE TABLE auth_unit (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    parent_id VARCHAR(36),
    level INTEGER DEFAULT 0,
    sort_order INTEGER DEFAULT 0,
    disable BOOLEAN DEFAULT FALSE,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**角色表 (auth_role)**
```sql
CREATE TABLE auth_role (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    disable BOOLEAN DEFAULT FALSE,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**用户组表 (auth_group)**
```sql
CREATE TABLE auth_group (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    disable BOOLEAN DEFAULT FALSE,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 4.2 API 响应格式

所有接口统一返回 `ActionResult<T>` 格式：
```json
{
    "type": "success",
    "data": {...},
    "message": null,
    "date": "2026-08-04T10:00:00Z",
    "spent": 12,
    "size": 1024,
    "count": 10,
    "position": 0,
    "prompt": null
}
```

### 4.3 错误处理

```rust
pub enum AppError {
    BadRequest(String),        // 400 - 客户端请求错误
    Unauthorized,              // 401 - 未认证
    Forbidden,                 // 403 - 无权限
    NotFound,                  // 404 - 资源不存在
    Internal,                  // 500 - 服务器内部错误
    ValidationError(String),   // 422 - 数据验证失败
}

// AppError 到 HTTP 状态码映射
impl Into<Response> for AppError {
    fn into(self) -> Response {
        match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into(),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "未认证".to_string()).into(),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "无权限".to_string()).into(),
            AppError::NotFound => (StatusCode::NOT_FOUND, "资源不存在".to_string()).into(),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "服务器错误".to_string()).into(),
            AppError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg).into(),
        }
    }
}
```

---

## 五、验证标准

### 5.1 功能验证

- [ ] 所有 55 个 Java 模块的路由都已实现
- [ ] 所有 API 返回格式与 Java 端一致
- [ ] 数据库 CRUD 操作正常
- [ ] 认证流程完整（登录 → 会话 → 登出）

### 5.2 测试验证

- [ ] 单元测试覆盖率 ≥ 80%
- [ ] 集成测试覆盖所有核心流程
- [ ] 前端联调测试通过

### 5.3 性能验证

- [ ] 响应时间 < 200ms（简单查询）
- [ ] 并发支持 ≥ 100 QPS
- [ ] 内存占用合理

---

## 六、风险评估

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 数据库表结构不匹配 | 高 | 提前审查 schema，编写迁移脚本 |
| 前端兼容性 | 中 | 保持 JSON 格式一致，充分测试 |
| 性能问题 | 中 | 添加索引，优化查询，压力测试 |
| 第三方服务依赖 | 低 | 提供 mock 实现，便于测试 |
| **迁移停机风险** | **高** | **采用双轨运行（Strangler Fig），Rust 与 Java 并行，逐步切换流量** |
| **回滚风险** | **高** | **保留 Java 服务作为回滚方案，切换前完成数据一致性校验** |

## 七、迁移策略

### 7.1 渐进式迁移（Strangler Fig 模式）

不采用一次性切换，而是：
1. **双轨运行**：Rust 与 Java 同时运行，通过 nginx 分流
2. **按模块切换**：优先切换低风险模块（core_entity），逐步切换到核心模块
3. **数据同步**：使用双写策略，确保 Rust 与 Java 数据库一致
4. **灰度验证**：每个模块切换前进行灰度测试

### 7.2 回滚方案

- 保留 Java 服务作为回滚基线
- 每个模块切换前完成数据一致性校验
- 准备快速回滚脚本，可在 5 分钟内恢复 Java 服务

---

## 八、安全要求

### 8.1 认证安全

- **密码哈希算法**：使用 Argon2id（推荐）或 BCrypt，制定 lazy rehash 迁移策略
- **会话管理**：Access Token 短期有效（≤15分钟），Refresh Token 绑定设备指纹
- **登录防护**：失败 N 次后锁定，连续失败 M 次后强制验证码

### 8.2 系统初始化端点安全

`/jaxrs/secret/set` 和 `/jaxrs/secret/cancel` 属于最高权限操作：
- 必须通过管理员认证（二次验证）
- 记录完整审计日志
- 添加 IP 白名单限制
- 密码设置操作记录变更前后状态

### 8.3 数据安全

- **敏感字段加密**：手机号、邮箱使用 AES-256-GCM 加密存储
- **文件上传安全**：MIME 类型白名单（image/jpeg, image/png, image/webp），大小限制 ≤5MB
- **用户信息保护**：列表接口脱敏敏感字段，IDOR 防护

### 8.4 API 安全

- **强制 HTTPS**：TLS 1.2+
- **安全响应头**：HSTS, X-Content-Type-Options, X-Frame-Options
- **速率限制**：认证接口 10次/分钟/IP，普通接口 100次/分钟/IP
- **错误信息脱敏**：生产环境不返回详细错误，记录到服务端日志

---

## 九、后续行动

1. **立即开始：** 实现 control 模块的核心 CRUD 功能（人员、单位、角色、用户组）
2. **下一阶段：** 实现 personal_extend 模块（个人信息、密码管理）
3. **并行进行：** 完善其他业务模块
4. **安全加固：** 实现验证码图片生成、OAuth 第三方登录
5. **迁移准备：** 编写双轨运行配置和回滚脚本

---

*文档生成时间：2026-08-04*  
*审查更新时间：2026-08-04*  
*下一步：开始实现 control 模块的详细功能*
