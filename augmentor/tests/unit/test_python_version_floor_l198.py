# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L198：Python 版本口径三处合一 + 机器守卫

**原状**：同一件事在仓里写了三个数 —— 顶层 README `3.8+`、`docs/README.md`
`3.10+（开发验证环境为 3.13）`、`docs/DEPLOYMENT.md` 镜像与服务器段 `3.11`。
读者无从判断哪个是真的，而这三个数都可能误导。

**实测怎么定的**（不是拍脑袋，也不是抄任何文档）：

1. **产品码语法下限**：把 `augmentor/`、`api/`、`cli.py` 全部 `.py` 用
   `ast.parse(source, feature_version=(3, N))` 从 N=8 起逐版本试，实测
   **3.8 即全部通过**（`match`/PEP604/`slots=True`/`except*` 全仓 0 命中）。
2. **测试套件真正的地板**：`tests/unit/test_micro_branches_l66.py` 有一处
   `storage: str | None` 运行期注解（PEP604）⇒ 测试面需要 **3.10**。
3. **依赖面**：`requirements-core.txt` 声明的区间（`numpy>=1.21,<3` /
   `fastapi>=0.78,<1`）在 3.8 上装得起来，但**本机实际装上的** `fastapi` /
   `uvicorn` / `requests` / `pytest` / `coverage` 自己都声明 `>=3.10`。
4. **部署面**：`docker/Dockerfile` 钉 `python:3.11-slim`；`numpy 2.x` / `pandas 3.0`
   / `chromadb 1.5` 自己声明 `>=3.11`。

⇒ 定 **3.10+** 为最低要求，并显式写出「装全套可选依赖需 3.11、部署镜像钉 3.11、
验证环境 3.13.14 与 3.14.4」。定这个数的理由是它比三个旧数都更接近事实，
且**不夸大**（不说 3.13）：核心依赖在 3.10 上确实装得起来。

