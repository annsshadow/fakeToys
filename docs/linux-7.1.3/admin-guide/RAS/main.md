
## 可靠性、可用性与可服务性（RAS

本文档介绍了内核中存在的 RAS 功能的不同方面
######## RAS 概念


可靠性、可用性与可服务性（RAS）是服务器上用于衡量其健壮性的概念
可靠性（Reliability  系统产生正确输出的概率
  - 通常以平均无故障时间（MTBF）来衡量
  - 通过有助于避免、检测和修复硬件故障的特性来增强

可用性（Availability  系统在给定时刻可运行的概
  - 通常以一段时间内停机时间的百分比来衡  - 经常使用在运行时检测和纠正硬件故障的机制；

可服务性（或可维护性，Serviceability  系统可以被修复或维护的简便程度和速度

  - 通常以平均修复时间（MTBR）来衡量

### 改进 RAS


为了减少系统停机时间，系统应当能够检测硬件错误，并尽可能在运行时纠正它们。它还应当提供检测硬件退化的机制，以便警告系统管理员采取在组件导致数据丢失或系统停机之前更换该组件的行动
在监控措施中，最常见的包括：

- CPU —在指令执行以L1/L2/L3 缓存处检测错误；
- 内存 —添加纠错逻辑（ECC）以检测和纠正错误- I/O —为传输的数据添加 CRC 校验和；
- 存储 —RAID、日志型文件系统、校验和  自我监测、分析与报告技术（SMART）
通过监控错误检测的发生次数，可以确定硬件错误的概率是否正在增加，并在这种情况下进行预防性维护，以在这些错误仍可被纠正时更换已退化的组件
### 错误类型


现代系统上使用的大多数机制采用诸如汉明码之类的技术，当位数据包中的错误数量低于某个阈值时允许纠错。如果错误数量超出阈值，这些机制可以以很高的置信度指示发生了错误，但无法纠正
此外，有时错误发生在未被使用的组件上。例如，一块当前未被分配的内存
这定义了一些错误类别：

- **可纠正错误（CE，Correctable Error* - 错误检测机制检测到并纠正了错误。此类错误通常不致命，尽管某些内核机制允许系统管理员将其视为致命错误
- **不可纠正错误（UE，Uncorrected Error* - 发生的错误数量超出了纠错阈值，系统无法自动纠正
- **致命错误（Fatal Error* - UE 错误发生在系统的关键组件上时（例如，内核的一部分UE 损坏），避免数据损坏的唯一可靠方法是挂起或重启机器
- **非致命错误（Non-fatal Error* - UE 错误发生在未使用的组件上时，例如处于掉电状态的 CPU 或未使用的内存条，系统可能仍可运行，并在可用时通过热备件替换受影响的硬件
  此外，当错误发生在用户空间进程上时，也可以终止该进程并让用户空间重新启动它
处理非致命错误的机制通常很复杂，可能需要某些用户空间应用程序的帮助，以实施系统管理员所期望的策略
### 识别不良硬件组件


仅仅检测到一个硬件缺陷通常是不够的，因为系统需要精确定位到为了使硬件再次可靠而应当更换的最小可更换单元（MRU）
因此，它不仅需要错误记录设施，还需要将错误消息转换为该 MRU 的丝印或组件标签的机制
通常，对于内存来说这非常复杂，因为现CPU 会对来自不同内存模块的内存进行交错，以提供更好的性能。DMI BIOS 通常有一个内存模块标签列表，可以从中获取
```

	Memory Device
		Total Width: 64 bits
		Data Width: 64 bits
		Size: 16384 MB
		Form Factor: SODIMM
		Set: None
		Locator: ChannelA-DIMM0
		Bank Locator: BANK 0
		Type: DDR4
		Type Detail: Synchronous
		Speed: 2133 MHz
		Rank: 2
		Configured Clock Speed: 2133 MHz

```
在上述示例中，一DDR4 SO-DIMM 内存模块位于系统内存中标记为 "BANK 0"（由 **bank locator** 字段给出）的位置。请注意，在此类系统上，**total width** 等于 **data width**。这意味着该内存模块没有错误检纠正机制
不幸的是，并非所有系统都使用相同的字段来指定内存
```

	Memory Device
		Array Handle: 0x1000
		Error Information Handle: Not Provided
		Total Width: 72 bits
		Data Width: 64 bits
		Size: 8192 MB
		Form Factor: DIMM
		Set: 1
		Locator: DIMM_A1
		Bank Locator: Not Specified
		Type: DDR3
		Type Detail: Synchronous Registered (Buffered)
		Speed: 1600 MHz
		Rank: 2
		Configured Clock Speed: 1600 MHz

```
在那里，DDR3 RDIMM 内存模块位于系统内存中标记为 "DIMM_A1"（由 **locator** 字段给出）的位置。请注意，该内存模块具有 64 位的 **data width** 72 位的 **total width**。因此，它有 8 个额外的位供错误检测和纠正机制使用。这种内存被称为纠错码内存（ECC 内存）
更糟糕的是，系统主板上带有不同标签却使用完全相同 BIOS 的情况并不少见，这意味着 BIOS 提供的标签与真实标签不匹配
### ECC 内存


