"""API 集成测试

验证路由注册、参数校验、错误码与核心业务流程的端到端可用性。
"""

import json
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def pipeline(tmp_path):
    """构造使用临时目录的管道实例

    Args:
        tmp_path: pytest 临时目录

    Returns:
        AugmentorPipeline 实例
    """
    from augmentor import AugmentorPipeline, load_config
    from augmentor.checkpoint import CheckpointManager
    from augmentor.versioning import VersionManager

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")

    instance = AugmentorPipeline(config)
    # 重定向到临时目录，避免污染仓库
    instance.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    instance.checkpoint_manager = CheckpointManager(
        checkpoint_dir=str(tmp_path / "checkpoints")
    )
    return instance


@pytest.fixture
def client(pipeline, monkeypatch):
    """构造 TestClient，并注入管道实例

    Args:
        pipeline: 管道实例
        monkeypatch: pytest fixture

    Returns:
        TestClient 实例
    """
    import api.deps as deps
    from api.main import app

    monkeypatch.setattr(deps, "_pipeline", pipeline)
    return TestClient(app)


@pytest.fixture
def data_file(tmp_path, sample_items):
    """写入临时数据文件

    Args:
        tmp_path: 临时目录
        sample_items: 样本数据

    Returns:
        文件路径字符串
    """
    path = tmp_path / "train_data_api.json"
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(sample_items, f, ensure_ascii=False)
    return str(path)


class TestHealth:
    """健康检查"""

    def test_health_ok(self, client):
        """健康检查必须返回 ok，供探针使用"""
        response = client.get("/api/health")

        assert response.status_code == 200
        assert response.json()["status"] == "ok"

    def test_process_time_header_present(self, client):
        """中间件应注入耗时头，便于定位慢请求"""
        response = client.get("/api/health")
        assert "X-Process-Time" in response.headers


class TestDataRoutes:
    """数据管理路由"""

    def test_list_files(self, client):
        """文件列表需返回 JSON 结构"""
        response = client.get("/api/data/list")

        assert response.status_code == 200
        assert "files" in response.json()

    def test_load_with_pagination(self, client, data_file):
        """分页需正确切分数据"""
        response = client.get(f"/api/data/load/{data_file}", params={"page": 1, "page_size": 2})

        assert response.status_code == 200
        payload = response.json()
        assert payload["total"] == 5
        assert len(payload["items"]) == 2

    def test_load_with_search(self, client, data_file):
        """搜索需按关键词过滤"""
        response = client.get(f"/api/data/load/{data_file}", params={"search": "退租"})

        payload = response.json()
        assert payload["total"] == 1

    def test_load_missing_file_404(self, client, tmp_path):
        """文件不存在需返回 404"""
        response = client.get(f"/api/data/load/{tmp_path / 'nope.json'}")
        assert response.status_code == 404

    def test_update_item(self, client, data_file):
        """更新后重新读取应生效"""
        response = client.put(
            f"/api/data/update/{data_file}",
            params={"index": 0},
            json={"instruction": "新问题", "input": "", "output": "新回答"}
        )

        assert response.status_code == 200
        assert response.json()["success"] is True

        reloaded = client.get(f"/api/data/load/{data_file}", params={"page_size": 1})
        assert reloaded.json()["items"][0]["instruction"] == "新问题"

    def test_update_index_out_of_range_400(self, client, data_file):
        """索引越界需返回 400"""
        response = client.put(
            f"/api/data/update/{data_file}",
            params={"index": 999},
            json={"instruction": "x"}
        )
        assert response.status_code == 400

    def test_delete_item(self, client, data_file):
        """删除后总数应减少"""
        response = client.delete(f"/api/data/delete/{data_file}", params={"index": 0})

        assert response.status_code == 200
        reloaded = client.get(f"/api/data/load/{data_file}")
        assert reloaded.json()["total"] == 4


