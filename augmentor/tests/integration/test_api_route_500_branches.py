"""export/version/multimodal 路由剩余 500 异常分支注入测试

正常路径与 404/400 分支已在 test_api_export_multimodal /
test_api_version_middleware 覆盖；本文件把底层依赖换成「抛普通
异常」的假对象，确定性命中 except Exception → 500 的收尾分支。
"""

import json
import os
from pathlib import Path
from types import SimpleNamespace

import pytest


@pytest.fixture
def env(tmp_path, monkeypatch):
    from fastapi.testclient import TestClient

    from api.main import app

    monkeypatch.chdir(tmp_path)
    items = [
        {"instruction": "如何申请？", "input": "", "output": "登录"},
        {"instruction": "多少钱？", "input": "", "output": "按月"},
    ]
    Path(os.getcwd(), "a.json").write_text(
        json.dumps(items, ensure_ascii=False), encoding="utf-8"
    )
    yield TestClient(app), tmp_path


class _BoomProcessor:
    """构造即抛/调用即抛的多模态假处理器"""

    def fuse_modalities(self, *args, **kwargs):
        raise RuntimeError("fuse boom")

    def process_directory(self, directory):
        raise RuntimeError("scan boom")

    def generate_report(self, records):
        return {}


class TestMultimodal500:
    def test_process_generic_500(self, env, monkeypatch):
        import augmentor.data as data_pkg

        monkeypatch.setattr(data_pkg, "MultimodalProcessor", _BoomProcessor)
        client, _ = env
        response = client.post("/api/multimodal/process", json={"text": "你好"})
        assert response.status_code == 500
        assert "fuse boom" in response.json()["detail"]

    def test_scan_generic_500(self, env, monkeypatch):
        import augmentor.data as data_pkg

        monkeypatch.setattr(data_pkg, "MultimodalProcessor", _BoomProcessor)
        client, _ = env
        response = client.post("/api/multimodal/scan", json={"directory": "."})
        assert response.status_code == 500
        assert "scan boom" in response.json()["detail"]


class _FakeExporter:
    def __init__(self, boom: bool = False):
        self._boom = boom

    def export_batch(self, datasets, output_dir, formats):
        raise RuntimeError("batch export boom")

    def get_supported_formats(self):
        if self._boom:
            raise RuntimeError("formats boom")
        return ["json", "jsonl"]


class TestExport500:
    def test_batch_export_generic_500(self, env, monkeypatch):
        import augmentor.export as export_module

        monkeypatch.setattr(export_module, "Exporter", _FakeExporter)
        client, _ = env
        response = client.post(
            "/api/export/batch",
            json={"datasets": {"ds": "a.json"}, "output_dir": "out"},
        )
        assert response.status_code == 500
        assert "batch export boom" in response.json()["detail"]

    def test_formats_generic_500(self, env, monkeypatch):
        import augmentor.export as export_module

        monkeypatch.setattr(export_module, "Exporter", lambda: _FakeExporter(boom=True))
        client, _ = env
        response = client.get("/api/export/formats")
        assert response.status_code == 500
        assert "formats boom" in response.json()["detail"]


class _FakeVersionManager:
    def __init__(self, fail_create=False, fail_rollback=False):
        self._fail_create = fail_create
        self._fail_rollback = fail_rollback

    def create_version(self, *args, **kwargs):
        if self._fail_create:
            raise RuntimeError("create boom")
        return SimpleNamespace(version_id="v-new")

    def rollback(self, version_id):
        if self._fail_rollback:
            raise RuntimeError("rollback boom")
        return True


def _patch_pipeline(monkeypatch, vm):
    import api.routes.version as version_module

    monkeypatch.setattr(
        version_module, "get_pipeline",
        lambda: SimpleNamespace(version_manager=vm),
    )


class TestVersion500:
    def test_create_version_generic_500(self, env, monkeypatch):
        _patch_pipeline(monkeypatch, _FakeVersionManager(fail_create=True))
        client, _ = env
        response = client.post(
            "/api/versions/create", params={"filename": "a.json"}, json={}
        )
        assert response.status_code == 500
        assert "create boom" in response.json()["detail"]

    def test_rollback_generic_500(self, env, monkeypatch):
        _patch_pipeline(monkeypatch, _FakeVersionManager(fail_rollback=True))
        client, _ = env
        response = client.post("/api/versions/v-old/rollback")
        assert response.status_code == 500
        assert "rollback boom" in response.json()["detail"]

    def test_create_version_http_exception_passes_through(self, env, monkeypatch):
        """路由内的 HTTPException（如 404）必须原样直通，不得被包成 500"""
        from fastapi import HTTPException

        import api.routes.version as version_module

        def vm_create_http_404(*args, **kwargs):
            raise HTTPException(status_code=404, detail="版本不存在")

        monkeypatch.setattr(
            version_module, "get_pipeline",
            lambda: SimpleNamespace(
                version_manager=SimpleNamespace(create_version=vm_create_http_404)
            ),
        )
        client, _ = env
        response = client.post(
            "/api/versions/create", params={"filename": "a.json"}, json={}
        )
        assert response.status_code == 404
        assert response.json()["detail"] == "版本不存在"
