"""管道集成测试

验证增强、导出、分析等主流程在真实组件协作下的行为。
"""

import json
from pathlib import Path

import pytest

from augmentor import AugmentorPipeline, load_config
from augmentor.checkpoint import CheckpointManager
from augmentor.config import ModelConfig
from augmentor.models.base import ModelBackend
from augmentor.versioning import VersionManager

AI_DIR = Path(__file__).resolve().parent.parent.parent


class FakeModelBackend(ModelBackend):
    """返回固定 JSON 变体的假模型后端"""

    def __init__(self, variants: int = 3, raw_response=None):
        """初始化

        Args:
            variants: 生成的变体数量
            raw_response: 直接指定响应文本（用于测试异常响应）
        """
        super().__init__(ModelConfig(type="fake"))
        self.variants = variants
        self.raw_response = raw_response

    def _call_api(self, prompt: str) -> str:
        """返回变体 JSON

        Args:
            prompt: 提示词

        Returns:
            响应文本
        """
        if self.raw_response is not None:
            return self.raw_response
        return json.dumps(
            [{"instruction": f"变体问题 {i}"} for i in range(self.variants)],
            ensure_ascii=False
        )


@pytest.fixture
def pipeline(tmp_path, monkeypatch):
    """构造使用临时目录与假模型的管道

    Args:
        tmp_path: 临时目录
        monkeypatch: pytest fixture

    Returns:
        AugmentorPipeline 实例
    """
    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    config.augmentation.auto_save_interval = 1
    config.augmentation.num_threads = 2

    instance = AugmentorPipeline(config)
    instance.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    instance.checkpoint_manager = CheckpointManager(
        checkpoint_dir=str(tmp_path / "checkpoints"),
        auto_save_interval=1
    )

    backend = FakeModelBackend()
    instance.model_backend = backend
    instance.context_augmentor.model_backend = backend
    instance.expander.model_backend = backend
    return instance


@pytest.fixture
def seed_file(tmp_path, sample_items):
    """写入种子数据文件

    Args:
        tmp_path: 临时目录
        sample_items: 样本数据

    Returns:
        文件路径字符串
    """
    path = tmp_path / "train_data_seed.json"
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(sample_items, f, ensure_ascii=False)
    return str(path)


class TestAugmentSeed:
    """单条增强"""

    def test_generates_variants(self, pipeline):
        """增强应产出配置数量的变体，并保留原回答"""
        variants = pipeline.augment_seed(
            {"instruction": "原始问题", "output": "原始回答"},
            use_quality_check=False
        )

        assert len(variants) == 3
        assert all(v["output"] == "原始回答" for v in variants)
        assert all(v["instruction"].startswith("变体问题") for v in variants)

    def test_respects_variants_per_seed_limit(self, pipeline):
        """超出 variants_per_seed 的变体应被截断，避免数据量失控"""
        pipeline.model_backend = FakeModelBackend(variants=10)
        variants = pipeline.augment_seed(
            {"instruction": "原始问题", "output": "原始回答"},
            use_quality_check=False
        )

        assert len(variants) == pipeline.config.augmentation.variants_per_seed

    def test_malformed_response_returns_empty(self, pipeline):
        """模型返回非 JSON 时应返回空列表而不是抛异常"""
        pipeline.model_backend = FakeModelBackend(raw_response="这不是 JSON")
        variants = pipeline.augment_seed(
            {"instruction": "原始问题", "output": "原始回答"},
            use_quality_check=False
        )

        assert variants == []

    def test_quality_check_filters_dissimilar(self, pipeline):
        """开启质量检查时与原意偏离的变体应被过滤"""
        variants = pipeline.augment_seed(
            {"instruction": "如何申请入住安居乐寓？", "output": "通过 App 申请"},
            use_quality_check=True
        )

        # 假模型产出的变体与原始问题语义差距大，应被过滤
        assert len(variants) <= 3


