# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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


class TestConverterExtended:
    """DatasetConverter 扩展测试"""

    def test_convert_empty_dataset(self):
        """转换空数据集"""
        converter = DatasetConverter()
        result = converter.convert([], "json", "csv")
        assert result == []

    def test_convert_single_item(self):
        """转换单条数据"""
        converter = DatasetConverter()
        data = [{"instruction": "q1", "input": "", "output": "a1"}]
        result = converter.convert(data, "json", "csv")
        assert len(result) == 1

    def test_get_supported_formats(self):
        """获取支持的格式"""
        formats = get_supported_formats()
        assert "json" in formats
        assert "csv" in formats

    def test_format_enum_values(self):
        """格式枚举值"""
        assert DataFormat.JSON.value == "json"
        assert DataFormat.CSV.value == "csv"
        assert DataFormat.JSONL.value == "jsonl"

    def test_convert_json_to_alpaca(self, sample_dataset):
        """转换 JSON 到 Alpaca 格式"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "alpaca")
        assert len(result) == len(sample_dataset)

    def test_convert_json_to_sharegpt(self, sample_dataset):
        """转换 JSON 到 ShareGPT 格式"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "sharegpt")
        assert len(result) == len(sample_dataset)

    def test_convert_with_history(self, sample_with_history):
        """转换带历史记录的数据"""
        converter = DatasetConverter()
        result = converter.convert(sample_with_history, "json", "json")
        assert len(result) == 1
        assert "history" in result[0]


class TestConverterExtended2:
    """DatasetConverter 第二轮扩展测试（覆盖转换链路与未注册格式）"""

    def test_same_format_returns_input(self, sample_dataset):
        """源格式与目标格式相同时应原样返回"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "json")
        assert result is sample_dataset

    def test_jsonl_to_json_round_trip(self, sample_dataset):
        """jsonl -> json 应还原字典"""
        converter = DatasetConverter()
        jsonl_lines = converter.convert(sample_dataset, "json", "jsonl")
        back = converter.convert(jsonl_lines, "jsonl", "json")
        assert back == sample_dataset

    def test_json_to_csv_returns_dicts(self, sample_dataset):
        """json -> csv 当前实现返回字典列表（保留字段）"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "csv")
        assert len(result) == len(sample_dataset)
        assert all(isinstance(item, dict) for item in result)

    def test_unsupported_target_raises(self, sample_dataset):
        """未注册的目标格式应报错"""
        converter = DatasetConverter()
        with pytest.raises(ValueError, match="不支持的转换"):
            converter.convert(sample_dataset, "json", "parquet")

    def test_jsonl_direct_loads_iterable(self, sample_dataset):
        """_jsonl_to_json 对字符串列表逐行解析"""
        converter = DatasetConverter()
        lines = [json.dumps(item, ensure_ascii=False) for item in sample_dataset]
        result = converter._jsonl_to_json(lines)
        assert result == sample_dataset

    def test_csv_to_json_list_passthrough(self, sample_dataset):
        """_csv_to_json 对已是列表的输入直接返回"""
        converter = DatasetConverter()
        assert converter._csv_to_json(sample_dataset) is sample_dataset

    def test_csv_to_json_empty_non_list(self):
        """_csv_to_json 非列表输入返回空列表"""
        converter = DatasetConverter()
        assert converter._csv_to_json(None) == []

    def test_tsv_to_json_mirrors_the_csv_edge(self, sample_dataset):
        """`tsv → json` 与 `csv → json` 是同一条边：分隔符在读写两侧，边本身只搬运行"""
        converter = DatasetConverter()
        assert converter._tsv_to_json(sample_dataset) is sample_dataset
        assert converter._tsv_to_json(None) == []

    def test_tsv_to_csv_edge_is_a_passthrough(self, sample_dataset):
        """`tsv → csv` 经规范形中转：两份表头不同、数据同一份"""
        converter = DatasetConverter()
        assert converter.convert(sample_dataset, "tsv", "csv") == sample_dataset

    def test_normalize_format_case_insensitive(self):
        """格式名标准化应忽略大小写"""
        converter = DatasetConverter()
        assert converter._normalize_format("JSON") == "json"
        assert converter._normalize_format("ALPACA") == "alpaca"

    def test_normalize_unknown_format_lowercases(self):
        """未知格式应转小写并原样保留"""
        converter = DatasetConverter()
        assert converter._normalize_format("MyFmt") == "myfmt"

    def test_json_to_llama_factory(self, sample_dataset):
        """json -> llama_factory 字段映射"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "llama_factory")
        assert len(result) == len(sample_dataset)

    def test_json_to_vicuna(self, sample_dataset):
        """json -> vicuna 字段映射"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "vicuna")
        assert len(result) == len(sample_dataset)

    def test_json_to_belle(self, sample_dataset):
        """json -> belle 字段映射"""
        converter = DatasetConverter()
        result = converter.convert(sample_dataset, "json", "belle")
        assert len(result) == len(sample_dataset)

    def test_json_to_chatml_roles(self, sample_with_history):
        """json -> chatml 应展开为对话角色列表"""
        converter = DatasetConverter()
        result = converter.convert(sample_with_history, "json", "chatml")
        assert len(result) == 1
        assert result[0]["messages"] or "messages" in result[0]

    def test_convert_via_json_intermediate(self, sample_dataset):
        """jsonl -> alpaca 应经 JSON 中间格式完成"""
        converter = DatasetConverter()
        jsonl = converter.convert(sample_dataset, "json", "jsonl")
        result = converter.convert(jsonl, "jsonl", "alpaca")
        assert len(result) == len(sample_dataset)


