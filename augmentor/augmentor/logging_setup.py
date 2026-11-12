# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""`logging` 节的生效面（A97）

`config.py` 早就把 `level` / `file` / `format` 三键读进 `LoggingConfig`，但从 L54
到本轮**没有任何应用方** —— 旋钮存在，指针不接任何负载。本模块是那根指针。

## 口径：写了才动手

`load_config` 只在配置文件里**真的出现 `logging` 节**时调用
`apply_logging_config`，且只对**节里真的写出的键**动手。两条都是实测拍的：

- 默认值必须等于今天的形状。出厂状态下 CLI 进程 root logger 一个 handler 都没有，
  日志走 `logging.lastResort`：它在 WARNING 档把 `%(message)s` 裸打进 stderr。
  实测（Temp `l57/probe1.txt` NONCE-L57-PROBE1）root 装上 `StreamHandler` +
  `Formatter("%(message)s")` + `setLevel(WARNING)` 之后，同一条 WARNING 的 stderr
  **逐字节不变** —— 所以 `LoggingConfig` 的默认值改成这三档，不是一次行为变更，
  只是把「默认」写成「今天实际发生的事」。
- 不做「按默认值覆盖」会改掉 API 的日志形状：`api/main.py` 自己
  `basicConfig(level=INFO, format="…%(name)s…")`，而 CLI 那一侧的默认档是
  WARNING + 裸消息。同一个默认值不可能同时等于两个面，所以没写出的键一律不动。

## 幂等

`load_config` 会被重复调用（API 的 `/api/config` 每次都读盘），所以本模块记住自己
装过的 handler，下次先撤再装 —— 不撤就是每调一次多一个 handler，日志跟着翻倍。

