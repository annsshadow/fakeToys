## 内核同页合并（Kernel Samepage Merging，KSM


## 概述


KSM 是一个节省内存的去重特性，CONFIG_KSM=y 启用，在 2.6.32 中加Linux 内核。其实现参见 `mm/ksm.c`，以http://lwn.net/Articles/306704/ https://lwn.net/Articles/330589/

KSM 最初为配合 KVM 开发（当时称为 Kernel Shared Memory，内核共享内存），通过共享虚拟机之间的公共数据，使更多虚拟机能够装入物理内存。但任何会生成大量相同数据实例的应用都可以从中受益

KSM 守护进程 ksmd 会周期性地扫描那些已向其注册的用户内存区域，寻找内容相同的页，并用一个写保护的单一页来替换它们（如果某个进程稍后想要更新其内容，该页会被自动复制）。KSM 守护进程单次扫描的页数以及扫描之间的间隔通过 :ref:`sysfs 接口 <ksm_sysfs>` 配置

KSM 只合并匿名（私有）页，从不合pagecache（文件）页。KSM 合并的页最初被锁定在内核内存中，但现在可以像其他用户页一样被换出（但在换回时共享会被打破：ksmd 必须重新发现它们的同一性并再次合并）

## madvise 控制 KSM


KSM 只对应用程序通过 madvise(2) 建议为可能合并候选的地址空间区域起作
```
	int madvise(addr, length, MADV_MERGEABLE)
```
应用可以调用
```
	int madvise(addr, length, MADV_UNMERGEABLE)
```
来取消该建议并恢复未共享的页：于KSM 会取消在该范围内合并的所有内容。注意：这个取消合并的调用可能突然需要比可用量更多的内存——可能以 EAGAIN 失败，但更可能招Out-Of-Memory killer

如果 KSM 没有配置进运行中的内核，madvise MADV_MERGEABLE MADV_UNMERGEABLE 会简单地EINVAL 失败。如果运行中的内核以 CONFIG_KSM=y 构建，这些调用通常都会成功：即KSM 守护进程当前没有运行，MADV_MERGEABLE 仍会KSM 守护进程启动后的任意时刻注册该范围；即使该范围不可能包含任何 KSM 实际能够合并的页；即MADV_UNMERGEABLE 被应用到从未 MADV_MERGEABLE 的范围

如果一个内存区域必须被拆分为至少一个新MADV_MERGEABLE MADV_UNMERGEABLE 区域，当进程将超`vm.max_map_count` 时，madvise 可能返回 ENOMEM（参Documentation/admin-guide/sysctl/vm.rst）

与其madvise 调用一样，它们用于用户地址空间的已映射区域：如果指定范围包含未映射的间隙，它们会报ENOMEM（不过仍会处理其中已映射的区域），并且如果内部结构的可用内存不足，可能以 EAGAIN 失败

应用在使MADV_MERGEABLE 时应保持节制，将其限制在可能受益的区域。KSM 的扫描可能消耗大量处理能力：出于这个原因，某些部署会禁用 KSM


## KSM 守护进程 sysfs 接口


KSM 守护进程`/sys/kernel/mm/ksm/` 下的 sysfs 文件控制，所有用户可读，但只root 可写

pages_to_scan
        ksmd 进入睡眠之前要扫描多少页
        例如 `echo 100 > /sys/kernel/mm/ksm/pages_to_scan`

        如果 `advisor_mode` 已设置为 scan-time，则 pages_to_scan 值不能被改变

        默认00（出于演示目的而选择

sleep_millisecs
        ksmd 在下次扫描之前应睡眠多少毫秒
        例如 `echo 20 > /sys/kernel/mm/ksm/sleep_millisecs`

        默认0（出于演示目的而选择

merge_across_nodes
        指定来自不同 NUMA 节点的页是否可以合并
        当设置为 0 时，ksm 只合并物理上位于同一 NUMA 节点内存区域的页。这会降
        访问共享页的延迟。具有更多节点、且 NUMA 距离显著的系统，很可能从 0 
        较低延迟中受益。需要最小化内存使用的小型系统，很可能从 1（默认）的更
        共享中受益。在决定使用哪个设置之前，你可能希望比较你的系统在这两种设置下的表现。`merge_across_nodes` 设置只能在系统中没有 ksm 共享页时更改：先run 设为 2 以先取消合并页，然后在更`merge_across_nodes` 后设1，以根据新设置重新合并

        默认（跨节点合并，与早期版本相同

