# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置键读者普查：把「写了没人读」从一次性探针升成仓内可复跑的守卫（L217）

**为什么要入仓**：B5 的原始尺子住在 `Temp/` 里，是一次性探针——而 `Temp/` 与 `data/`
那两堆探针底料在当前工作树压根不存在（本轮实测），于是 B5 那串读数今天**没有任何东西能复跑**。
「消费能力 100%」这种主张如果只由一份跑不了的草稿撑着，它就和没测过一样。本文件把同一把尺子
重写成仓内守卫：键清单从 `AppConfig` 实推、扫描面从磁盘现取，一次全量门禁就跑一次。

**三档归档 + 一个撞车桶**（每只 schema 键必须落在且只落在三档中的一档）：
1. `_AUTO_READ`（现量，不写死）：AST 在产品树里找到读者——直接链 `config.<节>.<键>`，
   或经别名（`vec = self.config.vector` 之后 `vec.enabled`、`def f(cfg: VectorConfig)` 之后
   `cfg.backend`）；
2. `_LOADER_CONSUMED`（点名 + 引用行）：读者真的存在，但形状是**鸭子形参**，静态属性链绑不到。
   目前只有 `logging` 那三键：加载器把整节交给 `apply_logging_config(settings)`，函数体读
   `settings.level` / `settings.file` / `settings.format`，形参注解是 `Any`。每条目带
   `(文件, 行号, 该行的标识符)`，由 `TestArchiveCitations` 逐条验真；
3. `_UNREAD`（逐键写理由）：产品树里零读者。这一档是本守卫的**棘轮**：新键没人归档就红，
   归档的键接上读者也红（逼着清账的人改档），所以它不是一堆借口的堆放处。

第四个桶 `_NAME_COLLISIONS` 装的是**不算键**的同名属性：`self.sampler.analyze_coverage`
里的 `sampler` 是管道的 `ActiveSampler` 实例，与配置节 `sampler` 撞名。按名字判的尺子会把它
算成读者，所以它单列一档，并由 `TestNameCollisions` 双向钉住（撞车还在 ⇒ 名单不许空；
撞车消失 ⇒ 名单不许还留着）。

**尺子看不见的东西（写下来，不让它假装成 0）**：
- 鸭子形参与 `**asdict(section)` 展开——所以第 2 档存在；
- 回显面：`api/routes/config.py` 只把值印给客户端，属 A138 的 ② 档，刻意不算读者；
- 校验面：`augmentor/config_validator.py` 与 `augmentor/config.py` 本身（加载器）也不在扫描面里，
  否则「键被自己的校验器读了一遍」会把所有键都算成有读者。
  注意这与 A84 的 `unread_key_messages` 是**两个轴**：那条查「YAML 里写了但 schema 没有的键」
  （拼错键名），本守卫查「schema 里有但产品代码不读的键」，两者互不覆盖。

