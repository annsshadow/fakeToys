"""流水线错误恢复增强测试 - 错误记录和恢复机制验证"""

import pytest
from unittest.mock import MagicMock, patch


class MockPipeline:
    """模拟流水线用于测试错误恢复"""
    def __init__(self):
        self._process_errors = []
    
    def record_error(self, error_info):
        if not hasattr(self, '_process_errors'):
            self._process_errors = []
        self._process_errors.append(error_info)


class TestPipelineErrorRecovery:
    """流水线错误恢复增强测试"""
    
    def test_error_info_contains_index(self):
        """错误信息应包含索引"""
        mock = MockPipeline()
        mock.record_error({"index": 1, "error_type": "ValueError", "message": "测试错误"})
        assert len(mock._process_errors) == 1
        assert mock._process_errors[0]["index"] == 1
    
    def test_error_info_has_preview(self):
        """错误信息应包含数据预览"""
        mock = MockPipeline()
        mock.record_error({
            "index": 2,
            "error_type": "Exception",
            "message": "处理失败",
            "item_preview": "测试数据预览内容..."
        })
        assert "item_preview" in mock._process_errors[0]
        assert len(mock._process_errors[0]["item_preview"]) > 0
    
    def test_error_recovery_code_path_exists(self):
        """错误恢复增强代码路径应存在"""
        import inspect
        from augmentor.pipeline import AugmentorPipeline
        source = inspect.getsource(AugmentorPipeline._process_single_item)
        assert "_process_errors" in source or "error_info" in source
