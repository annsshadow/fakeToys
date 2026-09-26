r"""L81 守卫：热路径不得再有「字面量模式的 `re.<fn>`」调用，且常量化必须逐字同串。

**为什么立这一条**：`re.sub(r'\s+', ...)` 看着无害，实际每次调用都要走一遍
`re._compile()` 的缓存查找（字典 + `type()` 判断 + 锁）。放在**逐条记录**的循环里它就是
纯浪费：真实语料 6902 条跑一遍「分析 + 清洗 + 校验 + 统计」电池实测 `re._compile()`
被调 **129,057 次**，加上「建倒排索引 / 逐条词频 / 换行规范化」三支残余 **27,608 次**
（站点现量见 `Temp/l81q/recompile_census.json` 与 `Temp/l81q/before2_census.json`），
L81 改完 **0 次**。这类退化不会让任何既有用例变红 —— 它只是让产品变慢 —— 所以要有一条
形状守卫，否则下一轮谁在循环里新写一处字面量 `re.findall` 也没有人出声。

**口径**：
1. 只管「第一实参是**字符串字面量**」的 `re.sub/findall/split/search/match/fullmatch`；
   传已 `re.compile` 的常量不算（那正是本轮要求的形状）。
2. 覆盖三个出厂面 `augmentor/`、`api/`、`cli.py`，判据是**硬 0** 而不是棘轮：本轮已把
   全包残额清零，留棘轮等于允许「以后再犯」。
3. `scripts/` 不在口径内（一次性工具，实测 1 处且不在逐条循环里）。
4. **常量必须与被替换掉的那处字面量逐字同串**（`PINNED_PATTERNS`）。L81 的字节级改写
   脚本真的写出过两行丢字符的模式（丢了取反 `^` 会把整档正常字符全删），只数行数和行尾
   抓不到这一类反转语义的错，所以这里既钉 `.pattern` 又跑一遍公开入口的行为对账。
5. 期望串一律用 `chr(92)` 现拼：编辑/写入工具的入参会把 `\u4e00` 解成真字符，那样钉的就
   是「语义相同但字面不同」的第四种写法，读不出它钉的是源码里哪一处。
6. 反空转：守卫必须真的扫到现量个 `.py`、钉住 13 条常量（路径写错 ⇒ 扫了空目录 ⇒ 恒绿，
   就靠这一条拦住）。
"""
import ast
import pathlib
import re

import pytest

ROOT = pathlib.Path(__file__).resolve().parents[2]
REGEX_FNS = {"sub", "findall", "split", "search", "match", "fullmatch"}

#: 出厂面（与覆盖率口径 `--cov=augmentor|api|cli` 同三支）
TARGETS = ("augmentor", "api", "cli.py")

#: 现量（L81 收尾）：三支出厂面的 `.py` 文件数 = 104 + 21 + 1
MIN_PY_FILES = 126
MIN_PINNED = 13

B = chr(92)                     # 反斜杠，见模块 docstring 第 5 条
R9FFF = B + "u4e00-" + B + "u9fff"          # 范围本体（不含方括号）
R9FA5 = B + "u4e00-" + B + "u9fa5"
TOKENS = "[" + R9FFF + "]+|[a-zA-Z]+|" + B + "d+"      # 词法切分（中文/英文/数字）
SENTENCES = "[" + "。！？.!?" + "]+"
SPECIAL = "[^" + R9FFF + B + "w" + B + "s.,!?;:" + "、。，！？；：" \
    + B + "u201c" + B + "u201d" + B + "u2018" + B + "u2019" + "（）" \
    + B + "[" + B + "]" + "【】]"
REPEATED_PUNCT = "([" + "。！？.!?" + "])" + B + "1+"
CJK_WORD = "[" + R9FA5 + "]+"                        # sampler / visualizer 那支（无英文档）
KEYWORDS = "[" + R9FFF + "]+|[a-zA-Z]+"

#: `(文件, 常量名) -> 它在 L81 之前替换掉的那处字面量`（逐字，含转义写法）
PINNED_PATTERNS = {
    ("augmentor/analytics.py", "_TOKEN_PATTERN"): TOKENS,
    ("augmentor/analytics.py", "_SENTENCE_PATTERN"): SENTENCES,
    ("augmentor/cleaner.py", "WHITESPACE_PATTERN"): B + "s+",
    ("augmentor/cleaner.py", "SPECIAL_CHAR_PATTERN"): SPECIAL,
    ("augmentor/cleaner.py", "REPEATED_PUNCTUATION_PATTERN"): REPEATED_PUNCT,
    ("augmentor/cleaner.py", "KEYWORD_PATTERN"): KEYWORDS,
    ("augmentor/data/cleaner.py", "CRLF_PATTERN"): B + "r" + B + "n?",
    ("augmentor/data/cleaner.py", "NEWLINE_RUN_PATTERN"): B + "n{3,}",
    ("augmentor/sampler.py", "_CJK_WORD_PATTERN"): CJK_WORD,
    ("augmentor/search_enhanced.py", "_TOKEN_PATTERN"): TOKENS,
    ("augmentor/statistics.py", "_TOKEN_PATTERN"): TOKENS,
    ("augmentor/validation.py", "_WHITESPACE_PATTERN"): B + "s+",
    ("augmentor/visualizer.py", "_CJK_WORD_PATTERN"): CJK_WORD,
}
assert len(PINNED_PATTERNS) == MIN_PINNED

