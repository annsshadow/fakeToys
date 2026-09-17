"""多格式导出单元测试

导出格式必须严格符合下游训练框架的字段约定，否则数据无法被消费。
"""

import json
from pathlib import Path

import pytest

from augmentor.export import Exporter, ExportFormat


class TestFormatConversion:
    """各格式转换规则"""

    def test_jsonl_fields(self):
        """JSONL 每条记录必须包含 input/instruction/output"""
        exporter = Exporter()
        lines = exporter._convert_to_jsonl([
            {"instruction": "问题", "input": "", "output": "回答"}
        ])
        record = json.loads(lines[0])

        assert set(record.keys()) == {"input", "instruction", "output"}

    def test_llama_factory_injects_system_prompt(self):
        """Llama-Factory 需要 system 字段，缺失时应补默认值"""
        exporter = Exporter()
        data = exporter._convert_to_llama_factory([
            {"instruction": "问题", "output": "回答"}
        ])
        assert data[0]["system"]

    def test_alpaca_structure(self):
        """Alpaca 保持 instruction/input/output 三段式"""
        exporter = Exporter()
        data = exporter._convert_to_alpaca([{"instruction": "问题", "output": "回答"}])
        assert set(data[0].keys()) == {"instruction", "input", "output"}

    def test_sharegpt_conversations(self):
        """ShareGPT 使用 conversations 数组且以 human/gpt 标记角色"""
        exporter = Exporter()
        data = exporter._convert_to_sharegpt([
            {"instruction": "问题", "output": "回答"}
        ])
        conversations = data[0]["conversations"]

        assert conversations[0]["from"] == "human"
        assert conversations[-1]["from"] == "gpt"

    def test_sharegpt_includes_history(self):
        """存在 history 时多轮对话必须被保留，否则上下文丢失"""
        exporter = Exporter()
        data = exporter._convert_to_sharegpt([{
            "instruction": "第二轮问题",
            "output": "第二轮回答",
            "history": [{"role": "user", "content": "第一轮问题"}]
        }])
        conversations = data[0]["conversations"]

        assert len(conversations) == 3
        assert conversations[0]["value"] == "第一轮问题"

    def test_chatml_messages(self):
        """ChatML 必须包含 system/user/assistant 三种角色"""
        exporter = Exporter()
        data = exporter._convert_to_chatml([{"instruction": "问题", "output": "回答"}])
        roles = [m["role"] for m in data[0]["messages"]]

        assert "system" in roles
        assert "user" in roles
        assert "assistant" in roles


class TestExportToFile:
    """导出落盘"""

    def test_export_jsonl_file(self, tmp_path):
        """JSONL 应为每行一条记录，且可直接被 json.loads 解析"""
        exporter = Exporter()
        output = tmp_path / "out.jsonl"
        fmt = exporter.export(
            [{"instruction": "问题", "output": "回答"}], str(output), "jsonl"
        )

        assert fmt == "jsonl"
        assert output.exists()
        lines = output.read_text(encoding='utf-8').strip().split("\n")
        assert len(lines) == 1
        assert json.loads(lines[0])["instruction"] == "问题"

    def test_export_csv_file(self, tmp_path):
        """CSV 导出应包含表头"""
        exporter = Exporter()
        output = tmp_path / "out.csv"
        exporter.export(
            [{"instruction": "问题", "output": "回答"}], str(output), "csv"
        )

        content = output.read_text(encoding='utf-8')
        assert "instruction" in content.split("\n")[0]

    def test_export_creates_parent_dir(self, tmp_path):
        """输出目录不存在时应自动创建，避免 FileNotFoundError"""
        exporter = Exporter()
        output = tmp_path / "nested" / "dir" / "out.json"
        exporter.export([{"instruction": "问题", "output": "回答"}], str(output), "alpaca")

        assert output.exists()

    def test_export_empty_dataset_jsonl(self, tmp_path):
        """空数据集导出应生成空文件而不是报错"""
        exporter = Exporter()
        output = tmp_path / "empty.jsonl"
        exporter.export([], str(output), "jsonl")

        assert output.exists()
        assert output.read_text(encoding='utf-8') == ""

    def test_invalid_format_raises(self, tmp_path):
        """未知格式必须报错，避免静默产出空文件"""
        exporter = Exporter()
        with pytest.raises(ValueError):
            exporter.export([{"instruction": "x"}], str(tmp_path / "x.json"), "unknown")

    def test_default_format_used_when_omitted(self, tmp_path):
        """未指定格式时使用默认格式"""
        exporter = Exporter(default_format="alpaca")
        output = tmp_path / "default.json"
        fmt = exporter.export([{"instruction": "问题", "output": "回答"}], str(output))

        assert fmt == "alpaca"


