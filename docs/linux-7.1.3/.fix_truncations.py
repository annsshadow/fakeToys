import os
import re

files_fixed = []

# Fix START_HERE.md
fpath = '.\\START_HERE.md'
if os.path.exists(fpath):
    with open(fpath, 'r', encoding='utf-8') as f:
        content = f.read()

    fixes = [
        # Line 3: duplicated blockquote marker
        ('> 3934 篇内核文档中精选出的阅读路线图> 按此路径阅读',
         '> 3934 篇内核文档中精选出的阅读路线图\n> 按此路径阅读'),
        # Line 7: truncated
        ('不可或缺的文- **选读**', '不可或缺的文档- **选读**'),
        # Line 9: truncated
        ('由 `tools/docs/rst-to-md.py` 转换生成', '由 `tools/docs/rst-to-md.py` 转换生成'),
        # Line 16: truncated
        ('按角色（开发研究安全专家/系统管理员）给你的入|', '按角色（开发研究安全专家/系统管理员）给你的入门指南|'),
        # Line 17: truncated
        ('了解整个文档树的组织结|', '了解整个文档树的组织结构|'),
        # Line 18: truncated
        ('"内核是什——硬件支持、构建安装、快速概|', '"内核是什——硬件支持、构建安装、快速概览|'),
        # Line 19: truncated
        ('如何成为内核开发者——工具、邮件列表、社区规|', '如何成为内核开发者——工具、邮件列表、社区规范|'),
        # Line 20: truncated
        ('内核社区如何运作——发布周期、合并窗口、补丁生命周|', '内核社区如何运作——发布周期、合并窗口、补丁生命周期|'),
        # Line 25: truncated
        ('目标：建立对内核执行模型的核心直觉。这是最重要的阶段', '目标：建立对内核执行模型的核心直觉。这是最重要的阶段'),
        # Line 28: truncated
        ('**最重要的单篇文*：CPU 执行上下文', '**最重要的单篇文档**：CPU 执行上下文'),
        # Line 29: truncated
        ('内核 API 分类图谱——快速扫一遍，知道有什么接口可|', '内核 API 分类图谱——快速扫一遍，知道有什么接口可用|'),
        # Line 31: truncated
        ('进程调度：CFS 设计 + 当前 EEVDF 调度|', '进程调度：CFS 设计 + 当前 EEVDF 调度器|'),
        # Line 32: truncated
        ('同步原语：自旋锁、互斥锁、RCU 锁的分类与使用场|', '同步原语：自旋锁、互斥锁、RCU 锁的分类与使用场景|'),
        # Line 33: truncated
        ('RCU 机制：读-复制-更新的核心思想与实|', 'RCU 机制：读-复制-更新的核心思想与实践|'),
        # Line 36: formatting
        ('**阅读建议*', '**阅读建议**'),
        # Line 37: truncated
        ('它会在 30 分钟内给内核是怎么跑起的整体感- 然后', '它会在 30 分钟内给内核是怎么跑起的整体感- 然后'),
        # Line 46: truncated
        ('superblock/inode/dentry 模型、路径查找、挂载命名空|', 'superblock/inode/dentry 模型、路径查找、挂载命名空间|'),
        # Line 50: truncated
        ('安全架构：LSM 框架、内核自防御机制、凭证管|', '安全架构：LSM 框架、内核自防御机制、凭证管理|'),
        # Line 54: truncated
        ('每个子系统通常只需要读 `index.md` + 1-2 篇核心设计文-', '每个子系统通常只需要读 `index.md` + 1-2 篇核心设计文档-'),
        # Line 62: truncated
        ('内核编码规范——缩进、命名、注释、空格与制表|', '内核编码规范——缩进、命名、注释、空格与制表符|'),
        # Line 64: truncated
        ('构建系统：顶Makefile、Kconfig 语法、模块构|', '构建系统：顶Makefile、Kconfig 语法、模块构建|'),
        # Line 65: truncated
        ('开发工具：checkpatch、KUnit 单元测试、调试工|', '开发工具：checkpatch、KUnit 单元测试、调试工具|'),
        # Line 83: missing pipe
        ('| 快速了解内核全貌|', '| 快速了解内核全貌 |'),
        # Line 84: missing pipe
        ('| 写第一个内核模块|', '| 写第一个内核模块 |'),
        # Line 85: missing pipe
        ('| 理解内存管理 |', '| 理解内存管理 |'),
        # Line 86: missing pipe
        ('| 理解文件系统 |', '| 理解文件系统 |'),
        # Line 87: missing pipe
        ('| 理解网络|', '| 理解网络 |'),
        # Line 94: truncated
        ('所有Markdown 文件`tools/docs/rst-to-md.py` Sphinx `.rst` 源文件自动转换生- 如果发现转换质量问题，可以查看原`.rst` 文件或修复转换脚- 文档内容随内核版本更新，本指南基v7.1.3Baby Opossum Posse"',
         '所有Markdown 文件由 `tools/docs/rst-to-md.py` 从 Sphinx `.rst` 源文件自动转换生成- 如果发现转换质量问题，可以查看原`.rst` 文件或修复转换脚本- 文档内容随内核版本更新，本指南基于v7.1.3'),
    ]

    for old, new in fixes:
        if old in content:
            content = content.replace(old, new)
            print('  START_HERE.md: fixed "{}..."'.format(old[:40]))

    with open(fpath, 'w', encoding='utf-8') as f:
        f.write(content)
    files_fixed.append(fpath)

