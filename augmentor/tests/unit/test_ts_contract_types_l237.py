# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L237：TS 类型面与后端契约面**逐只对账**——「后端在回、前端类型上不存在」这一族从此有尺子

**改前实测**（仓外探针，跑完即删；权威是 `app.openapi()` 与 `GET /api/config` 的真响应）：

| 事实 | 改前 | 改后 |
| --- | --- | --- |
| `services/api.ts` 里带 `Promise<T>` 的 fetcher | 75 只 | 75 只（一只未增未减） |
| 其中字段集与该操作契约 200 响应属性集**相等** | **72 只** | **75 只** |
| `AppConfig` 少的键 | 9 处（`versioning` / `visualization` 两整节，加 `vector` 3 键、`rag` 2 键、`multimodal` 2 键） | 0 |
| `StatusResponse` 少的键 | 2 处（`model_retry_count` / `model_retry_wait_seconds`） | 0 |
| `UpdateConfigResponse` 少的键 | 1 处（`ignored_keys`） | 0 |
| 机器对账的守卫 | **没有**（A281 立账时只是一份仓外一次性脚本） | 本文件 |
| 页面上看得见这些值 | 看不见 | 系统状态页两格重试读数、配置页一张只读回显卡 + 一张保存回执卡 |

**为什么是两层尺子，一层不够**：`ConfigResponse` 的九节在契约里全是
`{"type": "object", "additionalProperties": true}` 的自由字典 ⇒ 节内键**根本不进契约**，
「TS 属性集 == 契约属性集」这把尺子在节名那一层就停了。改前那 9 处缺口里只有 2 处
（两整节的节名）是它量得到的，另 7 处（`vector.dimension` / `vector.storage_dir` /
`vector.collection` / `rag.chunk_size` / `rag.chunk_overlap` /
`multimodal.image_extensions` / `multimodal.audio_extensions`）要用**活响应**当权威才看得见。
所以第二层直接打 `GET /api/config`、递归比对键树——这一层只碰只读端点，不写盘。

A281 立账时写的是「3 只缺字段」，那 3 只**逐格复核仍然成立**（`AppConfig` /
`UpdateConfigResponse` / `StatusResponse`），只是缺口比立账时数的多 7 处：立账那把尺子
只比到契约的属性集，节内键在它视野外。这不是立账数错，是尺子浅一层——两层都建起来之后
才量得全，这句话留在这里免得日后有人拿「立账写 3 处、怎么改了 12 处」当缺陷查。

**四条解析口径**（立账时点名「进仓前必须先解决」的那些形状，逐条定档）：

1. **联合别名**：`AugmentProgress = NoCheckpointProgress | AugmentProgressDetail` ⇒ 取各员的
   并集。改前 naive 正则对这只直接失效，现在它进得了对账。
2. **`extends`**：`VersionDetail extends VersionInfo` ⇒ 把父类型字段并进来（带环检测）。
3. **内联对象字面量**：只取**顶层**字段名（`vector: { enabled: boolean … }` 记作一个字段
   `vector`）；节内键归第二层活响应尺子管。第一版探针在这里栽过一次——把内联块压成一行
   再用「名字:」切，结果每块只切出第一个字段，于是 `num_threads` / `threshold` / `formats` /
   `backend` 全被误报成「后端在回、TS 没有」。所以本文件解析时**保留换行**，并且带一支
   尺子自证（`test_the_ruler_reads_back_known_shapes`）先证明自己读得对。
4. **数组返回型**：`getDemoData(): Promise<DataItem[]>` 对上契约
   `{"type": "array", "items": {"$ref": "…/DemoItem"}}` ⇒ 比**元素**的属性集。
   名字两侧不同（`DataItem` / `DemoItem`）不影响，比的是字段集；实测两侧同为
   `{instruction, input, output}`。

**路径配对口径**：fetcher 写的是 `baseURL: '/api'` 之下的相对路径，契约里带 `/api` 前缀 ⇒
两种都试。命中多个时按**段级规则**收敛，而不是「取第一个」：fetcher 那一段是字面量就要求
契约同段字面量相等，是 `${…}` 就要求契约同段是 `{param}`。这条规则是必需的——
`getVersion` 的 `/versions/${versionId}` 用纯通配会同时命中 `/api/versions/{version_id}`
与 `/api/versions/history`（改前探针实测「matched 2 ops」，只能进白名单），按段级规则收敛后
唯一命中前者，白名单因此**是空的**。

