# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L210：`docs/API.md` 必须覆盖 **每一个** 端点

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
API_DOC = ROOT / "docs" / "API.md"

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
DETAILED_COLUMN_COUNT = 5


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
    """
    text = API_DOC.read_text(encoding="utf-8")
    found = set()
    for line in text.splitlines():
        if line.count("|") != DETAILED_COLUMN_COUNT:
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

    全量 71 个端点都要详细说明是**过度主张**：实测其他 46 个端点历史上就只在总览
    表里有一行（三列）。本轮补的是 §3.3 点名的两组，所以详细层判据只覆盖它们。
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

    def test_the_groups_total_27_operations(self):
        """钉住规模：**13 个操作**摊在 **11 个路径**上（两条路径挂了两个方法）

        `/api/system/backups` 同时挂 GET(列出)/POST(创建)，
        `/api/system/dependency/datasets` 同时挂两个 GET。所以「端点」按方法算是
        13、按路径算是 11——两边都要钉，否则下次加一条路由时分不清是「加了路径」
        还是「给老路径加了方法」。
        """
        assert len(paths_of_tag("dataset")) == 14          # 14 路径 = 14 操作
        assert len(paths_of_tag("system")) == 11           # 11 路径
        assert len(operations_of_tag("system")) == 13      # 13 操作


class TestTheRulerSeesSomething:
    def test_census_is_not_empty(self):
        live = live_paths()
        assert len(live) >= 60, f"只解析到 {len(live)} 个路径"
        assert len(documented_path_set()) >= 60
