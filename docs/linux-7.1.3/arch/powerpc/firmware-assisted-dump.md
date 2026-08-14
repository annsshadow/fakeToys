## 固件辅助转储（Firmware-Assisted Dump）

2011 年 7 月

固件辅助转储的目标，是在一个完全复位后的系统上实现对崩溃系统的转储，
并最小化系统恢复生产使用所需的总耗时。

- 固件辅助转储（FADump）基础设施旨在取代现有的 phyp 辅助转储。
- FADump 使用与 phyp 辅助转储相同的固件接口与内存保留模型。
- 与 phyp dump 不同，FADump 通过 /proc/vmcore 以 ELF 格式导出内存转储，
  方式与 kdump 相同。这有助于我们复用 kdump 基础设施来进行转储捕获与过滤。
- 与 phyp dump 不同，用户空间工具在读取 /proc/vmcore 时无需引用任何 sysfs 接口。
- 与 phyp dump 不同，FADump 允许用户通过单步操作 `echo 1 > /sys/kernel/fadump_release_mem`
  来释放为转储保留的所有内存。
- 一旦通过内核启动参数启用，FADump 可通过 /sys/kernel/fadump_registered 接口
  （参见下文的 sysfs 文件小节）启动/停止，并可轻松地与 kdump 服务的启动/停止 init 脚本集成。

与 kdump 或其它策略相比，固件辅助转储提供了若干强大且实用的优势：

- 与 kdump 不同，系统已被复位，并加载了一份全新的内核副本。特别地，
  PCI 和 I/O 设备已被重新初始化，处于干净、一致的状态。
- 一旦转储被复制出来，持有转储的内存立即对运行中的内核可用。因此，与 kdump 不同，
  FADump 不需要第二次重启来将系统恢复到生产配置。

上述目标只能通过 Power 固件的协调与协助来实现。其流程如下：

- 第一个内核在 OS 初始化期间，向 Power 固件注册用于转储保留的内存段。
  这些被注册的内存段由第一个内核在早期启动期间保留。

- 当系统崩溃时，Power 固件会将已注册的低位内存区域（启动内存）从源区复制到目标区。
  它还会保存硬件 PTE。

  注意：
        术语“boot memory（启动内存）”是指一个内核在受限内存下成功启动所需的
        低位内存块大小。默认情况下，启动内存大小为系统 RAM 的 5% 与 256MB 中的较大者。
        或者，用户也可以通过启动参数 'crashkernel=' 指定启动内存大小，以覆盖默认计算值。
        若默认启动内存大小不足以让第二个内核成功启动，请使用此选项。关于 crashkernel=
        参数的语法，请参阅 Documentation/admin-guide/kdump/kdump.rst。如果在 crashkernel=
        参数中提供了任何偏移，它将被忽略，因为 FADump 使用预定义的偏移来为启动内存转储保留
        内存，以应对崩溃情况。

- 低位内存（启动内存）区域被保存后，固件将复位 PCI 及其它硬件状态。它**不会**清除 RAM。
  随后它会像平常一样启动引导加载程序。

- 全新启动的内核会注意到设备树中有一个新节点（pSeries 上为 rtas/ibm,kernel-dump，
  或在 OPAL 平台上的 ibm,opal/dump/mpipl-boot），表明存在来自上一次启动的崩溃数据。
  在早期启动期间，OS 将保留启动内存大小以上的其余内存，从而有效地以受限内存大小启动。
  这将确保该内核（也称为第二个内核或捕获内核）不会触碰任何转储内存区域。

- 用户空间工具将读取 /proc/vmcore 以获取内存内容，其中以前崩溃内核的转储以 ELF 格式保存。
  用户空间工具可按需将此信息复制到磁盘、网络、nas、san、iscsi 等。

- 一旦用户空间工具完成转储保存，它会向 /sys/kernel/fadump_release_mem 写入 '1'，
  将保留的内存释放回一般使用，保留下一次固件辅助转储注册所需的内存除外。

