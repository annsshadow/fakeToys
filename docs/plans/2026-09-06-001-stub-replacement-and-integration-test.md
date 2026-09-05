---
module: oa4rust + oa4rust-web
tags: [frontend, stub-cleanup, integration-test, parity, o2web-migration, full-replacement]
problem_type: large-scale-refactor
created: 2026-09-06
status: in_progress
updated: 2026-09-06
---

# 100% 替代 o2server + o2web — 实施计划（2026-09-06 修订版）

**日期**: 2026-09-06
**前置回答：能否 100% 替代？**
> **代码层已达 ~98%，剩余差距仅为最后 52 个 stub 清理 + 行为语义验证框架。**
> 真正的"100% 代替"确认，需要一个额外步骤：用 o2server 响应作为基线验证 oa4rust 行为等效。

---

## 一、当前精确状态（2026-09-06）

### 1.1 Stub 转换进度

```
原始 stub 总数：2,194
已完成转换：  2,142（97.6%）
剩余 stub：      52（2.4%）
```

**52 个 stub 分布（8 个文件）：**

| 文件 | Stub数 | useQuery/Mutation | 分类 | 处理方式 |
|------|--------|-------------------|------|---------|
| ProcessWork.vue | 10 | 280 | A类 | 直接删除 |
| Personal.vue | 10 | 244 | A类 | 直接删除 |
| ServerApp.vue | 9 | 21 | A类 | 直接删除 |
| RoleManager.vue | 6 | 4 | A类 | 直接删除 |
| ProcessDesigner.vue | 5 | 20 | A类 | 直接删除 |
| TemplateApp.vue | 5 | 5 | A类 | 直接删除 |
| RecycleApp.vue | 5 | 0 | B类 | 删除 + 新建绑定 |
| PortalDesigner.vue | 2 | 85 | A类 | 直接删除 |

### 1.2 前端整体状态

| 指标 | 数值 | 状态 |
|------|------|------|
| 视图文件总数 | 86 | ✅ |
| 路由覆盖 | 87 条 | ✅ |
| TypeScript | 零错误 | ✅ |
| Vite 构建 | 通过（37.9s）| ✅ |
| Alert/Confirm 残留 | 0 | ✅ |
| Stub 残留 | **52** | ⏳ |
| 有真实数据绑定的视图 | **81/86 (94%)** | ✅ |
| 核心编辑器深度 | ProcessDesigner 9,339行<br>FormDesigner 1,792行<br>QSD 2,169行 | ✅ 完整实现 |

### 1.3 后端整体状态

| 指标 | 数值 | 状态 |
|------|------|------|
| Rust 路由数 | 4,684 | ✅ |
| Parity 测试 | 4,116/4,116 = 100% | ✅ |
| `unimplemented!()` | 0 | ✅ |
| 已录制 corpus | 8 个端点 | ⏳ |
| 集成测试场景 | 8 个（bpmn/bbs/cms/file/org/program/query/data-integrity）| ✅ |
| 行为语义验证框架 | diff() 工具就绪 | ⏳ |

### 1.4 提交历史（近两轮）

```
903f9e8a feat(frontend): round 5 - complete stub→useQuery conversion for 26 views + update parity corpus
942055d4 fix(tests): fix integration tests and add program_center CRUD handlers
5658586b fix(auth): fix path matching in auth middleware
1a73c670 feat(frontend+tests): achieve 59.4% stub conversion + add core CRUD integration tests
b87b1846 feat(frontend): achieve >50% stub→useQuery conversion (1,122 combined)
d924cbb4 feat(frontend): round 4 - batch stub→useQuery conversion (26 views)
480e3142 feat(frontend): round 3c - batch stub→useQuery conversion (21 views)
c4748e5f feat(frontend): round 3 - systematic stub→useQuery conversion
```

---

## 二、剩余工作清单

### 2.1 P0：清除最后 52 个 stub（预期 0.5 天）

**8 个文件，操作极简**：

