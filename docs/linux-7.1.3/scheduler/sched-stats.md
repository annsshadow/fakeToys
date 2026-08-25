## 调度器统计（Scheduler Statistics


schedstats 的第 17 版移除了 'lb_imbalance' 字段，因为它
不再有意义，转而添加了更具相关性的字段，即
'lb_imbalance_load'銆?lb_imbalance_util'銆?lb_imbalance_task' 鍜。
'lb_imbalance_misfit'。domain 字段从此版本起打
相应调度域的名称

schedstats 的第 16 版更改了 'enum cpu_idle_type' 内部
定义顺序，从而改变了 show_schedstat() 
[CPU_MAX_IDLE_TYPES] 列的顺序。特别是 CPU_IDLE
__CPU_NOT_IDLE 的位置互换了。数组大小不变

schedstats 的第 15 版删除了部分 sched_yield 的计数器
yld_exp_empty、yld_act_empty yld_both_empty。除此之外，
它与14 版完全相同。详细信息见

	https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/scheduler/sched-stats.txt?id=1e1dbb259c79b

schedstats 的第 14 版包含对 sched_domains 的支持，该特性在
2.6.20 进入主线内核，尽管它与第 12 版的统计相同
（第 12 版存在于 2.6.13-2.6.19 内核中，13 版从未发布）
 某些计数器更适合按运行队列（runqueue）统计，另一些则
 按域统计。请注意，域（及其相关信息）仅在
 使用 CONFIG_SMP 的机器上才相关且可用

schedstat 14 版中，列出的每个 cpu 至少有一
域统计，并且很可能不止一
域。在此实现中域没有特定名称，
 编号最高的域通常负责协调整机上所
 cpu 的均衡，domain0 是最聚焦的域
 有时仅在成对cpu 之间进行均衡。目
 没有架构需要超过三级域。域统计中的
 第一个字段是一个位图，指示哪些 cpu 受该域影
 。详细信息见

	https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/sched-stats.txt?id=b762f3ffb797c

schedstat 文档从第 10 版起维护，第 11 12 版未更新
10 版的详细信息

	https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/sched-stats.txt?id=1da177e4c3f4

这些字段都是计数器，只会递增。使用这些字段的
程序需要先进行一次基线观测，然后计算
每次后续观测时计数器的变化。有一perl 脚本
可以对其中许多字段执行此操作，见

    http://eaglet.pdxhosts.com/rick/linux/schedstat/

请注意，任何此类脚本都必然与版本相关，因为更改版本的
主要原因就是输出格式的变动。对于希
编写自己脚本的人，字段在此描述

### CPU 统计

cpu<N> 1 2 3 4 5 6 7 8 9

第一个字段是 sched_yield() 统计

     1) 调用 sched_yield() 的次

接下来三个是 schedule() 统计

     2) 该字段是 O(1) 调度器中遗留的数组过期计数，出于 ABI 兼容性予以保留，但其值始终为零
     3) 调用 schedule() 的次
     4) schedule() 使处理器进入空闲状态的次数

接下来两个是 try_to_wake_up() 统计

     5) 调用 try_to_wake_up() 的次
     6) 为唤醒本cpu 而调try_to_wake_up() 的次

接下来三个是描述调度延迟的统计：

     7) 本处理器上各任务运行所耗费的总时间（纳秒
     8) 本处理器上各任务等待运行所耗费的总时间（纳秒
     9) 在本 cpu 上运行的时片（timeslice）数


### 域统

对每个所描述cpu，每个域都会生成一行这样的输出。（请注意，如果
 未定CONFIG_SMP，则**不会**使用任何域，这些
 不会出现在输出中。）

domain<N> <name> <cpumask> 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45

<name> 字段打印调度域的名称，仅schedstat 版本 >= 17 时支持
在之前的版本中，<cpumask> 是第一
字段

<cpumask> 字段是一个位掩码，指示该域在哪些 cpu 上运
銆。

