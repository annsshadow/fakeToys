# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""FastAPI 后端入口"""

import logging
import sys
from pathlib import Path

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from starlette.middleware import Middleware

sys.path.insert(0, str(Path(__file__).parent.parent))

from augmentor import __version__
from augmentor.config import load_config
from api.deps import config_file_path
from api.middleware import (
    RateLimitMiddleware,
    RequestLoggingMiddleware,
    RequestTraceMiddleware,
    UploadBodyGate,
)
from api.middleware.rate_limit import RateLimiter
from api.routes import (
    augment,
    audit,
    config,
    data,
    dataset_tools,
    export,
    leakage,
    multimodal,
    privacy,
    quality,
    status,
    system_ops,
    version,
)
from api.routes.status import HealthResponse

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(name)s - %(message)s"
)

# ============ OpenAPI 元数据 ============
#
# 此前 FastAPI 只设了 title/version，`/docs` 与 `/openapi.json` 里既没有
# 能力说明也没有分组描述，11 个 tag 在 Swagger UI 里是无标题的裸列表。

_DESCRIPTION = """
AI 训练数据增强平台的 HTTP 接口。

## 能力分组

| tag | 覆盖范围 |
| --- | --- |
| `data` | 数据集列表 / 分页读取 / 增删改 / 上传 / 分析 / 可视化 |
| `dataset` | 统计 / 验证 / 格式转换 / 合并 / 采样 / 分割 / 搜索 / 对比 / 特征 / 聚合 / 配置推荐 / RAG |
| `augment` | 调用模型后端做数据增强、进度查询、断点列表 |
| `quality` | 质量评分、去重、质量报告、清洗、标注、基准、离群点、数据画像 |
| `export` | 导出、批量导出、预览、支持的格式 |
| `version` | 数据集版本创建 / 对比 / 历史 / 回滚 / 删除 |
| `config` | 配置读取与更新、可用模型列表 |
| `multimodal` | 多模态条目处理与目录扫描 |
| `privacy` | PII 脱敏与模式清单 |
| `leakage` | 训练集 / 测试集泄漏检测 |
| `audit` | 数据集就绪审计 |
| `status` | 服务综合状态与依赖诊断 |
| `system` | 依赖诊断 / 配置校验 / 质量监控 / 自动化测试 / 迁移 / 流式处理 / 依赖登记 / 备份 |

## 鉴权

写操作（改配置 / 删数据 / 上传 / 导出 / 回滚版本）在设置了环境变量
`AUGMENTOR_API_KEY` 时要求请求头 `X-API-Key`，不匹配返回 `401`。

**未设置该环境变量时不鉴权**——这是零配置可用的既有约定，但意味着任何能访问
该端口的人都可以改配置、删数据、回滚版本，只应部署在受信网络中。
启动日志会对此给出告警。

## 其他横切行为

- **限流**：单客户端在时间窗口内请求数超限返回 `429`（窗口与阈值见配置）。
- **追踪**：响应带 `X-Request-ID`，请求携带时透传，否则自动生成。
- **上传尺寸**：声明了 `Content-Length` 的 `multipart` 请求在**解析之前**判一次整包长度，
  超过 `web.max_upload_bytes` 加 multipart 余量直接 `413`（文案含「未解析、未落盘」）；
  未声明长度（chunked）时这一道看不见，由解析后按实际写入字节累加的 `UploadFile.size`
  兜底，仍是 `413`。
"""

