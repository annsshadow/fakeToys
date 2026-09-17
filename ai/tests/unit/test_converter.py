"""数据集格式转换模块测试"""

import csv
import json
import pytest
from pathlib import Path
from augmentor.converter import (
    DatasetConverter, DataFormat, convert_dataset,
    convert_file, get_supported_formats
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


@pytest.fixture
def sample_with_history():
    """创建带历史记录的测试数据集"""
    return [
        {
            "instruction": "继续之前的问题",
            "input": "",
            "output": "好的，我来回答",
            "history": [
                {"role": "user", "content": "问题1"},
                {"role": "assistant", "content": "回答1"}
            ]
        }
    ]


@pytest.fixture
def temp_json_file(tmp_path, sample_dataset):
    """创建临时JSON文件"""
    file_path = tmp_path / "dataset.json"
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(sample_dataset, f, ensure_ascii=False)
    return str(file_path)


@pytest.fixture
def temp_jsonl_file(tmp_path, sample_dataset):
    """创建临时JSONL文件"""
    file_path = tmp_path / "dataset.jsonl"
    with open(file_path, 'w', encoding='utf-8') as f:
        for item in sample_dataset:
            f.write(json.dumps(item, ensure_ascii=False) + '\n')
    return str(file_path)


class TestDatasetConverter:
    """DatasetConverter 测试"""
    
    def test_init(self):
        """测试初始化"""
        converter = DatasetConverter()
        assert converter._converters is not None
    
    def test_convert_same_format(self, sample_dataset):
        """测试相同格式转换"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "json")
        
        assert result == sample_dataset
    
    def test_convert_json_to_jsonl(self, sample_dataset):
        """测试 JSON 转 JSONL"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "jsonl")
        
        assert isinstance(result, list)
        assert len(result) == 3
        # 每行应该是有效的JSON
        for line in result:
            parsed = json.loads(line)
            assert "instruction" in parsed
    
    def test_convert_jsonl_to_json(self, sample_dataset):
        """测试 JSONL 转 JSON"""
        # 先转换为JSONL
        jsonl_data = [json.dumps(item, ensure_ascii=False) for item in sample_dataset]
        
        converter = DatasetConverter()
        result = converter.convert(jsonl_data, "jsonl", "json")
        
        assert isinstance(result, list)
        assert len(result) == 3
        assert result[0]["instruction"] == sample_dataset[0]["instruction"]
    
    def test_convert_json_to_alpaca(self, sample_dataset):
        """测试 JSON 转 Alpaca"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "alpaca")
        
        assert isinstance(result, list)
        assert len(result) == 3
        for item in result:
            assert "instruction" in item
            assert "input" in item
            assert "output" in item
    
    def test_convert_json_to_sharegpt(self, sample_dataset):
        """测试 JSON 转 ShareGPT"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "sharegpt")
        
        assert isinstance(result, list)
        assert len(result) == 3
        for item in result:
            assert "conversations" in item
            assert len(item["conversations"]) >= 2
    
    def test_convert_json_to_chatml(self, sample_dataset):
        """测试 JSON 转 ChatML"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "chatml")
        
        assert isinstance(result, list)
        assert len(result) == 3
        for item in result:
            assert "messages" in item
            # 应该有 user 和 assistant 消息
            roles = [m["role"] for m in item["messages"]]
            assert "user" in roles
            assert "assistant" in roles
    
    def test_convert_json_to_vicuna(self, sample_dataset):
        """测试 JSON 转 Vicuna"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "vicuna")
        
        assert isinstance(result, list)
        assert len(result) == 3
        for item in result:
            assert "conversations" in item
            assert len(item["conversations"]) == 2
    
    def test_convert_with_history(self, sample_with_history):
        """测试带历史记录的转换"""
        converter = DatasetConverter()
        result = converter.convert(sample_with_history, "json", "sharegpt")
        
        assert isinstance(result, list)
        assert len(result) == 1
        # 应该包含历史记录
        assert len(result[0]["conversations"]) > 2
    
    def test_convert_json_to_csv(self, sample_dataset):
        """测试 JSON 转 CSV"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "csv")
        
        assert isinstance(result, list)
        assert len(result) == 3
    
    def test_convert_csv_to_json(self, sample_dataset):
        """测试 CSV 转 JSON"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "csv", "json")
        
        assert isinstance(result, list)
        assert len(result) == 3
    
    def test_convert_json_to_llama_factory(self, sample_dataset):
        """测试 JSON 转 Llama-Factory"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "llama_factory")
        
        assert isinstance(result, list)
        assert len(result) == 3
        for item in result:
            assert "instruction" in item
            assert "input" in item
            assert "output" in item
    
    def test_convert_json_to_belle(self, sample_dataset):
        """测试 JSON 转 BELLE"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "belle")
        
        assert isinstance(result, list)
        assert len(result) == 3
        for item in result:
            assert "instruction" in item
            assert "output" in item
    
    def test_normalize_format(self):
        """测试格式标准化"""
        converter = DatasetConverter()
        
        assert converter._normalize_format("JSON") == "json"
        assert converter._normalize_format("Json") == "json"
        assert converter._normalize_format("json") == "json"
        assert converter._normalize_format("unknown") == "unknown"
    
    def test_convert_with_system_message(self):
        """测试带系统消息的转换"""
        data = [
            {
                "instruction": "问题",
                "output": "回答",
                "system": "你是一个助手"
            }
        ]
        converter = DatasetConverter()
        result = converter.convert(data, "json", "chatml")
        
        assert len(result) == 1
        assert result[0]["messages"][0]["role"] == "system"


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_convert_dataset(self, sample_dataset):
        """测试转换数据集"""
        result = convert_dataset(sample_dataset, "alpaca")
        
        assert isinstance(result, list)
        assert len(result) == 3
    
    def test_convert_file_json_to_jsonl(self, tmp_path, sample_dataset):
        """测试转换文件 JSON 到 JSONL"""
        input_file = tmp_path / "input.json"
        output_file = tmp_path / "output.jsonl"
        
        with open(input_file, 'w', encoding='utf-8') as f:
            json.dump(sample_dataset, f, ensure_ascii=False)
        
        result = convert_file(str(input_file), str(output_file))
        
        assert result["source_format"] == "json"
        assert result["target_format"] == "jsonl"
        assert output_file.exists()
    
    def test_convert_file_jsonl_to_json(self, tmp_path, sample_dataset):
        """测试转换文件 JSONL 到 JSON"""
        input_file = tmp_path / "input.jsonl"
        output_file = tmp_path / "output.json"
        
        with open(input_file, 'w', encoding='utf-8') as f:
            for item in sample_dataset:
                f.write(json.dumps(item, ensure_ascii=False) + '\n')
        
        result = convert_file(str(input_file), str(output_file))
        
        assert result["source_format"] == "jsonl"
        assert result["target_format"] == "json"
        assert output_file.exists()
    
    def test_get_supported_formats(self):
        """测试获取支持的格式"""
        formats = get_supported_formats()
        
        assert isinstance(formats, list)
        assert "json" in formats
        assert "jsonl" in formats
        assert "alpaca" in formats
        assert "sharegpt" in formats
        assert "chatml" in formats


class TestConverterEdgeCases:
    """边界情况测试"""
    
    def test_convert_empty_dataset(self):
        """测试转换空数据集"""
        converter = DatasetConverter()
        result = converter.convert([], "json", "jsonl")
        
        assert isinstance(result, list)
        assert len(result) == 0
    
    def test_convert_single_item(self):
        """测试转换单条数据"""
        data = [{"instruction": "问题", "input": "", "output": "回答"}]
        converter = DatasetConverter()
        result = converter.convert(data, "json", "alpaca")
        
        assert len(result) == 1
    
    def test_unsupported_conversion(self):
        """测试不支持的转换"""
        converter = DatasetConverter()
        
        with pytest.raises(ValueError):
            converter.convert([], "unknown", "json")


class TestDataFormat:
    """DataFormat 测试"""
    
    def test_all_formats(self):
        """测试所有格式"""
        formats = list(DataFormat)
        assert len(formats) == 10
    
    def test_format_values(self):
        """测试格式值"""
        assert DataFormat.JSON.value == "json"
        assert DataFormat.JSONL.value == "jsonl"
        assert DataFormat.CSV.value == "csv"
        assert DataFormat.TSV.value == "tsv"
        assert DataFormat.ALPACA.value == "alpaca"
        assert DataFormat.SHAREGPT.value == "sharegpt"
        assert DataFormat.CHATML.value == "chatml"
        assert DataFormat.LLAMA_FACTORY.value == "llama_factory"
        assert DataFormat.VICUNA.value == "vicuna"
        assert DataFormat.BELLE.value == "belle"


class TestConverterFileOperations:
    """文件操作测试"""
    
    def test_infer_format(self):
        """测试格式推断"""
        converter = DatasetConverter()
        
        assert converter._infer_format(Path("test.json")) == "json"
        assert converter._infer_format(Path("test.jsonl")) == "jsonl"
        assert converter._infer_format(Path("test.csv")) == "csv"
        assert converter._infer_format(Path("test.tsv")) == "tsv"
        assert converter._infer_format(Path("test.unknown")) == "json"
    
    def test_convert_file_creates_output_directory(self, tmp_path, sample_dataset):
        """测试转换文件时创建输出目录"""
        input_file = tmp_path / "input.json"
        output_file = tmp_path / "subdir" / "output.jsonl"
        
        with open(input_file, 'w', encoding='utf-8') as f:
            json.dump(sample_dataset, f, ensure_ascii=False)
        
        result = convert_file(str(input_file), str(output_file))
        
        assert output_file.exists()

    def test_read_file_jsonl(self, tmp_path, sample_dataset):
        """测试读取 JSONL 文件"""
        input_file = tmp_path / "input.jsonl"
        with open(input_file, 'w', encoding='utf-8') as f:
            for item in sample_dataset:
                f.write(json.dumps(item, ensure_ascii=False) + '\n')
        
        converter = DatasetConverter()
        data = converter._read_file(input_file, "jsonl")
        
        assert len(data) == 3
        assert data[0]["instruction"] == sample_dataset[0]["instruction"]

    def test_read_file_csv(self, tmp_path, sample_dataset):
        """测试读取 CSV 文件"""
        input_file = tmp_path / "input.csv"
        with open(input_file, 'w', encoding='utf-8', newline='') as f:
            writer = csv.DictWriter(f, fieldnames=sample_dataset[0].keys())
            writer.writeheader()
            writer.writerows(sample_dataset)
        
        converter = DatasetConverter()
        data = converter._read_file(input_file, "csv")
        
        assert len(data) == 3

    def test_read_file_json(self, tmp_path, sample_dataset):
        """测试读取 JSON 文件"""
        input_file = tmp_path / "input.json"
        with open(input_file, 'w', encoding='utf-8') as f:
            json.dump(sample_dataset, f, ensure_ascii=False)
        
        converter = DatasetConverter()
        data = converter._read_file(input_file, "json")
        
        assert len(data) == 3

    def test_write_file_jsonl(self, tmp_path, sample_dataset):
        """测试写入 JSONL 文件"""
        output_file = tmp_path / "output.jsonl"
        converter = DatasetConverter()
        converter._write_file(output_file, sample_dataset, "jsonl")
        
        assert output_file.exists()
        with open(output_file, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        assert len(lines) == 3

    def test_write_file_csv(self, tmp_path, sample_dataset):
        """测试写入 CSV 文件"""
        output_file = tmp_path / "output.csv"
        converter = DatasetConverter()
        converter._write_file(output_file, sample_dataset, "csv")
        
        assert output_file.exists()

    def test_write_file_json(self, tmp_path, sample_dataset):
        """测试写入 JSON 文件"""
        output_file = tmp_path / "output.json"
        converter = DatasetConverter()
        converter._write_file(output_file, sample_dataset, "json")
        
        assert output_file.exists()

    def test_convert_file_json_to_csv(self, tmp_path, sample_dataset):
        """测试文件转换 JSON 到 CSV"""
        input_file = tmp_path / "input.json"
        output_file = tmp_path / "output.csv"
        
        with open(input_file, 'w', encoding='utf-8') as f:
            json.dump(sample_dataset, f, ensure_ascii=False)
        
        result = convert_file(str(input_file), str(output_file))
        
        assert output_file.exists()
        assert result["source_format"] == "json"
        assert result["target_format"] == "csv"

    def test_convert_file_csv_to_json(self, tmp_path, sample_dataset):
        """测试文件转换 CSV 到 JSON"""
        input_file = tmp_path / "input.csv"
        output_file = tmp_path / "output.json"
        
        with open(input_file, 'w', encoding='utf-8', newline='') as f:
            writer = csv.DictWriter(f, fieldnames=sample_dataset[0].keys())
            writer.writeheader()
            writer.writerows(sample_dataset)
        
        result = convert_file(str(input_file), str(output_file))
        
        assert output_file.exists()

    def test_convert_file_with_explicit_format(self, tmp_path, sample_dataset):
        """测试显式指定格式的文件转换"""
        input_file = tmp_path / "input.dat"
        output_file = tmp_path / "output.dat"
        
        with open(input_file, 'w', encoding='utf-8') as f:
            json.dump(sample_dataset, f, ensure_ascii=False)
        
        result = convert_file(str(input_file), str(output_file), 
                             source_format="json", target_format="jsonl")
        
        assert result["source_format"] == "json"
        assert result["target_format"] == "jsonl"

    def test_to_json_unsupported_format(self):
        """测试不支持的中间格式"""
        converter = DatasetConverter()
        
        with pytest.raises(ValueError, match="无法从"):
            converter._to_json([], "unsupported")
