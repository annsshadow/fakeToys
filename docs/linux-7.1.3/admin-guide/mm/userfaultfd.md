## Userfaultfd


## 目标


用户缺页（userfaults）允许从用户空间实现按需分页，更一般地说，它们允许用户空间
控制各种内存页错误，否则这一工作只有内核代码才能完成。

例如，用户缺页允许对 `PROT_NONE+SIGSEGV` 技巧进行恰当且更优的实现。


## 设计


用户空间创建一个新的 `userfaultfd`，对其进行初始化，并注册一个或多个虚拟内存区域。
然后，在区域（们）内发生的任何页错误都会向 `userfaultfd` 投递一条消息，通知用户空间
该错误。

`userfaultfd`（除了注册和注销虚拟内存范围之外）提供两个主要功能：

1) `read/POLLIN` 协议，用于通知用户空间线程发生了错误

2) 各种 `UFFDIO_*` ioctl，可以管理在 `userfaultfd` 中注册的虚拟内存区域，允许
   用户空间高效地解决它通过 1) 接收到的用户缺页，或在后台管理虚拟内存

与 mremap/mprotect 的常规虚拟内存管理相比，用户缺页真正的优势在于：其所有操作
从不涉及像 vma 这样的重量级结构（实际上 `userfaultfd` 运行时加载从不为写操作获取
mmap_lock）。在处理可能跨越数 TB 的虚拟地址空间时，vma 不适合以页（或 hugepage）
为粒度的错误跟踪。为此需要太多的 vma。

一旦创建，`userfaultfd` 也可以使用 unix 域套接字传递给一个管理进程，这样同一个
管理进程可以处理大量不同进程的用户缺页，而它们对此毫无察觉（当然，除非它们之后
试图在管理器已在跟踪的同一区域上自己使用 `userfaultfd`，这是一个当前会返回
`-EBUSY` 的边界情况）。


## API


### 创建 userfaultfd


创建新的 `userfaultfd` 有两种方式，每种都提供了限制此功能访问的方法（因为历史上
处理内核页错误的 userfaultfd 一直是一种有用的内核利用工具）。

第一种方式自 userfaultfd 引入以来一直受支持，即 userfaultfd(2) 系统调用。对其的
访问通过以下几种方式控制：

- 任何用户始终可以创建只捕获用户空间页错误的 userfaultfd。这种 userfaultfd 可以
  使用带有 UFFD_USER_MODE_ONLY 标志的 userfaultfd(2) 系统调用来创建。

- 为了也能捕获地址空间的内核页错误，进程要么需要 CAP_SYS_PTRACE 能力，要么系统
  必须将 vm.unprivileged_userfaultfd 设置为 1。默认情况下，vm.unprivileged_userfaultfd
  被设置为 0。

第二种方式较新加入内核，是通过打开 /dev/userfaultfd 并对其进行 USERFAULTFD_IOC_NEW
ioctl 操作。这种方式产生与 userfaultfd(2) 系统调用等价的 userfaultfd。

与 userfaultfd(2) 不同，对 /dev/userfaultfd 的访问通过常规文件系统权限
（user/group/mode）控制，这可以在不同时也授予其他无关特权的情况下，对 userfaultfd
进行细粒度访问（例如授予 CAP_SYS_PTRACE 就会这样）。有权访问 /dev/userfaultfd 的
用户始终可以创建捕获内核页错误的 userfaultfd；vm.unprivileged_userfaultfd 不被
考虑。


### 初始化 userfaultfd


首次打开时，`userfaultfd` 必须被启用，即调用 `UFFDIO_API` ioctl，指定设置为
`UFFD_API`（或更高 API 版本）的 `uffdio_api.api` 值，它将指定用户空间打算在 `UFFD`
上使用的 `read/POLLIN` 协议以及用户空间需要的 `uffdio_api.features`。如果
`UFFDIO_API` ioctl 成功（即运行的核也支持所请求的 `uffdio_api.api`，且所请求的
特性将被启用），它将分别在 `uffdio_api.features` 和 `uffdio_api.ioctls` 中返回两个
64 位位掩码，分别代表 read(2) 协议的所有可用特性以及可用的通用 ioctl。

