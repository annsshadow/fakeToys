# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A118 余四节（L82）：`export` / `vector` / `rag` / `multimodal` 四节 14 键四面同判

改前取证（`Temp/l82q/before.json`）：十档坏值在**三面**（SDK 直构 / `load_config` /
`POST /api/config`）全部静默通过，端照样回 200；静态面更彻底 —— 那四节在
`ConfigValidator.KNOWN_FIELDS` 里**一条规格都没有**（结构证据：
`git show HEAD:./augmentor/config_validator.py | grep -cE '"(export|vector|rag|multimodal)\\.'`
⇒ 0）。改后（`Temp/l82q/after.json`，`meta.timestamp` = 2026-09-26T15:26:54，
python 3.14.4）二十一档 × 四面全拒、每档静态面恰好一条错。

**读数的可信度分档写在这里**：`after.json` 带 `meta` 与 `guard`（含
`repo_config_untouched: True`），`before.json` **不带** —— 它是本轮第一版取证房的产物，
那一版没有隔离 CWD，于是 20 档 `POST /api/config` 把出厂 `config.yaml` 真写脏了
（326 行注释被 `save_config` 的重序列化吃掉）。改前读数本身仍成立（200 是当时真值），
但它是一次**累积态**的读数、且无法自证时刻，所以本文件不引它的逐档字段，只引它的
档位名单。护栏与教训见 `Temp/l82q/probe_faces.py` 文档串与本账本 L82 回看段。

四节的症状各不相同，所以分开记（同一批界、不同的代价来源）：
- `export.default_format` 有真实消费方（`pipeline.py:115`），坏值的症状不是「配置被拒」
  而是 `Exporter` 抛裸 `ValueError: 'xls' is not a valid ExportFormat` ⇒ 走 API 兜底
  `except Exception` 读成 500。`TestEnumErrorIsNotADomainError` 把这条不对称钉住。
- `vector` 节的两条界**本来就存在**，只是站在建库那一天（`create_vector_db`）。本轮
  搬家不新造数值：清单 `VECTOR_BACKENDS` 推导自 `vector.SUPPORTED_BACKENDS`，
  下界 `VECTOR_DIMENSION_MIN` 与静态规格同一个常数。
- `rag.chunk_*` 是四节里唯一一条**两侧都已有一半**的界：`RAGFormatter` 早判了
  `overlap >= size` 却不判正负，实测 `-1` 配 `-5` 构造成功、分块整件静默失效。
  现在那一判据的唯一产地是 `validation.require_chunk_window`，三处（`RAGFormatter`
  / `RAGConfig` / 静态面 `_validate_rag_window`）调的是同一个函数。
- `multimodal` 两键只判**形状**：标量写法会按字符拆成四个「扩展名」（与 `data_roots`
  同形）。「扩展名必须以 `.` 开头」刻意**不判** —— 那是消费方语义，而本仓还没有读
  这两键的产品消费者。那一洞连同「写了没人读」的 11 键一起记在 **A138**。
