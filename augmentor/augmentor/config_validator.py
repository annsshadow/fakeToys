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
from dataclasses import dataclass, field, fields, is_dataclass
from pathlib import Path
from enum import Enum

# 区间常量与运行时判据同一批来源（`config.py` 开头 + `retry.MAX_RETRY_AFTER`）。
# 这里是 A77 的修法本体：此前两边各抄一遍数字，抄漏的一边就成了「校验器独有
# 天花板」—— `max_retries: 10**6` 在这里报红、在 SDK 直构那边畅通无阻。
from .config import (AUTO_SAVE_INTERVAL_MIN, MAX_OUTPUT_TOKENS_MIN,
                     MAX_RETRIES_RANGE, MODEL_TYPES, NUM_THREADS_RANGE,
                     PORT_RANGE,
                     RATE_LIMIT_MIN_REQUESTS, RATE_LIMIT_MIN_WINDOW_SECONDS,
                     REQUEST_TIMEOUT_RANGE, RETRY_DELAY_RANGE,
                     TEMPERATURE_RANGE, TOP_P_RANGE, VARIANTS_PER_SEED_RANGE,
                     AppConfig, ModelConfig)
from .logging_setup import LOGGING_LEVELS, build_formatter
from .retry import MAX_RETRY_AFTER

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
        "augmentation.variants_per_seed": {
            "type": int, "min": VARIANTS_PER_SEED_RANGE[0],
            "max": VARIANTS_PER_SEED_RANGE[1]},
        "augmentation.num_threads": {
            "type": int, "min": NUM_THREADS_RANGE[0],
            "max": NUM_THREADS_RANGE[1]},
        # 每 N 条自动存一次增量。下界 1 是 L51 补的：实测改前 0 与 −1 都和 1 逐字
        # 同答（`processed - last >= interval` 恒真），20 条样本触发 20 次增量写盘，
        # 默认档 10 只 2 次 —— 不是崩，是 10 倍静默写放大，而且两侧都没有判据。
        "augmentation.auto_save_interval": {
            "type": int, "min": AUTO_SAVE_INTERVAL_MIN},
        # 重试两旋钮自 L45 起真的被消费（经 create_model_backend 透传给模型后端），
        # 所以要在校验器里有对应门槛。上界不是形式主义：`max_retries: 1000000` 在最坏
        # 情况下是 10^6 × 30s 的等待，`retry_delay` 再大也会被退避上限夹住，但 60s 已经
        # 远超任何合理的单次退避基数。**L51 起这两条不再只是校验器独有的天花板**：
        # 同一批常量已经进了 `AugmentationConfig.__post_init__`（A77）。
        "augmentation.max_retries": {
            "type": int, "min": MAX_RETRIES_RANGE[0],
            "max": MAX_RETRIES_RANGE[1]},
        "augmentation.retry_delay": {
            "type": float, "min": RETRY_DELAY_RANGE[0],
            "max": RETRY_DELAY_RANGE[1]},
        # 等待预算两旋钮（L49 / A73 + A75）。上界与运行时判据**同源**，不是校验器
        # 独有的天花板：`max_retry_wait` 本身就是那道封顶，放大它会作废
        # 「服务端指令一支封顶 300 s」的承诺（retry.MAX_RETRY_AFTER）；
        # `retry_jitter` 的 0-1 与 `validation.require_ratio` 的默认闭区间逐字一致。
        "augmentation.max_retry_wait": {"type": float, "min": 0.0,
                                        "max": MAX_RETRY_AFTER},
        "augmentation.retry_jitter": {"type": float, "min": 0.0, "max": 1.0},
        # 单次请求超时的全局档（A74 / L71）。区间与运行时判据同一批常量
        # （`config.REQUEST_TIMEOUT_RANGE`，A77 口径）：下界 1 是实测硬界 ——
        # `requests` 对 `timeout=0` 直接抛 `ValueError`，而那发生在第一次真实
        # 调用上，「校验绿、跑时炸」正是本仓逐轮封死的那道缝；上界 600 对齐
        # `docs/DEPLOYMENT.md` 的 nginx `proxy_read_timeout 600s`。
        # 每模型的覆盖档 `models.<名字>.request_timeout` 的规格住在下面那张
        # `MODEL_ENTRY_FIELDS`（L72 补的，同一条界同一个常数）：本表表达不了
        # 「任意键名下的一种子键」，所以按 `models.*.<键>` 归一后去那张表查。
        # （L71 曾把这一档只留在运行时那一侧，实测留下过一条 A77 镜像缝，见下。）
        "augmentation.request_timeout": {
            "type": float, "min": REQUEST_TIMEOUT_RANGE[0],
            "max": REQUEST_TIMEOUT_RANGE[1]},
        "quality": {"type": dict},
        "quality.enabled": {"type": bool},
        "quality.threshold": {"type": float, "min": 0.0, "max": 1.0},
        "dedup": {"type": dict},
        "dedup.enabled": {"type": bool},
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
        "web.port": {"type": int, "min": PORT_RANGE[0], "max": PORT_RANGE[1]},
        "web.host": {"type": str, "non_empty": True},
        "web.static_dir": {"type": str, "non_empty": True},
        "web.cors_origins": {"type": list, "items": str},
        "web.cors_credentials": {"type": bool},
        "web.data_roots": {"type": list, "items": str},
        "web.rate_limit_max_requests": {"type": int,
                                        "min": RATE_LIMIT_MIN_REQUESTS},
        "web.rate_limit_window_seconds": {"type": float,
                                          "min": RATE_LIMIT_MIN_WINDOW_SECONDS},
        "web.rate_limit_exempt_paths": {"type": list, "items": str},
        # `logging` 节自 L57 起真的被装配（`logging_setup.apply_logging_config`），
        # 所以三键也进了规格表 —— 否则就是 A77 的镜像症状：`level: INFORMATION`
        # 这种看着像拼错的写法在 `validate-config` 绿灯、在 `load_config` 抛。
        # `choices` / `renderable` 两个形状键都是「运行时拒的这里才拒」（承 L51 的
        # `items` / `non_empty`）：允许集只有一份（`LOGGING_LEVELS`），可渲染性判据
        # 直接调运行时那同一个函数，两边不可能漂。
        # `file` 不写 `non_empty`：空串是**合法值**，意思是「不落文件」，正是默认档。
        "logging": {"type": dict},
        "logging.level": {"type": str, "choices": LOGGING_LEVELS},
        "logging.file": {"type": str},
        "logging.format": {"type": str, "non_empty": True, "renderable": True},
    }

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
    MODEL_ENTRY_FIELDS = {
        # `choices` 这一维 L57 就有了（`logging.level` 是第一位用户），
        # `type` 是第二个：清单与运行时 `ModelConfig.__post_init__` 共引
        # `MODEL_TYPES`，两边不可能各抄一份再漂（A77）。
        "type": {"type": str, "choices": MODEL_TYPES},
        "temperature": {"type": float, "min": TEMPERATURE_RANGE[0],
                        "max": TEMPERATURE_RANGE[1]},
        "top_p": {"type": float, "min": TOP_P_RANGE[0], "max": TOP_P_RANGE[1]},
        "max_output_tokens": {"type": int, "min": MAX_OUTPUT_TOKENS_MIN},
        "request_timeout": {"type": float, "min": REQUEST_TIMEOUT_RANGE[0],
                            "max": REQUEST_TIMEOUT_RANGE[1], "nullable": True},
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

    @classmethod
    def consumed_section_keys(cls) -> Dict[str, Set[str]]:
        """推出「被消费的节 → 有人读的键集」，权威只有一个：`AppConfig` 的字段类型

        为什么可以放心推导（L52 的结构论据）：`load_config` 用
        `_load_section(raw, key, cls, defaults)` 加载每一节，**只按 `defaults` 的键**
        取字段，而 `tests/unit/test_config.py::TestSectionDefaultsMatchTheMappingTable`
        （L49 立、L50 清空豁免清单）已经把「`defaults` 的键集 == 该 dataclass 的字段集」
        冻成常驻断言。于是 dataclass 字段集与那张映射表在同一件事上同值，抄第三份
        清单（像 A77 之前的两份区间数字那样）就是本仓已经付过代价的错误。

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
                    sections[f.name] = {sf.name for sf in fields(value)}
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

    def _warn_unread_model_keys(self, models: Dict, result: ValidationResult) -> None:
        """模型条目里的子键按 `ModelConfig` 的字段集判（同一套推导，第二个面）

        `models.<名字>` 的键集用户自己起，所以顶层不进判据；但它的**子项**形状是
        封闭的：`load_config` 对每个条目读 `type` / `api_key` / `secret_key` /
        `base_url` / `model` / `temperature` / `top_p` / `max_output_tokens` /
        `request_timeout` 九个键，实测与 `ModelConfig` 的字段集逐字相等（差集两侧
        都空；L52 记的是八个，L71 加 `request_timeout` 后本句跟着改数）。清单本身
        是 `sorted(f.name for f in fields(ModelConfig))` 推导的，不是抄的 ⇒ 加字段
        不需要记得改这里；反方向（规格表里有、字段集没有）由
        `tests/unit/test_model_entry_ranges_l72.py::test_no_ghost_model_spec` 钉。
        方向守护见
        `tests/unit/test_config_validator.py::TestUnreadKeyWarnings`。
        """
        known = sorted(f.name for f in fields(ModelConfig))
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
        self._validators = {
            dict: self._validate_dict,
            list: self._validate_list,
            str: self._validate_string,
            int: self._validate_int,
            float: self._validate_float,
            bool: self._validate_bool,
        }
    
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

        # 「写了没人读」的键（A76）：独立一遍走，不塞进上面那个规格走查里，
        # 因为它的权威来源是 `AppConfig` 的字段集而不是 `KNOWN_FIELDS`
        self._warn_unread_keys(config, result)

        # 验证环境变量引用
        self._validate_env_refs(config, "", result)
        
        return result
    
    def _check_required_fields(self, config: Dict, prefix: str, result: ValidationResult):
        """检查必填字段"""
        for field_path, spec in self.KNOWN_FIELDS.items():
            if not spec.get("required"):
                continue
            
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
                elif spec.get("non_empty") and not value:
                    result.add_error(field_path, "值不能为空字符串")
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
    
    def _validate_dict(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证字典"""
        if not isinstance(value, dict):
            result.add_error(path, f"期望字典类型, 实际 {type(value).__name__}")
    
    def _validate_list(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证列表"""
        if not isinstance(value, list):
            result.add_error(path, f"期望列表类型, 实际 {type(value).__name__}")
    
    def _validate_string(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证字符串"""
        if not isinstance(value, str):
            result.add_error(path, f"期望字符串类型, 实际 {type(value).__name__}")
        elif "enum" in spec and value not in spec["enum"]:
            result.add_error(path, f"值不在允许范围内: {value}")
    
    def _validate_int(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证整数"""
        if not isinstance(value, int) or isinstance(value, bool):
            result.add_error(path, f"期望整数类型, 实际 {type(value).__name__}")
        else:
            if "min" in spec and value < spec["min"]:
                result.add_error(path, f"值过小: {value} < {spec['min']}")
            if "max" in spec and value > spec["max"]:
                result.add_error(path, f"值过大: {value} > {spec['max']}")
    
    def _validate_float(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证浮点数"""
        if not isinstance(value, (int, float)) or isinstance(value, bool):
            result.add_error(path, f"期望数值类型, 实际 {type(value).__name__}")
        else:
            if "min" in spec and value < spec["min"]:
                result.add_error(path, f"值过小: {value} < {spec['min']}")
            if "max" in spec and value > spec["max"]:
                result.add_error(path, f"值过大: {value} > {spec['max']}")
    
    def _validate_bool(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证布尔值"""
        if not isinstance(value, bool):
            result.add_error(path, f"期望布尔类型, 实际 {type(value).__name__}")


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
