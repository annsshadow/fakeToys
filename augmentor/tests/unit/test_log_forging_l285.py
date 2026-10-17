# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L285：日志注入（CWE-117）——一条记录必须恒等于一行

**缺陷形状**（改前实测，仓外探针 `l285_probe_loginject.py`）：`POST /api/system/dependency/datasets`
的 `name` 是任意 JSON 字符串，`augmentor/dependency.py` 把它原样拼进
`logger.info(f"数据集注册成功: {name} ({dataset_id})")`。一个带 `\n` 的 name 让**同一条记录**在
汇上落成 2 行（实测读数 `RENDERED_LINES=2`，第二行逐字是攻击者自造的
`[2026-10-17 02:00:00] INFO ROOT admin login success from attacker`）；ANSI 变体 `x\x1b[2J…`
则整串原样进流，清屏与光标定位在终端里真生效。于是「日志里那一行是谁写的」失去可信度——
伪造的登录成功 / 伪造的时间戳行与真记录逐字节同形。

**这一族的三条负数结果**（一并钉住，免得下一轮重扫）：① URL 路径不是口子——uvicorn 的 access
log 打的是 `raw_path`，`%0A` 保持百分号编码（实测 access 行逐字含 `%0A` 而非换行）；② 经
TestClient 的 `request.url.path` 也没有落下真换行（实测 `慢请求 GET /api/status[2026-10-17 …]`
那一条 `multiline=0`）；③ 请求头不可能带 CR/LF——HTTP 帧本身禁止，`X-Request-ID` 的透传因此
不构成响应拆分。

**修法落在汇上，不落在 170 处调用点上**：`augmentor/logging_setup.py` 尾部新增
`ControlSafeFormatter`，只在 `formatMessage` 这一跳把控制字符换成 `\xNN` 字面量。理由：一条
record 要经过 formatter 才变成「一行」，所以 `logging.format` 写成什么串都覆盖得到；调用点
逐个消毒既改 170 处又必然漏。`build_formatter` 交出的就是这个子类，API 入口的
`logging.basicConfig(handlers=[build_handler(…)])` 也走它（3.12 的 `basicConfig` 不收
`formatter=`，实测 `ValueError: Unrecognised argument(s): formatter`）。

**刻意不消毒的两格**（都记进待拍，不在本轮单方面改口径）：① `Formatter.format()` 在
`formatMessage` 之后才追加 traceback，栈保持原样多行是可观测性的底线，所以栈里的
`ValueError: inner\n伪造行` 仍会落成两行（A388）；② CLI 在「配置文件没写 `logging` 节」时
root 没有 handler，日志走 `logging.lastResort` 的裸 `%(message)s`，那一条不过消毒器（A387）——
CLI 的输入是本机用户自己的参数，不构成信任边界，但形状确实与 API 不同。

