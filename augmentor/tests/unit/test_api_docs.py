"""核心模块API文档测试 - 验证文档完整性和可访问性"""

import pytest
from pathlib import Path

# 以测试文件位置为锚点解析文档路径，避免依赖 pytest 的当前工作目录
# （CI 中 working-directory 为 augmentor/，而本地可能从仓库根运行）。
AI_DIR = Path(__file__).resolve().parent.parent.parent


class TestCoreAPIDocumentation:
    """核心模块API文档测试"""
    
    def test_api_docs_file_exists(self):
        """API文档文件应存在"""
        docs_path = AI_DIR / "docs" / "API_CORE.md"
        assert docs_path.exists(), "核心模块API文档文件缺失"
    
    def test_api_docs_contains_key_modules(self):
        """API文档应包含关键模块说明"""
        docs_path = AI_DIR / "docs" / "API_CORE.md"
        content = docs_path.read_text(encoding='utf-8')
        
        # 验证包含主要模块名称
        key_modules = [
            "AugmentorPipeline",
            "QualityScorer",
            "MemoryMonitor",
            "StreamProcessor",
            "ContextAugmentor"
        ]
        for module in key_modules:
            assert module in content, f"API文档缺少模块说明: {module}"
    
    def test_api_docs_contains_method_descriptions(self):
        """API文档应包含方法描述"""
        docs_path = AI_DIR / "docs" / "API_CORE.md"
        content = docs_path.read_text(encoding='utf-8')
        
        # 验证包含关键方法名称
        key_methods = [
            "augment_dataset",
            "augment_async",
            "score",
            "batch_score",
            "take_snapshot",
            "process"
        ]
        for method in key_methods:
            assert method in content, f"API文档缺少方法说明: {method}"
    
    def test_usage_examples_file_exists(self):
        """使用示例文档应存在"""
        docs_path = AI_DIR / "docs" / "USAGE_EXAMPLES.md"
        assert docs_path.exists(), "使用示例文档文件缺失"
    
    def test_usage_examples_contains_code_samples(self):
        """使用示例应包含代码示例"""
        docs_path = AI_DIR / "docs" / "USAGE_EXAMPLES.md"
        content = docs_path.read_text(encoding='utf-8')
        assert "```python" in content, "使用示例缺少Python代码块"
        assert "from augmentor" in content, "使用示例缺少导入示例"