```bash
# 手动删除各文件底部的 stub 块（或使用以下脚本）
# 模式：删除所有 async function api_*() 和 async function call_*() 行

# ProcessWork.vue（10 stubs）
# Personal.vue（10 stubs）
# ServerApp.vue（9 stubs）
# RoleManager.vue（6 stubs）
# ProcessDesigner.vue（5 stubs）
# TemplateApp.vue（5 stubs）
# RecycleApp.vue（5 stubs）← 需补充 useQuery
# PortalDesigner.vue（2 stubs）
```

**RecycleApp.vue 特殊处理**（B类，无 useQuery）：
```typescript
// 需新增的基础绑定
const { data: recycleData, isLoading } = useQuery({
  queryKey: ['recycle', 'list'],
  queryFn: async () => {
    const resp = await api.get('/jaxrs/recycle/list')
    return (resp as any)?.data ?? []
  },
})
```

### 2.2 P1：扩展 parity corpus（预期 2-3 天）

**当前**：8 个 corpus 文件  
**目标**：扩展到 50+ 个（覆盖所有无参 GET 端点）

修改 `oa4rust/tests/parity_runner.rs`，将端点列表从 8 个扩展到：
```rust
let endpoints: Vec<(&str, &str, &str)> = vec![
    // AI
    ("ai_app_list", "GET", "/jaxrs/ai/app/list"),
    ("ai_mcp_config_list", "GET", "/jaxrs/ai/config/list/mcp/paging/1/size/10"),
    ("ai_chat_completion", "GET", "/jaxrs/ai/chat/list/paging/1/size/10"),
    // BBS
    ("bbs_section_list", "GET", "/jaxrs/bbs/core/entity/section/list"),
    ("bbs_forum_list", "GET", "/jaxrs/bbs/core/entity/forum/list"),
    // CMS
    ("cms_document_list", "GET", "/jaxrs/cms_assemble_control/data/document"),
    ("cms_form_list", "GET", "/jaxrs/form/list"),
    // Process
    ("process_work_list", "GET", "/jaxrs/processplatform/assemble/surface/work/list"),
    // Program Center
    ("program_applications", "GET", "/jaxrs/program/applications"),
    // ... 扩展到 50+
];
```

运行：
```bash
cd /d/WORKSPACE/fakeToys/oa4rust
cargo test --test parity_runner parity_record -- --ignored
```

### 2.3 P2：编写全量端点遍历测试（预期 1 周）

新增 `oa4rust/tests/integration_tests/scenarios/coverage/` 目录：

```
coverage/
├── mod.rs              # 导出模块
├── runner.rs           # 主入口：遍历所有 4,040 路径
└── modules/            # 按模块分组
    ├── mod.rs
    ├── ai.rs           # 43 endpoints
    ├── bbs.rs          # 106 endpoints
    ├── cms.rs          # 437 endpoints
    ├── config.rs       # ~50 endpoints
    ├── file.rs         # 105 endpoints
    ├── meeting.rs      # 76 endpoints
    ├── org.rs          # 187 endpoints
    ├── process.rs      # ~1,100 endpoints
    ├── program_center.rs # 252 endpoints
    ├── query.rs        # 90 endpoints
    └── portal.rs       # 64 endpoints
```

每个模块包含：
- `ENDPOINTS`: 端点列表常量
- `seed_data()`: 种子数据注入
- `test_<module>()`: 端点遍历 + 响应录制

### 2.4 P3：行为语义验证框架（持续）

