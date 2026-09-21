"""多格式导出单元测试

导出格式必须严格符合下游训练框架的字段约定，否则数据无法被消费。
"""

import json
from pathlib import Path

import pytest

from augmentor.export import Exporter, ExportFormat, NATIVE_FORMATS


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
        """export_all_formats 必须覆盖全部原生格式

        注意断言的是 `NATIVE_FORMATS`（6 种）而不是整个 `ExportFormat` 枚举
        （13 种）：枚举统一后两者不再相等，而 export_all_formats 刻意只导出原生
        实现的那 6 种，以免调用方产物数量翻倍。
        """
        exporter = Exporter()
        results = exporter.export_all_formats(
            [{"instruction": "问题", "output": "回答"}], str(tmp_path)
        )

        assert set(results.keys()) == {f.value for f in NATIVE_FORMATS}
        for path in results.values():
            assert Path(path).exists()

    def test_serial_mode(self, tmp_path):
        """串行模式结果应与并行模式一致"""
        exporter = Exporter()
        results = exporter.export_all_formats(
            [{"instruction": "问题", "output": "回答"}], str(tmp_path), use_parallel=False
        )
        assert len(results) == len(NATIVE_FORMATS)


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
        """默认格式应为 jsonl，且显式传入时被正确解析为枚举

        原用例函数体只有一行 docstring，没有任何断言——它永远通过。
        """
        assert Exporter().default_format == ExportFormat.JSONL
        assert Exporter(default_format="alpaca").default_format == ExportFormat.ALPACA

    def test_export_chatml_system_history(self, tmp_path):
        """ChatML 导出应包含 system 与历史对话"""
        exporter = Exporter()
        items = [
            {
                "instruction": "q",
                "output": "a",
                "system": "你是客服",
                "history": [{"role": "user", "content": "之前的问题"}]
            }
        ]
        out = str(tmp_path / "chatml.json")
        exporter.export(items, out, format="chatml")
        with open(out, encoding="utf-8") as f:
            data = json.load(f)
        roles = [m["role"] for m in data[0]["messages"]]
        assert "system" in roles
        assert "user" in roles
        assert "assistant" in roles

    def test_export_unsupported_format_raises(self, tmp_path):
        """不支持的导出格式应报错"""
        exporter = Exporter()
        with pytest.raises(ValueError):
            exporter.export([{"instruction": "q"}], str(tmp_path / "x.xml"), format="xml")

    def test_export_default_format_is_jsonl(self, tmp_path):
        """未指定格式应使用默认 jsonl"""
        exporter = Exporter()
        used = exporter.export([{"instruction": "q", "output": "a"}], str(tmp_path / "d.jsonl"))
        assert used == "jsonl"

    def test_export_creates_parent_dirs(self, tmp_path):
        """导出应自动创建父目录"""
        exporter = Exporter()
        out = str(tmp_path / "nested" / "dir" / "out.jsonl")
        exporter.export([{"instruction": "q", "output": "a"}], out)
        assert Path(out).exists()

    def test_export_csv_empty_creates_file(self, tmp_path):
        """空数据 CSV 导出：父目录被创建，文件本体可能为空"""
        exporter = Exporter()
        out_dir = tmp_path / "csvdir"
        out = str(out_dir / "empty.csv")
        exporter.export([], out, format="csv")
        assert out_dir.exists()

    def test_export_sharegpt_format(self, tmp_path):
        """sharegpt 格式导出"""
        exporter = Exporter()
        out = str(tmp_path / "sg.json")
        exporter.export([{"instruction": "q", "output": "a"}], out, format="sharegpt")
        with open(out, encoding="utf-8") as f:
            data = json.load(f)
        assert "conversations" in data[0]

    def test_export_llama_factory_format(self, tmp_path):
        """llama_factory 格式导出"""
        exporter = Exporter()
        out = str(tmp_path / "lf.json")
        exporter.export([{"instruction": "q", "output": "a"}], out, format="llama_factory")
        with open(out, encoding="utf-8") as f:
            data = json.load(f)
        assert "system" in data[0]
        exporter = Exporter(default_format="csv")
        assert exporter.default_format == ExportFormat.CSV


