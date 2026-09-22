# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""前端契约端到端测试

前端通过 /api/* 与后端交互，任何一侧单方面改动都会导致线上白屏。
这里校验：前端源码结构完整 + 前端调用的每个接口在后端都有对应路由。

说明：本测试不启动浏览器。真实的浏览器级 E2E 需要先构建前端产物
（npm install && npm run build），在无网络/未构建环境下会失败，
因此这里以「接口契约」形式固化前后端一致性，作为 CI 的快速门禁。
"""

import re
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent
WEB_SRC = AI_DIR / "web" / "src"


def read_source(path: Path) -> str:
    """读取前端源码

    Args:
        path: 文件路径

    Returns:
        文件内容
    """
    return path.read_text(encoding='utf-8')


class TestWebSources:
    """前端源码结构"""

    def test_entry_files_exist(self):
        """入口文件必须存在，否则无法构建"""
        for name in ["App.tsx", "main.tsx", "services/api.ts"]:
            assert (WEB_SRC / name).exists(), f"缺少前端文件: {name}"

    @pytest.mark.parametrize("page", [
        "DataManagement.tsx",
        "Augmentation.tsx",
        "Analysis.tsx",
        "Versions.tsx",
        "Settings.tsx",
        "Quality.tsx",
        "Export.tsx",
        "Multimodal.tsx",
        "Dashboard.tsx",
    ])
    def test_pages_exist(self, page):
        """计划要求的功能页面都必须存在"""
        assert (WEB_SRC / "pages" / page).exists(), f"缺少页面: {page}"

    @pytest.mark.parametrize("component", [
        "DataList.tsx",
        "AugmentForm.tsx",
        "QualityChart.tsx",
        "ExportDialog.tsx",
        "VersionTimeline.tsx",
    ])
    def test_components_exist(self, component):
        """计划要求的可复用组件都必须存在"""
        assert (WEB_SRC / "components" / component).exists(), f"缺少组件: {component}"

    def test_app_registers_all_pages(self):
        """App.tsx 必须为每个页面注册路由与菜单项，否则页面无法访问"""
        app_source = read_source(WEB_SRC / "App.tsx")

        for page in ["Quality", "Export", "Multimodal", "Dashboard"]:
            # 支持静态导入和动态导入（lazy）
            assert (
                f"import {page} from" in app_source
                or f"{page} = lazy(() => import('./pages/{page}'))" in app_source
                or f"const {page} = lazy" in app_source
            ), f"App.tsx 未导入页面 {page}"
            assert f"<{page} />" in app_source, f"App.tsx 未注册路由 {page}"

    def test_app_menu_covers_new_pages(self):
        """新页面需出现在侧边菜单中，否则用户无法导航"""
        app_source = read_source(WEB_SRC / "App.tsx")

        for path in ["/quality", "/export", "/multimodal", "/dashboard"]:
            assert f"'{path}'" in app_source, f"菜单缺少入口 {path}"


@pytest.fixture(scope="module")
def frontend_calls():
    """提取前端调用的接口路径

    Returns:
        (method, path) 集合
    """
    source = read_source(WEB_SRC / "services" / "api.ts")
    pattern = re.compile(r"api\.(get|post|put|delete)\(\s*[`'\"]([^`'\"]+)[`'\"]")
    calls = set()

    for method, raw_path in pattern.findall(source):
        # 模板变量统一视为路径参数；查询串不属于路径
        path_only = raw_path.split("?")[0]
        normalized = re.sub(r"\$\{[^}]+\}", "{}", path_only)
        calls.add((method.upper(), f"/api{normalized}"))

    assert calls, "未从 api.ts 中解析到任何接口调用"
    return calls


@pytest.fixture(scope="module")
def backend_routes():
    """提取后端注册的路由

    通过 OpenAPI schema 枚举，避免依赖不同 FastAPI 版本内部的路由容器实现
    （部分版本会把 include_router 的产物包装为 _IncludedRouter，不展开子路由）。

    Returns:
        (method, path) 集合
    """
    from api.main import app

    routes = set()
    for path, operations in app.openapi().get("paths", {}).items():
        for method in operations:
            routes.add((method.upper(), path))
    return routes


class TestApiContract:
    """前后端接口契约"""

    @staticmethod
    def _to_regex(path: str) -> str:
        """把路径中的参数占位符转成正则

        Args:
            path: 含 {} 或 {name} 的路径

        Returns:
            正则字符串
        """
        token = "\x00PH\x00"
        templated = re.sub(r"\{[^}]*\}", token, path)
        return re.escape(templated).replace(re.escape(token), "[^/]+")

    @classmethod
    def matches(cls, frontend_path: str, backend_path: str) -> bool:
        """判断前端路径模板是否匹配后端路由

        Args:
            frontend_path: 前端路径（{} 表示参数占位）
            backend_path: 后端路径（{param} 表示参数占位）

        Returns:
            是否匹配
        """
        concrete = re.sub(r"\{[^}]*\}", "sample", frontend_path)
        return re.fullmatch(cls._to_regex(backend_path), concrete) is not None

    def test_all_frontend_calls_are_served(self, frontend_calls, backend_routes):
        """前端调用的每个接口都必须有后端路由，否则线上必然 404"""
        missing = []

        for method, path in frontend_calls:
            if any(
                method == backend_method and self.matches(path, backend_path)
                for backend_method, backend_path in backend_routes
            ):
                continue
            missing.append(f"{method} {path}")

        assert not missing, f"以下前端接口在后端缺失: {missing}"

    def test_health_endpoint_registered(self, backend_routes):
        """健康检查接口必须存在"""
        assert ("GET", "/api/health") in backend_routes

    def test_quality_endpoints_registered(self, backend_routes):
        """计划要求的质量接口必须注册"""
        paths = {path for _, path in backend_routes}
        assert "/api/quality/evaluate" in paths
        assert "/api/quality/report" in paths

    def test_export_endpoints_registered(self, backend_routes):
        """计划要求的导出接口必须注册"""
        paths = {path for _, path in backend_routes}
        assert "/api/export/batch" in paths
        assert "/api/export/preview" in paths

    def test_multimodal_endpoints_registered(self, backend_routes):
        """计划要求的多模态接口必须注册"""
        paths = {path for _, path in backend_routes}
        assert "/api/multimodal/process" in paths


class TestWebBuildConfig:
    """前端构建配置"""

    def test_package_json_has_build_script(self):
        """package.json 必须提供 build 脚本，供 CI 与 Docker 使用"""
        import json

        package = json.loads((AI_DIR / "web" / "package.json").read_text(encoding='utf-8'))
        assert "build" in package["scripts"]
        assert "vite" in package["devDependencies"]

    def test_vite_proxies_api(self):
        """开发服务器需把 /api 代理到后端，否则本地联调失败"""
        vite_config = read_source(AI_DIR / "web" / "vite.config.ts")
        assert "/api" in vite_config
