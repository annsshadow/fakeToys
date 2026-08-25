## 错误检测与纠正（EDAC）设

### EDAC 子系统使用的主要概念


有一些不那么显而易见、需要留意的事情，例**sockets（插槽）*socket sets
（插槽集合）***banks（存储体**rows（行**chip-select rows
（片选行**channels（通道*，等等…
这些是诸多被随意使用、却并不总是如其字面意思的术语（不可思议！）。为创建一个用于讨论的共同基础，我们将确立这些术语及其定义
- Memory devices（内存设备）

单个DRAM 芯片，位于内存条上。这些器件通常各自输出 4 位和 8 位（x4、x8）将其中若干片并行分组，就提供了内存控制器所期望的位数：通常72 位，以便
提供 64 + 8 ECC 数据
- Memory Stick（内存条
一块将多个内存器件并行聚合在一起的印刷电路板。一般来说，它是现场可更单元（FRU），在出现过多错误时被更换。大多数情况下它也被称为 DIMM（双列直内存模块）
- Memory Socket（内存插槽）

主板上接受单个内存条的物理连接器。在若干数据手册中也被称为“slot（插槽）”
- Channel（通道
一个内存控制器通道，负责与一DIMM 通信。每个通道有自己独立控制（命令和数据总线，可以独立使用，也可以与其他通道组合使用
- Branch（分支）

它通常是全缓冲 DIMM 内存控制器上的最高层级。通常包含两条通道。同一分支
上的两条通道可以以单通道模式或锁步（lockstep）模式使用。当启用锁步时，
缓存行大小加倍，但通常会带来一定的性能损失。此外，当发生错误时，通常无法
只指向某一个内存条，因为纠错码是用两个 DIMM 而非一个计算出来的。因此，能够纠正比单通道模式更多的错误
- Single-channel（单通道
内存控制器访问的数据仅包含在一DIMM 中。例如，如果数据64 位宽，则数据
通过一64 位并行访问流CPU。通常用于 SDR、DDR、DDR2 DDR3 内存。FB-DIMM
RAMBUS 对通道使用了不同的概念，因此这一概念不适用于它们
- Double-channel（双通道
内存控制器访问的数据大小被交错到两个同时访问DIMM 中。例如，如果 DIMM 64 位宽（含 ECC 72 位），则数据通过一128 位并行访问流CPU
- Chip-select row（片选行
这是用于选择要访问的 DRAM rank DRAM 信号名称。单通道常见的片选行64 位，
双通道128 位。某DIMM 类型带有一个内存缓冲器，可以向内存控制器隐藏对其的
直接访问，因此内存控制器可能看不到它
- Single-Ranked stick（单列内存条
单列内存条有 1 个片选行内存。主板通常会向一个内存条驱动两个片选引脚。单内存条只会占据其中一行，另一行将未被使用
- Double-Ranked stick（双列内存条
双列内存条有两条访问不同内存器件集合的片选行。这两行不能同时被访问
- Double-sided stick（双面内存条
**已弃用术*，见 Double-Ranked stick <doubleranked>
双面内存条有两条访问不同内存器件集合的片选行。这两行不能同时被访问。“双面与内存器件安装在内存条的哪一面无关
- Socket set（插槽集合）

单次内存访问所需的所有内存条，或者一条片选行所横跨的所有内存条。单个插集合有两条片选行，如果使用双面内存条，它们将占据这些片选行
- Bank（存储体
此术语被避免使用，因为在需要区分片选行和插槽集合时它含义不清
- High Bandwidth Memory（HBM，高带宽内存
HBM 是一种低功耗、具有超宽通信通道的新内存类型。它使用通过称为“硅通孔（through-silicon vias，简TSVs）的微小连线互连的垂直堆叠内存芯片（DRAM die）
若干 HBM 芯片堆栈通过一个称为“中介层（interposer）”的超快互连连接CPU GPU。因此，HBM 的特性几乎与片上集成RAM 难以区分
### Memory Controllers（内存控制器

EDAC 核心的大部分工作都集中在内存控制器错误检测上。即 `edac_mc_alloc`。它
在内部使struct `mem_ctl_info` 来描述内存控制器，对 EDAC 驱动程序来说
这是一个不透明结构体。只EDAC 核心才允许触碰它

### PCI Controllers（PCI 控制器）


EDAC 子系统提供了一种通过调用 `edac_pci_alloc_ctl_info` 来处PCI 控制器的
机制。它将使struct `edac_pci_ctl_info` 来描PCI 控制器

### EDAC Blocks（EDAC 块）


EDAC 子系统还通过 `edac_device_alloc_ctl_info` 函数提供了一种通用机制，用报告硬件其他部分的错误
结构`edac_dev_sysfs_block_attribute`、`edac_device_block``edac_device_instance` `edac_device_ctl_info` sysfs 中提供了一个通用
或抽象的“edac_device”表示
这套结构体以及实现其 API 的代码，提供了用于注册非标准内存PCI EDAC 类型
设备的机制，例如
- CPU 缓存（L1 L2- DMA 引擎
- 鏍稿績 CPU 浜ゆ崲鍣?- 缁撴瀯浜ゆ崲鍗曞厓
- PCIe 接口控制- 其他可以监测错误EDAC/ECC 类型设备等
它允许两级层次结构
例如，一个缓存可以由 L1、L2 L3 级缓存组成。每CPU 核心拥有自己L1
缓存，同时共L2 以及可能L3 缓存。在这种情况下，这些可以通过下面sysfs
```

	/sys/devices/system/edac/..

	pci/		<existing pci directory (if available)>
	mc/		<existing memory device directory>
	cpu/cpu0/..	<L1 and L2 block directory>
		/L1-cache/ce_count
			 /ue_count
		/L2-cache/ce_count
			 /ue_count
	cpu/cpu1/..	<L1 and L2 block directory>
		/L1-cache/ce_count
			 /ue_count
		/L2-cache/ce_count
			 /ue_count
	...

	the L1 and L2 directories would be "edac_device_block's"

```


