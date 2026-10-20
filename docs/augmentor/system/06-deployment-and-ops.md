# 06 · 配置与部署 / 运维

> 本篇覆盖 **环境要求、依赖分层、配置中心、裸机 / Docker / Compose 部署、
> 环境变量、安全基线、测试与运维**。逐键配置详解见
> [`docs/augmentor/README.md § 配置说明`](../README.md) 与 `config.yaml` 内注释。

---

## 1. 环境要求

| 项 | 要求 |
|----|------|
| Python | **3.10+**（核心依赖下限；Docker 钉 `python:3.11-slim`；装全量可选依赖需 3.11+） |
| Node.js | **18+**（仅构建 Web UI 时需要；Docker 用 `node:22-alpine` 构建） |
| 操作系统 | Windows / Linux / macOS |

---

## 2. 依赖分层（缺失自动降级，不崩进程）

| 文件 | 用途 |
|------|------|
| `requirements-core.txt` | **最小可运行**：requests / pyyaml / numpy / fastapi / uvicorn / pydantic v2 / python-multipart。用于 Docker 默认镜像与轻量部署 |
| `requirements.txt` | **全量**：core + 语义模型（sentence-transformers 6.x）/ 向量库（faiss-cpu / chromadb）/ 可视化（matplotlib / wordcloud / scikit-learn）/ 多模态（Pillow / soundfile）/ LLM 框架（langchain / llama-index） |
| `requirements-dev.txt` | **测试**：core + pytest / pytest-cov / httpx / pandas / openpyxl |

**关键版本约束**：`pydantic>=2`（平台代码用 `model_dump()`，必须锁 v2）；
`sentence-transformers>=6.1.0`（6.x 依赖 transformers 5.0+，才修掉 4.x 的已知漏洞）。

### 降级路径（缺某依赖时的行为）

| 缺失 | 行为 |
|------|------|
| `sentence-transformers` | 回退 n-gram 相似度 |
| `faiss-cpu` | 回退 numpy 精确检索 |
| `chromadb` | 该后端构造时抛 `ImportError`，faiss 仍可用 |
| `matplotlib`/`wordcloud`/`scikit-learn` | 对应图表跳过并返回空路径 |
| `Pillow`/`soundfile` | 回退标准库解析文件头 |
| `langchain`/`llama-index` | 返回等价的普通字典结构 |

---

## 3. 配置中心

- 配置文件 `config.yaml`，`load_config(path)` 加载为 `AppConfig`（dataclass）。
- `${VAR}` 语法引用环境变量；**未设置替换为空串**，不让配置加载失败。
- **写操作鉴权**走环境变量 `AUGMENTOR_API_KEY`（**不是**配置项）；配置路径可用
  `AUGMENTOR_CONFIG_PATH` 覆盖；数据白名单可用 `AUGMENTOR_DATA_ROOTS`（`os.pathsep` 分隔）覆盖。

逐段键清单（`models` / `augmentation` / `quality` / `dedup` / `export` / `context` /
`versioning` / `sampler` / `expander` / `tracker` / `visualization` / `multilingual` /
`rag` / `evaluation` / `vector` / `multimodal` / `benchmark` / `active_learning` /
`frameworks` / `web` / `logging`）见 README 配置说明表与 `config.yaml` 注释。

---

## 4. 部署方式

### 4.1 裸机（本地开发）

```bash
cd augmentor
python -m venv .venv && source .venv/bin/activate   # Windows: .venv\Scripts\activate
pip install -r requirements.txt          # 或仅 requirements-core.txt（最小）

# 配置至少一个模型后端
export BAIDU_API_KEY=... BAIDU_SECRET_KEY=...        # 或 OPENAI_API_KEY 等

# 后端（:8000）
uvicorn api.main:app --host 0.0.0.0 --port 8000     # 交互式文档 /docs

# 前端
cd web && npm install && npm run build               # 产物 web/dist，由后端静态托管
# 或开发模式：npm run dev（:3000，/api 代理到 :8000）
```

启动后访问 `http://localhost:8000` 即前端（`web/dist` 存在时自动挂载）。

### 4.2 Docker（多阶段）

`docker/Dockerfile` 两阶段：
1. **Stage 1（node:22-alpine）**：构建 Web UI → `web/dist`。
2. **Stage 2（python:3.11-slim）**：装 `requirements-core.txt`（`INSTALL_OPTIONAL=1` 时加全量），
   拷入 `web/dist`，建非 root `appuser`，预建 `data/checkpoints/experiments/visualizations`。

关键指令：
- `HEALTHCHECK`：`curl -fsS http://localhost:8000/api/health`。
- `CMD`：`uvicorn api.main:app --host 0.0.0.0 --port 8000`。
- `EXPOSE 8000`。

```bash
cd augmentor/docker
cp .env.example .env            # 填模型密钥
# 设 INSTALL_OPTIONAL=1 装全量可选依赖（镜像显著变大）
docker compose up -d --build
```

