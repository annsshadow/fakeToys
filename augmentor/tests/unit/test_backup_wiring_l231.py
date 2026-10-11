# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L231：备份那一族四条端点接进系统状态页之后，形状、判决、口径与那道越界闸都不许漂

**改前实测**（`git show HEAD:./web/src/services/api.ts` / 同 `pages/System.tsx`，L231 取证）：

| 事实 | 改前 | 改后 |
| --- | --- | --- |
| 前端调用 `/api/system/backups` 一族 | **0 处**（4 条全在 l224 的 `UNCONSUMED` 里） | `listBackups` / `createBackup` / `restoreBackup` / `deleteBackup` + 一张「数据备份」卡 |
| `name="../../escaped"` 打创建 | **200**，且真写到备份目录之外（探针量到 tmp 顶层多出 `escaped.json`） | **400**「备份名称不能指向备份目录之外: ../../escaped」，顶层零文件 |

**仓外探针量到的行为矩阵**（白名单根为临时目录，探针两支跑完即删）：

* 空目录列表 ⇒ **200** `{"backups": []}`，不是 404 也不是 null。
* 创建不给 `name` ⇒ 200，`backup_id` 是 `backup_<YYYYmmdd_HHMMSS>`（秒级），回执七键
  （`backup_id` / `timestamp` / `source_path` / `backup_path` / `item_count` / `file_size` /
  `checksum`），`source_path` 与 `backup_path` 都是**服务端绝对路径**，`checksum` 是备份文件的
  sha256，实测 `item_count 5` / `file_size 231`。
* 同名创建两次（`name=mybak`）⇒ 列表回**三条**（时间戳 id 一条 + `mybak` 两条），磁盘上只有
  **一份** `mybak.json`；两条 `mybak` 的 `timestamp` 不同（`.991804` 与 `.997820`），
  `backup_path` 逐字相同（**A298**）。
* 删一次 ⇒ 索引里**两条同 id 记录一起消失**（delete 按 id 过滤全部匹配项），之后再恢复 / 再删
  都是 **404**「备份不存在」（**A298**）。
* 恢复 ⇒ 回执三键 `{backup_id, output_path, item_count}`，落点键叫 **`output_path`**（同族其余
  五只写盘端点叫 `output_file`，`split` 叫 `output_dir`）；目标已存在时**整份覆盖**，不问。
* 把备份文件改坏再恢复 ⇒ **200**，只在日志里留一句 `WARNING 备份文件校验和不匹配，文件可能已损坏`，
  交回来的就是改坏的内容（实测 `item_count 1`）⇒ checksum 算了、存了、比了，但**从不拦截**（**A299**）。
* 幽灵 id ⇒ 恢复与删除都 **404**「备份不存在」（同一个状态码、同一句文案）。
* 源文件不存在 ⇒ **404**「文件不存在」；`name=""` ⇒ **200** 且回时间戳 id（`name or …` 把空串
  读成没给）。
* `backup_dir` 四档：空串 ⇒ **400**「目录路径 不能为空」；`../outside` ⇒ **400**「路径包含非法
  组件」；白名单外绝对路径 ⇒ **403**「路径超出允许的数据目录范围」；白名单内绝对路径 ⇒ **200**。
* 源与恢复目标同受白名单管：源给白名单外绝对路径 ⇒ **403**，给 `../outside/secret.json` ⇒
  **400**；恢复目标给白名单外绝对路径 ⇒ **403**，给 `..` ⇒ **400**，给空串 ⇒ **400**「文件路径
  不能为空」。三档都**没有**任何文件落到白名单之外（探针跑完逐目录点过）。
* `name="sub/deep"` ⇒ **200**，在备份目录下建 `sub/` 再落 `deep.json`；带 `/` 的名字放行，
  带 `..` 的名字被本轮那道闸拦下（闸**不是一刀切禁分隔符**，见 `TestTheTraversalGateDiscriminates`）。
