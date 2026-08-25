## 移植（Porting

取自列表存档 http://lists.arm.linux.org.uk/pipermail/linux-arm-kernel/2001-July/004064.html

### 初始定义


以下符号定义依赖于你了解 __virt_to_phys() 为你的机器所做的转换。此宏将传入的虚拟地址转换为物理地址通常，它简单地是：

		phys = virt - PAGE_OFFSET + PHYS_OFFSET


### 解压器符

ZTEXTADDR
	解压器的起始地址。这里讨论虚拟或物理地址没有意义，因为在你调用解压器代码MMU 是关闭的	你通常在此地址调用内核以启动引导。这不必位于 RAM 中，它可以位于闪存或其他只读或读写的
	可寻址介质中
ZBSSADDR
	解压器零初始化工作区的起始地址。这必须指向 RAM。解压器会为你将其零初始化。同样，MMU 是关闭的
ZRELADDR
	这是解压后内核将被写入并最终执行的位置。以下约束必须成立：

		__virt_to_phys(TEXTADDR) == ZRELADDR

	内核的初始部分被仔细编写为位置无关
INITRD_PHYS
	放置初始 RAM 磁盘的物理地址。仅当你使用 bootpImage 那一套东西（它只适用于旧struct param_struct）时相关
INITRD_VIRT
	初始 RAM 磁盘的虚拟地址。以下约束必须成立：

		__virt_to_phys(INITRD_VIRT) == INITRD_PHYS

PARAMS_PHYS
	struct param_struct 或标签列表的物理地址，用于向内核提供关于其执行环境的各种参数

### 内核符号


PHYS_OFFSET
	第一RAM bank 的物理起始地址
PAGE_OFFSET
	第一RAM bank 的虚拟起始地址。在内核引导阶段，虚拟地址 PAGE_OFFSET 将被映射到物理地址 PHYS_OFFSET	连同你提供的任何其他映射。这应与 TASK_SIZE 具有相同的值
TASK_SIZE
	用户进程的最大大小（字节）。由于用户空间总是从零开始，这是用户进程可以访问的最大地址+1。用户空间栈
	从此地址向下增长
	任何低于 TASK_SIZE 的虚拟地址都被视为用户进程区域，因此由内核按进程动态管理。我称之为用户段
	任何高于 TASK_SIZE 的地址对所有进程都是共有的。我称之为内核段
	（换句话说，你不能将 IO 映射放在 TASK_SIZE 之下，因此也不能放在 PAGE_OFFSET 之下。）

TEXTADDR
	内核的虚拟起始地址，通常PAGE_OFFSET + 0x8000。这就是内核映像最终所在的位置。对于最新的内核	它必须位于一128MB 区域32768 字节处。以前的内核在此处施加了 256MB 的限制
DATAADDR
	内核数据段的虚拟地址。使用解压器时绝不能定义
VMALLOC_START / VMALLOC_END
	界定 vmalloc() 区域的虚拟地址。此区域内不得有任何静态映射；vmalloc 会覆盖它们。这些地址也必须位	内核段中（见上）。通常，vmalloc() 区域从最后一个虚RAM 地址（使用变high_memory 找到）之	VMALLOC_OFFSET 字节处开始
VMALLOC_OFFSET
	通常并入 VMALLOC_START 的偏移量，用于在虚拟 RAM vmalloc 区域之间提供一个空洞。我们这样做是为	能够捕获越界内存访问（例如，某个东西写到映射内存映射的末尾之外）。通常设置8MB
### 架构特定

BOOT_MEM(pram,pio,vio)
	`pram` 指定 RAM 的物理起始地址。必须始终存在，并且应与 PHYS_OFFSET 相同
	`pio` 是包含用arch/arm/kernel/debug-armv.S 中调试宏IO 8MB 区域的物理地址
	`vio` 是该 8MB 调试区域的虚拟地址
	预计该调试区域稍后会被架构特定代码（通过 MAPIO 函数）重新初始化
BOOT_PARAMS
	PARAMS_PHYS，参PARAMS_PHYS
FIXUP(func)
	机器特定的修复，在内存子系统初始化之前运行
MAPIO(func)
	机器特定的函数，用于映射 IO 区域（包括上面的调试区域）
INITIRQ(func)
	机器特定的函数，用于初始化中断