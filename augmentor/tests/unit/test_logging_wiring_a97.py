# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A97：`logging` 节三键的生效面 + 两侧同判；A101：配置节写成 null / 标量；
A105：渲染探针在同串上重复做（本轮自己的代价，同轮修）

L54 把「写了没人读」的出声通道建在 `load_config` 上，L55 又给 CLI 入口加了
`harden_stdio()` —— 两次都路过 `logging` 这块地，才发现旋钮是空的：`grep` 全仓
`config.logging` / `LoggingConfig` 的命中全在 `config.py` 自己体内，三键
（`level` / `file` / `format`）**零应用方**。改 `logging: {level: ERROR}` 既不能
静音那条 WARNING，`logging.file` 对应的「日志进文件」这件事也从来不存在。

本轮接上之后，口径必须同时守住两件事，所以用例分三面：

1. **不写这节 = 一字不变**。默认值不是「想要的样子」而是今天实测的形状
   （Temp `l57/probe1.txt`：root 装 `StreamHandler` + `Formatter("%(message)s")` +
   `setLevel(WARNING)` 之后，同一条 WARNING 的 stderr 与 `logging.lastResort`
   **逐字节相同**），并且装配的生效条件是「配置里真的写了这节」。
2. **写了就必须生效，且判据两边同一份**：`level` 的允许集是
   `logging_setup.LOGGING_LEVELS` 一处定义（校验器的 `choices` 与运行时都引它），
   `format` 的可渲染性判据调的是运行时那同一个 `build_formatter`（构造期判据走
   `assert_format_renderable` 那层缓存，见 `TestRenderProbeIsNotRepeated`）。
   这条是 L51 的两侧同判口径，防的是 A77 的镜像症状「校验器绿 / 运行时红」。
3. **装配只碰写出的键**。`api/main.py` 自己 `basicConfig(level=INFO, format=…
   %(name)s…)`，CLI 的默认档却是 WARNING + 裸消息 —— 同一个默认值不可能同时等于
   两个面，所以「没写的键不动」是唯一不破坏任何一侧的做法。

A101（本轮撞出，同轮修）：节名写了却没给值（`logging:`）或写成标量
（`logging: app.log`）时，`_load_section` 在 `conf.get` 上抛
`AttributeError: 'NoneType' object has no attribute 'get'` —— 实测**每一节**同形
（Temp `l57/probe1.txt` P4/P5，`web` / `quality` / `dedup` 全中），与 A94 是同一根
因的第二层。修法同 A94：null = 该节全默认，标量 = 明说是摆错了形状。

