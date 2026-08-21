## unshare 系统调用

本文档描述新的系统调unshare()。文档概述了该特性、为何需要它、如何使用、其接口规范、设计、实现以及如何测试
### Change Log（变更日志）

version 0.1  初始文档，Janak Desai (janak@us.ibm.com)006 1 11 
### Contents（目录）

 1) Overview（概述）
 2) Benefits（收益）
 3) Cost（代价）
 4) Requirements（需求）
 5) Functional Specification（功能规范）
 6) High Level Design（高层设计）
 7) Low Level Design（底层设计）
 8) Test Specification（测试规范）
 9) Future Work（未来工作）

### 1) Overview（概述）

大多数传统操作系统内核对线程做了抽象，将其视为进程内的多个执行上下文。这些内核提供特殊的资源和机制来维护这些 “线程”。Linux 内核以一种巧妙而简单的方式，不在进程和 “线程之间做区分。内核允许进程共享资源，从而无需在内核中引入额外的数据结构和机制就能实现传统“线程行为。以这种方式实现线程的强大之处，不仅来自其简洁性，也来自让应用程序员能够摆脱传统线程那“要么全共享、要么全不共享的资源束缚。在 Linux 上，使用 clone 系统调用创建线程时，应用程序可以有选择地决定在哪些线程之间共享哪些资源
unshare() 系统调用Linux 线程模型增加了一个原语，允许线程在创建时 “取消共享”（unshare）当时正在被共享的任何资源。unshare() Al Viro 2000 8 月的 Linux-Kernel 邮件列表上，作为关于 Linux POSIX 线程讨论的一部分而提出概念。unshare() 增强Linux 线程对那些希望在不创建新进程的情况下控制共享资源的应用程序的实用性。unshare() Linux 上实“将进程/线程视作虚拟机这一概念的一组可用原语的一个自然补充
### 2) Benefits（收益）

unshare() PAM 这类大型应用框架会很有用，因为在这种框架下，通过创建新进程来控制进程资源的共取消共享是不可能的。由于使fork clone 创建新进程时默认会共享命名空间，所以即使是非线程化的应用程序，只要它们有与默认共享命名空间脱离关联的需求，也能unshare() 中受益。下面列出两个可使用 unshare() 的用例
#### 2.1 Per-security context namespaces（按安全上下文的命名空间
unshare() 可用于借助内核的每进程命名空间机制来实现多实例化（polyinstantiated）目录。多实例化目录，例如每用户和/或每安全上下文的 /tmpvar/tmp 实例，或用户主目录的每安全上下文实例，能在处理这些目录时隔离用户进程。借助 unshare()，一PAM 模块可以在用户登录时轻松为其建立一个私有命名空间。多实例化目录是带标签系统保护规范（Labeled System Protection Profile）通过通用准则（Common Criteria）认证所必需的；然而，由于 Linux 内核中提供了共享树（shared-tree）特性，即便是普通的 Linux 系统也能从在登录时建立私有命名空间、并/tmpvar/tmp 以及系统管理员认为合适的其他目录做多实例化中受益
#### 2.2 unsharing of virtual memory and/or open files（取消共享虚拟内存和/或打开的文件）

考虑一个客户端/服务器应用程序，其中服务器通过创建共享虚拟内存和打开文件等资源的进程来处理客户端请求。如果没unshare()，服务器就必须在创建服务该请求的进程时决定哪些需要共享。unshare() 让服务器有能力在处理请求的过程中解除部分上下文的共享。对于大型而复杂的中间件应用框架，这种在进程创建之后再 unshare() 的能力会非常有用
### 3) Cost（代价）

为了不重复代码，并且为了处理 unshare() 作用于一个活动任务（与作用于新分配的、尚未活动的任务clone/fork 相对）这一事实，unshare() 必须clone/fork 系统调用所使用copy_* 函数做了较小的重组性改动。改动已有的、经过充分测试且稳定的代码来实现一个新特性，是存在代价的，尤其是该特性在初期可能不会被广泛运用。不过，只要对这些改动做了恰当的设计与代码审查，并为 LTP 创建unshare() 测试，那么这一新特性的收益就能超过其代价
### 4) Requirements（需求）