class TestExportAllFormats:
    """全格式导出"""

    def test_exports_every_format(self, tmp_path):
        """export_all_formats 必须覆盖所有支持的格式"""
        exporter = Exporter()
        results = exporter.export_all_formats(
            [{"instruction": "问题", "output": "回答"}], str(tmp_path)
        )

        assert set(results.keys()) == {f.value for f in ExportFormat}
        for path in results.values():
            assert Path(path).exists()

    def test_serial_mode(self, tmp_path):
        """串行模式结果应与并行模式一致"""
        exporter = Exporter()
        results = exporter.export_all_formats(
            [{"instruction": "问题", "output": "回答"}], str(tmp_path), use_parallel=False
        )
        assert len(results) == len(ExportFormat)


class TestExportBatch:
    """批量导出多个数据集"""

    def test_export_multiple_datasets_and_formats(self, tmp_path):
        """批量导出需为每个数据集 × 每种格式产出独立文件"""
        exporter = Exporter()
        datasets = {
            "alpha": [{"instruction": "A", "output": "a"}],
            "beta": [{"instruction": "B", "output": "b"}],
        }
        results = exporter.export_batch(
            datasets, str(tmp_path), formats=["jsonl", "alpaca"]
        )

        assert set(results.keys()) == {"alpha", "beta"}
        for name in datasets:
            assert set(results[name].keys()) == {"jsonl", "alpaca"}
            for path in results[name].values():
                assert Path(path).exists()

    def test_rejects_invalid_format_upfront(self, tmp_path):
        """格式非法应在开始导出前报错，避免部分产出"""
        exporter = Exporter()
        with pytest.raises(ValueError):
            exporter.export_batch({"a": []}, str(tmp_path), formats=["nope"])

    def test_empty_dataset_batch(self, tmp_path):
        """空数据集批量导出不应抛异常"""
        exporter = Exporter()
        results = exporter.export_batch({"empty": []}, str(tmp_path), formats=["jsonl"])
        assert Path(results["empty"]["jsonl"]).exists()


class TestSupportedFormats:
    """格式清单"""

    def test_lists_all_formats(self):
        """格式清单需包含计划要求的 5 种格式"""
        formats = Exporter().get_supported_formats()
        for expected in ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"]:
            assert expected in formats


class TestConvertToCSV:
    """CSV 转换直接测试"""

    def test_csv_fields(self):
        """CSV 应包含 instruction/input/output 三个字段"""
        exporter = Exporter()
        data = exporter._convert_to_csv([
            {"instruction": "问题", "input": "上下文", "output": "回答"}
        ])
        assert set(data[0].keys()) == {"instruction", "input", "output"}

    def test_csv_empty_input_default(self):
        """缺失字段应使用空字符串默认值"""
        exporter = Exporter()
        data = exporter._convert_to_csv([{"instruction": "问题"}])
        assert data[0]["input"] == ""
        assert data[0]["output"] == ""

    def test_csv_multiple_items(self):
        """多条数据应全部转换"""
        exporter = Exporter()
        items = [{"instruction": f"q{i}", "output": f"a{i}"} for i in range(5)]
        data = exporter._convert_to_csv(items)
        assert len(data) == 5

    def test_csv_empty_list(self):
        """空列表应返回空"""
        exporter = Exporter()
        data = exporter._convert_to_csv([])
        assert data == []


