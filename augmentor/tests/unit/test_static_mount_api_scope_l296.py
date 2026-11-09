# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A361① / L296 守卫：`/` 上的静态兜底只接非 API 路径

**缺陷形状**：`StaticFiles(html=True)` 挂在 `/` 时会吃掉所有没被前面路由命中的路径，
包括 `/api/...`；而 `StaticFiles` 对非 GET 一律回 **405 Method Not Allowed**。于是端点名
拼错的客户端收到的是「方法不允许」而不是「不存在」，且这个读数随 `web/dist` 是否在场
漂移（构建过前端的机器 405、干净克隆 404）。

**修法**：`_NonApiStaticMount` 在**匹配阶段**让 `/api` 命名空间落空（`Match.NONE`），
路由表自己的判决重新成为唯一权威——未知 API 路径 **404**、已知路径的错误方法仍 **405**；
非 API 路径继续由 `StaticFiles` 处理（`/` 的 index、静态资源、非 GET 的 405 都保持不变）。

**三面守卫**：

1. 真实装配（`api.main.app`）：未知 `/api` 路径无论 GET / POST / PUT / PATCH / DELETE
   都是 404；已知路径的错误方法仍是 405；合法请求不受影响。这些读数在「挂载在场」与
   「挂载不在场」两档一致，所以可以直接钉常数。
2. 临时静态目录（`_NonApiStaticMount` + 真 `StaticFiles`）：不依赖本机有没有 `web/dist`，
   在任意干净克隆上都能量到挂载本身的行为；`/apix` 必须仍归静态（证明判据只认 `/api`
   命名空间，不是裸 `startswith("/api")`）。
3. 直接 `matches()`：`/api`、`/api/`、`/api/x` 落空；`/`、`/apix`、`/assets/x` 仍匹配；
   `root_path` 先被减掉再判。

**注入取证**（改完逐字节复原）：① 装配退回普通 `Mount` ⇒ 真实装配的未知 API 路径与
已知路径错方法两支红，接线 AST 支也红（临时目录臂仍绿，因为它直接实例化本类）；
② 摘掉本类的 `/api` 拒绝 ⇒ 临时目录臂、命名空间边界与真实装配的未知 API 路径支全红。

