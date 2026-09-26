# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A126（L78）：`models` 这一族的键集只有一份权威，而且那份权威认 `init`

L77 给节面（`_load_section`）加了 `init` 过滤，理由是实测：`save_config` 用
`dataclasses.asdict()` 写文件，而 asdict **不看 `init`** ⇒ 非 init 字段会被写进 YAML，
下一趟加载把它喂回构造器就 `TypeError`（「产品自己写出的配置文件把自己打崩」）。

本轮把同一句话在三处落地，三处都是改前**现量**撞出来的（命令与读数见每条用例的理由
和 `Temp/l78q/` 的三份工件），而不是「看着不一致就顺手统一」：

1. `MODEL_ENTRY_KEYS`（加载侧决定哪些键进构造器）—— 改前不判 `init`。
2. `ConfigValidator._warn_unread_model_keys` 里那行 `known`（反馈侧决定哪些键算有人读）
   —— 改前自己写一遍 `sorted(f.name for f in fields(ModelConfig))`，与第 1 处**同源
   两份推导**；本轮让它直引第 1 处，权威从两份变一份。
3. `_reject_null_fields` 的整节推导那一支（「写了键却没给值」判据）—— 改前按**全字段**
   判，实测对一个非 init 的 `None` 字段抛
   `probe.derived 不能是 null（配置里写了这个键却没有给值）`
   （`Temp/l78q/perf_a126.json` 每跑 `verdicts` 的 `non-init-None` 档）—— 一句指向
   **不存在的用户笔误**的报错。

今天 `config` 模块的 22 个 dataclass（20 节 + `ModelConfig` + `AppConfig` 本体）里
非 init 字段 **0 个**
（`Temp/l78q/a126_census.json` 的 `non_init_fields` 实测为空）⇒ 本轮没有现行故障可修，
是结构收口；正因如此，**能红的用例必须自己造出那一形状**（本文件的探针类），
而不是写一条「今天两侧恰好相等」的永绿断言。引用一律用名字锚点，不用行号（A127）。

