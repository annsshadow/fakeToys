"""L79 / A127 守卫：两份活文档里的「定位引用」必须真的指得到东西

**为什么立这一条**：本仓的架构文档与循环账本大量用反引号引用（`` `path/file.py:123` ``）给
读者指路。代码一改行号就漂，漂到极点是引用指向**空行 / 注释 / 已经不存在的文件** —— 读者照
引用跳过去什么也找不到，而这条失败**没有任何机械通道会出声**。L77 / L78 各抓到一次「账本里
写着的 file:line 指向别处」，两次都是人工回读撞上的（A87 的现场样本，见 §3.29 末尾那段勘误），
所以本轮把它变成用例。

**口径（本轮拍定）**：

1. 反引号里的「文件名 + 冒号 + 行号」= **定位引用**，本守卫只管这一种。
2. 要留**历史读数**（「当时在第 46 行」）就写成不带反引号的散文，或写成名字锚点。架构文档里
   原有的 7 处此类例子（§3.27 那段勘误的 4 个数字 + 同一句话在 §3.27 正文与 §6 表的 3 次出现）
   在 L79 就地改成散文；另有 30 处补了目录前缀、账本 22 处改指名字锚点。
3. 「一个名字对应仓里多个真实文件」= **歧义引用**，同样不许（`config.py` 有 `augmentor/config.py`
   与 `api/routes/config.py` 两支，读者无从判断）。
4. 指向**取证脚本**（`Temp/` 下、不入仓）的引用不算活引用：单独计数、只当棘轮。
5. 但 **Temp 里有同名副本不洗白产品路径**：`Temp/l*/head/` 那些 HEAD 快照让
   `` `augmentor/api/routes/dataset_tools.py` `` 这种**从未存在过的路径**也能"找得到文件"，
   所以先按顶层目录判产品路径，再考虑工件（`name_bucket` 的顺序即口径）。
6. **一个桶不许混两类**：可证失效（`dead_line` / `unresolved_source`，硬 0）与已记欠账
   （棘轮档）同住一个桶时，真 bug 会被棘轮的「只降不升」读成正常状态 —— 本轮 4 处产品
   路径写错就是这么藏住的（`scratch_missing` 里 111 个草稿名把它们盖住了）。

**判据一律「只降不升」，改进也得改常数**（承 L76 / L77 的棘轮口径）：`CEILING` 里每条都是
**精确相等**而不是「≤」。所以「顺手清掉一条歧义」同样会让用例红 —— 那是设计：它逼着清账的人
在账本上留一句「谁清的、清完剩多少」。

**残余项**：`code_but_no_name_match` 那一档（行号指着代码行、但被点名的标识符不在该行声明）
本轮**不清**、只量（架构文档 37 处 / 账本 165 处）。它要逐条读原句判断「那句话指的是哪件事」，
属 A127 的后半程。**本守卫的边界（A129）**：散文里的行号它看不见，而本轮实测到「连勘误段自己
写下的那两个位移读数也位移了」⇒ 这一条不在这里修，只在 §3.33 声明。
"""
import ast
import collections
import functools
import os
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]

# 文件名段必须**至少含一个标识符字符**：否则散文里「以 `.py` 结尾」这种话会被当成引用。
FILE_TOKEN = re.compile(r"`([A-Za-z0-9_/\\.\-]*[A-Za-z0-9_]\.py)`")
LINE_REF = re.compile(r"`([A-Za-z0-9_/\\.\-]*[A-Za-z0-9_]\.py):(\d+)(?:-(\d+))?`")
BARE_REF = re.compile(r"`:(\d+)(?:-(\d+))?`")

ARCH = "docs/ARCHITECTURE.md"
LEDGER = "OPTIMIZATION_LOOP.md"
DOCS = (ARCH, LEDGER)

