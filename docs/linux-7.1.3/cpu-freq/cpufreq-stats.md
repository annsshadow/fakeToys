
## sysfs CPUFreq 统计通用描述


面向用户的信息


Author: Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>


   1. 简介
   2. 提供的统计（附示例）
   3. 配置 cpufreq-stats


## 1. 简介


cpufreq-stats 是一个为每个 CPU 提供 CPU 频率统计的驱动。这些统计以一组只读接口的形式
在 /sysfs 中提供。该接口（配置后）会为每个 CPU 出现在 /sysfs 中 cpufreq 下的一个独立
目录（<sysfs root>/devices/system/cpu/cpuX/cpufreq/stats/）。各种统计数据将构成该目录下的
只读文件。

该驱动被设计为独立于任何可能运行在你 CPU 上的特定 cpufreq_driver。因此，它可以与任何
cpufreq_driver 一起工作。


## 2. 提供的统计（附示例）


cpufreq stats 提供以下统计（下文详细解释）。

- time_in_state
- total_trans
- trans_table

所有统计数据都从 stats 驱动被插入（或 stats 被重置）的时刻起，到你读取某个特定统计的时刻
为止。显然，stats 驱动不会拥有任何关于 stats 驱动插入之前的频率切换的信息。

```

    <mysystem>:/sys/devices/system/cpu/cpu0/cpufreq/stats # ls -l
    total 0
    drwxr-xr-x  2 root root    0 May 14 16:06 .
    drwxr-xr-x  3 root root    0 May 14 15:58 ..
    --w-------  1 root root 4096 May 14 16:06 reset
    -r--r--r--  1 root root 4096 May 14 16:06 time_in_state
    -r--r--r--  1 root root 4096 May 14 16:06 total_trans
    -r--r--r--  1 root root 4096 May 14 16:06 trans_table

```
- **reset**

只写属性，可用于重置统计计数器。这对于在不同调速器（governor）下评估系统行为时很有用，
而无需重启。

- **time_in_state**

这给出该 CPU 在每个所支持频率上花费的时间量。cat 输出每一行将有一个 "<frequency> <time>"
对，表示该 CPU 在 <frequency> 上花费了 <time> 个用户时间单位。输出对每个所支持的频率会有一行。
这里的用户时间单位是 10mS（类似于 /proc 中导出的其它时间）。

```

    <mysystem>:/sys/devices/system/cpu/cpu0/cpufreq/stats # cat time_in_state
    3600000 2089
    3400000 136
    3200000 34
    3000000 67
    2800000 172488


```
- **total_trans**

这给出该 CPU 上频率切换的总次数。cat 输出将有一个单独的计数，即频率切换的总次数。

```

    <mysystem>:/sys/devices/system/cpu/cpu0/cpufreq/stats # cat total_trans
    20

```
- **trans_table**

这将给出关于所有 CPU 频率切换的细粒度信息。这里的 cat 输出是一个二维矩阵，其中条目
<i,j>（第 i 行，第 j 列）表示从 Freq_i 到 Freq_j 的切换次数。Freq_i 行和 Freq_j 列
遵循驱动最初向 cpufreq 核心提供频率表时的排序顺序，因此可以是已排序（升序或降序）或未排序。
这里的输出也包含每行每列的实际频率值以提高可读性。

如果切换表大于 PAGE_SIZE，读取它将返回 -EFBIG 错误。

```

    <mysystem>:/sys/devices/system/cpu/cpu0/cpufreq/stats # cat trans_table
    From  :    To
	    :   3600000   3400000   3200000   3000000   2800000
    3600000:         0         5         0         0         0
    3400000:         4         0         2         0         0
    3200000:         0         1         0         2         0
    3000000:         0         0         1         0         3
    2800000:         0         0         0         2         0

```
## 3. 配置 cpufreq-stats


```

	Config Main Menu
		Power management options (ACPI, APM)  --->
			CPU Frequency scaling  --->
				[*] CPU Frequency scaling
				[*]   CPU frequency translation statistics


```
要配置 cpufreq-stats，应启用 "CPU Frequency scaling"（CONFIG_CPU_FREQ）。

"CPU frequency translation statistics"（CONFIG_CPU_FREQ_STAT）提供包含 time_in_state、
total_trans 和 trans_table 的统计。

一旦启用此选项且你的 CPU 支持 cpufrequency，你就能在 /sysfs 中看到 CPU 频率统计。
