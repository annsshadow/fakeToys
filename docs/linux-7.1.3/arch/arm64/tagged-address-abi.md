## AArch64 带标签地址 ABI


Authors: Vincenzo Frascino <vincenzo.frascino@arm.com>
         Catalin Marinas <catalin.marinas@arm.com>

Date: 21 August 2019

本文档描述了 AArch64 Linux 上带标签地址（Tagged Address）ABI 的用法和语义。

### 1. 简介


在 AArch64 上，`TCR_EL1.TBI0` 位默认被设置，允许用户空间（EL0）通过具有非零最高字节的 64 位指针执行内存访问。本文档描述了 syscall ABI 的放宽，该放宽允许用户空间将某些带标签的指针传递给内核系统调用。

### 2. AArch64 带标签地址 ABI


从内核系统调用接口的角度以及出于本文档的目的，“有效的带标签指针”是指一个可能具有非零最高字节、且引用了用户进程地址空间中通过以下方式之一获得的地址的指针：

- `mmap()` 系统调用，且满足以下任一条件：

  - 标志设置了 `MAP_ANONYMOUS` 位，或
  - 文件描述符引用一个普通文件（包括由 `memfd_create()` 返回的文件）或 `/dev/zero`

- `brk()` 系统调用（即进程创建时程序断点初始位置与其当前位置之间的堆区域）。

- 内核在进程地址空间中创建的、具有与上面 `mmap()` 相同限制（例如数据、bss、栈）的任何内存映射。

AArch64 带标签地址 ABI 根据内核如何使用用户地址，分为两个阶段的放宽：

1. 不被内核访问但用于地址空间管理的用户地址（例如 `mprotect()`、`madvise()`）。在此上下文中允许使用有效的带标签指针，但有以下例外：

   - `brk()`、`mmap()` 以及 `mremap()` 的 `new_address` 参数，因为它们有可能与现有的用户地址产生别名。

     ﻿注意：此行为在 v5.6 中发生了变化，因此某些较早的内核可能会错误地接受针对 `brk()`、`mmap()` 和 `mremap()` 系统调用的有效带标签指针。

   - 在从 `userfaultfd()` 获得的文件描述符上使用的 `UFFDIO_*` `ioctl()` 的 `range.start`、`start` 和 `dst` 参数，因为随后通过读取该文件描述符获得的故障地址将是去标签的，否则可能会让不知道标签的程序感到困惑。

     ﻿注意：此行为在 v5.14 中发生了变化，因此某些较早的内核可能会错误地接受针对该系统调用的有效带标签指针。

2. 被内核访问的用户地址（例如 `write()`）。此 ABI 放宽默认是禁用的，应用程序线程需要通过 `prctl()` 显式启用，如下所示：

   - `PR_SET_TAGGED_ADDR_CTRL`：为调用线程启用或禁用 AArch64 带标签地址 ABI。

     `(unsigned int) arg2` 参数是一个描述所用控制模式的位掩码：

     - `PR_TAGGED_ADDR_ENABLE`：启用 AArch64 带标签地址 ABI。默认状态为禁用。

     ﻿参数 `arg3`、`arg4` 和 `arg5` 必须为 0。

   - `PR_GET_TAGGED_ADDR_CTRL`：获取调用线程的 AArch64 带标签地址 ABI 状态。

     ﻿参数 `arg2`、`arg3`、`arg4` 和 `arg5` 必须为 0。

   上述 ABI 属性是线程作用域的，在 clone() 和 fork() 时继承，在 exec() 时清除。

   如果 AArch64 带标签地址 ABI 被 `sysctl abi.tagged_addr_disabled=1` 全局禁用，那么调用 `prctl(PR_SET_TAGGED_ADDR_CTRL, PR_TAGGED_ADDR_ENABLE, 0, 0, 0)` 将返回 `-EINVAL`。默认的 `sysctl abi.tagged_addr_disabled` 配置为 0。

当为某线程启用了 AArch64 带标签地址 ABI 时，保证以下行为：

- 除第 3 节提到的情形外，所有系统调用都可以接受任何有效的带标签指针。

- 对于无效的带标签指针，系统调用的行为是未定义的：它可能导致返回错误码、引发（致命）信号，或其他失败模式。

- 对于有效的带标签指针，系统调用的行为与对应的去标签指针相同。


AArch64 上带标签指针含义的定义可以在 Documentation/arch/arm64/tagged-pointers.rst 中找到。

### 3. AArch64 带标签地址 ABI 例外


无论 ABI 放宽与否，以下系统调用参数必须去标签：

- `prctl()` 中除直接间接作为内核要访问的参数传递的用户数据指针之外的其他参数。

- `ioctl()` 中除直接间接作为内核要访问的参数传递的用户数据指针之外的其他参数。

- `shmat()` 和 `shmdt()`。

- `brk()`（自内核 v5.6 起）。

- `mmap()`（自内核 v5.6 起）。

- `mremap()` 的 `new_address` 参数（自内核 v5.6 起）。

任何使用非零带标签指针的尝试都可能导致返回错误码、引发（致命）信号，或其他失败模式。

### 4. 正确用法示例


   #include <stdlib.h>
   #include <string.h>
   #include <unistd.h>
   #include <sys/mman.h>
   #include <sys/prctl.h>

   #define PR_SET_TAGGED_ADDR_CTRL	55
   #define PR_TAGGED_ADDR_ENABLE	(1UL << 0)

   #define TAG_SHIFT		56

   int main(void)
   {
   	int tbi_enabled = 0;
   	unsigned long tag = 0;
   	char *ptr;

   	/** check/enable the tagged address ABI **/
   	if (!prctl(PR_SET_TAGGED_ADDR_CTRL, PR_TAGGED_ADDR_ENABLE, 0, 0, 0))
   		tbi_enabled = 1;

   	/** memory allocation **/
   	ptr = mmap(NULL, sysconf(_SC_PAGE_SIZE), PROT_READ | PROT_WRITE,
   		   MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
   	if (ptr == MAP_FAILED)
   		return 1;

   	/** set a non-zero tag if the ABI is available **/
   	if (tbi_enabled)
   		tag = rand() & 0xff;
   	ptr = (char *)((unsigned long)ptr | (tag << TAG_SHIFT));

   	/** memory access to a tagged address **/
   	strcpy(ptr, "tagged pointer\n");

   	/** syscall with a tagged pointer **/
   	write(1, ptr, strlen(ptr));

   	return 0;
   }
