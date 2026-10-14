# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L265：``except HTTPException: raise`` 保码面普查 —— 「4xx 被收尾分支降级成 500」那一族没有尺子

**这一轮收的是哪一格**：`api/deps.py` 的闸一律用 ``HTTPException`` 出声（401 缺 key、
403 路径越界、400 非法组件、404 文件不存在、413 上传越界），而路由收尾住在一处
（``to_http_error`` / ``raise_internal_error`` / ``raise_boundary_error``）：**凡不是
``FileNotFoundError`` / ``JSONDecodeError`` / ``ValueError`` 的异常都收敛成 500**。
``to_http_error`` 的 docstring 把这一格写得清清楚楚 —— ``HTTPException`` **不**在这里处理，
它应当由调用方原样抛出，否则会被降级成 500。于是「原样抛出」这一支（``except HTTPException:
raise``，且**必须排在**宽 ``except Exception`` **之前**）是鉴权与白名单判决能否到达客户端的
唯一通道。全仓今天有 58 只这样的支、52 个「try 体内可达 raiser 且带宽收尾」的配对点，
**而没有一把尺子量它们**：既有测试是按端点逐个注入的样本（``tests/integration/
test_api_passthrough_branches.py``，L69），删掉某一路由的那一支、或新写一条漏掉那一支的路由，
样本一条都不会红 —— 这正是 L242「本账第 NNN 行那一形没有尺子」同款形状的另一个实例。

**为什么这一族值得钉**：被降级掉的是 401 与 403。客户端拿 500 时既不知道「被闸拦了」也不知道
「要带 key」，而运维日志里那条 ``logger.exception`` 会把一次**正常的**授权判决当成故障记下来；
更糟的是若收尾写成 ``detail=str(e)``，``HTTPException`` 的 ``str`` 形带 ``detail`` 原文，
那条 500 就顺带把内部路径说给调用方。判据与 A155①（500 档不回显原文）是同一条口径的两半。

**判据（AST 推导，不抄清单；承 L56「判据型脚本一律走 AST」）**，语料面 = ``api/`` 全部 24 个 ``.py``：

- ``raisers``：会 ``raise HTTPException``（或 ``raise <收尾函数>`` 的返回值）的函数名，
  按「体内调用了已知 raiser」迭代到不动点扩张。现量 101 只 = 面内 68 只直接出声 + 33 只间接。
- ``clauses``：所有 ``except HTTPException`` 支，按**体形**分三档 —— ``bare``（整支就一个裸
  ``raise``）、``conditional``（带条件的再抛，全仓唯一一只是 ``api/routes/data.py`` 里
  ``_upload_landing_path`` 的「先解释一次、越界才救」那一支，A160/A163）、``swallow``
  （把异常咽下去）= **缺陷**，现量空集。
- ``paired``：带宽 ``except Exception`` 的 ``try``，且其体内**装载**（load）过 raiser 名字
  —— 装载而不只是调用，是因为 ``await run_in_thread(load_items, name)`` 把 raiser 当**值**传出去，
  只看 ``Call`` 的判据会整条漏掉 —— raiser 是**参数**不是被这个 try 调用的函数（实测：52 只配对点
  里只看调用会少 13 只，且 ``audit.py`` / ``leakage.py`` / ``privacy.py`` 三只文件
  **整块**从配对面消失，见 ``MEASURED_LOAD_ONLY_TRIES`` 与
  ``test_the_load_dimension_is_what_keeps_three_files_in_the_face``）。
  每一只配对点都必须有一个排在宽支**之前**的
  ``HTTPException`` 支；没有 ⇒ ``missing`` = **缺陷**，现量空集。

**已知盲区（写明，不装）**：① 通过**非模块对象**的属性调用可达的 raiser（``p.method()``）不进面 ——
依据是 ``augmentor/`` 那一侧根本不 import fastapi（`test_the_sdk_side_cannot_raise_http_exception`
把这条依据钉在磁盘上，而不是靠我说「应该没有」）；② 嵌套函数里 raise 的 raiser 只按**名字**参与面，
同名会并档；③ ``try`` 体外 raise 的 4xx（如 ``analyze_data`` 在 try 之前调 ``_safe_data_path``）
不进配对面，因此那 5 支是纯防御性冗余 —— 由
`test_the_defensive_clauses_are_the_measured_five` 钉成名册，而不是任其看起来像漏判。