class TestCsvWriteSurface:
    """CSV 落盘的字节级正确性。

    两处缺陷都是「本地看着没事、下游读坏了」那一类：

    1. 写侧漏 `newline=''`。`csv.writer` 写出 `\r\n` 行尾，文本模式默认又把它里面的
       `\n` 翻译成 `os.linesep`，Windows 上得到 `\r\r\n`——严格解析器会在每条记录
       之间读出一个空行。POSIX 上不显形（`os.linesep` 就是 `\n`），所以这个缺陷只在
       Windows 实测里露头。
    2. `fieldnames` 只取 `data[0].keys()`。异构数据（增强/合并后某几条多了字段）
       会让 `DictWriter` 抛 `ValueError: dict contains fields not in fieldnames`，
       整次导出失败。
    """

    def test_csv_target_has_no_stray_cr(self, tmp_path, sample_dataset):
        """`convert --format csv` 的产物行尾必须是单个 `\r\n`"""
        src = tmp_path / "src.json"
        src.write_text(json.dumps(sample_dataset, ensure_ascii=False), encoding="utf-8")
        out = tmp_path / "out.csv"

        convert_file(str(src), str(out), target_format="csv")

        raw = out.read_bytes()
        assert b"\r\r\n" not in raw, "Windows 上双写行尾，严格解析器会读出空行"
        assert raw.count(b"\r\n") == len(sample_dataset) + 1  # 表头 + 数据行

    def test_heterogeneous_items_export_without_error(self, tmp_path):
        """异构条目（后一条多出字段）不得让整次导出失败"""
        src = tmp_path / "src.json"
        items = [
            {"instruction": "a", "output": "b"},
            {"instruction": "c", "output": "d", "category": "e"},
        ]
        src.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
        out = tmp_path / "out.csv"

        convert_file(str(src), str(out), target_format="csv")

        with open(out, encoding="utf-8", newline="") as f:
            rows = list(csv.DictReader(f))
        assert rows[0]["category"] == ""          # 缺的字段落为空单元格
        assert rows[1]["category"] == "e"

    def test_empty_dataset_writes_nothing(self, tmp_path):
        """空数据集仍走 `if data` 守卫：写出 0 字节，且不抛异常"""
        src = tmp_path / "src.json"
        src.write_text("[]", encoding="utf-8")
        out = tmp_path / "out.csv"

        convert_file(str(src), str(out), target_format="csv")

        assert out.exists()
        assert out.read_bytes() == b""

    def test_read_side_uses_newline_empty(self, tmp_path, sample_dataset):
        """含换行的字段要能原样往返：写侧引号包裹，读侧不得预先折叠行尾"""
        src = tmp_path / "src.json"
        items = [{"instruction": "第一行\n第二行", "output": "x"}]
        src.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
        out = tmp_path / "out.csv"

        convert_file(str(src), str(out), target_format="csv")
        converter = DatasetConverter()
        rows = converter._read_file(out, "csv")

        assert rows == items, f"多行字段被换行拆解: {rows}"
        assert out.read_bytes().count(b"\r\r\n") == 0