class TestExportFileFormats:
    """各格式文件导出测试"""

    def test_export_sharegpt_file(self, tmp_path):
        """ShareGPT 导出应生成有效 JSON"""
        exporter = Exporter()
        output = tmp_path / "out.json"
        fmt = exporter.export(
            [{"instruction": "问题", "output": "回答"}], str(output), "sharegpt"
        )
        assert fmt == "sharegpt"
        with open(output, 'r', encoding='utf-8') as f:
            data = json.load(f)
        assert "conversations" in data[0]

    def test_export_chatml_file(self, tmp_path):
        """ChatML 导出应生成有效 JSON"""
        exporter = Exporter()
        output = tmp_path / "out.json"
        fmt = exporter.export(
            [{"instruction": "问题", "output": "回答"}], str(output), "chatml"
        )
        assert fmt == "chatml"
        with open(output, 'r', encoding='utf-8') as f:
            data = json.load(f)
        assert "messages" in data[0]

    def test_export_llama_factory_file(self, tmp_path):
        """Llama-Factory 导出应生成有效 JSON"""
        exporter = Exporter()
        output = tmp_path / "out.json"
        fmt = exporter.export(
            [{"instruction": "问题", "output": "回答"}], str(output), "llama_factory"
        )
        assert fmt == "llama_factory"
        with open(output, 'r', encoding='utf-8') as f:
            data = json.load(f)
        assert "system" in data[0]

    def test_export_alpaca_file(self, tmp_path):
        """Alpaca 导出应生成有效 JSON"""
        exporter = Exporter()
        output = tmp_path / "out.json"
        fmt = exporter.export(
            [{"instruction": "问题", "output": "回答"}], str(output), "alpaca"
        )
        assert fmt == "alpaca"
        with open(output, 'r', encoding='utf-8') as f:
            data = json.load(f)
        assert data[0]["instruction"] == "问题"

    def test_export_csv_empty_dataset_no_file(self, tmp_path):
        """空数据集 CSV 导出不创建文件（无数据可写）"""
        exporter = Exporter()
        output = tmp_path / "empty.csv"
        exporter.export([], str(output), "csv")
        assert not output.exists()


class TestExportAllFormatsExtended:
    """全格式导出扩展测试"""

    def test_custom_base_name(self, tmp_path):
        """自定义 base_name"""
        exporter = Exporter()
        results = exporter.export_all_formats(
            [{"instruction": "q", "output": "a"}],
            str(tmp_path),
            base_name="custom"
        )
        for path in results.values():
            assert "custom" in Path(path).name

    def test_all_files_valid(self, tmp_path):
        """所有导出文件应可被解析"""
        exporter = Exporter()
        data = [{"instruction": "问题", "output": "回答"}]
        results = exporter.export_all_formats(data, str(tmp_path))
        
        for fmt_name, path in results.items():
            p = Path(path)
            if fmt_name == "csv":
                content = p.read_text(encoding='utf-8')
                assert "instruction" in content
            elif fmt_name == "jsonl":
                content = p.read_text(encoding='utf-8')
                if content.strip():
                    json.loads(content.strip().split('\n')[0])
            else:
                with open(p, 'r', encoding='utf-8') as f:
                    json.load(f)


class TestExportBatchExtended:
    """批量导出扩展测试"""

    def test_batch_serial_mode(self, tmp_path):
        """串行模式应与并行模式产出一致"""
        exporter = Exporter()
        datasets = {"ds1": [{"instruction": "q", "output": "a"}]}
        results = exporter.export_batch(
            datasets, str(tmp_path), formats=["jsonl"], use_parallel=False
        )
        assert Path(results["ds1"]["jsonl"]).exists()

    def test_batch_single_dataset(self, tmp_path):
        """单数据集批量导出"""
        exporter = Exporter()
        results = exporter.export_batch(
            {"only": [{"instruction": "q", "output": "a"}]},
            str(tmp_path),
            formats=["jsonl", "alpaca"]
        )
        assert set(results["only"].keys()) == {"jsonl", "alpaca"}


class TestExporterInit:
    """Exporter 初始化测试"""

    def test_custom_max_workers(self):
        """自定义 max_workers"""
        exporter = Exporter(max_workers=8)
        assert exporter.max_workers == 8

    def test_default_format(self):
        """默认格式应正确设置"""
        exporter = Exporter(default_format="csv")
        assert exporter.default_format == ExportFormat.CSV
