# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""pipeline 剩余分支测试

覆盖 augment_async（线程池+gather）、串行路径成功/失败、并行质量过滤
与进度日志、并行失败降级、build_task_id 稳定性。
"""

import asyncio
import json

import pytest

from augmentor import AugmentorPipeline, load_config


AI_DIR = None


@pytest.fixture
def pipeline(tmp_path, monkeypatch):
    """无模型（离线）管道，版本/检查点/日志重定向到 tmp

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        AugmentorPipeline 实例
    """
    from pathlib import Path

    global AI_DIR
    AI_DIR = Path(__file__).resolve().parent.parent.parent
    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    config.versioning.auto_snapshot = False
    instance = AugmentorPipeline(config)
    instance.version_manager.storage_dir = Path(str(tmp_path / "versions"))
    instance.checkpoint_manager.checkpoint_dir = Path(str(tmp_path / "checkpoints"))
    monkeypatch.chdir(tmp_path)
    return instance


def _write_items(tmp_path, name, count):
    from pathlib import Path

    items = [
        {"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"}
        for i in range(count)
    ]
    path = Path(tmp_path) / name
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return str(path)


class TestAugmentAsync:
    def test_async_offline_writes_empty_output(self, pipeline, tmp_path):
        """无模型时异步增强应优雅产出空结果而非崩溃"""
        src = _write_items(tmp_path, "async_in.json", 4)
        out = str(tmp_path / "async_out.json")
        report = asyncio.run(
            pipeline.augment_async(
                src, out, use_checkpoint=False, use_quality_check=False, use_dedup=False
            )
        )
        assert report["async_mode"] is True
        assert report["input_count"] == 4
        assert report["output_count"] == 0
        assert json.loads(open(out, encoding="utf-8").read()) == []

    def test_async_with_variants_dedup(self, pipeline, tmp_path, monkeypatch):
        """注入伪变体后异步去重分支应生效并落盘"""
        import augmentor.pipeline as pl

        src = _write_items(tmp_path, "async_in2.json", 3)
        out = str(tmp_path / "async_out2.json")

        def fake_seed(self, item, use_quality_check=False, existing_generated=None):
            return [
                {"instruction": item["instruction"] + "（改写）", "input": "", "output": item["output"]},
                {"instruction": item["instruction"] + "（改写）", "input": "", "output": item["output"]},
            ]

        monkeypatch.setattr(pl.AugmentorPipeline, "augment_seed", fake_seed)
        report = asyncio.run(
            pipeline.augment_async(src, out, use_dedup=True, use_quality_check=False, use_checkpoint=False)
        )
        # 3 条 × 2 变体 = 6，去重后 3
        assert report["output_count"] == 3
        assert len(json.loads(open(out, encoding="utf-8").read())) == 3


class TestSerialPath:
    def test_serial_success_checkpoint(self, pipeline, tmp_path, monkeypatch):
        """use_parallel=False 串行成功需更新断点进度"""
        import augmentor.pipeline as pl

        src = _write_items(tmp_path, "serial_in.json", 2)
        out = str(tmp_path / "serial_out.json")

        monkeypatch.setattr(
            pl.AugmentorPipeline,
            "augment_seed",
            lambda self, item, use_quality_check=False, existing_generated=None: [
                {"instruction": item["instruction"] + "v", "input": "", "output": item["output"]}
            ],
        )
        report = pipeline.augment_dataset(
            src, out, use_parallel=False, use_checkpoint=True,
            use_quality_check=False, use_dedup=False,
        )
        assert report["parallel"] is False
        assert report["output_count"] == 2

    def test_serial_failure_updates_checkpoint(self, pipeline, tmp_path, monkeypatch):
        """串行处理抛异常需记录失败进度而非崩溃"""
        import augmentor.pipeline as pl

        src = _write_items(tmp_path, "serial_in3.json", 2)
        out = str(tmp_path / "serial_out3.json")

        def boom(self, item, use_quality_check=False, existing_generated=None):
            raise RuntimeError("模拟生成失败")

        monkeypatch.setattr(pl.AugmentorPipeline, "augment_seed", boom)
        report = pipeline.augment_dataset(
            src, out, use_parallel=False, use_checkpoint=True,
            use_quality_check=False, use_dedup=False,
        )
        assert report["output_count"] == 0


class TestParallelQualityFilter:
    def test_parallel_quality_pass_filters(self, pipeline, tmp_path, monkeypatch):
        """并行 + 质量通过时保留变体（覆盖 _process_single_item 质量分支）"""
        import augmentor.pipeline as pl
        from augmentor.quality import QualityScore

        src = _write_items(tmp_path, "par_in.json", 15)
        out = str(tmp_path / "par_out.json")

        monkeypatch.setattr(
            pl.AugmentorPipeline,
            "_generate_variants",
            lambda self, item: [
                {"instruction": item["instruction"] + "（v1）", "input": "", "output": item["output"]}
            ],
        )
        monkeypatch.setattr(
            pipeline.quality_scorer,
            "score",
            lambda original, generated, output: QualityScore(
                semantic_similarity=1.0, relevance=1.0, diversity=1.0,
                total_score=1.0, passed=True,
            ),
        )
        report = pipeline.augment_dataset(
            src, out, use_parallel=True, use_checkpoint=False,
            use_quality_check=True, use_dedup=False,
        )
        assert report["output_count"] == 15

    def test_parallel_failure_degrades(self, pipeline, tmp_path, monkeypatch):
        """并行单条生成失败需降级跳过而非中断整批"""
        import augmentor.pipeline as pl

        src = _write_items(tmp_path, "par_in2.json", 3)
        out = str(tmp_path / "par_out2.json")

        def boom(self, item):
            raise RuntimeError("生成器故障")

        monkeypatch.setattr(pl.AugmentorPipeline, "_generate_variants", boom)
        report = pipeline.augment_dataset(
            src, out, use_parallel=True, use_checkpoint=True,
            use_quality_check=False, use_dedup=False,
        )
        assert report["output_count"] == 0
        assert len(pipeline._process_errors) == 3


class TestTaskId:
    def test_build_task_id_stable(self, pipeline):
        """同一输入文件需派生稳定 task_id（断点续传依赖）"""
        a = pipeline.build_task_id("data/in.json")
        b = pipeline.build_task_id("data/in.json")
        assert a == b
        c = pipeline.build_task_id("data/other.json")
        assert a != c
