
## 资源控制特性（resctrl）的用户接口


:Copyright: |copy| 2016 Intel Corporation
:Authors: - Fenghua Yu <fenghua.yu@intel.com>
          - Tony Luck <tony.luck@intel.com>
          - Vikas Shivappa <vikas.shivappa@intel.com>


Intel 将本特性称为 Intel Resource Director Technology（Intel(R) RDT）。
AMD 将本特性称为 AMD Platform Quality of Service（AMD QoS）。

本特性由 CONFIG_X86_CPU_RESCTRL 以及 x86 的 /proc/cpuinfo 标志位启用：

=============================================================== ================================
RDT（资源导向技术）分配			"rdt_a"
CAT（缓存分配技术）				"cat_l3", "cat_l2"
CDP（代码与数据优先级划分）				"cdp_l3", "cdp_l2"
CQM（缓存 QoS 监控）					"cqm_llc", "cqm_occup_llc"
MBM（内存带宽监控）				"cqm_mbm_total", "cqm_mbm_local"
MBA（内存带宽分配）				"mba"
SMBA（慢速内存带宽分配）				""
BMEC（带宽监控事件配置）			""
ABMC（可分配带宽监控计数器）			""
SDCIAE（智能数据缓存注入分配强制）	""
=============================================================== ================================

历史上，新特性默认会在 /proc/cpuinfo 中可见。这导致这些特性标志变得难以被人解析。如果
用户空间可以从 resctrl 的 info 目录获取关于该特性的信息，则应避免向 /proc/cpuinfo
添加新的标志。

```

 # mount -t resctrl resctrl [-o cdp[,cdpl2][,mba_MBps][,debug]] /sys/fs/resctrl

```
挂载选项如下：

"cdp":
		在 L3 缓存分配中启用代码/数据优先级划分。
"cdpl2":
		在 L2 缓存分配中启用代码/数据优先级划分。
"mba_MBps":
		启用 MBA 软件控制器（mba_sc）以 MiBps 为单位指定 MBA
		带宽
"debug":
		使调试文件可访问。可用的调试文件标注有
		“仅在使用 debug 选项时可用”。

L2 和 L3 的 CDP 是分别控制的。

RDT 各特性彼此正交。某个特定系统可能仅支持监控、仅支持控制，或同时支持监控与控制。
缓存伪锁定（cache pseudo-locking）是一种利用缓存控制在缓存中“钉住”或“锁定”数据的
独特方式。更多细节可参见“Cache Pseudo-Locking”。


挂载会在分配或监控二者之一存在时成功，但只会创建该系统所支持的文件和目录。
关于监控和分配期间该接口行为的更多细节，请参见“Resource alloc and monitor groups”一节。

## Info 目录


'info' 目录包含关于已启用资源的信息。每个资源都有自己的子目录。子目录名称
反映资源名称。

资源子目录中的大多数文件是只读的，用于描述该资源的属性。支持全局配置选项的资源
还包含可写文件，可用于修改这些设置。

每个子目录包含以下与分配相关的文件：

缓存资源（L3/L2）子目录包含以下与分配相关的文件：

"num_closids":
		该资源有效的 CLOSID 数量。内核使用所有已启用资源中
		最小的 CLOSID 数量作为上限。
"cbm_mask":
		该资源有效的位掩码。该掩码等价于 100%。
"min_cbm_bits":
		写入掩码时必须设置的连续位的最小数量。

"shareable_bits":
		与其他执行实体（例如 I/O）共享的资源的位掩码。
		适用于该资源的所有实例。用户在设置独占缓存分区时
		可以使用它。注意某些平台支持拥有自己缓存使用设置的
		设备，这些设置可能会覆盖这些位。

		当启用 "io_alloc" 时，每个缓存实例的一部分可以
		配置为在硬件和软件之间共享使用。应使用 "bit_usage"
		来查看每个缓存实例中哪些部分通过 "io_alloc" 特性
		配置为供硬件使用，因为每个缓存实例的 "io_alloc" 位掩码
		都可以通过 "io_alloc_cbm" 独立配置。

"bit_usage":
		标注的容量位掩码，显示资源的全部实例如何被使用。图例如下：

			"0":
				对应区域未被使用。当系统资源已分配且在 "bit_usage" 中
				发现 "0" 时，表明资源被浪费了。

			"H":
				对应区域仅由硬件使用，但可供软件使用。如果某个资源
				在 "shareable_bits" 或 "io_alloc_cbm" 中设置了位，
				但并非所有这些位都出现在资源组的 schemata 中，则出现在
				"shareable_bits" 或 "io_alloc_cbm" 中但没有出现在
				任何资源组中的位将被标记为 "H"。
			"X":
				对应区域可供共享，并且由硬件和软件共同使用。这些是在
				"shareable_bits" 或 "io_alloc_cbm" 中以及资源组
				的分配中出现的位。
			"S":
				对应区域由软件使用，且可供共享。
			"E":
				对应区域被一个资源组独占使用。不允许共享。
			"P":
				对应区域被伪锁定。不允许共享。
"sparse_masks":
		指示是否支持 CBM 中非连续的 1 值。

			"0":
				仅支持 CBM 中连续的 1 值。
			"1":
				支持 CBM 中非连续的 1 值。

"io_alloc":
		"io_alloc" 使系统软件能够配置分配给 I/O 流量的缓存部分。仅当
		系统在其某些缓存资源上支持该特性时，该文件才可能存在。

			"disabled":
				资源支持 "io_alloc" 但该特性被禁用。用于分配 I/O 流量的
				缓存部分无法配置。
			"enabled":
				用于分配 I/O 流量的缓存部分可以使用 "io_alloc_cbm" 配置。
			"not supported":
				该资源不支持此特性。

		可以通过写入接口来修改该特性，例如：

```

			# echo 1 > /sys/fs/resctrl/info/L3/io_alloc

		To disable::

			# echo 0 > /sys/fs/resctrl/info/L3/io_alloc

		底层实现可能会减少可用于通用（CPU）缓存分配的资源。请参阅
		下文特定架构的说明。根据使用需求，该特性可以启用或禁用。

		在 AMD 系统上，io_alloc 特性由 L3 Smart Data Cache Injection
		Allocation Enforcement（SDCIAE）支持。io_alloc 的 CLOSID 是该资源
		支持的最高 CLOSID。当 io_alloc 启用时，最高的 CLOSID 专用于
		io_alloc，不再可用于通用（CPU）缓存分配。当启用 CDP 时，io_alloc
		使用分配给指令缓存（CDP_CODE）的最高 CLOSID 来路由 I/O 流量，
		使得该 CLOSID 对于 CDP_CODE 和 CDP_DATA 资源都不再可用于
		通用（CPU）缓存分配。

```
"io_alloc_cbm":
		描述缓存实例部分的容量位掩码，当 "io_alloc" 启用时，来自受支持
		I/O 设备的 I/O 流量会被路由到这些缓存实例部分。

		CBM 以下列格式显示：

			<cache_id0>=<cbm>;<cache_id1>=<cbm>;...

```

			# cat /sys/fs/resctrl/info/L3/io_alloc_cbm
			0=ffff;1=ffff

		CBM 可以通过写入接口来配置。

		Example::

			# echo 1=ff > /sys/fs/resctrl/info/L3/io_alloc_cbm
			# cat /sys/fs/resctrl/info/L3/io_alloc_cbm
			0=ffff;1=00ff

			# echo "0=ff;1=f" > /sys/fs/resctrl/info/L3/io_alloc_cbm
			# cat /sys/fs/resctrl/info/L3/io_alloc_cbm
			0=00ff;1=000f

		一个 "*" ID 会用所提供的 CBM 配置所有域。

		在不需要掩码中最小连续位数量的系统上的示例::

			# echo "*=0" > /sys/fs/resctrl/info/L3/io_alloc_cbm
			# cat /sys/fs/resctrl/info/L3/io_alloc_cbm
			0=0;1=0

		当启用 CDP 时，与 CDP_DATA 和 CDP_CODE 资源关联的 "io_alloc_cbm"
		可能反映相同的值。例如，从 /sys/fs/resctrl/info/L3DATA/io_alloc_cbm
		读取和写入的值可能会由 /sys/fs/resctrl/info/L3CODE/io_alloc_cbm
		反映出来，反之亦然。

```
内存带宽（MB）子目录包含以下与分配相关的文件：

"min_bandwidth":
		用户可以请求的最小内存带宽百分比。

"bandwidth_gran":
		分配内存带宽百分比的粒度。分配的
		b/w 百分比会被舍入到硬件上可用的
		下一个控制步长。可用的带宽控制步长为：
		min_bandwidth + N * bandwidth_gran。

"delay_linear":
		指示延迟刻度是线性的还是非线性的。该字段
		纯粹是信息性的。

"thread_throttle_mode":
		在 Intel 系统上指示当一个物理核的线程请求不同的内存带宽
		百分比时，这些线程如何被节流：

		"max":
			最小的百分比被应用于所有线程
		"per-thread":
			带宽百分比被直接应用于运行在核上的
			线程