A105（本轮自己造出来的代价，同轮修）：判据接上之后，`load_config` 在**配置里从没写
`logging` 节**的机器上慢了 +30~+37 µs（三轮同号，Temp `l57/cost_prememo.txt`）。根因
不在判据本身，而在它被挂了**两遍** —— 一次加载会构造两次 `LoggingConfig`
（`AppConfig()` 的 default_factory、`_load_section` 各一次），每次构造都要对着真
record 试渲染一次默认串。修法是缓存「已通过」的格式串（`assert_format_renderable`），
判据一条不省：坏串永远进不了缓存，每次构造照样出声。
"""

import contextlib
import dataclasses
import io
import logging
import logging.handlers
import os
import sys
import uuid

import pytest
import yaml

from augmentor import logging_setup
from augmentor.config import (AppConfig, DedupConfig, LoggingConfig, QualityConfig,
                              WebConfig, _load_section, load_config, save_config)
from augmentor.config_validator import ConfigValidator, validate_config
from augmentor.exceptions import ConfigError, DataValidationError
from augmentor.logging_setup import (LOGGING_KEYS, LOGGING_LEVELS,
                                     apply_logging_config, build_formatter,
                                     level_number)


@pytest.fixture(autouse=True)
def isolate_root_logger():
    """把 root logger 的 handler 与级别隔离在单条用例内

    本模块的用例全部在**进程内**动 root logger（集成面另有真子进程文件）。不隔离
    就会互相污染：上一条留下的 handler 会让下一条「root 有没有 handler」这一支的
    走向随执行顺序而变，那是比断言失败更难查的假绿。
    """
    root = logging.getLogger()
    saved_handlers, saved_level = root.handlers[:], root.level
    for handler in logging_setup._INSTALLED[:]:
        handler.close()
    logging_setup._INSTALLED.clear()
    # A106 之后本模块还记着「上次完整装配」的签名。夹具手工清空 root/INSTALLED 时
    # 走的是绕过 `_detach_installed` 的路子，必须一并把 `_APPLIED` 归 None，否则上一
    # 条用例留下的签名会漏进下一条，让「该 miss 的调用」假命中。
    logging_setup._APPLIED = None
    root.handlers[:] = []
    root.setLevel(saved_level)
    yield
    for handler in logging_setup._INSTALLED[:]:
        if handler in root.handlers:
            root.removeHandler(handler)
        handler.close()
    logging_setup._INSTALLED.clear()
    logging_setup._APPLIED = None
    root.handlers[:] = saved_handlers
    root.setLevel(saved_level)


def _write(tmp_path, name, text):
    path = tmp_path / name
    path.write_text(text, encoding="utf-8", newline="\n")
    return str(path)


def _console():
    """root 上的**控制台** handler（按精确类型取）

    不能按 `isinstance(h, logging.StreamHandler)` 筛：pytest 的 `LogCaptureHandler`
    正是它的子类，混进来会让「装了几条」这类计数断言随 pytest 的夹具而变（实测
    `[0, 0] == [0]` 那一红就是这个形状）。
    """
    return [h for h in logging.getLogger().handlers if type(h) is logging.StreamHandler]


def _files():
    """root 上的**文件** handler（按精确类型取）

    与 `_console()` 同一口径：钉在具体类上，行为变更时当场红。L62 起本模块装的是
    `RotatingFileHandler`（`logging.file` 有界轮转，A104），不再是裸 `FileHandler`。
    """
    return [h for h in logging.getLogger().handlers if type(h) is logging.handlers.RotatingFileHandler]


@contextlib.contextmanager
def _empty_root():
    """把 root 清成「CLI 进程今天的形状」（一条 handler 都没有）后执行体内做事

    本模块装不装控制台 handler，判据只有一个 —— `root.handlers` 是否为空。pytest 在
    每条用例的 **call 阶段**才挂上它自己的 `LogCaptureHandler`，所以在夹具(setup)里
    清空是没用的（清空之后它又加回来，实测 `_console() == []` 那两条因此假绿），必须
    在测试体内用 `with _empty_root():` 清。不这么做的用例测的是 API 进程那一支
    （root 已有 handler），而不是 CLI 那一支。
    """
    root = logging.getLogger()
    saved = root.handlers[:]
    root.handlers[:] = []
    try:
        yield root
    finally:
        for handler in logging_setup._INSTALLED[:]:
            if handler in root.handlers:
                root.removeHandler(handler)
            handler.close()
        logging_setup._INSTALLED.clear()
        root.handlers[:] = saved


PROBE_LOGGER = "augmentor.logging_setup"


@contextlib.contextmanager
def _count_probe_records():
    """数「本模块为了判可渲染性而构造的 LogRecord」有几条

    按 logger 名筛，不按总条数筛：pytest 自己会发记录，产品面那条「键没人读取」的
    warning 也发记录（它的 logger 是 `augmentor.config`），混在一起计数就会随夹具
    与执行顺序而变 —— 那就是一条测不准的断言。
    """
    seen = []
    original = logging.LogRecord.__init__

    def counted(self, *args, **kwargs):
        name = args[0] if args else kwargs.get("name")
        if name == PROBE_LOGGER:
            seen.append(name)
        return original(self, *args, **kwargs)

    logging.LogRecord.__init__ = counted
    try:
        yield seen
    finally:
        logging.LogRecord.__init__ = original


class TestLevelNumber:
    """`logging.level` 的允许集：只有一份，且大写才认"""

    @pytest.mark.parametrize("name", LOGGING_LEVELS)
    def test_every_allowed_name_maps_to_the_stdlib_constant(self, name):
        assert level_number(name) == getattr(logging, name)

    def test_typo_that_looks_like_info_is_refused(self):
        with pytest.raises(DataValidationError) as got:
            level_number("INFORMATION")
        assert "CRITICAL" in str(got.value), "拒收却不给允许集，等于让人再猜一次"

    def test_lowercase_name_is_refused_too(self):
        """`logging.setLevel` 对小写是 `ValueError: Unknown level`（实测 P4）⇒ 两边
        同判的代价就是不做大小写归一：宁可明确拒，也不在校验器与运行时各归一套。"""
        with pytest.raises(DataValidationError):
            level_number("warning")

    @pytest.mark.parametrize("value", [30, None, True, ["INFO"]])
    def test_non_name_values_are_refused(self, value):
        with pytest.raises(DataValidationError):
            level_number(value)


class TestBuildFormatter:
    """`logging.format` 判的是「对着真 record 打得开」，不是「像不像格式串」"""

    @pytest.mark.parametrize("fmt", [
        "%(message)s",
        "%(asctime)s - %(levelname)s - %(message)s",
        "%(levelname)-8s %(name)s %(message)s",
    ])
    def test_renderable_formats_pass(self, fmt):
        assert isinstance(build_formatter(fmt), logging.Formatter)

    def test_format_that_only_fails_at_emit_time_is_refused(self):
        """`Formatter("%(nope)s")` 构造期不报错，到发第一条日志才抛 ⇒ 接线前无害，
        接线后每条日志变成 31 行 `--- Logging error ---`（实测 P2/P3）"""
        with pytest.raises(DataValidationError) as got:
            build_formatter("%(nope)s")
        assert "%(nope)s" in str(got.value), "报错里要看得见那个坏串，否则还得自己找"

    def test_invalid_style_format_is_refused(self):
        with pytest.raises(DataValidationError):
            build_formatter("%d")

    def test_error_is_a_value_error_so_existing_handlers_catch_it(self):
        """`cli.py:main()` 的兜底 `except Exception` 之外，`ValueError` 这一档还保证
        配置校验面（`validate_config`）与 SDK 直构拿到的是同一类异常"""
        with pytest.raises(ValueError):
            build_formatter("%(nope)s")


class TestRenderProbeIsNotRepeated:
    """A105：探针只判「这个串第一次出现」，判据一条不省，省的是重复

    选题来自本轮自己的代价实测。`load_config` 一次会把 `LoggingConfig` 构造**两遍**
    （`AppConfig()` 的 default_factory 一遍、`_load_section` 一遍），而渲染判据挂在
    `__post_init__` 上 ⇒ 一份**从没写过 `logging` 节**的配置，每次加载也为一个谁也没
    写的默认值做两次真 record 试渲染。实测（Temp `l57/rec_prememo.txt`
    NONCE-L57-REC-PREMEMO）2 条/次加载，`load_config` 因此慢 +30~+37 µs、三轮同号
    （Temp `l57/cost_prememo.txt`）。缓存「已判过的串」之后同一读数 0 条、差值
    落进单臂抖动里不再同号（Temp `l57/cost.txt`）。

    修法必须是缓存而不是省判据，所以下面三条分别钉住：只探一次、坏串每次都出声、
    以及产品面一次加载 0 条。
    """

    @staticmethod
    def _unique_valid_format():
        """一个**可通过判据**的一次性格式串：缓存是进程级的，写死串的话上一条用例
        判过它，这一条就测不到「第一次」了。"""
        return uuid.uuid4().hex[:12] + " %(message)s"

    def test_a_string_is_probed_once_not_once_per_construction(self):
        fmt = self._unique_valid_format()
        with _count_probe_records() as seen:
            LoggingConfig(level="WARNING", file="", format=fmt)
            after_first = len(seen)
            for _ in range(50):
                LoggingConfig(level="WARNING", file="", format=fmt)
        assert after_first == 1, "第一次构造必须真判一次，否则缓存成了跳过判据"
        assert len(seen) == 1, "同串重复构造 51 次只该探 1 次"

    def test_a_bad_string_raises_on_every_construction(self):
        """缓存只收**通过之后**的串 ⇒ 坏串永远走真判据。这一条是上一条的防空转：
        把缓存改成「收所有串（含坏的）」，第二条构造就悄悄放行了。"""
        for _ in range(3):
            with pytest.raises(DataValidationError):
                LoggingConfig(level="WARNING", file="", format="%(nope)s")

    def test_load_of_config_without_logging_section_probes_nothing(self, tmp_path):
        """产品面口径：没写这节的机器，一次加载一条探针记录都不该有

        先暖一次默认串 —— 这条断言说的是「稳态每次加载」，不是「本进程第一条」。
        与代价实测（Temp `l57/rec.py`）暖 50 次再数的做法同一口径。
        """
        logging_setup.assert_format_renderable(LoggingConfig().format)
        path = _write(tmp_path, "cfg.yaml", "models:\n  default: ernie\n")
        with _count_probe_records() as seen:
            load_config(path)
        assert seen == []

    def test_repeated_identical_load_hits_cache_and_probes_nothing(self, tmp_path):
        """A106 落地后：写出 `format` 的第二次相同加载命中装配缓存，探针归 0。

        L57 曾把「稳态每次加载 1 条探针」钉成规格，并在注释里明写这条是给 A106 的
        tripwire —— 装配一旦改成条件重装，第二次加载就不会再花这条探针。L65 正是
        实现 A106 的那一轮，于是把期望从「恒为 1」翻成「命中即 0」。这条翻面本身
        就是 A106 生效的行为证据；「缓存不是恒命中」由下一条守着，两条合起来才有牙。
        """
        fmt = self._unique_valid_format()
        path = _write(tmp_path, "cfg.yaml",
                     "models:\n  default: ernie\nlogging:\n  format: %s\n" % fmt)
        load_config(path)  # 首次：完整装配并写下缓存签名
        with _count_probe_records() as seen:
            load_config(path)  # 再次：同配置 + 同拓扑 ⇒ 命中缓存，一条不探
        assert seen == [], "A106：写出 format 的第二次相同加载应命中缓存、零探针"

    def test_changed_format_breaks_assembly_cache(self, tmp_path):
        """换 `format` 必须打破缓存、重新装配（证明上一条的 0 不是「恒命中」凑出来的）"""
        path_a = _write(tmp_path, "a.yaml", "models:\n  default: ernie\nlogging:\n  format: %s\n"
                        % self._unique_valid_format())
        load_config(path_a)  # 暖：写下 A 的签名
        path_b = _write(tmp_path, "b.yaml", "models:\n  default: ernie\nlogging:\n  format: %s\n"
                        % self._unique_valid_format())
        with _count_probe_records() as seen:
            load_config(path_b)  # 换了 format ⇒ 签名打破 ⇒ 重新装配并探
        assert len(seen) >= 1, "改 format 后缓存必须失效、重新装配（否则上一条的 0 无判别力）"



class TestLoggingConfigJudgment:
    """dataclass 那一半：不经 `validate-config` 的 SDK 直构同样要认这套判据"""

    def test_defaults_are_todays_shape_not_a_wished_shape(self):
        """三档默认值 = `logging.lastResort` 今天做的事。这一条同时钉住两边：
        一边漂了（改了默认值或 stdlib 换了兜底），装配就会在「没写这节」的用户
        身上造成可见变化 —— 那是本轮最大的破坏面，必须由用例拦住。"""
        cfg = LoggingConfig()
        assert (cfg.level, cfg.file, cfg.format) == ("WARNING", "", "%(message)s")
        assert logging.lastResort.level == logging.WARNING
        assert logging.Formatter()._fmt == "%(message)s"

    @pytest.mark.parametrize("kwargs", [
        {"level": "INFORMATION"},
        {"level": None},
        {"level": ""},
        {"format": ""},
        {"format": "%(nope)s"},
        {"format": None},
        {"file": None},
        {"file": 1},
        {"file": True},
    ])
    def test_bad_knob_values_are_refused_at_construction(self, kwargs):
        base = {"level": "WARNING", "file": "", "format": "%(message)s"}
        base.update(kwargs)
        with pytest.raises(DataValidationError):
            LoggingConfig(**base)

    def test_empty_file_is_a_legal_value_meaning_no_file(self):
        """`file: ""` 与 `file: null` 不是一回事：前者是「明确不落盘」，后者是
        「写了键没给值」。混为一谈就会让人没法在写出 `file` 键的同时关掉文件。"""
        assert LoggingConfig(level="WARNING", file="", format="%(message)s").file == ""


class TestApplyOnlyWritesKeys:
    """装配的键粒度：写了哪几键就动哪几键"""

    @pytest.fixture()
    def cfg(self):
        return LoggingConfig()

    def test_written_none_means_all_three_keys(self, cfg):
        cfg.level, cfg.file, cfg.format = "DEBUG", "", "%(levelname)s|%(message)s"
        with _empty_root():
            summary = apply_logging_config(cfg)
            assert summary["level"] == logging.DEBUG
            assert [h.level for h in _console()] == [0]
            assert summary["handlers"] == len(logging.getLogger().handlers)

    def test_level_key_moves_the_root_level(self, cfg):
        cfg.level = "ERROR"
        before = logging.getLogger().level
        apply_logging_config(cfg, written={"level"})
        assert logging.getLogger().level == logging.ERROR != before

    def test_unwritten_level_leaves_the_root_level_alone(self, cfg):
        """API 面的今天形状靠这条守住：`basicConfig` 给的 INFO 不能被 CLI 式默认档
        压回 WARNING，否则「只想加个日志文件」会顺手关掉服务的全部 INFO 日志。"""
        logging.getLogger().setLevel(logging.INFO)
        cfg.level = "WARNING"
        apply_logging_config(cfg, written={"file"})
        assert logging.getLogger().level == logging.INFO

    def test_unwritten_format_leaves_existing_formatter_alone(self, cfg):
        keeper = logging.StreamHandler(io.StringIO())
        keeper.setFormatter(logging.Formatter("KEEP|%(message)s"))
        logging.getLogger().addHandler(keeper)
        cfg.format = "%(name)s"
        apply_logging_config(cfg, written={"level"})
        assert keeper.formatter._fmt == "KEEP|%(message)s"

    def test_written_format_replaces_formatter_on_existing_handlers(self, cfg):
        keeper = logging.StreamHandler(io.StringIO())
        keeper.setFormatter(logging.Formatter("KEEP|%(message)s"))
        logging.getLogger().addHandler(keeper)
        cfg.format = "%(levelname)s|%(message)s"
        apply_logging_config(cfg, written={"format"})
        assert keeper.formatter._fmt == "%(levelname)s|%(message)s"


class TestApplyHandlers:
    """handler 的装与撤：不双打、不叠装、不锁文件"""

    def test_is_repeatable_so_reloading_config_does_not_stack(self):
        """`load_config` 会被反复调用（API 每次读盘），不撤旧的 ⇒ 每调一次多一个
        handler，日志按次数翻倍。"""
        cfg = LoggingConfig(level="INFO", file="", format="%(message)s")
        with _empty_root():
            apply_logging_config(cfg, written={"level"})
            first = len(_console())
            for _ in range(3):
                apply_logging_config(cfg, written=LOGGING_KEYS)
            assert len(_console()) == first == 1

    def test_does_not_add_a_second_handler_when_the_app_already_has_one(self):
        """`api/main.py` 已经 `basicConfig` 过 ⇒ 再装一条就是每条日志打两遍
        （实测 P6 的 `A|…` + `B|…` 双行）。"""
        existing = logging.StreamHandler(io.StringIO())
        logging.getLogger().addHandler(existing)
        summary = apply_logging_config(LoggingConfig(), written={"level"})
        assert _console() == [existing]
        assert logging_setup._INSTALLED == []
        # 摘出来的 `handlers` 是 root 的真实条数，不是「本模块装了几条」
        assert summary["handlers"] == len(logging.getLogger().handlers)

    def test_file_key_writes_the_line_and_console_keeps_it(self, tmp_path):
        """只写 `file` 一键：控制台**不能**少字。root 一旦有了 handler，
        `logging.lastResort` 就不再兜底 —— 不自己补一条 stderr handler，这条
        WARNING 只会进文件，CLI 的 stderr 当场变空。

        这里刻意用 `_empty_root()` 复现 CLI 进程的前提（pytest 自己挂着一条
        `LogCaptureHandler`，不清空就走不到「root 无 handler」那一支）。断言只做
        结构判定：控制台那条 handler 在不在、是不是指向 stderr；真正的字节等价由
        集成面的真子进程用例去量。
        """
        target = tmp_path / "app.log"
        cfg = LoggingConfig(level="WARNING", file=str(target), format="%(message)s")
        with _empty_root():
            summary = apply_logging_config(cfg, written={"file"})
            logging.getLogger("probe").warning("line-to-both")
            assert len(_console()) == 1, "file 键把控制台那条日志吃掉了"
            assert _console()[0].stream is sys.stderr
            assert summary["file"] == os.path.abspath(str(target))
        assert "line-to-both" in target.read_text(encoding="utf-8")

    def test_file_key_can_be_turned_off_again(self, tmp_path):
        first, second = tmp_path / "a.log", tmp_path / "b.log"
        cfg = LoggingConfig(level="WARNING", file=str(first), format="%(message)s")
        apply_logging_config(cfg, written={"file"})
        logging.getLogger("probe").warning("into-a")
        cfg.file = str(second)
        apply_logging_config(cfg, written={"file"})
        logging.getLogger("probe").warning("into-b")
        text_a = first.read_text(encoding="utf-8")
        assert "into-a" in text_a and "into-b" not in text_a
        assert len(_files()) == 1, "换路径时必须撤掉旧的文件 handler，否则旧文件被双写"

    def test_empty_file_after_a_real_one_releases_the_handler(self, tmp_path):
        target = tmp_path / "c.log"
        cfg = LoggingConfig(level="WARNING", file=str(target), format="%(message)s")
        apply_logging_config(cfg, written={"file"})
        logging.getLogger("probe").warning("first")
        cfg.file = ""
        apply_logging_config(cfg, written={"file"})
        logging.getLogger("probe").warning("second")
        assert "second" not in target.read_text(encoding="utf-8")
        assert _files() == []

    def test_unopenable_path_is_said_actionably(self, tmp_path):
        """Windows 上 `FileHandler` 的原始错是 `[Errno 2] No such file or directory`，
        看不出「父目录不存在」与「没有相对路径基准」这两件事，所以补文案。"""
        cfg = LoggingConfig(file=str(tmp_path / "nope" / "x.log"))
        with pytest.raises(DataValidationError) as got:
            apply_logging_config(cfg, written={"file"})
        assert "父目录" in str(got.value)


class TestSurfaceRatchets:
    """三面对同一批键的判断必须彼此一致（承 L50 / L51 的「名单不许手抄」）"""

    def test_logging_keys_equal_the_dataclass_fields(self):
        declared = {f.name for f in dataclasses.fields(LoggingConfig)}
        assert set(LOGGING_KEYS) == declared, sorted(LOGGING_KEYS ^ declared)

    def test_validator_specs_cover_exactly_the_dataclass_fields(self):
        declared = {f.name for f in dataclasses.fields(LoggingConfig)}
        specced = {p.split(".", 1)[1] for p in ConfigValidator.KNOWN_FIELDS
                   if p.startswith("logging.") and "." in p}
        assert declared == specced, sorted(declared ^ specced)

    @pytest.mark.parametrize("name", LOGGING_LEVELS)
    def test_every_allowed_name_is_a_real_stdlib_level(self, name):
        assert name in logging._nameToLevel


class TestBothSidesJudgeTheSameValues:
    """`validate-config` 与 `load_config` 不许对同一批值各判一套（A77 的镜像）"""

    @pytest.mark.parametrize("key,value", [
        ("level", "INFORMATION"),
        ("level", "warning"),
        ("level", 30),
        ("level", None),
        ("file", 1),
        ("file", None),
        ("format", ""),
        ("format", None),
        ("format", "%(nope)s"),
        ("format", 2),
    ])
    def test_illegal_value_is_red_on_both_sides(self, tmp_path, key, value):
        raw = {"logging": {key: value}}
        paths = [e.path for e in validate_config(raw).errors]
        assert f"logging.{key}" in paths, (key, value, paths)
        with pytest.raises(ValueError):
            load_config(_write(tmp_path, "bad.yaml", yaml.safe_dump(raw)))

    @pytest.mark.parametrize("key,value", [
        ("level", "INFO"),
        ("level", "NOTSET"),
        ("file", ""),
        ("file", "x.log"),
        ("format", "%(message)s"),
        ("format", "%(asctime)s %(levelname)s %(name)s %(message)s"),
    ])
    def test_legal_value_is_green_on_both_sides(self, tmp_path, key, value):
        raw = {"logging": {key: value}}
        paths = [e.path for e in validate_config(raw).errors]
        assert f"logging.{key}" not in paths, (key, value, paths)
        loaded = load_config(_write(tmp_path, "ok.yaml", yaml.safe_dump(raw)))
        assert getattr(loaded.logging, key) == value


class TestSectionShapes:
    """A101：节写成 null / 标量，两档都要有人话结果"""

    def test_null_and_scalar_rules_hold_for_every_derived_section(self, tmp_path):
        """判据对**全部**要过 `_load_section` 的节成立，名单从字段类型推导

        A101 的实测结论是「每一节同形」（Temp `l57/probe1.txt` P4/P5），所以只测
        `logging` 一处就是假结案：下一节新增时会带着同一个崩法活下来。名单口径
        沿用配置面那条「白名单 = `load_config` 实际读走的键集」（`docs/API.md`），
        这里等价为「`AppConfig` 里字段类型是 dataclass 的那些节」。
        """
        sections = [(f.name, f.default_factory) for f in dataclasses.fields(AppConfig)
                    if f.default_factory is not dataclasses.MISSING
                    and dataclasses.is_dataclass(f.default_factory)]
        assert len(sections) >= 20, f"只推导出 {len(sections)} 个节 ⇒ 名单塌了"
        for name, cls in sections:
            obj = _load_section({name: None}, name, cls, {})
            assert obj == cls(), f"{name} 节写成 null 时不是全默认"
            with pytest.raises(ConfigError):
                load_config(_write(tmp_path, f"{name}.yaml", f"{name}: 8080\n"))

    @pytest.mark.parametrize("section,cls", [
        ("logging", LoggingConfig),
        ("web", WebConfig),
        ("quality", QualityConfig),
        ("dedup", DedupConfig),
    ])
    def test_bare_section_name_is_all_defaults_for_that_section(self, section, cls):
        """`logging:`（下面什么都没有）在 YAML 里是 `None` ⇒ 与 A94 的 0 字节文件
        同解：该节全默认。实测改前四节**全部**抛 `AttributeError`（Temp
        `l57/probe1.txt` P4/P5），所以这条按节名参数化，防止修了一处漏三处。"""
        obj = _load_section({section: None}, section, cls, {})
        assert isinstance(obj, cls)
        assert obj == cls()

    @pytest.mark.parametrize("body", ["logging:\n", "logging: {}\n"])
    def test_bare_logging_section_does_not_wire_anything(self, tmp_path, body):
        """「节名写了但没给任何键」有两种形状，两种都不许装配：`logging:` 在 YAML 里
        是 `None`（根本不进装配），`logging: {}` 是空映射（进装配但写出键集为空）。
        后者正是本模块 `if keys and not root.handlers` 那一支存在的理由 —— 少了
        `keys` 这一半，空映射就会在 CLI 进程里凭空装出一条 handler，而用户什么都没
        要求。这条钉的是「激活条件」而不是「装配结果」，写反了就等于把默认值当成用
        户指令。"""
        path = _write(tmp_path, "bare.yaml", body)
        with _empty_root():
            config = load_config(path)
            assert (config.logging.level, config.logging.file,
                    config.logging.format) == ("WARNING", "", "%(message)s")
            assert logging_setup._INSTALLED == []
            assert _console() == []

    def test_section_written_as_scalar_says_what_shape_was_expected(self, tmp_path):
        path = _write(tmp_path, "scalar.yaml", "logging: app.log\n")
        with pytest.raises(ConfigError) as got:
            load_config(path)
        assert "映射" in str(got.value)
        assert "app.log" in str(got.value)

    def test_same_rule_for_a_non_logging_section(self, tmp_path):
        path = _write(tmp_path, "webscalar.yaml", "web: 8080\n")
        with pytest.raises(ConfigError):
            load_config(path)


class TestSaveConfigDoesNotAuthorTheSection:
    """`save_config` 不代笔 `logging` 节，否则「没写」会被悄悄变成「写了」"""

    def test_saving_a_default_config_does_not_create_the_section(self, tmp_path):
        path = str(tmp_path / "out.yaml")
        save_config(AppConfig(), path)
        assert "logging" not in yaml.safe_load(io.open(path, encoding="utf-8").read())

    def test_an_existing_section_survives_a_save_verbatim(self, tmp_path):
        path = _write(tmp_path, "rt.yaml",
                      "models:\n  default: ernie\nlogging:\n  level: INFO\n"
                      "  file: mine.log\n  format: \"%(message)s\"\n")
        save_config(load_config(path), path)
        saved = yaml.safe_load(io.open(path, encoding="utf-8").read())
        assert saved["logging"] == {"level": "INFO", "file": "mine.log",
                                    "format": "%(message)s"}

    def test_round_trip_through_save_does_not_wire_anything_new(self, tmp_path):
        path = _write(tmp_path, "rt2.yaml",
                      "models:\n  default: ernie\naugmentation:\n  num_threads: 4\n")
        save_config(load_config(path), path)
        assert "logging" not in yaml.safe_load(io.open(path, encoding="utf-8").read())
        with _empty_root():
            load_config(path)
            assert logging_setup._INSTALLED == []
            assert _console() == []


class TestUnreadKeyWarningIsMutable:
    """A97 与 A84 同屏：反馈通道建好之后，旋钮也得真的能管住它"""

    TYPO_YAML = "augmentation:\n  variants_per_seeds: 5\n"

    def _records(self, caplog):
        return [r.getMessage() for r in caplog.records if "没人读取" in r.getMessage()]

    def test_without_a_logging_section_the_warning_is_said(self, tmp_path, caplog):
        load_config(_write(tmp_path, "t.yaml", self.TYPO_YAML))
        assert self._records(caplog) != []

    def test_level_error_silences_it_and_still_loads(self, tmp_path, caplog):
        """`level: ERROR` 是本轮才存在的静音旋钮，且排在 `_log_unread_keys` 之前，
        所以对**同一次**加载就生效（装配若排在出声之后，就得再跑一次才静音）。

        这里刻意**不用** `caplog.at_level`：它会把 `augmentor.config` 这个 logger
        自身的级别设成传入值，从而盖掉 root 的 ERROR —— 静音机制走的是
        `logger.isEnabledFor` 看有效级别，被夹具一改就测不到那条支路了（实测红在
        「记录仍然进了 caplog」）。上面那条无节的用例是正例对照，两条形不成空断言。
        """
        path = _write(tmp_path, "t2.yaml", self.TYPO_YAML + "logging:\n  level: ERROR\n")
        config = load_config(path)
        assert config.logging.level == "ERROR"
        assert logging.getLogger().level == logging.ERROR
        assert self._records(caplog) == []