**守卫防的是漂移不是数字本身**：产品码一旦引入更高版本的语法（有人写了
`match`、PEP604 注解、`slots=True`），`FLOOR` 实推值就变，用例当场红，逼着同步
三处文档。手抄的数字没有这个性质。
"""

import ast
import re
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]

#: 产品码（不含测试）：测试面的地板另算，见 MODULE_PEP604_SITES
PRODUCT_ROOTS = (ROOT / "augmentor", ROOT / "api")
PRODUCT_FILES = [ROOT / "cli.py"]

#: 文档里的版本主张：路径 → 该处必须出现的统一口径（按各文件真实写法给，
#: 不硬套一个模板 —— 表格行与散文行的形状本来就不一样）
DOCS = {
    "README.md": "**Python 3.10+**",
    "docs/README.md": "| Python | 3.10+",
    "docs/DEPLOYMENT.md": "python:3.11-slim",
}

#: 统一口径的核心数字，三处都必须出现（独立于上面各文件的具体形状）
CLAIM = "3.10+"

#: 三处曾写过的过期读数，一律不许再出现（防「改了新的忘了旧的」）
STALE = ("Python 3.8+", "3.10+（开发验证环境为 3.13）")

#: 已知的、把测试面地板抬到 3.10 的那处 PEP604 注解（有它才有 3.10 这个数；
#: 改成 Optional[str] 也不会把产品码地板抬上去，只是让测试面回到 3.9）
MODULE_PEP604_SITES = ("tests/unit/test_micro_branches_l66.py",)


def _product_files():
    files = list(PRODUCT_FILES)
    for root in PRODUCT_ROOTS:
        files += [p for p in root.rglob("*.py")
                  if not any(x in {"__pycache__", ".backups", ".pytest_cache"}
                             for x in p.parts)]
    return sorted(files)


def syntax_floor(files):
    """全部文件都能通过 `ast.parse(feature_version=(3, N))` 的最低 N"""
    for minor in range(8, 15):
        for path in files:
            try:
                ast.parse(path.read_text(encoding="utf-8"), str(path),
                          feature_version=(3, minor))
            except SyntaxError:
                break
        else:
            return minor
    return None


class TestProductSyntaxFloor:
    """产品码的语法下限必须实推，且不许高于文档声称的下限"""

    def test_product_code_still_parses_at_the_documented_minimum(self):
        files = _product_files()
        assert files, "一个产品 .py 都没扫到 ⇒ glob 坏了"
        floor = syntax_floor(files)
        assert floor is not None, "连 3.14 都解析不了 ⇒ 扫描或判据坏了"
        assert floor <= 10, (
            f"产品码语法下限实测是 3.{floor}，高于文档声称的 3.10 ⇒ "
            "要么把文档的最低版本抬上去，要么把新语法改掉"
        )

    def test_floor_probe_is_real(self, tmp_path):
        """防空转：造一个只有 3.12 才有的语法，floor 必须真被抬起来

        没有这一条，「全部通过 3.8」可能只是因为扫描面是空的。
        """
        probe = tmp_path / "probe_new_syntax.py"
        probe.write_text("type Alias = int\n", encoding="utf-8")   # PEP 695，3.12
        assert syntax_floor([probe]) == 12

    def test_no_newer_syntax_constructs_snuck_in(self):
        """静态扫一遍会抬高地板的形状（不依赖 ast.parse 的版本矩阵）"""
        patterns = {
            "match/case": re.compile(r"^\s*match\s+[\w\s.]+:\s*$", re.M),
            "PEP604 运行期注解": re.compile(r"def\s+\w+\([^)]*:\s*\w+\s*\|\s*\w+", re.M),
            "dataclass slots=True": re.compile(r"slots\s*=\s*True"),
            "except* / ExceptionGroup": re.compile(r"except\s*\*|\bExceptionGroup\b"),
            "typing.Self": re.compile(r"\btyping\.Self\b"),
            "PEP695 type 别名": re.compile(r"^\s*type\s+\w+\s*=", re.M),
        }
        bad = []
        for path in _product_files():
            text = path.read_text(encoding="utf-8")
            for label, pat in patterns.items():
                if pat.search(text):
                    bad.append(f"{path.relative_to(ROOT)}: {label}")
        assert not bad, (
            "产品码里出现会抬高最低版本的新语法：\n  " + "\n  ".join(bad)
        )


class TestDocsAgreeOnOneNumber:
    """三处文档必须写同一个口径，且过期读数一律不许复活"""

    @pytest.mark.parametrize("rel, snippet", sorted(DOCS.items()))
    def test_documented_claim_is_present(self, rel, snippet):
        text = (ROOT / rel).read_text(encoding="utf-8")
        assert snippet in text, (
            f"{rel} 里找不到统一口径「{snippet}」——三处文档必须写同一件事"
        )
        assert CLAIM in text, f"{rel} 里连「{CLAIM}」这个数都没有"

    def test_no_document_claims_a_lower_minimum(self):
        """不许再有人写「最低 3.8 / 3.9」——那正是 L198 之前的三处漂移之一

        判据收得很窄：只认 `Python` 后面跟**个位数次版本号**这一种形状
        （`Python 3.8` / `Python**3.9**`）。这样 `Python 3.10`、`CPython 3.14`、
        `Python 3.13.14` 全都不会被误扫——架构文档里的 `### 3.8` 是章节号、
        API 文档里的 `3.13.14` 是验证环境，都不进这道闸。
        """
        lower = re.compile(r"Python\s*\*{0,2}3\.[0-9](?!\d)")
        offenders = []
        for md in [ROOT / "README.md"] + sorted((ROOT / "docs").glob("*.md")):
            for lineno, line in enumerate(md.read_text(encoding="utf-8").splitlines(), 1):
                if lower.search(line):
                    offenders.append(f"{md.name}:{lineno}: {line.strip()[:70]}")
        assert not offenders, (
            "这些行把 Python 最低版本写成了 3.9 或更低：\n  " + "\n  ".join(offenders)
        )

    @pytest.mark.parametrize("stale", STALE)
    def test_stale_readings_are_gone(self, stale):
        for rel in DOCS:
            assert stale not in (ROOT / rel).read_text(encoding="utf-8"), (
                f"{rel} 里还留着过期读数「{stale}」"
            )

    def test_deployment_pin_is_at_least_the_minimum(self):
        """部署镜像钉的版本不许低于声称的下限（否则「最低 3.10」是空话）"""
        text = (ROOT / "docs" / "DEPLOYMENT.md").read_text(encoding="utf-8")
        m = re.search(r"python:3\.(\d+)-slim", text)
        assert m, "DEPLOYMENT.md 里找不到 python:3.x-slim 镜像钉"
        assert int(m.group(1)) >= 10, (
            f"部署镜像钉的是 3.{m.group(1)}，低于声称的下限 3.10"
        )

    def test_test_suite_floor_is_the_reason_for_ten(self):
        """3.10 这个数的另一半理由是测试面 —— 那一处 PEP604 注解必须还在"""
        for rel in MODULE_PEP604_SITES:
            path = ROOT / rel
            if not path.exists():
                continue
            text = path.read_text(encoding="utf-8")
            assert re.search(r"str\s*\|\s*None", text), (
                f"{rel} 里那处 `str | None` 注解不在了 ⇒ 测试面地板可能已降回 3.9，"
                "重新实测一遍再决定文档写几"
            )
