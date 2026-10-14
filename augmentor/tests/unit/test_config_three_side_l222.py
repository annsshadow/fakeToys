# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L222（A275）：配置面的**三侧同源对账**——`docs/README.md` 表格 / `AppConfig` / `config.yaml`

**为什么需要这一把**：L220 新增 `evaluation.generated_field` 时，README 的配置表与
`config.yaml` 两处全靠人肉同步，而 `test_doc_contract_numbers_l200.py` 只核端点数、导出格式数、
页面数那一类数字，**逐节键清单没有任何机器守**。本轮用最终版尺子对 HEAD 那份文档量出的真漂移是
**21 红 / 38 绿**：表里只有 16 行、schema 节覆盖 34 只键，`context` / `expander` / `sampler` /
`tracker` / `visualization` **5 整节缺席**，13 节的键清单只写了一半上下，恰好列全的只有
`export` 与 `multilingual` 两节。

**`config.yaml` 那一侧没有漂移，而且它教本尺子改了一次判据**：第一版把「文件里没有 `logging`
节」当成漂移，实测被文件自己那段注释推翻——那是**刻意的产品决策**（整节不存在时 CLI 与 API 的
日志一字不变，写出来才装配）。所以 yaml 侧的判据不是「与 schema 逐节全等」，而是
**「要么整节写全，要么整节注释列全」**：一个节可以整个不落地，但不许只写半节、不许写幽灵键、
也不许新增一只键而文件里连提都没提。三条都是机器判的。

**三侧而不是两两**：只比 README↔schema 会放过「文档说有了但出厂文件里没有」，只比
schema↔yaml 会放过「文档少了整整一节」。三条读数各自与 `fields()` 推出来的那份键集全等，
两两自然一致。

**权威只有一份**：节清单与键清单全部从 `fields(AppConfig)` 与 `getattr` 出来的节实例推，
和 `test_section_registry_l77.py` 同一口径——测试侧再抄一份清单就是第二个权威，正是 A123
那一族病的形状。本文件里唯一手写的名单是 `META_ROWS`（`models` / `app`），而它每一员都得
**同时**过两条机器验真（既不是 dataclass 节、又在 `ConfigValidator.META_TOP_SECTIONS` 白名单里），
所以「把一个真节塞进例外名单」蒙不过去。

