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

from augmentor import __version__, logging_setup
from augmentor.config import load_config
from api.deps import config_file_path
from api.middleware import (
    RateLimitMiddleware,
    RequestLoggingMiddleware,
    RequestTraceMiddleware,
    UploadBodyGate,
    WriteTransportGate,
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
    language,
    multimodal,
    privacy,
    quality,
    status,
    system_ops,
    vector,
    version,
)
from api.routes.status import HealthResponse

logging.basicConfig(  # `handlers=` 而非 `format=`：3.12 的 basicConfig 不收 Formatter，消毒口径见 `logging_setup.ControlSafeFormatter`
    level=logging.INFO,
    handlers=[logging_setup.build_handler("%(asctime)s - %(levelname)s - %(name)s - %(message)s")],
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

**未设置该环境变量时不鉴权**——零配置可用是既有约定（容器探针、本地开发、CI 冒烟
都依赖它），但服务默认监听 `0.0.0.0`，等于把「任何人可改配置、删数据、回滚版本」
交到整个网络手里。因此 `WriteTransportGate` 在这道缺口上补了一层连接本地侧地址
（ASGI `scope["server"]`，客户端伪造不了）判断：**未设密钥时只放行来自回环地址的
`POST/PUT/PATCH/DELETE`**，非本机匿名写返回 `403`，文案点名两条出路（设
`AUGMENTOR_API_KEY`，或把服务绑回 `127.0.0.1`）。读操作与 `/api/health` 不受影响。
启动日志同步给出告警。

## 其他横切行为

- **限流**：单客户端在时间窗口内请求数超限返回 `429`（窗口与阈值见配置）。客户端
  标识取**连接对端地址**，不读 `X-Forwarded-For`（那条头自报，换值就能买到新桶）；
  跑在反代后面请用 uvicorn `--proxy-headers --forwarded-allow-ips=...`，它会把
  `scope["client"]` 改写成真实来源。限流装在 CORS **之内**：`429` 因此带得上
  `Access-Control-Allow-Origin`，且 CORS 预检不消耗配额。
- **追踪**：响应带 `X-Request-ID`，请求携带时透传，否则自动生成。
- **请求体尺寸**：声明了 `Content-Length` 的写请求（`multipart` 与 JSON / urlencoded
  一体）在**解析之前**判一次长度，超过 `web.max_upload_bytes`（multipart 另加 framing
  余量）直接 `413`（文案含「未解析、未落盘」）；未声明长度（chunked）时这一道看不见，
  由解析后按实际写入字节累加的 `UploadFile.size` 兜底，仍是 `413`。
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
#
# 限流同样装在这一侧（改前经 `add_middleware` 落在 CORS **之外**）：实测超限那支 `429`
# 不带 `Access-Control-Allow-Origin`，浏览器里的前端只看见一个空的 CORS 错误；而 CORS
# 预检在 CORS 层就被短路掉了，挪进来之后它不再消耗配额（改前一次预检白吃 1/3 预算）。
# 顺序按「便宜的判断在前」排：尺寸闸 → 匿名写传输闸 → 限流，被前两道拒收的请求
# 不占限流预算。
_config = load_config(str(config_file_path()))

# 模块级共享实例，便于测试逐用例重置（见 tests/conftest.py）。
rate_limiter = RateLimiter(
    max_requests=_config.web.rate_limit_max_requests,
    window_seconds=_config.web.rate_limit_window_seconds,
)

app = FastAPI(
    title="AI 训练数据增强平台",
    summary="数据集增强 / 质量 / 版本 / 导出的 HTTP 接口",
    description=_DESCRIPTION,
    version=__version__,
    openapi_tags=_OPENAPI_TAGS,
    contact={"name": "annsshadow"},
    license_info={"name": "AGPL-3.0-or-later"},
    middleware=[
        Middleware(UploadBodyGate),
        Middleware(WriteTransportGate),
        Middleware(RateLimitMiddleware, limiter=rate_limiter,
                   exempt_paths=_config.web.rate_limit_exempt_paths),
    ],
)

# CORS 配置（从配置文件读取，生产环境请显式配置 origins）
app.add_middleware(
    CORSMiddleware,
    allow_origins=_config.web.cors_origins,
    allow_credentials=_config.web.cors_credentials,
    allow_methods=["*"],
    allow_headers=["*"],
)

# 请求日志与耗时统计
app.add_middleware(RequestLoggingMiddleware)

# 限流：窗口内单客户端请求数超限返回 429。装法见上面 `FastAPI(middleware=[...])`
# 那段注释——必须在 CORS 之内，不是历史习惯而是实测出来的归属。

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
app.include_router(vector.router)
app.include_router(language.router)
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
        "因此 `WriteTransportGate` 只放行本机（回环）来的写请求，"
        "从非回环监听地址进来的写请求一律 403。要跨机器写请设置该环境变量。",
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


# ============ 422 回执的有限值兜底 ============
#
# FastAPI 的默认校验失败处理器把错误项直接交给 JSON 响应，而 pydantic v2 的错误项
# 带 `input` —— 即**回显解析后的请求体**。Python 的 JSON 解码默认接受 `NaN` /
# `Infinity` / `-Infinity` 三个字面量，所以这类载荷只要同时踩到别的校验错（缺必填
# 字段等），错误体里就躺着非有限浮点，而 starlette 的 JSON 响应渲染钉的是
# `allow_nan=False` ⇒ 序列化抛 `ValueError: Out of range float values are not JSON
# compliant: nan`。崩的是**错误回执自己**：实测（本轮 L284 的仓外探针，改前）
# `/api/dataset/search`、`/api/dataset/compare`、`/api/leakage/check`、
# `/api/dataset/impact`、`/api/dataset/evaluate` 五只端点上，合法载荷回 422 带可行动
# 细节，把其中一只数字字段换成 `NaN` 就变成裸文本 500 `Internal Server Error` ⇒
# 客户端丢掉 422 细节，服务端这条路径也绕开了 `raise_internal_error` 的日志收口。
# Python 客户端用默认设置序列化一个 `float('nan')` 就会写出这种载荷，不是构造出来的边角。
#
# 只兜错误路径，不动成功路径：先走默认那支，渲染抛 ValueError 才把非有限浮点换成
# 字符串 token 重渲染，状态码仍是 422、其余逐字节不变（实测合法 422 与 200 回执
# 挂上本处理器前后逐字节相同）。刻意**不**换 `default_response_class`——
# 那会把服务端自己算出来的 NaN 也在成功响应里抹成 token，等于掩盖计算缺陷；
# 也不加解析前的体二次读取（`api/middleware/upload_gate.py` 记过 `BaseHTTPMiddleware`
# 每请求 3.72 倍的代价，本节同理）。
#
# 导入刻意落在这里而不是文件头：账本与测试对本文件的行号引用最大指到 :216，
# 在文件头补这几行会把 :42-45 / :116 / :150 / :190 / :197 / :216 那几格整体平移，
# 正是 L217 / L219 记下过的同一缺陷类（插行顶出代码跨度，两把行号守卫当场红）。
import math

from fastapi.encoders import jsonable_encoder
from fastapi.exception_handlers import (
    http_exception_handler,
    request_validation_exception_handler,
)
from fastapi.exceptions import RequestValidationError
from fastapi.responses import JSONResponse

from starlette.exceptions import HTTPException as StarletteHTTPException
from starlette.requests import Request


def _finite_safe(value: object) -> object:
    """把错误回显里的非有限浮点换成字符串 token，使 `allow_nan=False` 渲染得动

    Args:
        value: `jsonable_encoder(exc.errors())` 的某个节点（容器只会是 dict / list，
            因为 `loc` 那类元组已被 `jsonable_encoder` 转成列表）

    Returns:
        `NaN` / `Infinity` / `-Infinity` 换成同名字符串，dict / list 逐元素递归，
        其余原样返回
    """
    if isinstance(value, float):
        if math.isnan(value):
            return "NaN"
        if math.isinf(value):
            return "Infinity" if value > 0 else "-Infinity"
        return value
    if isinstance(value, dict):
        return {key: _finite_safe(item) for key, item in value.items()}
    if isinstance(value, list):
        return [_finite_safe(item) for item in value]
    return value


@app.exception_handler(RequestValidationError)
async def validation_error_with_finite_echo(
    request: Request, exc: RequestValidationError
) -> JSONResponse:
    """422 回执渲染不动非有限浮点时，换成 token 再渲染一次，状态码不变

    Args:
        request: 触发校验失败的请求（默认处理器只用它，本函数不回显它的内容）
        exc: FastAPI/pydantic 的校验失败异常

    Returns:
        422 的 `JSONResponse`。载荷合法时与 FastAPI 默认那支逐字节相同；
        默认那支因 `allow_nan=False` 抛 ValueError 时，回有限值安全渲染
    """
    try:
        return await request_validation_exception_handler(request, exc)
    except ValueError:
        return JSONResponse(
            status_code=422,
            content={"detail": _encodable(
                _finite_safe(jsonable_encoder(exc.errors())))},
        )


# ============ 4xx 判决的孤立代理对兜底 ============
#
# 与上一节同一形状、不同触发字节：JSON 的 `"\ud800"` 这类**孤立代理对**能被 Python
# 解出来（`json.loads('"\\ud800"')` 默认放行），但编码不成 UTF-8，而 starlette 的
# JSON 响应是 `json.dumps(..., ensure_ascii=False).encode("utf-8")`。实测（本轮 L284
# 的仓外探针）两类落点同崩：`POST /api/dataset/convert` 的 `source_format` 喂一枚孤立
# 高代理，400 那句判决把**请求原文**写进 `detail`（「src.csv 声明为 \<原文\>，但它既不是
# 合法 JSON…」）；`POST /api/dataset/search` 的 `query` / `pattern` / `regex_pattern`
# 落到上一节那条 422 回显上。两条都在渲染阶段抛 `UnicodeEncodeError`（它是 `ValueError`
# 的子类，所以上一节的 `except` 天然接得住）⇒ 客户端拿到裸文本 500，那句中文判决整条
# 丢失。它与刚修的 `allow_nan` 那一格是同一件事的两面：错误路径的可用性由**渲染**决定，
# 而渲染不看判决对不对。
#
# 兜法与上一节一致：先走默认那支，抛了才把字符串节点里编不出的字符转成**反斜杠原文**
# （`\ud800` 六个可见字符）重渲染，状态码与响应头不变、其余字节不动。选 backslashreplace
# 而不是 U+FFFD：前者可逆地指出「是哪一枚字符」，后者把信息吃掉还和语料里合法存在的
# 替换字符同形。`success` 路径**不**接这一层，理由同上一节（不掩盖服务端算出来的东西）。
def _encodable(value: object) -> object:
    """把回显里编码不成 UTF-8 的字符换成反斜杠原文，使 JSON 响应渲染得动

    Args:
        value: 错误响应的某个节点（`detail` 可以是字符串、列表或字典）

    Returns:
        字符串节点里孤立代理对等编不出的部分转成反斜杠原文（六个可见字符），容器逐元素递归，
        其余原样返回
    """
    if isinstance(value, str):
        try:
            value.encode("utf-8")
        except UnicodeEncodeError:
            return value.encode("utf-8", "backslashreplace").decode("utf-8")
        return value
    if isinstance(value, dict):
        return {key: _encodable(item) for key, item in value.items()}
    if isinstance(value, (list, tuple)):
        return [_encodable(item) for item in value]
    return value


@app.exception_handler(StarletteHTTPException)
async def http_error_with_encodable_echo(
    request: Request, exc: StarletteHTTPException
) -> JSONResponse:
    """4xx 回执渲染不动孤立代理对时，转义成原文再渲染一次，状态码与响应头不变

    Args:
        request: 触发该 4xx 的请求
        exc: 携带 `detail`（可能嵌着请求原文）的 HTTP 异常

    Returns:
        `JSONResponse`。能正常渲染时与 FastAPI 默认那支逐字节相同；默认那支在
        `JSONResponse.__init__` 的 render 里抛 `UnicodeEncodeError` 时，回转义后的判决
    """
    try:
        return await http_exception_handler(request, exc)
    except UnicodeEncodeError:
        return JSONResponse(
            status_code=exc.status_code,
            content={"detail": _encodable(exc.detail)},
            headers=getattr(exc, "headers", None),
        )


if __name__ == "__main__":
    import uvicorn

    # 监听地址与端口取自配置。改动前是 `host="0.0.0.0", port=8000` 两个字面量，
    # 于是 `web.host: 127.0.0.1` 这种「只想绑回环」的收紧意图无声失效。
    uvicorn.run(app, host=_config.web.host, port=_config.web.port)
