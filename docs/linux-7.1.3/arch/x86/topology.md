
## x86 拓扑


本文档记录并阐明x86 拓扑在内核中的建模与表示的主要方面。在对相应代码进行更改时，请同步更新/修改本文档
架构无关的拓扑定义位Documentation/admin-guide/cputopology.rst。本文件保存的是 x86 特有的差特殊性，这些不一定适用于通用定义。因此，x86 上了Linux 拓扑的方法是：先阅读通用定义，同时对照本文件查看 x86 特有的部分
不用说，代码应当使用通用的函数——本文件**仅仅**是为*记录** x86 拓扑的内部运作机制
Thomas Gleixner <tglx@kernel.org> Borislav Petkov <bp@alien8.de> 发起
拓扑设施的主要目标是向那些需要了查询/使用运行系统的结构（涉及线程、核、封装等）的代码，提供恰当的接口
内核并不关心物理插槽（socket）这一概念，因为插槽与软件无关，它只是一个机电组件。过去一个插槽总是包含一个封装（见下文），但随着多芯片模块（MCM）的出现，一个插槽可以容纳多个封装。因此代码中可能仍有对插槽的引用，但它们属于历史遗留，应当被清理掉
系统的拓扑用以下单位描述
    - 封装（packages    - 核（cores    - 线程（threads
## 封装（Package

封装包含一个或多个核以及共享资源，例如 DRAM 控制器、共享缓存等
现代系统也可能用术语 “Die来表示封装
AMD 对封装的术语“Node”
内核中与封装相关的拓扑信息：

  - topology_num_threads_per_package()

    一个封装中的线程数量
  - topology_num_cores_per_package()

    一个封装中的核数量
  - topology_max_dies_per_package()

    一个封装中 die 的最大数量
  - cpuinfo_x86.topo.die_id:

    die 的物ID
  - cpuinfo_x86.topo.pkg_id:

    封装的物ID。该信息通过 CPUID 获取，并由封装中各个核的 APIC ID 推导而来
    现代系统将此值用于插槽。一个插槽内可能存在多个封装。该值可能与 topo.die_id 不同
  - cpuinfo_x86.topo.logical_pkg_id:

    封装的逻辑 ID。由于我们不信任 BIOS 以一致的方式枚举封装，因此引入了逻辑封装 ID 的概念，这样我们就能合理地计算出系统中最大可能的封装数量，并让封装被线性枚举
  - topology_max_packages():

    系统中可能的封装最大数量。对于按封装的设施而言，可用于预分配每个封装的信息
  - cpuinfo_x86.topo.llc_id:

      - Intel 上，是共享末级缓存（Last Level Cache）的 CPU 列表中的第一APIC ID
      - AMD 上，是包含末级缓存的 Node ID Core Complex ID。一般来说，它是一个能在系统上唯一标识一LLC 的编号
## 核（Cores

一个核1 个或多个线程组成。线程是 SMT 类型还是 CMT 类型并无影响
AMD CMT 核的术语“Compute Unit”。内核始终使“core”
## 线程（Threads

一个线程是一个单一的调度单元。它等价于一个逻辑 Linux CPU
AMD CMT 线程的术语是 “Compute Unit Core”。内核始终使“thread”
内核中与线程相关的拓扑信息：

  - topology_core_cpumask():

    cpumask 包含该线程所属封装中的所有在线线程
    在线线程的数量也会打印在 /proc/cpuinfo “siblings中
  - topology_sibling_cpumask():

    cpumask 包含该线程所属核中的所有在线线程
  - topology_logical_package_id():

    该线程所属的逻辑封装 ID
  - topology_physical_package_id():

    该线程所属的物理封装 ID
  - topology_core_id();

    该线程所属核ID。它也会打印/proc/cpuinfo “core_id中
  - topology_logical_core_id();

    该线程所属的逻辑ID


## 系统拓扑枚举