如上一节所述，ECC 内存有额外的位用于纠错。在上面的示例中，一个内存模块具64 位的 **data width** 72 位的 **total width**。用于错误检测和纠正机制的额8 位被称为 **syndrome**\ [#f1]_ [#f2]_
因此，当 CPU 请求内存控制器以 **data width** 写入一个字时，内存控制器使用汉明码或某些其他纠错码（如 SECDED+）实时计**syndrome**，生成具**total width** 大小的码。然后将该码写入内存模块
在读取时，使用写入时相同ECC 码将 **total width** 位的码转换回来，生成一个具**data width** 和一**syndrome** 的字。即便发生错误，具有 **data width** 的字也会被发送给 CPU
内存控制器还会查**syndrome**，以检查是否发生了错误，以ECC 码是否能够修复该错误。如果错误被纠正，则发生了一次可纠正错误（CE）。如果没有，则发生了一次不可纠正错误（UE）
关于 CE/UE 错误的信息存储在内存控制器的一些特殊寄存器中，可以通过读取这些寄存器来访问，访问者可以是 BIOS、某些特殊的 CPU Linux EDAC 驱动。在 x86 64 CPU 上，这些错误也可以通过机器检查架构（MCA）\ [#f3]_ 获取
  一种称"Lock-Step" 的模式，它将两个内存模块组合在一起，进行 128 位读/写。这为纠错提供了 16 位，显著改进了纠错机制，代价是当发生错误时，无法知道是哪个内存模块的过错。因此，它必须归咎于两个内存模块
  在此模式下，相同的数据被写入两个内存模块。在读取时，系统检查两个内存模块，以确认它们是否提供相同的数据。在这种配置下，当发生错误时，无法知道是哪个内存模块的过错。因此，它必须归咎于两个内存模块（如果系统也处于 Lock-step 模式，则4 个内存模块）
  请阅读内核树中的 Documentation/arch/x86/x86_64/machinecheck.rst
######## EDAC - 错误检测与纠正（Error Detection And Correction

   "bluesmoke" 是该设备驱动子系统在 "out-of-tree" 时以及维护于 http://bluesmoke.sourceforge.net 时的名称。该站点现在基本已过时，仅可用于历史用途
   当该子系统首次被推向上游时（内核 2.6.16），它被重命名为 `EDAC`
### 目的


`edac` 内核模块的目标是检测并报告在运行于 Linux 下的计算机系统中发生的硬件错误
### 内存


内存可纠正错误（CE）和不可纠正错误（UE）是要采集的主要错误。这些类型的错误`edac_mc` 设备采集
检CE 事件，然后采集这些事件并报告它们*可以**但不一定是未来 UE 事件的预测指标。仅CE 事件时，系统可以且将继续运行，因为尚未有数据损坏
然而，对表现出 CE 的内存模块进行预防性维护和主动更换部件，可以降低可怕的 UE 事件和系统崩溃的可能性
### 其他硬件元素


EDAC 的一个新特性，`edac_device` 类设备，在内2.6.23 版本中添加
这种新的设备类型允许非内存类型的 ECC 硬件检测器将其状态采集并通过 sysfs 接口呈现给用户空间
一些架构具有用L1、L2 L3 缓存ECC 检测器，以DMA 引擎、交换结构、主数据通路交换机、互连以及各种其他硬件数据通路。如果硬件报告了它，那么很可能可以构建一edac_device 设备来采集并将其呈现给用户空间

### PCI 总线扫描


此外，还会扫PCI 设备以查PCI 总线奇偶校验SERR 错误，以确定数据传输过程中是否发生错误
PCI 奇偶校验错误的存在必须带着怀疑态度来审视。有若干个附加适配**遵循 PCI 规范关于奇偶校验生成和报告的要求。规范说，如果供应商不打算生成奇偶校验，则应将奇偶校验状态位 tied 0。一些供应商没有这样做，因此奇偶校验位可浮动"，从而产生误报
EDAC PCI 扫描代码会检查位sysfs 中的一PCI 设备属性。如果该属性被设置，则 PCI 奇偶校验/错误
```

	broken_parity_status

```
并且位于 PCI 设备`/sys/devices/pci<XXX>/0000:XX:YY.Z` 目录中

### 版本管理


EDAC 由一"core" 模块（`edac_core.ko`）和若干个内存控制器（MC）驱动模块组成。在给定系统上，CORE 会被加载，并且会加载一MC 驱动。CORE MC 驱动（或 `edac_device` 驱动）都有各自的版本，反映了它们各自模块的当前发布级别
因此，要"报告"系统运行的版本，必须同时报告 CORE MC 驱动的版本

### 加载


如果 `edac` 是静态链接到内核中的，则无需加载。如`edac` 被构建为模块，则只需 modprobe 你需要的 `edac` 部件即可。你应该能够 modprobe 硬件特定的模块，并由依赖关系加载必要的核心模块```

	$ modprobe amd76x_edac

```
这会同时加载 `amd76x_edac.ko` 内存控制器模块和 `edac_mc.ko` 核心模块

### sysfs 接口


EDAC 提供了一`sysfs` 接口用于控制和报告。它位于 /sys/devices/system/edac 目录中
在此目录中目前驻留着 2 个组件：

	======= ==============================
	mc	内存控制器（memory controller）系	pci	PCI 控制与状态系	======= ==============================



### 内存控制器（mc）模

每个 `mc` 设备控制一组内存模[#f4]_。这些模块被布置在片选行（`csrowX`）和通道表（`chX`）中。可以有多个 csrow 和多个通道
  用于指代内存模块，尽管还有其他内存封装形式，SO-DIMM、SIMM 等。UEFI 规范（版2.7）将通用平台错误记录（CPER）节中的内存模块定义SMBIOS 内存设备（类17）。在本文档以EDAC 子系统内部，术语 "dimm" 用于所有内存模块，即使它们使用了不同类型的封装
内存控制器允许多csrow，典型值为 8 csrow。然而，实际csrow 数量取决于给定主板、内存控制器和内存模块特性的布局
双通道允许双向数据长度（例如在 64 位系统上128 位）的数据在 CPU 与内存之间传输。一些较新的芯片组允许超2 个通道，如全缓DIMM（FB-DIMM）内存控制器。以下示例将假设 2 个通道
	+------------+-----------------------+
	| CS Rows    |       Channels        |
	+------------+-----------+-----------+
	|            |  `ch0`  |  `ch1`  |
	+============+===========+===========+
	|            |**DIMM_A0**|**DIMM_B0**|
	+------------+-----------+-----------+
	| `csrow0` |   rank0   |   rank0   |
	+------------+-----------+-----------+
	| `csrow1` |   rank1   |   rank1   |
	+------------+-----------+-----------+
	|            |**DIMM_A1**|**DIMM_B1**|
	+------------+-----------+-----------+
	| `csrow2` |    rank0  |  rank0    |
	+------------+-----------+-----------+
	| `csrow3` |    rank1  |  rank1    |
	+------------+-----------+-----------+

在上述示例中，主板上用于内存 DIMM 的有 4 个物理插槽：

	+---------+---------+
	| DIMM_A0 | DIMM_B0 |
	+---------+---------+
	| DIMM_A1 | DIMM_B1 |
	+---------+---------+

这些插槽的标签通常丝印在主板上。标记为 `A` 的插槽在此示例中是通道 0。标记为 `B` 的插槽是通道 1。注意，一个物DIMM 上可能有两个 csrow。这csrow 根据其插入的内存 DIMM 插槽分配csrow 归属。因此，当每个通道上放1 DIMM 时，csrow 会跨越两DIMM
内存 DIMM 有单 "rank" 或双 "rank" 之分。一rank 是一个已填充csrow。在上面的示例中，类似地放置2 个双 rank DIMM。因此，csrow0 csrow1 都被填充了。另一方面，当 2 个单 rank DIMM 放置DIMM_A0 DIMM_B0 插槽中时，它们将只有一csrow（csrow0），csrow1 将为空。该模式csrow2 csrow3 重复。还要注意，某些内存控制器没有任何逻辑来识别内存模块，请参见下面的 `rankX` 目录
上述表示反映EDAC sysfs 接口中的目录树里。从目录 `/sys/devices/system/edac/mc` 开始，每个内存控制器将由自己的 `mcX` 目录表示，其`X` ```

	..../edac/mc/
		   |
		   |->mc0
		   |->mc1
		   |->mc2
		   ....

```
在每`mcX` 目录中都有若EDAC 控制和属性文件
### ``mcX`` 目录


`mcX` 目录中是针对`X` 实例内存控制器的 EDAC 控制和属性文件
有关 sysfs API 的描述，请参见：

	Documentation/ABI/testing/sysfs-devices-edac


### ``dimmX`` ``rankX`` 目录


使用 EDAC 子系统的推荐方式是查`dimmX` `rankX` 目录提供的信[#f5]_
一个典型的 EDAC 系统在以下位置具有以下结```

	/sys/devices/system/edac/
	鈹溾攢鈹€ mc
	鈹偮犅，鈹溾攢鈹€ mc0
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ce_count
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ce_noinfo_count
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm0
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_ce_count
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_dev_type
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_edac_mode
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_label
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_location
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_mem_type
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_ue_count
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ size
	鈹偮犅，鈹偮犅，鈹偮犅，鈹斺攢鈹€ uevent
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ max_location
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ mc_name
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ reset_counters
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ seconds_since_reset
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ size_mb
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ue_count
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ue_noinfo_count
	鈹偮犅，鈹偮犅，鈹斺攢鈹€ uevent
	鈹偮犅，鈹溾攢鈹€ mc1
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ce_count
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ce_noinfo_count
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm0
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_ce_count
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_dev_type
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_edac_mode
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_label
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_location
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_mem_type
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ dimm_ue_count
	鈹偮犅，鈹偮犅，鈹偮犅，鈹溾攢鈹€ size
	鈹偮犅，鈹偮犅，鈹偮犅，鈹斺攢鈹€ uevent
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ max_location
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ mc_name
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ reset_counters
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ seconds_since_reset
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ size_mb
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ue_count
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ue_noinfo_count
	鈹偮犅，鈹偮犅，鈹斺攢鈹€ uevent
	鈹偮犅，鈹斺攢鈹€ uevent
	鈹斺攢鈹€ uevent

```
`dimmX` 目录中是针对`X` 内存模块EDAC 控制和属性文件：

- `size` - csrow 属性文件管理的全部内存

	此属性文件以兆字节数显示csrow 包含的内存
- `dimm_ue_count` - 不可纠正错误计数属性文
	此属性文件显示在DIMM 上发生的不可纠正错误的总数。如果设置了 panic_on_ue，此计数器将没有机会递增，因EDAC 将使系统崩溃
- `dimm_ce_count` - 可纠正错误计数属性文
	此属性文件显示在DIMM 上发生的可纠正错误的总数。此计数非常值得检查。CE 提供DIMM 开始故障的早期指示。应监视此计数字段是否出现非零值，并将此类信息报告给系统管理员
- `dimm_dev_type`  - 设备类型属性文
	此属性文件将显示DIMM 上使用的 DRAM 设备类型	示例
  - x1
  - x2
  - x4
  - x8

- `dimm_edac_mode` - EDAC 操作模式属性文
	此属性文件将显示正在使用的错误检测和纠正类型
- `dimm_label` - 内存模块标签控制文件

	此控制文件允许为DIMM 分配一个标签。有了模块中的这个标签，当发生错误时，输出可以在系统日志中提DIMM 标签。这对于崩溃事件隔离 UE 事件的原因至关重要
	DIMM 标签必须在启动后分配，使用能够正确将物理插槽与其丝印标签关联的信息。此信息目前非常依赖主板，并且必须在用户空间确定
- `dimm_location` - 内存模块的位
	位置最多可以有 3 级，并描述内存控制器如何识别内存模块的位置。根据内存和内存控制器的类型，可以是
  - **csrow** **channel** - 当内存控制器不识别单DIMM 时使- 例如`rankX` 目录中；
  - **branch***channel***slot** - 通常用于 FB-DIMM 内存控制器；
  - **channel***slot** - 用于 Nehalem 及更新的 Intel 驱动
- `dimm_mem_type` - 内存类型属性文
	此属性文件将显示csrow 上当前的内存类型。通常是缓冲或未缓冲内存	示例
  - Registered-DDR
  - Unbuffered-DDR

  用于识别内存模块。在此类系统上，该目录称`rankX`。在现代 Intel 内存控制器上，内存控制器直接识别内存模块。在此类系统上，该目录称`dimmX`
  sysfs 映射中由 sysfs 子系统自动创建的 symlinks。目前，它们没有任何用途

### 系统日志


如果UE CE 启用了日志，则系统日志将包含
```

  EDAC MC0: CE page 0x283, offset 0xce0, grain 8, syndrome 0x6ec3, row 0, channel 1 "DIMM_B1": amd76x_edac
  EDAC MC0: CE page 0x1e5, offset 0xfb0, grain 8, syndrome 0xb741, row 0, channel 1 "DIMM_B1": amd76x_edac


```
消息的结构为
	+---------------------------------------+-------------+
	| Content                               | Example     |
	+=======================================+=============+
	| 内存控制                           | MC0         |
	+---------------------------------------+-------------+
	| 错误类型                              | CE          |
	+---------------------------------------+-------------+
	| 内存                               | 0x283       |
	+---------------------------------------+-------------+
	| 页内偏移                              | 0xce0       |
	+---------------------------------------+-------------+
	| 字节粒度                              | grain 8     |
	| 或错误分辨率                          |             |
	+---------------------------------------+-------------+
	| 错误 syndrome                         | 0xb741      |
	+---------------------------------------+-------------+
	| 内存                               | row 0       |
	+---------------------------------------+-------------+
	| 内存通道                              | channel 1   |
	+---------------------------------------+-------------+
	| 已设置的 DIMM 标签（若有）            | DIMM B1     |
	+---------------------------------------+-------------+
	| 然后是可选的、驱动特定的              |             |
	| 可能包含额外信息的消               |             |
	+---------------------------------------+-------------+

无信息的 UE CE 将只缺少内存控制器、错误类型no info" 提示，然后是可选的、驱动特定的错误消息

### PCI 总线奇偶校验检

对于 Header Type 00 设备，无论设备上是否启用了奇偶校验，都会查看主状态以查找任何奇偶校验错误。（规范指出在某些情况下会生成奇偶校验）。对Header Type 01 桥，还会查看次级状态寄存器，以查看在桥另一侧的总线上是否发生了奇偶校验

### sysfs 配置


`/sys/devices/system/edac/pci` 下有如下控制和属性文件：


- `check_pci_parity` - 启用/禁用 PCI 奇偶校验检查控制文
	此控制文件启用或禁用 PCI 总线奇偶校验扫描操作。向此文件写1 启用扫描。向此文件写0 禁用扫描```

		echo "1" >/sys/devices/system/edac/pci/check_pci_parity

	禁用::

		echo "0" >/sys/devices/system/edac/pci/check_pci_parity


```
- `pci_parity_count` - 奇偶校验计数

	此属性文件将显示已检测到的奇偶校验错误数量

### 模块参数


- `edac_mc_panic_on_ue` - 发生 UE 时崩溃控制文
	不可纠正错误将导致机器崩溃。这通常是我们所期望的。当发生不可纠正错误时继续运行是个坏主意——无法确定的内容未被纠正，并且操作系统上下文可能被破坏得如此严重，以至于继续运行将导致进一步的损坏。如果内核配置了 MCE，那EDAC 将永远不会注意到UE```

		module/kernel parameter: edac_mc_panic_on_ue=[0|1]

	RUN TIME::

		echo "1" > /sys/module/edac_core/parameters/edac_mc_panic_on_ue


```
- `edac_mc_log_ue` - 记录 UE 控制文件


	生成描述不可纠正错误的内核消息。这些错误通过系统消息日志系统报告。即使禁用了 UE 日志，UE 统计信息仍会累积```

		module/kernel parameter: edac_mc_log_ue=[0|1]

	RUN TIME::

		echo "1" > /sys/module/edac_core/parameters/edac_mc_log_ue


```
- `edac_mc_log_ce` - 记录 CE 控制文件


	生成描述可纠正错误的内核消息。这些错误通过系统消息日志系统报告。即使禁用了 CE 日志，CE 统计信息仍会累积```

		module/kernel parameter: edac_mc_log_ce=[0|1]

	RUN TIME::

		echo "1" > /sys/module/edac_core/parameters/edac_mc_log_ce


```
- `edac_mc_poll_msec` - 轮询周期控制文件


	轮询错误信息的时间周期（以毫秒为单位）。值太小会浪费资源。值太大可能会延迟必要的错误处理，并可能丢失用于定位错误的有价值信息000 毫秒（每秒一次）是当前默认值。需要尽可能多带宽的系统可能会增加此值```

		module/kernel parameter: edac_mc_poll_msec=[0|1]

	RUN TIME::

		echo "1000" > /sys/module/edac_core/parameters/edac_mc_poll_msec


```
- `panic_on_pci_parity` - 发生 PCI 奇偶校验错误时崩

	此控制文件启用或禁用在检测到奇偶校验错误时使系统崩溃```

			edac_panic_on_pci_pe=[0|1]

	启用::

		echo "1" > /sys/module/edac_core/parameters/edac_panic_on_pci_pe

	禁用::

		echo "0" > /sys/module/edac_core/parameters/edac_panic_on_pci_pe



```
### EDAC 设备类型


在头文件 edac_pci.h 中，有一系列用于 EDAC_DEVICE edac_device 结构API
用户空间通过 sysfs 接口访问 edac_device
在位`/sys/devices/system/edac`（sysfs）处将出现新edac_device 设备
在上`edac` 目录之下有一个三级树。例如，`test_device_edac` 设备（位http://bluesmoke.sourceforget.net
```

	/sys/devices/system/edac/test-instance

```
在此目录中有各种控制、一个符号链接和一个或多个 `instance` 目录
标准默认控制为：

	==============	=======================================================
	log_ce		记录 CE 事件的布尔	log_ue		记录 UE 事件的布尔	panic_on_ue	遇到 UE 时使系统 `panic` 的布尔			（默认关闭，可通过启动脚本设为 true	poll_msec	事件 POLL 周期之间的时间段
	==============	=======================================================

`test_device_edac` 设备至少添加了它自己的一个自定义控制
	==============	==================================================
	test_bits	在当前测试驱动中除了展示它是如何安装的之			不做任何事情。移植的驱动可以添加
			一个或多个此类控制或属			用于特定用途			一个树外驱动使用此处的控制来允			向硬件注入错误（ERROR INJECTION）寄存器
	==============	==================================================

该符号链接指向为edac_device 注册'struct dev'
### 实例（Instances

存在一个或多个实例目录。对`test_device_edac` 情况
	+----------------+
	| test-instance0 |
	+----------------+

在此目录中有两个默认计数器属性，它们是更深子目录中计数的总和
	==============	====================================
	ce_count	子目录中 CE 事件的总数
	ue_count	子目录中 UE 事件的总数
	==============	====================================

### 块（Blocks

在最低目录级别是 `block` 目录。每个实例中可以指定 0 或更多个块：

	+-------------+
	| test-block0 |
	+-------------+

在此目录中默认属性为
	==============	================================================
	ce_count	作为`block` 所监视硬件
			CE 事件计数	ue_count	作为`block` 所监视硬件
			UE 事件计数	==============	================================================


`test_device_edac` 设备添加4 个属性和 1 个控制：

	================== ====================================================
	test-block-bits-0	每个 POLL 周期此计数器递增
	test-block-bits-1	10 个周期，此计数器递增一次，
				并将 test-block-bits-0 设为 0
	test-block-bits-2	100 个周期，此计数器递增一次，
				并将 test-block-bits-1 设为 0
	test-block-bits-3	1000 个周期，此计数器递增一次，
				并将 test-block-bits-2 设为 0
	================== ====================================================

	================== ====================================================
	reset-counters		向此控制写入任何内容都将
				重置上述所有计数器	================== ====================================================


使用 `test_device_edac` 驱动应当能使其他任何人创建他们自己针对其硬件系统的独特驱动
`test_device_edac` 示例驱动位于 EDAC http://bluesmoke.sourceforge.net 项目站点

### Nehalem 及更新的 Intel CPU 上使EDAC API


在较旧的 Intel 架构上，内存控制器是北桥芯片组的一部分。Nehalem、Sandy Bridge、Ivy Bridge、Haswell、Sky Lake 及更新的 Intel 架构将增强版的内存控制器（MC）集成到CPU 内部
本章将介绍在较新 Intel CPU 上发现的增强内存控制器的差异，例`i7core_edac`、`sb_edac` `sbx_edac` 驱动

   Xeon E7 处理器系列使用一个单独的芯片作为内存控制器，称为 Intel 可扩展内存缓冲器（Intel Scalable Memory Buffer）。本节不适用于此类系列
1) 每个快速通道互连（QPI，Quick Patch Interconnect）有一个内存控制器。在驱动中，术语 "socket" 表示一QPI。它与一个物CPU 插槽相关联
   每个 MC 3 个物理读通道 个物理写通道3 个逻辑通道。驱动目前将其视为仅 3 个通道。每个通道最多可以有 3 DIMM
   已知的最小单元是 DIMM。没有关csrow 的信息。由EDAC API 映射的最小单元是 csrow，驱动将通道/DIMM 顺序映射到不同的 csrow```

	Ch0 phy rd0, wr0 (0x063f4031): 2 ranks, UDIMMs
	  dimm 0 1024 Mb offset: 0, bank: 8, rank: 1, row: 0x4000, col: 0x400
	  dimm 1 1024 Mb offset: 4, bank: 8, rank: 1, row: 0x4000, col: 0x400
        Ch1 phy rd1, wr1 (0x063f4031): 2 ranks, UDIMMs
	  dimm 0 1024 Mb offset: 0, bank: 8, rank: 1, row: 0x4000, col: 0x400
	Ch2 phy rd3, wr3 (0x063f4031): 2 ranks, UDIMMs
	  dimm 0 1024 Mb offset: 0, bank: 8, rank: 1, row: 0x4000, col: 0x400

   驱动将其映射:

	csrow0: channel 0, dimm0
	csrow1: channel 0, dimm1
	csrow2: channel 1, dimm0
	csrow3: channel 2, dimm0

   每个 csrow 导出一DIMM
   每个 QPI 被导出为不同的内存控制器
```
2) MC 具有注入错误以测试驱动的能力。驱动通过一些错误注入节点实现此功能
   要注入内存错误，`/sys/devices/system/edac/mc/mc/` 下有一sysfs 节点
   - `inject_addrmatch/*`      控制错误注入掩码寄存器。可以指```

         dimm = 受影响的 dimm。数字相对于一个通道         rank = 内存 rank         channel = 将生成错误的通道         bank = 受影响的 bank         page = 页地址         column (col) = 地址列
      上述每个值都可以设置"any" 以匹配任何有效值
      在驱动初始化时，所有值都设置any
      例如，要dimm 2 rank 1 上、针对任何通道、任bank、任何页、任何列生成错误::
```

		echo 2 >/sys/devices/system/edac/mc/mc0/inject_addrmatch/dimm
		echo 1 >/sys/devices/system/edac/mc/mc0/inject_addrmatch/rank

	要返回匹配任何的默认行为，你可以执行::

		echo any >/sys/devices/system/edac/mc/mc0/inject_addrmatch/dimm
		echo any >/sys/devices/system/edac/mc/mc0/inject_addrmatch/rank

   - ``inject_eccmask``          指定哪些位会出现问题
   - ``inject_section``       指定将获得错误的 ECC 缓存:

		3 表示两		2 表示最		1 表示最
   - ``inject_type``       指定错误类型，是以下位的组合::

		bit 0 - repeat
		bit 1 - ecc
		bit 2 - parity

   - ``inject_enable``       当写入非 0 的值时启动错误生成
   所inject 变量都可以读取。写入需root 权限
   数据手册指出，错误只会在对匹inject_addrmatch 的地址进行写操作之后生成。然而，似乎读取也会产生错误
   例如，以下代码将socket 0、通道 2 上的任何 DIMM/地址处生成错:
```

	echo 2 >/sys/devices/system/edac/mc/mc0/inject_addrmatch/channel
	echo 2 >/sys/devices/system/edac/mc/mc0/inject_type
	echo 64 >/sys/devices/system/edac/mc/mc0/inject_eccmask
	echo 3 >/sys/devices/system/edac/mc/mc0/inject_section
	echo 1 >/sys/devices/system/edac/mc/mc0/inject_enable
	dd if=/dev/mem of=/dev/null seek=16k bs=4k count=1 >& /dev/null

   对于 socket 1，需要将上述命令中的 "mc0" 替换"mc1"
   生成的错误消息将类似:

	EDAC MC0: UE row 0, channel-a= 0 channel-b= 0 labels "-": NON_FATAL (addr = 0x0075b980, socket=0, Dimm=0, Channel=2, syndrome=0x00000040, count=1, Err=8c0000400001009f:4000080482 (read error: read ECC error))

```
3) 可纠正错误内存寄存器计数
   这些较新MC 有一些用于计数内存错误的寄存器。驱动使用这些寄存器在具有寄存式 DIMM 的设备上报告可纠正错误
   然而，这些计数器对未寄存式 DIMM 不起作用。由于芯片组提供了一些也UDIMM 有效（但粒度比默认的差）的计数器，驱动为 UDIMM 内存导出这些寄存器```

     $ for i in /sys/devices/system/edac/mc/mc0/all_channel_counts/*; do echo $i; cat $i; done
	/sys/devices/system/edac/mc/mc0/all_channel_counts/udimm0
	0
	/sys/devices/system/edac/mc/mc0/all_channel_counts/udimm1
	0
	/sys/devices/system/edac/mc/mc0/all_channel_counts/udimm2
	0

   这里发生的情况是，位于不csrow 但具有相dimm 编号的错误将递增同一个计数器。因此，在这种内存映射中::

	csrow0: channel 0, dimm0
	csrow1: channel 0, dimm1
	csrow2: channel 1, dimm0
	csrow3: channel 2, dimm0

   硬件将为 csrow0、csrow2 csrow3 处第一dimm 上的错误递增 udimm0
   硬件将为 csrow0、csrow2 csrow3 处第二个 dimm 上的错误递增 udimm1
   硬件将为 csrow0、csrow2 csrow3 处第三个 dimm 上的错误递增 udimm2
```
4) 标准错误计数
   当驱动接收到 mcelog 错误时，会生成标准错误计数器。由于使UDIMM 时是由软件计数的，可能会丢失一些错误。使RDIMM 时，它们显示寄存器的内容
