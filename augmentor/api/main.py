# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""FastAPI 后端入口"""

import logging
import sys
from pathlib import Path

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles

sys.path.insert(0, str(Path(__file__).parent.parent))

from augmentor import __version__
from augmentor.config import load_config
from api.middleware import (
    RateLimitMiddleware,
    RequestLoggingMiddleware,
    RequestTraceMiddleware,
)
from api.middleware.rate_limit import RateLimiter
from api.routes import augment, audit, config, data, export, leakage, multimodal, privacy, quality, status, version

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(name)s - %(message)s"
)

# 版本号单一来源：augmentor.__version__（不要在别处硬编码）
app = FastAPI(title="AI 训练数据增强平台", version=__version__)

# CORS 配置（从配置文件读取，生产环境请显式配置 origins）
_config = load_config("config.yaml")
app.add_middleware(
    CORSMiddleware,
    allow_origins=_config.web.cors_origins,
    allow_credentials=_config.web.cors_credentials,
    allow_methods=["*"],
    allow_headers=["*"],
)

# 请求日志与耗时统计
app.add_middleware(RequestLoggingMiddleware)

# 限流：窗口内单客户端请求数超限返回 429。
# 模块级共享实例，便于测试逐用例重置（见 tests/conftest.py）。
rate_limiter = RateLimiter(
    max_requests=_config.web.rate_limit_max_requests,
    window_seconds=_config.web.rate_limit_window_seconds,
)
app.add_middleware(
    RateLimitMiddleware,
    limiter=rate_limiter,
    exempt_paths=_config.web.rate_limit_exempt_paths,
)

# 请求追踪 ID（透传或生成 X-Request-ID）
app.add_middleware(RequestTraceMiddleware)

# ============ 路由注册 ============
app.include_router(data.router)
app.include_router(augment.router)
app.include_router(quality.router)
app.include_router(export.router)
app.include_router(version.router)
app.include_router(config.router)
app.include_router(multimodal.router)
app.include_router(privacy.router)
app.include_router(leakage.router)
app.include_router(audit.router)
app.include_router(status.router)


# ============ 健康检查 ============

@app.get("/api/health")
async def health_check():
    """健康检查"""
    return {"status": "ok", "version": "2.0.0"}


# ============ 启动自检 ============
#
# 未配置写操作密钥时给出显式告警。不阻断启动：零配置即可用是既有约定
# （容器探针、本地开发、CI 冒烟测试都依赖它）。但未鉴权时任何能访问该端口的人
# 都能改配置、删数据、回滚版本，因此必须在日志里说清楚。
from api.deps import API_KEY_ENV as _API_KEY_ENV, api_key_required as _api_key_required

if not _api_key_required():
    logging.getLogger(__name__).warning(
        "未设置 %s：写操作（改配置/删数据/上传/导出/回滚版本）当前**无鉴权**，"
        "仅应在受信网络中暴露。生产部署请设置该环境变量。",
        _API_KEY_ENV,
    )


# ============ 静态资源（构建产物存在时才挂载） ============

_static_dir = Path(__file__).parent.parent / "web" / "dist"
if _static_dir.is_dir():
    app.mount("/", StaticFiles(directory=str(_static_dir), html=True), name="static")


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
