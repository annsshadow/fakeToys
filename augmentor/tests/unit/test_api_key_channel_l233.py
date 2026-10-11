# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L233（A302 / A312）：写操作鉴权的前端通道 —— 规模、形状与口径

**为什么有这一支**：L231 量到「服务端一旦设上 `AUGMENTOR_API_KEY`，前端没有任何送
`X-API-Key` 的通道」，但当时只数了备份那一族的 3 只写操作。本轮用仓外探针把整张契约面
量了一遍，规模比那时候以为的大一个量级：

| 事实 | L233 探针读数 |
| --- | --- |
| 契约面操作总数 | **75**（与 l224 同一个家，这里不另钉一份） |
| 挂 `verify_api_key` 的操作 | **24** |
| 其中前端**已接线**的形状 | **24** ⇒ 开了鉴权就有 24 个页面动作点不动（L236 接走 translate 后与门禁总数相等） |
| 挂门禁但还没接线的 | **0**（L236 接走 translate；此前 L234 把 migrate / stream 接进页面后从 3 降到 1） |

真链路上量到的档位（`TestClient`，白名单根指向临时目录，跑完不留产物）：无头 ⇒ **401**，
detail 逐字「缺少或无效的 X-API-Key」；错值 ⇒ 同样 **401**；对值 ⇒ 放行（拿「删一个不存在
的备份」这一支验放行，它只会 404「备份不存在」，不落盘）。**头名大小写无关**：送
`X-Api-Key` 同样放行 —— HTTP 头名不区分大小写，所以前端写哪种都对；代码里仍与后端
`Header(alias=...)` 逐字一致，免得读代码时以为是两个不同的头。环境变量没设时那道门禁
整个不存在（`verify_api_key` 直接 return），这是「零配置可用」的既有行为，本轮不动。

三面判据，各对应一种真实失效形状：

1. **规模**（`TestGateScale`）：24 / 24 与那 24 条的逐条名单（L233 定标时是 24 / 21，
   L234 把 migrate / stream 接进页面后挪到 23，L236 接走 translate 后到 24 ⇒ 尾巴清空）。
   多挂一条门禁、或前端多接
   一条写操作，都会红 —— 那一格正是「页面上有多少动作会被鉴权掐死」的真值。
2. **通道形状**（`TestTheChannelShape`）：头名的带引号字面量在前端**只有一处**
   （`api.ts` 的 `API_KEY_HEADER`），拦截器用它、存取用同一个 localStorage 键、空值走
   `removeItem` 而不是存空串，`localStorage` 也只在那一个文件里出现。第二处字面量就等于
   第二条各自为政的通道（两处存法不一致时，一处清不掉另一处）。
3. **口径**（`TestTheHonestProse`）：页面与文档不许再说「前端没有送这个头的通道」——
   那句在 L233 之后是假话，而假话比没有话更糟。

行为面（存了 key 真的挂上头、没存真的不挂、401 那句指路、卡片上填得进清得掉）钉在
`web/src/services/api.test.ts` 与 `web/src/pages/Settings.test.tsx`：那边能真跑拦截器。
这里跑不了 JS，所以只钉**源码形状**，两面各钉一半、不重复。

