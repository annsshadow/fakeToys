## 内存热插拔


本文档描述了 Linux 对内存热插拔（hot(un)plug）的通用支持，重点介绍 System RAM，包括 ZONE_MOVABLE 支持。


## 简介


内存热插拔允许在运行时增加或减少机器可用的物理内存大小。最简单的情况下，它由在运行时物理插入或拔出 DIMM，并与操作系统协调完成。

内存热插拔有多种用途：

- 可在运行时调整机器可用的物理内存，向上或向下改变内存容量。这种动态内存调整，有时称为“按需容量”（capacity on demand），常用于虚拟机和逻辑分区。

- 在不停机的情况下更换硬件，例如 DIMM 或整个 NUMA 节点。一个例子是更换故障内存模块。

- 通过物理拔出内存模块，或通过将（部分）内存模块从 Linux 中逻辑拔出，来降低能耗。

此外，Linux 中基本的内存热插拔基础设施如今也被用来将持久内存、其他性能有差异的内存以及保留内存区域，作为普通系统 RAM 暴露给 Linux。

Linux 仅在选择的部分 64 位架构上支持内存热插拔，例如 x86_64、arm64、ppc64 和 s390x。

### 内存热插拔粒度


Linux 中的内存热插拔使用 SPARSEMEM 内存模型，该模型将物理内存地址空间划分为大小相同的块：内存段（memory section）。内存段的大小与架构相关。例如，x86_64 使用 128 MiB，ppc64 使用 16 MiB。

内存段被组合成称为“内存块”（memory block）的块。内存块的大小与架构相关，并对应于可以热插拔的最小粒度。除非架构另有指定，内存块的默认大小与内存段大小相同。

所有内存块具有相同的大小。

### 内存热插拔的阶段


内存热插拔由两个阶段组成：

(1) 将内存加入 Linux
(2) 将内存块上线（online）

在第一阶段，分配并初始化元数据，例如内存映射（“memmap”）以及用于直接映射的页表，并创建内存块；后者还会为管理新建的内存块创建 sysfs 文件。

在第二阶段，新增的内存被暴露给页分配器。此阶段之后，该内存在系统的内存统计（如空闲内存和总内存）中可见。

### 内存热拔出的阶段


内存热拔出由两个阶段组成：

(1) 将内存块下线（offline）
(2) 将内存从 Linux 中移除

在第一阶段，内存再次被页分配器“隐藏”，例如通过将繁忙内存迁移到其他内存位置、并从页分配器中移除所有相关的空闲页。此阶段之后，该内存在系统的内存统计中不再可见。

在第二阶段，内存块被移除，元数据被释放。

## 内存热插拔通知


Linux 收到内存热插拔事件通知、从而能够开始添加热插拔内存的方式有多种。本说明仅限于支持 ACPI 的系统；特定于其他固件接口或虚拟机的机制不在描述范围内。

### ACPI 通知


支持 ACPI 的平台（如 x86_64）可以通过 ACPI 支持内存热插拔通知。

一般而言，支持内存热插拔的固件会定义一个内存类对象 HID “PNP0C80”。当收到新增内存设备的热插拔通知时，ACPI 驱动会将该内存热插拔到 Linux。

如果固件支持 NUMA 节点的热插拔，它会定义一个对象 _HID “ACPI0004”、“PNP0A05” 或 “PNP0A06”。当收到热插拔事件通知时，所有被分配的内存设备都会被 ACPI 驱动加入 Linux。

类似地，Linux 也可以通过 ACPI 收到热拔出某个内存设备或 NUMA 节点的请求通知。ACPI 驱动会尝试将所有相关内存块下线，若成功，则从 Linux 中热拔出该内存。

### 手动探测


在某些架构上，固件可能无法就内存热插拔事件通知操作系统。在这种情况下，必须从用户空间手动探测该内存。

```

	/sys/devices/system/memory/probe

```
只能探测完整的内存块。逐个内存块通过以下方式探测：
```

	% echo addr > /sys/devices/system/memory/probe

```
这将创建覆盖范围 [addr, addr + memory_block_size) 的一个内存块。


  使用探测接口是不推荐的，因为它很容易导致内核崩溃，因为 Linux 无法校验用户输入；该接口将来可能会被移除。