unshare() 会撤销使用 clone(2) 系统调用所做的共享，所unshare() 应当拥有clone(2) 类似的接口。也就是说，由于 clone(int flags, void \*stack) 中的 flags 指定了应当共享什么，那么 unshare(int flags) 中类似的 flags 就应当指定应当取消共享什么。遗憾的是，这看起来像是flags 的含义相对于它们clone(2) 中的用法做了反转。然而，在当时并没有一个更不易混淆、且能允许未来在不改ABI 的情况下增量取消共享上下文的更优解
unshare() 的接口应当能容纳未来可能新增的上下文 flags，而无需重新编译旧的应用程序。如果将来新增了上下flags，unshare() 的设计应当允许按需对这些资源做增量取消共享
### 5) Functional Specification（功能规范）

NAME
	unshare - 解除进程执行上下文中的部分共
SYNOPSIS
	#include <sched.h>

	int unshare(int flags);

DESCRIPTION
	unshare() 允许一个进程解除其当前正与其他进程共享的执行上下文中的部分内容的关联。部分执行上下文，例如命名空间，	使用 fork(2) 创建新进程时默认是共享的；而其他部分，例如虚拟内存、打开的文件描述符等，可能是在使用 clone(2)
	创建进程时通过显式请求共享才会被共享
	unshare() 的主要用途是允许一个进程在不创建新进程的情况下控制其共享的执行上下文
	flags 参数指定下列常量中的一个，或由若干个按位或（bitwise-or）组合而成
	CLONE_FS
		如果设置CLONE_FS，调用者的文件系统信息将与共享的文件系统信息解除关联
	CLONE_FILES
		如果设置CLONE_FILES，调用者的文件描述符表将与共享的文件描述符表解除关联
	CLONE_NEWNS
		如果设置CLONE_NEWNS，调用者的命名空间将与共享的命名空间解除关联
	CLONE_VM
		如果设置CLONE_VM，调用者的虚拟内存将与共享的虚拟内存解除关联
RETURN VALUE
	成功时返0。失败时返回 -1，并设置 errno
ERRORS
	EPERM	root 进程（不具备 CAP_SYS_ADMIN 的进程）指定CLONE_NEWNS
	ENOMEM	无法分配足够的内存来复制调用者上下文中需要取消共享的部分
	EINVAL	指定的参数是一个无效的 flag
CONFORMING TO
	unshare() 调用Linux 特有的，不应在意图可移植的程序中使用
SEE ALSO
	clone(2), fork(2)

### 6) High Level Design（高层设计）

根据 flags 参数，unshare() 系统调用会分配合适的进程上下文结构，用当前共享版本中的值填充它，将新复制出来的结构关联到当前任务结构，并释放相应的共享版本。unshare() 不能直接使用 clone 的辅助函数（copy_*），原因如下
  1) clone 作用于一个新分配的、尚未活动的任务结构，unshare() 作用于当前的活动任务。因unshare() 在关联新复制的上下文结构之前，必须先取得恰当task_lock()
  2) unshare() 必须先分配并复制所有正在被取消共享的上下文结构，再将它们关联到当前任务，并释放旧的共享结构。不这样做的话，在出错试图回退时会产生竞态和/oops。考虑同时取消共享虚拟内存和命名空间的情况：在成功取消共享 vm 之后，如果系统调用在分配新命名空间结构时遇到错误，那么错误返回码就必须反转对 vm 的取消共享。作为反转的一部分，系统调用必须回到旧的、共享的 vm 结构，而那个结构可能已不复存在
