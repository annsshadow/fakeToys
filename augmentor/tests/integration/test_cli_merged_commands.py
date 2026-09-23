# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 合并命令族集成测试

覆盖 T1.7 合并过的那 8 个命令：`clean` / `export` / `analyze` / `stats` /
`visualize` / `compare` / `search` / `version`，以及未参与合并的 `quality-report`。

3.0 起这些命令**各自只剩一套实现**，`--enhanced` 开关已移除。因此本文件除了
逐命令的正常路径，还专门守住三件事：

1. `--enhanced` 必须被 argparse 拒绝（退出码 2），不能静默忽略；
2. 落败侧独有的能力必须仍然可用（`--formats` 批量导出、`--shuffle`、
   `--method ngram`、`version --action rollback` / `history`、
   `clean` 的 `remove_urls`、`analyze` 的 `coverage_analysis`、
   `compare` 的质量向结论 …）——否则「合并」就变成了静默的能力删除；
3. stdout 契约不变：报告型命令的 stdout 仍是机器可读的（`clean` / `analyze` /
   `stats` 为 JSON，`search` 为「摘要 + 条目 JSON 列表」，`compare` 为 markdown）。
   合并不得把 stdout 换成人类可读文本，那会让下游管道静默解析失败。
   `--output` 的提示语走 stderr。
