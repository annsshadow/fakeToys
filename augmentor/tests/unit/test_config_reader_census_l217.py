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
   L218 起这一档**每条理由再配一条机器可验的证据**（`_UNREAD_KNOB`，五种形状：构造形参 /
   方法形参 / 明说那里没有 / 请求模型字段 / 无同名旋钮），由 `TestKnobClaimsAreFalsifiable`
   拿 `inspect.signature` 与 `api/routes` 的 AST 逐条验真。由来很硬：L217 第一版的 20 条理由里
   **有 8 条是假的**（宣称存在并不存在的请求参数或构造形参），散文写得好看不等于说得对。

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
import importlib
import inspect
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
    "evaluation.generated_field",
    "evaluation.metrics",
    "evaluation.reference_field",
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
#: **L277 重量（行号 +3，判据一字未动）**：本轮在 `_open_file_handler` 的 `except` 之上补了
#: 三行注释（NUL 路径 ⇒ `ValueError` 也要折成 `DataValidationError`），把 `apply_logging_config`
#: 整段往下推了三格。原读数 187/188/189 现在落进 A106 那段快路注释里，红的是
#: `assert 'settings.file' in '    signature = ('` 这一类「引用落在隔壁一行」的形状，
#: 不是「读者没了」——三行标识符现在依次在 190/191/192，仍是同一个 `signature` 元组里
#: 对三只键的无条件读取（比 `if "level" in keys` 那几行更硬：那三行是条件消费）。
#: **L289 再重量（行号 +20，判据仍一字未动）**：A393 把 `apply_logging_config` 拆成「锁壳 +
#: `_apply_logging_config_locked`」，壳的 docstring 记着改前 16 支并发的实测读数（7/14/11 条
#: handler、6/13/10 行），把装配体整体顶下二十格。红形与 L277 那一次同一款——
#: `assert 'settings.level' in ...` 落在别的行上，读者一处未少；三行标识符现量 210/211/212。
#: 重钉只跟着物理行号走，判据形状与标识符一个字未松；再插行还会再红，这正是本档的用途。
_LOADER_CONSUMED = (
    ("logging.level", "augmentor/logging_setup.py", 210, "settings.level"),
    ("logging.file", "augmentor/logging_setup.py", 211, "settings.file"),
    ("logging.format", "augmentor/logging_setup.py", 212, "settings.format"),
)

#: 第 3 档：产品树零读者，逐键签字。
#:
#: **理由第一版有 8 条是假的**（L217 记账后 L218 复核实码时翻出来的）：它们声称「等价旋钮
#: 走请求参数 / 是某构造形参」，而 `api/routes/` 里压根没有那些字段、`ActiveSampler.__init__`
#: 也没有任何形参。散文骗过人眼，所以这一档从 L218 起**每键再挂一条机器可验的证据**
#: （见 `_UNREAD_KNOB`），理由与证据不一致就红。
_UNREAD = {
    "active_learning.enabled": "主动学习循环在产品侧连实例都没建过（只有包导出），这只闸无人问",
    "active_learning.strategy": "同名旋钮在 `ActiveLearningLoop.__init__`，但那个类没有产品调用者",
    "active_learning.batch_size": "同名旋钮同在 `ActiveLearningLoop.__init__`，同样没有产品调用者",
    "active_learning.max_iterations": "轮数旋钮是 `run` 的方法形参 `iterations`，且默认 None⇒1 轮（与出厂 10 不同，接线即行为变更）",
    "benchmark.enabled": "基准端点 `/api/quality/benchmark` 无条件可用，不看闸",
    "context.enabled": "`ContextAugmentor` 由管道无条件构造（只读 `num_turns`），这只闸是装饰",
    "evaluation.enabled": "评估端点无条件可用，不看闸",
    "expander.enabled": "扩展器由管道无条件 `DomainExpander(backend)` 构造，不看本节",
    "expander.strategies": "策略清单是 `batch_expand` 的方法形参，而该方法无产品调用者（`self.expander` 构造之后从未被用，见 A273）",
    "frameworks.enabled": "框架导出在产品侧没有入口，这只闸无处可判",
    "frameworks.frameworks": "两个集成类的构造器只收 `text_key`/`answer_key`，没有清单型入口，且产品侧零实例化",
    "multimodal.enabled": "扫描与格式端点无条件可用，不看闸（L214 只接了两只 extensions）",
    "rag.enabled": "RAG 端点无条件可用，不看闸（L214 接了 chunk_size/chunk_overlap/default_format）",
    "sampler.enabled": "采样端点无条件可用，本节闸从未被读（与 benchmark/evaluation 同形状）",
    "sampler.dimensions": "四个分布硬写在 `analyze_coverage` 体内，它只收 `items`；端点侧也没有同名清单",
    "tracker.enabled": "追踪器由管道无条件 `ExperimentTracker()` 构造，不看本节",
    "tracker.metrics": "指标名只有逐条运行时入参 `log_metric(metric_name, value)`，没有清单型入口（`self.tracker` 构造后也从未被用，见 A273）",
    "visualization.types": "出图清单硬编码在 `DataVisualizer` 的方法集里，`visualize_dataset` 只收 `input_file`/`output_dir`；B224/A267 已在案",
}

