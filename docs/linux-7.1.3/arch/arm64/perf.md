
## Perf


## Perf 事件属

:Author: Andrew Murray <andrew.murray@arm.com>
:Date: 2019-03-06

### exclude_user


该属性排除用户空间
用户空间始终运行EL0，因此该属性会排除 EL0

### exclude_kernel


该属性排除内核
内核在开VHE 时运行在 EL2，否则运行在 EL1。客户机内核始终运行EL1
对于宿主机，该属性会排除 EL1，在 VHE 系统上还会额外排EL2
对于客户机，该属性会排除 EL1。请注意，EL2 在客户机内部永远不会被计数

### exclude_hv


该属性排除管理程序（hypervisor）
对于 VHE 宿主机，该属性被忽略，因为我们将宿主机内核视为管理程序
对于VHE 宿主机，该属性会排除 EL2，因为我们将管理程序视为任何运行EL2 的代码，EL2 主要用于客户宿主机切换
对于客户机，该属性不起作用。请注意，EL2 在客户机内部永远不会被计数

### exclude_host / exclude_guest


这些属性分别排KVM 宿主机和客户机
KVM 宿主机可能运行在 EL0（用户空间）、EL1（非 VHE 内核）和 EL2（VHE 内核或非 VHE 管理程序）
KVM 客户机可能运行在 EL0（用户空间）EL1（内核）
由于宿主机与客户机之间的异常级别存在重叠，我们不能完全依PMU 的硬件异常过滤——因此必须在进入和退出客户机时启禁用计数。这VHE 和非 VHE 系统上的做法不同
对于VHE 系统，我们对 exclude_host 排除 EL2——在进入和退出客户机时，我们根据 exclude_host exclude_guest 属性适当地禁启用事件
对于 VHE 系统，我们对 exclude_guest 排除 EL1，对 exclude_host 排除 EL0 EL2。在进入和退出客户机时，我们根据 exclude_host exclude_guest 属性修改事件，以适当地包排除 EL0
上述说明同样适用于在VHE 客户机内部使用这些属性的情况，但请注意，EL2 在客户机内部永远不会被计数

### 鍑嗙‘鎬。

在非 VHE 宿主机上，我们会EL2 处的宿主客户机切换进入与退出时启用/禁用计数器——但在启禁用计数器与进入/退出客户机之间存在一段时间间隔。通过在计数客户机事件时过滤掉 EL2（exclude_host），我们可以消除在客户机进入/退出边界上计数宿主事件的状况。然而，当使!exclude_hv 时，在客户机进入/退出的边界会存在一个小的盲区窗口，期间宿主事件不会被捕获
VHE 系统上没有盲区窗口
## Perf 用户空间 PMU 硬件计数器访

### 概述

perf 用户空间工具依赖 PMU 来监视事件。由于底层实现与 CPU 相关，它在硬件计数器之上提供了一个抽象层Arm64 允许用户空间工具直接访问存储硬件计数器值的寄存器
这专门面向自监控任务，目的是通过直接访问寄存器（无需经由内核）来降低开销
### 使用方法

重点在于 armv8 PMUv3，它确保PMU 寄存器的访问被启用，并且用户空间能够获取使用它们所需的相关信息
为了能够访问硬件计数器，必须先启用全局 sysctl 参数 kernel/perf_user_access

  echo 1 > /proc/sys/kernel/perf_user_access

必须使用设置config1:1 属性位perf 工具接口打开事件：sys_perf_event_open 系统调用会返回一fd，随后可配合 mmap 系统调用使用，以获取包含该事件信息的一页内存。PMU 驱动利用这一页向用户暴露硬件计数器的索引及其他必要数据。借助该索引，用户可以使用 `mrs` 指令访问 PMU 寄存器。对 PMU 寄存器的访问仅在顺序锁（sequence lock）未改变时有效。尤其是，每次顺序锁发生变化时，PMSELR_EL0 寄存器都会被清零
用户空间访问libperf 中通过 perf_evsel__mmap() perf_evsel__read() 函数获得支持。示例见 `tools/lib/perf/tests/test-evsel.c`_
### 关于异构系统

big.LITTLE 之类的异构系统上，只有当任务被绑定（pinned）到一组同构的核心子集，并且通过指定 'type' 属性打开相应PMU 实例时，用户空间 PMU 计数器访问才能被启用。这种情况下不支持使用通用事件类型
示例`tools/perf/arch/arm64/tests/user-events.c`_。可以用 perf 工具运行它，以检查从用户空间访问寄存器是否正常：


  perf test -v user

### 关于链式事件与计数器大小

用户可以请求一32 位（config1:0 == 0）或 64 位（config1:0 == 1）的计数器，并配合用户空间访问。如果请求了 64 位计数器而硬件不支持 64 位计数器，sys_perf_event_open 系统调用会失败。链式事件不支持与用户空间计数器访问同时使用。如果在具有 64 位计数器的硬件上请求32 位计数器，那么用户空间必须把从该计数器读到的32 位视为未知（UNKNOWN）。用户页中的 'pmc_width' 字段会指明计数器的有效位宽，应当据此在需要时屏蔽高位
   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/perf/arch/arm64/tests/user-events.c
   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/lib/perf/tests/test-evsel.c

## 事件计数阈

### 概述


FEAT_PMUv3_TH（Armv8.8）允PMU 计数器仅在事件的计数值满足指定阈值条件时才递增。例如，threshold_compare 设为 2大于或等），且阈值设2，那PMU 计数器现在只会在某个事件原本会在单个处理器周期内PMU 计数器递增 2 或更多时才递增
若希望在通过阈值条件后1 递增，而不是按该周期内的事件数量递增，可在命令行中加'threshold_count' 选项
### 使用方法


以下是用于控制该特性的参数
   :header-rows: 1

   - - 参数
     - 说明
   - - threshold
     - 用于对该事件设置阈值的取值。值为 0 表示禁用阈值功能，其他参数不起作用   - - threshold_compare
     - | 要使用的比较函数，支持以下取值：
       |
       | 0: 不等       | 1: 等于
       | 2: 大于或等       | 3: 小于
   - - threshold_count
     - 若设置了该项，则在通过阈值条件后1 计数，而不是按该周期内的事件取值计数
threshold、threshold_compare threshold_count 的值可以按每个事件分别提供，例如：


  perf stat -e stall_slot/threshold=2,threshold_compare=2/ \
            -e dtlb_walk/threshold=10,threshold_compare=3,threshold_count/

在此示例中，stall_slot 事件会在每个发生 2 次或更多停顿的周期内2 或更多计数。dtlb_walk 会在每个 dtlb 行走次数少于 10 的周期内1 计数
支持的最大阈值可以从每个 PMU caps 中读取，例如

  cat /sys/bus/event_source/devices/armv8_pmuv3/caps/threshold_max

  0x000000ff

如果给出高于此值的值，打开该事件会导致错误。最大可能的最大值为 4095，因为阈值的 config 字段限制12 位，Perf 工具会拒绝解析更大的值
如果 PMU 不支FEAT_PMUv3_TH，则 threshold_max 读为 0，并且尝试设置阈值也会导致错误。即使在支持该特性的硬件上运行，threshold_max aarch32 客户机上也会读为 0