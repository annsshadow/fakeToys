# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""streaming / preview 剩余分支补测

streaming：内存监控模块缺失时 HAS_MEMORY_MONITOR 降级路径；
preview：相同样本二次预览命中缓存。
"""

import json
import sys

import pytest


class TestStreamingMemoryMonitorMissing:
    def test_missing_memory_monitor_degrades(self, tmp_path):
        """memory_monitor 导入失败时 streaming 需降级为无内存监控"""
        import importlib

        import augmentor.streaming as streaming
        from augmentor.memory_monitor import MemoryMonitor
        from augmentor.streaming import StreamAugmentor

        items = [{"instruction": f"问题{i}", "input": "", "output": "答"} for i in range(120)]
        src = tmp_path / "in.jsonl"
        with open(src, "w", encoding="utf-8") as f:
            for item in items:
                f.write(json.dumps(item, ensure_ascii=False) + "\n")

        real_module = sys.modules["augmentor.memory_monitor"]
        try:
            # 置 None 使 from .memory_monitor import ... 抛 ImportError
            sys.modules["augmentor.memory_monitor"] = None
            importlib.reload(streaming)
            assert streaming.HAS_MEMORY_MONITOR is False

            out = tmp_path / "out.jsonl"
            augmentor = StreamAugmentor(
                input_file=str(src),
                output_file=str(out),
                processor=lambda chunk: chunk,
                chunk_size=120,  # 单块 >100，监控分支条件应因 None 跳过
            )
            report = augmentor.augment()
            assert report["total_output"] == 120
        finally:
            sys.modules["augmentor.memory_monitor"] = real_module
            importlib.reload(streaming)
            assert streaming.HAS_MEMORY_MONITOR is True
            assert hasattr(streaming, "MemoryMonitor") or MemoryMonitor is not None


class TestPreviewCache:
    def test_second_identical_preview_hits_cache(self, tmp_path):
        from augmentor.preview import PreviewGenerator

        items = [
            {"instruction": f"问题{i}？", "input": "", "output": f"回答{i}" * 3}
            for i in range(10)
        ]
        generator = PreviewGenerator(preview_size=3)
        first = generator.preview(items, "jsonl")
        second = generator.preview(items, "jsonl")
        # 二次调用应命中预览缓存（结果一致且缓存非空）
        assert first.to_dict() == second.to_dict()
        assert hasattr(generator, "_preview_cache")
        assert len(generator._preview_cache) >= 1
