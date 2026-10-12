# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""备份一族六员的「契约声明的错误档 == 真请求打得出的错误档」（A330 / L252 收口轮）

**这一轮收的是哪一格**：A330 记的是「判决全在运行时，OpenAPI 里看不见」——六员当年一律只声明
`{200, 422}`，也就是一个只看契约的客户端**无从知道**恢复会被 409 拦、四个写操作会被 401 挡、
目录传错会 403、备份不存在会 404。L243 当年把这件事量成数字立成待拍，本轮走它写的修法①：
六员各补 `responses=`，**只补实测到过的档**。

**判据为什么是集合相等而不是「补齐了就行」**（本轮唯一要紧的取舍）：声明少了 → 客户端踩坑；
声明多了 → 客户端去处理一个永远不会来的分支，那是同一种病的反面。所以每一员的判据都写成
`声明的错误档集合 == 实测的错误档集合`，两个方向同一把尺子。「实测」这一侧不是手抄的清单，而是
**真的把请求打出去**（`ARMS` 那 29 个请求），每一臂必须命中它挂在的那一档才算数：路由行为漂了
（比如有人把 409 又收回 400），臂先红，不会等到对账那一步才发现。现量：**六员 20 格错误档、
29 个真请求**，改前一格也没有。

**探针量到的矩阵**（本轮补写的依据，逐档都由一个真请求打出来）：

| 端点 | 实测到的错误档 | 说明 |
| --- | --- | --- |
| `GET /api/system/backups` | 400 / 403 | 只读面：无 key 回 200，**所以不声明 401** |
| `POST /api/system/backups` | 400 / 401 / 403 / 404 | 404 是「源在白名单内但不在场」，与 403 是两件事 |
| `POST …/{backup_id}/restore` | 400 / 401 / 403 / 404 / 409 | 409 全家族独此一档（A299 那道闸） |
| `DELETE …/{backup_id}` | 400 / 401 / 403 / 404 | 索引有条目而文件已丢 ⇒ **200**，不算 404 |
| `GET …/verify` | 400 / 403 | 只读面：幽灵 id 与坏备份都回 200 逐行读数，**不声明 401/404/409** |
| `POST …/cleanup` | 400 / 401 / 403 | `max_backups` 是整数但小于 1 ⇒ 400；不是整数 ⇒ 422 |

**`422` 单列一档**：它是 FastAPI 见「有参数」就自动挂上去的，本轮没动它，而它在六员上分两种——
带请求体的三员（create / restore / cleanup）打得出来，不带体的三员（list / verify / delete）
**构造不出**任何一个 422 请求（`backup_dir` 是字符串查询参数，任何值都合法）。这一格是本轮探针
顺手量到的**反方向**缺口（契约声明了不可达的档），本轮按事实钉成数字并另立 **A336** 待拍，
不在这里顺手改公开契约。同场量到的另一件事：同名查询参数重复给值时 FastAPI 取**最后一个**
（`?backup_dir=<根外>&backup_dir=x` ⇒ 200，反序 ⇒ 403），所以 403 那一族的臂一律只给一个值。

**顺带钉的两件事**（都来自同一批请求，白捡的牙齿）：
1. 错误档的请求**不在磁盘上留痕**：29 个请求跑完之后，备份目录、`index.json`、数据根三处逐字
   相同。这一格是 A299「409 一个字节都不写」的续量，把它从一只端点扩到整个家族。
2. 每一档的 `description` 要说清是哪颗旋钮或哪一种状态，响应体声明成 `{detail: string}`——
   `HTTPException` 出去的永远是那一句，声明成别的形状就是新的假话。旋钮清单不靠手抄：
   `TestTheKnobListIsDerived` 把它与三只请求模型的字段集加 `backup_dir` 做集合相等。

