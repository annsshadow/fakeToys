# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L201（B266①）：流式写入器不再把写崩的产物伪装成半份合法 JSON

**缺陷（探针实测产物字节）**：`StreamWriter` 的 `write_chunk` 是「先写分隔符 `,`
再 `json.dumps(item)`」，而 `__exit__` 无条件补 `]`。于是一条记录序列化失败时：

    盘上字节 = '[{"a": 1},]'      # json.loads ⇒ Illegal trailing comma

用户先看到 `TypeError: Object of type Boom is not JSON serializable`，回头看文件
还以为「写到一半了」—— 实际是**永久损坏**，且没有任何信号说明它非法。

**两处必须一起改**（分开任何一半都还会坏）：
1. `write_chunk` 改成**先序列化再写分隔符** ⇒ 崩的时候盘上没有悬挂逗号；
2. `__exit__` 在 `exc_type is not None` 时**不补**闭合括号 ⇒ 盘上是明确的截断
   形态，而调用方本来就要处理正在传播的那个异常。

**口径**：这不是「失败也要产出合法文件」—— 崩了就是崩了。这里争取的是
**崩掉的产物不许看起来像好的**：不补括号的 `'[{"a": 1}'` 一看就是截断，
补了括号的 `'[{"a": 1},]'` 会被 `json.loads` 当成「差一点」而被反复重试。

`close()` 维持原语义（显式 close = 成功路径，照旧补括号）。
"""

import json

import pytest

from augmentor.exceptions import StreamError
from augmentor.streaming import StreamWriter
from augmentor.exceptions import DataValidationError


class Boom:
    """一被 json.dumps 就炸的对象"""

    def __repr__(self):
        raise ValueError("no")


class TestCrashMidWriteDoesNotLookSalvageable:
    """写崩的产物必须是明确的截断，而不是差一点就合法的非法 JSON"""

    def test_no_trailing_comma_is_left_behind(self, tmp_path):
        out = tmp_path / "crash.json"
        with pytest.raises(TypeError):
            with StreamWriter(str(out), format="json", mode="w") as writer:
                writer.write_chunk([{"a": 1}])
                writer.write_chunk([{"b": Boom()}])
        raw = out.read_text(encoding="utf-8")
        assert raw == '[{"a": 1}', (
            f"产物应是明确的截断形态，实为 {raw!r}——悬挂逗号会让它看起来"
            "「差一点就合法」，从而被反复重试"
        )

    def test_closing_bracket_is_not_written_on_exception(self, tmp_path):
        out = tmp_path / "crash.json"
        with pytest.raises(TypeError):
            with StreamWriter(str(out), format="json", mode="w") as writer:
                writer.write_chunk([{"a": 1}])
                writer.write_chunk([{"b": Boom()}])
        assert not out.read_text(encoding="utf-8").endswith("]")

    def test_exception_still_propagates(self, tmp_path):
        """修法不许把异常咽了——那会把「写崩」变成「写完了」"""
        out = tmp_path / "crash.json"
        with pytest.raises(TypeError):
            with StreamWriter(str(out), format="json", mode="w") as writer:
                writer.write_chunk([{"a": 1}])
                raise TypeError("boom")

    def test_already_written_items_survive_the_crash(self, tmp_path):
        """崩之前落盘的记录不许被抹掉（截断 ≠ 清空）"""
        out = tmp_path / "crash.json"
        with pytest.raises(TypeError):
            with StreamWriter(str(out), format="json", mode="w") as writer:
                writer.write_chunk([{"a": 1}, {"c": 3}])
                writer.write_chunk([{"b": Boom()}])
        assert '{"a": 1}' in out.read_text(encoding="utf-8")
        assert '{"c": 3}' in out.read_text(encoding="utf-8")


class TestHappyPathIsUnchanged:
    """防修过头：正常路径一个字节都不许变"""

    def test_json_roundtrip(self, tmp_path):
        out = tmp_path / "ok.json"
        with StreamWriter(str(out), format="json", mode="w") as writer:
            writer.write_chunk([{"a": 1}])
            writer.write_chunk([{"b": 2}])
        assert json.loads(out.read_text(encoding="utf-8")) == [{"a": 1}, {"b": 2}]

    def test_jsonl_roundtrip(self, tmp_path):
        out = tmp_path / "ok.jsonl"
        with StreamWriter(str(out), format="jsonl", mode="w") as writer:
            writer.write_chunk([{"a": 1}, {"b": 2}])
        lines = out.read_text(encoding="utf-8").splitlines()
        assert [json.loads(x) for x in lines] == [{"a": 1}, {"b": 2}]

    def test_append_json_is_refused_rather_than_writing_garbage(self, tmp_path):
        """`mode='a' + format='json'` 必须**拒绝**，不许静默产出非法 JSON

        L201 时这条还是「钉住已知限制」——追加会写出 `'[{"a": 1}]{"b": 2}'`（非法）。
        L208 拍定口径：追加一份**已闭合**的 JSON 数组，任何写法都产不出合法 JSON，
        而候选②（读回原数组、追加后整体重写）是 O(n) 读 + O(n) 写、违背「流式写入器」
        的初衷 ⇒ 索性拒绝。多批数据进同一文件的正确做法是一次性写或用 jsonl。
        谁把这里改成「能跑了」，先回来改这条。
        """
        out = tmp_path / "app.json"
        with StreamWriter(str(out), format="json", mode="w") as writer:
            writer.write_chunk([{"a": 1}])
        with pytest.raises(DataValidationError) as exc:
            StreamWriter(str(out), format="json", mode="a")
        assert "jsonl" in str(exc.value), (
            "报错文案要指出替代方案（jsonl 可追加），否则用户只知道不让干"
        )
        # 盘上那份不许被这次失败的构造破坏
        assert json.loads(out.read_text(encoding="utf-8")) == [{"a": 1}]

    def test_explicit_close_still_writes_the_bracket(self, tmp_path):
        """显式 close = 成功路径，语义不动"""
        out = tmp_path / "closed.json"
        writer = StreamWriter(str(out), format="json", mode="w")
        writer.__enter__()
        writer.write_chunk([{"a": 1}])
        writer.close()
        assert out.read_text(encoding="utf-8").endswith("]")

    def test_unopened_write_still_raises(self, tmp_path):
        writer = StreamWriter(str(tmp_path / "x.jsonl"))
        with pytest.raises(StreamError):
            writer.write_chunk([{"a": 1}])
