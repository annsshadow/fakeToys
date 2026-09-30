# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L90：超限上传的**解析前**拒收闸（`api/middleware/upload_gate.py`）

L87 那道 `web.max_upload_bytes` 判据量的是「文件部件」的实际字节，权威但**迟**：
Starlette 的 multipart 解析在进入路由之前就把文件部分逐块交给 `SpooledTemporaryFile`。
本轮把「在解析器拿到任何字节之前拒收」这一格量清楚（`Temp/l90q/spool_census2.py`）：

| 形状 | `Request.form` 次数 / 交给部件文件的字节 |
| --- | --- |
| FastAPI 依赖（`include_router(dependencies=...)`） | 1 / 4 194 304 ⇒ **管不到**，body 先被读 |
| 纯 ASGI 闸（本文件被测形状） | 0 / 0 |
| `BaseHTTPMiddleware` 闸 | 0 / 0，但每请求 3.72 倍开销、`dispatch` 里 raise 变 500 |

这里锁四件事：
1. **拒收真的发生在解析之前**：把 `Request.form` 换成「一调用就抛」，越界请求仍回 413；
2. **文案归属**：闸那句是「（未解析、未落盘）」，路由那两道是「（未读取、未落盘）」/
   「（未落盘）」，两者不混；
3. **诚实边界**：不声明 `Content-Length`（chunked）时闸看不见尺寸，必须由路由兜底；
4. **嵌套位置**：闸必须在 CORSMiddleware 的**里面**（实测挪到外面时 413 不带
   `Access-Control-Allow-Origin`，浏览器读不到文案）。判据只走真实中间件链的顺序，
   **不认属性名也不认列表位置** —— 1.2.1 把 `FastAPI(middleware=[...])` 追加到
   `user_middleware` 末尾（`app.router` 上根本没有 middleware 属性），换版本装法可能不同，
   而这一条用例在两个解释器（starlette 1.2.1 / 1.6.0）上各自跑过。

判据常数 `FRAMING_MARGIN`（1 MiB）也单独钉：整包比文件部件多的是 boundary 行与各部件
头部（本仓单部件形状实测 134 字节），余量不足会误杀合法上传，所以边界两侧各测一格。