class TestReverseFlatEdges:
    """字段映射类格式（alpaca / belle / llama_factory）→ 规范形。

    转换图此前只有 `json → 训练格式` 的单向边，导出的数据集再也回不到规范形
    （`convert(rows, "alpaca", "json")` 只会抛 `UnsupportedFormatError`），
    「导出 → 换工具增强 → 再导回」这条回路断在第一步。下面每条断言的预言都是
    **手写字面量**，不拿正向转换器当预言机。
    """

    def test_alpaca_keeps_every_source_key(self):
        converter = DatasetConverter()
        rows = [{"instruction": "如何租房", "input": "北京", "output": "登录官网",
                 "category": "租房"}]
        assert converter.convert(rows, "alpaca", "json") == rows

    def test_record_without_input_does_not_gain_one(self):
        """不补空 `input`：`validation` 里它是可选字段，凭空造键会让「源文件有没有
        这一列」失去可辨性，下游稀疏字段检测因此漏报。"""
        converter = DatasetConverter()
        out = converter.convert([{"instruction": "如何租房", "output": "登录官网"}],
                                "alpaca", "json")
        assert out == [{"instruction": "如何租房", "output": "登录官网"}]
        assert "input" not in out[0]

    def test_result_is_a_copy_of_the_source_records(self):
        converter = DatasetConverter()
        rows = [{"instruction": "q", "output": "a"}]
        out = converter.convert(rows, "alpaca", "json")
        out[0]["output"] = "被改了"
        assert rows[0]["output"] == "a", "结果与源记录共享对象，改结果会脏了输入"

    @pytest.mark.parametrize("fmt", ["belle", "llama_factory"])
    def test_other_flat_formats_follow_the_same_rules(self, fmt):
        converter = DatasetConverter()
        rows = [{"instruction": "q", "output": "a", "system": "你是助手"}]
        assert converter.convert(rows, fmt, "json") == rows

    def test_missing_required_field_names_row_and_field(self):
        converter = DatasetConverter()
        rows = [{"instruction": "q", "output": "a"}, {"instruction": "只有问题"}]
        with pytest.raises(ValueError, match="第 2 条 belle 记录缺少字段: output"):
            converter.convert(rows, "belle", "json")

    def test_top_level_must_be_an_array(self):
        converter = DatasetConverter()
        with pytest.raises(ValueError, match="顶层必须是 JSON 数组，当前是dict"):
            converter.convert({"instruction": "q"}, "alpaca", "json")

    def test_record_must_be_an_object(self):
        converter = DatasetConverter()
        with pytest.raises(ValueError, match="第 1 条 alpaca 记录必须是 JSON 对象，当前是str"):
            converter.convert(["不是对象"], "alpaca", "json")

    def test_empty_dataset_still_yields_empty_list(self):
        converter = DatasetConverter()
        assert converter.convert([], "alpaca", "json") == []

    def test_alpaca_to_csv_goes_through_the_json_middle(self):
        """反向边顺带接通了中转链路：`alpaca → csv` 以前无路可走"""
        converter = DatasetConverter()
        rows = [{"instruction": "q", "input": "", "output": "a"}]
        assert converter.convert(rows, "alpaca", "csv") == rows


