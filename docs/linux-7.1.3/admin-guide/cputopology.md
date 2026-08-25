## CPU 拓扑信息如何通过 sysfs 导出


CPU 拓扑信息通过 sysfs 导出。条目（属性）类似于某些架构的 /proc/cpuinfo 输出。它位于 /sys/devices/system/cpu/cpuX/topology/。请参ABI 文件Documentation/ABI/stable/sysfs-devices-system-cpu
与架构无关的代码 drivers/base/topology.c 导出这些属性。但是，die、cluster、book drawer 层级相关sysfs 文件，只有在架构按如下所述提供了相关宏时才会被创建
要支持该特性，架构必须定义以下部分宏：
```

	#define topology_physical_package_id(cpu)
	#define topology_die_id(cpu)
	#define topology_cluster_id(cpu)
	#define topology_core_id(cpu)
	#define topology_book_id(cpu)
	#define topology_drawer_id(cpu)
	#define topology_sibling_cpumask(cpu)
	#define topology_core_cpumask(cpu)
	#define topology_cluster_cpumask(cpu)
	#define topology_die_cpumask(cpu)
	#define topology_book_cpumask(cpu)
	#define topology_drawer_cpumask(cpu)

```
`**_id` 宏的类型int`**_cpumask` 宏的类型`(const) struct cpumask *`。后者对应相应的 `**_siblings` sysfs
属性（topology_sibling_cpumask() 除外，它对应 thread_siblings）
为在所有架构上保持一致，include/linux/topology.h 为上述任何未include/asm-XXX/topology.h 定义的宏提供默认定义
1) topology_physical_package_id: -1
2) topology_die_id: -1
3) topology_cluster_id: -1
4) topology_core_id: 0
5) topology_book_id: -1
6) topology_drawer_id: -1
7) topology_sibling_cpumask: 仅给定的 CPU
8) topology_core_cpumask: 仅给定的 CPU
9) topology_cluster_cpumask: 仅给定的 CPU
10) topology_die_cpumask: 仅给定的 CPU
11) topology_book_cpumask:  仅给定的 CPU
12) topology_drawer_cpumask: 仅给定的 CPU

此外，CPU 拓扑信息/sys/devices/system/cpu 下提供，并包含以下文件。输出的内部来源
在括号（“[]”）中
    =========== ==========================================================
    kernel_max: 内核配置允许的最CPU 索引		[NR_CPUS-1]

    offline:	因已被热插拔（HOTPLUGGED）关闭或超出内核配置
		（上面的 kernel_max）允许的 CPU 数量限制而不在线CPU		[~cpu_online_mask + cpus >= NR_CPUS]

    online:	在线且正在被调度CPU [cpu_online_mask]

    possible:	已分配资源、若存在则可被带入在线的 CPU。[cpu_possible_mask]

    present:	已被识别为系统中存在CPU。[cpu_present_mask]
    =========== ==========================================================

上述输出的格式兼cpulist_parse() [参见 <linux/cpumask.h>]。下面给出一些示例
在此示例中，系统中有 64 CPU，但 cpu 32-63 超出了内核最大值，该最大值由 NR_CPUS
配置选项（为 32）限制为 0..31。另请注意，CPU 2 4-31 不在线，但可以被
```

     kernel_max: 31
        offline: 2,4-31,32-63
         online: 0-1,3
       possible: 0-31
        present: 0-31

```
在此示例中，NR_CPUS 配置选项128，但内核possible_cpus=144 启动。系统中4 CPU，cpu2 被手动离线（且是唯一可以被带入在线的 CPU```

     kernel_max: 127
        offline: 2,4-127,128-143
         online: 0-1,3
       possible: 0-127
        present: 0-3

```
参见 Documentation/core-api/cpu_hotplug.rst 了解 possible_cpus=NUM 内核启动参数以及
关于各种 cpumask 的更多信息