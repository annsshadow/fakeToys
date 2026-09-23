# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""`augmentor.streaming` 剩余边界分支测试

已有的 `test_streaming.py` / `test_streaming_preview_branches.py` /
`test_streaming_sampler_branches.py` 覆盖了主流程，但以下几条边界一直空着：

* `_peek_first_non_space` 的**分块**读取路径。既有用例的数据都以非空白字符
  开头，所以「首字符是空白」「前 4096 个字符全是空白」「整个流都是空白」
  这三条分支从未被走到 —— 而它们正是**大文件带 BOM/缩进**时的真实路径。
* `_is_single_json_value` 的补缓冲路径（`buf += more`）。要触发它必须让
  单个 JSON 值跨越 `read_size` 边界，默认 64KB 的缓冲在测试里不会跨。
* `_iter_json_array_items` 的两种「文件被截断」语义：数组未闭合但元素完整
  （按已读到的元素结束）与元素本身被截断（必须报错）。两者**行为相反**，
  合并成一条断言就会丢掉其中一个。
* JSONL 读取路径里的空行跳过。
* `StreamWriter` 的 `__exit__` 未开即退、以及非 json/w 模式下的 `close()`。

这些分支不影响主流程正确性，但一旦被改动，`fail loud` 的承诺就会破一个口子。
"""

import io
import json

import pytest

from augmentor.exceptions import DataFormatError
from augmentor.streaming import (
    StreamReader,
    StreamWriter,
    _is_single_json_value,
    _iter_json_array_items,
    _peek_first_non_space,
)

ITEMS = [
    {"instruction": "问题1", "input": "", "output": "回答1"},
    {"instruction": "问题2", "input": "", "output": "回答2"},
]


def _jsonl(items) -> str:
    """把数据列表渲染成 JSONL 文本

    Args:
        items: 数据列表

    Returns:
        JSONL 文本
    """
    return "".join(json.dumps(x, ensure_ascii=False) + "\n" for x in items)


class TestPeekFirstNonSpace:
    """`_peek_first_non_space` 的分块读取路径"""

    def test_empty_stream_returns_none(self):
        """空流 → None（而不是抛 StopIteration 或死循环）"""
        assert _peek_first_non_space(io.StringIO("")) is None

    def test_whitespace_only_stream_returns_none(self):
        """纯空白流 → None

        这条路径要求 `read()` 一直有返回、但全是空白，直到流耗尽。
        """
        assert _peek_first_non_space(io.StringIO("   \n\t  \r\n ")) is None

    def test_leading_whitespace_returns_first_real_char(self):
        """首字符是空白时继续向后找"""
        assert _peek_first_non_space(io.StringIO("  \n\t[1, 2]")) == "["

    def test_whitespace_spanning_multiple_reads(self):
        """前 read_size 个字符全是空白 → 需要再读一块

        对应「文件带大段缩进 / BOM 前导空白」的真实场景。用小 read_size
        构造跨块，避免真的写 64KB 数据。
        """
        stream = io.StringIO(" " * 20 + '{"a": 1}')
        assert _peek_first_non_space(stream, read_size=8) == "{"


class TestIsSingleJsonValue:
    """`_is_single_json_value` 的补缓冲路径"""

    def test_single_value_split_across_reads(self):
        """单个 JSON 值跨越 read_size 边界 → 必须补缓冲后判定成功"""
        stream = io.StringIO('{"key": "一个比较长的值", "n": 1}')
        assert _is_single_json_value(stream, read_size=5) is True

    def test_whitespace_only_stream_is_not_single_value(self):
        """纯空白流 → False（在补缓冲循环里撞到 eof）"""
        assert _is_single_json_value(io.StringIO("   \n  "), read_size=4) is False

    def test_leading_whitespace_before_value(self):
        """值前面有空白时也要能正确跳过"""
        stream = io.StringIO('   {"a": 1}')
        assert _is_single_json_value(stream, read_size=4) is True

    def test_trailing_whitespace_split_across_reads(self):
        """值之后的尾随空白跨越 read_size 边界 → 第三段循环里补缓冲

        这条路径决定「值后面还有没有第二个值」，直接关系到 JSONL 判定，
        因此不能因为「空白还没读完」就提前返回。
        """
        stream = io.StringIO('{"a": 1}' + " " * 40)
        assert _is_single_json_value(stream, read_size=6) is True

    def test_two_values_is_not_single_value(self):
        """两个值 → False（JSONL 判定）"""
        stream = io.StringIO(_jsonl(ITEMS))
        assert _is_single_json_value(stream, read_size=8) is False


class TestJsonArrayTruncation:
    """`_iter_json_array_items` 的两种截断语义"""

    def test_unclosed_array_after_complete_elements(self):
        """数组未闭合但元素完整 → 按已读到的元素结束，不报错"""
        assert list(_iter_json_array_items(io.StringIO('[{"a": 1}, {"b": 2}'))) == [
            {"a": 1},
            {"b": 2},
        ]

    def test_truncated_element_raises(self):
        """元素本身被截断 → 必须报错（与上一条相反，不能静默丢数据）"""
        with pytest.raises(json.JSONDecodeError):
            list(_iter_json_array_items(io.StringIO('[{"a": ')))

    def test_non_array_stream_raises_data_format_error(self):
        """不是以 `[` 开头 → DataFormatError（函数契约里写明的前提）"""
        with pytest.raises(DataFormatError, match="以 '\\[' 开头"):
            list(_iter_json_array_items(io.StringIO(_jsonl(ITEMS))))

    def test_elements_spanning_read_size(self):
        """元素跨越 read_size 边界时仍能正确切分"""
        payload = "[" + ", ".join(json.dumps(x, ensure_ascii=False) for x in ITEMS) + "]"
        assert list(_iter_json_array_items(io.StringIO(payload), read_size=7)) == ITEMS


class TestCountItemsEdgeBranches:
    """`StreamReader._count_items` 的边界输入"""

    def test_empty_file_counts_zero(self, tmp_path):
        """空文件 → 0 条（走 `first is None` 分支）"""
        path = tmp_path / "empty.json"
        path.write_text("", encoding="utf-8")
        assert StreamReader(str(path)).total_count == 0

    def test_whitespace_only_file_counts_zero(self, tmp_path):
        """纯空白文件 → 0 条"""
        path = tmp_path / "blank.json"
        path.write_text("   \n\t\n", encoding="utf-8")
        assert StreamReader(str(path)).total_count == 0

    def test_leading_whitespace_before_array(self, tmp_path):
        """数组前有空白（缩进 / 换行）不影响格式判定与计数"""
        path = tmp_path / "indented.json"
        path.write_text("\n  " + json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
        reader = StreamReader(str(path))
        assert reader.total_count == len(ITEMS)
        assert list(reader.read_chunks()) == [ITEMS]


class TestJsonlReadBranches:
    """JSONL 读取路径的空行与坏行"""

    def test_blank_lines_are_skipped(self, tmp_path):
        """JSONL 中间的空行不产生空记录"""
        path = tmp_path / "with_blanks.jsonl"
        path.write_text(
            _jsonl(ITEMS[:1]) + "\n   \n" + _jsonl(ITEMS[1:]), encoding="utf-8"
        )
        assert list(StreamReader(str(path)).read_chunks()) == [ITEMS]

    def test_chunk_size_splits_jsonl(self, tmp_path):
        """JSONL 也按 chunk_size 分块产出"""
        path = tmp_path / "chunked.jsonl"
        path.write_text(_jsonl(ITEMS), encoding="utf-8")
        chunks = list(StreamReader(str(path), chunk_size=1).read_chunks())
        assert chunks == [[ITEMS[0]], [ITEMS[1]]]


class TestStreamWriterBranches:
    """`StreamWriter` 的收尾分支"""

    def test_exit_without_enter_is_safe(self, tmp_path):
        """未开即退（`__exit__` 时 `_file` 为 None）不得抛异常"""
        writer = StreamWriter(str(tmp_path / "unopened.json"))
        writer.__exit__(None, None, None)  # 不应抛异常

    def test_context_manager_writes_valid_json(self, tmp_path):
        """上下文管理器退出时补上右括号，产物是合法 JSON"""
        path = tmp_path / "ctx.json"
        with StreamWriter(str(path)) as writer:
            writer.write_chunk(ITEMS)
        assert json.loads(path.read_text(encoding="utf-8")) == ITEMS

    def test_close_jsonl_writer_does_not_add_bracket(self, tmp_path):
        """jsonl 模式关闭时不补 `]`

        注意 `close()` 必须在 `with` 块**内**调用：出了块 `__exit__` 已经把
        `_file` 置空，再调 `close()` 就只会走「已关闭」的短路分支，测不到
        「打开着但不是 json/w」这条真正的收尾路径。
        """
        path = tmp_path / "out.jsonl"
        with StreamWriter(str(path), format="jsonl") as writer:
            writer.write_chunk(ITEMS)
            writer.close()
        assert writer._file is None
        lines = [ln for ln in path.read_text(encoding="utf-8").splitlines() if ln.strip()]
        assert [json.loads(ln) for ln in lines] == ITEMS

    def test_close_append_mode_does_not_add_bracket(self, tmp_path):
        """json 格式但追加模式 → 不补 `[` / `]`，避免破坏已有内容"""
        path = tmp_path / "append.json"
        path.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
        writer = StreamWriter(str(path), mode="a", format="json")
        with writer:
            writer.write_chunk([ITEMS[0]])
        assert path.read_text(encoding="utf-8").startswith("[")
