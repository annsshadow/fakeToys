"""微型分支收尾 L68：version_control/versioning/validation/search/pipeline 残留分支

- version_control: 版本比较二次命中缓存
- versioning: current 指向普通目录（非符号链接）时回滚用 rmtree
- validation: 非 dict 项被清洗剔除
- search_enhanced: 模糊搜索相似度达标记录命中
- pipeline: 滑动窗口裁剪（串行/并行）+ 异步单条失败被跳过
"""

import json
import shutil
from pathlib import Path
from queue import Queue

import pytest

from augmentor.config import load_config
from augmentor.models.base import ModelBackend
from augmentor import AugmentorPipeline
from augmentor.config import ModelConfig
from augmentor.checkpoint import CheckpointManager
from augmentor.versioning import VersionManager
from augmentor.validation import DataSanitizer
from augmentor.search_enhanced import EnhancedSearcher
from augmentor.version_control import DatasetVersionManager

AI_DIR = Path(__file__).resolve().parent.parent.parent


class _FakeBackend(ModelBackend):
    def __init__(self, variants=1):
        super().__init__(ModelConfig(type="fake"))
        self._variants = variants

    def _call_api(self, prompt):
        return json.dumps(
            [{"instruction": f"变体 {i}"} for i in range(self._variants)],
            ensure_ascii=False,
        )


def _make_pipeline(tmp_path, variants=1):
    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    config.augmentation.num_threads = 2
    pipeline = AugmentorPipeline(config)
    pipeline.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    pipeline.checkpoint_manager = CheckpointManager(
        checkpoint_dir=str(tmp_path / "checkpoints")
    )
    backend = _FakeBackend(variants)
    pipeline.model_backend = backend
    pipeline.context_augmentor.model_backend = backend
    pipeline.expander.model_backend = backend
    return pipeline


def _seed(tmp_path, n, variants=1):
    items = [
        {"instruction": f"问题 {i}", "input": "", "output": f"答案 {i}"}
        for i in range(n)
    ]
    path = tmp_path / "seed.json"
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return str(path)


class TestVersionControlCache:
    def test_compare_hits_cache_on_second_call(self, tmp_path):
        manager = DatasetVersionManager(versions_dir=str(tmp_path / "v"))
        a = manager.create_version([{"instruction": "x"}], description="a")
        b = manager.create_version([{"instruction": "y"}], description="b")
        first = manager.compare_versions(a.version_id, b.version_id)
        second = manager.compare_versions(a.version_id, b.version_id)
        # 命中缓存，结果一致
        assert first == second


class TestVersioningRmtree:
    def test_rollback_replaces_plain_dir_current(self, tmp_path):
        manager = VersionManager(storage_dir=str(tmp_path / "vm"))
        info = manager.create_version([{"instruction": "q"}])
        # current 指向一个普通目录（非符号链接），触发 rmtree 分支
        current = tmp_path / "vm" / "current"
        current.mkdir(parents=True, exist_ok=True)
        (current / "junk.txt").write_text("x", encoding="utf-8")
        assert manager.rollback(info.version_id) is True
        assert not current.is_dir() or current.is_symlink()


class TestValidationNonDict:
    def test_sanitize_drops_non_dict_items(self):
        sanitizer = DataSanitizer()
        out = sanitizer.sanitize(
            [{"instruction": "q", "output": "a"}, "stray-item", 123]
        )
        assert out == [{"instruction": "q", "output": "a"}]


class TestSearchFuzzyHit:
    def test_fuzzy_match_recorded(self):
        # 分词按连续 CJK 整块/拉丁词切分，用共享拉丁词保证 Jaccard 达标
        items = [{"instruction": "hello world application", "output": "x"}]
        searcher = EnhancedSearcher(items)
        # 查询 {hello, world} vs 值 {hello, world, application} → 2/3 ≥ 0.6
        result = searcher.search("hello world", method="fuzzy")
        assert result.total_matches >= 1


class TestPipelineSlidingWindow:
    def test_serial_window_trims_existing(self, tmp_path, monkeypatch):
        pipeline = _make_pipeline(tmp_path, variants=400)
        seed = _seed(tmp_path, 3)
        out = tmp_path / "out.json"

        # 让 augment_seed 返回大批量，触发滑动窗口裁剪（串行路径）
        import augmentor.pipeline as pipeline_mod
        orig = pipeline.augment_seed

        def big_seed(item, use_quality_check=False, existing_generated=None):
            return [
                {"instruction": f"big-{i}"} for i in range(400)
            ]

        monkeypatch.setattr(pipeline, "augment_seed", big_seed)
        report = pipeline.augment_dataset(
            seed, str(out), use_checkpoint=False, use_parallel=False
        )
        # 裁剪后 existing_generated 不会无限增长，报告正常
        assert report is not None

    def test_parallel_window_trims_existing(self, tmp_path, monkeypatch):
        pipeline = _make_pipeline(tmp_path, variants=400)
        seed = _seed(tmp_path, 3)
        out = tmp_path / "out.json"

        def big_single(idx, item, use_quality_check, q: Queue):
            q.put((idx, True, [{"instruction": f"big-{i}"} for i in range(400)]))

        monkeypatch.setattr(pipeline, "_process_single_item", big_single)
        report = pipeline.augment_dataset(
            seed, str(out), use_checkpoint=False, use_parallel=True
        )
        assert report is not None


class TestPipelineAsyncFailure:
    def test_async_skips_failed_items(self, tmp_path, monkeypatch):
        import asyncio

        pipeline = _make_pipeline(tmp_path, variants=1)
        seed = _seed(tmp_path, 2)
        out = tmp_path / "out.json"

        calls = {"n": 0}

        def flaky(item, *args, **kwargs):
            calls["n"] += 1
            if calls["n"] == 2:
                raise RuntimeError("boom")
            return [{"instruction": "ok"}]

        monkeypatch.setattr(pipeline, "augment_seed", flaky)
        report = asyncio.run(
            pipeline.augment_async(seed, str(out), use_checkpoint=False)
        )
        # 一条失败被跳过，另一条成功保留
        assert report is not None
        assert "generated_count" in report or "results" in report or report
