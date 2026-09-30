# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A118（L76）：`quality` / `dedup` 两节的开关与阈值两侧同判

改前取证（`Temp/l76q/before.json`，跑在提交态 b63648018 的工作树上）是十档坏值 ×
三个面（SDK 直构 / `load_config` / `validate_config`）。两节的症状**不一样**，所以
分开记：

- `quality` 那一节改前**两侧都没有界**（静态面只有裸数字 `min: 0.0 / max: 1.0`，
  运行时零判据）。实测读数：`threshold: 5.0` ⇒ 三档各 0.5 的样本总分 0.5 判
  `passed=False`，闸门把所有数据判成不合格；`threshold: -1.0` ⇒ `0.5 >= -1.0` 永真，
  闸门静默失效。两个方向都是「配置写错一个数字，产物整体反掉」而不出声。
  **账本 A118 原记的症状是错的**：它把这条写成「放行全部低质数据」，那只对负数
  那一支成立；`5.0` 那支是反向的。本轮按实测纠框，见文末纪律。
- `dedup.threshold` 反过来：运行时**有**界（`Deduplicator.__init__` 的
  `< 0 or > 1` 抛 `DedupError`），静态面**没有规格**（实测 `dedup: {threshold: 1.7}`
  得到 `is_valid=True` / 0 错），于是「校验绿、建对象才红」；而 `threshold: 'x'`
  连那道运行时判据也绕过 —— 字符串在 `<` 上直接
  `TypeError: '<' not supported between instances of 'str' and 'int'`。
- 两节的 `enabled` 共有同一支：`enabled: 'no'` 加载放行、`bool('no')` 是真 ⇒
  用户写「关掉」拿到「照样开」，而静态面对同一条已经报「期望布尔类型, 实际 str」。
  `dedup.enabled: 0` 那类非布尔今天靠真值判断**碰巧**对上意图，四个写法两种语义。

