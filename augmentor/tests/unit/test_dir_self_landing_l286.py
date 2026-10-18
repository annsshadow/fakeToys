"""L286 格一：`require_within_dir` 缺「落点就是基座目录本身」那一档

改前的实测形状（探针在仓外 Temp 抓的读数，基座取 `<tmp>/versions`）：

- 名字 `.`、`./`、纯空白包裹的 `  .  `、Windows 盘符相对的 `C:` 规范化之后**落点等于基座本身**，
  而 `path_lands_within` 判的是形状、「相等」也算落在之内 ⇒ 权威一路放行。
- `VersionManager.delete_version(".")` 于是走到 `shutil.rmtree(整个版本根)`：全部版本目录 +
  `history.jsonl` + `current.txt` 一次消失（改前读数逐字 `root after delete_version('.') -> False / WIPED`）。
- 同一枚形状**从 HTTP 可达**：`DELETE /api/versions/%2E`（以及双重编码的 `%252E`）把 `'.'` 原样
  送进路径参数 ⇒ 这不是「只有 SDK 直接调用才会自伤」那一档，与 `notes%2Fdeep` 拿到 404 是两种命运。

处置：把这一档下推进**权威本身**（判据只有一处，所有调用点一起获得），既不新增第二个出生点，
也不改动 `path_lands_within` 的形状口径。判据写成两次 `path_lands_within` 而不是 `resolved == root`：
`resolve()` 的扩展长度前缀形状在两次调用之间可以不一致（L273 现量到的那一格），直接比 `==` 会
**漏判**而不是误判，归一交给权威自己做。

尺子面（四组，逐组都有牙齿）：

1. 拒绝清单逐档点名（平台相关的档单独标注），文案只含名字与目录标签、不含解析后的绝对落点。
2. 接受清单钉住「这把尺子量的是路径组件，不是字符串长得危不危险」——`a/.` 落点是 `a`，照旧放行。
3. 盘面事实：`delete_version(".")` 之后版本根与它的全部内容仍然在场，而合法删除一格没被误伤。
4. HTTP 边界与牙齿：`%2E` 到得了处理函数、判决是 400；把权威换成改前那一档的忠实复刻，
   rmtree 必须当场重现（复刻体自己也得拦住 `../`，否则这一支是在打稻草人）。
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import pytest

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from augmentor.exceptions import DataValidationError  # noqa: E402
from augmentor.validation import path_lands_within, require_within_dir  # noqa: E402
from augmentor.version_control import DatasetVersionManager  # noqa: E402
from augmentor.versioning import VersionManager  # noqa: E402

IS_WIN = sys.platform.startswith("win")

#: 落点就是基座本身的那些形状（`str.strip()` 之后的 `.` 与盘符相对前缀都属这一档）。
SELF_LANDING_SHAPES = [".", "./", "  .  ", ".//"]
if IS_WIN:
    SELF_LANDING_SHAPES.append("C:")

#: 照旧放行的形状 —— 判据量的是「落点是不是基座」，不是「名字里有没有点」。
STILL_ACCEPTED = ["v_20261013_120000", "sub/dir_ok", "plain.json", "a/.", ".._checkpoint.json"]


def _mk_root(tmp_path: Path) -> Path:
    root = tmp_path / "versions"
    root.mkdir()
    (root / "history.jsonl").write_text(
        json.dumps({"version_id": "v_1", "action": "create"}) + "\n", encoding="utf-8"
    )
    (root / "current.txt").write_text("v_1", encoding="utf-8")
    for name in ("v_1", "v_2"):
        d = root / name
        d.mkdir()
        (d / "data.json").write_text(json.dumps([{"instruction": name}]), encoding="utf-8")
    return root


def _contents(root: Path) -> list[str]:
    return sorted(p.name for p in root.iterdir())


class TestSelfLandingIsRejected:
    def test_every_self_landing_shape_is_rejected_by_name(self, tmp_path):
        base = _mk_root(tmp_path)
        for shape in SELF_LANDING_SHAPES:
            with pytest.raises(DataValidationError) as exc:
                require_within_dir("版本 ID", shape, base)
            assert "它本身" in str(exc.value), (shape, str(exc.value))
            assert "版本 ID" in str(exc.value), shape

    def test_the_escaped_shapes_keep_their_original_verdict(self, tmp_path):
        """`..` 那一支仍走原来的「之外」判决 —— 本轮没有把它折进新档、改一句文案"""
        base = _mk_root(tmp_path)
        for shape in ("..", "../x", "a/../../b"):
            with pytest.raises(DataValidationError) as exc:
                require_within_dir("版本 ID", shape, base)
            assert "之外" in str(exc.value), (shape, str(exc.value))

    def test_the_verdict_carries_no_resolved_absolute_landing(self, tmp_path):
        base = _mk_root(tmp_path)
        with pytest.raises(DataValidationError) as exc:
            require_within_dir("版本 ID", ".", base)
        verdict = str(exc.value)
        assert str(base.resolve()) not in verdict, verdict
        assert re.search(r"[A-Za-z]:[\\/]", verdict) is None, verdict

    @pytest.mark.parametrize("shape", STILL_ACCEPTED)
    def test_ordinary_names_still_land_strictly_inside(self, tmp_path, shape):
        base = _mk_root(tmp_path)
        landing = require_within_dir("版本 ID", shape, base)
        assert landing != base
        assert path_lands_within(base.resolve(), landing)


class TestTheVersionsRootSurvives:
    def test_delete_of_the_directory_itself_raises_and_wipes_nothing(self, tmp_path):
        root = _mk_root(tmp_path)
        before = _contents(root)
        mgr = VersionManager(storage_dir=str(root))
        with pytest.raises(DataValidationError):
            mgr.delete_version(".")
        assert root.exists(), "版本根被 rmtree 掉了（改前的实测形状）"
        assert _contents(root) == before, "删除被拒却动盘了"
        assert (root / "v_1" / "data.json").exists()

    def test_a_legitimate_delete_still_deletes(self, tmp_path):
        """新档不许顺手把合法删除一起拦掉 —— 这一支钉的是判据宽度，不是深度"""
        root = _mk_root(tmp_path)
        mgr = VersionManager(storage_dir=str(root))
        (root / "current.txt").write_text("v_2", encoding="utf-8")
        mgr.delete_version("v_1")
        assert not (root / "v_1").exists(), "合法版本删不掉 ⇒ 新档收得太宽"
        assert (root / "v_2").exists() and (root / "history.jsonl").exists()

    @pytest.mark.parametrize("face", ["load_version", "get_version_info"])
    def test_the_read_faces_reject_the_same_shape(self, tmp_path, face):
        root = _mk_root(tmp_path)
        mgr = VersionManager(storage_dir=str(root))
        with pytest.raises(DataValidationError):
            getattr(mgr, face)(".")

    def test_the_other_version_manager_inherits_the_same_authority(self, tmp_path):
        """`version_control.DatasetVersionManager` 也传裸 version_id ⇒ 同一处判据，不另写一套

        实测它的 `delete_version(".")` 在未登记时**短路回 `False`**，压根走不到权威（与 L258
        给这一处记的同一句归因：那把权威是纵深防御，不是当前可达面）⇒ 判据按实测写，
        只有读面 `load_version` 当场 raise。
        """
        root = _mk_root(tmp_path)
        before = _contents(root)
        mgr = DatasetVersionManager(versions_dir=str(root))
        with pytest.raises(DataValidationError):
            mgr.load_version(".")
        assert mgr.delete_version(".") is False
        assert root.exists()
        after = set(_contents(root))
        assert set(before) <= after, after
        # 读面在走到权威之前先落了一只 `index.json`（这一族的既有副作用，本轮没有动它）
        assert after - set(before) <= {"index.json"}, after


class TestTheHttpBoundary:
    @pytest.fixture()
    def client(self, tmp_path, monkeypatch):
        """把处理函数手里的版本根指向仓外临时目录 —— 牙齿臂会真的删盘，不许打在仓库上"""
        from types import SimpleNamespace

        from fastapi.testclient import TestClient

        import api.routes.version as version_route
        from api.main import app

        root = _mk_root(tmp_path)
        mgr = VersionManager(storage_dir=str(root))
        monkeypatch.setattr(
            version_route, "get_pipeline", lambda: SimpleNamespace(version_manager=mgr)
        )
        return SimpleNamespace(
            http=TestClient(app, raise_server_exceptions=False), root=root, mgr=mgr
        )

    @pytest.mark.parametrize("encoded", ["%2E", "%2e", "%252E"])
    def test_the_self_landing_id_is_a_four_hundred_and_costs_nothing(self, client, encoded):
        http, root = client.http, client.root
        before = _contents(root)
        resp = http.delete(f"/api/versions/{encoded}")
        assert resp.status_code == 400, (encoded, resp.status_code, resp.text)
        assert "它本身" in resp.json()["detail"], resp.text
        assert root.exists(), "HTTP 面把整个版本根删掉了（改前的实测形状）"
        assert _contents(root) == before

    def test_the_double_encoded_form_reaches_the_handler_as_a_dot(self, client, monkeypatch):
        """`%252E` 不是「拿到一串 `%252E` 字面量」：实测解码后就是 `'.'`，因而必须同判

        这一支钉的是可达性前提本身 —— 若哪天 Starlette 不再二次解码，上面那一组的
        `%252E` 档会跟着失效，得由这一支先出声。记账点设在 `version_manager.delete_version`
        那一格上：路由在装饰期就把函数对象交给了 FastAPI，换模块属性不会改变已注册的处理器。
        """
        seen: list[str] = []
        real = client.mgr.delete_version

        def recording(version_id: str):
            seen.append(version_id)
            return real(version_id)

        monkeypatch.setattr(client.mgr, "delete_version", recording)
        client.http.delete("/api/versions/%252E")
        assert seen == ["."], seen

    def test_a_dot_that_decodes_into_a_traversal_keeps_the_original_verdict(self, client):
        """`.%2E` 解码成 `..` ⇒ 走「之外」那一档，本轮没有把两档文案混成一句"""
        http, root = client.http, client.root
        resp = http.delete("/api/versions/.%2E")
        assert resp.status_code == 400, resp.text
        assert "之外" in resp.json()["detail"], resp.text
        assert root.exists()

    def test_a_ghost_id_still_reports_success_on_delete(self, client):
        """本轮只动自指落点这一档，没顺手改「未登记 id」那一族的回执（实测仍是 200 / success）"""
        http, root = client.http, client.root
        resp = http.delete("/api/versions/v_ghost")
        assert resp.status_code == 200, (resp.status_code, resp.text)
        assert resp.json() == {"success": True}
        assert root.exists()

    def test_the_detail_leaks_no_deploy_path(self, client):
        verdict = client.http.delete("/api/versions/%2E").json()["detail"]
        assert str(REPO) not in verdict, verdict


class TestTheRulerHasTeeth:
    """把权威换成改前那一档的**忠实复刻**，盘面事实必须当场重现 —— 否则上面三组是恒真"""

    @staticmethod
    def _pre_fix_require_within_dir(label: str, value, base: Path) -> Path:
        """改前的判据：只问「落点是否在基座之内」，相等也算通过（`.parents` 那一档原样保留）"""
        candidate = Path(str(value).strip())
        if candidate.is_absolute() or ".." in candidate.parts:
            raise DataValidationError(f"{label} 不能指向 {base.name} 目录之外")
        root = Path(base).resolve()
        resolved = (root / candidate).resolve()
        if not path_lands_within(root, resolved):
            raise DataValidationError(f"{label} 不能指向 {base.name} 目录之外")
        return resolved

    def test_the_replica_is_faithful_not_a_straw_man(self, tmp_path):
        base = _mk_root(tmp_path)
        for shape in ("../victim", "/abs/path", "a/../../b"):
            with pytest.raises(DataValidationError):
                self._pre_fix_require_within_dir("版本 ID", shape, base)
        # 而 `.` 在它手里是**放行**的 —— 那正是本轮要堵的那一档
        assert self._pre_fix_require_within_dir("版本 ID", ".", base) == base.resolve()

    def test_with_the_replica_installed_the_root_is_wiped(self, tmp_path, monkeypatch):
        import augmentor.versioning as versioning

        root = _mk_root(tmp_path)
        mgr = VersionManager(storage_dir=str(root))
        monkeypatch.setattr(
            versioning, "require_within_dir", self._pre_fix_require_within_dir
        )
        mgr.delete_version(".")
        assert not root.exists(), (
            "注入之后仍然删不掉 ⇒ 第 3 组的盘面事实是恒真的，那把尺子没有牙齿"
        )

    def test_the_tier_is_written_once_in_the_authority_not_hand_copied_at_a_callsite(self):
        """判据必须留在权威里，且写成两次 `path_lands_within` 而不是直接比 `==`

        读的是 AST 打出来的**代码**，不是源码文本 —— 权威自己的 docstring 里就写着「不是
        `resolved == root`」这句话，按文本比会被自己的说明文判红（同一形状的坑本账记过多次）。
        """
        import ast
        import inspect

        from augmentor.validation import require_within_dir

        tree = ast.parse(inspect.getsource(require_within_dir))
        code = "\n".join(
            ast.unparse(node)
            for node in tree.body[0].body
            if not (isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant))
        )
        assert code.count("path_lands_within(") == 2, code
        assert " == root" not in code, code
        assert " == base" not in code, code
