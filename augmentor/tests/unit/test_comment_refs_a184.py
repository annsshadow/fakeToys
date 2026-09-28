# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A184 入库：**全部**注释与自有文档里的「定位引用」都要真的指得到东西

**为什么单独立一支，而不是把 L79 那把尺子调宽**：L79 的守卫只有两条正则，而它们都写死了
`.py` 后缀 ⇒ 它对「非 Python 载体上的引用」是**结构性失明**的。本轮普查里第一条真失效就是这么
漏的：出厂配置模板里那两个重试键的行号被两轮改动搬走之后，指向 YAML 的行号引用落在的是
**空行**，而仓库里任何一条已入库的判据都读不到它 —— 不是没跑，是那个形状压根不进正则
（这一格由 `TestCensusIsProven.test_the_rulers_own_regex_still_cannot_see_a_yaml_line_ref`
钉住，不是本文档的一句话主张）。所以这一支不是 L79 的副本，而是它看不见的**那一整个载体族**
（YAML、Markdown、ts/tsx/js/vue 注释）的补集守卫；两支的扫描面交集只有两份文档，而桶的判定
（名字怎么解析、区间怎么才算可证失效）全部从 L79 import，不留第二份实现。

**三条口径（A184 原文说「要先定这三条」，本轮拍定）**：

1. **扫描面** = 产品树里 Python 文件的**注释 + docstring**（tokenize 的 COMMENT 加 AST 的
   docstring 区间，正文代码不参与）、web 那侧 ts/tsx/js/jsx/vue 的注释行、以及全部 Markdown。
   跳过 Temp / .backups / bak / node_modules 与各类缓存目录 —— 草稿与依赖不是案面。
2. **索引** = 全部扩展名，不再只收 Python 文件。这一步是判据成立的前提：TS 源文件名在旧索引里
   压根不存在，于是「注释里写了一个不存在的 TS 文件」这件事今天没有任何东西会红。
3. **指向 Temp 的引用沿用「指向工件」而不是「失效」** —— 与 L79 同口径，单独成档、只当棘轮。
4. **符号主张档（`名字()` 这种形状）的「宇宙」只收代码面**，不收注释、docstring、字符串样本
   与 Markdown。这一条是注入实测逼出来的：第一版宇宙是「全文本扫标识符」，于是往一份真文件的
   docstring 里写一支根本不存在的 API 主张，守卫**全绿** —— 那句主张自己把名字喂给了宇宙，
   假主张只要被写下来就自动变成真名字。改成代码面之后同一支注入立刻红（报告见 Temp 下那支注入
   探针）。代价一并写清：点号右边那一截不验证，只按「点号左边打得开一个真模块」放行外部主张
   （`re._compile` 与 `str.isspace` 这类标准库形状本仓代码里不会调用，逐条 getattr 既慢又要
   在测试里 import 任意模块）。

**两条本轮新拍的口径**（L79 没面对过，因为它只读两份 augmentor 相对书写的文档）：

- **系别要问两次**：注释与文档同时被「augmentor 目录内」和「外层仓库根」两种读者读，所以把
  产品路径多包一层 augmentor 前缀的写法**不是缺陷**。判法是同一条引用按两系各问一次、再按
  优先级合并（见 `reference_state`），而不是硬选一系 —— 硬按 augmentor 相对判会把成百条真引用
  算成死链，硬按仓库根判则反过来把裸产品路径算成死链。**这一条让注释面比 L79 的两份文档更宽**
  （那边第 5 条口径按 augmentor 相对一系判），是刻意的不对称：那两份文档自己规定了书写系别，
  注释没有。两支守卫对同一批句子可以一严一宽，但**不存在洗白** —— 更严的那支会先把多系别的
  写法挡在文档门外。
- **父目录有没有被跟踪的文件**是「坏链接」与「示例路径」的分界：文档目录里全是真文件，所以
  指向该目录一个不存在的配置文档名**本该存在** ⇒ 判死；而示例路径的父目录一个被跟踪文件都
  没有 ⇒ 单独成档 runtime_ns，判它死等于把散文当契约。用 git 的跟踪集合而不是磁盘，因为磁盘
  上还站着不入仓的用户数据：按「目录里有文件」判，本轮第一版实测把 414 条示例路径全判成死链。