跑法：`python -m pytest tests/unit/test_static_mount_api_scope_l296.py -q --no-cov`
"""

import ast
import pathlib

import pytest
from fastapi.testclient import TestClient
from starlette.applications import Starlette
from starlette.responses import PlainTextResponse
from starlette.routing import Match, Route
from starlette.staticfiles import StaticFiles

from api.main import _NonApiStaticMount, app

ROOT = pathlib.Path(__file__).resolve().parents[2]
MAIN_PY = ROOT / "api" / "main.py"

API_METHODS = ("GET", "POST", "PUT", "PATCH", "DELETE")


@pytest.fixture
def http(tmp_path, monkeypatch):
    """真实装配：数据根指到临时目录，避免读写到开发者本机数据"""
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    monkeypatch.chdir(tmp_path)
    return TestClient(app, raise_server_exceptions=False)


class TestTheRealAssembly:
    """真实 `api.main.app`：API 面的读数不随 `web/dist` 是否在场分叉"""

    def test_unknown_api_paths_are_404_for_every_method(self, http):
        for method in API_METHODS:
            resp = http.request(method, "/api/definitely-not-a-path-xyz",
                                json={} if method in ("POST", "PUT", "PATCH") else None)
            assert resp.status_code == 404, f"{method}: {resp.status_code} {resp.text[:120]}"

    def test_the_api_root_itself_is_404(self, http):
        for method in ("GET", "POST", "DELETE"):
            resp = http.request(method, "/api",
                                json={} if method == "POST" else None)
            assert resp.status_code == 404, f"{method}: {resp.status_code} {resp.text[:120]}"

    def test_known_api_paths_keep_the_router_405(self, http):
        # GET-only 端点收到 POST ⇒ 路由表部分匹配 405（不是静态挂载的 405，也不是 404）
        assert http.post("/api/health", json={}).status_code == 405
        # POST-only 端点收到 GET ⇒ 同上；改前挂载在场时这一格是 404
        assert http.get("/api/system/backups/whatever/restore").status_code == 405

    def test_known_api_paths_still_answer_their_own_method(self, http):
        resp = http.get("/api/health")
        assert resp.status_code == 200, resp.text[:120]
        assert resp.json()["status"] == "ok"

    def test_non_api_unknown_get_is_still_404(self, http):
        assert http.get("/not-an-api-route").status_code == 404


class TestTheMountItself:
    """临时静态目录：不依赖本机 `web/dist`，两档环境都能量到挂载行为"""

    @pytest.fixture
    def static_http(self, tmp_path):
        dist = tmp_path / "dist"
        dist.mkdir()
        (dist / "index.html").write_text("<html>index</html>", encoding="utf-8")
        (dist / "app.js").write_text("console.log(1)", encoding="utf-8")

        async def known(request):
            return PlainTextResponse("ok")

        mini = Starlette(routes=[
            Route("/api/known", known, methods=["GET"]),
            _NonApiStaticMount(
                "/",
                app=StaticFiles(directory=str(dist), html=True),
                name="static",
            ),
        ])
        return TestClient(mini, raise_server_exceptions=False)

    def test_unknown_api_paths_are_not_swallowed(self, static_http):
        for method in API_METHODS:
            resp = static_http.request(method, "/api/unknown",
                                       json={} if method in ("POST", "PUT", "PATCH") else None)
            assert resp.status_code == 404, f"{method}: {resp.status_code} {resp.text[:120]}"
        for method in ("GET", "POST"):
            resp = static_http.request(method, "/api",
                                       json={} if method == "POST" else None)
            assert resp.status_code == 404, f"{method}: {resp.status_code} {resp.text[:120]}"

    def test_known_api_path_keeps_the_router_405(self, static_http):
        assert static_http.get("/api/known").status_code == 200
        assert static_http.post("/api/known").status_code == 405

    def test_non_api_paths_still_go_to_static(self, static_http):
        assert static_http.get("/").status_code == 200
        assert static_http.get("/").text == "<html>index</html>"
        assert static_http.get("/app.js").status_code == 200
        # 非 API 路径的非 GET 仍由 StaticFiles 回 405 —— 本轮没有把静态面整体改成 404
        assert static_http.post("/app.js").status_code == 405
        # `/apix` 不是 `/api` 命名空间：GET 走静态查找（404），POST 仍由静态回 405
        assert static_http.get("/apix").status_code == 404
        assert static_http.post("/apix").status_code == 405


class TestTheNamespaceBoundary:
    """直接量 `matches()`：只拒 `/api` 命名空间，不拒同前缀的普通路径"""

    @pytest.fixture
    def mount(self, tmp_path):
        (tmp_path / "index.html").write_text("index", encoding="utf-8")
        return _NonApiStaticMount(
            "/", app=StaticFiles(directory=str(tmp_path), html=True), name="static"
        )

    def _match(self, mount, path, root_path=""):
        return mount.matches({"type": "http", "path": path, "root_path": root_path})[0]

    def test_the_api_namespace_is_declined(self, mount):
        for path in ("/api", "/api/", "/api/x", "/api/x/y"):
            assert self._match(mount, path) is Match.NONE, path

    def test_ordinary_paths_are_still_matched(self, mount):
        for path in ("/", "/apix", "/assets/app.js", "/apiary"):
            assert self._match(mount, path) is Match.FULL, path

    def test_root_path_is_subtracted_before_the_check(self, mount):
        assert self._match(mount, "/prefix/api/x", root_path="/prefix") is Match.NONE
        assert self._match(mount, "/prefix/apix", root_path="/prefix") is Match.FULL


def test_the_assembly_uses_the_non_api_mount():
    """接线面：`api/main.py` 必须在 `app.router.routes.append(...)` 里实例化本类

    只查类名不够：类定义还在、装配退回普通 `Mount(...)` 时，裸字符串断言照样绿。
    这里按 AST 钉住「追加的实参是 `_NonApiStaticMount(...)`」且全文没有 `Mount(...)` 调用。
    """
    tree = ast.parse(MAIN_PY.read_text(encoding="utf-8"))

    def _is_routes_append(node):
        if not isinstance(node, ast.Call):
            return False
        func = node.func
        if not (isinstance(func, ast.Attribute) and func.attr == "append"):
            return False
        routes = func.value
        if not (isinstance(routes, ast.Attribute) and routes.attr == "routes"):
            return False
        router = routes.value
        return (isinstance(router, ast.Attribute) and router.attr == "router"
                and isinstance(router.value, ast.Name) and router.value.id == "app")

    appends = [node for node in ast.walk(tree) if _is_routes_append(node)]
    assert len(appends) == 1, f"`app.router.routes.append` 现量 {len(appends)} 处，期望 1"
    mounted = appends[0].args[0]
    assert isinstance(mounted, ast.Call)
    assert isinstance(mounted.func, ast.Name) and mounted.func.id == "_NonApiStaticMount"

    plain_mounts = [
        node for node in ast.walk(tree)
        if isinstance(node, ast.Call)
        and isinstance(node.func, ast.Name) and node.func.id == "Mount"
    ]
    assert not plain_mounts, "装配不许退回普通 `Mount(...)`"