x86 系统上的拓扑可以通过组合各厂商特定的 CPUID 叶子（leaf）来发现，这些叶子枚举了处理器拓扑与缓存层次结构
x86 厂商在解析时优先顺序如下CPUID 叶子
1) AMD

   1) CPUID leaf 0x80000026 [Extended CPU Topology] (Core::X86::Cpuid::ExCpuTopology)

      扩展 CPUID 叶子 0x80000026 CPUID 叶子 0xB 的扩展，提供了每一层级Core、Complex、CCD（Die）和 Socket 的拓扑信息
      通过检查最大扩CPUID 级别是否 >= 0x80000026，然后检查特定层级（0 开始）`EBX[15:0]` 中的 `LogProcAtThisLevel` 是否非零，来发现对该叶子的支持
      该层级中 `ECX[15:8]` 里的 `LevelType` 给出了该层级所描述的拓扑域——Core、Complex、CCD（Die）或 Socket
      内核使用 `EAX[4:0]` 中的 `CoreMaskWidth` 来获知需要从 `EDX[31:0]` 中的 `ExtendedLocalApicId` 右移多少位，以得到该拓扑层级的唯一拓扑 ID。具有相同拓ID CPU 共享该层级的资源
      CPUID 叶子 0x80000026 还提供了关于功耗与效能等级、以及具有异构特性的 AMD 处理器上核类型方面的更多信息
      如果支持 CPUID 叶子 0x80000026，则无需进一步解析
   2) CPUID leaf 0x0000000B [Extended Topology Enumeration] (Core::X86::Cpuid::ExtTopEnum)

      扩展 CPUID 叶子 0x0000000B 是扩CPUID 叶子 0x80000026 的前身，仅描述处理器拓扑的核与插槽域
      通过检查最大支持的 CPUID 级别是否 >= 0xB，然后检查特定层级（0 开始）`EBX[31:0]` 是否非零，来发现对该叶子的支持
      该层级中 `ECX[15:8]` 里的 `LevelType` 给出了该层级所描述的拓扑域——Thread Processor（Socket）
      内核使用 `EAX[4:0]` 中的 `CoreMaskWidth` 来获知需要从 `EDX[31:0]` 中的 `ExtendedLocalApicId` 右移多少位，以得到该拓扑层级的唯一拓扑 ID。共享该拓扑 ID CPU 共享该层级的资源
      如果支持 CPUID 叶子 0xB，则无需进一步解析

   3) CPUID leaf 0x80000008 ECX [Size Identifiers] (Core::X86::Cpuid::SizeId)

      如果既不支持 CPUID 叶子 0x80000026 也不支持 0xB，则使用 Size Identifier 叶子 0x80000008 ECX 来检测封装上CPU 数量
      通过检查支持的扩展 CPUID 级别是否 >= 0x80000008，来发现对该叶子的支持
      `ECX[15:12]` 中的 `ApicIdSize` 字段非零，则APIC ID Socket ID 的位移量由该字段计算得出
      如果 `ApicIdSize` 报告为零，则位移量按 `ECX[7:0]` `NC` 字段（描述封装上 `线程- 1`）计算出`线程数` 的阶来计算
      除非支持 Extended APIC ID，否则用于查Socket ID APIC ID 来自 CPUID 叶子 0x00000001 `EBX[31:24]` 中的 `LocalApicId` 字段
      拓扑解析将继续检测是否支Extended APIC ID

   4) CPUID leaf 0x8000001E [Extended APIC ID, Core Identifiers, Node Identifiers]
      (Core::X86::Cpuid::{ExtApicId,CoreId,NodeId})

      可以通过检CPUID 叶子 0x80000001 [Feature Identifiers]
      (Core::X86::Cpuid::FeatureExtIdEcx) `ECX[^22^]` 中是否存`TopologyExtensions`，来检测对 Extended APIC ID 的支持
      如果支持 Topology Extensions，则应优先使CPUID 叶子 0x8000001E `EAX[31:0]` `ExtendedApicId` APIC ID，而非来自 CPUID 叶子 0x00000001 `EBX[31:24]` `LocalApicId` 字段APIC ID，用于拓扑枚举
      Family 0x17 及以上、且不支CPUID 叶子 0x80000026 CPUID 叶子 0xB 的处理器上，APIC ID Core ID 的位移量使用 `EBX[15:8]` `ThreadsPerCore` 字段（描`每核线程- 1`）计算出`每核线程数` 的阶来计算
      Family 0x15 的处理器上，`EBX[7:0]` 中的 Core ID 被用`cu_id`（Compute Unit ID），以检测共享计算单元的 CPU

   所有支`TopologyExtensions` 特性的 AMD 处理器都会将 CPUID 叶子 0x8000001E
   `ECX[7:0]` 中的 `NodeId` (Core::X86::Cpuid::NodeId) 存储为每 CPU `node_id`。在较旧的处理器上，`node_id` 是通过 MSR_FAM10H_NODE_ID MSR（MSR
   0x0xc001_100c）发现的。NODE_ID MSR 的存在是通过检CPUID 叶子 0x80000001 [Feature Identifiers]
   (Core::X86::Cpuid::FeatureExtIdEcx) `ECX[^19^]` 来检测的