#: L79 就地清账后的基线。键是 `audit()` 的桶名；**只降不升**，见模块 docstring。
#: 现量读数见 `Temp/l79q/lineref_buckets.json` 里 `runs` 的最后一条（命令
#: `PYTHONIOENCODING=utf-8 C:/Python314/python.exe Temp/l79q/buckets.py <label>`）：
#: 架构文档 = 0 / 0 / 0 / 0 / 20 / 29，账本 = 0 / 0 / 69 / 145 / 153 / 111。
#: `unresolved_source` = 「点名的路径第一段是仓内某个装 `.py` 的顶层目录（现量
#: `api` / `archive` / `augmentor` / `scripts` / `tests`），可这个文件不存在」—— 它与
#: 「指向不入仓的 Temp 草稿」必须分家：本轮 4 处**产品源码路径写错**原本被
#: `scratch_missing` 或 Temp 快照副本吞掉（判据太粗 ⇒ 把真失效藏起来），分家后当场可见。
CEILING = {
    ARCH: {"dead_line": 0, "unresolved_source": 0, "ambiguous_line": 0,
           "ambiguous_file": 0, "bare": 20, "scratch_missing": 29},
    LEDGER: {"dead_line": 0, "unresolved_source": 0, "ambiguous_line": 69,
             "ambiguous_file": 145, "bare": 153, "scratch_missing": 111},
}

#: 反空转下界：`line_refs` / `file_tokens` 是**两种语法各自的匹配总数**（不是桶的加和 ——
#: 上一版让两个总数共享 `ambiguous` 那一桶，账本那 214 条歧义引用同时顶在两个总数上）。
#: 下界取整留了余量 —— 这两个数是**好引用的规模**，不是缺陷计数，删引用是改进、不该被
#: 反空转下界卡住；它只负责证明「普查真的看见了成百条引用」，而不是钉住总数。
FLOOR = {
    ARCH: {"line_refs": 40, "file_tokens": 200},
    LEDGER: {"line_refs": 270, "file_tokens": 1050},
}

#: **现量**读数（`Temp/l79q/buckets.py` 的最后一个 run）。凡是在注释、`docs/ARCHITECTURE.md`
#: §3.33 或账本里写「实测 N 处」，那个 N 只许从这份常量取 —— A77「一份权威只住一处」用在
#: **测试自己的注释**上：本轮 §3.33 里「40 处 / 114 处 / 10 例」三个数就是在同一轮内过期的，
#: 因为文档段落写在了判据改完之前。
#:
#: **这一份每轮都要重量**（L79 收尾现场兑现）：账本日志块写完之后，正文自己引用的 7 个仓内文件
#: 与 7 个 Temp 草稿名把 `file_tokens` 从 1075 顶到 1089 —— 与 `CEILING` 里那些「随代码变坏而涨」
#: 的缺陷档不同，这里是**对账位**：它红说的是「文档里的数靠了回忆」，不是「代码变坏了」。
MEASURED = {
    ARCH: {"line_refs": 47, "file_tokens": 213, "code_but_no_name_match": 37},
    LEDGER: {"line_refs": 281, "file_tokens": 1089, "code_but_no_name_match": 165},
}

#: 日志块标题行上「下一轮才填得出自己哈希」的占位符。精确相等 = **只许有该填的那几条**。
#: 实测命中 15 = L58..L71 那 14 条从未回填的历史欠账 + 上一轮（L78）待回填那 1 条；
#: 本轮把 L78 填掉、加上 L79 自己写的，仍是 15 ⇒ 谁忘了回填上一轮，用例当场红。
#: **不能写成 `text.count("哈希待")`**：账本里那个串出现 26 次，多出的 11 次是散文在
#: **谈论**「哈希待回填」这件事（如 L64 那句「本账自身 numstat 待 L65 量」），不是欠账本身。
#: 补那 14 处历史正文要不要做属 A130（待用户拍）。
HASH_PLACEHOLDER = re.compile(r"^- \*\*L\d+\*\* `哈希待 L\d+ 回填`", re.MULTILINE)
PLACEHOLDER_CEILING = 15


@functools.lru_cache(maxsize=1)
def build_index():
    """`{文件名: [仓内相对路径的段元组]}` —— 按名字分桶，查找代价与全仓规模无关

    排除 `.git` / `__pycache__` / `.pytest_cache` / `node_modules`；`Temp/` **保留**，因为
    账本里的取证脚本引用要能被认成「指向工件」而不是「指向不存在的文件」。
    """
    index = collections.defaultdict(list)
    for base, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in (".git", "__pycache__",
                                                ".pytest_cache", "node_modules")]
        for name in files:
            if name.endswith(".py"):
                rel = os.path.relpath(os.path.join(base, name), ROOT)
                index[name].append(tuple(pathlib.PurePath(rel).parts))
    return {k: tuple(sorted(v)) for k, v in index.items()}