### ``amd64_edac`` 使用的参考文

`amd64_edac` 模块基于以下文档
（可http://support.amd.com/en-us/search/tech-docs 获取）：

1. :Title:  AMD Athlon 64 AMD Opteron 处理器的 BIOS 与内核开发者指   :AMD publication #: 26094
   :Revision: 3.26
   :Link: http://support.amd.com/TechDocs/26094.PDF

2. :Title:  AMD NPT Family 0Fh 处理器的 BIOS 与内核开发者指   :AMD publication #: 32559
   :Revision: 3.00
   :Issue Date: May 2006
   :Link: http://support.amd.com/TechDocs/32559.pdf

3. :Title:  AMD Family 10h 处理器的 BIOS 与内核开发者指南（BKDG   :AMD publication #: 31116
   :Revision: 3.00
   :Issue Date: September 07, 2007
   :Link: http://support.amd.com/TechDocs/31116.pdf

4. :Title: AMD Family 15h Models 30h-3Fh 处理器的 BIOS 与内核开发者指南（BKDG   :AMD publication #: 49125
   :Revision: 3.06
   :Issue Date: 2/12/2015 (latest release)
   :Link: http://support.amd.com/TechDocs/49125_15h_Models_30h-3Fh_BKDG.pdf

5. :Title: AMD Family 15h Models 60h-6Fh 处理器的 BIOS 与内核开发者指南（BKDG   :AMD publication #: 50742
   :Revision: 3.01
   :Issue Date: 7/23/2015 (latest release)
   :Link: http://support.amd.com/TechDocs/50742_15h_Models_60h-6Fh_BKDG.pdf

6. :Title: AMD Family 16h Models 00h-0Fh 处理器的 BIOS 与内核开发者指南（BKDG   :AMD publication #: 48751
   :Revision: 3.03
   :Issue Date: 2/23/2015 (latest release)
   :Link: http://support.amd.com/TechDocs/48751_16h_bkdg.pdf

## 致谢（Credits

- Doug Thompson <dougthompson@xmission.com> 撰写

  - 2005 骞?12 鏈?7 鏃?  - 2007 骞?7 鏈?17 鏃，鏇存柊

- |copy| Mauro Carvalho Chehab

  - 2009 8 5 Nehalem 接口
  - 2016 10 26 转换ReST 并对 Nehalem 节进行了清理

- EDAC 作维护者：

  - Doug Thompson、Dave Jiang、Dave Peterson 等，
  - Mauro Carvalho Chehab
  - Borislav Petkov
  - 原作者：Thayne Harbaugh
