
## Linuxized ACPICA —— ACPICA 发布自动化简介

:Copyright: |copy| 2013-2016, Intel Corporation

:Author: Lv Zheng <lv.zheng@intel.com>


## Abstract（摘要）

本文档描述了 ACPICA 项目以及 ACPICA 与 Linux 之间的关系。它也描述了 drivers/acpi/acpica、include/acpi 和 tools/power/acpi 中的 ACPICA 代码是如何被自动更新以跟随上游的。

## ACPICA Project（ACPICA 项目）

ACPI 组件架构（Advanced Configuration and Power Interface Specification，ACPICA）项目提供了一个操作系统（OS）无关的、关于高级配置与电源接口规范（ACPI）的参考实现。它已被各种宿主操作系统所采纳。通过直接集成 ACPICA，Linux 也能从 ACPICA 在其他宿主操作系统上的应用经验中受益。

ACPICA 项目的主页是：www.acpica.org，它由 Intel Corporation 维护和支持。

下图描绘了 Linux ACPI 子系统，其中 ACPICA
```

      +---------------------------------------------------------+
      |                                                         |
      |   +---------------------------------------------------+ |
      |   | +------------------+                              | |
      |   | | Table Management |                              | |
      |   | +------------------+                              | |
      |   | +----------------------+                          | |
      |   | | Namespace Management |                          | |
      |   | +----------------------+                          | |
      |   | +------------------+       ACPICA Components      | |
      |   | | Event Management |                              | |
      |   | +------------------+                              | |
      |   | +---------------------+                           | |
      |   | | Resource Management |                           | |
      |   | +---------------------+                           | |
      |   | +---------------------+                           | |
      |   | | Hardware Management |                           | |
      |   | +---------------------+                           | |
      |   +---------------------------------------------------+ | |
      | | |                            +------------------+ | | |
      | | |                            | OS Service Layer | | | |
      | | |                            +------------------+ | | |
      | | +-------------------------------------------------|-+ |
      | |   +--------------------+                          |   |
      | |   | Device Enumeration |                          |   |
      | |   +--------------------+                          |   |
      | |   +------------------+                            |   |
      | |   | Power Management |                            |   |
      | |   +------------------+     Linux/ACPI Components  |   |
      | |   +--------------------+                          |   |
      | |   | Thermal Management |                          |   |
      | |   +--------------------+                          |   |
      | |   +--------------------------+                    |   |
      | |   | Drivers for ACPI Devices |                    |   |
      | |   +--------------------------+                    |   |
      | |   +--------+                                      |   |
      | |   | ...... |                                      |   |
      | |   +--------+                                      |   |
      | +---------------------------------------------------+   |
      |                                                         |
      +---------------------------------------------------------+

                 Figure 1. Linux ACPI Software Components

```
    A. OS Service Layer —— 由 Linux 提供，用于给出预定义 ACPICA 接口（acpi_os_*）的 OS 相关实现。
```
         include/acpi/acpiosxf.h
         drivers/acpi/osl.c
         include/acpi/platform
         include/asm/acenv.h
    B. ACPICA Functionality —— 从 ACPICA 代码库发布，用于给出 ACPICA 接口（acpi_*）的 OS 无关实现。
       ::

         drivers/acpi/acpica
         include/acpi/ac*.h
         tools/power/acpi
    C. Linux/ACPI Functionality —— 向其他 Linux 内核子系统以及用户空间程序提供 Linux 特定的 ACPI 功能。
       ::

         drivers/acpi
         include/linux/acpi.h
         include/linux/acpi*.h
         include/acpi
         tools/power/acpi
    D. Architecture Specific ACPICA/ACPI Functionalities —— 由 ACPI 子系统提供，用于给出 ACPI 接口的架构相关实现。它们是 Linux 特定的组件，不在本文档范围内。
       ::

         include/asm/acpi.h
         include/asm/acpi*.h
         arch/*/acpi

```
## ACPICA Release（ACPICA 发布）

ACPICA 项目在其以下仓库 URL 维护代码库：https://github.com/acpica/acpica.git。按惯例，每月发布一次。

由于 ACPICA 项目所采用的编码风格不被 Linux 接受，因此存在一套发布流程，将 ACPICA 的 git 提交转换为 Linux 补丁。该流程生成的补丁被称为 “linuxized ACPICA patches”（Linux 化的 ACPICA 补丁）。该发布流程在 ACPICA git 仓库的一份本地副本上进行。每月发布中的每个提交都被转换为一个 linuxized ACPICA 补丁。它们共同构成了面向 Linux ACPI 社区的每月 ACPICA 发布补丁集。此流程
```

    +-----------------------------+
    | acpica / master (-) commits |
    +-----------------------------+
       /|\         |
        |         \|/
        |  /---------------------\    +----------------------+
        | < Linuxize repo Utility >-->| old linuxized acpica |--+
        |  \---------------------/    +----------------------+  |
        |                                                       |
     /---------\                                                |
    < git reset >                                                \
     \---------/                                                  \
       /|\                                                        /+-+
        |                                                        /   |
    +-----------------------------+                             |    |
    | acpica / master (+) commits |                             |    |
    +-----------------------------+                             |    |
                   |                                            |    |
                  \|/                                           |    |
         /-----------------------\    +----------------------+  |    |
        < Linuxize repo Utilities >-->| new linuxized acpica |--+    |
         \-----------------------/    +----------------------+       |
                                                                    \|/
    +--------------------------+                  /----------------------\
    | Linuxized ACPICA Patches |<----------------< Linuxize patch Utility >
    +--------------------------+                  \----------------------/
                   |
                  \|/
     /---------------------------\
    < Linux ACPI Community Review >
     \---------------------------/
                   |
                  \|/
    +-----------------------+    /------------------\    +----------------+
    | linux-pm / linux-next |-->< Linux Merge Window >-->| linux / master |
    +-----------------------+    \------------------/    +----------------+

                Figure 2. ACPICA -> Linux Upstream Process

```
    A. Linuxize Utilities —— 由 ACPICA 仓库提供，包括位于 source/tools/acpisrc 文件夹中的一个实用程序，以及位于 generate/linux 文件夹中的若干脚本。
    B. acpica / master —— 位于 <https://github.com/acpica/acpica.git> 的 git 仓库的 “master” 分支。
    C. linux-pm / linux-next —— 位于 <https://git.kernel.org/pub/scm/linux/kernel/git/rafael/linux-pm.git> 的 git 仓库的 “linux-next” 分支。
    D. linux / master —— 位于 <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git> 的 git 仓库的 “master” 分支。

   在 linuxized ACPICA 补丁被发送给 Linux ACPI 社区审查之前，有一个质量保证的构建测试流程，用以减少移植问题。目前此构建流程只照顾以下内核配置选项：
   CONFIG_ACPI/CONFIG_ACPI_DEBUG/CONFIG_ACPI_DEBUGGER

