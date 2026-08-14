
## FUSE 概述


## 定义


用户空间文件系统：
  数据与元数据由普通用户空间进程提供的文件系统。该文件系统可以像平常一样通过内核接口访问。

文件系统守护进程：
  提供文件系统数据与元数据的进程。

非特权挂载（或用户挂载）：
  由非特权（非 root）用户挂载的用户空间文件系统。文件系统守护进程以挂载用户的权限运行。注意：这与 /etc/fstab 中允许带 "user" 选项的挂载不同，本文不讨论后者。

文件系统连接：
  文件系统守护进程与内核之间的连接。该连接一直存在，直到守护进程死亡或文件系统被卸载。注意，分离（或惰性卸载）文件系统**不会**断开连接，这种情况下它将一直存在，直到对文件系统的最后一个引用被释放。

挂载者：
  执行挂载的用户。

用户：
  正在执行文件系统操作的用户。

## 什么是 FUSE？


FUSE 是一个用户空间文件系统框架。它由一个内核模块（fuse.ko）、一个用户空间库（libfuse.*）和一个挂载工具（fusermount）组成。

FUSE 最重要的特性之一是允许安全的非特权挂载。这为文件系统的使用开辟了新的可能。一个很好的例子是 sshfs：一个使用 sftp 协议的安全网络文件系统。

用户空间库与工具可从 `FUSE 主页：<https://github.com/libfuse/>`_ 获取

## 文件系统类型


传给 mount(2) 的文件系统类型可以是以下之一：

    fuse
      这是挂载 FUSE 文件系统的常规方式。mount 系统调用的第一个参数可包含任意字符串，内核不会解释它。

    fuseblk
      该文件系统基于块设备。mount 系统调用的第一个参数被解释为设备名。

## 挂载选项


fd=N
  用于用户空间文件系统与内核之间通信的文件描述符。该文件描述符必须是通过打开 FUSE 设备（'/dev/fuse'）获得的。

rootmode=M
  文件系统根目录的文件模式，以八进制表示。

user_id=N
  挂载者的数字用户 ID。

group_id=N
  挂载者的数字组 ID。

default_permissions
  默认情况下 FUSE 不检查文件访问权限，文件系统可自行实现其访问策略，或留给底层文件访问机制（例如在网络文件系统的情况下）。此选项启用权限检查，基于文件模式限制访问。它通常与 'allow_other' 挂载选项一起使用。

allow_other
  此选项覆盖了将文件访问限制为挂载用户的安全措施。默认情况下此选项仅允许 root 使用，但可以通过一个（用户空间）配置选项移除该限制。

max_read=N
  通过该选项可设置读取操作的最大大小。默认是无限。注意，读取请求的大小无论如何都限制为 32 页（在 i386 上为 128k 字节）。

blksize=N
  设置文件系统的块大小。默认是 512。此选项仅对 'fuseblk' 类型挂载有效。

## 控制文件系统


```

  mount -t fusectl none /sys/fs/fuse/connections

```
将其挂载到 '/sys/fs/fuse/connections' 目录下，可与早期版本保持向后兼容。

在 FUSE 控制文件系统下，每个连接都有一个以唯一编号命名的目录。

每个连接在该目录中存在以下文件：

	waiting
	  正在等待传送到用户空间或由文件系统守护进程处理的请求数。如果没有文件系统活动且 'waiting' 非零，则文件系统已挂起或死锁。

	abort
	  向此文件写入任何内容都会中止文件系统连接。这意味着所有等待中的请求将被中止，所有已中止和新请求都会返回错误。

        max_background
          一次可以 outstanding 的最大后台请求数。当后台请求数达到此限制时，进一步的请求将被阻塞，直到部分完成，可能导致 I/O 操作停滞。

        congestion_threshold
          内核认为文件系统拥塞的后台请求阈值。当后台请求数超过此值时，内核将跳过异步预读操作，减少预读优化但保留必要的 I/O，同时暂停非同步回写操作（WB_SYNC_NONE），延迟向文件系统刷新页缓存。

只有挂载的所有者可以读取或写入这些文件。

# 中断文件系统操作


如果发出 FUSE 文件系统请求的进程被中断，将发生以下情况：

  - 如果请求尚未发送到用户空间，且信号是致命的（SIGKILL 或未被处理的致命信号），则请求被出队并立即返回。

  - 如果请求尚未发送到用户空间，且信号并非致命，则为该请求设置一个中断标志。当请求已成功传送到用户空间且此标志已设置时，将排队一个 INTERRUPT 请求。

  - 如果请求已发送到用户空间，则排队一个 INTERRUPT 请求。