#: 每只零读者键的「等价旋钮现在住在哪」，必须是下面五种证据之一（由
#: `TestKnobClaimsAreFalsifiable` 机器验真，不许留空、不许凭散文过关）：
#: - `("ctor", "模块:类.__init__", 形参名)`：同名形参**在**该构造器签名里；
#: - `("method", "模块:类.方法", 形参名)`：同名形参**在**该方法签名里；
#: - `("absent", "模块:类.可调用", 形参名)`：明说那里**没有**这个形参（签名里不许出现该名）；
#: - `("request", "api/routes/x.py", 字段名)`：该路由文件的模型里**真有**这个类注解字段；
#: - `("none", None, None)`：产品侧不存在任何同名旋钮（只许用于「闸」这类无旋钮可接的键）。
#: 目标写法与 `_LOADER_CONSUMED` 同规：路径相对仓库根，符号名逐字可查。
_UNREAD_KNOB = {
    "active_learning.enabled": ("none", None, None),
    "active_learning.strategy": ("ctor", "augmentor.active_learning:ActiveLearningLoop.__init__", "strategy"),
    "active_learning.batch_size": ("ctor", "augmentor.active_learning:ActiveLearningLoop.__init__", "batch_size"),
    "active_learning.max_iterations": ("method", "augmentor.active_learning:ActiveLearningLoop.run", "iterations"),
    "benchmark.enabled": ("none", None, None),
    "context.enabled": ("none", None, None),
    "evaluation.enabled": ("none", None, None),
    "expander.enabled": ("none", None, None),
    "expander.strategies": ("method", "augmentor.expander:DomainExpander.batch_expand", "strategies"),
    "frameworks.enabled": ("none", None, None),
    "frameworks.frameworks": ("absent", "augmentor.frameworks.langchain:LangChainIntegration.__init__", "frameworks"),
    "multimodal.enabled": ("none", None, None),
    "rag.enabled": ("none", None, None),
    "sampler.enabled": ("none", None, None),
    "sampler.dimensions": ("absent", "augmentor.sampler:ActiveSampler.analyze_coverage", "dimensions"),
    "tracker.enabled": ("none", None, None),
    "tracker.metrics": ("absent", "augmentor.tracker:ExperimentTracker.__init__", "metrics"),
    "visualization.types": ("absent", "augmentor.pipeline:AugmentorPipeline.visualize_dataset", "types"),
}

#: 散文与证据的一致性判据：理由里点了「请求」就必须真是 request 档，点了「形参」就必须
#: 真是 ctor/method 档。这正是 L217 那 8 条假理由共同的形状——它们都宣称有一个并不存在
#: 的请求参数或形参。两条窄规则够用，不做「理由写得好不好」的判决（那种断言已删过一次）。
_PROSE_CLAIMS = (("请求", frozenset({"request"})),
                 ("形参", frozenset({"ctor", "method"})))

#: 证据形状的全集：不在全集里的 kind 一律红，防止「新造一档但没人验真」
_KNOB_KINDS = frozenset({"ctor", "method", "absent", "request", "none"})


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