"""

import json
import re
from pathlib import Path

import pytest

from cli import main

AI_DIR = Path(__file__).resolve().parent.parent.parent

SAMPLE_ITEMS = [
    {"instruction": "如何申请租房？", "input": "", "output": "登录官网申请"},
    {"instruction": "租房多少钱？", "input": "", "output": "按房型定价"},
    {"instruction": "如何退租押金？", "input": "", "output": "满一年后退还"},
    {"instruction": "可以申请月付吗？", "input": "", "output": "支持月付"},
    {"instruction": "租期最短多久？", "input": "", "output": "一个月起租"},
]
DIRTY_ITEMS = SAMPLE_ITEMS + [
    {},  # 全空条目，remove_empty 规则应移除
]


def run_cli(argv):
    """以给定参数运行 main()，返回 (stdout, stderr, exit_code)

    Args:
        argv: 完整参数列表（含程序名）

    Returns:
        (stdout, stderr, SystemExit 码或 None)
    """
    import io
    import sys
    from contextlib import redirect_stderr, redirect_stdout

    old_argv = sys.argv
    sys.argv = argv
    out, err = io.StringIO(), io.StringIO()
    exit_code = None
    try:
        with redirect_stdout(out), redirect_stderr(err):
            main()
    except SystemExit as exc:
        exit_code = exc.code
    finally:
        sys.argv = old_argv
    return out.getvalue(), err.getvalue(), exit_code


@pytest.fixture
def dataset_context(tmp_path, monkeypatch):
    """写入干净与脏两个数据集，切到 ai 目录

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (干净数据, 脏数据, 临时目录)
    """
    monkeypatch.chdir(AI_DIR)
    clean = tmp_path / "clean.json"
    dirty = tmp_path / "dirty.json"
    clean.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
    dirty.write_text(json.dumps(DIRTY_ITEMS, ensure_ascii=False), encoding="utf-8")
    return clean, dirty, tmp_path


class TestCleanCommand:
    def test_removes_dirty_items(self, dataset_context):
        """脏数据（全空条目）需被清洗规则移除"""
        _, dirty, tmp = dataset_context
        out_file = tmp / "cleaned.json"
        out, err, code = run_cli(
            ["cli", "clean", "--input", str(dirty), "--output", str(out_file)]
        )
        assert code is None, err
        items = json.loads(out_file.read_text(encoding="utf-8"))
        assert len(items) < len(DIRTY_ITEMS)

    def test_summary_counts_are_consistent(self, dataset_context):
        """摘要计数需满足 原始 = 清洗后 + 移除 的不变式"""
        _, dirty, _ = dataset_context
        out_file = Path(dirty.parent / "cleaned2.json")
        out, err, code = run_cli(
            ["cli", "clean", "--input", str(dirty), "--output", str(out_file)]
        )
        assert code is None, err
        summary = json.loads(out)
        assert summary["cleaned_count"] + summary["removed_count"] == \
            summary["original_count"]

    def test_default_rules_strip_urls(self, dataset_context):
        """默认规则必须移除 URL

        合并前「基础实现」（`data.cleaner.DataCleaner`）默认移除 URL。合并到
        规则式实现时若把 `remove_urls` 漏掉，`clean` 的默认行为就静默变了——
        用户不会收到任何提示，只会发现 URL 突然留在数据里。
        """
        _, _, tmp = dataset_context
        data = tmp / "with_url.json"
        data.write_text(
            json.dumps(
                [{"instruction": "见 https://example.com/a?b=1", "output": "ok"}],
                ensure_ascii=False,
            ),
            encoding="utf-8",
        )
        out_file = tmp / "stripped.json"
        out, err, code = run_cli(
            ["cli", "clean", "--input", str(data), "--output", str(out_file)]
        )
        assert code is None, err
        assert "https://example.com" not in out_file.read_text(encoding="utf-8")

    def test_keeping_urls_is_expressible_via_rules(self, dataset_context):
        """旧 `--no-url-removal` 的语义必须仍能表达

        等价写法是 `--rules` 里**不带** `remove_urls`。注意这不等于
        `remove_special_chars`：后者删的是「非中文/英文/数字/常用标点」的字符，
        只会把 `https://a.com` 削成 `https:a.com`，并不能移除 URL。所以这里断言
        保留 `remove_urls` 时 URL 原样留下，且**整条 URL 仍在**。
        """
        _, _, tmp = dataset_context
        data = tmp / "with_url.json"
        data.write_text(
            json.dumps(
                [{"instruction": "见 https://example.com/a?b=1", "output": "ok"}],
                ensure_ascii=False,
            ),
            encoding="utf-8",
        )
        kept = tmp / "kept.json"

        out, err, code = run_cli(
            ["cli", "clean", "--input", str(data), "--output", str(kept),
             "--rules", "remove_empty", "trim_whitespace"]
        )
        assert code is None, err
        assert "https://example.com/a?b=1" in kept.read_text(encoding="utf-8"), (
            "不带 remove_urls 时应原样保留 URL"
        )

    def test_unknown_rule_is_rejected(self, dataset_context):
        """拼错的规则名必须被 argparse 拒绝，而不是被静默忽略

        静默忽略会让人以为清洗跑过了，实际什么都没做。
        """
        _, dirty, tmp = dataset_context
        _, err, code = run_cli(
            ["cli", "clean", "--input", str(dirty), "--output", str(tmp / "x.json"),
             "--rules", "remove_empy"]  # 拼错
        )
        assert code == 2
        assert "remove_empy" in err


class TestExportCommand:
    @pytest.mark.parametrize("fmt", ["csv", "jsonl"])
    def test_single_file_formats(self, dataset_context, fmt):
        """单文件模式需按格式落盘且条数正确"""
        clean, _, tmp = dataset_context
        out_file = tmp / f"exp.{fmt}"
        out, err, code = run_cli(
            ["cli", "export", "--input", str(clean),
             "--output", str(out_file), "--format", fmt]
        )
        assert code is None, err
        assert out_file.exists()
        assert "导出数量: 5" in out

    def test_max_items_truncates(self, dataset_context):
        """--max-items 需截断导出数量"""
        clean, _, tmp = dataset_context
        out_file = tmp / "exp_trunc.json"
        out, err, code = run_cli(
            ["cli", "export", "--input", str(clean),
             "--output", str(out_file), "--max-items", "2"]
        )
        assert code is None, err
        assert "导出数量: 2" in out
        assert len(json.loads(out_file.read_text(encoding="utf-8"))) == 2

    def test_shuffle_is_deterministic_by_seed(self, dataset_context):
        """--shuffle --seed 需可复现（同种子同结果）

        原先只有 `--enhanced` 那条路支持打乱，且实现用的是全局 `random.seed()`，
        会污染同进程其它调用方的随机性。合并后改用局部 Random，因此这里除了
        可复现还顺带守住「不污染全局 RNG」。
        """
        clean, _, tmp = dataset_context
        first = tmp / "s1.json"
        second = tmp / "s2.json"

        for path in (first, second):
            out, err, code = run_cli(
                ["cli", "export", "--input", str(clean), "--output", str(path),
                 "--shuffle", "--seed", "7"]
            )
            assert code is None, err

        assert first.read_text(encoding="utf-8") == second.read_text(encoding="utf-8")

    def test_batch_mode_exports_every_format(self, dataset_context):
        """--output-dir + --formats 批量模式（原先只有基础实现支持）"""
        clean, _, tmp = dataset_context
        out_dir = tmp / "batch"
        out, err, code = run_cli(
            ["cli", "export", "--input", str(clean),
             "--output-dir", str(out_dir), "--formats", "json", "csv"]
        )
        assert code is None, err
        produced = sorted(p.name for p in out_dir.iterdir())
        assert produced == ["clean_csv.csv", "clean_json.json"], produced

    def test_requires_an_output_target(self, dataset_context):
        """既不给 --output 也不给 --output-dir 时需显式报错"""
        clean, _, _ = dataset_context
        _, err, code = run_cli(["cli", "export", "--input", str(clean)])
        assert code == 1
        assert "--output" in err

    def test_batch_mode_requires_formats(self, dataset_context):
        """--output-dir 缺少 --formats 时需显式报错（否则会静默只导一种格式）"""
        clean, _, tmp = dataset_context
        _, err, code = run_cli(
            ["cli", "export", "--input", str(clean), "--output-dir", str(tmp / "b")]
        )
        assert code == 1
        assert "--formats" in err

    def test_negative_max_items_is_rejected(self, dataset_context):
        clean, _, tmp = dataset_context
        _, err, code = run_cli(
            ["cli", "export", "--input", str(clean),
             "--output", str(tmp / "x.json"), "--max-items", "-1"]
        )
        assert code == 1
        assert "不能为负数" in err


class TestAnalyzeCommand:
    def test_prints_scores_and_coverage(self, dataset_context):
        """stdout 需是可解析 JSON，且同时含分数与覆盖分析

        覆盖分析（`coverage_analysis`）来自合并前的基础实现。它若在合并中丢掉，
        调用方拿 `report["coverage_analysis"]` 会 KeyError——所以这条断言是
        「合并不得变成能力删除」的守门。
        """
        clean, _, _ = dataset_context
        out, err, code = run_cli(["cli", "analyze", "--input", str(clean)])
        assert code is None, err
        parsed = json.loads(out)
        assert parsed["dataset_size"] == 5
        assert set(parsed["scores"]) == {"quality", "diversity", "completeness"}
        assert parsed["coverage_analysis"]["total_items"] == 5
        assert "statistics" in parsed
        assert "dedup_report" in parsed

    def test_saves_report(self, dataset_context):
        """--output 需写入与 stdout 同构的报告 JSON"""
        clean, _, tmp = dataset_context
        out_file = tmp / "analysis.json"
        out, err, code = run_cli(
            ["cli", "analyze", "--input", str(clean), "--output", str(out_file)]
        )
        assert code is None, err
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert parsed["dataset_size"] == 5
        assert "insights" in parsed
        assert "coverage_analysis" in parsed


class TestStatsCommand:
    def test_field_stats_and_totals(self, dataset_context):
        """stdout 需是可解析 JSON，含逐字段统计与总质量指标"""
        clean, _, tmp = dataset_context
        out, err, code = run_cli(["cli", "stats", "--input", str(clean)])
        assert code is None, err
        parsed = json.loads(out)
        assert parsed["total_items"] == 5
        assert "instruction" in parsed["field_statistics"]
        assert parsed["quality_metrics"]

    def test_saves_report(self, dataset_context):
        """--output 需写入与 stdout 同构的统计 JSON"""
        clean, _, tmp = dataset_context
        out_file = tmp / "stats.json"
        out, err, code = run_cli(
            ["cli", "stats", "--input", str(clean), "--output", str(out_file)]
        )
        assert code is None, err
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert parsed["total_items"] == 5


class TestSearchCommand:
    def test_fuzzy_method(self, dataset_context):
        """fuzzy 方法搜索需返回匹配摘要"""
        clean, _, _ = dataset_context
        out, err, code = run_cli(
            ["cli", "search", "--input", str(clean), "--query", "租房", "--method", "fuzzy"]
        )
        assert code is None, err
        assert "找到" in out

    def test_saves_output(self, dataset_context):
        """--output 需写入可解析的结果 JSON"""
        clean, _, tmp = dataset_context
        out_file = tmp / "se.json"
        out, err, code = run_cli(
            ["cli", "search", "--input", str(clean), "--query", "租房",
             "--output", str(out_file)]
        )
        assert code is None, err
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert "total_matches" in parsed

    def test_ngram_method_is_still_available(self, dataset_context):
        """`ngram` 必须仍可用

        它原先只存在于被合并掉的那套实现（`indexer.DatasetIndexer`）里，
        没有移植进 `EnhancedSearcher` 的话，合并后 `--method ngram` 会凭空消失。
        断言精确到条数，避免「方法名被忽略、退化成 contains」也蒙混过关。
        """
        clean, _, tmp = dataset_context
        out_file = tmp / "ngram.json"
        out, err, code = run_cli(
            ["cli", "search", "--input", str(clean), "--query", "租房",
             "--method", "ngram", "--output", str(out_file)]
        )
        assert code is None, err
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert parsed["method"] == "ngram", "方法名被忽略了"
        assert parsed["total_matches"] == 2, parsed["total_matches"]

    def test_offset_paginates(self, dataset_context):
        """--offset 需跳过前 N 条"""
        clean, _, tmp = dataset_context
        first = tmp / "p0.json"
        second = tmp / "p1.json"
        for path, offset in ((first, 0), (second, 1)):
            out, err, code = run_cli(
                ["cli", "search", "--input", str(clean), "--query", "租房",
                 "--method", "ngram", "--limit", "1", "--offset", str(offset),
                 "--output", str(path)]
            )
            assert code is None, err
        assert json.loads(first.read_text(encoding="utf-8"))["items"] != \
            json.loads(second.read_text(encoding="utf-8"))["items"]

    def test_items_are_json_on_stdout(self, dataset_context):
        """匹配条目必须以 JSON 列表打在 stdout

        合并前基础实现的 stdout 就是「摘要 + 条目 JSON 列表」。若合并后只剩摘要，
        条目就只能靠 `--output` 落盘再读——那是输出能力的静默缩水。
        """
        clean, _, _ = dataset_context
        out, err, code = run_cli(
            ["cli", "search", "--input", str(clean), "--query", "租房",
             "--method", "contains", "--field", "instruction"]
        )
        assert code is None, err
        items = json.loads(out[out.index("["):])
        assert len(items) == 2
        assert all("instruction" in item for item in items)


class TestVisualizeCommand:
    def test_report_mode(self, dataset_context):
        """传 --output 时走报告模式，落盘可解析 JSON"""
        clean, _, tmp = dataset_context
        out_file = tmp / "vis.json"
        out, err, code = run_cli(
            ["cli", "visualize", "--input", str(clean),
             "--output", str(out_file), "--format", "json"]
        )
        assert code is None, err
        assert json.loads(out_file.read_text(encoding="utf-8"))

    def test_chart_mode_is_selected_without_output(self, dataset_context, monkeypatch):
        """不传 --output 时走图表模式

        本环境没装 matplotlib / wordcloud，真实生成图表会得到空结果，无法区分
        「走了图表模式但缺依赖」和「根本没走图表模式」。因此这里把
        `generate_all_visualizations` 换成哨兵，直接断言模式选择。
        """
        clean, _, tmp = dataset_context
        from augmentor.visualizer import DataVisualizer

        seen = {}

        def fake_generate(self, items, *args, **kwargs):
            seen["output_dir"] = self.output_dir
            seen["count"] = len(items)
            return {"sentinel": "chart-mode"}

        monkeypatch.setattr(DataVisualizer, "generate_all_visualizations", fake_generate)

        out, err, code = run_cli(
            ["cli", "visualize", "--input", str(clean),
             "--output-dir", str(tmp / "charts")]
        )
        assert code is None, err
        assert seen["count"] == 5, "图表模式未拿到数据"
        assert Path(seen["output_dir"]) == tmp / "charts"
        assert "chart-mode" in out


class TestCompareCommand:
    def test_identical_datasets(self, dataset_context):
        """同数据集自比较需 100% 相似且共同数=总量"""
        clean, _, tmp = dataset_context
        out_file = tmp / "ce.json"
        out, err, code = run_cli(
            ["cli", "compare", "--dataset-a", str(clean), "--dataset-b", str(clean),
             "--output", str(out_file)]
        )
        assert code is None, err
        assert "100.00%" in out
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert parsed["overlap_comparison"]["metrics"]["common_items"] == 5
        assert parsed["overlap_comparison"]["metrics"]["unique_a"] == 0

    def test_different_datasets(self, dataset_context):
        """脏 vs 干净需报告 unique_b 非零"""
        clean, dirty, _ = dataset_context
        out, err, code = run_cli(
            ["cli", "compare", "--dataset-a", str(clean), "--dataset-b", str(dirty)]
        )
        assert code is None, err
        assert "仅在B中: 1" in out

    def test_reports_both_quality_and_overlap(self, dataset_context):
        """质量向 A/B 结论与重叠度结论必须**同时**出现

        合并前两条实现各给一半结论：基础实现给质量分 / 获胜方，增强实现给
        相似度 / 共同条数。只保留任一侧都会丢一半。
        """
        clean, dirty, _ = dataset_context
        out, err, code = run_cli(
            ["cli", "compare", "--dataset-a", str(clean), "--dataset-b", str(dirty)]
        )
        assert code is None, err
        assert "### 质量对比" in out, "缺少质量向 A/B 对比"
        assert "### 结论" in out, "缺少获胜方结论"
        assert "### 重叠度对比" in out, "缺少重叠度对比"

    def test_names_default_to_file_stems(self, dataset_context):
        """未给 --name-a / --name-b 时名称取文件名 stem

        否则报告里的名字会变成空串。
        """
        clean, dirty, _ = dataset_context
        out, err, code = run_cli(
            ["cli", "compare", "--dataset-a", str(clean), "--dataset-b", str(dirty)]
        )
        assert code is None, err
        assert "## 数据集对比: clean vs dirty" in out

    def test_explicit_names_override_stems(self, dataset_context):
        """--name-a / --name-b 需覆盖文件名 stem（旧基础实现的能力）"""
        clean, dirty, _ = dataset_context
        out, err, code = run_cli(
            ["cli", "compare", "--dataset-a", str(clean), "--dataset-b", str(dirty),
             "--name-a", "线上集", "--name-b", "候选集"]
        )
        assert code is None, err
        assert "## 数据集对比: 线上集 vs 候选集" in out


class TestVersionCommand:
    """单一后端 `DatasetVersionManager`（独立 --versions-dir）"""

    def _create(self, data, vdir, description=""):
        return run_cli(
            ["cli", "version", "--action", "create", "--input", str(data),
             "--versions-dir", str(vdir), "--description", description]
        )

    def test_create_list_load_roundtrip(self, dataset_context):
        clean, _, tmp = dataset_context
        vdir = tmp / "versions"

        out, err, code = self._create(clean, vdir, "初始版本")
        assert code is None, err
        assert "版本创建成功" in out
        version_id = re.search(r"版本ID: (\S+)", out).group(1)

        out, err, code = run_cli(
            ["cli", "version", "--action", "list", "--versions-dir", str(vdir)]
        )
        assert code is None, err
        assert version_id in out

        loaded = tmp / "loaded.json"
        out, err, code = run_cli(
            ["cli", "version", "--action", "load", "--version", version_id,
             "--versions-dir", str(vdir), "--output", str(loaded)]
        )
        assert code is None, err
        assert len(json.loads(loaded.read_text(encoding="utf-8"))) == 5

    def test_list_empty(self, dataset_context):
        _, _, tmp = dataset_context
        out, err, code = run_cli(
            ["cli", "version", "--action", "list", "--versions-dir", str(tmp / "none")]
        )
        assert code is None, err
        assert "没有找到版本" in out

    def test_compare_requires_version(self, dataset_context):
        """compare 缺 --version 且无当前版本时需报错"""
        _, _, tmp = dataset_context
        _, err, code = run_cli(
            ["cli", "version", "--action", "compare", "--versions-dir", str(tmp / "v")]
        )
        assert code == 1
        assert "--version" in err

    def test_compare_full_flow(self, dataset_context):
        """compare 缺省与当前版本比较（旧 diff 的等价用法）"""
        clean, dirty, tmp = dataset_context
        vdir = tmp / "versions"

        out, err, code = self._create(clean, vdir)
        assert code is None, err
        v1 = re.search(r"版本ID: (\S+)", out).group(1)

        out, err, code = self._create(dirty, vdir)
        assert code is None, err
        v2 = re.search(r"版本ID: (\S+)", out).group(1)

        # 当前版本应是刚创建的 v2；与 v1 比较
        out, err, code = run_cli(
            ["cli", "version", "--action", "compare", "--version", v1,
             "--versions-dir", str(vdir)]
        )
        assert code is None, err
        assert "仅在B中:" in out and "共同数据:" in out
        assert v1 in out and v2 in out

    def test_rollback_sets_current_version(self, dataset_context):
        """`rollback` 必须可用（映射到 set_current_version）

        它原先只存在于被合并掉的那套 pipeline 后端上，没映射过来就会变成
        argparse 的「无效选择」，用户会以为功能被删了。
        """
        clean, dirty, tmp = dataset_context
        vdir = tmp / "versions"

        out, err, code = self._create(clean, vdir)
        assert code is None, err
        v1 = re.search(r"版本ID: (\S+)", out).group(1)

        out, err, code = self._create(dirty, vdir)
        assert code is None, err

        out, err, code = run_cli(
            ["cli", "version", "--action", "rollback", "--version", v1,
             "--versions-dir", str(vdir)]
        )
        assert code is None, err
        assert f"已回滚到 {v1}" in out

        from augmentor.version_control import DatasetVersionManager

        assert DatasetVersionManager(str(vdir)).get_current_version() == v1

    def test_rollback_unknown_version_fails(self, dataset_context):
        _, _, tmp = dataset_context
        _, err, code = run_cli(
            ["cli", "version", "--action", "rollback", "--version", "nope",
             "--versions-dir", str(tmp / "v")]
        )
        assert code == 1
        assert "回滚失败" in err

    def test_create_without_input_fails(self, dataset_context):
        _, _, tmp = dataset_context
        _, err, code = run_cli(
            ["cli", "version", "--action", "create", "--versions-dir", str(tmp / "v")]
        )
        assert code == 1
        assert "--input" in err

    def test_load_requires_version_and_output(self, dataset_context):
        _, _, tmp = dataset_context
        _, err, code = run_cli(
            ["cli", "version", "--action", "load", "--versions-dir", str(tmp / "v")]
        )
        assert code == 1
        assert "--version" in err

    def test_history_records_operations(self, dataset_context):
        """`history` 必须可用，且记录的是「做过什么操作」

        它原先只存在于被合并掉的 pipeline 后端上。`list` 给的是「现在有哪些版本」，
        两者是不同数据，所以不能用 `list` 顶替 `history`。这里断言操作类型与版本
        一一对应，避免退化成一个「把 list 重命名」的空壳。
        """
        clean, dirty, tmp = dataset_context
        vdir = tmp / "versions"

        out, err, code = self._create(clean, vdir)
        assert code is None, err
        v1 = re.search(r"版本ID: (\S+)", out).group(1)

        out, err, code = self._create(dirty, vdir)
        assert code is None, err
        v2 = re.search(r"版本ID: (\S+)", out).group(1)

        out, err, code = run_cli(
            ["cli", "version", "--action", "rollback", "--version", v1,
             "--versions-dir", str(vdir)]
        )
        assert code is None, err

        out, err, code = run_cli(
            ["cli", "version", "--action", "history", "--versions-dir", str(vdir)]
        )
        assert code is None, err
        history = json.loads(out)
        assert [e["action"] for e in history] == ["set_current", "create", "create"]
        assert history[0]["version_id"] == v1
        assert history[1]["version_id"] == v2

    def test_history_empty(self, dataset_context):
        """没有操作记录时 history 返回空列表而非报错"""
        _, _, tmp = dataset_context
        out, err, code = run_cli(
            ["cli", "version", "--action", "history", "--versions-dir", str(tmp / "none")]
        )
        assert code is None, err
        assert json.loads(out) == []


class TestQualityReportCommand:
    def test_markdown(self, dataset_context):
        """markdown 格式报告需落盘且含标题"""
        clean, _, tmp = dataset_context
        out_file = tmp / "qr.md"
        out, err, code = run_cli(
            ["cli", "quality-report", "--input", str(clean),
             "--output", str(out_file), "--format", "markdown"]
        )
        assert code is None, err
        assert out_file.exists()
        assert "总体" in out or "评分" in out


class TestStdoutStaysMachineReadableWithOutput:
    """`--output` 落盘时 stdout 仍必须是纯报告

    `--output` 只是「同一份内容另外存一份」，它的提示语必须写 stderr。若提示语
    进了 stdout，`clean/analyze/stats | jq` 这类管道会在加了 `--output` 之后突然
    解析失败——这种破坏最难排查，因为它只在加了某个参数时才出现。
    """

    @pytest.mark.parametrize("command", ["clean", "analyze", "stats"])
    def test_stdout_is_pure_json(self, dataset_context, command):
        clean, _, tmp = dataset_context
        out_file = tmp / f"{command}_out.json"
        argv = ["cli", command, "--input", str(clean), "--output", str(out_file)]
        out, err, code = run_cli(argv)
        assert code is None, err
        json.loads(out)  # 混进任何提示语都会在这里抛 JSONDecodeError
        assert "已保存到" not in out, "落盘提示语混进了 stdout"

    def test_save_notice_goes_to_stderr(self, dataset_context):
        """落盘提示语必须在 stderr，不能在 stdout 里"""
        clean, _, tmp = dataset_context
        out_file = tmp / "analysis_notice.json"
        out, err, code = run_cli(
            ["cli", "analyze", "--input", str(clean), "--output", str(out_file)]
        )
        assert code is None, err
        assert "已保存到" in err
        assert "已保存到" not in out

    def test_search_stdout_keeps_summary_then_json(self, dataset_context):
        clean, _, tmp = dataset_context
        out_file = tmp / "search_out.json"
        out, err, code = run_cli(
            ["cli", "search", "--input", str(clean), "--query", "租房",
             "--method", "contains", "--field", "instruction",
             "--output", str(out_file)]
        )
        assert code is None, err
        json.loads(out[out.index("["):])
        assert "已保存到" in err


class TestLegacyCommandNamesAreRemoved:
    """3.0 移除了 8 个旧命令名。

    旧名不再注册为 argparse alias，因此解析阶段就直接报「无效选择」并以
    退出码 2 结束。这比静默归一化更安全：用户不会以为自己还在用旧行为，
    实际已经被换到了另一份实现上。
    """

    LEGACY = [
        "export-enhanced", "analyze-data", "visualize-data", "version-control",
        "compare-enhanced", "search-enhanced", "clean-enhanced", "stats-enhanced",
    ]

    def test_legacy_names_are_not_registered(self):
        """解析器的 choices 里不得再有旧名"""
        from augmentor.cli.parser import build_parser

        parser = build_parser()
        sub = next(
            a for a in parser._actions
            if getattr(a, "choices", None) and "export" in a.choices
        )
        for legacy in self.LEGACY:
            assert legacy not in sub.choices, f"`{legacy}` 仍被注册为命令/别名"

    def test_legacy_mapping_table_is_gone(self):
        """`LEGACY_ALIASES` 映射表本身也必须删除，避免死代码回流"""
        import augmentor.cli.parser as parser_mod

        assert not hasattr(parser_mod, "LEGACY_ALIASES")

    def test_legacy_name_is_rejected_with_exit_code_2(self, dataset_context):
        """用旧名调用必须失败，且不得产生任何输出文件"""
        _, dirty, tmp = dataset_context
        out_file = tmp / "should_not_exist.json"

        out, err, code = run_cli(
            ["cli", "clean-enhanced", "--input", str(dirty), "--output", str(out_file)]
        )

        assert code == 2, f"旧名未被拒绝: code={code}, err={err!r}"
        assert not out_file.exists(), "旧名仍然被映射到了实现并写了文件"

    def test_canonical_commands_have_no_misleading_suffix(self):
        """规范命令名里不应再出现 `-enhanced` / `-data` / `-control` 这类后缀

        `quality-report` 不在被禁之列：它是真实能力名（生成报告），
        与 `quality`（筛数据）是两件事，T1.7 刻意没有合并它们。
        """
        from augmentor.cli.commands import COMMANDS

        assert "quality-report" in COMMANDS, "quality-report 不应被合并掉"
        for name in COMMANDS:
            assert not name.endswith("-enhanced"), f"{name} 是应被合并的旧名"
            assert name not in {"analyze-data", "visualize-data", "version-control"}, (
                f"{name} 是应被合并的旧名"
            )


class TestEnhancedFlagIsRemoved:
    """3.0 移除了 `--enhanced` 开关。

    这个开关是「同一能力两套实现」的最后残留：它不影响能力，只影响走哪条代码
    路径。移除后传它必须被 argparse 拒绝（退出码 2），而不是被静默忽略——
    静默忽略会让用户以为自己在跑另一套实现。
    """

    # 每个命令的最小必填参数（不含 --enhanced）
    MINIMAL = {
        "export": ["--input", "data.json"],
        "clean": ["--input", "data.json", "--output", "out.json"],
        "analyze": ["--input", "data.json"],
        "stats": ["--input", "data.json"],
        "visualize": ["--input", "data.json"],
        "compare": ["--dataset-a", "a.json", "--dataset-b", "b.json"],
        "search": ["--input", "data.json", "--query", "x"],
        "version": ["--action", "list"],
    }

    def test_help_constant_is_gone(self):
        import augmentor.cli.parser as parser_mod

        assert not hasattr(parser_mod, "ENHANCED_HELP"), "ENHANCED_HELP 是死常量"

    @pytest.mark.parametrize("command", sorted(MINIMAL))
    def test_enhanced_flag_is_rejected(self, dataset_context, command):
        argv = ["cli", command, *self.MINIMAL[command], "--enhanced"]
        _, err, code = run_cli(argv)
        assert code == 2, f"`{command} --enhanced` 未被拒绝: code={code}"
        assert "--enhanced" in err, f"错误信息未指出 --enhanced: {err!r}"