如果 L3 监控可用，将存在一个 "L3_MON" 目录，包含以下文件：

"num_rmids":
		硬件支持的用于 L3 监控事件的 RMID 数量。

"mon_features":
		如果为该资源启用了监控，则列出监控事件。
```

			# cat /sys/fs/resctrl/info/L3_MON/mon_features
			llc_occupancy
			mbm_total_bytes
			mbm_local_bytes

		如果系统支持带宽监控事件配置（BMEC），则带宽事件将
		可配置。输出将为::

			# cat /sys/fs/resctrl/info/L3_MON/mon_features
			llc_occupancy
			mbm_total_bytes
			mbm_total_bytes_config
			mbm_local_bytes
			mbm_local_bytes_config

```
"mbm_total_bytes_config"、"mbm_local_bytes_config":
		当支持带宽监控事件配置（BMEC）特性时，包含 mbm_total_bytes
		和 mbm_local_bytes 事件配置信息的读/写文件。事件配置设置是
		域特定的，并影响该域中的所有 CPU。当任一事件配置被更改时，
		该域中两个事件的所有 RMID 的带宽计数器（mbm_total_bytes 以及
		mbm_local_bytes）都会被清零。对每个 RMID 的后续读取将报告
		“Unavailable”，再之后的读取将报告有效值。

	支持的事件类型如下：

	====    ========================================================
	位    描述
	====    ========================================================
	6       来自 QoS 域、发往所有类型内存的脏受害者（Dirty Victims）
	5       对非本地 NUMA 域中慢速内存的读取
	4       对本地 NUMA 域中慢速内存的读取
	3       对非本地 NUMA 域的非临时（non-temporal）写入
	2       对本地 NUMA 域的非临时（non-temporal）写入
	1       对非本地 NUMA 域中内存的读取
	0       对本地 NUMA 域中内存的读取
	====    ========================================================

	默认情况下，mbm_total_bytes 配置被设为 0x7f 以统计所有事件类型，
	mbm_local_bytes 配置被设为 0x15 以统计所有本地内存事件。

	示例：

```

	  ::

	    # cat /sys/fs/resctrl/info/L3_MON/mbm_total_bytes_config
	    0=0x7f;1=0x7f;2=0x7f;3=0x7f

	    # cat /sys/fs/resctrl/info/L3_MON/mbm_local_bytes_config
	    0=0x15;1=0x15;3=0x15;4=0x15

	* 要将 mbm_total_bytes 改为只统计域 0 上的读操作，需要设置位 0、1、4 和 5，
	  即二进制的 110011b（十六进制 0x33）：
	  ::

	    # echo  "0=0x33" > /sys/fs/resctrl/info/L3_MON/mbm_total_bytes_config

	    # cat /sys/fs/resctrl/info/L3_MON/mbm_total_bytes_config
	    0=0x33;1=0x7f;2=0x7f;3=0x7f

	* 要将 mbm_local_bytes 改为统计域 0 和域 1 上所有慢速内存读操作，
	  需要设置位 4 和 5，即二进制的 110000b（十六进制 0x30）：
	  ::

	    # echo  "0=0x30;1=0x30" > /sys/fs/resctrl/info/L3_MON/mbm_local_bytes_config

	    # cat /sys/fs/resctrl/info/L3_MON/mbm_local_bytes_config
	    0=0x30;1=0x30;3=0x15;4=0x15

```
"mbm_assign_mode":
	支持的计数器分配模式。方括号表示当前启用的模式。当 "mbm_assign_mode"
	被更改时，与计数器关联的 MBM 事件可能会复位。
```

	  # cat /sys/fs/resctrl/info/L3_MON/mbm_assign_mode
	  [mbm_event]
	  default

	"mbm_event":

	mbm_event 模式允许用户将硬件计数器分配给 RMID、事件对，并在分配期间
	监控带宽使用情况。硬件会持续跟踪已分配的计数器，直到用户显式解除分配。
	resctrl 组内的每个事件可以独立分配。

	在此模式下，监控事件只有在有硬件计数器支撑时才能累加数据。使用每个
	CTRL_MON 和 MON 组中的 "mbm_L3_assignments" 来指定哪些事件应分配计数器。
	可用计数器数量在 "num_mbm_cntrs" 文件中描述。更改模式可能导致资源上的
	所有计数器复位。

	切换到 mbm_event 计数器分配模式需要用户将计数器分配给事件。否则，
	MBM 事件计数器在读取时将返回 'Unassigned'。

	该模式对支持比可用硬件计数器更多 CTRL_MON 和 MON 组的 AMD 平台有益。
	默认情况下，该特性在具有 ABMC（Assignable Bandwidth Monitoring Counters）
	能力的 AMD 平台上启用，确保即使相应的 RMID 未被任何处理器主动使用，
	计数器也保持分配状态。

	"default":

	在默认模式下，resctrl 假设每个 CTRL_MON 和 MON 组中的每个事件都有
	一个硬件计数器。在 AMD 平台上，建议使用 mbm_event 模式（若支持），
	以防止由于硬件重新分配计数器导致读取之间 MBM 事件复位。如果没有
	为事件分配计数器，这可能导致误导性数值或显示 "Unavailable"。

	* 启用 "mbm_event" 计数器分配模式：
	  ::

	    # echo "mbm_event" > /sys/fs/resctrl/info/L3_MON/mbm_assign_mode

	* 启用 "default" 监控模式：
	  ::

	    # echo "default" > /sys/fs/resctrl/info/L3_MON/mbm_assign_mode

```
"num_mbm_cntrs":
	当系统支持 mbm_event 模式时，每个域中计数器（可用与已分配计数器之和）
	的最大数量。

	例如，在一个每个 L3 域最多有 32 个内存带宽监控计数器的系统上：
```

	  # cat /sys/fs/resctrl/info/L3_MON/num_mbm_cntrs
	  0=32;1=32

```
"available_mbm_cntrs":
	当系统上启用了 mbm_event 模式时，每个域中可用于分配的计数器数量。

	例如，在一个每个 L3 域有 30 个可用[硬件]可分配计数器的系统上：
```

	  # cat /sys/fs/resctrl/info/L3_MON/available_mbm_cntrs
	  0=30;1=30

```
"event_configs":
	当支持 "mbm_event" 计数器分配模式时存在的目录。为每个可分配给计数器的
	MBM 事件包含一个子目录。

	默认支持两个 MBM 事件：mbm_local_bytes 和 mbm_total_bytes。每个 MBM 事件的
	子目录包含一个名为 "event_filter" 的文件，用于查看和修改该 MBM 事件
	配置的是哪些内存事务。该文件仅在启用 "mbm_event" 计数器分配模式时
	可访问。

	支持的内存事务类型列表：

	==========================  ========================================================
	名称			    描述
	==========================  ========================================================
	dirty_victim_writes_all     来自 QoS 域、发往所有类型内存的脏受害者（Dirty Victims）
	remote_reads_slow_memory    对非本地 NUMA 域中慢速内存的读取
	local_reads_slow_memory     对本地 NUMA 域中慢速内存的读取
	remote_non_temporal_writes  对非本地 NUMA 域的非临时（non-temporal）写入
	local_non_temporal_writes   对本地 NUMA 域的非临时（non-temporal）写入
	remote_reads                对非本地 NUMA 域中内存的读取
	local_reads                 对本地 NUMA 域中内存的读取
	==========================  ========================================================

```

	  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_total_bytes/event_filter
	  local_reads,remote_reads,local_non_temporal_writes,remote_non_temporal_writes,
	  local_reads_slow_memory,remote_reads_slow_memory,dirty_victim_writes_all

	  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_local_bytes/event_filter
	  local_reads,local_non_temporal_writes,local_reads_slow_memory

	通过写入 "event_configs" 目录中的 "event_filter" 文件来修改事件配置。
	读/写 "event_filter" 文件包含该事件的配置，反映被其统计的是哪些内存事务。

	例如::

	  # echo "local_reads, local_non_temporal_writes" >
	    /sys/fs/resctrl/info/L3_MON/event_configs/mbm_total_bytes/event_filter

	  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_total_bytes/event_filter
	   local_reads,local_non_temporal_writes

```
"mbm_assign_on_mkdir":
	当支持 "mbm_event" 计数器分配模式时存在。仅当启用 "mbm_event" 计数器
	分配模式时可访问。

	决定在使用 mkdir 创建其关联的监控组时，是否自动将计数器分配给 RMID、MBM 事件对。
	在启动时默认启用，从 "default" 模式切换到 "mbm_event" 计数器分配模式时
	也默认启用。用户可以通过写入接口来禁用此能力。

	"0":
		自动分配被禁用。
	"1":
		自动分配被启用。

```

	  # echo 0 > /sys/fs/resctrl/info/L3_MON/mbm_assign_on_mkdir
	  # cat /sys/fs/resctrl/info/L3_MON/mbm_assign_on_mkdir
	  0

```
"max_threshold_occupancy":
		读/写文件，提供先前使用的 LLC_occupancy 计数器可被考虑
		复用时的最大值（以字节为单位）。