# Fix index.md
fpath = '.\\index.md'
if os.path.exists(fpath):
    with open(fpath, 'r', encoding='utf-8') as f:
        content = f.read()

    fixes = [
        ('进行中的工作', '进行中的工作'),
        ('与开发社区协', '与开发社区协作'),
        ('供需要与内核其余部分对接的开发人员使用的手册', '供需要与内核其余部分对接的开发人员使用的手册'),
        ('为所有内核开发人员提供有用信息的各种其他手册', '为所有内核开发人员提供有用信息的各种其他手册'),
        ('下列手册是为内核用户编写的——即那些试图让内核在给定系统上以最佳状态运行的人', '下列手册是为内核用户编写的——即那些试图让内核在给定系统上以最佳状态运行的人，以及寻求内核用户空间 API 信息的应用程序开发人员'),
        ('最佳状态运行的人，以及寻求内核用户空间 API 信息的应用程序开发人员', ''),  # duplicate, remove
        ('有几份未分类的文档似乎不适合放在文档的其他部分，或者可能需要进行一', '有几份未分类的文档似乎不适合放在文档的其他部分，或者可能需要进行一些调整'),
        ('调整或转换为 reStructuredText 格式，或者干脆太旧了', ''),  # duplicate, remove
    ]

    for old, new in fixes:
        if old in content:
            content = content.replace(old, new)
            print('  index.md: fixed "{}..."'.format(old[:40]))

    with open(fpath, 'w', encoding='utf-8') as f:
        f.write(content)
    files_fixed.append(fpath)

# Fix watchdog/index.md
fpath = '.\\watchdog\\index.md'
if os.path.exists(fpath):
    with open(fpath, 'r', encoding='utf-8') as f:
        content = f.read()

    if '## 看门狗（watchdog' in content:
        content = content.replace('## 看门狗（watchdog', '## 看门狗（watchdog）')
        print('  watchdog/index.md: fixed truncated heading')
        files_fixed.append(fpath)
        with open(fpath, 'w', encoding='utf-8') as f:
            f.write(content)

# Fix w1/slaves/index.md
fpath = '.\\w1\\slaves\\index.md'
if os.path.exists(fpath):
    with open(fpath, 'r', encoding='utf-8') as f:
        content = f.read()

    fixes = [
        ('## 1-Wire 从设备驱', '## 1-Wire 从设备驱动'),
        ('本页1-Wire 从设备驱动的索引', '本页面为1-Wire 从设备驱动的索引'),
        ('列DS2406', '列出了DS2406'),
    ]

    for old, new in fixes:
        if old in content:
            content = content.replace(old, new)
            print('  w1/slaves/index.md: fixed "{}..."'.format(old[:40]))
            files_fixed.append(fpath)

    with open(fpath, 'w', encoding='utf-8') as f:
        f.write(content)

# Fix hwmon/index.md - fix garbled heading
fpath = '.\\hwmon\\index.md'
if os.path.exists(fpath):
    with open(fpath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Replace garbled heading
    if '纭' in content and '欢鐩' in content:
        content = content.replace('纭欢鐩戞帶', '硬件监控')
        print('  hwmon/index.md: fixed garbled heading')
        files_fixed.append(fpath)

    with open(fpath, 'w', encoding='utf-8') as f:
        f.write(content)

# Fix _w2_test_copy.md - verify it matches PROJECT.md (already fixed)
# No additional fixes needed since both had the same 鈥? pattern

print('\nFiles fixed: {}'.format(files_fixed))