2) Intel

   Intel 平台上，枚举处理器拓扑的 CPUID 叶子如下
   1) CPUID leaf 0x1F (V2 Extended Topology Enumeration Leaf)

      CPUID 叶子 0x1F CPUID 叶子 0xB 的扩展，提供了每一层级Core、Module、Tile、Die、DieGrp Socket 的拓扑信息
      通过检查支持的 CPUID 级别是否 >= 0x1F，然后特定层级（0 开始）`EBX[31:0]` 是否非零，来发现对该叶子的支持
      子叶子中 `ECX[15:8]` 里的 `Domain Type` 给出了该层级所描述的拓扑域——Core、Module、Tile、Die、DieGrp Socket
      内核使用 `EAX[4:0]` 中的值来获知需要从 `EDX[31:0]` 中的 `x2APIC ID` 右移多少位，以得到该拓扑层级的唯一拓扑 ID。具有相同拓ID CPU 共享该层级的资源
      如果支持 CPUID 叶子 0x1F，则无需进一步解析

   2) CPUID leaf 0x0000000B (Extended Topology Enumeration Leaf)

      扩展 CPUID 叶子 0x0000000B V2 扩展拓扑枚举叶子 0x1F 的前身，仅描述处理器拓扑的核与插槽域
      通过检查支持的 CPUID 级别是否 >= 0xB，然后检查特定层级（0 开始）`EBX[31:0]` 是否非零，来发现对该叶子的支持
      CPUID 叶子 0x0000000B CPUID 叶子 0x1F 具有相同的布局，应以类似方式枚举
      如果支持 CPUID 叶子 0xB，则无需进一步解析

   3) CPUID leaf 0x00000004 (Deterministic Cache Parameters Leaf)

      在既不支CPUID 叶子 0x1F 也不支持 CPUID 叶子 0xB Intel 处理器上，SMT 域的位移量使用共L1 缓存CPU 数量来计算
      支持超线程（Hyper-Threading）的处理器通过 CPUID 叶子 0x1（Basic CPUID Information）的 `EDX[^28^]` 来检测
      来自 CPUID 0x4 0 `EAX[25:14]` `Maximum number of addressable IDs for logical processors sharing this cache` 的阶，提供了APIC ID 计算 Core ID 所需的位移量
      APIC ID 与封装信息使用来CPUID 叶子 0x1 的数据计算

   4) CPUID leaf 0x00000001 (Basic CPUID Information)

      用于推导物理封装（插槽）ID 的掩码与位移，使CPUID 叶子 0x1 `EBX[23:16]` 中的 `Maximum number of addressable IDs for logical processors in this physical package` 来计算
     传统平台上的 APIC ID CPUID 叶子 0x1 `EBX[31:24]` 中的 `Initial APIC ID` 字段推导

3) Centaur 涓?Zhaoxin

   Intel 类似，Centaur Zhaoxin 使用 CPUID 叶子 0x00000004（Deterministic Cache Parameters Leaf）与 CPUID 叶子 0x00000001（Basic CPUID Information）的组合来推导拓扑信息


## 系统拓扑示例


  Linux 的另一CPU 枚举方式取决BIOS 如何枚举线程。许BIOS 会先枚举所有的线程 0，然后再枚举所有的线程 1。这样做有一个“好处”：无论是否启用线程，线0 的逻辑 Linux CPU 编号都保持不变。这仅仅是一个实现细节，没有实际影响
```

   [package 0] -> [core 0] -> [thread 0] -> Linux CPU 0

```
2) 单封装，双核

```

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
		    -> [core 1] -> [thread 0] -> Linux CPU 1

   b) 每核两个线程::

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
				-> [thread 1] -> Linux CPU 1
		    -> [core 1] -> [thread 0] -> Linux CPU 2
				-> [thread 1] -> Linux CPU 3

      Alternative enumeration::

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
				-> [thread 1] -> Linux CPU 2
		    -> [core 1] -> [thread 0] -> Linux CPU 1
				-> [thread 1] -> Linux CPU 3

      AMD nomenclature for CMT systems::

	[node 0] -> [Compute Unit 0] -> [Compute Unit Core 0] -> Linux CPU 0
				     -> [Compute Unit Core 1] -> Linux CPU 1
		 -> [Compute Unit 1] -> [Compute Unit Core 0] -> Linux CPU 2
				     -> [Compute Unit Core 1] -> Linux CPU 3

```
4) 双封装，双核

```

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
		    -> [core 1] -> [thread 0] -> Linux CPU 1

	[package 1] -> [core 0] -> [thread 0] -> Linux CPU 2
		    -> [core 1] -> [thread 0] -> Linux CPU 3

   b) 每核两个线程::

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
				-> [thread 1] -> Linux CPU 1
		    -> [core 1] -> [thread 0] -> Linux CPU 2
				-> [thread 1] -> Linux CPU 3

	[package 1] -> [core 0] -> [thread 0] -> Linux CPU 4
				-> [thread 1] -> Linux CPU 5
		    -> [core 1] -> [thread 0] -> Linux CPU 6
				-> [thread 1] -> Linux CPU 7

      Alternative enumeration::

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
				-> [thread 1] -> Linux CPU 4
		    -> [core 1] -> [thread 0] -> Linux CPU 1
				-> [thread 1] -> Linux CPU 5

	[package 1] -> [core 0] -> [thread 0] -> Linux CPU 2
				-> [thread 1] -> Linux CPU 6
		    -> [core 1] -> [thread 0] -> Linux CPU 3
				-> [thread 1] -> Linux CPU 7

      AMD nomenclature for CMT systems::

	[node 0] -> [Compute Unit 0] -> [Compute Unit Core 0] -> Linux CPU 0
				     -> [Compute Unit Core 1] -> Linux CPU 1
		 -> [Compute Unit 1] -> [Compute Unit Core 0] -> Linux CPU 2
				     -> [Compute Unit Core 1] -> Linux CPU 3

	[node 1] -> [Compute Unit 0] -> [Compute Unit Core 0] -> Linux CPU 4
				     -> [Compute Unit Core 1] -> Linux CPU 5
		 -> [Compute Unit 1] -> [Compute Unit Core 0] -> Linux CPU 6
				     -> [Compute Unit Core 1] -> Linux CPU 7

```
