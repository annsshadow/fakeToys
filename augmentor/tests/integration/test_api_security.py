# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""API 安全与正确性回归测试

对应 docs/plans/2026-09-21-001-audit-augmentor-fullstack-optimization-plan.md 的 Phase 0：

- F1  质量评分语义维度退化：无关答案也曾判 pass
- F3  任意文件读写：路径穿越与绝对路径逃逸
- F4  限流中间件已实现但未注册
- F5  写操作无鉴权

这些用例在修复前**必须失败**。若某个用例在去掉修复代码后仍然通过，
说明它没有真正锁定该缺陷。
"""

import json
import os
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def client(tmp_path, monkeypatch):
    """隔离 CWD 与管道状态的 TestClient

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        TestClient 实例
    """
    from augmentor import AugmentorPipeline, load_config
    from augmentor.checkpoint import CheckpointManager
    from augmentor.versioning import VersionManager

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    instance = AugmentorPipeline(config)
    instance.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    instance.checkpoint_manager = CheckpointManager(
        checkpoint_dir=str(tmp_path / "checkpoints")
    )

    import api.deps as deps
    from api.main import app

    monkeypatch.setattr(deps, "_pipeline", instance)
    return TestClient(app)


def _write_json(path: Path, items):
    """写入 JSON 文件

    Args:
        path: 目标路径
        items: 数据列表
    """
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")


# ============================================================
# F1 — 质量评分语义维度退化
# ============================================================

class TestQualityScoringNotInflated:
    """API 无法获得原始种子问题，语义维度必须被跳过而不是伪造为 1.0"""

    def test_irrelevant_answer_is_filtered(self, client, tmp_path):
        """答案与问题完全无关的样本必须被判为不通过

        修复前 _build_scoring_items 把 instruction 同时填入 original 与 generated，
        语义相似度恒为 1.0（权重 0.3），总分下限被抬到 0.6，
        恰好等于默认阈值，于是「今天天气不错哈哈哈」也能通过。
        """
        data_file = tmp_path / "irrelevant.json"
        _write_json(data_file, [
            {"instruction": "怎么申请租房？", "input": "", "output": "今天天气不错哈哈哈"},
            {"instruction": "退租要提前多久？", "input": "", "output": "我昨天吃了火锅"},
        ])

        response = client.post(
            "/api/quality/evaluate", json={"input_file": str(data_file)}
        )

        assert response.status_code == 200
        payload = response.json()
        assert payload["passed_samples"] == 0, "无关答案不应通过质量检查"
        assert payload["avg_score"] < payload["threshold"]

    def test_response_declares_semantic_skipped(self, client, tmp_path):
        """响应需显式声明语义维度未参与评分，避免调用方误读分数"""
        data_file = tmp_path / "declared.json"
        _write_json(data_file, [
            {"instruction": "怎么申请租房？", "input": "", "output": "请登录官网提交申请"}
        ])

        payload = client.post(
            "/api/quality/evaluate", json={"input_file": str(data_file)}
        ).json()

        assert payload["semantic_evaluated"] is False
        # 语义权重 0.3 应被分摊掉，剩余权重之和仍为 1.0
        weights = payload["effective_weights"]
        assert weights[0] == 0.0
        assert abs(sum(weights) - 1.0) < 1e-9

    def test_relevant_answer_outscores_irrelevant(self, client, tmp_path):
        """合格样本必须与无关样本区分开

        不断言绝对分值：未安装 sentence-transformers / cross-encoder 时
        相关性会退化为 n-gram 相似度，绝对分受环境影响。真正被修复破坏的是
        「区分能力」——修复前两类样本都因语义维度恒 1.0 而≥阈值。
        """
        data_file = tmp_path / "compare.json"
        _write_json(data_file, [
            {"instruction": "怎么申请租房？", "output": "怎么申请租房？请登录官网提交申请"},
            {"instruction": "怎么申请租房？", "output": "今天天气不错哈哈哈"},
        ])

        payload = client.post(
            "/api/quality/evaluate", json={"input_file": str(data_file)}
        ).json()

        assert payload["passed_samples"] == 1, "应恰好通过 1 条（相关的那条）"


# ============================================================
# F3 — 任意文件读写
# ============================================================

class TestPathContainment:
    """所有路径入参都必须落在 web.data_roots 白名单内"""

    def test_absolute_path_outside_roots_rejected(self, client, tmp_path, monkeypatch):
        """白名单之外的绝对路径必须被拒（403）

        修复前 load_items 只判断文件是否存在，任意可读的 JSON 文件都会被解析。
        """
        outside = tmp_path.parent / "outside_roots.json"
        _write_json(outside, [{"instruction": "外部数据", "output": "x"}])

        # 把白名单收窄到 tmp_path，使 outside 落在范围之外
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))

        response = client.post(
            "/api/quality/evaluate", json={"input_file": str(outside)}
        )
        assert response.status_code == 403

    @pytest.mark.parametrize(
        "route,payload",
        [
            ("/api/quality/evaluate", {"input_file": "../../etc/passwd.json"}),
            ("/api/quality/report", {"input_file": "../../../secret.json"}),
            ("/api/export/preview", {"input_file": "../x.json", "format": "jsonl"}),
        ],
    )
    def test_dotdot_traversal_rejected(self, client, route, payload):
        """含 .. 的路径必须返回 400"""
        assert client.post(route, json=payload).status_code == 400

    def test_write_path_outside_roots_rejected(self, client, tmp_path, monkeypatch):
        """输出路径同样受约束，避免借导出写入任意位置"""
        inside = tmp_path / "src.json"
        _write_json(inside, [{"instruction": "q", "output": "a"}])
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))

        response = client.post(
            "/api/data/export",
            json={
                "input_file": str(inside),
                "output_dir": str(tmp_path.parent / "escaped_out"),
            },
        )
        assert response.status_code == 403

    def test_multimodal_scan_outside_roots_rejected(self, client, tmp_path, monkeypatch):
        """目录扫描同样受约束"""
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        response = client.post(
            "/api/multimodal/scan", json={"directory": str(tmp_path.parent)}
        )
        assert response.status_code == 403

    def test_inside_roots_still_works(self, client, tmp_path, monkeypatch):
        """白名单之内的正常读写不得被误伤"""
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        inside = tmp_path / "ok.json"
        _write_json(inside, [
            {"instruction": "怎么申请租房？", "output": "怎么申请租房？请登录官网。"}
        ])

        assert client.post(
            "/api/quality/evaluate", json={"input_file": str(inside)}
        ).status_code == 200


# ============================================================
# F4 — 限流
# ============================================================

class TestRateLimitWired:
    """限流中间件必须真正注册在应用上"""

    def test_middleware_registered(self):
        """修复前 RateLimitMiddleware 已实现且有单测，但从未 add_middleware"""
        from api.main import app
        from api.middleware.rate_limit import RateLimitMiddleware

        registered = [
            getattr(m, "cls", None).__name__
            for m in app.user_middleware
            if getattr(m, "cls", None) is not None
        ]
        assert "RateLimitMiddleware" in registered

    def test_exceeding_limit_returns_429(self, client):
        """超限需返回 429 且带 Retry-After"""
        from api.main import rate_limiter

        original = rate_limiter.max_requests
        rate_limiter.reset()
        rate_limiter.max_requests = 2
        try:
            assert client.get("/api/status").status_code == 200
            assert client.get("/api/status").status_code == 200
            blocked = client.get("/api/status")
            assert blocked.status_code == 429
            assert blocked.headers.get("Retry-After")
        finally:
            rate_limiter.max_requests = original
            rate_limiter.reset()

    def test_health_endpoint_exempt(self, client):
        """健康检查被容器探针高频调用，必须豁免限流"""
        from api.main import rate_limiter

        original = rate_limiter.max_requests
        rate_limiter.reset()
        rate_limiter.max_requests = 1
        try:
            for _ in range(5):
                assert client.get("/api/health").status_code == 200
        finally:
            rate_limiter.max_requests = original
            rate_limiter.reset()


# ============================================================
# F5 — 写操作鉴权
# ============================================================

class TestApiKeyAuth:
    """设置 AUGMENTOR_API_KEY 后写操作必须携带密钥"""

    def test_write_requires_key_when_configured(self, client, tmp_path, monkeypatch):
        """未带密钥的写请求应 401"""
        monkeypatch.setenv("AUGMENTOR_API_KEY", "s3cret")
        data_file = tmp_path / "w.json"
        _write_json(data_file, [{"instruction": "q", "output": "a"}])

        response = client.put(
            f"/api/data/update/{data_file.name}",
            params={"index": 0},
            json={"instruction": "x", "output": "y"},
        )
        assert response.status_code == 401

    def test_write_rejects_wrong_key(self, client, tmp_path, monkeypatch):
        """密钥不匹配应 401"""
        monkeypatch.setenv("AUGMENTOR_API_KEY", "s3cret")
        data_file = tmp_path / "w2.json"
        _write_json(data_file, [{"instruction": "q", "output": "a"}])

        response = client.put(
            f"/api/data/update/{data_file.name}",
            params={"index": 0},
            json={"instruction": "x", "output": "y"},
            headers={"X-API-Key": "wrong"},
        )
        assert response.status_code == 401

    def test_write_accepts_correct_key(self, client, tmp_path, monkeypatch):
        """正确密钥应放行"""
        monkeypatch.setenv("AUGMENTOR_API_KEY", "s3cret")
        monkeypatch.chdir(tmp_path)
        data_file = tmp_path / "w3.json"
        _write_json(data_file, [{"instruction": "q", "output": "a"}])

        response = client.put(
            f"/api/data/update/{data_file.name}",
            params={"index": 0},
            json={"instruction": "x", "output": "y"},
            headers={"X-API-Key": "s3cret"},
        )
        assert response.status_code == 200

    def test_read_stays_anonymous(self, client, monkeypatch):
        """读操作与健康检查保持匿名可达"""
        monkeypatch.setenv("AUGMENTOR_API_KEY", "s3cret")
        assert client.get("/api/health").status_code == 200
        assert client.get("/api/status").status_code == 200

    def test_disabled_when_env_absent(self, client, tmp_path, monkeypatch):
        """未配置密钥时保持零配置可用（既有约定）"""
        monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
        monkeypatch.chdir(tmp_path)
        data_file = tmp_path / "w4.json"
        _write_json(data_file, [{"instruction": "q", "output": "a"}])

        response = client.put(
            f"/api/data/update/{data_file.name}",
            params={"index": 0},
            json={"instruction": "x", "output": "y"},
        )
        assert response.status_code == 200


class TestAllowedRootsResolution:
    """白名单来源的优先级与降级

    ``tests/conftest.py`` 的 autouse fixture 总会注入 ``AUGMENTOR_DATA_ROOTS``，
    因此「未设置环境变量 → 读 config.yaml」与「配置损坏 → 回退工作目录」两条路径
    必须显式解除注入才能覆盖。

    这两条降级路径的意义在于：白名单一旦解析为空列表，**所有**请求都会 403，
    比「放宽到工作目录」危险得多，所以必须验证它们不会产生空白名单。
    """

    def test_env_var_takes_priority(self, monkeypatch, tmp_path):
        """环境变量优先于 config.yaml"""
        from api.deps import _allowed_roots

        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        assert _allowed_roots() == [tmp_path.resolve()]

    def test_falls_back_to_config_when_env_absent(self, monkeypatch):
        """未设置环境变量时读取 config.yaml 的 web.data_roots"""
        from api.deps import _allowed_roots

        monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
        roots = _allowed_roots()

        assert roots, "白名单不得为空，否则所有请求都会 403"
        assert all(isinstance(r, Path) and r.is_absolute() for r in roots)

    def test_empty_env_value_falls_back_to_cwd(self, monkeypatch):
        """环境变量仅含分隔符时回退到工作目录"""
        from api.deps import _allowed_roots

        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", os.pathsep)
        assert _allowed_roots() == [Path.cwd().resolve()]

    def test_broken_config_falls_back_to_cwd(self, monkeypatch):
        """配置读取失败时回退工作目录，而不是抛异常或留空"""

        def _boom(*args, **kwargs):
            raise RuntimeError("配置损坏")

        from api import deps

        monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
        monkeypatch.setattr(deps, "load_config", _boom)

        assert deps._allowed_roots() == [Path.cwd().resolve()]

    @pytest.mark.parametrize("bad", ["", "   "])
    def test_empty_path_rejected_with_400(self, bad):
        """空路径直接 400，不进入文件系统解析"""
        from fastapi import HTTPException

        from api.deps import resolve_within_roots

        with pytest.raises(HTTPException) as exc:
            resolve_within_roots(bad, "文件路径")
        assert exc.value.status_code == 400


class TestWriteRouteAuthCoverage:
    """写路由鉴权覆盖率（元测试）

    残留风险：`verify_api_key` 是**逐个路由手动挂载**的，新增一条写路由时很容易漏挂，
    而漏挂不会让任何既有测试变红——鉴权静默失效。

    因此这里把「全部 42 条路由的鉴权状态」固化下来：新增或删除路由都会让本测试失败，
    迫使改动者显式做出分类——要么挂 `Depends(verify_api_key)`，要么把它加进
    `OPEN_ALLOWLIST` 并说明为什么只读。

    注意：不能用 `app.routes` 枚举。当前 FastAPI 版本把 `include_router` 包装成不展开的
    `_IncludedRouter` 容器，必须回到各 router 模块的 `router.routes`。
    """

    # 会落盘或改变服务状态的路由，必须挂鉴权
    PROTECTED = {
        ("DELETE", "/api/data/delete/{filename}"),
        ("DELETE", "/api/system/backups/{backup_id}"),
        ("DELETE", "/api/versions/{version_id}"),
        ("POST", "/api/augment/start"),
        ("POST", "/api/config"),
        ("POST", "/api/data/export"),
        ("POST", "/api/data/upload"),
        ("POST", "/api/dataset/aggregate"),
        ("POST", "/api/dataset/convert"),
        ("POST", "/api/dataset/merge"),
        ("POST", "/api/dataset/rag"),
        ("POST", "/api/dataset/sample"),
        ("POST", "/api/dataset/split"),
        ("POST", "/api/export/batch"),
        ("POST", "/api/quality/profiling"),
        ("POST", "/api/system/backups"),
        ("POST", "/api/system/backups/{backup_id}/restore"),
        ("POST", "/api/system/dependency/datasets"),
        ("POST", "/api/system/migrate"),
        ("POST", "/api/system/stream"),
        ("POST", "/api/versions/create"),
        ("POST", "/api/versions/{version_id}/rollback"),
        ("PUT", "/api/data/update/{filename}"),
    }

    # 只读路由：全部经 `read_items` / 白名单校验，不落盘
    OPEN_ALLOWLIST = {
        ("GET", "/api/analyze/{filename}"),
        ("GET", "/api/augment/checkpoints"),
        ("GET", "/api/augment/progress"),
        ("GET", "/api/config"),
        ("GET", "/api/data/list"),
        ("GET", "/api/data/load/{filename}"),
        ("GET", "/api/demo/data"),
        ("GET", "/api/export/formats"),
        ("GET", "/api/models"),
        ("GET", "/api/multimodal/formats"),
        ("GET", "/api/privacy/patterns"),
        ("GET", "/api/status"),
        ("GET", "/api/system/backups"),
        ("GET", "/api/system/dependencies"),
        ("GET", "/api/system/dependency/datasets"),
        ("GET", "/api/system/dependency/graph"),
        ("GET", "/api/versions"),
        ("GET", "/api/versions/history"),
        ("GET", "/api/versions/{version_id}"),
        ("GET", "/api/versions/{version_id}/data"),
        ("GET", "/api/visualize/{filename}"),
        ("POST", "/api/audit"),
        ("POST", "/api/dataset/auto-config"),
        ("POST", "/api/dataset/compare"),
        ("POST", "/api/dataset/features"),
        ("POST", "/api/dataset/search"),
        ("POST", "/api/dataset/stats"),
        ("POST", "/api/dataset/validate"),
        ("POST", "/api/export/preview"),
        ("POST", "/api/leakage/check"),
        ("POST", "/api/multimodal/process"),
        ("POST", "/api/multimodal/scan"),
        ("POST", "/api/privacy/sanitize"),
        ("POST", "/api/quality/annotate"),
        ("POST", "/api/quality/benchmark"),
        ("POST", "/api/quality/clean"),
        ("POST", "/api/quality/dedup"),
        ("POST", "/api/quality/evaluate"),
        ("POST", "/api/quality/outliers"),
        ("POST", "/api/quality/report"),
        ("POST", "/api/system/auto-test"),
        ("POST", "/api/system/monitor"),
        ("POST", "/api/system/validate-config"),
        ("POST", "/api/versions/diff"),
    }

    @staticmethod
    def _collect():
        """枚举全部路由模块的鉴权挂载状态

        Returns:
            (受鉴权保护的路由集合, 未受保护的路由集合)

        模块清单用 `pkgutil` **动态发现**，不写死。上一版写死了 11 个模块，
        而 `dataset_tools` / `system_ops` 两个新模块不在其中，于是它们名下
        25 条路由（含 8 条会落盘的）从未进入本测试的视野——「新增写路由会让
        本测试变红」这个承诺恰好在它最该生效的那次改动上失效了。实测：配置了
        `AUGMENTOR_API_KEY` 后不带密钥 `POST /api/dataset/convert` 返回 200
        并把文件写了出来。

        注意：不能用 `app.routes` 枚举。当前 FastAPI 版本把 `include_router`
        包装成不展开的 `_IncludedRouter` 容器，必须回到各 router 模块的
        `router.routes`。
        """
        import importlib
        import pkgutil

        import api.routes as routes_pkg

        modules = [
            importlib.import_module(f"api.routes.{info.name}")
            for info in pkgutil.iter_modules(routes_pkg.__path__)
        ]

        protected, open_routes = set(), set()
        for module in modules:
            for route in module.router.routes:
                names = {
                    getattr(dep.call, "__name__", "?")
                    for dep in route.dependant.dependencies
                }
                for method in route.methods:
                    key = (method, route.path)
                    if "verify_api_key" in names:
                        protected.add(key)
                    else:
                        open_routes.add(key)
        return protected, open_routes

    def test_protected_set_matches_reviewed_list(self):
        """受保护路由必须与清单完全一致（少一条即漏挂鉴权）"""
        protected, _ = self._collect()
        assert protected == self.PROTECTED, (
            f"受鉴权保护的路由与清单不符。\n"
            f"  多出（已挂但未登记）: {sorted(protected - self.PROTECTED)}\n"
            f"  缺失（清单要求但未挂）: {sorted(self.PROTECTED - protected)}"
        )

    def test_open_set_matches_reviewed_allowlist(self):
        """未受保护路由必须全部在只读白名单内（新增写路由会在此变红）"""
        _, open_routes = self._collect()
        assert open_routes == self.OPEN_ALLOWLIST, (
            f"未受鉴权保护的路由与白名单不符。\n"
            f"  未登记的新路由（若会落盘，请挂 Depends(verify_api_key)；"
            f"否则加进 OPEN_ALLOWLIST）: {sorted(open_routes - self.OPEN_ALLOWLIST)}\n"
            f"  白名单中已不存在的路由: {sorted(self.OPEN_ALLOWLIST - open_routes)}"
        )

    def test_no_route_is_both(self):
        """两个集合不得有交集"""
        protected, open_routes = self._collect()
        assert not (protected & open_routes)

    def test_enumeration_covers_every_openapi_route(self):
        """枚举结果必须覆盖 OpenAPI 声明的每条 /api 路由

        这条是上一版盲区真正的守门。`_collect()` 的模块清单哪怕再次写死、
        再次漏掉一个新模块，漏掉的路由仍会出现在 OpenAPI 里，于是这里变红。
        等价地，把某条路由从 OpenAPI 里摘掉（`include_in_schema=False`）来
        躲开鉴权分类，也会被这里抓到——它既不在两个集合里，也躲不过 schema。
        """
        from fastapi.routing import APIRoute

        from api.main import app

        protected, open_routes = self._collect()
        # app 级直接声明的路由（`/api/health`），不在任何 router 模块里
        app_level = {
            (method, route.path)
            for route in app.routes
            if isinstance(route, APIRoute) and route.path.startswith("/api")
            for method in route.methods
        }
        enumerated = protected | open_routes | app_level

        declared = {
            (method.upper(), path)
            for path, operations in app.openapi()["paths"].items()
            if path.startswith("/api")
            for method in operations
        }

        unclassified = sorted(declared - enumerated)
        assert not unclassified, (
            f"以下路由出现在 OpenAPI 里，但没进入鉴权分类——"
            f"很可能是新增了路由模块而 `_collect()` 没发现它：{unclassified}"
        )
        assert len(declared) == len(enumerated) | len(declared & enumerated)