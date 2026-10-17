# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L284：错误回执（422 / 4xx）自己渲染崩掉时的兜底

**缺陷形状**（改前实测，本轮仓外探针逐只端点量到）：请求体里写 JSON 的
`NaN` / `Infinity` / `-Infinity` 字面量（Python 的 JSON 解码默认接受这三个），
解出来是真浮点。FastAPI 的默认校验失败处理器把错误项直接交给 JSON 响应，而
pydantic v2 的每一项都带 `input`，即**回显解析后的请求体**；starlette 那侧渲染钉的是
`allow_nan=False` ⇒ 抛 `ValueError: Out of range float values are not JSON compliant: nan`。
于是崩的是**错误回执自己**：客户端从「可行动的 422」掉成裸文本 500，
服务端这条路径也绕开了 `api.deps.raise_internal_error` 的日志收口。

实测五只端点同形（`/api/dataset/search`、`/api/dataset/compare`、`/api/leakage/check`、
`/api/dataset/impact`、`/api/dataset/evaluate`）。判据一律把非有限值放进 `input_file`：
五只都要缺别的必填字段 ⇒ 必定 422，而错误项的 `input` 就是整份请求体，回显里必然带着
那个浮点，因此不依赖具体是哪一类校验错。

**同一形状的第二格**（触发字节不同）：JSON 里的 `"\ud800"` 这类**孤立代理对**解得出来
却编不成 UTF-8，而 starlette 的 JSON 响应是 `ensure_ascii=False` 再 `.encode("utf-8")`。
`detail` 文案里嵌了请求原文的 4xx 因此在渲染阶段抛 `UnicodeEncodeError`，客户端同样掉成
裸文本 500。本文件的靶子钉在 `POST /api/dataset/convert` 的 `source_format`（见
`TestSurrogateEchoGuard`），`/api/dataset/search` 的 `query` / `pattern` / `regex_pattern`
三只缺别的必填键时走的是 422 回显那一支，由同一处兜底的 `_encodable` 覆盖。

修法只兜错误路径：`api/main.py` 那两节各先走默认处理器，渲染抛了才把非有限浮点换成
字符串 token、把编不出的字符换成反斜杠原文，重渲染后状态码不变。**成功路径刻意不脱敏**
（不换 `default_response_class`），否则会掩盖服务端自己算出来的 NaN 或坏字符。