## 内存块的上线与下线


在创建内存块之后，必须指示 Linux 实际使用这块内存：该内存块必须被“上线”。

在移除一个内存块之前，Linux 必须停止使用属于该内存块的任何内存部分：该内存块必须被“下线”。

可以配置 Linux 内核自动将新增的内存块上线，并且驱动在尝试热拔出内存时会自动触发内存块的下线。内存块只有在下线成功后才能被移除，并且驱动在尝试热拔出内存时可能会触发内存块的下线。

### 手动将内存块上线


如果未启用内存块的自动上线，则必须由用户空间手动触发内存块的上线。通常，udev 规则被用于用户空间中自动化此任务。

```

	% echo online > /sys/devices/system/memory/memoryXXX/state

```
```

	% echo 1 > /sys/devices/system/memory/memoryXXX/online

```
内核会根据配置的 `online_policy` 自动选择目标区域（zone）。

也可以显式请求将一个离线内存块关联到以下区域：
```

	% echo online_movable > /sys/devices/system/memory/memoryXXX/state

```
```

	% echo online_kernel > /sys/devices/system/memory/memoryXXX/state

```
无论如何，如果上线成功，该内存块的状态会变为“online”。如果失败，内存块的状态保持不变，上述命令也会失败。

### 自动将内存块上线


可以配置内核来尝试自动上线新增的内存块。如果此特性被禁用，内存块将保持离线，直到从用户空间显式上线。

```

	% cat /sys/devices/system/memory/auto_online_blocks

```
可以通过写入 `online`、`online_kernel` 或以下值来启用自动上线：
```

	% echo online > /sys/devices/system/memory/auto_online_blocks

```
与手动上线类似，使用 `online` 时，内核会根据配置的 `online_policy` 自动选择目标区域。

修改自动上线行为只会影响之后所有新增的内存块。


  在极端情况下，自动上线可能失败。内核不会重试。注意，在默认配置下，自动上线不应当失败。


  ppc64 上的 DLPAR 会忽略 `offline` 设置，仍然会将新增的内存块上线；如果上线失败，内存块会再次被移除。

### 内存块下线


在当前实现中，Linux 的内存下线会尝试将受影响内存块上的所有可移动页迁移出去。由于大多数内核分配（如页表）是不可移动的，页迁移可能会失败，从而阻碍内存下线的成功。

让内存块由 ZONE_MOVABLE 管理，可以显著提高内存下线的可靠性；尽管如此，在某些极端情况下内存下线仍可能失败。

此外，内存下线可能会重试很长时间（甚至永远），直到被用户中止。

```

	% echo offline > /sys/devices/system/memory/memoryXXX/state

```
```

	% echo 0 > /sys/devices/system/memory/memoryXXX/online

```
如果下线成功，该内存块的状态会变为“offline”。如果失败，内存块的状态保持不变，上述命令也会失败。
```

	bash: echo: write error: Device or resource busy

```
```

	bash: echo: write error: Invalid argument

```
### 观察内存块的状态


可以通过读取以下文件观察内存块的状态（online/offline/going-offline）：
```

	% cat /sys/devices/system/memory/memoryXXX/state

```
```

	% cat /sys/devices/system/memory/memoryXXX/online

```
```

	% cat /sys/devices/system/memory/memoryXXX/valid_zones

```
## 配置内存热插拔


系统管理员有多种方式配置内存热插拔并与内存块交互，尤其是将内存块上线。

### 通过 Sysfs 配置内存热插拔


```

	/sys/devices/system/memory/

```
目前定义了以下文件：

====================== =========================================================
`auto_online_blocks` read-write: 设置或获取新内存块的默认状态；配置自动上线。

		       默认值取决于
		       CONFIG_MHP_DEFAULT_ONLINE_TYPE 内核配置选项。

		       详见内存块的 `state` 属性。

`block_size_bytes`   read-only: 内存块的大小（以字节为单位）。
`probe`	       write-only: 通过提供物理起始地址，从用户空间手动添加（探测）所选的内存块。

		       可用性取决于 CONFIG_ARCH_MEMORY_PROBE 内核配置选项。
