"""数据集可视化增强模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.visualize_enhanced import (
    EnhancedVisualizer, VisualizationConfig, visualize_dataset
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


class TestEnhancedVisualizer:
    """EnhancedVisualizer 测试"""
    
    def test_init(self):
        """测试初始化"""
        visualizer = EnhancedVisualizer()
        assert visualizer._config is not None
    
    def test_init_with_config(self):
        """测试带配置初始化"""
        config = VisualizationConfig(width=1024, height=768)
        visualizer = EnhancedVisualizer(config)
        assert visualizer._config.width == 1024
    
    def test_generate_text_report(self, sample_dataset):
        """测试生成文本报告"""
        visualizer = EnhancedVisualizer()
        report = visualizer.generate_text_report(sample_dataset)
        
        assert isinstance(report, str)
        assert "数据集可视化报告" in report
        assert "数据总量" in report
    
    def test_generate_json_report(self, sample_dataset):
        """测试生成JSON报告"""
        visualizer = EnhancedVisualizer()
        report = visualizer.generate_json_report(sample_dataset)
        
        assert isinstance(report, dict)
        assert "total_items" in report
        assert "fields" in report
        assert report["total_items"] == 3
    
    def test_save_text_report(self, sample_dataset, tmp_path):
        """测试保存文本报告"""
        visualizer = EnhancedVisualizer()
        output_path = tmp_path / "report.txt"
        
        visualizer.save_text_report(sample_dataset, str(output_path))
        
        assert output_path.exists()
        
        with open(output_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        assert "数据集可视化报告" in content
    
    def test_save_json_report(self, sample_dataset, tmp_path):
        """测试保存JSON报告"""
        visualizer = EnhancedVisualizer()
        output_path = tmp_path / "report.json"
        
        visualizer.save_json_report(sample_dataset, str(output_path))
        
        assert output_path.exists()
        
        with open(output_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        assert data["total_items"] == 3


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_visualize_dataset_text(self, sample_dataset):
        """测试可视化数据集文本格式"""
        result = visualize_dataset(sample_dataset, format="text")
        
        assert isinstance(result, str)
        assert "数据集可视化报告" in result
    
    def test_visualize_dataset_json(self, sample_dataset):
        """测试可视化数据集JSON格式"""
        result = visualize_dataset(sample_dataset, format="json")
        
        assert isinstance(result, str)
        data = json.loads(result)
        assert "total_items" in data
    
    def test_visualize_dataset_save(self, sample_dataset, tmp_path):
        """测试保存可视化结果"""
        output_path = tmp_path / "visualize.txt"
        
        result = visualize_dataset(sample_dataset, str(output_path), format="text")
        
        assert output_path.exists()
        assert isinstance(result, str)


class TestVisualizationConfig:
    """VisualizationConfig 测试"""
    
    def test_default_values(self):
        """测试默认值"""
        config = VisualizationConfig()
        
        assert config.width == 800
        assert config.height == 600
        assert config.theme == "default"
        assert len(config.color_palette) > 0
    
    def test_custom_values(self):
        """测试自定义值"""
        config = VisualizationConfig(
            width=1024,
            height=768,
            theme="dark",
            color_palette=["#ff0000", "#00ff00"]
        )
        
        assert config.width == 1024
        assert config.height == 768
        assert config.theme == "dark"
        assert len(config.color_palette) == 2
