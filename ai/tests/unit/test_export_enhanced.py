"""数据集导出增强模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.export_enhanced import (
    EnhancedExporter, ExportOptions, ExportFormat,
    export_dataset, get_supported_formats
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


class TestEnhancedExporter:
    """EnhancedExporter 测试"""
    
    def test_init(self):
        """测试初始化"""
        exporter = EnhancedExporter()
        assert len(exporter._formatters) > 0
    
    def test_export_json(self, sample_dataset, tmp_path):
        """测试导出JSON格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output.json"
        
        result = exporter.export(sample_dataset, str(output_path))
        
        assert output_path.exists()
        assert result["format"] == "json"
        assert result["item_count"] == 3
    
    def test_export_jsonl(self, sample_dataset, tmp_path):
        """测试导出JSONL格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output.jsonl"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.JSONL))
        
        assert output_path.exists()
        assert result["format"] == "jsonl"
    
    def test_export_csv(self, sample_dataset, tmp_path):
        """测试导出CSV格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output.csv"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.CSV))
        
        assert output_path.exists()
        assert result["format"] == "csv"
    
    def test_export_tsv(self, sample_dataset, tmp_path):
        """测试导出TSV格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output.tsv"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.TSV))
        
        assert output_path.exists()
        assert result["format"] == "tsv"
    
    def test_export_alpaca(self, sample_dataset, tmp_path):
        """测试导出Alpaca格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_alpaca.json"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.ALPACA))
        
        assert output_path.exists()
        assert result["format"] == "alpaca"
    
    def test_export_sharegpt(self, sample_dataset, tmp_path):
        """测试导出ShareGPT格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_sharegpt.json"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.SHAREGPT))
        
        assert output_path.exists()
        assert result["format"] == "sharegpt"
    
    def test_export_chatml(self, sample_dataset, tmp_path):
        """测试导出ChatML格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_chatml.json"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.CHATML))
        
        assert output_path.exists()
        assert result["format"] == "chatml"
    
    def test_export_with_options(self, sample_dataset, tmp_path):
        """测试带选项导出"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_options.json"
        
        options = ExportOptions(
            format=ExportFormat.JSON,
            indent=4,
            max_items=2,
            shuffle=True,
            seed=42
        )
        
        result = exporter.export(sample_dataset, str(output_path), options)
        
        assert output_path.exists()
        assert result["item_count"] == 2
    
    def test_export_with_field_filter(self, sample_dataset, tmp_path):
        """测试字段过滤导出"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_filter.json"
        
        options = ExportOptions(
            format=ExportFormat.JSON,
            include_fields=["instruction", "output"]
        )
        
        result = exporter.export(sample_dataset, str(output_path), options)
        
        assert output_path.exists()
        
        with open(output_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        for item in data:
            assert "instruction" in item
            assert "output" in item
            assert "input" not in item
    
    def test_export_with_exclude_fields(self, sample_dataset, tmp_path):
        """测试排除字段导出"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_exclude.json"
        
        options = ExportOptions(
            format=ExportFormat.JSON,
            exclude_fields=["input"]
        )
        
        result = exporter.export(sample_dataset, str(output_path), options)
        
        assert output_path.exists()
        
        with open(output_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        for item in data:
            assert "instruction" in item
            assert "output" in item
            assert "input" not in item
    
    def test_export_empty_dataset(self, tmp_path):
        """测试导出空数据集"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_empty.json"
        
        result = exporter.export([], str(output_path))
        
        assert output_path.exists()
        assert result["item_count"] == 0
    
    def test_export_llama_factory(self, sample_dataset, tmp_path):
        """测试导出Llama-Factory格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_llama.json"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.LLAMA_FACTORY))
        
        assert output_path.exists()
        assert result["format"] == "llama_factory"
    
    def test_export_vicuna(self, sample_dataset, tmp_path):
        """测试导出Vicuna格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_vicuna.json"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.VICUNA))
        
        assert output_path.exists()
        assert result["format"] == "vicuna"
    
    def test_export_belle(self, sample_dataset, tmp_path):
        """测试导出BELLE格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_belle.json"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.BELLE))
        
        assert output_path.exists()
        assert result["format"] == "belle"
    
    def test_export_openai(self, sample_dataset, tmp_path):
        """测试导出OpenAI格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_openai.json"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.OPENAI))
        
        assert output_path.exists()
        assert result["format"] == "openai"
    
    def test_export_huggingface(self, sample_dataset, tmp_path):
        """测试导出HuggingFace格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_hf.json"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.HUGGINGFACE))
        
        assert output_path.exists()
        assert result["format"] == "huggingface"
    
    def test_export_raw(self, sample_dataset, tmp_path):
        """测试导出Raw格式"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output_raw.json"
        
        result = exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.RAW))
        
        assert output_path.exists()
        assert result["format"] == "raw"
    
    def test_export_creates_parent_directories(self, sample_dataset, tmp_path):
        """测试导出时创建父目录"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "subdir" / "output.json"
        
        result = exporter.export(sample_dataset, str(output_path))
        
        assert output_path.exists()
    
    def test_export_json_content(self, sample_dataset, tmp_path):
        """测试导出JSON内容正确"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output.json"
        
        exporter.export(sample_dataset, str(output_path))
        
        with open(output_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        assert len(data) == 3
        assert data[0]["instruction"] == "如何申请租房？"
    
    def test_export_jsonl_content(self, sample_dataset, tmp_path):
        """测试导出JSONL内容正确"""
        exporter = EnhancedExporter()
        output_path = tmp_path / "output.jsonl"
        
        exporter.export(sample_dataset, str(output_path), ExportOptions(format=ExportFormat.JSONL))
        
        with open(output_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        assert len(lines) == 3
        for line in lines:
            item = json.loads(line)
            assert "instruction" in item


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_export_dataset(self, sample_dataset, tmp_path):
        """测试导出数据集"""
        output_path = tmp_path / "output.json"
        
        result = export_dataset(sample_dataset, str(output_path))
        
        assert output_path.exists()
        assert result["item_count"] == 3
    
    def test_get_supported_formats(self):
        """测试获取支持的格式"""
        formats = get_supported_formats()
        
        assert isinstance(formats, list)
        assert "json" in formats
        assert "jsonl" in formats
        assert "alpaca" in formats
        assert "sharegpt" in formats


class TestExportOptions:
    """ExportOptions 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        options = ExportOptions(
            format=ExportFormat.JSON,
            indent=4,
            max_items=100
        )
        
        d = options.to_dict()
        
        assert d["format"] == "json"
        assert d["indent"] == 4
        assert d["max_items"] == 100
    
    def test_default_values(self):
        """测试默认值"""
        options = ExportOptions()
        
        assert options.format == ExportFormat.JSON
        assert options.indent == 2
        assert options.ensure_ascii is False
        assert options.include_fields is None
        assert options.exclude_fields is None
        assert options.max_items is None
        assert options.shuffle is False
        assert options.seed == 42


class TestExportFormat:
    """ExportFormat 测试"""
    
    def test_all_formats(self):
        """测试所有格式"""
        formats = list(ExportFormat)
        assert len(formats) == 13
    
    def test_format_values(self):
        """测试格式值"""
        assert ExportFormat.JSON.value == "json"
        assert ExportFormat.JSONL.value == "jsonl"
        assert ExportFormat.CSV.value == "csv"
        assert ExportFormat.TSV.value == "tsv"
        assert ExportFormat.ALPACA.value == "alpaca"
        assert ExportFormat.SHAREGPT.value == "sharegpt"
        assert ExportFormat.CHATML.value == "chatml"
        assert ExportFormat.LLAMA_FACTORY.value == "llama_factory"
        assert ExportFormat.VICUNA.value == "vicuna"
        assert ExportFormat.BELLE.value == "belle"
        assert ExportFormat.OPENAI.value == "openai"
        assert ExportFormat.HUGGINGFACE.value == "huggingface"
        assert ExportFormat.RAW.value == "raw"