自 A106 起「先撤再装」是**条件性**的：写出的键 + 三值 + root 的 handler 拓扑与上次
完整装配后一致时直接返回缓存摘要，不再撤装（省掉每请求关掉/重开一次日志文件，也消掉
撤与装之间 root 无 handler 的丢日志窗口）；任一项变了、或有外部 handler 挂进/摘出 root，
签名就打破、回到完整装配。`_detach_installed`（含测试收尾）会一并清掉这份缓存签名。
"""

import logging
import logging.handlers
import os
import sys
import threading
from typing import Any, Dict, Iterable, List, Optional, Tuple

from .exceptions import DataValidationError

# 允许集就这一份：`config_validator` 的 `choices` 规格与运行时的判据共用它，
# 「校验器绿 / 运行时红」的缝隙就是这么被封住的（承 L51 的两侧同判）。
# 只收大写名（`logging.setLevel` 对小写是 `ValueError: Unknown level`，实测见
# Temp `l57/probe2.txt` P4），因为校验器那一侧表达不了「大小写不敏感的允许集」，
# 两边各判一套比两边都严一点更糟。
LOGGING_LEVELS = ("CRITICAL", "ERROR", "WARNING", "INFO", "DEBUG", "NOTSET")

#: 本模块会装配的键（= `LoggingConfig` 的字段集，`tests/` 里有棘轮守着这个等式）
LOGGING_KEYS = ("level", "file", "format")

# 本模块装进 root logger 的 handler。撤装只撤自己装的那些，
# 不碰 `api/main.py` 之类由应用自己装的 handler。
_INSTALLED: List[logging.Handler] = []

# 上一次「完整装配」的 (签名, 摘要)（A106）。签名命中就原样返回缓存摘要、
# 不撤不装：`load_config` 在 API 的 `/api/config` 上是**每请求一次**，无条件重装配
# 会让每个请求都关掉再重开一次日志文件（实测约 103 µs/请求，Temp `l65/out.txt`），
# 且撤与装之间 root 短暂无 handler，并发请求可能正好落在那一瞬丢日志行。
# 签名带 root.handlers 拓扑元组 ⇒ 任何外部 handler 增减都会打破命中、回到完整装配。
_APPLIED: Optional[Tuple[Tuple[Any, ...], Dict[str, Any]]] = None

# 装配锁（A393 / L289）：上面那份缓存判的是「配置变没变」，判不了「有没有两支同时在装」。
# 撤与装这一整段必须一次只让一支进来，否则 handler 按到场人数翻倍、日志跟着翻倍。
# 只在同进程内有效；跨进程那一档与 A156 / A158 同口径。
_APPLY_LOCK = threading.Lock()


def level_number(value: Any) -> int:
    """把 `logging.level` 翻成数值；不在允许集内就出声拒收

    走 `getattr(logging, 名)` 而不是 `logging.getLevelName`：后者对未知字符串回
    的是 `"Level BOGUS"` 这种**字符串**，拿去 `setLevel` 才炸，炸点离错拼的键更远。
    """
    if isinstance(value, str) and value in LOGGING_LEVELS:
        return getattr(logging, value)
    raise DataValidationError(
        f"logging.level 必须是 {'/'.join(LOGGING_LEVELS)} 之一（大写），"
        f"当前是 {value!r}（{type(value).__name__}）"
    )


#: 已经对着真 record 试渲染成功过的格式串。只由 `build_formatter` 在**通过之后**写入，
#: 所以坏的串永远不会进来 —— 判据一次都不省，省的是「同一个串在同一进程里重复探」。
#: 串只来自本地配置文件与 SDK 直构，一个进程里的种类有界，不设淘汰。
_RENDER_OK = set()


def build_formatter(fmt: str) -> logging.Formatter:
    """造一个「对着真 record 打得开、且消毒控制字符」的 Formatter，打不开就出声

    非空与类型由 `validation.require_string` 判，这里补的是**渲染**这一维：
    `Formatter("%(nope)s")` 构造期不报错，到发第一条日志才抛，而 `Handler.emit`
    会把那次抛错变成 `--- Logging error ---` + 31 行 traceback 打在 stderr 上，
    并且**每条日志一行**（实测 Temp `l57/probe2.txt` P2/P3，程序不崩、日志全废）。
    一个拼错的 `format` 在接线之前是无害的，接线之后就是这台机器 —— 所以判在加载。
    """
    probe = logging.LogRecord(
        "augmentor.logging_setup", logging.WARNING, __file__, 0, "探针", (), None
    )
    try:
        formatter = ControlSafeFormatter(fmt)
        formatter.format(probe)
    except Exception as exc:
        raise DataValidationError(
            f"logging.format 不是可用的日志格式串 {fmt!r}：{exc}"
        ) from exc
    _RENDER_OK.add(fmt)
    return formatter


def assert_format_renderable(fmt: str) -> None:
    """只做渲染判据，不返回 Formatter —— 给「每次构造都要判、但用不到对象」的那一侧

    `LoggingConfig.__post_init__` 是这条：一次 `load_config` 会把 `LoggingConfig`
    构造两遍（`AppConfig()` 的 default_factory 一遍，`_load_section` 一遍），
    而配置里**没有** `logging` 节时这两遍都在为一个谁也没写的默认值做真 record
    试渲染。实测（Temp `l57/cost.txt` NONCE-L57-COST，三轮同号）那样一次加载
    多付 +30~+37 µs，其中归因到探针自身的是 `LogRecord.__init__` 两条
    （Temp `l57/rec.txt` NONCE-L57-REC：单条构造 2.2~2.3 µs）。
    """
    if fmt in _RENDER_OK:
        return
    build_formatter(fmt)


def _detach_installed(root: logging.Logger) -> None:
    """撤掉本模块上次装的 handler（并关闭文件 handler 放掉句柄）

    Windows 上不 `close()` 就锁着文件，测试里的 `tmp_path` 清理会当场失败。
    撤装即「root 上不再挂着本模块的东西」，A106 的命中签名随之失效，一并清掉，
    否则残留签名会让下一次相同配置的调用误命中一个已不存在的装配态。
    """
    global _APPLIED
    _APPLIED = None
    for handler in _INSTALLED:
        if handler in root.handlers:
            root.removeHandler(handler)
        handler.close()
    _INSTALLED.clear()


# `logging.file` 的封顶档：单文件到 `_LOG_FILE_MAX_BYTES` 就滚到 `.1`、`.2`…，最多
# 留 `_LOG_FILE_BACKUP_COUNT` 份备份（连主文件总量约 20 MiB）。改前这里用的是裸
# `logging.FileHandler`——无上限、无轮转，`augmenter serve`/API 这类长驻进程把
# INFO/DEBUG 落盘会一直写到耗尽磁盘（A104）。`mode` 仍是默认的 `a`（追加），所以
# 「重启不清零」的既有语义一字未动，只是给它加了个界。
_LOG_FILE_MAX_BYTES = 5 * 1024 * 1024
_LOG_FILE_BACKUP_COUNT = 3


def _open_file_handler(path: str) -> logging.handlers.RotatingFileHandler:
    try:
        return logging.handlers.RotatingFileHandler(
            path,
            maxBytes=_LOG_FILE_MAX_BYTES,
            backupCount=_LOG_FILE_BACKUP_COUNT,
            encoding="utf-8",
        )
    except (OSError, ValueError) as exc:
        # ValueError 也要接住：Windows 上 `"a\x00b.log"` 这类含 NUL 的路径由
        # `os.path` 抛 `ValueError: embedded null character`，是 `logging.file`
        # 写错的一种，漏在外面就变成裸异常（API 500 / CLI 一坨栈）。
        raise DataValidationError(
            f"logging.file 打不开 {path!r}：{exc}（相对路径按当前工作目录 "
            f"{os.getcwd()!r} 解释，父目录需要已存在）"
        ) from exc


def apply_logging_config(settings: Any, written: Optional[Iterable[str]] = None) -> Dict[str, Any]:
    """按 `logging` 节装配 root logger，返回实际生效的摘要

    本函数是**序列化**的（A393 / L289）。A106 那条快路是 check-then-act：签名不命中就
    「先撤再装」。撤与装之间没有任何互斥 ⇒ 两支并发各装一套，root 上挂着的 handler 按
    到场人数翻倍，而本模块 docstring 早就把这一族写成了「不撤就是每调一次多一个 handler，
    日志跟着翻倍」—— 单发不翻了，并发又翻回来。实测（16 支并发 `load_config`，同进程同
    root）：handler 从 2 涨到 **7 / 14 / 11**（三跑），一条 INFO 在日志文件里变成
    **6 / 13 / 10** 行；同一份配置单发两次是 2 / 1。快路只挡住了「同配置重复装配」，
    挡不住「同时装配」，所以这里加一把进程内的装配锁。

    Args:
        settings: 带 `level` / `file` / `format` 三属性的对象（`LoggingConfig`；
            刻意不做类型导入 —— 本模块由 `config` 加载，反向 import 会成环）
        written: 配置里真的写出的键。为 `None` 时按「三键全写出」处理，那是给
            SDK 直接调用留的口径；`load_config` 一律显式传它，因为「没写 = 不动」
            正是本模块的生效条件。

    Returns:
        ``{"level": 本轮施加的级别（没写 level 键则 None）, "handlers": root
        handler 条数, "file": 落盘绝对路径或空串}`` —— 给用例断言用，不参与任何
        对外契约。
    """
    with _APPLY_LOCK:
        return _apply_logging_config_locked(settings, written)


def _apply_logging_config_locked(settings: Any, written: Optional[Iterable[str]]) -> Dict[str, Any]:
    """真正的装配体，只在 `_APPLY_LOCK` 之内跑（本函数体与加锁前逐字相同）"""
    keys = set(LOGGING_KEYS) if written is None else set(written)
    root = logging.getLogger()
    global _APPLIED

    # A106：写出的键 + 三值 + root.handlers 拓扑与上次完整装配后完全一致 ⇒ 什么都不做，
    # 直接回缓存摘要。API 的 `/api/config` 每请求读盘 → 每请求一次本调用，配置不变时
    # 这条快路省掉「关掉再重开日志文件」，也消掉撤/装之间 root 无 handler 的丢日志窗口。
    signature = (
        frozenset(keys),
        settings.level,
        settings.file,
        settings.format,
        tuple(root.handlers),
    )
    if _APPLIED is not None and _APPLIED[0] == signature:
        return dict(_APPLIED[1])

    _detach_installed(root)

    level = None
    if "level" in keys:
        level = level_number(settings.level)
        root.setLevel(level)

    formatter = build_formatter(settings.format) if "format" in keys else None
    if keys and not root.handlers:
        # 与 `logging.lastResort` 同形：stderr + `%(message)s` + WARNING 档。
        # root 已经有 handler（API 进程）就不再装第二个，否则每条日志打两遍
        # （实测 Temp `l57/probe2.txt` P6：装第二个之后同一条 WARNING 出两行）。
        # `keys` 为空（`logging:` 写了节名却没给键）时一条都不装：那是「没提任何
        # 要求」，而本模块的口径是「写了才动手」。
        handler = logging.StreamHandler(sys.stderr)
        root.addHandler(handler)
        _INSTALLED.append(handler)
    if formatter is not None:
        for handler in root.handlers:
            handler.setFormatter(formatter)

    file_path = ""
    if "file" in keys:
        if settings.file:
            file_handler = _open_file_handler(settings.file)
            if formatter is not None:
                file_handler.setFormatter(formatter)
            root.addHandler(file_handler)
            _INSTALLED.append(file_handler)
            file_path = os.path.abspath(settings.file)

    summary = {
        "level": level,
        "handlers": len(root.handlers),
        "file": file_path,
    }
    # 记的是「完整装配之后」的拓扑（`_detach_installed` 已把 `_APPLIED` 清 None）：
    # 下一次调用进来时 root.handlers 正是这一份，相同配置即可命中。
    _APPLIED = (signature[:4] + (tuple(root.handlers),), dict(summary))
    return summary


# ============ 日志注入消毒（L285，CWE-117）============
#
# 新代码一律落在本模块**尾部**，`import re` 也在这一格：文件头的 import 块加一行会把
# 账本对本模块的四处活引用整体顶下一行（L285 记账时是 95 / 122 / 166 / 206，L289 把装配
# 体拆进锁壳后顶成 101 / 128 / 172 / 226，逐格按 a184 与 l79 的红读数重钉）⇒ 尾部只追加。
import re  # noqa: E402  （刻意不在文件头，理由见上）

#: 会改写「一条记录恒等于一行」这个物理契约、或会驱动终端重绘的字符。
#: `\x09`（Tab）**不在**这一集里：它既不起新行也不起转义序列，钉掉只会把对齐的表式日志打散。
_CONTROL_CHARS = re.compile(r"[\x00-\x08\x0a-\x1f\x7f-\x9f]")


def escape_control(text: str) -> str:
    r"""把控制字符写成 `\xNN` 两串字面量，可逆且不吞掉任何一码点

    先 `search` 再 `sub`：绝大多数日志行是干净的，让干净行**逐字节原样**返回（实测
    `identity_when_clean` 为真），代价只在一次 C 档扫描上。
    """
    if not _CONTROL_CHARS.search(text):
        return text
    return _CONTROL_CHARS.sub(lambda m: "\\x%02x" % ord(m.group()), text)


class ControlSafeFormatter(logging.Formatter):
    r"""消毒只加在渲染的最后一跳，不改格式串的语义

    为什么落在汇（sink）而不是 170 处调用点：一条 record 要经过 formatter 才变成「一行」，
    所以 `logging.format` 写成什么串都覆盖得到，而调用点逐个消毒既要改 170 处又必然漏。
    消毒覆写**两跳**（L320 / A388）：`formatMessage` 管消息本体，`formatException` 管
    traceback 里那段**载荷**文本 —— 栈帧（`  File "..."` 与源码行）保持原样多行，那是
    可观测性的底线，而它们的行首由结构决定、攻击者写不进去；被消毒的是异常消息本身
    （`str(exc)`）、`__notes__` 与 `SyntaxError` 的 `msg`，它们的换行会变成 `\x0a`
    字面量 ⇒ 伪造行回到同一行内。
    """

    def formatMessage(self, record: logging.LogRecord) -> str:
        return escape_control(super().formatMessage(record))

    def formatException(self, ei) -> str:
        r"""traceback 的载荷段也消毒，栈帧的结构换行原样保留（L320 / A388）

        `Formatter.format()` 的形状是「`formatMessage` → 若带 `exc_info` 则把
        `formatException` 的结果**追加**在后面」。上一跳只覆盖前一半 ⇒ 异常消息里的
        载荷原样进流：实测 `ValueError("inner\n[2026-10-17 02:00:00] INFO ROOT admin
        login")` 让第二行看起来是一条**真的**日志（时间戳 / 级别 / logger 名全是
        攻击者写的），与 A387 同一族、只是换了触发点。

        改法只动**载荷**那一半：栈帧保持原样多行（可观测性的底线，且行首由结构决定），
        被消毒的是异常消息 / `__notes__` / `SyntaxError` 的 `msg`。因果链
        （`__cause__` / `__context__`）上的每一段照同一口径处理。

        Args:
            ei: `sys.exc_info()` 那三元组（类型 / 值 / traceback 对象）

        Returns:
            与 stdlib `Formatter.formatException` **逐字节相同**的栈文本，只是载荷段里
            的控制字符被写成 `\xNN` 字面量
        """
        # 与 stdlib 的 `print_exception` 同一取法：异常名取 `type(value)` 而不是 `ei[0]`
        # （`exc_info` 的类型位在「没有异常」时是 None，那时 stdlib 打的也是 NoneType）。
        value = ei[1]
        te = traceback.TracebackException(type(value), value, ei[2])
        _escape_traceback_payload(te)
        text = "".join(te.format())
        if text.endswith("\n"):
            text = text[:-1]
        return text


def build_handler(fmt: str) -> logging.StreamHandler:
    """stderr + 消毒 Formatter 的 handler —— 给 API 入口用

    `logging.basicConfig()` 在 3.12 不收 `formatter=`（实测 `ValueError: Unrecognised
    argument(s): formatter`），只认 `handlers=`，所以入口侧只能从这里递对象进去。
    """
    handler = logging.StreamHandler(sys.stderr)
    handler.setFormatter(ControlSafeFormatter(fmt))
    return handler


# ============ 栈载荷的消毒与 CLI 默认出口（L320，A388 / A387）============
#
# 仍落在本模块**尾部**（理由见上面那段注释）：文件头与中段插行会把账本对本模块的活引用
# 整体顶下一行。`import traceback` 同理——它只在本模块的函数体里按名取，模块级导入在
# 文件尾部执行时早已完成，运行期取得到。
import traceback  # noqa: E402  （刻意不在文件头，理由见上)


def _escape_traceback_payload(te) -> None:
    """把一条 traceback 链上「载荷」那一段的控制字符消毒（就地改写，L320 / A388）

    载荷 = 异常消息（`str(exc)`）、`__notes__` 与 `SyntaxError` 的 `msg`；**不含**栈帧，
    栈帧是结构（行首恒为 `  File "..."` / 源码行），保持原样多行。

    因果链只走**已经是 `TracebackException` 的那一支**（3.11+ 的形状）：3.10 那一版把
    `__cause__` / `__context__` 放在**异常对象**上而不是 `TracebackException` 上，改它就得
    改用户的异常对象（本轮不做）⇒ 那一档的链上载荷不消毒，链上消息仍可能劈行。

    Args:
        te: `traceback.TracebackException` 实例（就地改写它的消息字段）
    """
    te._str = escape_control(te._str)
    notes = getattr(te, "__notes__", None)
    if isinstance(notes, list):
        te.__notes__ = [
            escape_control(item) if isinstance(item, str) else item for item in notes
        ]
    msg = getattr(te, "msg", None)
    if isinstance(msg, str):
        te.msg = escape_control(msg)
    for attr in ("__cause__", "__context__"):
        nxt = getattr(te, attr, None)
        if isinstance(nxt, traceback.TracebackException):
            _escape_traceback_payload(nxt)


class _CliStderrHandler(logging.StreamHandler):
    """stderr handler，但 `stream` **每次现取**（与 stdlib 的 `_StderrHandler` 同形）

    `logging.StreamHandler(sys.stderr)` 在构造那一刻把流对象钉死 ⇒ 之后谁替换了
    `sys.stderr`（测试捕获、`contextlib.redirect_stderr`），日志就写到旧流上去了，
    观察者会以为「这条日志没出来」。这里改成属性现取，形状与 stdlib 那一支一致。

    `__init__` 刻意**不**调 `StreamHandler.__init__`（它会把 `self.stream = stream`
    写死在一个只读属性上）：只调 `logging.Handler.__init__`，与 stdlib 的
    `_StderrHandler` 同一手法。
    """

    def __init__(self, level: int = logging.NOTSET) -> None:
        logging.Handler.__init__(self, level)

    @property
    def stream(self):
        """当前那支 stderr（现取，不缓存）"""
        return sys.stderr


def install_default_cli_logging() -> Optional[logging.Handler]:
    """给纯 CLI 进程装一条与 `logging.lastResort` 同形、但**带消毒**的 handler（L320 / A387）

    **为什么需要它**：出厂配置不写 `logging` 节时，root logger 一条 handler 都没有 ⇒
    stdlib 走 `logging.lastResort` —— 它是 stdlib 内置的 handler，Formatter 不经我们
    的构造点 ⇒ **CLI 面的日志至今没有消毒**。实测（API 面与「装了 `logging` 节的 CLI
    面」都被 L285 那一支覆盖，缺的就是这一格）：载荷 `evil\n[2026] INFO ROOT admin
    login` 在没有 `logging` 节的 CLI 进程里落成**两支物理行**，第二行逐字是
    `[2026] INFO ROOT admin login` —— 一条凭空多出来的「日志」。

    **口径：与 `lastResort` 逐字节同形，只多一道消毒**。`lastResort` 是 stderr +
    `%(message)s` 裸消息 + WARNING 档 ⇒ 这一支照抄那三档，于是**干净载荷的输出逐字节
    不变**（既有那条「出厂默认档一字未改」的主张继续成立），脏载荷才被消毒。
    级别钉在 WARNING 是为了与 `lastResort` 一致：`logging.level: DEBUG` 那时 root 的
    级别被调低，但 `lastResort` 自己仍只放 WARNING ⇒ 控制台的可见档位一字不变。

    **只在 root 没有 handler 时装**：`lastResort` 本来就只在「root 一条 handler 都
    没有」时才兜底，已有出口时装第二条就是让同一条日志打两遍（`apply_logging_config`
    的 `and not root.handlers` 那一支防的正是这件事）。

    Returns:
        装上的那条 handler；root 已有 handler 时返回 `None`（没装，也没该装）
    """
    root = logging.getLogger()
    if root.handlers:
        return None
    handler = _CliStderrHandler(logging.WARNING)
    handler.setFormatter(ControlSafeFormatter("%(message)s"))
    root.addHandler(handler)
    return handler