#: 对抗文本：空白/换行/连续标点/全角括号/零宽/BOM/数字混排/空串/纯符号
TEXTS = [
    "",
    "  ",
    "你好　世界",
    "multiple   spaces\tand\nnewline\r\nhere",
    "真的？？？非常非常好好好！！！啊啊啊",
    "价格（含税）【已确认】",
    "混合 English 2024 中文 3.5",
    "emoji🙂与​零宽﻿字符",
    "<html>&amp; 特殊@#$%^&*()_+-=",
    "，。；：！？“‘’”（）",
    "a",
    "1234567890",
    "a\r\n\r\n\r\n\r\nb",
]


def py_files(target):
    p = ROOT / target
    return sorted(p.rglob("*.py")) if p.is_dir() else [p]


def literal_regex_calls(target):
    """[(相对路径, 行号, 方法名)]：该出厂面里第一实参为字符串字面量的 `re.<fn>` 调用。"""
    hits = []
    for path in py_files(target):
        tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
        for node in ast.walk(tree):
            if (isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute)
                    and isinstance(node.func.value, ast.Name)
                    and node.func.value.id == "re" and node.func.attr in REGEX_FNS
                    and node.args and isinstance(node.args[0], ast.Constant)
                    and isinstance(node.args[0].value, str)):
                hits.append((str(path.relative_to(ROOT)).replace("\\", "/"),
                             node.lineno, node.func.attr))
    return hits


def module_constants(rel):
    """{常量名: 模式串}：某产品源码文件里模块级 `NAME = re.compile(字面量)`。"""
    tree = ast.parse((ROOT / rel).read_text(encoding="utf-8"), filename=rel)
    out = {}
    for node in tree.body:
        if not (isinstance(node, ast.Assign) and len(node.targets) == 1
                and isinstance(node.targets[0], ast.Name)):
            continue
        call = node.value
        if (isinstance(call, ast.Call) and isinstance(call.func, ast.Attribute)
                and isinstance(call.func.value, ast.Name)
                and call.func.value.id == "re" and call.func.attr == "compile"
                and call.args and isinstance(call.args[0], ast.Constant)
                and isinstance(call.args[0].value, str)):
            out[node.targets[0].id] = call.args[0].value
    return out


# --------------------------------------------------------------------------- #
# ① 形状守卫 + 同串对账
# --------------------------------------------------------------------------- #
@pytest.mark.parametrize("target", TARGETS)
def test_no_literal_regex_calls_in_shipped_code(target):
    assert literal_regex_calls(target) == []


def test_guard_is_not_vacuous():
    """反空转：守卫真扫到成百个文件、真钉住十几条非空常量。"""
    scanned = sum(len(py_files(t)) for t in TARGETS)
    assert scanned >= MIN_PY_FILES, scanned
    assert all(pattern for pattern in PINNED_PATTERNS.values())
    assert len(PINNED_PATTERNS) >= MIN_PINNED


@pytest.mark.parametrize("rel,name", sorted(PINNED_PATTERNS))
def test_pinned_constant_is_the_same_string(rel, name):
    """常量的模式字面量必须与被替换掉的那一处**逐字**相同（防常量化时抄错）。"""
    got = module_constants(rel).get(name)
    assert got == PINNED_PATTERNS[(rel, name)], (rel, name, repr(got))


# --------------------------------------------------------------------------- #
# ② 行为对账：走公开入口，与「内联字面量参考实现」比结果
# --------------------------------------------------------------------------- #
def test_text_normalizer_matches_inline_literal_reference():
    from augmentor.cleaner import TextNormalizer

    normalizer = TextNormalizer()
    for text in TEXTS:
        if not text:
            assert normalizer.normalize(text) == text
            continue
        want = re.sub(B + "s+", " ", text.strip())
        want = re.sub("([" + "。！？.!?" + "])" + B + "1+", B + "1", want)
        assert normalizer.normalize(text) == want, text


def test_clean_rules_match_inline_literal_reference():
    from augmentor.cleaner import DatasetCleaner

    cleaner = DatasetCleaner()
    for text in TEXTS:
        got, _ = cleaner._normalize_whitespace([{"instruction": text}], ["instruction"])
        assert got[0]["instruction"] == re.sub(B + "s+", " ", text), text

        got, _ = cleaner._remove_special_chars([{"instruction": text}], ["instruction"])
        assert got[0]["instruction"] == re.sub(SPECIAL, "", text), text


def test_extract_keywords_matches_inline_literal_reference():
    from augmentor.cleaner import extract_keywords

    for text in TEXTS:
        freq = {}
        for word in re.findall(KEYWORDS, text):
            if len(word) > 1:
                freq[word] = freq.get(word, 0) + 1
        ordered = sorted(freq.items(), key=lambda x: x[1], reverse=True)
        assert extract_keywords(text, top_k=50) == [w for w, _ in ordered[:50]], text