接下来的 33 个字段是 sched_balance_rq() 的各种统计，
空闲类型（busy、idle newly idle）分组：

    1) 在此域中，cpu 繁忙时调sched_balance_rq() 的次
    2) 在此域中，cpu 繁忙sched_balance_rq() 检查后发现负载无需均衡的次
    3) 在此域中，cpu 繁忙sched_balance_rq() 尝试迁移一个或多个任务但失败的次数
    4) cpu 绻佸繖鏃讹紝姝ゅ煙鍐呰礋杞界殑鎬讳笉鍧囪　閲。
    5) cpu 繁忙时，此域内利用率的总不均衡
    6) cpu 繁忙时，此域内任务总数的总不均衡
    7) cpu 繁忙时，此域内由不匹配（misfit）任务导致的总不均衡
    8) 在此域中，cpu 繁忙时调detach_task() 的次
    9) 在此域中，cpu 繁忙时即使目标任cache-hot 仍调detach_task() 的次
    10) 在此域中，cpu 繁忙sched_balance_rq() 被调用但未找到更繁忙队列的次
    11) 在此域中，cpu 繁忙时发现更繁忙的队列但未发现更繁忙的分组（group）的次数

    12) 在此域中，cpu 空闲时调sched_balance_rq() 的次
    13) 在此域中，cpu 空闲sched_balance_rq() 检查后发现负载无需均衡的次
    14) 在此域中，cpu 空闲sched_balance_rq() 尝试迁移一个或多个任务但失败的次数
    15) cpu 绌洪棽鏃讹紝姝ゅ煙鍐呰礋杞界殑鎬讳笉鍧囪　閲。
    16) cpu 空闲时，此域内利用率的总不均衡
    17) cpu 空闲时，此域内任务总数的总不均衡
    18) cpu 空闲时，此域内由不匹配（misfit）任务导致的总不均衡
    19) 在此域中，cpu 空闲时调detach_task() 的次
    20) 在此域中，cpu 空闲时即使目标任cache-hot 仍调detach_task() 的次
    21) 在此域中，cpu 空闲sched_balance_rq() 被调用但未找到更繁忙队列的次
    22) 在此域中，cpu 空闲时发现更繁忙的队列但未发现更繁忙的分组（group）的次数

    23) 在此域中，cpu 即将进入空闲时调sched_balance_rq() 的次
    24) 在此域中，cpu 即将进入空闲sched_balance_rq() 检查后发现负载无需均衡的次
    25) 在此域中，cpu 即将进入空闲sched_balance_rq() 尝试迁移一个或多个任务但失败的次数
    26) cpu 鍗冲皢杩涘叆绌洪棽鏃讹紝姝ゅ煙鍐呰礋杞界殑鎬讳笉鍧囪　閲。
    27) cpu 即将进入空闲时，此域内利用率的总不均衡
    28) cpu 即将进入空闲时，此域内任务总数的总不均衡
    29) cpu 即将进入空闲时，此域内由不匹配（misfit）任务导致的总不均衡
    30) 在此域中，新进入空闲（newly idle）状态时调用 detach_task() 的次
    31) 在此域中，cpu 即将进入空闲时即使目标任cache-hot 仍调detach_task() 的次
    32) 在此域中，cpu 即将进入空闲sched_balance_rq() 被调用但未找到更繁忙队列的次
    33) 在此域中，cpu 即将进入空闲时发现更繁忙的队列但未发现更繁忙的分组（group）的次数

   接下来的三项active_load_balance() 的统计：

    34) 调用 active_load_balance() 的次
    35) active_load_balance() 尝试迁移任务但失败的次数
    36) active_load_balance() 成功迁移任务的次

   接下来的三项sched_balance_exec() 的统计：

    37) sbe_cnt 未被使用
    38) sbe_balanced 未被使用
    39) sbe_pushed 未被使用

   接下来的三项sched_balance_fork() 的统计：

    40) sbf_cnt 未被使用
    41) sbf_balanced 未被使用
    42) sbf_pushed 未被使用

   接下来的三项try_to_wake_up() 的统计：

    43) 在此域中，try_to_wake_up() 唤醒了上次运行于本域中另一 cpu 的任务的次数
    44) 在此域中，try_to_wake_up() 将任务迁移到唤醒 cpu 的次数（因其自身 cpu cache-cold
    45) 在此域中，try_to_wake_up() 启动被动均衡（passive balancing）的次数

### /proc/<pid>/schedstat

schedstats 还新增了一/proc/<pid>/schedstat 文件，以包含
进程级别的相同信息。该文件
有三个字段，对应于该进程

     1) cpu 上花费的时间（纳秒）
     2) 在运行队列上等待的时间（纳秒
     3) 在本 cpu 上运行的时片（timeslice）数

可以很容易地编写一个程序来利用这些额外字段，以
报告特定进程或一组进程在
调度器策略下的运行情况。此类程序的一个简单版本见
銆。


    http://eaglet.pdxhosts.com/rick/linux/schedstat/v12/latency.c
