# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A123（L77）：配置节的回落值只有一个权威，节清单只有一个形状

`load_config` 从前带着两张手抄物：一张 20 行的「节 → 默认值字典」表（68 个键，交给
`_load_section` 的第四个参数），和 `models.default` 那一句 `get('default', 'ernie')`。
同族第一格是 L75 收掉的 `conf.get(key, 手抄默认)` 九行（A114），本轮收的是它的**第二、
第三格**：同一个回落值有两个权威，症状永远是同一句话 ——「改了 dataclass 没反应」。

改前三份实测：

- **键集与值的 census**（`Temp/l77q/a123_census.json`，**Python 字典面**：把值直接喂
  `_load_section`，提交态 edb26ddbe）：表里的 68 个键与 20 节 dataclass 的 init 字段
  双向差集为 0，68 个共有键逐键比 `==` 与 `type()` 两档漂移都是 0，六档输入 × 20 节 =
  120 例落值全等（其中 5 例两侧抛逐字相同的 `不能是 null`）⇒ 本轮是**纯搬家**，
  改前改后产出的对象相同。
- **YAML 面跨树 A/B**（`Temp/l77q/ab_tree.json`，17 例，HEAD 树与工作树各跑一遍真
  `load_config(path)`）：差异 0 例，含一节写成标量那一例（两侧同抛
  `ConfigError: rag 必须是「键: 值」的映射，当前是 'app'（str）`）。
- **`default_model` 的回落**（`Temp/l77q/default_model.json`，17 例里的两例）：
  `AppConfig().default_model` 与字面量 `'ernie'` 逐字相同 ⇒ 「键在场才覆盖」与改前
  等价，而回落值从此只有字段默认一个权威。