class TestReverseConversationEdges:
    """对话类格式（sharegpt / vicuna / chatml）→ 规范形。

    三家只差在**容器字段名**与**角色写法**，折叠规则（末两轮是当前问答、开头的
    system 轮还原成字段、其余进 history）是共用的：所以形态各测一遍、边界集中测。
    """

    def test_sharegpt_single_pair(self):
        converter = DatasetConverter()
        rows = [{"conversations": [{"from": "human", "value": "你好"},
                                   {"from": "gpt", "value": "你好呀"}]}]
        assert converter.convert(rows, "sharegpt", "json") == [
            {"instruction": "你好", "output": "你好呀"}
        ]

    def test_sharegpt_history_and_system_split_back_to_fields(self):
        converter = DatasetConverter()
        rows = [{"conversations": [
            {"from": "system", "value": "你是租房顾问"},
            {"from": "human", "value": "问题1"}, {"from": "gpt", "value": "回答1"},
            {"from": "human", "value": "问题2"}, {"from": "gpt", "value": "回答2"}]}]
        assert converter.convert(rows, "sharegpt", "json") == [{
            "system": "你是租房顾问",
            "history": [{"role": "user", "content": "问题1"},
                        {"role": "assistant", "content": "回答1"}],
            "instruction": "问题2",
            "output": "回答2",
        }]

    def test_history_without_leading_system_is_still_history(self):
        """首两轮不是 system 时不硬套 system 字段，一律按顺序进 history"""
        converter = DatasetConverter()
        rows = [{"messages": [
            {"role": "user", "content": "问题1"}, {"role": "assistant", "content": "回答1"},
            {"role": "user", "content": "问题2"}, {"role": "assistant", "content": "回答2"}]}]
        assert converter.convert(rows, "chatml", "json") == [{
            "history": [{"role": "user", "content": "问题1"},
                        {"role": "assistant", "content": "回答1"}],
            "instruction": "问题2",
            "output": "回答2",
        }]

    def test_chatml_uses_messages_and_content(self):
        converter = DatasetConverter()
        rows = [{"messages": [{"role": "user", "content": "怎么退租"},
                              {"role": "assistant", "content": "满一年后退还"}]}]
        assert converter.convert(rows, "chatml", "json") == [
            {"instruction": "怎么退租", "output": "满一年后退还"}
        ]

    def test_vicuna_keeps_keys_outside_the_container(self):
        """`id` 这类容器外的键原样保留：它是 vicuna 数据的唯一标识，丢了就没法定位"""
        converter = DatasetConverter()
        rows = [{"id": "v-1", "conversations": [{"from": "user", "value": "q"},
                                                {"from": "assistant", "value": "a"}]}]
        assert converter.convert(rows, "vicuna", "json") == [
            {"id": "v-1", "instruction": "q", "output": "a"}
        ]

    def test_role_spellings_are_unified_and_case_insensitive(self):
        """human/user、gpt/assistant 两家写法都要认，大小写不敏感"""
        converter = DatasetConverter()
        rows = [{"conversations": [{"from": "HUMAN", "value": "q"},
                                   {"from": "GPT", "value": "a"}]},
                {"conversations": [{"from": "user", "value": "q2"},
                                   {"from": "assistant", "value": "a2"}]}]
        assert converter.convert(rows, "sharegpt", "json") == [
            {"instruction": "q", "output": "a"},
            {"instruction": "q2", "output": "a2"},
        ]

    def test_unknown_role_is_rejected_rather_than_guessed(self):
        """不认识的角色（tool / planner / 函数调用）必须报错：猜成 user 会把
        工具输出静默变成「用户说的话」，训练数据从此不可信。"""
        converter = DatasetConverter()
        rows = [{"conversations": [{"from": "tool", "value": "外部返回"},
                                   {"from": "gpt", "value": "a"}]}]
        with pytest.raises(ValueError, match="第 1 条 sharegpt 记录含未认识的角色: tool"):
            converter.convert(rows, "sharegpt", "json")

    def test_missing_container_array_is_named_by_key(self):
        converter = DatasetConverter()
        with pytest.raises(ValueError, match="第 1 条 chatml 记录缺少 `messages` 数组"):
            converter.convert([{"instruction": "q", "output": "a"}], "chatml", "json")

    def test_turn_missing_role_or_value_is_named(self):
        converter = DatasetConverter()
        rows = [{"messages": [{"role": "user"}, {"role": "assistant", "content": "a"}]}]
        with pytest.raises(ValueError, match="某一轮缺少 `role` / `content`"):
            converter.convert(rows, "chatml", "json")

    def test_single_turn_cannot_restore_a_pair(self):
        converter = DatasetConverter()
        rows = [{"messages": [{"role": "user", "content": "只有问"}]}]
        with pytest.raises(ValueError, match="第 1 条 chatml 对话少于两轮"):
            converter.convert(rows, "chatml", "json")

    def test_conversation_must_end_with_user_then_assistant(self):
        """末尾不是「一问一答」时无法判定哪条是 output，报错比按位置硬切安全"""
        converter = DatasetConverter()
        rows = [{"conversations": [{"from": "human", "value": "q"},
                                   {"from": "human", "value": "q2"}]}]
        with pytest.raises(ValueError,
                           match="对话必须以「user → assistant」结尾，实际是「user → user」"):
            converter.convert(rows, "sharegpt", "json")

    def test_record_must_be_an_object(self):
        converter = DatasetConverter()
        rows = [[{"from": "human", "value": "q"}]]
        with pytest.raises(ValueError, match="第 1 条 vicuna 记录必须是 JSON 对象，当前是list"):
            converter.convert(rows, "vicuna", "json")

    def test_top_level_must_be_an_array(self):
        converter = DatasetConverter()
        with pytest.raises(ValueError, match="vicuna 数据集的顶层必须是 JSON 数组"):
            converter.convert({"conversations": []}, "vicuna", "json")

    def test_empty_dataset_still_yields_empty_list(self):
        converter = DatasetConverter()
        assert converter.convert([], "sharegpt", "json") == []

    def test_sharegpt_to_chatml_goes_through_the_json_middle(self):
        """`_to_json` 改为查表后，对话类之间的互转也要能经规范形中转"""
        converter = DatasetConverter()
        rows = [{"conversations": [{"from": "human", "value": "q"},
                                   {"from": "gpt", "value": "a"}]}]
        assert converter.convert(rows, "sharegpt", "chatml") == [
            {"messages": [{"role": "system", "content": "You are a helpful assistant."},
                          {"role": "user", "content": "q"},
                          {"role": "assistant", "content": "a"}]}
        ]


