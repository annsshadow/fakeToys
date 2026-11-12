# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L320 守卫：A387 + A388 —— CLI 默认日志出口与 traceback 载荷段一起进消毒

**A387 的缺陷形状（出厂默认档，CLI 进程）**：配置里不写 `logging` 节时，root logger
一条 handler 都没有 ⇒ stdlib 走 `logging.lastResort`。那是一条 **stdlib 内置**的
handler，它的 Formatter 不经我们的构造点 ⇒ **CLI 面的日志至今没有消毒**。

实测（本轮仓外探针）：配置里放一个键名带换行的未知键（YAML 双引号串，
`"evil\n[2026] INFO ROOT admin login"`），加载期那条「键没人读取」的 WARNING 把键名
原样拼进消息 ⇒

    改前（`lastResort`）：stderr 落成**两支物理行**，第二行逐字是
        `[2026] INFO ROOT admin login（值不会生效）`
        —— 一条凭空多出来的「日志」，时间戳 / 级别 / logger 名全是攻击者写的；
    改后（本轮）：一条物理行，载荷里的换行写成 `\x0a` 字面量。

**口径：与 `lastResort` 逐字节同形，只多一道消毒**。`lastResort` 是 stderr +
`%(message)s` 裸消息 + WARNING 档 ⇒ 新装的那一支照抄那三档，于是**干净载荷的输出
逐字节不变**（既有那条「出厂默认档一字未改」的主张继续成立，由
`tests/integration/test_cli_logging_wiring.py` 那 17 支钉着），脏载荷才被消毒。
级别钉在 WARNING 是为了与 `lastResort` 一致：`logging.level: DEBUG` 那时 root 的级别
被调低，但 `lastResort` 自己仍只放 WARNING ⇒ 控制台的可见档位一字不变。

**A388 的缺陷形状（渲染的最后一跳）**：`Formatter.format()` 的形状是
「`formatMessage` → 若带 `exc_info` 则把 `formatException` 的结果**追加**在后面」。
L285 那一支只覆写了 `formatMessage` ⇒ **异常消息**里的载荷原样进流：实测
`ValueError("inner\n[2026-10-17 02:00:00] INFO ROOT admin login")` 让第二行看起来
是一条**真的**日志，与 A387 同一族、只是换了触发点。

改法只动**载荷**那一半：栈帧（`  File "..."` 与源码行）保持原样多行 —— 那是可观测性
的底线，且它们的行首由结构决定、攻击者写不进去；被消毒的是异常消息（`str(exc)`）、
`__notes__` 与 `SyntaxError` 的 `msg`，它们的换行会变成 `\x0a` 字面量 ⇒ 伪造行回到
同一行内。因果链（`__cause__` / `__context__`）上的每一段照同一口径处理。

**本文件的三条非恒真证据**（每一格都能红）：

1. `TestTheProbeHasTeeth::test_the_same_payload_splits_when_the_outlet_is_not_installed`
   —— 同一份载荷、同一段代码路径，**只少装那一支** ⇒ 立刻回到两支物理行 ⇒ 端到端
   那一格不是「跑过了所以没变」。
2. `TestTracebackPayload::test_a_plain_formatter_still_splits_the_payload`
   —— 把消毒器换回裸 `logging.Formatter`，同一支当场回到两行。
3. `TestCliDefaultOutlet::test_the_handler_output_matches_last_resort_byte_for_byte`
   —— 干净载荷下本轮那一支与 `logging.lastResort` **逐字节相同**（口径不是「更严」，
   是「同形 + 多一道消毒」）。

