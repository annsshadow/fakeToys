# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 数据集工具命令集成测试

覆盖 sample / split / stats / validate / convert / search 命令，
以及 `analyze` / `visualize` 两个曾静默失败的命令的回归
（3.0 起 `--enhanced` 已移除，改为守规范命令名本身）。
"""

import csv
import json
from pathlib import Path

import pytest

from cli import main

from augmentor.converter import DataFormat, get_supported_formats

AI_DIR = Path(__file__).resolve().parent.parent.parent

SAMPLE_ITEMS = [
    {"instruction": "如何申请租房？", "input": "", "output": "登录官网申请"},
    {"instruction": "租房多少钱？", "input": "", "output": "按房型定价"},
    {"instruction": "如何退租押金？", "input": "", "output": "满一年后退还"},
    {"instruction": "可以申请月付吗？", "input": "", "output": "支持月付"},
    {"instruction": "租期最短多久？", "input": "", "output": "一个月起租"},
]


@pytest.fixture
def dataset(tmp_path, monkeypatch):
    """在临时目录写入 5 条样本数据集

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        数据集文件路径
    """
    monkeypatch.chdir(AI_DIR)
    path = tmp_path / "dataset.json"
    path.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
    return path


def run_cli(argv):
    """以给定参数运行 main()，返回 (stdout, 解析结果, exit_code)

    Args:
        argv: 完整参数列表（含程序名）

    Returns:
        (stdout 文本, JSON 解析结果或 None, SystemExit 码或 None)
    """
    import io
    import sys
    from contextlib import redirect_stdout

    old_argv = sys.argv
    sys.argv = argv
    buffer = io.StringIO()
    exit_code = None
    try:
        with redirect_stdout(buffer):
            main()
    except SystemExit as exc:
        exit_code = exc.code
    finally:
        sys.argv = old_argv
    content = buffer.getvalue()
    try:
        parsed = json.loads(content)
    except json.JSONDecodeError:
        parsed = None
    return content, parsed, exit_code


class TestSampleCommand:
    def test_sample_by_ratio(self, dataset, tmp_path):
        """按比例采样需产出确定数量（0.4 * 5 = 2 条）"""
        out = tmp_path / "sampled.json"
        out2, parsed, code = run_cli(
            [
                "cli", "sample", "--input", str(dataset),
                "--output", str(out), "--ratio", "0.4", "--seed", "7",
            ]
        )
        assert code is None
        items = json.loads(out.read_text(encoding="utf-8"))
        assert len(items) == 2
        assert parsed["output_count"] == 2

    def test_sample_fixed_size(self, dataset, tmp_path):
        """固定数量采样需严格输出该数量"""
        out = tmp_path / "sampled2.json"
        out2, parsed, code = run_cli(
            [
                "cli", "sample", "--input", str(dataset),
                "--output", str(out), "--size", "3", "--method", "stratified",
            ]
        )
        assert code is None
        assert len(json.loads(out.read_text(encoding="utf-8"))) == 3


class TestSplitCommand:
    def test_split_three_way(self, dataset, tmp_path):
        """0.6/0.2/0.2 三分割需落盘三个文件且计数守恒"""
        out_dir = tmp_path / "splits"
        out2, parsed, code = run_cli(
            [
                "cli", "split", "--input", str(dataset),
                "--output-dir", str(out_dir),
                "--train-ratio", "0.6", "--val-ratio", "0.2", "--test-ratio", "0.2",
            ]
        )
        assert code is None
        splits = parsed["splits"]
        total = sum(s["count"] for s in splits.values())
        assert total == len(SAMPLE_ITEMS)
        for name, meta in splits.items():
            assert Path(meta["file"]).exists()


class TestStatsCommand:
    def test_stats_prints_field_statistics(self, dataset):
        """统计输出需含总量与逐字段统计段（stdout 为可解析 JSON）"""
        out, parsed, code = run_cli(["cli", "stats", "--input", str(dataset)])
        assert code is None
        assert parsed["total_items"] == 5
        assert "instruction" in parsed["field_statistics"]
        assert parsed["field_statistics"]["instruction"]["unique_count"] == 5


class TestValidateCommand:
    def test_validate_strict_preset(self, dataset, tmp_path):
        """strict 预设需生成结果文件并报告通过/未通过状态"""
        out_file = tmp_path / "validation.json"
        out2, parsed, code = run_cli(
            [
                "cli", "validate", "--input", str(dataset),
                "--preset", "strict", "--output", str(out_file),
            ]
        )
        assert code is None
        result = json.loads(out_file.read_text(encoding="utf-8"))
        assert result["total_items"] == 5

    def test_validate_reports_invalid_data(self, dataset, tmp_path):
        """缺字段的脏数据需被 strict 预设判为无效"""
        bad = tmp_path / "bad.json"
        bad.write_text(json.dumps([{"instruction": ""}], ensure_ascii=False), encoding="utf-8")
        out_file = tmp_path / "bad_validation.json"
        out2, parsed, code = run_cli(
            [
                "cli", "validate", "--input", str(bad),
                "--preset", "strict", "--output", str(out_file),
            ]
        )
        assert code is None
        result = json.loads(out_file.read_text(encoding="utf-8"))
        assert result["is_valid"] is False


class TestConvertCommand:
    @pytest.mark.parametrize("fmt", ["csv", "alpaca", "chatml"])
    def test_convert_formats(self, dataset, tmp_path, fmt):
        """主流目标格式转换需成功且条数守恒"""
        out_file = tmp_path / f"converted.{fmt}"
        out2, parsed, code = run_cli(
            [
                "cli", "convert", "--input", str(dataset),
                "--output", str(out_file), "--format", fmt,
            ]
        )
        assert code is None
        assert parsed["input_count"] == 5
        assert parsed["output_count"] == 5
        assert out_file.exists()


class TestSearchCommand:
    def test_search_contains_limit(self, dataset, tmp_path):
        """contains 搜索需按 limit 截断返回条目，且条目以 JSON 列表打在 stdout

        stdout 契约沿用合并前的基础实现：两行摘要 + 匹配条目的 JSON 列表。
        条目就是这个命令的产物，只打印摘要会逼下游改用 `--output` 落盘再读。
        """
        out_file = tmp_path / "search.json"
        out, parsed, code = run_cli(
            [
                "cli", "search", "--input", str(dataset),
                "--query", "租", "--limit", "2",
                "--method", "contains", "--field", "instruction",
                "--output", str(out_file),
            ]
        )
        assert code is None
        assert "找到" in out
        # 摘要两行之后是 JSON 列表，需截取列表段解析
        items = json.loads(out[out.index("["):])
        assert len(items) == 2

        result = json.loads(out_file.read_text(encoding="utf-8"))
        assert result["total_matches"] == 4, "全量匹配数应为 4"
        assert len(result["items"]) == 2, "limit=2 未截断返回条目"

    def test_search_exact_no_match(self, dataset):
        """exact 方法搜索不存在的问题应得 0 匹配"""
        out, parsed, code = run_cli(
            [
                "cli", "search", "--input", str(dataset),
                "--query", "不存在的长问题", "--method", "exact",
            ]
        )
        assert code is None
        assert "找到 0 条" in out


class TestAnalyzeVisualizeRegression:
    """`analyze` / `visualize` 曾因分发名不匹配而静默无输出。

    3.0 起 `--enhanced` 已移除，这里改为守规范命令名本身仍有输出。
    """

    def test_analyze_prints_report(self, dataset):
        out, parsed, code = run_cli(["cli", "analyze", "--input", str(dataset)])
        assert code is None
        assert parsed["dataset_size"] == 5
        assert parsed["scores"]["quality"] >= 0
        # 合并不得丢掉旧基础实现的覆盖分析
        assert parsed["coverage_analysis"]["total_items"] == 5

    def test_analyze_saves_output(self, dataset, tmp_path):
        out_file = tmp_path / "analysis.json"
        out, parsed, code = run_cli(
            ["cli", "analyze", "--input", str(dataset), "--output", str(out_file)]
        )
        assert code is None
        saved = json.loads(out_file.read_text(encoding="utf-8"))
        assert saved["dataset_size"] == 5
        assert "coverage_analysis" in saved

    def test_visualize_report_mode_json(self, dataset, tmp_path):
        """`visualize --output --format json` 需写出可解析 JSON 报告"""
        out_file = tmp_path / "vis.json"
        out, parsed, code = run_cli(
            ["cli", "visualize", "--input", str(dataset),
             "--output", str(out_file), "--format", "json"]
        )
        assert code is None
        report = json.loads(out_file.read_text(encoding="utf-8"))
        assert report["total_items"] == 5


class TestConvertFormatSurface:
    """`convert --format` 的可选值必须等于转换图真的支持的格式集合。

    两侧曾经说的是两套话：`converter.get_supported_formats()` 直接由 `DataFormat`
    枚举生成，于是把转换图里没有 `json -> tsv` 这条边的 `tsv` 也报成支持能力
    （照它调用只会拿到 `UnsupportedFormatError`），而 CLI 的 `choices` 里没有 tsv。
    现在公开清单由 `_converters` 反推，守住「公开清单 == 能真跑通的目标 == CLI 收的
    那批」。`tsv` 两条边已在 L20 补齐，所以它两侧都收。
    """

    @pytest.mark.parametrize("fmt", get_supported_formats())
    def test_every_supported_target_really_converts(self, dataset, tmp_path, fmt):
        """公开清单里的每个目标格式都要真能转换落盘"""
        out_file = tmp_path / f"conv.{fmt}"
        _, _, code = run_cli(
            ["cli", "convert", "--input", str(dataset),
             "--output", str(out_file), "--format", fmt]
        )
        assert code is None
        assert out_file.exists()

    def test_format_outside_the_graph_is_rejected_by_cli(self, dataset, tmp_path, capsys):
        """清单外的格式 CLI 要拒收（`invalid choice`），且不留下产物

        用 `xml` 而不是某个 `DataFormat` 成员举例：转换图现已覆盖枚举全量，成员
        里挑不出「枚举有、图里没有」的那个了；这一条守的是反方向不漂移——将来往
        枚举加成员而没补转换边时，CLI 不能跟着虚报。
        """
        assert "xml" not in get_supported_formats()

        out_file = tmp_path / "x.xml"
        _, _, code = run_cli(
            ["cli", "convert", "--input", str(dataset),
             "--output", str(out_file), "--format", "xml"]
        )
        err = capsys.readouterr().err
        assert code == 2, f"CLI 未拒绝 xml: code={code}"
        assert "invalid choice" in err, err
        assert not out_file.exists()

    def test_cli_choices_cover_the_whole_graph(self, dataset, tmp_path):
        """清单里的每个格式 CLI 都收：两侧清单必须等价，不是各写一份常量

        `EXPORT_FORMATS` 那条漏了 `raw` 的老事故就是「两份清单各写各的」的结果，
        这里用「清单逐个跑通」把等价关系钉住（`--format` 的 choices 是硬编码列表，
        所以只能这么验）。
        """
        assert set(get_supported_formats()) == {f.value for f in DataFormat}, \
            "转换图与 DataFormat 枚举已不再同集合：先确认新格式该不该支持，再改这里"

        for fmt in get_supported_formats():
            out_file = tmp_path / f"cover.{fmt}"
            _, _, code = run_cli(
                ["cli", "convert", "--input", str(dataset),
                 "--output", str(out_file), "--format", fmt]
            )
            assert code is None, f"CLI 不收清单内的 {fmt}"


class TestConvertInputFormat:
    """`convert --input-format`：把「训练格式 → 规范形」的反向边接到命令行。

    容器格式（sharegpt / vicuna / chatml）落盘也是 `.json`，**扩展名推不出源格式**，
    只能由调用方显式声明。不声明时按 json 原样读，而 `json → chatml` 这条边只认
    `instruction` / `history` 字段，`conversations` 整个被忽略。L16 加了声明入口，
    L21 补上了「没声明就直接失败」的护栏——以前它会静默产出全空问答且退出码 0。
    """

    def write(self, tmp_path, payload):
        path = tmp_path / "in.json"
        path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
        return path

    def test_sharegpt_file_converts_to_chatml(self, tmp_path):
        """声明 `--input-format sharegpt` 后，跨格式转换（经规范形中转）能跑通"""
        src = self.write(tmp_path, [{"conversations": [
            {"from": "human", "value": "如何退租"}, {"from": "gpt", "value": "满一年后退还"}]}])
        out_file = tmp_path / "out.json"

        _, parsed, code = run_cli(
            ["cli", "convert", "--input", str(src), "--output", str(out_file),
             "--input-format", "sharegpt", "--format", "chatml"]
        )
        assert code is None
        assert parsed["source_format"] == "sharegpt"
        assert parsed["input_count"] == 1
        assert json.loads(out_file.read_text(encoding="utf-8")) == [{
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "如何退租"},
                {"role": "assistant", "content": "满一年后退还"},
            ]}]

    def test_alpaca_file_returns_to_canonical_fields(self, tmp_path):
        """alpaca → json：源字段原样回来，不补 `input` 空列"""
        src = self.write(tmp_path, [{"instruction": "可以月付吗", "output": "支持月付",
                                     "category": "付款"}])
        out_file = tmp_path / "out.json"

        _, parsed, code = run_cli(
            ["cli", "convert", "--input", str(src), "--output", str(out_file),
             "--input-format", "alpaca", "--format", "json"]
        )
        assert code is None
        assert json.loads(out_file.read_text(encoding="utf-8")) == [
            {"instruction": "可以月付吗", "output": "支持月付", "category": "付款"}]

    def test_omitting_the_flag_fails_loud_instead_of_emptying_the_dataset(self, tmp_path, capsys):
        """不给 `--input-format` 时以非 0 退出，并把该声明什么写在报错里

        这条以前断言的是**坏结果**（退出码 0 + 全空问答），理由是「CLI 不能凭猜测
        改老调用的语义」。L21 改了：判据窄到「取不到问答 且 记录里有对话数组」，
        而这种产物的业务价值恒为 0，静默比失败更坏，所以宁可让老调用在这里停下。
        """
        src = self.write(tmp_path, [{"conversations": [
            {"from": "human", "value": "如何退租"}, {"from": "gpt", "value": "满一年后退还"}]}])
        out_file = tmp_path / "out.json"

        _, _, code = run_cli(
            ["cli", "convert", "--input", str(src), "--output", str(out_file),
             "--format", "chatml"]
        )
        err = capsys.readouterr().err
        assert code == 1, f"误标源格式应当以非 0 退出: code={code}"
        assert "`conversations`" in err and "--input-format" in err, err
        assert not out_file.exists()

        # 同一个文件声明后立刻能转，报错里的建议是真能照做的
        _, parsed, code = run_cli(
            ["cli", "convert", "--input", str(src), "--output", str(out_file),
             "--input-format", "sharegpt", "--format", "chatml"]
        )
        assert code is None
        assert parsed["output_count"] == 1
        assert json.loads(out_file.read_text(encoding="utf-8"))[0]["messages"][1] == {
            "role": "user", "content": "如何退租"}

    def test_plain_json_still_infers_from_the_extension(self, tmp_path):
        """不给 `--input-format` 的常规路径不变：按扩展名当 json 读、正常产出

        报错只针对「取不到问答 + 有对话数组」这一种误标，普通规范形数据不受影响。
        """
        src = self.write(tmp_path, [{"instruction": "如何退租", "output": "满一年后退还"}])
        out_file = tmp_path / "out.json"

        _, parsed, code = run_cli(
            ["cli", "convert", "--input", str(src), "--output", str(out_file),
             "--format", "chatml"]
        )
        assert code is None
        assert parsed["source_format"] == "json"
        assert json.loads(out_file.read_text(encoding="utf-8"))[0]["messages"][1] == {
            "role": "user", "content": "如何退租"}

    def test_bad_container_row_fails_with_row_number(self, tmp_path, capsys):
        """源文件里第 2 条缺 `output`：退出码非 0、报错带条目下标、且不留半截产物"""
        src = self.write(tmp_path, [{"instruction": "q", "output": "a"},
                                    {"instruction": "只有问题"}])
        out_file = tmp_path / "out.json"

        _, _, code = run_cli(
            ["cli", "convert", "--input", str(src), "--output", str(out_file),
             "--input-format", "alpaca", "--format", "json"]
        )
        assert code == 1, f"坏数据应当以非 0 退出: code={code}"
        assert "第 2 条 alpaca 记录缺少字段: output" in capsys.readouterr().err
        assert not out_file.exists()

    def test_tsv_is_accepted_on_both_sides(self, tmp_path):
        """`tsv` 两条边补齐后，CLI 两侧都收它，且引号规则与 csv 同严

        断言而不是只看退出码：`--input-format` 曾经根本不认 tsv，光看「不报错」
        分不清是支持了还是绕过了。字段里塞进制表符、逗号、换行各一个，严格解析器
        （`csv.reader`，非宽松模式）能原样读回才算真支持。
        """
        rows = [{"instruction": "含\t制表", "input": "含,逗号", "output": "含\n换行"},
                {"instruction": "正常", "input": "", "output": "回答"}]
        src = tmp_path / "in.tsv"
        with src.open("w", encoding="utf-8", newline="") as f:
            writer = csv.DictWriter(f, fieldnames=["instruction", "input", "output"],
                                    delimiter="\t")
            writer.writeheader()
            writer.writerows(rows)
        out_file = tmp_path / "out.json"

        _, parsed, code = run_cli(
            ["cli", "convert", "--input", str(src), "--output", str(out_file),
             "--input-format", "tsv", "--format", "json"]
        )
        assert code is None
        assert parsed["input_count"] == 2
        assert json.loads(out_file.read_text(encoding="utf-8")) == rows

        back = tmp_path / "back.tsv"
        _, _, code = run_cli(
            ["cli", "convert", "--input", str(out_file), "--output", str(back),
             "--format", "tsv"]
        )
        assert code is None
        with back.open(newline="", encoding="utf-8") as f:
            assert list(csv.reader(f, delimiter="\t"))[0] == [
                "instruction", "input", "output"]
        assert json.loads(out_file.read_text(encoding="utf-8")) == rows
