---
title: "feat: AI 训练数据增强工具全面升级"
type: feat
status: active
date: 2026-09-16
origin: docs/brainstorms/ai-data-augmentation-tool-requirements.md
---

# AI 训练数据增强工具全面升级

## Summary

基于现有 `enhanceTXT.py` 脚本，构建完整的 AI 训练数据增强工具。核心改造包括：统一模型调用框架、质量评分系统、智能去重、多格式导出、Web UI 管理界面、数据版本管理等 12 个功能模块。采用插件化架构，支持本地和服务器两种部署模式。

## Problem Frame

当前工具仅为单一脚本（`enhanceTXT.py`），存在以下问题：
- 仅支持百度 ERNIE 一种模型
- 无数据质量评估机制
- 无去重功能
- 导出格式单一
- 无 Web 管理界面
- 无数据版本管理

改造后将成为一个功能完整的数据增强平台，支持多种 LLM 后端、自动质量评估、可视化管理和多格式导出。

## Requirements

- R1. 支持多种 LLM 后端（ERNIE、OpenAI、Ollama/vLLM 本地模型）
- R2. 自动数据质量评分（语义相似度、回答相关性、多样性）
- R3. 智能去重（向量相似度检测近义重复）
- R4. 多格式导出（JSONL、Llama-Factory、Alpaca、ShareGPT、ChatML）
- R5. 对话上下文增强（多轮对话数据生成）
- R6. Web UI 管理界面（数据管理、增强监控、配置管理、数据可视化）
- R7. 断点续传优化（自动保存、进度可视化）
- R8. 数据版本管理（快照、对比、回滚、历史记录）
- R9. 主动学习选样（识别覆盖不足的问答类型）
- R10. 自动领域扩展（基于 seed 自动生成新话题）
- R11. 微调效果追踪（对接微调流程、效果对比）
- R12. 数据分布可视化（词云、长度分布、话题聚类）

## Scope Boundaries

### In Scope
- 12 个功能模块的完整实现
- CLI 和 Web UI 两种使用方式
- 本地和服务器两种部署模式

### Out of Scope
- 多用户权限系统（个人使用）
- 分布式集群部署
- 自定义微调训练流程

### Deferred to Follow-Up Work
- 移动端适配
- 国际化支持

---

## Context & Research

### Relevant Code and Patterns

- `ai/enhanceTXT.py` — 现有增强脚本，40 线程并发、断点续传
- `ai/train_data.json` — 原始种子数据（~200+ Q&A）
- `ai/train_data_final02.json` — 增强后数据（~28K+ 条）
- 数据格式：`{"input": "", "instruction": "用户提问", "output": "客服回答"}`

### Institutional Learnings

- 百度 ERNIE API 调用需要 `BAIDU_API_KEY` 和 `BAIDU_SECRET_KEY` 环境变量
- 40 线程并发可处理大规模数据
- 断点续传通过 `train_data_cached.json` 实现

### External References

- 百度 ERNIE API 文档
- OpenAI API 文档
- Ollama API 文档
- Llama-Factory 数据格式规范
- Alpaca 数据格式规范
- ShareGPT 数据格式规范

---

## Key Technical Decisions

- **插件化模型架构**：使用策略模式，每种模型后端实现统一接口
- **SQLite 存储**：用于数据版本管理和元数据存储（替代纯 JSON 文件）
- **FastAPI + React**：Web UI 技术栈，FastAPI 提供 API，React + Ant Design 提供前端
- **sentence-transformers**：用于语义相似度计算
- **Docker 容器化**：支持本地和服务器两种部署模式

## Open Questions

### Resolved During Planning

- 无

### Deferred to Implementation

- 具体的向量相似度阈值需要根据实际数据调整
- Web UI 的具体交互细节在实现时确定

---

## Implementation Units

### U1. 项目架构重构 - 统一框架

**Goal:** 重构现有代码为插件化架构，支持模块化扩展

**Requirements:** R1-R12

**Dependencies:** None