class TestRoundTripLossiness:
    """`json → 训练格式 → json` 的**已知信息损失**，逐家钉住。

    反向边不是无损压缩：写侧本来就丢字段，回读自然没有。把损失写成用例是为了——
    哪天有人给写侧补上这些字段，这里会红，从而逼他确认往返口径而不是悄悄改语义。
    `RICH` 每条都带 input / system / history，正好把各家的丢弃面照出来。
    """

    RICH = {
        "instruction": "怎么月付",
        "input": "北京朝阳",
        "output": "支持月付",
        "system": "你是租房顾问",
        "history": [{"role": "user", "content": "问题1"},
                    {"role": "assistant", "content": "回答1"}],
    }

    def convert_back(self, fmt):
        """正向导出 + 反向回读，返回 (导出产物, 回读结果)"""
        converter = DatasetConverter()
        exported = converter.convert([self.RICH], "json", fmt)
        return exported, converter.convert(exported, fmt, "json")

    @pytest.mark.parametrize("fmt", ["alpaca", "belle"])
    def test_field_mapping_formats_drop_system_and_history(self, fmt):
        exported, back = self.convert_back(fmt)
        assert exported == [{"instruction": "怎么月付", "input": "北京朝阳", "output": "支持月付"}]
        assert back == exported, f"{fmt} 的三个字段应原样回来"

    def test_llama_factory_keeps_system_but_not_history(self):
        exported, back = self.convert_back("llama_factory")
        assert exported == [{"instruction": "怎么月付", "input": "北京朝阳",
                             "output": "支持月付", "system": "你是租房顾问"}]
        assert back == exported

    def test_sharegpt_keeps_history_but_drops_input_and_system(self):
        exported, back = self.convert_back("sharegpt")
        assert exported == [{"conversations": [
            {"from": "user", "value": "问题1"}, {"from": "assistant", "value": "回答1"},
            {"from": "human", "value": "怎么月付"}, {"from": "gpt", "value": "支持月付"}]}]
        assert back == [{
            "history": [{"role": "user", "content": "问题1"},
                        {"role": "assistant", "content": "回答1"}],
            "instruction": "怎么月付", "output": "支持月付"}]

    def test_vicuna_drops_history_system_and_input(self):
        exported, back = self.convert_back("vicuna")
        assert exported == [{"id": "", "conversations": [
            {"from": "human", "value": "怎么月付"}, {"from": "gpt", "value": "支持月付"}]}]
        assert back == [{"id": "", "instruction": "怎么月付", "output": "支持月付"}]

    def test_chatml_keeps_system_and_history_but_drops_input(self):
        exported, back = self.convert_back("chatml")
        assert back == [{
            "system": "你是租房顾问",
            "history": [{"role": "user", "content": "问题1"},
                        {"role": "assistant", "content": "回答1"}],
            "instruction": "怎么月付", "output": "支持月付"}]


