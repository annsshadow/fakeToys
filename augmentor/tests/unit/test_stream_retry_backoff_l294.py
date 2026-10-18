"""L294 守卫：`streaming` 的增量解析在「单个元素跨多块」时不得平方级重扫。

**为什么立案**：`_iter_json_array_items` 与 `_is_single_json_value` 的重试循环原本是「读满一块
就再试一次 `raw_decode`」。一次失败的 `raw_decode` 会把整段缓冲从头扫到尾（C 扫描器不能续扫），
所以一只跨块的巨型元素要付 `s + 2s + 3s + … ≈ n²/(2s)` 的重扫量。两个消费面读的都是本地文件
（`_count_items` / `read_chunks` 各自 `open(self.file_path)`），没有「早点拿到部分缓冲」的延迟收益，
纯粹是白扫 —— 一份 100MB 的单元素数组文件按 64KB 一块要白扫约 7.6e10 个字符。

**判据是计数，不是 wall**（本轮口径）。尺子：在内存里把 `streaming.json` 换成计数版 `JSONDecoder`，
每次 `raw_decode(s, idx)` 记 `len(s) - idx`，失败尝试时它就是 C 扫描器真正走过的字符数。
现量（`read_size=1024`，单元素数组）：

| N | 修前 scanned | 修前 /len | 修后 scanned | 修后 /len | 修前 attempts | 修后 attempts |
|---|---|---|---|---|---|---|
| 2^14 | 155642 | 9.49 | 48133 | 2.94 | 17 | 6 |
| 2^15 | 573418 | 17.49 | 97284 | 2.97 | 33 | 7 |
| 2^16 | 2195402 | 33.49 | 195587 | 2.98 | 65 | 8 |
| 2^17 | 8585098 | 65.49 | 392194 | 2.99 | 129 | 9 |
| 2^18 | 33947402 | 129.49 | 785409 | 3.00 | 257 | 10 |

修前 N 每翻倍比率 → ×4（平方级），修后 → ×2 且比值钉在 3.00；尝试次数每次翻倍 +1（对数）。

**尺子自己返工过两次，留着不美化**：
1. 第一版探针拿「累计读到的字节和」当工作量，被等长小元素那一档推翻 —— 小元素每产出一条就
   `compact()`，读过 ≠ 重扫，那把尺子给两种输入都读出平方形状。
2. 攒块条件第一版写成 `len(buf) - pos < gap`：缓冲里未扫部分已经 ≥ gap 时循环一次 `fill()` 都不做，
   于是对同一份缓冲原地重试。`TestRulerHasTeeth` 把节奏缝改成恒等（旧 cadence）后那一臂**永不
   推进**（把整支守卫挂在 120 秒超时上），普通档也只多出 4 次无意义重扫 ⇒ 判据改成
   `target = len(buf) + gap`（必须真的新增 `gap` 个字符才重试），恒等档随即退化成
   「每 `read_size` 重试一次」= 与修前逐格相同的形状。
"""

import io
import json
import math
from typing import Dict, List

import pytest

from augmentor import streaming

READ_SIZE = 1024
SIZES = [1 << k for k in (14, 15, 16, 17, 18)]
BIG = 1 << 18

#: 修后逐格现量：`sum(len(buf) - pos)` 与尝试次数
MEASURED_SCANNED_ITER: Dict[int, int] = {
    16384: 48133,
    32768: 97284,
    65536: 195587,
    131072: 392194,
    262144: 785409,
}
MEASURED_ATTEMPTS_ITER: Dict[int, int] = {16384: 6, 32768: 7, 65536: 8, 131072: 9, 262144: 10}
MEASURED_SCANNED_SINGLE: Dict[int, int] = {
    16384: 48139,
    32768: 97291,
    65536: 195595,
    131072: 392203,
    262144: 785419,
}
MEASURED_ATTEMPTS_SINGLE: Dict[int, int] = {16384: 6, 32768: 7, 65536: 8, 131072: 9, 262144: 10}