@functools.lru_cache(maxsize=1)
def _request_fields() -> dict:
    """`api/routes/*.py` 里模型类的**类注解字段名** ⇒ {文件名: 字段集合}

    只认 `ClassDef` 体内的 `AnnAssign`（pydantic 模型就是这么声明字段的），不认函数体内的
    局部变量——否则「某个函数里恰好有个同名局部量」会被算成请求侧旋钮。回显面
    `api/routes/config.py` 与读者普查同口径排除在外。
    """
    out: dict = {}
    for path in sorted((ROOT / "api" / "routes").glob("*.py")):
        if path in SCAN_EXCLUDE_FILES:
            continue
        tree = ast.parse(path.read_text(encoding="utf-8"))
        names: set = set()
        for node in ast.walk(tree):
            if isinstance(node, ast.ClassDef):
                for stmt in node.body:
                    if isinstance(stmt, ast.AnnAssign) and isinstance(stmt.target, ast.Name):
                        names.add(stmt.target.id)
        if names:
            out[path.name] = names
    return out


@functools.lru_cache(maxsize=64)
def _callable_params(spec: str) -> set:
    """把 `模块:类.可调用` 解成形参名集合（真 import + `inspect.signature`，不读源码文本）

    读源码文本的写法挡不住「签名被折行 / 形参在 `*args` 之后」这类形状，而这一档的判据是
    「同名旋钮到底在不在」，所以要问解释器而不是问正则。
    """
    module_name, _, rest = spec.partition(":")
    target = importlib.import_module(module_name)
    for part in rest.split("."):
        target = getattr(target, part)
    return set(inspect.signature(target).parameters)


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
    """70 只键必须逐键落进三档之一，且不重不漏"""

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