class TestReverseEdgesOnFiles:
    """文件层的源格式声明：容器格式落盘也是 `.json`，扩展名推不出来。"""

    def write(self, tmp_path, name, payload):
        path = tmp_path / name
        path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
        return path

    def test_alpaca_file_to_chatml_file(self, tmp_path):
        src = self.write(tmp_path, "in.json", [{"instruction": "q", "input": "i", "output": "a"}])
        out = tmp_path / "out.json"

        result = convert_file(str(src), str(out),
                              target_format="chatml", source_format="alpaca")

        assert result["source_format"] == "alpaca"
        assert result["input_count"] == 1
        assert result["output_count"] == 1
        assert json.loads(out.read_text(encoding="utf-8")) == [
            {"messages": [{"role": "system", "content": "You are a helpful assistant."},
                          {"role": "user", "content": "q"},
                          {"role": "assistant", "content": "a"}]}
        ]

    def test_sharegpt_file_without_declaration_keeps_the_old_behaviour(self, tmp_path):
        """不声明 `source_format` 时仍按扩展名当 json 读：老调用的行为不变。

        这条同时说明了为什么必须有 `--input_format` / `source_format`：
        按 json 读走的 `json → chatml` 边只认 `instruction`/`history` 字段，
        对话数组整个被忽略，产物是空问答——静默坏数据。
        """
        rows = [{"conversations": [{"from": "human", "value": "q"},
                                   {"from": "gpt", "value": "a"}]}]
        src = self.write(tmp_path, "in.json", rows)
        out = tmp_path / "out.json"

        convert_file(str(src), str(out), target_format="chatml")

        assert json.loads(out.read_text(encoding="utf-8")) == [{
            "messages": [{"role": "system", "content": "You are a helpful assistant."},
                         {"role": "user", "content": ""},
                         {"role": "assistant", "content": ""}]}]

        declared = tmp_path / "out2.json"
        convert_file(str(src), str(declared), target_format="chatml", source_format="sharegpt")
        assert json.loads(declared.read_text(encoding="utf-8")) == [{
            "messages": [{"role": "system", "content": "You are a helpful assistant."},
                         {"role": "user", "content": "q"},
                         {"role": "assistant", "content": "a"}]}]

    def test_broken_container_error_mentions_the_row(self, tmp_path):
        """文件里的坏记录，报错要带行号（条目下标），否则调用方只能整份手查"""
        src = self.write(tmp_path, "in.json", [
            {"instruction": "q", "output": "a"},
            {"instruction": "只有问题"},
        ])
        out = tmp_path / "out.json"

        with pytest.raises(ValueError, match="第 2 条 alpaca 记录缺少字段: output"):
            convert_file(str(src), str(out), target_format="json", source_format="alpaca")


