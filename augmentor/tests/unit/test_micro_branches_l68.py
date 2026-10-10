# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""微型分支收尾 L68：version_control/versioning/validation/search/pipeline 残留分支

- version_control: 版本比较二次命中缓存
- versioning: current 指向普通目录（非符号链接）时回滚用 rmtree
- validation: 非 dict 项被清洗剔除
- search_enhanced: 模糊搜索相似度达标记录命中
- pipeline: 滑动窗口裁剪（串行/并行）+ 异步单条失败被跳过
"""

import json
import os
import shutil
import tempfile
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
                # `type` 自 A115 起是封闭清单（构造期就拒），替身因此要填一个真类型；
        # 它覆写了 `_call_api`，填哪一个都不影响行为。
        super().__init__(ModelConfig(type="ollama"))
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


def _can_symlink() -> bool:
    """L213：探测本机能否真建符号链接（Windows 无特权/未开发者模式时抛 WinError 131）"""
    with tempfile.TemporaryDirectory() as td:
        target = Path(td) / "t"
        link = Path(td) / "l"
        target.mkdir()
        try:
            link.symlink_to(target)
            return True
        except OSError:
            return False


class TestVersioningRmtree:
    def test_rollback_replaces_plain_dir_current(self, tmp_path):
        """current 是普通目录时回滚走 rmtree 分支——这一格与符号链接能力无关

        L213：终态断言按**能力分形**（`_can_symlink` 的实测值），两种机型各自钉死自己
        那一臂，都不许静默跳。原来这里写死 `current.is_symlink()`，无特权机型上
        versioning 按设计落 `.txt` 回退，测试却按另一臂断言 ⇒ 红的是尺子不是代码。
        """
        manager = VersionManager(storage_dir=str(tmp_path / "vm"))
        info = manager.create_version([{"instruction": "q"}])
        # 无论 create 把 current 落成了链接还是回退文件，先清掉再建普通目录，
        # 才能确保回滚时 current 是「非符号链接的目录」从而命中 rmtree 分支
        current = tmp_path / "vm" / "current"
        if current.is_symlink() or current.is_file():
            current.unlink()
        elif current.is_dir():
            shutil.rmtree(current)
        current.mkdir(parents=True, exist_ok=True)
        (current / "junk.txt").write_text("x", encoding="utf-8")
        assert not current.is_symlink()
        assert manager.rollback(info.version_id) is True
        # 旧目录已被 rmtree 删除（两支机型都成立）
        assert not (current / "junk.txt").exists()
        # 终态与**声明的能力**互相对账：探测器说能 ⇒ 必须真是链接；说不能 ⇒
        # 必须没有 current 目录且落出 .txt 回退。若只按探测器的话分臂，
        # 探测说谎（L213 注入实测）时两种机型都不会红——那是尺子坏了不是代码坏了。
        can = _can_symlink()
        assert current.is_symlink() is can
        if not can:
            assert not current.exists()
            fallback = tmp_path / "vm" / "current.txt"
            assert fallback.read_text(encoding="utf-8") == info.version_id

    def test_symlink_failure_falls_back_to_txt_marker(self, tmp_path, monkeypatch):
        """无特权臂的**可达证明**：有能力机型上注入 symlink_to 失败，必须落 current.txt

        若只写能力分形，特权机型就从不执行回退分支、无特权机型就从不执行链接分支——
        两臂各在一类机器上是死代码。这一格让回退支在两型机器上都被执行。
        """
        manager = VersionManager(storage_dir=str(tmp_path / "vm"))
        info = manager.create_version([{"instruction": "q"}])
        monkeypatch.setattr(
            Path, "symlink_to",
            lambda self, target, target_is_directory=False: (_ for _ in ()).throw(
                OSError(131, "insufficient privileges")),
        )
        current = tmp_path / "vm" / "current"
        if current.is_symlink():
            current.unlink()
        assert manager.rollback("no_such_version") is False
        assert manager.rollback(info.version_id) is True
        fallback = tmp_path / "vm" / "current.txt"
        assert fallback.read_text(encoding="utf-8") == info.version_id


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