跑法：`python -m pytest tests/unit/test_upload_gate_l90.py -q --no-cov`
"""

import asyncio
import json

import pytest
from fastapi import FastAPI
from starlette.middleware import Middleware
from starlette.middleware.cors import CORSMiddleware
from starlette.requests import Request

from api import deps
from api.main import app
from api.middleware.upload_gate import (
    FRAMING_MARGIN,
    UPLOAD_METHODS,
    UploadBodyGate,
    _declared_multipart_bytes,
)

LIMIT = 1 << 20
BIG = 4 << 20
CHUNK = 64 << 10
ORIGIN = "http://localhost:5173"

GATE_MARK = "未解析、未落盘"
ROUTE_MARK = "未读取、未落盘"


def payload(size: int = BIG) -> bytes:
    return b"[" + b"A" * (size - 2) + b"]"


def multipart(body: bytes) -> bytes:
    """手造一次单文件部件的 multipart body（探针同款，framing 开销可复点）"""
    boundary = b"----gate"
    head = (b"--" + boundary + b"\r\nContent-Disposition: form-data; name=\"file\"; "
            b"filename=\"payload.json\"\r\nContent-Type: application/json\r\n\r\n")
    return head + body + b"\r\n--" + boundary + b"--\r\n"


def scope_for(headers, method="POST", path="/api/data/upload"):
    return {"type": "http", "asgi": {"version": "3.0"}, "http_version": "1.1",
            "method": method, "path": path, "raw_path": path.encode(),
            "query_string": b"", "headers": headers, "scheme": "http",
            "server": ("testserver", 80), "client": ("127.0.0.1", 12345),
            "root_path": "", "state": {}}


def headers_for(declared, ctype="multipart/form-data; boundary=----gate"):
    out = [(b"host", b"testserver")]
    if ctype is not None:
        out.append((b"content-type", ctype.encode()))
    if declared is not None:
        out.append((b"content-length", str(declared).encode()))
    return out


def drive(scope, body=b""):
    """用一个「被走到就记一笔」的下游驱动闸

    Returns:
        dict：状态码 / 响应体 / 是否下传 / 闸门自己读了几次请求体
    """
    seen = {"down": False, "status": None, "body": b"", "reads": 0}
    pieces = [body[i:i + CHUNK] for i in range(0, len(body), CHUNK)]
    position = {"i": 0}

    async def inner(_scope, receive, send):
        seen["down"] = True
        await send({"type": "http.response.start", "status": 200,
                    "headers": [(b"content-length", b"2")]})
        await send({"type": "http.response.body", "body": b"ok"})

    async def receive():
        seen["reads"] += 1
        if position["i"] >= len(pieces):
            return {"type": "http.disconnect"}
        piece = pieces[position["i"]]
        position["i"] += 1
        return {"type": "http.request", "body": piece,
                "more_body": position["i"] < len(pieces)}

    async def send(message):
        if message["type"] == "http.response.start":
            seen["status"] = message["status"]
        elif message["type"] == "http.response.body":
            seen["body"] += message.get("body", b"")

    async def run():
        await UploadBodyGate(inner)(scope, receive, send)

    asyncio.run(run())
    return seen


def raw_post(asgi_app, body: bytes, declare_length: bool):
    """直接驱动 ASGI：唯一能造出「不声明长度的 multipart 请求」的姿势"""
    headers = [(b"host", b"testserver"),
               (b"content-type", b"multipart/form-data; boundary=----gate")]
    if declare_length:
        headers.append((b"content-length", str(len(body)).encode()))
    else:
        headers.append((b"transfer-encoding", b"chunked"))

    state = {"position": 0, "status": None, "chunks": b""}

    async def receive():
        if state["position"] >= len(body):
            return {"type": "http.disconnect"}
        piece = body[state["position"]:state["position"] + CHUNK]
        state["position"] += len(piece)
        return {"type": "http.request", "body": piece,
                "more_body": state["position"] < len(body)}

    async def send(message):
        if message["type"] == "http.response.start":
            state["status"] = message["status"]
        elif message["type"] == "http.response.body":
            state["chunks"] += message.get("body", b"")

    async def run():
        await asgi_app(scope_for(headers), receive, send)

    asyncio.run(run())
    return state["status"], state["chunks"].decode("utf-8", "replace")


def middleware_order(asgi_app):
    """沿真实包装链从外到内列出中间件类名（不认 starlette 的内部属性名）"""
    stack = asgi_app.build_middleware_stack()
    names = []
    node = stack
    while node is not None:
        names.append(type(node).__name__)
        node = getattr(node, "app", None)
    return names


@pytest.fixture
def gate_limit(monkeypatch):
    """把闸读到的上限压到 1 MiB（闸走 `deps.max_upload_bytes`，一处 patch 即覆盖）"""
    monkeypatch.setattr(deps, "max_upload_bytes", lambda: LIMIT)


@pytest.fixture
def route_limit(monkeypatch):
    """同样压路由那两道读到的上限"""
    from api.routes import data as data_route

    monkeypatch.setattr(data_route, "max_upload_bytes", lambda: LIMIT)


class TestDeclaredSizeReading:
    """`_declared_multipart_bytes`：只有「multipart + 声明了整数长度」才看得见尺寸"""

    @pytest.mark.parametrize("length", [BIG, LIMIT, 0])
    def test_multipart_with_length(self, length):
        assert _declared_multipart_bytes(scope_for(headers_for(length))) == length

    def test_content_type_is_case_insensitive(self):
        scope = scope_for([(b"host", b"testserver"),
                           (b"content-type", b"MultiPart/Form-Data; boundary=x"),
                           (b"content-length", str(BIG).encode())])
        assert _declared_multipart_bytes(scope) == BIG

    def test_header_order_does_not_matter(self):
        scope = scope_for([(b"content-length", str(BIG).encode()),
                           (b"content-type", b"multipart/form-data")])
        assert _declared_multipart_bytes(scope) == BIG

    @pytest.mark.parametrize("ctype", [
        None,
        "application/json",
        "application/x-www-form-urlencoded",
        "multipart/x-mixed-replace",
    ])
    def test_non_multipart_is_invisible(self, ctype):
        assert _declared_multipart_bytes(scope_for(headers_for(BIG, ctype))) is None

    def test_missing_length_is_invisible(self):
        """chunked 那一半边界：头部没尺寸 ⇒ 后面必须放行，不许因为看不见就拒"""
        assert _declared_multipart_bytes(scope_for(headers_for(None))) is None

    def test_non_numeric_length_is_invisible(self):
        scope = scope_for([(b"content-type", b"multipart/form-data"),
                           (b"content-length", b"abc")])
        assert _declared_multipart_bytes(scope) is None

    def test_empty_headers_is_invisible(self):
        assert _declared_multipart_bytes({"type": "http", "headers": []}) is None


class TestGateBehaviour:
    """闸本体的五格：拒收 / 文案 / 边界 / 无体方法 / 不消费请求体"""

    def test_rejects_without_touching_the_body(self, gate_limit):
        seen = drive(scope_for(headers_for(BIG)), body=payload(1024))
        assert seen["status"] == 413
        assert GATE_MARK in seen["body"].decode()
        assert seen["down"] is False, "越界请求被下传了，那就不叫解析前拒收"
        assert seen["reads"] == 0, "闸门读了请求体，拒收就不再是 O(1)"

    def test_reject_message_names_the_limit(self, gate_limit):
        seen = drive(scope_for(headers_for(BIG)))
        detail = json.loads(seen["body"])["detail"]
        assert f"web.max_upload_bytes={LIMIT}" in detail
        assert str(BIG) in detail

    @pytest.mark.parametrize("declared", [LIMIT, LIMIT + FRAMING_MARGIN])
    def test_margin_boundary_passes(self, gate_limit, declared):
        """等于上限、以及「上限 + framing 余量」这两格都必须放行 —— 余量不足会误杀"""
        seen = drive(scope_for(headers_for(declared)))
        assert seen["down"] is True and seen["status"] == 200

    def test_one_byte_over_margin_rejected(self, gate_limit):
        seen = drive(scope_for(headers_for(LIMIT + FRAMING_MARGIN + 1)))
        assert seen["status"] == 413 and seen["down"] is False

    @pytest.mark.parametrize("method", ["GET", "HEAD", "OPTIONS", "DELETE"])
    def test_bodyless_methods_always_pass(self, gate_limit, method):
        """OPTIONS 是 CORS 预检，误伤它等于整个跨域上传失效"""
        assert method not in UPLOAD_METHODS
        seen = drive(scope_for(headers_for(BIG), method=method, path="/api/health"))
        assert seen["down"] is True and seen["status"] == 200

    def test_non_http_scope_passes(self, gate_limit):
        seen = drive({"type": "websocket", "headers": []})
        assert seen["down"] is True and seen["status"] == 200

    def test_limit_follows_config(self, monkeypatch):
        """同一份请求头，上限从 1 MiB 抬到 8 MiB 之后必须从拒收变放行"""
        monkeypatch.setattr(deps, "max_upload_bytes", lambda: LIMIT)
        assert drive(scope_for(headers_for(BIG)))["status"] == 413

        monkeypatch.setattr(deps, "max_upload_bytes", lambda: 8 << 20)
        seen = drive(scope_for(headers_for(BIG)))
        assert seen["down"] is True and seen["status"] == 200

    def test_limit_reader_failure_is_not_swallowed(self, monkeypatch):
        """上限读数自己坏了 ⇒ 让它冒出去，而不是静默放行一次看不见的越界上传

        `deps.max_upload_bytes` 本体已经把「配置损坏」这一类降级成出厂默认，所以这格
        测的是闸的取向：它不叠第二层 try —— 判据装在故障路径上时宁可出声。
        """
        monkeypatch.setattr(deps, "max_upload_bytes",
                            lambda: (_ for _ in ()).throw(RuntimeError("配置坏了")))
        with pytest.raises(RuntimeError):
            drive(scope_for(headers_for(BIG)))


class TestRealAppRejectsBeforeParsing:
    """经完整 app 的行为面：闸真的在解析器之前，且不误杀合法请求"""

    def test_declared_oversize_upload_gets_gate_message(self, gate_limit):
        from fastapi.testclient import TestClient

        resp = TestClient(app).post(
            "/api/data/upload",
            files={"file": ("payload.json", payload(), "application/json")})
        assert resp.status_code == 413
        assert GATE_MARK in resp.text, f"拿到的是路由那句：{resp.text[:140]}"

    def test_parser_never_runs_on_rejected_upload(self, gate_limit, monkeypatch):
        """把 `Request.form` 换成「一调用就抛」，越界上传仍必须回闸的 413"""
        from fastapi.testclient import TestClient

        async def boom(self, *args, **kwargs):
            raise AssertionError("multipart 解析器被调用了，拒收发生在解析之后")

        monkeypatch.setattr(Request, "form", boom)
        resp = TestClient(app).post(
            "/api/data/upload",
            files={"file": ("payload.json", payload(), "application/json")})
        assert resp.status_code == 413
        assert GATE_MARK in resp.text

    def test_legal_upload_still_reaches_the_route(self, gate_limit):
        """小上传不能被误杀：进到路由之后才会撞上「不是合法 JSON」"""
        from fastapi.testclient import TestClient

        resp = TestClient(app).post(
            "/api/data/upload",
            files={"file": ("payload.json", b"{not json", "application/json")})
        assert resp.status_code == 400
        assert "不是合法 JSON" in resp.text

    def test_chunked_upload_falls_back_to_route_guard(self, gate_limit, route_limit):
        """不声明长度：闸看不见尺寸必须放行，由路由那道权威判据回 413"""
        status, text = raw_post(app, multipart(payload(BIG)), declare_length=False)
        assert status == 413
        assert ROUTE_MARK in text, f"chunked 那一格应当由路由回话：{text[:140]}"

    def test_declared_small_upload_still_parses(self, gate_limit, route_limit):
        """反证：闸放行的小 body 正常进解析器（否则前面那些 0/0 只是表坏了）"""
        status, text = raw_post(app, multipart(payload(1024)), declare_length=True)
        assert status == 400
        assert "不是合法 JSON" in text


class TestGateScopeIsMultipartOnly:
    """闸只管「声明了长度的 multipart」，别的 POST body 一律不归它"""

    def _app(self):
        sink = FastAPI(middleware=[Middleware(UploadBodyGate)])

        @sink.post("/echo")
        async def echo():
            return {"ok": True}

        return sink

    def test_large_json_post_is_left_alone(self, gate_limit):
        from fastapi.testclient import TestClient

        big = json.dumps({"blob": "A" * (BIG * 3)}).encode()
        resp = TestClient(self._app()).post(
            "/echo", content=big, headers={"Content-Type": "application/json"})
        assert resp.status_code == 200
        assert GATE_MARK not in resp.text

    def test_small_multipart_reaches_the_handler(self, gate_limit):
        from fastapi.testclient import TestClient

        resp = TestClient(self._app()).post(
            "/echo", files={"file": ("payload.json", payload(1024),
                                     "application/json")})
        assert resp.status_code == 200


class TestPlacementIsGuarded:
    """嵌套位置：闸在 CORS 之内，实测挪到外面就丢 CORS 头"""

    def test_real_app_nests_the_gate_inside_cors(self):
        names = middleware_order(app)
        assert "UploadBodyGate" in names, f"真实链里找不到闸：{names}"
        assert names.index("CORSMiddleware") < names.index("UploadBodyGate"), \
            f"闸被装到了 CORS 之外，413 将不带 Access-Control-Allow-Origin：{names}"

    def test_router_level_gate_keeps_cors_header_on_413(self, gate_limit):
        from fastapi.testclient import TestClient

        inside = FastAPI(middleware=[Middleware(UploadBodyGate)])
        inside.add_middleware(CORSMiddleware, allow_origins=[ORIGIN],
                              allow_methods=["*"], allow_headers=["*"])

        @inside.post("/api/data/upload")
        async def sink():
            raise AssertionError("不该走到这里")

        resp = TestClient(inside).post(
            "/api/data/upload",
            files={"file": ("payload.json", payload(), "application/json")},
            headers={"Origin": ORIGIN})
        assert resp.status_code == 413
        assert resp.headers.get("access-control-allow-origin") == ORIGIN

    def test_gate_outside_cors_loses_the_header(self, gate_limit):
        """反证：同一条闸挪到 CORS 之外后，浏览器读不到那句文案"""
        from fastapi.testclient import TestClient

        outside = FastAPI()
        outside.add_middleware(CORSMiddleware, allow_origins=[ORIGIN],
                               allow_methods=["*"], allow_headers=["*"])
        outside.add_middleware(UploadBodyGate)

        @outside.post("/api/data/upload")
        async def sink():
            raise AssertionError("不该走到这里")

        resp = TestClient(outside).post(
            "/api/data/upload",
            files={"file": ("payload.json", payload(), "application/json")},
            headers={"Origin": ORIGIN})
        assert resp.status_code == 413
        assert resp.headers.get("access-control-allow-origin") is None