`uevent`	       read-write: 设备子系统的通用 udev 文件。
`crash_hotplug`      read-only: 当由于内存热拔插导致系统内存映射发生变化时，
		       如果该文件包含 '1'，表示内核自行更新 kdump 捕获内核的内存映射
		       （通过 elfcorehdr 及其他相关的 kexec 段）；如果包含 '0'，
		       则表示必须由用户空间更新 kdump 捕获内核的内存映射。

		       可用性取决于 CONFIG_MEMORY_HOTPLUG 内核配置选项。
====================== =========================================================


  当启用 CONFIG_MEMORY_FAILURE 内核配置选项时，还有两个额外的文件 `hard_offline_page` 和 `soft_offline_page` 可用，用于触发页面的 hwpoison（硬件中毒），例如用于测试目的。注意，该功能与内存热插拔或内存块的实际下线并无真正关联。

### 通过 Sysfs 配置内存块


每个内存块都表示为一个内存块设备，可以被上线或下线。所有内存块的设备信息都位于 sysfs 中。每个存在的内存块都列在
```

	/sys/devices/system/memory/memoryXXX

```
之下，其中 XXX 是内存块 id；其位数可变。

一个“存在”的内存块表示该范围内存在某些内存；然而，一个内存块可能跨越内存空洞（memory hole）。跨越内存空洞的内存块无法被下线。

例如，假设内存块大小为 1 GiB。起始地址为
```

	(0x100000000 / 1Gib = 4)

```
的设备覆盖地址范围 [0x100000000 ... 0x140000000)

目前定义了以下文件：

=================== ============================================================
`online`	    read-write: 用于触发上线/下线、以及观察内存块状态的简化接口。
		    上线时，区域由内核自动选择。
`phys_device`	    read-only: 仅在 s390x 上使用的遗留接口，用于暴露所覆盖的存储增量。
`phys_index`	    read-only: 内存块 id（XXX）。
`removable`	    read-only: 遗留接口，曾用于指示一个内存块是否可被下线。如今，
		    内核当且仅当支持内存下线时才返回 `1`。
`state`	    read-write: 用于触发上线/下线、以及观察内存块状态的高级接口。

		    写入时支持 `online`、`offline`、`online_kernel` 和
		    `online_movable`。

		    `online_movable` 指定上线到 ZONE_MOVABLE。
		    `online_kernel` 指定上线到内存块的默认内核区域，例如 ZONE_NORMAL。
                    `online` 让内核自动选择区域。

		    读取时可能返回 `online`、`offline` 和 `going-offline`。
`uevent`	    read-write: 设备的通用 uevent 文件。
`valid_zones`     read-only: 当一个块处于上线状态时，显示其所属的区域；当一个块处于离线状态时，
		    显示该块在上线时将由哪个区域管理。

		    对于上线内存块，可能返回 `DMA`、`DMA32`、`Normal`、
		    `Movable` 和 `none`。`none` 表示一个内存块提供的由多个区域管理、
		    或跨越多个节点的内存；此类内存块无法被下线。`Movable` 表示 ZONE_MOVABLE。
		    其他值表示某个内核区域。

		    对于离线内存块，第一列显示内核在“现在就上线该内存块而不进一步指定区域”时会选择的区域。

		    可用性取决于 CONFIG_MEMORY_HOTREMOVE 内核配置选项。
=================== ============================================================


  如果启用了 CONFIG_NUMA 内核配置选项，则 memoryXXX/ 目录也可以通过位于
  `/sys/devices/system/node/node*` 目录中的符号链接来访问。

```

	/sys/devices/system/node/node0/memory9 -> ../../memory/memory9

  A backlink will also be created::

	/sys/devices/system/memory/memory9/node0 -> ../../node/node0

```
### 命令行参数


一些命令行参数会影响内存热插拔的处理。以下命令行参数与之相关：