```

     # echo 1 > /sys/kernel/fadump_release_mem

```
请注意，固件辅助转储特性仅在 pSeries（PowerVM）平台上的 POWER6 及更高系统，
以及 PowerNV（OPAL）平台上 OP940 或更高固件版本的 POWER9 及更高系统上可用。
注意，当 PowerNV 平台支持 FADump 时，OPAL 固件会导出 ibm,opal/dump 节点。

在基于 OPAL 的机器上，系统会在启动到捕获内核之前先启动一个中间内核
（称为 petitboot 内核）。该内核具有最小的内核和/或用户空间支持来处理崩溃数据。
这样的内核需要为后续的捕获内核启动保留先前崩溃内核的内存以处理此崩溃数据。
必须在此类内核上启用内核配置选项 CONFIG_PRESERVE_FA_DUMP，以确保崩溃数据被保留供后续处理。

-- 在基于 OPAL 的机器（PowerNV）上，如果内核以 CONFIG_OPAL_CORE=y 构建，
崩溃时的 OPAL 内存也会作为 /sys/firmware/opal/mpipl/core 文件导出。此 procfs 文件
有助于用 GDB 调试 OPAL 崩溃。用于导出此 procfs 文件的内核内存可通过向
/sys/firmware/opal/mpipl/release_core 节点写入 '1' 来释放。

   e.g.
     # echo 1 > /sys/firmware/opal/mpipl/release_core

-- Fadump 中附加内核参数的支持
   Fadump 有一项特性，允许向 fadump 内核传递附加的内核参数。该特性主要设计用于
   禁用 fadump 内核不需要的内核功能，并在收集转储时减少其内存占用。

  向 Fadump 添加附加内核参数的命令：
  e.g.
  # echo "nr_cpus=16" > /sys/kernel/fadump/bootargs_append

  上述命令足以向 fadump 添加附加参数。不需要显式重启服务。

  检索附加 Fadump 参数的命令：
  e.g.
  # cat /sys/kernel/fadump/bootargs_append

注意：使用 HASH MMU 的 fadump 附加内核参数仅在 RMA 大小大于 768 MB 时受支持。
如果 RMA 大小小于 768 MB，内核不会导出 /sys/kernel/fadump/bootargs_append sysfs 节点。

### 实现细节：

在启动期间，会检查固件是否在该特定机器上支持此特性。如果支持，则检查是否有
等待处理的活跃转储。如果有，则在早期启动期间保留除启动内存大小以外的全部 RAM
（参见图 2）。一旦我们完成从用户空间脚本（例如 kdump 脚本）收集转储，该区域即被释放。
如果有转储数据，则会创建 /sys/kernel/fadump_release_mem 文件，并持有保留的内存。

如果没有等待处理的转储数据，则通常仅在大于启动内存大小的偏移处保留用于保存 CPU 状态、
HPTE 区域、启动内存转储以及 FADump 头的内存（参见图 1）。该区域**不会**被释放：
此区域将永久保留，以便在不发生崩溃的正常情况下，它可作为引导内存内容副本的接收容器，
此外还容纳 CPU 状态与 HPTE 区域。

