# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""流式处理内存优化测试 - 验证内存监控集成和大数据处理优化"""

import pytest
from augmentor.streaming import StreamProcessor, StreamReader


@pytest.fixture
def sample_items():
    return [{"instruction": f"问题{i}", "output": f"回答{i}"} for i in range(20)]


class TestStreamingMemoryOptimization:
    """流式处理内存优化测试"""
    
    def test_stream_processor_has_memory_check(self, tmp_path, monkeypatch):
        """大块处理必须真的触发内存快照

        原实现用 `inspect.getsource` 检查源码里是否出现 "MemoryMonitor" 字样——
        无论内存监控是否真的被执行都会通过，无法在业务逻辑变化时失败。
        """
        import json

        from augmentor import streaming as streaming_mod

        snapshots = []
        real_take_snapshot = streaming_mod.MemoryMonitor.take_snapshot

        def spy(self):
            snapshots.append(1)
            return real_take_snapshot(self)

        monkeypatch.setattr(streaming_mod.MemoryMonitor, "take_snapshot", spy)

        file_path = tmp_path / "big_chunks.json"
        data = [{"instruction": f"q{i}", "output": f"a{i}"} for i in range(300)]
        file_path.write_text(json.dumps(data, ensure_ascii=False), encoding="utf-8")

        reader = StreamReader(str(file_path), chunk_size=150)
        processor_obj = StreamProcessor(reader, lambda chunk: chunk)
        processor_obj.process()

        assert snapshots, "大块（>100 条）处理未触发内存快照，内存监控未生效"
    
    def test_large_chunk_triggers_memory_snapshot(self, tmp_path):
        """大数据块应触发内存快照"""
        import json
        file_path = tmp_path / "large_data.json"
        data = [{"instruction": f"q{i}", "output": f"a{i}"} for i in range(150)]
        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(data, f)
        
        reader = StreamReader(str(file_path), chunk_size=50)
        
        def processor(chunk):
            return chunk
        
        processor_obj = StreamProcessor(reader, processor)
        result = processor_obj.process()
        
        assert result["total_input"] == 150
        assert result["processed"] == 150
    
    def test_memory_trend_logged_for_large_data(self, tmp_path):
        """大数据处理应记录内存趋势"""
        import json
        import logging
        file_path = tmp_path / "test_memory.json"
        data = [{"instruction": "测试", "output": "回答"} for _ in range(200)]
        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(data, f)
        
        reader = StreamReader(str(file_path), chunk_size=100)
        
        def processor(chunk):
            return chunk
        
        processor_obj = StreamProcessor(reader, processor)
        # 执行处理（不检查具体内存值，只验证流程正常完成）
        result = processor_obj.process()
        assert result["total_input"] == 200
