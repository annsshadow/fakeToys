# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置写边的原子性（L269 / A182 判据口径外的那一族：YAML 整份写）

**缺陷形**：`augmentor/config.py::save_config` 从前是 `open(config_path, "w")` +
`yaml.safe_dump(data, f)` —— 整份截断直写。截断发生在打开的那一刻，于是整个写窗口里盘上是
半份 YAML；而这份文件的读侧把坏内容读成**一个错误答案**：`api/deps.py` 装配时对
`OSError / YAMLError` 的那一档直接回落成「文件里什么都没有」，下次加载拿到的是出厂默认而
不是用户写下的配置。按 `atomic_write` 模块 docstring 里那条判据口径（只看读侧拿到截断内容
时的行为，「变成错误答案」那一档必收），这一边必收。

**为什么 A182 的逐面收漏了它而不是放过它**：那把普查尺数的是「`open(...,'w')` 紧跟
`json.dump`」，配置写的是 YAML —— 同一条边在 `json` 普查面之外。本轮不扩任何既有判据，只是
把一条早该按现口径收的边接进同一把工具（`atomic_write_text`，L163 为 JSONL 导出边加的那个）。

**面怎么定义**（三面闭合，全部可复算）：
- 形状面：产品码里 YAML 整份序列化调用现量 **1** 处（AST 扫 `api/ + augmentor/ + cli/`，
  属性名必须是 `safe_dump`/`dump` **且表达式根是 `yaml`** —— 只看属性会把 `json.dump` 那族
  算进来，而那一族有自己的尺：账本 A182 与 l99 棘轮）；`save_config` 体内**写模式 `open`
  计数为 0**，读模式那一条（读现有配置做合并）现量 1 且刻意留着，恰有一次 `atomic_write_text`。
- 字节面（零漂移钉）：把 `yaml.safe_dump` 的第一个实参截下来，用**改前的写法**（text 模式
  直写）重放一遍，与原子写出来的那份逐字节相等 —— 证明本轮只换了「原子与否」，没换写盘字节。
- 中断面：替换被拒时旧内容一格不动、临时件不残留；正常写之后目录里没有以 `.` 开头的残留件。

**行尾这一维按平台分档**（不是恒真判据，也不许造跨平台假红）：改前走 text 模式，每个 `\\n`
由平台翻译成行尾（Windows 是 `\\r\\n`），而 `atomic_write_text` 用 `newline=""` 不翻译，所以
`save_config` 显式做了 `.replace("\\n", os.linesep)`。POSIX 上 `os.linesep == "\\n"`，这一步
是空转；Windows 上它有牙。字节面那一钉走「同一份 data 重放改前写法」，两平台都成立，不依赖
这一步。

**牙齿**：`TestTheRulerHasTeeth` 三支全部零落盘（只在 pytest 的 tmp 目录里读写，仓库
`config.yaml` 从头到尾没被打开过）——① 把工具换成「旧写法复刻 + 中途抛错」，旧内容当场被截断，
证明中断面不是恒真；② 内存里把源码改回 `open(config_path, "w")` 那一形，形状面当场红；
③ 摘掉 `.replace(os.linesep)` 那一步，Windows 上写盘字节与重放结果不等（POSIX 上等，如实分档）。