### 4.3 Docker Compose（`docker/docker-compose.yml`）

- 构建上下文为 `../`（augmentor/，因 Dockerfile 需同时访问 `api/` `augmentor/` `web/`）。
- 变量优先读本目录 `.env`，回落宿主环境变量。
- 端口：`${API_PORT:-8000}:8000`。
- 卷（持久化）：`../data` `../checkpoints` `../experiments` `../visualizations` → `/app/...`。
- **`replicas: 1`**：断点/版本/向量库基于本地文件，单副本避免写竞争（多 worker 下 `path_lock` 跨进程失效，见账本）。
- 日志：json-file，`max-size: 10m / max-file: 3`。

### 4.4 生产加固清单（要点）

| 项 | 建议 |
|----|------|
| **鉴权** | 设 `AUGMENTOR_API_KEY`；未设时匿名写闸只放行回环写——**服务默认监听 `0.0.0.0`，务必配反代或绑回环** |
| **CORS** | 出厂 `cors_origins: []`（同源 UI 不需要）；确有跨源前端再显式白名单 + 谨慎开 `cors_credentials` |
| **反代** | 跑在反代后用 `uvicorn --proxy-headers --forwarded-allow-ips=...`（限流取连接对端地址，不读可伪造的 `X-Forwarded-For`）；全网络层封上传用反代 `client_max_body_size` |
| **白名单** | `web.data_roots` 收紧到最小；每多一个根目录 = 多放行整棵子树 |
| **密钥不落盘** | 模型密钥在配置里用 `${ENV_VAR}` 占位；写配置时只有原文件是占位符才保留，否则移除 |

---

## 5. 安全基线（维护须知）

- **数据白名单**：客户端路径 resolve 后须落白名单根，越界 403（防符号链接逃逸）。
- **500 不回传原文**：未分类异常 → 固定文案 + 原文落日志（防部署路径/文件状态外泄）。
- **密钥明文存储告警**：Web 端 `localStorage` 存 `X-API-Key` 是**明文、单机、非保密措施**，文档已注明；强保护需后端会话/CSRF。
- **匿名写缺口**：未设 key 时 `WriteTransportGate` 补一层「只放行回环写」，非本机匿名写 403。
- **单副本约束**：`mutate_json_file` 的 `path_lock` 只管**同进程**；多 worker 部署该锁跨不出去。

---

## 6. 测试与覆盖率

```bash
pytest                     # pytest.ini 默认参数
pytest -m "not slow"       # 只跑快速用例
pytest --cov-report=html:htmlcov   # HTML 报告
```

`pytest.ini` 配置：`--cov=augmentor --cov=api --cov=cli`、`--cov-fail-under=80`
（**覆盖率 < 80% 直接失败**）、`--cov-report=xml:coverage.xml`。

关键机器守门（与文档一致性）：

| 测试 | 守门对象 |
|------|----------|
| `tests/integration/test_api_openapi_contract.py` | 77 端点全声明 `response_model` + 全真实存在（双向） |
| `tests/unit/test_api_endpoint_consumption_l224.py` | 每个端点都有消费入口（UNCONSUMED/CLI_ENTRY/NO_ENTRY 为空集） |
| `tests/unit/test_config_three_side_l222.py` | `AppConfig.fields()` 与出厂 `config.yaml` 逐节全等 |
| `tests/integration/test_cli_verdict_wiring.py` | CLI `--gate` 声明与 handler 源码双向推导 |
| `tests/unit/test_readme_cli_index_l204.py` | README CLI 全表与 37 命令相等 |
| `web/src/services/api.wiring.test.ts` | 前端每个 `api.ts` 导出函数都真接进页面 |

---

## 7. 常见问题（运维速查）

| 症状 | 原因 / 处置 |
|------|-------------|
| 启动报 `ImportError: ChromaDB 后端需要安装 chromadb` | `pip install chromadb`，或 `config.yaml` 把 `vector.backend` 改 `faiss` |
| 相似度低、变体大量被过滤 | 未装 `sentence-transformers` 会回退 n-gram；装它或调低 `quality.threshold` |
| 可视化没出图 | 缺 matplotlib/wordcloud/scikit-learn 时跳图表，属预期降级 |
| 重复 `augment` 没新数据 | 断点识别同输入已处理，跳过并复用输出；重跑先删断点（`GET /api/augment/checkpoints`） |
| 前端空白 | 确认 `npm run build` 且 `web/dist` 存在（后端仅在静态目录存在时挂载）；否则用 `npm run dev` |
| Windows 中文词云乱码 | 词云依赖 `C:/Windows/Fonts/simhei.ttf` |

> **注意**：百度等 LLM API 有频率限制与费用，控制 `augmentation.num_threads`；
> 原始种子数据含业务敏感信息，勿外泄；日志默认不落文件（`logging.file` 出厂空串），要落盘在 `config.yaml` 写明。