因此，copy_* 函数中负责分配并复制当前上下文结构的代码被移入新dup_* 函数。现在，copy_** 函数调用 dup_** 函数来分配并复制相应的上下文结构，然后将它们关联到正在构建的任务结构。unshare() 系统调用则执行以下步骤：

  1) 检flags 以强制补全缺失但被隐含的 flags

  2) 对每个上下文结构，若 flags 参数中相应的位被置位，则调用对应unshare() 辅助函数来分配并复制一个新的上下文结构
  3) 如果在分配和复制过程中没有错误，并且存在新的上下文结构，则锁定当前任务结构，将新的上下文结构关联到当前任务结构，然后释放当前任务结构上的锁
  4) 适当地释放旧的、共享的上下文结构
### 7) Low Level Design（底层设计）

unshare() 的实现可归为以下 4 个不同的部分
  a) 对已copy_* 函数的重
  b) unshare() 系统调用服务函数

  c) 针对每种不同进程上下文的 unshare() 辅助函数

  d) 为不同架构注册系统调用号

#### 7.1) Reorganization of copy_* functions（copy_* 函数的重组）

每个 copy 函数，例copy_mm、copy_namespace、copy_files 等，大致由两部分组成。第一部分分配并复制相应的结构，第二部分将其链接到作为入参传给 copy 函数的任务结构。第一部分被拆分为独立的函数。这dup_* 函数负责分配并复制相应的上下文结构。重组后copy_* 函数调用对应dup_* 函数，然后将新复制出来的结构链接到调copy 函数时所用的任务结构
#### 7.2) unshare() system call service function（unshare() 系统调用服务函数
       - 检flags
	 强制隐含flags。如果设置了 CLONE_THREAD 则强CLONE_VM	 如果设置CLONE_VM，则强制 CLONE_SIGHAND。如果设置了 CLONE_SIGHAND
	 且信号也正在被共享，则强CLONE_THREAD。如果设置了 CLONE_NEWNS，则强制 CLONE_FS
       - 对每个上下文 flag，调用对应的 unshare_* 辅助例程，传入传给系统调用的 flags	 以及一个指向指向新取消共享结构的指针的引用
       - 如果 unshare_* 辅助函数创建了任何新结构，则对当前任务取 task_lock()	 修改相应的上下文指针，然后释放该任务锁
       - 对所有新取消共享的结构，释放相应的旧的、共享的结构
#### 7.3) unshare_* helper functions（unshare_* 辅助函数
对于对应CLONE_SYSVSEM、CLONE_SIGHAND CLONE_THREAD unshare_* 辅助函数，由于尚未实现，返回 -EINVAL。对于其余的，则检flag 值以判断该结构是否需要取消共享。如果需要，则调用对应的 dup_* 函数来分配并复制该结构，并返回指向它的指针
#### 7.4) Finally（最后）

适当地修改架构相关代码以注册这一新的系统调用
### 8) Test Specification（测试规范）

unshare() 的测试应当测试以下内容：

  1) 有效flags：测试那些针对信号及信号处理函数、且尚未实现取消共享clone flags 是否返回 -EINVAL
  2) 缺失/隐含flags：测试确保在不指定取消共享文件系统时，取消共享命名空间能正确地同时取消共享命名空间和文件系统信息
  3) 对于四种受支持的取消共享（命名空间、文件系统、文件和 vm），验证系统调用能正确地取消共享相应的结构。验证分别对它们做取消共享，以及将它们彼此组合做取消共享，都能按预期工作
  4) 并发执行：使用共享内存段和基shm 段中某地址futex 来同步约 10 个线程的执行。让其中几个线程执行 execve，几个执_exit，其余的以不同的 flag 组合执行 unshare。验证取消共享按预期执行，且没有 oops 或挂起
### 9) Future Work（未来工作）

unshare() 的当前实现不允许取消共享信号和信号处理函数。信号本身就很复杂，而要取消共享一个正在运行的进程的信号和/或信号处理函数则更为复杂。如果将来有特定的需求需要允许取消共享信号和/或信号处理函数，可以在不影响使用 unshare() 的遗留应用程序的前提下，以增量的方式加入 unshare()