INTERRUPT 请求优先于其他请求，因此用户空间文件系统会在其他任何请求之前收到排队的 INTERRUPT。

用户空间文件系统可以完全忽略 INTERRUPT 请求，也可以通过向**原始**请求发送回复（错误设为 EINTR）来满足它们。

处理原始请求与其 INTERRUPT 请求之间也可能存在竞争。有两种可能：

  1. INTERRUPT 请求在原始请求之前被处理

  2. INTERRUPT 请求在原始请求已被应答之后被处理

如果文件系统找不到原始请求，它应等待某个超时和/或若干个新请求到达，之后以 EAGAIN 错误回复 INTERRUPT 请求。在情况 1) 下，INTERRUPT 请求将被重新排队。在情况 2) 下，INTERRUPT 回复将被忽略。

## 中止文件系统连接


可能会陷入文件系统无响应的某些情况。原因可能是：

  a) 损坏的用户空间文件系统实现

  b) 网络连接中断

  c) 意外死锁

  d) 恶意死锁

（关于 c) 和 d) 的更多内容见后续章节）

在这些情况下，中止到文件系统的连接可能很有用。有几种方法可以做到：

  - 杀死文件系统守护进程。在 a) 和 b) 情况下有效

  - 杀死文件系统守护进程以及文件系统的所有使用者。在所有情况下都有效，除了某些恶意死锁

  - 使用强制卸载（umount -f）。在所有情况下都有效，但仅当文件系统仍被挂载时（尚未惰性卸载）

  - 通过 FUSE 控制文件系统中止文件系统。最强大的方法，总是有效。

## 非特权挂载如何工作？


由于 mount() 系统调用是特权操作，需要一个辅助程序（fusermount），它被安装为 setuid root。

提供非特权挂载意味着挂载者不能利用此能力来危害系统。由此产生的明显要求是：

 A) 挂载者不应能借助所挂载的文件系统获得提升的权限

 B) 挂载者不应非法获取其他用户及超级用户进程的信息

 C) 挂载者不应能在其他用户或超级用户的进程中引发非预期行为

## 需求如何满足？


 A) 挂载者可能通过以下任一种方式获得提升的权限：

    1. 创建包含设备文件的文件系统，然后打开该设备

    2. 创建包含 suid 或 sgid 应用的文件系统，然后执行该应用

    解决方案是不允许打开设备文件，并在执行程序时忽略 setuid 和 setgid 位。为确保这一点，fusermount 总是为非特权挂载在挂载选项中加上 "nosuid" 和 "nodev"。

 B) 如果另一个用户正在访问文件系统中的文件或目录，提供服务的文件系统守护进程可以记录所执行操作的确切顺序和时机。这些信息对挂载者而言原本无法获取，因此这算作信息泄露。

    此问题的解决方案将在 C) 的第 2) 点中给出。

 C) 挂载者可以通过几种方式在其他用户的进程中引发非预期行为，例如：

     1) 在挂载者原本无法修改（或只能做有限修改）的文件或目录上挂载文件系统。

        这在 fusermount 中通过检查挂载点的访问权限来解决，仅当挂载者可以做无限修改（对挂载点有写访问权限，且挂载点不是"sticky"目录）时才允许挂载。

     2) 即便 1) 已解决，挂载者仍能改变其他用户进程的行为。

         i) 它可以拖慢或无限延迟文件系统操作的执行，对用户或整个系统造成 DoS。例如，一个 suid 应用锁定系统文件，然后访问挂载者文件系统上的文件，可能被阻止，从而导致系统文件被永久锁定。

         ii) 它可以呈现长度无限的文件或目录，或深度无限的目录结构，可能导致系统进程耗尽磁盘空间、内存或其他资源，再次造成 **DoS**。

	对此以及 B) 的解决方案，是不允许那些挂载者原本无法监控或操纵的进程访问文件系统。因为若挂载者能 ptrace 一个进程，它无需使用 FUSE 挂载就能做到上述所有事情，所以可以使用 ptrace 中采用的相同标准来检查进程是否被允许访问文件系统。

	注意，**ptrace** 检查对于防范 C/2/i 并非严格必需，检查挂载者是否有足够权限向访问文件系统的进程发送信号就足够了，因为 **SIGSTOP** 可以产生类似效果。

## 我认为这些限制不可接受？