## ACPICA Divergences（ACPICA 分歧）

理想情况下，所有 ACPICA 提交都应当被自动转换为 Linux 补丁而无需手动修改，“linux / master” 树应当包含与 “new linuxized acpica” 树中所含 ACPICA 代码精确对应的 ACPICA 代码，并且应当可以完全自动地运行发布流程。

然而，事实上，Linux 中的 ACPICA 代码与上游 ACPICA 代码之间存在源代码差异，这被称为 “ACPICA Divergences”（ACPICA 分歧）。

ACPICA 分歧的各种来源包括：
   1. 遗留分歧（Legacy divergences）—— 在当前的 ACPICA 发布流程建立之前，Linux 与 ACPICA 之间就已经存在分歧。过去几年中这些分歧已被大幅减少，但仍有若干存在，并且需要时间来找出它们存在背后的根本原因。
   2. 手动修改（Manual modifications）—— 任何直接在 Linux 源码中做的手动修改（例如编码风格修正）显然会损害 ACPICA 发布自动化。因此建议在上游 ACPICA 源码中修复此类问题，并使用 ACPICA 发布实用程序生成 linuxized 修复（详见下文第 4 节）。
   3. Linux 特定功能（Linux specific features）—— 有时无法使用当前的 ACPICA API 来实现 Linux 内核所需的功能，因此 Linux 开发者偶尔不得不直接修改 ACPICA 代码。这些修改可能不被上游 ACPICA 接受，在这种情况下，除非 ACPICA 一方能够实现新的机制来替代它们，否则它们会作为已提交的 ACPICA 分歧保留下来。
   4. ACPICA 发布修复（ACPICA release fixups）—— ACPICA 只使用一组用户空间模拟实用程序来测试提交，因此 linuxized ACPICA 补丁可能会破坏 Linux 内核，给我们留下构建/启动失败。为了避免破坏 Linux 的二分（bisection），在发布流程中会将修复直接应用到 linuxized ACPICA 补丁上。当这些发布修复被反向移植到上游 ACPICA 源码时，它们必须遵循上游 ACPICA 的规则，因此可能会出现进一步的修改。这可能导致新分歧的出现。
   5. ACPICA 提交快速跟踪（Fast tracking of ACPICA commits）—— 某些 ACPICA 提交是回归修复或稳定候选材料，因此会相对于 ACPICA 发布流程而提前应用。如果此类提交在 ACPICA 一方被回退或变基，以提供更优的解决方案，就会生成新的 ACPICA 分歧。

## ACPICA Development（ACPICA 开发）

本段引导 Linux 开发者使用 ACPICA 上游发布实用程序，在它们从 ACPICA 发布流程可用之前，获取对应于上游 ACPICA 提交的 Linux 补丁。

   1. Cherry-pick 一个 ACPICA 提交

   首先您需要 git clone 该 ACPICA 仓库，并且您想要 cherry-pick 的 ACPICA 修改必须已提交到本地仓库。

   然后 gen-patch.sh 命令可以帮助 cherry-pick 一个 ACPICA 提交
```

   $ git clone https://github.com/acpica/acpica
   $ cd acpica
   $ generate/linux/gen-patch.sh -u [commit ID]

   这里的 commit ID 是您想要 cherry-pick 的 ACPICA 本地仓库提交 ID。如果提交是 “HEAD”，则可以省略。

   2. Cherry-pick 最近的 ACPICA 提交

   有时您需要将代码变基到尚未应用到 Linux 的最新 ACPICA 修改之上。

   您可以自己生成 ACPICA 发布系列，并将代码变基到生成的 ACPICA 发布补丁之上：：

   $ git clone https://github.com/acpica/acpica
   $ cd acpica
   $ generate/linux/make-patches.sh -u [commit ID]

   该 commit ID 应当是 Linux 接受的最后一个 ACPICA 提交。通常，它是修改 ACPI_CA_VERSION 的提交。它可以通过执行 "git blame source/include/acpixf.h" 并参考包含 "ACPI_CA_VERSION" 的那一行来找到。

   3. 检查当前的分歧

   如果您同时拥有 Linux 和上游 ACPICA 的本地副本，您可以生成一个 diff 文件，指示当前分歧的状态：：

   # git clone https://github.com/acpica/acpica
   # git clone https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git
   # cd acpica
   # generate/linux/divergence.sh -s ../linux

```
