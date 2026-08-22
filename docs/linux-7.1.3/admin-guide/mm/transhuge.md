## 透明大页支持


## 目标


处理大型内存工作集、对性能要求苛刻的计算应用，已经libhugetlbfs 之上、进而在
hugetlbfs 之上运行。透明大页支持（THP，Transparent HugePage）是另一种使用大页作虚拟内存后端的方式，它支持页大小的自动提升（promotion）和降级（demotion），并且
没有 hugetlbfs 的那些缺点
目前 THP 仅适用于匿名内存映射以tmpfs/shmem。但未来可以扩展到其它文件系统
   在下面的示例中，我们假设基本页大小为 4K，大页大小为 2M，尽管实际数值可能因
   CPU 架构而异
应用程序运行得更快有两个原因。第一个因素几乎完全无关紧要，而且它没有重大价值，
因为它也有缺点：在页错误（page fault）时需要更大的 clear-page / copy-page，这
是一个潜在的负面影响。第一个因素由用户态每触及 2M 虚拟区域只产生一次页错误组成
（从而将进入/退出内核的频率降低512 倍）。这只对内存映射生命周期内首次访内存时才有意义。第二个影响持久且重要得多的因素，会在应用程序整个运行期间影对内存的所有后续访问。第二个因素由两个部分组成：

1) TLB 缺失（miss）会运行得更快（尤其是在使用嵌套页表（nested pagetables）的
   虚拟化中，但几乎在裸机没有虚拟化时也总是如此
2) 单个 TLB 表项将映射大得多的虚拟内存量，进而减TLB 缺失的次数。在虚拟化和
   嵌套页表下，只有KVM Linux 客户机都使用大页时，TLB 才能映射更大的尺寸，
   但只要两者之一使用大页，就已经会有显著的加速，原因仅仅在于 TLB 缺失会运   得更快
现代内核支持“multi-size THP”（mTHP，多尺寸 THP），它引入了以大于基本页、但小于
传统 PMD 尺寸（如上所述）的块来分配内存的能力，以 2 的幂次页数为增量。mTHP 可以
作为匿名内存的后端（例如 16K2K4K 等）。这THP 仍然PTE 映射的，但在许多
情况下仍能提供与上面概述类似的收益：页错误显著减少（减少系数为例46 等）但延迟尖峰不那么明显，因为每页的大小不像 PMD 尺寸的变体那么大，并且每次页错误需要清零的内存也更少。某些架构还采用 TLB 压缩机制，当一PTE 在虚拟和物理上连并适当对齐时，把更多表项挤进去。在这种情况下，TLB 缺失会更少发生
THP 可以在系统范围内启用，也可以限制到某些任务，甚至任务地址空间内的某些内存范围除非完全禁用 THP，否则会有一`khugepaged` 守护进程扫描内存，并将一系列基本折叠（collapse）为 PMD 尺寸的大页
THP 的行为通过 sysfs <thp_sysfs> 接口以及使用 madvise(2) prctl(2) 系统调用控制
透明大页支持hugetlbfs 的预留（reservation）方式相比，通过允许所有未使用的内被用作缓存或其它可移动（甚至不可移动）实体，最大化了空闲内存的用处。它不需要预来防止从用户态察觉到的大页分配失败。它允许在大页上使用分页以及所有其它高VM
特性。应用程序利用它无需任何修改
不过，应用程序可以进一步优化以利用此特性，就像以前它们被优化以避免每次 malloc(4k)
都涌入大mmap 系统调用一样。优化用户态远非强制性的，而且 khugepaged 已经可以长期存在的页分配兜底，即使是那些对大页无感知、却处理大量内存的应用程序也是如此
在某些情况下，当大页在系统范围内启用时，应用程序可能最终分配更多内存资源。一个应可能 mmap 一个很大的区域但只触及其中1 字节，这种情况下可能会分配一2M 页而非
4K 页，白白浪费。这就是为什么可以系统范围内禁用大页、并只在 MADV_HUGEPAGE madvise
区域内拥有它们的原因
嵌入式系统应只在 madvise 区域内启用大页，以消除浪费任何宝贵内存字节的风险，并且只
运行得更快
从大页中获益良多、且不会因使用大页而面临丢失内存风险的应用程序，应在它们的关键
mmap 区域上使madvise(MADV_HUGEPAGE)

## sysfs


### 全局 THP 控制