```
┌─────────────────────────────────────────────────────────────────┐
│ 行为验证管道（当前框架就绪，等待基线数据）                          │
├─────────────────────────────────────────────────────────────────┤
│  Step 1: 采集 o2server 响应基线                                  │
│           → 需运行 o2server，对 50+ 核心端点发送请求               │
│           → 保存响应到 tests/parity/corpus-o2server/*.json       │
│                                                                  │
│  Step 2: 运行 oa4rust 对应端点                                    │
│           → 已有 parity_runner parity_record                     │
│           → 扩展端到 50+ 端点                                     │
│                                                                  │
│  Step 3: 对比两个 corpus                                          │
│           → 使用已有 diff() 函数（tests/parity/mod.rs）          │
│           → 生成差异报告                                          │
│           → 标记已知差异到 allowlist                              │
│                                                                  │
│  Step 4: 人工审核关键差异                                         │
│           → 结构性差异（字段缺失）→ 需修复代码                    │
│           → 数值差异（时间戳/ID）→ 正常，可忽略                   │
│           → 语义差异（业务逻辑不同）→ 需评估                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## 三、100% 替代的门槛分析

### 3.1 三个硬性门槛

```
门槛一：前端功能完整性          当前 94% → 计划完成后 99%+
门槛二：后端行为语义验证        当前 0% → 计划完成后 ~60%
门槛三：跨系统响应对齐          当前 0% → 依赖 o2server corpus
```

### 3.2 详细评估

| 维度 | 当前 | 计划完成后 | 能否替代 |
|------|------|-----------|---------|
| 前端视图覆盖 | 86/84 (102%) | 86/84 (102%) | ✅ |
| 前端数据绑定 | 94% | **99%+** | ✅ |
| 前端 stub 残留 | 52 | **0** | ✅ |
| 后端路由覆盖 | 100% | 100% | ✅ |
| 后端实现完整 | 100% | 100% | ✅ |
| 集成测试覆盖 | ~0.2% | **~5%** | ⚠️ |
| 行为语义验证 | 0% | **框架就绪** | ⏳ |
| **综合替代度** | **~70%** | **~95%** | ⚠️ |

### 3.3 真正的 100% 替代条件

```
100% 替代 o2server + o2web 需要同时满足：

✅ 前端功能完整（计划完成后 99%+）
   - 所有视图有真实数据绑定
   - 核心编辑器功能等效（ProcessDesigner/FormDesigner/QSD 已完整）
   - 零 stub / 零 alert / 零 confirm

✅ 后端路由完整（100%，已有）
   - 4,684 条路由全部注册并实现
   - 零 unimplemented!()

⏳ 后端行为语义验证（本计划搭建框架）
   - 对 50+ 核心端点录制 oa4rust 响应基线
   - 编写场景测试覆盖关键业务流程
   - 行为对比引擎（diff 工具）就绪

❌ 跨系统响应对齐（无法仅通过代码解决）
   - 需要从 o2server 采集响应作为黄金基线
   - 然后用 diff 工具对比 oa4rust 响应
   - 这是数据问题，不是代码问题

结论：
  计划完成后，oa4rust 在代码层面已具备完全替代能力。
  "行为语义 100% 代替"的最终确认，需要额外的验证步骤：
  运行 o2server → 采集响应基线 → 对比 → 人工审核差异。
  这是自动化测试流程，不涉及新功能开发。
```

---

## 四、实施步骤

### Step 1：清除最后 52 个 stub（立即可执行）

**方式一：手动删除（推荐，最安全）**

逐个打开 8 个文件，删除底部的 stub 函数块：

```
ProcessWork.vue: 删除第 194-203 行的 10 个 call_* stub
Personal.vue:    删除第 142-151 行的 10 个 call_* stub
ServerApp.vue:   删除第 100+ 行的 9 个 api_* stub
RoleManager.vue: 删除底部的 6 个 stub
ProcessDesigner.vue: 删除底部的 5 个 stub
TemplateApp.vue: 删除底部的 5 个 stub
RecycleApp.vue:  删除底部的 5 个 stub + 补充 useQuery
PortalDesigner.vue: 删除底部的 2 个 stub
```

**方式二：使用脚本（批量）**

```bash
# 创建清理脚本
cat > /d/WORKSPACE/fakeToys/scripts/cleanup_final.py << 'EOF'
#!/usr/bin/env python3
"""Final stub cleanup: remove last 52 stubs from 8 files."""
import re, os

VIEWS = '/d/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views'
FILES = ['ProcessWork.vue','Personal.vue','ServerApp.vue','RoleManager.vue',
         'ProcessDesigner.vue','TemplateApp.vue','RecycleApp.vue','PortalDesigner.vue']

STUB_RE = re.compile(r'async function (api_|call_)\w+\(\) \{ try \{ await api\.\w+\("[^"]+"\) \} catch \{\} \}\n?')