跑法：`python -m pytest tests/unit/test_backup_contract_error_codes_l252.py -q --no-cov`
"""

import json
import os
from types import SimpleNamespace

import pytest
from tests.unit.test_backup_integrity_l243 import _client, _corrupt

LIST = ("GET", "/api/system/backups")
CREATE = ("POST", "/api/system/backups")
RESTORE = ("POST", "/api/system/backups/{backup_id}/restore")
DELETE = ("DELETE", "/api/system/backups/{backup_id}")
VERIFY = ("GET", "/api/system/backups/verify")
CLEANUP = ("POST", "/api/system/backups/cleanup")

SIX = (LIST, CREATE, RESTORE, DELETE, VERIFY, CLEANUP)

#: 只读的两员：不挂 `verify_api_key` 那道闸，因此契约里也不许出现 401
READ_ONLY = (LIST, VERIFY)
#: 带请求体的三员：框架自动声明的 422 在它们身上打得出来
BODY_BEARING = (CREATE, RESTORE, CLEANUP)

#: `200` 与 `422` 不参与对账方程：前者是成功档，后者是框架自动挂的（见模块 docstring）
AUTO = {"200", "422"}

#: 六员全部入参名的封闭清单，由 `TestTheKnobListIsDerived` 与请求模型现量对账
KNOBS = {"backup_dir", "input_file", "output_file", "name", "max_backups"}


def outside_dir(ctx):
    """白名单外那个目录（每处 403 的落点，一律给绝对路径，见 docstring 里那条「重复给值」）"""
    return str(ctx.outside)


def _get(ctx, url):
    return ctx.http.get(url)


def _post(ctx, url, payload, *, with_key=True):
    return ctx.http.post(url, json=payload, headers=ctx.key if with_key else {})


def _delete(ctx, url, *, with_key=True):
    return ctx.http.delete(url, headers=ctx.key if with_key else {})


#: 每员每档一组**真请求**。外层键是状态码，内层是 `(人读标签, 打请求的函数)`；
#: 同一档多臂是因为「同一状态码有好几种触发条件」，每一臂都必须命中那一档。
ARMS = {
    LIST: {
        400: [("目录传空串", lambda c: _get(c, "/api/system/backups?backup_dir="))],
        403: [("目录在白名单外", lambda c: _get(c, f"/api/system/backups?backup_dir={outside_dir(c)}"))],
    },
    CREATE: {
        400: [
            ("源为空串", lambda c: _post(c, "/api/system/backups", {"input_file": "", "name": "x"})),
            ("源含 ..", lambda c: _post(c, "/api/system/backups",
                                        {"input_file": "../p.json", "name": "x"})),
            ("name 指向备份目录之外", lambda c: _post(
                c, "/api/system/backups", {"input_file": "p.json", "name": "../../escaped"})),
            ("目录传空串", lambda c: _post(
                c, "/api/system/backups?backup_dir=", {"input_file": "p.json", "name": "x"})),
        ],
        401: [("不带 key", lambda c: _post(c, "/api/system/backups",
                                           {"input_file": "p.json", "name": "x"}, with_key=False))],
        403: [
            ("源在白名单外", lambda c: _post(
                c, "/api/system/backups",
                {"input_file": f"{outside_dir(c)}{c.sep}secret.json", "name": "x"})),
            ("目录在白名单外", lambda c: _post(
                c, f"/api/system/backups?backup_dir={outside_dir(c)}",
                {"input_file": "p.json", "name": "x"})),
        ],
        404: [("源在白名单内但不在场", lambda c: _post(
            c, "/api/system/backups", {"input_file": "ghost.json", "name": "x"}))],
    },
    RESTORE: {
        400: [
            ("目标为空串", lambda c: _post(c, "/api/system/backups/a/restore", {"output_file": ""})),
            ("目标含 ..", lambda c: _post(c, "/api/system/backups/a/restore",
                                          {"output_file": "../escaped.json"})),
            ("目录传空串", lambda c: _post(c, "/api/system/backups/a/restore?backup_dir=",
                                           {"output_file": "r.json"})),
        ],
        401: [("不带 key", lambda c: _post(c, "/api/system/backups/a/restore",
                                           {"output_file": "r.json"}, with_key=False))],
        403: [
            ("目标在白名单外", lambda c: _post(
                c, "/api/system/backups/a/restore",
                {"output_file": f"{outside_dir(c)}{c.sep}out.json"})),
            ("目录在白名单外", lambda c: _post(
                c, f"/api/system/backups/a/restore?backup_dir={outside_dir(c)}",
                {"output_file": "r.json"})),
        ],
        404: [("幽灵 id", lambda c: _post(c, "/api/system/backups/ghost/restore",
                                          {"output_file": "r.json"}))],
        409: [("备份内容被改坏", lambda c: _post(c, "/api/system/backups/b/restore",
                                                 {"output_file": "r.json"}))],
    },
    DELETE: {
        400: [("目录传空串", lambda c: _delete(c, "/api/system/backups/a?backup_dir="))],
        401: [("不带 key", lambda c: _delete(c, "/api/system/backups/a", with_key=False))],
        403: [("目录在白名单外", lambda c: _delete(
            c, f"/api/system/backups/a?backup_dir={outside_dir(c)}"))],
        404: [("幽灵 id", lambda c: _delete(c, "/api/system/backups/ghost"))],
    },
    VERIFY: {
        400: [("目录传空串", lambda c: _get(c, "/api/system/backups/verify?backup_dir="))],
        403: [("目录在白名单外", lambda c: _get(
            c, f"/api/system/backups/verify?backup_dir={outside_dir(c)}"))],
    },
    CLEANUP: {
        400: [
            ("份数为零", lambda c: _post(c, "/api/system/backups/cleanup", {"max_backups": 0})),
            ("份数为负", lambda c: _post(c, "/api/system/backups/cleanup", {"max_backups": -1})),
            ("目录传空串", lambda c: _post(c, "/api/system/backups/cleanup?backup_dir=",
                                           {"max_backups": 10})),
        ],
        401: [("不带 key", lambda c: _post(c, "/api/system/backups/cleanup",
                                           {"max_backups": 10}, with_key=False))],
        403: [("目录在白名单外", lambda c: _post(
            c, f"/api/system/backups/cleanup?backup_dir={outside_dir(c)}", {"max_backups": 10}))],
    },
}

#: 现量：逐员的错误档集合。`TestEveryArmHitsItsOwnVerdict` 逐员点名，`_matrix` 用它做总账
EXPECTED = {
    LIST: {"400", "403"},
    CREATE: {"400", "401", "403", "404"},
    RESTORE: {"400", "401", "403", "404", "409"},
    DELETE: {"400", "401", "403", "404"},
    VERIFY: {"400", "403"},
    CLEANUP: {"400", "401", "403"},
}


@pytest.fixture
def ctx(tmp_path, monkeypatch):
    """白名单根 + 开着 key 的服务 + 备份 a（好）与 b（内容被改坏）+ 根外的 `outside/secret.json`"""
    http, root, bd = _client(tmp_path, monkeypatch, api_key="k")
    outside = tmp_path / "outside"
    outside.mkdir()
    (outside / "secret.json").write_text("[]", encoding="utf-8")
    for name in ("a", "b"):
        made = http.post("/api/system/backups", json={"input_file": "p.json", "name": name},
                         headers={"X-API-Key": "k"})
        assert made.status_code == 200, made.text
    _corrupt(bd, "b")
    return SimpleNamespace(http=http, root=root, bd=bd, outside=outside,
                           key={"X-API-Key": "k"}, sep=os.sep)


def openapi_ops():
    """契约面：`{(方法, 路径模板): 该员声明的全部响应码}`，只取备份一族六员"""
    from api.main import app

    return {
        (verb.upper(), path): spec["responses"]
        for path, ops in app.openapi()["paths"].items() if "backups" in path
        for verb, spec in ops.items()
    }


def all_openapi_responses():
    """全仓契约面：`{(方法, 路径模板): 该员声明的响应码字典}`（本轮 census 两支读这一份）"""
    from api.main import app

    return {
        (verb.upper(), path): spec["responses"]
        for path, ops in app.openapi()["paths"].items()
        for verb, spec in ops.items()
    }


def measure(ctx, key):
    """把某一员的臂逐个打出去：每臂必须命中它挂在的那一档，返回那员实际出现过的错误码"""
    seen = set()
    for code, arms in ARMS[key].items():
        for label, arm in arms:
            response = arm(ctx)
            assert response.status_code == code, (
                f"{key} 的「{label}」本该回 {code}，实际 {response.status_code} "
                f"{response.text[:160]}")
        seen.add(str(code))
    return seen


def disk_state(ctx):
    """磁盘读数三元组：备份目录里的文件名、`index.json` 的逐条形状、数据根顶层文件名"""
    index = json.loads((ctx.bd / "index.json").read_text(encoding="utf-8"))
    return (
        sorted(p.name for p in ctx.bd.iterdir()),
        [{k: row.get(k) for k in ("backup_id", "name", "item_count")} for row in index["backups"]],
        sorted(p.name for p in ctx.root.iterdir() if p.name != ".backups"),
    )


class TestTheContractEqualsTheMeasurement:
    """本轮主判据：声明的错误档集合 == 实测的错误档集合，两个方向同一把尺"""

    def test_the_family_is_still_six_operations(self):
        """六员的名单本身要钉住：多一员、少一员、改名，都先在这里红"""
        assert set(openapi_ops()) == set(SIX), sorted(openapi_ops())

    def test_declared_error_codes_equal_measured_ones(self, ctx):
        for key in SIX:
            got = set(openapi_ops()[key]) - AUTO
            want = measure(ctx, key)
            assert got == want, (
                f"{key} 声明的错误档 {sorted(got)} 与实测 {sorted(want)} 不一致："
                "少声明 = 客户端踩坑，多声明 = 客户端处理永远不会来的分支")

    def test_the_matrix_is_not_vacuous(self):
        """现量 20 格 / 29 臂：缩到比这少就说明有人把臂或声明删了，而不是尺子松了"""
        per_op = {k: len(v) for k, v in ARMS.items()}
        arms = sum(len(arm_list) for entry in ARMS.values() for arm_list in entry.values())
        assert per_op == {LIST: 2, CREATE: 4, RESTORE: 5, DELETE: 4, VERIFY: 2, CLEANUP: 3}, per_op
        assert arms == 29, arms
        assert sum(per_op.values()) == 20


class TestEveryArmHitsItsOwnVerdict:
    """逐员点名：把「集合相等」拆成一员一支，红的时候能直接说出是哪一员漂了"""

    def test_list_arms(self, ctx):
        assert measure(ctx, LIST) == EXPECTED[LIST]

    def test_create_arms(self, ctx):
        assert measure(ctx, CREATE) == EXPECTED[CREATE]

    def test_restore_arms(self, ctx):
        assert measure(ctx, RESTORE) == EXPECTED[RESTORE]

    def test_delete_arms(self, ctx):
        assert measure(ctx, DELETE) == EXPECTED[DELETE]

    def test_verify_arms(self, ctx):
        assert measure(ctx, VERIFY) == EXPECTED[VERIFY]

    def test_cleanup_arms(self, ctx):
        assert measure(ctx, CLEANUP) == EXPECTED[CLEANUP]

    def test_only_restore_admits_the_integrity_verdict(self, ctx):
        """409 是全家族独此一档：坏备份在 verify 里是一行读数，不是错误码"""
        others = {k: set(v) - AUTO for k, v in openapi_ops().items() if k != RESTORE}
        assert all("409" not in codes for codes in others.values()), others
        broken = ctx.http.get("/api/system/backups/verify?backup_id=b")
        assert broken.status_code == 200, broken.text
        row = broken.json()["results"][0]
        assert (row["exists"], row["checksum_match"]) == (True, False), row

    def test_the_expected_matrix_matches_the_declared_surface(self):
        """`EXPECTED` 与契约面同场比一次：让「实测矩阵」在守卫里也有一张可对账的表"""
        assert {k: set(v) - AUTO for k, v in openapi_ops().items()} == EXPECTED


class TestTheReadOnlyTwoAreNotClaimedAsAuthenticated:
    """只读两员的**反面**判据：它们真的不要 key，所以契约里也不许写 401

    这一族是本轮唯一「不许出现」型断言，扫描面收在契约那六员的声明集上（与禁令同族），
    并与行为面同场验证——写了 401 而行为不放行、或行为放行而没写 401，两侧都会红。
    """

    def test_neither_read_only_entry_declares_401(self):
        for key in READ_ONLY:
            assert "401" not in openapi_ops()[key], key

    def test_all_four_writes_declare_401(self):
        for key in SIX:
            if key in READ_ONLY:
                continue
            assert "401" in openapi_ops()[key], key

    def test_the_two_read_only_entries_serve_without_any_key(self, ctx):
        assert ctx.http.get("/api/system/backups").status_code == 200
        assert ctx.http.get("/api/system/backups/verify").status_code == 200

    def test_verify_never_returns_not_found_for_a_ghost_id(self, ctx):
        """契约里没有 404，就必须真的回 200 空结果——声明与行为同一件事"""
        ghost = ctx.http.get("/api/system/backups/verify?backup_id=ghost")
        assert ghost.status_code == 200, ghost.text
        assert ghost.json()["results"] == []


class TestTheContractSaysWhichKnobOrWhichState:
    """说明面：每一档要说清是哪颗旋钮或哪一种状态，响应体声明成 `detail` 字符串"""

    def test_every_declared_error_code_has_a_specific_description(self):
        for key, responses in openapi_ops().items():
            texts = []
            for code, spec in sorted(responses.items()):
                if code in AUTO:
                    continue
                text = spec.get("description") or ""
                assert len(text) >= 12, (key, code, text)
                assert text not in ("Successful Response", "Validation Error"), (key, code)
                texts.append(text)
            assert len(set(texts)) == len(texts), (key, texts)

    def test_every_400_names_a_request_knob(self):
        """400 的说法是「你的请求不合法」，不点名哪颗旋钮等于没说"""
        for key, responses in openapi_ops().items():
            if "400" not in responses:
                continue
            text = responses["400"]["description"]
            assert {k for k in KNOBS if f"`{k}`" in text}, (key, text)

    def test_every_403_says_the_whitelist(self):
        for key, responses in openapi_ops().items():
            if "403" in responses:
                assert "白名单" in responses["403"]["description"], (key, responses["403"])

    def test_every_401_names_the_header(self):
        for key, responses in openapi_ops().items():
            if "401" in responses:
                assert "X-API-Key" in responses["401"]["description"], key

    def test_the_409_line_keeps_the_a299_promise(self):
        """契约里那句 409 必须带上「什么都没写」——那是 A299 收口的内容，不只是状态码"""
        text = openapi_ops()[RESTORE]["409"]["description"]
        assert "校验和" in text and "未被改写" in text, text

    def test_error_bodies_are_declared_as_detail_strings(self):
        for key, responses in openapi_ops().items():
            for code, spec in responses.items():
                if code in AUTO:
                    continue
                media = (spec.get("content") or {}).get("application/json")
                assert media, (key, code, spec)
                schema = media["schema"]
                assert schema["type"] == "object", (key, code, schema)
                assert set(schema["properties"]) == {"detail"}, (key, code, schema)
                assert schema["properties"]["detail"]["type"] == "string", (key, code)
                assert schema["required"] == ["detail"], (key, code, schema)

    def test_the_200_face_is_untouched(self):
        """补错误档没顺手改成功档：`200` 仍 `$ref` 到各自响应模型，说明仍是框架那句"""
        for key, responses in openapi_ops().items():
            spec = responses["200"]["content"]["application/json"]["schema"]
            assert spec["$ref"].rsplit("/", 1)[-1], (key, spec)
            assert responses["200"]["description"] == "Successful Response", key

    def test_the_knob_list_is_derived(self):
        """旋钮清单不是手抄的：三只请求模型的字段集并上查询参数 `backup_dir` 就是它"""
        from api.routes.system_ops import (
            BackupCleanupRequest,
            CreateBackupRequest,
            RestoreBackupRequest,
        )

        fields = set()
        for model in (CreateBackupRequest, RestoreBackupRequest, BackupCleanupRequest):
            fields |= set(model.model_fields)
        assert fields | {"backup_dir"} == KNOBS, sorted(fields)


class TestErrorRequestsLeaveNoTraceOnDisk:
    """29 个错误请求跑完，备份目录、索引、数据根三处逐字相同（A299 那一格扩到全家族）"""

    def test_the_whole_arm_sweep_is_side_effect_free(self, ctx):
        before = disk_state(ctx)
        for key in SIX:
            measure(ctx, key)
        after = disk_state(ctx)
        assert after == before, (before, after)
        assert {row["backup_id"] for row in after[1]} == {"a", "b"}

    def test_the_data_root_gained_no_restored_file(self, ctx):
        """restore 的五档全被拒在写盘之前：数据根顶层仍然只有 `p.json`"""
        for code, arms in ARMS[RESTORE].items():
            for label, arm in arms:
                assert arm(ctx).status_code == code, (label, code)
        assert [p.name for p in ctx.root.iterdir() if p.name != ".backups"] == ["p.json"]


class TestTheAutoFortyTwoTwoIsHonestOnlyOnThreeEntries:
    """422 的现量：框架见「有参数」就挂，但只有带请求体的三员真的打得出来（A336）"""

    def test_declared_on_all_six(self):
        for key in SIX:
            assert "422" in openapi_ops()[key], key
        assert len(BODY_BEARING) == 3 and len(READ_ONLY) == 2

    def test_reachable_on_the_body_bearing_three(self, ctx):
        cases = [
            ("/api/system/backups", {"name": "x"}),
            ("/api/system/backups/a/restore", {}),
            ("/api/system/backups/cleanup", {"max_backups": "abc"}),
        ]
        for path, payload in cases:
            response = _post(ctx, path, payload)
            assert response.status_code == 422, (path, response.text[:120])
            assert isinstance(response.json()["detail"], list), path

    def test_unreachable_on_the_three_without_a_body(self, ctx):
        """`backup_dir` 是字符串查询参数：任何值都合法，六档尝试没有一档落到 422

        读数逐格来自探针（2026-10-13）：空串 ⇒ 400、根外绝对路径 ⇒ 403、重复给值取**最后一个**
        （先根外后 `x` ⇒ 200，先 `x` 后根外 ⇒ 403，`x`/`y` 这类相对名走「首个根解释」那条兜底所以
        合法 —— delete 那一档因此是 404「那个目录里没有 a」，而不是 403）、多余键被忽略 ⇒ 200 / 404。
        摘掉自动 422 要动契约生成路径，属 A336 待拍，本轮只钉事实。
        """
        attempts = [
            ctx.http.get("/api/system/backups?backup_dir="),
            ctx.http.get(f"/api/system/backups?backup_dir={outside_dir(ctx)}&backup_dir=x"),
            ctx.http.get(f"/api/system/backups?backup_dir=x&backup_dir={outside_dir(ctx)}"),
            ctx.http.get("/api/system/backups/verify?backup_id=ghost&bogus=1"),
            ctx.http.delete(f"/api/system/backups/a?backup_dir={outside_dir(ctx)}&backup_dir=y",
                            headers=ctx.key),
            ctx.http.delete("/api/system/backups/ghost?bogus=", headers=ctx.key),
        ]
        codes = [r.status_code for r in attempts]
        assert codes == [400, 200, 403, 200, 404, 404], codes
        assert 422 not in codes


class TestTheFamilyCensus:
    """全仓读数：错误档今天只有备份一族有，其余 71 员仍按框架默认声明"""

    def test_only_the_backup_family_declares_error_codes(self):
        with_extra = {k for k, v in all_openapi_responses().items() if set(v) - AUTO}
        assert with_extra == set(SIX), sorted(with_extra)

    def test_the_measured_split_of_the_whole_surface(self):
        """现量（2026-10-13）：77 员 = 6（本轮补的）+ 58（`{200,422}`）+ 13（只有 `200`）

        那 13 员全是**无任何参数**的 GET（`/api/health`、`/api/config`、`/api/models` 那一类），
        框架连 422 都不挂，所以它们不在 A336 的面上。新增端点会动这三个数：按现量重钉并写归因，
        不许把尺子改松。
        """
        sets = [set(v) for v in all_openapi_responses().values()]
        assert len(sets) == 77, len(sets)
        assert sum(1 for s in sets if s == AUTO) == 58
        assert sum(1 for s in sets if s == {"200"}) == 13
        assert sum(1 for s in sets if s - AUTO) == 6

    def test_no_operation_declares_a_code_we_cannot_show(self):
        """六员声明过的错误档只有这五码：出现第六码就说明有人补了没实证的档"""
        codes = {c for v in openapi_ops().values() for c in v if c not in AUTO}
        assert codes == {"400", "401", "403", "404", "409"}, codes
