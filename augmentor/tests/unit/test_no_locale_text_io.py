# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""产品侧文本 I/O 不许依赖 locale（L56，A98 的落盘那一半）

A98 的根因不是某个字符，而是「文本 I/O 跟着 locale 走」：控制台管道上它让一条 emoji
崩掉整条命令（`tests/integration/test_cli_stdio_encoding.py` 守的那半），文件上它让同一份
数据在 GBK 机器写成 GBK、在 UTF-8 机器读成乱码。L56 实测产品包里只剩 `versioning.py`
两处写 / 一处读没钉编码（Temp `l56/encaudit.txt`），本轮钉死并把判据冻成 AST 守卫：
**新增一处不带 `encoding=` 的文本读写，CI 当场变红。**

判据用 AST 而不是正则：正则版把 `open(x.with_suffix('.txt'), 'w')` 这种带嵌套括号的调用
整个漏掉，既不进总数也不进漏写名单 ⇒ 数错还是小事，指错方向才是（L55 立的「引用计数器
前先验换算」，本轮第一次现形于我自己的第一版脚本）。
"""

import ast
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent
PACKAGE_DIR = AI_DIR / "augmentor"

# 只有这三类调用会按 locale 落文本；`wave.open` / `Image.open` 是别的对象的同名方法
TEXT_WRITERS = ("write_text",)
TEXT_READERS = ("read_text",)


def product_sources():
    """产品侧 Python 源文件：`augmentor/` 包全部 + 仓库根的 CLI 入口

    不扫 `tests/`：那里的文本 I/O 同样该钉编码（实测 4 处待办），但测试语料的编码由测试
    自己造自己读，坏不了交付面；而 `web/node_modules` 与 `Temp/` 更不是产品。
    """
    files = sorted(p for p in PACKAGE_DIR.rglob("*.py") if "__pycache__" not in p.parts)
    return files + [AI_DIR / "cli.py"]


def locale_text_io_calls(source, label):
    """数出一份源文件里「文本 I/O 没钉 encoding」的调用，返回 `label:行号 说明` 列表

    Args:
        source: 源文件文本
        label: 报告用的文件名

    Returns:
        违规描述列表；二进制档（mode 含 `b`）与已带 `encoding=` 的不算
    """
    tree = ast.parse(source)
    violations = []
    for node in ast.walk(tree):
        if not isinstance(node, ast.Call):
            continue
        func = node.func
        if isinstance(func, ast.Name) and func.id == "open":
            if any(kw.arg == "encoding" for kw in node.keywords):
                continue
            mode = None
            if len(node.args) >= 2 and isinstance(node.args[1], ast.Constant):
                mode = str(node.args[1].value)
            for kw in node.keywords:
                if kw.arg == "mode" and isinstance(kw.value, ast.Constant):
                    mode = str(kw.value.value)
            if mode is not None and "b" in mode:
                continue
            # mode 是变量时也判违规：静态上证明不了它是二进制档，就得显式写编码
            violations.append("%s:%d open(mode=%r)" % (label, node.lineno, mode))
        elif isinstance(func, ast.Attribute) and func.attr in TEXT_WRITERS + TEXT_READERS:
            if any(kw.arg == "encoding" for kw in node.keywords):
                continue
            violations.append("%s:%d %s()" % (label, node.lineno, func.attr))
    return violations


FILES = product_sources()


class TestNoLocaleDependentTextIO:
    """产品包里每一处文本读写都必须自带编码 ⇒ 换机器不花、不崩、不产生第二种字节"""

    def test_scan_actually_scanned(self):
        assert len(FILES) >= 100, (
            f"只扫到 {len(FILES)} 份产品源文件 ⇒ 守卫范围塌了，这条「0 违规」不作数")

    def test_no_violations(self):
        found = []
        for path in FILES:
            found.extend(locale_text_io_calls(
                path.read_text(encoding="utf-8"),
                path.relative_to(AI_DIR).as_posix()))
        assert not found, (
            "这些文本 I/O 跟着 locale 走，换一台 GBK/UTF-8 机器就是两种字节: "
            + "; ".join(found))


class TestCheckerCatchesInjectedSources:
    """判据自己得会红：四种注入各抓一次，两种合法形态各放过一次"""

    @pytest.mark.parametrize("source, kind", [
        ("open(path, 'w').write('x')\n", "open 文本档无编码"),
        ("p.write_text('中文')\n", "write_text 无编码"),
        ("p.read_text().strip()\n", "read_text 无编码"),
        ("mode = 'w'\nopen(path, mode).write('x')\n", "mode 是变量 ⇒ 静态证不了二进制"),
    ])
    def test_injected_violation_is_caught(self, source, kind):
        hits = locale_text_io_calls(source, "probe.py")
        assert len(hits) == 1, f"{kind} 没被抓到: {hits}"

    @pytest.mark.parametrize("source", [
        "open(path, 'rb').read()\n",
        "open(path, 'w', encoding='utf-8').write('x')\n",
        "p.write_text('中文', encoding='utf-8')\n",
        "p.read_text(encoding='utf-8').strip()\n",
        "wave.open(path, 'wb')\n",
        "Image.open(path)\n",
    ])
    def test_legitimate_form_is_ignored(self, source):
        assert locale_text_io_calls(source, "probe.py") == [], (
            "合法形态被误判 ⇒ 下一轮只会把这行 `# noqa` 掉，判据就此作废")