**Files:**
- Create: `ai/augmentor/` (package)
- Create: `ai/augmentor/__init__.py`
- Create: `ai/augmentor/models/` (模型后端目录)
- Create: `ai/augmentor/models/base.py` (模型基类)
- Create: `ai/augmentor/models/ernie.py` (百度 ERNIE)
- Create: `ai/augmentor/models/openai_model.py` (OpenAI)
- Create: `ai/augmentor/models/ollama.py` (本地模型)
- Create: `ai/augmentor/quality.py` (质量评分)
- Create: `ai/augmentor/dedup.py` (去重)
- Create: `ai/augmentor/export.py` (格式导出)
- Create: `ai/augmentor/context.py` (对话上下文)
- Create: `ai/augmentor/versioning.py` (版本管理)
- Create: `ai/augmentor/sampler.py` (主动学习)
- Create: `ai/augmentor/expander.py` (领域扩展)
- Create: `ai/augmentor/tracker.py` (效果追踪)
- Create: `ai/augmentor/config.py` (配置管理)
- Create: `ai/config.yaml` (默认配置)
- Create: `ai/requirements.txt`

**Approach:**
- 定义统一的 `ModelBackend` 抽象基类
- 每个功能模块独立实现，通过依赖注入组合
- 配置文件驱动行为

**Technical design:**
```
ModelBackend (abstract)
├── ERNIEBackend
├── OpenAIBackend
└── OllamaBackend

AugmentorPipeline
├── ModelBackend
├── QualityScorer
├── Deduplicator
├── ContextAugmentor
└── Exporter
```

**Patterns to follow:**
- 现有 `enhanceTXT.py` 的线程池模式
- Python dataclass 用于配置

**Test scenarios:**
- Happy path: 创建 AugmentorPipeline 实例，配置 ERNIE 后端
- Happy path: 创建 AugmentorPipeline 实例，配置 OpenAI 后端
- Happy path: 创建 AugmentorPipeline 实例，配置 Ollama 后端
- Edge case: 配置文件缺失时使用默认配置
- Error path: 模型后端不可用时抛出明确错误

**Verification:**
- 所有模块可独立导入
- 配置文件可正确加载
- 模型后端可正确实例化

---

### U2. 多模型后端支持

**Goal:** 实现三种 LLM 后端的统一调用接口

**Requirements:** R1

**Dependencies:** U1

**Files:**
- Modify: `ai/augmentor/models/base.py`
- Modify: `ai/augmentor/models/ernie.py`
- Create: `ai/augmentor/models/openai_model.py`
- Create: `ai/augmentor/models/ollama.py`
- Test: `ai/tests/test_models.py`

**Approach:**
- `ModelBackend` 基类定义 `generate(prompt) -> str` 接口
- 每种后端实现具体的 API 调用逻辑
- 支持配置 temperature、top_p 等参数
- 指数退避重试机制（3 次重试，间隔 1s/2s/4s）

**Patterns to follow:**
- 现有 `MyBD.ask()` 方法的请求模式

**Test scenarios:**
- Happy path: ERNIE 后端成功调用并返回结果
- Happy path: OpenAI 后端成功调用并返回结果
- Happy path: Ollama 后端成功调用并返回结果
- Edge case: API 超时后自动重试
- Error path: API 密钥无效时抛出明确错误
- Error path: 重试 3 次后仍失败时抛出异常

**Verification:**
- 三种后端均可成功调用
- 重试机制正常工作
- 错误信息清晰明确

---

### U3. 数据质量评分系统

**Goal:** 自动评估生成数据的质量，过滤低质量样本

**Requirements:** R2

**Dependencies:** U1

**Files:**
- Create: `ai/augmentor/quality.py`
- Create: `ai/tests/test_quality.py`

**Approach:**
- 使用 `sentence-transformers` 计算语义相似度
- 评分维度：语义相似度(0.3)、回答相关性(0.4)、多样性(0.3)
- 可配置质量阈值（默认 0.6）
- 输出质量报告（评分分布、过滤统计）