跑法：`python -m pytest tests/unit/test_api_key_channel_l233.py -q --no-cov`
"""

import pathlib
import re

import pytest
from tests.unit.test_api_endpoint_consumption_l224 import consumed_operations, operations
from tests.unit.test_wired_dropdown_choices_l226 import needs_web_src, strip_comments

from api.deps import API_KEY_ENV, verify_api_key

ROOT = pathlib.Path(__file__).resolve().parents[2]
SRC = ROOT / "web" / "src"
API_TS = SRC / "services" / "api.ts"
SETTINGS_TSX = SRC / "pages" / "Settings.tsx"
SYSTEM_TSX = SRC / "pages" / "System.tsx"

#: 后端那个头的别名，逐字来自 `api/deps.py` 的 `Header(None, alias="X-API-Key")`
TOKEN = "X-API-Key"
#: 401 的 detail 原文（探针逐字量到，两种成因——没送与送错——共用这一句）
DETAIL_401 = "缺少或无效的 X-API-Key"
#: 「通道」只认带引号的字面量：页面正文里那句 401 文案是缺口的回声，不是通道本身
QUOTED_TOKEN = re.compile(rf"['\"`]{re.escape(TOKEN)}")
#: 存 key 的那一格在页面上的落点（401 的指路与两张卡的正文都用这个名字）
CARD = "配置管理 → 写操作鉴权"

#: 探针量到的两个数（`GATED_CONSUMED`：L233 定标 21 → L234 挪到 23（migrate / stream 进页面）
#: → L236 到 24（translate 进页面）⇒ 与 `GATED_TOTAL` 相等，「挂门禁但没接线」那一档清空）
GATED_TOTAL = 24
GATED_CONSUMED = 24

#: 那 24 条：前端已接线、且服务端开了鉴权就会 401 的形状（逐条来自探针，不是推的）
CONSUMED_AND_GATED = frozenset({
    ("DELETE", "/api/data/delete/{filename}"),
    ("DELETE", "/api/system/backups/{backup_id}"),
    ("DELETE", "/api/versions/{version_id}"),
    ("POST", "/api/augment/start"),
    ("POST", "/api/config"),
    ("POST", "/api/data/export"),
    ("POST", "/api/data/upload"),
    ("POST", "/api/dataset/aggregate"),
    ("POST", "/api/dataset/convert"),
    ("POST", "/api/dataset/merge"),
    ("POST", "/api/dataset/rag"),
    ("POST", "/api/dataset/sample"),
    ("POST", "/api/dataset/split"),
    ("POST", "/api/dataset/translate"),
    ("POST", "/api/export/batch"),
    ("POST", "/api/quality/profiling"),
    ("POST", "/api/system/backups"),
    ("POST", "/api/system/backups/{backup_id}/restore"),
    ("POST", "/api/system/dependency/datasets"),
    ("POST", "/api/system/migrate"),
    ("POST", "/api/system/stream"),
    ("POST", "/api/versions/create"),
    ("POST", "/api/versions/{version_id}/rollback"),
    ("PUT", "/api/data/update/{filename}"),
})

#: 挂门禁但前端还没接线的尾巴：**空**（L236 接走 translate 后 24 = 24 + 0）。这一格清空之后
#: 判据仍会响——它比的是集合相等，不是「非空」；往契约面新挂一条门禁而不接线，这里当场多出一员
GATED_NOT_YET_CONSUMED = frozenset()


def walk_api_routes(routes):
    """把 included router 展平，逐条交出 `(method, path, dependant)`

    **不递归就是瞎的**：这版 FastAPI 的 `include_router` 不在 `app.routes` 上摊平，而是留
    一个 `_IncludedRouter` 占位，真路由挂在它的 `original_router.routes` 上。探针首跑实测：
    直接数 `app.routes` ⇒ 75 条只读到 **1** 条、门禁 **0** 条，于是「21 个动作会 401」
    被读成「0 个」，后面每一条交集判据都空转成全绿。L231 也踩过这一格（那边改用
    `app.openapi()`，但 openapi 里没有依赖信息，量不到门禁）。
    """
    out = []
    for route in routes:
        inner = getattr(route, "original_router", None) or getattr(route, "router", None)
        if inner is not None and getattr(inner, "routes", None):
            out.extend(walk_api_routes(inner.routes))
            continue
        dependant = getattr(route, "dependant", None)
        methods = getattr(route, "methods", None)
        path = getattr(route, "path", "")
        if dependant is None or not methods or not path.startswith("/api"):
            continue
        for method in methods:
            if method != "HEAD":
                out.append((method.upper(), path, dependant))
    return out


def gated_operations():
    """契约面上挂 `verify_api_key` 的操作集合（递归看依赖，router 级 `dependencies=` 也算）"""
    from api.main import app

    def has_key_check(dependant) -> bool:
        return any(
            dep.call is verify_api_key or has_key_check(dep)
            for dep in dependant.dependencies
        )

    return {
        (method, path)
        for method, path, dependant in walk_api_routes(app.routes)
        if has_key_check(dependant)
    }


def _code(path: pathlib.Path) -> str:
    assert path.is_file(), f"{path.name} 不在场 ⇒ 本文件的判据全部失效"
    return strip_comments(path.read_text(encoding="utf-8"))


def _corpus():
    """前端非测试源码（口径承 L231：`.test.` 那一份是替身表，不是接线）"""
    files = [p for p in SRC.rglob("*")
             if p.is_file() and p.suffix in {".ts", ".tsx"} and ".test." not in p.name]
    assert files, "前端语料为空 ⇒ 判据空转"
    return files


@pytest.fixture
def http(tmp_path, monkeypatch):
    """白名单根指向临时目录：验放行那一支会真跑到 handler，产物不许落进仓库"""
    root = tmp_path / "ds"
    root.mkdir()
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient

    from api.main import app

    return TestClient(app), root, monkeypatch


#: 一个不存在的备份 id：拿它验「鉴权放行」，因为 handler 只会 404「备份不存在」，不写盘
GHOST_BACKUP = "/api/system/backups/20990101_000000_probe"


class TestGateScale:
    """第 1 面：门禁的规模。数字全部现量，24 / 24 两个常数是探针读数的钉子"""

    def test_the_walker_is_not_blind(self):
        """反向元断言：走法写错时这里先红，而不是让后面所有交集判据空转成绿"""
        from api.main import app

        walked = {(m, p) for m, p, _ in walk_api_routes(app.routes)}
        assert walked == operations(), (
            f"展平后读到 {len(walked)} 条，契约面是 {len(operations())} 条 ⇒ 走法漂了")

    def test_24_of_the_75_operations_are_key_gated(self):
        assert len(gated_operations()) == GATED_TOTAL, sorted(gated_operations())

    def test_24_gated_operations_are_already_wired_into_the_ui(self):
        consumed = consumed_operations()
        assert consumed & gated_operations() == CONSUMED_AND_GATED
        assert len(CONSUMED_AND_GATED) == GATED_CONSUMED

    def test_the_unwired_tail_is_empty(self):
        """24 = 24 + 0：L236 接走 translate 之后，挂门禁却没接线的形状一个都不剩

        这一格**不是**因为清空就变成恒真——它比的是集合相等：往契约面新挂一条门禁而前端
        不接线，右边空集合立刻与左边不等；反过来把已接线的一条从 `CONSUMED_AND_GATED` 里
        删掉，上一支会先红（交集那一格），这一支则照旧绿，所以两支配对才把「多一条 / 少一条」
        两个方向都堵住。
        """
        gated = gated_operations()
        assert gated - consumed_operations() == GATED_NOT_YET_CONSUMED
        assert len(gated) == GATED_CONSUMED + len(GATED_NOT_YET_CONSUMED)

    def test_no_read_operation_is_gated(self):
        """门禁只管写：24 条里没有一条 GET（开了鉴权后页面仍然读得到东西）"""
        assert not [op for op in gated_operations() if op[0] == "GET"]


class TestTheChannelShape:
    """第 2 面：前端那一条通道的源码形状（行为面在 `api.test.ts`）"""

    @needs_web_src
    def test_the_header_literal_lives_in_exactly_one_place(self):
        hits = [f"{p.relative_to(SRC).as_posix()}"
                for p in _corpus() if QUOTED_TOKEN.search(strip_comments(
                    p.read_text(encoding="utf-8")))]
        assert hits == ["services/api.ts"], (
            f"头名的带引号字面量现量只有 api.ts 一处（多一处＝多一条各自为政的通道），实得 {hits}")

    @needs_web_src
    def test_the_interceptor_sets_that_constant_from_local_storage(self):
        code = _code(API_TS)
        assert f"export const API_KEY_HEADER = '{TOKEN}'" in code
        assert "api.interceptors.request.use(" in code
        registered = code.split("api.interceptors.request.use(", 1)[1].split("})", 1)[0]
        assert "getApiKey()" in registered, f"拦截器不再读那份 key：{registered}"
        assert "config.headers.set(API_KEY_HEADER, key)" in registered, registered
        # 「没存就不挂头」是零配置可用的那一半：这一格被删掉，服务端没开鉴权时也会多送一个空头
        assert re.search(r"if \(key\) \{", registered), registered

    @needs_web_src
    def test_storage_has_one_home_and_an_empty_key_is_removed_not_stored(self):
        code = _code(API_TS)
        assert "export const API_KEY_STORAGE_KEY = 'augmentor.api_key'" in code
        assert "localStorage.getItem(API_KEY_STORAGE_KEY)" in code
        assert "localStorage.setItem(API_KEY_STORAGE_KEY, trimmed)" in code
        assert "localStorage.removeItem(API_KEY_STORAGE_KEY)" in code
        body = code.split("export const setApiKey", 1)[1].split("\n}", 1)[0]
        assert "if (trimmed)" in body and "removeItem" in body, (
            f"空输入必须走 removeItem，否则「留空即不送头」变成「送一个空头」：{body}")
        # 「第二处」只数**存取动作**，不数提到这个词：页面上那句「存哪儿：localStorage，明文」
        # 是诚实口径要求的正文（首跑就是被它判红的 —— 尺子把回声当成了信号，与 L231 那条
        # 「带引号字面量才算通道」同一个教训）。
        others = [p.relative_to(SRC).as_posix() for p in _corpus()
                  if p != API_TS and re.search(
                      r"localStorage\.(?:get|set|remove)Item",
                      strip_comments(p.read_text(encoding="utf-8")))]
        assert not others, f"存取 key 的地方多了第二处：{others}"

    @needs_web_src
    def test_the_settings_page_is_the_one_input(self):
        code = _code(SETTINGS_TSX)
        assert "getApiKey" in code and "setApiKey" in code
        assert "写操作鉴权" in code
        assert 'aria-label="写操作鉴权 API Key"' in code
        # 本层没有能力证明这份 key 是对的（服务端只在写操作请求上校验）⇒ 这张卡不许说「校验通过」。
        # **扫描面收到 key 卡自身那一段**（L234 改的）：同一个文件里另有「配置文件校验」卡，
        # 那张卡说「这份配置合法」是它真查过的判决，与 key 无关。禁令面是 key 卡，写成整个文件
        # 的 not in 就会被无关成员判假红 —— 尺子要量的是「key 卡有没有吹牛」，不是「这个文件
        # 有没有出现过四个字」。
        card = code.split('<Card title="写操作鉴权"', 1)[1].split("\n      </Card>", 1)[0]
        assert "校验通过" not in card

    @needs_web_src
    def test_the_401_hint_points_at_that_card(self):
        code = _code(API_TS)
        assert "response?.status !== 401" in code, "401 不再单独指路 ⇒ 24 个动作又只剩一句判决原文"
        assert CARD in code
        # 两种成因要分开说：存着却 401 是「那份不对」，没存是「还没填」
        assert "浏览器里存着的那一份与服务端不一致" in code
        assert "这台浏览器还没有存 key" in code


class TestTheHonestProse:
    """第 3 面：口径。L233 之前那句「前端没有送这个头的通道」现在必须是假的"""

    @needs_web_src
    def test_no_page_still_claims_there_is_no_channel(self):
        stale = [f"{p.relative_to(SRC).as_posix()}:{line.strip()[:60]}"
                 for p in _corpus()
                 for line in p.read_text(encoding="utf-8").splitlines()
                 if "没有送这个头的通道" in line or "没有任何送" in line]
        assert not stale, f"页面上还留着 L233 之前的假话：{stale}"

    @needs_web_src
    def test_every_gated_card_points_to_where_the_key_is_set(self):
        """L234 起是三次（备份卡 / 血缘卡 / 迁移与流式卡）：页面上每一张含挂门禁写操作的卡，
        都要指着存 key 的那一格 —— 少指一次，用户在那张卡上撞 401 就没有去处"""
        page = SYSTEM_TSX.read_text(encoding="utf-8")
        assert page.count(CARD) == 3, (
            f"备份卡、血缘卡与迁移流式卡各指一次那张卡，现量 {page.count(CARD)} 次")
        assert TOKEN in page, "401 的判决原文里带着这个头名，页面上要如实引用"

    @needs_web_src
    def test_the_impact_numbers_on_the_key_card_are_the_measured_ones(self):
        """卡上那句影响面的两个数必须与本文件在路由上量到的那两个常数同源

        L234 把 migrate / stream 接进页面之后，这一句在 `Settings.tsx` 上仍写着 21 —— 页面上
        的数字此前没人钉，接线一漂它就成假话；而且漂的方向是「少说了两个点不动的动作」，
        用户只会觉得提示不准，不会怀疑门禁本身。这里把页面那半句与 `GATED_TOTAL` /
        `GATED_CONSUMED` 绑死：接线数再变，两处必须一起动。
        """
        page = SETTINGS_TSX.read_text(encoding="utf-8")
        assert f"有 {GATED_TOTAL} 条挂着这道门禁" in page, (
            f"门禁总数现量是 {GATED_TOTAL}，卡上那句影响面要跟着改")
        assert f"其中 {GATED_CONSUMED} 条是页面上已" in page, (
            f"已接线的挂门禁动作现量是 {GATED_CONSUMED} 条（L234 从 21 挪上来），卡上那句要跟着改")

    def test_the_docs_no_longer_say_the_frontend_has_no_channel(self):
        stale = []
        for doc in (ROOT / "docs" / "API.md", ROOT / "README.md"):
            text = doc.read_text(encoding="utf-8")
            stale += [f"{doc.name}:{line.strip()[:60]}" for line in text.splitlines()
                      if "没有送这个头的通道" in line or "前端没有送" in line]
        assert not stale, f"文档里还留着 L233 之前的假话：{stale}"


@needs_web_src
class TestTheRealLink:
    """真链路：那一档 401 与那一档放行，逐个量（不推）"""

    def test_no_header_is_401_with_that_exact_detail(self, http):
        client, _, monkeypatch = http
        monkeypatch.setenv(API_KEY_ENV, "s3cret")
        r = client.delete(GHOST_BACKUP)
        assert r.status_code == 401
        assert r.json()["detail"] == DETAIL_401

    def test_a_wrong_value_is_401_whatever_the_header_case(self, http):
        client, _, monkeypatch = http
        monkeypatch.setenv(API_KEY_ENV, "s3cret")
        for headers in ({"x-api-key": "wrong"}, {"X-API-KEY": "wrong"}):
            r = client.delete(GHOST_BACKUP, headers=headers)
            assert r.status_code == 401, headers
            assert r.json()["detail"] == DETAIL_401

    def test_the_right_value_passes_and_the_header_name_is_case_insensitive(self, http):
        """放行用「删一个不存在的备份」验：它只会 404，不落盘 ⇒ 探针不在仓里留产物"""
        client, _, monkeypatch = http
        monkeypatch.setenv(API_KEY_ENV, "s3cret")
        for name in ("X-API-Key", "X-Api-Key", "x-api-key"):
            r = client.delete(GHOST_BACKUP, headers={name: "s3cret"})
            assert r.status_code == 404, f"{name} 没被认下：{r.status_code} {r.text[:80]}"
            assert r.json()["detail"] == "备份不存在"

    def test_without_the_env_var_the_gate_does_not_exist(self, http):
        """没设环境变量时不校验 —— 「零配置可用」的既有行为，本轮不动它"""
        client, _, monkeypatch = http
        monkeypatch.setenv(API_KEY_ENV, "")
        assert client.delete(GHOST_BACKUP).status_code == 404

    def test_reads_stay_open_while_the_gate_is_up(self, http):
        client, _, monkeypatch = http
        monkeypatch.setenv(API_KEY_ENV, "s3cret")
        assert client.get("/api/system/backups").status_code == 200
