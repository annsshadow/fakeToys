# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L243：备份完整性那两道新判决（409 拦坏备份、cleanup 判份数）按实测逐个钉

A300 那两员进契约面之后，`test_backup_wiring_l231.py` 里那一支只留下「入口的形状」，
并写明**判决与形状由本文件钉**。这份用例就是那句话的兑现：把仓库外探针量到的行为矩阵
逐格搬成断言，别留「函数接上了但没人管它回什么」的中间态。

**改前实测**（探针两轮，跑完即删；权威是 `TestClient` 打的真端点）：

| 事实 | 改前（L231 量的） | 改后（本轮量的） |
| --- | --- | --- |
| 备份文件改坏再恢复 | **200**，交回坏内容，只在日志里一句 WARNING（**A299**） | **409**「备份文件校验和不匹配，文件可能已损坏，未执行恢复: b」，目标文件**不存在** |
| 幽灵 id 恢复 | 404「备份不存在」 | 404「备份不存在」（**没被 409 那档顺手吞掉**，见 `TestTheGateSeparatesThreeVerdicts`） |
| `GET /api/system/backups/verify` | **404**（端点不存在，**A300**） | 200 `{"results": [...]}`，五键逐只 |
| `POST /api/system/backups/cleanup` | **404**（同上） | 200 `{"deleted_count", "max_backups"}`；0 / -1 ⇒ **400**；`"abc"` ⇒ 422 |
| 恢复被拦之后磁盘 | — | 目标文件没建，索引里那格 checksum **也没被改写**（闸不自我修复） |

**四档「与直觉不同」的实测**，逐档都有对应用例，写在这里免得日后被人当缺陷改回去：

1. **缺目录时 verify 与 cleanup 都不建目录**（L243 当时是「verify 不建、cleanup 建」，
   差在 `DatasetBackup` 的构造无条件 `mkdir(parents=True, exist_ok=True)` 并落一份
   `index.json`：cleanup 走构造所以留垃圾，verify 靠路由里那道 `if not resolved.exists()`
   挡在构造之前）。L249 收 A332 / A307 时把副作用从构造里摘掉，六只只读面与「无债可清的
   清理」现在同源：**请求前后目录都不在场**，而响应仍是 200 空结果，两面口径差异随之消失
   （A307 盯的正是「只读端点写盘」）。
2. **同 id 两条索引记录在 verify 里塌成一行**（SDK 那份结果以 `backup_id` 为键），而且留下
   的是**索引里后出现的那一条**的 timestamp（探针把 dup 追加到末尾并写成 2020 ⇒ 回执里就是
   2020）。这是 A298 的另一面：列表出两行、完整性出那一行。
3. **零字节文件算丢**：`exists` 判的是「存在**且非空**」，所以 `z.json`（空文件）回
   `exists=false`、`checksum_match=false`，而改坏内容的 `b.json` 回 `exists=true`、
   `checksum_match=false` —— 两种 false 在页面上分列「文件丢了」与「对不上」。
4. **cleanup 的「保留 N 份」按索引行数算，不按磁盘文件数算**：探针造了 4 行索引
   （`a` 两行 + `b` + `z`）与 3 个文件，`max_backups=1` 回的是 `deleted_count=2`（不是 3）：
   删除算术取出 3 条待删记录，前两条都是 `a`，第一次 `delete_backup("a")` 按 id 连删两行
   并计 1，第二次对已消失的 `a` 返回 False **不计数**。留下的是时间戳最新的 `z`。

**鉴权那一半**（探针设了 `AUGMENTOR_API_KEY=secret`）：`GET …/verify` 无 key ⇒ **200**、
带**错** key 也 ⇒ **200**（只读面压根不读那个头）；`POST …/cleanup` 无 key ⇒ **401**
「缺少或无效的 X-API-Key」、带对 key ⇒ **200**。

**契约面看得见判决了（A330 已于 L252 收口）**：L243 立账时六员一律只声明 `{200, 422}`，那是当时的
现状事实；L252 走它写下的修法①，按实测给六员补齐 400 / 401 / 403 / 404 / 409。逐档实证（29 个真请求）
与「声明的错误档 == 实测的错误档」的对账住在 `tests/unit/test_backup_contract_error_codes_l252.py`。