本文件的非恒真证据：`TestSinkTeeth` 把捕获 handler 换回裸 `logging.Formatter`，同一个请求当场
落成 2 行，并且同一支用例证明 record 里的原文仍在（消毒只在渲染那一跳，不在 record 上）；
`TestFamilyClosureCensus` 的构造点普查用**仓外副本**喂进一处真构造（`logging.Formatter(`）并被
当场量到，同串写成正注释则不算站点 —— 走的是零写入通道，不给仓库留残留；`TestApiEntryWiring`
在**干净子进程**里量入口装配——pytest 进程 root 已有 handler，`basicConfig` 在那里是空转，
不另开进程就测不到入口那一格。
"""

import logging
import logging.handlers
import os
import re
import subprocess
import sys
from pathlib import Path

from fastapi.testclient import TestClient

from api.main import app
from augmentor import logging_setup
from augmentor.logging_setup import (
    ControlSafeFormatter,
    build_formatter,
    build_handler,
    escape_control,
)

#: 攻击者自造的那一行：改前它会作为**第二条物理行**出现在汇上
FORGE_PAYLOAD = "evil\n[2026-10-17 02:00:00] INFO ROOT admin login success"
ANSI_PAYLOAD = "ds\x1b[2J\x1b[1;1HFAKED"

#: 逐字取自产品实现的那一集：`\x09`（Tab）刻意不在里面
CONTROL_SAMPLES = (
    ("\x00", r"\x00"),
    ("\x08", r"\x08"),
    ("\n", r"\x0a"),
    ("\x0b", r"\x0b"),
    ("\x0c", r"\x0c"),
    ("\r", r"\x0d"),
    ("\x1b", r"\x1b"),
    ("\x1f", r"\x1f"),
    ("\x7f", r"\x7f"),
    ("\x85", r"\x85"),
    ("\x9f", r"\x9f"),
)


class TestEscapeControlUnit:
    r"""消毒函数本身：形状、例外、以及「不吞掉任何一码点」"""

    def test_clean_text_returns_the_same_object(self):
        """干净行走快路：逐字节不变，连对象都是同一个（`identity_when_clean`）"""
        text = "创建断点: task-7f3a, 总条目: 6902"
        assert escape_control(text) is text

    def test_every_control_char_becomes_a_two_char_escape(self):
        r"""允许集里每一个码点都得变成 `\xNN` 字面量，一个都不能漏成原字符"""
        for raw, expected in CONTROL_SAMPLES:
            got = escape_control("a" + raw + "b")
            assert expected in got, (raw, got)
            assert raw not in got, (raw, got)
            assert len(got.splitlines()) == 1, (raw, got)

    def test_tab_is_deliberately_not_escaped(self):
        """Tab 不起新行也不触转义序列；钉掉它只会把对齐的表式日志打散"""
        assert escape_control("列A\t列B\t1234") == "列A\t列B\t1234"

    def test_cjk_percent_and_quotes_are_untouched(self):
        """非控制字符一码点不吞：中文、`%`、引号、反斜杠都得逐字留在原位"""
        text = '数据集 "x" 100% \\ \\u41 正常'
        assert escape_control(text) == text

    def test_the_escape_is_reversible(self):
        r"""`\xNN` 档的可逆性：反解回原码点，取证时不丢任何信息（U+FFFD 档做不到）"""
        got = escape_control("x\ny")
        assert got == "x\\x0ay"
        assert got.encode("ascii").decode("unicode_escape") == "x\ny"

    def test_lone_surrogate_is_not_a_control_char(self):
        """代理位归 L284 那两族管（渲染崩溃），本尺子不越界改它"""
        assert escape_control("x\ud800?") == "x\ud800?"


class TestFormatterRenderShape:
    r"""渲染面：一行一条、不改 record、不动 traceback"""

    @staticmethod
    def _record(msg, args=(), level=logging.INFO):
        return logging.LogRecord("augmentor.checkpoint", level, __file__, 212, msg, args, None)

    def test_message_with_newline_renders_exactly_one_line(self):
        got = ControlSafeFormatter("%(message)s").format(self._record(FORGE_PAYLOAD))
        assert len(got.splitlines()) == 1, got
        assert r"\x0a" in got, got

    def test_the_record_itself_is_not_mutated(self):
        """消毒只发生在渲染的最后一跳：record 的原文与 args 一个字节都不能被改写

        否则 CLI 的 `--format` 之类同一 record 的第二次消费会读到被我们动过的串。
        """
        record = self._record("a%sb\n", ("x",))
        ControlSafeFormatter("%(message)s").format(record)
        assert record.getMessage() == "axb\n"
        assert record.args == ("x",)

    def test_args_are_escaped_after_interpolation_not_before(self):
        """控制字符藏在 `args` 里也一样：判的是渲染出来的那一行，不是消息模板"""
        got = ControlSafeFormatter("%(message)s").format(self._record("v=%s", ("\x1b[2J",)))
        assert got == "v=\\x1b[2J", got

    def test_traceback_stays_multiline(self):
        """栈是可观测性的底线：`exc_info` 那一段刻意不消毒（A388 待拍）"""
        record = self._record("boom", level=logging.ERROR)
        try:
            raise ValueError("inner\nforged-traceback-line")
        except ValueError:
            record.exc_info = sys.exc_info()
        got = ControlSafeFormatter("%(message)s").format(record)
        assert len(got.splitlines()) > 2, got
        assert "forged-traceback-line" in got, "栈里的原文必须还在，否则这一格就悄悄改了口径"

    def test_every_format_string_field_is_sanitized(self):
        """消毒在整行渲染之后：`%(name)s` 这类非 message 字段带控制字符也覆盖得到"""
        record = self._record("ok")
        record.name = "aug\nforged"
        got = ControlSafeFormatter("%(name)s %(message)s").format(record)
        assert len(got.splitlines()) == 1, got
        assert r"aug\x0aforged ok" == got, got


class TestSinkWiring:
    """消毒必须挂在唯一的构造点上，而不是散落各处"""

    def test_build_formatter_returns_the_control_safe_subclass(self):
        fmt = "%(asctime)s - %(levelname)s - %(message)s"
        got = build_formatter(fmt)
        assert type(got) is ControlSafeFormatter
        assert isinstance(got, logging.Formatter), "既有棘轮认的是 Formatter 子类，这里不能反"
        assert got._fmt == fmt, "格式串语义一字未改，只加消毒"

    def test_build_handler_uses_stderr_and_the_sanitizer(self):
        handler = build_handler("%(message)s")
        assert handler.stream is sys.stderr
        assert type(handler.formatter) is ControlSafeFormatter

    def test_apply_logging_config_installs_the_sanitizer(self, tmp_path):
        """`logging` 节写出来的那条装配路（API 与 SDK 共用）也得带消毒器"""
        log_file = tmp_path / "app.log"
        settings = type("S", (), {"level": "INFO", "file": str(log_file),
                                  "format": "%(levelname)s|%(message)s"})()
        root = logging.getLogger()
        before = list(root.handlers)
        try:
            summary = logging_setup.apply_logging_config(settings, written=("level", "file", "format"))
            installed = [h for h in root.handlers if h not in before]
            assert summary["file"] == str(log_file)
            assert installed, "装配没装上 handler，那这条用例就是恒真"
            for handler in installed:
                assert type(handler.formatter) is ControlSafeFormatter, handler
        finally:
            for handler in [h for h in root.handlers if h not in before]:
                root.removeHandler(handler)
                handler.close()

    def test_file_sink_gets_the_escaped_line_too(self, tmp_path):
        """落盘那一条才是伪造行的正题：文件里必须只有一行"""
        log_file = tmp_path / "forge.log"
        handler = logging.handlers.RotatingFileHandler(str(log_file), encoding="utf-8")
        handler.setFormatter(ControlSafeFormatter("%(message)s"))
        root = logging.getLogger()
        before = list(root.handlers)
        root.addHandler(handler)
        try:
            logging.getLogger("augmentor.dependency").info(FORGE_PAYLOAD)
            handler.flush()
        finally:
            root.removeHandler(handler)
            handler.close()
            assert [h for h in root.handlers if h not in before] == []
        assert log_file.read_text(encoding="utf-8").splitlines() == [
            r"evil\x0a[2026-10-17 02:00:00] INFO ROOT admin login success"
        ]


class TestFamilyClosureCensus:
    r"""族收口的尺子：产品码里裸 `logging.Formatter(` 的构造点数恰好为 0

    范围刻意钉在「产品面」这一族（`api/` + `augmentor/`，跳过 `__pycache__` 与注释行），
    而不是全仓正则扫 `Formatter` —— 后者会被 `RAGFormatter` 那种同名异物与散文判红
    （同一缺陷形状在 L284 的端点族上实测过）。
    """

    PRODUCT_DIRS = ("api", "augmentor")

    @staticmethod
    def _sites(root, dirs):
        hits = []
        for part in dirs:
            for path in (Path(root) / part).rglob("*.py"):
                if "__pycache__" in path.parts:
                    continue
                for no, line in enumerate(
                    path.read_text(encoding="utf-8", errors="replace").splitlines(), 1
                ):
                    if line.lstrip().startswith("#"):
                        continue
                    if "logging.Formatter(" in line:
                        hits.append(f"{path.name}:{no}")
        return sorted(hits)

    def _product_root(self):
        return Path(logging_setup.__file__).resolve().parent.parent

    def test_no_bare_formatter_construction_left_in_product_code(self):
        assert self._sites(self._product_root(), self.PRODUCT_DIRS) == []

    def test_the_ruler_is_not_vacuous(self, tmp_path):
        """注入臂（零写入通道）：在仓外副本里造一处裸构造，同一把尺子必须当场量到"""
        pkg = tmp_path / "augmentor"
        pkg.mkdir()
        (pkg / "injected.py").write_text(
            "import logging\n\nF = logging.Formatter('%(message)s')\n", encoding="utf-8"
        )
        hits = self._sites(tmp_path, ("augmentor",))
        assert hits == ["injected.py:3"], hits
        # 同一条注入改成正注释，就必须不算站点 —— 判的是构造，不是散文
        (pkg / "injected.py").write_text("# logging.Formatter(...) 只是说明\n", encoding="utf-8")
        assert self._sites(tmp_path, ("augmentor",)) == []


class TestHttpEndToEnd:
    """坐实的那只端点：改前 2 行，改后 1 行，而对外响应一字未改"""

    @staticmethod
    def _seed(tmp_path):
        data = tmp_path / "seed.json"
        data.write_text('[{"text": "a"}]', encoding="utf-8")
        return data

    @staticmethod
    def _capture_sink():
        """一个「按产品口径渲染」的捕获 handler：用它自己的 formatter 数行数

        刻意装 `ControlSafeFormatter` 而不是裸 `logging.Formatter` —— 这条链要量的就是
        渲染那一跳，用裸 Formatter 等于把判据降级成 `record.getMessage()` 有没有换行。
        """
        handler = logging.Handler()
        handler.setFormatter(ControlSafeFormatter("%(levelname)s %(message)s"))
        holder = []
        handler.emit = lambda record: holder.append(handler.format(record))
        return handler, holder

    @staticmethod
    def _register(tmp_path, monkeypatch, name):
        monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
        data = TestHttpEndToEnd._seed(tmp_path)
        (tmp_path / "reg").mkdir(exist_ok=True)
        with TestClient(app) as client:
            return client.post(
                "/api/system/dependency/datasets",
                params={"registry_path": str(tmp_path / "reg")},
                json={"name": name, "input_file": str(data)},
            )

    def test_forged_newline_now_renders_one_line(self, tmp_path, monkeypatch):
        handler, rendered = self._capture_sink()
        root = logging.getLogger()
        root.addHandler(handler)
        try:
            response = self._register(tmp_path, monkeypatch, FORGE_PAYLOAD)
        finally:
            root.removeHandler(handler)
        assert response.status_code == 200, response.text
        hits = [line for line in rendered if "数据集注册成功" in line]
        assert len(hits) == 1, rendered
        assert len(hits[0].splitlines()) == 1, hits[0]
        assert r"evil\x0a[" in hits[0], hits[0]

    def test_ansi_escape_no_longer_reaches_the_stream_raw(self, tmp_path, monkeypatch):
        handler, rendered = self._capture_sink()
        root = logging.getLogger()
        root.addHandler(handler)
        try:
            response = self._register(tmp_path, monkeypatch, ANSI_PAYLOAD)
        finally:
            root.removeHandler(handler)
        assert response.status_code == 200, response.text
        hits = [line for line in rendered if "数据集注册成功" in line]
        assert len(hits) == 1, rendered
        assert "\x1b" not in hits[0], hits[0]

    def test_the_response_body_still_echoes_the_raw_name(self, tmp_path, monkeypatch):
        """只治日志，不治对外语义：登记回来的 `name` 必须还是原文，否则就成了行为变更"""
        response = self._register(tmp_path, monkeypatch, FORGE_PAYLOAD)
        assert response.status_code == 200, response.text
        assert response.json()["name"] == FORGE_PAYLOAD


class TestSinkTeeth:
    """非恒真：把消毒器换回裸 Formatter，同一支用例必须当场回到 2 行"""

    def test_plain_formatter_alone_still_forges_the_second_line(self, tmp_path, monkeypatch):
        handler = logging.Handler()
        handler.setFormatter(logging.Formatter("%(levelname)s %(message)s"))
        rendered = []
        handler.emit = lambda record: rendered.append(handler.format(record))
        root = logging.getLogger()
        root.addHandler(handler)
        try:
            response = TestHttpEndToEnd._register(tmp_path, monkeypatch, FORGE_PAYLOAD)
        finally:
            root.removeHandler(handler)
        assert response.status_code == 200, response.text
        hits = [line for line in rendered if "数据集注册成功" in line]
        assert len(hits) == 1, "裸 Formatter 下这条记录应当落成两行，而不是被别的消毒器救下"
        body = hits[0]
        assert len(body.splitlines()) == 2, body
        # 语句是 `f"数据集注册成功: {name} ({dataset_id})"`：换行插在 name 里，所以 ID 尾巴
        # 落在被伪造出来的那一行末尾，而 ID 是按内容算的哈希 —— 钉尾巴形状而不是钉字面值。
        assert re.fullmatch(
            r"\[2026-10-17 02:00:00\] INFO ROOT admin login success \([0-9a-f]{8}\)",
            body.splitlines()[1],
        ), body

    def test_the_captured_record_still_holds_the_raw_message(self, tmp_path, monkeypatch):
        """消毒只发生在渲染那一跳：`record.getMessage()` 里原文必须在，否则查案时读不到真值

        这一支同时把上一条的牙齿收紧：如果「消毒」其实是改 record，那裸 Formatter 也
        伪造不出第二行，两条一起才说明改动落在 formatter 上。
        """
        seen = []
        grabber = logging.Handler()
        grabber.emit = lambda record: seen.append(record.getMessage())
        root = logging.getLogger()
        root.addHandler(grabber)
        try:
            TestHttpEndToEnd._register(tmp_path, monkeypatch, FORGE_PAYLOAD)
        finally:
            root.removeHandler(grabber)
        assert any("\n" in msg and "数据集注册成功" in msg for msg in seen), seen


class TestApiEntryWiring:
    r"""入口装配：pytest 进程里 `basicConfig` 是空转，只能开干净子进程量

    子进程脚本逐字做三件事：导入 `api.main`、问 root handler 的 formatter 是什么类、
    发一条带 `\n` 的 WARNING 并数它落成几行。判据钉在「同一个类 + 一行」两条上，
    单钉类型会漏掉「装了子类但没接 formatMessage」这一形。
    """

    SCRIPT = (
        "import logging, sys\n"
        "sys.path.insert(0, %r)\n"
        "import api.main  # noqa: F401 —— 装配发生在 import 期\n"
        "root = logging.getLogger()\n"
        "kinds = sorted({type(h.formatter).__name__ for h in root.handlers})\n"
        "logging.getLogger('augmentor.checkpoint').warning('a\\nb')\n"
        "for h in root.handlers:\n"
        "    h.flush()\n"
        "sys.stderr.flush()\n"
        "print('KINDS=' + ','.join(kinds))\n"
    )

    def test_clean_import_installs_the_sanitizer_and_one_line(self):
        repo = Path(logging_setup.__file__).resolve().parent.parent
        # 规则 A（`test_subprocess_text_decoding.py`）：父进程侧连编码一起钉，子进程侧用
        # PYTHONIOENCODING 钉 —— 这条子进程的 stderr 里有中文产品文案，两侧任一按 locale 走
        # 都会把「读那一条日志」变成读乱码，而本用例判的正是那一行的形状。
        got = subprocess.run(
            [sys.executable, "-c", self.SCRIPT % str(repo)],
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            cwd=str(repo),
            timeout=180,
            env={**os.environ, "PYTHONIOENCODING": "utf-8"},
        )
        assert got.returncode == 0, got.stderr
        stdout = got.stdout
        stderr = got.stderr
        assert "KINDS=ControlSafeFormatter" in stdout, stdout
        forged = [line for line in stderr.splitlines() if line.startswith("a") or "augmentor.checkpoint" in line]
        assert len(forged) == 1, stderr
        assert r"a\x0ab" in forged[0], forged[0]
