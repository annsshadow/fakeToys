# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L87：`web.max_upload_bytes` 三面同判 + 上传读体一次都不越界 + 写面节级出声

改前取证（`Temp/l87q/upload_face_before.json`，python 3.14.4）：`POST /api/data/upload`
把 body 一次 `read()` 进内存、尺寸由客户端决定，形态判据一个字不判，异常出口只有一
个兜底 500。同一份仓内文件（8.4 MiB）实测让进程峰值比控制档高 34.8 MiB（≈4.1 倍），
而 `json.dumps("hello")` 是 **200 + 落盘 7 字节的 `"hello"` + 响应 `count: 5`**（把
字符数当条数）——上传端点写出一份读端点自己会拒绝的文件。

改后（`Temp/l87q/upload_face_after.json` + `Temp/l87q/cap_memory_l87.py` 四臂）：

- 上限生效：`cap − ctl = +0.0 MiB`（同一 64.1 MiB payload，`limit=1 MiB` 那一臂峰值
  与「什么都没读」的控制臂持平），对照改前 `old − ctl = +65.1 MiB` ⇒ 峰值不再由客户端决定
- 默认档无回归：`big − old = +0.0 MiB`、墙钟 `big/old = 1.043` ⇒ 加判据不改变合法路径
- 越界出声：413 `上传内容 67173427 字节，超过 web.max_upload_bytes=1048576（未读取、未落盘）`

**「四面」为什么在这一键上只有三面**：`POST /api/config` 根本不写 `web` 节。这是本轮
给新键配第四面时**当场撞出来的第二个缺陷**（A151）：请求体里不认识的顶层节被 pydantic
的默认 `extra="ignore"` 先吃掉，路由再只遍历那七节，实测 `{"web": {"max_upload_bytes":
0}}` 与 `{"web": {"max_upload_bytes": 4096}}` **都是 200 +「配置已保存」+
`ignored_keys: []`**，内存对象与磁盘上那一节一字未改。本轮修的是「静默 → 出声」
（`TestWriteFaceSectionFeedback`），不是「把 `web` 变成可写节」——后者牵动端口/白名单/
CORS 的生效时机，属于要单独拍的重启语义，记在 A151 的余下那一半。

**为什么是 O(1) 预检而不是分块攒**（本文件 `TestReadUploadBodyO1` 钉住的就是这条）：
第一版实现写的是分块 `bytearray` 累积，实测同一个 64.1 MiB 上传比一次 `read()`
多 64.0 MiB 峰值、慢 5.9 倍（扩容要留旧缓冲区）——那是「在默认路径上造一个新的越界」。
改判的根据是 starlette 1.2.1 的 `UploadFile.write()` 里有 `self.size += len(data)`：
解析完的 `size` 是服务端按实际写入字节累加出来的，与客户端报的 Content-Length 无关，
所以「上限」可以在读之前一次判掉。