* `clean_old_backups` 与 `verify_backup_integrity` 在 SDK 里都存在且可调用，而
  `GET /api/system/backups/ck/verify` ⇒ **404**（没有这只端点）⇒ 页面上做不到「清理旧备份」与
  「单独校验一份备份」（**A300**）。
* 「默认备份目录」在仓内有**三份**：CLI 的 `--backup-dir` 默认 `.backups`（相对工作目录）、SDK
  四个模块级函数的形参默认也是 `.backups`、端点走 `default_backup_dir()`＝白名单首个根目录下的
  `.backups`（出厂 `data/.backups`）。端点总是显式传目录 ⇒ SDK 那份默认在 HTTP 面上不可达，
  但 CLI 与页面看的**不是同一个目录**（**A301**）。
* 三只写操作都挂 `Depends(verify_api_key)`，`GET` 那一只没挂；`AUGMENTOR_API_KEY` 一旦设上，
  页面上这三只一律 401（**A302**）。当时量到「前端零通道」，**L233 已补上那条通道**。
* `backup()` 里有**两次** `datetime.now()`：一次取秒级时间戳进 id，一次取 isoformat 进
  `timestamp` ⇒ 跨秒时两者对不上（**A303**）；`name` 本身不在回执里，它就是 `backup_id`。
* `/api/data/list` 只列白名单根目录下名为 `train_data*.json` 的文件（探针放了 `p.json` /
  `train_data1.json` / `train_dataX.json` 三份，回执只有后两份）⇒ 备份源的候选口径**窄于**
  白名单，页面因此用可输入下拉而不是纯下拉（**A304**）。