`UFFDIO_API` ioctl 返回的 `uffdio_api.features` 位掩码定义了 `userfaultfd` 支持的
内存类型，以及除页错误通知外可能生成的事件：

- `UFFD_FEATURE_EVENT_*` 标志表示支持页错误之外的各种其他事件。这些事件在下面的
  `非协作 userfaultfd`_ 一节中有更详细的描述。

- `UFFD_FEATURE_MISSING_HUGETLBFS` 和 `UFFD_FEATURE_MISSING_SHMEM` 分别表示内核
  支持针对 hugetlbfs 和共享内存（覆盖所有 shmem API，即 tmpfs、`IPCSHM`、
  `/dev/zero`、`MAP_SHARED`、`memfd_create` 等）虚拟内存区域的
  `UFFDIO_REGISTER_MODE_MISSING` 注册。

- `UFFD_FEATURE_MINOR_HUGETLBFS` 表示内核支持针对 hugetlbfs 虚拟内存区域的
  `UFFDIO_REGISTER_MODE_MINOR` 注册。`UFFD_FEATURE_MINOR_SHMEM` 是类似特性，表示
  对 shmem 虚拟内存区域的支持。

- `UFFD_FEATURE_MOVE` 表示内核支持从用户空间移动现有页内容。

用户空间应用程序在调用 `UFFDIO_API` ioctl 时，应设置在打算使用的特性标志，以请求
在支持时启用这些特性。

一旦 `userfaultfd` API 被启用，应调用 `UFFDIO_REGISTER` ioctl（如果出现在返回的
`uffdio_api.ioctls` 位掩码中），通过相应地设置 uffdio_register 结构来注册
`userfaultfd` 中的内存范围。`uffdio_register.mode` 位掩码将向内核指定对该范围要
跟踪哪种错误。`UFFDIO_REGISTER` ioctl 将返回适合解决所注册范围上用户缺页的
`uffdio_register.ioctls` ioctl 位掩码。并非所有 ioctl 都必然对所有内存类型（例如
匿名内存 vs. shmem vs. hugetlbfs）或所有类型的被拦截错误受支持。


用户空间可以使用 `uffdio_register.ioctls` 在后台管理虚拟地址空间（添加或可能移除
`userfaultfd` 注册范围中的内存）。这意味着用户缺页可能在用户空间在后台映射该
缺页页之前恰好触发。


### 解决用户缺页


有三种基本方法来解决用户缺页：

- `UFFDIO_COPY` 原子地将某些现有页内容从用户空间复制。

- `UFFDIO_ZEROPAGE` 原子地将新页清零。

- `UFFDIO_CONTINUE` 映射一个已存在、先前已填充的页。

这些操作是原子的，因为它们保证没有任何东西能看到半填充的页，因为读者会一直
触发用户缺页直到操作完成。

默认情况下，这些操作会唤醒阻塞在相关范围上的用户缺页。它们支持 `UFFDIO_*_MODE_DONTWAKE`
`mode` 标志，表示唤醒将在稍后某个时间单独进行。

选择哪个 ioctl 取决于页错误的种类，以及我们希望如何来解决它：

- 对于 `UFFDIO_REGISTER_MODE_MISSING` 错误，需要通过提供新页（`UFFDIO_COPY`）或
  映射零页（`UFFDIO_ZEROPAGE`）来解决。默认情况下，内核会为缺失错误映射零页。有了
  userfaultfd，用户空间可以在缺页线程继续之前决定提供什么内容。

- 对于 `UFFDIO_REGISTER_MODE_MINOR` 错误，存在一个现有页（在页缓存中）。用户空间
  可以选择在解决错误之前修改页内容。一旦内容正确（无论是否修改），用户空间就请求
  内核映射该页，并让缺页线程用 `UFFDIO_CONTINUE` 继续。

注意事项：

- 你可以通过检查 `uffd_msg` 中的 `pagefault.flags`，并检查 `UFFD_PAGEFAULT_FLAG_*`
  标志，来判断发生了哪种类型的错误。

- 没有哪个页投递 ioctl 默认作用于你注册的范围。你必须填写相应 ioctl 结构的所有
  字段，包括范围。