def source_dirs(index):
    """仓内**装 `.py` 的顶层目录**（`Temp` 除外）

    用它区分「这条未解析引用是**写错了的产品源码路径**」还是「一个不入仓的草稿名」。
    不硬编目录清单：下一轮新增 `tools/` 时这里自动认得，而写错的 `tolus/foo.py`
    会因为第一段不存在而被归进草稿那一档（宁可漏判成棘轮，不误判成硬门禁）。
    只取 `len(parts) > 1` 的路径 —— 根目录的 `cli.py` 自身会给出 `('cli.py',)`，
    它不是目录，把它当目录会让「`cli.py` 的草稿邻居」这类引用错误地升格成硬失效。
    """
    return frozenset(p[0] for paths in index.values() for p in paths
                     if len(p) > 1 and p[0] != "Temp")


def name_bucket(rel, index, src):
    """引用名 → `(结论桶, 唯一活路径)`；桶为 `None` 才表示「这一名指得到唯一文件」

    **顺序就是口径**（L79 实测教训）：先判「第一段是真实源码目录」，再考虑 Temp 里的同名
    尾巴。反过来的话 `` `augmentor/api/routes/dataset_tools.py` `` 会被 Temp 快照副本
    「洗白」成工件引用 —— 可那个前缀从来不存在（真实路径是 `api/routes/dataset_tools.py`），
    一条**可证失效**就此隐形。

    同理，未解析也不能只留一档：`scratch_missing` 若同时吞「不入仓草稿名」和「产品路径写错」，
    而它是只降不升的棘轮 ⇒ 真 bug 被记成欠账、当场没有任何东西会红（本轮实测到 4 条）。
    """
    live, scratch = resolve(rel, index)
    if len(live) == 1:
        return None, live[0]
    if len(live) > 1:
        return "ambiguous", None
    if rel.replace("\\", "/").split("/")[0] in src:
        return "unresolved_source", None
    return ("scratch" if scratch else "scratch_missing"), None


def resolve(rel, index):
    """文档里写的相对名 → 仓内真实路径，返回 `(活文件, Temp 工件)` 两组；歧义**不猜**"""
    parts = tuple(p for p in rel.replace("\\", "/").split("/") if p)
    live, scratch = [], []
    for rp in index.get(parts[-1], ()):
        if rp[-len(parts):] == parts:
            (scratch if rp[0] == "Temp" else live).append("/".join(rp))
    return sorted(live), sorted(scratch)


@functools.lru_cache(maxsize=None)
def declared_lines(rel):
    """`{行号: 以该行为声明起点的标识符}`，只认 def / class / 赋值（含带注解的赋值）"""
    table = collections.defaultdict(set)
    tree = ast.parse((ROOT / rel).read_text(encoding="utf-8"))
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            names = [node.name]
        elif isinstance(node, ast.Assign):
            names = [t.id for t in node.targets if isinstance(t, ast.Name)]
        elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
            names = [node.target.id]
        else:
            continue
        for n in names:
            table[node.lineno].add(n)
    return dict(table)


def classify(lines, start, end, names_at):
    """`dead_line` 按引用形状分两档 —— 这是**口径**，不是把判据放宽

    单行 `` `f.py:42` `` 的语义是「跳到 42 行」⇒ 那一行必须是代码行。
    区间 `` `f.py:76-78` `` 的语义是「这件事在 76..78 之间」⇒ 只有**整段都不是代码**
    （全空行 / 全注释 / 越界）才算可证失效；起点空、终点是代码的那档属「区间报歪一行」，
    归漂移档（A127 后半程），不该由硬门禁开火。
    """
    last = start if end is None else max(end, start)
    span = []
    for ln in range(max(start, 1), last + 1):
        if ln > len(lines):
            span.append("out-of-range")
            continue
        raw = lines[ln - 1].strip()
        span.append("blank" if not raw
                    else "comment" if raw.startswith("#") else "code")
    if "code" not in span:
        return "dead_line"
    return "name_hit" if names_at.get(start) else "code_but_no_name_match"