如果遥测监控可用，将存在一个 "PERF_PKG_MON" 目录，包含以下文件：

"num_rmids":
		用于遥测监控事件的 RMID 数量。

		在 Intel 上，如果可并发跟踪的 RMID 数量低于支持的 RMID 总数，
		resctrl 将不会启用遥测事件。可以使用 "rdt=" 内核参数强制启用
		遥测事件，但这可能会减少可创建的监控组数量。

"mon_features":
		列出该系统上已启用的遥测监控事件。

可创建的 "CTRL_MON" + "MON" 数量上限，是 L3_MON 和 PERF_PKG_MON 的
"num_rmids" 值中的较小者。

最后，在 "info" 目录的顶层有一个名为 "last_cmd_status" 的文件。它随着每次通过
文件系统发出的“命令”（创建新目录或写入任何控制文件）而重置。如果命令成功，
它将读为 "ok"。如果命令失败，它将提供比文件操作错误返回中能传达的更多信息。例如：
```

	# echo L3:0=f7 > schemata
	bash: echo: write error: Invalid argument
	# cat info/last_cmd_status
	mask f7 has non-consecutive 1-bits

```
## 资源分配与监控组


资源组在 resctrl 文件系统中以目录形式表示。默认组是根目录，挂载后
立即拥有系统中的所有任务和 CPU，并可以充分利用所有资源。

在具有 RDT 控制特性的系统上，可以在根目录中创建额外的目录，用以指定
每种资源的不同数量（参见下文的 "schemata"）。根目录以及这些额外的顶级
目录在下文中被称为 "CTRL_MON" 组。

在具有 RDT 监控的系统上，根目录和其他顶级目录包含一个名为 "mon_groups" 的
目录，在其中可以创建额外的目录来监控作为其祖先的 CTRL_MON 组中任务的子集。
在本文档的其余部分中，这些被称为 "MON" 组。

移除一个目录会将其所代表的组拥有的所有任务和 CPU 移动到父目录。移除某个
已创建的 CTRL_MON 组会自动移除其下方的所有 MON 组。

支持将 MON 组目录移动到新的父 CTRL_MON 组，目的是在不影响其监控数据或
已分配任务的情况下更改 MON 组的资源分配。对于监控 CPU 的 MON 组不允许
此操作。除了简单地重命名 CTRL_MON 或 MON 组之外，当前不允许其他移动操作。

所有组都包含以下文件：

"tasks":
		读取该文件会显示属于该组的所有任务的列表。向文件写入一个
		任务 id 会将该任务添加到组中。可以通过用逗号分隔任务 id 来
		添加多个任务。任务将按顺序分配。不支持多个失败。在尝试分配
		任务时遇到的单次失败将导致操作中止，而失败之前已添加的任务
		将保留在组中。失败将被记录到 /sys/fs/resctrl/info/last_cmd_status。

		如果该组是 CTRL_MON 组，则该任务会从先前拥有该任务的 CTRL_MON 组
		以及任何拥有该任务的 MON 组中移除。如果该组是 MON 组，则该任务
		必须已经属于该组的 CTRL_MON 父组。该任务会从任何先前的 MON 组中移除。


"cpus":
		读取该文件会显示该组拥有的逻辑 CPU 的位掩码。向该文件写入
		一个掩码会向该组添加或从中移除 CPU。与 tasks 文件一样，会维护
		一个层级关系，即 MON 组只能包含父 CTRL_MON 组拥有的 CPU。
		当资源组处于伪锁定模式时，该文件将仅为只读，反映与伪锁定
		区域关联的 CPU。


"cpus_list":
		与 "cpus" 类似，只是使用 CPU 范围而不是位掩码。


当启用控制时，所有 CTRL_MON 组还将包含：

"schemata":
		可供该组使用的所有资源的列表。每个资源有自己的行和格式
		——详见下文。

"size":
		镜像 "schemata" 文件的显示，以字节显示每个分配的大小，
		而不是显示代表分配的位。

"mode":
		资源组的 "mode" 决定其分配的共享方式。"shareable" 资源组
		允许共享其分配，而 "exclusive" 资源组则不允许。缓存伪锁定
		区域是通过先向 "mode" 文件写入 "pseudo-locksetup"，再将缓存
		伪锁定区域的 schemata 写入资源组的 "schemata" 文件来创建的。
		伪锁定区域创建成功后，模式会自动变为 "pseudo-locked"。

"ctrl_hw_id":
		仅在使用 debug 选项时可用。硬件用于标识控制组的标识符。
		在 x86 上这就是 CLOSID。

当启用监控时，所有 MON 组还将包含：

"mon_data":
		它包含每个监控域的目录。

		如果启用了 L3 监控，将为每个 L3 缓存实例提供一个 "mon_L3_XX" 目录。
		每个目录包含已启用 L3 事件的文件（例如 "llc_occupancy"、
		"mbm_total_bytes" 和 "mbm_local_bytes"）。

		如果启用了遥测监控，将为每个物理处理器封装提供一个 "mon_PERF_PKG_YY"
		目录。每个目录包含已启用遥测事件的文件（例如 "core_energy"、
		"activity"、"uops_retired" 等）。

		info/`*`/mon_features 文件提供已启用事件/文件名的完整列表。

		"core energy" 报告一个浮点数，表示在当前监控组所对应的封装上，
		所有逻辑 CPU 在执行指令期间核心（寄存器、算术单元、TLB 和
		L1/L2 缓存）消耗的能量（以焦耳为单位）。

		"activity" 也报告一个浮点值（以法拉第为单位）。它提供与 CPU
		用于执行的频率无关的已完成工作的估计。

		注意 "core energy" 和 "activity" 仅测量 CPU "core" 中的能量/活动
		（算术单元、TLB、L1 和 L2 缓存等）。它们不包括 L3 缓存、内存、
		I/O 设备等。

		所有其他事件报告十进制整数值。

		在 MON 组中，这些文件提供该组中所有任务的事件当前值的读数。
		在 CTRL_MON 组中，这些文件提供 CTRL_MON 组以及所有 MON 组中
		所有任务的合计值。更多使用细节请参见示例部分。

		在启用了 Sub-NUMA Cluster（SNC）的系统上，每个节点都有额外的
		目录（位于其所占据的 L3 缓存的 "mon_L3_XX" 目录内）。这些目录
		命名为 "mon_sub_L3_YY"，其中 "YY" 是节点编号。

		当启用 'mbm_event' 计数器分配模式时，如果 MON 组的某个 MBM 事件
		没有分配硬件计数器，读取该事件将返回 'Unassigned'。对于 CTRL_MON
		组，如果某个 MBM 事件在 CTRL_MON 组及其任何关联的 MON 组中都没有
		已分配的计数器，则返回 'Unassigned'。

"mon_hw_id":
		仅在使用 debug 选项时可用。硬件用于标识监控组的标识符。
		在 x86 上这就是 RMID。

当启用监控时，所有 MON 组还可能包含：

"mbm_L3_assignments":
		当支持 "mbm_event" 计数器分配模式时存在，并列出该组的计数器
		分配状态。

		分配列表以下列格式显示：

	<Event>:<Domain ID>=<Assignment state>;<Domain ID>=<Assignment state>

	Event: 一个有效的 MBM 事件，位于
	       /sys/fs/resctrl/info/L3_MON/event_configs 目录中。

	Domain ID: 一个有效的域 ID。写入时，'*' 将更改应用到
		   所有域。

	Assignment states:

	_ : 未分配计数器。

	e : 以独占方式分配了计数器。

	示例：

	显示默认组的计数器分配状态。
```

	 # cd /sys/fs/resctrl
	 # cat /sys/fs/resctrl/mbm_L3_assignments
	   mbm_total_bytes:0=e;1=e
	   mbm_local_bytes:0=e;1=e

	分配可以通过写入接口来修改。

	示例：

	解除域 0 上与 mbm_total_bytes 事件关联的计数器的分配：
	::

	 # echo "mbm_total_bytes:0=_" > /sys/fs/resctrl/mbm_L3_assignments
	 # cat /sys/fs/resctrl/mbm_L3_assignments
	   mbm_total_bytes:0=_;1=e
	   mbm_local_bytes:0=e;1=e

	解除所有域上与 mbm_total_bytes 事件关联的计数器的分配：
	::

	 # echo "mbm_total_bytes:*=_" > /sys/fs/resctrl/mbm_L3_assignments
	 # cat /sys/fs/resctrl/mbm_L3_assignments
	   mbm_total_bytes:0=_;1=_
	   mbm_local_bytes:0=e;1=e

	以独占模式为所有域分配与 mbm_total_bytes 事件关联的计数器：
	::

	 # echo "mbm_total_bytes:*=e" > /sys/fs/resctrl/mbm_L3_assignments
	 # cat /sys/fs/resctrl/mbm_L3_assignments
	   mbm_total_bytes:0=e;1=e
	   mbm_local_bytes:0=e;1=e

```
当使用 "mba_MBps" 挂载选项时，所有 CTRL_MON 组还将包含：