======================== =======================================================
`memhp_default_state`	 通过实质上设置 `/sys/devices/system/memory/auto_online_blocks` 来配置自动上线。
`movable_node`	 在使用 `contig-zones` 上线策略时，配置内核中的自动区域选择。当
		 设置时，内核在上线一个内存块时会默认使用 ZONE_MOVABLE，
		 除非其他区域能保持连续。
======================== =======================================================

关于这些命令行参数更通用的描述，请参阅 Documentation/admin-guide/kernel-parameters.txt。

### 模块参数


`memory_hotplug` 子系统现在提供了一个专用的命名空间用于模块参数，以取代额外的命令行参数或 sysfs 文件。可以通过命令行设置模块参数，方式为在参数前加
```

	memory_hotplug.memmap_on_memory=1

```
```

	/sys/module/memory_hotplug/parameters/

```
目前定义了以下模块参数：

================================ ===============================================
`memmap_on_memory`		 read-write: 从新增的内存块自身分配 memmap 所需的内存。即使启用，
				 实际支持仍取决于各种其他系统属性，并且只应被视为
				 对该行为是否合意的一个提示。

				 虽然从内存块自身分配 memmap 能降低内存热插拔失败的可能性，
				 并且在任何情况下都将 memmap 保留在同一 NUMA 节点上，
				 但它会以某种方式碎片化物理内存，使得在热插拔内存上无法
				 形成更大粒度的巨页。

				 当值为 "force" 时，由于 memmap 大小的限制，可能导致内存浪费。
				 例如，若某个内存块的 memmap 需要 1 MiB，但页块（pageblock）大小为
				 2 MiB，则会有 1 MiB 的热插拔内存被浪费。注意，仍有一些情况下无法
				 强制启用该特性：例如，当 memmap 小于单个页面，或当架构并非在
				 所有配置中都支持强制模式时。

`online_policy`		 read-write: 设置在上线内存块而未指定目标区域时，
				 用于自动区域选择的基本策略。在该参数加入之前，
				 `contig-zones` 一直是内核默认值。在配置了上线策略并且
				 内存已经上线之后，不应再更改该策略。

				 当设置为 `contig-zones` 时，内核会尝试保持区域连续。如果
				 一个内存块与多个区域相交或不属于任何区域，则行为取决于
				 `movable_node` 内核命令行参数：若已设置则默认 ZONE_MOVABLE，
				 若未设置则默认使用适用的内核区域（通常是 ZONE_NORMAL）。

				 当设置为 `auto-movable` 时，内核会尽量根据配置和内存设备细节，
				 将内存块上线到 ZONE_MOVABLE。采用此策略，可以在以后最终热插拔
				 大量内存、且仍希望尽可能可靠地热拔出时，避免区域失衡，这在
				 虚拟化环境中非常可取。该策略会忽略 `movable_node` 内核命令行参数，
				 并且在需要它的环境中（例如带有可热拔出节点的裸金属，其中热插拔内存
				 可能通过固件提供的早期内存映射在启动初期就暴露给系统，而不是在
				 启动后期才被检测到、添加并上线，如 virtio-mem 或某些实现
				 模拟 DIMM 的管理程序所做的那样）并不真正适用。作为一个例子，
				 一个热插拔的 DIMM 要么完全上线到 ZONE_MOVABLE，要么完全上线到
				 ZONE_NORMAL，而不会是混合的。作为另一个例子，属于某个 virtio-mem
				 设备的尽可能多的内存块会被上线到 ZONE_MOVABLE，并对只能整体一起
				 热拔出的内存块单元做特殊处理。*此策略并不能防止那些对 ZONE_MOVABLE
				 有问题的配置，也不会在内存块上线之后动态地改变其区域。*