正对照的教训记在账本 L87 回看段：`ctl_none` 那一臂必须是「一次字节都不读」，
`hold-both` 那一臂必须多**新分配**一份全尺寸拷贝，否则尺子按构造就是瞎的。
"""

import asyncio
import dataclasses
import json
import os
import pathlib

import pytest
import yaml

from api.deps import (assert_dataset_shape, _config_upload_limit_cache,
                      max_upload_bytes, read_items)
from api.routes.data import _read_upload_body
from augmentor.config import (MAX_UPLOAD_BYTES_MIN, WebConfig, load_config)
from augmentor.config_validator import ConfigValidator
from augmentor.exceptions import DataValidationError
from fastapi import HTTPException

AI_DIR = pathlib.Path(__file__).resolve().parent.parent.parent
SHIPPED_YAML = AI_DIR / "config.yaml"

DEFAULT = WebConfig().max_upload_bytes

# (键值, 一句话原因)。坏值四面同红，缺任何一面就是 A118 的复发。
BAD_VALUES = [
    (0, "「0」有两种人话读法（不限 / 全拒），后果相反，不能靠读法决定"),
    (-1, "负数比较恒成立 ⇒ 每个上传都被拒，看起来像上传功能坏了"),
    (True, "`bool` 是 `int` 的子类，不显式判就当 1 字节收下"),
    ("1024", "YAML 加了引号的数字，静默不参与整数比较"),
    (None, "空值写成 None 会被 `Path(str(p))` 那一类回落吃掉"),
    (1.5, "浮点字节数：`require_count` 只收整数，半字节不是长度"),
]

# (键值, 一句话原因)。收紧的代价记在这条轴上。
GOOD_VALUES = [
    (MAX_UPLOAD_BYTES_MIN, "闭区间下界"),
    (1024, "小写上限，运维想卡多紧都可以"),
    (DEFAULT, "出厂档"),
    (10 ** 12, "比默认还大是操作者自己的选择：本键只判下界，不造本地天花板"),
]


def _defaults():
    return {f.name: getattr(WebConfig(), f.name) for f in dataclasses.fields(WebConfig())}


def _construct(value):
    return WebConfig(**dict(_defaults(), max_upload_bytes=value))


def _section_errors(result):
    return [e for e in result.errors
            if e.path == "web" or e.path.startswith("web.")]


def _write_yaml(tmp_path, body, name="one.yaml"):
    cfg = tmp_path / name
    cfg.write_text(yaml.safe_dump({"web": body}, allow_unicode=True),
                   encoding="utf-8")
    return str(cfg)


@pytest.fixture
def client(tmp_path, monkeypatch):
    """隔离到临时目录的 TestClient：`POST /api/config` 成功即 `save_config`

    与 L82 那间取证房同一套护栏：`chdir` 到 `tmp_path`，并把仓库那份 `config.yaml`
    逐字节复制过去，这样写面的判决与产品同源，而失败路径可以断言磁盘字节没动。
    """
    monkeypatch.chdir(tmp_path)
    pristine = SHIPPED_YAML.read_bytes()
    (tmp_path / "config.yaml").write_bytes(pristine)
    import api.deps as deps
    from fastapi.testclient import TestClient
    from api.main import app
    instance = type("PipelineStub", (), {})()
    instance.config = load_config(str(SHIPPED_YAML))
    monkeypatch.setattr(deps, "_pipeline", instance)
    return TestClient(app), instance, pristine


class TestBadValueFaces:
    """坏值：三面判红（SDK 直构 / `load_config` / `validate-config`），第四面出声不写

    第四面（`POST /api/config`）本来就不写 `web` 节，它欠的是**出声**（A151），
    不是判负。判负那三面少任何一面就是 A118 的复发。
    """

    @pytest.mark.parametrize("value,why", BAD_VALUES)
    def test_runtime_construct_raises(self, value, why):
        with pytest.raises(DataValidationError, match="web.max_upload_bytes"):
            _construct(value)

    @pytest.mark.parametrize("value,why", BAD_VALUES)
    def test_load_config_raises_at_load_time(self, tmp_path, value, why):
        path = _write_yaml(tmp_path, {"max_upload_bytes": value})
        with pytest.raises(DataValidationError, match="web.max_upload_bytes"):
            load_config(path)

    @pytest.mark.parametrize("value,why", BAD_VALUES)
    def test_static_face_reports_exactly_one(self, value, why):
        result = ConfigValidator().validate_config(
            {"web": {"max_upload_bytes": value}})
        errors = _section_errors(result)
        assert errors, "web.max_upload_bytes=%r（%s）在 validate-config 上零反馈" % (
            value, why)
        assert errors[0].severity.value == "error"
        # 既有口径：位置由 `path` 承担，文案只写值与界（L82 同族，不再重复点名键）
        assert errors[0].path == "web.max_upload_bytes"
        assert len(errors) == 1, "同一件事不许报两遍"

    @pytest.mark.parametrize("value,why", BAD_VALUES)
    def test_api_write_face_names_the_dropped_section(self, client, value, why):
        """A151：`web` 节不在 `POST /api/config` 的写入清单里，改前连出声都没有

        这一条**不是**「写面也判了这个坏值」——写面根本不写这一节。判负由其余三面
        负责（直构 / `load_config` / `validate-config`）。本用例钉的是 A151 补上的
        那一半：不写可以，但必须点名，不能回「配置已保存」而什么都不说。
        """
        import os
        http, instance, _ = client
        response = http.post("/api/config",
                             json={"web": {"max_upload_bytes": value}})
        assert response.status_code == 200
        payload = response.json()
        assert "web" in payload["ignored_keys"]
        assert "不修改该节" in payload["message"]
        assert instance.config.web.max_upload_bytes == DEFAULT
        # 磁盘侧只断言「这一节没被写脏」：路由末尾无条件 `save_config`，整份文件的
        # 字节本来就会变（注释被重序列化吃掉那一笔另立 A152，不在本轮修）。
        on_disk = yaml.safe_load(
            (pathlib.Path(os.getcwd()) / "config.yaml").read_text(encoding="utf-8"))
        assert on_disk["web"]["max_upload_bytes"] == DEFAULT


class TestLegalValuesStayLegal:
    """反向护栏：四面同判不等于四面全判负"""

    @pytest.mark.parametrize("value,why", GOOD_VALUES)
    def test_runtime_construct(self, value, why):
        assert _construct(value).max_upload_bytes == value

    @pytest.mark.parametrize("value,why", GOOD_VALUES)
    def test_static_face_is_quiet(self, value, why):
        result = ConfigValidator().validate_config(
            {"web": {"max_upload_bytes": value}})
        assert _section_errors(result) == []

    @pytest.mark.parametrize("value,why", GOOD_VALUES)
    def test_api_write_face_still_does_not_write_the_section(self, client, value, why):
        """合法档同样不落地 —— A151 的出声不是「坏值才报」，是整个 `web` 节都不写

        这一档刻意与 BAD 值同形：如果只有坏值点名，那就是「判负才出声」，而真实
        症状（用户改了合法值、看到「配置已保存」、重启才发现没写）两边都会踩。
        """
        http, instance, _ = client
        response = http.post("/api/config",
                             json={"web": {"max_upload_bytes": value}})
        assert response.status_code == 200
        assert "web" in response.json()["ignored_keys"]
        assert instance.config.web.max_upload_bytes == DEFAULT


class TestWriteFaceSectionFeedback:
    """A151 本体：`POST /api/config` 对**节级**丢弃出声（A86 的同构另一半）"""

    def test_known_but_unwritable_section_is_named_without_a_guess(self, client):
        http, _, _ = client
        response = http.post("/api/config", json={"logging": {"level": "WARNING"}})
        payload = response.json()
        assert payload["ignored_keys"] == ["logging"]
        assert "本端点不修改该节" in payload["message"]
        # 节名是对的，不许编造「是否想写 X」——那是给拼错用的
        assert "是否想写" not in payload["message"]

    @pytest.mark.parametrize("typo,suggest", [
        ("augmenation", "augmentation"),
        ("qualityy", "quality"),
    ])
    def test_misspelled_section_gets_a_suggestion(self, client, typo, suggest):
        http, _, _ = client
        response = http.post("/api/config", json={typo: {"x": 1}})
        payload = response.json()
        assert payload["ignored_keys"] == [typo]
        assert "是否想写 %s？" % suggest in payload["message"]

    def test_writable_and_dropped_keys_are_reported_together(self, client):
        """两种形状共存于同一份清单：`section.key`（节内）与 `section`（节级）"""
        http, _, _ = client
        response = http.post("/api/config",
                             json={"quality": {"no_such_key": 1}, "web": {"port": 9000}})
        assert response.json()["ignored_keys"] == ["quality.no_such_key", "web"]

    def test_clean_request_is_still_quiet(self, client):
        """反向护栏：不许多报噪声（A86 同族，`已忽略` 一支不许在无丢弃时出现）"""
        http, _, _ = client
        response = http.post("/api/config", json={"quality": {"enabled": False}})
        payload = response.json()
        assert payload["ignored_keys"] == []
        assert "已忽略" not in payload["message"]

    def test_writable_list_matches_the_request_schema(self):
        """`WRITABLE_SECTIONS` 与 `ConfigUpdateRequest` 的七个 dict 字段一一对应

        两份名单分开维护就是 A123 那一类漂移的现形方式：加了字段的节仍然被静默丢弃。
        """
        from api.routes.config import ConfigUpdateRequest, WRITABLE_SECTIONS

        dict_fields = {name for name, field in
                       ConfigUpdateRequest.model_fields.items()
                       if "dict" in str(field.annotation)}
        assert set(WRITABLE_SECTIONS) == dict_fields
        assert "default_model" not in WRITABLE_SECTIONS


class TestTheRangeIsOneCopyOnly:
    """下界常数只能有一处产地，否则「配了不生效」与「两边各判一套」迟早复发"""

    def test_validator_spec_borrows_the_runtime_constant(self):
        spec = ConfigValidator.KNOWN_FIELDS["web.max_upload_bytes"]
        assert spec["min"] == MAX_UPLOAD_BYTES_MIN
        assert spec["type"] is int

    def test_shipped_yaml_is_accepted_by_both_faces(self):
        body = yaml.safe_load(SHIPPED_YAML.read_text(encoding="utf-8"))
        value = body["web"]["max_upload_bytes"]
        assert isinstance(value, int) and not isinstance(value, bool)
        assert value >= MAX_UPLOAD_BYTES_MIN
        # 模板与 dataclass 默认不许各写一套数（A123 那类漂移在 config.yaml 上的版本）
        assert value == DEFAULT
        assert _construct(value).max_upload_bytes == value
        assert _section_errors(ConfigValidator().validate_config(body)) == []

    def test_shipped_yaml_explains_the_number(self):
        """活文档口径：这条键是资源上限，写在哪、为什么是这个数必须能被读到"""
        text = SHIPPED_YAML.read_text(encoding="utf-8")
        assert "max_upload_bytes" in text
        assert "MiB" in text or "字节" in text


class TestReadUploadBodyO1:
    """`_read_upload_body` 的两道判据：先判申报尺寸，且越界时**一次都不读**

    第一版（分块攒）在这条轴上是错的：它默认路径多 64.0 MiB / 慢 5.9 倍。
    所以这里钉的不是「能不能拒绝」，而是「拒绝之前有没有已经开始搬字节」。
    """

    class _Part:
        """鸭型 UploadFile：只用到 `.size` 与 `.read()`"""

        def __init__(self, size, payload):
            self.size = size
            self.payload = payload
            self.reads = 0

        async def read(self, *args, **kwargs):
            self.reads += 1
            return self.payload

    def test_over_declared_size_never_calls_read(self):
        part = self._Part(4096, b"MARKER" + b"y" * 4090)
        with pytest.raises(HTTPException) as exc:
            asyncio.run(_read_upload_body(part, 1024))
        assert exc.value.status_code == 413
        assert part.reads == 0, "越界档一次都不该读：读了就等于把上限交给内存"
        detail = exc.value.detail
        assert "未读取" in detail and "1024" in detail and "4096" in detail
        assert "MARKER" not in detail, "不许回显上传内容"

    def test_declared_exactly_at_limit_is_read(self):
        part = self._Part(1024, b"x" * 1024)
        content = asyncio.run(_read_upload_body(part, 1024))
        assert len(content) == 1024 and part.reads == 1

    def test_size_none_falls_back_to_post_read_check(self):
        """不走解析器的构造方给不出 `size`，第二道判据负责形状（读完了才知道多大）"""
        part = self._Part(None, b"x" * 2048)
        with pytest.raises(HTTPException) as exc:
            asyncio.run(_read_upload_body(part, 1024))
        assert exc.value.status_code == 413
        assert part.reads == 1
        assert "未落盘" in exc.value.detail and "未读取" not in exc.value.detail

    def test_under_limit_returns_the_bytes(self):
        part = self._Part(8, b"[]")
        assert asyncio.run(_read_upload_body(part, 1024)) == b"[]"


class TestUploadLimitSource:
    """`max_upload_bytes()`：配置 → 出厂默认，缓存认 mtime，降级不冻进缓存

    缓存是 monkeypatchable 的模块级字典，测试前手动清空，别把上一用例的读数带来。
    """

    @pytest.fixture(autouse=True)
    def clear_cache(self):
        _config_upload_limit_cache.clear()
        yield
        _config_upload_limit_cache.clear()

    def test_reads_the_number_from_config(self, tmp_path, monkeypatch):
        path = tmp_path / "config.yaml"
        path.write_text(yaml.safe_dump({"web": {"max_upload_bytes": 4096}}),
                        encoding="utf-8")
        import api.deps as deps
        monkeypatch.setattr(deps, "config_file_path", lambda: path)
        assert max_upload_bytes() == 4096

    def test_caches_by_mtime_and_invalidates_on_rewrite(self, tmp_path, monkeypatch):
        path = tmp_path / "config.yaml"
        path.write_text(yaml.safe_dump({"web": {"max_upload_bytes": 4096}}),
                        encoding="utf-8")
        import api.deps as deps
        monkeypatch.setattr(deps, "config_file_path", lambda: path)
        assert max_upload_bytes() == 4096
        assert _config_upload_limit_cache, "成功读数应当进缓存"
        calls = []
        real_load = deps.load_config

        def counting(*args, **kwargs):
            calls.append(1)
            return real_load(*args, **kwargs)

        monkeypatch.setattr(deps, "load_config", counting)
        assert max_upload_bytes() == 4096
        assert calls == [], "同一 mtime 命中缓存，不该重解析 YAML"
        path.write_text(yaml.safe_dump({"web": {"max_upload_bytes": 8192}}),
                        encoding="utf-8")
        # 把第二次写入的 mtime **显式往前推一秒**：这台机器的 mtime tick 只有 0.5 ms，
        # 连续两次写落在同一个 tick 的概率现量 61–65%（`Temp/l87q/mtime_tick_l87.py`，
        # 300 轮 × 两个解释器）⇒ 「改了文件就该失效」这条判据不许交给操作系统时钟，
        # 否则这条用例每跑一次掷一次骰子（本轮真掷红过一次，读数见账本 L87）。
        st = path.stat()
        os.utime(path, ns=(st.st_atime_ns, st.st_mtime_ns + 10 ** 9))
        assert max_upload_bytes() == 8192
        assert len(calls) == 1

    def test_identical_mtime_ns_is_the_known_staleness_window(self, tmp_path, monkeypatch):
        """把「mtime 没动 ⇒ 缓存不失效」钉成**已知边界**，不是 endorsing：A156 立案的就是这一格

        上一条靠把 mtime 往前推一秒来保证确定性，这一条反过来把第二次写的 mtime **按回
        第一次的值**，于是缓存看不见那次改写。它是上一轮那条纪律（「缓存键要能被真实
        时钟推翻」）的代价：本机 mtime tick 现量 0.5 ms，连续两次 `write_text` 落在同一
        tick 的概率 61–65% ⇒ 真实场景里这条窗口存在，只是宽度不到一毫秒。

        **L88 的两句更正**（本轮跑变异探针时才发现这两个说法都不准确）：① 61–65% 量的
        是测试里连续两次 `write_text`（单次几微秒），不是产品写配置的 `save_config` ——
        后者实测 12.0–22.5 ms，是本机 tick 的 24–45 倍，连续两次落同一 tick 的概率
        **0/300**（两解释器各 300 轮）。② 本进程那一半在 L88 由 `deps.invalidate_config_caches`
        关掉（写配置成功后显式作废），所以这条边界现在测的是**跨进程**那一半：写方不走
        这条端点时（手编、`cp -p` / `rsync -t` 这类保住 mtime 的部署工具），窗口按定义
        仍然存在，且对「保住 mtime 的写方」是**无界**的（另立 A158）。将来若给键加上
        size/inode/内容哈希，**这条必须翻红**。
        """
        path = tmp_path / "config.yaml"
        path.write_text(yaml.safe_dump({"web": {"max_upload_bytes": 4096}}),
                        encoding="utf-8")
        import api.deps as deps
        monkeypatch.setattr(deps, "config_file_path", lambda: path)
        assert max_upload_bytes() == 4096
        first = path.stat().st_mtime_ns
        path.write_text(yaml.safe_dump({"web": {"max_upload_bytes": 8192}}),
                        encoding="utf-8")
        os.utime(path, ns=(path.stat().st_atime_ns, first))
        assert max_upload_bytes() == 4096, "mtime_ns 未变 ⇒ 这一格缓存按设计看不见那次改写"

    def test_broken_config_falls_back_to_factory_default(self, tmp_path, monkeypatch,
                                                         caplog):
        """配置坏了不该让上传 500，也不该回「无上限」——判据不能装在故障路径上"""
        path = tmp_path / "config.yaml"
        path.write_text("web:\n  max_upload_bytes: 0\n", encoding="utf-8")
        import api.deps as deps
        monkeypatch.setattr(deps, "config_file_path", lambda: path)
        assert max_upload_bytes() == DEFAULT
        assert "web.max_upload_bytes" in caplog.text or "回退" in caplog.text
        assert _config_upload_limit_cache == {}, "降级结果不该冻进缓存"

    def test_missing_config_file_is_not_cached(self, tmp_path, monkeypatch):
        path = tmp_path / "nonexistent.yaml"
        import api.deps as deps
        monkeypatch.setattr(deps, "config_file_path", lambda: path)
        assert max_upload_bytes() == DEFAULT
        assert _config_upload_limit_cache == {}


class TestDatasetShapeIsShared:
    """形态判据读侧与上传侧共用一份：`assert_dataset_shape`

    读侧文案主体「数据文件」逐字不动（既有 21 处用例引用它），上传侧传「上传内容」。
    """

    @pytest.mark.parametrize("payload,want", [
        ({"a": 1}, "顶层必须是 JSON 数组，当前是对象"),
        ('"just a string"', "顶层必须是 JSON 数组，当前是字符串"),
        (42, "顶层必须是 JSON 数组，当前是数字"),
        (None, "顶层必须是 JSON 数组，当前是空值"),
        ([1, 2], "第 1 个数据项必须是 JSON 对象，当前是数字"),
        (["a"], "第 1 个数据项必须是 JSON 对象，当前是字符串"),
        ([[{"instruction": "q"}]], "第 1 个数据项必须是 JSON 对象，当前是数组"),
        ([{"a": 1}, 42], "第 2 个数据项必须是 JSON 对象，当前是数字"),
    ])
    def test_read_side_wording_unchanged(self, tmp_path, payload, want):
        path = tmp_path / "d.json"
        path.write_text(json.dumps(payload), encoding="utf-8")
        with pytest.raises(HTTPException) as exc:
            read_items(path)
        assert exc.value.status_code == 400
        assert exc.value.detail.startswith("数据文件的")
        assert want in exc.value.detail

    @pytest.mark.parametrize("payload,subject", [
        ("hello", "上传内容"),
        ([1], "上传内容"),
    ])
    def test_upload_side_names_itself(self, payload, subject):
        with pytest.raises(HTTPException) as exc:
            assert_dataset_shape(payload, subject)
        assert exc.value.detail.startswith(f"{subject}的")

    def test_valid_dataset_passes_through_unchanged(self):
        data = [{"instruction": "q", "output": "a"}, {}]
        assert assert_dataset_shape(data) is data

    def test_empty_array_is_legal(self):
        """`[]` 是「零条数据」不是「形态不对」——上传一份空数据集应当能落盘"""
        assert assert_dataset_shape([]) == []


class TestEndpointShapeFace:
    """上传面的形态判决：拒绝得干净 = 没有半成品文件"""

    def post(self, client, filename, content):
        http, _, _ = client
        return http.post("/api/data/upload",
                        files={"file": (filename, content, "application/json")})

    @pytest.mark.parametrize("content,want", [
        (json.dumps("hello").encode(), "顶层必须是 JSON 数组"),
        (b"[1, 2, 3]", "第 1 个数据项必须是 JSON 对象"),
    ])
    def test_400_and_no_file_lands(self, client, content, want):
        import os
        target = pathlib.Path(os.getcwd()) / "l87_shape.json"
        response = self.post(client, "l87_shape.json", content)
        assert response.status_code == 400
        assert want in response.json()["detail"]
        assert not target.exists(), "拒绝之后数据目录里不许有半成品"

    def test_non_json_is_400_not_500(self, client):
        response = self.post(client, "l87_csv.json", b"instruction,output\nq,a\n")
        assert response.status_code == 400
        detail = response.json()["detail"]
        assert "Traceback" not in detail and "Expecting value" not in detail

    def test_invalid_utf8_is_400_without_echoing_bytes(self, client):
        """改前是 500 且把解码原文（含非法字节）回显给客户端"""
        response = self.post(client, "l87_utf8.json", b"\xff\xfe\x00not text")
        assert response.status_code == 400
        detail = response.json()["detail"]
        assert "UTF-8" in detail
        assert "b'" not in detail and "\x00" not in detail

    def test_empty_body_is_400(self, client):
        response = self.post(client, "l87_empty.json", b"")
        assert response.status_code == 400

    def test_valid_array_lands_and_roundtrips(self, client):
        import os
        http, _, _ = client
        content = json.dumps([{"instruction": "问", "output": "答"}],
                             ensure_ascii=False).encode("utf-8")
        response = self.post(client, "l87_ok.json", content)
        assert response.status_code == 200, response.json()
        assert response.json()["count"] == 1
        saved = pathlib.Path(os.getcwd()) / "l87_ok.json"
        assert json.loads(saved.read_text(encoding="utf-8")) == [
            {"instruction": "问", "output": "答"}]
        # 写进去的必须读得出来：两条路径共用同一份形态判据
        loaded = http.get("/api/data/load/l87_ok.json")
        assert loaded.status_code == 200
        assert loaded.json()["total"] == 1


class TestEndpointCeilingFace:
    """真实上传端点的上限档：边界两侧 + 拒绝后不落盘"""

    def post(self, http, filename, content):
        return http.post("/api/data/upload",
                         files={"file": (filename, content, "application/json")})

    @pytest.fixture(autouse=True)
    def patch_limit(self, monkeypatch):
        import api.routes.data as route
        self.limit_calls = []

        def fake_limit():
            return getattr(self, "limit", DEFAULT)

        monkeypatch.setattr(route, "max_upload_bytes", fake_limit)

    def test_exactly_at_limit_is_accepted(self, client):
        import os
        http, _, _ = client
        content = json.dumps([{"a": "x" * 40}]).encode("utf-8")
        self.limit = len(content)
        response = self.post(http, "l87_edge_ok.json", content)
        assert response.status_code == 200, response.json()
        assert (pathlib.Path(os.getcwd()) / "l87_edge_ok.json").exists()

    def test_one_byte_over_limit_is_413_and_no_file(self, client):
        import os
        http, _, _ = client
        content = json.dumps([{"a": "x" * 40}]).encode("utf-8")
        self.limit = len(content) - 1
        response = self.post(http, "l87_edge_no.json", content)
        assert response.status_code == 413
        detail = response.json()["detail"]
        assert "max_upload_bytes" in detail
        assert str(self.limit) in detail and str(len(content)) in detail
        assert not (pathlib.Path(os.getcwd()) / "l87_edge_no.json").exists()

    def test_factory_default_does_not_reject_today_biggest_real_file(self, client):
        """定标护栏：出厂默认不许拦掉今天任何能成功的上传

        取证面 = 工作目录根 **加上出厂白名单根 `data/`**。2026-09-28 那批遗留数据集按
        `OPTIMIZATION_PLAN.md` 3.2 的处方从工作目录根搬进了 `data/`，只 glob 一处会在
        搬家后取不到证（实测红在 `biggest = 0`）；落点本来就是可搬的，判据不该跟着钉死。
        仓内最大的数据集文件是 `train_data_final.json`（3 596 159 B）。
        """
        http, _, _ = client
        biggest = max(
            (p.stat().st_size
             for root in (AI_DIR, AI_DIR / "data")
             for p in root.glob("train_data*.json")),
            default=0)
        assert biggest, "取证需要在工作目录根或 data/ 里找到真实数据集文件"
        assert biggest < DEFAULT, "出厂默认应当远大于仓内最大文件，实测值：%d" % biggest
        assert MAX_UPLOAD_BYTES_MIN <= max_upload_bytes()

    def test_413_detail_carries_no_content(self, client):
        http, _, _ = client
        marker = "SECRET" + "z" * 200
        content = json.dumps([{"a": marker}]).encode("utf-8")
        self.limit = 64
        response = self.post(http, "l87_echo.json", content)
        assert response.status_code == 413
        assert "SECRET" not in response.text


class TestUploadPathIsHonoured:
    """改前 `Path(filename).name` 那一刀把 `data/xxx.json` 写成拿不到的出口

    白名单闸一字未松：`..` 仍 400，越界绝对路径仍 403。
    """

    def post(self, http, filename, content):
        return http.post("/api/data/upload",
                         files={"file": (filename, content, "application/json")})

    def test_subdirectory_target_is_written_where_the_client_said(self, client):
        import os
        http, _, _ = client
        (pathlib.Path(os.getcwd()) / "l87dir").mkdir(exist_ok=True)
        response = self.post(http, "l87dir/nested.json",
                             json.dumps([{"a": 1}]).encode("utf-8"))
        assert response.status_code == 200, response.json()
        assert response.json()["path"].replace("\\", "/").endswith("l87dir/nested.json")
        assert (pathlib.Path(os.getcwd()) / "l87dir" / "nested.json").exists()

    @pytest.mark.parametrize("filename,code", [
        ("../escape.json", 400),
        ("/etc/escape.json", 403),
    ])
    def test_escape_still_refused(self, client, filename, code):
        http, _, _ = client
        response = self.post(http, filename, json.dumps([{"a": 1}]).encode("utf-8"))
        assert response.status_code == code