"mba_MBps_event":
		读取该文件会显示哪个内存带宽事件被用作软件反馈回路的输入，
		该回路使内存带宽保持在 schemata 文件中指定的值以下。写入在
		/sys/fs/resctrl/info/L3_MON/mon_features 中找到的某个受支持
		内存带宽事件的名称，可更改输入事件。

### 资源分配规则


当任务运行时，以下规则定义了哪些资源对其可用：

1) 如果任务是非默认组的成员，则使用该组的 schemata。

2) 否则，如果任务属于默认组，但运行在分配给某个特定组的 CPU 上，则
   使用该 CPU 所在组的 schemata。

3) 其他情况下，使用默认组的 schemata。

### 资源监控规则


1) 如果任务是 MON 组或非默认 CTRL_MON 组的成员，则该任务的 RDT 事件
   将在该组中报告。

2) 如果任务是默认 CTRL_MON 组的成员，但运行在分配给某个特定组的 CPU 上，
   则该任务的 RDT 事件将在该组中报告。

3) 其他情况下，该任务的 RDT 事件将在根级别的 "mon_data" 组中报告。


## 缓存占用监控与控制的注意事项


当将任务从一个组移动到另一个组时，你应当记住这只影响任务**新**的
缓存分配。例如，你可能有一个任务在监控组中显示 3 MB 的缓存占用。
如果你移动到一个新组并立即检查旧组和新组的占用，你很可能会看到旧组
仍显示 3 MB，而新组为零。当任务访问移动前仍在缓存中的位置时，硬件
不会更新任何计数器。在繁忙的系统上，你会看到旧组的占用随着缓存行被
驱逐和重用而下降，而新组的占用随着任务访问内存并加载到缓存（基于其在
新组中的成员关系计数）而上升。

这同样适用于缓存分配控制。将任务移动到具有更小缓存分区的组不会
驱逐任何缓存行。进程可能会继续从旧分区使用它们。

硬件使用 CLOSid（Class of service ID）和 RMID（Resource monitoring ID）
分别标识控制组和监控组。每个资源组根据这些组的类型映射到这些 ID。
CLOSid 和 RMID 的数量受硬件限制，因此当 CLOSID 或 RMID 任一耗尽时，
创建 "CTRL_MON" 目录可能会失败；而当 RMID 耗尽时，创建 "MON" 组
可能会失败。

### max_threshold_occupancy —— 通用概念


注意，一个 RMID 一旦被释放，可能不会立即可用，因为该 RMID 仍标记着
先前 RMID 用户的缓存行。因此，此类 RMID 会被放入 limbo（边缘）列表，
并在缓存占用下降时重新检查。如果系统中存在大量 limbo RMID 但尚未
准备好使用，用户在 mkdir 时可能会看到 -EBUSY。

max_threshold_occupancy 是一个用户可配置的值，用于确定 RMID 可以被
释放时的占用水平。

mon_llc_occupancy_limbo 跟踪点给出了不可立即分配的一部分 RMID 的精确
占用（以字节为单位）。不能依赖它每秒都产生输出，可能需要尝试创建一个
空的监控组来强制更新。只有在创建控制组或监控组失败时才会产生输出。

### Schemata 文件 —— 通用概念


文件中的每一行描述一个资源。该行以资源名称开头，后跟要应用于系统上
该资源每个实例的特定值。

### 缓存 ID


在当前一代系统上，每个插槽（socket）有一个 L3 缓存，而 L2 缓存通常
仅由核上的超线程共享，但这并非架构要求。我们可能在一个插槽上有多个
独立的 L3 缓存，多个核也可能共享一个 L2 缓存。因此，我们不使用
"socket" 或 "core" 来定义共享某个资源的逻辑 CPU 集合，而是使用
"Cache ID"。在给定的缓存级别上，它将是整个系统中的一个唯一编号（但不
保证是连续序列，可能存在空隙）。要查找每个逻辑 CPU 的 ID，请查看
/sys/devices/system/cpu/cpu**/cache/index**/id

### 缓存位掩码（CBM）


对于缓存资源，我们使用位掩码描述可用于分配的缓存部分。掩码的最大值
由每个 CPU 型号定义（并且对于不同的缓存级别可能不同）。它可以通过
CPUID 找到，但也由 resctrl 文件系统的 "info" 目录以
"info/{resource}/cbm_mask" 提供。某些 Intel 硬件要求这些掩码的所有
'1' 位都在一个连续的块中。因此 0x3、0x6 和 0xC 是包含两个置位位的
合法 4 位掩码，但 0x5、0x9 和 0xA 不是。请查看
/sys/fs/resctrl/info/{resource}/sparse_masks 以确认是否支持非连续的
1 值。在一个具有 20 位掩码的系统上，每个位代表缓存容量的 5%。你可以用
掩码 0x1f、0x3e0、0x7c00、0xf8000 将缓存分成四个相等的部分。

## 关于 Sub-NUMA Cluster 模式的注意事项


当启用 SNC 模式时，Linux 可能比在常规 NUMA 节点之间更积极地在 Sub-NUMA
节点之间平衡任务，因为 Sub-NUMA 节点上的 CPU 共享同一个 L3 缓存，并且
系统报告的 Sub-NUMA 节点之间的 NUMA 距离可能低于常规 NUMA 节点使用的
值。

每个 "mon_L3_XX" 目录中的顶级监控文件提供共享一个 L3 缓存实例的所有
SNC 节点上的数据总和。将任务绑定到特定 Sub-NUMA 节点 CPU 的用户可以
读取 "mon_sub_L3_YY" 目录中的 "llc_occupancy"、"mbm_total_bytes" 和
"mbm_local_bytes" 来获取节点本地数据。

内存带宽分配仍然在 L3 缓存级别执行。即节流控制应用于所有 SNC 节点。

L3 缓存分配位图也应用于所有 SNC 节点。但请注意，每个位所代表的 L3
缓存量要除以每个 L3 缓存的 SNC 节点数。例如，在一个具有 100MB 缓存、
10 位分配掩码的系统上，每个位通常代表 10MB。启用 SNC 模式且每个 L3
缓存有两个 SNC 节点时，每个位仅代表 5MB。

## 内存带宽的分配与监控


对于内存带宽资源，默认情况下用户通过指示总内存带宽的百分比来控制
该资源。

每个 CPU 型号的最小带宽百分比值是预定义的，可以通过 "info/MB/min_bandwidth"
查找。所分配的带宽粒度也取决于 CPU 型号，可以通过 "info/MB/bandwidth_gran"
查找。可用的带宽控制步长为：min_bw + N * bw_gran。中间值会被舍入到
硬件上可用的下一个控制步长。

在某些 Intel SKU 上，带宽节流是一种核特定的机制。在两个共享一个核的
线程上使用高带宽和低带宽设置，可能导致两个线程都被节流为使用低带宽
（参见 "thread_throttle_mode"）。

内存带宽分配（MBA）可能是核特定的机制，而内存带宽监控（MBM）是在
封装级别完成的，这一事实可能导致用户在尝试通过 MBA 应用控制然后监控
带宽以查看控制是否有效时感到困惑。以下是此类场景：

1. 当用户提高百分比值时，可能**不会**看到实际带宽增加：

当聚合的 L2 外部带宽大于 L3 外部带宽时会发生这种情况。考虑一个 SKL
SKU，一个封装上有 24 个核，L2 外部带宽为 10GBps（因此聚合 L2 外部带宽为
240GBps），L3 外部带宽为 100GBps。现在一个 '20 个线程、具有 50% 带宽、
每个消耗 5GBps' 的工作负载消耗了 100GBps 的最大 L3 带宽，尽管指定的
百分比值仅为 50% << 100%。因此增加带宽百分比不会产生更多带宽。这是
因为尽管 L2 外部带宽仍有容量，但 L3 外部带宽已完全用尽。另请注意，这
将取决于基准测试运行的核数。

2. 相同的带宽百分比可能意味着不同的实际带宽，具体取决于线程数：

对于 #1 中相同的 SKU，'单线程、10% 带宽' 和 '4 线程、10% 带宽' 可以
分别消耗高达 10GBps 和 40GBps，尽管它们具有相同的 10% 带宽百分比。
这仅仅是因为随着线程开始在 rdtgroup 中使用更多核，实际带宽可能会增加或
变化，即使用户指定的带宽百分比相同。

为了缓解这种情况并使接口更友好，resctrl 增加了以 MiBps 指定带宽的支持。
底层内核将使用软件反馈机制或 "Software Controller（mba_sc）"，它使用
MBM 计数器读取实际带宽
```

	"actual bandwidth < user specified bandwidth".

```
默认情况下，schemata 采用带宽百分比值，而用户可以使用挂载选项 'mba_MBps'
切换到 "MBA software controller" 模式。schemata 格式在下文各节中指定。
### L3 schemata 文件细节（代码与数据优先级划分已禁用）