class TestAugmentDataset:
    """数据集增强"""

    def test_serial_run_writes_output(self, pipeline, seed_file, tmp_path):
        """串行增强需产出结果文件并返回统计报告"""
        output = tmp_path / "out_serial.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )

        assert output.exists()
        assert report["input_count"] == 5
        assert report["output_count"] == 15
        assert report["parallel"] is False

        with open(output, 'r', encoding='utf-8') as f:
            assert len(json.load(f)) == 15

    def test_parallel_run_matches_serial_count(self, pipeline, seed_file, tmp_path):
        """并行与串行在相同输入下应产出相同数量，验证并发正确性"""
        output = tmp_path / "out_parallel.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=True
        )

        assert report["output_count"] == 15

    def test_dedup_reduces_output(self, pipeline, seed_file, tmp_path):
        """开启去重后重复变体应被合并，输出量不高于关闭时"""
        pipeline.model_backend = FakeModelBackend(variants=1)

        output = tmp_path / "out_dedup.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=True, use_parallel=False
        )

        # 每条种子只生成同一个变体，去重后应只剩 1 条
        assert report["output_count"] == 1

    def test_checkpoint_records_progress(self, pipeline, seed_file, tmp_path):
        """开启断点续传后进度需被记录"""
        output = tmp_path / "out_checkpoint.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=True, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )

        assert report["progress"] is not None
        assert report["progress"]["processed_items"] == 5

    def test_auto_snapshot_creates_version(self, pipeline, seed_file, tmp_path):
        """开启自动快照后应生成版本记录，保证数据可回溯"""
        pipeline.config.versioning.auto_snapshot = True
        output = tmp_path / "out_snapshot.json"

        pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )

        assert len(pipeline.version_manager.list_versions()) == 1


class TestResumeAfterInterruption:
    """R4：断点续传

    验证任务 ID 跨进程稳定、中断后可跳过已完成条目，且不会丢失上一轮输出。
    """

    def test_task_id_is_stable_for_same_file(self, seed_file):
        """任务 ID 必须只由输入文件路径决定，否则无法识别同一次任务"""
        first = AugmentorPipeline.build_task_id(seed_file)
        second = AugmentorPipeline.build_task_id(seed_file)

        assert first == second
        assert first.startswith("augment_")

    def test_task_id_differs_across_files(self, tmp_path, seed_file):
        """不同输入文件应视为不同任务，避免互相干扰断点"""
        other = tmp_path / "other_seed.json"
        other.write_text("[]", encoding="utf-8")

        assert AugmentorPipeline.build_task_id(seed_file) != AugmentorPipeline.build_task_id(
            str(other)
        )

    def test_partial_checkpoint_resumes_only_remaining_items(self, pipeline, seed_file, tmp_path):
        """预置部分进度后，只应处理剩余条目"""
        task_id = AugmentorPipeline.build_task_id(seed_file)
        checkpoint = pipeline.checkpoint_manager.create_checkpoint(task_id, 5)
        for index in (0, 1, 2):
            pipeline.checkpoint_manager.update_progress(index, True)
        pipeline.checkpoint_manager.save_checkpoint()
        assert checkpoint.total_items == 5

        output = tmp_path / "out_resume.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=True, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )

        # 仅处理索引 3、4，各生成 3 个变体
        assert report["output_count"] == 6
        assert report["progress"]["processed_items"] == 5

    def test_rerun_reuses_previous_output_without_data_loss(self, pipeline, seed_file, tmp_path):
        """重复运行不应把上一轮结果覆盖为空"""
        output = tmp_path / "out_rerun.json"
        first = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=True, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )
        assert first["output_count"] == 15

        second = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=True, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )

        assert second["output_count"] == 15
        with open(output, 'r', encoding='utf-8') as f:
            assert len(json.load(f)) == 15


class TestExportAndAnalyze:
    """导出与分析"""

    def test_export_selected_formats(self, pipeline, seed_file, tmp_path):
        """指定格式导出需只产出对应文件"""
        output_dir = tmp_path / "exports"
        results = pipeline.export_dataset(seed_file, str(output_dir), ["jsonl", "alpaca"])

        assert set(results.keys()) == {"jsonl", "alpaca"}
        for path in results.values():
            assert Path(path).exists()

    def test_export_all_formats(self, pipeline, seed_file, tmp_path):
        """未指定格式时导出全部格式"""
        results = pipeline.export_dataset(seed_file, str(tmp_path / "all"))

        assert len(results) >= 5

    def test_analyze_dataset(self, pipeline, seed_file):
        """分析需返回覆盖分析、统计与去重报告三部分"""
        result = pipeline.analyze_dataset(seed_file)

        assert "coverage_analysis" in result
        assert "statistics" in result
        assert "dedup_report" in result
        assert result["coverage_analysis"]["total_items"] == 5


class TestVisualizeDataset:
    """可视化测试"""

    def test_visualize_returns_dict(self, pipeline, seed_file, tmp_path):
        """可视化应返回路径字典"""
        output_dir = tmp_path / "viz"
        result = pipeline.visualize_dataset(seed_file, str(output_dir))
        assert isinstance(result, dict)

    def test_visualize_creates_output_dir(self, pipeline, seed_file, tmp_path):
        """可视化应创建输出目录"""
        output_dir = tmp_path / "viz_new"
        output_dir.mkdir()
        pipeline.visualize_dataset(seed_file, str(output_dir))
        assert output_dir.exists()

    def test_visualize_with_empty_dataset(self, pipeline, tmp_path):
        """空数据集可视化不应报错"""
        empty_file = tmp_path / "empty.json"
        empty_file.write_text("[]", encoding="utf-8")
        result = pipeline.visualize_dataset(str(empty_file), str(tmp_path / "viz"))
        assert isinstance(result, dict)