class TestKnobClaimsAreFalsifiable:
    """第 3 档每只键的「等价旋钮住在哪」必须机器可验，不许由散文自证

    立这一档的由来：L217 给 20 只零读者键各写了一条理由，其中 **8 条是假的**——它们说
    存在某个请求参数或构造形参，而实码里压根没有。人眼读散文读不出这件事，所以 L218 起
    理由与证据成对存放（`_UNREAD` 配 `_UNREAD_KNOB`），证据由解释器（`inspect.signature`）
    和 AST（`api/routes` 的模型字段）验真，两边不一致或证据说谎就红。

    取循环而不是参数化：这一档的失败面是「一张逐键对账表」（L220 收走
    `evaluation.reference_field` 之后现量 18 行），一次跑完并报出**全部**
    不一致条目，比逐键一支测试更贴近用途（少一条键就少一次对账），也不给参数化棘轮添新形状。
    """

    def test_every_unread_key_carries_an_evidence_entry(self):
        assert set(_UNREAD_KNOB) == set(_UNREAD), (
            f"零读者名单与证据表不配对。\n"
            f"  缺证据: {sorted(set(_UNREAD) - set(_UNREAD_KNOB))}\n"
            f"  证据多余（键已接上读者）: {sorted(set(_UNREAD_KNOB) - set(_UNREAD))}"
        )

    def test_evidence_shapes_are_one_of_the_five_kinds(self):
        bad = []
        for key, entry in sorted(_UNREAD_KNOB.items()):
            if not isinstance(entry, tuple) or len(entry) != 3:
                bad.append(f"{key}: 证据必须是三元组，实为 {entry!r}")
                continue
            kind, spec, name = entry
            if kind not in _KNOB_KINDS:
                bad.append(f"{key}: 未知证据形状 {kind!r}（可用: {sorted(_KNOB_KINDS)}）")
            elif kind == "none":
                if (spec, name) != (None, None):
                    bad.append(f"{key}: none 档不许挂目标，实为 {spec!r}/{name!r}")
            elif not (isinstance(spec, str) and spec and isinstance(name, str) and name):
                bad.append(f"{key}: {kind} 档必须同时给出目标与名字")
        assert not bad, "证据表形状不合法:\n  " + "\n  ".join(bad)

    def test_none_tier_is_only_for_switches_without_a_knob(self):
        """`none` 是「没有任何同名旋钮可接」的强主张，只许用于闸类键

        否则它会变成假理由的新堆放处——写不动就往 `none` 一扔。
        """
        none_keys = sorted(k for k, e in _UNREAD_KNOB.items() if e[0] == "none")
        assert none_keys, "证据表里一个 `none` 都没有 ⇒ 这条判据在空转"
        assert all(k.endswith(".enabled") for k in none_keys), (
            f"这些非闸键声称没有任何等价旋钮，要去 `TestKnobClaimsAreFalsifiable` 的真档里挂证据: "
            f"{[k for k in none_keys if not k.endswith('.enabled')]}"
        )

    def test_ctor_and_method_params_really_are_in_the_signature(self):
        bad = []
        for key, (kind, spec, name) in sorted(_UNREAD_KNOB.items()):
            if kind not in {"ctor", "method"}:
                continue
            params = _callable_params(spec)
            if name not in params:
                bad.append(f"{key}: 声称 `{spec}` 有 {kind} 形参 `{name}`，实签 {sorted(params)}")
        assert not bad, "构造/方法形参主张与实码不符:\n  " + "\n  ".join(bad)

    def test_absent_claims_really_are_absent(self):
        """`absent` 是最容易被谎用的一档：它主张某个可调用处**没有**这个名字"""
        bad = []
        for key, (kind, spec, name) in sorted(_UNREAD_KNOB.items()):
            if kind != "absent":
                continue
            params = _callable_params(spec)
            if name in params:
                bad.append(f"{key}: 声称 `{spec}` 不收 `{name}`，实签 {sorted(params)}")
        assert not bad, "`absent` 主张被实码否证:\n  " + "\n  ".join(bad)

    def test_request_fields_really_are_on_a_route_model(self):
        bad = []
        fields_by_file = _request_fields()
        for key, (kind, spec, name) in sorted(_UNREAD_KNOB.items()):
            if kind != "request":
                continue
            path = ROOT / spec
            assert path.exists(), f"{key}: 证据点了不存在的路由文件 {spec}"
            declared = fields_by_file.get(path.name)
            if declared is None:
                bad.append(f"{key}: {spec} 里没有任何模型字段可查")
            elif name not in declared:
                bad.append(f"{key}: 声称 {spec} 的模型有字段 `{name}`，实际类注解字段是 {sorted(declared)}")
        assert not bad, "请求侧主张与实码不符:\n  " + "\n  ".join(bad)

    def test_prose_claims_do_not_outrun_the_evidence(self):
        """理由点了「请求」就得真是 request 档，点了「形参」就得真是 ctor/method 档

        反向不成立：证据是 request 而理由只说「端点侧提供」也放行——这一档只堵
        「散文说了实码没有的东西」，不评判散文写得够不够具体。
        """
        bad = []
        for key, reason in sorted(_UNREAD.items()):
            kind = _UNREAD_KNOB[key][0]
            for word, allowed in _PROSE_CLAIMS:
                if word in reason and kind not in allowed:
                    bad.append(
                        f"{key}: 理由写了「{word}」但证据是 {kind} 档"
                        f"（该词只许用于 {sorted(allowed)}）: {reason}"
                    )
        assert not bad, "散文与证据不一致（L217 的 8 条假理由就是这个形状）:\n  " + "\n  ".join(bad)

    def test_the_verifiers_are_not_vacuous(self):
        """正控制：两把验真的尺子都要能看见已知为真的主张，否则上面几条会集体空转"""
        fields_by_file = _request_fields()
        assert fields_by_file, "模型字段扫描一个字段都没抓到 ⇒ `request` 档全在空转"
        total = sum(len(v) for v in fields_by_file.values())
        assert total >= 40, f"请求字段面缩到 {total}，扫描臂大概改坏了"
        evaluate_fields = fields_by_file.get("dataset_tools.py", set())
        assert {"metrics", "generated_field", "reference_field"} <= evaluate_fields, (
            f"请求字段扫描读不到 `/api/dataset/evaluate` 的那三只字段名/指标键，"
            f"`request` 档的验真臂在空转: {sorted(evaluate_fields)}"
        )

        real_params = _callable_params("augmentor.active_learning:ActiveLearningLoop.__init__")
        assert {"strategy", "batch_size"} <= real_params, (
            f"签名读取失效或构造器改了形状: {sorted(real_params)}"
        )