#: 修前（节奏缝改成恒等）同一靶子的读数，由 `TestRulerHasTeeth` 现量核对
LEGACY_SCANNED_ITER = 33947402
LEGACY_SCANNED_SINGLE = 33947659
LEGACY_ATTEMPTS = 257

#: 承重不等式：重扫总量至多 4 倍文本长度。修前最紧的一档是 129.49 倍 ⇒ 这条有牙齿。
SCANNED_MULTIPLE_LIMIT = 4
#: 尝试次数随 N 翻倍的增长上限（修前每翻倍 +N/2，修后 +1）
ATTEMPT_STEP_LIMIT = 2

SMALL_UNIT_CHARS = 4096
SMALL_ITEMS = 64
MEASURED_SMALL_ATTEMPTS = 192
MEASURED_SMALL_SCANNED = 725697


class _Shim:
    """替 `streaming.json`：只把 `JSONDecoder` 换成计数版，其余原样转发"""

    def __init__(self, sink: List[int]) -> None:
        self.sink = sink

    def __getattr__(self, name):
        if name == "JSONDecoder":
            sink = self.sink

            class _Counting(json.JSONDecoder):
                def raw_decode(self, s, idx=0):
                    sink.append(len(s) - idx)
                    return super().raw_decode(s, idx)

            return _Counting
        return getattr(json, name)


@pytest.fixture
def sink(monkeypatch):
    """在内存里给 `streaming` 装计数解码器，零落盘、用例结束自动复原"""
    recorded: List[int] = []
    monkeypatch.setattr(streaming, "json", _Shim(recorded))
    return recorded


def single_element_text(n: int) -> str:
    """一只长度约 n 的合法 JSON 对象，独自构成整个数组 ⇒ 必然跨块"""
    return "[" + json.dumps({"d": "x" * n}) + "]"


class TestTheRescanIsLinear:
    def test_array_parser_scanned_matches_measured(self, sink):
        for n in SIZES:
            del sink[:]
            text = single_element_text(n)
            items = list(streaming._iter_json_array_items(io.StringIO(text), read_size=READ_SIZE))
            assert items == json.loads(text), "改法不许动判决：产出必须与整份 `json.loads` 逐字节相同"
            assert sum(sink) == MEASURED_SCANNED_ITER[n], (n, len(sink), sum(sink))
            assert len(sink) == MEASURED_ATTEMPTS_ITER[n], (n, len(sink))

    def test_single_value_checker_scanned_matches_measured(self, sink):
        for n in SIZES:
            del sink[:]
            text = single_element_text(n)
            assert streaming._is_single_json_value(io.StringIO(text), read_size=READ_SIZE) is True
            assert sum(sink) == MEASURED_SCANNED_SINGLE[n], (n, len(sink), sum(sink))
            assert len(sink) == MEASURED_ATTEMPTS_SINGLE[n], (n, len(sink))

    def test_scanned_stays_a_small_multiple_of_input(self, sink):
        """承重那一格：`scanned / len(text)` 必须是常数级，不随 N 增长"""
        for n in SIZES:
            del sink[:]
            text = single_element_text(n)
            list(streaming._iter_json_array_items(io.StringIO(text), read_size=READ_SIZE))
            assert sum(sink) <= SCANNED_MULTIPLE_LIMIT * len(text), (n, sum(sink) / len(text))

    def test_attempts_grow_logarithmically(self, sink):
        steps = []
        for n in SIZES:
            del sink[:]
            text = single_element_text(n)
            list(streaming._iter_json_array_items(io.StringIO(text), read_size=READ_SIZE))
            steps.append(len(sink))
        assert all(b - a <= ATTEMPT_STEP_LIMIT for a, b in zip(steps, steps[1:])), steps
        assert steps[-1] - steps[0] <= int(math.log2(SIZES[-1] / SIZES[0]))


