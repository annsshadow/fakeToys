# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""定向缺陷注入验证器

用法（在 `augmentor/` 目录下，用装了 pytest 的解释器执行）：

    python scripts/inject_defect_check.py

## 它解决什么问题

**绿灯本身不是证据。** 一个测试通过，可能只是因为它在测别的东西、
断言写反了、或者压根没跑到被测分支。只有当「把修复改回缺陷」会让它变红时，
这个测试才真正锁定了行为。

本脚本把每条修复**改回缺陷**、跑对应的测试、确认变红，然后还原源文件。
与 `scripts/mutation/` 的区别：

| | `scripts/mutation/`（cosmic-ray） | 本脚本 |
|---|---|---|
| 粒度 | 全量变异体，自动生成 | 手写，每条对应一处**具体修复** |
| 目的 | 度量测试套件**整体**强度 | 验证**本轮新增**用例是否真的咬住 |
| 耗时 | 分钟级（需抽样） | 秒级 |
| 输出 | 变异得分 | 每条注入的「变红 / 仍全绿」判定 |

两者互补：变异得分高但某条新用例没咬住，本脚本能指出来；
反之本脚本只覆盖已知缺陷，发现不了「谁都没想过的变异」。

## 怎么用

把本轮修复清单写进下面的 `CASES`。每条是：

    (说明, 要改的文件, 原文, 替换为, [pytest 节点 ID, ...])

**原文必须在目标文件里唯一出现**（脚本会自检出现次数）。若锚点有歧义，
往上多带几行上下文 —— 本项目踩过：`dataset_ops` 的 split 与 sample 结尾
代码相同，只带 `seed=request.seed` 会匹配到两处。

源文件的还原放在 `try/finally` 里：任何一条注入崩溃都不会留下脏改动。
跑完后建议 `git diff` 复核一遍，确认只剩预期的改动。

## 判据

* 每条注入都必须让**对应用例失败** → 说明用例有效；
* 若某条注入后测试**仍然全绿** → 该用例是假测试，必须重写或删除。

退出码：全部有效为 0，存在无效用例或锚点歧义为 1。
"""

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PY = sys.executable
TEST_FILE = "tests/integration/test_api_dataset_system_tools.py"
STREAM_TEST = "tests/unit/test_streaming_edge_branches.py"

# ============================================================
# 注入清单
# ============================================================
# 每条：说明 / 目标文件 / 原文 / 替换为 / 期望变红的用例
CASES = [
    (
        "A. to_http_error 不再翻译 JSONDecodeError（回退成转发英文原文）",
        "api/deps.py",
        '''    if isinstance(exc, json.JSONDecodeError):
        # 只回传行列号：足够定位，又不泄漏实现细节。
        return HTTPException(
            status_code=400,
            detail=f"数据文件不是合法 JSON（第 {exc.lineno} 行第 {exc.colno} 列）",
        )
''',
        "",
        [
            f"{TEST_FILE}::TestSharedErrorSemantics"
            "::test_broken_json_returns_400_without_parser_leak",
            f"{TEST_FILE}::TestDatasetSearch::test_broken_json_returns_400",
            f"{TEST_FILE}::TestDatasetCompareFeaturesAutoConfig"
            "::test_compare_broken_json_returns_400",
        ],
    ),
    (
        "B. 恢复备份不再把 BackupError 当「不存在」（回退成 400）",
        "api/deps.py",
        """    if isinstance(exc, FileNotFoundError) or (
        not_found_types and isinstance(exc, not_found_types)
    ):
        return HTTPException(status_code=404, detail=not_found)""",
        """    if isinstance(exc, FileNotFoundError):
        return HTTPException(status_code=404, detail=not_found)""",
        [f"{TEST_FILE}::TestSystemBackups::test_restore_unknown_backup_returns_404"],
    ),
    (
        "C. 去掉 validate 的预设白名单（未知预设静默回退 basic）",
        "api/routes/dataset_tools.py",
        """    if request.preset not in VALIDATION_PRESETS:
        raise HTTPException(
            status_code=400,
            detail=(
                f"未知的验证预设: {request.preset}"
                f"（可选 {' / '.join(VALIDATION_PRESETS)}）"
            ),
        )
""",
        "",
        [f"{TEST_FILE}::TestDatasetValidate::test_unknown_preset_rejected"],
    ),
    (
        'D. ValidateRequest.preset 默认值改回 "default"',
        "api/routes/dataset_tools.py",
        '    preset: str = "basic"',
        '    preset: str = "default"',
        [f"{TEST_FILE}::TestDatasetValidate::test_default_preset_equals_basic"],
    ),
    (
        "E. 去掉 search 的方法白名单（未知方法静默回退 contains）",
        "api/routes/dataset_tools.py",
        """    if request.method not in SEARCH_METHODS:
        raise HTTPException(
            status_code=400,
            detail=(
                f"未知的搜索方法: {request.method}"
                f"（可选 {' / '.join(SEARCH_METHODS)}）"
            ),
        )
""",
        "",
        [f"{TEST_FILE}::TestDatasetSearch::test_unknown_method_rejected"],
    ),
    (
        "F. 去掉 auto-test 的套件白名单（未知套件静默回退 default）",
        "api/routes/system_ops.py",
        """    if request.suite is not None and request.suite not in AUTO_TEST_SUITES:
        raise HTTPException(
            status_code=400,
            detail=(
                f"未知的测试套件: {request.suite}"
                f"（当前只支持 {' / '.join(AUTO_TEST_SUITES)}；"
                "自定义套件需要先由调用方注册，本端点暂无注册入口）"
            ),
        )