_OPENAPI_TAGS = [
    {"name": "data", "description": "数据集列表、分页读取、增删改、上传、分析与可视化"},
    {"name": "dataset", "description": "数据集统计、验证、格式转换、合并、采样、分割、搜索、对比、特征检测、聚合、配置推荐与 RAG 转换"},
    {"name": "augment", "description": "调用模型后端做数据增强，含进度与断点"},
    {"name": "quality", "description": "质量评分、去重、报告、清洗、标注、基准、离群点与数据画像"},
    {"name": "export", "description": "数据集导出、批量导出与预览"},
    {"name": "version", "description": "数据集版本创建、对比、历史、回滚与删除"},
    {"name": "config", "description": "配置读取与更新、可用模型列表"},
    {"name": "multimodal", "description": "多模态条目处理与目录扫描"},
    {"name": "privacy", "description": "PII 脱敏与脱敏模式清单"},
    {"name": "leakage", "description": "训练集 / 测试集泄漏检测"},
    {"name": "audit", "description": "数据集就绪审计"},
    {"name": "status", "description": "服务综合状态与依赖诊断"},
    {"name": "system", "description": "依赖诊断、配置校验、质量监控、自动化测试、数据迁移、流式处理、数据集依赖登记与备份管理"},
]

# 版本号单一来源：augmentor.__version__（不要在别处硬编码）
#
# 构造参数 `middleware=[...]` 被**追加到 `app.user_middleware` 末尾**（实测 1.2.1：`app.router`
# 上并没有 middleware 属性），而下面的 `add_middleware` 插在列表头部 ⇒ 它比所有装饰式装的层都更靠内。
# 上传尺寸闸必须待在那一侧：实测（L90，`Temp/l90q/spool_census2.py`）把它经
# `add_middleware` 装到 CORS 之外时，它发回的 413 不带 `Access-Control-Allow-Origin`，
# 浏览器里的前端读不到那句可行动的文案；挂在最内（CORS 之内）时头保留，
# 而「在解析器拿到任何字节之前拒收」这件事不受位置影响（两种位置都是 form 0 / 字节 0）。
app = FastAPI(
    title="AI 训练数据增强平台",
    summary="数据集增强 / 质量 / 版本 / 导出的 HTTP 接口",
    description=_DESCRIPTION,
    version=__version__,
    openapi_tags=_OPENAPI_TAGS,
    contact={"name": "annsshadow"},
    license_info={"name": "AGPL-3.0-or-later"},
    middleware=[Middleware(UploadBodyGate)],
)

# CORS 配置（从配置文件读取，生产环境请显式配置 origins）
_config = load_config(str(config_file_path()))
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
app.include_router(dataset_tools.router)
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
app.include_router(system_ops.router)


# ============ 健康检查 ============

@app.get(
    "/api/health",
    response_model=HealthResponse,
    tags=["status"],
    summary="健康检查",
)
async def health_check():
    """健康检查（无需鉴权，供容器探针使用）"""
    return HealthResponse(status="ok", version=__version__)


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
#
# 目录由 `web.static_dir` 决定。相对值按**本仓根**解析，与改动前那条硬编码
# `Path(__file__).parent.parent / "web" / "dist"` 同一锚点 ⇒ 出厂默认 `web/dist`
# 逐字指向今天同一个目录；绝对值原样用。锚在仓库根而不是工作目录是刻意的：
# 这条路径跟着代码走，换 cwd 启动后端不该让随包 UI 消失。
_project_root = Path(__file__).parent.parent
_static_dir = Path(_config.web.static_dir)
if not _static_dir.is_absolute():
    _static_dir = _project_root / _static_dir
if _static_dir.is_dir():
    app.mount("/", StaticFiles(directory=str(_static_dir), html=True), name="static")
else:
    # 不挂载本身是既有行为（没构建就不该 500），但「配了目录却不存在」以前
    # 与「压根没配」在日志里一模一样都是白屏，这里补一条点名路径的告警。
    logging.getLogger(__name__).warning(
        "web.static_dir 指向的目录不存在，未挂载前端静态界面：%s", _static_dir
    )


if __name__ == "__main__":
    import uvicorn

    # 监听地址与端口取自配置。改动前是 `host="0.0.0.0", port=8000` 两个字面量，
    # 于是 `web.host: 127.0.0.1` 这种「只想绑回环」的收紧意图无声失效。
    uvicorn.run(app, host=_config.web.host, port=_config.web.port)