class TestExportContractGaps:
    """变异测试（T1.4）暴露出的真实盲区

    这些用例都来自 `scripts/mutation/run.py` 报出的**存活变异体**。原来的断言
    （如「6 种格式都导出了」「角色里有 user」）会被**错误来源**满足，所以条件写反、
    循环被清空都测不出来。每条用例都注明它杀死的变异体。
    """

    def test_export_all_formats_uses_correct_extension(self, tmp_path):
        """文件名后缀必须与格式一致

        杀死 `ext = ".csv" if fmt == ExportFormat.CSV.value else ".json"` 的条件翻转：
        原用例只断言 `Path(path).exists()`，而返回值就是刚写入的路径，
        后缀写错也照样存在。
        """
        results = Exporter().export_all_formats(
            [{"instruction": "q", "output": "a"}], str(tmp_path)
        )

        assert Path(results["csv"]).suffix == ".csv"
        for fmt in NATIVE_FORMATS:
            if fmt is not ExportFormat.CSV:
                assert Path(results[fmt.value]).suffix == ".json", fmt

    def test_export_batch_uses_correct_extension(self, tmp_path):
        """批量导出同样要保证后缀正确（杀死 `export_batch` 里的 ext 条件翻转）"""
        results = Exporter().export_batch(
            {"ds": [{"instruction": "q", "output": "a"}]},
            str(tmp_path),
            formats=["csv", "jsonl"],
        )

        assert Path(results["ds"]["csv"]).suffix == ".csv"
        assert Path(results["ds"]["jsonl"]).suffix == ".json"

    def test_export_all_formats_creates_multilevel_output_dir(self, tmp_path):
        """多级输出目录必须被递归创建（杀死 `mkdir(parents=True)` → `parents=False`）"""
        nested = tmp_path / "a" / "b" / "c"
        results = Exporter().export_all_formats(
            [{"instruction": "q", "output": "a"}], str(nested)
        )

        assert nested.is_dir()
        assert all(Path(p).exists() for p in results.values())

    def test_export_batch_creates_multilevel_output_dir(self, tmp_path):
        """批量导出同样要递归创建目录"""
        nested = tmp_path / "a" / "b" / "c"
        results = Exporter().export_batch(
            {"ds": [{"instruction": "q", "output": "a"}]}, str(nested), formats=["jsonl"]
        )

        assert nested.is_dir()
        assert Path(results["ds"]["jsonl"]).exists()

    def test_serial_mode_does_not_start_thread_pool(self, tmp_path, monkeypatch):
        """use_parallel=False 必须真的走串行路径

        原用例只断言「结果与并行一致」，但两条路径的输出本来就一致，
        所以条件写反也测不出来。这里直接断言契约本身：串行模式不得创建线程池。
        可杀死 `if not use_parallel:` 的四种翻转变异。
        """

        def _forbidden(*args, **kwargs):
            raise AssertionError("串行模式不应创建 ThreadPoolExecutor")

        monkeypatch.setattr("augmentor.export.ThreadPoolExecutor", _forbidden)

        results = Exporter().export_all_formats(
            [{"instruction": "q", "output": "a"}], str(tmp_path), use_parallel=False
        )
        assert len(results) == len(NATIVE_FORMATS)

    def test_batch_serial_mode_does_not_start_thread_pool(self, tmp_path, monkeypatch):
        """export_batch 的串行模式同样不得创建线程池"""

        def _forbidden(*args, **kwargs):
            raise AssertionError("串行模式不应创建 ThreadPoolExecutor")

        monkeypatch.setattr("augmentor.export.ThreadPoolExecutor", _forbidden)

        results = Exporter().export_batch(
            {"ds": [{"instruction": "q", "output": "a"}]},
            str(tmp_path),
            formats=["jsonl"],
            use_parallel=False,
        )
        assert results["ds"]["jsonl"]

    def test_chatml_preserves_history_turns(self, tmp_path):
        """ChatML 的历史轮次必须真的进到 messages 里

        原断言 `"user" in roles` 会被「当前问题」那条 user 消息满足，
        历史被整段丢掉也照样通过（杀死 `for turn in history:` → `for turn in []:`）。
        """
        out = str(tmp_path / "chatml.json")
        Exporter().export(
            [{
                "instruction": "当前问题",
                "output": "当前回答",
                "history": [{"role": "user", "content": "历史问题"}],
            }],
            out,
            format="chatml",
        )

        with open(out, encoding="utf-8") as f:
            messages = json.load(f)[0]["messages"]

        assert [m["content"] for m in messages if m["role"] == "user"] == [
            "历史问题",
            "当前问题",
        ]