def audit(text, index):
    """扫一份文档，返回 `(counts, rows)`；`rows` 是「桶 → 引用原文」，供失败信息直接抄

    桶的名字就是**结论**：`dead_line` / `unresolved_source` = 可证失效（硬 0），
    `ambiguous_line` / `ambiguous_file` / `bare` / `scratch_missing` = 已清账的残余（棘轮），
    `name_hit` / `file_unique` = 好引用，`code_but_no_name_match` = 漂移档（A127 后半程），
    `scratch_line_ref` / `scratch_artifact` = 指向不入仓工件（A128 的地盘）。
    """
    src = source_dirs(index)
    rows = collections.defaultdict(list)
    for m in LINE_REF.finditer(text):
        bucket, rel = name_bucket(m.group(1), index, src)
        if bucket is None:
            lines = (ROOT / rel).read_text(encoding="utf-8").split("\n")
            bucket = classify(lines, int(m.group(2)),
                              int(m.group(3)) if m.group(3) else None,
                              declared_lines(rel))
        elif bucket == "ambiguous":
            bucket = "ambiguous_line"
        elif bucket == "scratch":
            bucket = "scratch_line_ref"
        rows[bucket].append(m.group(0))
    for m in FILE_TOKEN.finditer(text):
        bucket, _ = name_bucket(m.group(1), index, src)
        if bucket is None:
            bucket = "file_unique"
        elif bucket == "ambiguous":
            bucket = "ambiguous_file"
        elif bucket == "scratch":
            bucket = "scratch_artifact"
        rows[bucket].append(m.group(0))
    counts = {key: len(val) for key, val in rows.items()}
    # 总数 = **该种语法自身的匹配次数**，不由桶加和：上一版让 `line_refs` 与 `file_tokens`
    # 共享 `ambiguous` 那一桶，同一批引用被同时计入两个总数，账本里的 303 被读成了 427。
    counts["line_refs"] = len(LINE_REF.findall(text))
    counts["file_tokens"] = len(FILE_TOKEN.findall(text))
    counts["bare"] = len(BARE_REF.findall(text))
    return counts, {key: sorted(val) for key, val in rows.items()}


class TestDeadAndAmbiguousReferences:
    """硬门禁：可证失效 = 0；架构文档的歧义引用 = 0（账本的歧义在 `CEILING` 里挂着）"""

    def test_no_reference_points_at_a_blank_comment_or_missing_line(self):
        index = build_index()
        bad = {}
        for doc in DOCS:
            _, rows = audit((ROOT / doc).read_text(encoding="utf-8"), index)
            dead = rows.get("dead_line", []) + rows.get("unresolved_source", [])
            if dead:
                bad[doc] = dead
        assert bad == {}, bad

    def test_architecture_doc_has_no_ambiguous_reference(self):
        """歧义分两桶（带行号 / 只有文件名），但架构文档两边都必须是 0"""
        _, rows = audit((ROOT / ARCH).read_text(encoding="utf-8"), build_index())
        assert rows.get("ambiguous_line", []) == []
        assert rows.get("ambiguous_file", []) == []


class TestL79CleanupActuallyHappened:
    """本轮清掉的具体例子逐条钉住（行为面而不是计数面：计数由 `TestRatchetsAreExact` 管）"""

    def test_the_erratum_note_keeps_its_numbers_as_prose_not_as_pointers(self):
        text = (ROOT / ARCH).read_text(encoding="utf-8")
        assert "**行号勘误（L53 关掉 A89 之后补，原文不删）**" in text
        for pointer in ("`cli.py:46`", "`cli.py:49`", "`cli.py:49-51`", "`cli.py:51-53`",
                        "`cli.py:46-53`"):
            assert pointer not in text, pointer
        assert "第 49-51 行" in text          # 历史读数还在，只是不再冒充定位引用
        # 口径立了就得一处不留：同一句话在 §3.27 正文与 §6 表的两次出现也在本轮改了形状
        assert "`cli.py` 当时第 46 行" in text
        # A129：勘误段自己那两个读数也位移了，文档必须明说「散文数字没有机械通道」
        assert "这就是 A129" in text

    def test_the_module_names_this_round_qualified(self):
        """`config.py` 一名两指在架构文档里已彻底消失，但**账本**里还挂着（棘轮分开记）"""
        index = build_index()
        _, arch_rows = audit((ROOT / ARCH).read_text(encoding="utf-8"), index)
        _, ledger_rows = audit((ROOT / LEDGER).read_text(encoding="utf-8"), index)
        assert "`config.py`" not in arch_rows.get("ambiguous_file", [])
        assert "`config.py`" in ledger_rows.get("ambiguous_file", [])

    def test_the_three_wrong_source_paths_the_old_bucket_was_hiding_are_fixed(self):
        """`unresolved_source` 这一档**为什么存在**：它上一版被 `scratch_missing` 吞掉

        三条都是**产品源码路径写错**（不是草稿名），改前不会让任何东西红。逐条钉住现在的
        正确形状，而不是只钉「计数为 0」—— 计数为 0 也能靠「把那几句整个删掉」达成。
        """
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        # ERNIE 后端住在 ernie.py，仓里从来没有 baidu.py（`baidu` 只是配置里的后端 id）
        assert "`augmentor/models/baidu.py`" not in text
        assert "`augmentor/models/ernie.py`" in text
        # 两处「订正 A86 文件引用」的历史记录保留原错引，但**不再用反引号冒充定位引用**
        assert text.count("`api/routes/config_ops.py`") == 0
        assert "api/routes/config_ops.py" in text
        assert "`api/routes/config.py`" in text


