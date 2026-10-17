# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""docs/augmentor/API.md 的逐端点「人读字段提示」必须有**有位置的证据源**（A277）

判据：`docs/augmentor/API.md` 两张**详细说明表**（四列）里，每格的「说明 + 人读字段提示」中所有
反引号包住的**小写蛇形标识符**，都必须落进下面五种证据之一（外加一份封闭的 JSON 词表）：

1. `face` —— 该端点在 `app.openapi()` 里的契约面：请求体属性（递归过 `$ref` /
   `allOf` / `additionalProperties` / `items`）、`parameters[].name`、响应体属性。
   这一档是**逐端点**的，也是唯一能钉住「字段名拼错」的一档。
2. `literal` —— 字符串字面量（含 dict 键），但**作用域限制在该端点实际会跑到的文件里**
   （路由文件 + 它 import 的产品模块，两跳）。
3. `symbol` —— 产品代码里的 `def` / `class` 名或模块名（文档提到 SDK 函数时走这档）。
4. `config` —— `dataclasses.fields(AppConfig)` 的节名/键名（权威共引 L222 的 `schema_keys`）。
5. `cli` —— CLI parser 的子命令名、`choices=` 字面量、选项 dest（文档说「与 CLI 一致」时走这档）。

## 为什么第 2 档必须限作用域

`output_path` 在 `augmentor/backup.py:199` 和 `augmentor/export_enhanced.py:143` **确实是
字面量**。如果按「全仓字面量」判据，它就成了「有据可查」，而 `/api/dataset/convert` 的
请求字段实为 `output_file`（`api/routes/dataset_tools.py:90`）—— 全局字面量会把一个真
用户可见的文档缺陷洗白。作用域取「路由文件 + 两跳 import 闭包」，两跳这个数字是量的：
hops=1 时 `stratify_distribution`（权威在 `augmentor/data_splitter.py:40`）落不进闭包 ⇒ 误判；
hops=2 正解；hops=3 时 convert 的闭包涨到 70 个文件、`output_path` 反而进了 ⇒ 判据失去
区分力。`TestLiteralScopeIsNotGlobal.test_hop_budget_is_load_bearing` 把 hops=2 与 hops=3
两侧都断言死，谁把预算放宽一档就当场红。

## 建尺子过程中量到的两个读数（2026-10-11）

* 详细表 31 行（dataset 18 + system 13），提取到 **58** 个标识符形状的声称，其中 **40**
  个（69%）直接落在该端点的契约面上。
* 首跑（文档未改）无据的有 **3** 格：`output_path`（真缺陷）、`stratify_distribution`、
  `mixed`。后两格是尺子自己的 bug —— 相对导入 `from .multilingual import …` 的层级换算
  写错（`[:level - 1]` 应为 `[:len(parts) - (level - 1)]`），整个 `augmentor` 包内部的
  相对导入一条都没解析出来，闭包只剩一跳。修完只剩 `output_path` 那一格，也就是本轮
  要修的文档缺陷（4 处，其中总览散文 1 处不在这把尺子的扫描面里，由
  `TestTheRealDriftIsFixed` 补覆盖）。

## 与既有尺子的分工

