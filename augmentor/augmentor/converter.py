# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集格式转换模块

提供多种数据集格式之间的转换功能。
"""

import json
import csv
import logging
from typing import List, Dict, Optional, Any
from pathlib import Path
from enum import Enum
from .exceptions import DataFormatError, UnsupportedFormatError

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


# 「字段映射类」格式：与规范形（instruction/input/output）共用字段名，逆向只是补默认值
FLAT_FORMATS = ("alpaca", "belle", "llama_factory")

# 「对话类」格式：正文存在对话数组里，逆向要把它折回 instruction/output/history
CONVERSATION_FORMATS = ("sharegpt", "vicuna", "chatml")

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
    if turns[-2]["role"] != "user" or turns[-1]["role"] != "assistant":
        raise DataFormatError(
            f"{format_name} 对话必须以「user → assistant」结尾，"
            f"实际是「{turns[-2]['role']} → {turns[-1]['role']}」"
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
        
        # 查找转换器
        key = (source_format, target_format)
        if key in self._converters:
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
            target_format: 目标格式（为 None 时从扩展名推断）
        
        Returns:
            转换结果
        """
        input_path = Path(input_path)
        output_path = Path(output_path)
        
        # 推断格式
        if source_format is None:
            source_format = self._infer_format(input_path)
        if target_format is None:
            target_format = self._infer_format(output_path)
        
        # 读取输入
        data = self._read_file(input_path, source_format)
        
        # 转换
        converted = self.convert(data, source_format, target_format, **kwargs)
        
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
        """从文件扩展名推断格式"""
        ext_map = {
            ".json": "json",
            ".jsonl": "jsonl",
            ".csv": "csv",
            ".tsv": "tsv",
        }
        return ext_map.get(path.suffix.lower(), "json")
    
    def _read_file(self, path: Path, format: str) -> Any:
        """读取文件

        `newline=""` 是 csv 模块的硬要求：不传时 Python 会先把 `\\r\\n` 归一成
        `\\n`，被引号包裹的字段内换行因此错位（写侧同理）。

        只有 `CONTAINER_FORMATS` 里那四个值决定「怎么解析这个文件」。声明成
        alpaca/sharegpt/chatml/… 时它只是**行内 schema**，容器交给内容嗅探：整份能
        解析成一个 JSON 数组（或单条对象）就按 JSON 读，否则按 JSONL 一行一条读。
        旧实现把 schema 当容器用、一律 `json.load`，于是同一批 alpaca 记录写成
        `.jsonl` 就抛 `JSONDecodeError: Extra data: line 2 column 1`——调用方已经
        声明了正确的源格式，却绊在容器上。
        """
        with open(path, 'r', encoding='utf-8', newline='') as f:
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
            return self._read_schema_container(f, path, format)

    @staticmethod
    def _read_schema_container(f, path: Path, format: str) -> Any:
        """读「schema 已声明、容器未声明」的文件：先整份 JSON，再退到一行一条

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
                    f"{path.name} 声明为 {format}，但它既不是合法 JSON，"
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
        """
        path.parent.mkdir(parents=True, exist_ok=True)
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
        target_format: 目标格式
        source_format: 源格式（为 None 时从扩展名推断）。alpaca/sharegpt/chatml 这类
            只是**行内 schema**，扩展名推不出来，只能显式给；给了之后文件本身是
            `.json` 还是 `.jsonl` 由内容嗅探，不用再声明第三个参数。

    Returns:
        转换结果
    """
    converter = DatasetConverter()
    return converter.convert_file(input_path, output_path,
                                  source_format=source_format,
                                  target_format=target_format, **kwargs)


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