```

	L3:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...

```
### L3 schemata 文件细节（通过挂载选项为 resctrl 启用 CDP）


当启用 CDP 时，L3 控制被拆分为两个独立的资源
```

	L3DATA:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...
	L3CODE:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...

```
### L2 schemata 文件细节


L2 通过 'cdpl2' 挂载选项支持 CDP。其 schemata
```

	L2:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...

```
或

	L2DATA:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...
	L2CODE:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...


### 内存带宽分配（默认模式）


内存 b/w 域是 L3 缓存。
```

	MB:<cache_id0>=bandwidth0;<cache_id1>=bandwidth1;...

```
### 以 MiBps 指定的内存带宽分配


内存带宽域是 L3 缓存。
```

	MB:<cache_id0>=bw_MiBps0;<cache_id1>=bw_MiBps1;...

```
### 慢速内存带宽分配（SMBA）


AMD 硬件支持慢速内存带宽分配（SMBA）。
CXL.memory 是唯一受支持的“慢速”内存设备。借助 SMBA 的支持，硬件
在慢速内存设备上启用带宽分配。如果系统中有多个此类设备，节流逻辑会
将所有慢速来源归为一组，并对它们整体施加限制。

SMBA（配合 CXL.memory）的存在与是否存在慢速内存设备无关。如果系统上
没有此类设备，则配置 SMBA 不会对系统性能产生影响。

慢速内存的带宽域是 L3 缓存。其 schemata 文件格式如下：
```

	SMBA:<cache_id0>=bandwidth0;<cache_id1>=bandwidth1;...

```
### 读/写 schemata 文件


读取 schemata 文件会显示所有域上所有资源的状态。写入时只需指定
你希望更改的那些值。例如：
```

  # cat schemata
  L3DATA:0=fffff;1=fffff;2=fffff;3=fffff
  L3CODE:0=fffff;1=fffff;2=fffff;3=fffff
  # echo "L3DATA:2=3c0;" > schemata
  # cat schemata
  L3DATA:0=fffff;1=fffff;2=3c0;3=fffff
  L3CODE:0=fffff;1=fffff;2=fffff;3=fffff

```
### 读/写 schemata 文件（在 AMD 系统上）


读取 schemata 文件会显示所有域上的当前带宽限制。所分配的资源是
八分之一 GB/s 的整数倍。写入文件时，需要指定要配置带宽限制的缓存 id。

例如，要在第一个缓存 id 上分配 2GB/s 的限制：

```

  # cat schemata
    MB:0=2048;1=2048;2=2048;3=2048
    L3:0=ffff;1=ffff;2=ffff;3=ffff

  # echo "MB:1=16" > schemata
  # cat schemata
    MB:0=2048;1=  16;2=2048;3=2048
    L3:0=ffff;1=ffff;2=ffff;3=ffff

```
### 读/写 schemata 文件（在 AMD 系统上，带 SMBA 特性）


schemata 文件的读写与上一节中不带 SMBA 时相同。

例如，要在第一个缓存 id 上分配 8GB/s 的限制：

```

  # cat schemata
    SMBA:0=2048;1=2048;2=2048;3=2048
      MB:0=2048;1=2048;2=2048;3=2048
      L3:0=ffff;1=ffff;2=ffff;3=ffff

  # echo "SMBA:1=64" > schemata
  # cat schemata
    SMBA:0=2048;1=  64;2=2048;3=2048
      MB:0=2048;1=2048;2=2048;3=2048
      L3:0=ffff;1=ffff;2=ffff;3=ffff

```
## 缓存伪锁定


CAT 使用户能够指定应用程序可以填充的缓存空间大小。缓存伪锁定建立在一个
事实之上：CPU 在缓存命中时仍然可以读写其当前分配区域之外、预先分配的数据。
通过缓存伪锁定，数据可以被预加载到缓存中一个保留的、任何应用程序都无法
填充的部分，并从那时起只服务于缓存命中。缓存伪锁定的内存被提供给用户空间，
应用程序可以将其映射到自己的虚拟地址空间，从而拥有一块平均读取延迟降低的
内存区域。

缓存伪锁定区域的创建由用户发出的一则请求触发，该请求附带待伪锁定区域的
schemata。缓存伪锁定区域按如下方式创建：

- 创建一个 CAT 分配 CLOSNEW，其 CBM 匹配将包含伪锁定内存的缓存区域的
  用户 schemata。该区域不得与系统上任何当前的 CAT 分配/CLOS 重叠，并且在
  伪锁定区域存在期间不允许将来与该缓存区域发生重叠。
- 创建一块与缓存区域大小相同的连续内存区域。
- 刷新缓存，禁用硬件预取器，禁用抢占。
- 将 CLOSNEW 设为活动 CLOS，并触碰已分配的内存以将其加载到缓存中。
- 将先前的 CLOS 设为活动 CLOS。
- 此时可以释放 closid CLOSNEW —— 只要其 CBM 不出现在任何 CAT 分配中，
  缓存伪锁定区域就受到保护。尽管缓存伪锁定区域从此时起不会出现在任何
  CLOS 的任何 CBM 中，但运行于任何 CLOS 下的应用程序都将能够访问伪锁定
  区域中的内存，因为该区域会继续服务于缓存命中。
- 加载到缓存中的连续内存区域作为字符设备暴露给用户空间。

缓存伪锁定通过仔细配置 CAT 特性并控制应用程序行为，来提高数据保留在
缓存中的概率。但无法保证数据一定被放入缓存。诸如 INVD、WBINVD、CLFLUSH
等指令仍可能将“锁定”的数据从缓存中驱逐。电源管理 C-states 可能会收缩或
关闭缓存。在创建伪锁定区域时，更深的 C-states 会被自动限制。

使用伪锁定区域的应用程序必须以亲和性运行在与伪锁定区域所在缓存关联的
核（或核的子集）上。代码中的一项健全性检查将不允许应用程序映射伪锁定
内存，除非它以亲和性运行在与伪锁定区域所在缓存关联的核上。该健全性检查
仅在初始的 mmap() 处理期间进行，之后没有强制，应用程序自身需要确保保持
对正确核的亲和性。

伪锁定分两个阶段完成：

1) 在第一阶段，系统管理员分配一部分应专用于伪锁定的缓存。此时会分配
   等量的内存，加载到已分配的缓存部分，并作为字符设备暴露。
2) 在第二阶段，用户空间应用程序将伪锁定内存映射（mmap()）到其地址空间。

### 缓存伪锁定接口


使用 resctrl 接口创建伪锁定区域的方式如下：

1) 在 /sys/fs/resctrl 中创建一个新目录来创建新的资源组。
2) 通过向 "mode" 文件写入 "pseudo-locksetup"，将新资源组的模式改为
   "pseudo-locksetup"。
3) 将伪锁定区域的 schemata 写入 "schemata" 文件。根据 "bit_usage" 文件，
   schemata 中的所有位都应为 "unused"。

伪锁定区域创建成功后，"mode" 文件将包含 "pseudo-locked"，并且一个与资源组
同名的新字符设备将存在于 /dev/pseudo_lock 中。用户空间可以对这个字符设备
进行 mmap()，以获取对伪锁定内存区域的访问。

缓存伪锁定区域的创建与使用示例见下文。

### 缓存伪锁定调试接口


伪锁定调试接口默认启用（如果启用了 CONFIG_DEBUG_FS），可以在
/sys/kernel/debug/resctrl 中找到。

内核没有显式的方法测试某个给定的内存位置是否存在于缓存中。伪锁定调试接口
使用跟踪基础设施提供两种测量伪锁定区域缓存驻留度的方式：

1) 使用 pseudo_lock_mem_latency 跟踪点的内存访问延迟。这些测量的数据最好
   使用 hist 触发器可视化（见下例）。在此测试中，伪锁定区域以 32 字节的
   步长被遍历，同时硬件预取器和抢占被禁用。这也提供了缓存命中与未命中的
   替代可视化。
2) 如果可用，使用特定于型号的精确计数器测量缓存命中与未命中。根据系统上
   缓存的级别，pseudo_lock_l2 和 pseudo_lock_l3 跟踪点可用。

当创建伪锁定区域时，会在 debugfs 中为其创建一个新目录：
/sys/kernel/debug/resctrl/<newdir>。该目录中存在一个只写文件
pseudo_lock_measure。伪锁定区域的测量取决于写入此 debugfs 文件的数字：

1:
     向 pseudo_lock_measure 文件写入 "1" 将触发 pseudo_lock_mem_latency
     跟踪点捕获的延迟测量。见下例。
2:
     向 pseudo_lock_measure 文件写入 "2" 将触发 L2 缓存驻留度（缓存命中与
     未命中）测量，由 pseudo_lock_l2 跟踪点捕获。见下例。
3:
     向 pseudo_lock_measure 文件写入 "3" 将触发 L3 缓存驻留度（缓存命中与
     未命中）测量，由 pseudo_lock_l3 跟踪点捕获。