`test_api_doc_endpoint_coverage_l210.py` 钉的是「端点**路径**集合与说明**存在性**」，
它不看字段名。本轮共引它的行解析器（`DOC_ROW` / `DETAILED_COLUMN_COUNT` /
`operations_of_tag`），不重抄一份正则；逐端点的字段级承诺在此之前无任何机器对账。
"""

import ast
import re
import sys
from pathlib import Path

from tests.unit.test_api_doc_endpoint_coverage_l210 import (
    API_DOC,
    DOC_ROW,
    DETAILED_COLUMN_COUNT,
    column_count,
    operations_of_tag,
    split_cells,
)
from tests.unit.test_config_three_side_l222 import schema_keys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

PRODUCT_DIRS = (ROOT / "augmentor", ROOT / "api")

#: 反引号片段里「像是个标识符」的形状：小写开头的蛇形。带 `/`、`*`、`-`、大写字母的
#: 一律不收 —— `/api/xxx`、`config.vector.*`、`faiss-cpu`、`POST` 都不是字段名。
IDENT = re.compile(r"^[a-z][a-z0-9_]*$")
BACKTICK = re.compile(r"`([^`]+)`")

#: 封闭 JSON 词表：文档里 `null` / `true` / `false` 说的是 JSON 值本身，不是字段。
#: 每员必须真的被用到，且不许同时被别的证据档绑走（见 `TestClosedRosters`）。
JSON_LITERALS = frozenset({"null", "true", "false"})

#: 短标识符不当字段看：`id` / `in` / `if` 这类词在中文散文里是别的东西，
#: 收进来只会制造噪音。3 个字符以下一律跳过（现量文档里没有 ≤3 字的字段声称）。
MIN_TOKEN_LEN = 4

_ROUTE_DECORATOR = re.compile(
    r'@(?:router|\w+_router)\.(get|post|put|delete|patch)\(\s*"(/api/[^"]+)"')


# ---------------------------------------------------------------- 契约面（第 1 档）

def openapi():
    from api.main import app
    return app.openapi()


def _schema_names(node, components, out, seen, depth=0):
    """递归收集一个 schema 里出现的全部**字段名**

    `properties` 的键是字段名；`items` / `additionalProperties` / `allOf` 继续下钻，
    所以嵌套模型（如 `SplitResponse.splits` 的值类型 `SplitPart`）的字段也算契约面。
    `additionalProperties: true` 那种自由字典**给不出字段名** —— 这是 A278 记的盲区，
    也是本尺子必须留「非 face 档」的原因。
    """
    if not isinstance(node, dict) or depth > 8:
        return
    ref = node.get("$ref")
    if isinstance(ref, str):
        name = ref.rsplit("/", 1)[-1]
        if name in seen:
            return
        _schema_names(components.get("schemas", {}).get(name, {}), components, out,
                      seen | {name}, depth + 1)
        return
    for key in ("allOf", "oneOf", "anyOf"):
        for sub in node.get(key) or []:
            _schema_names(sub, components, out, seen, depth + 1)
    for name, sub in (node.get("properties") or {}).items():
        out.add(name)
        _schema_names(sub, components, out, seen, depth + 1)
    items = node.get("items")
    if isinstance(items, list):
        for sub in items:
            _schema_names(sub, components, out, seen, depth + 1)
    else:
        _schema_names(items, components, out, seen, depth + 1)
    extra = node.get("additionalProperties")
    if isinstance(extra, dict):
        _schema_names(extra, components, out, seen, depth + 1)


FACE_CACHE = {}


def face_of(method, path):
    """该端点的契约字段名集合（同一 (方法, 路径) 只算一次）"""
    key = (method, path)
    if key not in FACE_CACHE:
        FACE_CACHE[key] = _build_face(method, path)
    return FACE_CACHE[key]


def _build_face(method, path):
    spec = openapi()
    op = spec["paths"][path][method.lower()]
    components = spec.get("components", {})
    out = set()
    for param in op.get("parameters") or []:
        name = param.get("name")
        if name:
            out.add(name)
    body = ((op.get("requestBody") or {}).get("content") or {})
    for media in body.values():
        _schema_names(media.get("schema"), components, out, frozenset())
    for response in (op.get("responses") or {}).values():
        for media in (response.get("content") or {}).values():
            _schema_names(media.get("schema"), components, out, frozenset())
    return out


# ------------------------------------------------------ 文档侧：详细表的每一格

def hint_rows():
    """详细说明表的行 → `[(方法, 路径, 说明+人读字段提示)]`

    列数判据共引 l210（4 列 = 5 枚**未转义** `|`，L278 起按 GFM 口径数）；总览表只有 3 列，天然被排除。
    """
    rows = []
    for line in API_DOC.read_text(encoding="utf-8").splitlines():
        if column_count(line) != DETAILED_COLUMN_COUNT:
            continue
        match = DOC_ROW.match(line)
        if not match:
            continue
        cells = split_cells(line)
        rows.append((match.group(1), match.group(2), f"{cells[2]} {cells[3]}"))
    return rows


def tokens_in(text):
    """格子里所有「像是字段名」的反引号片段"""
    out = set()
    for span in BACKTICK.findall(text):
        span = span.strip()
        if len(span) >= MIN_TOKEN_LEN and IDENT.match(span):
            out.add(span)
    return out


# ------------------------------------------- 产品代码侧：字面量 / 定义 / 模块名

def _is_product(path):
    return any(path == base or base in path.parents for base in PRODUCT_DIRS)


def _walk_product_files():
    out = []
    for base in PRODUCT_DIRS:
        out.extend(p for p in sorted(base.rglob("*.py")) if "__pycache__" not in p.parts)
    return out


def _module_to_path(module):
    """模块名 → 产品内文件路径（包取 `__init__.py`），不存在返回 None"""
    if not module:
        return None
    stem = Path(*module.split("."))
    for candidate in (ROOT / stem.with_suffix(".py"), ROOT / stem / "__init__.py"):
        if _is_product(candidate) and candidate.is_file():
            return candidate
    return None


def _imported_files(path):
    """该文件 import 到的产品内文件（绝对名、`from .x` 与 `from ..x` 相对名都算）

    相对导入的换算按标准口径：`level` 减一才是要往上退的层数，`from .x` 在包
    `augmentor` 里指向 `augmentor.x`（不是仓库根）。第一版把它写成 `[:level - 1]`，
    于是整个 `augmentor` 包内部的相对导入一条都没解析出来，闭包只剩一跳，
    `stratify_distribution` / `mixed` 两格被误判成无据 —— 尺子的漏判和误判一样贵。
    """
    out = []
    package_parts = list(path.relative_to(ROOT).parts[:-1])
    for node in ast.walk(ast.parse(path.read_text(encoding="utf-8"), "x")):
        modules = []
        if isinstance(node, ast.ImportFrom):
            if node.level:
                prefix = package_parts[:len(package_parts) - (node.level - 1)]
                modules.append(".".join(prefix + ([node.module] if node.module else [])))
            else:
                modules.append(node.module or "")
        elif isinstance(node, ast.Import):
            modules.extend(alias.name for alias in node.names)
        for module in modules:
            target = _module_to_path(module)
            if target:
                out.append(target)
    return out


def closure_of(route_file, hops=2):
    """路由文件 + 它 import 的产品模块，再往外 `hops` 跳 —— 第 2 档的证据作用域"""
    seen, frontier = {route_file}, [route_file]
    for _ in range(hops):
        nxt = []
        for current in frontier:
            for target in _imported_files(current):
                if target not in seen:
                    seen.add(target)
                    nxt.append(target)
        frontier = nxt
    return seen


def _literal_index(files):
    """token → 出现位置（字符串常量与 dict 键，带行号）"""
    index = {}
    for path in files:
        tree = ast.parse(path.read_text(encoding="utf-8"), str(path))
        for node in ast.walk(tree):
            if isinstance(node, ast.Constant) and isinstance(node.value, str):
                index.setdefault(node.value, []).append(f"{path.relative_to(ROOT)}:{node.lineno}")
            elif isinstance(node, ast.keyword) and node.arg:
                index.setdefault(node.arg, []).append(f"{path.relative_to(ROOT)}:{node.lineno}")
    return index


def _definition_index(files):
    """token → 定义位置（`def` / `class` 名与模块文件名）"""
    index = {}
    for path in files:
        rel = f"{path.relative_to(ROOT)}"
        index.setdefault(path.stem, []).append(rel)
        tree = ast.parse(path.read_text(encoding="utf-8"), str(path))
        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
                index.setdefault(node.name, []).append(f"{rel}:{node.lineno}")
    return index


def _cli_index():
    """CLI parser 里的子命令名、`choices=` 字面量、选项 dest"""
    index = {}
    for path in sorted((ROOT / "augmentor" / "cli").rglob("*.py")):
        if "__pycache__" in path.parts:
            continue
        rel = f"{path.relative_to(ROOT)}"
        tree = ast.parse(path.read_text(encoding="utf-8"), str(path))
        for node in ast.walk(tree):
            if isinstance(node, ast.Call):
                func = node.func
                name = func.attr if isinstance(func, ast.Attribute) else getattr(func, "id", "")
                if name == "add_parser" and node.args and isinstance(node.args[0], ast.Constant):
                    index.setdefault(node.args[0].value, []).append(f"{rel}:{node.lineno}")
                for keyword in node.keywords:
                    if keyword.arg == "choices" and isinstance(keyword.value, (ast.List, ast.Tuple)):
                        for element in keyword.value.elts:
                            if isinstance(element, ast.Constant):
                                index.setdefault(element.value, []).append(f"{rel}:{node.lineno}")
                    if keyword.arg == "dest" and isinstance(keyword.value, ast.Constant):
                        index.setdefault(keyword.value.value, []).append(f"{rel}:{node.lineno}")
            if isinstance(node, ast.Constant) and isinstance(node.value, str) \
                    and node.value.startswith("--"):
                index.setdefault(node.value.lstrip("-").replace("-", "_"), []).append(
                    f"{rel}:{node.lineno}")
    return index


PRODUCT_FILES = _walk_product_files()
LITERALS = _literal_index(PRODUCT_FILES)
DEFINITIONS = _definition_index(PRODUCT_FILES)
CLI_TOKENS = _cli_index()
CONFIG_KEYS = {}
for _section, _keys in schema_keys().items():
    CONFIG_KEYS.setdefault(_section, []).append(f"AppConfig.{_section}")
    for _key in _keys:
        CONFIG_KEYS.setdefault(_key, []).append(f"AppConfig.{_section}.{_key}")

ROW_ROUTE_FILE = {}
for _path in sorted((ROOT / "api").rglob("*.py")):
    if "__pycache__" in _path.parts:
        continue
    for _m in _ROUTE_DECORATOR.finditer(_path.read_text(encoding="utf-8")):
        ROW_ROUTE_FILE.setdefault(_m.group(1).upper() + " " + _m.group(2), _path)

ROWS = hint_rows()


ROW_LITERAL_CACHE = {}


def row_literal_tokens(method, path):
    """该格的字面量证据作用域：路由文件 + 两跳 import 闭包里的字符串常量"""
    key = (method, path)
    if key not in ROW_LITERAL_CACHE:
        route_file = ROW_ROUTE_FILE.get(f"{method} {path}")
        ROW_LITERAL_CACHE[key] = set(_literal_index(closure_of(route_file))) if route_file else set()
    return ROW_LITERAL_CACHE[key]


def evidence_for(method, path, token):
    """该 token 在这一格里有哪些证据档（空 = 文档在说一件契约里没有的事）"""
    kinds = []
    if token in face_of(method, path):
        kinds.append("face")
    if token in row_literal_tokens(method, path):
        kinds.append("literal")
    if token in DEFINITIONS:
        kinds.append("symbol")
    if token in CONFIG_KEYS:
        kinds.append("config")
    if token in CLI_TOKENS:
        kinds.append("cli")
    if token in JSON_LITERALS:
        kinds.append("json")
    return kinds


def unbound_rows():
    """`[(方法, 路径, [无据 token, ...])]`，只含有问题的行"""
    out = []
    for method, path, text in ROWS:
        bad = sorted(t for t in tokens_in(text) if not evidence_for(method, path, t))
        if bad:
            out.append((method, path, bad))
    return out


# ------------------------------------------------------------------------ 判据

class TestRulerSeesTheRealSurface:
    """尺子先要看得见真实现场，否则后面的绿没有意义"""

    def test_detailed_rows_parse(self):
        """行数与 (方法, 路径) 集合按现量钉死：dataset 18 操作 + system 15 操作

        立账轮（L223）是 18 + 13 = 31 行；**L243** 给备份完整性添了两条带说明的端点
        （`GET …/backups/verify`、`POST …/backups/cleanup`），system 侧 13 → 15。
        """
        rows = hint_rows()
        assert len(rows) == 33, (
            f"详细说明表解析到 {len(rows)} 行，现量应为 dataset 18 + system 15 = 33")
        pairs = [(m, p) for m, p, _ in rows]
        assert len(set(pairs)) == len(pairs), "说明表里有重复的 (方法, 路径)"
        assert set(pairs) == set(operations_of_tag("dataset")) | set(operations_of_tag("system"))

    def test_every_row_has_an_openapi_operation(self):
        spec = openapi()
        missing = []
        for method, path, _ in ROWS:
            op = spec["paths"].get(path, {}).get(method.lower())
            if op is None:
                missing.append(f"{method} {path}")
        assert not missing, f"文档里有说明行但 OpenAPI 里不存在：{missing}"

    def test_face_covers_request_body_fields(self):
        """convert 那格的字段名权威：请求体里是 `output_file`，没有 `output_path`"""
        face = face_of("POST", "/api/dataset/convert")
        assert "output_file" in face, f"convert 的契约面里没有 output_file：{sorted(face)}"
        assert "output_path" not in face

    def test_face_resolves_nested_refs(self):
        """`$ref` 必须递归展开：`SplitResponse.splits` 的值类型 `SplitPart` 的 `file` 也算契约面"""
        face = face_of("POST", "/api/dataset/split")
        assert {"output_dir", "splits", "file", "count"} <= face, sorted(face)

    def test_face_includes_parameters(self):
        """路径参数也在契约面里，否则 `{backup_id}` 那组每格都要手写豁免"""
        face = face_of("DELETE", "/api/system/backups/{backup_id}")
        assert "backup_id" in face, sorted(face)

    def test_route_file_lookup_is_populated(self):
        assert len(ROW_ROUTE_FILE) >= 60, f"只解析到 {len(ROW_ROUTE_FILE)} 条路由装饰器"
        for method, path, _ in ROWS:
            assert f"{method} {path}" in ROW_ROUTE_FILE, f"{method} {path} 找不到路由文件"


class TestFieldHintsBindToRealEvidence:
    """主判据：每一格里声称的字段名都得有档可查"""

    def test_no_unbound_tokens(self):
        bad = unbound_rows()
        detail = "\n".join(f"  {m} {p}: {toks}" for m, p, toks in bad)
        assert not bad, (
            "docs/augmentor/API.md 的逐端点提示里有这些名字在契约面与证据档都找不到"
            f"（字段名拼错/已改名/压根不存在）：\n{detail}"
        )


class TestEvidenceKindsAllHaveTeeth:
    """五种证据档谁都不是摆设

    `face` / `literal` / `symbol` 在**今天的语料**里各有独占成员（现量 face 3、
    literal 6、symbol 1 格）。`config` 与 `cli` 两档一个独占成员都没有 —— 它们今天
    绑到的名字全被 face/literal 补上了，所以这两档的牙齿不靠语料凑，各由一支
    「喂一个只有它认得的名字」的直接判据钉住；池子空了就红，红了就说明该连档带判据
    一起删。两档的索引是**全仓**的（`literal` 才是逐格限作用域的），这一点由
    `TestRulerIsNotBlind.test_variable_names_are_not_evidence` 兜住：真存在过的局部
    变量名（`output_path`）不许从任何一档混进来。
    """

    def test_three_kinds_have_exclusive_members(self):
        exclusive = {kind: [] for kind in ("face", "literal", "symbol")}
        for method, path, text in ROWS:
            for token in tokens_in(text):
                kinds = evidence_for(method, path, token)
                if len(kinds) == 1 and kinds[0] in exclusive:
                    exclusive[kinds[0]].append(f"{token} ({method} {path})")
        dead = {k: v for k, v in exclusive.items() if not v}
        assert not dead, f"这些证据档在语料里一个独占成员都没有，等于死代码：{dead}"

    def test_config_kind_catches_a_real_key(self):
        """`config` 档的独占能力：认得「只作为配置键存在」的名字"""
        pool = sorted(
            key for key in CONFIG_KEYS
            if key not in LITERALS and key not in DEFINITIONS and key not in CLI_TOKENS
            and len(key) >= MIN_TOKEN_LEN
        )
        assert pool, "没有任何「只有配置面认得」的键 ⇒ config 档是死代码，应连同判据一起删"
        pick = pool[0]
        assert evidence_for("POST", "/api/dataset/convert", pick) == ["config"]

    def test_cli_kind_catches_a_real_option(self):
        """`cli` 档的独占能力：认得「只有 CLI 选项面存在」的名字"""
        scoped = row_literal_tokens("POST", "/api/dataset/convert")
        face = face_of("POST", "/api/dataset/convert")
        pool = sorted(
            token for token in CLI_TOKENS
            if token not in DEFINITIONS and token not in CONFIG_KEYS
            and token not in scoped and token not in face and len(token) >= MIN_TOKEN_LEN
        )
        assert pool, "没有任何「只有 CLI 面认得」的名字 ⇒ cli 档是死代码，应连同判据一起删"
        assert evidence_for("POST", "/api/dataset/convert", pool[0]) == ["cli"]

    def test_every_kind_binds_something_in_the_corpus(self):
        bound = {kind: 0 for kind in ("face", "literal", "symbol", "config", "cli")}
        for method, path, text in ROWS:
            for token in tokens_in(text):
                for kind in evidence_for(method, path, token):
                    if kind in bound:
                        bound[kind] += 1
        assert all(bound.values()), f"有档在语料里一次都没绑到名字：{bound}"

    def test_face_is_the_dominant_kind(self):
        """大多数声称应当直接落在契约面上；face 占比塌了就说明判据被稀释"""
        total, faced = 0, 0
        for method, path, text in ROWS:
            for token in tokens_in(text):
                total += 1
                if "face" in evidence_for(method, path, token):
                    faced += 1
        assert total >= 60, f"整张表只提取到 {total} 个 token，提取器八成坏了"
        assert faced * 2 >= total, f"只有 {faced}/{total} 落在契约面上，占比过低"


class TestLiteralScopeIsNotGlobal:
    """第 2 档的牙齿：作用域退回全局，这个缺陷就会被洗白"""

    def test_output_path_exists_as_a_literal_somewhere(self):
        assert "output_path" in LITERALS, (
            f"预期它至少在 backup/export 侧是字面量（{sorted(k for k in LITERALS if 'output' in k)[:6]}）"
            "；这份前置条件没了就说明下面的作用域判据失去了意义，要重跑普查改判据")

    def test_output_path_is_not_evidence_for_convert(self):
        scope = closure_of(ROW_ROUTE_FILE["POST /api/dataset/convert"])
        assert "output_path" not in _literal_index(scope)
        assert "output_file" in _literal_index(scope)

    def test_closure_stays_bounded(self):
        """跳数预算是承重的：多跳一整档，那个真缺陷就会被洗白

        现量（2026-10-11，闭包按 hops 计）：
        * hops=1 ⇒ convert 作用域 17 个文件，`stratify_distribution` 落不进去（误判）；
        * hops=2 ⇒ 24 个文件，`stratify_distribution` 进去、`output_path` 不进（正解）；
        * hops=3 ⇒ 70 个文件，`output_path` 也进去了 —— 判据当场失去区分力。
        所以两跳是量出来的，不是随手定的；`max` 那格现量 66。
        """
        sizes = {f: len(closure_of(f)) for f in set(ROW_ROUTE_FILE.values())}
        assert max(sizes.values()) <= 66, f"最宽的一格闭包 {max(sizes.values())} 个文件，比现量还大"
        assert min(sizes.values()) >= 3, sizes

    def test_hop_budget_is_load_bearing(self):
        """把跳数放宽一档，`output_path` 就从「无据」变成「有据」—— 本判据的关键牙齿"""
        route_file = ROW_ROUTE_FILE["POST /api/dataset/convert"]
        assert "output_path" not in _literal_index(closure_of(route_file, 2))
        assert "output_path" in _literal_index(closure_of(route_file, 3))


#: 写盘变换族的七条路径：五条落 `output_file`、`split` 落 `output_dir`，而 `output_path`
#: 一行都不许出现（它是路由里的局部变量名，不是任何一条 dataset 路由的请求字段）。
#: L231 起这个名字在**另一族**里是真的：备份恢复端点的响应落点键就叫 `output_path`。
WRITE_SIDE_PATHS = ["/api/dataset/convert", "/api/dataset/merge", "/api/dataset/sample",
                    "/api/dataset/aggregate", "/api/dataset/rag", "/api/system/migrate",
                    "/api/system/stream"]


class TestTheRealDriftIsFixed:
    """本轮建尺子的起因：`output_path` 不是任何一条 dataset 路由的请求字段"""

    def test_doc_never_names_the_ghost_field(self):
        """整篇 API.md 里 `output_path` 只许出现在 restore 那一行

        原判据是「整篇都不许出现」，立它的起因是 `output_path` 不是任何一条 dataset 路由的
        请求字段（散文注里还写着旧名）。L231 量出一条**真的同名键**：
        `POST /api/system/backups/{backup_id}/restore` 的响应落点键就叫 `output_path`
        （同族其余五条写盘端点叫 `output_file`，`split` 叫 `output_dir`），照实写进文档
        就撞上那道全文禁令。所以按现量**收窄到逐行点名**而不是把禁令摘掉：dataset 那七条
        写盘变换族一行都不许出现（原缺陷照旧喊红），全文只许 restore 那一行出现，多一行即红。
        """
        text = API_DOC.read_text(encoding="utf-8")
        hits = [line for line in text.splitlines() if "output_path" in line]
        assert len(hits) == 1, f"`output_path` 现量只该出现 1 行，实得 {len(hits)} 行：{hits}"
        assert "/api/system/backups/{backup_id}/restore" in hits[0], hits
        for path in WRITE_SIDE_PATHS:
            rows = [line for line in text.splitlines() if f"`{path}`" in line]
            assert rows, f"API.md 里没有 {path} 这一行 ⇒ 判据空转"
            assert not any("output_path" in row for row in rows), (path, rows)

    def test_write_side_rows_name_their_own_field(self):
        """六条写盘变换类里五条的字段是 `output_file`，`split` 是 `output_dir`"""
        for path in WRITE_SIDE_PATHS:
            assert "output_file" in face_of("POST", path), path
        assert "output_dir" in face_of("POST", "/api/dataset/split")
        assert "output_file" not in face_of("POST", "/api/dataset/split")


class TestClosedRosters:
    """封闭清单不许当逃生舱（A266 家族的口径）"""

    def test_json_literals_are_all_used(self):
        """词表每一员都得在文档里真的以「独立反引号片段」出现（现量 null 4 / true 11 / false 8）

        判据取整篇文档而不只是详细表：这三词在别处（配置口径、CLI 说明）也在用，
        只看详细表会把「详细表暂时没写」误判成「这词没用」。
        """
        text = API_DOC.read_text(encoding="utf-8")
        unused = [w for w in sorted(JSON_LITERALS)
                  if not re.search(rf"`{w}(?![A-Za-z0-9_])", text)]
        assert not unused, f"JSON 词表里有 {len(unused)} 员从未在文档出现：{unused}"

    def test_json_literals_are_not_also_bindable(self):
        """若某个词真能由别档绑定，就不该藏在词表里（词表只收「值本身」）"""
        smuggled = {
            t: sorted(set(LITERALS) & {t}) for t in JSON_LITERALS if t in CONFIG_KEYS or t in CLI_TOKENS
        }
        assert not smuggled, f"这些词其实有别档可查，词表多余：{sorted(smuggled)}"


class TestRulerIsNotBlind:
    """反空转：判据必须能失败，而且失败得对"""

    def test_invented_token_has_no_evidence(self):
        for method, path, _ in ROWS[:1]:
            assert evidence_for(method, path, "no_such_field_l223") == []

    def test_variable_names_are_not_evidence(self):
        """路由里的局部变量不算证据 —— 这正是 `output_path` 曾伪装成合法的原因"""
        assert "output_path" not in DEFINITIONS
        assert "output_path" not in CONFIG_KEYS
        assert "output_path" not in CLI_TOKENS

    def test_corpora_are_nontrivial(self):
        assert len(LITERALS) > 500, f"字面量索引只有 {len(LITERALS)} 条"
        assert len(DEFINITIONS) > 300, f"定义索引只有 {len(DEFINITIONS)} 条"
        assert len(CLI_TOKENS) > 40, f"CLI 索引只有 {len(CLI_TOKENS)} 条"
        assert len(CONFIG_KEYS) >= len(schema_keys())
