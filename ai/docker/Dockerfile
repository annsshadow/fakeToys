# syntax=docker/dockerfile:1

# ============================================================
# Stage 1 — 构建 Web UI
# ============================================================
FROM node:20-alpine AS web-builder

WORKDIR /web

# 先只拷贝依赖清单，利用层缓存：改源码时不会重新安装依赖
# 有 lock 文件时用 npm ci 保证可复现安装
COPY web/package.json web/package-lock.json* ./
RUN if [ -f package-lock.json ]; then \
        npm ci --no-audit --no-fund; \
    else \
        npm install --no-audit --no-fund; \
    fi

COPY web/ ./
RUN npm run build


# ============================================================
# Stage 2 — Python 运行时
# ============================================================
FROM python:3.11-slim AS runtime

# 设为 1 时额外安装全部可选依赖（语义模型 / 向量库 / 多模态 / LLM 框架）
ARG INSTALL_OPTIONAL=0

ENV PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1 \
    PIP_NO_CACHE_DIR=1 \
    PIP_DISABLE_PIP_VERSION_CHECK=1

# libgomp1      FAISS / OpenMP 运行时
# libsndfile1   音频解码（soundfile）
# libgl1        图像处理（Pillow / OpenCV 依赖）
# libglib2.0-0  图像处理运行时
# curl          容器健康检查
RUN apt-get update && apt-get install -y --no-install-recommends \
        libgomp1 \
        libsndfile1 \
        libgl1 \
        libglib2.0-0 \
        curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 先装依赖再拷代码，依赖不变时可复用缓存
COPY requirements-core.txt requirements.txt ./
RUN pip install --no-cache-dir -r requirements-core.txt \
    && if [ "$INSTALL_OPTIONAL" = "1" ]; then \
           pip install --no-cache-dir -r requirements.txt; \
       fi

COPY . .
COPY --from=web-builder /web/dist ./web/dist

# 以非 root 用户运行，并预建运行时目录
RUN useradd --create-home --uid 10001 appuser \
    && mkdir -p /app/data /app/checkpoints /app/experiments /app/visualizations \
    && chown -R appuser:appuser /app

USER appuser

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD curl -fsS http://localhost:8000/api/health || exit 1

CMD ["uvicorn", "api.main:app", "--host", "0.0.0.0", "--port", "8000"]