**Technical design:**
```python
def calculate_quality_score(original, generated, output, existing_embeddings):
    semantic_sim = model.similarity(original, generated)
    relevance = cross_encoder.predict(generated, output)
    diversity = 1 - max_cosine_similarity(generated, existing_embeddings)
    
    score = 0.3 * semantic_sim + 0.4 * relevance + 0.3 * diversity
    return score
```

**Test scenarios:**
- Happy path: 高质量样本获得高分（>0.8）
- Happy path: 低质量样本获得低分（<0.4）
- Edge case: 空输入返回 0 分
- Edge case: 重复输入返回低多样性分

**Verification:**
- 评分结果在 0-1 范围内
- 过滤后的数据质量明显提升
- 质量报告格式正确

---

### U4. 智能去重模块

**Goal:** 检测并合并近义重复问题

**Requirements:** R3

**Dependencies:** U1, U3

**Files:**
- Create: `ai/augmentor/dedup.py`
- Create: `ai/tests/test_dedup.py`

**Approach:**
- 基于向量相似度的近义检测
- 可配置相似度阈值（默认 0.9）
- 合并策略：保留质量评分最高的版本
- 输出去重报告

**Test scenarios:**
- Happy path: 相似度 >0.9 的问题被标记为重复
- Happy path: 不相似的问题保留
- Edge case: 阈值为 1.0 时只去除完全相同的文本
- Edge case: 阈值为 0.0 时所有内容都被视为重复

**Verification:**
- 去重后的数据量合理减少
- 保留的数据质量更高
- 去重报告准确反映处理结果

---

### U5. 多格式导出

**Goal:** 支持主流微调框架的数据格式

**Requirements:** R4

**Dependencies:** U1

**Files:**
- Create: `ai/augmentor/export.py`
- Create: `ai/tests/test_export.py`

**Approach:**
- 每种格式实现一个导出函数
- 命令行参数指定输出格式
- 支持批量导出所有格式

**格式规范:**
- JSONL: `{"input": "", "instruction": "...", "output": "..."}`
- Llama-Factory: `{"instruction": "...", "input": "", "output": "...", "system": "..."}`
- Alpaca: `{"instruction": "...", "input": "", "output": "..."}`
- ShareGPT: `{"conversations": [{"from": "human", "value": "..."}, {"from": "gpt", "value": "..."}]}`
- ChatML: `{"messages": [{"role": "user", "content": "..."}, {"role": "assistant", "content": "..."}]}`

**Test scenarios:**
- Happy path: 导出为 JSONL 格式
- Happy path: 导出为 Llama-Factory 格式
- Happy path: 导出为 Alpaca 格式
- Happy path: 导出为 ShareGPT 格式
- Happy path: 导出为 ChatML 格式
- Edge case: 空数据导出为空文件

**Verification:**
- 导出的文件可被对应框架正确解析
- 格式转换无数据丢失

---

### U6. 对话上下文增强

**Goal:** 支持多轮对话数据生成

**Requirements:** R5

**Dependencies:** U1, U2

**Files:**
- Create: `ai/augmentor/context.py`
- Create: `ai/tests/test_context.py`

**Approach:**
- 从单轮 Q&A 生成多轮对话上下文
- 支持配置对话轮数（2-5 轮）
- 保持对话连贯性
- 添加 `history` 字段存储历史对话

**Technical design:**
```python
def generate_multi_turn(seed_qa, model_backend, num_turns=3):
    history = []
    current_q = seed_qa["instruction"]
    
    for i in range(num_turns):
        # 使用 LLM 生成后续问题
        follow_up = model_backend.generate(
            f"基于以下对话，生成一个相关的后续问题：\n{history}\n最后一个问题：{current_q}"
        )
        history.append({"role": "user", "content": current_q})
        history.append({"role": "assistant", "content": seed_qa["output"]})
        current_q = follow_up
    
    return {
        "instruction": current_q,
        "input": "",
        "output": seed_qa["output"],
        "history": history
    }
```

**Test scenarios:**
- Happy path: 生成 2 轮对话数据
- Happy path: 生成 5 轮对话数据
- Edge case: 轮数为 1 时返回原始单轮数据
- Edge case: 模型生成失败时回退到原始数据