- 你从在线程中从 uffd 读取的 struct uffd_msg 中获取触发缺失页事件的访问地址。你
  可以用这些 IOCTL 提供任意多的页。请记住，除非你使用了 DONTWAKE，否则任何这些
  IOCTL 中的第一个都会唤醒缺页线程。

- 务必测试所有错误，包括 (`pollfd[^0^].revents & POLLERR`)。这可能发生，例如当
  提供的范围不正确时。


### 写保护通知


这等同于（但快于）使用 mprotect 和 SIGSEGV 信号处理程序。

首先，你需要用 `UFFDIO_REGISTER_MODE_WP` 注册一个范围。你使用
`ioctl(uffd, UFFDIO_WRITEPROTECT, struct *uffdio_writeprotect)`，同时传入结构中
`mode = UFFDIO_WRITEPROTECT_MODE_WP`，而不是使用 mprotect(2)。该范围不默认为你注册
的范围，也不必与之相同。你可以写保护任意多个范围（在注册范围内）。然后，在从 uffd
读取的线程中，结构将具有 `msg.arg.pagefault.flags & UFFD_PAGEFAULT_FLAG_WP` 置位。
现在你再次发送 `ioctl(uffd, UFFDIO_WRITEPROTECT, struct *uffdio_writeprotect)`，而
`pagefault.mode` 没有设置 `UFFDIO_WRITEPROTECT_MODE_WP`。这会唤醒线程，该线程将带着
写操作继续运行。这允许你在 ioctl 之前在 uffd 读取线程中做关于该写操作的记账。

如果你同时用 `UFFDIO_REGISTER_MODE_MISSING` 和 `UFFDIO_REGISTER_MODE_WP` 注册，则
你需要考虑提供页与撤销写保护之间的顺序。注意对 WP 区域和 !WP 区域的写入之间存在
差异。前者将置位 `UFFD_PAGEFAULT_FLAG_WP`，后者置位 `UFFD_PAGEFAULT_FLAG_WRITE`。
后者并非因保护而失败，但在使用 `UFFDIO_REGISTER_MODE_MISSING` 时你仍然需要提供一个页。

Userfaultfd 写保护模式目前在不同类型内存上对 none pte（例如页缺失时）的行为不同。

对于匿名内存，`ioctl(UFFDIO_WRITEPROTECT)` 会忽略 none pte（例如当页缺失且未填充
时）。对于 shmem 和 hugetlbfs 等文件后备内存，none pte 将像 present pte 一样被写
保护。换句话说，只要在写缺页页时对文件类型内存设置了写保护，就会生成一条
userfaultfd 写错误消息。默认情况下，匿名内存上不会生成这样的消息。

如果应用程序希望对匿名内存上的 none pte 进行写保护，可以预先用例如
MADV_POPULATE_READ 填充内存。在较新的内核上，也可以检测 UFFD_FEATURE_WP_UNPOPULATED
特性并提前设置特性位，以确保即便对于匿名内存，none pte 也会被写保护。

当将 `UFFDIO_REGISTER_MODE_WP` 与 `UFFDIO_REGISTER_MODE_MISSING` 或
`UFFDIO_REGISTER_MODE_MINOR` 结合使用时，在用 `UFFDIO_COPY` 或 `UFFDIO_CONTINUE`
分别解决缺失/次要错误时，可能希望新页/映射被写保护（以便未来的写操作也会导致 WP
错误）。这些 ioctl 支持一个 mode 标志（分别为 `UFFDIO_COPY_MODE_WP` 或
`UFFDIO_CONTINUE_MODE_WP`）来以这种方式配置映射。

如果 userfaultfd 上下文设置了 `UFFD_FEATURE_WP_ASYNC` 特性位，任何以写保护注册的
vma 将以异步模式工作，而不是默认的同步模式。