本轮不扩面到 `quality.weights`：它的形状与「和为 1」判据的权威在
`quality.QualityScorer`（A77 不许在这里抄第二份），而改前的
`weights: 'abc'` 恰恰是「长度判据被字符串满足、崩在 `sum()` 的 `TypeError`」——
两侧同批补才是修，单边加严是造新缝。那一洞记在 **A124**，本文件用
`test_weights_is_the_only_waived_field_of_quality` 把它钉成显式豁免而不是遗忘。
"""

import dataclasses
import pathlib

import pytest
import yaml

from augmentor.config import (DEDUP_THRESHOLD_RANGE, QUALITY_THRESHOLD_RANGE,
                              DedupConfig, QualityConfig, apply_section_update,
                              load_config)
from augmentor.config_validator import ConfigValidator
from augmentor.dedup import Deduplicator
from augmentor.exceptions import DataValidationError
from augmentor.validation import require_bool

SHIPPED_YAML = pathlib.Path(__file__).resolve().parent.parent.parent / "config.yaml"

SECTIONS = {"quality": QualityConfig, "dedup": DedupConfig}

# (节, 键, 坏值, 一句话原因)。每例都要在**两个面**上同时红。
BAD_VALUES = [
    ("quality", "enabled", "no", "字符串按真值读成「打开」，用户要的是关"),
    ("quality", "enabled", "false", "YAML 里加了引号的 false 是字符串"),
    ("quality", "enabled", 0, "整数不是布尔，今天靠真值判断碰巧对上"),
    ("quality", "enabled", 1, "同上，且 `1` 与 `true` 在下游不可区分"),
    ("quality", "enabled", [], "空列表也是假值"),
    ("quality", "enabled", None, "写了键没给值"),
    ("quality", "threshold", 5.0, "上界之外：闸门把所有数据判成不合格"),
    ("quality", "threshold", -1.0, "下界之外：闸门静默失效"),
    ("quality", "threshold", "0.6", "字符串"),
    ("quality", "threshold", True, "布尔被算术读成 1"),
    ("quality", "threshold", None, "写了键没给值"),
    ("quality", "threshold", float("nan"), "NaN 过不了任何比较"),
    ("dedup", "enabled", "no", "同 quality 那一支"),
    ("dedup", "enabled", 0, "同 quality 那一支"),
    ("dedup", "enabled", None, "写了键没给值"),
    ("dedup", "threshold", 1.7, "上界之外"),
    ("dedup", "threshold", -0.1, "下界之外"),
    ("dedup", "threshold", "x", "改前崩在 Deduplicator 的 `<`，报的是 TypeError"),
    ("dedup", "threshold", None, "写了键没给值"),
    ("dedup", "threshold", float("nan"), "NaN 改前过不了 `<`，也进不了去重"),
]

# (节, 键, 合法值, 一句话原因)。每例都要在**两个面**上同时绿。
GOOD_VALUES = [
    ("quality", "enabled", True, "出厂档"),
    ("quality", "enabled", False, "关掉闸门是一档合法语义，不许被判负"),
    ("quality", "threshold", 0.0, "闭区间下界：一条都不放过"),
    ("quality", "threshold", 1.0, "闭区间上界：只要满分"),
    ("quality", "threshold", 0.6, "出厂档"),
    ("quality", "threshold", 1, "YAML 写整数：运行时收 int，静态面同样收"),
    ("dedup", "enabled", False, "同上"),
    ("dedup", "threshold", 0.0, "闭区间下界"),
    ("dedup", "threshold", 1.0, "闭区间上界：与 `Deduplicator` 的既有判据同集合"),
    ("dedup", "threshold", 0.9, "出厂档"),
]


def _defaults(section):
    cls = SECTIONS[section]
    return {f.name: getattr(cls(), f.name) for f in dataclasses.fields(cls)}


def _construct(section, key, value):
    """SDK 直构那一面：其余键取该节默认值，只把被测键换掉"""
    cls = SECTIONS[section]
    return cls(**dict(_defaults(section), **{key: value}))


def _raw(section, key, value):
    """`validate_config` 吃字典：一份只含一个坏键的最小配置"""
    return {section: {key: value}}


def _static_errors(section, key, value):
    result = ConfigValidator().validate_config(_raw(section, key, value))
    return [e for e in result.errors if e.path == "%s.%s" % (section, key)]


class TestBothFacesReject:
    """同一坏值：运行时构造抛、静态面报红 —— 少一边就是 A77 / A118 的复发"""

    @pytest.mark.parametrize("section,key,value,why", BAD_VALUES)
    def test_runtime_rejects(self, section, key, value, why):
        with pytest.raises(DataValidationError, match=key):
            _construct(section, key, value)

    @pytest.mark.parametrize("section,key,value,why", BAD_VALUES)
    def test_static_face_reports_the_same_path(self, section, key, value, why):
        errors = _static_errors(section, key, value)
        assert errors, "%s.%s=%r（%s）在 validate-config 上零反馈" % (
            section, key, value, why)
        assert errors[0].severity.value == "error"

    @pytest.mark.parametrize("section,key,value,why", BAD_VALUES)
    def test_runtime_face_does_not_depend_on_the_spec_table(self, section, key, value, why):
        """判据在构造期真的动手：把规格表里那一行删掉，运行时那一侧仍然红。

        上面两条钉「两边都判」，这条钉「两边各自都成立」—— 于是「只留规格表、
        掏空 `__post_init__`」那种写法当场红（L72 的同名用例，参数化到每一个坏值
        而不是只测一档，理由见该文件的 m4 注入实测）。
        """
        table = ConfigValidator.KNOWN_FIELDS
        path = "%s.%s" % (section, key)
        saved = table.get(path)
        try:
            table.pop(path, None)
            with pytest.raises(DataValidationError):
                _construct(section, key, value)
        finally:
            if saved is not None:
                table[path] = saved


class TestBothFacesAccept:
    """合法档不许被任何一面误判 —— 收紧判据的代价就记在这条轴上"""

    @pytest.mark.parametrize("section,key,value,why", GOOD_VALUES)
    def test_runtime_constructs(self, section, key, value, why):
        cfg = _construct(section, key, value)
        assert getattr(cfg, key) == value

    @pytest.mark.parametrize("section,key,value,why", GOOD_VALUES)
    def test_static_face_is_quiet(self, section, key, value, why):
        assert _static_errors(section, key, value) == []

    @pytest.mark.parametrize("section", sorted(SECTIONS))
    def test_disabling_a_section_still_disables_it(self, tmp_path, section):
        """收紧的代价不许落在「关掉这一节」上：`enabled: false` 必须仍然关得掉

        本轮拒的是**非布尔**写法，不是加严开关本身。这里从 YAML 一路走到字段，
        因为症状当初就发生在「下游按真值判断」那一步。
        """
        cfg = tmp_path / "off.yaml"
        cfg.write_text("%s:\n  enabled: false\n" % section, encoding="utf-8")
        loaded = load_config(str(cfg))
        assert getattr(getattr(loaded, section), "enabled") is False


class TestTheLoadPathAppliesTheGates:
    """产品面：`load_config` 不再放行这些写法，且报错点就在加载这一步"""

    @pytest.mark.parametrize("section,key,body", [
        ("quality", "enabled", "  enabled: 'no'"),
        ("quality", "threshold", "  threshold: 5.0"),
        ("quality", "threshold", "  threshold: -1.0"),
        ("quality", "threshold", "  threshold: 'x'"),
        ("quality", "threshold", "  threshold:"),
        ("dedup", "threshold", "  threshold: 'x'"),
        ("dedup", "threshold", "  threshold: 1.7"),
        ("dedup", "enabled", "  enabled: 0"),
    ])
    def test_loading_a_bad_yaml_raises_at_load_time(self, tmp_path, section, key, body):
        cfg = tmp_path / ("%s_%s.yaml" % (section, key))
        cfg.write_text("%s:\n%s\n" % (section, body), encoding="utf-8")
        with pytest.raises(DataValidationError, match="%s.%s" % (section, key)):
            load_config(str(cfg))

    @pytest.mark.parametrize("section,key,body", [
        ("quality", "enabled", "  enabled: 'no'"),
        ("quality", "threshold", "  threshold: 5.0"),
        ("dedup", "threshold", "  threshold: 'x'"),
        ("dedup", "threshold", "  threshold: 1.7"),
    ])
    def test_the_message_names_the_key_and_shows_the_value(self, tmp_path, section, key, body):
        """改前那两支 `TypeError` 的问题不是「拒了」而是「指不出哪儿」

        报错文案要同时含节名.键名与原值，否则用户还是要回头读源码（A120 的口径：
        判对不够，要指得准）。
        """
        cfg = tmp_path / "msg.yaml"
        cfg.write_text("%s:\n%s\n" % (section, body), encoding="utf-8")
        with pytest.raises(DataValidationError) as exc:
            load_config(str(cfg))
        assert "%s.%s" % (section, key) in str(exc.value)
        assert body.split(":", 1)[1].strip() in str(exc.value)

    @pytest.mark.parametrize("section", sorted(SECTIONS))
    def test_writing_only_the_section_name_still_gives_defaults(self, tmp_path, section):
        """只写节名（`quality:`）仍然是「该节全默认」，本轮判据没把它读成坏值"""
        cfg = tmp_path / "bare.yaml"
        cfg.write_text("%s:\n" % section, encoding="utf-8")
        loaded = load_config(str(cfg))
        assert getattr(loaded, section) == SECTIONS[section]()

    def test_a_valid_pair_still_loads_and_flows_to_the_consumers(self, tmp_path):
        """合法配置的落点：阈值真的进了 `QualityScorer` / `Deduplicator`

        这条是「判据没顺手改变合法路径」的证据，不是重复 `pipeline` 的装配测试：
        两节的新界与消费方自己的界同集合，所以 0.7 / 0.8 一档必须照常构造成功。
        """
        cfg = tmp_path / "ok.yaml"
        cfg.write_text("quality:\n  threshold: 0.7\ndedup:\n  threshold: 0.8\n",
                       encoding="utf-8")
        loaded = load_config(str(cfg))
        assert loaded.quality.threshold == 0.7
        assert loaded.dedup.threshold == 0.8
        assert Deduplicator(threshold=loaded.dedup.threshold).threshold == 0.8


class TestDedupParityWithItsExistingGate:
    """`DedupConfig` 的新判据与 `Deduplicator` 的既有判据**对数值等判**

    改前 `Deduplicator.__init__` 的 `if threshold < 0 or threshold > 1` 是这一族
    唯一的界（含两端）。本轮把同一条界搬到配置侧，所以两边的**判决**必须逐值相同；
    若哪天有人只改一边的界，这条当场红 —— 那正是 A77 抱怨的「两份权威各自漂」。

    下面的档位是**数值**那一半。`bool` 与 `NaN` 两边**不等判**（配置侧拒、消费侧放），
    那是消费侧的一处既有洞、不是本轮的口径选择，由 `TestDedupGateHasABoolNanHole`
    单独钉住并记成 A125。把两件事写在一个类里，是为了让读代码的人不会以为
    「等判」是全域成立的。
    """

    @pytest.mark.parametrize("value", [0.0, 0.5, 0.9, 1.0, -0.0001, 1.0001, 1.7,
                                       -1, 2, 0.7, "0.9", None])
    def test_config_and_deduplicator_give_the_same_verdict(self, value):
        def config_raises():
            try:
                DedupConfig(threshold=value)
            except Exception as exc:  # noqa: BLE001
                assert isinstance(exc, DataValidationError)
                return True
            return False

        def consumer_raises():
            try:
                Deduplicator(threshold=value)
            except Exception:  # noqa: BLE001
                return True
            return False

        assert config_raises() == consumer_raises(), \
            "threshold=%r：配置侧与消费侧判决劈叉" % (value,)

    def test_the_config_side_names_the_key_while_the_consumer_side_does_not(self):
        """两边**都**拒，但只有配置侧的文案指得出是哪个键

        `DedupError: 阈值必须在 0-1 之间` 不提 `dedup.threshold`，它是在建对象时
        抛的（`pipeline.py` 那一层）；本轮把出声点提前到加载/写入这一步，并把键名
        带进文案。这条钉的是「两边同判不等于两边同文案」，别哪天为了统一把配置侧
        的键名去掉。
        """
        with pytest.raises(DataValidationError, match="dedup.threshold"):
            DedupConfig(threshold=1.7)
        with pytest.raises(Exception, match=r"阈值必须在 0-1 之间") as exc:
            Deduplicator(threshold=1.7)
        assert "dedup.threshold" not in str(exc.value)
        assert not issubclass(exc.type, DataValidationError), \
            "消费侧改抛 DataValidationError 会让 `dedup` 的公开异常契约漂走"


class TestDedupGateHasABoolNanHole:
    """A125（本轮取证、下一轮收）：`Deduplicator` 自己那道界判不住 `bool` 与 `NaN`

    实测（`Temp/l76q/parity.json`，同一份三条样本、其中两条逐字相同）：

    - `threshold=0.9` → removed=1（正常）
    - `threshold=float('nan')` → **removed=0 / groups=[]**，构造成功、一声不吭。
      判据是 `if threshold < 0 or threshold > 1`，而 `nan < 0` 与 `nan > 1` 都是
      `False` ⇒ NaN 穿过那道界，再到 `dist >= self.threshold` 上恒假 ⇒ **去重整条
      静默失效**。这是 A118 家族里最坏的一种读法：越界的配置不是「报错」也不是
      「反过来」，而是「什么都没做」——与 `retry.compute_delay` 的负 jitter
      （L48 / `require_ratio` 的立员理由）同形。
    - `threshold=True` → 构造成功、按 1.0 用（`1.0 >= True` 是真），实测 removed=1
      （只删逐字相同的那一组）；`threshold=False` → 按 0.0 用，实测 **removed=2、
      groups=[[0, 1, 2]]** —— 三条不同的样本被并成一组删掉两条。用户写的是布尔，
      被算术读成了一个他没写过的阈值，而且误删的是**真数据**。
      A125 收口时这两个方向都要判掉。

    本轮**不动** `dedup.py`：那里的判据是 SDK 公开契约的一部分（改它等于改变
    `DedupError` 的触发集合，与 `use_faiss` 那一支同批评审才划算），而配置路径上
    这两个值已经被 `require_ratio` 挡在 `load_config` / `POST /api/config` 两步。
    这条用例钉的是「今天两边不等判」这个事实：A125 收口时它会红，那时按红字翻转，
    不许静默删（L50 清空豁免清单的同一口径）。
    """

    @pytest.mark.parametrize("value", [True, False, float("nan")])
    def test_config_side_rejects_what_the_consumer_side_still_accepts(self, value):
        with pytest.raises(DataValidationError, match="dedup.threshold"):
            DedupConfig(threshold=value)
        Deduplicator(threshold=value)  # 不许抛：这就是 A125 的洞本身

    def test_a_nan_threshold_silently_deduplicates_nothing(self):
        """产品面读数：NaN 穿过消费侧那道界之后，去重对逐字重复的样本零动作"""
        items = [{"instruction": "租房合同到期怎么办"},
                 {"instruction": "租房合同到期怎么办"},
                 {"instruction": "今天天气如何"}]
        baseline = Deduplicator(threshold=0.9).deduplicate(items)
        assert baseline.removed_count == 1, "这份样本不再是「含一组真重复」，用例失效"
        nan_result = Deduplicator(threshold=float("nan")).deduplicate(items)
        assert nan_result.removed_count == 0
        assert nan_result.duplicate_groups == []


class TestTheRangesAreOneCopyOnly:
    """界住在 `config.py`，规格表只是引它"""

    @pytest.mark.parametrize("section,key,expected", [
        ("quality", "threshold", QUALITY_THRESHOLD_RANGE),
        ("dedup", "threshold", DEDUP_THRESHOLD_RANGE),
    ])
    def test_the_spec_bounds_are_the_config_constants(self, section, key, expected):
        spec = ConfigValidator.KNOWN_FIELDS["%s.%s" % (section, key)]
        assert (spec["min"], spec["max"]) == expected

    @pytest.mark.parametrize("section,key", [
        ("quality", "threshold"), ("dedup", "threshold")])
    def test_one_step_outside_the_table_is_red_on_both_faces(self, section, key):
        """拿规格表自己的界算探针，要求两边同时拒 —— 只改一边界的写法当场红"""
        spec = ConfigValidator.KNOWN_FIELDS["%s.%s" % (section, key)]
        for probe in (spec["min"] - 0.5, spec["max"] + 0.5):
            with pytest.raises(DataValidationError, match=key):
                _construct(section, key, probe)
            assert _static_errors(section, key, probe), \
                "%s=%r 规格表拒了、运行时没拒" % (key, probe)

    @pytest.mark.parametrize("section", sorted(SECTIONS))
    def test_every_field_of_the_two_sections_has_a_spec(self, section):
        """两节的每个字段都要有静态规格 —— A118 的根之一就是「整条规格不存在」

        清单从 `dataclasses.fields` 推导，不写死。唯一的豁免是 `quality.weights`，
        由下面那条单独钉住（豁免要显式写，不能靠对账漏掉）。
        """
        missing = []
        for f in dataclasses.fields(SECTIONS[section]):
            if f.name == "weights":
                continue
            if "%s.%s" % (section, f.name) not in ConfigValidator.KNOWN_FIELDS:
                missing.append(f.name)
        assert missing == [], "%s 节有字段在 validate-config 上零反馈：%s" % (
            section, missing)

    def test_weights_is_the_only_waived_field_of_quality(self):
        """`quality.weights` 是**记下**的豁免（A124），不是对账漏掉的洞

        这条看着像「把上一条例外硬编进来」，它守的是方向：A124 两侧同批补完之后，
        这里必须跟着翻（`"weights" not in KNOWN_FIELDS` 变红），否则豁免就成了
        永久的。与 L50 清空豁免清单同一个做法。
        """
        assert "quality.weights" not in ConfigValidator.KNOWN_FIELDS
        with pytest.raises(DataValidationError, match="weights"):
            QualityConfig(weights=None)


class TestTheWritePathIsNowGatedToo:
    """`POST /api/config` 那一面：L73 的 `apply_section_update` 从此对两节真的复查

    改前两节没有 `__post_init__`，那副机制对它们是空转（`post_init is None` ⇒
    直接写入）。本轮判据落地后同一段代码开始动手 —— 这是 A117 那条
    「当次请求成功、重启后起不来」在两节上的封死。
    """

    @pytest.mark.parametrize("section,key,value", [
        ("quality", "threshold", 5.0),
        ("quality", "threshold", -1.0),
        ("quality", "enabled", "no"),
        ("dedup", "threshold", 1.7),
        ("dedup", "enabled", 0),
    ])
    def test_bad_update_raises_and_leaves_the_section_untouched(self, section, key, value):
        obj = SECTIONS[section]()
        before = dataclasses.asdict(obj)
        with pytest.raises(DataValidationError, match=key):
            apply_section_update(obj, {key: value})
        assert dataclasses.asdict(obj) == before, "判负后没回滚"

    @pytest.mark.parametrize("section,key,value", [
        ("quality", "threshold", 0.75),
        ("dedup", "threshold", 0.5),
        ("dedup", "enabled", False),
    ])
    def test_good_update_still_lands(self, section, key, value):
        obj = SECTIONS[section]()
        ignored = apply_section_update(obj, {key: value})
        assert ignored == []
        assert getattr(obj, key) == value

    def test_a_batch_that_fails_late_rolls_back_the_earlier_key(self):
        """批次里第一条合法、第二条越界 ⇒ 两条都不留（L73 的「要么全落要么全不落」）

        改前这一节根本没有判据，所以 `{"threshold": 0.7, "enabled": "no"}` 会**整批
        写进去**并被 `save_config` 落到磁盘上；现在第二条抛、第一条回滚。
        """
        obj = QualityConfig()
        with pytest.raises(DataValidationError, match="enabled"):
            apply_section_update(obj, {"threshold": 0.7, "enabled": "no"})
        assert obj.threshold == 0.6 and obj.enabled is True


class TestRequireBoolIsAFamilyMemberNotAOneOff:
    """新加的 `require_bool` 自己的契约：与同族其他成员同读法

    族里其他成员都被多节共用；本员今天只有两节用。把契约钉在这里，是为了下一节
    （A118 剩下的 `export` / `rag` / `vector` / `multimodal`，四节各有 `enabled`
    或同类开关）复用同一判据而不是另写一份。
    """

    def test_none_means_no_argument_was_passed(self):
        """与 `require_count` / `require_ratio` 同口径：`None` 短路放行

        配置对象那一侧不会被这一条放过 —— `_reject_null_fields` 先拒（上面的
        `threshold: None` 两档就是证据）。这里钉的是「函数本身不替调用点决定默认值」。
        """
        assert require_bool("x.enabled", None) is None

    @pytest.mark.parametrize("value", [True, False])
    def test_both_bools_pass(self, value):
        assert require_bool("x.enabled", value) is value

    @pytest.mark.parametrize("value", ["true", "no", 0, 1, [], {}, 0.0])
    def test_everything_else_is_rejected_by_name(self, value):
        """不夹、不折、不 `bool(value)`：报错文案点名 true/false 两种合法写法"""
        with pytest.raises(DataValidationError) as exc:
            require_bool("x.enabled", value)
        message = str(exc.value)
        assert "x.enabled" in message
        assert "true 或 false" in message


class TestShippedValuesStillFit:
    """出厂 `config.yaml` 的两节必须仍在新界内（收紧的代价不能砸在自己的模板上）"""

    def test_shipped_yaml_loads_and_validates_clean(self):
        with open(SHIPPED_YAML, encoding="utf-8") as handle:
            raw = yaml.safe_load(handle)
        result = ConfigValidator().validate_config(raw)
        assert [e for e in result.errors
                if e.path.startswith(("quality.", "dedup."))] == []
        loaded = load_config(str(SHIPPED_YAML))
        assert QUALITY_THRESHOLD_RANGE[0] <= loaded.quality.threshold <= QUALITY_THRESHOLD_RANGE[1]
        assert DEDUP_THRESHOLD_RANGE[0] <= loaded.dedup.threshold <= DEDUP_THRESHOLD_RANGE[1]
        assert loaded.quality.enabled is True
        assert loaded.dedup.enabled is True
