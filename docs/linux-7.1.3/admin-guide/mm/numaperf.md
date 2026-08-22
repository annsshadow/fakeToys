## NUMA 内存性能


## NUMA 局部

某些平台可能在一个计算节点上附带多种类型的内存。这些不同的内存范围可能共享
一些特性，例如 CPU 缓存一致性，但可能具有不同的性能。例如，不同的介质类型和
总线会影响带宽和延迟
系统通过将每种内存类型根据局部性和性能特性分组到不同的域节点"来支持这异构内存。某些内存可能与 CPU 共享同一个节点，而另一些则作为纯内存节点提供虽然纯内存节点不提供 CPU，但相对于其它节点，它们可能仍然对一个或多个计算节点
是局部的。下图显示了两个计算节点的一个此类示例：
```

 +------------------+     +------------------+
 | Compute Node 0   +-----+ Compute Node 1   |
 | Local Node0 Mem  |     | Local Node1 Mem  |
 +--------+---------+     +--------+---------+
          |                        |
 +--------+---------+     +--------+---------+
 | Slower Node2 Mem |     | Slower Node3 Mem |
 +------------------+     +--------+---------+

```
"内存发起是包含一个或多个能够发起内存请求的设备（CPU 或独立的内存
IO 设备）的节点内存目标"是包含一个或多个可从内存发起者访问的物理地址范围节点
当存在多个内存发起者时，它们在访问给定内存目标时可能并非都具有相同的性能每个发起目标对可以被组织到不同的分级访问类中以表示这种关系。到给定目标性能最高的发起者被视为该目标的局部发起者之一，并被赋予最高访问类。任何给目标可能有一个或多个局部发起者，而任何给定发起者可能有多个局部内存目标
为了帮助应用程序将内存目标与其发起者匹配，内核提供相互之间的符号链接。以示例列出```

	# symlinks -v /sys/devices/system/node/nodeX/access0/targets/
	relative: /sys/devices/system/node/nodeX/access0/targets/nodeY -> ../../nodeY

	# symlinks -v /sys/devices/system/node/nodeY/access0/initiators/
	relative: /sys/devices/system/node/nodeY/access0/initiators/nodeX -> ../../nodeX

```
一个内存发起者在同一访问类中可能有多个内存目标。给定类中的目标内存的发起表明这些节点的访问特性相对于其它被链接的发起者节点具有相同的性能。不过，发起访问类中的每个目标彼此之间不一定表现相同
访问1"用于区分作为 CPU 因而适合通用任务调度的发起者，GPU NIC IO 发起者。与访问0 不同，只考虑包含 CPU 的节点
## NUMA 性能


应用程序可能希望根据其节点的性能特性来考虑希望从哪个节点分配内存。如果系提供这些属性，内核会通过在节点的 sysfs 层次结构下追加属性目录，将它们导出在
```

	/sys/devices/system/node/nodeY/access0/initiators/

```
之下。这些属性仅在从链接在此访问的发起者下的节点访问时适用
内核为局部发起者提供的性能特```

	# tree -P "read*|write*" /sys/devices/system/node/nodeY/access0/initiators/
	/sys/devices/system/node/nodeY/access0/initiators/
	|-- read_bandwidth
	|-- read_latency
	|-- write_bandwidth
	`-- write_latency

```
带宽属性以 MiB/秒为单位提供
延迟属性以纳秒为单位提供
此处报告的值对应于该平台的额定延迟和带宽
访问1 采用相同的形式，但只包含 CPU 到内存活动的值
## NUMA 缓存


系统内存可以构建为具有各种性能特性的元素层次结构，以便提供由较小、较高性能内存缓存的较大较慢性能内存的地址空间。内存发起者所知的系统物理地址由层次结中的最后一级内存提供。与此同时，系统使用性能更高的内存透明地缓存对逐级变慢级别的访问
术语"远端内存"用于表示层次结构中的最后一级内存。每一更高的缓存级别提供更
高性能的发起者访问，术语"近端内存"表示系统提供的最快缓存
此编号不同于 CPU 缓存，其中缓存级别（例如：L1、L2、L3）使CPU 侧视角，每一
更高等级性能更低。相比之下，内存缓存级别以最后一级内存为中心，因此编号更高的
缓存级别对应于更靠近 CPU、离远端内存更远的内存
内存侧缓存不能被软件直接寻址。当软件访问系统地址时，如果近端内存缓存中存在该
地址，系统会从中返回它。如果不存在，系统会访问下一级内存，直到在该缓存级别命中或到达远端内存
应用程序不需要了解缓存属性即可使用系统。软件可以可选地查询内存缓存属性，以从
这种设置中最大化性能。如果系统为内核提供了发现此信息的方式，例如通过 ACPI HMAT
（异构内存属性表），内核会将这些属性追加到 NUMA 节点内存目标
当内核首次向节点注册内存缓存时，内核
```

	/sys/devices/system/node/nodeX/memory_side_cache/

```
如果该目录不存在，则系统要么不提供内存侧缓存，要么该信息对内核不可访问
每一级缓存的属性在其缓```

	/sys/devices/system/node/nodeX/memory_side_cache/indexA/
	/sys/devices/system/node/nodeX/memory_side_cache/indexB/
	/sys/devices/system/node/nodeX/memory_side_cache/indexC/

```
之下提供。每个缓存级别的目录提供其属性。例如，以下显示了一个缓存级别及其可用的
```

	# tree /sys/devices/system/node/node0/memory_side_cache/
	/sys/devices/system/node/node0/memory_side_cache/
	|-- index1
	|   |-- indexing
	|   |-- line_size
	|   |-- size
	|   `-- write_policy

```
"indexing" 如果是直接映射缓存则0，对于任何其他基于索引的多路组相联则为非零
"line_size" 是未命中时从下一级缓存访问的字节数
"size" 是此缓存级别提供的字节数
"write_policy" 对于写回0，对于写通缓存为非零
## 另请参阅


[^1^] https://www.uefi.org/sites/default/files/resources/ACPI_6_2.pdf
- 绗?5.2.27 鑺?