用于匿名内存的透明大页支持可以被禁用（主要出于调试目的），或只MADV_HUGEPAGE
区域内启用（以避免消耗更多内存资源的风险），或启```

	echo always >/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/enabled
	echo madvise >/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/enabled
	echo never >/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/enabled

```
其中 <size> 是所针对的大页大小，其可用大小因系统而异
          透明巨页全局。这是因``madvise(..., MADV_COLLAPSE)`` 忽略这些设置，并
          无条件地将范围折叠为 PMD 尺寸的大页```

	echo always >/sys/kernel/mm/transparent_hugepage/hugepages-2048kB/enabled

```
或者，也可以指定给定的大页大小
```

	echo inherit >/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/enabled

```
```

	echo inherit >/sys/kernel/mm/transparent_hugepage/hugepages-2048kB/enabled

```
用于“inherit”的顶层设置可以通过发出以下命令来设```

	echo always >/sys/kernel/mm/transparent_hugepage/enabled
	echo madvise >/sys/kernel/mm/transparent_hugepage/enabled
	echo never >/sys/kernel/mm/transparent_hugepage/enabled

```
默认情况下，PMD 尺寸的大enabled="inherit"，所有其它大页大enabled="never"如果启用多个大页大小，内核将为给定的分配选择最合适的已启用大小
也可能在 VM 中限制碎片整理（defrag）努力，以生成匿名大页，以防它们不能立即空闲
用于 madvise 区域，或者永远不尝试整理内存、直接回退到常规页，除非大页立即可用显然，如果我们花CPU 时间来整理内存，我们会期望由于之后使用大页而非常规页而获更多收益。这并不总是有保证的，但在分配是面向 MADV_HUGEPAGE 区域的情况下更有可能```

	echo always >/sys/kernel/mm/transparent_hugepage/defrag
	echo defer >/sys/kernel/mm/transparent_hugepage/defrag
	echo defer+madvise >/sys/kernel/mm/transparent_hugepage/defrag
	echo madvise >/sys/kernel/mm/transparent_hugepage/defrag
	echo never >/sys/kernel/mm/transparent_hugepage/defrag

```
always
	表示请求 THP 的应用程序将在分配失败时停滞，并直接回收页、压缩内存，
	以努力立即分配一THP。这对于THP 使用中获益匪浅、并愿意延迟虚拟	启动来利用它们的虚拟机可能是理想的
defer
	表示应用程序将在后台唤醒 kswapd 来回收页、唤kcompactd 来压缩内存，
	以便 THP 在不久的将来可用。之后由 khugepaged 负责稍后安装 THP 页
defer+madvise
	将像 `always` 一样进入直接回收和压缩，但仅针对使用了
	madvise(MADV_HUGEPAGE) 的区域；所有其它区域将在后台唤kswapd 回收
	页、唤kcompactd 压缩内存，以THP 在不久的将来可用
madvise
	将像 `always` 一样进入直接回收，但仅针对使用	madvise(MADV_HUGEPAGE) 的区域。这是默认行为
never
	应该不言自明。注意，即使到处都指定了此模式，``madvise(..., MADV_COLLAPSE)``
	仍可能导致获得透明大页
默认情况下，内核在匿名映射的读页错误时尝试使用巨大的、PMD 可映射的零页。可禁用巨大零页
```

	echo 0 >/sys/kernel/mm/transparent_hugepage/use_zero_page
	echo 1 >/sys/kernel/mm/transparent_hugepage/use_zero_page

```
某些用户空间（例如测试程序，或优化过的内存分配库）可能想知道（以字节为单位的大小
```

	cat /sys/kernel/mm/transparent_hugepage/hpage_pmd_size

```
故障和折叠时的所THP 都会被加_deferred_list，因此如果它们被视为“未充分利用（underused），将在内存压力下被拆分。如果一THP 中零填充页的数量超过
max_ptes_none（见下文），THP 就是未充分利用的。可以通过shrink_underused 写入
0 来禁用此行为，写1 来启用它
```

	echo 0 > /sys/kernel/mm/transparent_hugepage/shrink_underused
	echo 1 > /sys/kernel/mm/transparent_hugepage/shrink_underused

```
PMD 尺寸THP 被启用时（per-size anon 控制或顶层控制之一被设为“always”或
“madvise”），khugepaged 会自动启动；PMD 尺寸THP 被禁用时（per-size anon 控制
和顶层控制都为“never”），它会自动关闭
### 杩涚▼绾?THP 鎺у埗