run
        - 设为 0 停止 ksmd 运行但保留已合并的页
        - 设为 1 运行 ksmd，例`echo 1 > /sys/kernel/mm/ksm/run`
        - 设为 2 停止 ksmd 并取消合并当前所有已合并的页，但
	  保留可合并区域以备下次运行

        默认（必须改1 才能激KSM，除CONFIG_SYSFS 被禁用）

use_zero_pages
        指定空页（即只包含零的已分配页）是否应被特殊处理。当设置1 时，
        空页会与内核零页合并，而非像通常那样彼此合并。根据工作负载的不同
        这可以在带有着色零页的架构上提升性能。启用此设置时应谨慎，因为它
        可能降低某些工作负载KSM 的性能，例如当候选合并页的校验和
        空页的校验和匹配时。该设置可以随时更改，它只对更改之后合并的页有效

        默认（与早期版本相同的正KSM 行为

max_page_sharing
        每个 KSM 页允许的最大共享数。这强制了一个去重上限，以避免涉及遍
        共享KSM 页的虚拟映射的虚拟内存操作产生高延迟。最小值为 2，因
        新创建的 KSM 页至少有两个共享者。该值越高，KSM 合并内存的速度越快
        去重因子也越高，但最坏情况下给定 KSM 页的虚拟映射遍历可能越慢。减
        这种遍历意味着在换出、压缩、NUMA 平衡与页迁移期间某些虚拟内存操作
        延迟会更高，进而降低这些虚拟内存操作调用方的响应性。不参与这些
        虚拟映射遍历VM 操作的其他任务的调度器延迟不受此参数影响，因为这
        遍历本身始终是调度友好的

stable_node_chains_prune_millisecs
        指定 KSM 多频繁检查命中去重上限的页的元数据中的过期信息
        较小的毫秒值会以更低延迟释KSM 元数据，但会ksmd 在扫描期
        使用更多 CPU。如果还没有任何 KSM 页命`max_page_sharing`，则
        是一个空操作（noop）

smart_scan
        历史KSM 在每次扫描中检查每个候选页。它没有考虑历史信息。启smart scan 后，先前未被去重的页会被跳过。这些页被跳过的频率取决于去重已经尝试并失败的次数。默认启用此优化。`pages_skipped` 指标显示了该设置的有效性

advisor_mode
        `advisor_mode` 选择当前的顾问（advisor）。支持两种模式：none scan-time。默认为 none。将 `advisor_mode` 设为 scan-time 可启用扫描时间顾问。关`advisor` 的小节详细解释了扫描时间顾问的工作原理

adivsor_max_cpu
        指定 ksmd 后台线程 CPU 使用百分比的上限。默认为 70

advisor_target_scan_time
        指定扫描所有候选页的目标扫描时间，以秒为单位。默认值为 200 秒

advisor_min_pages_to_scan
        指定扫描时间顾问`pages_to_scan` 参数的下限。默认为 500

adivsor_max_pages_to_scan
        指定扫描时间顾问`pages_to_scan` 参数的上限。默认为 30000

KSM MADV_MERGEABLE 的有效性显示在 `/sys/kernel/mm/ksm/` 中：

general_profit
        KSM 的有效性如何。计算方式解释如下
pages_scanned
        有多少页正在被扫描以用于 ksm
pages_shared
        正在使用多少个共享页
pages_sharing
        还有多少处站点在共享它们，即节省了多
pages_unshared
        多少页是唯一的，但被反复检查以进行合并
pages_volatile
        多少页变化太快而无法放入树
pages_skipped
        “smart页扫描算法跳过了多少
full_scans
        所有可合并区域已被扫描了多少次
stable_node_chains
        命中 `max_page_sharing` 限制KSM 页数
stable_node_dups
        重复KSM 页数
ksm_zero_pages
        当去重时KSM 映射、且仍映射到进程中的零页数量

`use_zero_pages` 曾被启用时，`pages_sharing` + `ksm_zero_pages` 的和表示 KSM 实际节省的页数量。如`use_zero_pages` 从未被启用，`ksm_zero_pages` 0

