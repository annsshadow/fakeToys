# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""augment API 路由扩展测试

覆盖 start 端点的后台任务执行、progress 结构、checkpoints 列表
以及 get_pipeline 异常时的 500 降级。
"""

import json
import os
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def augment_env(tmp_path, monkeypatch):
    """隔离 CWD + 真实管道（版本/检查点存 tmp），写样本数据

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Yields:
        (TestClient, 输入文件名, 输出文件名, 临时目录)
    """
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

    in_name = "augment_in.json"
    out_name = "augment_out.json"
    items = [{"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"} for i in range(3)]
    Path(os.getcwd(), in_name).write_text(
        json.dumps(items, ensure_ascii=False), encoding="utf-8"
    )
    yield TestClient(app), in_name, out_name, tmp_path


class TestStartAugmentation:
    def test_start_returns_success_and_runs_background(self, augment_env):
        """TestClient 会等待后台任务：任务结束后输出文件应生成"""
        client, in_name, out_name, _ = augment_env
        response = client.post(
            "/api/augment/start",
            json={
                "input_file": in_name,
                "output_file": out_name,
                "use_quality": False,
                "use_dedup": False,
                "use_checkpoint": False,
            },
        )
        assert response.status_code == 200
        assert response.json()["success"] is True
        assert Path(os.getcwd(), out_name).exists()

    def test_start_with_quality_and_dedup(self, augment_env):
        """默认质量+去重开关下任务需正常完成（无模型时优雅降级）"""
        client, in_name, out_name, _ = augment_env
        response = client.post(
            "/api/augment/start",
            json={"input_file": in_name, "output_file": out_name},
        )
        assert response.status_code == 200


class TestProgressAndCheckpoints:
    def test_progress_structure(self, augment_env):
        client, *_ = augment_env
        response = client.get("/api/augment/progress")
        assert response.status_code == 200
        assert isinstance(response.json(), dict)

    def test_checkpoints_list(self, augment_env):
        client, in_name, out_name, _ = augment_env
        # 先创建一个断点
        client.post(
            "/api/augment/start",
            json={
                "input_file": in_name,
                "output_file": out_name,
                "use_checkpoint": True,
            },
        )
        response = client.get("/api/augment/checkpoints")
        assert response.status_code == 200
        assert "checkpoints" in response.json()


class TestErrorPaths:
    def test_progress_500_when_pipeline_missing(self, monkeypatch):
        import api.routes.augment as aug

        def _boom():
            raise RuntimeError("no pipeline")

        monkeypatch.setattr(aug, "get_pipeline", _boom)
        from fastapi.testclient import TestClient

        from api.main import app

        client = TestClient(app)
        assert client.get("/api/augment/progress").status_code == 500

    def test_checkpoints_500_when_pipeline_missing(self, monkeypatch):
        import api.routes.augment as aug

        def _boom():
            raise RuntimeError("no pipeline")

        monkeypatch.setattr(aug, "get_pipeline", _boom)
        from fastapi.testclient import TestClient

        from api.main import app

        client = TestClient(app)
        assert client.get("/api/augment/checkpoints").status_code == 500
