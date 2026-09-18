"""FastAPI 后端入口"""

import logging
import sys
from pathlib import Path

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles

sys.path.insert(0, str(Path(__file__).parent.parent))

from augmentor.config import load_config
from api.middleware import RequestLoggingMiddleware
from api.routes import augment, audit, config, data, export, leakage, multimodal, privacy, quality, version

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(name)s - %(message)s"
)

app = FastAPI(title="AI 训练数据增强平台", version="2.0.0")

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


# ============ 健康检查 ============

@app.get("/api/health")
async def health_check():
    """健康检查"""
    return {"status": "ok", "version": "2.0.0"}


# ============ 静态资源（构建产物存在时才挂载） ============

_static_dir = Path(__file__).parent.parent / "web" / "dist"
if _static_dir.is_dir():
    app.mount("/", StaticFiles(directory=str(_static_dir), html=True), name="static")


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