在异步模式下，发生写操作时不会生成消息，同时写保护将由内核自动解决。它可以被视为
soft-dirty 跟踪的更精确版本，并且在几个方面可能有所不同：

  - 脏结果不受 vma 变化（例如 vma 合并）的影响，因为脏只由 pte 跟踪。

  - 默认支持范围操作，因此只要页对齐，就可以在任意内存范围上启用跟踪。

  - 如果 pte 由于各种原因（例如在 shmem 透明大页拆分期间）被清除，脏信息不会丢失。

  - 由于 soft-dirty 含义的反转（设置 uffd-wp 位时页干净；清除 uffd-wp 位时脏），
    它在某些内存操作上具有不同的语义。例如：匿名内存上的 `MADV_DONTNEED`（或文件
    映射上的 `MADV_REMOVE`）在过程中通过丢弃 uffd-wp 位而被当作内存的弄脏。

用户应用可以通过在 /proc/pagemap 中查找感兴趣的页的 uffd-wp 位来收集"已写/脏"
状态。

在页被 `ioctl(UFFDIO_WRITEPROTECT)` 显式写保护（设置 mode 标志
`UFFDIO_WRITEPROTECT_MODE_WP`）之前，该页不会处于 uffd-wp 异步模式的跟踪之下。尝试
解决由异步模式 userfaultfd-wp 跟踪的页错误是无效的。

当 userfaultfd-wp 异步模式单独使用时，它可以应用于所有类型的内存。


### 内存中毒模拟


作为对错误（缺失或次要）的响应，用户空间可以采取的一个"解决"动作是发出
`UFFDIO_POISON`。这将导致任何未来的触发者收到 SIGBUS，或者在 KVM 的情况下，客户机
将收到一个 MCE，就像发生了硬件内存中毒一样。

这用于模拟硬件内存中毒。想象一个运行在经历过真实硬件内存错误的机器上的 VM。随后，
我们将 VM 实时迁移到另一台物理机器。由于我们想让迁移对客户机透明，我们希望同一地址
范围表现得就像它仍然被毒化一样，即使它位于一台新的物理主机上，而该主机在完全相同的
位置显然并没有内存错误。


## QEMU/KVM


QEMU/KVM 正在使用 `userfaultfd` 系统调用来实现 postcopy 实时迁移。Postcopy 实时迁移
是内存外部化的一种形式，由一个部分或全部内存驻留在云中不同节点上的虚拟机运行组成。
`userfaultfd` 抽象足够通用，以至于无需修改 KVM 内核代码的一行即可将 postcopy 实时
迁移添加到 QEMU。

客户机异步页错误、`FOLL_NOWAIT` 以及所有其他 `GUP*` 特性与用户缺页结合使用完全没有
问题。用户缺页在客户机调度器中触发异步页错误，因此那些没有在等待用户缺页（即受网络
限制）的客户机进程可以在客户机 vCPU 中继续运行。

通常有益的做法是在开始 postcopy 实时迁移之前先运行一轮 precopy 实时迁移，以避免为
只读客户机区域生成用户缺页。

postcopy 实时迁移的当前实现使用一个单一的双向套接字，但将来会使用两个不同的套接字
（以在不必减小 `/proc/sys/net/ipv4/tcp_wmem` 的情况下将用户缺页的延迟降到最低）。

源节点中的 QEMU 将它知道在目标节点中缺失的所有页写入套接字，而运行在目标节点中的
QEMU 的迁移线程在 `userfaultfd` 上运行 `UFFDIO_COPY|ZEROPAGE` ioctl，以便将收到的页
映射到客户机（`UFFDIO_ZEROCOPY` 用于源页是零页的情况）。

目标节点中另一个 postcopy 线程用 poll() 并行监听 `userfaultfd`。当在用户缺页触发后
生成 `POLLIN` 事件时，postcopy 线程从 `userfaultfd` 执行 read() 并接收错误地址（或
`-EAGAIN`，如果用户在并行 QEMU 迁移线程运行的 `UFFDIO_COPY|ZEROPAGE` 解析并唤醒之后
才被读取）。

在目标节点运行的 QEMU postcopy 线程获得用户缺页地址后，它将关于缺失页的信息写入
套接字。QEMU 源节点接收该信息，并大致"寻找"到该页地址，并继续从该新页偏移发送所有
剩余缺失页。此后不久（只需将 tcp_wmem 队列通过网络冲刷的时间），目标节点运行的 QEMU
中的迁移线程将收到触发用户缺页的那个页，并像往常一样用 `UFFDIO_COPY|ZEROPAGE` 映射
它（而实际上并不知道它是源节点自发发送的，还是通过用户缺页请求的紧急页）。