本文件的非恒真证据：`TestFallbackTeeth` 把处理器换回 FastAPI 默认那支，同一个请求当场
500、异常身份逐字 `ValueError: Out of range float values...`；`TestSurrogateEchoGuard`
对 4xx 那支做同一件事（异常身份 `UnicodeEncodeError`，判决整条丢失）；摘掉
`@app.exception_handler` 那一行则 `test_handler_is_registered_on_the_app` 与
`test_http_handler_is_registered_on_the_app` 各自红（判据钉在注册本身）。
"""

import json

import pytest
from fastapi.exception_handlers import (
    http_exception_handler,
    request_validation_exception_handler,
)
from fastapi.exceptions import RequestValidationError
from fastapi.testclient import TestClient
from starlette.exceptions import HTTPException as StarletteHTTPException

from api.main import (
    _encodable,
    _finite_safe,
    app,
    http_error_with_encodable_echo,
    validation_error_with_finite_echo,
)

#: 逐字取自 `api/deps.py` 的 header 别名
TOKEN = "X-API-Key"
KEY = "l284-probe-key"
HEADERS = {TOKEN: KEY, "Content-Type": "application/json"}

#: 五只实测同形的端点（全部 POST，全部会回显请求体）
ENDPOINTS = (
    "/api/dataset/search",
    "/api/dataset/compare",
    "/api/leakage/check",
    "/api/dataset/impact",
    "/api/dataset/evaluate",
)

NAN = float("nan")
INF = float("inf")
NEG_INF = float("-inf")
#: 孤立高代理：JSON 里写作 `"\ud800"`，Python 解得出来却编不成 UTF-8
SURR = "\ud800"


def _body(**fields) -> bytes:
    """拼出含非有限浮点字面量的请求体

    Args:
        **fields: 请求体字段，值直接用 Python 浮点 ⇒ 默认序列化写出 `NaN` /
            `Infinity` / `-Infinity` 三个字面量

    Returns:
        UTF-8 字节串。刻意不用 `TestClient` 的 `json=` 参数，免得序列化口径被换掉
    """
    return json.dumps(fields).encode("utf-8")


def _use_default_handler():
    """把 `RequestValidationError` 换回 FastAPI 默认那支，返回还原所需的原值

    中间件栈在首次请求时固化，所以换完处理器必须复位一次才看得见。

    Returns:
        换下来的原处理器（`finally` 里交回 `_restore_handler`）
    """
    saved = app.exception_handlers[RequestValidationError]
    app.exception_handlers[RequestValidationError] = request_validation_exception_handler
    app.middleware_stack = None
    return saved


def _restore_handler(saved):
    """装回原处理器并复位中间件栈

    Args:
        saved: `_use_default_handler` 换下来的那支
    """
    app.exception_handlers[RequestValidationError] = saved
    app.middleware_stack = None


def _use_plain_http_handler():
    """把 `StarletteHTTPException` 换回 FastAPI 默认那支，返回还原所需的原值

    Returns:
        换下来的原处理器（`finally` 里交回 `_restore_http_handler`）
    """
    saved = app.exception_handlers[StarletteHTTPException]
    app.exception_handlers[StarletteHTTPException] = http_exception_handler
    app.middleware_stack = None
    return saved


def _restore_http_handler(saved):
    """装回原 4xx 处理器并复位中间件栈

    Args:
        saved: `_use_plain_http_handler` 换下来的那支
    """
    app.exception_handlers[StarletteHTTPException] = saved
    app.middleware_stack = None


@pytest.fixture
def client(monkeypatch):
    """带密钥的 TestClient（`raise_server_exceptions=False` ⇒ 未捕获异常落成 500）

    `WriteTransportGate` 在未配密钥时按监听地址判信任传输，而 TestClient 给的
    `server` 是 `("testserver", …)`，解析不成 IP ⇒ 匿名 POST 一律 403，量不到 422。
    因此这里显式设密钥并随请求携带（与 `tests/integration/test_api_dataset_system_tools.py`
    同一手法）。

    Args:
        monkeypatch: pytest 内置 fixture

    Returns:
        `TestClient`
    """
    monkeypatch.setenv("AUGMENTOR_API_KEY", KEY)
    return TestClient(app, raise_server_exceptions=False)


class TestReproClosed:
    """改前 500 的那批载荷，改后必须回 422 且回执是合法 JSON"""

    @pytest.mark.parametrize("path", ENDPOINTS)
    def test_nan_echo_returns_422_not_500(self, client, path):
        """`input_file` 换成 NaN ⇒ 422 而不是裸文本 500

        Args:
            client: 带密钥的 TestClient
            path: 实测同形的端点路径
        """
        resp = client.post(path, content=_body(input_file=NAN), headers=HEADERS)

        assert resp.status_code == 422, resp.text
        assert "Internal Server Error" not in resp.text
        payload = resp.json()  # 「渲染得动」本身就是要钉的那件事
        assert "NaN" in resp.text

        detail = payload["detail"]
        assert detail, "422 必须带可行动的错误项，不能退化成空数组"
        #: 逐条点名要有 `type` / `loc` / `msg` 三格才谈得上「可行动」。判据不钉具体
        #: 是哪一类错：`input_file` 只在 search 上是字符串字段，另外四只把它当额外键忽略，
        #: 于是同一份载荷在它们那边报的是「缺必填字段」，回显里躺着的仍是那个 NaN。
        for item in detail:
            assert {"type", "loc", "msg"} <= set(item), item

    def test_inf_and_neg_inf_get_named_tokens(self, client):
        """`Infinity` 与 `-Infinity` 各回同名 token，不合并成一格

        Args:
            client: 带密钥的 TestClient
        """
        pos = client.post("/api/dataset/search", content=_body(input_file=INF),
                          headers=HEADERS)
        neg = client.post("/api/dataset/search", content=_body(input_file=NEG_INF),
                          headers=HEADERS)

        assert pos.status_code == 422 and neg.status_code == 422
        assert '"Infinity"' in pos.text and '"Infinity"' not in neg.text
        assert '"-Infinity"' in neg.text

    def test_nan_inside_nested_containers_is_walked(self, client):
        """非有限值藏在数组与对象里同样要换成 token（错误项的 `input` 是整份请求体）

        Args:
            client: 带密钥的 TestClient
        """
        resp = client.post(
            "/api/dataset/search",
            content=_body(input_file="train_data.json", tags=["a", NAN],
                          meta={"k": NEG_INF}),
            headers=HEADERS,
        )

        assert resp.status_code == 422, resp.text
        assert "NaN" in resp.text and "-Infinity" in resp.text
        json.loads(resp.text)  # 递归面漏一层就会在这里抛

    def test_legal_422_payload_unchanged(self, client):
        """载荷里没有非有限值时，回执与 FastAPI 默认那支逐字节相同

        这条是「改动克制」的钉子：兜底只允许在默认那支渲染不动时才生效。

        Args:
            client: 带密钥的 TestClient
        """
        ours = client.post("/api/dataset/search", content=_body(input_file=123),
                           headers=HEADERS)
        saved = _use_default_handler()
        try:
            default = client.post("/api/dataset/search", content=_body(input_file=123),
                                  headers=HEADERS)
        finally:
            _restore_handler(saved)

        assert ours.status_code == 422 == default.status_code
        assert ours.content == default.content


class TestFallbackTeeth:
    """摘掉接线就红：证明这套兜底真的在挡那一格 500"""

    def test_default_handler_alone_still_500s(self, client):
        """把处理器换回默认那支 ⇒ 同一个请求当场 500，装回来又复 422

        Args:
            client: 带密钥的 TestClient
        """
        saved = _use_default_handler()
        try:
            resp = client.post("/api/dataset/search", content=_body(input_file=NAN),
                               headers=HEADERS)
            assert resp.status_code == 500, resp.text
            assert "Internal Server Error" in resp.text
        finally:
            _restore_handler(saved)

        again = client.post("/api/dataset/search", content=_body(input_file=NAN),
                            headers=HEADERS)
        assert again.status_code == 422, again.text

    def test_exception_identity_is_value_error(self, monkeypatch):
        """改前的异常身份逐字钉住，免得把兜底接到一个不会抛的形状上

        Args:
            monkeypatch: pytest 内置 fixture
        """
        monkeypatch.setenv("AUGMENTOR_API_KEY", KEY)
        saved = _use_default_handler()
        strict = TestClient(app, raise_server_exceptions=True)
        try:
            with pytest.raises(ValueError, match="Out of range float values"):
                strict.post("/api/dataset/search", content=_body(input_file=NAN),
                            headers=HEADERS)
        finally:
            _restore_handler(saved)

    def test_handler_is_registered_on_the_app(self):
        """接线本身进判据：默认处理器必须被本仓那支顶掉

        摘掉 `@app.exception_handler` 那一行就红，不依赖具体载荷。
        """
        assert app.exception_handlers[RequestValidationError] is (
            validation_error_with_finite_echo
        )


class TestDelegationToDefault:
    """兜底不是替身：默认那支才是主路，本仓那支只在它渲染抛错时接手"""

    @pytest.mark.parametrize(
        ("fields", "status"),
        [({"input_file": 123}, 422), ({"input_file": NAN}, 422)],
        ids=["finite-echo", "non-finite-echo"],
    )
    def test_default_handler_called_once(self, client, monkeypatch, fields, status):
        """两种载荷都恰好调用默认那支一次

        「逐字节不变」那条主张要靠这个才不只是巧合：如果本仓那支直接自己渲染，
        合法载荷同样能对出相同字节，但它已经悄悄换掉了 FastAPI 的口径。

        Args:
            client: 带密钥的 TestClient
            monkeypatch: pytest 内置 fixture
            fields: 请求体字段
            status: 期望状态码
        """
        calls = []
        real = request_validation_exception_handler

        async def spy(request, exc):
            """替身：记一次调用后原样交回默认那支

            Args:
                request: 触发校验失败的请求
                exc: 校验失败异常

            Returns:
                默认那支的响应
            """
            calls.append(exc)
            return await real(request, exc)

        monkeypatch.setattr("api.main.request_validation_exception_handler", spy)

        resp = client.post("/api/dataset/search", content=_body(**fields),
                           headers=HEADERS)

        assert resp.status_code == status, resp.text
        assert len(calls) == 1, f"默认处理器应被调用一次，实得 {len(calls)}"


class TestSurrogateEchoGuard:
    """同形状的第二格：`detail` 里嵌着**请求原文**的 4xx 也渲染不动孤立代理对

    实测靶子（本轮 L284 的仓外探针）：`POST /api/dataset/convert` 把 `source_format`
    喂一枚孤立高代理，那句判决是「src.csv 声明为 （原文） ，但它既不是合法 JSON…」——
    **请求原文原样进文案**，于是 400 的回执在 `JSONResponse.render` 里抛
    `UnicodeEncodeError`。摘掉本仓的 `HTTPException` 处理器，同一请求回裸文本 500；
    装上就回 400，且 `detail` 里那枚字符变成反斜杠原文（六个可见字符），判决的中文
    部分一字未动。
    """

    def test_convert_verdict_renders_with_the_char_escaped(self, client, tmp_path):
        """`source_format` 含孤立代理 ⇒ 400 且文案把那一枚转义成原文

        Args:
            client: 带密钥的 TestClient
            tmp_path: pytest 临时目录（已 autouse 进数据白名单）
        """
        src = tmp_path / "src.csv"
        src.write_text("instruction,output\na,b\n", encoding="utf-8")
        out = tmp_path / "out.json"
        resp = client.post("/api/dataset/convert",
                           content=_body(input_file=str(src), output_file=str(out),
                                         source_format=SURR),
                           headers=HEADERS)

        assert resp.status_code == 400, resp.text
        detail = resp.json()["detail"]
        assert "声明为" in detail and "src.csv" in detail, detail
        assert SURR not in detail, "原样回带那枚字符就还是会渲染崩"
        assert "\\ud800" in detail, detail

    def test_legal_http_error_bytes_unchanged(self, client, tmp_path):
        """能正常渲染的 4xx 一个字节都没变：兜底只在默认那支抛错时接手

        Args:
            client: 带密钥的 TestClient
            tmp_path: pytest 临时目录
        """
        missing = tmp_path / "nope.json"
        ours = client.get(f"/api/data/load/{missing.name}", headers=HEADERS)
        saved = _use_plain_http_handler()
        try:
            plain = client.get(f"/api/data/load/{missing.name}", headers=HEADERS)
        finally:
            _restore_http_handler(saved)

        assert ours.status_code == 404 == plain.status_code
        assert ours.content == plain.content
        assert ours.json() == {"detail": "文件不存在"}

    def test_removing_the_guard_brings_the_bare_500_back(self, client, tmp_path):
        """摘掉接线就红：默认那支渲染不动时是裸文本 500，且不带任何判决

        Args:
            client: 带密钥的 TestClient
            tmp_path: pytest 临时目录
        """
        src = tmp_path / "src.csv"
        src.write_text("instruction,output\na,b\n", encoding="utf-8")
        payload = _body(input_file=str(src), output_file=str(tmp_path / "o.json"),
                        source_format=SURR)
        saved = _use_plain_http_handler()
        try:
            bare = client.post("/api/dataset/convert", content=payload, headers=HEADERS)
            assert bare.status_code == 500, bare.text
            assert "Internal Server Error" in bare.text
            assert "detail" not in bare.text
        finally:
            _restore_http_handler(saved)

        guarded = client.post("/api/dataset/convert", content=payload, headers=HEADERS)
        assert guarded.status_code == 400, guarded.text

    def test_http_handler_is_registered_on_the_app(self):
        """接线本身进判据：`StarletteHTTPException` 由本仓那支顶掉默认

        摘掉 `@app.exception_handler(StarletteHTTPException)` 那一行就红。
        """
        assert app.exception_handlers[StarletteHTTPException] is (
            http_error_with_encodable_echo
        )

    @pytest.mark.parametrize("field", ["query", "pattern", "regex_pattern"])
    def test_surrogate_in_the_422_echo_is_also_escaped(self, client, field):
        """422 那一支也吃下了孤立代理：`UnicodeEncodeError` 是 `ValueError` 的子类

        探针实测（本轮 L284 仓外）：这三只字段缺别的必填键 ⇒ 必定 422，而 pydantic 的
        错误项把**整份请求体**放进 `input`，那枚代理就在回显里。上一节的
        `except ValueError` 因此天然覆盖这一格，兜底里串的 `_encodable` 负责转义。

        Args:
            client: 带密钥的 TestClient
            field: 回显里带孤立代理的字段名
        """
        resp = client.post("/api/dataset/search",
                           content=_body(**{field: SURR}), headers=HEADERS)

        assert resp.status_code == 422, resp.text
        assert SURR not in resp.text, "原样回带那枚字符就还是渲染崩"
        #: 响应文本是 JSON，字符串里的那枚反斜杠自身再被转义一次 ⇒ 两个反斜杠
        assert "\\\\ud800" in resp.text, resp.text
        json.loads(resp.text)

    @pytest.mark.parametrize(
        ("value", "expected"),
        [
            (SURR, "\\ud800"),
            ("\udcff", "\\udcff"),
            ("正常文本", "正常文本"),
            ({"k": [SURR, "ok"]}, {"k": ["\\ud800", "ok"]}),
            (("a", SURR), ["a", "\\ud800"]),
            (None, None),
            (3, 3),
        ],
    )
    def test_encodable_walks_each_shape(self, value, expected):
        """逐形状一档：只有编不出的那段被转义，容器递归，元组降成列表

        Args:
            value: 输入节点
            expected: 期望输出
        """
        assert _encodable(value) == expected

    def test_valid_surrogate_pair_is_left_alone(self):
        """合法代理对（emoji）编码得动 ⇒ 兜底一个字都不许改

        判据是「能不能编成 UTF-8」而不是「是不是在代理区」，所以成对的高/低代理
        必须原样通过，否则这套兜底就成了对中文与 emoji 的过度收紧。
        """
        emoji = "\U0001F600"

        assert _encodable(emoji) == emoji
        assert emoji.encode("utf-8") == b"\xf0\x9f\x98\x80"


class TestFiniteSafeUnit:
    """递归函数的逐档读数（覆盖率与「只改非有限值」这一主张都要它）"""

    @pytest.mark.parametrize(
        ("value", "expected"),
        [
            (NAN, "NaN"),
            (INF, "Infinity"),
            (NEG_INF, "-Infinity"),
            (0.5, 0.5),
            ("text", "text"),
            (7, 7),
            (None, None),
            (True, True),
            ({"k": [NAN, INF], "j": 1.5}, {"k": ["NaN", "Infinity"], "j": 1.5}),
            ([{"a": NEG_INF}], [{"a": "-Infinity"}]),
            ({}, {}),
            ([], []),
        ],
    )
    def test_each_shape(self, value, expected):
        """逐形状一档：非有限浮点换 token，容器递归，其余原样

        Args:
            value: 输入节点
            expected: 期望输出
        """
        assert _finite_safe(value) == expected

    def test_does_not_mutate_input(self):
        """换 token 走新建容器，不改调用方给的那份

        默认那支已经把同一份载荷交给过一次，改坏了会让「其余逐字节不变」那条主张失效。
        """
        source = {"detail": [{"input": {"v": NAN}}]}
        _finite_safe(source)

        assert isinstance(source["detail"][0]["input"]["v"], float)
