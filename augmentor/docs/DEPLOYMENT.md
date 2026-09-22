# 部署指南

提供三种部署方式：本地部署、Docker 部署、服务器部署。

- [本地部署](#1-本地部署)
- [Docker 部署](#2-docker-部署)
- [服务器部署](#3-服务器部署)
- [环境变量清单](#4-环境变量清单)
- [运维与排查](#5-运维与排查)

---

## 1. 本地部署

### 1.1 最小可用

```bash
cd augmentor

python -m venv .venv
source .venv/bin/activate        # Windows: .venv\Scripts\activate

pip install requests pyyaml numpy "pydantic>=2" fastapi uvicorn python-multipart
```

### 1.2 完整功能

```bash
pip install -r requirements.txt
```

`requirements.txt` 中的可选依赖较重（`sentence-transformers` 会拉取 PyTorch）。
若只需 Web + 增强 + 导出，按 1.1 安装即可；缺失的可选能力会自动降级。

### 1.3 启动服务

```bash
# 后端
uvicorn api.main:app --host 0.0.0.0 --port 8000

# 前端（另开一个终端）
cd web
npm install
npm run build          # 产物 web/dist，后端检测到后自动挂载
```

访问 <http://localhost:8000>。开发模式下用 `npm run dev` 启动 Vite 热更新服务。

### 1.4 后台常驻（Linux）

```bash
nohup uvicorn api.main:app --host 0.0.0.0 --port 8000 > app.log 2>&1 &
```

### 1.5 仅用 CLI

不需要 Web 时，直接使用命令行：

```bash
python cli.py augment --input train_data.json --output out.json
python cli.py quality --input train_data.json --report report.md
python cli.py export --input train_data.json --output-dir exports
```

### 1.6 运行测试

```bash
pytest
```

覆盖率低于 80% 会判定失败。生成 HTML 报告：

```bash
pytest --cov-report=html:htmlcov
```

---

## 2. Docker 部署

`docker/` 目录提供 `Dockerfile` 与 `docker-compose.yml`。

### 2.1 构建镜像

```bash
cd augmentor
docker build -f docker/Dockerfile -t ai-data-platform:2.2.0 .
```

镜像基于 `python:3.11-slim`，预装 `libgomp1`（FAISS/OpenMP）、`libsndfile1`（音频）、
`libgl1` 与 `libglib2.0-0`（图像）等系统库。

构建参数：

| 参数 | 默认 | 说明 |
|------|------|------|
| `INSTALL_OPTIONAL` | `0` | 设为 `1` 时安装全部可选依赖（镜像体积显著增大） |

```bash
docker build -f docker/Dockerfile \
  --build-arg INSTALL_OPTIONAL=1 \
  -t ai-data-platform:2.2.0-full .
```

### 2.2 启动

```bash
cd augmentor/docker
docker compose up -d
```

`docker-compose.yml` 会：

- 把宿主机的 `8000` 端口映射到容器；
- 从 `.env` 文件（若存在）或当前 shell 环境读取模型密钥；
- 挂载 `./data` 用于持久化数据、断点、版本与向量库；
- 配置健康检查（`GET /api/health`）。

### 2.3 验证

```bash
docker compose ps
docker compose logs -f api
curl http://localhost:8000/api/health
```

### 2.4 环境变量

在 `augmentor/docker/` 下创建 `.env`：

```bash
BAIDU_API_KEY=your_api_key
BAIDU_SECRET_KEY=your_secret_key
OPENAI_API_KEY=your_openai_key
ANTHROPIC_API_KEY=your_anthropic_key
GOOGLE_API_KEY=your_google_key
```

未配置的后端在构造时会报错，配置了至少一个即可正常使用。

### 2.5 停止与清理

```bash
docker compose down              # 停止并删除容器（保留卷）
docker compose down -v           # 同时删除数据卷
```

---

## 3. 服务器部署

### 3.1 系统准备（Ubuntu 22.04）

```bash
sudo apt-get update
sudo apt-get install -y python3.11 python3.11-venv python3-pip \
    libgomp1 libsndfile1 libgl1 libglib2.0-0
```

### 3.2 部署应用

```bash
sudo mkdir -p /opt/ai-data-platform
sudo chown $USER:$USER /opt/ai-data-platform
# 上传代码到 /opt/ai-data-platform

cd /opt/ai-data-platform/augmentor
python3.11 -m venv .venv
.venv/bin/pip install -r requirements.txt

cd web && npm ci && npm run build && cd ..
```

### 3.3 systemd 服务

`/etc/systemd/system/ai-data-platform.service`：

```ini
[Unit]
Description=AI Training Data Platform
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/ai-data-platform/augmentor
Environment="BAIDU_API_KEY=your_api_key"
Environment="BAIDU_SECRET_KEY=your_secret_key"
Environment="PYTHONUNBUFFERED=1"
ExecStart=/opt/ai-data-platform/augmentor/.venv/bin/uvicorn api.main:app --host 127.0.0.1 --port 8000 --workers 1
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now ai-data-platform
sudo systemctl status ai-data-platform
```

> **worker 数量建议为 1**。断点、版本与向量库均基于本地文件，多 worker 会产生写竞争。
> 需要更高吞吐时，请横向扩展为多实例并让每个实例使用独立的数据目录。

### 3.4 Nginx 反向代理

`/etc/nginx/sites-available/ai-data-platform`：

```nginx
server {
    listen 80;
    server_name your-domain.com;

    client_max_body_size 200m;

    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # 增强与导出可能耗时较长
        proxy_read_timeout 600s;
        proxy_send_timeout 600s;
    }
}
```

```bash
sudo ln -s /etc/nginx/sites-available/ai-data-platform /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx
```

### 3.5 HTTPS

```bash
sudo apt-get install -y certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

### 3.6 定时任务示例

```bash
# 每日 02:00 增强新数据并生成质量报告
0 2 * * * cd /opt/ai-data-platform/augmentor && .venv/bin/python cli.py augment \
    --input data/new_seeds.json --output data/augmented_$(date +\%F).json \
    >> logs/daily.log 2>&1
```

注意 `%` 在 crontab 中需转义为 `\%`。

---

## 4. 环境变量清单

| 变量 | 必需 | 说明 |
|------|------|------|
| `BAIDU_API_KEY` | 使用 ERNIE 时必需 | 百度智能云 API Key |
| `BAIDU_SECRET_KEY` | 使用 ERNIE 时必需 | 百度智能云 Secret Key |
| `OPENAI_API_KEY` | 使用 OpenAI 时必需 | OpenAI API Key |
| `ANTHROPIC_API_KEY` | 使用 Claude 时必需 | Anthropic API Key |
| `GOOGLE_API_KEY` | 使用 Gemini 时必需 | Google AI API Key |

未设置的环境变量会被替换为空字符串，不会让配置加载失败；但对应后端在**构造阶段**会因密钥为空而报错。

`config.yaml` 中通过 `${VAR}` 引用：

```yaml
models:
  ernie:
    api_key: ${BAIDU_API_KEY}
    secret_key: ${BAIDU_SECRET_KEY}
```

---

## 5. 运维与排查

### 5.1 健康检查

```bash
curl -f http://localhost:8000/api/health
```

Kubernetes 探针：

```yaml
livenessProbe:
  httpGet:
    path: /api/health
    port: 8000
  initialDelaySeconds: 10
  periodSeconds: 30
```

### 5.2 日志

- 应用日志默认写入 `app.log`（由 `config.yaml` 的 `logging.file` 控制）
- 每个 HTTP 响应携带 `X-Process-Time`，超过 2.0 秒的请求会写 WARNING
- Docker：`docker compose logs -f api`
- systemd：`journalctl -u ai-data-platform -f`

### 5.3 常见问题

| 现象 | 原因 | 处理 |
|------|------|------|
| 启动报 `ImportError: ChromaDB 后端需要安装 chromadb` | 未装 chromadb | `pip install chromadb`，或把 `vector.backend` 改为 `faiss` |
| 启动报模型密钥为空 | 环境变量未生效 | 检查 `systemctl show ai-data-platform -p Environment` |
| 端口被占用 | 8000 已被占用 | `--port 8080`，或 `lsof -i :8000` 找到占用进程 |
| 容器启动后立即退出 | 端口冲突 / 依赖安装失败 | `docker compose logs api` |
| 增强任务无进展 | API 限流或网络不通 | 查看 `app.log`；降低 `augmentation.num_threads` |
| 前端白屏 | `web/dist` 不存在 | 执行 `npm run build`，或改用 `npm run dev` |
| 图表未生成 | matplotlib 等缺失 | 属预期降级；安装对应依赖 |
| 重复运行没有新数据 | 断点续传跳过了已完成条目 | 删除断点（见下） |
| 磁盘持续增长 | 版本快照累积 | 清理 `versioning.storage_dir` 下的旧版本 |
| 内存占用高 | `sentence-transformers` 模型常驻 | 属预期（约 400MB）；或改用降级模式 |

### 5.4 删除断点以重跑

```bash
python -c "
from augmentor import AugmentorPipeline
p = AugmentorPipeline()
task_id = AugmentorPipeline.build_task_id('train_data.json')
p.checkpoint_manager.delete_checkpoint(task_id)
print('deleted', task_id)
"
```

或通过 API：`GET /api/augment/checkpoints` 查看现有任务 ID。

### 5.5 备份

需要备份的目录（默认路径）：

| 目录 | 内容 |
|------|------|
| `data/versions/` | 版本快照与 `history.jsonl` |
| `checkpoints/` | 断点文件（可安全删除） |
| `experiments/` | 实验追踪记录 |
| `data/vectors/` | 向量库持久化数据 |
| `data/benchmark_baseline.json` | 质量基准 |

```bash
tar czf backup-$(date +%F).tar.gz \
    data/versions checkpoints experiments data/vectors \
    data/benchmark_baseline.json 2>/dev/null
```

### 5.6 资源建议

| 场景 | CPU | 内存 | 磁盘 |
|------|-----|------|------|
| 仅 CLI + 核心依赖 | 2 核 | 2 GB | 10 GB |
| CLI + 语义模型 | 4 核 | 6 GB | 30 GB |
| Web 服务 + 语义模型 | 4 核 | 8 GB | 50 GB |
| 全量依赖（含多模态） | 8 核 | 16 GB | 100 GB |

磁盘占用主要来自版本快照与向量库。2.8 万条数据的快照约 10–20 MB，
建议对 `data/versions/` 设置定期清理策略。

### 5.7 安全建议

- 原始种子数据含业务敏感信息，勿对外暴露；生产环境务必启用 HTTPS
- API 未内置鉴权，请通过 Nginx Basic Auth、API 网关或内网访问控制来限制
- `POST /api/data/upload` 会把文件写入服务端当前工作目录，仅对可信调用方开放
- 不要在镜像或仓库中硬编码密钥，统一走环境变量或密钥管理服务
