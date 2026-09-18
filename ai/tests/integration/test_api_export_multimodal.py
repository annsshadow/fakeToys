"""export 与 multimodal API 路由扩展测试

export: 单数据集导出/批量/预览/格式列表；
multimodal: 融合处理、目录扫描（含 400 非法目录）、格式列表。
"""

import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent

SAMPLE = [
    {"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"}
    for i in range(1, 6)
]


@pytest.fixture
def export_env(tmp_path, monkeypatch):
    """隔离 CWD、准备样本数据与真实管道，返回 (client, 文件名, tmp)

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Yields:
        (TestClient, 样本文件名, 临时目录)
    """
    import os

    from fastapi.testclient import TestClient

    from augmentor import AugmentorPipeline, load_config
    from augmentor.checkpoint import CheckpointManager
    from augmentor.versioning import VersionManager

    from api.main import app

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    instance = AugmentorPipeline(config)
    instance.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    instance.checkpoint_manager = CheckpointManager(checkpoint_dir=str(tmp_path / "checkpoints"))

    import api.deps as deps
    monkeypatch.setattr(deps, "_pipeline", instance)
    monkeypatch.chdir(tmp_path)

    name = "export_src.json"
    Path(os.getcwd(), name).write_text(json.dumps(SAMPLE, ensure_ascii=False), encoding="utf-8")
    yield TestClient(app), name, tmp_path


class TestExportEndpoint:
    def test_export_single_formats(self, export_env):
        client, name, tmp = export_env
        out_dir = str(tmp / "exp_out")
        response = client.post(
            "/api/data/export",
            json={"input_file": name, "output_dir": out_dir, "formats": ["jsonl", "csv"]},
        )
        assert response.status_code == 200
        files = response.json()["files"]
        assert "jsonl" in files and "csv" in files
        for path in files.values():
            assert Path(path).exists()

    def test_export_missing_input_500(self, export_env):
        client, _, _ = export_env
        response = client.post(
            "/api/data/export",
            json={"input_file": "no_such.json", "output_dir": "x"},
        )
        assert response.status_code == 500


class TestBatchExportEndpoint:
    def test_batch_export(self, export_env):
        client, name, tmp = export_env
        out_dir = str(tmp / "batch_out")
        response = client.post(
            "/api/export/batch",
            json={"datasets": {"ds1": name}, "output_dir": out_dir, "formats": ["jsonl"]},
        )
        assert response.status_code == 200
        assert response.json()["success"] is True

    def test_batch_missing_file_404(self, export_env):
        client, _, tmp = export_env
        response = client.post(
            "/api/export/batch",
            json={"datasets": {"bad": "no_such.json"}, "output_dir": str(tmp / "o")},
        )
        assert response.status_code == 404


class TestPreviewEndpoint:
    def test_preview_jsonl(self, export_env):
        client, name, _ = export_env
        response = client.post(
            "/api/export/preview", json={"input_file": name, "format": "jsonl", "size": 2}
        )
        assert response.status_code == 200
        payload = response.json()
        assert payload["format"] == "jsonl"
        assert len(payload["converted_data"]) == 2

    def test_preview_missing_404(self, export_env):
        client, _, _ = export_env
        response = client.post("/api/export/preview", json={"input_file": "nope.json"})
        assert response.status_code == 404


class TestFormatsEndpoint:
    def test_list_formats(self, export_env):
        client, _, _ = export_env
        response = client.get("/api/export/formats")
        assert response.status_code == 200
        formats = response.json()["formats"]
        assert "jsonl" in formats
        assert "csv" in formats


class TestMultimodalProcess:
    def test_fuse_text_only(self, export_env):
        client, _, _ = export_env
        response = client.post(
            "/api/multimodal/process", json={"text": "纯文本数据"}
        )
        assert response.status_code == 200
        record = response.json()
        assert "text" in record

    def test_fuse_invalid_image_reports_error(self, export_env):
        client, _, tmp = export_env
        response = client.post(
            "/api/multimodal/process",
            json={"text": "带图", "image": str(tmp / "no_such.jpg")},
        )
        assert response.status_code == 200
        record = response.json()
        assert record.get("errors"), "不存在的图片应记录处理错误"


class TestMultimodalScan:
    def test_scan_directory_fuses_pair(self, export_env):
        client, _, tmp = export_env
        media = tmp / "media"
        media.mkdir()
        (media / "sample.jpg").write_bytes(b"fake")
        (media / "sample.wav").write_bytes(b"fake")
        response = client.post(
            "/api/multimodal/scan", json={"directory": str(media)}
        )
        assert response.status_code == 200
        assert "records" in response.json()

    def test_scan_missing_dir_400(self, export_env):
        client, _, tmp = export_env
        response = client.post(
            "/api/multimodal/scan", json={"directory": str(tmp / "no_such_dir")}
        )
        assert response.status_code == 400


class TestMultimodalFormats:
    def test_supported_formats(self, export_env):
        client, _, _ = export_env
        response = client.get("/api/multimodal/formats")
        assert response.status_code == 200
        payload = response.json()
        assert ".jpg" in payload["image_extensions"]
        assert ".wav" in payload["audio_extensions"]
