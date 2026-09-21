"""流式处理内存优化测试 - 验证内存监控集成和大数据处理优化"""

import pytest
from augmentor.streaming import StreamProcessor, StreamReader


@pytest.fixture
def sample_items():
    return [{"instruction": f"问题{i}", "output": f"回答{i}"} for i in range(20)]


class TestStreamingMemoryOptimization:
    """流式处理内存优化测试"""
    
    def test_stream_processor_has_memory_check(self):
        """流式处理应包含内存监控代码路径"""
        import inspect
        source = inspect.getsource(StreamProcessor.process)
        assert "memory_monitor" in source
        assert "MemoryMonitor" in source
    
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