到用户缺页开始的时候，目标节点中的 QEMU 不需要保留任何与实时迁移相关的每页状态
位图，而源节点中运行的 QEMU 必须维护一个单一的每页位图，以知道哪些页在目标节点中
仍然缺失。检查源节点中的位图以找到要按轮询发送的缺失页，并在接收传入用户缺页时在其上
查找。当然，发送每页之后位图会相应更新。它还有助于避免发送同一页两次（以防 postcopy
线程在迁移线程中的 `UFFDIO_COPY|ZEROPAGE` 运行之前刚好读取了用户缺页）。


## 非协作 userfaultfd


当 `userfaultfd` 由外部管理器监控时，管理器必须能够跟踪进程虚拟内存布局的变化。
Userfaultfd 可以使用与页错误通知相同的 read(2) 协议将这些变化通知管理器。管理器
必须通过设置在传递给 `UFFDIO_API` ioctl 的 `uffdio_api.features` 中的相应位来显式
启用这些事件：

`UFFD_FEATURE_EVENT_FORK`
	为 fork() 启用 `userfaultfd` 钩子。启用此特性后，父进程的 `userfaultfd`
	上下文会被复制到新创建的进程中。管理器在 `uffd_msg.fork` 中收到带有新
	`userfaultfd` 上下文文件描述符的 `UFFD_EVENT_FORK`。

`UFFD_FEATURE_EVENT_REMAP`
	启用关于 mremap() 调用的通知。当非协作进程将虚拟内存区域移动到不同
	位置时，管理器将收到 `UFFD_EVENT_REMAP`。`uffd_msg.remap` 将包含该区域的
	新旧地址及其原始长度。

`UFFD_FEATURE_EVENT_REMOVE`
	启用关于 madvise(MADV_REMOVE) 和 madvise(MADV_DONTNEED) 调用的通知。对这些
	madvise() 调用会生成 `UFFD_EVENT_REMOVE` 事件。`uffd_msg.remove` 将包含被
	移除区域的起始和结束地址。

`UFFD_FEATURE_EVENT_UNMAP`
	启用关于内存取消映射的通知。管理器将收到 `UFFD_EVENT_UNMAP`，其中
	`uffd_msg.remove` 包含被取消映射区域的起始和结束地址。

尽管 `UFFD_FEATURE_EVENT_REMOVE` 和 `UFFD_FEATURE_EVENT_UNMAP` 非常相似，但它们在
`userfaultfd` 管理器预期的动作上差别很大。在前一种情况下，虚拟内存被移除，但区域
还在，该区域仍由 `userfaultfd` 监控，如果在该区域发生页错误，它将被投递给管理器。
对此类页错误的恰当解决是对错误地址进行 zeromap。然而，在后一种情况下，当区域被取消
映射时，无论是显式地（通过 munmap() 系统调用）还是隐式地（例如在 mremap() 期间），
区域被移除，进而该区域的 `userfaultfd` 上下文也随之消失，管理器将不再从该被移除的
区域收到进一步的用户空间页错误。不过，仍然需要该通知，以防止管理器在被取消映射的
区域上使用 `UFFDIO_COPY`。

与必须同步并需要显式或隐式唤醒的用户空间页错误不同，所有事件都是异步投递的，一旦
管理器执行 read()，非协作进程就恢复执行。`userfaultfd` 管理器应仔细地将对
`UFFDIO_COPY` 的调用与事件处理同步。为了辅助同步，`UFFDIO_COPY` ioctl 在监控的进程
在 `UFFDIO_COPY` 时刻退出时返回 `-ENOSPC`，而当非协作进程在 `UFFDIO_COPY` 操作进行
的同时改变了其虚拟内存布局时返回 `-ENOENT`。

当前的事件投递异步模型对于单线程非协作 `userfaultfd` 管理器实现是最优的。同步事件
投递模型以后可以作为一个新的 `userfaultfd` 特性添加，以促进非协作管理器的多线程增强，
例如允许 `UFFDIO_COPY` ioctl 与事件接收并行运行。单线程实现应该继续使用当前的异步
事件投递模型。