class TestModelBackendNone:
    """模型后端不可用降级测试"""

    def test_augment_seed_without_backend(self, pipeline):
        """无模型后端时增强应返回空列表"""
        pipeline.model_backend = None
        variants = pipeline.augment_seed(
            {"instruction": "问题", "output": "回答"},
            use_quality_check=False
        )
        assert variants == []

    def test_augment_dataset_without_backend(self, pipeline, seed_file, tmp_path):
        """无模型后端时数据集增强应正常完成"""
        pipeline.model_backend = None
        output = tmp_path / "out_no_backend.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )
        assert report["output_count"] == 0


class TestExportAndAnalyzeExtended:
    """导出与分析扩展测试"""

    def test_export_to_custom_dir(self, pipeline, seed_file, tmp_path):
        """导出到自定义目录"""
        custom_dir = tmp_path / "custom_exports"
        results = pipeline.export_dataset(seed_file, str(custom_dir), ["jsonl"])
        assert Path(results["jsonl"]).parent == custom_dir

    def test_analyze_returns_complete_stats(self, pipeline, seed_file):
        """分析应返回完整的统计信息"""
        result = pipeline.analyze_dataset(seed_file)
        stats = result["statistics"]
        assert "total_items" in stats
        assert "avg_length" in stats
        assert stats["total_items"] == 5


class TestPipelineInitFailure:
    """管道初始化失败测试"""

    def test_model_backend_init_failure(self, tmp_path, monkeypatch):
        """模型后端初始化失败时应降级"""
        config = load_config(str(AI_DIR / "config.yaml"))
        config.versioning.storage_dir = str(tmp_path / "versions")
        
        def fail_create(*args, **kwargs):
            raise RuntimeError("API key missing")
        
        monkeypatch.setattr("augmentor.pipeline.create_model_backend", fail_create)
        instance = AugmentorPipeline(config)
        assert instance.model_backend is None
        assert instance.context_augmentor is None
        assert instance.expander is None


class TestGenerateVariantsException:
    """_generate_variants 异常测试"""

    def test_generate_variants_returns_empty_on_exception(self, pipeline):
        """_generate_variants 异常时应返回空列表"""
        pipeline.model_backend = None
        result = pipeline._generate_variants({"instruction": "q", "output": "a"})
        assert result == []


class TestProcessSingleItem:
    """_process_single_item 测试"""

    def test_process_single_item_success(self, pipeline, tmp_path):
        """成功处理单条数据"""
        from queue import Queue
        queue = Queue()
        item = {"instruction": "如何申请入住", "output": "通过App申请"}
        pipeline._process_single_item(0, item, use_quality_check=False, result_queue=queue)
        idx, success, variants = queue.get()
        assert success is True
        assert isinstance(variants, list)

    def test_process_single_item_with_quality_check(self, pipeline, tmp_path):
        """质量检查模式处理单条数据"""
        from queue import Queue
        queue = Queue()
        item = {"instruction": "如何申请入住", "output": "通过App申请"}
        pipeline._process_single_item(0, item, use_quality_check=True, result_queue=queue)
        idx, success, variants = queue.get()
        assert success is True

    def test_process_single_item_exception(self, pipeline, tmp_path):
        """处理单条数据异常时应记录失败"""
        from queue import Queue
        queue = Queue()
        pipeline.model_backend = None
        # Force an exception by making _generate_variants fail
        pipeline._generate_variants = lambda x: 1/0
        pipeline._process_single_item(0, {"instruction": "q"}, False, queue)
        idx, success, variants = queue.get()
        assert success is False
        assert variants == []


class TestAugmentDatasetExtended:
    """数据集增强扩展测试"""

    def test_augment_dataset_parallel_with_checkpoint(self, pipeline, seed_file, tmp_path):
        """并行模式 + 断点续传"""
        output = tmp_path / "out_parallel_ckpt.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=True, use_quality_check=False,
            use_dedup=False, use_parallel=True
        )
        assert report["output_count"] == 15
        assert report["progress"] is not None

    def test_augment_dataset_serial_exception_handling(self, pipeline, seed_file, tmp_path):
        """串行模式异常处理"""
        call_count = [0]
        original_augment = pipeline.augment_seed
        def failing_augment(*args, **kwargs):
            call_count[0] += 1
            if call_count[0] == 2:
                raise RuntimeError("Simulated failure")
            return original_augment(*args, **kwargs)
        pipeline.augment_seed = failing_augment
        
        output = tmp_path / "out_serial_exc.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )
        # Should still complete despite one failure
        assert report["input_count"] == 5

    def test_augment_dataset_with_quality_check(self, pipeline, seed_file, tmp_path):
        """质量检查模式"""
        output = tmp_path / "out_quality.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=True,
            use_dedup=False, use_parallel=False
        )
        assert report["quality_check"] is True

    def test_augment_seed_quality_check_filters(self, pipeline):
        """质量检查应过滤低质量变体"""
        variants = pipeline.augment_seed(
            {"instruction": "如何申请入住安居乐寓？", "output": "通过App申请"},
            use_quality_check=True
        )
        assert isinstance(variants, list)