所有测量都通过跟踪基础设施记录。这要求在触发测量之前启用相关的跟踪点。

#### 延迟调试接口示例


在此示例中，创建了一个名为 "newlock" 的伪锁定区域。下面我们展示如何测量
从该区域读取的延迟（以周期为单位），并使用在启用 CONFIG_HIST_TRIGGERS 时
可用的直方图将其可视化
```

  # :> /sys/kernel/tracing/trace
  # echo 'hist:keys=latency' > /sys/kernel/tracing/events/resctrl/pseudo_lock_mem_latency/trigger
  # echo 1 > /sys/kernel/tracing/events/resctrl/pseudo_lock_mem_latency/enable
  # echo 1 > /sys/kernel/debug/resctrl/newlock/pseudo_lock_measure
  # echo 0 > /sys/kernel/tracing/events/resctrl/pseudo_lock_mem_latency/enable
  # cat /sys/kernel/tracing/events/resctrl/pseudo_lock_mem_latency/hist

  # event histogram
  #
  # trigger info: hist:keys=latency:vals=hitcount:sort=hitcount:size=2048 [active]
  #

  { latency:        456 } hitcount:          1
  { latency:         50 } hitcount:         83
  { latency:         36 } hitcount:         96
  { latency:         44 } hitcount:        174
  { latency:         48 } hitcount:        195
  { latency:         46 } hitcount:        262
  { latency:         42 } hitcount:        693
  { latency:         40 } hitcount:       3204
  { latency:         38 } hitcount:       3484

  Totals:
      Hits: 8192
      Entries: 9
    Dropped: 0

```
#### 缓存命中/未命中调试示例


在此示例中，在一个平台的 L2 缓存上创建了一个名为 "newlock" 的伪锁定区域。
下面我们展示如何使用平台的精确计数器获取缓存命中与未命中的详情。
```

  # :> /sys/kernel/tracing/trace
  # echo 1 > /sys/kernel/tracing/events/resctrl/pseudo_lock_l2/enable
  # echo 2 > /sys/kernel/debug/resctrl/newlock/pseudo_lock_measure
  # echo 0 > /sys/kernel/tracing/events/resctrl/pseudo_lock_l2/enable
  # cat /sys/kernel/tracing/trace

  # tracer: nop
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
  pseudo_lock_mea-1672  [002] ....  3132.860500: pseudo_lock_l2: hits=4097 miss=0


```
#### RDT 分配使用示例


1) 示例 1

在一台双插槽机器（每插槽一个 L3 缓存）上，缓存位掩码仅有 4 位，最小
b/w 为 10%，内存带宽粒度为 10%。
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir p0 p1
  # echo "L3:0=3;1=c\nMB:0=50;1=50" > /sys/fs/resctrl/p0/schemata
  # echo "L3:0=3;1=3\nMB:0=50;1=50" > /sys/fs/resctrl/p1/schemata

```
默认资源组未被修改，因此我们可以访问所有缓存的所有部分（其 schemata 文件
读为 "L3:0=f;1=f"）。

处于 "p0" 组控制下的任务只能从缓存 ID 0 的“较低” 50% 和缓存 ID 1 的
“较高” 50% 中分配。处于 "p1" 组中的任务在两个插槽上都使用缓存的“较低”
50%。

类似地，处于 "p0" 组控制下的任务在 socket0 上最多可使用 50% 的内存 b/w，
在 socket 1 上最多 50%。处于 "p1" 组中的任务在两个插槽上也最多可使用 50%
的内存 b/w。注意，与缓存掩码不同，内存 b/w 无法指定这些分配是否可以重叠。
分配指定的是该组可能能够使用的最大 b/w，系统管理员可以相应地配置 b/w。

如果 resctrl 使用软件控制器（mba_sc），则用户可以输入以 MB 为单位的最大
b/w，而不是百分比值。
```

  # echo "L3:0=3;1=c\nMB:0=1024;1=500" > /sys/fs/resctrl/p0/schemata
  # echo "L3:0=3;1=3\nMB:0=1024;1=500" > /sys/fs/resctrl/p1/schemata

```
在上述示例中，socket 0 上 "p1" 和 "p0" 中的任务将使用 1024MB 的最大 b/w，
而在 socket 1 上它们将使用 500MB。

2) 示例 2

同样是双插槽，但这次使用更实际的 20 位掩码。

在一台双插槽双核机器上，socket 0 上有两个实时任务：运行在处理器 0 上的
pid=1234 和运行在处理器 1 上的 pid=5678。为了避免吵闹的邻居，这两个实时
任务各自独占占用 socket 0 上 L3 缓存的四分之一。
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl

```
首先我们重置默认组的 schemata，使得 socket 0 上 L3 缓存的“较高” 50% 和
50% 的内存 b/w 无法被使用
```

  # echo "L3:0=3ff;1=fffff\nMB:0=50;1=100" > schemata

```
接下来我们为第一个实时任务创建一个资源组，并让它访问 socket 0 上缓存的
“顶部” 25%。
```

  # mkdir p0
  # echo "L3:0=f8000;1=fffff" > p0/schemata

```
最后我们将第一个实时任务移入这个资源组。我们还使用 taskset(1) 确保该任务
始终运行在 socket 0 上专用的 CPU 上。大多数资源组的使用也会限制任务运行在
哪些处理器上。
```

  # echo 1234 > p0/tasks
  # taskset -cp 1 1234

```
```

  # mkdir p1
  # echo "L3:0=7c00;1=fffff" > p1/schemata
  # echo 5678 > p1/tasks
  # taskset -cp 2 5678

```
对于同样的双插槽系统，带有内存 b/w 资源和 CAT L3，schemata 将如下所示
（假设 min_bandwidth 为 10，bandwidth_gran 为 10）：

对于第一个实时任务，这将请求 socket 0 上 20% 的内存 b/w。
```

  # echo -e "L3:0=f8000;1=fffff\nMB:0=20;1=100" > p0/schemata

```
对于第二个实时任务，这将请求 socket 0 上另外 20% 的内存 b/w。
```

  # echo -e "L3:0=f8000;1=fffff\nMB:0=20;1=100" > p0/schemata

```
3) 示例 3

一个单插槽系统，实时任务运行在核 4-7 上，非实时工作负载分配到核 0-3。
实时任务共享代码和数据，因此不需要逐任务的关联；并且由于与内核的交互，
希望这些核上的内核与任务共享 L3。
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl

```
首先我们重置默认组的 schemata，使得 socket 0 上 L3 缓存的“较高” 50%，以及
socket 0 上 50% 的内存带宽无法被使用
```

  # echo "L3:0=3ff\nMB:0=50" > schemata

```
接下来我们为实时核创建一个资源组，并让它访问 socket 0 上缓存的“顶部” 50%
以及 socket 0 上 50% 的内存带宽。
```

  # mkdir p0
  # echo "L3:0=ffc00\nMB:0=50" > p0/schemata

```
最后我们将核 4-7 移到新组，并确保运行在那里的内核和任务获得 50% 的缓存。
假设核 4-7 是 SMT 兄弟核，并且只有实时线程被调度到核 4-7 上，它们也应该
获得 50% 的内存带宽。
```

  # echo F0 > p0/cpus

```
4) 示例 4

前面示例中的资源组都处于默认的 "shareable" 模式，允许共享其缓存分配。如果
一个资源组配置了缓存分配，没有任何东西能阻止另一个资源组与该分配重叠。

在此示例中，将在一个具有两个 L2 缓存实例的 L2 CAT 系统上创建一个新的独占
资源组，这两个实例可以用 8 位容量位掩码配置。新的独占资源组将被配置为使用
每个缓存实例的 25%。
```

  # mount -t resctrl resctrl /sys/fs/resctrl/
  # cd /sys/fs/resctrl

```
首先，我们观察到默认组被配置为分配到所有 L2
```

  # cat schemata
  L2:0=ff;1=ff

```
我们本可以在此时尝试创建新的资源组，但它会
```

  # mkdir p0
  # echo 'L2:0=0x3;1=0x3' > p0/schemata
  # cat p0/mode
  shareable
  # echo exclusive > p0/mode
  -sh: echo: write error: Invalid argument
  # cat info/last_cmd_status
  schemata overlaps

```
为确保不与另一个资源组重叠，必须更改默认资源组的 schemata，使新的资源组
能够变为独占。
```

  # echo 'L2:0=0xfc;1=0xfc' > schemata
  # echo exclusive > p0/mode
  # grep . p0/*
  p0/cpus:0
  p0/mode:exclusive
  p0/schemata:L2:0=03;1=03
  p0/size:L2:0=262144;1=262144

```
新创建的资源组不会与独占资源组重叠
```

  # mkdir p1
  # grep . p1/*
  p1/cpus:0
  p1/mode:shareable
  p1/schemata:L2:0=fc;1=fc
  p1/size:L2:0=786432;1=786432

