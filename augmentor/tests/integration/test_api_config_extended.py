# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""config API 路由扩展测试

覆盖 POST /api/config 的更新/持久化/未知键守卫，以及
get_pipeline 异常时各 GET/POST 的 500 降级。
"""

import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def pipeline(tmp_path, monkeypatch):
    """构造使用临时目录的管道实例（同 test_api.py）

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Yields:
        AugmentorPipeline 实例
    """
    from augmentor import load_config

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    instance = type("PipelineStub", (), {})()
    instance.config = config
    monkeypatch.chdir(tmp_path)  # save_config 写 CWD 的 config.yaml，隔离到临时目录
    import api.deps as deps
    monkeypatch.setattr(deps, "_pipeline", instance)
    yield instance


def _client(monkeypatch):
    from fastapi.testclient import TestClient

    from api.main import app
    return TestClient(app)


class TestUpdateConfig:
    def test_update_default_model_and_sections(self, pipeline, monkeypatch):
        """POST 需更新 default_model 与分区字段并返回成功"""
        client = _client(monkeypatch)
        response = client.post(
            "/api/config",
            json={
                "default_model": "openai",
                "quality": {"threshold": 0.8},
                "augmentation": {"variants_per_seed": 3},
            },
        )
        assert response.status_code == 200
        assert response.json()["success"] is True
        assert pipeline.config.default_model == "openai"
        assert pipeline.config.quality.threshold == 0.8
        assert pipeline.config.augmentation.variants_per_seed == 3

    def test_unknown_section_key_is_ignored(self, pipeline, monkeypatch):
        """分区内未知键需被 hasattr 守卫丢弃而非崩溃"""
        client = _client(monkeypatch)
        response = client.post(
            "/api/config",
            json={"quality": {"no_such_key": 123}},
        )
        assert response.status_code == 200
        assert pipeline.config.quality.threshold == 0.6

    def test_unknown_section_key_named_in_message(self, pipeline, monkeypatch):
        """丢弃未知子键时 message 与 ignored_keys 都要点名是哪个键（A86：写了没生效要出声）"""
        client = _client(monkeypatch)
        response = client.post("/api/config", json={"quality": {"no_such_key": 123}})
        assert response.status_code == 200
        assert response.json()["success"] is True
        assert "quality.no_such_key" in response.json()["message"]
        assert response.json()["ignored_keys"] == ["quality.no_such_key"]

    def test_unknown_key_offers_close_match_suggestion(self, pipeline, monkeypatch):
        """拼错键与真实字段相近时 message 带「是否想写 X」，但 ignored_keys 保持干净标识符"""
        client = _client(monkeypatch)
        response = client.post(
            "/api/config", json={"augmentation": {"variant_per_seed": 5}}
        )
        assert response.status_code == 200
        payload = response.json()
        assert "augmentation.variant_per_seed" in payload["message"]
        assert "variants_per_seed" in payload["message"]  # 建议指向真实字段
        assert payload["ignored_keys"] == ["augmentation.variant_per_seed"]

    def test_valid_update_has_no_ignored_notice(self, pipeline, monkeypatch):
        """全是合法键时 ignored_keys 为空、message 不出现「已忽略」（钉住不误报噪声）"""
        client = _client(monkeypatch)
        response = client.post("/api/config", json={"quality": {"threshold": 0.7}})
        assert response.status_code == 200
        payload = response.json()
        assert payload["ignored_keys"] == []
        assert "已忽略" not in payload["message"]

    def test_ignored_keys_across_sections_are_joined(self, pipeline, monkeypatch):
        """多节多个未知键需全部列出（不能只报第一个），顺序按分区遍历序"""
        client = _client(monkeypatch)
        response = client.post(
            "/api/config",
            json={"quality": {"bad_a": 1}, "dedup": {"bad_b": 2}},
        )
        assert response.status_code == 200
        payload = response.json()
        assert payload["ignored_keys"] == ["quality.bad_a", "dedup.bad_b"]
        assert "quality.bad_a" in payload["message"]
        assert "dedup.bad_b" in payload["message"]

    def test_post_persists_to_cwd_config(self, pipeline, monkeypatch):
        """POST 成功后 CWD 的 config.yaml 需被实际写回，且能被重新加载

        断言落在 `models.default` 上而不是顶层 `default_model`：
        `load_config` 只从 `models.default` 读默认模型，写成顶层键等于没保存
        （下次加载会静默回落到 `ernie`）。
        """
        import os

        from augmentor.config import load_config

        client = _client(monkeypatch)
        cwd = Path(os.getcwd())
        (cwd / "config.yaml").write_text(
            (AI_DIR / "config.yaml").read_text(encoding="utf-8"), encoding="utf-8"
        )
        response = client.post("/api/config", json={"default_model": "gemini"})
        assert response.status_code == 200
        import yaml

        data = yaml.safe_load((cwd / "config.yaml").read_text(encoding="utf-8"))
        assert data["models"]["default"] == "gemini"
        # 真正的判据：保存后的文件必须能读回同一个默认模型
        assert load_config(str(cwd / "config.yaml")).default_model == "gemini"

    def test_models_section_updates(self, pipeline, monkeypatch):
        client = _client(monkeypatch)
        response = client.post(
            "/api/config", json={"vector": {"enabled": True, "backend": "faiss"}}
        )
        assert response.status_code == 200
        assert pipeline.config.vector.backend == "faiss"


class TestConfigErrorPaths:
    def test_get_config_500_when_pipeline_fails(self, monkeypatch):
        """get_pipeline 抛异常需降级 500 而非 500-unhandled"""
        import api.routes.config as cfg_module

        def _boom():
            raise RuntimeError("pipeline broken")

        monkeypatch.setattr(cfg_module, "get_pipeline", _boom)
        client = _client(monkeypatch)
        response = client.get("/api/config")
        assert response.status_code == 500
        assert "pipeline broken" in response.json()["detail"]

    def test_models_500_when_pipeline_fails(self, monkeypatch):
        import api.routes.config as cfg_module

        def _boom():
            raise RuntimeError("no pipeline")

        monkeypatch.setattr(cfg_module, "get_pipeline", _boom)
        client = _client(monkeypatch)
        response = client.get("/api/models")
        assert response.status_code == 500

    def test_update_500_when_pipeline_fails(self, monkeypatch):
        import api.routes.config as cfg_module

        def _boom():
            raise RuntimeError("no pipeline")

        monkeypatch.setattr(cfg_module, "get_pipeline", _boom)
        client = _client(monkeypatch)
        response = client.post("/api/config", json={"default_model": "x"})
        assert response.status_code == 500