由于此保留内存区域仅在系统崩溃后才被使用，将这一大块内存从生产内核中隔离出来没有意义。
因此，如果内核配置了 CMA，实现使用 Linux 内核的连续内存分配器（CMA）来进行内存保留。
通过 CMA 保留，此内存可供应用程序使用，同时内核被阻止使用它。借助 FADump，仍将能够
捕获全部内核内存以及大部分用户空间内存，但用户页除外
```

  o Memory Reservation during first kernel

  Low memory                                                  Top of memory
  0    boot memory size   |<------ Reserved dump area ----->|     |
  |           |           |      Permanent Reservation      |     |
  V           V           |                                 |     V
  +-----------+-----/ /---+---+----+-----------+-------+----+-----+
  |           |           |///|////|    DUMP   |  HDR  |////|     |
  +-----------+-----/ /---+---+----+-----------+-------+----+-----+
        |                   ^    ^       ^         ^      ^
        |                   |    |       |         |      |
        \                  CPU  HPTE     /         |      |
         --------------------------------          |      |
      Boot memory content gets transferred         |      |
      to reserved area by firmware at the          |      |
      time of crash.                               |      |
                                           FADump Header  |
                                            (meta area)   |
                                                          |
                                                          |
                      Metadata: This area holds a metadata structure whose
                      address is registered with f/w and retrieved in the
                      second kernel after crash, on platforms that support
                      tags (OPAL). Having such structure with info needed
                      to process the crashdump eases dump capture process.

                   Fig. 1


  o Memory Reservation during second kernel after crash

  Low memory                                              Top of memory
  0      boot memory size                                      |
  |           |<------------ Crash preserved area ------------>|
  V           V           |<--- Reserved dump area --->|       |
  +----+---+--+-----/ /---+---+----+-------+-----+-----+-------+
  |    |ELF|  |           |///|////|  DUMP | HDR |/////|       |
  +----+---+--+-----/ /---+---+----+-------+-----+-----+-------+
       |   |  |                            |     |             |
       -----  ------------------------------     ---------------
         \              |                               |
           \            |                               |
             \          |                               |
               \        |    ----------------------------
                 \      |   /
                   \    |  /
                     \  | /
                  /proc/vmcore


        +---+
        |///| -> Regions (CPU, HPTE & Metadata) marked like this in the above
        +---+    figures are not always present. For example, OPAL platform
                 does not have CPU & HPTE regions while Metadata region is
                 not supported on pSeries currently.

        +---+
        |ELF| -> elfcorehdr, it is created in second kernel after crash.
        +---+

        Note: Memory from 0 to the boot memory size is used by second kernel

                   Fig. 2


```
当前，转储将在用户干预下从 /proc/vmcore 复制到新文件。通过 /proc/vmcore 可用的转储数据
将为 ELF 格式。因此，经轻微修改后，现有的 kdump 基础设施（kdump 脚本）用于保存转储即可正常工作。
主流发行版上的 KDump 脚本已被修改，以在将 FADump 用作转储机制（而非 KDump）时无缝工作
（保存转储无需用户干预）。

用于检查转储的工具将与用于 kdump 的相同。

### 如何启用固件辅助转储（FADump）：

1. 设置配置选项 CONFIG_FA_DUMP=y 并构建内核。
2. 以 'fadump=on' 内核命令行选项启动进入 Linux 内核。
   默认情况下，FADump 保留内存将被初始化为 CMA 区域。
   或者，用户可以以 'fadump=nocma' 启动 Linux 内核，以防止 FADump 使用 CMA。
3. 用户还可以可选地设置 'crashkernel=' 内核命令行，以指定为启动内存转储保留而保留的内存大小。

注意：
     1. 'fadump_reserve_mem=' 参数已被弃用。请改用 'crashkernel=' 指定为启动内存转储保留的
        内存大小。
     2. 如果固件辅助转储无法保留内存，那么若内核命令行设置了 'crashkernel=' 选项，
        它将回退到现有的 kdump 机制。
     3. 如果用户希望捕获全部用户空间内存，且可以接受保留内存对生产系统不可用，则可以使用
        'fadump=nocma' 内核参数回退到旧的行为。

### sysfs/debugfs 文件：

固件辅助转储特性使用 sysfs 文件系统保存控制文件，并使用 debugfs 文件显示保留的内存区域。

以下是内核 sysfs 下的文件列表：

 /sys/kernel/fadump_enabled
    此文件用于显示 FADump 状态。

    - 0 = FADump 已禁用
    - 1 = FADump 已启用

    此接口可被 kdump init 脚本用来识别内核中是否启用了 FADump，并据此采取行动。

 /sys/kernel/fadump_registered
    此文件用于显示 FADump 注册状态，以及控制（启动/停止）FADump 注册。

    - 0 = FADump 未注册。
    - 1 = FADump 已注册，并准备好处理系统崩溃。

    要注册 FADump，写入 echo 1 > /sys/kernel/fadump_registered；要注销并停止 FADump，
    写入 echo 0 > /sys/kernel/fadump_registered。一旦 FADump 被注销，系统崩溃将不会被处理，
    也不会捕获 vmcore。此接口可轻松与 kdump 服务的启动/停止集成。

 /sys/kernel/fadump/mem_reserved

   此文件用于显示 FADump 为保存崩溃转储而保留的内存。

 /sys/kernel/fadump_release_mem
    此文件仅在第二个内核期间 FADump 处于活跃状态时可用。它用于释放为保存崩溃转储而持有的
    保留内存区域。要释放