**`app` 那一格不是本轮发明的分类，也不是新事实**：`app.name` / `app.version` / `app.debug`
没有产品读者这件事，仓内早记在三处（`OPTIMIZATION_PLAN.md` §2.4、`README.md` 变更说明第 6 条、
`tests/integration/test_cli_validate_config.py` 的 docstring），历史处置是把它们从必填**降为可选**。
本轮新增的只有「它对只覆盖 schema 键的死键普查**结构上不可见**」这一格：`AppConfig` 没有 `app`
字段，而校验器把它列进 `META_TOP_SECTIONS` 免检，于是零读者名单永远数不到它。处置（接成节 /
删掉 / 维持并写明）另立 A276，本轮不顺手做；本文件的验真臂把这个形状钉在明处：白名单成员必须
**确实不是节**，而不是「我们记得它不是节」。
"""

import dataclasses
import re
from pathlib import Path

import pytest
import yaml

from augmentor.config import AppConfig
from augmentor.config_validator import ConfigValidator

AI_DIR = Path(__file__).resolve().parent.parent.parent
#: 子项目文档已于 2026-10-14 汇总到仓库根 `docs/augmentor/`；本文件自持这一真源
DOCS_DIR = AI_DIR.parent / "docs" / "augmentor"

#: 不是 dataclass 节、但确实出现在 README 表格与 `config.yaml` 里的顶格名字。
#: 每一员由 `TestMetaRosterIsNotAnEscapeHatch` 机器验真，见模块 docstring。
META_ROWS = frozenset({"models", "app"})

_ROW = re.compile(r"^\|\s*`([A-Za-z_][A-Za-z0-9_]*)`\s*\|")
_BACKTICK = re.compile(r"`([^`]+)`")

#: 配置表的表头锚点。只在这一张表里取行，README 别处的表（接口清单、命令清单）
#: 一样是「第一格反引号包标识符」的形状，不锚就会把它们当成配置节报成幽灵节。
_HEADER_MARK = "| 配置段 |"


def _section_classes() -> dict:
    """节名 → 该节的 dataclass 类型（同 `test_section_registry_l77.py` 的推法）"""
    baseline = AppConfig()
    return {f.name: type(getattr(baseline, f.name))
            for f in dataclasses.fields(AppConfig)
            if dataclasses.is_dataclass(getattr(baseline, f.name))}


def commented_section_keys(text: str, section: str) -> set:
    """该节「整节被注释掉」时，注释块里列出的键集（没有这种块就返回空集）

    取**最后**一处 `# <section>:` 起向后扫：文件里那段注释先是「最小写法」的示例，
    再是真正被注释掉的整节，只有后者才是完整清单。示例那处即便被扫到，键集也仍是
    同一份的子集，所以判据不受影响；取最后一段是为了读数不含糊。
    """
    lines = text.splitlines()
    header = re.compile(rf"^#\s*{re.escape(section)}:\s*$")
    starts = [i for i, line in enumerate(lines) if header.match(line.strip())]
    if not starts:
        return set()
    keys = set()
    key_re = re.compile(r"^#\s+([A-Za-z_][A-Za-z0-9_]*):")
    for line in lines[starts[-1]:]:
        stripped = line.strip()
        if stripped and not stripped.startswith("#"):
            break
        match = key_re.match(stripped)
        if match and match.group(1) != section:
            keys.add(match.group(1))
    return keys


def schema_keys() -> dict:
    """节名 → 该节**可写**（init）键集；`init=False` 的键按定义写不进去，不算清单"""
    return {name: {f.name for f in dataclasses.fields(cls) if f.init}
            for name, cls in _section_classes().items()}


def readme_rows(text: str) -> dict:
    """从 README 的配置表解析 `节名 → 键集`

    表头之后的连续表格行才算这张表的成员，遇非表格行即收摊；第一个单元格必须是反引号
    包住的标识符，第二个单元格里所有反引号片段都算键名。重名节直接抛错——漏一行会由
    逐节判据红，重复的一行也必须红。
    """
    rows: dict = {}
    inside = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith(_HEADER_MARK):
            inside = True
            continue
        if not inside:
            continue
        if not stripped.startswith("|"):
            inside = False
            continue
        match = _ROW.match(stripped)
        if not match:
            continue
        cells = [c.strip() for c in stripped.strip("|").split("|")]
        name = match.group(1)
        keys = set(_BACKTICK.findall(cells[1]))
        if name in rows:
            raise AssertionError(f"README 配置表有重复节：{name}")
        rows[name] = keys
    return rows


def yaml_sections(raw: dict) -> dict:
    """`config.yaml` 里「写成映射」的顶格节 → 其键集（标量节不算，它本来就加载期报错）"""
    return {k: set(v) for k, v in raw.items() if isinstance(v, dict)}


def _diff(a: dict, b: dict) -> dict:
    """逐节的 {节: (a 有 b 无, b 有 a 无)}，只保留有差别的节"""
    out = {}
    for name in sorted(set(a) | set(b)):
        left = a.get(name, set()) - b.get(name, set())
        right = b.get(name, set()) - a.get(name, set())
        if left or right:
            out[name] = (left, right)
    return out


SCHEMA = schema_keys()
README = readme_rows((DOCS_DIR / "README.md").read_text(encoding="utf-8"))
RAW_TEXT = (AI_DIR / "config.yaml").read_text(encoding="utf-8")
RAW_YAML = yaml.safe_load(RAW_TEXT)
YAML = yaml_sections(RAW_YAML)


def yaml_side() -> dict:
    """config.yaml 这一侧的读数：写出来的节取实键，整节注释的节取注释里列出的键"""
    return {s: (YAML[s] if s in YAML else commented_section_keys(RAW_TEXT, s))
            for s in SCHEMA}


class TestRulerSeesTheRealSurface:
    """尺子本身的前提：三侧都真的读到了东西，且权威份的量与在册数对得上"""

    def test_schema_side_is_not_empty(self):
        assert len(SCHEMA) >= 20
        assert sum(len(keys) for keys in SCHEMA.values()) == 70

    def test_readme_and_yaml_sides_are_parsed_at_all(self):
        """前提档只判「解析器不是瞎的」这一维；行数的**对错**由下面两类逐节判据负责"""
        assert len(README) >= 5, f"README 表格解析出的行数异常：{sorted(README)}"
        assert len(YAML) >= 20, f"config.yaml 解析出的节数异常：{sorted(YAML)}"


class TestReadmeMatchesSchema:
    """README 的配置表必须逐节列全 schema 的键——「关键项」不做「挑着写」的挡箭牌"""

    def test_every_schema_section_has_exactly_one_row(self):
        assert set(README) - META_ROWS == set(SCHEMA)

    def test_no_phantom_row(self):
        """表里出现一个既不是节、也不在例外名单的名字，就是文档在描述不存在的配置面"""
        assert set(README) <= set(SCHEMA) | META_ROWS

    @pytest.mark.parametrize("section", sorted(SCHEMA))
    def test_this_sections_key_list_is_complete(self, section):
        listed = README.get(section, set())
        assert listed == SCHEMA[section], (
            f"`{section}`：README 多出 {sorted(listed - SCHEMA[section])}、"
            f"漏了 {sorted(SCHEMA[section] - listed)}"
        )


class TestWrittenConfigMatchesSchema:
    """出厂 `config.yaml` 的判据是「整节写全，或整节注释列全」，不是「必须写出来」

    这条判据的形状不是设计出来的，是被文件自己纠正出来的：`logging` 那三键刻意留在注释里
    （整节不存在时 CLI 与 API 的日志一字不变），而 `config.yaml` 明写了这个生效条件。所以
    「不落地」是合法状态，「半节」「幽灵键」「新键没人提」才是漂移。
    """

    def test_every_live_yaml_section_is_a_schema_section(self):
        ghosts = set(YAML) - set(SCHEMA) - META_ROWS
        assert not ghosts, f"config.yaml 写了 schema 里没有的节：{sorted(ghosts)}"

    @pytest.mark.parametrize("section", sorted(SCHEMA))
    def test_this_section_is_either_fully_written_or_fully_commented(self, section):
        expected = SCHEMA[section]
        if section in YAML:
            written = YAML[section]
            assert written == expected, (
                f"`{section}`：config.yaml 多出 {sorted(written - expected)}、"
                f"漏了 {sorted(expected - written)}（半节既不合法也不易读）"
            )
        else:
            commented = commented_section_keys(RAW_TEXT, section)
            assert commented == expected, (
                f"`{section}` 整节没落地，那注释里必须逐键列全它：缺 "
                f"{sorted(expected - commented)}、多 {sorted(commented - expected)}"
            )

    def test_the_commented_sections_are_a_declared_shaped_exception(self):
        """允许「整节注释」的节必须**真的**带一段解释为什么整节不存在才有意义

        本仓对产品路径的口径是「写了就要有人读」，一节整体缺席同样得说明它缺席时行为是什么。
        这条判据查的是文件里有没有那句生效条件的说明，而不是靠人记得那段注释。
        """
        absent = sorted(set(SCHEMA) - set(YAML))
        assert absent, "本臂的正控制要求当前确实存在整节缺席的节"
        for section in absent:
            assert re.search(rf"^#.*{re.escape(section)}", RAW_TEXT, re.M), (
                f"`{section}` 整节缺席却没有任何注释提到它"
            )
        assert "logging" in absent


class TestThreeSidesAgree:
    """把前两类的结论合成一条可行动的读数：同一份 70 键，三侧一个不少"""

    def test_all_three_sides_count_the_same_keys(self):
        written, commented = YAML, yaml_side()
        per_side = {
            "schema": sum(len(k) for k in SCHEMA.values()),
            "README": sum(len(README.get(s, set())) for s in SCHEMA),
            "config.yaml(写出+注释两态)": sum(len(commented[s]) for s in SCHEMA),
        }
        assert per_side == {"schema": 70, "README": 70,
                           "config.yaml(写出+注释两态)": 70}, per_side
        assert written, "yaml 侧解析为空，上一条读数就是恒真的"

    def test_no_section_differs_between_any_two_sides(self):
        assert _diff(SCHEMA, {k: v for k, v in README.items() if k in SCHEMA}) == {}
        assert _diff(SCHEMA, yaml_side()) == {}


class TestMetaRosterIsNotAnEscapeHatch:
    """例外名单只有「确实不是节」才成立，且必须是校验器明写的白名单成员"""

    @pytest.mark.parametrize("name", sorted(META_ROWS))
    def test_this_meta_name_is_really_not_a_section(self, name):
        """例外成员必须**确实不是 dataclass 节**，且必须是校验器明写的白名单成员"""
        baseline = AppConfig()
        assert name not in SCHEMA, f"{name} 是 dataclass 节，不该待在例外名单里"
        assert not dataclasses.is_dataclass(getattr(baseline, name, None))
        assert name in ConfigValidator.META_TOP_SECTIONS, (
            f"{name} 不在校验器的免检白名单里，它就是普通死面，不该由本名单解释"
        )

    def test_app_is_the_one_meta_section_with_no_loader_behind_it(self):
        """`models` 与 `app` 同名册而不同命：前者是 `AppConfig` 的字段，后者**根本没有字段**

        这条是 A276 的机器证据而不是散文：`load_config` 把 `models` 交给模型注册面，
        而 `app.*` 三键写进出厂文件后没有任何归宿，校验器又按白名单免检，所以它在
        「只覆盖 schema 键」的死键普查里结构上看不见。将来若有人把 `app` 接成节或删掉
        那一段，这一臂都会红——它不许两个名字继续被同一格含糊地并在一起。
        """
        field_names = {f.name for f in dataclasses.fields(AppConfig)}
        assert "models" in field_names
        assert "app" not in field_names
        assert set(ConfigValidator.KNOWN_FIELDS) >= {"app", "app.name", "app.version", "app.debug"}

    def test_the_whitelist_has_no_member_that_is_not_declared_here(self):
        """校验器白名单若加员，这里必须跟着加——两存必有一盲，盲的是后加的那个"""
        assert set(ConfigValidator.META_TOP_SECTIONS) <= META_ROWS | set(SCHEMA)


class TestRulerIsNotBlind:
    """反空转：把三侧各自喂一份**改坏过的**输入进同一批比对函数，必须报出差异

    不这样写的风险是「解析器其实一行没解析出来，于是两边都是空集，等式恒成立」。
    四臂分别对应四类真实漂移：整节缺失、键名改名、多写一个不存在的节、少写一只键。
    """

    def test_dropping_a_whole_row_is_caught(self):
        mangled = dict(README)
        mangled.pop("multilingual")
        assert set(mangled) - META_ROWS != set(SCHEMA)

    def test_renaming_a_key_is_caught(self):
        mangled = dict(SCHEMA)
        mangled["evaluation"] = mangled["evaluation"] | {"reference_col"}
        assert _diff(mangled, README)  # 非空：改名后的键在文档侧不存在

    def test_phantom_section_row_is_caught(self):
        mangled = dict(README)
        mangled["pipeline"] = {"enabled"}
        assert not set(mangled) <= set(SCHEMA) | META_ROWS

    def test_yaml_missing_a_key_is_caught(self):
        mangled = {k: set(v) for k, v in YAML.items()}
        mangled["web"] = mangled["web"] - {"max_upload_bytes"}
        assert _diff(SCHEMA, mangled)["web"] == ({"max_upload_bytes"}, set())

    def test_the_commented_scanner_is_not_blind(self):
        """整节注释那一臂的读数来自扫描器，所以扫描器自己也得有正控制：抹掉一键要少一键"""
        keys = commented_section_keys(RAW_TEXT, "logging")
        assert keys == {"level", "file", "format"}
        mangled = "\n".join(l for l in RAW_TEXT.splitlines() if not l.strip().endswith('format: "%(message)s"'))
        assert commented_section_keys(mangled, "logging") == {"level", "file"}
        assert commented_section_keys(RAW_TEXT, "no_such_section") == set()

    def test_the_parser_actually_reads_key_names(self):
        """直接喂一段人造表格，确认解析器认的是反引号片段、且收摊于表外内容"""
        parsed = readme_rows("| 配置段 | 键清单 | 说明 |\n"
                             "|---|---|---|\n"
                             "| `demo` | `alpha` `beta` | 说明 |\n"
                             "\n正文里的 `| gamma` 不该被算进来\n")
        assert parsed == {"demo": {"alpha", "beta"}}

    def test_a_row_outside_the_config_table_is_not_collected(self):
        """README 别处的表（接口清单一类）形状一样，锚点必须把它们挡在外面"""
        parsed = readme_rows("| 别的表 | 第二格 |\n|---|---|\n| `phantom` | `x` |\n"
                             "| 配置段 | 键清单 | 说明 |\n| `demo` | `alpha` |\n")
        assert parsed == {"demo": {"alpha"}}