**正控制（防止尺子自己瞎掉还全绿）**：`_MUST_BE_ALIAS_ONLY` 点名一批「只能经别名才看得见读者」
的键——把它们接进产品树的写法全都是 `cfg = p.config.<节>` 之后 `cfg.<键>`。把别名解析去掉，
这批键就会掉进零读者，第 1 档与实推清单当场不匹配，同时 `TestRulerIsNotBlind` 直接红。
`_MUST_BE_DIRECT` 反向钉住纯链形状，两臂合起来才说明扫描面与解析臂都真的在跑。
"""

import ast
import dataclasses
import functools
from dataclasses import MISSING, fields
from pathlib import Path

import pytest

from augmentor.config import AppConfig

ROOT = Path(__file__).resolve().parents[2]

#: 扫描面入口：SDK 包、API 层、CLI 入口
SCAN_BASES = (ROOT / "augmentor", ROOT / "api")
SCAN_FILES = (ROOT / "cli.py",)

#: 不在扫描面里的文件：加载器与校验器读自己的字段不算「产品读者」，
#: 配置回显端点属 A138 ② 档（可见 ≠ 被消费）
SCAN_EXCLUDE_FILES = {
    ROOT / "augmentor" / "config.py",
    ROOT / "augmentor" / "config_validator.py",
    ROOT / "api" / "routes" / "config.py",
}

#: 目录黑名单：测试、草稿、依赖与前端（前端有另一套消费面，不在这把尺子的轴上）
SCAN_EXCLUDE_PARTS = {"tests", "Temp", "data", "web", "node_modules", ".git",
                      "docs", "archive", "__pycache__", "bak"}

#: 只能经别名才看得见的读者（别名臂的正控制）
_MUST_BE_ALIAS_ONLY = (
    "multilingual.supported_langs",
    "multilingual.translate_batch_size",
    "multimodal.image_extensions",
    "rag.default_format",
    "vector.backend",
    "vector.dimension",
)

#: 纯属性链就该抓到的读者（扫描面的正控制）
_MUST_BE_DIRECT = (
    "versioning.storage_dir",
    "web.data_roots",
    "augmentation.num_threads",
)

#: 第 2 档：读者存在但静态绑不到；每条 `(节.键, 文件, 行号, 该行应出现的标识符)`
_LOADER_CONSUMED = (
    ("logging.level", "augmentor/logging_setup.py", 187, "settings.level"),
    ("logging.file", "augmentor/logging_setup.py", 188, "settings.file"),
    ("logging.format", "augmentor/logging_setup.py", 189, "settings.format"),
)

#: 第 3 档：产品树零读者，逐键签字。理由一句话说清「等价的旋钮现在住在哪」，
#: 否则下一轮无从判断该接线还是该删键。
_UNREAD = {
    "active_learning.enabled": "选样器由管道无条件 `ActiveSampler()` 构造，不看本节",
    "active_learning.strategy": "策略名只被 CLI 参数与 `ActiveSampler` 形参拿走，配置侧无读者",
    "active_learning.batch_size": "同上：批尺寸住在请求参数，不在本节",
    "active_learning.max_iterations": "迭代上限在产品侧压根没有对应入口",
    "benchmark.enabled": "基准端点 `/api/quality/benchmark` 无条件可用，不看闸",
    "context.enabled": "`ContextAugmentor` 由管道无条件构造（只读 `num_turns`），这只闸是装饰",
    "evaluation.enabled": "评估端点无条件可用，不看闸",
    "evaluation.metrics": "指标清单由 `/api/dataset/evaluate` 的请求体 `metrics` 提供",
    "evaluation.reference_field": "参考侧字段由请求体 `reference_field` 提供，默认写在同一条请求模型里",
    "expander.enabled": "扩展器由管道无条件 `DomainExpander(backend)` 构造，不看本节",
    "expander.strategies": "策略清单在产品侧没有读者（扩展请求走参数）",
    "frameworks.enabled": "框架导出走请求参数，本节整体无读者",
    "frameworks.frameworks": "框架名由调用方在请求里给，与 `export.formats` 同一口径",
    "multimodal.enabled": "扫描与格式端点无条件可用，不看闸（L214 只接了两只 extensions）",
    "rag.enabled": "RAG 端点无条件可用，不看闸（L214 接了 chunk_size/chunk_overlap/default_format）",
    "sampler.enabled": "采样端点走请求参数，闸从未被读",
    "sampler.dimensions": "维度清单由 `/api/dataset/sample` 的请求参数提供",
    "tracker.enabled": "追踪器由管道无条件 `ExperimentTracker()` 构造，不看本节",
    "tracker.metrics": "指标名由调用方在请求里给，配置侧无读者",
    "visualization.types": "可视化类型清单走请求参数；B224 已立案（L154 普查同款）",
}


#: 节名与产品代码里的普通属性名撞车的地方：`self.sampler` 是管道的 `ActiveSampler`
#: 实例而不是配置节，所以 `sampler.analyze_coverage` 会被按名字算成读者。这些键**不**
#: 算进任何一档，由 `TestNameCollisions` 单独钉住——写死成名单是为了让「撞车消失了」
#: 也能被发现（那时该把条目摘掉，而不是让它悄悄飘着）。
_NAME_COLLISIONS = {"sampler.analyze_coverage"}


def _attr_chain(node: ast.Attribute) -> list:
    """Attribute 链上的属性名序列，从链根往尾；链根本身（Name/Call/Subscript）不计"""
    out = []
    cur: ast.AST = node
    while isinstance(cur, ast.Attribute):
        out.append(cur.attr)
        cur = cur.value
    return list(reversed(out))


def _chain_root(node: ast.Attribute) -> ast.AST:
    cur: ast.AST = node
    while isinstance(cur, ast.Attribute):
        cur = cur.value
    return cur


def _config_sections() -> dict:
    """`AppConfig` 里那些「值是一个配置 dataclass」的字段 ⇒ {节名: [键名]}"""
    out = {}
    for f in fields(AppConfig):
        if f.default_factory is MISSING or f.default_factory is None:
            continue
        instance = f.default_factory()
        if not dataclasses.is_dataclass(instance):
            continue
        out[f.name] = [ff.name for ff in fields(instance)]
    return out


_SECTIONS = _config_sections()
_SECTION_NAMES = set(_SECTIONS)
_CLASS_OF_SECTION = {
    f.default_factory.__name__: f.name
    for f in fields(AppConfig)
    if f.default_factory is not MISSING and f.default_factory is not None
    and dataclasses.is_dataclass(f.default_factory())
}


def _aliases_in(scope: ast.AST, own_nodes: list) -> dict:
    """作用域内「名字 = 某节对象」的不动点闭包

    三类来源：赋值右值是 `...<节>` 的属性链（链根任意，含 `get_pipeline().config.rag`
    这种 Call 根）；形参注解直接写节类型（`def f(cfg: VectorConfig)`）；别名链 `x = y`。
    只收本作用域自己的语句（嵌套函数/类里的赋值不算），否则函数局部的别名会泄漏到
    同文件其他函数，把「其实没人读」洗成「有读者」。
    """
    aliases: dict = {}
    if isinstance(scope, (ast.FunctionDef, ast.AsyncFunctionDef)):
        args = scope.args
        for arg in args.posonlyargs + args.args + args.kwonlyargs:
            ann = arg.annotation
            name = None
            if isinstance(ann, ast.Name):
                name = ann.id
            elif isinstance(ann, ast.Attribute) and _attr_chain(ann):
                name = _attr_chain(ann)[-1]
            if name in _CLASS_OF_SECTION:
                aliases[arg.arg] = _CLASS_OF_SECTION[name]

    assigns = [n for n in own_nodes
               if isinstance(n, (ast.Assign, ast.AnnAssign)) and n.value is not None]
    for _ in range(6):
        changed = False
        for node in assigns:
            targets = node.targets if isinstance(node, ast.Assign) else [node.target]
            names = [t.id for t in targets if isinstance(t, ast.Name)]
            section = None
            if isinstance(node.value, ast.Attribute):
                chain = _attr_chain(node.value)
                if chain and chain[-1] in _SECTION_NAMES:
                    section = chain[-1]
            elif isinstance(node.value, ast.Name) and node.value.id in aliases:
                section = aliases[node.value.id]
            for name in names:
                if section and aliases.get(name) != section:
                    aliases[name] = section
                    changed = True
        if not changed:
            break
    return aliases


def _own_nodes(scope: ast.AST) -> list:
    """本作用域的节点，遇嵌套函数/类不下钻（其内部由 `walk` 另作为作用域处理）"""
    out = []
    stack = list(getattr(scope, "body", []))
    while stack:
        node = stack.pop()
        out.append(node)
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            continue
        stack.extend(ast.iter_child_nodes(node))
    return out


def _scan_file(path: Path) -> tuple:
    """返回 (直接链命中, 别名命中)，元素是 `节.键`"""
    try:
        tree = ast.parse(path.read_text(encoding="utf-8", errors="replace"))
    except (SyntaxError, ValueError):
        return set(), set()
    direct, via_alias = set(), set()
    scopes = [n for n in ast.walk(tree)
              if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef, ast.Module))]
    for scope in scopes:
        aliases = _aliases_in(scope, _own_nodes(scope))
        for node in ast.walk(scope):
            if not isinstance(node, ast.Attribute):
                continue
            chain = _attr_chain(node)
            for i in range(len(chain) - 1):
                if chain[i] in _SECTION_NAMES:
                    direct.add(f"{chain[i]}.{chain[i + 1]}")
            root = _chain_root(node)
            if isinstance(root, ast.Name) and root.id in aliases:
                via_alias.add(f"{aliases[root.id]}.{node.attr}")
    return direct, via_alias


def _product_files() -> list:
    files = [p for p in SCAN_FILES if p.exists()]
    for base in SCAN_BASES:
        for p in sorted(base.rglob("*.py")):
            if set(p.relative_to(ROOT).parts) & SCAN_EXCLUDE_PARTS:
                continue
            if p in SCAN_EXCLUDE_FILES:
                continue
            files.append(p)
    return files


@functools.lru_cache(maxsize=1)
def _census() -> dict:
    """`{"direct": set, "alias": set, "readers": {key: [文件, ...]}, "files": int}`"""
    direct_all, alias_all = set(), set()
    readers: dict = {}
    files = _product_files()
    for path in files:
        direct, alias = _scan_file(path)
        relative = str(path.relative_to(ROOT)).replace("\\", "/")
        for key in direct | alias:
            readers.setdefault(key, []).append(relative)
        direct_all |= direct
        alias_all |= alias
    return {
        "direct": direct_all,
        "alias": alias_all,
        "readers": readers,
        "files": len(files),
    }


def _all_keys() -> set:
    return {f"{section}.{key}" for section, keys in _SECTIONS.items() for key in keys}


class TestRulerIsNotBlind:
    """尺子自己要能看见已知的读者，否则「全部落进第 3 档」也能骗过归档判据"""

    def test_product_surface_is_not_empty(self):
        assert _census()["files"] >= 120, "扫描面缩水到不足以代表产品树（目录黑名单改坏了？）"

    @pytest.mark.parametrize("key", _MUST_BE_DIRECT)
    def test_direct_chains_are_seen(self, key):
        assert key in _census()["direct"], f"{key} 有正牌属性链读者，尺子却没看见"

    @pytest.mark.parametrize("key", _MUST_BE_ALIAS_ONLY)
    def test_alias_readers_are_seen(self, key):
        assert key in _census()["alias"], (
            f"{key} 在产品树里经别名被读（`cfg = p.config.<节>` 之后 `cfg.<键>`），"
            f"尺子的别名臂没跑起来"
        )

    def test_readers_are_attributed_to_a_file(self):
        for key in _MUST_BE_ALIAS_ONLY:
            assert _census()["readers"][key], f"{key} 判为有读者却报不出文件"


class TestEveryKeyIsArchived:
    """69 只键必须逐键落进三档之一，且不重不漏"""

    def test_the_three_tiers_partition_the_schema(self):
        census = _census()
        read = set(census["direct"]) | set(census["alias"])
        loader = {key for key, _f, _l, _i in _LOADER_CONSUMED}
        universe = _all_keys()
        assert read.isdisjoint(loader), "同一只键不许既算读者又算加载器装配"
        assert set(_UNREAD).isdisjoint(read), (
            f"归档说没人读，尺子却找到了读者: {sorted(set(_UNREAD) & read)}"
        )
        missing = universe - read - loader - set(_UNREAD)
        assert not missing, f"这些键没归档（新键不许无人签字）: {sorted(missing)}"
        ghosts = (read | loader | set(_UNREAD)) - universe - _NAME_COLLISIONS
        assert not ghosts, f"名单里点了并不存在的键: {sorted(ghosts)}"

    def test_unread_tier_is_exactly_the_archived_list(self):
        """棘轮的另一侧：接上读者的键必须从名单里挪走，不许留成僵尸条目"""
        census = _census()
        read = set(census["direct"]) | set(census["alias"])
        loader = {key for key, _f, _l, _i in _LOADER_CONSUMED}
        actual_unread = {k for k in _all_keys() if k not in read and k not in loader}
        assert actual_unread == set(_UNREAD), (
            f"零读者名单与现量不符。\n"
            f"  新掉进零读者（读者被移走）: {sorted(actual_unread - set(_UNREAD))}\n"
            f"  已接上读者却没改档: {sorted(set(_UNREAD) - actual_unread)}"
        )

    def test_unread_reasons_are_not_placeholders(self):
        for key, reason in _UNREAD.items():
            assert len(reason) >= 12, f"{key} 的归档理由太薄，看不出等价旋钮住在哪"


class TestArchiveCitations:
    """第 2 档的引用行必须真的落在那一行上，并且那一行确实读的是那只键"""

    @pytest.mark.parametrize("key,path,line,identifier", _LOADER_CONSUMED)
    def test_citation_points_at_the_reading_line(self, key, path, line, identifier):
        target = ROOT / path
        assert target.exists(), f"{key} 的引用文件不存在: {path}"
        lines = target.read_text(encoding="utf-8").splitlines()
        assert 0 < line <= len(lines), f"{key} 的引用行号 {line} 越界（文件只有 {len(lines)} 行）"
        assert identifier in lines[line - 1], (
            f"{key} 的引用落在 {path}:{line} 却是 `{lines[line - 1].strip()}`"
        )

    def test_logging_section_is_the_loader_assembled_one(self):
        """第 2 档只有 logging 一族不是偶然：装配发生在加载器里，鸭子形参绑不到节名"""
        sections = {key.split(".")[0] for key, _p, _l, _i in _LOADER_CONSUMED}
        assert sections == {"logging"}, f"新增第 2 档条目要先证明它是同一形状: {sorted(sections)}"


class TestNameCollisions:
    """撞车名单本身要可证伪：条目既真实存在，又确实不属于 schema 键集"""

    def test_collisions_are_actually_seen_by_the_ruler(self):
        census = _census()
        seen = census["direct"] | census["alias"]
        assert _NAME_COLLISIONS <= seen, (
            f"这些撞车名今天不再出现，应该从名单里摘掉: {sorted(_NAME_COLLISIONS - seen)}"
        )

    def test_collisions_are_not_schema_keys(self):
        assert not _NAME_COLLISIONS & _all_keys(), "撞车名单里的键其实存在于 schema ⇒ 它不是撞车，是漏归档"
