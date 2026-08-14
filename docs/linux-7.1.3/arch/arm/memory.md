## ARM Linux 上的内核内存布局


		Russell King <rmk@arm.linux.org.uk>

		     2005 年 11 月 17 日 (2.6.15)

本文档描述了 Linux 内核用于 ARM 处理器的虚拟内存布局。它指出了哪些区域可供平台使用，
哪些区域由通用代码使用。

ARM CPU 最多可寻址 4GB 虚拟内存空间，这必须在用户空间进程、内核以及硬件设备之间共享。

随着 ARM 架构的成熟，有必要为新的功能保留某些 VM 空间区域；因此本文档可能会随着时间
推移保留更多的 VM 空间。

=============== =============== ===============================================
Start		End		Use
=============== =============== ===============================================
ffff8000	ffffffff	copy_user_page / clear_user_page 使用。
				对于 SA11xx 和 Xscale，用于
				建立 minicache 映射。

ffff4000	ffffffff	ARMv6 及更新 CPU 上的缓存别名（cache aliasing）。

ffff1000	ffff7fff	保留区。
				平台不得使用此地址范围。

ffff0000	ffff0fff	CPU 向量页。
				如果 CPU 支持向量重定位（控制
				寄存器 V 位），则 CPU 向量映射于此。

fffe0000	fffeffff	XScale 缓存刷新区域。这用于
				proc-xscale.S 中以刷新整个数据
				缓存。（XScale 没有 TCM。）

fffe8000	fffeffff	CPU 内置 DTCM 的平台的 DTCM 映射区域。

fffe0000	fffe7fff	CPU 内置 ITCM 的平台的 ITCM 映射区域。

ffc80000	ffefffff	Fixmap 映射区域。fix_to_virt() 提供的
				地址将位于此区域。

ffc00000	ffc7ffff	保护区域（Guard region）

ff800000	ffbfffff	固件提供的 DT blob 的永久、固定只读映射

fee00000	feffffff	PCI I/O 空间的映射。这是 vmalloc 空间内
				的一个静态映射。

VMALLOC_START	VMALLOC_END-1	vmalloc() / ioremap() 空间。
				由 vmalloc/ioremap 返回的内存将
				被动态放置在该区域中。机器特定的
				静态映射也通过 iotable_init() 位于此处。
				VMALLOC_START 基于 high_memory 变量的值，
				VMALLOC_END 等于 0xff800000。

PAGE_OFFSET	high_memory-1	内核直接映射的 RAM 区域。
				它映射平台的 RAM，通常以 1:1 的关系
				映射所有平台 RAM。

PKMAP_BASE	PAGE_OFFSET-1	永久内核映射
				将 HIGHMEM 页映射到内核空间的
				一种方式。

MODULES_VADDR	MODULES_END-1	内核模块空间
				通过 insmod 插入的内核模块使用
				动态映射放置于此。

TASK_SIZE	MODULES_VADDR-1	启用 KASan 时的 KASan 影子内存。
				从 MODULES_VADDR 到内存顶端的
				范围在此处以每字节内存 1 位的方式
				被映射为影子。

00001000	TASK_SIZE-1	用户空间映射
				每线程映射通过 mmap() 系统调用
				放置于此。

00000000	00000fff	CPU 向量页 / 空指针陷阱
				不支持向量重映射的 CPU 将其向量页
				放置于此。内核和用户空间的 NULL 指针
				解引用也通过此映射被捕获。
=============== =============== ===============================================

请注意，与上述区域冲突的映射可能导致内核无法启动，或导致内核在运行时（最终）发生 panic。

由于未来的 CPU 可能会影响内核映射布局，用户程序不得访问其 0x0001000 到 TASK_SIZE
地址范围之外任何未被映射的内存。如果希望访问这些区域，必须通过 open() 与 mmap() 自行
建立映射。
