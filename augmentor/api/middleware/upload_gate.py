# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""超限 multipart 上传的**解析前**拒收闸

`api/routes/data.py:_read_upload_body` 那两道判据量的是「文件部件」的字节，权威但**迟**：
Starlette 的 multipart 解析在进入路由之前就把文件部分逐块交给 `SpooledTemporaryFile`
（`max_size=1 MiB`，超了溢到临时目录）。也就是说一次 4 MiB 的越界上传，即使最终被
`413` 拒绝，也已经付了「解析 + 写临时文件对象」这一份代价。

本闸把那一份代价提前拒掉，判据只用**请求头**（`Content-Type` 是不是 multipart、
`Content-Length` 报了多少），因此在解析器拿到任何一个字节之前就能回话。

两种形状的实测（`Temp/l90q/spool_census2.py`，两把尺子是 `Request.form` 调用次数与
交给 `SpooledTemporaryFile.write` 的字节数）：

| 形状 | 结果 |
| --- | --- |
| FastAPI 依赖（`include_router(dependencies=...)`） | 解析 1 次 / 部件 4 194 304 字节 ⇒ **管不到**，FastAPI 先读 body 再解依赖 |
| 纯 ASGI 闸（本文件这一形状） | 解析 0 次 / 字节 0 |
| `BaseHTTPMiddleware` 闸 | 同样解析 0 次 / 字节 0，但每请求多付 **3.72 倍**（`Temp/l90q/middleware_cost.py`：3 000 请求 × 5 轮中位数，base 30.71 µs / 纯 ASGI 31.36 µs / BaseHTTP 114.26 µs），且 `dispatch` 里 `raise HTTPException` 在生产侧变成 **500**（两侧解释器各量一次：starlette 1.2.1 与 1.6.0 都是 500，见 `Temp/l90q/base_mw_500_both.py`） |

所以这里刻意不跟 `api/middleware/` 的 `BaseHTTPMiddleware` 约定：那三件的活都要读
或改请求/响应体，本闸的活只看头部，用它们的基类是为一次判断付整条内存流的代价。
「出错时 `return JSONResponse` 而不是 raise」这一条既有约定是跟的（`RateLimitMiddleware`
的 429 就是 return）。

**如实记下管不到的那一半**：客户端不声明 `Content-Length`（chunked 传输）时头部里没有
尺寸可判，本闸放行，仍由路由那两道兜底。声明值是客户端自报的，**不是**服务端权威值，
因此本闸只用来「提前拒掉明显越界的整包」，权威判据依旧是解析之后那个按实际写入字节
累加出来的 `UploadFile.size`。

**安装位置**：必须经构造参数 `FastAPI(middleware=[...])` 装，不要 `add_middleware`。
实测它在 1.2.1 里被**追加到 `app.user_middleware` 末尾**（`app.router` 上没有 middleware
属性），而 `add_middleware` 插在列表头部 ⇒ 构造参数装的那一件比所有装饰式装的层都靠内。
同一探针量到的 CORS 归属：无闸时路由自己发的 413 带 `Access-Control-Allow-Origin`；
闸经 `add_middleware` 落在 CORS **之外**时那份 413 的 CORS 头是 `None`，浏览器读不到
文案；闸在 CORS **之内**时头保留。回归由 `tests/unit/test_upload_gate_l90.py`
钉住（它判的是链上顺序，不判属性名）。
"""

from typing import Optional

from starlette.responses import JSONResponse

from api import deps

# 只有会带请求体的方法才可能灌上传体；GET/HEAD/OPTIONS（含 CORS 预检）一律直接放行
UPLOAD_METHODS = frozenset({"POST", "PUT", "PATCH"})

# multipart 的 `Content-Type` 前缀（后面还跟 `; boundary=...`，因此判前缀）
_MULTIPART_PREFIX = b"multipart/form-data"

# 整包比「文件部件」多出来的 framing 上界：boundary 行 + 每个部件的
# Content-Disposition/Content-Type 头。本仓上传路由的单部件形状实测开销 134 字节
# （`Temp/l90q/spool_census2.py`），1 MiB 是「绝不误杀合法上传」的保守余量。
FRAMING_MARGIN = 1 << 20


def _declared_multipart_bytes(scope: dict) -> Optional[int]:
    """从 ASGI scope 的头部取出「声明了总长度的 multipart 请求」的总字节数

    Args:
        scope: ASGI http scope（`headers` 是 `(bytes, bytes)` 列表）

    Returns:
        声明的整包字节数；非 multipart、没有 `Content-Length`、或值不是十进制整数时
        返回 `None` —— 调用方据此**放行**，绝不因为「看不见」就拒
    """
    content_type = None
    content_length = None
    for name, value in scope.get("headers") or ():
        low = name.lower()
        if low == b"content-type":
            content_type = value
        elif low == b"content-length":
            content_length = value
        if content_type is not None and content_length is not None:
            break

    if content_type is None or not content_type.lower().startswith(_MULTIPART_PREFIX):
        return None
    if content_length is None:
        return None
    try:
        return int(content_length)
    except ValueError:
        return None


class UploadBodyGate:
    """只看请求头的上传尺寸闸，装在校验与解析之前"""

    def __init__(self, app):
        """初始化中间件

        Args:
            app: 下游 ASGI 应用
        """
        self.app = app

    async def __call__(self, scope, receive, send):
        """声明长度越界的 multipart 请求直接 413，其余一律原样下传

        Args:
            scope: ASGI 连接范围
            receive: 接收消息的可等待函数
            send: 发送消息的可等待函数
        """
        if scope["type"] == "http" and scope["method"] in UPLOAD_METHODS:
            declared = _declared_multipart_bytes(scope)
            if declared is not None:
                limit = deps.max_upload_bytes()
                if declared > limit + FRAMING_MARGIN:
                    response = JSONResponse(
                        status_code=413,
                        content={
                            "detail": (
                                f"上传声明 {declared} 字节，超过 "
                                f"web.max_upload_bytes={limit} 加 multipart 余量 "
                                f"{FRAMING_MARGIN}（未解析、未落盘）"
                            )
                        },
                    )
                    await response(scope, receive, send)
                    return

        await self.app(scope, receive, send)


__all__ = ["FRAMING_MARGIN", "UPLOAD_METHODS", "UploadBodyGate"]