**写了就落地**那一维是 68 字段逐个真写进 YAML 量的（`Temp/l77q/landing_matrix.json`，
68/68），本文件把那张矩阵冻成常驻用例。
"""

import ast
import dataclasses
import inspect
import logging

import pytest
import yaml

from augmentor import config as config_module
from augmentor import logging_setup
from augmentor.config import AppConfig, _load_section, load_config

BASELINE = AppConfig()


@pytest.fixture(autouse=True)
def isolate_root_logger():
    """把 root logger 的 handler 与级别隔离在单条用例内（口径照 `test_logging_wiring_a97.py`）

    本文件不是日志测试，但它**必须**动日志：矩阵里 `logging.level` / `logging.file`
    两格走的是真 `load_config(path)`，而加载器在 `config.py:848` 会当场装配 root
    logger（A97 定的产品行为）。不隔离就会污染后面那些模块 —— 实测未隔离时
    `test_logging_wiring_a97.py` 红 4 条（A106 的命中签名漏进下一条用例，让该 miss
    的调用假命中），而症状看起来像「那个模块自己坏了」。
    """
    root = logging.getLogger()
    saved_handlers, saved_level = root.handlers[:], root.level
    for handler in logging_setup._INSTALLED[:]:
        handler.close()
    logging_setup._INSTALLED.clear()
    logging_setup._APPLIED = None
    root.handlers[:] = []
    root.setLevel(saved_level)
    yield
    for handler in logging_setup._INSTALLED[:]:
        if handler in root.handlers:
            root.removeHandler(handler)
        handler.close()
    logging_setup._INSTALLED.clear()
    logging_setup._APPLIED = None
    root.handlers[:] = saved_handlers
    root.setLevel(saved_level)

#: 20 个配置节的名字（从 `AppConfig` 的字段类型推，不写死清单 —— A123 的根正是
#: 「抄了一份就会漂」，测试这边再抄一份就是第二个权威）。
SECTION_NAMES = {f.name for f in dataclasses.fields(AppConfig)
                 if dataclasses.is_dataclass(getattr(BASELINE, f.name))}

#: 节名 → 该节的 dataclass 类型。
SECTION_CLASSES = {f.name: type(getattr(BASELINE, f.name)) for f in
                   dataclasses.fields(AppConfig)
                   if dataclasses.is_dataclass(getattr(BASELINE, f.name))}


def _auto_value(current):
    """从出厂默认造一个**必然不等**的合法候选值；被该节判据拒掉的字段走 `_OVERRIDES`"""
    if isinstance(current, bool):
        return not current
    if isinstance(current, int):
        return current + 1
    if isinstance(current, float):
        return current + 0.25
    if isinstance(current, str):
        return current + "_x" if current else "x"
    if isinstance(current, list):
        return list(current) + ["zz"]
    if isinstance(current, dict):
        return dict(current, zz=1)
    raise AssertionError("字段默认出现了新类型：%r" % (current,))


#: 自动生成值会被**该节自己的运行时判据**拒掉的字段。每一档都有实测出处
#: （`Temp/l77q/landing_matrix.json` 的 `bad` 清单，四档全部逐条读过）：
#: `augmentation.max_retry_wait` 的 `+0.25` 越过 300.0 上界、`dedup.threshold` 越过
#: 1.0 上界、`logging.level` 撞封闭清单（`必须是 CRITICAL/ERROR/WARNING/INFO/DEBUG/
#: NOTSET 之一`），`quality.weights` 的 `+["zz"]` 不是数字。
#: 这份表只该装「必须手写」的项：本探针的第一版在这里凭印象抄了九个字符串字段，
#: 实测九个取值**全部与出厂默认逐字相等**，于是「写了就落地」被我自己的判据误判成
#: 失败 —— 教训是进表之前先读默认。
_OVERRIDES = {
    ("augmentation", "max_retry_wait"): 120.0,
    ("dedup", "threshold"): 0.42,
    ("logging", "level"): "ERROR",
    ("quality", "weights"): [0.2, 0.5, 0.3],
}


def _written_fields():
    """全部 (节, 字段, 要写的值)，值从默认推、只有越界的走显式表

    只喂 **init** 字段：`init=False` 的字段按定义写不进去（加载器会把它丢掉，见
    `test_a_non_init_key_is_dropped_instead_of_crashing_the_loader`），把它放进矩阵
    等于造一条永远只能红的档。
    """
    out = []
    for section, cls in sorted(SECTION_CLASSES.items()):
        for f in dataclasses.fields(cls):
            if not f.init:
                continue
            current = getattr(getattr(BASELINE, section), f.name)
            value = _OVERRIDES.get((section, f.name), _auto_value(current))
            out.append((section, f.name, value))
    return out


WRITTEN = _written_fields()


def _write(tmp_path, body, name="l77.yaml"):
    path = tmp_path / name
    path.write_text(yaml.safe_dump(body, allow_unicode=True), encoding="utf-8")
    return str(path)


def _key_position_literals():
    """加载链路上出现在「键位」的字符串字面量（字典键 / `get·pop·setdefault` 的首参 /
    下标），不含 docstring 与报错文案 —— 文案里提历史写法是合法的，键位里点名读字段是
    危险的（L75 的 AST 判据就是为这个分寸而立的）。"""
    lits = set()
    for func in (load_config, _load_section):
        for node in ast.walk(ast.parse(inspect.getsource(func))):
            if isinstance(node, ast.Dict):
                lits |= {k.value for k in node.keys
                         if isinstance(k, ast.Constant) and isinstance(k.value, str)}
            elif (isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute)
                  and node.func.attr in ("get", "pop", "setdefault") and node.args
                  and isinstance(node.args[0], ast.Constant)
                  and isinstance(node.args[0].value, str)):
                lits.add(node.args[0].value)
            elif (isinstance(node, ast.Subscript)
                  and isinstance(node.slice, ast.Constant)
                  and isinstance(node.slice.value, str)):
                lits.add(node.slice.value)
    return lits


class TestLoaderHasNoSecondFallbackAuthority:
    """A123 本体：`_load_section` 不再接受回落表，加载器不再手抄默认值"""

    def test_load_section_signature_has_no_defaults_slot(self):
        """第四个参数 `defaults` 必须真的不存在，而不是留着不用"""
        params = list(inspect.signature(_load_section).parameters)
        assert params == ["raw_config", "key", "config_class"], params
        assert "defaults" not in params

    def test_no_defaulted_read_survives_in_the_loader(self):
        """机械守卫：加载链路上不许再有 `.get(键, 手抄默认)` 这种第二权威

        与 L75 的 `test_no_source_reads_an_entry_key_by_name_anymore` 同式。实测改后
        只剩一处按名字取键：`raw_config.get('logging')`，它是**单参**形态（不在场的
        答案由 `LoggingConfig` 的字段默认给），所以本条的红线是「带默认值的那一档」。
        """
        two_arg = []
        for func in (load_config, _load_section):
            for node in ast.walk(ast.parse(inspect.getsource(func))):
                if (isinstance(node, ast.Call)
                        and isinstance(node.func, ast.Attribute)
                        and node.func.attr in ("get", "pop", "setdefault")
                        and node.args and isinstance(node.args[0], ast.Constant)
                        and len(node.args) >= 2):
                    two_arg.append((func.__name__, node.args[0].value))
        assert two_arg == [], two_arg

    def test_loader_literals_are_only_section_names(self):
        """键位字面量只能是节名或 `models` / `default`，不许点名字段

        实测改后集合是 `{'default', 'logging', 'models'}`。这一条管的是「表以另一种
        形状回来」：把 68 个默认值写成字典字面量、或按字段名逐个点名，都会在这里现形，
        而它们在 `test_no_defaulted_read_survives_in_the_loader` 那一半可能躲过去
        （直接下标 `conf['temperature']` 就没有 `.get`）。
        """
        lits = _key_position_literals()
        assert lits <= SECTION_NAMES | {"models", "default"}, sorted(
            lits - SECTION_NAMES - {"models", "default"})

    def test_no_default_value_table_literal_reappears(self):
        """棘轮：加载器里不许再出现「多键字典字面量」当回落表

        改前的表就是 20 个 `{字段: 默认}` 字典字面量。实测改后 `load_config` 与
        `_load_section` 里**一个多键字典字面量都没有**，所以本条当下是空集；它是留给
        未来的：一旦有人把回落表搬回来（哪怕换个变量名），形状立刻红。
        """
        field_names = {n for cls in SECTION_CLASSES.values()
                       for n in (f.name for f in dataclasses.fields(cls))}
        dicts = []
        for func in (load_config, _load_section):
            for node in ast.walk(ast.parse(inspect.getsource(func))):
                if not isinstance(node, ast.Dict) or len(node.keys) < 2:
                    continue
                keys = [k.value for k in node.keys
                        if isinstance(k, ast.Constant) and isinstance(k.value, str)]
                if len(keys) >= 2 and set(keys) <= (field_names | SECTION_NAMES):
                    dicts.append((func.__name__, sorted(keys)))
        assert dicts == [], dicts


class TestSectionRegistryShape:
    """剩下的那张手写清单：只该有 (节名, 类) 两格，且与 `AppConfig` 两集相等"""

    @staticmethod
    def _registry_tuples():
        """从 `load_config` 的 AST 里取出节清单的字面量，不走「调一次看副作用」"""
        tree = ast.parse(inspect.getsource(load_config))
        lists = [n for n in ast.walk(tree)
                 if isinstance(n, ast.List) and n.elts
                 and all(isinstance(e, ast.Tuple) for e in n.elts)]
        assert len(lists) == 1, len(lists)
        out = []
        for elt in lists[0].elts:
            assert len(elt.elts) == 2, "清单的每一行必须是 (节名, 类)：%s" % ast.dump(elt)
            name, cls = elt.elts
            assert isinstance(name, ast.Constant) and isinstance(name.value, str)
            assert isinstance(cls, ast.Name), ast.dump(cls)
            out.append((name.value, cls.id))
        return out

    def test_registry_names_equal_appconfig_sections(self):
        """清单少一节 ⇒ 那节配置永远不加载；多一节 ⇒ `setattr` 挂出字段之外的属性"""
        names = [n for n, _ in self._registry_tuples()]
        assert len(names) == len(set(names)), "清单里有重复节名"
        assert set(names) == SECTION_NAMES, sorted(set(names) ^ SECTION_NAMES)
        assert len(names) == 20, len(names)

    def test_registry_classes_match_the_field_types(self):
        """每行配的类必须就是那个字段的类型（名字比对，不引实例）"""
        declared = {name: cls.__name__ for name, cls in SECTION_CLASSES.items()}
        wrong = [(name, cls, declared.get(name))
                 for name, cls in self._registry_tuples()
                 if declared.get(name) != cls]
        assert wrong == [], wrong

    def test_default_model_is_assigned_only_when_present(self, tmp_path):
        """`models:` 下没有 `default` 时不许碰 `AppConfig` 的那个字段

        改前那一句 `get('default', 'ernie')` 在键不在场时也会**写一遍** 'ernie'，值恰好
        与字段默认相同所以没症状（`Temp/l77q/default_model.json` 逐字比过）；真正的区别
        是权威：改后不写就是没写。本条用「换个字段默认」以外的方式判不了，所以钉的是
        在场 / 不在场两档的**结果**与 `AppConfig()` 一致。
        """
        absent = load_config(_write(tmp_path, {"models": {"m": {"type": "openai"}}}))
        assert absent.default_model == BASELINE.default_model
        present = load_config(_write(tmp_path, {"models": {"default": "ernie"}}))
        assert present.default_model == "ernie"


class TestSectionFieldShapes:
    """「只把在场的键交给构造器」这条策略对字段形状的两项前提"""

    def test_every_section_field_has_a_default(self):
        """有必填字段的那一节，缺键时 `config_class(**在场键)` 会 TypeError 而不是回落

        实测 20 节 68 字段今天**全部有默认值**（`Temp/l77q/a123_census.json` 的
        `init_without_default` 为空）。`ModelConfig.type` 是同族的反例，它走
        `_model_entry` 那条特判路径，所以本条不含它。
        """
        missing = []
        for name, cls in SECTION_CLASSES.items():
            for f in dataclasses.fields(cls):
                if (f.default is dataclasses.MISSING
                        and f.default_factory is dataclasses.MISSING):
                    missing.append((name, f.name))
        assert missing == [], missing

    def test_no_section_has_a_non_init_field(self):
        """棘轮：20 节里不许出现 `init=False` 字段

        加载器现在**丢**这样的键而不是吃下它（`names[k].init`，理由见
        `test_a_non_init_key_is_dropped_instead_of_crashing_the_loader`），于是这种
        字段的失效面从「加载即崩」变成「写了不生效、而且 `save_config` 每存一次就
        往文件里添一行死键」。后者更难查，所以这一格仍然只许由**明确决定**的人来开：
        改这条断言之前先想清楚谁来写这个字段。实测今天 20 节 68 字段无 `init=False`
        （`Temp/l77q/a123_census.json` 的 `non_init` 为空）。
        """
        non_init = []
        for name, cls in SECTION_CLASSES.items():
            non_init += [(name, f.name) for f in dataclasses.fields(cls)
                         if not f.init]
        assert non_init == [], non_init

    def test_a_non_init_key_is_dropped_instead_of_crashing_the_loader(self):
        """行为面（**Python 字典面**，直接喂 `_load_section`）：非 init 键不许变成崩

        为什么要有这一条而不是只靠上面的棘轮：`save_config` 用 `dataclasses.asdict()`
        序列化，而 asdict **不看 `init`** ⇒ 一旦某节有非 init 字段，它就会被写进 YAML，
        下一趟加载再把它喂回构造器。改前那版按 `__dataclass_fields__` 认键、不看
        `init`，实测（沙箱注入 `Temp/l77q/inject_l77.json` 的 m7 档）当场
        `TypeError: AugmentationConfig.__init__() got an unexpected keyword argument
        'l77_probe'` —— **产品自己写出的配置文件把自己打崩**。本用例用一个本地的类
        钉住修复后的形状，不依赖产品源码里恰好没有这种字段。
        """

        @dataclasses.dataclass
        class _Probe:
            kept: int = 1
            derived: int = dataclasses.field(init=False, default=9)

        got = _load_section({"probe": {"kept": 7, "derived": 123}}, "probe", _Probe)
        assert (got.kept, got.derived) == (7, 9)
        # 反空转：如果 `names[k].init` 那半句被删，这里拿到的是 TypeError 而不是落值
        with pytest.raises(TypeError):
            _Probe(kept=7, derived=123)


class TestEveryWrittenFieldLands:
    """68 个字段逐个写进 YAML，必须逐个落地（`Temp/l77q/landing_matrix.json` 的常驻版）

    这一类是 A123 的**反面**：`test_empty_sections_fall_back_to_dataclass_defaults`
    判「不写 = 由字段默认回答」，本类判「写了 = 真的落进节对象」。删表之后「字段有而
    表漏」这个失效面不复存在，但失效面换成了两个新的：清单漏一节（`TestSectionRegistryShape`
    判）、`fields()` 取键那一行被改坏（本类判）。取值全部从默认推，只有四个越界档
    走显式表（出处见 `_OVERRIDES` 注释），所以本类不需要任何豁免清单。
    """

    @pytest.mark.parametrize("section,field,value", WRITTEN)
    def test_written_field_lands(self, tmp_path, section, field, value):
        cfg = load_config(_write(tmp_path, {section: {field: value}},
                                 name="land_%s_%s.yaml" % (section, field)))
        landed = getattr(getattr(cfg, section), field)
        assert landed == value, (section, field, landed, value)
        # 反空转：取值必须与出厂默认不同，否则「落地」这条断言永远绿（探针第一版就
        # 红在这里 —— 九个手抄的取值与默认逐字相等，把搬家判成了失败）。
        assert landed != getattr(getattr(BASELINE, section), field), (section, field)

    def test_the_matrix_covers_every_field(self):
        """字段计数对账：新增字段而本矩阵没跟上，这里立刻红

        与 L75 的 `assert sorted(written) == sorted(MODEL_ENTRY_KEYS)` 同一口径，
        只是这里的清单从 `fields()` 推、不写死。
        """
        total = sum(len([f for f in dataclasses.fields(cls) if f.init])
                    for cls in SECTION_CLASSES.values())
        assert len(WRITTEN) == total, (len(WRITTEN), total)
        assert total == 68, total
        unknown = [(s, f) for s, f, _ in WRITTEN if (s, f) not in {
            (sec, fld.name) for sec, cls in SECTION_CLASSES.items()
            for fld in dataclasses.fields(cls) if fld.init}]
        assert unknown == [], unknown
        # 显式表里的每一项都必须真的对应一个字段：漂了的表项等于给未来的作者留陷阱。
        stale = [k for k in _OVERRIDES if k not in {(s, f) for s, f, _ in WRITTEN}]
        assert stale == [], stale


class TestAbsentAndNullShapes:
    """搬家之后「不在场」的四种形状照旧各自有解，且解与字段默认同值

    这四档就是 census 里 120 例的组成（`Temp/l77q/a123_census.json`，**Python 字典面**）；
    本类换到 YAML 面重跑一遍，因为产品入口只有 YAML。
    """

    SHAPES = ["absent", "bare", "empty", "unknown_only"]

    @staticmethod
    def _body(section, shape):
        if shape == "absent":
            return {}
        if shape == "bare":
            return {section: None}
        if shape == "empty":
            return {section: {}}
        if shape == "unknown_only":
            return {section: {"not_a_field_at_all": 1}}
        raise AssertionError("未知形状标签：%s" % shape)

    @pytest.mark.parametrize("section", sorted(SECTION_NAMES))
    @pytest.mark.parametrize("shape", SHAPES)
    def test_shapes_fall_back_to_the_class_default(self, tmp_path, section, shape):
        cfg = load_config(_write(tmp_path, self._body(section, shape),
                                 name="shape_%s_%s.yaml" % (section, shape)))
        assert getattr(cfg, section) == SECTION_CLASSES[section]()
