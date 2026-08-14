
## spu_run

## 名称

       spu_run - 执行一个 spu 上下文

## 概要

```

	    #include <sys/spu.h>

	    int spu_run(int fd, unsigned int *npc, unsigned int *event);

```
## 描述

       spu_run 系统调用用于实现了 Cell Broadband Engine Architecture 的 PowerPC
       机器，以便访问协同处理器单元（Synergistic Processor Units，SPUs）。它使用
       从 spu_create(2) 返回的 fd 来寻址一个特定的 SPU 上下文。当该上下文被调度到
       一个物理 SPU 上时，它会从传入的 npc 中的指令指针处开始执行。

       SPU 代码的执行是同步的，意味着在 SPU 仍在运行时 spu_run 不会返回。如果需要
       与主线 CPU 或其它 SPU 上的其它代码并行执行 SPU 代码，你需要先创建一个新的
       执行线程，例如使用 pthread_create(3) 调用。

       当 spu_run 返回时，SPU 指令指针的当前值会被写回 npc，因此你可以再次调用
       spu_run 而无需更新指针。

       event 可以是一个 NULL 指针，也可以指向一个扩展状态码，该状态码在 spu_run
       返回时被填充。它可以是以下常量之一：

       SPE_EVENT_DMA_ALIGNMENT
              一个 DMA 对齐错误

       SPE_EVENT_SPE_DATA_SEGMENT
              一个 DMA 分段错误

       SPE_EVENT_SPE_DATA_STORAGE
              一个 DMA 存储错误

       如果 event 参数传入 NULL，这些错误将导致向调用进程发送一个信号。

## 返回值

       spu_run 返回 spu_status 寄存器的值，或返回 -1 表示出错，并将 errno 设置为
       下面列出的错误码之一。spu_status 寄存器的值包含一个状态码的位掩码，以及
       （可选地）从 SPU 上的 stop-and-signal 指令返回的 14 位代码。状态码的位掩码
       如下：

       0x02
	      SPU 被 stop-and-signal 停止。

       0x04
	      SPU 被 halt 停止。

       0x08
	      SPU 正在等待一个通道。

       0x10
	      SPU 处于单步（single-step）模式。

       0x20
	      SPU 试图执行一条无效指令。

       0x40
	      SPU 试图访问一个无效通道。

       0x3fff0000
              与此值相掩码的位包含从 stop-and-signal 返回的代码。

       总是会设置低 8 位中的一个或多个，或者从 spu_run 返回一个错误码。

## 错误

       EAGAIN 或 EWOULDBLOCK
	      fd 处于非阻塞模式，且 spu_run 会阻塞。

       EBADF  fd 不是有效的文件描述符。

       EFAULT npc 不是有效指针，或者 status 既不是 NULL 也不是有效指针。

       EINTR  在 spu_run 进行期间发生了信号。必要时 npc 值已被更新为新的程序计数器
	      值。

       EINVAL fd 不是从 spu_create(2) 返回的文件描述符。

       ENOMEM 没有足够的内存来处理由 MFC 直接内存访问引发的页错误。

       ENOSYS 当前系统未提供该功能，因为要么硬件不提供 SPU，要么 spufs 模块未加载。

## 注意

       spu_run 旨在由实现了对 SPU 更抽象接口的库使用，而不是由常规应用程序使用。
       关于推荐的库，请参见 http://www.bsc.es/projects/deepcomputing/linuxoncell/。

## 遵循标准

       此调用是 Linux 特有的，并且仅由 ppc64 架构实现。使用此系统调用的程序不可移植。

## 缺陷

       代码尚未完全实现此处列出的所有功能。

## 作者

       Arnd Bergmann <arndb@de.ibm.com>

## 另请参阅

       capabilities(7), close(2), spu_create(2), spufs(7)