跑法：`python -m pytest tests/integration/test_config_write_atomic_l269.py -q --no-cov`
"""
import ast
import os
from pathlib import Path

import pytest
import yaml

from augmentor import atomic_write as atomic_module
from augmentor.config import AppConfig, save_config

AI_DIR = Path(__file__).resolve().parent.parent.parent
PRODUCT_DIRS = ("api", "augmentor", "cli")
CONFIG_PY = AI_DIR / "augmentor" / "config.py"

#: 现量（L269 立尺）：产品码里 YAML 整份序列化（`safe_dump` / `dump`）的调用点数。
MEASURED_YAML_DUMP_SITES = 1
#: 现量：`save_config` 体内 `atomic_write_text` 的调用点数。
MEASURED_ATOMIC_CALLS = 1


def product_source_files():
    """产品码的 `.py` 名单

    Args:
        无

    Returns:
        `[(相对路径, 源码文本)]`
    """
    out = []
    for part in PRODUCT_DIRS:
        for path in sorted((AI_DIR / part).rglob("*.py")):
            out.append((str(path.relative_to(AI_DIR)), path.read_text(encoding="utf-8")))
    return out


def yaml_dump_sites(text):
    """一段源码里所有 YAML 整份序列化调用（`yaml.safe_dump(...)` / `yaml.dump(...)`）

    表达式根必须是名字 `yaml`：`dump` 这个属性名同时属于 `json.dump`，只看属性会把全仓
    JSON 写边算进 YAML 面 —— 那一族有它自己的普查尺（账本 A182 与 l99 棘轮），不能混面。

    Args:
        text: 源码文本

    Returns:
        `list[int]` 行号
    """
    hits = []
    for node in ast.walk(ast.parse(text)):
        if not (isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute)):
            continue
        if node.func.attr not in ("safe_dump", "dump"):
            continue
        root = node.func
        while isinstance(root, ast.Attribute):
            root = root.value
        if getattr(root, "id", None) == "yaml":
            hits.append(node.lineno)
    return hits


def open_mode_counts(func):
    """函数体内 `open(...)` 按打开模式的计数

    Args:
        func: `ast.FunctionDef`

    Returns:
        `{模式字母: 计数}`，缺省模式按 `r` 计
    """
    counts = {}
    for node in ast.walk(func):
        if not (isinstance(node, ast.Call)
                and getattr(node.func, "id", None) == "open"):
            continue
        mode = "r"
        if len(node.args) >= 2 and isinstance(node.args[1], ast.Constant):
            mode = str(node.args[1].value)
        for kw in node.keywords:
            if kw.arg == "mode" and isinstance(kw.value, ast.Constant):
                mode = str(kw.value.value)
        counts[mode] = counts.get(mode, 0) + 1
    return counts


def function_node(text, name):
    """按名字取顶层函数定义的 AST 节点

    Args:
        text: 源码文本
        name: 函数名

    Returns:
        `ast.FunctionDef`
    """
    for node in ast.walk(ast.parse(text)):
        if isinstance(node, ast.FunctionDef) and node.name == name:
            return node
    raise AssertionError(f"no such function: {name}")


def calls_inside(func, names):
    """函数体内对被查名字的调用点数

    Args:
        func: `ast.FunctionDef`
        names: 名字集合（`open` / `atomic_write_text` / `save_config` 里的函数名形状）

    Returns:
        `{名字: 计数}`
    """
    counts = {n: 0 for n in names}
    for node in ast.walk(func):
        if isinstance(node, ast.Call):
            target = getattr(node.func, "id", None) or getattr(node.func, "attr", None)
            if target in counts:
                counts[target] += 1
    return counts


def legacy_bytes(data, path):
    """用**改前的写法**把同一份 `data` 重放到 `path` 并返回其字节

    Args:
        data: `save_config` 当时序列化掉的那个 dict
        path: 目标路径

    Returns:
        bytes
    """
    with open(path, "w", encoding="utf-8") as handle:
        yaml.safe_dump(data, handle, allow_unicode=True, sort_keys=False)
    return Path(path).read_bytes()


class TestTheSiteFace:
    """形状面：YAML 整份写只有一处，而它不再截断目标文件"""

    def test_the_yaml_dump_face_is_one_site(self):
        """现量 1 处 YAML 整份序列化，就在 `augmentor/config.py`"""
        sites = []
        for rel, text in product_source_files():
            sites.extend((rel, line) for line in yaml_dump_sites(text))
        assert len(sites) == MEASURED_YAML_DUMP_SITES, sites
        assert sites[0][0] == str(Path("augmentor") / "config.py"), sites

    def test_save_config_writes_through_no_open_call(self):
        """`save_config` 体内写模式 `open` 为 0，读模式那一条按现量留着（本轮没顺手改合并读边）"""
        func = function_node(CONFIG_PY.read_text(encoding="utf-8"), "save_config")
        modes = open_mode_counts(func)
        assert modes.get("w", 0) == 0, modes
        assert modes == {"r": 1}, modes

    def test_save_config_goes_through_the_atomic_tool(self):
        """恰好一次 `atomic_write_text` 调用，且没有手搓 `os.replace`"""
        func = function_node(CONFIG_PY.read_text(encoding="utf-8"), "save_config")
        counts = calls_inside(func, {"atomic_write_text", "replace"})
        assert counts["atomic_write_text"] == MEASURED_ATOMIC_CALLS, counts
        # `replace` 计数 1 是 `.replace("\n", os.linesep)` 那一维（行尾照旧），不是 os.replace
        assert counts["replace"] == 1, counts


class TestTheByteFace:
    """字节面：只换了原子与否，写盘字节与改前逐字相同"""

    def test_written_bytes_equal_the_pre_atomic_shape(self, tmp_path, monkeypatch):
        """截下同一次调用的 `data`，用改前写法重放一份，两份文件逐字节相等"""
        captured = {}
        real_dump = yaml.safe_dump

        def spy(data, *args, **kwargs):
            captured["data"] = data
            return real_dump(data, **kwargs)

        monkeypatch.setattr(yaml, "safe_dump", spy)
        atomic_path = tmp_path / "atomic.yaml"
        save_config(AppConfig(), str(atomic_path))
        assert "data" in captured, "没截到序列化实参"
        replay_path = tmp_path / "legacy.yaml"
        monkeypatch.undo()
        assert legacy_bytes(captured["data"], replay_path) == atomic_path.read_bytes()

    def test_round_trip_still_loads(self, tmp_path):
        """原子写出来的那份仍然是读侧认的形状（回读一条出厂键，不比对内容）"""
        path = tmp_path / "config.yaml"
        save_config(AppConfig(), str(path))
        from augmentor.config import load_config

        reloaded = load_config(str(path))
        assert reloaded.web.port == AppConfig().web.port

    def test_the_linesep_step_is_platform_shaped_not_tautological(self, tmp_path, monkeypatch):
        """行尾那一维按平台分档：Windows 上摘掉 `.replace` 就会写出与改前不同的字节，POSIX 上相同

        这一支的存在理由：不把「它有时是空转」写进判据，后来人会把跨平台相等当成判据失灵，
        或者反过来在 Windows 上以为没翻译也无所谓。
        """
        captured = {}
        real_dump = yaml.safe_dump

        def spy(data, *args, **kwargs):
            captured["data"] = data
            return real_dump(data, **kwargs)

        monkeypatch.setattr(yaml, "safe_dump", spy)
        written = tmp_path / "atomic.yaml"
        save_config(AppConfig(), str(written))
        monkeypatch.undo()

        raw = real_dump(captured["data"], allow_unicode=True, sort_keys=False)
        untranslated = tmp_path / "untranslated.yaml"
        with open(untranslated, "w", encoding="utf-8", newline="") as handle:
            handle.write(raw)
        differs = untranslated.read_bytes() != written.read_bytes()
        if os.linesep == "\r\n":
            assert differs, "Windows 上摘掉行尾翻译必须改变写盘字节"
        else:
            assert not differs, "POSIX 上 os.linesep 就是 \\n，这一步按定义为空转"


class TestTheInterruptionFace:
    """中断面：替换失败时旧内容一格不动，临时件不残留"""

    def test_rejected_swap_leaves_old_content_and_no_temp(self, tmp_path, monkeypatch):
        """`os.replace` 那一步抛错 ⇒ 上抛 OSError、旧文件原样、目录里没有残留临时件"""
        path = tmp_path / "config.yaml"
        save_config(AppConfig(), str(path))
        before = path.read_bytes()

        def refuse(src, dst):
            raise OSError("simulated swap refusal")

        monkeypatch.setattr(atomic_module, "_replace_with_retry", refuse)
        with pytest.raises(OSError):
            save_config(AppConfig(), str(path))
        monkeypatch.undo()
        assert path.read_bytes() == before
        assert [p.name for p in tmp_path.iterdir() if p.name.startswith(".")] == []

    def test_successful_write_leaves_no_temp_leftovers(self, tmp_path):
        """正常写之后目录里只有那一份配置，没有前导点的临时件"""
        path = tmp_path / "config.yaml"
        save_config(AppConfig(), str(path))
        save_config(AppConfig(), str(path))
        assert sorted(p.name for p in tmp_path.iterdir()) == ["config.yaml"]


class TestTheRulerHasTeeth:
    """牙齿：三支削弱臂，全部零落盘"""

    def test_arm_truncating_writer_destroys_the_old_file(self, tmp_path, monkeypatch):
        """把工具换成「旧写法复刻 + 中途抛错」⇒ 旧内容当场被截断，中断面不是恒真"""
        path = tmp_path / "config.yaml"
        save_config(AppConfig(), str(path))
        before = path.read_bytes()

        def legacy_truncate(file_path, text):
            with open(file_path, "w", encoding="utf-8") as handle:
                handle.write(text[: len(text) // 2])
                handle.flush()
                os.fsync(handle.fileno())
            raise OSError("simulated crash mid-write")

        monkeypatch.setattr(atomic_module, "atomic_write_text", legacy_truncate)
        with pytest.raises(OSError):
            save_config(AppConfig(), str(path))
        monkeypatch.undo()
        assert path.read_bytes() != before, "旧写法必须把旧内容写坏，否则这一支没牙"

    def test_arm_old_open_shape_reds_the_site_face(self):
        """内存里把源码改回 `open(config_path, 'w')` 那一形 ⇒ 形状面当场红

        AU-Q1 之后写回的形状多了一支 round-trip 分支（`_dump_roundtrip_document`），
        锚点按新形状对齐；这一臂判的仍是同一件事 —— 退化成截断直写时，
        写模式 `open` 出现且 `atomic_write_text` 消失。
        """
        text = CONFIG_PY.read_text(encoding="utf-8")
        write_pair = (
            '    text = _dump_roundtrip_document(data, config_path)\n'
            '    if text is None:\n'
            '        text = yaml.safe_dump(data, allow_unicode=True, sort_keys=False)\n'
            '    atomic_write_text(Path(config_path), text.replace("\\n", os.linesep))\n'
        )
        assert write_pair in text, "锚点不在：本轮写回的形状变了，先对齐再谈削弱"
        weakened = text.replace(
            write_pair,
            '    with open(config_path, "w", encoding="utf-8") as f:\n'
            "        yaml.safe_dump(data, f, allow_unicode=True, sort_keys=False)\n",
        )
        func = function_node(weakened, "save_config")
        modes = open_mode_counts(func)
        assert modes.get("w", 0) == 1, modes
        counts = calls_inside(func, {"atomic_write_text"})
        assert counts["atomic_write_text"] == 0, counts

    def test_arm_dump_site_count_moves_when_a_second_yaml_edge_appears(self):
        """内存里加一条新的 YAML 整份写边 ⇒ 站点面从 1 掉判据（现量对不上就是红）"""
        text = CONFIG_PY.read_text(encoding="utf-8")
        grown = text + "\n\ndef second_yaml_edge(data, handle):\n" \
            "    yaml.safe_dump(data, handle)\n"
        assert len(yaml_dump_sites(grown)) == MEASURED_YAML_DUMP_SITES + 1