一个进程可以使`PR_SET_THP_DISABLE` `PR_GET_THP_DISABLE` 这对 prctl(2) 调用控制自己THP 行为。使`PR_SET_THP_DISABLE` 设置THP 行为会跨 fork(2) execve(2) 继承。这些调```

	prctl(PR_SET_THP_DISABLE, 1, 0, 0, 0):
		This will disable THPs completely for the process, irrespective
		of global THP controls or madvise(..., MADV_COLLAPSE) being used.

	prctl(PR_SET_THP_DISABLE, 1, PR_THP_DISABLE_EXCEPT_ADVISED, 0, 0):
		This will disable THPs for the process except when the usage of THPs is
		advised. Consequently, THPs will only be used when:
		- Global THP controls are set to "always" or "madvise" and
		  madvise(..., MADV_HUGEPAGE) or madvise(..., MADV_COLLAPSE) is used.
		- Global THP controls are set to "never" and madvise(..., MADV_COLLAPSE)
		  is used. This is the same behavior as if THPs would not be disabled on
		  a process level.
		Note that MADV_COLLAPSE is currently always rejected if
		madvise(..., MADV_NOHUGEPAGE) is set on an area.

	prctl(PR_SET_THP_DISABLE, 0, 0, 0, 0):
		This will re-enable THPs for the process, as if they were never disabled.
		Whether THPs will actually be used depends on global THP controls and
		madvise() calls.

	prctl(PR_GET_THP_DISABLE, 0, 0, 0, 0):
		This returns a value whose bits indicate how THP-disable is configured:
		Bits
		 1 0  Value  Description
		|0|0|   0    No THP-disable behaviour specified.
		|0|1|   1    THP is entirely disabled for this process.
		|1|1|   3    THP-except-advised mode is set for this process.

```
### Khugepaged 控制


   khugepaged 目前只搜索折叠为 PMD 尺寸 THP 的机会，不会尝试折叠为其THP
   尺寸
khugepaged 通常以较低频率运行，因此虽然可能不想在页错误期间同步调用碎片整理
算法，但khugepaged 中至少调用一次碎片整理是值得的。不过也可以通过写入 0 禁用
khugepaged 中的碎片整理，或写入 1 启用
```

	echo 0 >/sys/kernel/mm/transparent_hugepage/khugepaged/defrag
	echo 1 >/sys/kernel/mm/transparent_hugepage/khugepaged/defrag

```
你也可以控制 khugepaged 每次扫描应扫描多少页
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/pages_to_scan

```
以及 khugepaged 在每轮之间等待多少毫秒（```

	/sys/kernel/mm/transparent_hugepage/khugepaged/scan_sleep_millisecs

```
以及如果有一个大页，khugepaged 等待多少毫秒
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/alloc_sleep_millisecs

```
khugepaged 的进度可以从已折叠页的数量看出（注意，这个计数器可能不是已折叠页数量精确计数，因为“已折叠”可能有多种含义1) PTE 映射PMD 映射替换，或 (2) 所4K
物理页被一2M 大页替换。每种情况可能独立发生，也可能一起发生，取决于内存类型和
发生的失败。因此，这个值应大致解释为进度的标志，/proc/vmstat 中的计数```

	/sys/kernel/mm/transparent_hugepage/khugepaged/pages_collapsed

```
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/full_scans

```
`max_ptes_none` 指定在折叠一组页时可以分配多少个额外的（尚未映射的）小页
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/max_ptes_none

```
较高的值会导致为程序使用额外内存。较低的值会导致获得thp 性能更少max_ptes_none 的值浪费的 cpu 时间极少，可以忽略它
`max_ptes_swap` 指定可以从以下位置引入多少页
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/max_ptes_swap

```
较高的值可能导致过度的交换 IO 并浪费内存。较低的值可能阻THP 被折叠，导致折叠
THP 的页更少，以及更低的内存访问性能
`max_ptes_shared` 指定可以跨多个进程共享多少页。如THP 的任何一```

	/sys/kernel/mm/transparent_hugepage/khugepaged/max_ptes_shared

```
较高的值可能会增加某些工作负载的内存占用
## 启动参数


你可以通过将参`transparent_hugepage=always` `transparent_hugepage=madvise` `transparent_hugepage=never` 传给内核命令行，来更改顶层“enabled”控制的 sysfs 启动
时间默认值
另外，每个受支持的匿THP 大小都可以通过传`thp_anon=<size>[KMG],<size>[KMG]:<state>;<size>[KMG]-<size>[KMG]:<state>` 来控制，
其中 `<size>` THP 大小（必须是 PAGE_SIZE 2 的幂，且为受支持的匿THP），
`<state>` `always`、`madvise`、`never` `inherit` 之一
例如，以下将16K2K4K THP 设为 `always`，把 128K12K 设为 `inherit`256K 设为 `madvise`，把 1MM
```

	thp_anon=16K-64K:always;128K,512K:inherit;256K:madvise;1M-2M:never