如果系统管理员足够信任用户，或能通过其他措施确保系统进程永远不会进入非特权挂载，它可以通过以下几种方式放宽最后一项限制：

  - 通过 'user_allow_other' 配置选项。如果设置了此配置选项，挂载用户可以添加 'allow_other' 挂载选项，从而禁用对其他用户进程的检查。

    用户命名空间与 'allow_other' 存在不直观的交互：一个非特权用户——通常被禁止以 'allow_other' 挂载——可以在它拥有特权的用户命名空间中这样做。如果任何进程都能访问这样的 'allow_other' 挂载，就会让挂载用户有能力操纵它在其中非特权的用户命名空间里的进程。出于这个原因，'allow_other' 将访问限制在同一用户命名空间或其后代中的用户。

  - 通过 'allow_sys_admin_access' 模块选项。如果设置了此选项，超级用户的进程对挂载拥有不受限制的访问权，无论 allow_other 设置或挂载用户的用户命名空间如何。

注意，这两种放宽都会使系统面临前一节 B 和 C/2/i-ii 点所描述的潜在信息泄露或 **DoS**。

## 内核 - 用户空间接口


以下示意图展示了一次文件系统操作（在本例中为 `rm /mnt/fuse/file`）在内核与用户空间之间的流程：
```

 |  "rm /mnt/fuse/file"               |  FUSE filesystem daemon
 |                                    |
 |                                    |  >sys_read()
 |                                    |    >fuse_dev_read()
 |                                    |      >request_wait()
 |                                    |        [sleep on fc->waitq]
 |                                    |
 |  >sys_unlink()                     |
 |    >fuse_unlink()                  |
 |      [get request from             |
 |       fc->unused_list]             |
 |      >request_send()               |
 |        [queue req on fc->pending]  |
 |        [wake up fc->waitq]         |        [woken up]
 |        >request_wait_answer()      |
 |          [sleep on req->waitq]     |
 |                                    |      <request_wait()
 |                                    |      [remove req from fc->pending]
 |                                    |      [copy req to read buffer]
 |                                    |      [add req to fc->processing]
 |                                    |    <fuse_dev_read()
 |                                    |  <sys_read()
 |                                    |
 |                                    |  [perform unlink]
 |                                    |
 |                                    |  >sys_write()
 |                                    |    >fuse_dev_write()
 |                                    |      [look up req in fc->processing]
 |                                    |      [remove from fc->processing]
 |                                    |      [copy write buffer to req]
 |          [woken up]                |      [wake up req->waitq]
 |                                    |    <fuse_dev_write()
 |                                    |  <sys_write()
 |        <request_wait_answer()      |
 |      <request_send()               |
 |      [add request to               |
 |       fc->unused_list]             |
 |    <fuse_unlink()                  |
 |  <sys_unlink()                     |

```

有几种方式会让 FUSE 文件系统陷入死锁。既然我们讨论的是非特权用户空间程序，必须针对这些做些处理。
```

 |  "rm /mnt/fuse/file"               |  FUSE filesystem daemon
 |                                    |
 |  >sys_unlink("/mnt/fuse/file")     |
 |    [acquire inode semaphore        |
 |     for "file"]                    |
 |    >fuse_unlink()                  |
 |      [sleep on req->waitq]         |
 |                                    |  <sys_read()
 |                                    |  >sys_unlink("/mnt/fuse/file")
 |                                    |    [acquire inode semaphore
 |                                    |     for "file"]
 |                                    |    *DEADLOCK*

```
对此的解决方案是允许中止文件系统。

**场景 2 - 棘手的死锁**


这个需要精心构造的文件系统。它是上述情况的一个变体，只是回调文件系统的方式不是显式的，
```

 |  Kamikaze filesystem thread 1      |  Kamikaze filesystem thread 2
 |                                    |
 |  [fd = open("/mnt/fuse/file")]     |  [request served normally]
 |  [mmap fd to 'addr']               |
 |  [close fd]                        |  [FLUSH triggers 'magic' flag]
 |  [read a byte from addr]           |
 |    >do_page_fault()                |
 |      [find or create page]         |
 |      [lock page]                   |
 |      >fuse_readpage()              |
 |         [queue READ request]       |
 |         [sleep on req->waitq]      |
 |                                    |  [read request to buffer]
 |                                    |  [create reply header before addr]
 |                                    |  >sys_write(addr - headerlength)
 |                                    |    >fuse_dev_write()
 |                                    |      [look up req in fc->processing]
 |                                    |      [remove from fc->processing]
 |                                    |      [copy write buffer to req]
 |                                    |        >do_page_fault()
 |                                    |           [find or create page]
 |                                    |           [lock page]
 |                                    |           * DEADLOCK *

```
其解决方案与上述基本相同。

另一个问题是，当写缓冲区正在被复制到请求时，该请求不能被中断/中止。这是因为复制的目标地址在请求返回后可能不再有效。

这通过以原子方式执行复制，并允许在属于写缓冲区的页通过 get_user_pages() 发生缺页时中止来解决。'req->locked' 标志指示复制何时发生，中止会延迟到该标志被清除。