class TestQualityRoutes:
    """质量路由"""

    def test_evaluate(self, client, data_file):
        """质量评估需返回通过率等统计"""
        response = client.post("/api/quality/evaluate", json={"input_file": data_file})

        assert response.status_code == 200
        payload = response.json()
        assert payload["total_samples"] == 5
        assert 0.0 <= payload["pass_rate"] <= 1.0

    def test_evaluate_missing_file_404(self, client, tmp_path):
        """文件不存在需返回 404"""
        response = client.post(
            "/api/quality/evaluate", json={"input_file": str(tmp_path / "nope.json")}
        )
        assert response.status_code == 404

    def test_dedup(self, client, data_file):
        """去重需返回移除数量"""
        response = client.post("/api/quality/dedup", json={"input_file": data_file})

        assert response.status_code == 200
        payload = response.json()
        assert payload["original_count"] == 5
        assert payload["deduplicated_count"] <= 5

    def test_report(self, client, data_file):
        """质量报告需包含图表数据"""
        response = client.post("/api/quality/report", json={"input_file": data_file})

        assert response.status_code == 200
        assert "charts" in response.json()

    def test_clean(self, client, data_file):
        """清洗接口需返回统计结果"""
        response = client.post("/api/quality/clean", json={"input_file": data_file})

        assert response.status_code == 200
        assert response.json()["original_count"] == 5

    def test_annotate(self, client, data_file):
        """标注接口需返回意图与情感分布"""
        response = client.post("/api/quality/annotate", json={"input_file": data_file})

        assert response.status_code == 200
        payload = response.json()
        assert "intent_distribution" in payload
        assert "sentiment_distribution" in payload

    def test_benchmark(self, client, data_file):
        """基准接口需返回指标"""
        response = client.post("/api/quality/benchmark", json={"input_file": data_file})

        assert response.status_code == 200
        assert "metrics" in response.json()


class TestExportRoutes:
    """导出路由"""

    def test_list_formats(self, client):
        """格式清单需包含五种目标格式"""
        response = client.get("/api/export/formats")

        assert response.status_code == 200
        formats = response.json()["formats"]
        for expected in ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"]:
            assert expected in formats

    def test_preview(self, client, data_file):
        """预览需返回转换结果与告警"""
        response = client.post(
            "/api/export/preview", json={"input_file": data_file, "format": "alpaca"}
        )

        assert response.status_code == 200
        payload = response.json()
        assert payload["format"] == "alpaca"
        assert payload["converted_data"]
        assert "warnings" in payload

    def test_preview_invalid_format_500(self, client, data_file):
        """非法格式应返回错误而不是静默成功"""
        response = client.post(
            "/api/export/preview", json={"input_file": data_file, "format": "nope"}
        )
        assert response.status_code == 500

    def test_export_to_dir(self, client, data_file, tmp_path):
        """导出接口需实际产出文件"""
        output_dir = tmp_path / "exports"
        response = client.post("/api/data/export", json={
            "input_file": data_file,
            "output_dir": str(output_dir),
            "formats": ["jsonl", "alpaca"]
        })

        assert response.status_code == 200
        files = response.json()["files"]
        assert set(files.keys()) == {"jsonl", "alpaca"}
        for path in files.values():
            assert Path(path).exists()

    def test_batch_export(self, client, tmp_path, sample_items):
        """批量导出需支持多数据集 × 多格式"""
        first = tmp_path / "ds_a.json"
        second = tmp_path / "ds_b.json"
        for path in (first, second):
            with open(path, 'w', encoding='utf-8') as f:
                json.dump(sample_items, f, ensure_ascii=False)

        response = client.post("/api/export/batch", json={
            "datasets": {"a": str(first), "b": str(second)},
            "output_dir": str(tmp_path / "batch"),
            "formats": ["jsonl", "chatml"]
        })

        assert response.status_code == 200
        results = response.json()["results"]
        assert set(results.keys()) == {"a", "b"}
        assert set(results["a"].keys()) == {"jsonl", "chatml"}


