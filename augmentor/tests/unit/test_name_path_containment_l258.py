# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L258 / 新立账：调用方给的**名字**被当成路径片段用 ⇒ 一律限制在自己的目录之内

改前实测（仓外探针 `Temp/l258_probe.py`，沙箱内跑，三条最重的已经复现）：

- `VersionManager(storage_dir=<tmp>/versions).delete_version("../victim/important")`
  返回 `None`（成功）而沙箱里那个**版本根之外**的整目录被 `shutil.rmtree` 掉了。
- 同一只管理器的 `load_version("../secrets")` 交出目录外的 `data.json` 内容。
  HTTP 面这两条**没有一道闸**：`GET /api/versions/%2e%2e`（无鉴权）与
  `/api/versions/%2e%2e/data` 都直接打到改前的 **500** —— 500 不是「拦下了」，
  而是「路径被原样拼出去、恰好那一格没有文件」。
- `CheckpointManager.create_checkpoint("../../escape", 3)` 把断点文件写到
  `checkpoint_dir` **之外**（实测 `outside_exists=True`、目录内为空）。
- `DiskCache(cache_dir=<数据目录>).clear()` 把目录里**别人的** `*.json` 一起 unlink
  （实测用户数据集 `my_dataset.json` 在 clear 之后不存在）。

同一条形状还有两处只在 SDK/CLI 面上：`Exporter.export_batch` 的**数据集名称**与
`DatasetOperations.split_file` 的**分片前缀**（`{"../../evilsite/x": items}` 实测把
产物写到输出目录之外，返回路径里赫然带着 `..\\..\\`）。

**权威只有一把**：`augmentor/validation.require_within_dir(label, value, base)`。
它与 `api/deps.py` 的 `resolve_within_roots` 是同一口径的两个尺度（那一把管「HTTP
传入的路径只能落在数据白名单之内」，这一把管「一个名字不许逃出它自己的目录」），
SDK 面必须自带，因为 CLI 与直接调 SDK 的调用方**不经过** HTTP 那道闸。

**判据八组**：

1. 权威本身的形状：拒绝清单**逐档带自己的判决**（类型错三档 / 空串 / 纯空白 / `..` 头 /
   中段 `..` / 根绝对路径，另加 Windows 独有的盘符绝对两档），接受清单（裸名、子目录名）
   逐档点名；空值那一维**不重抄**，交回同族的 `require_string`，判决文案也就跟着那把
   尺子走。判决本身**不含解析后的绝对落点**（`INTERNAL_ERROR_DETAIL` 那一族的部署路径
   外泄面）。`task_id=".."` 这一档**按实测是接受**（拼接后是一个文件名字面量，落点仍在
   根内），判据单独钉住，防后来人改成「凡带点就拦」。
2. 调用点名册按**文件 → 次数**与实测相等（10 处 / 5 个文件）；包含判据的
   `.parents` 比较在 SDK 包内**恰好两处实现**（本把权威 + L231 给备份名落下的
   那条手判）——第三份手抄就是 L238 / L256 / L257 收掉的那一族。
3. 版本面：三只入口（`delete_version` / `load_version` / `get_version_info`）对
   `..` 一律出声，且**磁盘上那个目录还在**；合法 id 的落点仍在根内（零行为漂移）。
4. 断点面：三只拼路径的私有方法一起拦，且拦下之后目录外**零字节**落盘。
5. 导出与分割面：`export_batch` / `export_all_formats` / `split_file` 三处拦，
   合法名称照旧落在输出目录内；并行支路的「咽下判决」形状按实测**钉成已知限制**
   （写边拦住了，回执却是一片空 dict 且照样 `success: true`，另立账）。
6. 缓存面：`clear()` 只清自己记账的条目，外来 `*.json` 与 `_metadata.json` 都活着，
   且清完之后统计口径仍然归零（不是「没删所以没少」）。
7. HTTP 边界：越界的 `version_id` 拿 **400** 而不是 500，正文是 SDK 那句判决；
   而 `v_ghost` 那一档**照旧 500** —— A280（一刀切收敛）不在本轮处置面内，这一支
   钉的就是「本轮没有顺手把状态码契约改掉」。