"""

import dataclasses
import pathlib
import re

import pytest
import yaml

from augmentor.config import (EXPORT_FORMATS, RAG_FORMATS, VECTOR_BACKENDS,
                              VECTOR_DIMENSION_MIN, ExportConfig,
                              MultimodalConfig, RAGConfig, VectorConfig,
                              apply_section_update, load_config)
from augmentor.config_validator import ConfigValidator
from augmentor.exceptions import DataValidationError
from augmentor.export import Exporter
from augmentor.export_enhanced import ExportFormat
from augmentor.rag import SUPPORTED_FORMATS, RAGFormatter
from augmentor.validation import require_choice, require_chunk_window
from augmentor.vector import SUPPORTED_BACKENDS, create_vector_db

AI_DIR = pathlib.Path(__file__).resolve().parent.parent.parent
SHIPPED_YAML = AI_DIR / "config.yaml"

SECTIONS = {"export": ExportConfig, "vector": VectorConfig,
            "rag": RAGConfig, "multimodal": MultimodalConfig}

# (节, 键, 坏值, 静态面该错的路径, 一句话原因)。`static_path` 单独写而不是从
# `section.key` 推，因为列表元素档的报错位置带下标（A120：判对还要指得出位置）。
BAD_VALUES = [
    ("export", "default_format", "xls", "export.default_format",
     "枚举里没有这一项，消费方抛的是裸 ValueError"),
    ("export", "default_format", "", "export.default_format",
     "空串在成员资格之前先被形状判据拒掉"),
    ("export", "default_format", None, "export.default_format", "写了键没给值"),
    ("export", "default_format", 5, "export.default_format", "整数不是格式名"),
    ("export", "formats", "jsonl", "export.formats",
     "写成标量 ⇒ 下游按字符迭代，与 data_roots 同形"),
    ("export", "formats", ["jsonl", 5], "export.formats[1]",
     "列表里混进非字符串"),
    ("export", "formats", ["jsonl", "not-a-format"], "export.formats[1]",
     "元素不在封闭清单里"),
    ("vector", "enabled", "yes", "vector.enabled", "字符串按真值读成「打开」"),
    ("vector", "enabled", None, "vector.enabled", "写了键没给值"),
    ("vector", "backend", "nonsense", "vector.backend",
     "界本来在 create_vector_db，本轮搬到读配置时"),
    ("vector", "dimension", 0, "vector.dimension", "消费方早就拒「非正整数」"),
    ("vector", "dimension", -384, "vector.dimension", "负维度"),
    ("vector", "dimension", "384", "vector.dimension", "字符串维度"),
    ("vector", "dimension", None, "vector.dimension", "写了键没给值"),
    ("vector", "storage_dir", "", "vector.storage_dir", "空路径"),
    ("vector", "collection", None, "vector.collection", "写了键没给值"),
    ("rag", "enabled", 0, "rag.enabled", "整数不是布尔，靠真值碰巧对上"),
    ("rag", "default_format", "pinecone", "rag.default_format",
     "RAG_FORMATS 里没有这一项"),
    # 下面三档的静态位置是**节名**而不是键名：那条界是跨键的，规格表只看单值，
    # 报它的是 `_validate_rag_window`（文案自己带两个全名，位置仍然指得出）。
    ("rag", "chunk_size", -1, "rag", "下界 1"),
    ("rag", "chunk_size", 0, "rag", "下界 1"),
    ("rag", "chunk_size", "512", "rag.chunk_size", "字符串"),
    ("rag", "chunk_overlap", -5, "rag", "下界 0"),
    ("multimodal", "enabled", "false", "multimodal.enabled",
     "YAML 里加了引号的 false 是字符串"),
    ("multimodal", "image_extensions", ".jpg", "multimodal.image_extensions",
     "标量会被拆成 . j p g 四个扩展名"),
    ("multimodal", "audio_extensions", [None], "multimodal.audio_extensions[0]",
     "列表里混进 None，下游变成一个名叫 None 的扩展名"),
]

# (节, 键, 合法值, 一句话原因)。收紧的代价记在这条轴上。
GOOD_VALUES = [
    ("export", "default_format", "jsonl", "出厂档"),
    ("export", "default_format", "alpaca", "清单里的另一项"),
    ("export", "formats", ["jsonl"], "单项列表"),
    ("export", "formats", [], "空清单是「一个格式也不导」，合法"),
    ("vector", "enabled", True, "打开向量库"),
    ("vector", "enabled", False, "出厂档"),
    ("vector", "backend", "faiss", "出厂档"),
    ("vector", "backend", "chromadb", "清单里的另一项"),
    ("vector", "dimension", 1, "闭区间下界（本轮只加下界，不加天花板）"),
    ("vector", "dimension", 4096, "高维嵌入模型合法，不许本地造第二道天花板"),
    ("vector", "storage_dir", "data/vectors", "出厂档"),
    ("rag", "enabled", False, "关掉这一节仍然关得掉"),
    ("rag", "default_format", "custom", "清单第三项"),
    # `chunk_size` 的合法档必须**看另一侧**：出厂 overlap 是 64，所以「大于 1」不够，
    # 得大于 64。这一档本身就是跨键界的一部分，写在参数表里最容易误当成单值界。
    ("rag", "chunk_size", 256, "出厂 overlap=64 之上的合法窗口"),
    ("rag", "chunk_overlap", 0, "不重叠是合法语义"),
    ("multimodal", "enabled", True, "打开多模态"),
    ("multimodal", "image_extensions", [".jpg"], "出厂形状"),
    ("multimodal", "image_extensions", [], "空清单 = 一个扩展名也不认"),
]


def _defaults(section):
    cls = SECTIONS[section]
    return {f.name: getattr(cls(), f.name) for f in dataclasses.fields(cls)}


def _construct(section, key, value):
    """SDK 直构那一面：其余键取该节默认值，只把被测键换掉"""
    cls = SECTIONS[section]
    return cls(**dict(_defaults(section), **{key: value}))


def _static(result, path):
    return [e for e in result.errors if e.path == path]


def _section_errors(result, section):
    """本节的全部错：位置可能是键名、`键[下标]`，也可能是节名（跨键那一遍）"""
    return [e for e in result.errors
            if e.path == section or e.path.startswith("%s." % section)]


def _static_errors(section, key, value):
    """`validate_config` 吃字典：一份只含一个坏键的最小配置"""
    result = ConfigValidator().validate_config({section: {key: value}})
    return _section_errors(result, section)


def _write_yaml(tmp_path, section, body, name="one.yaml"):
    cfg = tmp_path / name
    cfg.write_text(yaml.safe_dump({section: body}, allow_unicode=True),
                   encoding="utf-8")
    return str(cfg)


@pytest.fixture
def client(tmp_path, monkeypatch):
    """隔离到临时目录的 TestClient + 真配置桩

    `chdir` 是护栏而不是洁癖：`POST /api/config` 成功即 `save_config`，写的就是
    CWD 那份 `config.yaml`（本轮取证房第一版踩过，代价记在模块文档串）。桩里的
    配置从**仓库那份**加载，所以四节的运行时判据与产品同源。
    """
    config = load_config(str(SHIPPED_YAML))
    config.versioning.storage_dir = str(tmp_path / "versions")
    instance = type("PipelineStub", (), {})()
    instance.config = config
    monkeypatch.chdir(tmp_path)
    pristine = SHIPPED_YAML.read_bytes()
    (tmp_path / "config.yaml").write_bytes(pristine)
    import api.deps as deps
    from fastapi.testclient import TestClient
    from api.main import app
    monkeypatch.setattr(deps, "_pipeline", instance)
    return TestClient(app), instance, pristine


class TestFourFacesReject:
    """同一坏值：四面同时红。少任何一面就是 A77 / A118 的复发"""

    @pytest.mark.parametrize("section,key,value,static_path,why", BAD_VALUES)
    def test_runtime_constructs_raise(self, section, key, value, static_path, why):
        with pytest.raises(DataValidationError, match=re.escape(key)):
            _construct(section, key, value)

    @pytest.mark.parametrize("section,key,value,static_path,why", BAD_VALUES)
    def test_load_config_raises_at_load_time(self, tmp_path, section, key, value,
                                             static_path, why):
        path = _write_yaml(tmp_path, section, {key: value})
        with pytest.raises(DataValidationError, match=re.escape(key)):
            load_config(path)

    @pytest.mark.parametrize("section,key,value,static_path,why", BAD_VALUES)
    def test_static_face_reports_the_expected_path(self, section, key, value,
                                                   static_path, why):
        """静态面不仅要有错，还要报在**同一指法**的位置上（列表元素带下标）"""
        result = ConfigValidator().validate_config({section: {key: value}})
        errors = _static(result, static_path)
        assert errors, "%s=%r（%s）在 validate-config 上零反馈" % (
            static_path, value, why)
        assert errors[0].severity.value == "error"
        # 位置是节名那一档（跨键界）也必须指得出键：文案带全名，否则「报在 rag 上」
        # 等于把四个键重新混成一堆（A120 的口径）。其余档的文案按既有口径只写值
        # 与允许集合，位置由 `path` 承担，不在这里强求。
        if static_path == section:
            assert key in errors[0].message
        # 同一件事不许报两遍（`_validate_rag_window` 的「跳过」分支就是这个用途）
        assert len(_section_errors(result, section)) == 1


class TestApiFaceRejects:
    """`POST /api/config` 那一面：400 + 内存回滚 + 磁盘字节不变"""

    @pytest.mark.parametrize("section,key,value,static_path,why", BAD_VALUES)
    def test_400_named_by_the_key(self, client, section, key, value, static_path, why):
        http, instance, pristine = client
        response = http.post("/api/config", json={section: {key: value}})
        assert response.status_code == 400, why
        detail = response.json()["detail"]
        assert key in detail
        assert "Traceback" not in detail

    @pytest.mark.parametrize("section,key,value,static_path,why", BAD_VALUES)
    def test_400_leaves_live_config_and_disk_untouched(self, client, section, key,
                                                       value, static_path, why):
        import os
        http, instance, pristine = client
        before = dataclasses.asdict(getattr(instance.config, section))
        response = http.post("/api/config", json={section: {key: value}})
        assert response.status_code == 400
        assert dataclasses.asdict(getattr(instance.config, section)) == before
        on_disk = pathlib.Path(os.getcwd(), "config.yaml").read_bytes()
        assert on_disk == pristine, "坏值被 save_config 写进了 YAML"

    @pytest.mark.parametrize("section,key,value,why", GOOD_VALUES)
    def test_200_lands_a_legal_value(self, client, section, key, value, why):
        """反向护栏：合法档必须真的写得进去 —— 四面同判不等于四面全判负"""
        http, instance, _ = client
        response = http.post("/api/config", json={section: {key: value}})
        assert response.status_code == 200, "%s.%s=%r 被判负：%s" % (
            section, key, value, why)
        assert getattr(getattr(instance.config, section), key) == value

    def test_write_face_judges_the_window_against_the_live_other_side(self, client):
        """跨键界在写入面是真的动手：`chunk_size: 1` 单看合法，配上现存 overlap=64 就非法

        这一档不是设计出来的，是本轮跑测试时**当场撞出来的**（第一版把它当合法档写
        进了 GOOD_VALUES）。它是 A117 的跨键版：`apply_section_update` 写后复查整节
        ⇒ 单键「各自都合法」的两半合起来仍然可能被拒。症状从前是「200 之后重启即抛」，
        现在是 400 且一个字不落盘。
        """
        http, instance, pristine = client
        import os
        assert instance.config.rag.chunk_overlap == 64  # 出厂另一侧
        response = http.post("/api/config", json={"rag": {"chunk_size": 1}})
        assert response.status_code == 400
        detail = response.json()["detail"]
        assert "chunk_size" in detail and "chunk_overlap" in detail
        assert (instance.config.rag.chunk_size,
                instance.config.rag.chunk_overlap) == (512, 64)
        assert pathlib.Path(os.getcwd(), "config.yaml").read_bytes() == pristine

    def test_a_legal_pair_lands_in_one_batch(self, client):
        """同一批里两半一起改 ⇒ 判的是**写完后**的那一对，不是「旧 overlap 配新 size」"""
        http, instance, _ = client
        response = http.post("/api/config",
                             json={"rag": {"chunk_size": 1, "chunk_overlap": 0}})
        assert response.status_code == 200, response.json()
        assert (instance.config.rag.chunk_size,
                instance.config.rag.chunk_overlap) == (1, 0)


class TestLegalValuesStayLegal:
    """收紧判据的代价记在这条轴上：合法值四面同时绿"""

    @pytest.mark.parametrize("section,key,value,why", GOOD_VALUES)
    def test_runtime_constructs(self, section, key, value, why):
        assert getattr(_construct(section, key, value), key) == value

    @pytest.mark.parametrize("section,key,value,why", GOOD_VALUES)
    def test_static_face_is_quiet(self, section, key, value, why):
        result = ConfigValidator().validate_config({section: {key: value}})
        assert _section_errors(result, section) == []

    @pytest.mark.parametrize("section,key,value,why", GOOD_VALUES)
    def test_load_config_accepts(self, tmp_path, section, key, value, why):
        path = _write_yaml(tmp_path, section, {key: value})
        loaded = load_config(path)
        assert getattr(getattr(loaded, section), key) == value

    @pytest.mark.parametrize("size,overlap", [(1, 0), (7, 2), (512, 64),
                                              (65, 64), (2, 1)])
    def test_the_legal_windows_are_legal_on_all_three_config_faces(self, size,
                                                                  overlap):
        """闭区间一侧的成对合法档：`overlap == size - 1` 也必须放行（上界只判关系）"""
        body = {"chunk_size": size, "chunk_overlap": overlap}
        cfg = RAGConfig(**body)
        assert (cfg.chunk_size, cfg.chunk_overlap) == (size, overlap)
        assert RAGFormatter(**body) is not None
        result = ConfigValidator().validate_config({"rag": body})
        assert _section_errors(result, "rag") == []


class TestTheRangesAreOneCopyOnly:
    """界与清单住在 `config.py` / 各自的权威模块，规格表只是**引**它们（A77）"""

    @pytest.mark.parametrize("path,dim", [
        ("vector.dimension", "min"),
        ("export.default_format", "choices"),
        ("export.formats", "item_choices"),
        ("vector.backend", "choices"),
        ("rag.default_format", "choices"),
    ])
    def test_the_verdict_dims_are_gone_from_the_spec_table(self, path, dim):
        """L157 / B227 翻转：原先这条用 `is` 钉「表引用的就是那一份常数」

        判决维整批撤下后共引链移一格：清单与界住在运行时 `__post_init__` 的
        `require_choice` / `require_count` 调用点（同一批推导常数，本类其余用例
        钉其推导），规格表只声明类型。本条改钉「判决维不回表里」——谁给表里补
        一份抄写就红（A77：一条界不住两处）。
        """
        spec = ConfigValidator.KNOWN_FIELDS[path]
        assert dim not in spec,             "%s 的 %s 判决维回进规格表了（A139：判决权威只住运行时 + 回放）" % (path, dim)

    @pytest.mark.parametrize("section,key", [("rag", "chunk_size"),
                                             ("rag", "chunk_overlap")])
    def test_the_rag_window_bounds_are_not_in_the_spec_table(self, section, key):
        """反向的一条：这两行**故意不写 `min`**

        那条界是三刀的（`size >= 1`、`overlap >= 0`、`overlap < size`），唯一产地是
        `validation.require_chunk_window`。把 1 和 0 抄进规格表就是给同一条界造第二个
        家 —— 本用例让「顺手补上 min」当场红，除非那个人同时把 helper 里的下界删掉。
        """
        spec = ConfigValidator.KNOWN_FIELDS["%s.%s" % (section, key)]
        assert spec["type"] is int
        assert "min" not in spec, "%s 的下界被抄进了规格表" % key

    def test_the_derived_lists_are_the_authorities_not_a_copy(self):
        """三条清单必须由权威**推导**而来（对象身份级别，不是内容级别）"""
        assert EXPORT_FORMATS == tuple(f.value for f in ExportFormat)
        assert RAG_FORMATS == tuple(SUPPORTED_FORMATS)
        assert VECTOR_BACKENDS == tuple(SUPPORTED_BACKENDS)
        # 与 `models` 那一先例（`MODEL_TYPES` 本地抄 + 测试钉两集相等）的差别：
        # 那三个模块都不 import `config`，实测无循环导入 ⇒ 这里可以只有一份。
        # L157 / B227：choices 维撤出规格表后，`is` 共引链的那一端移到了运行时
        # （`ExportConfig.__post_init__` 对 `require_choice` 传的就是这份推导清单）；
        # 表里只剩类型声明，钉「清单维不回表里 + 运行时真拒」：
        assert "choices" not in ConfigValidator.KNOWN_FIELDS["export.default_format"]
        with pytest.raises(DataValidationError):
            ExportConfig(default_format="not-a-format")

    @pytest.mark.parametrize("section", sorted(SECTIONS))
    def test_every_field_of_the_four_sections_has_a_spec(self, section):
        """四节每个字段都要有静态规格 —— A118 的根之一就是「整条规格不存在」

        清单从 `dataclasses.fields` 推导，不写死。四节**没有豁免**（不像
        `quality.weights` 那条 A124 显式豁免），所以这里不需要 skip 分支。
        """
        missing = [f.name for f in dataclasses.fields(SECTIONS[section])
                   if "%s.%s" % (section, f.name)
                   not in ConfigValidator.KNOWN_FIELDS]
        assert missing == [], "%s 节有字段在 validate-config 上零反馈：%s" % (
            section, missing)


class TestTheTwoNewValidators:
    """`require_choice` / `require_chunk_window` 的单元面：判据本身的形状"""

    def test_require_choice_passes_a_listed_value(self):
        assert require_choice("backend", "faiss", VECTOR_BACKENDS) == "faiss"

    def test_require_choice_leaves_none_to_the_caller(self):
        """`None` 放行是全家族口径：配置那一侧由 `_reject_null_fields` 先拒

        本函数如果抢着判 `None`，`RAGFormatter(fmt=None)` 那种「没传就走默认」的
        调用点就会红 —— 那是要误伤的方向。
        """
        assert require_choice("backend", None, VECTOR_BACKENDS) is None

    @pytest.mark.parametrize("value", ["", "  ", 5, ["faiss"], True, 3.5])
    def test_require_choice_rejects_bad_shapes_before_membership(self, value):
        """形状错要报形状，不许被「不在清单里」含糊过去（A120 的「指得准」）"""
        with pytest.raises(DataValidationError):
            require_choice("vector.backend", value, VECTOR_BACKENDS)

    def test_require_choice_names_every_allowed_value(self):
        with pytest.raises(DataValidationError) as excinfo:
            require_choice("vector.backend", "nonsense", VECTOR_BACKENDS)
        message = str(excinfo.value)
        assert "vector.backend" in message and "nonsense" in message
        for backend in VECTOR_BACKENDS:
            assert backend in message, "文案没列出可选项，用户还要回头读源码"

    def test_chunk_window_accepts_the_shipped_shape(self):
        assert require_chunk_window("chunk_size", 512,
                                    "chunk_overlap", 64) is None

    @pytest.mark.parametrize("size,overlap,needle", [
        (0, 0, "chunk_size"),
        (-1, -5, "chunk_size"),      # 两个都负：只比关系永远看不出
        (100, 100, "chunk_overlap"),  # 相等也是「重叠不小于窗口」
        (100, 200, "chunk_overlap"),
        (7, -1, "chunk_overlap"),
        ("512", 64, "chunk_size"),
        (512, "64", "chunk_overlap"),
        (512, True, "chunk_overlap"),
    ])
    def test_chunk_window_three_cuts_in_order(self, size, overlap, needle):
        with pytest.raises(DataValidationError, match=needle):
            require_chunk_window("rag.chunk_size", size,
                                 "rag.chunk_overlap", overlap)

    @pytest.mark.parametrize("size,overlap", [(None, 64), (512, None),
                                              (None, None)])
    def test_chunk_window_requires_both_sides_of_the_comparison(self, size, overlap):
        """跨键界比不了 = 不能放行。

        改前 `RAGFormatter(chunk_size=None)` 抛的是 `TypeError`（响亮），若这里沿用
        家族那句「`None` 放行」，结果就成了静默构造成功 —— 把响亮换成静默不是对齐。
        """
        with pytest.raises(DataValidationError):
            require_chunk_window("rag.chunk_size", size,
                                 "rag.chunk_overlap", overlap)


class TestTheRagWindowIsSharedAcrossThreeCallsites:
    """配置节、`RAGFormatter`、静态面调的是**同一个**函数，不是三份长得像的判据"""

    @pytest.mark.parametrize("size,overlap", [(-1, -5), (0, 0), (100, 100),
                                              (512, 600), (7, 7)])
    def test_formatter_and_config_agree_without_reading_the_spec_table(self, size,
                                                                      overlap):
        with pytest.raises(DataValidationError):
            RAGFormatter(chunk_size=size, chunk_overlap=overlap)
        with pytest.raises(DataValidationError):
            RAGConfig(chunk_size=size, chunk_overlap=overlap)

    @pytest.mark.parametrize("size,overlap", [(0, 0), (100, 100), (512, 600),
                                              (-1, -5)])
    def test_static_face_replays_the_same_cuts(self, size, overlap):
        """`_validate_rag_window` 只回放，不重写：错在节名上，文案来自 helper"""
        result = ConfigValidator().validate_config(
            {"rag": {"chunk_size": size, "chunk_overlap": overlap}})
        errors = _static(result, "rag")
        assert len(errors) == 1
        assert "chunk" in errors[0].message

    def test_only_the_overlap_written_still_gets_the_cross_key_verdict(self):
        """`chunk_overlap: 600`、`chunk_size` 走默认（512）⇒ 默认值也从 dataclass 拿

        A123 之后 dataclass 是默认值的唯一权威，所以「只写一侧」这一档在
        `validate-config` 上不能绿（改前正是绿的：那条界当时只在 `RAGFormatter`）。
        """
        result = ConfigValidator().validate_config({"rag": {"chunk_overlap": 600}})
        errors = _static(result, "rag")
        assert len(errors) == 1
        assert "512" in errors[0].message
        assert str(RAGConfig().chunk_size) in errors[0].message

    @pytest.mark.parametrize("body,static_path", [
        ({"chunk_size": "x", "chunk_overlap": 600}, "rag.chunk_size"),
        ({"chunk_size": None, "chunk_overlap": 600}, "rag.chunk_size"),
    ])
    def test_shape_errors_are_not_reported_twice(self, body, static_path):
        """类型不对时跨键那一遍**跳过**：同一件事只报一次"""
        result = ConfigValidator().validate_config({"rag": body})
        assert len(_static(result, static_path)) == 1
        assert _static(result, "rag") == []

    def test_the_factory_window_still_validates_green(self):
        result = ConfigValidator().validate_config(
            {"rag": {"chunk_size": 512, "chunk_overlap": 64}})
        assert [e for e in result.errors if e.path.startswith("rag")] == []


class TestTheGatesBuyWhatTheConsumersCouldNot:
    """界搬到读配置时之后，消费点自己的界仍然在（本轮是搬家不是替换）"""

    def test_enum_error_is_not_a_domain_error(self):
        """钉住本节最硬的那条不对称：枚举错抛裸 `ValueError` ⇒ API 读成 500

        配置面先拒 ⇒ 同一个坏值再也走不到 `Exporter`。若哪天有人把 `ExportFormat`
        改成领域异常，本用例的 `ValueError` 断言会红，那时该重读的是这一段。
        """
        with pytest.raises(ValueError):
            Exporter(default_format="xls")
        with pytest.raises(DataValidationError):
            ExportConfig(default_format="xls")

    def test_vector_authority_still_judges_at_create_time(self):
        from augmentor.exceptions import VectorError

        with pytest.raises(VectorError):
            create_vector_db("nonsense")
        with pytest.raises(VectorError):
            create_vector_db("faiss", dimension=0)

    def test_formatter_still_cuts_the_legal_window(self):
        """反向护栏：收紧不许改掉合法档的分块行为"""
        blocks = RAGFormatter(chunk_size=7, chunk_overlap=2).chunk_text("x" * 23)
        assert [len(b) for b in blocks] == [7, 7, 7, 7, 3]


class TestTheWritePathRollsBack:
    """`apply_section_update`（写入面的唯一入口）对四节也真的在做事

    L73 写下那副机制时，四节还没有 `__post_init__`，于是对它们是空转。本轮判据
    落地后同一段代码开始动手 —— A117 那条「当次请求成功、重启后起不来」在四节上
    一并封死。
    """

    @pytest.mark.parametrize("section,key,value,static_path,why", BAD_VALUES)
    def test_bad_update_raises_and_leaves_the_section_untouched(
            self, section, key, value, static_path, why):
        obj = SECTIONS[section]()
        before = dataclasses.asdict(obj)
        with pytest.raises(DataValidationError, match=re.escape(key)):
            apply_section_update(obj, {key: value})
        assert dataclasses.asdict(obj) == before, "判负后没回滚"

    @pytest.mark.parametrize("body", [
        {"chunk_size": 1, "chunk_overlap": 0},
        {"chunk_overlap": 0, "chunk_size": 1},
    ])
    def test_a_legal_cross_key_batch_lands_whatever_the_dict_order(
            self, tmp_path, body):
        """成对改窗口：判的是**批次终态**那一一对，不是每写一条看一眼中间态

        与上面那条 400 用例合起来读：`{"chunk_size": 1}` 单独写仍然拒（现存
        overlap=64 与它配不成），两半一起写就放行。从前是反的 —— 每键一判让
        「中间态非法、终态合法」的批次彻底写不进去，那是本函数自己造出来的假负。
        """
        obj = RAGConfig()
        ignored = apply_section_update(obj, body)
        assert ignored == []
        assert (obj.chunk_size, obj.chunk_overlap) == (1, 0)
        # 落盘再来回一次：写入面放行终态 ⇒ 磁盘那份也必须加载得回来（A117 的口径）
        path = tmp_path / "pair.yaml"
        path.write_text(yaml.safe_dump({"rag": dataclasses.asdict(obj)}),
                        encoding="utf-8")
        assert load_config(str(path)).rag.chunk_size == 1


class TestTheFactoryConfigIsStillAccepted:
    """出厂 `config.yaml` 的四节 14 键：四面全绿，收紧不落在自己的默认值上"""

    def test_loads_and_matches_the_dataclass_defaults(self):
        config = load_config(str(SHIPPED_YAML))
        for section, cls in SECTIONS.items():
            body = yaml.safe_load(
                SHIPPED_YAML.read_text(encoding="utf-8")).get(section, {})
            merged = dict(_defaults(section), **body)
            loaded = dataclasses.asdict(getattr(config, section))
            assert loaded == dataclasses.asdict(cls(**merged))

    def test_static_face_is_quiet_on_the_four_sections(self):
        body = yaml.safe_load(SHIPPED_YAML.read_text(encoding="utf-8"))
        result = ConfigValidator().validate_config(body)
        noisy = [e for e in result.errors
                 if e.path.split(".")[0] in SECTIONS]
        assert noisy == []

    @pytest.mark.parametrize("section", sorted(SECTIONS))
    def test_disabling_a_section_still_disables_it(self, tmp_path, section):
        """本轮拒的是**坏形状**，不是加严开关：`enabled: false` 必须仍然关得掉"""
        if section == "export":
            pytest.skip("export 节没有开关，只有格式两键")
        path = _write_yaml(tmp_path, section, {"enabled": False})
        loaded = load_config(path)
        assert getattr(getattr(loaded, section), "enabled") is False


class TestTheGatesDoNotDependOnTheSpecTable:
    """判据在构造期真的动手：删掉规格表那一行，运行时仍然红

    上面几条钉「四面都判」，这条钉「四面各自成立」——「只留规格表、掏空
    `__post_init__`」那种写法当场红（L72 / L76 的同名用例，参数化到每一档）。
    """

    @pytest.mark.parametrize("section,key,value,static_path,why", BAD_VALUES)
    def test_runtime_still_raises_with_the_spec_row_deleted(self, section, key,
                                                            value, static_path, why):
        table = ConfigValidator.KNOWN_FIELDS
        saved = {k: v for k, v in table.items() if k.startswith("%s" % section)}
        try:
            for name in list(table):
                if name == section or name.startswith("%s." % section):
                    del table[name]
            with pytest.raises(DataValidationError):
                _construct(section, key, value)
        finally:
            table.update(saved)