class TestSchemaIsNotContainer:
    """`DataFormat` 里只有四个值决定**文件布局**，其余六个只是**行内 schema**。

    旧实现把两件事混成一件：`_read_file` 只特判 jsonl/csv，其余（含 alpaca/
    sharegpt/chatml/llama_factory/vicuna/belle）一律 `json.load` 整份文件。于是
    同一批 alpaca 记录，写成 `.json` 能读、写成 `.jsonl` 就抛
    `JSONDecodeError: Extra data: line 2 column 1`——调用方已经正确声明了源格式，
    却绊在没声明过的容器上。写侧同理：`target="alpaca"` 配 `.jsonl` 输出落成一整个
    JSON 数组，自己那一侧再也读不回来。

    现在读侧按内容嗅探（整份 JSON → 单条对象 → 逐行），写侧由**输出扩展名**定容器。
    下面每条用例都同时是「老路径会怎么坏」的说明。
    """

    def write(self, tmp_path, name, payload):
        path = tmp_path / name
        path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
        return path

    def write_jsonl(self, tmp_path, name, rows):
        path = tmp_path / name
        path.write_text("".join(json.dumps(r, ensure_ascii=False) + "\n" for r in rows),
                        encoding="utf-8")
        return path

    # ============ 读侧：schema 声明 + 任意容器 ============

    def test_alpaca_records_in_a_jsonl_file_read_back(self, tmp_path):
        """alpaca 落成 `.jsonl`：声明源格式后要能读（旧行为 Extra data 崩在第二行）"""
        src = self.write_jsonl(tmp_path, "a.jsonl", [
            {"instruction": "q1", "input": "", "output": "a1"},
            {"instruction": "q2", "input": "", "output": "a2"},
        ])
        out = tmp_path / "out.json"

        result = convert_file(str(src), str(out), target_format="json",
                              source_format="alpaca")

        assert result["input_count"] == 2
        assert json.loads(out.read_text(encoding="utf-8")) == [
            {"instruction": "q1", "input": "", "output": "a1"},
            {"instruction": "q2", "input": "", "output": "a2"},
        ]

    def test_conversation_schema_in_a_jsonl_file_reaches_the_other_side(self, tmp_path):
        """跨格式（sharegpt 的 `.jsonl` → chatml 的 `.json`）经规范形中转要跑通"""
        src = self.write_jsonl(tmp_path, "s.jsonl", [{
            "conversations": [{"from": "human", "value": "可以月付吗"},
                              {"from": "gpt", "value": "支持月付"}]}])
        out = tmp_path / "out.json"

        convert_file(str(src), str(out), target_format="chatml",
                     source_format="sharegpt")

        assert json.loads(out.read_text(encoding="utf-8")) == [{
            "messages": [{"role": "system", "content": "You are a helpful assistant."},
                         {"role": "user", "content": "可以月付吗"},
                         {"role": "assistant", "content": "支持月付"}]}]

    def test_single_object_jsonl_counts_as_one_record(self, tmp_path):
        """只有一行的 JSONL：整份本身也是合法 JSON（一个对象），按一条记录算

        这是嗅探最容易走偏的情形——整份解析成功不等于「顶层是数组」。按老写法
        `json.load` 返回 dict，下游会以「顶层必须是 JSON 数组」拒绝一份合法文件。
        """
        src = self.write_jsonl(tmp_path, "one.jsonl", [
            {"instruction": "q", "input": "", "output": "a"}])
        out = tmp_path / "out.json"

        result = convert_file(str(src), str(out), target_format="json",
                             source_format="alpaca")

        assert result["input_count"] == 1
        assert json.loads(out.read_text(encoding="utf-8")) == [
            {"instruction": "q", "input": "", "output": "a"}]

    def test_blank_jsonl_is_an_empty_dataset_not_a_broken_one(self, tmp_path):
        """全空白文件 = 零条记录，与 `_read_file` 的 jsonl 分支同口径

        报成「不是合法 JSON」会把一份空数据集说成一份坏数据集。
        """
        src = self.write_jsonl(tmp_path, "empty.jsonl", [])
        out = tmp_path / "out.json"

        result = convert_file(str(src), str(out), target_format="json",
                              source_format="sharegpt")

        assert result["input_count"] == 0
        assert json.loads(out.read_text(encoding="utf-8")) == []

    def test_unparsable_container_error_names_file_and_line(self, tmp_path):
        """两条路都不通时抛 `DataFormatError`，带文件名与行号，不漏解析器黑话"""
        src = tmp_path / "broken.jsonl"
        src.write_text("这不是 JSON\n第二行也不是\n", encoding="utf-8")
        out = tmp_path / "out.json"

        with pytest.raises(ValueError, match="broken.jsonl.*第 1 行也不是合法 JSON") as exc:
            convert_file(str(src), str(out), target_format="json",
                         source_format="alpaca")

        assert "Extra data" not in str(exc.value)

    # ============ 写侧：容器跟着输出扩展名走 ============

    def test_schema_target_lands_in_the_container_the_extension_asks_for(self, tmp_path):
        """`target="alpaca"` 写进 `.jsonl` 就真是一行一条

        旧实现一律 `json.dump` 成数组，写出来的 `.jsonl` 连转换器自己都读不回来。
        """
        src = self.write(tmp_path, "in.json", [
            {"instruction": "q1", "input": "", "output": "a1"},
            {"instruction": "q2", "input": "", "output": "a2"},
        ])
        out = tmp_path / "out.jsonl"

        convert_file(str(src), str(out), target_format="alpaca", source_format="json")

        lines = out.read_text(encoding="utf-8").splitlines()
        assert [json.loads(line) for line in lines] == [
            {"instruction": "q1", "input": "", "output": "a1"},
            {"instruction": "q2", "input": "", "output": "a2"},
        ]

    def test_schema_target_to_json_keeps_writing_an_array(self, tmp_path):
        """`.json` 输出的 schema 目标仍是一整个 JSON 数组：老行为不变"""
        src = self.write(tmp_path, "in.json",
                         [{"instruction": "q", "input": "", "output": "a"}])
        out = tmp_path / "out.json"

        convert_file(str(src), str(out), target_format="alpaca", source_format="json")

        text = out.read_text(encoding="utf-8")
        assert text.lstrip().startswith("[")
        assert json.loads(text) == [{"instruction": "q", "input": "", "output": "a"}]

    def test_alpaca_jsonl_round_trips_through_the_cli_surface(self, tmp_path):
        """json → alpaca `.jsonl` → json：写侧与读侧的容器判断必须对称

        只断言「绕一圈数据没丢」是测不出缺陷的：旧实现把两侧都写错成同一个形状
        （`.jsonl` 里落一整个 JSON 数组，读侧再整份 load 回来），往返自然对得上。
        所以这里必须把中间产物的**物理布局**也钉住。
        """
        rows = [{"instruction": "租期最短多久", "input": "", "output": "一个月起租"},
                {"instruction": "如何退租押金", "input": "", "output": "满一年后退还"}]
        src = self.write(tmp_path, "in.json", rows)
        mid = tmp_path / "mid.jsonl"
        back = tmp_path / "back.json"

        convert_file(str(src), str(mid), target_format="alpaca", source_format="json")

        lines = [line for line in mid.read_text(encoding="utf-8").splitlines() if line]
        assert len(lines) == 2, f"`.jsonl` 中间产物不是一行一条: {lines}"

        result = convert_file(str(mid), str(back), target_format="json",
                              source_format="alpaca")

        assert result["input_count"] == 2
        assert json.loads(back.read_text(encoding="utf-8")) == rows

    # ============ tsv：补齐的两条边 ============

    def test_blank_lines_between_records_are_skipped(self, tmp_path):
        """记录之间夹空行：跳过而不是当成坏数据

        手写/拼接出来的 `.jsonl` 常有尾随或中间空行，`jsonl` 分支一直容忍
        （`if line.strip()`），嗅探分支必须同口径。
        """
        src = tmp_path / "blank.jsonl"
        src.write_text(
            '{"instruction": "q1", "input": "", "output": "a1"}\n'
            "\n"
            '{"instruction": "q2", "input": "", "output": "a2"}\n'
            "\n",
            encoding="utf-8")
        out = tmp_path / "out.json"

        result = convert_file(str(src), str(out), target_format="json",
                              source_format="alpaca")

        assert result["input_count"] == 2
        assert json.loads(out.read_text(encoding="utf-8")) == [
            {"instruction": "q1", "input": "", "output": "a1"},
            {"instruction": "q2", "input": "", "output": "a2"},
        ]

    def test_scalar_lines_do_not_become_records(self, tmp_path):
        """整份是合法 JSON 但既非数组也非对象（如 `null`）：不得当成读通了

        嗅探只认「记录数组」和「单条记录」两种形态；顶层是标量时继续逐行走，让
        报错停在真正的问题上（这条记录不是对象），而不是凭空多出一条空记录。
        """
        src = tmp_path / "scalars.jsonl"
        src.write_text("null\n", encoding="utf-8")
        out = tmp_path / "out.json"

        with pytest.raises(ValueError, match="第 1 条 alpaca 记录必须是 JSON 对象") as exc:
            convert_file(str(src), str(out), target_format="json",
                         source_format="alpaca")

        assert not out.exists()

    def test_tsv_edges_are_in_the_graph_and_the_public_list(self):
        """`tsv` 从「枚举有成员、图里没有边」变成两侧同真"""
        assert "tsv" in get_supported_formats()
        converters = DatasetConverter()._converters
        assert ("json", "tsv") in converters and ("tsv", "json") in converters

    def test_tsv_write_is_tab_separated_and_quotes_like_csv(self, tmp_path):
        """写 tsv：制表符分列，字段内的制表符/逗号/换行由引号保住，严格解析读回

        与 csv 共用 `csv_fieldnames`（全量键并集），所以缺列也要补空表头。
        """
        rows = [{"instruction": "含\t制表", "output": "含,逗号"},
                {"instruction": "含\n换行", "output": "正常", "extra": "多出来的键"}]
        src = self.write(tmp_path, "in.json", rows)
        out = tmp_path / "out.tsv"

        convert_file(str(src), str(out), target_format="tsv", source_format="json")

        with out.open(newline="", encoding="utf-8") as f:
            reader = csv.DictReader(f, delimiter="\t")
            assert reader.fieldnames == ["instruction", "output", "extra"]
            assert list(reader) == [{"instruction": "含\t制表", "output": "含,逗号",
                                     "extra": ""},
                                    {"instruction": "含\n换行", "output": "正常",
                                     "extra": "多出来的键"}]

    def test_tsv_reads_by_extension_inference(self, tmp_path):
        """不给任何显式格式时，`.tsv → .json` 靠扩展名就该推断出来"""
        src = tmp_path / "in.tsv"
        src.write_text("instruction\toutput\n可以月付吗\t支持月付\n", encoding="utf-8")
        out = tmp_path / "out.json"

        result = convert_file(str(src), str(out))

        assert (result["source_format"], result["target_format"]) == ("tsv", "json")
        assert json.loads(out.read_text(encoding="utf-8")) == [
            {"instruction": "可以月付吗", "output": "支持月付"}]

    def test_empty_dataset_writes_an_empty_tsv(self, tmp_path):
        """空数据集写 `.tsv`：写出 0 字节、不抛异常（与 csv 同一守卫）

        连表头都不写，是 `_write_file` 里 `if data:` 的既有口径；tsv 分支照抄，
        两份表格格式在「空数据集」上不得有第二种行为。
        """
        src = self.write(tmp_path, "src.json", [])
        out = tmp_path / "out.tsv"

        convert_file(str(src), str(out), target_format="tsv", source_format="json")

        assert out.read_bytes() == b""

    def test_convert_dataset_to_tsv_no_longer_raises(self):
        """`convert_dataset(items, "tsv")` 曾是虚报：清单说有、一调就抛"""
        result = convert_dataset([{"instruction": "q", "input": "", "output": "a"}], "tsv")

        assert result == [{"instruction": "q", "input": "", "output": "a"}]

