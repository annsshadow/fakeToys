# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A145 的收口判据：一次流式请求把输入文件**恰好打开一次**

L85 的重复 I/O 普查量到 `StreamAugmentor(...).augment()` 一次产品请求对同一份输入
`open` 两次（`StreamProcessor.__init__` 在构造期为一条进度日志取 `reader.total_count`
⇒ 整份文件先被解析一遍，随后 `read_chunks()` 再解析一遍），而第一趟只产出一个整数。

本轮把条数改成边读边数，于是需要两件事同时成立，缺一条都可能是假绿：

1. **I/O 面**：一次请求对输入路径 `open` 恰好 1 次（计数器必须自带对照组，否则
   「计数为 1」可能只是计数器没看见）；
2. **语义面**：`total_input` 这条读数**逐文件形状**与旧的独立计数完全相同
   —— 只搬 I/O 不改语义，语义若真的变了必须在这里当场红。

纪律 (f)：判据双向。第三族把「预先取数」那条老路径显式跑一遍，证明计数器确实会
因为多一趟解析而计到 2 —— 也就是说第一族的 1 是被判出来的，不是量不出。
"""

import builtins
import json
from collections import Counter
from pathlib import Path

import pytest

from augmentor.streaming import StreamAugmentor, StreamProcessor, StreamReader


@pytest.fixture
def open_spy(monkeypatch):
    """在 `builtins.open` 上挂一个只记录、不吞错的代理

    代理本身**不做 try/except**：被观测代码出错时必须照常抛出，否则「计到 1 次」
    可能来自「第二次 open 之前就先炸了」（纪律 (u)）。
    """
    real_open = builtins.open
    calls = []

    def spy(file, *args, **kwargs):
        if isinstance(file, (str, bytes)) or hasattr(file, "__fspath__"):
            calls.append(str(Path(file).resolve()))
        else:
            # 文件描述符之类：不记路径，但照常放行
            calls.append(f"<fd:{file}>")
        return real_open(file, *args, **kwargs)

    monkeypatch.setattr(builtins, "open", spy)
    return calls


def _count_for(calls, path):
    return Counter(calls).get(str(Path(path).resolve()), 0)


def _identity(items):
    return items


class TestExactlyOneOpenPerRequest:
    """I/O 面：一次请求对输入路径恰好 open 1 次"""

    @pytest.mark.parametrize(
        "shape",
        ["json_array", "jsonl", "jsonl_with_blank_and_broken"],
    )
    def test_augment_opens_input_once(self, tmp_path, open_spy, shape):
        inp = tmp_path / "in.json"
        out = tmp_path / "out.jsonl"
        _write_shape(inp, shape)

        augmentor = StreamAugmentor(
            str(inp), str(out), _identity, chunk_size=3, output_format="jsonl"
        )
        report = augmentor.augment()

        assert _count_for(open_spy, inp) == 1, open_spy
        assert _count_for(open_spy, out) == 1, open_spy
        assert report["processed"] == _SHAPE_PROCESSED[shape]

    def test_control_group_sees_two_opens_when_reading_twice(self, tmp_path, open_spy):
        """对照组：手工把同一份文件读两遍 ⇒ 计数器确实计到 2

        没有这一条，上一族的「1」无法区分「只开了一次」与「计数器看不见」。
        """
        inp = tmp_path / "in.json"
        _write_shape(inp, "jsonl")
        for _ in range(2):
            with open(inp, "r", encoding="utf-8") as f:
                f.read()
        assert _count_for(open_spy, inp) == 2

    def test_prefetching_total_count_costs_a_second_open(self, tmp_path, open_spy):
        """反向注入本轮删掉的那一步：构造前预先取数 ⇒ 输入被开 2 次

        这就是 L85 普查量到的形状。判据必须能把它打红，否则「恰好一次」是空话。
        """
        inp = tmp_path / "in.json"
        out = tmp_path / "out.jsonl"
        _write_shape(inp, "jsonl")

        reader = StreamReader(str(inp), chunk_size=3)
        assert reader.total_count == _SHAPE_ITEMS["jsonl"]  # 独立那一趟
        writer_path = out
        with open(writer_path, "w", encoding="utf-8") as sink:
            for chunk in reader.read_chunks():
                json.dump(chunk, sink)
                sink.write("\n")
        assert _count_for(open_spy, inp) == 2, open_spy

    def test_constructing_processor_does_not_touch_the_file(self, tmp_path, open_spy):
        inp = tmp_path / "in.json"
        _write_shape(inp, "jsonl")
        StreamProcessor(StreamReader(str(inp), chunk_size=3), _identity)
        assert open_spy == [], open_spy

    def test_process_then_total_count_is_free(self, tmp_path, open_spy):
        """完整读完之后取 `total_count` 不再付 I/O —— 缓存必须由那一趟填上"""
        inp = tmp_path / "in.json"
        _write_shape(inp, "jsonl")
        reader = StreamReader(str(inp), chunk_size=3)
        report = StreamProcessor(reader, _identity).process()
        before = _count_for(open_spy, inp)
        assert before == 1, open_spy
        assert reader.total_count == report["total_input"] == _SHAPE_ITEMS["jsonl"]
        assert _count_for(open_spy, inp) == 1, open_spy


# 每种形状的「条数」权威读数（计数面）：与 `_count_items` 的分支规则一一对应
_SHAPE_ITEMS = {
    "json_array": 7,
    "jsonl": 4,
    "jsonl_with_blank_and_broken": 6,  # 5 合法 + 1 坏 + 2 空行（空行不计数，坏行计而不进产物）
    "empty": 0,
    "whitespace_only": 0,
    "single_json_object_one_line": 0,
    "single_json_object_pretty": 0,
    "trailing_comma_array": 3,
}

# 处理面读数（真正进产物的条数）：本轮一格未动，但必须与计数面**分开钉住**——
# `single_json_object_one_line` 那格是既有的「计数 0 / 处理 1」分叉，
# 它不是本轮造成的，本轮只搬 I/O，所以在这里把它写死而不是悄悄抹平。
_SHAPE_PROCESSED = {
    "json_array": 7,
    "jsonl": 4,
    "jsonl_with_blank_and_broken": 5,
    "empty": 0,
    "whitespace_only": 0,
    "single_json_object_one_line": 1,
    "single_json_object_pretty": 0,
    "trailing_comma_array": 3,
}


def _write_shape(path, shape):
    if shape == "json_array":
        path.write_text(
            json.dumps([{"instruction": f"q{i}"} for i in range(7)]), encoding="utf-8"
        )
    elif shape == "jsonl":
        path.write_text(
            "".join(json.dumps({"instruction": f"q{i}"}) + "\n" for i in range(4)),
            encoding="utf-8",
        )
    elif shape == "jsonl_with_blank_and_broken":
        lines = [json.dumps({"instruction": f"q{i}"}) for i in range(5)]
        lines.insert(2, "")
        lines.insert(4, "{not json")
        lines.append("")
        path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    elif shape == "empty":
        path.write_text("", encoding="utf-8")
    elif shape == "whitespace_only":
        path.write_text("  \n\t\n", encoding="utf-8")
    elif shape == "single_json_object_one_line":
        path.write_text(json.dumps({"a": 1}), encoding="utf-8")
    elif shape == "single_json_object_pretty":
        path.write_text(json.dumps({"a": 1, "b": {"c": 2}}, indent=2), encoding="utf-8")
    elif shape == "trailing_comma_array":
        path.write_text('[{"a": 1}, {"a": 2}, {"a": 3}, ]', encoding="utf-8")
    else:  # pragma: no cover - 参数表写错时立刻出声
        raise AssertionError(f"未知形状: {shape}")


class TestFusedCountMatchesStandaloneCount:
    """语义面：边读边数与独立那一趟**逐形状同号**，一条都不许漂"""

    @pytest.mark.parametrize("shape", sorted(_SHAPE_ITEMS))
    def test_total_input_equals_standalone_count(self, tmp_path, shape):
        inp = tmp_path / "in.json"
        out = tmp_path / "out.jsonl"
        _write_shape(inp, shape)

        # 先由 `_count_items`（独立那一趟）定基准
        baseline = StreamReader(str(inp), chunk_size=2).total_count
        # 再让融合计数的一趟产出报告
        fused = StreamAugmentor(str(inp), str(out), _identity, 2, "jsonl").augment()

        assert baseline == _SHAPE_ITEMS[shape], shape
        assert fused["total_input"] == baseline, shape

    @pytest.mark.parametrize("shape", sorted(_SHAPE_ITEMS))
    def test_processed_and_output_are_unchanged(self, tmp_path, shape):
        """处理面逐形状同号（本轮一格未动），并把计数面与处理面的**分叉**写死"""
        inp = tmp_path / "in.json"
        out = tmp_path / "out.jsonl"
        _write_shape(inp, shape)
        report = StreamAugmentor(str(inp), str(out), _identity, 2, "jsonl").augment()
        assert report["total_input"] == _SHAPE_ITEMS[shape], report
        assert report["processed"] == _SHAPE_PROCESSED[shape], report
        assert report["total_output"] == report["processed"], report


class TestAbandonedPassDoesNotWriteCache:
    """弃读那一趟不许留下半截计数"""

    def test_break_after_first_chunk_leaves_cache_empty_then_recounts(
        self, tmp_path, open_spy
    ):
        inp = tmp_path / "in.json"
        _write_shape(inp, "jsonl_with_blank_and_broken")
        reader = StreamReader(str(inp), chunk_size=2)
        gen = reader.read_chunks()
        first = next(gen)
        assert len(first) == 2
        assert reader._total_count is None
        gen.close()

        # 回落路径必须仍然给出权威读数（第二趟，这是弃读的代价，不是产品路径）
        assert reader.total_count == 6
        assert _count_for(open_spy, inp) == 2, open_spy

    def test_single_value_cache_is_a_file_property_not_a_partial_read(self, tmp_path):
        """单值形状在产出第一条**之前**就已写下 0 —— 那是整份文件的性质，不是半截读数"""
        inp = tmp_path / "in.json"
        _write_shape(inp, "single_json_object_one_line")
        reader = StreamReader(str(inp), chunk_size=2)
        gen = reader.read_chunks()
        assert len(next(gen)) == 1
        assert reader._total_count == 0
        gen.close()
        assert StreamReader(str(inp), chunk_size=2).total_count == 0

    def test_array_branch_also_writes_cache_only_after_completion(
        self, tmp_path, open_spy
    ):
        """数组分支的缓存同样只能在读完那一刻写：开读就写 0 会把弃读当成「文件是空的」

        上一条用例走的是 JSONL 分支，数组分支是另一段代码（`_iter_json_array_items`
        + 循环后赋值），注入分判里单独摘出的 s4 档就是「把赋值提到循环之前」——
        只有一条分支被盯住不算收口。
        """
        inp = tmp_path / "in.json"
        _write_shape(inp, "json_array")
        reader = StreamReader(str(inp), chunk_size=3)
        gen = reader.read_chunks()
        assert len(next(gen)) == 3
        assert reader._total_count is None
        gen.close()

        # 弃读后仍须回落到权威读数，代价是第二次 open（与上一条用例同口径）
        assert reader.total_count == _SHAPE_ITEMS["json_array"]
        assert _count_for(open_spy, inp) == 2, open_spy


class TestProgressLogStillReports:
    """进度日志换了形状但没丢信息：跑前逐块报已处理，跑完报总数"""

    def test_logs_processed_then_total(self, tmp_path, caplog):
        inp = tmp_path / "in.json"
        out = tmp_path / "out.jsonl"
        _write_shape(inp, "json_array")
        with caplog.at_level("INFO", logger="augmentor.streaming"):
            StreamAugmentor(str(inp), str(out), _identity, 3, "jsonl").augment()
        text = caplog.text
        assert "已处理 3 条" in text, text
        assert "流式处理完成：输入 7 条，已处理 7 条" in text, text
        # 旧形状里那个「/总数」的分母正是被删掉的那一趟 I/O，不该再出现
        assert "/None" not in text, text