class TestRatchetsAreExact:
    """每条残余按**精确值**记账：变差要改回来，变好也要改常数并在账本留一句"""

    def test_each_document_matches_its_own_ceiling(self):
        index = build_index()
        for doc in DOCS:
            counts, rows = audit((ROOT / doc).read_text(encoding="utf-8"), index)
            for bucket, ceiling in CEILING[doc].items():
                assert counts.get(bucket, 0) == ceiling, (
                    f"{doc} 的 {bucket} 实测 {counts.get(bucket, 0)}，基线写着 {ceiling}。"
                    f"清账清出来的就改小 `CEILING` 并在 OPTIMIZATION_LOOP.md 的 L 行留一句；"
                    f"新写坏的就把它改回去。\n明细：{rows.get(bucket, [])[:12]}")

    def test_only_the_current_round_leaves_a_hash_placeholder(self):
        """**每一轮的提交哈希必须当轮回填**：忘了就当场红

        日志块开头写「`哈希待 L{n+1} 回填`」是允许的（自己那一轮的哈希还不存在），
        但下一轮必须把它填掉 —— 所以非 0 的只许有本轮这一条。实测 L58..L71 共 14 条
        从未回填（那是 A130 的历史欠账，补正文要用户拍），加本轮 = 15。
        """
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        found = HASH_PLACEHOLDER.findall(text)
        assert len(found) == PLACEHOLDER_CEILING, found[-3:]