8. 尺子有牙齿：把 `versioning.require_within_dir` 换成裸拼接（内存里的零写入注入），
   第 3 组那三支当场红 ⇒ 证明那一处调用点真的在读这把权威，而不是注释里提了它。

**八组全部是结构判据、磁盘事实判据或状态码判据，不量墙钟**（L239 / A286 的口径）。
"""

import ast
import hashlib
import json
import os
import re
import subprocess
from pathlib import Path

import pytest

from augmentor.cache import DiskCache
from augmentor.checkpoint import CheckpointManager
from augmentor.dataset_ops import DatasetOperations
from augmentor.exceptions import DataValidationError
from augmentor.export import Exporter
from augmentor.validation import require_within_dir
from augmentor.version_control import DatasetVersionManager
from augmentor.versioning import VersionManager

REPO = Path(__file__).resolve().parents[2]
SDK = REPO / "augmentor"

#: 现量（L258）：逐文件的**调用点**数（AST 里的 `require_within_dir(...)`），不含 import 行与定义。
MEASURED_CALL_SITES = {
    "augmentor/checkpoint.py": 3,
    "augmentor/dataset_ops.py": 1,
    "augmentor/export.py": 2,
    "augmentor/version_control.py": 3,
    "augmentor/versioning.py": 1,
}

#: 改前实测的拒绝清单，逐档带**该档自己的判决**（判决文案也是判据的一维：`5` 不是
#: 「空」，它是类型错；把三个维度混成一句话就等于三档只量了一档）。
#: 空值那一维**不在本把权威里重抄**：交回同族的 `require_string`（`augmentor/validation.py`
#: 里 L199 那一批标量判据的同族），所以三条空值文案是那把尺子的原文。
#: 盘符绝对路径那一族按平台分组：POSIX 上 `"C:/abs"` 只是一个合法的文件名，
#: 把它列进拒绝档会得到一个只在 Windows 绿的判据。
#: `"C:abs"`（盘符相对）在任何平台都**不在**这一档：它拼进 base 之后仍然落在 base
#: 之内，而包含判据要守的是「落点在不在根内」这一件事，不是「字符串长得危不危险」。
REJECTED_SHAPES = [
    (None, "必须是字符串"),
    (5, "必须是字符串"),
    ([], "必须是字符串"),
    ("", "不能是空字符串"),
    ("   ", "纯空白"),
    ("..", "之外"),
    ("../x", "之外"),
    ("a/../../b", "之外"),
]
ABSOLUTE_REJECTED_SHAPES = [(f"{os.path.abspath(os.sep)}outside", "之外")]
if os.name == "nt":
    ABSOLUTE_REJECTED_SHAPES += [("C:\\abs", "之外"), ("C:/abs", "之外")]
ACCEPTED_SHAPES = ["v_20261013_120000", "sub/dir_ok", "plain.json"]

#: 现量（L258）：`.parents` 那一档包含比较在 SDK 包内的两处实现。
MEASURED_CONTAINMENT_IMPLEMENTATIONS = ["augmentor/backup.py", "augmentor/validation.py"]


def _tree(rel: str) -> ast.Module:
    return ast.parse((REPO / rel).read_text(encoding="utf-8"))


def _call_count(rel: str) -> int:
    return sum(
        1
        for node in ast.walk(_tree(rel))
        if isinstance(node, ast.Call)
        and isinstance(node.func, ast.Name)
        and node.func.id == "require_within_dir"
    )


def _containment_implementations() -> list:
    """扫 SDK 包，返回出现「`X (not) in <某表达式>.parents`」这一档比较的文件

    `mkdir(parents=True)` 是关键字参数不是属性访问，不会命中；`api/deps.py` 用的是
    `relative_to` 那一档，不在本尺子的扫描面内（它守的是数据白名单，不是「一个名字」）。
    """
    hits = []
    for path in sorted(SDK.rglob("*.py")):
        for node in ast.walk(ast.parse(path.read_text(encoding="utf-8"))):
            if not isinstance(node, ast.Compare):
                continue
            ops = list(node.ops)
            comparators = list(node.comparators)
            for op, comparator in zip(ops, comparators):
                if isinstance(op, (ast.In, ast.NotIn)) and isinstance(comparator, ast.Attribute):
                    if comparator.attr == "parents":
                        hits.append(path.relative_to(REPO).as_posix())
    return sorted(set(hits))


class TestTheAuthorityIsOneFunction:
    def test_every_rejected_shape_is_named_with_its_own_verdict(self):
        base = Path(".")
        for shape, verdict in REJECTED_SHAPES + ABSOLUTE_REJECTED_SHAPES:
            with pytest.raises(DataValidationError) as exc:
                require_within_dir("版本 ID", shape, base)
            assert verdict in str(exc.value), (shape, str(exc.value))
            assert "版本 ID" in str(exc.value), (shape, str(exc.value))

    def test_every_accepted_shape_lands_inside_the_root(self, tmp_path):
        base = tmp_path / "versions"
        base.mkdir()
        for shape in ACCEPTED_SHAPES:
            resolved = require_within_dir("版本 ID", shape, base)
            assert base.resolve() in resolved.parents or resolved == base.resolve()

    def test_the_verdict_does_not_leak_the_resolved_absolute_path(self, tmp_path):
        base = tmp_path / "versions"
        base.mkdir()
        with pytest.raises(DataValidationError) as exc:
            require_within_dir("版本 ID", "../../outside", base)
        verdict = str(exc.value)
        assert str(base.resolve()) not in verdict
        assert str(tmp_path.resolve()) not in verdict
        assert re.search(r"[A-Za-z]:[\\/]", verdict) is None, verdict

    def test_the_containment_idiom_is_implemented_twice_not_thrice(self):
        assert _containment_implementations() == MEASURED_CONTAINMENT_IMPLEMENTATIONS


class TestCallSiteRoster:
    def test_call_sites_equal_the_measured_roster_per_file(self):
        measured = {rel: _call_count(rel) for rel in MEASURED_CALL_SITES}
        assert measured == MEASURED_CALL_SITES, measured

    def test_total_is_ten_across_five_files(self):
        assert sum(MEASURED_CALL_SITES.values()) == 10
        assert len(MEASURED_CALL_SITES) == 5

    @pytest.mark.parametrize("rel", sorted(MEASURED_CALL_SITES))
    def test_each_guarded_module_imports_the_authority_not_a_local_copy(self, rel):
        imported = [
            alias.name
            for node in ast.walk(_tree(rel))
            if isinstance(node, ast.ImportFrom)
            for alias in node.names
        ]
        assert "require_within_dir" in imported
        source = (REPO / rel).read_text(encoding="utf-8")
        assert "def require_within_dir" not in source, f"{rel} 里手抄了第二份权威"


class TestVersioningFace:
    def test_delete_cannot_rmtree_outside_the_version_root(self, tmp_path):
        root = tmp_path / "versions"
        root.mkdir()
        victim = tmp_path / "victim" / "important"
        victim.mkdir(parents=True)
        (victim / "precious.txt").write_text("DO NOT DELETE", encoding="utf-8")
        mgr = VersionManager(storage_dir=str(root))
        with pytest.raises(DataValidationError):
            mgr.delete_version("../victim/important")
        assert victim.exists()
        assert (victim / "precious.txt").read_text(encoding="utf-8") == "DO NOT DELETE"

    def test_load_cannot_read_a_data_json_outside_the_root(self, tmp_path):
        root = tmp_path / "versions"
        root.mkdir()
        outside = tmp_path / "secrets"
        outside.mkdir()
        (outside / "data.json").write_text(
            json.dumps([{"instruction": "OUTSIDE-SCOPE"}]), encoding="utf-8"
        )
        mgr = VersionManager(storage_dir=str(root))
        with pytest.raises(DataValidationError):
            mgr.load_version("../secrets")

    def test_get_version_info_rejects_the_same_shape_as_load(self, tmp_path):
        root = tmp_path / "versions"
        root.mkdir()
        mgr = VersionManager(storage_dir=str(root))
        with pytest.raises(DataValidationError):
            mgr.get_version_info("..")

    def test_a_legitimate_id_still_lands_inside_the_root(self, tmp_path):
        root = tmp_path / "versions"
        root.mkdir()
        mgr = VersionManager(storage_dir=str(root))
        assert mgr._get_version_dir("v_20261013_120000") == root.resolve() / "v_20261013_120000"

    def test_the_second_version_manager_shares_the_same_authority(self, tmp_path):
        """`version_control.DatasetVersionManager` 是同一条形状的第二把尺子持有者

        它的 `delete_version` 只在 id 已登记时才走到 rmtree，因此攻击面比 `VersionManager`
        窄；但 `load_version` 直接把 id 拼进路径，改前同样能读根外的 `data.json`。
        """
        root = tmp_path / ".versions"
        outside = tmp_path / "leak"
        outside.mkdir()
        (outside / "data.json").write_text("[]", encoding="utf-8")
        mgr = DatasetVersionManager(versions_dir=str(root))
        with pytest.raises(DataValidationError):
            mgr.load_version("../leak")
        # 实测：`delete_version` 在未登记时就短路回 False，压根走不到拼路径那一格
        # ⇒ 那处权威是纵深防御而不是当前可达面，判据按实测写而不是按「应该 raise」写。
        assert mgr.delete_version("../leak") is False
        assert (outside / "data.json").exists()


class TestCheckpointFace:
    def test_create_cannot_write_outside_the_checkpoint_dir(self, tmp_path):
        ckpt = tmp_path / "ckpt"
        ckpt.mkdir()
        mgr = CheckpointManager(checkpoint_dir=str(ckpt))
        with pytest.raises(DataValidationError):
            mgr.create_checkpoint("../../escape", 3)
        assert not (tmp_path.parent / "escape_checkpoint.json").exists()
        assert list(ckpt.iterdir()) == []

    def test_both_delta_paths_are_guarded(self, tmp_path):
        ckpt = tmp_path / "ckpt"
        ckpt.mkdir()
        mgr = CheckpointManager(checkpoint_dir=str(ckpt))
        for task_id in ("../x", "/abs", "a/../../b"):
            with pytest.raises(DataValidationError):
                mgr._get_delta_path(task_id)
            with pytest.raises(DataValidationError):
                mgr._get_legacy_delta_path(task_id)
            with pytest.raises(DataValidationError):
                mgr._get_checkpoint_path(task_id)

    def test_a_leading_double_dot_as_a_name_prefix_is_inert_and_stays_inside(self, tmp_path):
        """`task_id=".."` **不该**被拦：拼接发生在判据之前，那一格得到的是文件名字面量

        实测 `_get_checkpoint_path("..")` → `<ckpt>/.._checkpoint.json`，`Path.parts`
        里只有一个组件、落点仍在根内 ⇒ 包含不变量成立。这一支钉的是「这把尺子量的是
        路径组件，不是字符串长得危不危险」，与拒绝清单里 `"C:abs"` 那一档同一个理由；
        后来人若把它改成「凡带点就拦」，就会把合法任务 ID（`v1.2`）一起拦掉。
        """
        ckpt = tmp_path / "ckpt"
        ckpt.mkdir()
        mgr = CheckpointManager(checkpoint_dir=str(ckpt))
        path = mgr._get_checkpoint_path("..")
        assert path.name == ".._checkpoint.json"
        assert ckpt.resolve() in path.parents

    def test_a_legitimate_task_still_lands_inside_the_dir(self, tmp_path):
        ckpt = tmp_path / "ckpt"
        ckpt.mkdir()
        mgr = CheckpointManager(checkpoint_dir=str(ckpt))
        path = mgr._get_checkpoint_path("task_ok")
        assert path.name == "task_ok_checkpoint.json"
        assert ckpt.resolve() in path.parents


class TestExportAndSplitFace:
    ITEMS = [{"instruction": "i", "input": "", "output": "o"}]

    def test_export_batch_rejects_a_name_that_escapes_the_output_dir(self, tmp_path):
        out = tmp_path / "batchout"
        out.mkdir()
        with pytest.raises(DataValidationError):
            Exporter().export_batch(
                {"../../evilsite/x": self.ITEMS}, str(out), ["jsonl"], use_parallel=False
            )
        assert not (tmp_path.parent / "evilsite").exists()

    def test_export_all_formats_rejects_an_escaped_base_name(self, tmp_path):
        out = tmp_path / "allout"
        out.mkdir()
        with pytest.raises(DataValidationError):
            Exporter().export_all_formats(
                self.ITEMS, str(out), base_name="../../evil", use_parallel=False
            )
        assert not (tmp_path.parent / "evil_json.csv").exists()

    def test_split_file_rejects_an_escaped_prefix(self, tmp_path):
        src = tmp_path / "src.json"
        src.write_text(json.dumps(self.ITEMS * 12), encoding="utf-8")
        out = tmp_path / "splitout"
        with pytest.raises(DataValidationError):
            DatasetOperations().split_file(str(src), str(out), prefixes=("../../evil", "v", "t"))
        assert not (tmp_path.parent / "evil.json").exists()

    def test_legitimate_names_still_land_inside_their_roots(self, tmp_path):
        out = tmp_path / "okout"
        out.mkdir()
        result = Exporter().export_batch(
            {"train": self.ITEMS}, str(out), ["jsonl"], use_parallel=False
        )
        produced = Path(result["train"]["jsonl"])
        assert produced.exists() and out.resolve() in produced.parents

    def test_the_parallel_arm_swallows_the_verdict_and_reports_success(self, tmp_path):
        """已知限制（本轮只钉不动，另立账）：默认并行支路把判决咽成一片空 dict

        实测 `use_parallel=True`（出厂默认，`POST /api/export/batch` 走的就是这一档）：
        写边**确实拦住了**、目录外零产物，但 `except Exception: logger.error` 让
        `DataValidationError` 出不去，调用方拿到 `{"../../evilsite/x": {}}` 而 HTTP 面
        照样回 `success: true`。串行支路对同一条输入会 raise ⇒ 两档判决不同。
        本轮处置的是包含判据本身；这一档「失败冒充成功」按实测钉住，收口要改
        `export_batch` / `export_all_formats` 的错误语义（契约变更），不在本轮范围内。
        """
        out = tmp_path / "parout"
        out.mkdir()
        results = Exporter().export_batch({"../../silent/x": self.ITEMS}, str(out), ["jsonl"])
        assert results == {"../../silent/x": {}}
        assert not (tmp_path.parent / "silent").exists()


class TestCacheClearKeepsForeignFiles:
    def test_clear_leaves_foreign_json_alone(self, tmp_path):
        foreign = tmp_path / "my_dataset.json"
        foreign.write_text(json.dumps([{"instruction": "REAL USER DATA"}]), encoding="utf-8")
        cache = DiskCache(cache_dir=str(tmp_path))
        cache.set("k1", {"v": 1})
        assert cache.get("k1") == {"v": 1}
        DiskCache(cache_dir=str(tmp_path)).clear()
        assert foreign.exists(), "clear() 把别人的 *.json 删掉了（改前的实测形状）"
        assert json.loads(foreign.read_text(encoding="utf-8")) == [{"instruction": "REAL USER DATA"}]

    def test_clear_still_removes_every_entry_it_tracks(self, tmp_path):
        cache = DiskCache(cache_dir=str(tmp_path))
        for i in range(3):
            cache.set(f"k{i}", {"v": i})
        tracked = [
            tmp_path / f"{hashlib.md5(f'k{i}'.encode()).hexdigest()}.json" for i in range(3)
        ]
        assert all(p.exists() for p in tracked)
        DiskCache(cache_dir=str(tmp_path)).clear()
        assert not any(p.exists() for p in tracked), "记账的条目没清干净：clear() 变成空转"

    def test_the_metadata_file_survives_and_stats_zero_out(self, tmp_path):
        DiskCache(cache_dir=str(tmp_path)).set("k", {"v": 1})
        DiskCache(cache_dir=str(tmp_path)).clear()
        assert (tmp_path / "_metadata.json").exists()
        after = DiskCache(cache_dir=str(tmp_path)).stats
        assert after["entries"] == 0, after


class TestTheHttpBoundary:
    @pytest.fixture()
    def client(self):
        from fastapi.testclient import TestClient

        from api.main import app

        return TestClient(app, raise_server_exceptions=False)

    @pytest.mark.parametrize(
        "method,url",
        [
            ("get", "/api/versions/%2e%2e"),
            ("get", "/api/versions/%2e%2e/data"),
            ("delete", "/api/versions/%2e%2e"),
        ],
    )
    def test_the_escaped_id_is_a_four_hundred_not_a_five_hundred(self, client, method, url):
        resp = getattr(client, method)(url)
        assert resp.status_code == 400, (method, url, resp.status_code, resp.text)
        assert "之外" in resp.json()["detail"], resp.text

    def test_the_detail_carries_no_absolute_deploy_path(self, client):
        detail = client.get("/api/versions/%2e%2e/data").json()["detail"]
        assert str(REPO) not in detail, detail
        assert re.search(r"[A-Za-z]:[\\/]", detail) is None, detail

    def test_a_ghost_id_still_lands_on_five_hundred(self, client):
        """A280（一刀切收敛成 500）不在本轮处置面：这一支钉住本轮没有顺手改状态码契约"""
        assert client.get("/api/versions/v_ghost").status_code == 500

    def test_the_legitimate_version_faces_are_untouched(self, client):
        assert client.get("/api/versions").status_code == 200
        assert client.get("/api/versions/history").status_code == 200


class TestTheRulerHasTeeth:
    def test_removing_the_authority_from_versioning_reopens_the_rmtree(self, tmp_path, monkeypatch):
        """零写入注入：把 `versioning` 里的权威换成裸拼接，删除必须当场越界

        这一支证明那一处调用点**真的在读这把权威**，而不是只在注释里提了它。
        """
        import augmentor.versioning as versioning

        root = tmp_path / "versions"
        root.mkdir()
        victim = tmp_path / "victim"
        victim.mkdir()
        mgr = VersionManager(storage_dir=str(root))
        monkeypatch.setattr(
            versioning, "require_within_dir", lambda label, value, base: Path(base) / value
        )
        mgr.delete_version("../victim")
        assert not victim.exists(), "注入之后仍然删不掉 ⇒ 判据是恒真的，第 3 组没有牙齿"


class TestTheResolvedEscapeArm:
    """`..` 与绝对路径之外的第三档：**规范化之后**才露出来的逃逸（符号链接/目录接点）

    这一档按能力探测：Windows 上目录接点（junction）不需要开发者权限，POSIX 上用符号链接；
    两者都造不出来时**如实 skip**，而不是把判据写成恒真。
    """

    def _make_escape_link(self, base: Path, link_name: str, target: Path) -> bool:
        link = base / link_name
        target.mkdir(parents=True, exist_ok=True)
        (target / "data.json").write_text("[]", encoding="utf-8")
        try:
            link.symlink_to(target, target_is_directory=True)
            return True
        except OSError:
            pass
        rc = subprocess.call(
            ["cmd", "/c", "mklink", "/J", str(link), str(target)],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        return rc == 0 and link.exists()

    def test_a_link_out_of_the_root_is_rejected_after_resolution(self, tmp_path):
        base = tmp_path / "versions"
        base.mkdir()
        if not self._make_escape_link(base, "out", tmp_path / "elsewhere"):
            pytest.skip("本机造不出目录链接（无符号链接权限且 mklink /J 不可用）")
        with pytest.raises(DataValidationError):
            require_within_dir("版本 ID", "out/x", base)