行为面那一半照 L77 的先例补齐（A126 行里点名的第三条修法）：`ModelConfig` 绑死在
`_model_entry` 上，探针类进不去，于是常驻用例钉的是**这条不变量本身** ——
`save_config` 写出的每一个键都必须在 `MODEL_ENTRY_KEYS` 里（见
`TestWriterNeverEmitsARefusedKey`）。沙箱侧同一形状的「不加过滤会怎样」见
`Temp/l78q/inject_l78.json` 的 m7 / m7b 两档：留着过滤器往返 `PROBE_OK`，撤掉过滤器
往返 `TypeError ... 'l78_probe'`。
"""

import ast
import dataclasses
import inspect
import logging
import textwrap

import pytest
import yaml

from augmentor import config as config_module
from augmentor import logging_setup
from augmentor.config import (MODEL_CREDENTIAL_KEYS, MODEL_ENTRY_KEYS, ModelConfig,
                              _reject_null_fields, load_config, save_config)


@pytest.fixture(autouse=True)
def isolate_root_logger():
    """把 root logger 的 handler 与级别隔离在单条用例内（口径照 `test_section_registry_l77.py`）

    本文件不是日志测试，但 `TestWriterNeverEmitsARefusedKey` 里那条真往返要走
    `load_config(path)`，而加载器会调 `apply_logging_config` 当场装配 root logger
    （A97 定的产品行为）⇒ 不隔离就会污染后面的模块。
    """
    root = logging.getLogger()
    saved_handlers, saved_level = root.handlers[:], root.level
    for handler in logging_setup._INSTALLED[:]:
        handler.close()
    logging_setup._INSTALLED.clear()
    logging_setup._APPLIED = None
    root.handlers[:] = []
    yield
    for handler in logging_setup._INSTALLED[:]:
        handler.close()
    logging_setup._INSTALLED.clear()
    root.handlers[:] = saved_handlers
    root.setLevel(saved_level)
from augmentor.config_validator import ConfigValidator


@dataclasses.dataclass
class _DerivedNull:
    """探针：一个 `init=False` 且值为 `None` 的字段 —— 用户写不出它，但 asdict 会写进文件"""
    kept: int = 1
    derived: int = dataclasses.field(init=False, default=None)


@dataclasses.dataclass
class _WrittenNull:
    """探针：一个 `init=True` 且值为 `None` 的字段 —— 这才是本判据要抓的那个笔误"""
    kept: int = 1
    broken: int = None


class TestNullPolicyOnlyJudgesWritableFields:
    """第 3 处：整节推导那一支从此只判「用户写得出」的字段"""

    def test_a_non_init_null_is_not_reported_as_a_phantom_typo(self):
        """非 init 字段的 `None` 不再冒充「配置里写了键却没给值」（**改前红**）

        反空转两半都在：先用**点名那一支**（`names=("derived",)`）跑出改前的判决，
        证明这个字段确实是 `None`、确实会被那条判据抓住 —— 也就是说本用例救回来的是
        「判据走错了字段集」这件事，不是「探针本身没有空值」。

        第二半顺带是本文件里接住 m8（过滤误套到点名那一支）的**唯一**一条：
        `Temp/l78q/inject_l78.json` 的 m8 档红名只有本条。
        """
        _reject_null_fields("probe", _DerivedNull())          # 不抛 = 本用例的断言
        with pytest.raises(config_module.DataValidationError) as exc:
            _reject_null_fields("probe", _DerivedNull(), ("derived",))
        assert "probe.derived 不能是 null" in str(exc.value)

    def test_an_init_null_still_raises_the_same_words(self):
        """加了过滤，判据本身一个字没改

        消息逐字钉住（不是只断言「抛了什么异常」）：这一族的价值就在于告诉用户
        「你写了这个键却没给值」，措辞漂了等于把 L51 那批用例的归因抹掉。
        """
        with pytest.raises(config_module.DataValidationError) as exc:
            _reject_null_fields("probe", _WrittenNull())
        assert str(exc.value) == "probe.broken 不能是 null（配置里写了这个键却没有给值）"

    def test_the_named_whitelist_branch_is_untouched(self):
        """点名那一支（`names=`）行为一字不改 —— 它不由字段集推导

        本条在改前也绿，是本文件的**白名单护栏**（注入对照里显式豁免）。它接住的变异
        是实测出来的，不是想象出来的：m8（把 `init` 过滤也套到点名那一支）红的是
        **上一条**用例的第二半，本条当时是绿的（探针类两个字段全是 init）；真正把本条
        打红的是 m8b（`names=` 整支变空操作，`Temp/l78q/inject_l78.json` 的 m8b 档）。
        所以本条钉的是「点名一支仍然判」这层语义 —— `ModelConfig.__post_init__` 的采样
        三键判据走的正是它（L72 / A113），那一支空掉等于运行时判据整块消失。
        """
        with pytest.raises(config_module.DataValidationError) as exc:
            _reject_null_fields("probe", _WrittenNull(), ("kept", "broken"))
        assert "probe.broken" in str(exc.value)


class TestModelKeySetHasOneAuthority:
    """第 1、2 处：加载侧那份键集认 `init`，反馈侧引它而不是再推一遍"""

    @staticmethod
    def _assignment_source(module, target):
        """取出 `target = <表达式>` 那条赋值的 AST（找不到就自杀，别让守卫空转）"""
        for node in ast.walk(ast.parse(inspect.getsource(module))):
            if (isinstance(node, ast.Assign)
                    and any(isinstance(t, ast.Name) and t.id == target
                            for t in node.targets)):
                return node
        raise AssertionError("赋值语句 %s 不在了" % target)

    def test_the_loader_derivation_filters_init(self):
        """`MODEL_ENTRY_KEYS` 的推导式里必须真的读 `.init`（**改前红**）

        为什么断言源码形状而不是断言值：今天非 init 字段 0 个，两种写法算出来的集合
        **完全相同** ⇒ 任何值层面的等式断言都是永绿的（承 L32「缓存类护栏必须双向钉」
        的同族道理：等值证不了判据在不在）。
        """
        node = self._assignment_source(config_module, "MODEL_ENTRY_KEYS")
        reads_init = [ast.unparse(n) for n in ast.walk(node.value)
                      if isinstance(n, ast.Attribute) and n.attr == "init"]
        assert reads_init, ast.unparse(node)

    def test_the_validator_reads_that_constant_instead_of_rederiving(self):
        """反馈侧那行必须引 `MODEL_ENTRY_KEYS`，且不许再自己 `fields(ModelConfig)`（**改前红**）

        这是「一份权威」的机械形式：只要还有第二处 `fields(ModelConfig)`，`init`
        判据就要在两处各写一遍、各漂一次 —— L77 在节面上刚为这句话付过账。
        """
        body = ast.parse(textwrap.dedent(inspect.getsource(
            ConfigValidator._warn_unread_model_keys))).body[0]
        names = {n.id for n in ast.walk(body) if isinstance(n, ast.Name)}
        assert "MODEL_ENTRY_KEYS" in names, sorted(names)
        rederived = [ast.unparse(c) for c in ast.walk(body)
                     if isinstance(c, ast.Call)
                     and getattr(c.func, "id", "") == "fields"]
        assert rederived == [], rederived

    def test_the_constant_equals_the_init_field_names(self):
        """值层面：那份键集就是 `ModelConfig` 全部可传入字段，一个不多一个不少

        改前也绿（非 init 字段 0 个），留在本文件是**正向钉**：上面两条结构守卫证明
        「判据在场」，这一条证明「判据没把该收的键挡掉」。

        「方向反了（`not f.init`）能不能被接住」这一维是量出来的，不是推理的：今天
        产品类里没有非 init 字段，`not f.init` 算出来是**空集**，于是变异当场把
        `tests/conftest.py:71` 导入的 `api.main` 打在模块级 `load_config` 上
        （`models.ernie.type 缺少必填字段`），实测整批 632 例 collect 期 error、
        0 条 FAILED（`Temp/l78q/inject_l78.json` 的 m2 档）—— 大声但没有单条归因。
        本条的牙齿由 m7b 证：非 init 字段在场时 `MODEL_ENTRY_KEYS` 多出一个键，
        本等式当场红（同工件的 m7b 红名含本条）。
        """
        assert sorted(MODEL_ENTRY_KEYS) == sorted(
            f.name for f in dataclasses.fields(ModelConfig) if f.init)
        assert len(MODEL_ENTRY_KEYS) == 9, sorted(MODEL_ENTRY_KEYS)


class TestNoConfigClassHasANonInitField:
    """棘轮：22 个配置类里不许出现非 init 字段 —— 本轮从「20 节」扩到整个 `config` 模块

    与 `tests/unit/test_section_registry_l77.py::TestSectionFieldShapes
    ::test_no_section_has_a_non_init_field` 的关系是**覆盖面扩张而不是重复**：那条
    逐节点名 `SECTION_CLASSES`（不含 `ModelConfig`，因为 `models` 是 dict 不是节），
    本条从模块里现推所有 dataclass，所以 `ModelConfig` 与以后新增的任何类都在网里。
    """

    @staticmethod
    def _module_dataclasses():
        return sorted((obj.__name__, obj) for obj in vars(config_module).values()
                      if dataclasses.is_dataclass(obj) and isinstance(obj, type))

    def test_no_config_dataclass_has_a_non_init_field(self):
        non_init = [(cls.__name__, f.name)
                    for _name, cls in self._module_dataclasses()
                    for f in dataclasses.fields(cls) if not f.init]
        assert non_init == [], non_init

    def test_the_ratchet_sees_every_section_class_and_the_model_class(self):
        """反空转：证明上一条真的看见了 22 个类，而不是遍历了一个空集合

        本仓为「计数器没生效」付过两次账（L14、L17），所以这类「遍历 + 断言空」的
        棘轮必须自带一份被遍历者的清单。
        """
        from augmentor.config import AppConfig
        base = AppConfig()
        found = dict(self._module_dataclasses())
        # 「20 节 + ModelConfig + AppConfig 本体」= 22，本模块的 dataclass 全景
        expected = {"ModelConfig", "AppConfig"} | {
            type(getattr(base, f.name)).__name__
            for f in dataclasses.fields(AppConfig)
            if dataclasses.is_dataclass(getattr(base, f.name))}
        assert len(expected) == 22, sorted(expected)   # 20 节 + ModelConfig + AppConfig
        missing = sorted(expected - set(found))
        assert missing == [], missing
        assert len(found) == 22, sorted(found)


class TestTheInitFilterIsFree:
    """`_reject_null_fields` 那一支的过滤必须待在「即将抛错」之后 —— 这是量出来的位置

    同一个判据有三种写法，实测差额极大（同进程交替，`AppConfig()` 每趟加载要走 6 次
    本函数；改后连两跑，`Temp/l78q/perf_a126.json` 的 `runs` 最后两条）：循环前建
    init 名单慢 +15.98 % / +16.18 %、循环第一条语句上查表慢 +6.23 % / +6.65 %、判据挪到
    「值确实是 `None`」之后 −2.14 % / −0.38 %（打平，不宣称每轮更快）。所以这里钉的是
    **结构位置**而不是墙钟（承 L12：墙钟预算用例
    不是性能缺陷的可靠预言机，计数与形状才是），两条合起来把三种写法分开：

    - 循环之前不许出现任何推导式（挡「先建名单」那一版）；
    - `.init` 必须在循环体内、且不在循环体第一条语句上（挡「开头就查表」那一版；
      也挡「根本不查」的改前形态）。
    """

    def test_the_init_test_sits_after_the_none_check_not_in_the_loop_head(self):
        parsed = ast.parse(textwrap.dedent(inspect.getsource(_reject_null_fields)))
        fdef = parsed.body[0]
        loops = [n for n in ast.walk(fdef) if isinstance(n, ast.For)]
        assert len(loops) == 1, [ast.unparse(n) for n in loops]
        loop = loops[0]
        head = fdef.body[:fdef.body.index(loop)]
        built_containers = [ast.unparse(n) for stmt in head
                            for n in ast.walk(stmt)
                            if isinstance(n, (ast.ListComp, ast.SetComp,
                                              ast.DictComp, ast.GeneratorExp))]
        assert built_containers == [], built_containers

        in_loop = [n for n in ast.walk(loop)
                   if isinstance(n, ast.Attribute) and n.attr == "init"]
        assert in_loop, ast.unparse(loop)
        first_reads_init = [n for n in ast.walk(loop.body[0])
                            if isinstance(n, ast.Attribute) and n.attr == "init"]
        assert first_reads_init == [], ast.unparse(loop.body[0])


class TestPerKeyNameListsAreConsumable:
    """两份**按键名点**的清单也必须落在同一份键集里（第 1 处的两个孪生面）

    它们不是键集权威（不决定「哪些键进构造器」），所以 `init` 过滤不加在它们身上；
    但一个非 init 字段一旦出现在这两份清单里，症状分别很难看：静态规格会去判一个
    加载侧根本不消费的键，`${ENV}` 解析会去改一个马上被丢弃的值。
    """

    def test_every_model_entry_spec_key_is_consumable(self):
        ghosts = sorted(set(ConfigValidator.MODEL_ENTRY_FIELDS) - set(MODEL_ENTRY_KEYS))
        assert ghosts == [], ghosts

    def test_every_credential_key_is_consumable(self):
        ghosts = sorted(set(MODEL_CREDENTIAL_KEYS) - set(MODEL_ENTRY_KEYS))
        assert ghosts == [], ghosts


class TestWriterNeverEmitsARefusedKey:
    """A126 的**行为面常驻版**：写出面的键集必须落在加载面认下的那份键集里

    A123 / L77 的先例是 `test_a_non_init_key_is_dropped_instead_of_crashing_the_loader`
    （节面用本地探针类）。模型面照搬不了那个写法：`_model_entry` 绑死了
    `ModelConfig`，探针类进不去。于是这里换成**这条不变量本身** —— 症状要发生，必须
    先有「`save_config` 写出一个加载侧拒绝的键」，而那正是「asdict 不看 `init`」与
    「键集只认 init 字段」两面相撞的地方。今天它成立（实测写出 7 个键，全在键集里），
    一旦有人给 `ModelConfig` 加非 init 字段，`asdict` 会把那个键写出去而
    `MODEL_ENTRY_KEYS` 不收 ⇒ 本条当场红，并报出那个键名（沙箱侧同一形状的崩溃见
    `Temp/l78q/inject_l78.json` 的 m7b 档）。
    """

    @staticmethod
    def _written_models(tmp_path):
        cfg = config_module.AppConfig()
        cfg.models = {"probe": ModelConfig(type="openai", model="m")}
        path = tmp_path / "c.yaml"
        save_config(cfg, str(path))
        return yaml.safe_load(path.read_text(encoding="utf-8"))["models"]

    def test_saved_entries_carry_no_key_the_loader_refuses(self, tmp_path):
        models = self._written_models(tmp_path)
        entries = {k: v for k, v in models.items() if isinstance(v, dict)}
        assert entries, models                       # 反空转：写出面确实写了条目
        refused = {name: sorted(set(body) - set(MODEL_ENTRY_KEYS))
                   for name, body in entries.items()}
        assert {n: k for n, k in refused.items() if k} == {}, refused
        # 反空转第二半：判据不是永绿，因为写出面确实写了 7 个键（凭据两键按「密钥不落盘」
        # 的规则被 pop 掉，见 `save_config` 里那个 `for secret in (...)`）
        assert len(entries["probe"]) >= 7, sorted(entries["probe"])

    def test_the_saved_file_loads_back_with_every_field_present(self, tmp_path):
        """往返必须活着，且落回的对象就是那 9 个字段（不多不少）

        这一条是 m7 档的常驻对照：留着 `if f.init` 时往返 `PROBE_OK`，撤掉它时
        `TypeError ... 'l78_probe'` —— 本用例把「活着」这一半钉进常跑套件。
        """
        path = tmp_path / "c.yaml"
        cfg = config_module.AppConfig()
        cfg.models = {"probe": ModelConfig(type="openai", model="m")}
        save_config(cfg, str(path))
        back = load_config(str(path))
        assert sorted(vars(back.models["probe"])) == sorted(MODEL_ENTRY_KEYS)
        assert back.models["probe"].type == "openai"