```
```

  # cat info/L2/bit_usage
  0=SSSSSSEE;1=SSSSSSEE

```
```

  # echo 'L2:0=0x1;1=0x1' > p1/schemata
  -sh: echo: write error: Invalid argument
  # cat info/last_cmd_status
  overlaps with exclusive group

```
#### 缓存伪锁定示例


使用 CBM 0x3 锁定缓存 id 1 上的部分 L2 缓存。伪锁定区域暴露在
/dev/pseudo_lock/newlock，可以作为 mmap() 的参数提供给应用程序。
```

  # mount -t resctrl resctrl /sys/fs/resctrl/
  # cd /sys/fs/resctrl

```
确保有可用于伪锁定的位，因为只有未使用的位才能被伪锁定，待伪锁定的位需要
```

  # cat info/L2/bit_usage
  0=SSSSSSSS;1=SSSSSSSS
  # echo 'L2:1=0xfc' > schemata
  # cat info/L2/bit_usage
  0=SSSSSSSS;1=SSSSSS00

```
创建一个将与伪锁定区域关联的新资源组，指明它将用于伪锁定区域，并
```

  # mkdir newlock
  # echo pseudo-locksetup > newlock/mode
  # echo 'L2:1=0x3' > newlock/schemata

```
成功后，资源组的模式将变为 pseudo-locked，bit_usage 将反映伪锁定区域，
并且字符设备
```

  # cat newlock/mode
  pseudo-locked
  # cat info/L2/bit_usage
  0=SSSSSSSS;1=SSSSSSPP
  # ls -l /dev/pseudo_lock/newlock
  crw------- 1 root root 243, 0 Apr  3 05:01 /dev/pseudo_lock/newlock

```
```

  /*
  * Example code to access one page of pseudo-locked cache region
  * from user space.
  */
  #define _GNU_SOURCE
  #include <fcntl.h>
  #include <sched.h>
  #include <stdio.h>
  #include <stdlib.h>
  #include <unistd.h>
  #include <sys/mman.h>

  /*
  * It is required that the application runs with affinity to only
  * cores associated with the pseudo-locked region. Here the cpu
  * is hardcoded for convenience of example.
  */
  static int cpuid = 2;

  int main(int argc, char *argv[])
  {
    cpu_set_t cpuset;
    long page_size;
    void *mapping;
    int dev_fd;
    int ret;

    page_size = sysconf(_SC_PAGESIZE);

    CPU_ZERO(&cpuset);
    CPU_SET(cpuid, &cpuset);
    ret = sched_setaffinity(0, sizeof(cpuset), &cpuset);
    if (ret < 0) {
      perror("sched_setaffinity");
      exit(EXIT_FAILURE);
    }

    dev_fd = open("/dev/pseudo_lock/newlock", O_RDWR);
    if (dev_fd < 0) {
      perror("open");
      exit(EXIT_FAILURE);
    }

    mapping = mmap(0, page_size, PROT_READ | PROT_WRITE, MAP_SHARED,
            dev_fd, 0);
    if (mapping == MAP_FAILED) {
      perror("mmap");
      close(dev_fd);
      exit(EXIT_FAILURE);
    }

    /* Application interacts with pseudo-locked memory @mapping */

    ret = munmap(mapping, page_size);
    if (ret < 0) {
      perror("munmap");
      close(dev_fd);
      exit(EXIT_FAILURE);
    }

    close(dev_fd);
    exit(EXIT_SUCCESS);
  }

```
### 应用程序之间的锁定


resctrl 文件系统上的某些操作由对多个文件的读/写组成，必须是原子的。

例如，分配 L3 缓存的独占保留涉及：

  1. 从每个目录或每资源的 "bit_usage" 读取 cbmmask
  2. 在全局 CBM 位掩码中找到一个在任何目录 cbmmask 中都清晰的连续位集合
  3. 创建一个新目录
  4. 将在第 2 步中找到的位设置到新目录的 "schemata" 文件

如果两个应用程序尝试并发分配空间，它们最终可能分配到相同的位，从而使得
保留是共享的而非独占的。

为了协调 resctrlfs 上的原子操作并避免上述问题，建议使用以下锁定过程：

锁定基于 flock，它在 libc 中可用，也可以作为 shell 脚本命令使用

写锁：

 A) 对 /sys/fs/resctrl 执行 flock(LOCK_EX)
 B) 读/写目录结构。
 C) funlock

读锁：

 A) 对 /sys/fs/resctrl 执行 flock(LOCK_SH)
 B) 若成功，读取目录结构。
 C) funlock

```

  # Atomically read directory structure
  $ flock -s /sys/fs/resctrl/ find /sys/fs/resctrl

  # Read directory contents and create new subdirectory

  $ cat create-dir.sh
  find /sys/fs/resctrl/ > output.txt
  mask = function-of(output.txt)
  mkdir /sys/fs/resctrl/newres/
  echo mask > /sys/fs/resctrl/newres/schemata

  $ flock /sys/fs/resctrl/ ./create-dir.sh

```
```

  /*
  * Example code do take advisory locks
  * before accessing resctrl filesystem
  */
  #include <sys/file.h>
  #include <stdlib.h>

  void resctrl_take_shared_lock(int fd)
  {
    int ret;

    /* take shared lock on resctrl filesystem */
    ret = flock(fd, LOCK_SH);
    if (ret) {
      perror("flock");
      exit(-1);
    }
  }

  void resctrl_take_exclusive_lock(int fd)
  {
    int ret;

    /* release lock on resctrl filesystem */
    ret = flock(fd, LOCK_EX);
    if (ret) {
      perror("flock");
      exit(-1);
    }
  }

  void resctrl_release_lock(int fd)
  {
    int ret;

    /* take shared lock on resctrl filesystem */
    ret = flock(fd, LOCK_UN);
    if (ret) {
      perror("flock");
      exit(-1);
    }
  }

  void main(void)
  {
    int fd, ret;

    fd = open("/sys/fs/resctrl", O_DIRECTORY);
    if (fd == -1) {
      perror("open");
      exit(-1);
    }
    resctrl_take_shared_lock(fd);
    /* code to read directory contents */
    resctrl_release_lock(fd);

    resctrl_take_exclusive_lock(fd);
    /* code to read and write directory contents */
    resctrl_release_lock(fd);
  }

```
## RDT 监控与分配使用示例


### 读取监控数据


读取一个事件文件（例如：mon_data/mon_L3_00/llc_occupancy）会显示相应
MON 组或 CTRL_MON 组的 LLC 占用情况的当前快照。


### 示例 1（监控 CTRL_MON 组以及 CTRL_MON 组中的任务子集）


在一台双插槽机器（每插槽一个 L3 缓存）上，仅有 4 位
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir p0 p1
  # echo "L3:0=3;1=c" > /sys/fs/resctrl/p0/schemata
  # echo "L3:0=3;1=3" > /sys/fs/resctrl/p1/schemata
  # echo 5678 > p1/tasks
  # echo 5679 > p1/tasks

```
默认资源组未被修改，因此我们可以访问所有缓存的所有部分（其 schemata 文件
读为 "L3:0=f;1=f"）。

处于 "p0" 组控制下的任务只能从缓存 ID 0 的“较低” 50% 和缓存 ID 1 的
“较高” 50% 中分配。处于 "p1" 组中的任务在两个插槽上都使用缓存的“较低”
50%。

创建监控组，并为每个监控组分配一部分任务。
```

  # cd /sys/fs/resctrl/p1/mon_groups
  # mkdir m11 m12
  # echo 5678 > m11/tasks
  # echo 5679 > m12/tasks

```
获取数据（数据以字节显示）
```

  # cat m11/mon_data/mon_L3_00/llc_occupancy
  16234000
  # cat m11/mon_data/mon_L3_01/llc_occupancy
  14789000
  # cat m12/mon_data/mon_L3_00/llc_occupancy
  16789000

```
父 ctrl_mon 组显示聚合数据。
```

  # cat /sys/fs/resctrl/p1/mon_data/mon_l3_00/llc_occupancy
  31234000

```
### 示例 2（从任务创建起开始监控）


```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir p0 p1

```
一旦组被创建，就会为它分配一个 RMID，因此下面的 <cmd> 从其创建起就被监控。
```

  # echo $$ > /sys/fs/resctrl/p1/tasks
  # <cmd>

```
```

  # cat /sys/fs/resctrl/p1/mon_data/mon_l3_00/llc_occupancy
  31789000

```
### 示例 3（在没有 CAT 支持时或创建 CAT 组之前进行监控）


假设一个类似 HSW 的系统只有 CQM 而没有 CAT 支持。在这种情况下 resctrl 仍会
挂载，但无法创建 CTRL_MON 目录。但用户可以在根组内创建不同的 MON 组，从而
能够监控包括内核线程在内的所有任务。

这也可用于在能够将作业分配到不同的分配组之前，对其缓存占用大小进行剖析。
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir mon_groups/m01
  # mkdir mon_groups/m02

  # echo 3478 > /sys/fs/resctrl/mon_groups/m01/tasks
  # echo 2467 > /sys/fs/resctrl/mon_groups/m02/tasks