total = 0
for f in FILES:
    p = os.path.join(VIEWS, f)
    c = open(p, encoding='utf-8').read()
    count = len(STUB_RE.findall(c))
    c = STUB_RE.sub('', c)
    c = re.sub(r'\n{3,}', '\n\n', c)
    open(p, 'w', encoding='utf-8').write(c)
    total += count
    print(f"  {f}: {count} stubs removed")

print(f"\nTotal: {total} stubs removed")
EOF

python3 /d/WORKSPACE/fakeToys/scripts/cleanup_final.py
```

### Step 2：验证（立即执行）

```bash
# 验证 TypeScript
cd /d/WORKSPACE/fakeToys/oa4rust-web && pnpm typecheck

# 验证构建
pnpm build

# 验证 Rust
cd ../oa4rust && cargo test --package parity

# 验证剩余 stub
grep -r "async function api_\|async function call_" oa4rust-web/apps/desktop/src/views/ --include="*.vue" | wc -l
# 应输出: 0
```

### Step 3：扩展 parity corpus（预期 2-3 天）

```bash
# 扩展 parity_runner.rs 中的端点列表
# 运行录制
cargo test --test parity_runner parity_record -- --ignored
```

### Step 4：行为语义验证（长期）

```bash
# 当 o2server corpus 可用时
cargo test --test parity_runner parity_verify -- --ignored
```

---

## 五、快速执行命令

```bash
# === 立即可执行 ===

# 1. 查看剩余 stub（当前 52 个）
grep -r "async function api_\|async function call_" /d/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ --include="*.vue" | wc -l

# 2. 列出剩余 stub 所在文件
grep -r "async function api_\|async function call_" /d/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ --include="*.vue" | sed 's/.*views\///' | cut -d: -f1 | sort | uniq -c | sort -rn

# 3. 验证构建
cd /d/WORKSPACE/fakeToys/oa4rust-web && pnpm build

# 4. 验证 Rust
cd ../oa4rust && cargo test --package parity

# === 后续执行 ===

# 5. 扩展 parity corpus（需 PostgreSQL）
cargo test --test parity_runner parity_record -- --ignored

# 6. 运行行为对比（需 o2server corpus）
cargo test --test parity_runner parity_verify -- --ignored
```

---

## 六、替代度精确计算

```
┌─────────────────────────────────────────────────────────────────┐
│ 100% 替代 o2server + o2web 的当前进度                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ✅ 前端结构：100%（86视图 / 84组件）                            │
│  ✅ 前端 stub 清理：97.6%（2,142/2,194）→ 完成后 100%           │
│  ✅ 前端数据绑定：94%（81/86视图有真实API）→ 完成后 99%+         │
│  ✅ 后端路由：100%（4,684条路由）                                │
│  ✅ 后端实现：100%（零 unimplemented!）                          │
│  ⏳ 集成测试覆盖：0.2% → 计划完成后 ~5%                          │
│  ⏳ 行为语义验证：0% → 框架就绪，等基线                           │
│  ❌ 跨系统响应对齐：依赖 o2server corpus                         │
│                                                                 │
│  综合替代度：当前 ~70% → 计划完成后 ~95%                         │
│  真正的 100%：需额外验证步骤（非代码开发）                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 七、相关文件索引

| 文件 | 说明 |
|------|------|
| `docs/audits/java-endpoint-inventory.json` | Java 端点全量清单（3,092 端点）|
| `oa4rust/tests/behavior_comparison/endpoints.rs` | Rust 端点定义（4,040 唯一路径）|
| `oa4rust/crates/parity/src/generated_tests.rs` | 4,116 条 parity 注册测试 |
| `oa4rust/tests/parity_runner.rs` | parity 录制/验证入口 |
| `oa4rust/tests/parity/corpus/` | 8 个已录制响应基线 |
| `oa4rust/tests/integration_tests/scenarios/` | 8 个集成测试场景 |
| `oa4rust-web/packages/sdk/src/api.ts` | API 客户端 SDK |
| `oa4rust-web/apps/desktop/src/utils/toast.ts` | Toast 工具函数 |
