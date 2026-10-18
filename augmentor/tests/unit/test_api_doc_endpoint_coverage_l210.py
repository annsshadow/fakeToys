# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L210：`docs/augmentor/API.md` 必须覆盖 **每一个** 端点

缺口来源：账本 §3.3 立的「为 `dataset` / `system` 两组补人读字段说明」。实测是
**27 个端点**（dataset 14 + system 13，比立项时写的 25 多两条——`dataset/impact`
与 `dataset/evaluate` 是 L8 之后新增的）在 API.md 里只有端点总览的一行表格，
**没有任何字段说明**。而 API.md 的详细章节（1–11）是按旧分组写的，整整少了两组。

**这类缺口不会让任何测试变红**：端点总览有行、OpenAPI 有 schema、契约测试只验
「响应键 == 模型键」，没有一个判据看「有没有人读说明」。

本守卫把三件事钉成集合相等：
1. API.md 里被文档化的路径集合 == OpenAPI 的真实路径集合（两个方向）；
2. 之前完全没说明的两组（dataset / system）现在**每条都要有说明**；
3. 「路径集合相等」本身防空转（端点总数 > 0 且 >= 68）。
"""

import re
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
#: 子项目文档已于 2026-10-14 汇总到仓库根 `docs/augmentor/`；本文件自持这一真源
DOCS_DIR = ROOT.parent / "docs" / "augmentor"
API_DOC = DOCS_DIR / "API.md"

#: API.md 里形如 `POST` | `/api/xxx` 的表格行
DOC_ROW = re.compile(
    r"^\|\s*`(GET|POST|PUT|DELETE|PATCH)`\s*\|\s*`(/api/[a-z0-9_/\-{}]+)`",
    re.M)

#: **只匹配详细说明表**（四列：方法/路径/说明/人读提示）。总览表只有三列。
#: 混在一起取会让守卫退化成「路径在文档里出现过就行」，而总览那一行就满足它
#: ⇒ 详细说明整段删光仍然全绿（L210 注入实测抓到过这个漏洞）。
DOC_ROW_DETAILED = re.compile(
    r"^\|\s*`(GET|POST|PUT|DELETE|PATCH)`\s*\|\s*`(/api/[a-z0-9_/\-{}]+)`"
)

#: 详细说明表的行有 **5 个 `|`**（四列）；总览表只有 4 个（三列）。用列数区分，
#: 比在正则里数后续列更稳（CRLF + 中文会让 `[^|]*` 的行为变得难猜）。
#:
#: **L278 起这里数的是「未转义」竖线**：GFM 规范写明单元格内容里的竖线必须转义，
#: 「包括在其他内联标记内部」⇒ 行内码 `` `a|b` `` 里的裸竖线照样把那一行切成第 5 列，
#: 而 `` `a\|b` `` 不会。本轮就栽在裸竖线上：`docs/augmentor/API.md` 的搜索行正文里写了
#: `(a|aa)+b`，这一格数出 6 枚 ⇒ 整行被下面的列数过滤**静默丢掉**，`/api/dataset/search`
#: 从详细层消失（l210 与 l223 同时红，结构守卫那一维却全绿 ⇒ 已把它的口径一并改成 GFM）。
#: 两把尺子共引本文件这一对助手，不许各写一份。
DETAILED_COLUMN_COUNT = 5
UNESCAPED_PIPE = re.compile(r"(?<!\\)\|")


def column_count(line):
    """一行里有几枚**未转义**竖线（= GFM 会切出的分隔符数）"""
    return len(UNESCAPED_PIPE.findall(line))


def split_cells(line):
    """按未转义竖线切单元格：先把反斜杠竖线换成占位符再切，切完还原"""
    guarded = line.strip().strip("|").replace("\\|", "\x00")
    return [cell.replace("\x00", "|").strip() for cell in guarded.split("|")]


def live_paths():
    import sys
    sys.path.insert(0, str(ROOT))
    from api.main import app

    spec = app.openapi()
    return {p for p in spec["paths"]}



def documented_path_set():
    """API.md 里**出现过**的全部路径（总览表即算）——用于总览层判据"""
    text = API_DOC.read_text(encoding="utf-8")
    found = set()
    for line in text.splitlines():
        m = DOC_ROW.match(line)
        if m:
            found.add(m.group(2))
    return found


def detailed_path_set():
    """**四列详细说明表**里的路径（按列数 5 过滤）——用于详细层判据

    总览表只有三列（4 个 `|`），详细表有四列（5 个 `|`）。用列数区分比在正则里
    数后续列稳（CRLF + 中文会让 `[^|]*` 的行为难猜，实测栽过一次）。
    数的口径是**未转义**竖线，见上面 `DETAILED_COLUMN_COUNT` 那一段。
    """
    text = API_DOC.read_text(encoding="utf-8")
    found = set()
    for line in text.splitlines():
        if column_count(line) != DETAILED_COLUMN_COUNT:
            continue
        m = DOC_ROW.match(line)
        if m:
            found.add(m.group(2))
    return found


def paths_of_tag(tag):
    import sys
    sys.path.insert(0, str(ROOT))
    from api.main import app

    spec = app.openapi()
    out = set()
    for path, methods in spec["paths"].items():
        for op in methods.values():
            if tag in op.get("tags", []):
                out.add(path)
    return out


def operations_of_tag(tag):
    """按 `(方法, 路径)` 计的端点操作数"""
    import sys
    sys.path.insert(0, str(ROOT))
    from api.main import app

    spec = app.openapi()
    out = set()
    for path, methods in spec["paths"].items():
        for m, op in methods.items():
            if tag in op.get("tags", []):
                out.add((m.upper(), path))
    return out


class TestEveryEndpointIsDocumented:
    """文档必须覆盖 OpenAPI 的每一个路径"""

    def test_no_live_path_is_missing_from_the_doc(self):
        missing = sorted(live_paths() - documented_path_set())
        assert not missing, (
            f"这些端点在 API.md 里没有任何说明：{missing}。"
            "这正是 §3.3 立的待办形状（27 个端点曾在总览之外零说明）"
        )

    def test_no_documented_path_has_disappeared(self):
        stale = sorted(documented_path_set() - live_paths())
        assert not stale, (
            f"API.md 里这些路径 OpenAPI 已经没有：{stale}"
        )

    def test_the_two_sets_are_exactly_equal(self):
        assert documented_path_set() == live_paths()


class TestDetailedLayerIsRealAndScoped:
    """详细层防空转 + 范围订正

    全量端点都要详细说明是**过度主张**：`dataset` / `system` 两组之外的操作，历史上
    就只在总览表里有一行（三列）。本轮补的是 §3.3 点名的两组，所以详细层判据只覆盖它们。
    （旧文在这里写了「其他 46 个端点」——那是个手抄的减法，L216 加两条端点后没人重算就得
    改口径：与其每次加端点都来订一个数字，不如只说范围。真判据在
    `test_overview_layer_covers_more_than_detailed` 那条「总览层严格大于详细层」里。）
    """

    def test_detailed_layer_is_not_empty(self):
        """防空转：详细层一条都没解析到 ⇒ 列数判据坏了"""
        assert detailed_path_set(), "四列说明表一行都没解析到"

    def test_detailed_layer_covers_exactly_the_two_groups(self):
        want = paths_of_tag("dataset") | paths_of_tag("system")
        got = detailed_path_set()
        missing = sorted(want - got)
        extra = sorted(got - want)
        assert not missing and not extra, (
            f"详细层覆盖范围漂移：少 {missing}、多 {extra}"
        )

    def test_overview_layer_covers_more_than_detailed(self):
        """总览层必须比详细层大——否则上面那条「恰好两组」就是恒真的"""
        assert len(documented_path_set()) > len(detailed_path_set())


class TestPreviouslyUndocumentedGroupsHaveNotes:
    """§3.3 点名的两组必须逐条有说明"""

    @pytest.mark.parametrize("tag", ["dataset", "system"])
    def test_every_endpoint_of_the_group_has_a_row(self, tag):
        want = paths_of_tag(tag)
        assert want, f"{tag} 组一个端点都没解析到 ⇒ 判据坏了"
        text = API_DOC.read_text(encoding="utf-8")
        missing = [p for p in sorted(want)
                   if f"`{p}`" not in text]
        assert not missing, f"{tag} 组这些端点没有说明行：{missing}"

    def test_the_groups_scale_is_pinned(self):
        """钉住规模：dataset **18 路径 / 18 操作**，system **13 路径 / 15 操作**

        `/api/system/backups` 同时挂 GET(列出)/POST(创建)，
        `/api/system/dependency/datasets` 同时挂两个 GET。所以「端点」按方法算比按路径算多
        ——两边都要钉，否则下次加一条路由时分不清是「加了路径」还是「给老路径加了方法」。

        名字里的 27 是**立项时**的读数（dataset 14 + system 13）。L216 给 dataset 组
        添了 `semantic-search` / `vector-status` 两条只读端点，于是本格从
        `test_the_groups_total_27_operations` 改名成 `..._scale_is_pinned`：数字是
        被钉的**当前**规模，不是历史叙述，名字继续写 27 就是句谎话。L217 又给 dataset 组
        添了 `language-profile` / `translate` 两条，于是 16 → 18。**L243 给 system 组添了
        备份完整性的两条**（`…/verify`、`…/cleanup`，各占一条新路径），于是 11 路径 / 13 操作
        → 13 路径 / 15 操作。
        """
        assert len(paths_of_tag("dataset")) == 18           # 18 路径 = 18 操作（L217 +2）
        assert len(paths_of_tag("system")) == 13            # 13 路径（L243 +2）
        assert len(operations_of_tag("system")) == 15       # 15 操作（L243 +2）


class TestTheRulerSeesSomething:
    def test_census_is_not_empty(self):
        live = live_paths()
        assert len(live) >= 60, f"只解析到 {len(live)} 个路径"
        assert len(documented_path_set()) >= 60

    def test_the_column_split_follows_gfm_escaping(self):
        """L278 的牙齿，双向：同一格写裸竖线就算多出第 5 列，写成转义形就还是四列

        前一半不是洁癖而是本轮真实发生过的形状：`docs/augmentor/API.md` 的搜索行正文里
        写了一枚交替组的裸竖线 ⇒ 列数读成 6 ⇒ 下面那两个按列数过滤的解析器把**整行**丢掉，
        `/api/dataset/search` 从详细层消失（当场红的是本文件与 l223 的集合对账维）。
        """
        bare = "| `POST` | `/api/x` | 说明 | 交替 `(a|aa)+b` 这一族 |"
        escaped = "| `POST` | `/api/x` | 说明 | 交替 `(a\\|aa)+b` 这一族 |"
        assert column_count(bare) == 6, "行内码里的裸竖线没算进列数 ⇒ 仍是旧的裸切口径"
        assert column_count(escaped) == 5, "转义竖线被算成列分隔符 ⇒ 合法的转义写法被误伤"
        assert len(split_cells(bare)) == 5
        assert split_cells(escaped) == [
            "`POST`", "`/api/x`", "说明", "交替 `(a|aa)+b` 这一族"], (
            "切列结果应当是四格，且还原出裸竖线")