`auto_movable_ratio`		 read-write: 为 `auto-movable` 上线策略设置最大的
				 MOVABLE:KERNEL 内存比例（以 % 计）。该比例是仅适用于
				 跨所有 NUMA 节点的整个系统，还是也适用于每个 NUMA 节点，
				 取决于 `auto_movable_numa_aware` 配置。

				 所有记账基于区域中存在的页面，并结合每个内存设备的记账。专用于
				 CMA 分配器的内存被记为 MOVABLE，尽管它位于某个内核区域之上。
				 可能的比例取决于实际的工作负载。内核默认值为 "301" %，例如，
				 允许向一个 8 GiB 的 VM 热插拔 24 GiB，并在许多配置下自动将
				 所有热插拔内存上线到 ZONE_MOVABLE。额外的 1% 用于处理某些页面
				 不存在的情况，例如由于某些固件分配。

				 注意，一个内存设备提供的 ZONE_NORMAL 内存，并不会让另一个
				 内存设备获得更多 ZONE_MOVABLE 内存。作为一个例子，将一个热插拔
				 DIMM 的内存上线到 ZONE_NORMAL，并不会让另一个热插拔 DIMM 自动
				 上线到 ZONE_MOVABLE。相反，由 virtio-mem 设备热插拔、并上线到
				 ZONE_NORMAL 的内存，会允许在**同一个** virtio-mem 设备内获得
				 更多的 ZONE_MOVABLE 内存。

`auto_movable_numa_aware`	 read-write: 配置是否将 `auto_movable_ratio` 在 `auto-movable`
				 上线策略中也适用于每个 NUMA 节点（除了跨所有 NUMA 节点的
				 整个系统之外）。内核默认值为 "Y"。

				 在处理应当完全可热拔出的 NUMA 节点时，若可能则自动将内存
				 完全上线到 ZONE_MOVABLE，禁用 NUMA 感知会很有帮助。

				 参数可用性取决于 CONFIG_NUMA。
================================ ===============================================

## ZONE_MOVABLE


ZONE_MOVABLE 是一种用于提高内存下线可靠性的重要机制。此外，让系统 RAM 由 ZONE_MOVABLE 而非某个内核区域来管理，可以增加可能的透明巨页以及动态分配巨页的数量。

大多数内核分配都是不可移动的。重要的例子包括内存映射（通常占内存的 1/64）、页表，以及 kmalloc()。此类分配只能由内核区域提供。

大多数用户空间页面，例如匿名内存和页缓存页面，是可移动的。此类分配可以由 ZONE_MOVABLE 和内核区域提供。

只有可移动分配才由 ZONE_MOVABLE 提供，这使得不可移动分配被限制在内核区域。如果没有 ZONE_MOVABLE，则完全无法保证一个内存块能否成功下线。

### 区域失衡


由 ZONE_MOVABLE 管理的系统 RAM 过多称为区域失衡，它可能损害系统或降低性能。例如，内核可能因为用于不可移动分配的自由内存耗尽而崩溃，尽管 ZONE_MOVABLE 中仍有大量空闲内存。

通常，最高 3:1 甚至 4:1 的 MOVABLE:KERNEL 比例是没问题的。而 63:1 的比例由于内存映射的开销，肯定是行不通的。

实际安全的区域比例取决于工作负载。极端情况，例如对页面的过度长期固定（pinning），可能根本无法应付 ZONE_MOVABLE。


  作为内核区域一部分的 CMA 内存，本质上表现得像 ZONE_MOVABLE 中的内存，并适用类似的考量，尤其是在将 CMA 与 ZONE_MOVABLE 结合使用时。

### ZONE_MOVABLE 容量规划考量


我们通常预期可用的系统 RAM 中很大一部分实际上会被用户空间消耗，无论是直接地，还是通过页缓存间接地。正常情况下，在分配此类页面时使用 ZONE_MOVABLE 完全没有问题。

考虑到这一点，让系统 RAM 的很大部分由 ZONE_MOVABLE 管理是合理的。然而，在使用 ZONE_MOVABLE 时，尤其是在微调区域比例时，有一些事情需要考虑：

- 存在大量离线内存块。即使是离线内存块，也会在直接映射中消耗用于元数据和页表的内存；不过，拥有大量离线内存块并非典型情况。

- 不支持气球内存迁移的内存气球（ballooning）与 ZONE_MOVABLE 不兼容。只有部分实现，例如 virtio-balloon 和 pseries CMM，完全支持气球内存迁移。

  此外，CONFIG_BALLOON_MIGRATION 内核配置选项可能被禁用。在这种情况下，气球膨胀只会执行不可移动分配，并悄悄地造成区域失衡，通常由来自管理程序的膨胀请求触发。