```
`thp_anon=` 可以多次指定，以按需配置所THP 大小。如果至少指定了一`thp_anon=`则命令行上未显式配置的任何匿THP 大小都隐式设`never`
`transparent_hugepage` 设置只影响全局开关。如果未指定 `thp_anon`，PMD_ORDER THP 默认`inherit`。但是，如果用户提供了有效的 `thp_anon` 设置，PMD_ORDER THP 策略将被
覆盖。如PMD_ORDER 的策略未在某个有效的 `thp_anon` 中定义，其策略将默认`never`
`transparent_hugepage` 类似，你可以使用内核参数
`transparent_hugepage_shmem=<policy>` 控制内部 shmem 挂载的大页分配策略，其中
`<policy>` shmem 的七个有效策略之一（`always`、`within_size`、`advise`、`never``deny` `force`）
`transparent_hugepage_shmem` 类似，你可以使用内核参数
`transparent_hugepage_tmpfs=<policy>` 控制 tmpfs 挂载的默认大页分配策略，其中
`<policy>` tmpfs 的四个有效策略之一（`always`、`within_size`、`advise`、`never`）tmpfs 挂载的默认策略是 `never`
此外，Kconfig 选项可用于在构建时设shmem 的默认大页策（`CONFIG_TRANSPARENT_HUGEPAGE_SHMEM_HUGE_*`）和 tmpfs 的默认大页策（`CONFIG_TRANSPARENT_HUGEPAGE_TMPFS_HUGE_*`）。更多细节请参阅 Kconfig 帮助
`thp_anon` 控制每个受支持的匿名 THP 大小一样，`thp_shmem` 控制每个受支持的 shmem
THP 大小。`thp_shmem` `thp_anon` 格式相同，但也支`within_size` 策略
`thp_shmem=` 可以多次指定，以按需配置所THP 大小。如果至少指定了一`thp_shmem=`则命令行上未显式配置的任shmem THP 大小都隐式设`never`
`transparent_hugepage_shmem` 设置只影响全局开关。如果未指定 `thp_shmem`PMD_ORDER 大页将默认为 `inherit`。但是，如果用户提供了有效的 `thp_shmem` 设置PMD_ORDER 大页策略将被覆盖。如PMD_ORDER 的策略未在某个有效的 `thp_shmem` 定义，其策略将默认为 `never`
## tmpfs/shmem 中的大页


传统上，tmpfs 只支持单一的大页大小（“PMD”）。如今，它也像匿名内存一样支持更小的
大小，通常被称为“multi-size THP”（mTHP，多尺寸 THP）。任何大小的大页在内核中通常
表示为“large folios”（folio）
虽然对用于内shmem 挂载使用的大页大小有精细控制（见下文），但普通的 tmpfs 挂载
会利用所有可用的大页大小，而无需对确切大小进行控制，表现得更像其它文件系统
### tmpfs 挂载


tmpfs 挂载THP 分配策略可以使用挂载选项：`huge=` 来调整。它可以有以下取值：

always
    每次需要新页时都尝试分配大页；
    总是先尝PMD 尺寸的大页，如果 PMD 尺寸的大页分配失败，则回退到更小尺寸的
    大页
never
    不分配大页。注意，即使到处都指定了此模式，`madvise(..., MADV_COLLAPSE)` 仍可    导致获得透明大页
within_size
    只有当大页将完全位于 i_size 内时才分配；
    总是先尝PMD 尺寸的大页，如果 PMD 尺寸的大页分配失败，则回退到更小尺寸的
    大页    也尊madvise() 提示
advise
    只有在使madvise() 请求时才分配大页
请记住，内核可能使用所有可用大小的大页，并且无法像内部 tmpfs 挂载那样进行精细控制
过去默认策略`never`，但现在可以使用内核参数 `transparent_hugepage_tmpfs=<policy>`
来调整
`mount -o remount,huge= /mountpoint` 在挂载后工作正常：重新挂`huge=never` 根本不会
尝试拆分大页，只是停止分配更多大页
除了上面列出的策略外，当设为以下值时，sysfs 旋钮
/sys/kernel/mm/transparent_hugepage/shmem_enabled 会影tmpfs 挂载的分配策略：

deny
    用于紧急情况，强制从所有挂载关huge 选项
force
    对所有挂载强制开huge 选项——对测试非常有用
### shmem / 内部 tmpfs


内部 tmpfs 挂载用于 SysV SHM、memfds、共享匿mmapdev/zero MAP_ANONYMOUS）GPU 驱动DRM 对象、Ashmem
要控制此内部 tmpfs 挂载THP 分配策略，可以使sysfs 旋钮
/sys/kernel/mm/transparent_hugepage/shmem_enabled，以'/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/shmem_enabled' 中每THP 大小的旋钮
全局旋钮的语义与 tmpfs 挂载`huge=` 挂载选项相同，不同之处在于可以单独控制不的大页大小，并且只有per-size 旋钮设为 'inherit' 时才会使用全局旋钮的设置
各个大小'force' 'deny' 选项已被去掉，它们是旧时代的测试产物
always
    每次需要新页时都尝试分<size> 大页
inherit
    继承顶层"shmem_enabled" 值。默认情况下，PMD 尺寸的大    enabled="inherit"，所有其它大页大enabled="never"
never
    不分<size> 大页。注意，即使到处都指定了此模式，``madvise(...,
    MADV_COLLAPSE)`` 仍可能导致获得透明大页
within_size
    只有<size> 大页将完全位i_size 内时才分配    也尊madvise() 提示
advise
    只有在使madvise() 请求时才分配 <size> 大页
## 需要重启应用程

transparent_hugepage/enabled transparent_hugepage/hugepages-<size>kB/enabled 以及 tmpfs 挂载选项只影响未来的行为。因此，要使它们生效，你需要重启任何可能一直在
使用大页的应用程序。这也适用于在 khugepaged 中注册的区域
## 监视使用情况


系统当前使用PMD 尺寸匿名透明大页数量，可通过读取 `/proc/meminfo` 中的 AnonHugePages
字段获得。要识别哪些应用程序正在使用 PMD 尺寸匿名透明大页，需要读`/proc/PID/smaps`
并对每个映射AnonHugePages 字段计数。（注意，出于历史原因，AnonHugePages 只适用传统PMD 尺寸 THP，本应被称为 AnonHugePmdMapped）
映射到用户空间的文件透明大页数量，可通过读取 `/proc/meminfo` 中的 ShmemPmdMapped ShmemHugePages 字段获得。要识别哪些应用程序正在映射文件透明大页，需要读`/proc/PID/smaps` 并对每个映射FilePmdMapped 字段计数
注意，读smaps 文件开销很大，频繁读取会带来开销
`/proc/vmstat` 中有一些计数器，可用于监视系统提供大页供使用的成功程度
thp_fault_alloc
	每次成功分配一个大页并计入（charge）以处理页错误时递增
thp_collapse_alloc
	khugepaged 找到一段应折叠为一个大页的页范围、并成功分配一个新大页
	来存储数据时递增
thp_fault_fallback
	如果页错误未能分配或计入一个大页，而是回退到使用小页，则递增
thp_fault_fallback_charge
	如果页错误未能计入一个大页，而是回退到使用小页（即使分配成功），则递增
thp_collapse_alloc_failed
	如果 khugepaged 找到一段应折叠为一个大页的页范围但分配失败，则递增
thp_file_alloc
	每次成功分配一shmem 大页时递增（注意，尽管以“file”命名，该计数器
	只计shmem）
thp_file_fallback
	如果尝试分配一shmem 大页但失败、而是回退到使用小页，则递增。（注意	尽管以“file”命名，该计数器只计shmem）
thp_file_fallback_charge
	如果一shmem 大页无法计入、而是回退到使用小页（即使分配成功），则递增	（注意，尽管以“file”命名，该计数器只计shmem）
thp_file_mapped
	每次一个文件或 shmem 大页被映射进用户地址空间时递增
thp_split_page
	每次一个大页被拆分为基本页时递增。这可能由于多种原因发生，但一个常见的
	原因是大页已旧并正在被回收。这个动作意味着拆分该页映射的所PMD
thp_split_page_failed
	如果内核未能拆分大页，则递增。这可能发生在该页被某人固定（pin）时
thp_deferred_split_page
	当一个大页被放入拆分队列时递增。这发生在大页被部分取消映射、拆分它	释放一些内存时。拆分队列上的页将在内存压力下被拆分
thp_underused_split_page
	当拆分队列上的一个大页因其未充分利用而被拆分时递增。如果一THP 中的
	零页数量超过某个阈	sys/kernel/mm/transparent_hugepage/khugepaged/max_ptes_none），THP 就是
	未充分利用的
thp_split_pmd
	每次一PMD 被拆分为 PTE 表时递增。例如，当应用程序对大页的一部分调用
	mprotect() munmap() 时可能发生。它不会拆分大页，只拆分页表项
thp_zero_page_alloc
	每次成功分配一个用thp 的巨大零页时递增。注意，它不计数巨大零页的每	映射，只计数其分配
thp_zero_page_alloc_failed
	如果内核未能分配巨大零页、并回退到使用小页，则递增
thp_swpout
	每次一个大页在不拆分的情况下整体交换出去（swapout）时递增
thp_swpout_fallback
	如果一个大页必须在交换出去之前拆分，则递增。通常是因为未能为该大页分	某些连续的交换空间
/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/stats 中，还有针对每个
大页大小的独立计数器，可用于监视系统提供大页供使用的有效性。每个计数器都有其对应的
文件
anon_fault_alloc
	每次成功分配一个大页并计入以处理页错误时递增
anon_fault_fallback
	如果页错误未能分配或计入一个大页，而是回退到使用更低阶的大页或小页，则递增
anon_fault_fallback_charge
	如果页错误未能计入一个大页，而是回退到使用更低阶的大页或小页（即使分	成功），则递增
zswpout
	每次一个大页在不拆分的情况下整体交换到 zswap 时递增
swpin
	每次一个大页在不拆分的情况下从zswap 交换设备整体换入（swapin）时递增
swpin_fallback
	如果换入未能分配或计入一个大页，而是回退到使用更低阶的大页或小页，则递增
swpin_fallback_charge
	如果换入未能计入一个大页，而是回退到使用更低阶的大页或小页（即使分	成功），则递增
swpout
	每次一个大页在不拆分的情况下整体交换到zswap 交换设备时递增
swpout_fallback
	如果一个大页必须在交换出去之前拆分，则递增。通常是因为未能为该大页分	某些连续的交换空间
shmem_alloc
	每次成功分配一shmem 大页时递增
shmem_fallback
	如果尝试分配一shmem 大页但失败、而是回退到使用小页，则递增
shmem_fallback_charge
	如果一shmem 大页无法计入、而是回退到使用小页（即使分配成功），则递增
split
	每次一个大页成功拆分为更小的阶（order）时递增。这可能由于多种原因发生，但
	一个常见的理由是大的页已旧并正在被回收
split_failed
	如果内核未能拆分大页，则递增。这可能发生在该页被某人固定（pin）时
split_deferred
        当一个大页被放入拆分队列时递增        这发生在大页被部分取消映射、拆分它将释放一些内存时。拆分队列上的页将在
        内存压力下被拆分（如果拆分是可能的）
nr_anon
       整个系统中我们拥有的匿名 THP 数量。这THP 可能当前被整体映射，或者具       部分取消映射/未使用的子页
nr_anon_partially_mapped
       可能部分映射、从而可能浪费内存、并已被排入延迟内存回收队列的匿THP 数量       注意，在边角情况下（例如迁移失败），我们可能将一个匿THP 检测为“部分映射       并在此计数，即使它实际上已不再部分映射
随着系统老化，分配大页可能很昂贵，因为系统使用内存压缩（memory compaction）在内存复制数据，以腾出一个大页供使用。`/proc/vmstat` 中有一些计数器可帮助监视这种开销
compact_stall
	每次一个进程停滞以运行内存压缩、从而腾出一个大页供使用时递增
compact_success
	如果系统压缩了内存并腾出一个大页供使用，则递增
compact_fail
	如果系统尝试压缩内存但失败，则递增
可以使用函数跟踪器（function tracer）记录花__alloc_pages() 中的时间，并使用
mm_page_alloc 跟踪点（tracepoint）来识别哪些分配是针对大页，从而确定停滞持续了
多久
## 优化应用程序


要保证内核会在任何内存区域立即映射一THP，mmap 区域必须自然按大页对齐posix_memalign() 可以提供这种保证
## Hugetlbfs


你可以在启用了透明大页支持的内核上照常使用 hugetlbfs，毫无问题。除了整体碎片化更少之外，hugetlbfs 中不会注意到任何差异。属hugetlbfs 的所有常用特性都得以保留
且不受影响。libhugetlbfs 也会像往常一样正常工作