```
分别监控这些组，也可以获取每域数据。从下面的结果可以看出，这些任务主要
在域（插槽）0 上工作。
```

  # cat /sys/fs/resctrl/mon_groups/m01/mon_L3_00/llc_occupancy
  31234000
  # cat /sys/fs/resctrl/mon_groups/m01/mon_L3_01/llc_occupancy
  34555
  # cat /sys/fs/resctrl/mon_groups/m02/mon_L3_00/llc_occupancy
  31234000
  # cat /sys/fs/resctrl/mon_groups/m02/mon_L3_01/llc_occupancy
  32789


```
### 示例 4（监控实时任务）


一个单插槽系统，实时任务运行在核 4-7 上，非实时任务运行在其他 CPU 上。
我们希望监控这些核上实时线程的缓存占用情况。
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir p1

```
```

  # echo f0 > p1/cpus

```
```

  # cat /sys/fs/resctrl/p1/mon_data/mon_L3_00/llc_occupancy
  11234000


```
## 使用 mbm_assign_mode 的示例


a. 检查是否支持 MBM 计数器分配模式。
```

  # mount -t resctrl resctrl /sys/fs/resctrl/

  # cat /sys/fs/resctrl/info/L3_MON/mbm_assign_mode
  [mbm_event]
  default

```
"mbm_event" 模式被检测到并启用。

b. 检查支持多少个可分配计数器。
```

  # cat /sys/fs/resctrl/info/L3_MON/num_mbm_cntrs
  0=32;1=32

```
c. 检查每个域中有多少个可分配计数器可用于分配。
```

  # cat /sys/fs/resctrl/info/L3_MON/available_mbm_cntrs
  0=30;1=30

```
d. 列出默认组的分配状态。
```

  # cat /sys/fs/resctrl/mbm_L3_assignments
  mbm_total_bytes:0=e;1=e
  mbm_local_bytes:0=e;1=e

```
e. 解除域 0 上与 mbm_total_bytes 事件关联的计数器的分配。
```

  # echo "mbm_total_bytes:0=_" > /sys/fs/resctrl/mbm_L3_assignments
  # cat /sys/fs/resctrl/mbm_L3_assignments
  mbm_total_bytes:0=_;1=e
  mbm_local_bytes:0=e;1=e

```
f. 解除所有域上与 mbm_total_bytes 事件关联的计数器的分配。
```

  # echo "mbm_total_bytes:*=_" > /sys/fs/resctrl/mbm_L3_assignments
  # cat /sys/fs/resctrl/mbm_L3_assignment
  mbm_total_bytes:0=_;1=_
  mbm_local_bytes:0=e;1=e

```
g. 以独占模式为所有域分配与 mbm_total_bytes 事件关联的计数器。
```

  # echo "mbm_total_bytes:*=e" > /sys/fs/resctrl/mbm_L3_assignments
  # cat /sys/fs/resctrl/mbm_L3_assignments
  mbm_total_bytes:0=e;1=e
  mbm_local_bytes:0=e;1=e

```
h. 读取默认组的事件 mbm_total_bytes 和 mbm_local_bytes。分配后读取事件没有变化。
```

  # cat /sys/fs/resctrl/mon_data/mon_L3_00/mbm_total_bytes
  779247936
  # cat /sys/fs/resctrl/mon_data/mon_L3_01/mbm_total_bytes
  562324232
  # cat /sys/fs/resctrl/mon_data/mon_L3_00/mbm_local_bytes
  212122123
  # cat /sys/fs/resctrl/mon_data/mon_L3_01/mbm_local_bytes
  121212144

```
i. 检查事件配置。
```

  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_total_bytes/event_filter
  local_reads,remote_reads,local_non_temporal_writes,remote_non_temporal_writes,
  local_reads_slow_memory,remote_reads_slow_memory,dirty_victim_writes_all

  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_local_bytes/event_filter
  local_reads,local_non_temporal_writes,local_reads_slow_memory

```
j. 更改 mbm_local_bytes 的事件配置。
```

  # echo "local_reads, local_non_temporal_writes, local_reads_slow_memory, remote_reads" >
  /sys/fs/resctrl/info/L3_MON/event_configs/mbm_local_bytes/event_filter

  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_local_bytes/event_filter
  local_reads,local_non_temporal_writes,local_reads_slow_memory,remote_reads

```
k. 现在再次读取本地事件。第一次读取可能返回 "Unavailable" 状态。随后对
mbm_local_bytes 的读取将显示当前值。
```

  # cat /sys/fs/resctrl/mon_data/mon_L3_00/mbm_local_bytes
  Unavailable
  # cat /sys/fs/resctrl/mon_data/mon_L3_00/mbm_local_bytes
  2252323
  # cat /sys/fs/resctrl/mon_data/mon_L3_01/mbm_local_bytes
  Unavailable
  # cat /sys/fs/resctrl/mon_data/mon_L3_01/mbm_local_bytes
  1566565

```
l. 用户可以选择在需要时回到 'default' mbm_assign_mode。这可以使用以下命令完成。
注意，切换 mbm_assign_mode 可能会重置所有 resctrl 组的所有 MBM 计数器（以及
因此所有 MBM 事件）。
```

  # echo "default" > /sys/fs/resctrl/info/L3_MON/mbm_assign_mode
  # cat /sys/fs/resctrl/info/L3_MON/mbm_assign_mode
  mbm_event
  [default]

```
m. 卸载 resctrl 文件系统。
```

  # umount /sys/fs/resctrl/

```
## Intel RDT 勘误


### Intel MBM 计数器可能错误地报告系统内存带宽


Skylake 服务器的勘误 SKX99 和 Broadwell 服务器的勘误 BDF102。

问题：Intel 内存带宽监控（MBM）计数器根据逻辑核分配的 Resource Monitor ID
（RMID）跟踪指标。用于报告这些指标的 IA32_QM_CTR 寄存器（MSR 0xC8E）可能
对某些 RMID 值报告不正确的系统带宽。

影响：由于该勘误，系统内存带宽可能与报告值不匹配。

规避方法：MBM 总读数和本地读数根据以下校正因子表进行校正：

+---------------+---------------+---------------+-----------------+
|核数量	|RMID 数量	|RMID 阈值	|校正因子|
+---------------+---------------+---------------+-----------------+
|1		|8		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|2		|16		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|3		|24		|15		|0.969650	  |
+---------------+---------------+---------------+-----------------+
|4		|32		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|6		|48		|31		|0.969650	  |
+---------------+---------------+---------------+-----------------+
|7		|56		|47		|1.142857	  |
+---------------+---------------+---------------+-----------------+
|8		|64		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|9		|72		|63		|1.185115	  |
+---------------+---------------+---------------+-----------------+
|10		|80		|63		|1.066553	  |
+---------------+---------------+---------------+-----------------+
|11		|88		|79		|1.454545	  |
+---------------+---------------+---------------+-----------------+
|12		|96		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|13		|104		|95		|1.230769	  |
+---------------+---------------+---------------+-----------------+
|14		|112		|95		|1.142857	  |
+---------------+---------------+---------------+-----------------+
|15		|120		|95		|1.066667	  |
+---------------+---------------+---------------+-----------------+
|16		|128		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|17		|136		|127		|1.254863	  |
+---------------+---------------+---------------+-----------------+
|18		|144		|127		|1.185255	  |
+---------------+---------------+---------------+-----------------+
|19		|152		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|20		|160		|127		|1.066667	  |
+---------------+---------------+---------------+-----------------+
|21		|168		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|22		|176		|159		|1.454334	  |
+---------------+---------------+---------------+-----------------+
|23		|184		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|24		|192		|127		|0.969744	  |
+---------------+---------------+---------------+-----------------+
|25		|200		|191		|1.280246	  |
+---------------+---------------+---------------+-----------------+
|26		|208		|191		|1.230921	  |
+---------------+---------------+---------------+-----------------+
|27		|216		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|28		|224		|191		|1.143118	  |
+---------------+---------------+---------------+-----------------+

如果 rmid > rmid 阈值，MBM 总读数和本地读数应乘以校正因子。

参见：

1. Intel Xeon 处理器可扩展家族规格更新中的勘误 SKX99：
http://web.archive.org/web/20200716124958/https://www.intel.com/content/www/us/en/processors/xeon/scalable/xeon-scalable-spec-update.html

2. Intel Xeon E5-2600 v4 处理器产品家族规格更新中的勘误 BDF102：
http://web.archive.org/web/20191125200531/https://www.intel.com/content/dam/www/public/us/en/documents/specification-updates/xeon-e5-v4-spec-update.pdf

3. 第二代 Intel Xeon 可扩展处理器参考手册中 Intel Resource Director Technology（Intel RDT）的勘误：
https://software.intel.com/content/www/us/en/develop/articles/intel-resource-director-technology-rdt-reference-manual.html

以获取更多信息。
