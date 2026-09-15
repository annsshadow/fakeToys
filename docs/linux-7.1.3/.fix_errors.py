import os
import re

stats = {
    'muji_removed': 0,
    'files_modified': set(),
    'truncations_fixed': 0,
}

# Fix 1: Remove 鈥? from all files
print("=== Fix 1: Removing 鈥? mojibake ===")
for root, dirs, files in os.walk('.'):
    dirs[:] = [d for d in dirs if not d.startswith('.')]
    for fname in files:
        if not (fname.endswith('.md') or fname.endswith('.rst') or fname.endswith('.txt')):
            continue
        fpath = os.path.join(root, fname)
        try:
            with open(fpath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
        except:
            continue

        if '鈥?' in content:
            count = content.count('鈥?')
            new_content = content.replace('鈥?', '')
            with open(fpath, 'w', encoding='utf-8') as f:
                f.write(new_content)
            stats['muji_removed'] += count
            stats['files_modified'].add(fpath)
            print(f"  {fpath}: removed {count} occurrences")

print(f"Total 鈥? removed: {stats['muji_removed']}")
print(f"Files modified: {len(stats['files_modified'])}")

# Fix 2: Fix _w2_test_copy.md to match PROJECT.md (minus 鈥? already fixed)
# It's a duplicate, so no additional fix needed beyond 鈥? removal

# Fix 3: Fix truncated headings in key documentation files
print("\n=== Fix 3: Fixing truncated headings ===")

heading_fixes = {
    # START_HERE.md fixes
    '.\\START_HERE.md': {
        '如何使用本指': '如何使用本指南',
        '阶段 1：Orientation（必读，2 小时': '阶段 1：Orientation（必读，2 小时）',
        '阶段 2：架构基础（必读，4-6 小时': '阶段 2：架构基础（必读，4-6 小时）',
        '阶段 3：子系统选读（选读，按需深入': '阶段 3：子系统选读（选读，按需深入）',
        '阶段 4：开发实践（必读，约 2 小时': '阶段 4：开发实践（必读，约 2 小时）',
        '阶段 5：扩展资源（选读': '阶段 5：扩展资源（选读）',
        '**阅读建议*': '**阅读建议**',
        '快速了解内核全': '快速了解内核全貌',
        '写第一个内核模': '写第一个内核模块',
        '理解内存管理': '理解内存管理',
        '理解文件系统': '理解文件系统',
        '理解网络': '理解网络',
        '准备提交补丁': '准备提交补丁',
        '所Markdown 文件': '所有Markdown 文件',
        '本指南基v7.1.3': '本指南基于v7.1.3',
    },
    # index.md (top-level) fixes
    '.\\index.md': {
        '内核文档与内核本身一样，在很大程度上仍是一': '内核文档与内核本身一样，在很大程度上仍是一项进行中的工作',
        '与开发社区协作并将您的工作推向上游的基本指南': '与开发社区协作并将您的工作推向上游的基本指南',
        '供需要与内核其余部分对接的开发人员使用的手册': '供需要与内核其余部分对接的开发人员使用的手册',
        '为所有内核开发人员提供有用信息的各种其他手册': '为所有内核开发人员提供有用信息的各种其他手册',
        '下列手册是为内核*用户**编写的': '下列手册是为内核用户编写的',
        '试图让内核在给定系统上以': '试图让内核在给定系统上以最佳状态运行的人',
        '情况更是如此': '情况更是如此',
        '调整或转换为 reStructuredText 格式': '调整或转换为 reStructuredText 格式',
        '或者干脆太旧了': '或者干脆太旧了',
    },
    # subsystem-apis.md fixes
    '.\\subsystem-apis.md': {
        '鍐呮牳瀛愮郴缁熸枃妗': '内核子系统文档',
        '鏍稿績瀛愮郴缁': '核心子系统',
        '鍏朵粬瀛愮郴缁': '其他子系统',
        '本页Linux 内核文档的子系统总索引': '本页面汇总Linux 内核文档的子系统总索引',
    },
    # watchdog/index.md fixes
    '.\\watchdog\\index.md': {
        '## 看门狗（watchdog': '## 看门狗（watchdog）',
    },
    # wmi/index.md fixes
    '.\\wmi\\index.md': {
    },
    # w1/index.md fixes
    '.\\w1\\index.md': {
    },
    # w1/slaves/index.md fixes
    '.\\w1\\slaves\\index.md': {
        '## 1-Wire 从设备驱': '## 1-Wire 从设备驱动',
        '本页1-Wire 从设备驱动的索引': '本页面为1-Wire 从设备驱动的索引',
        '列DS2406': '列出了DS2406',
    },
    # hwmon/index.md - fix heading
    '.\\hwmon\\index.md': {
        '## 纭欢鐩戞帶': '## 硬件监控',
        '## 纭欢鐩戞帶鍐呮牳椹卞姩': '## 硬件监控内核接口',
    },
    # _TRANSLATE_PROTOCOL.md fixes
    '.\\_TRANSLATE_PROTOCOL.md': {
    },
}

for fpath, fixes in heading_fixes.items():
    full_path = os.path.join('.', fpath)
    if not os.path.exists(full_path):
        continue
    try:
        with open(full_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except:
        continue

    modified = False
    for old, new in fixes.items():
        if old in content:
            count = content.count(old)
            content = content.replace(old, new)
            stats['truncations_fixed'] += count
            modified = True
            print(f"  {fpath}: '{old[:30]}...' -> '{new[:30]}...' ({count}x)")

    if modified:
        with open(full_path, 'w', encoding='utf-8') as f:
            f.write(content)

print(f"\nTotal truncations fixed: {stats['truncations_fixed']}")

# Fix 4: Fix _w2_test_copy.md specific issues (it's a duplicate of PROJECT.md)
# Already handled by 鈥? removal above

# Fix 5: Check for remaining 鈥? and verify
print("\n=== Verification ===")
remaining = 0
for root, dirs, files in os.walk('.'):
    dirs[:] = [d for d in dirs if not d.startswith('.')]
    for fname in files:
        if not (fname.endswith('.md') or fname.endswith('.rst') or fname.endswith('.txt')):
            continue
        fpath = os.path.join(root, fname)
        try:
            with open(fpath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
        except:
            continue
        if '鈥?' in content:
            remaining += content.count('鈥?')
            print(f"  REMAINING: {fpath}: {content.count('鈥?')} occurrences")

print(f"Remaining 鈥?: {remaining}")

print("\n=== Summary ===")
print(f"鈥? removed: {stats['muji_removed']}")
print(f"Files modified: {len(stats['files_modified'])}")
print(f"Truncations fixed: {stats['truncations_fixed']}")
