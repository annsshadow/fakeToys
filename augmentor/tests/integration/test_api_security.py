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
import time
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
    因此「未设置环境变量 → 读 config.yaml」与「配置损坏/为空 → 回退出厂默认」
    两条路径必须显式解除注入才能覆盖。

    两条降级都落到 ``WebConfig.data_roots``，理由有两个：
      - 白名单一旦解析为空列表，**所有**请求都会 403，比放宽更危险；
      - 降级到工作目录会把出厂默认刚收紧掉的范围又悄悄放开，等于修复失效。
    """

    def test_env_var_takes_priority(self, monkeypatch, tmp_path):
        """环境变量优先于 config.yaml"""
        from api.deps import allowed_data_roots

        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        assert allowed_data_roots() == [tmp_path.resolve()]

    def test_falls_back_to_config_when_env_absent(self, monkeypatch):
        """未设置环境变量时读取 config.yaml 的 web.data_roots"""
        from api.deps import allowed_data_roots

        monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
        roots = allowed_data_roots()

        assert roots, "白名单不得为空，否则所有请求都会 403"
        assert all(isinstance(r, Path) and r.is_absolute() for r in roots)

    def test_empty_env_value_falls_back_to_shipped_default(self, monkeypatch):
        """环境变量仅含分隔符时落到出厂默认，而不是工作目录"""
        from api.deps import allowed_data_roots

        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", os.pathsep)

        assert allowed_data_roots() == [(Path.cwd() / "data").resolve()]

    def test_broken_config_falls_back_to_shipped_default(self, monkeypatch):
        """配置读取失败时落到出厂默认，而不是抛异常、留空或退回工作目录"""

        def _boom(*args, **kwargs):
            raise RuntimeError("配置损坏")

        from api import deps

        monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
        monkeypatch.setattr(deps, "load_config", _boom)

        assert deps.allowed_data_roots() == [(Path.cwd() / "data").resolve()]

    @pytest.mark.parametrize("bad", ["", "   "])
    def test_empty_path_rejected_with_400(self, bad):
        """空路径直接 400，不进入文件系统解析"""
        from fastapi import HTTPException

        from api.deps import resolve_within_roots

        with pytest.raises(HTTPException) as exc:
            resolve_within_roots(bad, "文件路径")
        assert exc.value.status_code == 400


class TestShippedDataRootsDefault:
    """出厂默认 ``web.data_roots`` 必须是 ``["data"]``

    默认值在三个地方各写了一遍：``WebConfig`` 的字段、``load_config`` 的
    ``config_sections`` 兜底字典、仓库根的 ``config.yaml``。任何一处漏改都会
    让「有配置文件」和「没有配置文件」两种部署的可见范围不一致，因此这里
    分别用字面量钉死——不能拿 ``WebConfig()`` 去校验另外两处，那是同源预言机。
    """

    def test_webconfig_field_default(self):
        """SDK 配置类的字段默认值"""
        from augmentor.config import WebConfig

        assert WebConfig().data_roots == ["data"]

    def test_defaults_apply_when_config_omits_web_section(self, tmp_path):
        """配置文件存在但缺 web 段时，兜底字典同样给出 data"""
        from augmentor.config import load_config

        quiet = tmp_path / "no_web.yaml"
        quiet.write_text("models:\n  default: ernie\n", encoding="utf-8")

        assert load_config(str(quiet)).web.data_roots == ["data"]

    def test_shipped_config_yaml(self):
        """随仓库分发的 config.yaml"""
        import yaml

        from augmentor.config import WebConfig

        with open(AI_DIR / "config.yaml", encoding="utf-8") as handle:
            raw = yaml.safe_load(handle)

        shipped = raw["web"]["data_roots"]
        assert shipped == ["data"]
        # 三处默认值必须一致，否则有无配置文件是两种安全边界
        assert shipped == WebConfig().data_roots

    def test_workdir_is_not_a_default_root(self, monkeypatch, tmp_path):
        """出厂默认不得把工作目录整体暴露给 API"""
        from fastapi import HTTPException

        from api.deps import resolve_data_path

        monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
        inside = tmp_path / "train_data_leaked.json"
        inside.write_text("[]", encoding="utf-8")

        with pytest.raises(HTTPException) as exc:
            resolve_data_path(str(inside))
        assert exc.value.status_code == 403


class TestDataListScansWhitelist:
    """"列表能给的，路由必须能读"

    ``/api/data/list`` 早期硬编码扫描 ``Path(".")``，与 ``resolve_within_roots``
    的白名单是两套来源：收紧 ``data_roots`` 后它会报出读不到的文件（点开的
    403 看起来像白名单没生效），或漏报白名单内其它根目录的文件。
    """

    def test_only_whitelisted_roots_are_listed(self, client, monkeypatch, tmp_path):
        """结果严格等于白名单根目录内的文件"""
        listed = tmp_path / "in"
        hidden = tmp_path / "out"
        listed.mkdir()
        hidden.mkdir()
        for root in (listed, hidden):
            (root / "train_data_visible.json").write_text("[]", encoding="utf-8")
        # 非 train_data 前缀即使在白名单内也不列
        (listed / "other.json").write_text("[]", encoding="utf-8")

        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(listed))

        files = client.get("/api/data/list").json()["files"]

        assert len(files) == 1
        assert files[0]["name"] == "train_data_visible.json"
        assert Path(files[0]["path"]).resolve() == (listed / "train_data_visible.json").resolve()

    def test_duplicated_roots_are_deduplicated(self, client, monkeypatch, tmp_path):
        """同一目录被重复列出时只报一次

        去重按 resolve 后的路径比较，因此 ``x`` 与 ``x/.`` 这类等价写法不会
        产生两条同文件记录。
        """
        root = tmp_path / "data"
        root.mkdir()
        target = root / "train_data_same.json"
        target.write_text("[]", encoding="utf-8")

        monkeypatch.setenv(
            "AUGMENTOR_DATA_ROOTS", os.pathsep.join([str(root), str(root / ".")])
        )

        files = client.get("/api/data/list").json()["files"]

        assert len(files) == 1
        assert Path(files[0]["path"]).resolve() == target.resolve()
        assert "size" in files[0]

    def test_missing_root_lists_nothing(self, client, monkeypatch, tmp_path):
        """白名单指向不存在的目录时返回空列表，而不是 500

        出厂默认收到 ``data`` 之后，「数据目录还没建」是全新部署的正常状态。
        """
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path / "not_created_yet"))

        response = client.get("/api/data/list")

        assert response.status_code == 200
        assert response.json()["files"] == []


class TestBareNameResolvesInsideRoots:
    """裸文件名要在白名单根目录内解析，前端才用得起来

    收紧出厂默认后最容易踩的坑：``/api/data/list`` 返回的 ``name`` 只是文件名，
    而前端 8 处（``DataList.tsx``、``Analysis.tsx``、``AugmentForm.tsx`` 等）都把它
    原样拼回 ``/api/data/load/{filename}``；``{filename}`` 只匹配单个路径段，
    传不了 ``data/xxx.json``。若相对路径只按工作目录解释，文件选择器会集体 403。
    """

    def test_listed_name_can_be_loaded_back(self, client, monkeypatch, tmp_path):
        """列表 → 选择 → 加载 的完整闭环必须走通（前端契约）"""
        root = tmp_path / "data"
        root.mkdir()
        items = [{"instruction": "怎么办居住证", "output": "带上材料"}]
        (root / "train_data_ui.json").write_text(
            json.dumps(items, ensure_ascii=False), encoding="utf-8"
        )
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))

        listed = client.get("/api/data/list").json()["files"]
        assert [f["name"] for f in listed] == ["train_data_ui.json"]

        response = client.get(f"/api/data/load/{listed[0]['name']}")

        assert response.status_code == 200
        assert response.json()["items"][0]["instruction"] == "怎么办居住证"

    def test_root_wins_over_working_directory(self, monkeypatch, tmp_path):
        """候选顺序：白名单根目录在前，工作目录兜底"""
        from api.deps import _relative_candidates, allowed_data_roots

        root = tmp_path / "data"
        root.mkdir()
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))

        candidates = _relative_candidates(Path("train_data_ui.json"), allowed_data_roots())

        assert candidates[0] == (root / "train_data_ui.json").resolve()
        assert candidates[-1] == (Path.cwd() / "train_data_ui.json").resolve()

    def test_candidates_are_deduplicated(self, monkeypatch):
        """重复根目录不产生重复候选

        旧部署把 ``data_roots`` 配成工作目录（收紧前的出厂默认）时，「根目录解释」
        与「工作目录兜底」完全重合；若不去重，每个路径参数都要多跑一次同样的
        ``exists()``，且 ``x`` 与 ``x/.`` 这类等价写法会被当成两个候选。
        """
        from api.deps import _relative_candidates, allowed_data_roots

        cwd = Path.cwd()
        monkeypatch.setenv(
            "AUGMENTOR_DATA_ROOTS", os.pathsep.join([str(cwd), str(cwd / ".")])
        )

        candidates = _relative_candidates(Path("train_data_ui.json"), allowed_data_roots())

        assert candidates == [(cwd / "train_data_ui.json").resolve()]

    def test_new_file_is_not_guessed_into_a_root(self, monkeypatch, tmp_path):
        """写入不存在的路径时**不**替调用方挑目录

        猜测会把产物静默挪进某个根目录；这里宁可 403，让调用方显式写
        ``data/xxx.json``。
        """
        from fastapi import HTTPException

        from api.deps import resolve_data_path

        root = tmp_path / "data"
        root.mkdir()
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))

        with pytest.raises(HTTPException) as exc:
            resolve_data_path("brand_new_output.json", for_write=True)

        assert exc.value.status_code == 403
        assert not (root / "brand_new_output.json").exists()

    def test_explicit_root_relative_spelling_works(self, monkeypatch, tmp_path):
        """显式 ``sub/xxx.json`` 这种写法照旧可用"""
        from api.deps import resolve_within_roots

        root = tmp_path / "data"
        (root / "sub").mkdir(parents=True)
        target = root / "sub" / "x.json"
        target.write_text("[]", encoding="utf-8")
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))

        assert resolve_within_roots("sub/x.json", "文件路径") == target.resolve()

    def test_absolute_path_still_needs_to_be_inside_roots(self, monkeypatch, tmp_path):
        """绝对路径不参与候选回退，越界仍 403"""
        from fastapi import HTTPException

        from api.deps import resolve_data_path

        outside = tmp_path / "outside.json"
        outside.write_text("[]", encoding="utf-8")
        root = tmp_path / "data"
        root.mkdir()
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))

        with pytest.raises(HTTPException) as exc:
            resolve_data_path(str(outside))

        assert exc.value.status_code == 403


class TestDependencyRegistryDefaultPath:
    """依赖端点在不传 ``registry_path`` 时也必须可用

    残留缺陷：三个 ``/api/system/dependency/*`` 端点的默认注册表目录曾写死为
    ``.dependency_registry``（相对工作目录）。白名单收到 ``data`` 之后工作目录不再
    在闸内，于是**默认参数自己**会被自己设置的闸拦下：实测收紧后 POST/GET
    ``/api/system/dependency/datasets`` 与 GET ``/api/system/dependency/graph``
    全部 403 —— 缺省调用一条都走不通。默认值必须跟着白名单走。
    """

    def _root(self, monkeypatch, tmp_path):
        """造一个只含白名单根目录的环境，工作目录刻意留在闸外"""
        root = tmp_path / "data"
        root.mkdir()
        _write_json(root / "demo.json", [{"instruction": "q", "output": "a"}])
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
        monkeypatch.chdir(tmp_path)
        return root

    def test_dependency_endpoints_work_with_default_registry_path(
        self, client, monkeypatch, tmp_path
    ):
        """登记 → 列表 → 依赖图 三段缺省调用都要走通，且产物落在白名单内"""
        root = self._root(monkeypatch, tmp_path)

        registered = client.post(
            "/api/system/dependency/datasets", json={"name": "demo", "input_file": "demo.json"}
        )
        listed = client.get("/api/system/dependency/datasets")
        graph = client.get("/api/system/dependency/graph")

        assert registered.status_code == 200
        assert listed.status_code == 200
        assert [d["name"] for d in listed.json()["datasets"]] == ["demo"]
        assert graph.status_code == 200
        assert (root / ".dependency_registry").is_dir()
        assert not (tmp_path / ".dependency_registry").exists()

    def test_explicit_registry_path_outside_roots_is_still_rejected(
        self, client, monkeypatch, tmp_path
    ):
        """显式传参不参与缺省兜底，越界仍然 403

        否则「默认值跟白名单走」会被一个可选参数绕开，白名单形同虚设。
        """
        self._root(monkeypatch, tmp_path)

        response = client.get(
            "/api/system/dependency/datasets",
            params={"registry_path": str(tmp_path / "outside_registry")},
        )

        assert response.status_code == 403
        assert not (tmp_path / "outside_registry").exists()

    def test_empty_explicit_registry_path_is_not_silently_defaulted(
        self, client, monkeypatch, tmp_path
    ):
        """显式传空串是非法入参（400），不能因为「有默认值」就被吞掉"""
        self._root(monkeypatch, tmp_path)

        response = client.get(
            "/api/system/dependency/datasets", params={"registry_path": ""}
        )

        assert response.status_code == 400


class TestBackupDirDefaultPath:
    """备份端点在不传 ``backup_dir`` 时也必须可用（F-10，与 F-09 同构）

    四条 ``/api/system/backups*`` 端点把默认目录写成相对工作目录的 ``.backups``；
    白名单收到 ``data`` 之后工作目录不在闸内，实测不传参的 ``GET /api/system/backups``
    与 ``POST`` 全部 **403**。同一份默认值还让备份测试把 ``.backups/`` 写进版本树。
    """

    def _root(self, monkeypatch, tmp_path):
        root = tmp_path / "data"
        root.mkdir()
        _write_json(root / "demo.json", [{"instruction": "怎么办居住证", "output": "带上材料"}])
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
        monkeypatch.chdir(tmp_path)
        return root

    def test_backup_lifecycle_works_with_default_backup_dir(
        self, client, monkeypatch, tmp_path
    ):
        """创建 → 列表 → 恢复 → 删除，全程不传 ``backup_dir``"""
        root = self._root(monkeypatch, tmp_path)

        created = client.post(
            "/api/system/backups", json={"input_file": "demo.json", "name": "snap"}
        )
        listed = client.get("/api/system/backups")
        restored = client.post(
            "/api/system/backups/snap/restore",
            json={"output_file": "data/restored.json"},
        )
        deleted = client.delete("/api/system/backups/snap")

        assert created.status_code == 200, created.text
        assert created.json()["backup_id"] == "snap"
        assert created.json()["item_count"] == 1
        assert [b["backup_id"] for b in listed.json()["backups"]] == ["snap"]
        assert restored.status_code == 200, restored.text
        assert (root / "restored.json").is_file()
        assert deleted.status_code == 200
        # 产物一律落在白名单内，工作目录不留痕迹
        assert (root / ".backups").is_dir()
        assert not (tmp_path / ".backups").exists()

    def test_explicit_backup_dir_outside_roots_is_rejected(
        self, client, monkeypatch, tmp_path
    ):
        """显式传参不参与缺省兜底：越界仍 403"""
        self._root(monkeypatch, tmp_path)

        response = client.get(
            "/api/system/backups", params={"backup_dir": str(tmp_path / "outside")}
        )

        assert response.status_code == 403

    def test_empty_explicit_backup_dir_is_400(self, client, monkeypatch, tmp_path):
        """空串是非法入参，不能被静默换成缺省目录"""
        self._root(monkeypatch, tmp_path)

        response = client.get("/api/system/backups", params={"backup_dir": ""})

        assert response.status_code == 400


class TestServiceConfigPath:
    """服务自身的配置文件不过数据白名单（F-11）

    ``POST /api/system/validate-config`` 的默认值曾写死成 ``config.yaml``（相对工作
    目录），于是**不传参就是 403**；而配置文件本就不是数据集——它是服务端要读写的
    东西。收口方式：默认校验 ``deps.config_file_path()``（可用
    ``AUGMENTOR_CONFIG_PATH`` 指向别处），客户端**显式**给的路径仍按数据白名单校验。
    """

    def _env(self, monkeypatch, tmp_path):
        root = tmp_path / "data"
        root.mkdir()
        service_cfg = tmp_path / "service.yaml"
        service_cfg.write_text("models:\n  default: fallback\n", encoding="utf-8")
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
        monkeypatch.setenv("AUGMENTOR_CONFIG_PATH", str(service_cfg))
        monkeypatch.chdir(tmp_path)
        return root

    def test_default_validates_the_service_own_config(self, client, monkeypatch, tmp_path):
        """缺省体 ``{}`` → 校验服务自己在用的那份配置，且它可以在白名单外"""
        self._env(monkeypatch, tmp_path)

        response = client.post("/api/system/validate-config", json={})

        assert response.status_code == 200, response.text
        assert response.json()["is_valid"] is True

    def test_config_path_falls_back_to_cwd_and_explicit_stays_gated(
        self, client, monkeypatch, tmp_path
    ):
        """无 ``AUGMENTOR_CONFIG_PATH`` 时回到工作目录的 ``config.yaml``；
        但客户端**显式**传这个绝对路径仍然 403 —— 闸门只对服务端自己放行。
        """
        from api.deps import config_file_path

        self._env(monkeypatch, tmp_path)
        monkeypatch.delenv("AUGMENTOR_CONFIG_PATH")

        assert config_file_path() == (tmp_path / "config.yaml").resolve()

        response = client.post(
            "/api/system/validate-config", json={"path": str(AI_DIR / "config.yaml")}
        )

        assert response.status_code == 403
        assert "超出允许" in response.json()["detail"]

    def test_client_supplied_config_path_is_still_gated(
        self, client, monkeypatch, tmp_path
    ):
        """显式传入的配置路径仍在白名单内时照常可用"""
        root = self._env(monkeypatch, tmp_path)
        inside = root / "extra.yaml"
        inside.write_text("models:\n  default: fallback\n", encoding="utf-8")

        response = client.post(
            "/api/system/validate-config", json={"path": "extra.yaml"}
        )

        assert response.status_code == 200, response.text
        assert response.json()["is_valid"] is True


class TestConfigDataRootsCache:
    """配置文件来源的白名单不能每个请求重解析一遍

    实测：未设置 ``AUGMENTOR_DATA_ROOTS`` 时 ``allowed_data_roots()`` 每次
    **7.06 ms**，全部花在重新解析 ``config.yaml`` 上；而任何带路径参数的请求都
    至少要过一次白名单，等于给每个请求凭空加 7 ms。缓存按 ``(路径, mtime_ns)``
    建键——「改完配置立刻生效」这条不能为了省时间牺牲掉。
    """

    def _cfg(self, tmp_path, root_dir: Path):
        """写一份只声明 ``web.data_roots`` 的配置"""
        (tmp_path / "config.yaml").write_text(
            f"web:\n  data_roots:\n    - {root_dir.as_posix()}\n", encoding="utf-8"
        )

    def _instrument(self, monkeypatch, tmp_path):
        """清缓存、撤掉环境变量、返回 ``load_config`` 的调用记录"""
        import api.deps as deps

        calls: list = []
        real = deps.load_config

        def counting(config_path=None):
            calls.append(config_path)
            return real(config_path)

        monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
        monkeypatch.setattr(deps, "load_config", counting)
        monkeypatch.setattr(deps, "_config_roots_cache", {})
        monkeypatch.chdir(tmp_path)
        return deps, calls

    def test_config_parsed_once_across_calls(self, monkeypatch, tmp_path):
        """连查五次白名单，配置文件只解析一次"""
        allowed = tmp_path / "allowed_a"
        allowed.mkdir()
        self._cfg(tmp_path, allowed)
        deps, calls = self._instrument(monkeypatch, tmp_path)

        roots = [deps.allowed_data_roots() for _ in range(5)][-1]

        assert roots == [allowed.resolve()]
        assert len(calls) == 1

    def test_config_edit_takes_effect_immediately(self, monkeypatch, tmp_path):
        """换根目录后必须立刻生效——缓存不能把白名单冻在旧值上"""
        first = tmp_path / "allowed_a"
        second = tmp_path / "allowed_b"
        first.mkdir()
        second.mkdir()
        self._cfg(tmp_path, first)
        deps, calls = self._instrument(monkeypatch, tmp_path)

        assert deps.allowed_data_roots() == [first.resolve()]

        self._cfg(tmp_path, second)
        cfg = tmp_path / "config.yaml"
        stat = cfg.stat()
        os.utime(cfg, ns=(stat.st_atime_ns, stat.st_mtime_ns + 1_000_000))

        assert deps.allowed_data_roots() == [second.resolve()]
        assert len(calls) == 2

    def test_broken_config_is_not_cached(self, monkeypatch, tmp_path):
        """坏配置降级到出厂默认，且**不**进缓存（修好后下一次就该读到）"""
        (tmp_path / "config.yaml").write_text(
            "web:\n  data_roots: [unclosed\n", encoding="utf-8"
        )
        deps, calls = self._instrument(monkeypatch, tmp_path)

        first = deps.allowed_data_roots()
        second = deps.allowed_data_roots()

        assert first == second == [(Path.cwd() / "data").resolve()]
        assert len(calls) == 2

    def test_whitelist_lookup_stays_sub_millisecond(self, monkeypatch, tmp_path):
        """200 次白名单解析的总耗时预算（未加缓存时实测约 1400 ms）

        时间断言本身不是预言机——所以同时钉住「200 次查询只解析过 1 次配置」，
        否则缓存失效时只剩一个可能因机器抖动而误判的数字。
        """
        allowed = tmp_path / "allowed_a"
        allowed.mkdir()
        self._cfg(tmp_path, allowed)
        deps, calls = self._instrument(monkeypatch, tmp_path)

        deps.allowed_data_roots()
        start = time.perf_counter()
        for _ in range(200):
            deps.allowed_data_roots()
        elapsed_ms = (time.perf_counter() - start) * 1000

        assert len(calls) == 1, f"200 次查询解析了 {len(calls)} 次配置，缓存未生效"
        assert elapsed_ms < 200, f"白名单解析 200 次用了 {elapsed_ms:.1f} ms（缓存未生效？）"

    def test_returned_roots_are_a_fresh_list(self, monkeypatch, tmp_path):
        """缓存命中时也要返回新列表——调用方 append 不能污染白名单"""
        allowed = tmp_path / "allowed_a"
        allowed.mkdir()
        self._cfg(tmp_path, allowed)
        deps, _ = self._instrument(monkeypatch, tmp_path)

        first = deps.allowed_data_roots()
        first.append(Path(tmp_path.as_posix()))
        second = deps.allowed_data_roots()

        assert second == [allowed.resolve()]


class TestEnvDataRootsCache:
    """环境变量来源的白名单也要缓存，但缓存键必须带上工作目录

    实测：``AUGMENTOR_DATA_ROOTS`` 设两个根目录时 ``allowed_data_roots()``
    每次 **0.29 ms**，全花在 ``Path.resolve()`` 的系统调用上；测试环境恒设该
    变量，等于整个套件每个请求都重复解析同一串值。

    缓存按 ``(原值, 工作目录)`` 建键：白名单允许相对路径，只按原值建键会把
    上一个目录的解释带到新目录下——那是**放宽**边界，比慢更糟。
    """

    def _roots(self, monkeypatch, value: str):
        import api.deps as deps

        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", value)
        monkeypatch.setattr(deps, "_env_roots_cache", {})
        return deps

    def test_relative_env_root_follows_cwd(self, monkeypatch, tmp_path):
        """换工作目录后，同一个相对根目录必须重新解析"""
        first = tmp_path / "first"
        second = tmp_path / "second"
        first.mkdir()
        second.mkdir()

        deps = self._roots(monkeypatch, "data")

        monkeypatch.chdir(first)
        assert deps.allowed_data_roots() == [(first / "data").resolve()]

        monkeypatch.chdir(second)
        assert deps.allowed_data_roots() == [(second / "data").resolve()]

    def test_changed_env_value_takes_effect(self, monkeypatch, tmp_path):
        """改环境变量值必须立刻生效，不能读到上一个值的缓存"""
        deps = self._roots(monkeypatch, str(tmp_path / "a"))
        assert deps.allowed_data_roots() == [(tmp_path / "a").resolve()]

        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path / "b"))
        assert deps.allowed_data_roots() == [(tmp_path / "b").resolve()]

    def test_blank_env_value_falls_back_to_shipped_default(self, monkeypatch, tmp_path):
        """环境变量全是分隔符/空白时按「未设置」处理，降级到出厂默认"""
        deps = self._roots(monkeypatch, os.pathsep + "   " + os.pathsep)

        roots = deps.allowed_data_roots()

        assert roots == [(Path.cwd() / "data").resolve()]

    def test_env_lookup_budget(self, monkeypatch, tmp_path):
        """500 次白名单解析的总耗时预算（未缓存时实测约 145 ms，其中 resolve 占九成）"""
        deps = self._roots(
            monkeypatch, os.pathsep.join([str(tmp_path / "a"), str(tmp_path / "b")])
        )

        deps.allowed_data_roots()
        start = time.perf_counter()
        for _ in range(500):
            deps.allowed_data_roots()
        elapsed_ms = (time.perf_counter() - start) * 1000

        assert elapsed_ms < 50, f"环境变量白名单解析 500 次用了 {elapsed_ms:.1f} ms"


class TestWriteRouteAuthCoverage:
    """写路由鉴权覆盖率（元测试）

    残留风险：`verify_api_key` 是**逐个路由手动挂载**的，新增一条写路由时很容易漏挂，
    而漏挂不会让任何既有测试变红——鉴权静默失效。

    因此这里把「全部路由的鉴权状态」固化下来（条数不写死，以两份集合为准）：
    新增或删除路由都会让本测试失败，迫使改动者显式做出分类——要么挂
    `Depends(verify_api_key)`，要么把它加进 `OPEN_ALLOWLIST` 并说明为什么只读。

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
        ("POST", "/api/dataset/evaluate"),
        ("POST", "/api/dataset/features"),
        ("POST", "/api/dataset/impact"),
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