- 当架构不支持巨页迁移和/或 `movable_gigantic_pages` sysctl 为 false 时，巨页（gigantic page）是不可移动的。关于该 sysctl 的更多信息，请参阅 Documentation/admin-guide/sysctl/vm.rst。

- 当架构不支持巨页迁移时，巨页是不可移动的，从而导致与巨页类似的问题。

- 页表是不可移动的。过度的交换、映射极大的文件或 ZONE_DEVICE 内存都可能成为问题，尽管这仅在极端情况下才真正相关。当我们管理大量已被换出或来自文件/持久内存/……的用户空间内存时，一旦用户空间访问了这些内存，我们仍然需要大量页表来管理它。

- 在某些 DAX 配置中，设备内存的内存映射将从内核区域分配。

- KASAN 可能有显著的内存开销，例如消耗总系统内存大小的 1/8 作为（不可移动的）跟踪元数据。

- 页面的长期固定。依赖长期固定（尤其是 RDMA 和 vfio/mdev）的技术，与 ZONE_MOVABLE 以及由此带来的内存下线在根本上就存在问题。被固定的页面不能驻留在 ZONE_MOVABLE 中，否则会使这些页面变为不可移动。因此，它们在被固定的同时必须被迁移出该区域。即使 ZONE_MOVABLE 中有大量空闲内存，固定一个页面也可能失败。

  此外，使用 ZONE_MOVABLE 可能因页迁移的开销而使页面固定更加昂贵。

默认情况下，启动时配置的所有内存都由内核区域管理，不使用 ZONE_MOVABLE。

要启用 ZONE_MOVABLE 以包含启动时存在的内存，并控制可移动区域与内核区域之间的比例，有两个命令行选项：`kernelcore=` 和 `movablecore=`。关于它们的描述，请参阅 Documentation/admin-guide/kernel-parameters.rst。

### 内存下线与 ZONE_MOVABLE


即使使用了 ZONE_MOVABLE，仍有一些极端情况下下线某个内存块可能失败：

- 带有内存空洞的内存块；这适用于启动时存在的内存块，也可适用于通过 XEN 气球和 Hyper-V 气球热插拔的内存块。

- 单个内存块内的混合 NUMA 节点和混合区域会阻止内存下线；这仅适用于启动时存在的内存块。

- 被系统阻止下线的特殊内存块。例子包括 arm64 上启动期间可用的任何内存，或 s390x 上跨越 crashkernel 区域的内存块；这通常仅适用于启动时存在的内存块。

- 与 CMA 区域重叠的内存块无法被下线，这仅适用于启动时存在的内存块。

- 作用于相同物理内存区域的并发活动，例如分配巨页，可能导致临时下线失败。

- 当管理员将 `movable_gigantic_pages` sysctl 设为 true 时，允许在 ZONE_MOVABLE 中分配巨页。这只允许分配可迁移的巨页；然而，如果在下线时没有符合条件的目标巨页，下线操作将失败。

  利用 `movable_gigantic_pages` 的用户，应当权衡 ZONE_MOVABLE 在提高巨页分配可靠性方面的价值，与热拔出可靠性潜在下降之间的利弊。

- 在溶解巨页时内存不足，尤其是在启用 HugeTLB Vmemmap 优化（HVO）时。

  下线代码或许能够迁移巨页的内容，但可能无法溶解源巨页，因为它无法为 vmemmap 分配（不可移动的）页面，因为系统在内核区域中可能已没有剩余的空闲内存。

  依赖内存下线在可移动区域成功的用户，应当仔细考虑该功能带来的内存节省，是否值得在某些情况下可能因此无法下线内存的风险。

此外，当在迁移页面时遇到内存不足，或当仍然遇到 ZONE_MOVABLE 中永久不可移动的页面时（-> BUG），内存下线会一直重试，直到最终成功。

当从用户空间触发下线时，可以通过发送信号来终止下线操作。基于超时的下线可以很容易地
```

	% timeout $TIMEOUT offline_block | failure_handling

```