```

	echo 1  > /sys/kernel/fadump_release_mem

    在 echo 1 之后，/sys/kernel/debug/powerpc/fadump_region 文件的内容将改变以反映新的
    内存保留。

    现有的用户空间工具（kdump 基础设施）可轻松增强，以使用此接口释放为转储保留的内存，
    并在无需第二次重启的情况下继续。

```
注意：/sys/kernel/fadump_release_opalcore sysfs 已移至
      /sys/firmware/opal/mpipl/release_core

 /sys/firmware/opal/mpipl/release_core

    此文件仅在基于 OPAL 的机器上、捕获内核期间 FADump 处于活跃状态时可用。它用于释放
    内核用于导出 /sys/firmware/opal/mpipl/core 文件的内存。要释放此内存，向其写入 '1'：

    echo 1  > /sys/firmware/opal/mpipl/release_core

注意：以下 FADump sysfs 文件已被弃用。

+----------------------------------+--------------------------------+
| Deprecated                       | Alternative                    |
+----------------------------------+--------------------------------+
| /sys/kernel/fadump_enabled       | /sys/kernel/fadump/enabled     |
+----------------------------------+--------------------------------+
| /sys/kernel/fadump_registered    | /sys/kernel/fadump/registered  |
+----------------------------------+--------------------------------+
| /sys/kernel/fadump_release_mem   | /sys/kernel/fadump/release_mem |
+----------------------------------+--------------------------------+

以下是 powerpc debugfs 下的文件列表：
（假定 debugfs 挂载在 /sys/kernel/debug 目录下。）

 /sys/kernel/debug/powerpc/fadump_region
    如果启用了 FADump，此文件显示保留的内存区域，否则此文件为空。输出格式
```

      <region>: [<start>-<end>] <reserved-size> bytes, Dumped: <dump-size>

    而内核 DUMP 区域的格式为：

    DUMP: Src: <src-addr>, Dest: <dest-addr>, Size: <size>, Dumped: # bytes

    e.g.
    Contents when FADump is registered during first kernel::

      # cat /sys/kernel/debug/powerpc/fadump_region
      CPU : [0x0000006ffb0000-0x0000006fff001f] 0x40020 bytes, Dumped: 0x0
      HPTE: [0x0000006fff0020-0x0000006fff101f] 0x1000 bytes, Dumped: 0x0
      DUMP: [0x0000006fff1020-0x0000007fff101f] 0x10000000 bytes, Dumped: 0x0

    Contents when FADump is active during second kernel::

      # cat /sys/kernel/debug/powerpc/fadump_region
      CPU : [0x0000006ffb0000-0x0000006fff001f] 0x40020 bytes, Dumped: 0x40020
      HPTE: [0x0000006fff0020-0x0000006fff101f] 0x1000 bytes, Dumped: 0x1000
      DUMP: [0x0000006fff1020-0x0000007fff101f] 0x10000000 bytes, Dumped: 0x10000000
          : [0x00000010000000-0x0000006ffaffff] 0x5ffb0000 bytes, Dumped: 0x5ffb0000


```
注意：
      关于如何挂载 debugfs 文件系统，请参阅 Documentation/filesystems/debugfs.rst。


### 待办：

 - 需要提出更好的方法，以找出在受限内存下成功启动内核所需的更准确的启动内存大小。

作者：Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>

本文档基于 Linas Vepstas 和 Manish Ahuja 为 phyp 辅助转储所写的原始文档。