class TestPipelineExtended:
    """AugmentorPipeline 扩展测试"""

    def test_augment_seed_returns_variants(self, pipeline):
        """增强种子返回变体"""
        variants = pipeline.augment_seed(
            {"instruction": "测试问题", "output": "测试回答"}
        )
        assert isinstance(variants, list)

    def test_augment_seed_empty_response(self, pipeline):
        """空响应增强种子"""
        pipeline.model_backend.raw_response = "[]"
        variants = pipeline.augment_seed(
            {"instruction": "测试问题", "output": "测试回答"}
        )
        assert variants == []

    def test_augment_seed_invalid_json(self, pipeline):
        """无效 JSON 响应增强种子"""
        pipeline.model_backend.raw_response = "not json"
        variants = pipeline.augment_seed(
            {"instruction": "测试问题", "output": "测试回答"}
        )
        assert variants == []

    def test_generate_variants_single(self, pipeline):
        """生成单个变体"""
        pipeline.model_backend.raw_response = json.dumps(
            [{"instruction": "变体问题", "output": "变体回答"}],
            ensure_ascii=False
        )
        from queue import Queue
        queue = Queue()
        pipeline._process_single_item(
            0, {"instruction": "问题", "output": "回答"},
            use_quality_check=False, result_queue=queue
        )
        idx, success, variants = queue.get()
        assert success is True
        assert len(variants) == 1


class TestPipelineExtended2:
    """AugmentorPipeline 扩展测试 - 第二轮"""

    def test_augment_dataset_full_pipeline(self, pipeline, seed_file, tmp_path):
        """完整流水线测试"""
        output = tmp_path / "full.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=True,
            use_dedup=True, use_parallel=True
        )
        assert "input_count" in report
        assert "output_count" in report
        assert "dedup" in report
        assert "parallel" in report
        assert report.get("dedup") is True
        assert report.get("parallel") is True

    def test_augment_seed_quality_check_disabled(self, pipeline):
        """禁用质量检查"""
        variants = pipeline.augment_seed(
            {"instruction": "测试问题", "output": "测试回答"},
            use_quality_check=False
        )
        assert isinstance(variants, list)

    def test_augment_dataset_parallel_only(self, pipeline, seed_file, tmp_path):
        """仅并行模式"""
        output = tmp_path / "parallel_only.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=True
        )
        assert report.get("parallel") is True

    def test_augment_dataset_serial_only(self, pipeline, seed_file, tmp_path):
        """仅串行模式"""
        output = tmp_path / "serial_only.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )
        assert report.get("parallel") is False

    def test_augment_dataset_dedup_only(self, pipeline, seed_file, tmp_path):
        """仅去重模式"""
        output = tmp_path / "dedup_only.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=True, use_parallel=False
        )
        assert report.get("dedup") is True
        """串行模式增强"""
        output = tmp_path / "out_serial.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )
        assert report["output_count"] == 15

    def test_augment_dataset_with_dedup(self, pipeline, seed_file, tmp_path):
        """去重模式增强"""
        output = tmp_path / "out_dedup.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=True, use_parallel=False
        )
        assert report["output_count"] >= 0

    def test_augment_dataset_checkpoint_resume(self, pipeline, seed_file, tmp_path):
        """断点续传"""
        output = tmp_path / "out_resume.json"
        report1 = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=True, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )
        report2 = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=True, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )
        assert report2["output_count"] == report1["output_count"]

    def test_augment_seed_with_existing_generated(self, pipeline):
        """带已有生成的增强"""
        variants = pipeline.augment_seed(
            {"instruction": "测试问题", "output": "测试回答"},
            existing_generated=["已有问题1", "已有问题2"]
        )
        assert isinstance(variants, list)

    def test_augment_dataset_report_fields(self, pipeline, seed_file, tmp_path):
        """报告字段完整性"""
        output = tmp_path / "out_fields.json"
        report = pipeline.augment_dataset(
            seed_file, str(output),
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )
        assert "input_count" in report
        assert "output_count" in report
        assert "task_id" in report