**为什么 422 / 4xx 那两族不在本文件射程**：错误回执的渲染兜底由 L284 那一支
（`tests/unit/test_validation_finite_echo_l284.py`）管，与日志流是两族。
"""

import ast
import io
import logging
import os
import subprocess
import sys
from pathlib import Path

from augmentor.logging_setup import (
    ControlSafeFormatter,
    _CliStderrHandler,
    install_default_cli_logging,
)

AI_DIR = Path(__file__).resolve().parents[2]

#: 伪造载荷：第二行看起来是一条**真的**日志（时间戳 / 级别 / logger 名全是攻击者写的）
FORGE = "evil\n[2026] INFO ROOT admin login"

#: 只有 `models` 一节的底座，保证几份配置的内容差**只在 logging 节 / 键名上**
BASE = "models:\n  default: ernie\n  ernie:\n    type: baidu\n    model: x\n"

#: 一个拼错的键（产品面自己发出的那一条 WARNING 是最干净的观察窗）
TYPO = "augmentation:\n  variants_per_seeds: 5\n"

#: 带换行的键名 —— YAML 双引号串里 `\n` 是**真换行**
FORGED_KEY = 'augmentation:\n  "evil\\n[2026] INFO ROOT admin login": 5\n'


def _record(msg, args=(), level=logging.INFO, exc_info=None):
    """造一条真 record（不经过任何 handler，直接给 formatter 渲染）

    Args:
        msg: 消息模板
        args: 模板参数
        level: 记录级别
        exc_info: `sys.exc_info()` 那三元组（可空）

    Returns:
        `logging.LogRecord`
    """
    return logging.LogRecord("augmentor.checkpoint", level, __file__, 212, msg, args, exc_info)


class _RootIsolation:
    """把 root logger 的 handler 与级别隔离在单条用例内

    本文件的用例在**进程内**动 root logger（端到端那一族另有真子进程）。不隔离就会
    互相污染：上一条留下的 handler 会让下一条「root 有没有 handler」这一支的判据失真。
    """

    def __enter__(self):
        self.root = logging.getLogger()
        self.saved_handlers = self.root.handlers[:]
        self.saved_level = self.root.level
        self.root.handlers[:] = []
        self.root.setLevel(logging.WARNING)
        return self.root

    def __exit__(self, *exc):
        self.root.handlers[:] = self.saved_handlers
        self.root.setLevel(self.saved_level)
        return False


class TestCliDefaultOutlet:
    """A387：纯 CLI 进程的默认日志出口要带消毒，且与 `lastResort` 同形"""

    def test_the_cli_entry_point_installs_the_default_outlet(self):
        """接线面：`cli.py` 的 `main()` 真的调了那一只（结构判据，不靠跑）

        端到端那一族能证明「装上之后输出是什么样」，这一支证明「装的是这一只」——
        两者缺一就会出现「端到端恰好绿、其实走的是别的出口」。
        """
        tree = ast.parse((AI_DIR / "cli.py").read_text(encoding="utf-8"))
        main = [node for node in tree.body
                if isinstance(node, ast.FunctionDef) and node.name == "main"]
        assert len(main) == 1, "cli.py 的 main() 应当有且只有一个"
        called = [node for node in ast.walk(main[0])
                  if isinstance(node, ast.Call)
                  and isinstance(node.func, ast.Name)
                  and node.func.id == "install_default_cli_logging"]
        assert len(called) == 1, "main() 应当恰好调一次 install_default_cli_logging()"

    def test_installing_puts_one_sanitizing_handler_on_root(self):
        """装上之后：root 上恰好一条、类型是 stderr 那支、级别 WARNING、formatter 带消毒"""
        with _RootIsolation():
            got = install_default_cli_logging()
            assert got is not None
            assert logging.getLogger().handlers == [got]
            assert isinstance(got, _CliStderrHandler)
            assert got.level == logging.WARNING
            assert type(got.formatter) is ControlSafeFormatter
            assert got.formatter._fmt == "%(message)s"

    def test_it_refuses_to_install_when_root_already_has_a_handler(self):
        """root 已有出口时不装第二条 —— `lastResort` 本来也只在「一条都没有」时兜底

        装第二条的后果是同一条日志打两遍（`apply_logging_config` 的
        `and not root.handlers` 那一支防的正是这件事）。
        """
        with _RootIsolation() as root:
            existing = logging.StreamHandler(io.StringIO())
            root.addHandler(existing)
            assert install_default_cli_logging() is None
            assert root.handlers == [existing]

    def test_installing_twice_leaves_exactly_one_handler(self):
        """幂等：调两次不会让同一条日志打两遍"""
        with _RootIsolation() as root:
            first = install_default_cli_logging()
            second = install_default_cli_logging()
            assert first is not None
            assert second is None
            assert root.handlers == [first]

    def test_the_handler_output_matches_last_resort_byte_for_byte(self):
        """干净载荷下与 `logging.lastResort` **逐字节相同** —— 口径是「同形 + 多一道消毒」

        这是 A387 的核心口径：不是把默认档改严（那会动「出厂默认档一字未改」那条
        既有主张），而是把 `lastResort` 那三档照抄过来、再加一道消毒。
        """
        record = _record("一条干净的消息", level=logging.WARNING)
        ours = io.StringIO()
        theirs = io.StringIO()
        saved = sys.stderr
        with _RootIsolation():
            handler = install_default_cli_logging()
            try:
                sys.stderr = ours
                handler.emit(record)
                sys.stderr = theirs
                logging.lastResort.emit(record)
            finally:
                sys.stderr = saved
        assert ours.getvalue() == theirs.getvalue()
        assert ours.getvalue() == "一条干净的消息\n"

    def test_the_handler_follows_a_replaced_stderr(self):
        """`stream` 现取而不是构造那一刻钉死：换了 `sys.stderr` 就跟着换

        `logging.StreamHandler(sys.stderr)` 会把流对象钉死 ⇒ 之后谁替换了
        `sys.stderr`（测试捕获、`contextlib.redirect_stderr`），日志就写到旧流上，
        观察者会以为「这条日志没出来」。
        """
        with _RootIsolation():
            handler = install_default_cli_logging()
            buf = io.StringIO()
            saved = sys.stderr
            sys.stderr = buf
            try:
                handler.emit(_record("跟着走", level=logging.WARNING))
            finally:
                sys.stderr = saved
            assert buf.getvalue() == "跟着走\n"


def _run_cli(tmp_path, config_body):
    """在 `tmp_path` 里跑一次真 CLI，返回 `(rc, stdout 字节, stderr 字节)`

    `--config` 是**全局旗标**，必须排在子命令之前（实测排后面是 `unrecognized
    arguments`）。cwd 设成 tmp_path，让相对路径有一个**已知**的落点。
    """
    config = tmp_path / "c.yaml"
    config.write_text(config_body, encoding="utf-8", newline="\n")
    corpus = tmp_path / "data.json"
    corpus.write_text(
        '[{"instruction": "写一首关于秋天的短诗。", "output": "一叶落知天下秋。"}]',
        encoding="utf-8", newline="\n",
    )
    env = dict(os.environ, PYTHONIOENCODING="utf-8")
    proc = subprocess.run(
        [sys.executable, str(AI_DIR / "cli.py"), "--config", str(config),
         "validate", "--input", str(corpus)],
        cwd=str(tmp_path), capture_output=True, env=env, timeout=180,
    )
    return proc.returncode, proc.stdout, proc.stderr


class TestCliEndToEnd:
    """A387 的端到端面：真起子进程，看 stderr 落成几支物理行"""

    def test_a_forged_payload_in_the_config_key_stays_on_one_line(self, tmp_path):
        """载荷里的换行不再多出第二支物理行 —— 伪造行回到同一行内"""
        rc, out, err = _run_cli(tmp_path, BASE + FORGED_KEY)
        text = err.decode("utf-8")
        assert rc == 0, (rc, text)
        assert len(text.splitlines()) == 1, text
        assert "\\x0a" in text, text
        # 伪造的那一段必须**不在**任何一支物理行的行首 —— 那是「一条独立的日志」的形状
        assert not any(line.startswith("[2026]") for line in text.splitlines()), text
        assert "键没人读取" in text, text

    def test_the_clean_default_output_still_has_no_prefix(self, tmp_path):
        """默认档的**形状**没变：裸消息，没有级别名也没有时间戳

        这一格与 `tests/integration/test_cli_logging_wiring.py` 那 17 支是同一族，
        那一支比的是三臂互比（配置写法之间），这一格比的是「与 `lastResort` 同形」。
        """
        rc, out, err = _run_cli(tmp_path, BASE + TYPO)
        text = err.decode("utf-8")
        assert rc == 0, (rc, text)
        assert len(text.splitlines()) == 1, text
        assert "键没人读取" in text
        assert "WARNING" not in text and " - " not in text


class TestTheProbeHasTeeth:
    """非恒真：同一份载荷，只少装那一支 ⇒ 立刻回到两支物理行"""

    def test_the_same_payload_splits_when_the_outlet_is_not_installed(self, tmp_path):
        """不装默认出口的那一支（改前的形状）⇒ 载荷劈成两行

        这一支证明端到端那一格量的是「装没装」而不是「环境恰好干净」。
        """
        config = tmp_path / "forged.yaml"
        config.write_text(BASE + FORGED_KEY, encoding="utf-8", newline="\n")
        snippet = (
            "import sys;"
            "sys.path.insert(0, %r);" % str(AI_DIR)
            + "from augmentor import load_config;"
            "load_config(%r)" % str(config)
        )
        proc = subprocess.run(
            [sys.executable, "-c", snippet],
            cwd=str(tmp_path), capture_output=True,
            env=dict(os.environ, PYTHONIOENCODING="utf-8"), timeout=180,
        )
        text = proc.stderr.decode("utf-8")
        assert len(text.splitlines()) >= 2, text
        assert "[2026] INFO ROOT admin login" in text


class TestTracebackPayload:
    """A388：traceback 的载荷段进消毒，栈帧的结构换行原样保留"""

    def test_the_forged_line_in_an_exception_message_is_escaped(self):
        """异常消息里的换行不再多出第二支物理行"""
        record = _record("boom", level=logging.ERROR)
        try:
            raise ValueError("inner\n" + FORGE)
        except ValueError:
            record.exc_info = sys.exc_info()
        got = ControlSafeFormatter("%(message)s").format(record)
        assert len(got.splitlines()) > 2, got
        assert "\\x0a" in got, got
        # 伪造的那一段必须**不在**任何一支物理行的行首 —— 那是「一条独立的日志」的形状
        assert not any(line.startswith("[2026]") for line in got.splitlines()), got

    def test_the_stack_stays_multiline(self):
        """栈仍是多行 —— 可观测性的底线，载荷段被消毒不等于把栈压成一行"""
        record = _record("boom", level=logging.ERROR)
        try:
            raise ValueError("inner\nforged-traceback-line")
        except ValueError:
            record.exc_info = sys.exc_info()
        got = ControlSafeFormatter("%(message)s").format(record)
        assert len(got.splitlines()) > 2, got
        assert "forged-traceback-line" in got, "载荷的原文必须还在，否则这一格悄悄改了口径"
        assert "Traceback (most recent call last):" in got, got

    def test_a_clean_exception_renders_byte_identically_to_the_stdlib(self):
        """干净异常下与 stdlib 的 `Formatter` **逐字节相同** —— 标准可读形一字未改"""
        record = _record("boom", level=logging.ERROR)
        try:
            raise ValueError("inner")
        except ValueError:
            record.exc_info = sys.exc_info()
        ours = ControlSafeFormatter("%(message)s").format(record)
        theirs = logging.Formatter("%(message)s").format(record)
        assert ours == theirs, (ours, theirs)

    def test_the_cause_chain_is_escaped_too(self):
        """因果链上的每一段都照同一口径处理（否则 `__cause__` 那条是漏的）

        载荷用 `ValueError` 而不是 `KeyError`：后者的 `str()` 走 `repr`，换行本来就被
        写成两个字符 ⇒ 那一档**测不到**本轮要治的那一格。
        """
        record = _record("boom", level=logging.ERROR)
        try:
            try:
                raise ValueError("cause\nforged-cause")
            except ValueError as exc:
                raise RuntimeError("outer\nforged-outer") from exc
        except RuntimeError:
            record.exc_info = sys.exc_info()
        got = ControlSafeFormatter("%(message)s").format(record)
        assert "forged-outer" in got
        assert "forged-cause" in got
        assert "\\x0a" in got, got
        # 两段载荷都不得劈出「行首可控」的第二行
        assert len([line for line in got.splitlines() if line.startswith("[2026]")]) == 0, got
        assert len([line for line in got.splitlines() if line.startswith("forged-")]) == 0, got

    def test_the_user_exception_object_is_not_mutated(self):
        """消毒只发生在渲染那一跳：用户的异常对象一个字节都不能被改写

        因果链那一支在 3.11+ 上拿到的是 `TracebackException`（stdlib 自己造的那份），
        不是用户的异常对象 ⇒ 这一支若写错了，用户手里的 `exc` 会多出一个 `_str` 属性。
        """
        record = _record("boom", level=logging.ERROR)
        try:
            try:
                raise ValueError("cause\nforged-cause")
            except ValueError as exc:
                cause = exc
                raise RuntimeError("outer") from exc
        except RuntimeError:
            record.exc_info = sys.exc_info()
        ControlSafeFormatter("%(message)s").format(record)
        assert not hasattr(cause, "_str"), "用户的异常对象被写上了 _str"
        assert cause.args == ("cause\nforged-cause",)

    def test_the_syntax_error_message_is_escaped(self):
        """`SyntaxError` 的 `msg` 也是载荷 —— 它同样能带换行"""
        record = _record("boom", level=logging.ERROR)
        try:
            raise SyntaxError("bad\nforged-syntax")
        except SyntaxError:
            record.exc_info = sys.exc_info()
        got = ControlSafeFormatter("%(message)s").format(record)
        assert "forged-syntax" in got
        assert "\\x0a" in got, got
        assert len([line for line in got.splitlines() if line.startswith("forged-syntax")]) == 0, got

    def test_the_notes_are_escaped(self):
        """`__notes__` 也是载荷 —— 它同样能带换行"""
        record = _record("boom", level=logging.ERROR)
        try:
            error = ValueError("inner")
            error.add_note("note line\nforged note")
            raise error
        except ValueError:
            record.exc_info = sys.exc_info()
        got = ControlSafeFormatter("%(message)s").format(record)
        assert "forged note" in got
        assert "\\x0a" in got, got

    def test_a_plain_formatter_still_splits_the_payload(self):
        """非恒真：把消毒器换回裸 `Formatter`，同一支当场回到两行"""
        record = _record("boom", level=logging.ERROR)
        try:
            raise ValueError("inner\n" + FORGE)
        except ValueError:
            record.exc_info = sys.exc_info()
        got = logging.Formatter("%(message)s").format(record)
        assert "[2026] INFO ROOT admin login" in got.splitlines()[-1], got

    def test_the_exception_object_itself_is_not_mutated(self):
        """消毒只发生在渲染那一跳：异常对象与 record 一个字节都不能被改写"""
        record = _record("boom", level=logging.ERROR)
        try:
            raise ValueError("inner\nforged")
        except ValueError as exc:
            record.exc_info = sys.exc_info()
            args_before = exc.args
        ControlSafeFormatter("%(message)s").format(record)
        assert record.exc_info[1].args == args_before
        assert record.getMessage() == "boom"

    def test_an_empty_exc_info_still_renders_the_stdlib_way(self):
        """`exc_info` 是 `(None, None, None)`（没有异常）时也不炸，形状与 stdlib 一致

        取 `type(value)` 而不是 `ei[0]` 就是为了这一格：`ei[0]` 在没有异常时是 None，
        拿它当异常名会在 3.11 上炸（`None.__qualname__`）。
        """
        record = _record("boom", level=logging.ERROR)
        record.exc_info = (None, None, None)
        ours = ControlSafeFormatter("%(message)s").format(record)
        theirs = logging.Formatter("%(message)s").format(record)
        assert ours == theirs, (ours, theirs)