`pages_sharing` `pages_shared` 的高比率表示良好的共享，`pages_unshared` `pages_sharing` 的高比率表示浪费了精力。`pages_volatile` 包含几种不同类型的活动，但其中占较高比例也表明对 madvise MADV_MERGEABLE 的使用不当

最大可能的 `pages_sharing/pages_shared` 比率`max_page_sharing` 可调参数限制。要提高该比率，必须相应地增`max_page_sharing`

## 监控 KSM 收益


KSM 可以通过合并相同的页来节省内存，但也可能消耗额外的内存，因为它需要生成若rmap_item 来保存每个被扫描页的简rmap 信息。这些页中有些可能被合并，但有些在多次检查后可能仍无法合并，这些就是被消耗的无效内存

1) 如何判断 KSM 是在系统范围内节省内存还是消耗内
```
	general_profit =~ ksm_saved_pages * sizeof(page) - (all_rmap_items) *
			  sizeof(rmap_item);

   其中 ksm_saved_pages 等于系统``pages_sharing`` +
   ``ksm_zero_pages`` 之和，all_rmap_items 可以很容易地通过
   ``pages_sharing``、``pages_shared``、``pages_unshared`` 
   ``pages_volatile`` 相加得到
```
2) 单个进程内部KSM 收益可以通过类似方式得到
```
	process_profit =~ ksm_saved_pages * sizeof(page) -
			  ksm_rmap_items * sizeof(rmap_item).

   其中 ksm_saved_pages 等于 ``ksm_merging_pages`` ``ksm_zero_pages`` 之和
   二者都显示``/proc/<pid>/ksm_stat`` 目录下，ksm_rmap_items 也显示在
   ``/proc/<pid>/ksm_stat`` 中。进程收益也作为 ksm_process_profit 显示
   ``/proc/<pid>/ksm_stat`` 中
```
从应用的角度看，`ksm_rmap_items` `ksm_merging_pages` 的高比率意味着一个糟糕的 madvise 应用策略，因此开发者或管理员必须重新考虑如何更改 madvise 策略。给出一个供参考的例子：页的大小通常4K，rmap_item 的大小在 32 CPU 架构上为 32B，在 64 CPU 架构上为 64B。因此，如果 `ksm_rmap_items/ksm_merging_pages` 比率64 CPU 上超64，或32 CPU 上超128，那么应用的 madvise 策略应当被放弃，因为 KSM 收益近似为零或为负

## 监控 KSM 事件


/proc/vmstat 中有一些计数器可用于监KSM 事件。KSM 可能有助于节省内存，但它是一种权衡，可能承受 KSM COW 或在换入副本时的延迟。这些事件可以帮助用户评估是否以及如何使KSM。例如，如果 cow_ksm 增长过快，用户可以缩madvise(, , MADV_MERGEABLE) 的范围

cow_ksm
        每当一KSM 页触发写时复制（COW）时递增
        当用户尝试写入一KSM 页时，我们必须制作一份副本

ksm_swpin_copy
        每当一KSM 页在换入时被复制时递增
        注意 KSM 页在换入时可能被复制，因do_swap_page()
        无法进行重建一个跨 anon_vma KSM 页所需的全部加锁

## 顾问（Advisor


KSM 候选页的数量是动态的。经常可以观察到，在应用启动期间需要处理更多候选页。如果没有顾问，`pages_to_scan` 参数需要按照最大候选页数量来设定大小。扫描时间顾问可以根据需求改`pages_to_scan` 参数

可以启用顾问，这KSM 就能自动适应待扫描候选页数量的变化。实现了两种顾问：none scan-time。使none 时不启用任何顾问。默认为 none

扫描时间顾问根据观察到的扫描时间改变 `pages_to_scan` 参数。该参数 `pages_to_scan` 的可能取值受 `advisor_max_cpu` 参数限制。此外还`advisor_target_scan_time` 参数。该参数设定扫描所KSM 候选页的目标时间。`advisor_target_scan_time` 参数决定扫描时间顾问扫描候选页的激进程度。较低的值使扫描时间顾问扫描得更激进。这是扫描时间顾问配置中最重要的参数

初始值与最大值可以通过 `advisor_min_pages_to_scan` `advisor_max_pages_to_scan` 更改。默认值对大多数工作负载与用例都已足够

`pages_to_scan` 参数在一次扫描完成后被重新计算


--
Izik Eidus,
Hugh Dickins, 2009 骞?11 鏈?17 鏃。