""",
        "",
        [f"{TEST_FILE}::TestSystemAutoTest::test_unknown_suite_rejected"],
    ),
    (
        "G. 去掉 migrate 的规则白名单（未知规则被静默丢弃）",
        "api/routes/system_ops.py",
        """    unknown_rules = [r for r in (request.rules or []) if r not in MIGRATION_RULES]
    if unknown_rules:
        raise HTTPException(
            status_code=400,
            detail=(
                f"未知的迁移规则: {', '.join(unknown_rules)}"
                f"（可选 {' / '.join(MIGRATION_RULES)}）"
            ),
        )
""",
        "",
        [f"{TEST_FILE}::TestSystemMigrate::test_unknown_rules_rejected"],
    ),
    (
        "H. 删掉 split 的 except HTTPException: raise（HTTPException 被降级成 500）",
        "api/routes/dataset_tools.py",
        """                    ratios=(request.train_ratio, request.val_ratio, request.test_ratio),
                    seed=request.seed,
                ),
            )

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e""",
        """                    ratios=(request.train_ratio, request.val_ratio, request.test_ratio),
                    seed=request.seed,
                ),
            )

        return await run_in_thread(run)
    except Exception as e:
        raise to_http_error(e) from e""",
        [f"{TEST_FILE}::TestHttpExceptionPassthrough"
         "::test_split_rejects_traversal_output_dir"],
    ),
    (
        "I. 让 stats 的意外异常直接冒泡（不再转成 500）",
        "api/routes/dataset_tools.py",
        """        stats = calculate_statistics(items, Path(path).stem, request.fields)
        return stats.to_dict()
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e""",
        """        stats = calculate_statistics(items, Path(path).stem, request.fields)
        return stats.to_dict()
    except HTTPException:
        raise""",
        [f"{TEST_FILE}::TestUnexpectedFailureBecomes500"
         "::test_post_endpoint_returns_500"],
    ),
    (
        "J. _peek_first_non_space 不再跳过空白",
        "augmentor/streaming.py",
        """        for ch in chunk:
            if ch not in _WS:
                return ch""",
        """        for ch in chunk:
            return ch""",
        [
            f"{STREAM_TEST}::TestPeekFirstNonSpace"
            "::test_whitespace_only_stream_returns_none",
            f"{STREAM_TEST}::TestPeekFirstNonSpace"
            "::test_leading_whitespace_returns_first_real_char",
        ],
    ),
    (
        "K. 未闭合数组改成报错（不再按已读元素结束）",
        "augmentor/streaming.py",
        """        if not skip_ws():
            return  # 数组未闭合（文件被截断）：按已读到的元素结束""",
        """        if not skip_ws():
            raise DataFormatError("数组未闭合")""",
        [f"{STREAM_TEST}::TestJsonArrayTruncation"
         "::test_unclosed_array_after_complete_elements"],
    ),
    (
        "L. 单个 JSON 值判定忽略「值之后还有内容」",
        "augmentor/streaming.py",
        """        if pos < len(buf):
            return False  # 首个值之后仍有内容 → 不是单个值""",
        """        if pos < len(buf):
            pass""",
        [f"{STREAM_TEST}::TestIsSingleJsonValue"
         "::test_two_values_is_not_single_value"],
    ),
    (
        "M. close() 无条件补 ']'（jsonl 产物被污染）",
        "augmentor/streaming.py",
        """    def close(self):
        \"\"\"关闭写入器\"\"\"
        if self._file:
            if self.format == 'json' and self.mode == 'w':
                self._file.write(']')""",
        """    def close(self):
        \"\"\"关闭写入器\"\"\"
        if self._file:
            if True:
                self._file.write(']')""",
        [f"{STREAM_TEST}::TestStreamWriterBranches"
         "::test_close_jsonl_writer_does_not_add_bracket"],
    ),
]


def run_tests(node_ids):
    """跑指定用例

    Args:
        node_ids: pytest 节点 ID 列表

    Returns:
        (返回码, 末行摘要)
    """
    proc = subprocess.run(
        [PY, "-m", "pytest", "-q", "--no-cov", "-p", "no:randomly", *node_ids],
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    tail = [ln for ln in proc.stdout.strip().splitlines() if ln.strip()]
    return proc.returncode, (tail[-1] if tail else "(无输出)")


def main():
    """逐个注入缺陷并确认测试变红

    Returns:
        进程退出码：0 表示全部有效
    """
    invalid = []
    for label, rel_path, old, new, node_ids in CASES:
        target = ROOT / rel_path
        original = target.read_text(encoding="utf-8")
        count = original.count(old)
        if count != 1:
            print(f"[跳过] {label}: 锚点在 {rel_path} 中出现 {count} 次（应为 1）")
            invalid.append(label)
            continue
        try:
            target.write_text(original.replace(old, new), encoding="utf-8")
            code, summary = run_tests(node_ids)
        finally:
            target.write_text(original, encoding="utf-8")

        verdict = "已变红 ✓" if code != 0 else "仍然全绿 ✗（用例无效）"
        print(f"[{verdict}] {label}")
        print(f"           {summary}")
        if code == 0:
            invalid.append(label)

    print()
    if invalid:
        print(f"存在无效用例或锚点歧义：{len(invalid)}")
        for item in invalid:
            print("  -", item)
        return 1
    print(f"全部 {len(CASES)} 项注入都让对应用例失败 —— 用例确实锁定了行为")
    return 0


if __name__ == "__main__":
    sys.exit(main())
