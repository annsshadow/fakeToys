## page owner: Tracking about who allocated each page


## Introduction


page owner 用于跟踪每个页面是由谁分配的。它可用于调试内存泄漏，或找出内存占用大户
当发生分配时，关于分配的信息（如调用栈和页面order）会被存储到每个页面的特定存储中
当我们需要了解所有页面的状态时，可以获取并分析这些信息

尽管我们已经有用于跟踪页面分释放tracepoint，但用它来分析谁分配了每个页面相
复杂。我们需要扩trace 缓冲区，以防止在用户空间程序启动前发生重叠。而且，启动的程序
持续转储 trace 缓冲区以供后续分析，这比仅仅将其保留在内存中更可能改变系统行为，因此
利于调试

page owner 也可用于各种其他目的。例如，通过每个页面gfp 标志信息可以获得准确的碎
统计。如果启用了 page owner，这已经实现并激活。欢迎其他用法

它还可用于显示所有调用栈及其当前分配的基页数量，这让我们无需筛查所有页面并匹配分配
释放操作，就能快速了解内存的去向。也可以只显示所有调用栈（不含栈回溯）的数字标识符及
分配的基页数量（读取和解析更快，例如用于监控），之后可以与调用栈匹配（show_handles 
show_stacks_handles）

page owner 默认是禁用的。因此，如果你想使用它，需要在启动命令行中添加“page_owner=on”
如果内核构建page owner，但由于未启用启动选项而在运行时被禁用，运行时开销是微乎其微的
如果在运行时禁用，则不需要内存来存储所有者信息，因此没有运行时内存开销。而且，page owner
只向页面分配器热路径中插入了两个不太可能执行的分支，如果未启用，则分配就像没page owner
的内核一样进行。这两个不太可能执行的分支不应影响分配性能，特别是在可用静态键跳转标签
补丁（static keys jump label patching）功能的情况下。以下是该设施导致的内核代码大小变化

虽然启用 page owner 会使内核大小增加几千字节，但其中大部分代码位于页面分配器及其热路
之外。在需要时用启page owner 构建内核并在需要时打开它，是调试内核内存问题的绝佳选择

有一个由实现细节引起的注意事项。page owner 将信息存储到 struct page extension 的内存中
在稀疏内存（sparse memory）系统中，这块内存的初始化时间晚于页面分配器启动的时间，因此
初始化之前，许多页面可能已经被分配，它们将没有所有者信息。为了修复这一点，这些早期分配
页面在初始化阶段被检查并标记为已分配。虽然这并不意味着它们有正确的所有者信息，但至少我
可以更准确地判断页面是否被分配。在一2GB 内存x86-64 虚拟机上，捕获并标记13343 
早期分配的页面，尽管它们大多是从 struct page extension 特性分配的。无论如何，之后没有页面
处于未跟踪状态

## Usage


```

	cd tools/mm
	make page_owner_sort

```
2) 启用 page owner：在启动命令行中添加“page_owner=on”

3) 做你想要调试的工作

