# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置验证模块

提供配置文件的验证和校验功能。
"""

import os
import re
import yaml
import logging
import difflib
from typing import Any, Dict, List, Optional, Set, Union
from dataclasses import (dataclass, field, fields, is_dataclass, replace)
from pathlib import Path
from enum import Enum

# 区间常量与运行时判据同一批来源（`config.py` 开头 + `retry.MAX_RETRY_AFTER`）。
# 这里是 A77 的修法本体：此前两边各抄一遍数字，抄漏的一边就成了「校验器独有
# 天花板」—— `max_retries: 10**6` 在这里报红、在 SDK 直构那边畅通无阻。
from .config import (AUTO_SAVE_INTERVAL_MIN, DEDUP_THRESHOLD_RANGE,
                     EXPORT_FORMATS, MAX_OUTPUT_TOKENS_MIN,
                     MAX_RETRIES_RANGE, MAX_UPLOAD_BYTES_MIN,
                     MODEL_TYPES, NUM_THREADS_RANGE,
                     PORT_RANGE, QUALITY_THRESHOLD_RANGE, RAG_FORMATS,
                     RATE_LIMIT_MIN_REQUESTS, RATE_LIMIT_MIN_WINDOW_SECONDS,
                     REQUEST_TIMEOUT_RANGE, RETRY_DELAY_RANGE,
                     TEMPERATURE_RANGE, TOP_P_RANGE, VECTOR_BACKENDS,
                     VECTOR_DIMENSION_MIN, VARIANTS_PER_SEED_RANGE,
                     AppConfig, MODEL_ENTRY_KEYS, MODEL_ENTRY_PLACEHOLDER,
                     ModelConfig, RAGConfig)
from .exceptions import DataValidationError
from .logging_setup import LOGGING_LEVELS, build_formatter
from .retry import MAX_RETRY_AFTER
from .validation import is_blank_string, require_chunk_window, require_ratio_list

logger = logging.getLogger(__name__)


class Severity(Enum):
    """严重程度"""
    ERROR = "error"
    WARNING = "warning"
    INFO = "info"


@dataclass
class ValidationError:
    """验证错误"""
    path: str
    message: str
    severity: Severity
    line: Optional[int] = None
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "path": self.path,
            "message": self.message,
            "severity": self.severity.value,
            "line": self.line
        }


@dataclass
class ValidationResult:
    """验证结果"""
    is_valid: bool
    errors: List[ValidationError] = field(default_factory=list)
    warnings: List[ValidationError] = field(default_factory=list)
    info: List[ValidationError] = field(default_factory=list)
    
    def add_error(self, path: str, message: str, line: int = None):
        """添加错误"""
        self.errors.append(ValidationError(path, message, Severity.ERROR, line))
        self.is_valid = False
    
    def add_warning(self, path: str, message: str, line: int = None):
        """添加警告"""
        self.warnings.append(ValidationError(path, message, Severity.WARNING, line))
    
    def add_info(self, path: str, message: str, line: int = None):
        """添加信息"""
        self.info.append(ValidationError(path, message, Severity.INFO, line))
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "is_valid": self.is_valid,
            "errors": [e.to_dict() for e in self.errors],
            "warnings": [e.to_dict() for e in self.warnings],
            "info": [e.to_dict() for e in self.info],
            "summary": {
                "errors": len(self.errors),
                "warnings": len(self.warnings),
                "info": len(self.info)
            }
        }


class ConfigValidator:
    """配置验证器
    
    支持多种配置文件格式的验证。
    """
    
    # 已知的配置模式
    #
    # `app` 段是历史遗留的元信息：`AppConfig` 没有对应字段，`load_config` 也从不
    # 读它。因此**不能**把 `app.*` 设为必填——`save_config` 按 `AppConfig` 的字段集
    # 落盘，永远写不出这个段，一旦设为必填，「保存配置」再「校验配置」必然失败
    # （实测 4 个必填错误）。这里保留为已知字段，仍做类型检查，只是不强制存在。
    # A139 收口（L157 / B227）：本表只留**类型层 + 形状层**（type / items / non_empty /
    # non_blank / required / nullable / renderable），判决层（min / max / choices /
    # item_choices，原先 24 + 5 行）整批撤下，改由 `_validate_replay_sections` 逐键
    # 回放各节 `__post_init__`（运行时权威原样投影，`dataclasses.replace` 单键探针——
    # 整批构造会在第一个坏键上停手、吞掉同节其余键的判决，L157 现场撞出后改逐键）。
    # 一条界从此只住运行时一处（A77 终极形）；回放只在「该路径及其元素档、节级同文案
    # 档均无规格层错误」时出声，同一个错不出两声。
    KNOWN_FIELDS = {
        "app": {"type": dict},
        "app.name": {"type": str},
        "app.version": {"type": str},
        "app.debug": {"type": bool, "default": False},
        # `models.default` 是 `default_model` 的**唯一**来源（见 load_config），
        # 必须必填：缺了它默认模型会静默回落到 "ernie"。
        "models": {"type": dict, "required": True},
        "models.default": {"type": str, "required": True},
        "augmentation": {"type": dict},
        "augmentation.variants_per_seed": {"type": int},
        "augmentation.num_threads": {"type": int},
        # 每 N 条自动存一次增量。下界 1 是 L51 补的：实测改前 0 与 −1 都和 1 逐字
        # 同答（`processed - last >= interval` 恒真），20 条样本触发 20 次增量写盘，
        # 默认档 10 只 2 次 —— 不是崩，是 10 倍静默写放大，而且两侧都没有判据。
        "augmentation.auto_save_interval": {"type": int},
        # 重试两旋钮自 L45 起真的被消费（经 create_model_backend 透传给模型后端），
        # 所以要在校验器里有对应门槛。上界不是形式主义：`max_retries: 1000000` 在最坏
        # 情况下是 10^6 × 30s 的等待，`retry_delay` 再大也会被退避上限夹住，但 60s 已经
        # 远超任何合理的单次退避基数。**L51 起这两条不再只是校验器独有的天花板**：
        # 同一批常量已经进了 `AugmentationConfig.__post_init__`（A77）。
        "augmentation.max_retries": {"type": int},
        "augmentation.retry_delay": {"type": float},
        # 等待预算两旋钮（L49 / A73 + A75）。上界与运行时判据**同源**，不是校验器
        # 独有的天花板：`max_retry_wait` 本身就是那道封顶，放大它会作废
        # 「服务端指令一支封顶 300 s」的承诺（retry.MAX_RETRY_AFTER）；
        # `retry_jitter` 的 0-1 与 `validation.require_ratio` 的默认闭区间逐字一致。
        "augmentation.max_retry_wait": {"type": float},
        "augmentation.retry_jitter": {"type": float},
        # 单次请求超时的全局档（A74 / L71）。区间与运行时判据同一批常量
        # （`config.REQUEST_TIMEOUT_RANGE`，A77 口径）：下界 1 是实测硬界 ——
        # `requests` 对 `timeout=0` 直接抛 `ValueError`，而那发生在第一次真实
        # 调用上，「校验绿、跑时炸」正是本仓逐轮封死的那道缝；上界 600 对齐
        # `docs/DEPLOYMENT.md` 的 nginx `proxy_read_timeout 600s`。
        # 每模型的覆盖档 `models.<名字>.request_timeout` 的规格住在下面那张
        # `MODEL_ENTRY_FIELDS`（L72 补的，同一条界同一个常数）：本表表达不了
        # 「任意键名下的一种子键」，所以按 `models.*.<键>` 归一后去那张表查。
        # （L71 曾把这一档只留在运行时那一侧，实测留下过一条 A77 镜像缝，见下。）
        "augmentation.request_timeout": {"type": float},
        # `quality` / `dedup` 两节的规格自 L76 / A118 起与运行时判据同源：那两节
        # 补上了 `__post_init__`，界就住在 `config.QUALITY_THRESHOLD_RANGE` /
        # `config.DEDUP_THRESHOLD_RANGE`，这里只引常数。改前的不对称实测在两面上：
        # - `quality.threshold` **有**规格，但 0.0/1.0 是裸数字（与运行时不同源）；
        # - `dedup.threshold` **整条规格不存在** ⇒ 该键在静态面 **0 条反馈**。
        #   档位读数见 `Temp/l76q/yaml_faces.json`：`1.7` / `5.0` / `-1.0` / `'x'` /
        #   `'0.9'` 五档的静态反馈条数全是 0，而同样五档写在 `quality.threshold` 上
        #   就有「值过大 / 值过小 / 类型错误」出声。基座用的是**出厂全量配置**而不是
        #   半份配置 —— 后者会先报「缺少必填字段: models」，那条噪声会把「这个键到底
        #   有没有反馈」糊成一片红。运行时那一侧偏要等到建 `Deduplicator` 才抛，
        #   两半正好各缺一角。
        # `quality.weights` 本轮**不补规格**：它的形状与「和为 1」判据的权威在
        # `quality.QualityScorer`，配置侧目前只判 null（A77 的口径是「运行时拒的
        # 这里才拒」，反过来造一道运行时没有的界就是本表已经封死过的错误）。剩下的
        # 那一洞记在 A124，两侧同批补。
        "quality": {"type": dict},
        "quality.enabled": {"type": bool},
        "quality.threshold": {"type": float},
        # A124 收口（L153，B223）：weights 的形状权威在 `validation.require_ratio_list`，
        # 本表只判「是 list」这一层（非 list 交类型检查报），逐项/长度/和=1 在
        # `_validate_quality_weights` 回放里走运行时同一份判据（rag 窗先例同形）。
        "quality.weights": {"type": list},
        "dedup": {"type": dict},
        "dedup.enabled": {"type": bool},
        "dedup.threshold": {"type": float},
        # `output` / `output.export_dir` 两条规格在 L52 删掉了。它们是**规格表自己
        # 造的死旋钮**：`load_config` 里没有 `output` 节（真节后是 `export`，字段是
        # `default_format` / `formats`），实测 `output: {export_dir: out}` 得到
        # `is_valid=True` / 0 error / 0 warning，而 `load_config` 之后
        # `hasattr(config, "output") == False` —— 用户照表写就被静默丢弃。留着它，
        # 本轮新加的「没人读」判据会与它当场矛盾（一边给绿灯、一边报没人读），
        # 而 `consumed_section_keys()` 的方向守护（A76 的反向棘轮）正是要让这种
        # 幽灵规格无法存在。导出目录从来不是配置项，它一直是 CLI/API 的 `--output`
        # 参数（`docs/` 里也没有任何 `export_dir` 文档，删除不产生文档债）。
        # `web` 节的规格自 L50 起补齐（此前整节 9 个字段在 `validate-config` 上零反馈：
        # 实测把 port 写成字符串、data_roots 写成 YAML 标量、限流数写成负数与 NaN，
        # 一份配置里六个键全错仍判 is_valid=True / 0 error / 0 warning）。`data_roots`
        # 是这里最要紧的一条：`data_roots: data` 会被按字符拆成 d/a/t/a 四个根目录，
        # 于是**所有**数据端点一律 403，症状长得像后端坏了。区间与运行时判据同一批
        # 常量（`config.PORT_RANGE` 等）：L51 已给 `WebConfig` 补上 `__post_init__`，
        # SDK 直构也认这套区间，「校验器红 / 运行时绿」的缝隙就此封住（关闭 A80）。
        # `items` / `non_empty` 两个形状键同样是 L51 补的，方向相反（新立 A83）：
        # 改前 `data_roots: [null]` 与 `host: ""` 在校验器绿灯、在 `load_config` 抛。
        "web": {"type": dict},
        "web.port": {"type": int},
        "web.host": {"type": str, "non_empty": True},
        "web.static_dir": {"type": str, "non_empty": True},
        "web.cors_origins": {"type": list, "items": str},
        "web.cors_credentials": {"type": bool},
        "web.data_roots": {"type": list, "items": str},
        "web.max_upload_bytes": {"type": int},
        "web.rate_limit_max_requests": {"type": int},
        "web.rate_limit_window_seconds": {"type": float},
        "web.rate_limit_exempt_paths": {"type": list, "items": str},
        # `logging` 节自 L57 起真的被装配（`logging_setup.apply_logging_config`），
        # 所以三键也进了规格表 —— 否则就是 A77 的镜像症状：`level: INFORMATION`
        # 这种看着像拼错的写法在 `validate-config` 绿灯、在 `load_config` 抛。
        # `choices` / `renderable` 两个形状键都是「运行时拒的这里才拒」（承 L51 的
        # `items` / `non_empty`）：允许集只有一份（`LOGGING_LEVELS`），可渲染性判据
        # 直接调运行时那同一个函数，两边不可能漂。
        # `file` 不写 `non_empty`：空串是**合法值**，意思是「不落文件」，正是默认档。
        # 但它写 `non_blank`（A142 / L83 新维度）：纯空白不是「不落文件」而是手滑，
        # 运行时那一侧由 `config.py` 的 `LoggingConfig.__post_init__` 共引同一个
        # `validation.is_blank_string` 拒掉 —— 两把钥匙差一格，正是为了不把
        # 「空串合法」这一档设计抹平。
        "logging": {"type": dict},
        "logging.level": {"type": str},
        "logging.file": {"type": str, "non_blank": True},
        "logging.format": {"type": str, "non_empty": True, "renderable": True},
        # `export` / `vector` / `rag` / `multimodal` 四节 14 键的规格（A118 余四节 / L82）。
        # 本节规格此前**一条都没有**：实测 20 档越界写法在 `validate-config` 上 0 反馈
        # （`Temp/l82q/before.json`），四节里除 `quality.weights` 之外没有一个键有界。
        # 三条清单（`EXPORT_FORMATS` / `VECTOR_BACKENDS` / `RAG_FORMATS`）与运行时
        # `__post_init__` 共引 `config.py` 那三个**推导**出来的常数 —— 权威只有一处，
        # 本表连重抄都没有（比 `MODEL_TYPES` 那一先例更紧一档：那里因循环导入只能
        # 两边各持一份并由测试钉相等）。
        #
        # `chunk_size` / `chunk_overlap` 两行**故意不写 `min`**：那条界是三刀的
        # （下界 1、下界 0、关系 `overlap < size`），它的唯一产地是
        # `validation.require_chunk_window`，静态面在 `_validate_rag_window` 里直接
        # 调那同一个函数（同 `renderable` 直调 `build_formatter` 的先例）。把 1 与 0
        # 抄进本表就是给同一条界造第二个家 —— 那正是 A77 的成因。
        "export": {"type": dict},
        "export.default_format": {"type": str},
        # `item_choices` 是本表第一位新用户（下面 `items` 那段判据）：清单型列表的
        # 元素成员资格，与运行时 `ExportConfig.__post_init__` 那个逐项
        # `require_choice` 同一判据。
        "export.formats": {"type": list, "items": str},
        "vector": {"type": dict},
        "vector.enabled": {"type": bool},
        "vector.backend": {"type": str},
        "vector.dimension": {"type": int},
        "vector.storage_dir": {"type": str, "non_empty": True},
        "vector.collection": {"type": str, "non_empty": True},
        "rag": {"type": dict},
        "rag.enabled": {"type": bool},
        "rag.default_format": {"type": str},
        "rag.chunk_size": {"type": int},
        "rag.chunk_overlap": {"type": int},
        "multimodal": {"type": dict},
        "multimodal.enabled": {"type": bool},
        "multimodal.image_extensions": {"type": list, "items": str},
        "multimodal.audio_extensions": {"type": list, "items": str},
        # A140 余 11 节 29 键的规格（L154 / B224）：这 11 节此前**一条规格行都没有**
        # （静态面 0 反馈，与运行时 `__post_init__` 缺席同病；A140 现量房在
        # `Temp/l82q/ungated_census.json`，11 节名单与键数 29 都是那里推导的）。
        # 形状键（type / items / non_empty / min）与 `web` / `export` 各行同语法；四个
        # int 键的 `min: 1` 与运行时 `require_count(minimum=1)` 同档，`tests/unit/
        # test_config_gates_l154.py` 钉数值相等 —— 这里没有可共引的既有常数，1 住在
        # 调用点字面量里，本表与运行时同档，不另立第二产地。
        "context": {"type": dict},
        "context.enabled": {"type": bool},
        "context.num_turns": {"type": int},
        "versioning": {"type": dict},
        "versioning.enabled": {"type": bool},
        "versioning.storage_dir": {"type": str, "non_empty": True},
        "versioning.auto_snapshot": {"type": bool},
        "sampler": {"type": dict},
        "sampler.enabled": {"type": bool},
        "sampler.dimensions": {"type": list, "items": str},
        "expander": {"type": dict},
        "expander.enabled": {"type": bool},
        "expander.strategies": {"type": list, "items": str},
        "tracker": {"type": dict},
        "tracker.enabled": {"type": bool},
        "tracker.metrics": {"type": list, "items": str},
        "visualization": {"type": dict},
        "visualization.enabled": {"type": bool},
        "visualization.types": {"type": list, "items": str},
        "multilingual": {"type": dict},
        "multilingual.enabled": {"type": bool},
        "multilingual.default_target_lang": {"type": str, "non_empty": True},
        "multilingual.supported_langs": {"type": list, "items": str},
        "multilingual.translate_batch_size": {"type": int},
        "evaluation": {"type": dict},
        "evaluation.enabled": {"type": bool},
        "evaluation.metrics": {"type": list, "items": str},
        "evaluation.reference_field": {"type": str, "non_empty": True},
        "benchmark": {"type": dict},
        "benchmark.enabled": {"type": bool},
        "benchmark.baseline_file": {"type": str, "non_empty": True},
        "benchmark.metrics": {"type": list, "items": str},
        "active_learning": {"type": dict},
        "active_learning.enabled": {"type": bool},
        "active_learning.strategy": {"type": str, "non_empty": True},
        "active_learning.batch_size": {"type": int},
        "active_learning.max_iterations": {"type": int},
        "frameworks": {"type": dict},
        "frameworks.enabled": {"type": bool},
        "frameworks.frameworks": {"type": list, "items": str},    }

    # `models.<名字>.<键>` 的规格（L72 / A113，同时补掉 L71 记下的那个缺口）。
    #
    # 为什么单开一张表：`KNOWN_FIELDS` 的键是**固定路径**，表达不了「任意模型名下
    # 的一种子键」—— 模型名由用户自己起，抄不完。L71 就是按这句话把每模型覆盖档
    # 留在运行时那一侧的（原注释在本表 `augmentation.request_timeout` 上方），实测
    # 后果是 A77 的**镜像症状**：`models.m.request_timeout: 0` 在 `validate-config`
    # 上 0 错、在 `load_config` 里抛（取证 Temp `l72q/probe_before.json` 最后一例）。
    # 折法是把路径 `models.<名字>.<键>` 归一成 `models.*.<键>` 再查本表（见
    # `_spec_for`），界仍引 `config.py` 那批常数 ⇒ 一条界还是只住一个地方。
    #
    # `nullable` 是新加的一维，只给 `request_timeout`：它的 `None` 是「本模型不覆盖
    # 全局档」这一档**本身**（`conf.get(key)` 不带默认值），运行时 `require_seconds`
    # 放行 `None`，静态面也必须放行，否则两边又拆开了。采样三键**不进** `nullable`：
    # 它们有默认值，「写了键没给值」在两侧都是坏写法（运行时由
    # `_reject_null_fields(..., names=...)` 拒，静态面由类型判据拒）。
    #
    # `required` 这一维本表第一次有用户（A119 / L75）：必填检查原先只遍历
    # `KNOWN_FIELDS` 的**固定点分路径**，而本表按**键名**索引 —— 于是「键整个不在场」
    # 这一维对模型条目完全失明：实测 `models.qwen` 不写 `type` 得到 `is_valid=True` /
    # 0 error / 0 warning，只有 `load_config` 那侧抛（`Temp/l75q/before.json` 的
    # `missing-type` 档）。`_required_paths` 把两张表折成一份清单，判决文案仍只有一条
    # 来源。只有 `type` 带 `required`：它是唯一一个**没有** dataclass 默认值的字段，
    # 其余七个键「不在场」是合法状态（由 `config.py` 的字段默认回答，A114）。
    MODEL_ENTRY_FIELDS = {
        # `choices` 这一维 L57 就有了（`logging.level` 是第一位用户），
        # `type` 是第二个：清单与运行时 `ModelConfig.__post_init__` 共引
        # `MODEL_TYPES`，两边不可能各抄一份再漂（A77）。
        "type": {"type": str, "required": True},
        # `model` 是本轮补上的第二个字符串键（A114）：改前本表没有它，运行时也没有
        # 任何判据 ⇒ 实测 `model: ''` / `model:`（null）/ `model: 123` / `model: true`
        # 四形状**两侧全绿**（同探针的 `model_shapes` 档），而那值是要直发后端的
        # （`openai` / `claude` / `ollama` 的请求体、`gemini` 的 URL）。`non_empty`
        # 与运行时 `require_string` 同判据（那一维 L51 就有，`logging.format` 是
        # 第一位用户）。三条凭证键至今两侧都无判据，记在 A122。
        "model": {"type": str, "non_empty": True},
        "temperature": {"type": float},
        "top_p": {"type": float},
        "max_output_tokens": {"type": int},
        "request_timeout": {"type": float, "nullable": True},
    }
    
    # 环境变量模式
    ENV_VAR_PATTERN = re.compile(r'\$\{(\w+)\}')

    # 顶层段落里不进「没人读取」判据的两节（A76）：`app` 是历史遗留元信息（见
    # `KNOWN_FIELDS` 上方注释：`AppConfig` 没有对应字段、`load_config` 从不读它），
    # `models` 的键是**模型名**而不是旋钮，其子项另按 `ModelConfig` 判。
    META_TOP_SECTIONS = {"app", "models"}

    # 「写了没人读」这一路文案的共用词。诊断面（`validate_config`）与产品面
    # （`load_config` 的出声，见 A84）比对的是同一批条目，比对方式就是这个词在不在
    # 文案里 —— 所以它必须是常量，而不是两处各抄一份字符串。
    UNREAD_MARKER = "没人读取"

    # 推导缓存。`AppConfig()` 要构造 20 个节对象并跑各自的 `__post_init__` 判据，
    # 这个钱一次进程只该付一遍，而不是每次 `validate_config` 付一遍。
    _CONSUMED_SECTIONS: Optional[Dict[str, Set[str]]] = None

    # `rag` 两键的缺省档（L82 / A118）：静态面的跨键判据需要它，因为「只写
    # `chunk_overlap`、`chunk_size` 走默认」也是一份合法配置，而那条界要看两侧。
    # 值从 `RAGConfig` 的字段拿而不是抄数字 —— A123 之后 dataclass 就是默认值的唯一
    # 权威，抄第二份就又造出一个会漂的地方。
    _RAG_WINDOW_DEFAULTS = {
        "chunk_size": RAGConfig().chunk_size,
        "chunk_overlap": RAGConfig().chunk_overlap,
    }

    @classmethod
    def consumed_section_keys(cls) -> Dict[str, Set[str]]:
        """推出「被消费的节 → 有人读的键集」，权威只有一个：`AppConfig` 的字段类型

        为什么可以放心推导：`load_config` 用 `_load_section(raw, key, cls)` 加载每一节，
        而它取键的依据就是那个类自己的 dataclass 字段集（`cls.__dataclass_fields__`），
        并且**只算 `init` 字段** —— 非 init 字段写了也进不了构造器，本方法必须同样不认
        它，否则那个键既不会生效、又不在「写了没人读」名单里（A76 的静默形状）。
        A123 / L77 之前这里还多一个 `defaults` 参数，推导要借那张手抄表当中间人、靠
        L49/L50 的棘轮才敢等价；表删掉之后**加载器与本方法是同一个来源**，中间人不
        存在了。抄第三份清单（像 A77 之前的两份区间数字那样）就是本仓已经付过代价的
        错误。

        「推导」与「手写清单」还剩一处对账：`load_config` 里那份 20 行的
        「节名 → 类」清单是手写的，本类只推导字段类型，不知道哪些节真的会被加载。
        两边相等由
        `tests/unit/test_config_validator.py::TestUnreadKeyWarnings::test_the_derived_whitelist_equals_what_load_section_reads`
        与 `tests/unit/test_section_registry_l77.py::TestSectionRegistryShape` 钉住 ——
        清单漏一节时，判据会把那节的合法键报成「写了没人读」。

        返回里不含 `models` / `default_model`：前者是模型名到 `ModelConfig` 的映射，
        后者根本不是节。判据一侧对它们的处理见 `_warn_unread_keys`。
        """
        if cls._CONSUMED_SECTIONS is None:
            sections: Dict[str, Set[str]] = {}
            baseline = AppConfig()
            for f in fields(AppConfig):
                value = getattr(baseline, f.name)
                # `models` 是 dict、`default_model` 是 str：不是节，跳过
                if is_dataclass(value):
                    # 只认 **init** 字段：加载器一侧（`_load_section`）现在按
                    # `names[k].init` 取键，非 init 字段写了也不会生效 ⇒ 它必须出现在
                    # 「写了没人读」名单里，两侧同源同判（A77 的键集版）。
                    sections[f.name] = {sf.name for sf in fields(value) if sf.init}
            cls._CONSUMED_SECTIONS = sections
        return cls._CONSUMED_SECTIONS

    @staticmethod
    def _suggest(name: str, candidates: List[str]) -> str:
        """把「是否想写 X」拼进警告文案；没有相近项就不拼（别把猜测写成结论）"""
        match = difflib.get_close_matches(name, candidates, n=1, cutoff=0.7)
        return "" if not match else "；是否想写 %s？" % match[0]

    def _warn_unread_keys(self, config: Dict, result: ValidationResult) -> None:
        """配置里「写了没人读」的键必须出声（A76）

        改前实测（Temp `l52q/probe1.py` / `probe2.py` NONCE-45A0C1AB9510）：节名拼错
        （`augmenation.variants_per_seed: 999`）与节内键名拼错（`augmentation
        .variant_per_seed: 999`）两边都得到 `is_valid=True` / 0 error / 0 warning，
        而 `load_config` 那侧读回默认值 5 —— 症状是「配置不起作用」而不是「配置写错
        了」。把 20 个被消费的节各塞一个假键，今天的反馈同样是 0 / 0。

        只用 WARNING，不用 ERROR：多余的键不会让服务起不来（`_load_section` 直接忽略
        它们），而 `is_valid` 是 CLI 退出码与 `POST /api/system/validate-config` 的
        判据 —— 判负会把「配置里夹了自己段落」的老用户凭空挡红。
        """
        sections = self.consumed_section_keys()

        for key, value in config.items():
            if key in self.META_TOP_SECTIONS:
                if key == "models" and isinstance(value, dict):
                    self._warn_unread_model_keys(value, result)
                continue
            if key not in sections:
                result.add_warning(
                    key, "顶层段落%s: %s（写了不会生效）%s" % (
                        self.UNREAD_MARKER, key, self._suggest(key, sorted(sections))))
                continue
            if not isinstance(value, dict):
                # 形状问题（`augmentation: abc`）由 `KNOWN_FIELDS` 的 `type: dict`
                # 判成 ERROR，这里不重复报同一件事
                continue
            known = sections[key]
            for sub_key in value:
                if sub_key not in known:
                    result.add_warning(
                        "%s.%s" % (key, sub_key),
                        "键%s: %s.%s（值不会生效）%s" % (
                            self.UNREAD_MARKER, key, sub_key, self._suggest(sub_key, sorted(known))))

    def _validate_rag_window(self, config: Dict, result: ValidationResult) -> None:
        """`rag` 节的窗口三刀（L82 / A118）：本表唯一一条**跨键**判据

        为什么要单开一遍而不是写进 `KNOWN_FIELDS`：那张表每条规格只看**一个**值，
        而 `chunk_overlap < chunk_size` 是两个值的事。只判各自 `type: int` 的话，
        `rag.chunk_overlap: 600`（不写 `chunk_size`，默认 512）在 `validate-config`
        上绿灯、在 `load_config` 抛 ⇒ 又回到 A77 的镜像症状。本函数不重写判据，直接
        把运行时那**同一个** `validation.require_chunk_window` 放一遍（先例是
        `renderable` 直调 `build_formatter`），缺省档取 `_RAG_WINDOW_DEFAULTS` ⇒
        「界」与「默认值」两侧都没有第二份。

        报错路径用节名：这条判决的文案自己就带两个键名（`rag.chunk_size 必须是…` /
        `rag.chunk_overlap 必须小于 rag.chunk_size，当前是 600 对 512`），位置指得出。
        非整数的形状问题由 `type: int` 那一遍报，本函数遇到就**跳过**，免得同一件
        手滑报两次。
        """
        body = config.get("rag")
        if not isinstance(body, dict):
            return
        size = body.get("chunk_size", self._RAG_WINDOW_DEFAULTS["chunk_size"])
        overlap = body.get("chunk_overlap",
                           self._RAG_WINDOW_DEFAULTS["chunk_overlap"])
        for value in (size, overlap):
            if isinstance(value, bool) or not isinstance(value, int):
                return
        try:
            require_chunk_window("rag.chunk_size", size,
                                 "rag.chunk_overlap", overlap)
        except DataValidationError as exc:
            result.add_error("rag", str(exc))

    def _validate_quality_weights(self, config: Dict, result: ValidationResult) -> None:
        """`quality.weights` 的形状回放（L153 / A124）：把运行时那份 `require_ratio_list`
        原样放一遍（rag 窗先例：不重写判据，只回放，「界与判据」两侧没有第二份）。

        非 list 的形状由 `KNOWN_FIELDS` 的类型检查报（本函数遇到就跳过，免报两次）；
        `null` 由运行时的 `_reject_null_fields` 拒（YAML 面「写了键没给值」才走到这，
        按「没给 = 用默认」放行给类型层，与出厂配置行为一致）。
        """
        body = config.get("quality")
        if not isinstance(body, dict):
            return
        value = body.get("weights")
        if value is None or not isinstance(value, (list, tuple)):
            return
        try:
            require_ratio_list("quality.weights", list(value))
        except DataValidationError as exc:
            result.add_error("quality.weights", str(exc))

    def _warn_unread_model_keys(self, models: Dict, result: ValidationResult) -> None:
        """模型条目里的子键按**加载侧那一份键集**判（A126 / L78 起不再自己推导）

        `models.<名字>` 的键集用户自己起，所以顶层不进判据；但它的**子项**形状是
        封闭的，就是 `ModelConfig` 那些可传入的字段（今天九个：`type` / `api_key` /
        `secret_key` / `base_url` / `model` / `temperature` / `top_p` /
        `max_output_tokens` / `request_timeout`；L52 记的是八个，L71 加了
        `request_timeout` 之后本句跟着改数）。

        改前这里自己写一遍 `sorted(f.name for f in fields(ModelConfig))`，与加载侧的
        `MODEL_ENTRY_KEYS` 属于「同一个来源、两份推导」—— `init` 判据要各写一遍、
        也就各漂一次，正是 L77 在节面上刚收掉的那个形状。本行现在直引那个常量，
        于是「加载侧认为没人读的键，反馈侧一定报得出」是**结构上**成立，而不是靠
        用例钉着。顺带把每次调用新建字段元组变成排一份导入期算好的集合，实测本函数
        快 −40.29 % / −40.02 %（改后连两跑，各 9/9 交替块更快；逐字读数只住
        `Temp/l78q/perf_validator_face.json` 的 `runs` 键）。反方向（规格表里有、字段集没有）由
        `tests/unit/test_model_entry_ranges_l72.py::test_no_ghost_model_spec` 钉。
        方向守护见 `tests/unit/test_config_validator.py::TestUnreadKeyWarnings`。
        """
        known = sorted(MODEL_ENTRY_KEYS)
        for name, body in models.items():
            if name == "default" or not isinstance(body, dict):
                continue
            for key in body:
                if key not in known:
                    result.add_warning(
                        "models.%s.%s" % (name, key),
                        "键%s: models.%s.%s（值不会生效）%s" % (
                            self.UNREAD_MARKER, name, key, self._suggest(key, known)))

    def __init__(self):
        """初始化验证器"""
        # 旧的 _validators 分发表（dict/list/str/int/float/bool 映射六个 _validate_* 方法）
        # 从无任何调用点：类型检查一直走 _validate_known_fields 的本地 isinstance。
        # 死表与六个不可达方法已在 L126 一并删除
    
    def validate_file(self, file_path: str) -> ValidationResult:
        """验证配置文件
        
        Args:
            file_path: 配置文件路径
        
        Returns:
            验证结果
        """
        result = ValidationResult(is_valid=True)
        
        path = Path(file_path)
        if not path.exists():
            result.add_error("file", f"文件不存在: {file_path}")
            return result
        
        if not path.is_file():
            result.add_error("file", f"不是有效文件: {file_path}")
            return result
        
        try:
            with open(path, 'r', encoding='utf-8') as f:
                config = yaml.safe_load(f)
        except yaml.YAMLError as e:
            result.add_error("yaml", f"YAML解析错误: {str(e)}")
            return result
        except Exception as e:
            result.add_error("file", f"读取文件错误: {str(e)}")
            return result
        
        return self.validate_config(config)
    
    def validate_config(self, config: Dict) -> ValidationResult:
        """验证配置字典
        
        Args:
            config: 配置字典
        
        Returns:
            验证结果
        """
        result = ValidationResult(is_valid=True)
        
        if not isinstance(config, dict):
            result.add_error("config", "配置必须是字典类型")
            return result
        
        # 检查必填字段
        self._check_required_fields(config, "", result)
        
        # 验证已知字段
        self._validate_known_fields(config, "", result)

        # 跨键关系（L82 / A118）：规格表表达不了的那一类，单独一遍回放运行时判据
        self._validate_rag_window(config, result)

        # 权重三件套（L153 / A124）：逐项 bool/NaN/越界 + 长度 + 和=1
        self._validate_quality_weights(config, result)

        # 整节回放（L157 / A139）：判决层的唯一产地是各节 `__post_init__`，
        # 静态面逐键构造当投影，与规格层按路径去重
        self._validate_replay_sections(config, result)

        # 「写了没人读」的键（A76）：独立一遍走，不塞进上面那个规格走查里，
        # 因为它的权威来源是 `AppConfig` 的字段集而不是 `KNOWN_FIELDS`
        self._warn_unread_keys(config, result)

        # 验证环境变量引用
        self._validate_env_refs(config, "", result)
        
        return result
    
    def _required_paths(self, config: Dict) -> list:
        """必填检查要走的 `(路径, 规格)` 清单（A119 / L75）

        `KNOWN_FIELDS` 的键是点分**固定**路径，模型条目却是 `models.<用户起的名字>.<键>`
        ⇒ 那一条维只能按**键名**查 `MODEL_ENTRY_FIELDS`（L72 的折叠），于是「键整个
        不在场」对模型条目改前完全失明。本函数把两张表折成一份清单交给
        `_check_required_fields`，判决文案（「缺少必填字段」/「字段不能为null」）
        仍然只住那一处 —— 不给同一个判据造第二个权威。

        条目的 `default` 那一行不是模型条目（它是「默认用哪个条目名」的指针），
        形状已经不对的条目（`qwen: abc`）留给形状那一层判，这里不重复报一条
        「缺少 `type`」的噪声。
        """
        paths = [(field_path, spec)
                 for field_path, spec in self.KNOWN_FIELDS.items()
                 if spec.get("required")]
        models = config.get("models")
        if isinstance(models, dict):
            for name, body in models.items():
                if name == "default" or not isinstance(body, dict):
                    continue
                for key, spec in self.MODEL_ENTRY_FIELDS.items():
                    if spec.get("required"):
                        paths.append(("models.%s.%s" % (name, key), spec))
        return paths

    def _check_required_fields(self, config: Dict, prefix: str, result: ValidationResult):
        """检查必填字段"""
        for field_path, spec in self._required_paths(config):
            parts = field_path.split(".")
            current = config
            
            for part in parts:
                if isinstance(current, dict) and part in current:
                    current = current[part]
                else:
                    result.add_error(field_path, f"缺少必填字段: {field_path}")
                    break
            else:
                # 字段存在
                if current is None:
                    result.add_error(field_path, f"字段不能为null: {field_path}")
    
    def _spec_for(self, field_path: str) -> Optional[Dict[str, Any]]:
        """按路径取规格：固定路径查 `KNOWN_FIELDS`，模型条目折成 `models.*.<键>` 查
        `MODEL_ENTRY_FIELDS`。

        折叠只做一次、且只在「三段且首段是 `models`」这条路径上做，因为四段以上
        在本仓不存在（`web.cors_origins[0]` 那种元素路径由 `items` 判据在同一次
        命中里处理，不另开规格）。归一化把「用户起的模型名」从查找里摘掉，
        于是新增一个模型条目不需要动规格表 —— 反过来也成立：**规格表里没有的
        模型键就是一份「静态面没人管」的清单**，那条对账由
        `tests/unit/test_model_entry_ranges_l72.py` 钉住。
        """
        spec = self.KNOWN_FIELDS.get(field_path)
        if spec is not None:
            return spec
        parts = field_path.split(".")
        if len(parts) == 3 and parts[0] == "models":
            return self.MODEL_ENTRY_FIELDS.get(parts[2])
        return None

    def _validate_known_fields(self, config: Dict, prefix: str, result: ValidationResult):
        """验证已知字段"""
        for key, value in config.items():
            field_path = f"{prefix}.{key}" if prefix else key
            
            spec = self._spec_for(field_path)
            if spec is not None:
                # `nullable`（L72）：这个键的 `None` 是一档**合法语义**（「不覆盖，
                # 用上一层的默认」），与运行时 `require_*` 家族对 `None` 的短路同读法。
                # 判在类型检查之前，否则 `models.m.request_timeout:`（写了键没给值）
                # 会在这里报「期望 float, 实际 NoneType」，而那边照样放行 ⇒ 又拆两侧。
                if spec.get("nullable") and value is None:
                    continue
                # 类型检查
                expected_type = spec.get("type")
                # `float` 字段同时接受 `int`：YAML 里 `retry_delay: 1` / `threshold: 1`
                # 是最自然的写法，运行时的 `require_seconds` 也收 int。校验器若比运行
                # 时更严，就会把合法配置报成非法（实测：加本规则后 `retry_delay: 1`
                # 曾得到「类型错误: 期望 float, 实际 int」）。
                checked_type = (int, float) if expected_type is float else expected_type
                # `bool` 不算数值。`isinstance(True, int)` 恒真，所以 YAML 里写
                # `retry_jitter: true` 从前会被本校验器读成合法，而运行时四道判据
                # （`require_count` / `require_seconds` / `require_positive` /
                # `require_ratio`）**全部**显式拒 bool ⇒ `validate-config` 绿灯的配置
                # 会在建管道时抛（L49 实测：7 个数值规格键 7/7 都有这个洞）。
                # `type: bool` 的开关字段不受影响。
                if checked_type and (
                        (expected_type in (int, float) and isinstance(value, bool))
                        or not isinstance(value, checked_type)
                ):
                    result.add_error(field_path, 
                                   f"类型错误: 期望 {expected_type.__name__}, 实际 {type(value).__name__}")
                    continue
                # NaN 过不了任何比较（`nan < min` 与 `nan > max` 都是 False），于是
                # 会绕过下面的范围检查被读成「合法」，而运行时那一层拒收它——两边口径
                # 必须一致，否则 `validate-config` 绿灯的配置会在建管道时炸。
                if expected_type is float and value != value:
                    result.add_error(field_path, f"值不是有效数值: {value}")
                    continue
                
                # 范围检查
                if "min" in spec and value < spec["min"]:
                    result.add_error(field_path, 
                                   f"值过小: {value} < {spec['min']}")
                if "max" in spec and value > spec["max"]:
                    result.add_error(field_path, 
                                   f"值过大: {value} > {spec['max']}")
                # 形状判据（L51 / A83）：列表元素类型与非空。这两条以前只有运行时那一侧
                # 有（`validation.require_string_list` / `require_string`），于是
                # `data_roots: [null]` 与 `host: ""` 在 `validate-config` 上绿灯、在
                # `load_config` 里抛 —— A77 的镜像症状，同一轮一起封住。运行时拒的这里
                # 才拒（`items`/`non_empty` 只写在有运行时判据的键上），免得反向造出
                # 「校验器红 / 运行时绿」。
                if "items" in spec:
                    for i, item in enumerate(value):
                        if not isinstance(item, spec["items"]):
                            result.add_error(
                                f"{field_path}[{i}]",
                                f"元素类型错误: 期望 {spec['items'].__name__}, "
                                f"实际 {type(item).__name__}")
                        elif not item:
                            result.add_error(f"{field_path}[{i}]", "元素不能为空字符串")
                        elif is_blank_string(item):
                            # A142 / L83：与运行时 `require_string_list` 的第三刀同判据，
                            # 判的对象是「整串空白」而不是「长度为 0」——`data_roots: ["  "]`
                            # 在改前两面全绿，落盘得到一个名叫空格的目录根。
                            result.add_error(
                                f"{field_path}[{i}]",
                                f"元素不能是纯空白字符串: {item!r}")
                        elif "item_choices" in spec and item not in spec["item_choices"]:
                            # L82 / A118 的新维度：清单型列表（`export.formats`）的
                            # 元素成员资格。与下面 `choices` 同一个来源的清单，只是
                            # 判的对象是每一项。
                            result.add_error(
                                f"{field_path}[{i}]",
                                f"元素不在允许集合内: {item!r}（可选: "
                                f"{'/'.join(spec['item_choices'])}）"
                            )
                elif spec.get("non_empty"):
                    if not value:
                        result.add_error(field_path, "值不能为空字符串")
                    elif is_blank_string(value):
                        # 同上：`web.host: "   "` 改前 `validate-config` 绿灯、
                        # `load_config` 也绿灯（两面都只判 `not value`），本轮两面同改。
                        result.add_error(
                            field_path, f"值不能是纯空白字符串: {value!r}")
                # `non_blank` 是「空串合法、纯空白不合法」那一格（L83 / A142），目前只有
                # `logging.file` 用得上：空串 = 不落文件，`'   '` = 一个名叫空格的文件。
                if spec.get("non_blank") and is_blank_string(value):
                    result.add_error(
                        field_path, f"值不能是纯空白字符串: {value!r}")
                # 允许集合（L57）：`logging.level` 那类「看着像拼错」的写法。集合来自
                # 运行时判据同一个常量，不是校验器独有的天花板。
                if "choices" in spec and value not in spec["choices"]:
                    result.add_error(
                        field_path,
                        f"值不在允许集合内: {value!r}（可选: "
                        f"{'/'.join(spec['choices'])}）"
                    )
                # 可渲染性（L57）：`Formatter("%(nope)s")` 构造期不报错，到发第一条
                # 日志才抛，所以「类型对但值没用」必须在这里判，且判据就是运行时那
                # 同一个函数。
                if spec.get("renderable"):
                    try:
                        build_formatter(value)
                    except ValueError as exc:
                        result.add_error(field_path, str(exc))
            
            # 递归验证嵌套字典
            if isinstance(value, dict):
                self._validate_known_fields(value, field_path, result)
            elif isinstance(value, list):
                for i, item in enumerate(value):
                    if isinstance(item, dict):
                        self._validate_known_fields(item, f"{field_path}[{i}]", result)
    
    def _validate_replay_sections(self, config: Dict, result: ValidationResult) -> None:
        """A139 收口（L157 / B227）：静态面对每节逐键回放运行时判据

        规格表只管类型层与形状层（表头注释）；数值界、清单成员、null 档这些
        **判决**全部住在各节 `__post_init__`（A77：一条界只住运行时一处），
        本方法把构造器逐键跑一遍当静态面投影。

        三条纪律：

        - **类型层先短路**（L139 理由②）：节写成标量、值类型不对，规格走查
          先报「类型错误」；值类型错时探针抛的错与规格层同路径 ⇒ 去重吞掉。
        - **逐键探针**：`dataclasses.replace(默认底, 单键覆盖)`。整批构造在第一个
          坏键上停手（六键写坏只出五声），单键探针让每个键各判各的。
        - **去重按路径**：点分键名已有错误（含元素路径 `key[0]`、节级同文案）
          就不报；模型条目把占位符 `models.<名字>` 折成真实条目名。
        """
        # 专项回放已持有的键（跨键窗口的三刀 `require_chunk_window` 与权重三件套
        # `require_ratio_list` 各由 `_validate_rag_window` / `_validate_quality_weights`
        # 按整批终态回放）：通用回放不重复探——单键探针会把「单独合法、跨键关系
        # 非法」的档位按默认底判成违规（L82 的整批语义归专项回放负责）。
        cross_key_owned = {"rag": {"chunk_size", "chunk_overlap"},
                          "quality": {"weights"}}
        base_probe = AppConfig()
        model_entry_names = set(MODEL_ENTRY_KEYS)
        for f in fields(base_probe):
            cls = type(getattr(base_probe, f.name))
            if not is_dataclass(cls):
                continue
            raw = config.get(f.name)
            if raw is None or not isinstance(raw, dict):
                continue
            base = cls()
            init_names = {sf.name for sf in fields(cls) if sf.init}
            owned = cross_key_owned.get(f.name, set())
            err_paths = {e.path for e in result.errors}
            for k, v in raw.items():
                if k not in init_names or k in owned:
                    continue
                try:
                    replace(base, **{k: v})
                except DataValidationError as exc:
                    path = str(exc).split(" ", 1)[0]
                    same_msg_at_section = any(
                        e.path == path.split(".", 1)[0] and e.message == str(exc)
                        for e in result.errors)
                    if not (path in err_paths or
                            any(q == path or q.startswith(path + "[") for q in err_paths)
                            or same_msg_at_section):
                        result.add_error(path, str(exc))
                        err_paths.add(path)
        models_raw = config.get("models")
        if isinstance(models_raw, dict):
            err_paths = {e.path for e in result.errors}
            for name, entry in models_raw.items():
                if name == "default" or not isinstance(entry, dict):
                    continue
                kwargs = {k: v for k, v in entry.items() if k in model_entry_names}
                if "type" not in kwargs:
                    continue  # 缺必填由 `_check_required_fields` 报，不叠噪声
                type_value = kwargs["type"]
                base = None
                if isinstance(type_value, str) and type_value in MODEL_TYPES:
                    base = ModelConfig(type=type_value)
                for k, v in kwargs.items():
                    try:
                        if base is None:
                            # `type` 本身不在封闭清单：单独探针判「清单成员」那一刀
                            ModelConfig(type=v)
                        else:
                            replace(base, **{k: v})
                    except DataValidationError as exc:
                        path = str(exc).split(" ", 1)[0].replace(
                            MODEL_ENTRY_PLACEHOLDER, "models.%s" % name)
                        same_msg_at_entry = any(
                            e.path == "models.%s" % name and e.message == str(exc)
                            for e in result.errors)
                        if not (path in err_paths or
                                any(q == path or q.startswith(path + "[") for q in err_paths)
                                or same_msg_at_entry):
                            result.add_error(path, str(exc))
                            err_paths.add(path)


    def _validate_env_refs(self, config: Dict, prefix: str, result: ValidationResult):
        """验证环境变量引用"""
        for key, value in config.items():
            field_path = f"{prefix}.{key}" if prefix else key
            
            if isinstance(value, str):
                matches = self.ENV_VAR_PATTERN.findall(value)
                for var_name in matches:
                    if var_name not in os.environ:
                        result.add_warning(field_path, 
                                         f"环境变量未设置: {var_name}")
            elif isinstance(value, dict):
                self._validate_env_refs(value, field_path, result)
            elif isinstance(value, list):
                for i, item in enumerate(value):
                    if isinstance(item, str):
                        matches = self.ENV_VAR_PATTERN.findall(item)
                        for var_name in matches:
                            if var_name not in os.environ:
                                result.add_warning(f"{field_path}[{i}]", 
                                                 f"环境变量未设置: {var_name}")
    


def validate_config_file(file_path: str) -> ValidationResult:
    """验证配置文件
    
    Args:
        file_path: 配置文件路径
    
    Returns:
        验证结果
    """
    validator = ConfigValidator()
    return validator.validate_file(file_path)


def validate_config(config: Dict) -> ValidationResult:
    """验证配置字典
    
    Args:
        config: 配置字典
    
    Returns:
        验证结果
    """
    validator = ConfigValidator()
    return validator.validate_config(config)


def unread_key_messages(config: Dict) -> List[str]:
    """只跑「写了没人读」那一遍，返回给人看的条目（A84：给 `load_config` 在加载这一步出声）

    与 `validate_config` 共用同一个 `_warn_unread_keys`，两条通道由构造同源而不是靠抄写
    保持一致；刻意不跑 `KNOWN_FIELDS` 规格与环境变量引用那两遍 —— 每次加载都要出声的
    只能是一件事「你写的键没人读」，环境变量的条数还随本机 shell 而变（L52 的纪律）。
    同源性由用例守着：`tests/unit/test_config_unread_feedback.py`。
    """
    validator = ConfigValidator()
    result = ValidationResult(is_valid=True)
    validator._warn_unread_keys(config or {}, result)
    return [item.message for item in result.warnings]