class TestVersionRoutes:
    """版本路由"""

    def test_create_and_list(self, client, data_file):
        """创建版本后应出现在列表中"""
        created = client.post(
            f"/api/versions/create?filename={data_file}",
            json={"label": "v1", "description": "desc"}
        )

        assert created.status_code == 200
        version_id = created.json()["version_id"]

        listed = client.get("/api/versions")
        assert listed.status_code == 200
        assert any(v["version_id"] == version_id for v in listed.json()["versions"])

    def test_get_version_detail_and_data(self, client, data_file):
        """版本详情与数据均需可读取"""
        version_id = client.post(
            f"/api/versions/create?filename={data_file}", json={}
        ).json()["version_id"]

        detail = client.get(f"/api/versions/{version_id}")
        assert detail.status_code == 200
        assert detail.json()["item_count"] == 5

        data = client.get(f"/api/versions/{version_id}/data")
        assert len(data.json()["items"]) == 5

    def test_history_route_not_shadowed_by_version_id(self, client, data_file):
        """/api/versions/history 不能被 /{version_id} 路由吞掉"""
        client.post(f"/api/versions/create?filename={data_file}", json={})

        response = client.get("/api/versions/history")

        assert response.status_code == 200
        assert "history" in response.json()
        assert len(response.json()["history"]) >= 1

    def test_diff_versions(self, client, data_file):
        """版本对比需返回增删数量"""
        first = client.post(
            f"/api/versions/create?filename={data_file}", json={"label": "a"}
        ).json()["version_id"]
        second = client.post(
            f"/api/versions/create?filename={data_file}", json={"label": "b"}
        ).json()["version_id"]

        response = client.post("/api/versions/diff", json={
            "version1": first, "version2": second
        })

        assert response.status_code == 200
        assert response.json()["added_count"] == 0

    def test_rollback_and_delete(self, client, data_file):
        """回滚需成功，且当前版本不可删除"""
        first = client.post(
            f"/api/versions/create?filename={data_file}", json={"label": "first"}
        ).json()["version_id"]
        client.post(f"/api/versions/create?filename={data_file}", json={"label": "second"})

        rollback = client.post(f"/api/versions/{first}/rollback")
        assert rollback.json()["success"] is True

        # 回滚后 first 成为当前版本，删除应失败
        delete_current = client.delete(f"/api/versions/{first}")
        assert delete_current.status_code == 500

    def test_create_missing_file_404(self, client, tmp_path):
        """源文件不存在需返回 404"""
        response = client.post(
            f"/api/versions/create?filename={tmp_path / 'nope.json'}", json={}
        )
        assert response.status_code == 404


class TestConfigRoutes:
    """配置路由"""

    def test_get_config(self, client):
        """配置接口需返回模型与质量配置"""
        response = client.get("/api/config")

        assert response.status_code == 200
        payload = response.json()
        assert payload["default_model"] == "ernie"
        assert "quality" in payload
        assert "vector" in payload

    def test_list_models(self, client):
        """模型列表需包含新增后端"""
        response = client.get("/api/models")

        assert response.status_code == 200
        models = response.json()["models"]
        for expected in ["ernie", "openai", "ollama", "claude", "gemini"]:
            assert expected in models


class TestAugmentRoutes:
    """增强路由"""

    def test_progress(self, client):
        """进度接口需返回结构体"""
        response = client.get("/api/augment/progress")

        assert response.status_code == 200

    def test_checkpoints(self, client):
        """断点列表接口需返回列表"""
        response = client.get("/api/augment/checkpoints")

        assert response.status_code == 200
        assert "checkpoints" in response.json()


class TestMultimodalRoutes:
    """多模态路由"""

    def test_formats(self, client):
        """格式清单需返回图像与音频扩展名"""
        response = client.get("/api/multimodal/formats")

        assert response.status_code == 200
        payload = response.json()
        assert ".png" in payload["image_extensions"]
        assert ".wav" in payload["audio_extensions"]

    def test_process_text_only(self, client):
        """纯文本处理需返回有效记录"""
        response = client.post("/api/multimodal/process", json={"text": "说明文字"})

        assert response.status_code == 200
        assert response.json()["modalities"] == ["text"]

    def test_process_missing_image(self, client):
        """图像缺失时记录无效并带错误信息"""
        response = client.post(
            "/api/multimodal/process", json={"text": "说明", "image": "not-exists.png"}
        )

        assert response.status_code == 200
        assert response.json()["valid"] is False

    def test_scan_missing_directory_400(self, client):
        """目录不存在需返回 400"""
        response = client.post("/api/multimodal/scan", json={"directory": "not-a-dir"})
        assert response.status_code == 400