class TestExportFormatIsSingleEnum:
    """导出格式枚举必须全包唯一

    历史上 `augmentor/export.py` 与 `augmentor/export_enhanced.py` 各定义一份同名
    `ExportFormat`，而 `augmentor/__init__.py` 导出的是后者。两份枚举的值大部分
    重叠，但 `Enum.__call__` 按**成员身份**匹配（`cls._value2member_map_`），
    于是公开名 `augmentor.ExportFormat` 的成员传给 `Exporter` 时，**即使值完全
    一致**也被拒：

        ValueError: <ExportFormat.CHATML: 'chatml'> is not a valid ExportFormat

    实测修复前 13 个成员**没有一个**能用，用户唯一可用的写法是裸字符串。
    下面的用例分别锁住「定义只有一份」「公开成员能真的被消费」「历史拼写仍在」。
    """

    def test_single_definition_across_package(self):
        """三个入口必须是同一个枚举类（重新分裂成两份就会红）"""
        import augmentor
        import augmentor.export as export_module
        import augmentor.export_enhanced as enhanced_module

        assert augmentor.ExportFormat is export_module.ExportFormat
        assert augmentor.ExportFormat is enhanced_module.ExportFormat

    def test_public_members_are_accepted_by_exporter(self, tmp_path):
        """公开枚举成员必须能被 Exporter 直接消费（值相同的更要能）"""
        import augmentor

        for member in augmentor.ExportFormat:
            out = tmp_path / f"{member.value}.out"
            used = Exporter().export(
                [{"instruction": "q", "output": "a"}], str(out), member
            )
            assert used == member.value
            assert out.is_file(), member

    def test_legacy_share_gpt_spelling_still_works(self, tmp_path):
        """历史拼写 SHARE_GPT（带下划线）必须仍然可用

        它是 SHAREGPT 的同值别名而非独立成员——迭代枚举不应因此多出一项。
        """
        assert ExportFormat.SHARE_GPT is ExportFormat.SHAREGPT
        assert sum(1 for f in ExportFormat if f.value == "sharegpt") == 1

        out = tmp_path / "sharegpt.json"
        assert (
            Exporter().export(
                [{"instruction": "q", "output": "a"}], str(out), ExportFormat.SHARE_GPT
            )
            == "sharegpt"
        )
        assert out.is_file()

    def test_all_formats_writes_exactly_the_native_set(self, tmp_path):
        """export_all_formats 只写原生 6 种，不因枚举扩容而变成 13 个文件

        枚举统一后 `ExportFormat` 有 13 个成员，若 export_all_formats 直接迭代它，
        调用方（pipeline / CLI）的产物数量会从 6 变 13——属破坏性变更。
        """
        results = Exporter().export_all_formats(
            [{"instruction": "q", "output": "a"}], str(tmp_path)
        )

        assert set(results) == {f.value for f in NATIVE_FORMATS}
        assert len(results) < len(list(ExportFormat))

    def test_non_native_format_delegates_instead_of_silently_skipping(self, tmp_path):
        """非原生格式必须真的写出文件，而不是静默返回成功

        `Exporter.export` 的 if/elif 链只覆盖原生 6 种。枚举统一后 openai 等 7 种
        变成可达，若不加委托分支，它们会全部落空、**一个文件都不写**却返回
        `export_format.value`——比直接报错更难排查。
        """
        out = tmp_path / "openai.json"
        assert (
            Exporter().export(
                [{"instruction": "q", "output": "a"}], str(out), ExportFormat.OPENAI
            )
            == "openai"
        )
        assert out.is_file()
        assert json.loads(out.read_text(encoding="utf-8"))