```

	cat /sys/kernel/debug/page_owner_stacks/show_stacks > stacks.txt
	cat stacks.txt
	 post_alloc_hook+0x177/0x1a0
	 get_page_from_freelist+0xd01/0xd80
	 __alloc_pages+0x39e/0x7e0
	 allocate_slab+0xbc/0x3f0
	 ___slab_alloc+0x528/0x8a0
	 kmem_cache_alloc+0x224/0x3b0
	 sk_prot_alloc+0x58/0x1a0
	 sk_alloc+0x32/0x4f0
	 inet_create+0x427/0xb50
	 __sock_create+0x2e4/0x650
	 inet_ctl_sock_create+0x30/0x180
	 igmp_net_init+0xc1/0x130
	 ops_init+0x167/0x410
	 setup_net+0x304/0xa60
	 copy_net_ns+0x29b/0x4a0
	 create_new_namespaces+0x4a1/0x820
	nr_base_pages: 16
	...
	...
	echo 7000 > /sys/kernel/debug/page_owner_stacks/count_threshold
	cat /sys/kernel/debug/page_owner_stacks/show_stacks> stacks_7000.txt
	cat stacks_7000.txt
	 post_alloc_hook+0x177/0x1a0
	 get_page_from_freelist+0xd01/0xd80
	 __alloc_pages+0x39e/0x7e0
	 alloc_pages_mpol+0x22e/0x490
	 folio_alloc+0xd5/0x110
	 filemap_alloc_folio+0x78/0x230
	 page_cache_ra_order+0x287/0x6f0
	 filemap_get_pages+0x517/0x1160
	 filemap_read+0x304/0x9f0
	 xfs_file_buffered_read+0xe6/0x1d0 [xfs]
	 xfs_file_read_iter+0x1f0/0x380 [xfs]
	 __kernel_read+0x3b9/0x730
	 kernel_read_file+0x309/0x4d0
	 __do_sys_finit_module+0x381/0x730
	 do_syscall_64+0x8d/0x150
	 entry_SYSCALL_64_after_hwframe+0x62/0x6a
	nr_base_pages: 20824
	...

	cat /sys/kernel/debug/page_owner_stacks/show_handles > handles_7000.txt
	cat handles_7000.txt
	handle: 42
	nr_base_pages: 20824
	...

	cat /sys/kernel/debug/page_owner_stacks/show_stacks_handles > stacks_handles.txt
	cat stacks_handles.txt
	 post_alloc_hook+0x177/0x1a0
	 get_page_from_freelist+0xd01/0xd80
	 __alloc_pages+0x39e/0x7e0
	 alloc_pages_mpol+0x22e/0x490
	 folio_alloc+0xd5/0x110
	 filemap_alloc_folio+0x78/0x230
	 page_cache_ra_order+0x287/0x6f0
	 filemap_get_pages+0x517/0x1160
	 filemap_read+0x304/0x9f0
	 xfs_file_buffered_read+0xe6/0x1d0 [xfs]
	 xfs_file_read_iter+0x1f0/0x380 [xfs]
	 __kernel_read+0x3b9/0x730
	 kernel_read_file+0x309/0x4d0
	 __do_sys_finit_module+0x381/0x730
	 do_syscall_64+0x8d/0x150
	 entry_SYSCALL_64_after_hwframe+0x62/0x6a
	handle: 42
	...

	cat /sys/kernel/debug/page_owner > page_owner_full.txt
	./page_owner_sort page_owner_full.txt sorted_page_owner.txt

   The general output of ``page_owner_full.txt`` is as follows::

	Page allocated via order XXX, ...
	PFN XXX ...
	// Detailed stack

	Page allocated via order XXX, ...
	PFN XXX ...
	// Detailed stack
    By default, it will do full pfn dump, to start with a given pfn,
    page_owner supports fseek.

    FILE *fp = fopen("/sys/kernel/debug/page_owner", "r");
    fseek(fp, pfn_start, SEEK_SET);

   The ``page_owner_sort`` tool ignores ``PFN`` rows, puts the remaining rows
   in buf, uses regexp to extract the page order value, counts the times
   and pages of buf, and finally sorts them according to the parameter(s).

   See the result about who allocated each page
   in the ``sorted_page_owner.txt``. General output::

	XXX times, XXX pages:
	Page allocated via order XXX, ...
	// Detailed stack

   By default, ``page_owner_sort`` is sorted according to the times of buf.
   If you want to sort by the page nums of buf, use the ``-m`` parameter.
   The detailed parameters are:

   fundamental function::

	Sort:
		-a		Sort by memory allocation time.
		-m		Sort by total memory.
		-p		Sort by pid.
		-P		Sort by tgid.
		-n		Sort by task command name.
		-r		Sort by memory release time.
		-s		Sort by stack trace.
		-t		Sort by times (default).
		--sort <order>	Specify sorting order.  Sorting syntax is [+|-]key[,[+|-]key[,...]].
				Choose a key from the **STANDARD FORMAT SPECIFIERS** section. The "+" is
				optional since default direction is increasing numerical or lexicographic
				order. Mixed use of abbreviated and complete-form of keys is allowed.

		Examples:
				./page_owner_sort <input> <output> --sort=n,+pid,-tgid
				./page_owner_sort <input> <output> --sort=at

   additional function::

	Cull:
		--cull <rules>
				Specify culling rules.Culling syntax is key[,key[,...]].Choose a
				multi-letter key from the **STANDARD FORMAT SPECIFIERS** section.

		<rules> is a single argument in the form of a comma-separated list,
		which offers a way to specify individual culling rules.  The recognized
		keywords are described in the **STANDARD FORMAT SPECIFIERS** section below.
		<rules> can be specified by the sequence of keys k1,k2, ..., as described in
		the STANDARD SORT KEYS section below. Mixed use of abbreviated and
		complete-form of keys is allowed.

		Examples:
				./page_owner_sort <input> <output> --cull=stacktrace
				./page_owner_sort <input> <output> --cull=st,pid,name
				./page_owner_sort <input> <output> --cull=n,f

	Filter:
		-f		Filter out the information of blocks whose memory has been released.

	Select:
		--pid <pidlist>		Select by pid. This selects the blocks whose process ID
					numbers appear in <pidlist>.
		--tgid <tgidlist>	Select by tgid. This selects the blocks whose thread
					group ID numbers appear in <tgidlist>.
		--name <cmdlist>	Select by task command name. This selects the blocks whose
					task command name appear in <cmdlist>.

		<pidlist>, <tgidlist>, <cmdlist> are single arguments in the form of a comma-separated list,
		which offers a way to specify individual selecting rules.


		Examples:
				./page_owner_sort <input> <output> --pid=1
				./page_owner_sort <input> <output> --tgid=1,2,3
				./page_owner_sort <input> <output> --name name1,name2

```
## STANDARD FORMAT SPECIFIERS

```

  For --sort option:

	KEY		LONG		DESCRIPTION
	p		pid		process ID
	tg		tgid		thread group ID
	n		name		task command name
	st		stacktrace	stack trace of the page allocation
	T		txt		full text of block
	ft		free_ts		timestamp of the page when it was released
	at		alloc_ts	timestamp of the page when it was allocated
	ator		allocator	memory allocator for pages

  For --cull option:

	KEY		LONG		DESCRIPTION
	p		pid		process ID
	tg		tgid		thread group ID
	n		name		task command name
	f		free		whether the page has been released or not
	st		stacktrace	stack trace of the page allocation
	ator		allocator	memory allocator for pages

```