**Verification:**
- 生成的多轮对话逻辑连贯
- history 字段格式正确
- 对话轮数符合配置

---

### U7. Web UI 管理界面

**Goal:** 可视化管理数据增强流程

**Requirements:** R6

**Dependencies:** U1-U6

**Files:**
- Create: `ai/web/` (前端目录)
- Create: `ai/web/package.json`
- Create: `ai/web/src/` (React 源码)
- Create: `ai/api/` (后端目录)
- Create: `ai/api/main.py` (FastAPI 入口)
- Create: `ai/api/routes/` (API 路由)
- Create: `ai/tests/test_api.py`

**Approach:**
- 后端：FastAPI 提供 REST API
- 前端：React + Ant Design + ECharts
- 功能模块：
  - 数据管理（分页、搜索、筛选、编辑、导出）
  - 增强监控（实时进度、质量分布、错误日志）
  - 配置管理（模型配置、参数调整、API 密钥）
  - 数据可视化（词云、长度分布、话题聚类）

**Test scenarios:**
- Happy path: 访问数据列表页面
- Happy path: 执行数据增强任务
- Happy path: 查看增强进度
- Happy path: 导出数据为指定格式
- Edge case: 大数据量分页加载

**Verification:**
- Web UI 可正常访问
- API 接口响应正确
- 数据操作实时生效

---

### U8. 断点续传优化

**Goal:** 优化现有的断点续传机制

**Requirements:** R7

**Dependencies:** U1, U2

**Files:**
- Create: `ai/augmentor/checkpoint.py`
- Create: `ai/tests/test_checkpoint.py`

**Approach:**
- 自动保存进度（每 10 条）
- 进度文件包含：已完成索引、失败记录、质量评分
- 支持手动恢复
- 进度可视化（完成百分比、预计剩余时间）

**Test scenarios:**
- Happy path: 任务中断后可从断点恢复
- Happy path: 进度文件正确保存和加载
- Edge case: 进程文件损坏时从头开始
- Edge case: 并发写入时文件锁定

**Verification:**
- 中断后可正确恢复
- 进度信息准确
- 无数据丢失

---

### U9. 数据版本管理

**Goal:** 管理数据集的版本历史

**Requirements:** R8

**Dependencies:** U1

**Files:**
- Create: `ai/augmentor/versioning.py`
- Create: `ai/tests/test_versioning.py`
- Create: `ai/data/versions/` (版本存储目录)

**Approach:**
- 使用 SQLite 存储版本元数据
- 每个版本保存完整数据快照
- 支持 diff 对比和回滚

**Technical design:**
```
data/versions/
├── v1.0_20260916/
│   ├── data.json
│   ├── metadata.json
│   └── report.json
├── v1.1_20260917/
│   └── ...
└── current -> v1.1_20260917
```

**Test scenarios:**
- Happy path: 创建数据集快照
- Happy path: 对比两个版本的差异
- Happy path: 回滚到指定版本
- Edge case: 版本不存在时返回明确错误

**Verification:**
- 快照数据完整
- diff 结果准确
- 回滚操作成功

---

### U10. 主动学习选样

**Goal:** 识别需要更多增强的问答类型

**Requirements:** R9

**Dependencies:** U1, U3

**Files:**
- Create: `ai/augmentor/sampler.py`
- Create: `ai/tests/test_sampler.py`

**Approach:**
- 分析现有数据的覆盖情况
- 识别覆盖不足的话题/类型
- 优先增强这些 seed
- 输出选样报告

**分析维度:**
- 话题分布（基于 TF-IDF 或 LDA）
- 问题类型分布（疑问词、句式）
- 长度分布
- 答案复杂度

**Test scenarios:**
- Happy path: 识别出覆盖不足的话题
- Happy path: 输出推荐增强的 seed 列表
- Edge case: 数据量过少时返回所有 seed

**Verification:**
- 推荐的 seed 确实覆盖不足
- 选样报告格式正确

---

### U11. 自动领域扩展

**Goal:** 基于 seed 自动生成新话题分类

**Requirements:** R10

