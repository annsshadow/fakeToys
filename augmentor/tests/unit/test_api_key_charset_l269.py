# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""鉴权比较函数的字符集限制（L269）

**缺陷形**：`api/deps.py::verify_api_key` 从前直接把两个 **str** 交给
`secrets.compare_digest`，而 CPython 那条实现「对 str 只接受纯 ASCII」——非 ASCII 字符
当场抛 `TypeError`。抛点在依赖解析里，早于路由函数体，所以路由自己的 `except` 拦不到，
异常一路冒到框架：那句「401 未提供或密钥不匹配」的契约对非 ASCII 输入根本不成立。
改前后各跑一次同一个输入：改前 `TypeError`，改后 401。

**为什么这不是理论风险**（三面都是本轮实测，不是推演）：
- HTTP 头按 latin-1 解码（Starlette `Headers` 的口径），线路上任意 0x80–0xFF 字节都会成
  一个合法字符。客户端只改一个请求头，就能把「已配置密钥」的部署上**每一个写端点**稳定
  打成 500——`/api/config`、`/api/data/*`、`/api/system/*` 全在射程内。
- 更糟的是密钥自己：`AUGMENTOR_API_KEY` 里只要有一个非 ASCII 字符，`compare_digest` 对
  **任何**输入都抛，于是正确密钥也 500，鉴权面整体不可用（改前实测：配 `pässwort` 之后
  带同一串逐字正确的密钥请求 → `TypeError`）。
- `httpx` / `TestClient` 那一层量不到它：客户端先把头按 ASCII 编码，非 ASCII 直接
  `UnicodeEncodeError`，请求根本没发出去（实测）。所以这一形必须下到 **ASGI scope** 用
  原始字节头来量，否则守卫会写出「HTTP 面打不出来 ⇒ 只有 SDK 面」的假结论。

**判据面**（三面闭合，全部可复算）：
1. 站点面：产品码（`api/` + `augmentor/` + `cli/`）里 `compare_digest` 的**调用**恰好一处，
   且两个操作数都是 `.encode(...)` 的结果——走 AST，不看文本；本轮新写的注释里就出现
   `compare_digest` 这个词，文本扫会把注释算成站点。
2. 判决面：八档输入（缺失 / 空串 / 正确 ASCII / 错误 ASCII / 非 ASCII 输入配 ASCII 密钥 /
   非 ASCII 输入配非 ASCII 密钥且逐字相等 / 同一把非 ASCII 密钥差一个字符 / 多字节 UTF-8
   字符）的判决只有「放行」与 401 两种，任何一档都不许冒 `TypeError`。
3. 线路面：原始字节头 `0xff` 与 UTF-8 多字节头 `0xc3 0xa9` 两档都拿到**写好的响应**，
   而不是异常冒泡。

**牙齿**：`TestTheRulerHasTeeth` 三支全部零落盘——① 直接调「旧写法复刻」，同一批输入
当场 `TypeError`；② 把 `deps.verify_api_key` 换成那一档复刻，ASGI 面不再写出响应而是
异常冒泡；③ 在内存里把源码的 `.encode("utf-8")` 摘掉，AST 面的 bytes 操作数当场掉到 0。