class TestSemanticsSurviveTheBackoff:
    """攒够再试不许改语义：这几档是既有边界形状，逐条与 `json.loads` 或既有判决对齐"""

    def test_many_small_elements_stream_and_match_whole_file_parse(self, sink):
        unit = json.dumps({"d": "x" * SMALL_UNIT_CHARS})
        text = "[" + ",".join([unit] * SMALL_ITEMS) + "]"
        del sink[:]
        got = list(streaming._iter_json_array_items(io.StringIO(text), read_size=READ_SIZE))
        assert got == json.loads(text)
        assert len(sink) == MEASURED_SMALL_ATTEMPTS, len(sink)
        assert sum(sink) == MEASURED_SMALL_SCANNED, sum(sink)

    def test_element_spanning_many_chunks_yields_the_same_object(self, sink):
        payload = {"a": "y" * 5000, "b": [1, 2, {"c": "z" * 3000}]}
        text = "[" + json.dumps(payload) + ', 1, "tail"]'
        del sink[:]
        got = list(streaming._iter_json_array_items(io.StringIO(text), read_size=64))
        assert got == json.loads(text)
        assert len(sink) == 10, len(sink)

    def test_truncated_element_still_raises_at_eof(self, sink):
        with pytest.raises(json.JSONDecodeError):
            list(streaming._iter_json_array_items(io.StringIO('[{"a": '), read_size=READ_SIZE))

    def test_missing_comma_still_raises(self, sink):
        with pytest.raises(streaming.DataFormatError, match="缺少逗号"):
            list(streaming._iter_json_array_items(io.StringIO('[{"a": 1} {"b": 2}]'), read_size=READ_SIZE))

    def test_unterminated_array_returns_what_it_read(self, sink):
        text = '[{"a": 1}, {"b": 2}'
        assert list(streaming._iter_json_array_items(io.StringIO(text), read_size=READ_SIZE)) == [
            {"a": 1},
            {"b": 2},
        ]

    def test_jsonl_short_circuit_is_untouched(self, sink):
        line = json.dumps({"d": "x" * 200})
        del sink[:]
        assert streaming._is_single_json_value(io.StringIO("\n".join([line] * 32)), read_size=READ_SIZE) is False
        assert len(sink) == 1, len(sink)


class TestRulerHasTeeth:
    """注入臂：把节奏缝改成恒等 ⇒ 旧 cadence（每 `read_size` 重试一次）当场把平方读数顶红"""

    def _with_identity_gap(self, monkeypatch):
        monkeypatch.setattr(streaming, "_next_retry_gap", lambda gap: gap)

    def test_arm_identity_gap_breaks_the_multiple_bound(self, sink, monkeypatch):
        self._with_identity_gap(monkeypatch)
        text = single_element_text(BIG)
        del sink[:]
        assert list(streaming._iter_json_array_items(io.StringIO(text), read_size=READ_SIZE)) == json.loads(text)
        assert sum(sink) > SCANNED_MULTIPLE_LIMIT * len(text), sum(sink)
        assert sum(sink) == LEGACY_SCANNED_ITER, sum(sink)
        assert len(sink) == LEGACY_ATTEMPTS, len(sink)

    def test_arm_identity_gap_breaks_the_attempt_step_bound(self, sink, monkeypatch):
        self._with_identity_gap(monkeypatch)
        steps = []
        for n in (1 << 14, 1 << 15, 1 << 16):
            del sink[:]
            text = single_element_text(n)
            list(streaming._iter_json_array_items(io.StringIO(text), read_size=READ_SIZE))
            steps.append(len(sink))
        assert max(b - a for a, b in zip(steps, steps[1:])) > ATTEMPT_STEP_LIMIT, steps

    def test_arm_identity_gap_moves_the_single_value_checker_too(self, sink, monkeypatch):
        self._with_identity_gap(monkeypatch)
        text = single_element_text(BIG)
        del sink[:]
        assert streaming._is_single_json_value(io.StringIO(text), read_size=READ_SIZE) is True
        assert len(sink) == LEGACY_ATTEMPTS, len(sink)
        assert sum(sink) == LEGACY_SCANNED_SINGLE, sum(sink)

    def test_the_real_gap_doubles(self):
        assert streaming._next_retry_gap(1024) == 2048