跑法：`python -m pytest tests/unit/test_backup_integrity_l243.py -q --no-cov`
"""

import json
import pathlib

import pytest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SRC = ROOT / "web" / "src"
API_TS = SRC / "services" / "api.ts"
PAGE_TSX = SRC / "pages" / "System.tsx"
TYPES_TS = SRC / "types" / "api.ts"

#: 探针同款 5 条语料：`item_count 5` 与那份 sha256 都来自它
ITEMS = [{"instruction": f"q{i}", "input": "", "output": f"a{i}"} for i in range(5)]
CHECKSUM = "7c4463a1339d82cc94f5823483e9c23a3990671a9a2e1bcde83e0978148b8166"

GHOST_DETAIL = "备份不存在"
CORRUPT_DETAIL = "备份文件校验和不匹配，文件可能已损坏，未执行恢复: b"
NO_KEY_DETAIL = "缺少或无效的 X-API-Key"


def _client(tmp_path, monkeypatch, *, data_root=None, api_key=""):
    """白名单根 + key 都按环境变量现钉，回 (client, 数据根, 默认备份目录)"""
    root = data_root or tmp_path / "ds"
    root.mkdir(parents=True, exist_ok=True)
    (root / "p.json").write_text(
        json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8"
    )
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.setenv("AUGMENTOR_API_KEY", api_key)
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient

    from api.main import app

    return TestClient(app), root, root / ".backups"


def _create(client, name: str) -> dict:
    response = client.post(
        "/api/system/backups", json={"input_file": "p.json", "name": name}
    )
    assert response.status_code == 200, response.text
    return response.json()


def _corrupt(backup_dir: pathlib.Path, name: str) -> None:
    """把备份文件改成**合法 JSON 的坏内容**：与 L231 当年探针同一档（`item_count` 会变成 1）

    刻意不写成坏 JSON：那会先被恢复路径上的 JSON 解析判 400（臂 1 实测过），量不到这道闸。
    合法而内容不同 ⇒ 只有 checksum 这一格能发现它，正是 A299 那一档「交回来的就是坏的那份」。
    """
    (backup_dir / f"{name}.json").write_text(
        json.dumps([{"instruction": "被改坏的内容"}], ensure_ascii=False), encoding="utf-8"
    )


def _index(backup_dir: pathlib.Path) -> list:
    return json.loads((backup_dir / "index.json").read_text(encoding="utf-8"))["backups"]


def _write_index(backup_dir: pathlib.Path, rows: list) -> None:
    (backup_dir / "index.json").write_text(
        json.dumps({"backups": rows}, ensure_ascii=False), encoding="utf-8"
    )


@pytest.fixture
def three_backups(tmp_path, monkeypatch):
    """好 / 内容坏 / 零字节 三只备份 + 索引里再塞一条同 id 记录（实测 4 行、磁盘 3 份）"""
    client, root, bd = _client(tmp_path, monkeypatch)
    for name in ("a", "b", "z"):
        assert _create(client, name)["checksum"] == CHECKSUM
    _corrupt(bd, "b")
    (bd / "z.json").write_text("", encoding="utf-8")
    rows = _index(bd)
    # 同名创建两次的真实产物（A298）：这里手工塞，是为了把 timestamp 钉成**更旧**的 2020，
    # 从而能分辨「塌陷之后留下的是哪一条」——探针量到的是后写入的那条。
    dup = dict(rows[0])
    dup["timestamp"] = "2020-01-01T00:00:00"
    rows.append(dup)
    _write_index(bd, rows)
    return client, root, bd


class TestTheGateSeparatesThreeVerdicts:
    """三道判决互不塌：好 ⇒ 200、坏 ⇒ 409、幽灵 ⇒ 404"""

    def test_good_still_restores(self, tmp_path, monkeypatch):
        client, root, _bd = _client(tmp_path, monkeypatch)
        _create(client, "a")

        r = client.post("/api/system/backups/a/restore", json={"output_file": "ok.json"})

        assert r.status_code == 200, r.text
        assert r.json() == {"backup_id": "a", "output_path": str(root / "ok.json"),
                            "item_count": 5}

    def test_corrupt_is_409_and_ghost_is_404_in_the_same_run(self, three_backups):
        """两个状态码同场出现且各归各：409 没把 404 档挤掉，反之也没把 409 吞成 404

        `to_http_error(…, not_found_types=(BackupError,))` 只在 restore 那一支之外跑，而
        `BackupIntegrityError` 与 `BackupError` 是**平级**兄弟（`test_exceptions.py` 钉了
        层级）——若它成了子类，这一支的 409 会当场变成 404「备份不存在」，用户读到的是
        「没有这份备份」而不是「这份备份坏了」。
        """
        client, root, _bd = three_backups

        bad = client.post("/api/system/backups/b/restore", json={"output_file": "r4.json"})
        ghost = client.post(
            "/api/system/backups/ghost/restore", json={"output_file": "r5.json"}
        )

        assert bad.status_code == 409, bad.text
        assert bad.json()["detail"] == CORRUPT_DETAIL
        assert ghost.status_code == 404, ghost.text
        assert ghost.json()["detail"] == GHOST_DETAIL
        assert not (root / "r4.json").exists()
        assert not (root / "r5.json").exists()

    def test_the_fallthrough_would_have_been_400_not_404(self):
        """删掉端点里那句 `except BackupIntegrityError` 之后落哪一档：**400 原文**

        `BackupIntegrityError` 同时是 `ValueError` 的子类，而 `to_http_error` 的 ValueError
        分支把原文照回 ⇒ 400「boom」。所以那一句 409 是**路由里显式写的**，不是通用映射给的；
        这一支把「显式」量成数字。为什么不留在 400：400 的说法是「你的请求不合法」，用户会去改
        参数重试，而这里请求没毛病、坏的是服务端存着的**那份数据**——当前状态冲突是 409。
        另一维同场验证：它**不是** `BackupError` 的子类（`test_exceptions.py` 钉层级），
        所以 `not_found_types=(BackupError,)` 不会把它映射成 404。
        """
        from fastapi import HTTPException

        from api.deps import to_http_error
        from augmentor.exceptions import BackupError, BackupIntegrityError

        mapped = to_http_error(
            BackupIntegrityError("boom"),
            not_found="备份不存在",
            not_found_types=(BackupError,),
        )
        assert isinstance(mapped, HTTPException)
        assert (mapped.status_code, mapped.detail) == (400, "boom")
        assert not isinstance(BackupIntegrityError("x"), BackupError)


class TestTheGateIsNotABlanketRefusal:
    """闸只拦坏的那一只，且拦截时一个字节都不动"""

    def test_repairing_the_file_lets_the_same_id_through(self, three_backups):
        """同名再备份一次把文件修回来 ⇒ 同一只 id 恢复成功：闸只拦坏的，不是认 id 拉黑

        修法用的是现实里最朴素的那一条：源内容没错，重写一遍备份就对了。索引里因此多了
        第二条 `b`（A298 的形状），两条的 checksum 都是**同一份正确哈希**，所以恢复比到哪一条
        都过。
        """
        client, root, _bd = three_backups

        first = client.post(
            "/api/system/backups/b/restore", json={"output_file": "r6.json"}
        )
        assert first.status_code == 409, first.text

        assert _create(client, "b")["checksum"] == CHECKSUM
        again = client.post(
            "/api/system/backups/b/restore", json={"output_file": "r6.json"}
        )

        assert again.status_code == 200, again.text
        assert again.json()["item_count"] == 5
        assert json.loads((root / "r6.json").read_text(encoding="utf-8")) == ITEMS

    def test_a_refusal_leaves_the_index_checksum_alone(self, three_backups):
        """409 之后索引里那格仍是**原值**：闸既不写目标，也不「顺手」把新哈希写回索引

        后者听着像好事，其实是要拦的口径：把当前哈希写回索引等于让坏备份在下一次校验里
        变成好备份，完整性读数从此不可信。
        """
        client, _root, bd = three_backups
        before = {r["backup_id"]: r["checksum"] for r in _index(bd)}

        client.post("/api/system/backups/b/restore", json={"output_file": "r7.json"})

        after = {r["backup_id"]: r["checksum"] for r in _index(bd)}
        assert after == before
        assert before["b"] == CHECKSUM


class TestVerifyReportsEveryIndexRow:
    """`GET /api/system/backups/verify`：只读、不建目录、逐行哈希、同 id 塌一行"""

    def test_missing_default_dir_is_an_empty_list_and_stays_absent(self, tmp_path, monkeypatch):
        client, _root, bd = _client(tmp_path, monkeypatch)

        r = client.get("/api/system/backups/verify")

        assert r.status_code == 200, r.text
        assert r.json() == {"results": []}
        assert not bd.exists(), "只读端点把备份目录建出来了 ⇒ A307 那一族又犯一次"

    def test_explicit_missing_backup_dir_is_not_created_either(self, tmp_path, monkeypatch):
        client, root, _bd = _client(tmp_path, monkeypatch)

        r = client.get("/api/system/backups/verify", params={"backup_dir": "nodir"})

        assert r.status_code == 200, r.text
        assert r.json() == {"results": []}
        assert not (root / "nodir").exists()

    def test_the_three_shapes_are_three_different_readings(self, three_backups):
        client, _root, _bd = three_backups

        rows = {r["backup_id"]: r for r in
                client.get("/api/system/backups/verify").json()["results"]}

        assert rows["a"] == {"backup_id": "a", "exists": True, "checksum_match": True,
                             "item_count": 5, "timestamp": "2020-01-01T00:00:00"}
        assert rows["b"]["exists"] is True and rows["b"]["checksum_match"] is False
        # 零字节算丢：exists 判的是「存在且非空」
        assert rows["z"]["exists"] is False and rows["z"]["checksum_match"] is False

    def test_duplicate_index_rows_collapse_into_one_and_keep_the_later_one(self, three_backups):
        """索引 4 行 ⇒ 完整性 3 行，且 `a` 那一格带的是**后写入**那条的 timestamp（2020）

        SDK 那份结果是 `dict[backup_id] = {...}`，同 id 后一条覆盖前一条；路由把它摊成列表时
        补上 `backup_id` 字段，顺序沿用索引。页面上因此「列表能出两行、完整性只出一行」，
        这条口径写在 System.tsx 的卡上，判据钉在这里。
        """
        client, _root, bd = three_backups

        rows = client.get("/api/system/backups/verify").json()["results"]

        assert len(_index(bd)) == 4
        assert len(rows) == 3
        assert [r["backup_id"] for r in rows] == ["a", "b", "z"]
        assert rows[0]["timestamp"] == "2020-01-01T00:00:00"

    def test_ghost_backup_id_is_an_empty_list_not_a_404(self, three_backups):
        """这一只端点的语义是「报告目录里哪些备份是坏的」，幽灵 id 既不好也不坏 ⇒ 空列表"""
        client, _root, _bd = three_backups

        r = client.get("/api/system/backups/verify", params={"backup_id": "ghost"})

        assert r.status_code == 200, r.text
        assert r.json() == {"results": []}

    def test_filtering_to_one_id_returns_exactly_that_row(self, three_backups):
        client, _root, _bd = three_backups

        rows = client.get("/api/system/backups/verify",
                           params={"backup_id": "b"}).json()["results"]

        assert len(rows) == 1
        assert set(rows[0]) == {"backup_id", "exists", "checksum_match", "item_count",
                                "timestamp"}
        assert rows[0]["checksum_match"] is False


class TestCleanupKeepsTheNewestByIndexRows:
    """`POST /api/system/backups/cleanup`：判参、保留算术、连文件一起删"""

    def test_absent_key_uses_the_server_default_ten(self, three_backups):
        client, _root, bd = three_backups

        r = client.post("/api/system/backups/cleanup", json={})

        assert r.status_code == 200, r.text
        assert r.json() == {"deleted_count": 0, "max_backups": 10}
        assert len(_index(bd)) == 4, "保留份数够 ⇒ 一条都不许动"

    @pytest.mark.parametrize("bad,value", [
        ("zero", 0),
        ("negative", -1),
    ])
    def test_zero_and_negative_are_400_with_the_require_count_wording(
            self, three_backups, bad, value):
        client, _root, bd = three_backups

        r = client.post("/api/system/backups/cleanup", json={"max_backups": value})

        assert r.status_code == 400, r.text
        assert r.json()["detail"] == f"max_backups 必须是不小于 1 的整数，当前是 {value}"
        assert len(_index(bd)) == 4, "判参没挡住 ⇒ 目录被动过了"

    def test_a_string_is_422_at_the_body_field(self, three_backups):
        client, _root, _bd = three_backups

        r = client.post("/api/system/backups/cleanup", json={"max_backups": "abc"})

        assert r.status_code == 422, r.text
        first = r.json()["detail"][0]
        assert first["type"] == "int_parsing"
        assert first["loc"] == ["body", "max_backups"]

    def test_numeric_strings_are_coerced_by_pydantic(self, three_backups):
        """`"3"` 不是 422：pydantic 把它 coercion 成 3，判参之后走的是正常保留算术"""
        client, _root, _bd = three_backups

        r = client.post("/api/system/backups/cleanup", json={"max_backups": "3"})

        assert r.status_code == 200, r.text
        assert r.json()["max_backups"] == 3

    def test_keep_one_deletes_the_oldest_files_and_reports_two_not_three(self, three_backups):
        """4 行索引、3 个文件、保留 1 ⇒ `deleted_count == 2`，留下的是最新的 `z`

        删除算术按**索引行**取出 3 条待删（`a` 两行 + `b`），但 `delete_backup` 按 id 过滤
        全部匹配项：第一次删 `a` 连清两行并计 1，第二次对已消失的 `a` 返回 False **不计数**，
        再删 `b` 计 1 ⇒ 报 2。留下的必须是 timestamp 最新的那条（A298 的算术另一面）。
        """
        client, _root, bd = three_backups

        r = client.post("/api/system/backups/cleanup", json={"max_backups": 1})

        assert r.status_code == 200, r.text
        assert r.json() == {"deleted_count": 2, "max_backups": 1}
        assert [(b["backup_id"], b["timestamp"]) for b in _index(bd)] == [
            ("z", _index(bd)[0]["timestamp"])
        ]
        assert sorted(p.name for p in bd.iterdir()) == ["index.json", "z.json"]

    def test_keeping_more_than_exist_moves_nothing(self, three_backups):
        client, _root, bd = three_backups
        files_before = sorted(p.name for p in bd.iterdir())

        r = client.post("/api/system/backups/cleanup", json={"max_backups": 100})

        assert r.status_code == 200, r.text
        assert r.json() == {"deleted_count": 0, "max_backups": 100}
        assert sorted(p.name for p in bd.iterdir()) == files_before

    def test_a_missing_dir_stays_absent_even_for_this_write_endpoint(self, tmp_path, monkeypatch):
        """L249 收 A332 之后 cleanup 与 verify 同口径：没活可干就不建目录

        改前实测：`backup_dir=newdir` 回 200 `{0, 5}`、目录出现、里面只有 index.json，
        因为 `DatasetBackup` 构造即 mkdir 并落默认索引，而判据排在构造之后。摘掉构造的
        副作用之后，「清理一个还不存在的目录」仍然是 **200**（读盘交回内存空账，不炸），
        只是不再留一份空目录加一份空索引当垃圾。这一支钉的是新口径。
        """
        client, root, _bd = _client(tmp_path, monkeypatch)

        r = client.post("/api/system/backups/cleanup", json={"max_backups": 5},
                        params={"backup_dir": "newdir"})

        assert r.status_code == 200, r.text
        assert r.json() == {"deleted_count": 0, "max_backups": 5}
        assert not (root / "newdir").exists(), "无债可清的清理留下了目录 ⇒ 写面又提前落盘"


class TestTheAuthSplitMeasured:
    """设了 key：只读的 verify 两律放行（无 key、坏 key），写侧的 cleanup 一律 401"""

    def test_verify_never_reads_the_header(self, tmp_path, monkeypatch):
        client, _root, _bd = _client(tmp_path, monkeypatch, api_key="secret")

        assert client.get("/api/system/backups/verify").status_code == 200
        # 带**错** key 也放行：这一只端点不挂 `verify_api_key`，也就无从校验那个头
        assert client.get("/api/system/backups/verify",
                          headers={"X-API-Key": "wrong"}).status_code == 200

    def test_cleanup_is_401_without_the_key_and_200_with_it(self, tmp_path, monkeypatch):
        client, _root, _bd = _client(tmp_path, monkeypatch, api_key="secret")

        denied = client.post("/api/system/backups/cleanup", json={"max_backups": 5})
        assert denied.status_code == 401, denied.text
        assert denied.json()["detail"] == NO_KEY_DETAIL

        allowed = client.post("/api/system/backups/cleanup", json={"max_backups": 5},
                              headers={"X-API-Key": "secret"})
        assert allowed.status_code == 200, allowed.text


class TestTheContractAdmitsTheRuntimeVerdicts:
    """A330 的两面：L243 钉的是「契约只有 200 与 422」那个现状，L252 补齐之后重钉为现量

    这一支删不掉也不该删——它管的正是本轮新长出来的那一格：补齐的声明不许再缩回两只。
    六员逐档的实证与「声明的错误档集合 == 实测的错误档集合」由
    `tests/unit/test_backup_contract_error_codes_l252.py` 逐员守着，这里只钉 L243 那两员新端的读数。
    """

    def test_the_two_new_entries_now_admit_the_measured_verdicts(self):
        from api.main import app

        paths = app.openapi()["paths"]
        got = {
            op.upper(): sorted(spec["responses"])
            for op, spec in paths["/api/system/backups/verify"].items()
        } | {
            op.upper(): sorted(spec["responses"])
            for op, spec in paths["/api/system/backups/cleanup"].items()
        }

        assert got == {
            "GET": ["200", "400", "403", "422"],
            "POST": ["200", "400", "401", "403", "422"],
        }, got


class TestTheTwoEntriesAreModelledAndServed:
    """模型字段集、服务层默认值、页面两枚入口——三侧同一组数字，谁漂谁红"""

    def test_pydantic_shapes(self):
        from api.routes.system_ops import (
            BackupCleanupRequest,
            BackupCleanupResponse,
            BackupVerifyEntryResponse,
            BackupVerifyResponse,
        )

        assert set(BackupVerifyEntryResponse.model_fields) == {
            "backup_id", "exists", "checksum_match", "item_count", "timestamp"}
        assert set(BackupVerifyResponse.model_fields) == {"results"}
        assert set(BackupCleanupRequest.model_fields) == {"max_backups"}
        assert set(BackupCleanupResponse.model_fields) == {"deleted_count", "max_backups"}

    def test_the_server_default_and_the_service_layer_default_are_the_same_number(self):
        """两份「10」必须同数：`BackupCleanupRequest.max_backups` 与服务层 `?? 10`

        页面不送 `maxBackups` 时（InputNumber 被清空成一瞬间），落到服务层那份默认；如果它
        与后端默认不同数，同一次点击会有两种保留算术。l231 那条「服务层不许写第二份**目录**
        默认」管不到这里，所以数字单独钉一支。
        """
        from api.routes.system_ops import BackupCleanupRequest

        assert BackupCleanupRequest.model_fields["max_backups"].default == 10
        body = API_TS.read_text(encoding="utf-8")
        assert "max_backups: opts.maxBackups ?? 10" in body

    def test_the_page_offers_both_entries_and_no_backup_dir_input(self):
        page = PAGE_TSX.read_text(encoding="utf-8")
        assert "verifyBackups(" in page and "cleanupBackups(" in page
        assert 'aria-label="校验备份完整性"' in page and 'aria-label="清理旧备份"' in page
        # 份数那一格的下限：0 与负数是 400，所以页面把 min 钉在 1
        assert "min={1}" in page

    def test_ts_types_match_the_models(self):
        from tests.unit.test_transform_wiring_l229 import ts_interface_keys

        assert set(ts_interface_keys("BackupVerifyEntry")) == {
            "backup_id", "exists", "checksum_match", "item_count", "timestamp"}
        assert set(ts_interface_keys("BackupCleanupResponse")) == {
            "deleted_count", "max_backups"}
        assert "BackupVerifyResponse" in TYPES_TS.read_text(encoding="utf-8")


class TestInjectionArmsKeepTheGuardHonest:
    """三臂注入：把产品那一侧的判据摘掉，本文件的对应断言必须当场失效（零写入通道）"""

    def test_arm_blinding_the_checksum_check_puts_the_bad_content_back_on_disk(
            self, three_backups, monkeypatch):
        """臂 1：让 `_calculate_checksum` 恒回索引里那格 ⇒ 恢复不再被拦

        这一臂证明 `test_corrupt_is_409_and_ghost_is_404_in_the_same_run` 量的是那道闸，
        而不是「坏备份恰好恢复不了」。摘掉之后实测：**200**、目标文件出现、里面是
        `[{"instruction": "被改坏的内容"}]`（`item_count 1`）——与 L231 探针记下的 A299
        原判据逐字同形，那一支当年钉的就是这个读数。

        顺带量到一件事：坏法必须是「合法 JSON 而内容不同」。同一臂里若把文件写成坏 JSON，
        摘掉闸之后回的是 400「数据文件不是合法 JSON（第 1 行第 12 列）」⇒ 下游解析替这道闸
        挡了一半场景，那一半**不能**算这道闸的功劳。
        """
        from augmentor.backup import DatasetBackup

        monkeypatch.setattr(
            DatasetBackup, "_calculate_checksum",
            lambda self, path: CHECKSUM,
        )
        client, root, _bd = three_backups

        r = client.post("/api/system/backups/b/restore", json={"output_file": "r8.json"})

        assert r.status_code == 200, r.text
        assert r.json()["item_count"] == 1
        assert json.loads((root / "r8.json").read_text(encoding="utf-8")) == [
            {"instruction": "被改坏的内容"}
        ]

    def test_arm_noop_require_count_lets_negative_delete_everything(
            self, three_backups, monkeypatch):
        """臂 2：把 `require_count` 摘成 no-op ⇒ `max_backups=-1` 从 400 变 200 且删光

        证明 `test_zero_and_negative_are_400…` 挡的是**那颗旋钮**，不是 pydantic 的类型判据。
        摘掉之后实测：200、`deleted_count 3`（待删的 4 条索引记录里 `a` 占两条，第一次删 `a`
        连清两行计 1，第二次对已消失的 `a` 返回 False 不计，再各删 `b` 与 `z` 一计）、索引成空壳
        ——与 `clean_old_backups` docstring 里那段「HEAD 的删除算术在负数上等于把目录删光」
        逐字对上。
        """
        import augmentor.backup as bk

        monkeypatch.setattr(bk, "require_count", lambda *a, **k: None)
        client, _root, bd = three_backups

        r = client.post("/api/system/backups/cleanup", json={"max_backups": -1})

        assert r.status_code == 200, r.text
        assert r.json()["deleted_count"] == 3
        assert _index(bd) == [], _index(bd)

    def test_arm_stubbing_run_in_thread_empties_the_verify_readings(
            self, three_backups, monkeypatch):
        """臂 3：把 `run_in_thread` 换成恒回空字典的假线程 ⇒ verify 从 3 行变 0 行

        证明 `TestVerifyReportsEveryIndexRow` 真的在跑那次逐只哈希，而不是从索引里凑字段：
        摘掉之后目录、索引都在场，回执却是空列表。
        """
        import api.routes.system_ops as so

        async def fake(*_a, **_k):
            return {}

        monkeypatch.setattr(so, "run_in_thread", fake)
        client, _root, _bd = three_backups

        assert client.get("/api/system/backups/verify").json() == {"results": []}


class TestTheTwoNewEndpointsFinishTheirArms:
    """收尾两臂的点名用例：越界的 403 原样重抛、非 `ValueError` 收敛成 500

    全量门禁第一次跑出来的覆盖率是 99.81 %，比 L242 终态的 99.84 % 退了一档，多出的
    5 条未覆盖语句全落在本轮新写的两员上：verify 的 `except HTTPException: raise` 与
    `except Exception → to_http_error`、cleanup 的 `except HTTPException: raise`。
    cleanup 的兜底臂已经被「0 / -1 ⇒ 400」那两支走过（`require_count` 抛的是 `ValueError`，
    正是 `to_http_error` 的 400 支），所以只差这三臂。

    为什么要点名而不是留着当「边角料不计较」：同族既有四员（列表 / 创建 / 恢复 / 删除）
    的这两臂各自都有用例，唯独新两员没有 ⇒ 「形状齐了、收尾臂没人走」是半成品。而且这两臂
    是**安全语义**的落点：重抛臂保证越界仍是 403 原文，兜底臂保证别的异常不会被 `str(e)`
    带出部署路径（A155①）。
    """

    def test_verify_outside_the_whitelist_is_403_verbatim(self, tmp_path, monkeypatch):
        """403 走的是重抛臂：若它落进兜底臂，用户读到的是 500 固定文案而不是「路径越界」

        `to_http_error` 的 docstring 里写着「`HTTPException` 不在这里处理，否则会被降级成
        500」——这一支就是把那句话在两员中的只读那一员上量成数字。
        """
        client, root, _bd = _client(tmp_path, monkeypatch)
        outside = root.parent / "elsewhere"

        r = client.get("/api/system/backups/verify", params={"backup_dir": str(outside)})

        assert r.status_code == 403, r.text
        assert r.json()["detail"] == "路径超出允许的数据目录范围"
        assert not outside.exists(), "被拒的请求不许在磁盘上留下任何东西"

    def test_cleanup_outside_the_whitelist_is_403_and_touches_nothing(self, tmp_path, monkeypatch):
        """写端点同臂同档，且**先判路径再动目录**：越界时连 `index.json` 都不许建

        与 `test_a_missing_dir_is_created_because_this_is_a_write_endpoint` 对照着读才成立：
        那一支说「合法目录即便不存在也会被 mkdir」，这一支说「非法目录即便不存在也绝不碰」。
        两只合起来才是 cleanup 的完整口径，缺一支都只是半句话。
        """
        client, root, _bd = _client(tmp_path, monkeypatch)
        outside = root.parent / "nowhere"

        r = client.post("/api/system/backups/cleanup", json={"max_backups": 1},
                        params={"backup_dir": str(outside)})

        assert r.status_code == 403, r.text
        assert r.json()["detail"] == "路径超出允许的数据目录范围"
        assert not outside.exists()

    def test_an_unclassified_failure_is_500_without_the_original_text(
            self, three_backups, monkeypatch):
        """verify 的兜底臂：非 `ValueError` ⇒ 500 固定文案，原文只落日志（A155①）

        真造一条「读得出目录、算不出哈希」的磁盘异常太看运气，所以按零写入通道在导入后的
        模块对象上打补丁——`verify_backup_integrity` 是路由函数体内 `from augmentor.backup
        import …`，patch 那一侧的属性就够着。断言取两格：状态码与「原文里的哨兵字符串不在
        响应体里」，后者才是这一臂存在的理由（`str(e)` 里通常是绝对路径）。
        """
        from api.deps import INTERNAL_ERROR_DETAIL
        import augmentor.backup as bk

        def boom(*_a, **_k):
            raise RuntimeError("/very/private/path/that/must/not/leak")

        monkeypatch.setattr(bk, "verify_backup_integrity", boom)
        client, _root, bd = three_backups

        r = client.get("/api/system/backups/verify")

        assert r.status_code == 500, r.text
        assert r.json()["detail"] == INTERNAL_ERROR_DETAIL
        assert "private" not in r.text
        assert len(_index(bd)) == 4, "只读端点失败时不许顺手改过索引"