**白名单为空且必须仍可证伪**：`UNMEASURABLE` 是空集，但空集相等是一把会恒绿的尺子——
所以另有一支注入用例（`test_the_empty_unmeasured_roster_is_not_a_blind_ruler`）在**内存里**
把一只 fetcher 的 `Promise<>` 标注摘掉，断言它当场落进这份名单。整份文件零写入：所有注入
都是把源码读进来、在字符串上改、再喂给同一批纯函数。

跑法：`python -m pytest tests/unit/test_ts_contract_types_l237.py -q --no-cov`
"""

import functools
import pathlib
import re

import pytest

from tests.unit.test_wired_dropdown_choices_l226 import strip_comments

REPO = pathlib.Path(__file__).resolve().parents[2]
WEB_SRC = REPO / "web" / "src"
SERVICES_TS = WEB_SRC / "services" / "api.ts"
TYPES_TS = WEB_SRC / "types" / "api.ts"
SETTINGS_TSX = WEB_SRC / "pages" / "Settings.tsx"
SYSTEM_TSX = WEB_SRC / "pages" / "System.tsx"

needs_web_src = pytest.mark.skipif(
    not (SERVICES_TS.is_file() and TYPES_TS.is_file()),
    reason="前端源码不在场（只带构建产物的安装没有 web/）",
)

# ---------------------------------------------------------------------------
# TS 面解析
# ---------------------------------------------------------------------------

INTERFACE = re.compile(r"^export interface (\w+)(?:\s+extends\s+([\w,\s]+?))?\s*\{", re.M)
ALIAS = re.compile(r"^export type (\w+)\s*=\s*(.+)$", re.M)
FIELD_LINE = re.compile(r"^  (\w+)(\?)?:\s*(.*)$")
FREE_FORM = ("Record<", "any", "unknown", "{ [key: string]")


class TsFace:
    """`types/api.ts` 解析出来的那张脸：字段表、继承表、别名表"""

    def __init__(self, source: str):
        src = strip_comments(source)
        self.fields: dict[str, dict[str, str]] = {}
        self.parents: dict[str, list[str]] = {}
        self.aliases: dict[str, str] = {}
        for m in ALIAS.finditer(src):
            self.aliases[m.group(1)] = m.group(2).strip()
        for m in INTERFACE.finditer(src):
            name = m.group(1)
            self.parents[name] = [
                p.strip() for p in (m.group(2) or "").split(",") if p.strip()
            ]
            self.fields[name] = self._body_fields(src, m.end() - 1)

    @staticmethod
    def _body_fields(src: str, open_brace: int) -> dict[str, str]:
        """花括号平衡取出 interface 体，再按顶层缩进逐字段抓「名字 → 类型表达式」

        内联对象字面量跨多行，取值时**保留换行**（压成一行会让 `parse_inline` 只切出
        第一个字段，见模块 docstring 口径 3）。
        """
        depth = 0
        close = open_brace
        for i in range(open_brace, len(src)):
            if src[i] == "{":
                depth += 1
            elif src[i] == "}":
                depth -= 1
                if depth == 0:
                    close = i
                    break
        out: dict[str, str] = {}
        lines = src[open_brace + 1 : close].split("\n")
        k = 0
        while k < len(lines):
            fm = FIELD_LINE.match(lines[k])
            if not fm:
                k += 1
                continue
            name, opt, rest = fm.group(1), fm.group(2), fm.group(3).strip()
            if rest.startswith("{"):
                buf = [rest]
                bal = rest.count("{") - rest.count("}")
                while bal > 0 and k + 1 < len(lines):
                    k += 1
                    buf.append(lines[k].strip())
                    bal += lines[k].count("{") - lines[k].count("}")
                expr = "\n".join(buf)
            else:
                expr = rest.rstrip(",")
            out[name] = ("?" if opt else "") + expr
            k += 1
        return out

    def props(self, name: str, seen=()) -> set[str] | None:
        """类型名 → 顶层属性集；解析不动回 `None`（调用方要出声，不许静默跳过）"""
        name = name.strip()
        if name.endswith("[]"):
            return self.props(name[:-2], seen)
        if name in seen:
            return set()
        if name in self.aliases:
            rhs = self.aliases[name]
            if "|" not in rhs:
                return None
            out: set[str] = set()
            for part in rhs.split("|"):
                got = self.props(part.strip(), seen + (name,))
                if got is None:
                    return None
                out |= got
            return out
        if name not in self.fields:
            return None
        out = set(self.fields[name])
        for parent in self.parents.get(name, []):
            got = self.props(parent, seen + (name,))
            if got is None:
                return None
            out |= got
        return out


def parse_inline(expr: str) -> dict[str, str] | None:
    """内联对象字面量 → 字段表；多行按行切，单行按顶层逗号切"""
    expr = expr.lstrip("?").strip()
    if not expr.startswith("{"):
        return None
    body = expr[1 : expr.rfind("}")]
    units = [u for u in body.split("\n") if u.strip()]
    if len(units) <= 1:
        units, cur, depth = [], "", 0
        for ch in body:
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
            if ch == "," and depth == 0:
                units.append(cur)
                cur = ""
            else:
                cur += ch
        if cur.strip():
            units.append(cur)
    out: dict[str, str] = {}
    for unit in units:
        fm = re.match(r"^(\w+)(\?)?:\s*(.+?)\s*,?$", unit.strip(), re.S)
        if fm:
            out[fm.group(1)] = ("?" if fm.group(2) else "") + fm.group(3)
    return out or None


# ---------------------------------------------------------------------------
# fetcher 面解析
# ---------------------------------------------------------------------------

EXPORT_CONST = re.compile(r"^export const (\w+) ", re.M)
PROMISE = re.compile(r"Promise<([^>\n]+)>")
API_CALL = re.compile(r"api\.(get|post|put|delete|patch)\(\s*[`'\"]([^`'\"]*)[`'\"]")


def parse_fetchers(source: str) -> list[dict]:
    """切块取 fetcher：名字、返回类型标注、方法、路径、块内调用次数

    「fetcher」的判据是**块内出现 `api.<method>(` 调用**，不是「有没有 `Promise<>`」——
    后者当判据的话，摘掉标注就等于把这只 fetcher 从尺子上摘掉（静默跳过），
    正是本文件要禁的形状。
    """
    src = strip_comments(source)
    marks = [(m.start(), m.group(1)) for m in EXPORT_CONST.finditer(src)]
    out = []
    for idx, (pos, name) in enumerate(marks):
        end = marks[idx + 1][0] if idx + 1 < len(marks) else len(src)
        chunk = src[pos:end]
        calls = API_CALL.findall(chunk)
        if not calls:
            continue
        promise = PROMISE.search(chunk)
        out.append({
            "name": name,
            "ret": promise.group(1).strip() if promise else None,
            "method": calls[0][0].lower(),
            "path": calls[0][1],
            "call_count": len(calls),
        })
    return out


# ---------------------------------------------------------------------------
# 契约面
# ---------------------------------------------------------------------------


@functools.lru_cache(maxsize=1)
def openapi() -> dict:
    from api.main import app

    return app.openapi()


def resolve_operation(method: str, path: str) -> list[tuple[str, dict]]:
    """按段级规则把 fetcher 路径配到契约操作上（见模块 docstring「路径配对口径」）"""
    spec = openapi()
    base = re.sub(r"\?.*$", "", path)
    want = [s for s in base.split("/") if s]
    hits = []
    for route, ops in spec["paths"].items():
        if method not in ops:
            continue
        for prefix in ("", "/api"):
            if not route.startswith(prefix):
                continue
            tail = route[len(prefix):] if prefix else route
            got = [s for s in tail.split("/") if s]
            if len(got) != len(want):
                continue
            if all(_segment_ok(w, g) for w, g in zip(want, got)):
                hits.append((route, ops[method]))
                break
    return hits


def _segment_ok(fetcher_seg: str, contract_seg: str) -> bool:
    if fetcher_seg.startswith("${") and fetcher_seg.endswith("}"):
        return contract_seg.startswith("{") and contract_seg.endswith("}")
    return fetcher_seg == contract_seg


def response_props(op: dict) -> tuple[set[str] | None, str]:
    """契约 200 响应的属性集；数组型取元素的属性集

    返回 `(集合 | None, 标签)`，标签用于失败信息里点名形状。
    """
    schemas = openapi()["components"]["schemas"]
    r200 = op.get("responses", {}).get("200")
    if r200 is None:
        return None, "no-200"
    content = r200.get("content", {})
    if not content:
        return None, "no-content"
    sch = next(iter(content.values()))["schema"]
    if sch.get("type") == "array":
        sch = sch.get("items", {})
    name = None
    if "$ref" in sch:
        name = sch["$ref"].rsplit("/", 1)[-1]
    elif len(sch.get("allOf", [])) == 1 and "$ref" in sch["allOf"][0]:
        name = sch["allOf"][0]["$ref"].rsplit("/", 1)[-1]
    if name is None:
        return None, f"not-named({sch.get('type', '?')})"
    return set(schemas[name].get("properties", {})), name


# ---------------------------------------------------------------------------
# 对账（纯函数：喂进两份源码文本，吐回读数）
# ---------------------------------------------------------------------------

#: 量不到的 fetcher 名册。**空集**，每员都要机器验真（见对应的注入用例）。
UNMEASURABLE: frozenset[str] = frozenset()


def reconcile(services_src: str, types_src: str) -> dict:
    """一次跑完第一层对账，把所有读数摊开

    返回 `rows`（逐只对账结果）、`mismatched`（字段集不等的）、`unannotated`
    （是 fetcher 却没有 `Promise<>` 标注的）、`unresolved`（配不到唯一操作、
    或 TS/契约有一侧解析不动的）、`multi_call`（一块里发多次调用的）。
    """
    ts = TsFace(types_src)
    rows, mismatched, unannotated, unresolved, multi_call = [], [], [], [], []
    for f in parse_fetchers(services_src):
        if f["call_count"] != 1:
            multi_call.append((f["name"], f["call_count"]))
        if f["ret"] is None:
            unannotated.append(f["name"])
            continue
        hits = resolve_operation(f["method"], f["path"])
        if len(hits) != 1:
            unresolved.append((f["name"], f"{len(hits)} ops for {f['method']} {f['path']}"))
            continue
        route, op = hits[0]
        tset = ts.props(f["ret"])
        if tset is None:
            unresolved.append((f["name"], f"TS 解析不动 {f['ret']}"))
            continue
        cset, cname = response_props(op)
        if cset is None:
            unresolved.append((f["name"], f"契约 {cname}"))
            continue
        row = {
            "name": f["name"], "ret": f["ret"], "op": f"{f['method'].upper()} {route}",
            "contract": cname, "ts": tset, "contract_props": cset,
        }
        rows.append(row)
        if tset != cset:
            mismatched.append({
                **row,
                "ts_missing": sorted(cset - tset),
                "ts_extra": sorted(tset - cset),
            })
    return {
        "rows": rows, "mismatched": mismatched, "unannotated": unannotated,
        "unresolved": unresolved, "multi_call": multi_call, "ts": ts,
    }


@pytest.fixture(scope="module")
def face() -> dict:
    return reconcile(
        SERVICES_TS.read_text(encoding="utf-8"), TYPES_TS.read_text(encoding="utf-8")
    )


def live_config() -> dict:
    """`GET /api/config` 的真响应（只读端点，不写盘）"""
    from fastapi.testclient import TestClient

    from api.main import app

    resp = TestClient(app).get("/api/config")
    assert resp.status_code == 200, resp.text
    return resp.json()


def diff_key_tree(ts: TsFace, type_name: str, value, path: str,
                  missing: list, extra: list) -> None:
    """递归比对活响应键树与 TS 声明的键树

    自由字典 / 数组 / 解析不动的那一层不比——比不了，且那一层归第一层尺子管。
    """
    if not isinstance(value, dict):
        return
    fields = ts.fields.get(type_name)
    if fields is None:
        return
    for key, sub in value.items():
        here = f"{path}.{key}" if path else key
        if key not in fields:
            missing.append(here)
            continue
        expr = fields[key].lstrip("?").strip()
        if expr.startswith("{"):
            inline = parse_inline(expr)
            if inline is None or not isinstance(sub, dict):
                continue
            for sub_key, sub_value in sub.items():
                deep = f"{here}.{sub_key}"
                if sub_key not in inline:
                    missing.append(deep)
                elif isinstance(sub_value, dict):
                    named = inline[sub_key].lstrip("?").strip()
                    if named in ts.fields:
                        diff_key_tree(ts, named, sub_value, deep, missing, extra)
            for sub_key in inline:
                if sub_key not in sub:
                    extra.append(f"{here}.{sub_key}")
        elif isinstance(sub, dict) and expr in ts.fields:
            diff_key_tree(ts, expr, sub, here, missing, extra)
    for key in fields:
        if key not in value:
            extra.append(f"{path}.{key}" if path else key)


# ---------------------------------------------------------------------------
# 第 0 面：尺子自己先站得住
# ---------------------------------------------------------------------------


@needs_web_src
class TestTheRulerStandsUpFirst:
    """尺子自证：读数之前先证明它读得对，否则后面的相等断言全是摆设"""

    def test_the_ruler_reads_back_known_shapes(self):
        """四种失效形状各钉一处：联合别名、`extends`、内联字面量、数组元素"""
        ts = TsFace(TYPES_TS.read_text(encoding="utf-8"))
        # 联合别名 ⇒ 两名成员的并集
        union = ts.props("AugmentProgress")
        assert union is not None
        assert union == (ts.props("NoCheckpointProgress") | ts.props("AugmentProgressDetail"))
        assert len(union) > len(ts.props("NoCheckpointProgress"))
        # extends ⇒ 父类型字段并进来了
        detail = ts.props("VersionDetail")
        assert detail is not None
        assert ts.props("VersionInfo") <= detail
        assert detail > ts.props("VersionInfo")
        # 内联字面量 ⇒ 每块**全部**字段都切得出来（第一版栽在这里）
        cfg = ts.fields["AppConfig"]
        assert set(parse_inline(cfg["vector"])) == {
            "enabled", "backend", "dimension", "storage_dir", "collection"}
        assert set(parse_inline(cfg["augmentation"])) == {"variants_per_seed", "num_threads"}
        assert set(parse_inline(cfg["multimodal"])) == {
            "enabled", "image_extensions", "audio_extensions"}
        # 数组返回型 ⇒ 剥掉 `[]` 之后照旧解析
        assert ts.props("DataItem[]") == ts.props("DataItem") is not None

    def test_the_fetcher_scan_is_not_vacuous(self, face):
        """扫描面要有量：83 只 `export const` 里 75 只是 fetcher，一只都不能少扫

        数字与改前实测同值——本轮只补类型与渲染，服务层一只 fetcher 未增未减。
        """
        src = strip_comments(SERVICES_TS.read_text(encoding="utf-8"))
        assert len(EXPORT_CONST.findall(src)) == 83
        assert len(API_CALL.findall(src)) == 75
        assert len(face["rows"]) + len(face["unannotated"]) + len(face["unresolved"]) == 75
        assert face["multi_call"] == []
        names = {r["name"] for r in face["rows"]}
        assert {"getConfig", "updateConfig", "getServiceStatus", "getDemoData"} <= names


# ---------------------------------------------------------------------------
# 第 1 面：每只 fetcher 的字段集 == 该操作契约 200 响应的属性集
# ---------------------------------------------------------------------------


@needs_web_src
class TestEveryFetcherMatchesItsOwnOperation:
    """第一层尺子：静态、零运行，75 只逐只对账"""

    def test_no_fetcher_field_set_differs_from_its_contract(self, face):
        """名单相等（空集）。改前这里有 3 员，逐格是 A281 立账那三只"""
        assert face["mismatched"] == [], [
            (m["name"], m["op"], m["contract"], m["ts_missing"], m["ts_extra"])
            for m in face["mismatched"]
        ]

    def test_no_fetcher_is_left_unannotated(self, face):
        """是 fetcher 就必须带 `Promise<T>`：没标注等于从尺子上静默消失"""
        assert face["unannotated"] == []

    def test_every_fetcher_resolves_to_exactly_one_operation(self, face):
        """配不到唯一操作就出声，不许「取第一个」糊过去

        改前 `getVersion` 用纯通配会同时命中 `/api/versions/{version_id}` 与
        `/api/versions/history`；段级规则收敛之后这一格是空的。
        """
        assert face["unresolved"] == []

    def test_the_measured_scale_is_pinned(self, face):
        """规模钉：75 只对账、75 只相等。多一只少一只都要在这里响"""
        assert len(face["rows"]) == 75
        assert len(face["mismatched"]) == 0

    def test_the_three_a281_targets_are_now_whole(self, face):
        """A281 那三只逐个点名：缺口补齐之后属性集与契约**逐格相等**"""
        by_name = {r["name"]: r for r in face["rows"]}
        for fetcher, contract in (
            ("getConfig", "ConfigResponse"),
            ("updateConfig", "ConfigUpdateResponse"),
            ("getServiceStatus", "StatusResponse"),
        ):
            row = by_name[fetcher]
            assert row["contract"] == contract
            assert row["ts"] == row["contract_props"]
        # 补齐的那些键此刻**在**类型里（不是「两边都少了所以相等」那种假绿）
        ts = face["ts"]
        assert {"versioning", "visualization"} <= ts.props("AppConfig")
        assert {"dimension", "storage_dir", "collection"} <= set(parse_inline(ts.fields["AppConfig"]["vector"]))
        assert {"chunk_size", "chunk_overlap"} <= set(parse_inline(ts.fields["AppConfig"]["rag"]))
        assert {"image_extensions", "audio_extensions"} <= set(
            parse_inline(ts.fields["AppConfig"]["multimodal"]))
        assert {"model_retry_count", "model_retry_wait_seconds"} <= ts.props("StatusResponse")
        assert "ignored_keys" in ts.fields["UpdateConfigResponse"]

    def test_the_empty_unmeasured_roster_is_not_a_blind_ruler(self):
        """空集也要会响：内存里摘掉一只标注，它必须当场落进名单

        零写入——把源码读进来、在字符串上删掉那一行、喂给同一批纯函数。
        """
        assert UNMEASURABLE == frozenset()
        services = SERVICES_TS.read_text(encoding="utf-8")
        types = TYPES_TS.read_text(encoding="utf-8")
        anchor = "export const getServiceStatus = async (): Promise<StatusResponse> => {"
        assert services.count(anchor) == 1, "锚点不唯一 ⇒ 判据要跟着换形状"
        doctored = reconcile(services.replace(anchor, "export const getServiceStatus = async () => {"), types)
        assert doctored["unannotated"] == ["getServiceStatus"]
        assert all(r["name"] != "getServiceStatus" for r in doctored["rows"])

    def test_dropping_one_field_turns_the_equality_red(self):
        """内存里从类型上删掉一个字段，第一层尺子必须点名喊红

        这一支钉的是「相等断言不是恒真」：字段集真差一格就得响，且响的正是那一格。
        """
        services = SERVICES_TS.read_text(encoding="utf-8")
        types = TYPES_TS.read_text(encoding="utf-8")
        anchor = "  model_retry_count: number | null\n"
        assert types.count(anchor) == 1, "锚点不唯一 ⇒ 判据要跟着换形状"
        doctored = reconcile(services, types.replace(anchor, ""))
        assert [m["name"] for m in doctored["mismatched"]] == ["getServiceStatus"]
        assert doctored["mismatched"][0]["ts_missing"] == ["model_retry_count"]
        assert doctored["mismatched"][0]["ts_extra"] == []

    def test_a_ghost_type_name_is_reported_not_skipped(self):
        """把返回类型改成契约里不存在的名字 ⇒ 进 `unresolved` 而不是静默相等"""
        services = SERVICES_TS.read_text(encoding="utf-8")
        types = TYPES_TS.read_text(encoding="utf-8")
        anchor = "Promise<UpdateConfigResponse>"
        assert services.count(anchor) == 1
        doctored = reconcile(services.replace(anchor, "Promise<NoSuchTypeAtAll>"), types)
        assert ("updateConfig", "TS 解析不动 NoSuchTypeAtAll") in doctored["unresolved"]


# ---------------------------------------------------------------------------
# 第 2 面：契约声明为自由字典的那一族，只有活响应量得到
# ---------------------------------------------------------------------------


@needs_web_src
class TestTheFreeDictionaryFaceNeedsTheLiveResponse:
    """第二层尺子：`GET /api/config` 的真响应键树 vs TS 声明的键树（递归）

    只打只读端点，不写盘。这一层存在的理由写在模块 docstring 里：契约把九节全声明成
    `additionalProperties: true`，节内键不进契约 ⇒ 第一层尺子在节名那一层就停了。
    """

    def test_the_backend_returns_no_key_the_type_does_not_declare(self):
        """缺键名单相等（空集）。改前这里是 9 员，全在 `AppConfig` 上"""
        ts = TsFace(TYPES_TS.read_text(encoding="utf-8"))
        missing, extra = [], []
        diff_key_tree(ts, "AppConfig", live_config(), "", missing, extra)
        assert missing == [], missing
        assert extra == [], extra

    def test_the_live_face_actually_descends_into_the_sections(self):
        """反证：这一层真的比到了节内键，而不是在节名那一层就停了

        数一遍比过的键——十节 + 节内键，总数必须到得了两位数；否则「空集相等」
        只是因为递归没下去。
        """
        live = live_config()
        assert set(live) == {
            "default_model", "augmentation", "quality", "dedup", "export",
            "vector", "rag", "multimodal", "versioning", "visualization"}
        nested = sum(len(v) for k, v in live.items() if isinstance(v, dict))
        assert nested >= 17, nested
        # 契约面确实看不见这些键：九节全是自由字典，没有 `properties`
        schema = openapi()["components"]["schemas"]["ConfigResponse"]["properties"]
        for section in ("vector", "rag", "multimodal", "versioning", "visualization"):
            assert schema[section].get("additionalProperties") is True
            assert "properties" not in schema[section]

    def test_doctoring_the_type_turns_the_live_face_red(self):
        """内存里删掉一节 / 一个节内键，第二层尺子都要点名喊红"""
        src = TYPES_TS.read_text(encoding="utf-8")
        anchor = "  versioning: {\n    enabled: boolean\n  }\n"
        assert src.count(anchor) == 1, "锚点不唯一 ⇒ 判据要跟着换形状"
        missing, extra = [], []
        diff_key_tree(TsFace(src.replace(anchor, "")), "AppConfig",
                      live_config(), "", missing, extra)
        assert "versioning" in missing
        # 节内键同样量得到：单删 `chunk_size` 一个键，报的就是那一个
        anchor2 = "    chunk_size: number\n"
        assert src.count(anchor2) == 1
        missing2, extra2 = [], []
        diff_key_tree(TsFace(src.replace(anchor2, "")), "AppConfig",
                      live_config(), "", missing2, extra2)
        assert missing2 == ["rag.chunk_size"]
        assert extra2 == []


# ---------------------------------------------------------------------------
# 改前形状复现：docstring 那张表不是抄进来的数字，是尺子自己重跑出来的
# ---------------------------------------------------------------------------

#: 本轮补进类型面的 12 处，逐格锚点唯一（不唯一就当场响，不许悄悄改判据）
ADDED_IN_L237 = [
    "    dimension: number\n",
    "    storage_dir: string\n",
    "    collection: string\n",
    "    chunk_size: number\n",
    "    chunk_overlap: number\n",
    "    image_extensions: string[]\n",
    "    audio_extensions: string[]\n",
    "  model_retry_count: number | null\n",
    "  model_retry_wait_seconds: number | null\n",
    "  ignored_keys?: string[]\n",
    "  versioning: {\n    enabled: boolean\n  }\n"
    "  visualization: {\n    enabled: boolean\n  }\n",
]


def pre_change_types() -> str:
    """把本轮补进去的那些键从类型面上摘掉，还原改前的形状（全程在内存里）"""
    src = TYPES_TS.read_text(encoding="utf-8")
    for anchor in ADDED_IN_L237:
        assert src.count(anchor) == 1, f"锚点不唯一或已不在场：{anchor!r}"
        src = src.replace(anchor, "")
    return src


@needs_web_src
class TestThePreChangeReadingIsReproducible:
    """两层尺子在改前的类型面上各自报出**互不重叠**的那一半缺口

    这一支是模块 docstring 那张表的机器版：改前 12 处缺口里，第一层量到 5 处
    （两整节的节名 + `ignored_keys` + 两键重试），第二层量到另 7 处（全在自由字典节内）。
    两层加起来才是 12，少任何一层都会漏。
    """

    def test_layer_one_sees_exactly_the_five_the_contract_carries(self):
        got = reconcile(SERVICES_TS.read_text(encoding="utf-8"), pre_change_types())
        assert {m["name"]: m["ts_missing"] for m in got["mismatched"]} == {
            "getConfig": ["versioning", "visualization"],
            "updateConfig": ["ignored_keys"],
            "getServiceStatus": ["model_retry_count", "model_retry_wait_seconds"],
        }
        assert all(m["ts_extra"] == [] for m in got["mismatched"])
        assert sum(len(m["ts_missing"]) for m in got["mismatched"]) == 5

    def test_layer_two_sees_the_seven_the_contract_cannot(self):
        missing, extra = [], []
        diff_key_tree(TsFace(pre_change_types()), "AppConfig",
                      live_config(), "", missing, extra)
        assert sorted(missing) == [
            "multimodal.audio_extensions", "multimodal.image_extensions",
            "rag.chunk_overlap", "rag.chunk_size",
            "vector.collection", "vector.dimension", "vector.storage_dir",
            "versioning", "visualization",
        ]
        assert extra == []
        # 两层不重叠的证据：第二层报的 9 员里，节内那 7 员第一层一个都看不见
        layer_one = {
            k for m in reconcile(SERVICES_TS.read_text(encoding="utf-8"),
                                 pre_change_types())["mismatched"]
            if m["name"] == "getConfig" for k in m["ts_missing"]
        }
        assert layer_one == {"versioning", "visualization"}
        assert len(set(missing) - layer_one) == 7

    def test_both_layers_go_quiet_on_the_current_source(self, face):
        """反证：改后两层都归空集——否则「改前会响」可能只是尺子恒红"""
        assert face["mismatched"] == []
        missing, extra = [], []
        diff_key_tree(face["ts"], "AppConfig", live_config(), "", missing, extra)
        assert missing == [] and extra == []


# ---------------------------------------------------------------------------
# 第 3 面：补了类型不渲染，等于新开一个「写得进但没人读」的死面
# ---------------------------------------------------------------------------

def card_block(source: str, title: str) -> str:
    """按卡标题切出那一张 `Card` 的整块（到它自己的 `</Card>` 为止）

    禁令型断言的扫描面必须与禁令面同族：整页扫「不许出现 `?? 0`」会被无关的邻居判假红，
    所以只在这一张卡里扫。
    """
    marker = f'title="{title}"'
    assert source.count(marker) == 1, f"页面上「{title}」这张卡不唯一或已不在场"
    head, _, tail = source.partition(marker)
    assert tail, title
    end = tail.index("</Card>")
    return head[head.rindex("<Card"):] + tail[: end + len("</Card>")]


@needs_web_src
class TestTheRenderingFaceOwnsWhatTheTypesNowCarry:
    """类型面补齐之后，页面上必须真有读数——否则这一轮只是把死面从后端挪到前端"""

    def test_the_two_retry_keys_are_rendered_and_null_stays_apart_from_zero(self):
        """`null` 要渲染成「无后端」，不是 0：后端那两键的语义是「没有后端」vs「重试过 0 次」"""
        block = card_block(SYSTEM_TSX.read_text(encoding="utf-8"), "服务状态")
        for key in ("model_retry_count", "model_retry_wait_seconds"):
            assert f"status.{key} === null" in block, key
            assert f"status.{key}" in block
        assert block.count("'无后端'") == 2
        # 同一张卡里不许出现把 null 折成 0 的哨兵（扫描面已收窄到这张卡）
        assert "?? 0" not in block
        assert "|| 0" not in block

    def test_the_readonly_card_derives_from_the_response_keys(self):
        """只读回显卡的内容是从响应键里**扣掉有表单的三节**得来的，不是手抄一份节名清单

        手抄的话后端加一节这里就静默少一节——那正是本文件要禁的形状。
        """
        src = SETTINGS_TSX.read_text(encoding="utf-8")
        declared = re.findall(r"const FORM_SECTIONS = \[([^\]]*)\]", src)
        assert len(declared) == 1
        form_sections = set(re.findall(r"'(\w+)'", declared[0]))
        # 与页面上真有输入控件的那几节相等：控件是 `patch(c => ({ ...c.<节>, … }))` 这个形状
        patched = set(re.findall(r"(\w+):\s*\{\s*\.\.\.c\.\1", src, re.S))
        assert form_sections == patched == {"augmentation", "quality", "dedup"}
        assert "!FORM_SECTIONS.includes(k)" in src
        assert "Object.keys(config)" in src

    def test_the_readonly_card_faces_exactly_the_sections_the_backend_returns(self):
        """把页面的推导规则套在活响应上，得到的六节要与本轮补齐的那六节逐格相等"""
        live = live_config()
        form_sections = {"augmentation", "quality", "dedup"}
        echoed = {k for k in live if k != "default_model" and k not in form_sections}
        assert echoed == {
            "export", "vector", "rag", "multimodal", "versioning", "visualization"}
        # 这六节里的键此刻都在类型面上（不然回显卡渲染出来是 `undefined`）
        ts = TsFace(TYPES_TS.read_text(encoding="utf-8"))
        for section in echoed:
            declared = set(parse_inline(ts.fields["AppConfig"][section]))
            assert declared == set(live[section]), (section, declared, set(live[section]))

    def test_the_save_receipt_card_renders_both_keys_of_the_response(self):
        """保存回执卡渲染后端真回的 `message` 与 `ignored_keys`，不是页面自己拼一句话"""
        block = card_block(SETTINGS_TSX.read_text(encoding="utf-8"), "上次保存的回执")
        assert "saveResult.message" in block
        assert "saveResult.ignored_keys" in block
        assert block.count("ignored_keys") >= 3

    def test_the_pages_do_not_invent_a_second_default(self):
        """两页都不许给这几格写死第二套默认值（初值只能来自响应）

        扫描面收窄到两张卡；`?? []` 在回执卡上是**合法**的（契约里 `ignored_keys`
        非必填、默认 `[]`），所以这一支只禁数字型哨兵，不禁空数组兜底。
        """
        settings_card = card_block(SETTINGS_TSX.read_text(encoding="utf-8"), "其余节的生效值（本页只读）")
        assert "?? 0" not in settings_card
        assert "|| 0" not in settings_card
        assert "readonlySections" in settings_card