跑法：`python -m pytest tests/unit/test_backup_wiring_l231.py -q --no-cov`
"""

import ast
import json
import pathlib
import re

import pytest
from tests.unit.test_transform_wiring_l229 import (
    fetcher_body,
    ts_interface_keys,
)
from tests.unit.test_wired_dropdown_choices_l226 import (
    needs_web_src,
    strip_comments,
)

from api.deps import default_backup_dir, verify_api_key
from api.routes.system_ops import (
    BackupInfoResponse,
    CreateBackupRequest,
    RestoreBackupRequest,
)
from api.routes.system_ops import BackupListResponse as PyBackupListResponse
from api.routes.system_ops import RestoreBackupResponse as PyRestoreBackupResponse
from api.schemas import SuccessResponse

ROOT = pathlib.Path(__file__).resolve().parents[2]
SRC = ROOT / "web" / "src"
PAGE_TSX = SRC / "pages" / "System.tsx"
API_TS = SRC / "services" / "api.ts"
BACKUP_PY = ROOT / "augmentor" / "backup.py"
PARSER_PY = ROOT / "augmentor" / "cli" / "parser.py"

#: 探针用的 5 条语料：`item_count 5` / `file_size 231` 两个读数都来自这份内容
ITEMS = [{"instruction": f"q{i}", "input": "", "output": f"a{i}"} for i in range(5)]

#: 四条操作 ⇒ 服务层里那条 fetcher 的名字
FETCHERS = {
    ("GET", "/api/system/backups"): "listBackups",
    ("POST", "/api/system/backups"): "createBackup",
    ("POST", "/api/system/backups/{}/restore"): "restoreBackup",
    ("DELETE", "/api/system/backups/{}"): "deleteBackup",
}


def _page() -> str:
    assert PAGE_TSX.is_file(), "前端源码不在场 ⇒ 本文件的判据全部失效"
    return strip_comments(PAGE_TSX.read_text(encoding="utf-8"))


def _api_ts() -> str:
    return strip_comments(API_TS.read_text(encoding="utf-8"))


@pytest.fixture
def http_with_backups(tmp_path, monkeypatch):
    """白名单数据根：`p.json`(5 条) + 两份 `train_data*.json`；根**外**另有 `outside/`"""
    root = tmp_path / "ds"
    root.mkdir()
    outside = tmp_path / "outside"
    outside.mkdir()
    (root / "p.json").write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "train_data1.json").write_text("[]", encoding="utf-8")
    (root / "train_dataX.json").write_text("[]", encoding="utf-8")
    (outside / "secret.json").write_text("[]", encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root, outside, tmp_path


class TestTheFourFetchersMatchTheContract:
    """服务层那四条：动词、路径、body 与 query 的分工逐个对上契约"""

    def test_the_four_fetchers_hit_the_four_paths(self):
        for (verb, path), name in FETCHERS.items():
            body = fetcher_body(name)
            # 服务层的路径不带 `/api` 前缀（那一份住在 `baseURL` 里，l224 从源码读它）
            literal = path.replace("{}", "${backupId}").removeprefix("/api")
            assert f"api.{verb.lower()}(" in body, f"{name} 不是 {verb} 调用"
            assert literal in body, f"{name} 里没有 {path}（找的是 {literal}）"

    def test_nothing_else_in_the_service_layer_touches_the_backup_family(self):
        hits = re.findall(r"api\.(?:get|post|put|delete|patch)\([^)]*?/system/backups",
                          _api_ts().replace("\n", " "))
        assert len(hits) == 4, f"备份一族的调用现量 4 处，实得 {len(hits)}：{hits}"

    def test_backup_dir_is_a_query_param_and_never_a_body_key(self):
        for name in ("listBackups", "createBackup", "restoreBackup", "deleteBackup"):
            body = fetcher_body(name)
            assert "params: { backup_dir:" in body, f"{name} 没把 backup_dir 当 query 发"
        for name in ("createBackup", "restoreBackup"):
            body = fetcher_body(name)
            json_body = re.search(r"(?:const body = |,\s*)(\{[^{}]*\})", body)
            assert json_body, f"{name} 的请求体形状变了 ⇒ 判据要跟着换"
            assert "backup_dir" not in json_body.group(1), f"{name} 把 backup_dir 塞进了请求体"

    def test_request_bodies_carry_exactly_the_contract_keys(self):
        create = fetcher_body("createBackup")
        assert "input_file: inputFile" in create
        assert "name: opts.name || undefined" in create
        assert set(CreateBackupRequest.model_fields) == {"input_file", "name"}
        restore = fetcher_body("restoreBackup")
        assert "output_file: outputFile" in restore
        assert set(RestoreBackupRequest.model_fields) == {"output_file"}

    def test_the_service_layer_writes_no_second_default_for_the_backup_dir(self):
        """`.backups` 这个字面量只许住在后端（`api/deps.py` 与 SDK 形参），服务层**代码**里一份都不写

        页面正文里出现 `.backups` 是**诚实口径**（告诉用户缺省落点是哪、CLI 那份默认不是同一个
        目录，A301），不是第二份默认值 —— 判据只看代码面：`strip_comments` 之后的服务层。
        """
        assert ".backups" not in _api_ts(), "服务层代码里出现了第二份备份目录默认值"
        page = _page()
        assert not re.search(r"backup_?[Dd]ir\w*\s*[:=]\s*['\"]", page), \
            "页面上给 backup_dir 写死了值 ⇒ 那就是第二份默认"


class TestTsFaceMatchesThePydanticModels:
    """TS 侧三个接口与线上响应模型逐键相等（含 DELETE 复用的 `MutationResponse`）"""

    def test_backup_info_keys(self):
        assert set(ts_interface_keys("BackupInfo")) == set(BackupInfoResponse.model_fields)
        assert len(ts_interface_keys("BackupInfo")) == 7

    def test_backup_list_keys(self):
        assert set(ts_interface_keys("BackupListResponse")) == set(PyBackupListResponse.model_fields)

    def test_restore_receipt_keys(self):
        assert set(ts_interface_keys("RestoreBackupResponse")) == set(
            PyRestoreBackupResponse.model_fields)
        # 落点键叫 output_path 而不是同族其余五只的 output_file（A277 当年订正过文档）
        assert "output_path" in ts_interface_keys("RestoreBackupResponse")
        assert "output_file" not in ts_interface_keys("RestoreBackupResponse")

    def test_delete_reuses_the_shared_success_shape(self):
        assert set(ts_interface_keys("MutationResponse")) == set(SuccessResponse.model_fields)
        assert "MutationResponse" in fetcher_body("deleteBackup")


@needs_web_src
class TestThePageCarriesTheFourOperations:
    """页面那一面：四条都真被调到，且实测过的口径写在卡上"""

    def test_the_card_calls_all_four(self):
        page = _page()
        for name in FETCHERS.values():
            assert f"{name}(" in page, f"System.tsx 里没有调用 {name}"

    def test_candidates_come_from_the_data_list_endpoint(self):
        assert "getDataFiles(" in _page()

    def test_the_row_key_does_not_treat_backup_id_as_unique(self):
        """同 id 两行是**实测形状**（A298），所以行键必须拼上别的一格"""
        page = _page()
        found = re.search(r"rowKey=\{(.*?)\}\n", page)
        assert found, "备份表没有 rowKey ⇒ 同 id 两行会撞 React key"
        assert "backup_id" in found.group(1) and "timestamp" in found.group(1), found.group(1)

    def test_the_measured_caveats_stay_on_the_card(self):
        page = _page()
        for phrase in (
            "覆盖同一份文件",           # A298：同名覆盖
            "删一次两条一起消失",       # A298：删除按 id 清全部
            "不拦截恢复",               # A299：checksum 不闸恢复
            "output_path",              # 回执落点键名
            "clean_old_backups",        # A300：SDK 有函数
            "HTTP 面没有入口",          # A300：端点没有
            "不是同一个目录",           # A301：两个默认目录
            "X-API-Key",                # A302：设了 key 就 401
            "路径包含非法组件",         # 白名单三档之一
            "train_data*.json",         # A304：候选口径
        ):
            assert phrase in page, f"页面上少了这句口径：{phrase}"

    def test_the_page_offers_no_backup_dir_input(self):
        """页面刻意不送 `backup_dir`：那一格一送就要在服务层写第二份默认（见 A301）"""
        assert "backupDir" not in _page()


class TestMeasuredEndpointBehavior:
    """端点上的判决：逐格是探针量到的读数，不是从源码推的想象"""

    def test_empty_dir_lists_an_empty_array(self, http_with_backups):
        http, *_ = http_with_backups
        r = http.get("/api/system/backups")
        assert r.status_code == 200 and r.json() == {"backups": []}

    def test_create_receipt_has_the_seven_keys(self, http_with_backups):
        http, root, *_ = http_with_backups
        r = http.post("/api/system/backups", json={"input_file": "p.json"})
        assert r.status_code == 200, r.text
        got = r.json()
        assert set(got) == set(BackupInfoResponse.model_fields)
        assert got["item_count"] == 5
        assert got["file_size"] == 231
        assert re.fullmatch(r"backup_\d{8}_\d{6}", got["backup_id"]), got["backup_id"]
        assert pathlib.Path(got["source_path"]).is_absolute()
        assert pathlib.Path(got["backup_path"]).is_absolute()
        assert (root / ".backups" / f"{got['backup_id']}.json").is_file()

    def test_same_name_twice_leaves_two_index_rows_and_one_file(self, http_with_backups):
        """A298 的前半：索引按次追加，文件按名覆盖"""
        http, root, *_ = http_with_backups
        for _ in range(2):
            assert http.post(
                "/api/system/backups", json={"input_file": "p.json", "name": "mybak"}
            ).status_code == 200
        rows = http.get("/api/system/backups").json()["backups"]
        named = [b for b in rows if b["backup_id"] == "mybak"]
        assert len(named) == 2, rows
        assert named[0]["backup_path"] == named[1]["backup_path"]
        assert named[0]["timestamp"] != named[1]["timestamp"], "两条时间戳相同 ⇒ 行键会撞"
        assert sorted(p.name for p in (root / ".backups").glob("*.json")) == [
            "index.json", "mybak.json"]

    def test_one_delete_clears_every_row_with_that_id(self, http_with_backups):
        """A298 的后半：删一次两条索引项一起消失，之后恢复与删除都是 404"""
        http, root, *_ = http_with_backups
        for _ in range(2):
            http.post("/api/system/backups", json={"input_file": "p.json", "name": "mybak"})
        assert http.delete("/api/system/backups/mybak").json() == {"success": True}
        assert http.get("/api/system/backups").json()["backups"] == []
        assert not (root / ".backups" / "mybak.json").exists()
        for r in (http.post("/api/system/backups/mybak/restore", json={"output_file": "r.json"}),
                  http.delete("/api/system/backups/mybak")):
            assert r.status_code == 404 and r.json()["detail"] == "备份不存在", r.text

    def test_ghost_id_is_404_on_both_write_ops(self, http_with_backups):
        http, *_ = http_with_backups
        for r in (http.post("/api/system/backups/v_ghost/restore", json={"output_file": "r.json"}),
                  http.delete("/api/system/backups/v_ghost")):
            assert r.status_code == 404 and r.json()["detail"] == "备份不存在", r.text

    def test_restore_receipt_and_silent_overwrite(self, http_with_backups):
        http, root, *_ = http_with_backups
        http.post("/api/system/backups", json={"input_file": "p.json", "name": "b1"})
        (root / "r.json").write_text(json.dumps([{"instruction": "被覆盖前的内容"}]),
                                     encoding="utf-8")
        r = http.post("/api/system/backups/b1/restore", json={"output_file": "r.json"})
        assert r.status_code == 200, r.text
        assert set(r.json()) == {"backup_id", "output_path", "item_count"}
        assert r.json()["item_count"] == 5
        assert json.loads((root / "r.json").read_text(encoding="utf-8")) == ITEMS

    def test_a_corrupted_backup_still_restores_200(self, http_with_backups):
        """A299：checksum 只在日志里出声，不闸恢复 —— 交回来的就是改坏的内容"""
        http, root, *_ = http_with_backups
        http.post("/api/system/backups", json={"input_file": "p.json", "name": "ck"})
        (root / ".backups" / "ck.json").write_text(
            json.dumps([{"instruction": "被改坏的内容"}], ensure_ascii=False), encoding="utf-8")
        r = http.post("/api/system/backups/ck/restore", json={"output_file": "r4.json"})
        assert r.status_code == 200, r.text
        assert r.json()["item_count"] == 1
        assert json.loads((root / "r4.json").read_text(encoding="utf-8")) == [
            {"instruction": "被改坏的内容"}]

    def test_missing_source_is_404_and_empty_name_is_ignored(self, http_with_backups):
        http, *_ = http_with_backups
        r = http.post("/api/system/backups", json={"input_file": "nope.json"})
        assert r.status_code == 404 and r.json()["detail"] == "文件不存在", r.text
        r = http.post("/api/system/backups", json={"input_file": "p.json", "name": ""})
        assert r.status_code == 200 and r.json()["backup_id"].startswith("backup_"), r.text

    def test_backup_dir_four_ways(self, http_with_backups):
        http, root, outside, _ = http_with_backups
        cases = [
            ({"backup_dir": ""}, 400, "目录路径 不能为空"),
            ({"backup_dir": "../outside"}, 400, "路径包含非法组件"),
            ({"backup_dir": str(outside)}, 403, "路径超出允许的数据目录范围"),
        ]
        for params, code, detail in cases:
            r = http.get("/api/system/backups", params=params)
            assert (r.status_code, r.json()["detail"]) == (code, detail), r.text
        r = http.get("/api/system/backups", params={"backup_dir": str(root / ".backups")})
        assert r.status_code == 200, r.text

    def test_source_and_target_are_both_whitelisted(self, http_with_backups):
        """四个越界档一个都不写盘：白名单外绝对路径 403、带 `..` 400、空目标 400"""
        http, root, outside, tmp_path = http_with_backups
        http.post("/api/system/backups", json={"input_file": "p.json", "name": "b1"})
        assert http.post("/api/system/backups",
                         json={"input_file": str(outside / "secret.json")}).status_code == 403
        assert http.post("/api/system/backups",
                         json={"input_file": "../outside/secret.json"}).status_code == 400
        for payload, code, detail in (
            ({"output_file": str(outside / "leak.json")}, 403, "路径超出允许的数据目录范围"),
            ({"output_file": "../outside/leak2.json"}, 400, "路径包含非法组件"),
            ({"output_file": ""}, 400, "文件路径 不能为空"),
        ):
            r = http.post("/api/system/backups/b1/restore", json=payload)
            assert (r.status_code, r.json()["detail"]) == (code, detail), r.text
        assert sorted(p.name for p in outside.iterdir()) == ["secret.json"]
        assert sorted(p.name for p in tmp_path.glob("*.json")) == []
        assert not (root / "leak.json").exists()


class TestTheTraversalGateDiscriminates:
    """本轮收掉的那一格：`name` 里的 `..` 被拒，而 `/` 仍然放行（不是一刀切禁分隔符）"""

    def test_traversal_name_is_400_and_writes_nothing_outside(self, http_with_backups):
        http, root, _, tmp_path = http_with_backups
        r = http.post("/api/system/backups",
                      json={"input_file": "p.json", "name": "../../escaped"},
                      params={"backup_dir": str(root / ".backups")})
        assert r.status_code == 400, r.text
        assert r.json()["detail"] == "备份名称不能指向备份目录之外: ../../escaped"
        assert sorted(p.name for p in tmp_path.glob("*.json")) == []
        assert not list(tmp_path.rglob("escaped.json")), "越界文件还是落盘了"

    def test_a_name_with_a_separator_is_still_allowed_inside(self, http_with_backups):
        http, root, *_ = http_with_backups
        r = http.post("/api/system/backups", json={"input_file": "p.json", "name": "sub/deep"})
        assert r.status_code == 200, r.text
        assert (root / ".backups" / "sub" / "deep.json").is_file()

    def test_the_gate_raises_the_type_that_maps_to_400(self):
        """闸抛的是 `DataValidationError`（`ValueError` 子类）⇒ `to_http_error` 读成 400 而不是 500"""
        src = BACKUP_PY.read_text(encoding="utf-8")
        assert "raise DataValidationError(f\"备份名称不能指向备份目录之外: {name}\")" in src
        from augmentor.exceptions import DataError, DataValidationError
        assert issubclass(DataValidationError, DataError)
        assert issubclass(DataValidationError, ValueError)

    def test_the_sdk_rejects_the_escape_before_touching_the_disk(self, tmp_path):
        """直接打 SDK 也是同一档判决：闸住在库里，不只住在 HTTP 面"""
        from augmentor.backup import DatasetBackup
        from augmentor.exceptions import DataValidationError

        root = tmp_path / "ds"
        root.mkdir()
        source = root / "p.json"
        source.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
        manager = DatasetBackup(str(root / ".backups"))
        with pytest.raises(DataValidationError) as exc:
            manager.backup(str(source), name="../../escaped")
        assert "../../escaped" in str(exc.value)
        assert sorted(p.name for p in tmp_path.glob("*.json")) == []


class TestTheEntriesThatAreStillMissing:
    """A300 / A301 / A302 / A303 / A304：五条缺口各自的**现量**形状钉在这里"""

    def test_two_sdk_functions_have_no_http_entry(self):
        """A300：函数在、可调用，而契约面一个都不认

        判据读 `app.openapi()` 而不是 `app.routes`：这版 Starlette 的 `app.routes` 不展平
        （included router 是占位对象），拿它当语料会得到一份空名单，尺子就瞎了。
        """
        import augmentor.backup as bk
        from api.main import app

        assert callable(bk.clean_old_backups) and callable(bk.verify_backup_integrity)
        paths = app.openapi()["paths"]
        assert "/api/system/backups" in paths, "语料本身没读到备份面 ⇒ 判据空转"
        # 只数备份面这一族：早先写成「全契约里不许出现 verify / clean 字样」，实测被
        # `/api/quality/clean` 判红 —— 那是一条与备份无关的端点，尺子量错了范围。
        ops = {(m.upper(), p) for p, ms in paths.items()
               if p.startswith("/api/system/backups") for m in ms}
        assert ops == {
            ("GET", "/api/system/backups"), ("POST", "/api/system/backups"),
            ("POST", "/api/system/backups/{backup_id}/restore"),
            ("DELETE", "/api/system/backups/{backup_id}"),
        }, sorted(ops)

    def test_three_defaults_for_the_backup_dir(self, http_with_backups):
        """A301：CLI、SDK 形参、端点各有一份「默认」，而端点那份与前两份不是同一个目录"""
        _, root, *_ = http_with_backups
        cli = re.search(r'add_argument\("--backup-dir"[^)]*default="([^"]+)"',
                        PARSER_PY.read_text(encoding="utf-8"))
        assert cli and cli.group(1) == ".backups", "CLI 的默认目录换了 ⇒ 这条缺口要重新量"
        sdk = re.findall(r'def \w+\([^)]*backup_dir: str = "([^"]+)"',
                         BACKUP_PY.read_text(encoding="utf-8"))
        assert sdk and set(sdk) == {".backups"}, sdk
        assert default_backup_dir() == root / ".backups"
        assert str(default_backup_dir()) != ".backups"

    def test_three_write_ops_are_key_gated(self):
        """A302 的**后端那一半**：三只写操作挂 `verify_api_key`，`GET` 那一只没挂

        这一支原先还钉着「前端没有送 key 的通道」两维（提到该头名的文件恰好一处、带引号的
        字面量全文零处）。**L233 把那个前提改掉了**：用户拍板「先补 key 通道，再继续接线」，
        `services/api.ts` 现在有一个请求拦截器，把「配置管理」页存进 localStorage 的那份 key
        挂到 `X-API-Key` 上 ⇒「零处」那一维当场成了假话。通道本身改由
        `test_api_key_channel_l233.py`（规模与形状）与 `web/src/services/api.test.ts`（行为）
        钉，这里只留后端事实，不各钉一份。
        """
        from api.routes.system_ops import router

        gated = set()
        seen = set()
        for route in router.routes:
            path = getattr(route, "path", "")
            if not path.startswith("/api/system/backups"):
                continue
            seen.add((sorted(route.methods)[0], path))
            for dep in route.dependant.dependencies:
                if dep.call is verify_api_key:
                    gated.add((sorted(route.methods)[0], path))
        assert seen == {
            ("GET", "/api/system/backups"),
            ("POST", "/api/system/backups"),
            ("POST", "/api/system/backups/{backup_id}/restore"),
            ("DELETE", "/api/system/backups/{backup_id}"),
        }, sorted(seen)
        assert gated == seen - {("GET", "/api/system/backups")}, sorted(gated)

    def test_the_id_timestamp_and_the_receipt_timestamp_come_from_two_calls(self):
        """A303：`backup()` 里两次 `datetime.now()`，秒级 id 与微秒级 timestamp 可以跨秒对不上"""
        tree = ast.parse(BACKUP_PY.read_text(encoding="utf-8"))
        fn = next(n for n in tree.body if isinstance(n, ast.ClassDef) and n.name == "DatasetBackup")
        backup_fn = next(n for n in fn.body if isinstance(n, ast.FunctionDef) and n.name == "backup")
        calls = [n for n in ast.walk(backup_fn)
                 if isinstance(n, ast.Call) and isinstance(n.func, ast.Attribute)
                 and n.func.attr == "now"]
        assert len(calls) == 2, f"backup() 里 datetime.now() 现量 2 次，实得 {len(calls)}"
        assert "%Y%m%d_%H%M%S" in BACKUP_PY.read_text(encoding="utf-8")

    def test_the_candidate_list_is_narrower_than_the_whitelist(self, http_with_backups):
        """A304：根目录里三份 json，`/api/data/list` 只回两份 `train_data*.json`"""
        http, *_ = http_with_backups
        names = [f["name"] for f in http.get("/api/data/list").json()["files"]]
        assert names == ["train_data1.json", "train_dataX.json"], names
        assert "p.json" not in names