**Dependencies:** U1, U2, U10

**Files:**
- Create: `ai/augmentor/expander.py`
- Create: `ai/tests/test_expander.py`

**Approach:**
- 分析 seed 的话题结构
- 使用 LLM 生成相关话题建议
- 人工确认后批量生成

**扩展策略:**
- 同类扩展（相似话题）
- 关联扩展（相关话题）
- 场景扩展（使用场景）

**Test scenarios:**
- Happy path: 生成 5 个相关话题建议
- Happy path: 根据确认的话题生成 seed
- Edge case: 原始话题过于具体时生成更通用的话题

**Verification:**
- 生成的话题与原始话题相关
- 生成的 seed 质量可接受

---

### U12. 微调效果追踪

**Goal:** 验证增强数据对下游任务的提升

**Requirements:** R11

**Dependencies:** U1, U9

**Files:**
- Create: `ai/augmentor/tracker.py`
- Create: `ai/tests/test_tracker.py`

**Approach:**
- 对接微调流程（读取训练日志）
- 对比实验（有/无增强数据）
- 效果指标追踪
- 生成效果报告

**追踪指标:**
```json
{
  "experiment_id": "exp_20260916",
  "dataset_version": "v1.1",
  "metrics": {
    "train_loss": [0.5, 0.3, 0.2],
    "eval_accuracy": [0.6, 0.7, 0.75],
    "perplexity": [12.5, 8.3, 6.2]
  }
}
```

**Test scenarios:**
- Happy path: 读取训练日志并解析指标
- Happy path: 对比两个实验的效果
- Happy path: 生成效果报告

**Verification:**
- 指标解析准确
- 对比结果正确
- 报告格式清晰

---

### U13. 数据分布可视化

**Goal:** 可视化数据集的统计信息

**Requirements:** R12

**Dependencies:** U1, U7

**Files:**
- Create: `ai/augmentor/visualizer.py`
- Create: `ai/tests/test_visualizer.py`

**Approach:**
- 使用 wordcloud 生成词云图
- 使用 matplotlib/plotly 生成分布图
- 使用 sklearn 降维生成聚类图
- 集成到 Web UI

**可视化类型:**
- 词云图：高频词汇展示
- 长度分布：问题/回答长度直方图
- 话题聚类：t-SNE/UMAP 降维可视化
- 时间线：数据增强进度
- 质量分布：评分分布直方图

**Test scenarios:**
- Happy path: 生成词云图
- Happy path: 生成长度分布图
- Happy path: 生成话题聚类图

**Verification:**
- 图表正确生成
- 图表清晰可读
- 集成到 Web UI 正常显示

---

## System-Wide Impact

- **交互图:** Web UI 通过 API 调用后端服务，后端调用模型后端
- **错误传播:** 模型调用失败 → 重试 → 记录失败 → 继续处理下一条
- **状态风险:** 并发写入时需要文件锁定机制
- **API 表面:** FastAPI 提供完整 REST API
- **集成覆盖:** Web UI + CLI 双入口

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| 模型 API 限流 | 指数退避重试 + 可配置并发数 |
| 大数据量内存溢出 | 流式处理 + 分页加载 |
| 模型响应质量不稳定 | 质量评分 + 自动过滤 |
| Web UI 性能问题 | 数据分页 + 缓存 |

---

## Phased Delivery

### Phase 1: 核心框架（U1-U2）
- 项目架构重构
- 多模型后端支持

### Phase 2: 数据处理（U3-U6）
- 质量评分系统
- 智能去重
- 多格式导出
- 对话上下文增强

### Phase 3: 管理界面（U7-U9）
- Web UI 管理界面
- 断点续传优化
- 数据版本管理

### Phase 4: 智能功能（U10-U13）
- 主动学习选样
- 自动领域扩展
- 微调效果追踪
- 数据分布可视化

---

## Sources & References

- **Origin document:** docs/brainstorms/ai-data-augmentation-tool-requirements.md
- Related code: ai/enhanceTXT.py
- Related data: ai/train_data.json, ai/train_data_final02.json