### Heterogeneous system support（异构系统支持）


AMD 异构系统通过自定义的 xGMI 链路连接 CPU GPU 的数fabric（data fabric来构建。因此，GPU 节点上的数据 fabric 可以CPU 节点上的数据 fabric 一样被
访问
MI200 加速器是数据中GPU。它们有 2 个数fabric，每GPU 数据 fabric 包含
四个统一内存控制器（UMC）。每UMC 包含 8 个通道。每UMC 通道控制一128 HBM2eGB）通道（相当于 8 X 2GB rank）。这总共创建4096 位的 DRAM 数据
总线
UMC 在接口一16GB X 2GB DRAM）HBM 堆栈时，每个 UMC 通道在接2GB DRAM（表示为 rank）
AMD GPU 节点上的内存控制器可以在 EDAC 中这样表示：

	GPU DF / GPU Node -> EDAC MC
	GPU UMC           -> EDAC CSROW
	GPU UMC channel   -> EDAC CHANNEL

例如：一个带1 AMD CPU、通过 xGMI 连接4 MI200（Aldebaran）GPU 异构系统
更多异构硬件细节
- CPU UMC（统一内存控制器）GPU UMC 大体相同。它们有片选（csrows）和通道  不过，出于性能、物理布局或其他原因，布局是不同的- CPU UMC 使用 1 个通道，这种情况下 UMC = EDAC 通道。这符合市场宣传用语。CPU
  X 个内存通道，等等- CPU UMC 最多使4 个片选，因此 UMC 片= EDAC CSROW- GPU UMC 使用 1 个片选，因此 UMC = EDAC CSROW- GPU UMC 使用 8 个通道，因UMC 通道 = EDAC 通道
EDAC 子系统提供了一种机制，通过调用 CPU GPU 各自特定ops 来处AMD 异构
系统
AMD GPU 节点基于 PCI 层级按顺序排列，第一GPU 节点被假定具有一Node ID ```

	$ ls /sys/devices/system/edac/mc/
		mc0   - CPU MC node 0
		mc1  |
		mc2  |- GPU card[0] => node 0(mc1), node 1(mc2)
		mc3  |
		mc4  |- GPU card[1] => node 0(mc3), node 1(mc4)
		mc5  |
		mc6  |- GPU card[2] => node 0(mc5), node 1(mc6)
		mc7  |
		mc8  |- GPU card[3] => node 0(mc7), node 1(mc8)

```
例如，一个带有一AMD CPU、通过 xGMI 连接到四MI200（Aldebaran）GPU 的异系统。这个拓扑可以表示为
```

	/sys/devices/system/edac/mc/..

	CPU			# CPU node
	鈹溾攢鈹€ mc 0

	GPU Nodes are enumerated sequentially after CPU nodes have been populated
	GPU card 1		# Each MI200 GPU has 2 nodes/mcs
	鈹溾攢鈹€ mc 1		# GPU node 0 == mc1, Each MC node has 4 UMCs/CSROWs
	鈹偮犅，鈹溾攢鈹€ csrow 0		# UMC 0
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 0	# Each UMC has 8 channels
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 1   # size of each channel is 2 GB, so each UMC has 16 GB
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 2
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 3
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 4
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 5
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 6
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 7
	鈹偮犅，鈹溾攢鈹€ csrow 1		# UMC 1
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 0
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ..
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 7
	鈹偮犅，鈹溾攢鈹€ ..		..
	鈹偮犅，鈹溾攢鈹€ csrow 3		# UMC 3
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 0
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ ..
	鈹偮犅，鈹偮犅，鈹溾攢鈹€ channel 7
	鈹偮犅，鈹溾攢鈹€ rank 0
	鈹偮犅，鈹溾攢鈹€ ..		..
	鈹偮犅，鈹溾攢鈹€ rank 31		# total 32 ranks/dimms from 4 UMCs
	鈹?	鈹溾攢鈹€ mc 2		# GPU node 1 == mc2
	鈹偮犅，鈹溾攢鈹€ ..		# each GPU has total 64 GB

	GPU card 2
	鈹溾攢鈹€ mc 3
	鈹偮犅，鈹溾攢鈹€ ..
	鈹溾攢鈹€ mc 4
	鈹偮犅，鈹溾攢鈹€ ..

	GPU card 3
	鈹溾攢鈹€ mc 5
	鈹偮犅，鈹溾攢鈹€ ..
	鈹溾攢鈹€ mc 6
	鈹偮犅，鈹溾攢鈹€ ..

	GPU card 4
	鈹溾攢鈹€ mc 7
	鈹偮犅，鈹溾攢鈹€ ..
	鈹溾攢鈹€ mc 8
	鈹偮犅，鈹溾攢鈹€ ..

```