class TestTheAuditIsNotSpinning:
    """反空转 + 判据可失败：这台机器必须证明它既能看见好引用，也能抓住坏引用"""

    def test_both_docs_yield_a_known_amount_of_refs(self):
        index = build_index()
        for doc in DOCS:
            counts, _ = audit((ROOT / doc).read_text(encoding="utf-8"), index)
            for bucket, floor in FLOOR[doc].items():
                assert counts[bucket] >= floor, (doc, bucket, counts[bucket], floor)

    def test_the_index_sees_both_config_modules_and_full_paths_are_unique(self):
        live, _ = resolve("config.py", build_index())
        assert live == ["api/routes/config.py", "augmentor/config.py"], live
        for one in live:
            solo, _ = resolve(one, build_index())
            assert solo == [one], (one, solo)

    def test_a_reference_to_a_blank_line_is_reported_dead(self):
        """造一份含死引用的假文档，普查必须抓到它（L76 立的规矩：判据不能永不开火）

        明细存的是**引用原文**（含反引号），因为失败信息要能被读者原样 grep 回文档。
        """
        lines = (ROOT / "augmentor/logging_setup.py").read_text(
            encoding="utf-8").split("\n")
        blank = next(str(i + 1) for i, line in enumerate(lines) if not line.strip())
        fake = f"举例：见 `logging_setup.py:{blank}` 与 `config.py` 两份。\n"
        counts, rows = audit(fake, build_index())
        assert rows["dead_line"] == [f"`logging_setup.py:{blank}`"], rows
        assert counts["ambiguous_file"] == 1, rows.get("ambiguous_file")

    def test_an_out_of_range_reference_is_reported_dead(self):
        _, rows = audit("见 `augmentor/logging_setup.py:999999`\n", build_index())
        assert rows["dead_line"] == ["`augmentor/logging_setup.py:999999`"], rows

    def test_a_wrong_source_path_is_hard_failure_and_a_bogus_top_dir_is_only_a_draft(self):
        """`unresolved_source` 这一档**两个方向**都要能红 —— 分桶判据不偏不倚

        只钉「写错的产品路径要红」会纵容另一种错：把任何未解析名字都判成硬失效，于是
        111 个 Temp 草稿名当场把门禁撑成永红。两半一起钉。
        """
        index = build_index()
        _, rows = audit("见 `augmentor/models/baidu.py` 与 `api/routes/config_ops.py`\n", index)
        assert rows["unresolved_source"] == ["`api/routes/config_ops.py`",
                                            "`augmentor/models/baidu.py`"], rows
        assert rows.get("scratch_missing", []) == [], rows

        _, rows = audit("见 `l99q/probe.py` 与 `tolus/foo.py`\n", index)
        assert rows["scratch_missing"] == ["`l99q/probe.py`", "`tolus/foo.py`"], rows
        assert rows.get("unresolved_source", []) == [], rows

    def test_a_temp_snapshot_copy_does_not_launder_a_wrong_source_prefix(self):
        """分桶**顺序**的用例：这一条在账本里真的存在，且改前藏在工件档

        `` `augmentor/api/routes/dataset_tools.py` `` 指的是产品路径，可产品树里根本没有
        `augmentor/api/`（真实文件是 `api/routes/dataset_tools.py`）；Temp 下有 HEAD 快照副本
        恰好同尾 ⇒ 先查工件会把它判成「指向不入仓工件」，永远不红。
        """
        live, scratch = resolve("augmentor/api/routes/dataset_tools.py", build_index())
        assert live == [], live                       # 产品树里没有
        assert len(scratch) >= 1, scratch             # Temp 快照里有：先查就会被它骗过
        _, rows = audit("见 `augmentor/api/routes/dataset_tools.py:117-127`\n", build_index())
        assert rows["unresolved_source"] == ["`augmentor/api/routes/dataset_tools.py:117-127`"]
        assert rows.get("scratch_line_ref", []) == []

    def test_the_hash_placeholder_regex_counts_debt_not_prose(self):
        """正则 vs `count("哈希待")`：账本里 26 : 15，散文**谈论**这件事的 11 次不是欠账"""
        sample = ("- **L80** `哈希待 L81 回填`\n"
                  "- **L79** `deadbee0`（当轮已回填）\n"
                  "  - 顺手记一句：本行的哈希待 L81 回填，numstat 也待量。\n")
        assert HASH_PLACEHOLDER.findall(sample) == ["- **L80** `哈希待 L81 回填`"]
        assert sample.count("哈希待") == 2      # 宽松口径会多算一条 ⇒ 判据必须按形状

    def test_the_totals_quoted_in_the_prose_are_the_current_measurement(self):
        """`MEASURED` 与真实现量必须相等：**注释与文档里的数不许靠回忆**

        这条是本轮回看段抓到的病：`docs/ARCHITECTURE.md` §3.33 写完后，同一轮内的判据改动
        让「40 处 / 114 处 / 10 例」三个数当场过期，而**没有任何用例能出声**（散文数字在
        守卫的可见范围之外，即 A129）。修法是把数字搬进常量、让用例去比对现量。
        """
        index = build_index()
        for doc in DOCS:
            counts, _ = audit((ROOT / doc).read_text(encoding="utf-8"), index)
            for bucket, want in MEASURED[doc].items():
                assert counts[bucket] == want, (doc, bucket, counts[bucket], want)

    def test_a_range_whose_start_is_blank_but_holds_code_is_not_dead(self):
        """区间档的**反向用例**：全段皆非代码才算死，起点歪一行不算（口径必须两边都钉）"""
        lines = (ROOT / "augmentor/logging_setup.py").read_text(
            encoding="utf-8").split("\n")
        blank = next(i for i, line in enumerate(lines) if not line.strip())
        while lines[blank + 1].strip().startswith("#") or not lines[blank + 1].strip():
            blank += 1
        fake = f"见 `augmentor/logging_setup.py:{blank + 1}-{blank + 2}`\n"
        _, rows = audit(fake, build_index())
        assert rows.get("dead_line", []) == [], rows
        assert rows["code_but_no_name_match"] == [
            f"`augmentor/logging_setup.py:{blank + 1}-{blank + 2}`"], rows
