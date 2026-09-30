# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L89 / A152①：一次「什么都没写成的」 `POST /api/config` 不许在磁盘上留下任何痕迹

改前的形状（L87 写 A151 的用例时实测到，本轮才修）：路由末尾**无条件** `save_config`，
而 `save_config` 走「读现有文件 → 合并 → `yaml.safe_dump` 整份重写」。于是连
`json={}` 这种一个键都没提交的请求也会把出厂那份 `config.yaml` 重抹一遍。本轮把
差异**逐项对账**过（`Temp/l89q/roundtrip_diff.py` 的形状：产品那侧是就地重写已存在的
文件，不是写去一个不存在的路径 —— 后者会额外丢 `app` 节与 `${ENV}` 占位符，那是
"没有原文件可抄"而不是缺陷，一开始我把它读成了缺陷）：

- 注释行 **202 → 0**（出厂那 202 行里住的不是文档，是判据本身：自动保存档「10 倍
  静默写放大」的实测、`request_timeout` 取 120 而不是 60 的两侧代价、
  `max_upload_bytes` 默认 256 MiB 的定标依据）；
- 既有叶子键 **一个不丢、一个值不变** —— 所以「差异全是排版」这句是**成立的**；
- 但会**多出 7 个 `null` 键**（`models.<名字>.base_url` / `.request_timeout`，
  dataclass 的 `None` 字段被 `asdict` 具象化）⇒ A152 原文那句「差异 100 % 是注释与
  排版」在这里要修成「注释 + 排版 + 七个空值键」。

一次空写入不该付这笔账。三条口径写在这里，因为它们各对应一条用例的形状：

1. **判「写没写」而不是「有没有报错」**：`applied` 只看有没有真的 `setattr` 过，
   所以拼错的键（`variants_per_item`）、本端点不写的节（`web`）、空请求三种都算空写入；
   越界值走的是异常出口（400），本来就到不了落盘那一行。
2. **`success` 不借来表态**：空写入仍然 200 + `success: true`（请求被正常处理），
   出声的是 `message` 的主干与 `ignored_keys`。把 `success` 改成 false 会让既有
   客户端把「你的请求合法但我没动手」读成「服务端出错」，那是另一格契约。
3. **反证必须存在**：光测「空写入不落盘」不够 —— 把落盘整个关掉也能让它绿。所以同
   一份取证房里必须再测一次「真写入照样落盘、照样作废缓存、注释照样被抹掉」，两条
   一起才把「只在有写入时落盘」这个**条件**钉住。

