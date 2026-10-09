# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L200（A266）：契约面数字从代码**实推**，文档必须与它对上

**为什么有这一支**：L197 一次人肉对账就抓出六处历史遗留错误——端点总数写成
68 / 70（真值 71）、`quality` tag 标题写 8（表格里自己列了 9 行）、导出格式示例
只列 5 种（真值 13）、router 写 11（真值 13）、前端页面写 9（真值 11）、事件循环
守门家族写 12 条（真值 13）。这些数**没有一个会随产品变坏而报警**：端点加了一个、
文档不跟上，全量测试照样绿。本守卫把「文档里的契约数字」变成从
`app.openapi()` / `ExportFormat` / 前端页面目录**实推**后与文档断言的相等关系。

口径（与 L197 的修法一致）：
- **会漂移的清单不手抄**：导出格式逐个列全没有意义（实现会长），所以只断言
  **个数**与「文档自己声明了以枚举为准」这句话；
- 端点总数、tag 计数、router 数、页面数、守门家族条数都是**结构性事实**，
  实推值与文档断言值必须相等；
- 每个断言都配「文档里那一处确实存在」的前置判据，否则正则改坏了会退化成
  恒真（`re.search` 返回 None 时 `int(None)` 会炸，但那种炸要有可读信息）。
"""

import re
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]

README = ROOT / "README.md"
API_DOC = ROOT / "docs" / "API.md"
ARCH_DOC = ROOT / "docs" / "ARCHITECTURE.md"


def _text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def _one(pattern: str, text: str, where: str) -> re.Match:
    """在文档里找**恰好一处**的模式；找不到或找到多处都要炸，不许静默跳过"""
    hits = list(re.finditer(pattern, text))
    assert len(hits) == 1, (
        f"{where}: 模式 {pattern!r} 命中 {len(hits)} 处（应为 1）"
        "——文档写法变了就改判据，别让它退化成恒真"
    )
    return hits[0]


# ---------------------------------------------------------------- 实推面

def live_openapi():
    from api.main import app
    return app.openapi()


def endpoint_total() -> int:
    spec = live_openapi()
    return sum(len(methods) for methods in spec["paths"].values())


def tag_counts():
    from collections import Counter
    counter = Counter()
    for methods in live_openapi()["paths"].values():
        for op in methods.values():
            for tag in op.get("tags", ["<none>"]):
                counter[tag] += 1
    return counter


def export_formats():
    from augmentor.export import ExportFormat
    return [member.value for member in ExportFormat]


def router_count() -> int:
    text = _text(ROOT / "api" / "main.py")
    return len(re.findall(r"include_router\(", text))


def page_count() -> int:
    pages = [p for p in (ROOT / "web" / "src" / "pages").glob("*.tsx")
             if not p.name.endswith(".test.tsx")]
    return len(pages)


def event_loop_guard_count() -> int:
    """`BLOCKING_SITES` 清单的条数（事件循环守门家族）"""
    text = _text(ROOT / "tests" / "integration" / "test_api_event_loop_blocking.py")
    m = re.search(r"BLOCKING_SITES = \[(.*?)\n\]", text, re.S)
    assert m, "test_api_event_loop_blocking.py 里找不到 BLOCKING_SITES"
    return len(re.findall(r'"/api/[^"]+"', m.group(1)))


# ---------------------------------------------------------------- 文档断言

class TestEndpointTotal:
    """端点总数：三处文档写的数必须等于 OpenAPI 实推的 operation 数"""

    def test_readme_states_the_live_total(self):
        m = _one(r"(\d+) 个 REST 端点", _text(README), "README.md")
        assert int(m.group(1)) == endpoint_total(), (
            f"README 写 {m.group(1)} 个端点，实推 {endpoint_total()} 个"
        )

    def test_readme_api_section_states_the_live_total(self):
        m = _one(r"\*\*(\d+) 个端点\*\*", _text(README), "README.md")
        assert int(m.group(1)) == endpoint_total()

    def test_api_doc_contract_section_states_the_live_total(self):
        m = _one(r"（共 (\d+) 个）", _text(API_DOC), "docs/API.md")
        assert int(m.group(1)) == endpoint_total()

    def test_api_doc_overview_states_the_live_total(self):
        m = _one(r"共 \*\*(\d+)\*\* 个端点", _text(API_DOC), "docs/API.md")
        assert int(m.group(1)) == endpoint_total()


class TestTagCounts:
    """每个 tag 的端点条数必须与 OpenAPI 实推一致"""

    def test_every_tag_heading_matches(self):
        text = _text(API_DOC)
        live = tag_counts()
        headings = re.findall(r"^### (\w+)（(\d+)）\s*$", text, re.M)
        assert headings, "docs/API.md 里一个 `### tag（N）` 标题都没找到"
        seen = {}
        for tag, count in headings:
            assert tag not in seen, f"docs/API.md 里 `### {tag}（N）` 出现了两次"
            seen[tag] = int(count)
        assert seen == dict(live), (
            "文档 tag 计数与实推不一致：\n"
            + "\n".join(
                f"  {t}: 文档 {seen.get(t, '<缺>')} / 实推 {live.get(t, 0)}"
                for t in sorted(set(seen) | set(live))
                if seen.get(t) != live.get(t)
            )
        )


class TestExportFormats:
    """导出格式清单：文档示例的个数必须等于 `ExportFormat` 成员数"""

    def test_doc_example_lists_every_format(self):
        text = _text(API_DOC)
        m = _one(r'\{"formats": \[([^\]]*)\]\}', text, "docs/API.md")
        listed = re.findall(r'"([^"]+)"', m.group(1))
        live = export_formats()
        assert listed == live, (
            f"文档示例列了 {len(listed)} 种 {listed}，实推 {len(live)} 种 {live}"
        )

    def test_readme_declares_the_count_and_defers_to_the_enum(self):
        text = _text(README)
        m = _one(r"\*\*多格式导出\*\*: (\d+) 种", text, "README.md")
        assert int(m.group(1)) == len(export_formats())
        assert "ExportFormat" in text, (
            "README 的导出格式行必须声明「以 ExportFormat 为准」——否则这个数"
            "下一次实现增长时又会变成谎话"
        )


class TestStructureCounts:
    """router 数 / 前端页面数 / 事件循环守门条数"""

    def test_router_count(self):
        m = _one(r"按领域拆分的 (\d+) 个 router", _text(ARCH_DOC), "docs/ARCHITECTURE.md")
        assert int(m.group(1)) == router_count(), (
            f"文档写 {m.group(1)} 个 router，实推 {router_count()} 个"
        )

    def test_page_count(self):
        text = _text(ARCH_DOC)
        live = page_count()
        for pattern in (r"路由与菜单（(\d+) 个页面）", r"页面（(\d+) 个）"):
            m = _one(pattern, text, "docs/ARCHITECTURE.md")
            assert int(m.group(1)) == live, (
                f"ARCHITECTURE.md {pattern!r} 写 {m.group(1)}，实推 {live} 个页面"
            )

    def test_readme_page_count(self):
        m = _one(r"React 前端 (\d+) 个功能页", _text(README), "README.md")
        assert int(m.group(1)) == page_count()

    def test_event_loop_guard_family(self):
        """该句是历史叙述，只描述**第一个**守门家族（quality + 四个单点）

        全量 `BLOCKING_SITES` 后来扩到含 dataset / system 两族，所以不能拿清单
        总条数去对这句话。改判三件自洽的事：① 句内枚举的家族数加起来等于它自称
        的总数；② `/api/quality/*` 那条数等于实推的 quality tag 端点数；
        ③ 句里点名的每个路径在 OpenAPI 里真实存在（防文档编一个不存在的端点）。
        """
        text = _text(ARCH_DOC)
        # 该句跨两行（`+` 收尾换行继续枚举），故显式捕获「本行余下 + 下一行」
        m = _one(r"> (\d+) 条端点（`/api/quality/\*` (\d+) 条 \+ ((?:[^\n]*\n)[^\n]*)",
                 text, "docs/ARCHITECTURE.md")
        stated_total, stated_quality, rest = (
            int(m.group(1)), int(m.group(2)), m.group(3))
        named = re.findall(r"`(/api/[^`]+)`", rest)
        assert len(named) == 4, f"该句应点名 4 个单点路径，实得 {named}"
        assert stated_quality + len(named) == stated_total, (
            f"句内不自治：quality {stated_quality} + 单点 {len(named)} "
            f"≠ 自称的 {stated_total}"
        )
        assert stated_quality == tag_counts()["quality"], (
            f"该句写 quality 占 {stated_quality} 条，实推 {tag_counts()['quality']} 个端点"
        )
        live_paths = set(live_openapi()["paths"])
        for path in named:
            assert path in live_paths, f"该句点名的 {path} 不在 OpenAPI 里"

    def test_full_blocking_sites_list_is_all_live_paths(self):
        """整份 `BLOCKING_SITES` 里每个路径都必须在 OpenAPI 里真实存在"""
        live_paths = set(live_openapi()["paths"])
        text = _text(ROOT / "tests" / "integration"
                     / "test_api_event_loop_blocking.py")
        m = re.search(r"BLOCKING_SITES = \[(.*?)\n\]", text, re.S)
        assert m, "找不到 BLOCKING_SITES 清单"
        listed = re.findall(r'"(/api/[^"]+)"', m.group(1))
        assert listed, "BLOCKING_SITES 一条都没解析到 ⇒ 正则坏了"
        missing = [p for p in listed if p not in live_paths]
        assert not missing, f"清单里这些路径不在 OpenAPI: {missing}"


class TestTheRulerItselfIsAlive:
    """防空转：实推面必须真的取到东西，不能全靠 0 == 0 蒙过去"""

    def test_probe_sees_a_nonempty_openapi(self):
        assert len(live_openapi()["paths"]) > 50
        assert endpoint_total() > 50

    def test_probe_sees_formats(self):
        assert len(export_formats()) >= 10

    def test_probe_sees_pages_and_routers(self):
        assert page_count() >= 10
        assert router_count() >= 10