**判据一律「只降不升、精确相等」**（承 L76 / L77 / L79 的棘轮口径）：`CEILING` 每条都是等值断言，
所以「顺手清掉一条歧义」同样会让用例红 —— 那是设计，它逼着清账的人留一句「谁清的、清完剩多少」。
可证失效三档（dead path / dead line / dead symbol）硬 0，不进棘轮、不许混桶。

**本文件的注释与文档里不写反引号路径引用**：它自己就属于扫描面，写一条等于给自己造一条要维护
的引用（本轮普查数出来的 15 条失效里有 4 条正是守卫自己的案面写的）。需要引用形状的测试数据
一律用 `tick()` 在运行期拼，字面量在源码里永远不成形。
"""
import ast
import builtins
import collections
import functools
import importlib.util
import io
import os
import pathlib
import re
import subprocess
import sys
import tokenize

import pytest

from tests.unit.test_doc_line_refs_l79 import LINE_REF, name_bucket

ROOT = pathlib.Path(__file__).resolve().parents[2]

IDX_SKIP = frozenset({".git", "__pycache__", ".pytest_cache", "node_modules"})
SCAN_SKIP = IDX_SKIP | {".backups", "bak", "htmlcov", ".venv", "venv", ".mypy_cache"}
SRC_EXT = frozenset({".py", ".ts", ".tsx", ".js", ".jsx", ".vue"})
DOC_EXT = frozenset({".md", ".yaml", ".yml", ".toml", ".sql", ".ini", ".cfg",
                     ".json", ".txt"})
ALL_EXT = SRC_EXT | DOC_EXT
#: 进扫描面的扩展名：注释族全收，文档只收 Markdown —— YAML/JSON 里那些「像路径的串」绝大多数
#: 是数据而不是给读者指路的话，收进来只会淹掉信号
SCAN_EXT = SRC_EXT | {".md"}
#: 账本与架构文档的散文里全是「当年那条 API 主张后来作废」这类句子，符号档只在产品注释与
#: README 上用（案面与判面分开量，承 A183 那条读法）
NO_SYM_SCAN = frozenset({"OPTIMIZATION_LOOP.md", "docs/ARCHITECTURE.md",
                         "OPTIMIZATION_PLAN.md"})

_EXT_ALT = "|".join(sorted(e[1:] for e in ALL_EXT))
#: 定位引用：反引号里「带扩展名的路径」，可跟 `:行号` 或 `:起-止`
REF = re.compile(r"`([A-Za-z0-9_/\\.\-]*[A-Za-z0-9_]\.(?:" + _EXT_ALT +
                 r"))(?::(\d+)(?:-(\d+))?)?`")
_SYM = r"[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*"
#: 符号档只认「反引号闭合后紧跟左括号」这一种形状 —— 那是 API 主张。第一版把散文里的名字也
#: 当符号，普查当场数出 220 条假缺陷：逻辑运算符、文件名、键名全进了死档。
SYM_CALL = re.compile(r"`(" + _SYM + r")\(\)`")
SYM_PAREN = re.compile(r"`(" + _SYM + r")`\(")
#: 注释标记按**被指向文件**的语言取：`#` 在 Markdown/YAML 里是内容，只有 Python 里才是注释。
#: 这一条是本轮 selftest 抓出来的（第一版照抄 L79 的 `#`，于是把 README 的标题行读成注释）。
COMMENT_MARKERS = {".py": ("#",), ".ts": ("//", "*"), ".tsx": ("//", "*"),
                   ".js": ("//", "*"), ".vue": ("//", "*")}

#: 现量基线（本轮普查终态，扫描面 = 476 份文件的注释与自有文档、共 3 792 条定位引用、
#: 326 条符号主张）。**只降不升**。前三档 = 可证失效，硬 0；后四档 = 已记账的歧义欠账 +
#: 运行期示例路径 + 指向不入仓工件（scratch 两档合起来就是 A128 那块地盘的注释面读数）。
CEILING = {
    "dead_path": 0,
    "dead_line": 0,
    "symbol_dead": 0,
    "ambiguous": 328,
    "runtime_ns": 31,
    "scratch": 728,
    "scratch_missing": 289,
}

#: 反空转下界：这三个数**不是缺陷计数**，而是「普查真的看见了多大一片」的证据。
#: 删引用是改进，不该被下界卡住；它只负责证明扫描没空转 —— 路径拼错、跳过清单写歪、
#: git ls-files 返回空，三种事故都会让「全部死档为 0」靠「什么都没扫到」拿到手。
FLOOR = {"files": 400, "refs": 3000, "symbol_live": 300}

#: 判据能产出的**全部桶名**（词汇表，与 `reference_state` 的返回集 + 两档符号 + dead_line 对齐）。
#: `live` 与 `symbol_live` 是好引用，不进 `CEILING`；`dead_line` 只能由 `span_state` 产出，
#: 所以它不在 `_LIVE_ORDER` 里却必须在词汇表里。
BUCKETS = frozenset({"live", "ambiguous", "dead_path", "dead_line", "runtime_ns",
                     "scratch", "scratch_missing", "symbol_live", "symbol_dead"})

#: 两系别合并的优先级就是口径，顺序即结论（每一档为什么排在这个位置见 `reference_state`）。
#: `dead_path` 排在工件两档之前：Temp 里站着 HEAD 快照与历史副本，排在后面等于允许一份不入仓
#: 的拷贝把产品路径写错洗白 —— 这正是 L79 第 5 条口径的理由。`ambiguous` 更靠前，因为「这个名字
#: 有两条候选」说的是引用指得到东西、只是不唯一，它归欠账档而不该抢硬 0 档的红。
_LIVE_ORDER = ("live", "ambiguous", "dead_path", "scratch", "scratch_missing",
               "runtime_ns")


def tick(*parts):
    """运行期拼一个带反引号的引用：本文件属于扫描面，源码里的字面量永远不成形"""
    return "`" + "".join(parts) + "`"


def _walks(include_temp, skip):
    for base, dirs, files in os.walk(ROOT):
        if not include_temp and "Temp" in set(pathlib.PurePath(base).parts):
            dirs[:] = []
            continue
        dirs[:] = [d for d in dirs if d not in skip]
        for name in files:
            path = pathlib.Path(base) / name
            yield path.relative_to(ROOT).as_posix(), path


@functools.lru_cache(maxsize=1)
def full_index():
    """`{文件名: (仓内路径的段元组, ...)}` —— 与 L79 的 `build_index` 同式，但**收全部扩展名**

    口径 2 的落点。只在这里放宽索引，桶的判定仍由 L79 的 `name_bucket` 出，判据不留第二份。
    """
    index = collections.defaultdict(list)
    for rel, _ in _walks(include_temp=True, skip=IDX_SKIP):
        parts = pathlib.PurePath(rel).parts
        index[parts[-1]].append(parts)
    return {k: tuple(sorted(v)) for k, v in index.items()}


@functools.lru_cache(maxsize=1)
def source_dirs():
    """装源码的顶层目录（Temp 除外）—— 与 L79 同式，只是喂的是全扩展名那一份索引"""
    return frozenset(p[0] for paths in full_index().values() for p in paths
                     if len(p) > 1 and p[0] != "Temp")


@functools.lru_cache(maxsize=None)
def lines_of(rel):
    try:
        return (ROOT / rel).read_text(encoding="utf-8").splitlines()
    except (UnicodeDecodeError, OSError):
        return None


def python_comments(text):
    """Python 文件的**注释 + docstring**（逐行带行号）；正文代码不参与引用普查"""
    out, docs = [], set()
    try:
        tree = ast.parse(text)
    except SyntaxError:
        tree = None
    if tree is not None:
        for node in ast.walk(tree):
            if not isinstance(node, (ast.Module, ast.ClassDef, ast.FunctionDef,
                                     ast.AsyncFunctionDef)):
                continue
            body = getattr(node, "body", None)
            if (body and isinstance(body[0], ast.Expr)
                    and isinstance(body[0].value, ast.Constant)
                    and isinstance(body[0].value.value, str)):
                docs.update(range(body[0].lineno, body[0].end_lineno + 1))
    try:
        for tok in tokenize.generate_tokens(io.StringIO(text).readline):
            if tok.type == tokenize.COMMENT:
                out.append((tok.start[0], tok.string))
            elif tok.type == tokenize.STRING and tok.start[0] in docs:
                out.append((tok.start[0], tok.string))
    except (tokenize.TokenError, IndentationError, SyntaxError):
        pass
    return out


def script_comments(text):
    """ts / tsx / js / vue 的注释行（行首 `//`、块注释起始与 `*` 续行）"""
    return [(lineno, stripped)
            for lineno, line in enumerate(text.splitlines(), 1)
            if (stripped := line.strip()).startswith(("//", "/*", "*"))]


@functools.lru_cache(maxsize=1)
def corpus():
    """扫描面全文：`((相对路径, ((行号, 文本), ...)), ...)`"""
    rows = []
    for rel, path in _walks(include_temp=False, skip=SCAN_SKIP):
        ext = pathlib.PurePath(rel).suffix
        if ext not in SCAN_EXT:
            continue
        text = lines_of(rel)
        if text is None:
            continue
        if ext == ".py":
            notes = python_comments("\n".join(text))
        elif ext == ".md":
            notes = list(enumerate(text, 1))
        else:
            notes = script_comments("\n".join(text))
        rows.append((rel, tuple(notes)))
    return tuple(rows)


@functools.lru_cache(maxsize=1)
def symbol_universe():
    """产品树里**代码面**在场的标识符 —— 符号档「活着」的定义

    为什么宇宙不许包含注释与文档：本轮注入实测直接量出来的（Temp 下那支注入探针的
    symbol_dead 一档）—— 往一份真实文件的 docstring 里写一支根本不存在的 API 主张，守卫
    **全绿**。第一版的宇宙是「全文本扫标识符」，于是那句主张自己把名字喂给了宇宙：一条假
    主张只要被写下来就自动变成真名字，判据自证能力为 0。改成只收代码面之后，同一支注入
    立刻红。

    代价说清楚：代码面宇宙仍然只判「名字在场」，不解析归属，也不验证点号右边那一截。
    标准库那一类主张由 `declared_or_external` 单独放行（`re._compile`、`str.isspace` 这种
    末端属性本仓代码里压根不会调用，逐条 getattr 既慢又要在测试里 import 任意模块）。
    """
    names, ident = set(), re.compile(r"[A-Za-z_][A-Za-z0-9_]{2,}")
    for rel, path in _walks(include_temp=False, skip=SCAN_SKIP):
        ext = pathlib.PurePath(rel).suffix
        if ext not in ALL_EXT or ext == ".md":
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        if ext == ".py":
            try:
                tree = ast.parse(text)
            except SyntaxError:
                tree = None
            if tree is not None:
                for node in ast.walk(tree):
                    if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef,
                                         ast.ClassDef)):
                        names.add(node.name)
                    elif isinstance(node, ast.Assign):
                        names.update(t.id for t in node.targets
                                     if isinstance(t, ast.Name))
                    elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
                        names.add(node.target.id)
                    elif isinstance(node, ast.Attribute):
                        names.add(node.attr)
            try:
                # tokenize 的 NAME 天然不含注释与字符串内容 —— 自洗白就是这么挡掉的
                names.update(tok.string for tok in
                             tokenize.generate_tokens(io.StringIO(text).readline)
                             if tok.type == tokenize.NAME)
            except (tokenize.TokenError, IndentationError, SyntaxError):
                pass
        elif ext in SRC_EXT:
            # 脚本文件只取非注释行（块注释按整块跳过：续行的 `*` 里也可能带名字主张）
            for line in text.splitlines():
                stripped = line.strip()
                if stripped.startswith(("//", "/*", "*")):
                    continue
                names.update(ident.findall(stripped))
        else:
            names.update(ident.findall(text))
    return names


@functools.lru_cache(maxsize=None)
def external_head(head):
    """点号主张的**头**是不是一个打得开的模块（标准库或已装依赖）"""
    if head in dir(builtins) or head in sys.builtin_module_names:
        return True
    try:
        return importlib.util.find_spec(head) is not None
    except (ImportError, ValueError):
        return False


def declared_or_external(name, universe):
    """符号主张的放行判据：末端名字在代码面宇宙里，或那是一个「外部模块.成员」形状的主张"""
    leaf = name.split(".")[-1]
    if leaf in universe:
        return True
    # 只有带点号、且点号左边打得开一个真模块的形状才走外部档（`re._compile`、`str.isspace`）；
    # 裸名不存在就是不存在，这里不许有第二种解释。
    return "." in name and external_head(name.split(".")[0])


@functools.lru_cache(maxsize=1)
def tracked_dirs():
    """被跟踪路径的**全部父目录** —— 「坏链接」与「运行期示例路径」的那条分界线

    只要父目录，因为判据问的是「这个目录里本来有没有东西」；被跟踪文件本身由索引答。
    用 git 的跟踪集合而不是磁盘，因为磁盘上还站着不入仓的用户数据 —— 按「目录里有文件」判，
    本轮第一版实测把 414 条示例路径全判成了死链。
    """
    out = subprocess.run(["git", "ls-files", "--", "augmentor"], cwd=ROOT.parent,
                         capture_output=True, text=True)
    dirs = set()
    for line in out.stdout.splitlines():
        if not line.startswith("augmentor/"):
            continue
        parts = pathlib.PurePath(line).parts[1:]
        dirs.update("/".join(parts[:i]) for i in range(1, len(parts)))
    return frozenset(dirs)


def span_state(rel, start, end):
    """区间引用只有**整段都不是代码**才算可证失效 —— 这一条口径抄自 L79 的 `classify`

    单行的语义是「跳到那一行」⇒ 必须是代码行；区间的语义是「这件事在这段里」⇒ 起点空、
    终点是代码的那档属「区间报歪一行」，归 L79 的漂移档（A127 后半程），不该由硬门禁开火。
    """
    lines = lines_of(rel)
    if lines is None:
        return "unreadable"
    markers = COMMENT_MARKERS.get(pathlib.PurePath(rel).suffix, ())
    last = start if end is None else max(end, start)
    kinds = set()
    for lineno in range(max(start, 1), last + 1):
        if lineno > len(lines):
            kinds.add("out-of-range")
            continue
        raw = lines[lineno - 1].strip()
        kinds.add("blank" if not raw
                  else "comment" if markers and raw.startswith(markers) else "code")
    return "dead" if "code" not in kinds else "live"


def reference_state(ref, dirs, index, src):
    """一条引用 → `(结论, 唯一活路径)`；两系别各判一次，合并优先级就是口径

    优先级（先命中者说了算）与理由：

    1. 任一系指到唯一活文件 ⇒ `live`；
    2. 任一系说「名字有多条候选」⇒ `ambiguous`（欠账档）：引用指得到东西、只是不唯一，
       不该抢硬 0 档的红；
    3. 「父目录是被跟踪的命名空间、却没有这个文件」⇒ `dead_path`，**必须排在工件与示例档之前**
       （Temp 里的 HEAD 快照与历史副本会把产品路径写错洗白）；
    4. 最后才是不入仓工件（`scratch` / `scratch_missing`）与运行期示例路径（`runtime_ns`）。

    **只摘前缀、绝不加前缀**：索引取自 augmentor 内部，所以仓库根系别的写法要把 augmentor 前缀
    摘掉再问一次。反方向给裸名**加**前缀是没用的 —— 那个层级压根没有条目，它只会凭空造出
    「父目录被跟踪」的假证据（第二版就这么把 414 条示例路径全判成了死链）。
    """
    cands = [ref]
    if ref.startswith("augmentor/"):
        cands.append(ref[len("augmentor/"):])
    states = []
    for cand in cands:
        bucket, live = name_bucket(cand, index, src)
        if bucket is None:
            states.append(("live", live))
        elif bucket == "unresolved_source":
            # 父目录判据必须用 augmentor 相对坐标：`dirs` 就是从 augmentor 里算出来的。
            parent = "/".join(pathlib.PurePath(cand).parts[:-1])
            states.append(("runtime_ns" if parent not in dirs else "dead_path", None))
        else:
            states.append((bucket, None))
    for want in _LIVE_ORDER:
        for state, live in states:
            if state == want:
                return want, live
    # 走到这里说明 `name_bucket` 交回了一个本判据没登记过的桶名。宁可当场炸，也不许把未知
    # 桶并进某个棘轮档 —— 那正是「真 bug 被记成欠账、没有任何东西会红」的成形方式。
    raise AssertionError("未知桶：%r → %r" % (ref, states))


def audit(items, index, src, dirs, universe):
    """跑一遍普查 → `(档位计数, 可证失效行, 符号失效行)`；行里带上下文，失败信息直接点名

    计数键**就是桶名**（不带前缀）：`CEILING` 的键与这里的键一一对应，于是「桶名拼错」会当场
    红在 `test_every_ceiling_bucket_has_its_own_readout` 上，而不是让 `Counter` 的默认 0
    把硬 0 档伪造成绿。
    """
    counts = collections.Counter({"files_seen": len(items)})
    dead, sym_dead = [], []
    for rel, notes in items:
        for lineno, text in notes:
            for match in REF.finditer(text):
                ref, start, end = match.group(1), match.group(2), match.group(3)
                state, live = reference_state(ref, dirs, index, src)
                counts[state] += 1
                counts["refs_total"] += 1
                note = text.strip()[:64]
                if state == "dead_path":
                    dead.append((rel, lineno, match.group(0), "dead_path", note))
                elif start is not None and state == "live":
                    if span_state(live, int(start),
                                  int(end) if end else None) == "dead":
                        counts["dead_line"] += 1
                        dead.append((rel, lineno, match.group(0), "dead_line", note))
            if rel in NO_SYM_SCAN:
                continue
            seen = set()
            for pattern in (SYM_CALL, SYM_PAREN):
                for sym in pattern.finditer(text):
                    name = sym.group(1)
                    if name in seen:
                        continue
                    seen.add(name)
                    if declared_or_external(name, universe):
                        counts["symbol_live"] += 1
                    else:
                        counts["symbol_dead"] += 1
                        sym_dead.append((rel, lineno, name))
    return counts, dead, sym_dead


@functools.lru_cache(maxsize=1)
def census():
    """真身普查：整套判据只跑一次（全量 ~6 s），下面所有用例读同一份结果"""
    return audit(corpus(), full_index(), source_dirs(), tracked_dirs(),
                 symbol_universe())


def probe(items):
    """把合成语料并进真身普查：注入用例据此证明「判据会红」，而不是「数字恰好是 0」"""
    counts, rows, sym = _probe_raw(items)
    real_counts, real_rows, real_sym = census()
    return counts, rows, sym, real_counts, real_rows, real_sym


@functools.lru_cache(maxsize=1)
def _probe_raw(items):
    return audit(tuple(corpus()) + items, full_index(), source_dirs(),
                 tracked_dirs(), symbol_universe())


def ask_state(ref):
    """单条引用的结论档 —— selftest 的入口，也是「判据换方向时先红在这里」的那一格"""
    return reference_state(ref, tracked_dirs(), full_index(), source_dirs())[0]


def first_line_of(rel, want_blank):
    """现量取 `rel` 里第一个空白 / 非空白行的行号 —— 注入的靶子不许硬写行号（行号会搬家）"""
    lines = lines_of(rel)
    assert lines, rel
    for lineno, raw in enumerate(lines, 1):
        if (not raw.strip()) == want_blank:
            return lineno
    raise AssertionError("%s 里没有%s行" % (rel, "空白" if want_blank else "非空白"))


class TestCensusIsProven:
    """扫描面本身必须被证实在场，否则「全部死档为 0」可以靠「什么都没扫到」拿到"""

    def test_the_corpus_is_big_enough_to_be_the_claim(self):
        counts, _, _ = census()
        assert counts["files_seen"] >= FLOOR["files"], counts["files_seen"]
        assert counts["refs_total"] >= FLOOR["refs"], counts["refs_total"]
        assert counts["symbol_live"] >= FLOOR["symbol_live"], counts["symbol_live"]

    def test_the_index_is_not_python_only(self):
        """A184 的失明点就在这三格：YAML / TS / Markdown 都要能被解析成唯一活路径

        三个靶子是现量选的，不是随手写的：README 这个名字在仓里有**多份**（根上一份、
        子项目各一份），它进不了这一格 —— 它该进的是歧义档，硬写在这里只会让用例红在
        「我以为它唯一」上。
        """
        index, src = full_index(), source_dirs()
        for rel in ("config.yaml", "web/src/services/api.ts", "docs/ARCHITECTURE.md"):
            bucket, live = name_bucket(rel, index, src)
            assert bucket is None and live == rel, (rel, bucket, live)

    def test_the_rulers_own_regex_still_cannot_see_a_yaml_line_ref(self):
        """钉住「为什么需要这一支」：L79 的行号正则写死 Python 后缀，YAML 引用它读不到

        哪天 L79 把正则放宽了，这一格会红 ⇒ 届时该做的是**合并两支守卫**，而不是把这条断言
        删了事（红在这里是好消息，不是噪声）。
        """
        token = tick("config.yaml:", str(first_line_of("config.yaml", True)))
        assert LINE_REF.fullmatch(token) is None, token
        # 而本守卫读得到：同一个名字喂进判据，落点是那两份 YAML 里唯一的出厂模板
        state, live = reference_state("config.yaml", tracked_dirs(),
                                      full_index(), source_dirs())
        assert (state, live) == ("live", "config.yaml")

    def test_every_ceiling_bucket_is_a_bucket_the_census_can_produce(self):
        """桶名拼错 = 硬 0 永远绿：`CEILING` 的每个键都必须在普查的桶词汇表里

        这一格不吃读数，只吃词表 —— 因为 `Counter` 对不存在的键默认给 0，「拼错的桶名」与
        「真的 clean」在读数上完全同形。词汇表 `BUCKETS` 是判据能产出的全部结论名，写死在
        模块顶部，`reference_state` 的返回集变了就必须同时改它。
        """
        assert set(CEILING) <= BUCKETS, set(CEILING) - BUCKETS
        # 好引用与「symbol_live」不进 CEILING；其余每一档都必须钉一个常数
        assert BUCKETS - set(CEILING) == {"live", "symbol_live"}, BUCKETS - set(CEILING)
        # 合并优先级的表不许写出词汇表之外的档名
        assert set(_LIVE_ORDER) <= BUCKETS, set(_LIVE_ORDER) - BUCKETS


class TestHardZero:
    """可证失效 = 0：三档各自硬判，失败信息点名到文件与行"""

    @staticmethod
    def _named(rows, kind):
        return "\n".join("  %s:%d %s | %s" % (r[0], r[1], r[2], r[4])
                         for r in rows if r[3] == kind)

    def test_no_reference_points_at_a_file_that_should_exist(self):
        counts, rows, _ = census()
        assert counts["dead_path"] == CEILING["dead_path"], self._named(rows, "dead_path")

    def test_no_line_reference_lands_on_a_non_code_span(self):
        counts, rows, _ = census()
        assert counts["dead_line"] == CEILING["dead_line"], self._named(rows, "dead_line")

    def test_no_symbol_claim_points_at_an_absent_name(self):
        counts, _, rows = census()
        assert counts["symbol_dead"] == CEILING["symbol_dead"], \
            "\n".join("  %s:%d %s" % r for r in rows)


class TestRatchetsAreExact:
    """四档欠账/工件读数精确相等（只降不升；改进也得同时改常数并留一句「谁清的」）"""

    @pytest.mark.parametrize("bucket", ["ambiguous", "runtime_ns", "scratch",
                                        "scratch_missing"])
    def test_bucket_matches_its_ceiling(self, bucket):
        counts, _, _ = census()
        assert counts[bucket] == CEILING[bucket], \
            "%s 现量 %d，常数是 %d" % (bucket, counts[bucket], CEILING[bucket])

    def test_the_good_references_outweigh_the_debt(self):
        """好引用必须压过全部欠账：这条比例是「普查在量案面而不是在量噪声」的读数"""
        counts, _, _ = census()
        debt = sum(counts[b] for b in ("ambiguous", "runtime_ns", "scratch",
                                       "scratch_missing"))
        assert counts["live"] > debt, (counts["live"], debt)


class TestRedCapability:
    """判据必须能红：三种合成失效喂进同一条判据，要求逐档命中、且真身一格不多"""

    def _synthetic(self):
        # 靶子行号现量：硬写的行号下一轮就不是空行了，那时「红能力」会退化成永远绿
        blank = first_line_of("config.yaml", True)
        code = first_line_of("config.yaml", False)
        self.control = tick("config.yaml:", str(code))
        notes = ((1, "broken link " + tick("docs/NOT_A_REAL_FILE_", "a184.md")),
                 (2, "blank line " + tick("config.yaml:", str(blank))),
                 (3, "dead claim " + tick("no_such_symbol_", "a184", "()")),
                 (4, "control " + self.control))
        return (("tests/unit/_a184_injected_probe.py", notes),)

    def test_injection_turns_each_hard_bucket_red_exactly_once(self):
        counts, rows, sym, real, real_rows, real_sym = probe(self._synthetic())
        assert counts["dead_path"] - real["dead_path"] == 1, rows
        assert counts["dead_line"] - real["dead_line"] == 1, rows
        assert len([r for r in rows if r[3] == "dead_path"]) == 1, rows
        assert len([r for r in rows if r[3] == "dead_line"]) == 1, rows
        assert len(rows) == len(real_rows) + 2, rows
        assert len(sym) - len(real_sym) == 1, sym
        # 两条 YAML 引用都指得到那个文件（一条只是行是空的）⇒ live 档各 +1，
        # 而 symbol_live 一格不动 ⇒ 「红」来自判据，不是把一切判死。
        assert counts["live"] - real["live"] == 2, counts["live"]
        assert counts["symbol_live"] == real["symbol_live"]
        assert not any(r[2] == self.control for r in rows), self.control

    def test_the_injection_leaves_the_ratchet_buckets_untouched(self):
        counts, _, _, real, _, _ = probe(self._synthetic())
        for bucket in ("ambiguous", "runtime_ns", "scratch", "scratch_missing"):
            assert counts[bucket] == real[bucket], bucket


class TestRulerProvesItself:
    """判据自己得认得已知答案 —— 已知形状逐格验；方向写反的判据永远红（承 L98 的教训）"""

    def test_broken_link_inside_a_tracked_namespace_is_dead(self):
        assert ask_state("docs/NOT_A_REAL_FILE_xyz.md") == "dead_path"

    def test_the_historically_broken_config_pointer_is_dead(self):
        # 账本曾七次引用过一个从未存在的配置文档；本轮把它摘成散文，指针本身仍是坏链接
        assert ask_state("docs/CONFIG.md") == "dead_path"

    def test_example_path_in_a_runtime_namespace_is_not_dead(self):
        assert ask_state("data/xxx.json") == "runtime_ns"

    def test_a_live_augmentor_relative_path_is_recognised(self):
        assert ask_state("api/deps.py") == "live"

    def test_the_same_file_written_from_the_repo_root_is_also_live(self):
        assert ask_state("augmentor/api/deps.py") == "live"

    def test_a_temp_draft_counts_as_an_artifact_not_a_dead_link(self):
        assert ask_state("Temp/nope/probe_l99.py") in ("scratch", "scratch_missing")

    def test_span_judgement_follows_the_target_files_language(self):
        assert span_state("README.md", 1, 1) == "live"          # Markdown 的 `#` 是内容
        assert span_state("README.md", 99999, 99999) == "dead"  # 越界不是代码