跑法：`python -m pytest tests/unit/test_api_key_charset_l269.py -q --no-cov`
"""
import ast
import asyncio
import os
import secrets
from pathlib import Path
from typing import Optional

import pytest
from fastapi import Depends, FastAPI, Header, HTTPException

from api import deps

AI_DIR = Path(__file__).resolve().parent.parent.parent
PRODUCT_DIRS = ("api", "augmentor", "cli")

#: 现量（L269 立尺）：全产品码 `compare_digest` 调用点数。
MEASURED_SITES = 1

KEY = "test-key-ascii"
NON_ASCII_KEY = "pässwort"
#: latin-1 高位字符（HTTP 头解码后的形状）与一个多字节 UTF-8 字符。
HIGH_LATIN1 = "ÿ"
HIGH_UTF8 = "é"


def product_source_files():
    """产品码的 `.py` 名单

    Args:
        无

    Returns:
        `[(相对路径, 源码文本)]`，按目录与路径排序
    """
    out = []
    for part in PRODUCT_DIRS:
        for path in sorted((AI_DIR / part).rglob("*.py")):
            out.append((str(path.relative_to(AI_DIR)), path.read_text(encoding="utf-8")))
    return out


def compare_digest_calls(text):
    """源码文本里所有 `compare_digest(...)` 调用点（AST 形状，注释不进面）

    Args:
        text: 源码文本

    Returns:
        `list[ast.Call]`
    """
    hits = []
    for node in ast.walk(ast.parse(text)):
        if isinstance(node, ast.Call) and getattr(node.func, "attr", None) == "compare_digest":
            hits.append(node)
    return hits


def is_encode(node):
    """该表达式是不是 `<某值>.encode(...)`

    Args:
        node: `ast` 节点

    Returns:
        bool
    """
    return (
        isinstance(node, ast.Call)
        and isinstance(node.func, ast.Attribute)
        and node.func.attr == "encode"
    )


def byte_operand_sites(text):
    """`compare_digest` 调用点中「两个操作数都是 bytes 档」的那些

    Args:
        text: 源码文本

    Returns:
        `[(行号, 是否双 encode)]`
    """
    return [(c.lineno, all(is_encode(a) for a in c.args)) for c in compare_digest_calls(text)]


def verdict(x_api_key, configured):
    """在 `AUGMENTOR_API_KEY=configured` 之下调一次真函数，取判决

    Args:
        x_api_key: 请求头值（`None` 表示不带该头）
        configured: 环境变量里的密钥，`None` 表示不设置

    Returns:
        放行返回 `"ALLOW"`，被拒返回状态码 int；其他异常原样上抛——那正是要抓的形状
    """
    previous = os.environ.get(deps.API_KEY_ENV)
    if configured is None:
        os.environ.pop(deps.API_KEY_ENV, None)
    else:
        os.environ[deps.API_KEY_ENV] = configured
    try:
        deps.verify_api_key(x_api_key)
        return "ALLOW"
    except HTTPException as exc:
        return exc.status_code
    finally:
        if previous is None:
            os.environ.pop(deps.API_KEY_ENV, None)
        else:
            os.environ[deps.API_KEY_ENV] = previous


def legacy_verify(x_api_key, expected):
    """改前的写法复刻（两个 str 直传），只给注入臂用

    Args:
        x_api_key: 请求头值
        expected: 配置里的密钥

    Raises:
        HTTPException: 缺失或不匹配
        TypeError: 非 ASCII 输入 —— 这就是本轮修掉的缺陷形
    """
    if not expected:
        return
    if not x_api_key or not secrets.compare_digest(x_api_key, expected):
        raise HTTPException(status_code=401, detail="缺少或无效的 X-API-Key")


def legacy_dependency(x_api_key: Optional[str] = Header(None, alias="X-API-Key")) -> None:
    """旧写法挂在 HTTP 依赖上的形状（只给削弱臂用）

    参数标记必须与真函数同形（`Header(None, alias=...)`）：少了它 FastAPI 会把 `x_api_key`
    当查询参数绑，头就没进过比较，臂会「不红」而原因跟被修的缺陷毫无关系。

    Args:
        x_api_key: 请求头 X-API-Key

    Raises:
        HTTPException: 缺失或不匹配
        TypeError: 非 ASCII 输入 —— 改前那一档
    """
    legacy_verify(x_api_key, os.environ.get(deps.API_KEY_ENV, "").strip())


async def _asgi_call(header_value_bytes, dependency):
    """把原始字节头喂进一个只挂某条依赖的 ASGI 应用

    `TestClient` 走 httpx，它会先把头按 ASCII 编码，非 ASCII 被客户端自己拒掉，所以这一档
    绕开 httpx 直接调 ASGI——那也正是真实服务器（h11 按 latin-1 解头）交给应用的东西。

    Args:
        header_value_bytes: 头的原始字节
        dependency: 挂到路由上的依赖函数

    Returns:
        响应状态码 int

    Raises:
        Exception: 依赖里冒出来的异常（旧写法就是这一档，且响应一格没写）
    """
    app = FastAPI()

    @app.get("/probe", dependencies=[Depends(dependency)])
    def handler():
        return {"ok": 1}

    scope = {
        "type": "http",
        "asgi": {"version": "2.0", "spec_version": "2.1"},
        "http_version": "1.1",
        "method": "GET",
        "path": "/probe",
        "raw_path": b"/probe",
        "query_string": b"",
        "root_path": "",
        "scheme": "http",
        "server": ("testserver", 80),
        "client": ("127.0.0.1", 12345),
        "headers": [(b"x-api-key", header_value_bytes)],
    }
    messages = []

    async def receive():
        return {"type": "http.request", "body": b"", "more_body": False}

    async def send(message):
        messages.append(message)

    await app(scope, receive, send)
    started = next(m for m in messages if m["type"] == "http.response.start")
    return started["status"]


def asgi_status(header_value_bytes, dependency=None):
    """同步包装：跑一次 `_asgi_call`

    Args:
        header_value_bytes: 头的原始字节
        dependency: 依赖函数，默认取 `deps.verify_api_key` 的**当前**值（因此可被 monkeypatch）

    Returns:
        响应状态码 int
    """
    return asyncio.run(_asgi_call(header_value_bytes, dependency or deps.verify_api_key))


class TestTheSiteFace:
    """站点面：产品码只有一处常量时间比较，而它比的是 bytes"""

    def test_the_face_is_exactly_one_call_site(self):
        """现量 1 处调用点，就在 `api/deps.py`"""
        sites = []
        for rel, text in product_source_files():
            sites.extend((rel, c.lineno) for c in compare_digest_calls(text))
        assert len(sites) == MEASURED_SITES, sites
        assert sites[0][0] == str(Path("api") / "deps.py"), sites

    def test_that_one_site_compares_bytes(self):
        """两个操作数都是 `.encode(...)` 的结果（str 直传就是本轮的缺陷形）

        名册键不用行号：这一格要钉的是「那唯一一处的操作数形状」，把行号写进断言会让每次
        在 `verify_api_key` 上方加注释的正常改动都变成一次假红。
        """
        text = (AI_DIR / "api" / "deps.py").read_text(encoding="utf-8")
        sites = byte_operand_sites(text)
        assert len(sites) == MEASURED_SITES, sites
        assert all(bytes_pair for _, bytes_pair in sites), sites


class TestTheVerdictFace:
    """判决面：八档输入只有放行与 401 两种结果"""

    def test_missing_header_is_401(self):
        """不带密钥 → 401"""
        assert verdict(None, KEY) == 401

    def test_empty_header_is_401(self):
        """空串密钥 → 401（既不是「没传」也不是放行）"""
        assert verdict("", KEY) == 401

    def test_correct_ascii_key_passes(self):
        """ASCII 正确密钥 → 放行"""
        assert verdict(KEY, KEY) == "ALLOW"

    def test_wrong_ascii_key_is_401(self):
        """ASCII 错误密钥 → 401"""
        assert verdict("nope", KEY) == 401

    def test_unset_key_stays_open_as_documented(self):
        """未配置密钥时不校验——这是既有契约（`A341` 的放大器那一格），本轮未动它"""
        assert verdict(HIGH_LATIN1, None) == "ALLOW"

    def test_non_ascii_input_against_ascii_key_is_401(self):
        """改前那一档 500：非 ASCII 输入 + ASCII 配置 → 现在是 401"""
        assert verdict(HIGH_LATIN1 + KEY, KEY) == 401

    def test_non_ascii_key_matches_its_own_value(self):
        """改前连正确密钥都 500：配置里就是非 ASCII 时，逐字相等应当放行"""
        assert verdict(NON_ASCII_KEY, NON_ASCII_KEY) == "ALLOW"

    def test_non_ascii_key_rejects_a_truncated_value(self):
        """同一把非 ASCII 密钥，少一个字符仍要 401 而不是放行"""
        assert verdict(NON_ASCII_KEY[:-1], NON_ASCII_KEY) == 401

    def test_multibyte_utf8_char_is_401(self):
        """多字节 UTF-8 字符同样不许冒 `TypeError`"""
        assert verdict(HIGH_UTF8, KEY) == 401


class TestTheWireFace:
    """线路面：原始字节头拿到写好的响应，而不是异常冒泡"""

    def test_latin1_high_byte_header_gets_401(self, monkeypatch):
        """单字节 `0xff`（latin-1 解成 `ÿ`）→ 401，且响应真的写出来了"""
        monkeypatch.setenv(deps.API_KEY_ENV, KEY)
        assert asgi_status(b"\xff") == 401

    def test_multibyte_utf8_header_gets_401(self, monkeypatch):
        """两字节 `0xc3 0xa9`（latin-1 解成 `Ã©`）→ 401"""
        monkeypatch.setenv(deps.API_KEY_ENV, KEY)
        assert asgi_status(HIGH_UTF8.encode("utf-8")) == 401

    def test_matching_header_on_the_wire_is_200(self, monkeypatch):
        """正向对照臂：同一套 ASGI 驱动下，逐字正确的头拿到 200 —— 证明上面两支的 401
        不是因为「路由压根没跑依赖」"""
        monkeypatch.setenv(deps.API_KEY_ENV, KEY)
        assert asgi_status(KEY.encode("utf-8")) == 200


class TestTheRulerHasTeeth:
    """牙齿：三支削弱臂，全部零落盘"""

    def test_arm_legacy_shape_raises_type_error(self):
        """旧写法复刻在同一批输入上抛 `TypeError` ⇒ 判决面那几支不是恒真"""
        with pytest.raises(TypeError):
            legacy_verify(HIGH_LATIN1, KEY)
        with pytest.raises(TypeError):
            legacy_verify(KEY, NON_ASCII_KEY)

    def test_arm_legacy_dependency_breaks_the_wire_face(self, monkeypatch):
        """把依赖换成旧写法复刻，ASGI 面不再写出 401 而是异常冒泡 ⇒ 线路面有牙齿"""
        monkeypatch.setenv(deps.API_KEY_ENV, KEY)
        monkeypatch.setattr(deps, "verify_api_key", legacy_dependency)
        with pytest.raises(TypeError):
            asgi_status(b"\xff")
        # 正向对照：同一个臂上逐字正确的头仍然放行 —— 说明冒出来的不是「依赖没跑」
        with pytest.raises(HTTPException) as caught:
            legacy_verify("nope", KEY)
        assert caught.value.status_code == 401

    def test_arm_stripping_encode_leaves_the_site_face(self):
        """内存里摘掉 `.encode()` ⇒ 双 bytes 操作数那一格当场掉到 0"""
        text = (AI_DIR / "api" / "deps.py").read_text(encoding="utf-8")
        weakened = text.replace(".encode()", "")
        assert weakened != text
        assert [s for s in byte_operand_sites(weakened) if s[1]] == []