顺带说清 L88 那四条写面用例为什么本轮改了载荷：它们 POST 的 `variants_per_item`
**不是 `AugmentationConfig` 的字段**（真名 `variants_per_seed`），从前能绿正是因为
这里要修的无条件落盘 —— 详见那个文件的模块 docstring。
"""

import sys
from pathlib import Path

import pytest

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from augmentor.config import load_config  # noqa: E402

SHIPPED_YAML = REPO / "config.yaml"

#: 一个**不存在**的 augmentation 子键：`apply_section_update` 会把它退回 `ignored`，
#: 于是这条请求合法、200、点名出声，但什么都没写进配置对象（A152① 的空写入形状）。
#: 它同时是 L88 那四条用例从前误用的载荷 —— 留着当回归锚点。
TYPO_KEY = "variants_per_item"
#: 真键与合法新值：反向护栏用（「有写入 ⇒ 必须落盘」）。
REAL_KEY = "variants_per_seed"
REAL_VALUE = 3


@pytest.fixture(autouse=True)
def _clean_content_caches():
    """内容缓存前后各清：它们是模块级字典，跨用例带读数会互相骗"""
    import api.deps as deps

    deps._config_roots_cache.clear()
    deps._config_upload_limit_cache.clear()
    yield
    deps._config_roots_cache.clear()
    deps._config_upload_limit_cache.clear()


@pytest.fixture
def room(tmp_path, monkeypatch):
    """取证房：临时目录里放一份**逐字节**的出厂配置 + 一个真 `AppConfig` 的 pipeline stub

    与 L87/L88 同一套护栏。返回 `(http, path, config, pristine)`：`path`/`pristine`
    用来断言磁盘字节与 mtime，`config` 用来断言内存对象没被碰。
    """
    monkeypatch.chdir(tmp_path)
    pristine = SHIPPED_YAML.read_bytes()
    path = tmp_path / "config.yaml"
    path.write_bytes(pristine)
    import api.deps as deps
    from fastapi.testclient import TestClient
    from api.main import app

    instance = type("PipelineStub", (), {})()
    instance.config = load_config(str(SHIPPED_YAML))
    monkeypatch.setattr(deps, "_pipeline", instance)
    monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
    return TestClient(app), path, instance.config, pristine


def _comment_lines(raw: bytes) -> int:
    return sum(1 for ln in raw.decode("utf-8").splitlines() if ln.lstrip().startswith("#"))


class TestNoOpWritesLeaveNoTrace:
    """A152① 本体：三种「什么都没写成」的请求，磁盘字节 / mtime / 内存三处都不许动"""

    @pytest.mark.parametrize("payload,why", [
        ({}, "空请求"),
        ({"augmentation": {TYPO_KEY: 3}}, "节内键名拼错（L88 从前就这个形状）"),
        ({"web": {"max_upload_bytes": 4096}}, "整节存在但本端点不写（A151 点名那一格）"),
        ({"augmenation": {"threshold": 1}}, "节名拼错"),
    ])
    def test_bytes_and_mtime_untouched(self, room, payload, why):
        http, path, config, pristine = room
        before_ns = path.stat().st_mtime_ns

        response = http.post("/api/config", json=payload)

        assert response.status_code == 200, why
        assert path.read_bytes() == pristine, f"{why}：磁盘字节被动过"
        assert path.stat().st_mtime_ns == before_ns, f"{why}：连 mtime 都不该动"
        assert config.augmentation.variants_per_seed == 5, f"{why}：内存配置被写了"

    def test_noop_still_reports_ignored_keys_and_success(self, room):
        """不出声就是假话：改的是「动不动磁盘」，不是「要不要报告」"""
        http, _, _, _ = room
        response = http.post("/api/config", json={"augmentation": {TYPO_KEY: 3}})
        payload = response.json()
        assert payload["success"] is True
        assert payload["ignored_keys"] == [f"augmentation.{TYPO_KEY}"]
        assert "未写入任何配置项" in payload["message"]
        assert "已忽略未写入的配置项" in payload["message"]

    def test_empty_request_is_honest_in_message(self, room):
        """`json={}` 没有可点名的键 ⇒ 主干必须自己说明「没动手」，不能回「已保存」"""
        http, _, _, _ = room
        payload = http.post("/api/config", json={}).json()
        assert payload["ignored_keys"] == []
        assert "配置已保存" not in payload["message"]
        assert "未写入任何配置项" in payload["message"]


class TestTheShippedCommentsSurvive:
    """把「注释是判据的一部分」从推理变成断言（并给反向护栏留一根把手）"""

    def test_pristine_file_actually_has_comments_to_lose(self, room):
        """这条是判据的**把手**：出厂那份如果没有注释，下面两条就是空的"""
        _, _, _, pristine = room
        assert _comment_lines(pristine) > 100, "取证前提不成立：无注释可保护"

    def test_noop_keeps_every_comment(self, room):
        http, path, _, pristine = room
        http.post("/api/config", json={"web": {"port": 9000}})
        assert _comment_lines(path.read_bytes()) == _comment_lines(pristine)

    def test_real_write_still_drops_them_and_persists(self, room):
        """反向护栏：真写入照旧落盘（连「注释被 `safe_dump` 抹掉」也照旧）

        没有这一条，「空写入不落盘」可以用「谁都不落盘」蒙过去。它同时钉住本轮
        没有把 `save_config` 的既有能力改窄：改的只是**触发条件**。
        """
        http, path, config, pristine = room
        assert getattr(config.augmentation, REAL_KEY) != REAL_VALUE

        payload = http.post("/api/config", json={"augmentation": {REAL_KEY: REAL_VALUE}})
        assert payload.status_code == 200, payload.text
        body = payload.json()
        assert body["ignored_keys"] == []
        assert "配置已保存" in body["message"]

        written = path.read_bytes()
        assert written != pristine
        assert _comment_lines(written) == 0, "`safe_dump` 整份重写不该还剩注释"
        import yaml
        on_disk = yaml.safe_load(written.decode("utf-8"))
        assert on_disk["augmentation"][REAL_KEY] == REAL_VALUE
        assert getattr(config.augmentation, REAL_KEY) == REAL_VALUE


class TestDefaultModelCountsAsAWrite:
    """`default_model` 是直接赋值，不经 `apply_section_update` ⇒ 口径要单独钉"""

    def test_default_model_alone_triggers_the_persist(self, room, monkeypatch):
        import augmentor.config as cfgmod

        http, _, config, _ = room
        calls = []
        real = cfgmod.save_config

        def record(*a, **k):
            calls.append(a)
            return real(*a, **k)

        monkeypatch.setattr(cfgmod, "save_config", record)
        response = http.post("/api/config", json={"default_model": config.default_model})
        assert response.status_code == 200
        assert len(calls) == 1, "值与默认相同也算写入：动了配置对象就该落盘"
        assert "配置已保存" in response.json()["message"]


class TestNoOpDoesNotDisturbTheCaches:
    """空写入连内容缓存也不该碰 —— 缓存的主人只承认「文件被写过」这一件事

    A152① 顺带把 A156 的边界收紧了：从前一次 `json={}` 也会 `invalidate_config_caches()`，
    于是任何只读探测（例如契约用例里那句 `POST {}`）都换来一次 7.34–8.13 ms 的重解析
    （L88 实测）。现在没有写入就没有作废，缓存里那条旧值继续放行 —— 这条断言在改前
    是红的，也正是它证明了「作废」这件事仍然挂在「写盘成功」之后，而不是挂在请求上。
    """

    def test_stale_entry_survives_a_noop_post(self, room):
        import api.deps as deps

        http, path, _, _ = room
        ghost = path.parent / "ghost_dir_that_never_exists"
        deps._config_roots_cache[(str(path), path.stat().st_mtime_ns)] = [ghost]
        assert deps.allowed_data_roots() == [ghost], "前提：读边在走缓存"

        assert http.post("/api/config", json={"web": {"port": 9000}}).status_code == 200

        assert deps._config_roots_cache, "空写入不该把缓存洗一遍"
        assert deps.allowed_data_roots() == [ghost]

    def test_a_real_write_still_invalidates(self, room):
        """同室的反向护栏：有写入 ⇒ 两张内容缓存一起清空（A156 本进程那一半）"""
        import api.deps as deps

        http, path, _, _ = room
        ghost = path.parent / "ghost_dir_that_never_exists"
        deps._config_roots_cache[(str(path), path.stat().st_mtime_ns)] = [ghost]
        assert deps.allowed_data_roots() == [ghost]

        assert http.post(
            "/api/config", json={"augmentation": {REAL_KEY: REAL_VALUE}}
        ).status_code == 200

        assert deps._config_roots_cache == {}
        assert deps._config_upload_limit_cache == {}
