# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集格式转换模块

提供多种数据集格式之间的转换功能。
"""

import json
import csv
import io
import logging
import math
from typing import List, Dict, Optional, Any, Tuple
from pathlib import Path
from enum import Enum
from .exceptions import DataFormatError, DataLoadError, UnsupportedFormatError

logger = logging.getLogger(__name__)


def csv_fieldnames(items: List[Dict]) -> List[str]:
    """CSV/TSV 的表头：全量键的**并集**，按首次出现顺序排列

    不能只取 `items[0].keys()`。增强与合并的产物天然异构（某一路数据多了
    `category` 字段），用首条的键建表头会让 `DictWriter` 在后续行上抛
    `ValueError: dict contains fields not in fieldnames`，整次导出失败；
    取并集后缺字段的行由 `restval` 落成空单元格。
    """
    keys: List[str] = []
    seen = set()
    for item in items:
        for key in item:
            if key not in seen:
                seen.add(key)
                keys.append(key)
    return keys


#: 「容器」格式：只有这四个决定文件本身的物理布局（`_read_file` / `_write_file`
#: 选哪个解析器）。其余 `DataFormat` 成员（alpaca/sharegpt/chatml/llama_factory/
#: vicuna/belle）只是**行内 schema**，同一份 schema 既可以落成一个 JSON 数组，
#: 也可以落成一行一条的 JSONL。
CONTAINER_FORMATS = ("json", "jsonl", "csv", "tsv")

#: Excel 读边的内部格式名，以及调用方可以写的三种拼法（`--input-format` /
#: `source_format`）。它**不是** `DataFormat` 成员，也不进 `_converters` 转换图：
#: Excel 读出来就是一张「列 → 值」的行记录表，与 `csv.DictReader` 交出的形状相同，
#: 所以读完之后按 `csv` 那一族边走（见 `convert_file` 里的 `graph_source`）。
EXCEL_FORMAT = "excel"
EXCEL_SOURCE_NAMES = ("excel", "xlsx", "xls")

#: Excel **写边**认的两种拼法。`xls` 刻意不在内：写真正的 `.xls`（BIFF 老格式）要 xlwt，
#: 本仓库依赖表里没有它，装上 pandas + openpyxl 也只会得到一份「扩展名是 xls、字节是
#: zip」的假容器 —— 那正是 A133 要终结的症状，所以宁可不收。
EXCEL_TARGET_NAMES = ("excel", "xlsx")

#: 写侧必须**点名拒收**的输出扩展名（读侧认它，见 `INPUT_EXTENSION_FORMATS`）。
EXCEL_REJECTED_OUTPUT_SUFFIXES = (".xls",)

#: **写边**的「输出扩展名 → 落成哪种布局」权威表。它与 `INPUT_EXTENSION_FORMATS` 是
#: 两张表而不是同一张的两个视图：读边认 `.xls`（openpyxl 读得了老格式），写边不认。
#: 合并成一张表就等于把 `.xls` 输出重新放回去（继续写假容器），分开才能各自达标。
OUTPUT_EXTENSION_FORMATS = {
    ".json": "json",
    ".jsonl": "jsonl",
    ".csv": "csv",
    ".tsv": "tsv",
    ".xlsx": EXCEL_FORMAT,
}


#: **读边**的「扩展名 → 源格式」权威表。与 `_infer_format` 的区别是方向而不是大小写：
#: 那张表管「输出扩展名落成哪个容器」，认不出回落 json 无害（写的仍是 JSON 字节）；
#: 这张表管「这份文件该交给哪个解析器」，认不出**必须报错**——回落 json 会让一份
#: 二进制 xlsx 崩在 `codecs` 里（实测 `UnicodeDecodeError: 'utf-8' codec can't
#: decode byte 0xc7 in position 15`），CLI 只能把那句解码错转给用户，与真实原因
#: （没认出格式）毫无关系。
INPUT_EXTENSION_FORMATS = {
    ".json": "json",
    ".jsonl": "jsonl",
    ".csv": "csv",
    ".tsv": "tsv",
    ".xlsx": EXCEL_FORMAT,
    ".xls": EXCEL_FORMAT,
}

#: `--input-format` / `source_format` 的全部合法拼法。CLI 的 `choices` 直接引用这份
#: 常量（A77「一份权威只住一处」，照 `parser.EXPORT_FORMATS` 的先例），报错文案也
#: 从它生成 ⇒ 加一个源格式只需要改这一行。
INPUT_FORMAT_CHOICES = [
    "json", "jsonl", "csv", "tsv", "alpaca", "sharegpt", "chatml",
    "llama_factory", "vicuna", "belle",
] + list(EXCEL_SOURCE_NAMES)

#: `--format` / `target_format` 的全部合法拼法 = 转换图里的目标 + Excel 写边那两种。
#: 从 `INPUT_FORMAT_CHOICES` 减掉只属读边的 Excel 拼法再换上去 ⇒ 十个 schema 名仍只有
#: 一份字面（A77），加一个 schema 格式依旧只改 `INPUT_FORMAT_CHOICES` 那一行。
OUTPUT_FORMAT_CHOICES = [
    name for name in INPUT_FORMAT_CHOICES if name not in EXCEL_SOURCE_NAMES
] + list(EXCEL_TARGET_NAMES)


#: 「原样序列化」的目标格式：把输入当作任意 JSON 值落盘，因此**不要求**记录是对象。
#: 其余八个目标都必须逐条读字段——六个行内 schema 取 `instruction`/`output` 之类的键，
#: `csv`/`tsv` 用 `DictWriter` 按列名写盘——记录是标量时以前会崩在边里
#: （`AttributeError: 'NoneType' object has no attribute 'get'`，经 API 就是 500，
#: 且 csv/tsv 会留下半截文件）。只作用于「源是通用 json」的写边，见
#: `_require_object_records`。
PASSTHROUGH_TARGETS = ("json", "jsonl")


def excel_cell(value: Any) -> Any:
    """Excel 单元格 → JSON 可安全落盘的值

    `DataFrame.to_dict()` 交出的空格子是 `NaN`/`NaT`，日期是 `Timestamp`：三者都不能
    直接进 `json.dump`（前二者会写出**非法 JSON** 的 `NaN` 字面量，后者当场
    `TypeError: Object of type Timestamp is not JSON serializable`）。口径与 csv 读边
    对齐 —— **空单元格就是空串**（`DictReader` 对空列给的也是 `""`），其余非标量
    （日期、时间之类）取 `str()` 而不是丢列。

    先判 Python 原生标量再判「自不等」：`NaN != NaN` 与 `NaT != NaT` 都是 True，所以
    「不等于自身」是缺失标记的共同形状。pandas 的 `NA` 不走这条 —— `pd.NA != pd.NA`
    返回的是 `NA` 本身，对它做真值判断当场 `TypeError: boolean value of NA is
    ambiguous`（实测），因此那句比较要包在 `try` 里，把 TypeError 也当缺失。
    """
    if value is None:
        return ""
    if isinstance(value, float) and math.isnan(value):
        return ""
    if isinstance(value, (bool, int, float, str)):
        return value
    try:
        if value != value:  # NaN / NaT 及任何「不等于自身」的缺失标记
            return ""
    except TypeError:  # pd.NA：比较结果不可真值化，同样是缺失
        return ""
    return str(value)


class DataFormat(Enum):
    """数据格式"""
    JSON = "json"
    JSONL = "jsonl"
    CSV = "csv"
    TSV = "tsv"
    ALPACA = "alpaca"
    SHAREGPT = "sharegpt"
    CHATML = "chatml"
    LLAMA_FACTORY = "llama_factory"
    VICUNA = "vicuna"
    BELLE = "belle"


# 对话容器字段名与其中「角色」的写法差异
_CONVERSATION_KEY = {"sharegpt": "conversations", "vicuna": "conversations",
                     "chatml": "messages"}
_TURN_VALUE_KEY = {"sharegpt": "value", "vicuna": "value", "chatml": "content"}
_TURN_ROLE_KEY = {"sharegpt": "from", "vicuna": "from", "chatml": "role"}

# 各家的同义角色；未列出的角色一律报错而不是猜
_ROLE_ALIASES = {"human": "user", "user": "user",
                 "gpt": "assistant", "assistant": "assistant",
                 "system": "system"}


def turns_to_canonical(turns: List[Dict[str, str]], format_name: str) -> Dict[str, str]:
    """把一轮对话折成规范形的 `instruction` / `output` / `history` / `system`

    写侧（`_json_to_sharegpt` 等）总是把 `history` 摊在前面、`instruction`/`output`
    放在最后，所以逆运算取**最后两问一答**为当前轮，其余进 `history`；开头的
    system 轮还原成 `system` 字段。

    Raises:
        DataFormatError: 对话不足两轮，或末尾不是「用户 → 助手」
    """
    if len(turns) < 2:
        raise DataFormatError(f"{format_name} 对话少于两轮，无法还原问答")
    if turns[-2].get("role") != "user" or turns[-1].get("role") != "assistant":
        raise DataFormatError(
            f"{format_name} 对话必须以「user → assistant」结尾，"
            f"实际是「{turns[-2].get('role')} → {turns[-1].get('role')}」"
        )

    head = turns[:-2]
    item: Dict[str, Any] = {}
    if head and head[0]["role"] == "system":
        item["system"] = head[0]["content"]
        head = head[1:]
    if head:
        item["history"] = head
    item["instruction"] = turns[-2]["content"]
    item["output"] = turns[-1]["content"]
    return item


def read_conversation_turns(record: Any, format_name: str, index: int) -> List[Dict[str, str]]:
    """读出一条对话记录的轮次，角色按 `_ROLE_ALIASES` 归一

    Raises:
        DataFormatError: 记录不是对象、缺对话数组、轮次缺字段或角色不认识
    """
    if not isinstance(record, dict):
        raise DataFormatError(
            f"第 {index + 1} 条 {format_name} 记录必须是 JSON 对象，"
            f"当前是{type(record).__name__}"
        )
    raw_turns = record.get(_CONVERSATION_KEY[format_name])
    if not isinstance(raw_turns, list):
        raise DataFormatError(
            f"第 {index + 1} 条 {format_name} 记录缺少 "
            f"`{_CONVERSATION_KEY[format_name]}` 数组"
        )

    role_key = _TURN_ROLE_KEY[format_name]
    value_key = _TURN_VALUE_KEY[format_name]
    turns: List[Dict[str, str]] = []
    for turn in raw_turns:
        if not isinstance(turn, dict) or role_key not in turn or value_key not in turn:
            raise DataFormatError(
                f"第 {index + 1} 条 {format_name} 记录的某一轮缺少 `{role_key}` / `{value_key}`"
            )
        role = _ROLE_ALIASES.get(str(turn[role_key]).lower())
        if role is None:
            raise DataFormatError(
                f"第 {index + 1} 条 {format_name} 记录含未认识的角色: {turn[role_key]}"
            )
        turns.append({"role": role, "content": turn[value_key]})
    return turns


class DatasetConverter:
    """数据集格式转换器
    
    支持多种格式之间的转换。
    """
    
    def __init__(self):
        """初始化转换器"""
        self._converters = {
            ("json", "jsonl"): self._json_to_jsonl,
            ("jsonl", "json"): self._jsonl_to_json,
            ("json", "csv"): self._json_to_csv,
            ("csv", "json"): self._csv_to_json,
            ("json", "tsv"): self._json_to_tsv,
            ("tsv", "json"): self._tsv_to_json,
            ("json", "alpaca"): self._json_to_alpaca,
            ("json", "sharegpt"): self._json_to_sharegpt,
            ("json", "chatml"): self._json_to_chatml,
            ("json", "llama_factory"): self._json_to_llama_factory,
            ("json", "vicuna"): self._json_to_vicuna,
            ("json", "belle"): self._json_to_belle,
            ("alpaca", "json"): self._alpaca_to_json,
            ("belle", "json"): self._belle_to_json,
            ("llama_factory", "json"): self._llama_factory_to_json,
            ("sharegpt", "json"): self._sharegpt_to_json,
            ("vicuna", "json"): self._vicuna_to_json,
            ("chatml", "json"): self._chatml_to_json,
        }
    
    def convert(self, 
               data: List[Dict], 
               source_format: str, 
               target_format: str,
               **kwargs) -> Any:
        """转换数据格式
        
        Args:
            data: 数据列表
            source_format: 源格式
            target_format: 目标格式
            **kwargs: 额外参数
        
        Returns:
            转换后的数据
        """
        # 标准化格式名称
        source_format = self._normalize_format(source_format)
        target_format = self._normalize_format(target_format)
        
        # 如果格式相同，直接返回
        if source_format == target_format:
            return data
        
        # 写边形状复核：只有「源是通用 json」这一路需要，六条 schema 读边自己已经
        # 带格式名报「第 N 条 X 记录必须是 JSON 对象」（比这里的措辞更准），
        # 不必在分发口抢着替它们报错
        if source_format == "json" and target_format not in PASSTHROUGH_TARGETS:
            self._require_object_records(data, source_format, target_format)
        
        # 查找转换器
        key = (source_format, target_format)
        if key in self._converters:
            if source_format == "json" and target_format not in CONTAINER_FORMATS:
                self._reject_undeclared_conversations(data, target_format)
                self._reject_all_empty_qa(data, target_format)
            return self._converters[key](data, **kwargs)
        
        # 尝试通过JSON中间格式转换
        if source_format != "json":
            json_data = self._to_json(data, source_format)
            return self.convert(json_data, "json", target_format, **kwargs)
        
        raise UnsupportedFormatError(f"不支持的转换: {source_format} -> {target_format}")
    
    def _normalize_format(self, format_name: str) -> str:
        """标准化格式名称"""
        format_map = {
            "json": "json",
            "jsonl": "jsonl",
            "csv": "csv",
            "tsv": "tsv",
            "alpaca": "alpaca",
            "sharegpt": "sharegpt",
            "chatml": "chatml",
            "llama_factory": "llama_factory",
            "vicuna": "vicuna",
            "belle": "belle",
        }
        return format_map.get(format_name.lower(), format_name.lower())
    
    def _to_json(self, data: Any, source_format: str) -> List[Dict]:
        """转换为JSON格式"""
        to_json = self._converters.get((source_format, "json"))
        if to_json is not None:
            return to_json(data)
        raise DataFormatError(f"无法从 {source_format} 转换到 JSON")
    
    # ==================== 基础格式转换 ====================
    
    def _json_to_jsonl(self, data: List[Dict], **kwargs) -> List[str]:
        """JSON 转 JSONL"""
        return [json.dumps(item, ensure_ascii=False) for item in data]
    
    def _jsonl_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """JSONL 转 JSON"""
        if isinstance(data, list):
            return [json.loads(line) if isinstance(line, str) else line for line in data]
        return [json.loads(line) for line in data]
    
    def _json_to_csv(self, data: List[Dict], **kwargs) -> List[Dict]:
        """JSON 转 CSV（返回字典列表）

        这里**只**做「行集合的原样传递」：表头取全量键并集、缺列补空，是
        `_write_file` 用 `csv_fieldnames()` 干的事。曾经这里另有一份收集 `all_keys`
        的循环，算完既不进返回值也不进表头（`return [item for item in data]`），
        是一份看着像在做键归一、其实什么都没做的死代码。
        """
        return [item for item in data]

    def _csv_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """CSV 转 JSON"""
        if isinstance(data, list):
            return data
        return []

    def _json_to_tsv(self, data: List[Dict], **kwargs) -> List[Dict]:
        """JSON 转 TSV（与 `_json_to_csv` 同理，分隔符由读写两侧决定）"""
        return [item for item in data]

    def _tsv_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """TSV 转 JSON"""
        if isinstance(data, list):
            return data
        return []

    # ==================== 训练格式转换 ====================

    def _require_object_records(self, data: Any, source_format: str,
                                target_format: str) -> None:
        """`json → 八类要取字段的目标`，先确认每条记录真是对象，而不是崩在边里

        六条 schema 写边用 `item.get("instruction")` 取问答，`csv`/`tsv` 写盘用
        `DictWriter` 按列名取键——都要求记录是对象。以前一条 `null` 就能把整条链路
        砸成 `AttributeError: 'NoneType' object has no attribute 'get'`（经 API 是
        500、文案是解释器内部措辞），写 csv/tsv 时还会**先落一个半截文件**再崩
        （`DictWriter` 把字符串按字符展开成表头，实测产物是 `j,u,s,t, ,a,...`）。

        `json`/`jsonl` 目标不查：它们把输入原样序列化，任何合法 JSON 值都能落盘并
        原样读回。源不是 `json` 时也不查：`csv`/`tsv` 的读边交给 `DictReader`
        （交出来必是对象），`jsonl` 在内存里本来就是字符串列表，六个 schema 读边
        自己带格式名报错——那一侧的护栏早就在了。

        Raises:
            DataFormatError: 数据集不是列表，或第一条非对象记录（带条目下标）
        """
        hop = f"{source_format} → {target_format}"
        if not isinstance(data, list):
            raise DataFormatError(
                f"数据集必须是记录列表，当前是{type(data).__name__}："
                f"{hop} 需要逐条读取字段"
            )
        for index, record in enumerate(data):
            if not isinstance(record, dict):
                raise DataFormatError(
                    f"第 {index + 1} 条记录必须是 JSON 对象，当前是"
                    f"{type(record).__name__}：{hop} 需要每条记录带字段，"
                    f"标量与数组都没有可读的键"
                )

    #: 对话容器键 → 它可能对应的源格式，用于报错时给出可执行的建议
    _CONTAINER_HINT = {"conversations": "sharegpt / vicuna",
                       "messages": "chatml"}

    #: 写边能拿去构成「一问 / 一答」的键。`system` 不在其中：只有系统提示词的记录
    #: 转成任何训练格式都是空问答。判据比 `_reject_undeclared_conversations` 宽一点
    #: 是有意的——那条按记录找「误标的对话数组」，宁窄；这一条下的是整档结论。
    _QA_KEYS = ("instruction", "output")

    #: `history` 是**逐边**才作数的键：六条写边里只有 sharegpt 和 chatml 会把历史轮次
    #: 展开进产物，其余四条（alpaca / llama_factory / vicuna / belle）读都不读它。
    #: 所以「整档只有 history」对 chatml 是可用数据、对 alpaca 仍是一份空问答数据集，
    #: 按目标分键才不把后者放过去。
    _HISTORY_AWARE_TARGETS = ("sharegpt", "chatml")

    def _qa_keys(self, target_format: str) -> Tuple[str, ...]:
        """给定目标格式，返回它的写边真正会读走的问答键"""
        if target_format in self._HISTORY_AWARE_TARGETS:
            return self._QA_KEYS + ("history",)
        return self._QA_KEYS

    def _reject_all_empty_qa(self, data: List[Dict], target_format: str) -> None:
        """整档一条问答都取不到时当场失败，而不是交出一份空问答数据集

        六个 schema 写边只认 `_qa_keys` 给的那几把钥匙。真实语料四份（6902 / 6902 /
        1124 / 5778 条，共 20706 条）实测「四个键全空」的记录是 **0 条**，所以
        **整档**都取不到问答几乎只有一种成因：这批数据用的是别的字段名
        （`question`/`answer`、`text`、`prompt`/`completion`……），而不是它真的
        应该被转成一份空数据集。以前这一路是退出码 0、HTTP 200、产物结构合法但
        一行问答都没有（A26②）。

        只看整档不看单条：个别记录缺问答是数据本身的常态（增强、清洗都会引入），
        逐条报错会把可用数据集砸掉。空数据集不在这里管——`[]` 转任何格式都是
        合法的 0 条产物（既有行为，见 `test_empty_dataset_still_yields_empty_list`）。

        只按真值判断，不做 strip：`" "` 是 truthy，因此只含空格的字段算「有内容」。
        与 `_reject_undeclared_conversations` 同一口径，也是为了不替调用方裁决
        「空白到底算不算标签」——那是清洗该管的事，不是转换护栏。

        找到第一条可用问答立刻返回：干净数据上这是 O(1)，与扫描整档不同量级。

        Raises:
            DataFormatError: 非空数据集里没有任何一条带 `_qa_keys` 之一的非空值
        """
        keys = self._qa_keys(target_format)
        for record in data:
            if any(record.get(key) for key in keys):
                return
        if data:
            raise DataFormatError(
                f"{len(data)} 条记录里取不到任何问答：`json → {target_format}` 只认 "
                f"`{'` / `'.join(keys)}` 这几个键（任一非空）。产物会是 "
                f"{len(data)} 条空问答数据集，训练不出东西。若源数据另有字段名"
                f"（如 `question`/`answer`），请先改成这几个键"
            )

    def _reject_undeclared_conversations(self, data: List[Dict], target_format: str) -> None:
        """拦住「源格式没声明、其实是对话类」的误标，而不是产出全空问答

        `json → alpaca/sharegpt/chatml/…` 这六条写边只认 `instruction` / `output`
        / `history` / `system` 四个键。一份实际是 sharegpt 的文件如果没声明源格式，
        会被按通用 json 读成「每条都没有问答」，产物是**结构完全合法的空白数据集**，
        退出码 0、HTTP 200 —— 比崩溃更坏，因为下游会直接拿去训练。

        判据刻意取窄：记录里既有**可用的** `instruction`/`output`（任一非空）就不管，
        只有「取不到问答 + 躺着对话数组」才报。所以 csv/tsv 里恰好有一列叫
        `messages` 的字符串字段不会误伤（`DictReader` 交出来的是 str，不是 list）。
        真·认不出字段的记录（既无问答也无对话数组）不在这一条里逐条报，由紧随其后的
        `_reject_all_empty_qa` 下整档结论（L23）。

        调用前提是 `data` 已经过 `_require_object_records`（两者同在 `convert()`
        分发口），所以这里不再复核记录形状。

        Raises:
            DataFormatError: 第一条命中的记录，带条目下标与建议的 `source_format`
        """
        for index, record in enumerate(data):
            if record.get("instruction") or record.get("output"):
                continue
            for key, hint in self._CONTAINER_HINT.items():
                if isinstance(record.get(key), list):
                    raise DataFormatError(
                        f"第 {index + 1} 条记录取不到 `instruction`/`output`，"
                        f"问答在 `{key}` 数组里：按通用 json 转 {target_format} 会得到"
                        f"全空数据集。若源数据是 {hint}，请声明 source_format"
                        f"（CLI 为 --input-format）"
                    )
    
    def _json_to_alpaca(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 Alpaca 格式"""
        result = []
        for item in data:
            record = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", "")
            }
            result.append(record)
        return result
    
    def _json_to_sharegpt(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 ShareGPT 格式"""
        result = []
        for item in data:
            conversations = []
            
            # 添加历史对话
            history = item.get("history", [])
            for turn in history:
                conversations.append({
                    "from": turn.get("role", "user"),
                    "value": turn.get("content", "")
                })
            
            # 添加当前问答
            conversations.append({
                "from": "human",
                "value": item.get("instruction", "")
            })
            conversations.append({
                "from": "gpt",
                "value": item.get("output", "")
            })
            
            result.append({"conversations": conversations})
        return result
    
    def _json_to_chatml(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 ChatML 格式"""
        result = []
        for item in data:
            messages = []
            
            # 添加系统消息
            system = item.get("system", "You are a helpful assistant.")
            if system:
                messages.append({
                    "role": "system",
                    "content": system
                })
            
            # 添加历史对话
            history = item.get("history", [])
            for turn in history:
                messages.append({
                    "role": turn.get("role", "user"),
                    "content": turn.get("content", "")
                })
            
            # 添加当前问答
            messages.append({
                "role": "user",
                "content": item.get("instruction", "")
            })
            messages.append({
                "role": "assistant",
                "content": item.get("output", "")
            })
            
            result.append({"messages": messages})
        return result
    
    def _json_to_llama_factory(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 Llama-Factory 格式"""
        result = []
        for item in data:
            record = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", ""),
                "system": item.get("system", "")
            }
            result.append(record)
        return result
    
    def _json_to_vicuna(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 Vicuna 格式"""
        result = []
        for item in data:
            record = {
                "id": item.get("id", ""),
                "conversations": [
                    {
                        "from": "human",
                        "value": item.get("instruction", "")
                    },
                    {
                        "from": "gpt",
                        "value": item.get("output", "")
                    }
                ]
            }
            result.append(record)
        return result
    
    def _json_to_belle(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 BELLE 格式"""
        result = []
        for item in data:
            record = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", "")
            }
            result.append(record)
        return result
    
    # ==================== 训练格式 → JSON（反向边） ====================

    @staticmethod
    def _flat_to_json(data: Any, format_name: str) -> List[Dict]:
        """字段映射类格式 → 规范形：认形状、保字段，不发明源文件里没有的键

        与写侧相反的一侧**不**给 `input` 补空串：`validation` 里 `input` 是可选字段，
        凭空造一个空字段只会让「源文件有没有这一列」这件事失去可辨性。
        """
        if not isinstance(data, list):
            raise DataFormatError(
                f"{format_name} 数据集的顶层必须是 JSON 数组，当前是{type(data).__name__}"
            )
        result = []
        for index, record in enumerate(data):
            if not isinstance(record, dict):
                raise DataFormatError(
                    f"第 {index + 1} 条 {format_name} 记录必须是 JSON 对象，"
                    f"当前是{type(record).__name__}"
                )
            missing = [key for key in ("instruction", "output") if key not in record]
            if missing:
                raise DataFormatError(
                    f"第 {index + 1} 条 {format_name} 记录缺少字段: {', '.join(missing)}"
                )
            result.append(dict(record))
        return result

    def _alpaca_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """Alpaca 转 JSON"""
        return self._flat_to_json(data, "alpaca")

    def _belle_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """BELLE 转 JSON"""
        return self._flat_to_json(data, "belle")

    def _llama_factory_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """Llama-Factory 转 JSON"""
        return self._flat_to_json(data, "llama_factory")

    def _conversations_to_json(self, data: Any, format_name: str) -> List[Dict]:
        """对话类格式 → 规范形：容器字段之外的键原样保留，容器本身折成问答"""
        if not isinstance(data, list):
            raise DataFormatError(
                f"{format_name} 数据集的顶层必须是 JSON 数组，当前是{type(data).__name__}"
            )
        container = _CONVERSATION_KEY[format_name]
        result = []
        for index, record in enumerate(data):
            turns = read_conversation_turns(record, format_name, index)
            try:
                canonical = turns_to_canonical(turns, format_name)
            except DataFormatError as e:
                raise DataFormatError(f"第 {index + 1} 条 {e}") from e
            item = {key: value for key, value in record.items() if key != container}
            item.update(canonical)
            result.append(item)
        return result

    def _sharegpt_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """ShareGPT 转 JSON"""
        return self._conversations_to_json(data, "sharegpt")

    def _vicuna_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """Vicuna 转 JSON（`id` 随容器外的键一起保留）"""
        return self._conversations_to_json(data, "vicuna")

    def _chatml_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """ChatML 转 JSON"""
        return self._conversations_to_json(data, "chatml")

    # ==================== 文件操作 ====================

    def convert_file(self,
                    input_path: str,
                    output_path: str,
                    source_format: Optional[str] = None,
                    target_format: Optional[str] = None,
                    **kwargs) -> Dict:
        """转换文件格式
        
        Args:
            input_path: 输入文件路径
            output_path: 输出文件路径
            source_format: 源格式（为 None 时从扩展名推断）
            target_format: 目标格式（为 None 时从扩展名推断）。Excel 拼法在这里
                归一成内部名 `excel`，返回值里的 `target_format` 给的是归一后的值
                （与读边归一 `source_format` 同一条口径）
        
        Returns:
            转换结果
        """
        input_path = Path(input_path)
        output_path = Path(output_path)
        
        # 推断格式
        if source_format is None:
            source_format = self._resolve_input_format(input_path)
        elif source_format.lower() in EXCEL_SOURCE_NAMES:
            source_format = EXCEL_FORMAT
        if target_format is None:
            target_format = self._infer_format(output_path)
        elif target_format.lower() in EXCEL_TARGET_NAMES:
            target_format = EXCEL_FORMAT
        elif target_format.lower() in EXCEL_SOURCE_NAMES:
            raise UnsupportedFormatError(
                f"不支持的目标格式: {target_format}（xls 是老 BIFF 格式，写不出来；"
                f"Excel 写边只认 excel / xlsx，全部可选目标: "
                f"{', '.join(OUTPUT_FORMAT_CHOICES)}）"
            )
        # 目标格式与输出扩展名是否互相点名，判在 `_write_file` 的门上（那里是字节
        # 的唯一产地）：放在这里就只能判「走 `convert_file`」这条路，直接调用写边
        # 的调用方仍然能把 JSON 字节装进 `.xlsx` 名字。
        
        # 读取输入
        data = self._read_file(input_path, source_format)
        
        # 转换。Excel 读出来是一张全列行记录表，与 `csv` 读边（`csv.DictReader`）
        # **同形**，所以进转换图时按 csv 那一族边走 —— 图里不需要知道 pandas 存在，
        # `DataFormat` 也不因为这个就多一个成员。返回值里的 `source_format` 仍是
        # 真实读到的那份（`excel`），否则调用方会以为自己做了一次 csv 转换。
        graph_source = "csv" if source_format == EXCEL_FORMAT else source_format

        # `jsonl` 是图里唯一一条**已经序列化过**的边（`_json_to_jsonl` 交出字符串行），
        # 而 `_write_file` 的 jsonl 分支会对每条记录再 `json.dumps` 一次 ⇒ 落盘的每一行
        # 是一个 JSON **字符串**而不是对象（`"{\"instruction\": ...}"`）。自家读边恰好
        # 能双重解包读回来（`_jsonl_to_json` 对 str/非 str 都容忍），所以这个缺陷一直只
        # 在跨工具互操作时暴露：按标准 JSONL 读的外部程序拿到的全是字符串。
        # 修法与「容器布局归写边管」的既有口径一致（见 `_write_file` 的 docstring）：
        # 图只走到规范形，一份记录一行的布局交给写边，序列化只做一次。
        # 返回值里的 `target_format` 仍是调用方声明的 `jsonl`。
        # `excel` 走的是同一条口径，而且更必然：转换图里根本没有 excel 节点（它不是
        # `DataFormat` 成员），图只能走到「记录列表」这份规范形，zip 布局全交给写边。
        graph_target = ("json" if target_format in ("jsonl", EXCEL_FORMAT)
                        else target_format)
        converted = self.convert(data, graph_source, graph_target, **kwargs)
        
        # 写入输出
        self._write_file(output_path, converted, target_format)
        
        return {
            "input_file": str(input_path),
            "output_file": str(output_path),
            "source_format": source_format,
            "target_format": target_format,
            "input_count": len(data),
            "output_count": len(converted) if isinstance(converted, list) else 1
        }
    
    def _infer_format(self, path: Path) -> str:
        """从**输出**文件的扩展名推断目标格式

        认不出就回落 `json`，这是刻意的：写侧回落产出的仍是合法 JSON 字节，调用方
        只是拿到一个「扩展名与内容不符」的文件。**读边不能复用这张表**，因为那份
        文件可能是二进制 xlsx，回落 json 会崩在解码器里（见
        `_resolve_input_format`）。

        `.xlsx` 从 L84 起**认**（A133 关闭：写边接上了 `csv_excel_import.export_to_excel`）。
        `.xls` 不在表里，所以它在这里仍然回落 `json` —— 但那份「回落」走不到磁盘：
        `_write_file` 会拒收（见 `_reject_unwritable_output`），因为这一族里只有它是
        二进制容器，写错不会「只是名字不准」，是「Excel 打不开」。
        """
        return OUTPUT_EXTENSION_FORMATS.get(path.suffix.lower(), "json")

    @staticmethod
    def _reject_unwritable_output(path: Path, format: str) -> None:
        """写边「输出扩展名 ↔ 落盘布局」冲突的唯一产地（判在字节之前）

        `excel` 是写边唯一的**二进制容器**（其余四个是文本），所以只有它要求扩展名与
        声明**互相点名**，三个方向都要判：

        * 输出名是 `.xls` ⇒ 当场拒。读边认它（openpyxl 读得了 BIFF），写边写不出
          （要 xlwt，不在依赖表里）；放行等于继续制造假容器，而 `--format json
          --output out.xls` 这条路恰好绕过 `_infer_format`（显式声明时根本不推扩展名）。
        * 扩展名是 `.xlsx` 而目标不是 excel ⇒ 会写出「zip 名字下的一份 JSON 文本」，
          Excel 打不开。这就是 A133 的原始症状，而 `--format` 在 CLI 上是**必填**参数，
          走命令行绕不过这条声明。
        * 目标是 excel 而扩展名不是 `.xlsx` ⇒ 会把 zip 字节写进一个按扩展名说是文本
          容器的名字里（`--format excel --output out.json`）。

        文本那一族**不设**这条界：`--format alpaca --output out.jsonl` 是既有口径
        （schema 与容器分家，容器由扩展名说了算，见 `_write_file` docstring）。
        """
        suffix = path.suffix.lower()
        if suffix in EXCEL_REJECTED_OUTPUT_SUFFIXES:
            raise UnsupportedFormatError(
                f"无法写出 {path.name}：写 Excel 只产 .xlsx（需要 pandas 与 openpyxl），"
                f"老 .xls 格式要另装 xlwt。把输出名改成 .xlsx，"
                f"或用 --format 显式声明目标格式（可选: "
                f"{', '.join(OUTPUT_FORMAT_CHOICES)}）"
            )
        extension_wants_excel = OUTPUT_EXTENSION_FORMATS.get(suffix) == EXCEL_FORMAT
        target_wants_excel = format == EXCEL_FORMAT
        if extension_wants_excel == target_wants_excel:
            return
        if extension_wants_excel:
            raise UnsupportedFormatError(
                f"输出文件 {path.name} 的扩展名要求 Excel，但 target_format 是 "
                f"{format!r}：那样会写出一份内容是 {format} 的 .xlsx（Excel 打不开）。"
                f"改目标格式为 excel / xlsx，或换输出扩展名。"
            )
        raise UnsupportedFormatError(
            f"target_format 是 excel，但输出文件 {path.name} 的扩展名不是 .xlsx："
            f"那样会把 xlsx 的 zip 字节写进一个按扩展名说是 "
            f"{OUTPUT_EXTENSION_FORMATS.get(suffix, 'json')} 容器的名字里。"
            f"把输出名改成 .xlsx，或换目标格式（可选: "
            f"{', '.join(OUTPUT_FORMAT_CHOICES)}）。"
        )

    def _resolve_input_format(self, path: Path) -> str:
        """从**输入**文件的扩展名推断源格式，认不出就报错而不是回落

        `INPUT_EXTENSION_FORMATS` 是唯一权威，报错文案里的两份清单（扩展名、可显式
        声明的源格式）都从常量生成 ⇒ 加一个源格式不会让文案漂移。文案点出
        `--input-format`，因为那正是调用方跳过推断的那条路。
        """
        inferred = INPUT_EXTENSION_FORMATS.get(path.suffix.lower())
        if inferred is None:
            raise UnsupportedFormatError(
                f"无法从扩展名推断输入格式: {path.name}"
                f"（认得的扩展名: {', '.join(sorted(INPUT_EXTENSION_FORMATS))}；"
                f"或用 --input-format 显式声明: "
                f"{', '.join(INPUT_FORMAT_CHOICES)}）"
            )
        return inferred

    def _read_excel(self, path: Path) -> List[Dict[str, Any]]:
        """按路径读 Excel（判据与内存入口共用 `_read_excel_source`）"""
        return self._read_excel_source(path, path.name)

    @staticmethod
    def _read_excel_source(source: Any, name: str) -> List[Dict[str, Any]]:
        """读 Excel：整张表、全部列，空格子按 csv 读边口径落成空串

        `source` 可以是路径，也可以是 `BytesIO`，这是本方法与 `_read_excel` 的唯一区别。

        拆成「吃文件对象」而不是只吃路径，是因为上传面拿到的是**内存里的字节**：
        实测 `pd.read_excel(BytesIO(xlsx 字节))` 直接读通（pandas 按内容而不是文件名
        选引擎，见 `Temp/l91q/error_shapes.py` 第 4 档），于是上传不必先落一份临时
        文件再读回来（那是 `web.max_upload_bytes` 之外的又一处磁盘写）。

        **下面那道 `except Exception` 翻译是必需的，不是装饰**：同一份探针量到损坏
        xlsx 在 pandas 侧抛的是 `zipfile.BadZipFile` 与 `pandas.errors.OptionError`，
        两者都**不是** `ValueError` 子类 ⇒ 经 API 就是 500，把「你上传的文件坏了」
        说成「服务端故障」。`DataFormatError` 是 `ValueError` 子类，走 `to_http_error`
        回 400，且文案带文件名。

        pandas 是**可选依赖**，import 推迟到真要读 xlsx 的时候 —— `import augmentor`
        在没有 pandas 的环境里必须继续可用（aug 侧解释器就是这种环境，L50 口径下每轮
        都要在它上面跑全量）。

        不复用 `csv_excel_import.import_from_excel`：它按 `text_column`/`output_column`
        只留两列并把值一律 `str()` 化，那是「导入为训练数据」的收敛口径；`convert_file`
        是格式转换器，剥列会丢数据。
        """
        from . import csv_excel_import as cei
        if not cei.HAS_PANDAS:
            raise DataLoadError(
                f"读取 Excel 需要安装 pandas 与 openpyxl: "
                f"pip install pandas openpyxl（{name}）"
            )
        try:
            frame = cei.pd.read_excel(source)
        except Exception as e:
            raise DataFormatError(
                f"{name} 读不出 Excel 工作表：文件可能损坏、其实不是 xlsx，"
                f"或是老 .xls 格式而缺 xlrd（{type(e).__name__}）"
            ) from e
        return [
            {key: excel_cell(value) for key, value in row.items()}
            for row in frame.to_dict(orient="records")
        ]
    
    def _read_file(self, path: Path, format: str) -> Any:
        """读取文件

        `excel` 必须在这句 `open()` **之前**分流：xlsx 是二进制，用文本模式打开它，
        报错会停在 `codecs` 的解码错上（`UnicodeDecodeError: 'utf-8' codec can't
        decode byte ...`），而不是「这份文件要交给 pandas」这件事本身。

        文本那四路的判据住在 `_read_stream`（与内存上传共用一份），这里只负责开文件。
        """
        if format == EXCEL_FORMAT:
            return self._read_excel(path)

        with open(path, 'r', encoding='utf-8', newline='') as f:
            return self._read_stream(f, format, path.name)

    @classmethod
    def _read_stream(cls, f, format: str, name: str) -> Any:
        """从**已打开的文本流**按声明格式读，是 `_read_file` 与内存上传共用的那一半

        `newline=""` 是 csv 模块的硬要求：不传时 Python 会先把 `\\r\\n` 归一成
        `\\n`，被引号包裹的字段内换行因此错位（写侧同理）。路径入口由 `_read_file`
        带这个参数打开文件；内存入口给 `io.StringIO(text, newline="")` —— 两个方向
        都必须显式，否则同一份字节在两条路上会读出不同的记录。

        只有 `CONTAINER_FORMATS` 里那四个值决定「怎么解析」。声明成
        alpaca/sharegpt/chatml/… 时它只是**行内 schema**，容器交给内容嗅探：整份能
        解析成一个 JSON 数组（或单条对象）就按 JSON 读，否则按 JSONL 一行一条读。
        旧实现把 schema 当容器用、一律 `json.load`，于是同一批 alpaca 记录写成
        `.jsonl` 就抛 `JSONDecodeError: Extra data: line 2 column 1`——调用方已经
        声明了正确的源格式，却绊在容器上。
        """
        if format == "jsonl":
            return [json.loads(line) for line in f if line.strip()]
        elif format == "csv":
            reader = csv.DictReader(f)
            return list(reader)
        elif format == "tsv":
            reader = csv.DictReader(f, delimiter="\t")
            return list(reader)
        elif format == "json":
            return json.load(f)
        return cls._read_schema_container(f, name, format)

    @staticmethod
    def _read_schema_container(f, name: str, format: str) -> Any:
        """读「schema 已声明、容器未声明」的流：先整份 JSON，再退到一行一条

        只接受「记录数组」与「单条记录对象」两种整份形态；顶层是裸标量（`null`、
        `42`）时不算读通，继续走逐行分支，让报错停在「这条记录不是对象」上，而不是
        凭空多出一条空记录。

        两条路都不通时抛 `DataFormatError` 而不是漏出 `JSONDecodeError`：前者带
        文件名与行号，后者只有解析器的内部措辞（`Extra data: line 2 column 1`）。
        """
        text = f.read()
        try:
            value = json.loads(text)
        except json.JSONDecodeError:
            pass
        else:
            if isinstance(value, list):
                return value
            if isinstance(value, dict):
                return [value]

        rows: List[Any] = []
        for lineno, line in enumerate(text.splitlines(), start=1):
            if not line.strip():
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError as exc:
                raise DataFormatError(
                    f"{name} 声明为 {format}，但它既不是合法 JSON，"
                    f"第 {lineno} 行也不是合法 JSON: {exc}"
                ) from exc
        if not rows:
            # 全空白 = 零条记录，和 `_read_file` 的 jsonl 分支口径一致；
            # 报成「不是合法 JSON」会把一份空数据集说成一份坏数据集。
            return []
        return rows
    
    def _write_file(self, path: Path, data: Any, format: str) -> None:
        """写入文件

        `newline=""` 是 csv 模块的硬要求：默认文本模式会把 writer 的 `\\r\\n`
        行尾再翻译一次，Windows 上落成 `\\r\\r\\n`，严格解析器会在每条记录之间
        读出一个空行。`fieldnames` 取全量键并集而非 `data[0].keys()`，见
        `csv_fieldnames`。

        与 `_read_file` 对称：schema 格式（alpaca 等）自己不定容器，落盘布局由
        **输出扩展名**决定，所以 `convert_file(a.json, b.jsonl, target="alpaca")`
        写出来的真是一份 `.jsonl`，下一次能被 `_read_file` 原样读回。旧实现把
        任何非 jsonl/csv 的目标一律 `json.dump` 成一个数组，写进 `.jsonl` 之后
        自己那一侧就读不回来了。

        `excel` 是唯一走 `pandas` 的分支，也是唯一**不接受**「扩展名与声明不符」的
        分支：文本容器写错名字仍是可读的文本，zip 写错名字则什么都打不开，所以门口
        站了 `_reject_unwritable_output`（判在 `mkdir` 之前 —— 拒绝一次不该留下目录）。
        """
        self._reject_unwritable_output(path, format)
        path.parent.mkdir(parents=True, exist_ok=True)
        if format == EXCEL_FORMAT:
            self._write_excel(path, data)
            return
        if format not in CONTAINER_FORMATS:
            container = self._infer_format(path)
            format = container if container in CONTAINER_FORMATS else "json"

        with open(path, 'w', encoding='utf-8', newline='') as f:
            if format == "jsonl":
                for item in data:
                    f.write(json.dumps(item, ensure_ascii=False) + '\n')
            elif format == "csv":
                if data:
                    writer = csv.DictWriter(f, fieldnames=csv_fieldnames(data))
                    writer.writeheader()
                    writer.writerows(data)
            elif format == "tsv":
                if data:
                    writer = csv.DictWriter(f, fieldnames=csv_fieldnames(data),
                                            delimiter="\t")
                    writer.writeheader()
                    writer.writerows(data)
            else:
                json.dump(data, f, ensure_ascii=False, indent=2)

    def _write_excel(self, path: Path, data: Any) -> None:
        """写 Excel：整张表、全部列，与 csv 写边同一条列序口径

        本方法是 Excel 写边在本仓库的**唯一产品落点**（A133：`export_to_excel` 那份
        实现早就有，从来没有调用者）。列序显式传 `csv_fieldnames(data)` 而不是让它
        自己推：异构记录（某一路数据多了 `category` 列）下两个写边必须给出同一张表头，
        否则 `csv → xlsx` 与 `csv → csv` 会产出列序不同的两份东西。

        pandas 是**读边**的可选依赖，写边从 L85 起不是了：实现搬进
        `augmentor/excel_write.py`（只 import openpyxl），所以这里那条
        `from . import excel_write` 不会把 pandas 拉进进程。理由与读数见 `excel_write`
        模块 docstring —— 关键是 `csv_excel_import` 在 import 期就 `import pandas`，
        实现住在那里等于让每次写 xlsx 付一次 371.8 ms 的导入税。

        非对象记录在这里会被 `_require_object_records` 拦下（一张表的一行就是一条
        对象记录，标量没有列可放）；嵌套值（`history` 那种列表）**不**拦 —— openpyxl
        把它 `str()` 化进单元格，实测与 csv 写边对同样输入给的字面量一致。

        **那一刀判在依赖门之前**，顺序本身是契约：数据里有标量行这件事跟本机装没装
        openpyxl 无关，先报「装依赖」会把一个改数据就能修好的错误说成改依赖才能修好（aug
        侧解释器就是那种环境，L84 第一次跑两侧时正是这一格红）。
        """
        self._require_object_records(data, "json", EXCEL_FORMAT)
        from . import excel_write
        if not excel_write.HAS_OPENPYXL:
            raise DataLoadError(
                f"写入 Excel 需要安装 openpyxl: "
                f"pip install openpyxl（{path.name}）"
            )
        excel_write.export_table_to_excel(data, path, columns=csv_fieldnames(data))


def convert_dataset(data: List[Dict], 
                   target_format: str,
                   **kwargs) -> Any:
    """转换数据集格式
    
    Args:
        data: 数据列表
        target_format: 目标格式
    
    Returns:
        转换后的数据
    """
    converter = DatasetConverter()
    return converter.convert(data, "json", target_format, **kwargs)


def convert_file(input_path: str,
                output_path: str,
                target_format: Optional[str] = None,
                source_format: Optional[str] = None,
                **kwargs) -> Dict:
    """转换文件格式

    Args:
        input_path: 输入文件路径
        output_path: 输出文件路径
        target_format: 目标格式（为 None 时从扩展名推断）。写 Excel 写 `excel` / `xlsx`
            任一种，输出名必须是 `.xlsx`；老 `.xls` 写不出来（要 xlwt），两种方向都
            会报 `UnsupportedFormatError` 而不是落一份假容器。
        source_format: 源格式（为 None 时从扩展名推断）。alpaca/sharegpt/chatml 这类
            只是**行内 schema**，扩展名推不出来，只能显式给；给了之后文件本身是
            `.json` 还是 `.jsonl` 由内容嗅探，不用再声明第三个参数。
            Excel 写 `excel` / `xlsx` / `xls` 任一种都一样，`.xlsx` / `.xls` 后缀也能
            直接认出来；读出来是「整张表、全部列」的行记录表（缺 pandas 时报
            `DataLoadError`，不是 `ImportError`  traceback）。

    Returns:
        转换结果
    """
    converter = DatasetConverter()
    return converter.convert_file(input_path, output_path,
                                  source_format=source_format,
                                  target_format=target_format, **kwargs)


def read_records(content: bytes, source_format: str,
                 filename: str = "") -> List[Dict[str, Any]]:
    """把**内存里的**一份字节读成规范记录列表（`_read_file` 的孪生入口）

    **为什么要第二个入口**：上传端点拿到的是 multipart 部件的字节，不是路径。它此前
    只会 `json.loads`，于是 `.csv` / `.tsv` / `.jsonl` / `.xlsx` 四种进料一律 400
    （实测见 `Temp/l91q/upload_census.py`：表格三种回「数据文件不是合法 JSON」，
    二进制两种回「上传内容不是合法的 UTF-8 文本」），而这份能力早就住在转换器的读边。
    接到临时文件再 `convert_file` 也能做，但那要为一次拒收风险的字节再开一次磁盘写；
    实测 pandas 按**内容**而不是文件名选引擎，`read_excel(BytesIO(...))` 直接读通。

    与路径入口**共用同一套判据**（`_read_stream` / `_read_excel_source`），不另写一份
    csv/jsonl 解析 —— 一份权威只住一处。落点选择、空结果判决、扩展名撒谎的交叉检查
    属**产品面口径**，不在这里（这里只是格式工具）。

    Args:
        content: 原始字节
        source_format: 源格式，取 `INPUT_FORMAT_CHOICES` 任一种（Excel 的三种拼法
            `excel` / `xlsx` / `xls` 同义）
        filename: 只用于报错文案；为空时文案里称「上传内容」

    Returns:
        规范记录列表。**`source_format` 是 `json` 时不做顶层判决**（原样返回
        `json.loads` 的结果，可能是对象），形状判决归调用方（API 侧即
        `api/deps.py:assert_dataset_shape`）—— 与 `_read_file` 的 json 分支同口径。

    Raises:
        UnsupportedFormatError: `source_format` 不在读边清单里
        UnicodeDecodeError: 文本格式但字节不是 UTF-8（由调用方映射成 400）
        DataFormatError: 解析失败，或 Excel 文件损坏 / 缺 xlrd
        DataLoadError: 要读 Excel 但环境里没有 pandas
    """
    converter = DatasetConverter()
    declared = source_format.lower()
    if declared in EXCEL_SOURCE_NAMES:
        declared = EXCEL_FORMAT
    if declared != EXCEL_FORMAT and declared not in INPUT_FORMAT_CHOICES:
        raise UnsupportedFormatError(
            f"认不出的源格式: {source_format}（读边可选: "
            f"{', '.join(INPUT_FORMAT_CHOICES)}）"
        )
    name = filename or "上传内容"

    if declared == EXCEL_FORMAT:
        return converter._read_excel_source(io.BytesIO(content), name)

    data = converter._read_stream(
        io.StringIO(content.decode("utf-8"), newline=""), declared, name)
    if declared == "json":
        return data
    return converter.convert(data, declared, "json")


def get_supported_formats() -> List[str]:
    """列出**转换图真的支持**的目标格式

    由 `_converters` 反推，不按 `DataFormat` 成员列：枚举里可以有图里没有的边，
    以枚举为来源就会虚报能力（照它调用只会拿到 `UnsupportedFormatError`）。
    `tsv` 曾经是这种虚报——枚举有成员、`_infer_format` 认扩展名，但图里缺
    `json -> tsv` 这条边；补上边之后它才真的进这份清单。
    """
    converters = DatasetConverter()._converters
    supported = {"json"} | {target for source, target in converters if source == "json"}
    return [fmt.value for fmt in DataFormat if fmt.value in supported]