def test_sanitize_item_matches_inline_literal_reference():
    from augmentor.validation import DataSanitizer

    sanitizer = DataSanitizer()
    keep_control = {chr(10), chr(13), chr(9)}
    for text in TEXTS:
        got = sanitizer._sanitize_item({"instruction": text, "output": "答案"}, fix=True)
        want = re.sub(B + "s+", " ", text.strip())
        want = "".join(c for c in want if c.isprintable() or c in keep_control)
        if want:
            assert got is not None and got["instruction"] == want, (text, got)
        else:
            assert got is None, (text, got)


def test_tokenize_and_sentences_match_inline_literal_reference():
    from augmentor.analytics import DatasetAnalyzer
    from augmentor.search_enhanced import EnhancedSearcher

    analyzer = DatasetAnalyzer([])
    searcher = EnhancedSearcher([])
    for text in TEXTS:
        assert analyzer._tokenize(text) == re.findall(TOKENS, text), text
        assert searcher._tokenize(text) == re.findall(TOKENS, text), text
        parts = re.split(SENTENCES, text)
        assert analyzer._get_sentences(text) == [s.strip() for s in parts if s.strip()], text


def test_statistics_tokens_match_inline_literal_reference():
    from augmentor.statistics import _TOKEN_PATTERN

    for text in TEXTS:
        assert _TOKEN_PATTERN.findall(text) == re.findall(TOKENS, text), text


def test_word_frequency_sites_use_the_narrower_range():
    """`sampler` / `visualizer` 钉的是 `\u4e00-\u9fa5`，与统计那支 `\u4e00-\u9fff` 不同串。

    两档的差别落在 `龥`(U+9FA5) 与 `鿿`(U+9FFF) 之间的字上，故这里用边界字符显式验一次，
    防止哪天「顺手统一」成同一份常量而改掉词频口径。
    """
    from augmentor.sampler import _CJK_WORD_PATTERN as sampler_pattern
    from augmentor.visualizer import _CJK_WORD_PATTERN as visualizer_pattern

    probe = chr(0x9FA5) + chr(0x9FA6) + chr(0x9FFF) + "a"
    for pattern in (sampler_pattern, visualizer_pattern):
        assert pattern.findall(probe) == [chr(0x9FA5)], pattern.pattern
    from augmentor.statistics import _TOKEN_PATTERN

    assert _TOKEN_PATTERN.findall(probe) == [probe[:3], "a"], _TOKEN_PATTERN.pattern


def test_newline_normalization_matches_inline_literal_reference():
    from augmentor.data.cleaner import FULLWIDTH_PUNCTUATION, DataCleaner

    cleaner = DataCleaner()
    for text in TEXTS:
        want = text
        for full, half in FULLWIDTH_PUNCTUATION.items():
            want = want.replace(full, half)
        want = re.sub(B + "r" + B + "n?", B + "n", want)
        want = re.sub(B + "n{3,}", B + "n" + B + "n", want)
        want = re.sub("[" + B + "t " + chr(0x3000) + "]+", " ", want)
        assert cleaner.normalize_format(text) == want.strip(), text


# --------------------------------------------------------------------------- #
# ③ 单趟合并计数的等价性（analytics 质量分：两趟走查改成一趟）
# --------------------------------------------------------------------------- #
def two_pass_quality(items):
    """HEAD 形状的两趟实现，作为单趟合并版的参照。"""
    if not items:
        return 0.0
    score = 1.0
    complete_items = sum(1 for item in items
                         if item.get("instruction") and item.get("output"))
    score *= complete_items / len(items)
    consistent_items = 0
    for item in items:
        instruction = item.get("instruction", "")
        output = item.get("output", "")
        if instruction and output:
            if len(set(instruction) & set(output)) / max(len(set(instruction)), 1) < 0.8:
                consistent_items += 1
    score *= (0.5 + 0.5 * consistent_items / len(items))
    return min(max(score, 0.0), 1.0)


QUALITY_FIXTURES = [
    [],
    [{"instruction": "", "output": ""}],
    [{"instruction": "租房合同", "output": "租房合同"}],       # 问题与回答字符集全同 ⇒ 判不一致
    [{"instruction": "ab", "output": "ba"}],                   # 重合 1.0
    [{"instruction": "ab", "output": "cd"}],                   # 重合 0
    [{"instruction": "只有问题没有答案"}],                       # 缺 output ⇒ 只扣完整性
    [{"instruction": "x" * 30, "output": "y" * 30}],
    [{"instruction": "你好", "output": "世界"}, {"instruction": "a", "output": "b"},
     {"instruction": "", "output": "c"}, {"instruction": "d"}, {"output": "e"}],
    [{"instruction": None, "output": "x"}, {"instruction": 0, "output": "y"}],
]


@pytest.mark.parametrize("items", QUALITY_FIXTURES)
def test_single_pass_quality_score_equals_two_pass_reference(items):
    from augmentor.analytics import DatasetAnalyzer

    assert DatasetAnalyzer(items)._calculate_quality_score() == two_pass_quality(items)