**为什么零 parametrize**：``test_parametrize_ids_l97`` 与 ``test_parametrize_blind_face_l98``
把全仓 parametrize 三面冻成现量数，本守卫走名册字典循环，那两把普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_http_exception_passthrough_l265.py -q --no-cov`
"""
import ast
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
API = ROOT / "api"

#: 收尾三兄弟：把「不认识的异常」收敛成 500（或 400/404）的那三个函数
FUNNELS = frozenset({"to_http_error", "raise_internal_error", "raise_boundary_error"})
#: 宽收尾 —— ``except Exception`` 会连 ``HTTPException`` 一起接住
BROAD = frozenset({"Exception", "BaseException"})
HTTP = "HTTPException"

BARE = "bare"
CONDITIONAL = "conditional"
SWALLOW = "swallow"
SHAPES = (BARE, CONDITIONAL, SWALLOW)

#: ``_upload_landing_path`` 那一支的身份（文件, 所在函数）：全仓唯一一只非裸 raise
RESCUE = ("api/routes/data.py", "_upload_landing_path")


def sources():
    """语料面：``api/`` 下全部 ``.py`` ⇒ {相对 ROOT 的 posix 路径: 文本}

    判据函数一律收 ``sources`` 字典，注入臂因此在**内存里**改文本、零落盘。
    """
    out = {}
    for p in sorted(API.rglob("*.py")):
        if "__pycache__" in p.parts:
            continue
        out[p.relative_to(ROOT).as_posix()] = p.read_text(encoding="utf-8")
    return out


def _funcs(tree):
    """面内**任意深度**的函数定义（含类方法 —— 中间件里有类）"""
    return [n for n in ast.walk(tree)
            if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]


def _name(node):
    if isinstance(node, ast.Name):
        return node.id
    if isinstance(node, ast.Attribute):
        return node.attr
    return None


def _called(fn):
    """函数体内**被调用**的名字（闭包扩张只按调用算，避免「引用过就算」把面吹胀）"""
    out = set()
    for node in ast.walk(fn):
        if isinstance(node, ast.Call):
            nm = _name(node.func)
            if nm:
                out.add(nm)
    return out


def _produces_http(fn):
    """函数体里有没有 ``raise HTTPException(...)`` 或 ``raise <收尾函数>(...)``"""
    for node in ast.walk(fn):
        if isinstance(node, ast.Raise) and node.exc is not None:
            f = getattr(node.exc, "func", None)
            nm = _name(f) if f is not None else None
            if nm == HTTP or nm in FUNNELS:
                return True
    return False


def raisers(src=None):
    """会（直接或间接）以 ``HTTPException`` 出声的函数名，迭代到不动点"""
    src = sources() if src is None else src
    funcs = []
    for text in src.values():
        funcs.extend(_funcs(ast.parse(text)))
    known = {n.name for n in funcs if _produces_http(n)}
    for _ in range(20):
        grew = False
        for n in funcs:
            if n.name not in known and (_called(n) & known):
                known.add(n.name)
                grew = True
        if not grew:
            break
    return frozenset(known)


def _handler_types(handler):
    if handler.type is None:
        return frozenset({"Exception"})
    if isinstance(handler.type, ast.Tuple):
        return frozenset(_name(x) or "?" for x in handler.type.elts)
    return frozenset({_name(handler.type) or "?"})


def _owner(node, parents):
    n = parents.get(id(node))
    while n is not None:
        if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef)):
            return n.name
        n = parents.get(id(n))
    return "<module>"


def _with_parents(tree):
    parents = {}
    for node in ast.walk(tree):
        for child in ast.iter_child_nodes(node):
            parents[id(child)] = node
    return parents


def clauses(src=None):
    """每只 ``except HTTPException`` 支 ⇒ {file, owner, line, shape, n_types}"""
    src = sources() if src is None else src
    out = []
    for rel, text in src.items():
        tree = ast.parse(text)
        parents = _with_parents(tree)
        for node in ast.walk(tree):
            if not isinstance(node, ast.ExceptHandler) or node.type is None:
                continue
            if HTTP not in _handler_types(node):
                continue
            has_bare_raise = any(
                isinstance(x, ast.Raise) and x.exc is None
                for x in ast.walk(ast.Module(body=node.body, type_ignores=[]))
            )
            single = len(node.body) == 1 and isinstance(node.body[0], ast.Raise)
            if single and has_bare_raise:
                shape = BARE
            elif has_bare_raise:
                shape = CONDITIONAL
            else:
                shape = SWALLOW
            out.append({"file": rel, "owner": _owner(node, parents), "line": node.lineno,
                        "shape": shape, "n_types": len(_handler_types(node))})
    return out


def paired(src=None, mode="load"):
    """``try`` 体内可达 raiser 名字、且带宽收尾的配对点（**一只 try 一处**）

    ``mode="load"``（默认）认「装载」：名字出现在调用位**或**实参位都算 ——
    ``await run_in_thread(load_items, name)`` 把 raiser 当值传出去，只看调用会整条漏掉。
    ``mode="call"`` 只认调用位，用来量「装载维」到底多看见了什么（见
    ``test_the_load_dimension_is_what_keeps_data_py_in_the_face``）。

    每项 {file, owner, line, names, guarded}。``guarded`` = 第一个宽支之前是否有
    ``HTTPException`` 支 —— 顺序承重：Python 按**书写顺序**匹配 except 支，把
    ``except HTTPException`` 写在 ``except Exception`` 之后等于没写。
    """
    src = sources() if src is None else src
    known = raisers(src)
    out = []
    for rel, text in src.items():
        tree = ast.parse(text)
        parents = _with_parents(tree)
        visible = {n.name for n in tree.body
                   if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))}
        alias = {}
        for node in ast.walk(tree):
            if isinstance(node, ast.ImportFrom) and node.module and (
                    node.module.endswith("deps")
                    or node.module.startswith("api.routes")
                    or (node.level and node.module in {Path(p).stem for p in src})):
                for a in node.names:
                    visible.add(a.asname or a.name)
                    if a.asname:
                        alias[a.asname] = a.name
        for node in ast.walk(tree):
            if not isinstance(node, ast.Try) or not node.handlers:
                continue
            types = [_handler_types(h) for h in node.handlers]
            broad_at = next((i for i, t in enumerate(types) if t & BROAD), None)
            if broad_at is None:
                continue
            guarded = any(HTTP in t for t in types[:broad_at])
            hits = set()
            for sub in ast.walk(ast.Module(body=node.body, type_ignores=[])):
                if isinstance(sub, ast.Call):
                    nm = _visible_raiser(sub.func, known, visible, alias)
                    if nm:
                        hits.add(nm)
                    continue
                if mode != "load":
                    continue
                # 装载维：raiser 出现在**实参位**（把函数本身传出去）同样会出声
                if isinstance(sub, ast.Name) and isinstance(sub.ctx, ast.Load):
                    nm = _visible_raiser(sub, known, visible, alias)
                    if nm:
                        hits.add(nm)
                elif isinstance(sub, ast.Attribute) and isinstance(sub.value, ast.Name) \
                        and sub.value.id.endswith("deps") and sub.attr in known:
                    hits.add(sub.attr)
            if not hits:
                continue
            out.append({"file": rel, "owner": _owner(node, parents), "line": node.lineno,
                        "names": frozenset(hits), "guarded": guarded})
    return out


def _visible_raiser(node, known, visible, alias):
    """这个节点是不是「本模块可见的那个 raiser」？

    只认**裸名字**与 ``deps.x`` 两种形状：``p.list_checkpoints()`` 里那个属性名与路由里的
    同名函数撞车（``augment.py`` 的 ``list_checkpoints`` 端点 vs ``CheckpointManager`` 的
    方法），按属性名并档会把 SDK 方法误判成闸 —— 实测会把两格假写成缺陷。盲区 ① 就是
    这条口径的代价，写进 docstring 而不是偷偷吞掉。
    """
    if isinstance(node, ast.Name):
        nm = alias.get(node.id, node.id)
        return nm if nm in known and (node.id in visible or nm in visible) else None
    if isinstance(node, ast.Attribute) and isinstance(node.value, ast.Name) \
            and node.value.id.endswith("deps"):
        return node.attr if node.attr in known else None
    return None


def missing(src=None):
    """缺陷档：可达 raiser 却没有前置保码支的配对点（现量应为空集）"""
    return [p for p in paired(src) if not p["guarded"]]


def swallowers(src=None):
    """缺陷档：把 ``HTTPException`` 咽掉的支（现量应为空集）"""
    return [c for c in clauses(src) if c["shape"] == SWALLOW]


def identity(rows):
    return {(r["file"], r["owner"]) for r in rows}


#: ---- 现量（L265 立尺，L273 重钉）----
#: L273 把 update / delete 两只端点的读-改-写折进 `api/deps.py::mutate_json_file`，
#: 于是这一面同时长了四格：路由里的两个就地闭包 `_replace` / `_remove` 各自
#: `raise HTTPException(400, "索引越界")` ⇒ 直接出声 +2；`_read_modify_write` 调
#: `read_items`（体里 `raise to_http_error(e)`，本就在直接出声名册里）⇒ 间接 +1，
#: `mutate_json_file` 调它 ⇒ 间接再 +1。两只路由的 try 因此从「纯防御」升格为
#: 「可达 raiser 的配对点」——保码支从「本来用不上」变成「真的在用」，配对面 +2。
MEASURED_FILES = 24
MEASURED_RAISERS = 101
#: 名册里必须出现的锚点：闸与上传面各挑几枚，防止闭包算成一堆同音名字却漏掉正主
ANCHOR_RAISERS = ("resolve_within_roots", "resolve_data_path", "verify_api_key",
                  "assert_dataset_shape", "_read_upload_body", "_assert_within_roots",
                  "raise_internal_error", "raise_boundary_error")
#: 闭包 101 只 = 体里直接 ``raise HTTPException`` 的 68 只 + 由调用关系并进来的 33 只间接
#: raiser。播种规则是「任何直接出声的函数名都是种子」，所以面内**没有**一只直接出声函数
#: 落在闭包之外 —— 新增一只闸天然入面，不必等谁把它登记进名册（这一格的尺子见
#: ``test_the_closure_seeds_every_direct_producer``，注入臂见闭包播种臂）。
MEASURED_PRODUCERS = 68
MEASURED_INDIRECT_RAISERS = 33
MEASURED_CLAUSES = 58
#: 支数 58 而所在函数 57 —— 唯一一只「同一函数两只支」是 ``upload_data``（读侧与写侧各一支）
MEASURED_CLAUSE_OWNERS = 57
MEASURED_MULTI_CLAUSE_OWNERS = {("api/routes/data.py", "upload_data"): 2}
MEASURED_SHAPE_TALLY = {BARE: 57, CONDITIONAL: 1, SWALLOW: 0}
#: 配对面按 **try** 计数（一只 try 一处），52 处落在 52 个不同函数里
MEASURED_PAIRED = 52
MEASURED_PAIRED_OWNERS = 52
MEASURED_PAIRED_BY_FILE = {
    "api/routes/audit.py": 1,
    "api/routes/augment.py": 1,
    "api/routes/data.py": 3,
    "api/routes/dataset_tools.py": 14,
    "api/routes/export.py": 3,
    "api/routes/language.py": 2,
    "api/routes/leakage.py": 1,
    "api/routes/multimodal.py": 2,
    "api/routes/privacy.py": 1,
    "api/routes/quality.py": 9,
    "api/routes/system_ops.py": 14,
    "api/routes/vector.py": 1,
}
#: 只按「raisers 被调用」判的尺子看到 39 只 try，比装载维少 13 只——这 13 只的 raiser
#: 是**当值传出去**的（``await run_in_thread(load_items, name)`` 那一形），不是被直接调用的。
#: 少掉的那 13 只全部落在 quality/audit/data/export/leakage/privacy 六个文件里，
#: 若按调用维定配对面，``audit.py`` / ``leakage.py`` / ``privacy.py`` 三只文件的配对面会
#: 整块消失（L273 之前是四只：``data.py`` 的 update / delete 两只 try 体里只当值传闭包，
#: 现在它们直接调用 `mutate_json_file` —— 一只间接 raiser —— 于是从调用维也看得见）。
MEASURED_PAIRED_CALL_ONLY = 39
MEASURED_LOAD_ONLY_TRIES = {
    ("api/routes/audit.py", "audit_dataset_endpoint"),
    ("api/routes/data.py", "upload_data"),
    ("api/routes/export.py", "preview_export"),
    ("api/routes/leakage.py", "check_leakage"),
    ("api/routes/privacy.py", "sanitize_items"),
    ("api/routes/quality.py", "annotate_data"),
    ("api/routes/quality.py", "clean_data"),
    ("api/routes/quality.py", "deduplicate"),
    ("api/routes/quality.py", "detect_outliers_endpoint"),
    ("api/routes/quality.py", "evaluate_quality"),
    ("api/routes/quality.py", "generate_report"),
    ("api/routes/quality.py", "health_gate"),
    ("api/routes/quality.py", "run_benchmark"),
}
#: 调用维**整块**看不见的文件：这三只文件里唯一的配对 try 体中 raiser 只当参数出现
#: （``await run_in_thread(load_items, name)``），按 ``Call`` 判的尺子会认为体内无可达 raiser，
#: 于是连「这一格有没有支」这个问题都问不出来 —— 配面对象直接不存在。
MEASURED_CALL_BLIND_FILES = ("api/routes/audit.py",
                             "api/routes/leakage.py", "api/routes/privacy.py")
#: 纯防御性的支：所在函数没有「可达 raiser 的宽收尾 try」，因此不进配对面。
#: L265 立尺时 7 支，L273 之后 5 支——``update_data_item`` / ``delete_data_item`` 的
#: try 体现在可达 raiser（见本段开头的重钉说明），它们从「冗余」升格为「真在用的保码支」。
MEASURED_DEFENSIVE = {
    ("api/routes/data.py", "_upload_landing_path"),
    ("api/routes/data.py", "analyze_data"),
    ("api/routes/data.py", "visualize_data"),
    ("api/routes/vector.py", "vector_status"),
    ("api/routes/version.py", "create_version"),
}


class TestTheRulerSeesSomething:
    """反空转：三面都必须真取到东西，否则空集对空集也能绿"""

    def test_the_face_is_the_whole_api_tree(self):
        src = sources()
        assert len(src) == MEASURED_FILES, len(src)
        assert any(f.startswith("api/routes/") for f in src)
        assert any(f.startswith("api/middleware/") for f in src), "中间件不在面里 ⇒ 面被 glob 静默扫小"

    def test_the_raiser_closure_is_the_measured_one(self):
        known = raisers()
        assert len(known) == MEASURED_RAISERS, len(known)
        for anchor in ANCHOR_RAISERS:
            assert anchor in known, anchor

    def test_the_closure_seeds_every_direct_producer(self):
        """闭包 = 68 只直接出声 + 33 只间接，且「面内直接出声函数」一只不漏

        这条断言是播种规则的可证伪版本：``known`` 里必须**含下**每一只体里直接
        ``raise HTTPException`` 的函数，一只都不许掉在外面 —— 掉了就意味着「新写一只闸、
        宽 try 里漏一支保码」那种缺陷会静默通过（缺口的形状正是「raiser 不在名册里」）。
        """
        src = sources()
        prod = {fn.name for text in src.values()
                for fn in _funcs(ast.parse(text)) if _produces_http(fn)}
        known = raisers(src)
        assert len(prod) == MEASURED_PRODUCERS, len(prod)
        assert prod - known == set(), sorted(prod - known)
        assert len(known) == MEASURED_RAISERS, len(known)
        assert len(known - prod) == MEASURED_INDIRECT_RAISERS, sorted(known - prod)

    def test_the_clause_face_is_the_measured_one(self):
        rows = clauses()
        assert len(rows) == MEASURED_CLAUSES, len(rows)

    def test_the_paired_face_is_the_measured_one(self):
        rows = paired()
        assert len(rows) == MEASURED_PAIRED, len(rows)
        assert len(identity(rows)) == MEASURED_PAIRED_OWNERS, len(identity(rows))
        tally = {}
        for r in rows:
            tally[r["file"]] = tally.get(r["file"], 0) + 1
        assert tally == MEASURED_PAIRED_BY_FILE, tally

    def test_the_load_dimension_is_what_keeps_three_files_in_the_face(self):
        """两把尺子的差必须钉成断言：只看「raisers 被调用」会少 13 只 try，整块少 3 个文件

        ``await run_in_thread(load_items, name)`` 里 ``load_items`` 是**参数**不是被调用的函数，
        按 ``ast.Call.func`` 判的尺子会认为整个 try 体无可达 raiser ⇒ 这一格不进配对面 ⇒
        「这一格有没有保码支」这个问题根本问不出来。装载维（``ast.Name`` 且 ``ctx is Load``）
        才把它捞回来。
        """
        src = sources()
        load_rows = paired(src, "load")
        call_rows = paired(src, "call")
        assert len(load_rows) == MEASURED_PAIRED and len(call_rows) == MEASURED_PAIRED_CALL_ONLY
        sid = {(r["file"], r["owner"]) for r in call_rows}
        lid = {(r["file"], r["owner"]) for r in load_rows}
        assert lid - sid == MEASURED_LOAD_ONLY_TRIES, (
            sorted(lid - sid), sorted(MEASURED_LOAD_ONLY_TRIES))
        assert len(MEASURED_LOAD_ONLY_TRIES) == 13
        # 单向包含：调用维看到的每一格，装载维也必须看到（反过来不成立）
        assert sid - lid == set(), sorted(sid - lid)
        call_files = {r["file"] for r in call_rows}
        assert sorted(f for f in MEASURED_PAIRED_BY_FILE if f not in call_files) \
            == sorted(MEASURED_CALL_BLIND_FILES), sorted(call_files)

    def test_the_sdk_side_cannot_raise_http_exception(self):
        """盲区 ① 的依据必须现场量：``augmentor/`` 不 import fastapi ⇒ 属性调用面外无 raiser"""
        offenders = [p.relative_to(ROOT).as_posix()
                     for p in (ROOT / "augmentor").rglob("*.py")
                     if "__pycache__" not in p.parts
                     and ("fastapi" in p.read_text(encoding="utf-8"))]
        assert offenders == [], offenders


class TestTheDefectTiersAreEmpty:
    def test_no_reachable_raiser_site_lacks_the_clause(self):
        rows = missing()
        assert rows == [], [(r["file"], r["owner"], r["line"], sorted(r["names"])) for r in rows]
        assert not missing(sources())

    def test_no_clause_swallows_the_exception(self):
        assert swallowers() == []

    def test_the_one_conditional_clause_is_that_documented_rescue(self):
        rows = [c for c in clauses() if c["shape"] == CONDITIONAL]
        assert len(rows) == 1, rows
        assert (rows[0]["file"], rows[0]["owner"]) == RESCUE, rows[0]
        assert rows[0]["n_types"] == 1, "与别的异常同支 ⇒ 一只宽支就能咽掉它"

    def test_the_shape_tally_is_the_measured_one(self):
        got = {s: 0 for s in SHAPES}
        for c in clauses():
            got[c["shape"]] += 1
        assert got == MEASURED_SHAPE_TALLY, got

    def test_the_defensive_clauses_are_the_measured_five(self):
        """L265 立尺时 7 支，L273 之后 5 支：update / delete 的保码支从冗余升格为真在用"""
        got = identity(clauses()) - identity(paired())
        assert got == MEASURED_DEFENSIVE, (sorted(got - MEASURED_DEFENSIVE),
                                           sorted(MEASURED_DEFENSIVE - got))


class TestTheClauseFaceIsRostered:
    def test_every_clause_file_is_a_route_module(self):
        assert {c["file"] for c in clauses()} <= {f for f in sources()
                                                 if f.startswith("api/routes/")}, \
            "deps 或 main 里出现保码支 ⇒ 面变了，逐条点名后再重钉"

    def test_each_clause_owners_a_distinct_try_in_its_module(self):
        rows = clauses()
        assert len({(r["file"], r["line"]) for r in rows}) == len(rows)

    def test_no_clause_shares_its_handler_tuple_with_a_broad_type(self):
        """``except (HTTPException, Exception)`` 这种写法会把保码支变成咽码支"""
        assert [r for r in clauses() if r["n_types"] > 1] == []


class TestTheBehaviourIsReal:
    """判据不是纸面游戏：同一段路由代码，有那支与没那支，客户端拿到的码**不同**

    两个 mini app 全在内存里构造，不碰仓库、不碰真实路由，量的是「收尾三兄弟会把
    ``HTTPException`` 降级成 500」这条被 docstring 声明过的口径真成立。
    """

    def _app(self, guarded):
        from fastapi import FastAPI, HTTPException

        from api.deps import to_http_error

        app = FastAPI()

        @app.get("/gate")
        def gate():
            try:
                raise HTTPException(status_code=403, detail="路径超出允许的数据目录范围")
            except HTTPException:
                if guarded:
                    raise
                raise to_http_error(Exception("reached here"))
            except Exception as e:  # noqa: BLE001
                raise to_http_error(e) from e

        return app

    def _get(self, guarded):
        from fastapi.testclient import TestClient

        return TestClient(self._app(guarded)).get("/gate")

    def test_with_the_clause_the_gate_answer_reaches_the_client(self):
        r = self._get(True)
        assert r.status_code == 403, r.text
        assert "数据目录" in r.text

    def test_without_the_clause_the_same_answer_becomes_a_500(self):
        r = self._get(False)
        assert r.status_code == 500, r.text

    def test_the_downgrade_hides_the_detail_behind_the_fixed_sentence(self):
        from api.deps import INTERNAL_ERROR_DETAIL

        assert self._get(False).json()["detail"] == INTERNAL_ERROR_DETAIL
        assert self._get(True).json()["detail"] != INTERNAL_ERROR_DETAIL

class TestTheClauseOwnerRoster:
    def test_the_clause_owner_count_is_the_measured_one(self):
        rows = clauses()
        owners = {}
        for r in rows:
            owners[(r["file"], r["owner"])] = owners.get((r["file"], r["owner"]), 0) + 1
        assert len(owners) == MEASURED_CLAUSE_OWNERS, len(owners)
        multi = {k: v for k, v in owners.items() if v > 1}
        assert multi == MEASURED_MULTI_CLAUSE_OWNERS, multi


class TestRulerBites:
    """注入臂：全部在内存语料里做**行段手术**，零落盘

    为什么不用文本锚点：那四行收尾在同一个文件里出现 14 次（``dataset_tools.py`` 一整族
    端点同形），「锚点必须唯一」的断言会先把手术否掉；而按 ``(文件, 所在函数)`` 定位 AST
    节点再切行段，既稳定又能覆盖任意一只支 —— 这一格本身也是「判据要落在结构上而不是
    落在字面上」那条纪律的实例。
    """

    TARGET = ("api/routes/dataset_tools.py", "dataset_stats")
    #: 装载维独有的那一格：try 体是 ``await run_in_thread(load_items, request.input_file)``
    BLIND_TARGET = ("api/routes/audit.py", "audit_dataset_endpoint")
    #: 闭包播种臂的靶文件：往它尾部追加一只新闸 + 一条无支端点（既有模块，不是新建文件）
    CLOSURE_TARGET = "api/routes/system_ops.py"

    @staticmethod
    def _locate(src, file, owner):
        """取该 owner 的 ``except HTTPException`` 行段，与它所属 try 的宽支行段"""
        tree = ast.parse(src[file])
        parents = _with_parents(tree)
        found = [h for h in ast.walk(tree)
                 if isinstance(h, ast.ExceptHandler) and h.type is not None
                 and HTTP in _handler_types(h) and _owner(h, parents) == owner]
        assert len(found) == 1, (file, owner, len(found))
        clause = found[0]
        try_node = parents.get(id(clause))
        broad = [b for b in try_node.handlers if _handler_types(b) & BROAD]
        assert len(broad) == 1, len(broad)
        assert clause.lineno < broad[0].lineno, "目标点必须是「支在前、宽支在后」"
        return {"lines": src[file].split("\n"),
                "clause": (clause.lineno, clause.end_lineno, clause.col_offset),
                "broad": (broad[0].lineno, broad[0].end_lineno)}

    @classmethod
    def _cut(cls, mode, target=None):
        """对 target（默认 TARGET）那一格做一种手术，返回新语料（原语料不动）"""
        target = target or cls.TARGET
        src = sources()
        row = next((p for p in paired(src) if (p["file"], p["owner"]) == target), None)
        assert row is not None and row["guarded"], row
        where = cls._locate(src, target[0], target[1])
        lines = where["lines"]
        c0, c1, col = where["clause"]
        block = lines[c0 - 1:c1]
        indent = " " * (col + 4)
        if mode == "drop":
            src[target[0]] = "\n".join(lines[:c0 - 1] + lines[c1:])
            return src
        if mode == "reorder":
            rest = lines[:c0 - 1] + lines[c1:]
            shift = len(block)
            b1 = where["broad"][1] - shift
            src[target[0]] = "\n".join(rest[:b1] + block + rest[b1:])
            return src
        body = {"swallow": [indent + 'return {"ok": True}'],
                "recode": [indent + 'raise HTTPException(status_code=500, detail="降级")']}[mode]
        lines[c0:c1] = body
        src[target[0]] = "\n".join(lines)
        return src

    def test_arm_a_fresh_raiser_in_a_real_route_is_caught(self):
        """真语料上的闭包播种：往既有模块里追加一只全新的闸，漏支必须当场红

        合成新文件那一臂证的是「形状能被量到」，证不到的是另一格：**一只今天还不存在的
        raiser** 进了既有路由的宽收尾 try 时，闭包认不认得它。播种规则是「任何体里
        直接 ``raise HTTPException`` 的函数名都是种子」，所以新闸天然入面 —— 这一臂把
        这句话打在真模块上：往 ``api/routes/system_ops.py`` 尾部追加一只 ``gate_l265``
        和一条调用它的端点（无保码支），缺陷档必须恰好报它一处。
        """
        src = sources()
        assert missing(src) == [], "现量缺陷档必须先为空，否则『恰好一处』无从谈起"
        file = self.CLOSURE_TARGET
        assert "gate_l265" not in raisers(src), "前提：这一只今天不存在"
        before = len(src[file])
        src[file] += (
            "\n\n\ndef gate_l265(raw: str):\n"
            "    if not raw:\n"
            '        raise HTTPException(status_code=400, detail="空")\n'
            "    return raw\n"
            "\n\n"
            "async def fresh_handler(raw: str):\n"
            "    try:\n"
            "        return {\"raw\": gate_l265(raw)}\n"
            "    except Exception as e:\n"
            "        raise to_http_error(e) from e\n"
        )
        assert len(src[file]) > before
        known = raisers(src)
        assert "gate_l265" in known, "闭包没把新闸收进来 ⇒ 播种规则失效"
        rows = missing(src)
        assert len(rows) == 1, [(r["file"], r["owner"]) for r in rows]
        assert (rows[0]["file"], rows[0]["owner"]) == (file, "fresh_handler"), rows[0]
        assert rows[0]["names"] == frozenset({"gate_l265"}), rows[0]["names"]
        assert (file, "fresh_handler") in identity(paired(src)), "配对点没进面 ⇒ 缺陷档是巧合报的"
        assert rows[0]["guarded"] is False
        back = sources()
        assert len(back[file]) == before, "内存手术不得留残留（原语料对象必须未被改动）"

    def test_arm_dropping_a_load_only_site_is_blind_to_the_call_ruler(self):
        """装载维的牙齿打在**真语料**上：删掉 audit 那一格的支，调用维连问都问不出来

        ``api/routes/audit.py`` 的 try 体是
        ``items = await run_in_thread(load_items, request.input_file)`` ——raisers 是**参数**，
        不是被这个 try 直接调用的函数。调用维的尺子在这一格里既看不到可达 raiser，也看不到
        try，于是「这一格缺不缺支」在它的世界里不是一个成立的问题：整只 ``api/routes/audit.py``
        从配对面消失。装载维看到缺陷（下面第一档），调用维看到空集（第二档）——
        而「空集 == 空集」正是会绿的那种红。
        """
        src = self._cut("drop", self.BLIND_TARGET)
        rows = missing(src)
        assert len(rows) == 1, [(r["file"], r["owner"]) for r in rows]
        assert (rows[0]["file"], rows[0]["owner"]) == self.BLIND_TARGET, rows[0]
        assert rows[0]["names"] == frozenset({"load_items"}), rows[0]["names"]
        assert self.BLIND_TARGET[0] not in {p["file"] for p in paired(src, "call")}
        assert len(clauses(src)) == MEASURED_CLAUSES - 1, len(clauses(src))

    def test_the_target_is_a_guarded_paired_site(self):
        row = next((p for p in paired() if (p["file"], p["owner"]) == self.TARGET), None)
        assert row is not None and row["guarded"], row
        where = self._locate(sources(), self.TARGET[0], self.TARGET[1])
        assert where["clause"][0] < where["broad"][0]

    def test_arm_dropping_a_clause_makes_that_site_a_defect(self):
        src = self._cut("drop")
        rows = missing(src)
        assert len(rows) == 1, [(r["file"], r["owner"]) for r in rows]
        assert (rows[0]["file"], rows[0]["owner"]) == self.TARGET, rows[0]
        assert len(paired(src)) == MEASURED_PAIRED, "配对点还在，只是失去了保护"
        assert len(clauses(src)) == MEASURED_CLAUSES - 1, len(clauses(src))

    def test_arm_writing_the_clause_after_the_broad_one_does_not_count(self):
        """顺序承重：支仍在面里，但配对点已无保护 —— 只按「存在」判的尺子会漏这一格"""
        src = self._cut("reorder")
        rows = missing(src)
        assert len(rows) == 1 and (rows[0]["file"], rows[0]["owner"]) == self.TARGET, rows
        assert len(clauses(src)) == MEASURED_CLAUSES, "支没消失，只是排错了地方"

    def test_arm_turning_a_clause_into_a_swallow_moves_the_shape_tier(self):
        src = self._cut("swallow")
        got = {s: 0 for s in SHAPES}
        for c in clauses(src):
            got[c["shape"]] += 1
        assert got[SWALLOW] == 1 and got[BARE] == MEASURED_SHAPE_TALLY[BARE] - 1, got
        rows = swallowers(src)
        assert len(rows) == 1 and (rows[0]["file"], rows[0]["owner"]) == self.TARGET, rows

    def test_arm_re_raising_a_new_code_lands_the_swallow_tier_too(self):
        """换成 500 再抛同样致命：体里再没有裸 raise，判据必须认出它不是保码支"""
        src = self._cut("recode")
        rows = swallowers(src)
        assert len(rows) == 1 and rows[0]["shape"] == SWALLOW, rows

    def test_arm_a_new_route_without_the_clause_is_caught(self):
        """将来加一条路由、把闸直接写进宽收尾 try ⇒ 当场红（这一格才是守卫真正要防的）"""
        src = sources()
        src["api/routes/new_l265.py"] = (
            "from ..deps import resolve_data_path, to_http_error\n\n\n"
            "async def handler(name: str):\n"
            "    try:\n"
            "        p = resolve_data_path(name)\n"
            '        return {"path": str(p)}\n'
            "    except Exception as e:\n"
            "        raise to_http_error(e) from e\n"
        )
        rows = missing(src)
        assert len(rows) == 1 and rows[0]["file"] == "api/routes/new_l265.py", rows
        assert rows[0]["names"] == frozenset({"resolve_data_path"}), rows[0]["names"]

    def test_arm_a_load_that_is_not_a_raiser_keeps_the_site_out_of_the_face(self):
        """反证「配对面」不是恒真：try 体里只引用面外名字 ⇒ 这一格根本不进面"""
        src = sources()
        src["api/routes/new_l265.py"] = (
            "from ..deps import to_http_error\n\n\n"
            "async def handler(name: str):\n"
            "    try:\n"
            "        return other_thing(name)\n"
            "    except Exception as e:\n"
            "        raise to_http_error(e) from e\n"
        )
        assert [p for p in paired(src) if p["file"] == "api/routes/new_l265.py"] == []

    def test_arm_value_passed_raiser_still_counts(self):
        """装载维有牙齿：只按 ``Call`` 判的尺子会把 ``run_in_thread(load_items, ...)`` 整条漏掉"""
        src = sources()
        src["api/routes/pass_l265.py"] = (
            "from ..deps import load_items, to_http_error\n\n\n"
            "async def handler(name: str):\n"
            "    try:\n"
            "        return await run_in_thread(load_items, name)\n"
            "    except Exception as e:\n"
            "        raise to_http_error(e) from e\n"
        )
        rows = [p for p in paired(src) if p["file"] == "api/routes/pass_l265.py"]
        assert len(rows) == 1 and "load_items" in rows[0]["names"], rows
        assert len(missing(src)) == 1, missing(src)
        # 同一份语料、同一格，调用维看不见 ⇒ 「装载维多出来的那 13 只」不是噪声而是覆盖面
        assert [p for p in paired(src, "call") if p["file"] == "api/routes/pass_l265.py"] == []
        assert missing(src) == [rows[0]], "缺陷档报的就是装载维独量的那一格"

    def test_the_arms_bite_only_the_tier_each_one_claims(self):
        """四臂各打中一格，且打完之后盘面能原样读回来（语料是内存副本，磁盘未动）"""
        for mode in ("drop", "reorder", "swallow", "recode"):
            src = self._cut(mode)
            if mode in ("drop", "reorder"):
                assert len(missing(src)) == 1, mode
                assert swallowers(src) == [], mode
            else:
                assert len(swallowers(src)) == 1, mode
                assert missing(src) == [], mode
            back = sources()
            assert missing(back) == [] and swallowers(back) == []
            assert len(clauses(back)) == MEASURED_CLAUSES
            assert len(paired(back)) == MEASURED_PAIRED
