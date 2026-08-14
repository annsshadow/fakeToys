
## relay 接口（原 relayfs）


relay 接口为内核应用程序提供了一种手段，用于通过用户自定义的“relay 通道”（relay channel）高效地将大量数据从内核记录并传输到用户空间。

“relay 通道”是一种内核->用户的数据中继机制，实现为一组每 CPU 的内核缓冲区（“通道缓冲区”，channel buffer），每个缓冲区在用户空间中表现为一个常规文件（“relay 文件”，relay file）。内核客户端使用高效的写入函数写入通道缓冲区；这些函数会自动记录到当前 CPU 的通道缓冲区中。用户空间应用程序对 relay 文件执行 mmap() 或 read() 并在数据就绪时取回数据。relay 文件本身是创建于宿主文件系统（例如 debugfs）中的文件，并通过下文描述的 API 与通道缓冲区关联。

记录到通道缓冲区中的数据的格式完全由内核客户端决定；不过 relay 接口确实提供了一些钩子（hook），允许内核客户端对缓冲区数据施加某种结构。relay 接口不实现任何形式的数据过滤——这也留给内核客户端处理。其目的是尽可能让事情保持简单。

本文档概述了 relay 接口的 API。函数参数的详细信息与 relay 接口代码中的函数一同记录在案——详情请参阅代码。

## 语义


每个 relay 通道为每个 CPU 准备一个缓冲区；每个缓冲区有一个或多个子缓冲区（sub-buffer）。消息被写入第一个子缓冲区，直到它太满而无法容纳新消息，此时会写入下一个（如果可用）。消息绝不会被拆分到多个子缓冲区中。

此时，可以通知用户空间，由它清空第一个子缓冲区，而内核继续写入下一个子缓冲区。

当被告知某个子缓冲区已满时，内核知道其中有多少字节是填充（padding），即由于一个完整的消息无法放入子缓冲区而产生的未使用空间。用户空间可以利用这一信息只复制有效数据。

复制之后，用户空间可以通知内核某个子缓冲区已被消费（consumed）。

relay 通道可以运行在一种模式下：它会覆盖尚未被用户空间收集的数据，而不等待其被消费。

relay 通道本身不提供用户空间与内核之间此类数据的通信，从而让内核侧保持简单，并且不对用户空间强加单一接口。不过它确实提供了一组示例以及一个独立的辅助工具，如下文所述。

read() 接口既会移除填充，也会在内部消费已读取的子缓冲区；因此在用 read(2) 排空通道缓冲区的情况下，基本操作无需内核与用户之间的专用通信。

relay 接口的主要目标之一，是提供一种低开销的机制，将内核数据传送到用户空间。虽然 read() 接口易于使用，但它不如 mmap() 方式高效；示例代码试图让这两种方式之间的权衡尽可能小。

## klog 与 relay-apps 示例代码


relay 接口本身已可直接使用，但为了让事情更容易，提供了一对简单的工具函数以及一组示例。

relay-apps 示例压缩包（tarball）可在 relay 的 sourceforge 站点获取，其中包含一组自包含的示例，每个示例由一对 .c 文件组成，分别包含 relay 应用程序用户侧与内核侧的样板代码。将这两组样板代码组合在一起后，便提供了将数据轻松流式写入磁盘的“粘合剂”，而无需操心琐碎的内务管理工作。

“klog 调试函数”补丁（relay-apps 压缩包中的 klog.patch）向内核提供了一对高层日志函数，允许将格式化文本或原始数据写入一个通道，无论是否存在可写入的通道，甚至无论 relay 接口是否被编译进内核。这些函数允许你在内核或内核模块的任何位置放置无条件的“trace”语句；只有当注册了一个“klog 处理器”（klog handler）时，数据才会真正被记录（详见 klog 与 kleak 示例）。

当然也可以从零开始使用 relay 接口，即不使用任何 relay-apps 示例代码或 klog，但你必须实现用户空间与内核之间的通信，使双方都能传达缓冲区的状态（满、空、填充量）。read() 接口既会移除填充，也会在内部消费已读取的子缓冲区；因此在用 read(2) 排空通道缓冲区的情况下，基本操作无需内核与用户之间的专用通信。不过缓冲区满之类的状况仍需要通过某种通道来通信。

klog 与 relay-apps 示例可在 relay-apps 压缩包中找到，地址为 http://relayfs.sourceforge.net

## relay 接口用户空间 API


relay 接口实现了基本的文件操作，供用户空间访问 relay 通道缓冲区数据。以下是可用的文件操作及其行为的一些说明：

=========== ============================================================
open()	    允许用户打开一个_已存在_的通道缓冲区。

mmap()      将通道缓冲区映射到调用者的内存空间中。注意不能做部分映射
	    （partial mmap）——必须映射整个文件，其大小为
	    NRBUF * SUBBUFSIZE。

read()      读取通道缓冲区的内容。被读取的字节会被读取者“消费”
	    （consumed），即后续读取不会再得到它们。如果通道以
	    no-overwrite 模式（默认）使用，则即使存在活跃的写者
	    也可以随时读取。如果通道以 overwrite 模式使用且存在
	    活跃的通道写者，结果可能不可预测——用户应确保在使用
	    read() 的 overwrite 模式之前，所有向该通道的日志写入
	    都已结束。子缓冲区（sub-buffer）的填充会被自动移除，
	    读取者不会看到。

sendfile()  将数据从通道缓冲区传输到一个输出文件描述符。子缓冲区的
	    填充会被自动移除，读取者不会看到。

poll()      支持 POLLIN/POLLRDNORM/POLLERR。当跨越子缓冲区边界时会
	    通知用户应用程序。

close()     递减通道缓冲区的引用计数（refcount）。当引用计数降为 0，
	    即没有任何进程或内核客户端打开该缓冲区时，通道缓冲区
	    被释放。
=========== ============================================================

为了使用户应用程序能够利用 relay 文件，需

```

	mount -t debugfs debugfs /sys/kernel/debug

```

	宿主文件系统无需挂载，内核客户端就能创建或使用通道——只有
	当用户空间应用程序需要访问缓冲区数据时，才需要挂载它。


## relay 接口内核 API


以下是 relay 接口向内核内客户端提供的 API 的摘要：

TBD(curr. line MT:/API/)
```

    relay_open(base_filename, parent, subbuf_size, n_subbufs,
               callbacks, private_data)
    relay_close(chan)
    relay_flush(chan)
    relay_reset(chan)

  channel management typically called on instigation of userspace::

    relay_subbufs_consumed(chan, cpu, subbufs_consumed)

  write functions::

    relay_write(chan, data, length)
    __relay_write(chan, data, length)
    relay_reserve(chan, length)

  callbacks::

    subbuf_start(buf, subbuf, prev_subbuf, prev_padding)
    buf_mapped(buf, filp)
    buf_unmapped(buf, filp)
    create_buf_file(filename, parent, mode, buf, is_global)
    remove_buf_file(dentry)

  helper functions::

    relay_buf_full(buf)
    subbuf_start_reserve(buf, length)


```
### 创建通道


relay_open() 用于创建一个通道及其每 CPU 的通道缓冲区。每个通道缓冲区都会在宿主文件系统中创建一个关联的文件，用户空间可以对该文件执行 mmap 或读取。文件名形如 basename0...basenameN-1，其中 N 为在线 CPU 的数量，默认情况下创建于文件系统的根目录（如果 parent 参数为 NULL）。如果你希望用一个目录结构来包含你的 relay 文件，应当使用宿主文件系统的目录创建函数（例如 debugfs_create_dir()）创建它，并将父目录传递给 relay_open()。当通道关闭时，用户负责清理他们所创建的任何目录结构——同样应使用宿主文件系统的目录移除函数，例如 debugfs_remove()。

为了使一个通道被创建，并使其通道缓冲区关联的宿主文件系统文件就位，用户必须为两个回调函数 create_buf_file() 和 remove_buf_file() 提供定义。对于每个每 CPU 缓冲区，relay_open() 会调用一次 create_buf_file()，使用户能够创建用于表示相应通道缓冲区的文件。该回调应返回所创建的、用于表示通道缓冲区的文件的 dentry。remove_buf_file() 也必须被定义；它负责删除在 create_buf_file() 中创建的文件，并在 relay_close() 期间被调用。

以下是这些回调的一些典型定义，在本例中

```

    /*
    * create_buf_file() callback.  Creates relay file in debugfs.
    */
    static struct dentry *create_buf_file_handler(const char *filename,
						struct dentry *parent,
						umode_t mode,
						struct rchan_buf *buf,
						int *is_global)
    {
	    return debugfs_create_file(filename, mode, parent, buf,
				    &relay_file_operations);
    }

    /*
    * remove_buf_file() callback.  Removes relay file from debugfs.
    */
    static int remove_buf_file_handler(struct dentry *dentry)
    {
	    debugfs_remove(dentry);

	    return 0;
    }

    /*
    * relay interface callbacks
    */
    static struct rchan_callbacks relay_callbacks =
    {
	    .create_buf_file = create_buf_file_handler,
	    .remove_buf_file = remove_buf_file_handler,
    };

```
```

  chan = relay_open("cpu", NULL, SUBBUF_SIZE, N_SUBBUFS, &relay_callbacks, NULL);

```
如果 create_buf_file() 回调失败，或未定义，则通道创建以及随之的 relay_open() 都会失败。

每个每 CPU 缓冲区的总大小，由子缓冲区数量乘以传入 relay_open() 的子缓冲区大小计算得出。子缓冲区背后的思想是，它们本质上是将双缓冲（double-buffering）扩展到 N 个缓冲区，并且它们还允许应用程序轻松实现“缓冲区边界上的随机访问”方案，这对某些高吞吐量的应用很重要。子缓冲区的数量与大小完全取决于应用程序，即便对于同一个应用程序，在不同条件下也需要在不同时刻为这些参数取不同的值。通常，要使用何种正确的值最好在经过一些实验后决定；不过一般而言，可以安全地假设只使用 1 个子缓冲区是个糟糕的主意——根据你使用的通道模式，你注定要么覆盖数据，要么丢失事件。

create_buf_file() 的实现也可以定义为允许创建一个单一的“全局”（global）缓冲区，而不是默认的每 CPU 集合。这对于主要关心查看系统范围事件的相对顺序、而无需费心保存显式时间戳以便在后处理步骤中合并/排序每 CPU 文件的应用程序很有用。

要让 relay_open() 创建一个全局缓冲区，create_buf_file() 的实现除了创建用于表示单一缓冲区的文件外，还应将 is_global 输出参数的值设为非零值。对于全局缓冲区，create_buf_file() 和 remove_buf_file() 将只被调用一次。普通的通道写入函数（例如 relay_write()）仍然可以使用——来自任何 CPU 的写入都会透明地进入全局缓冲区——但由于它是全局缓冲区，调用者应当确保对此类缓冲区使用适当的加锁，要么将写入包裹在自旋锁（spinlock）中，要么从 relay.h 复制一个写入函数并创建一个在内部进行适当加锁的本地版本。

传入 relay_open() 的 private_data 允许客户端将用户自定义的数据与一个通道关联起来，并且可以立即通过 chan->private_data 或 buf->chan->private_data 获取（在 create_buf_file() 中亦可）。

### 通道“模式”


relay 通道可以在两种模式之一下使用——“overwrite”（覆盖）或“no-overwrite”（不覆盖）。该模式完全由 subbuf_start() 回调的实现决定，如下所述。如果未定义 subbuf_start() 回调，默认模式为“no-overwrite”。如果默认模式满足你的需要，并且你打算使用 read() 接口来取回通道数据，你可以忽略本节细节，因为它主要关乎 mmap() 实现。

在“overwrite”模式（也称“飞行记录器”，flight recorder 模式）下，写入会持续在缓冲区中循环，并且永远不会失败，但会无条件覆盖旧数据，无论它是否真的已被消费。在 no-overwrite 模式下，如果未消费的子缓冲区数量等于通道中总的子缓冲区数量，写入就会失败，即数据会丢失。应当清楚的是，如果没有消费者，或者消费者无法足够快地消费子缓冲区，那么在两种情况下数据都会丢失；唯一的区别在于数据是从缓冲区开头还是末尾丢失。

如上所述，一个 relay 通道由一个或多个每 CPU 通道缓冲区组成，每个缓冲区实现为一个环形缓冲区（circular buffer），被细分为一个或多个子缓冲区。消息通过下文描述的写入函数，写入通道当前每 CPU 缓冲区的当前子缓冲区。每当一条消息无法放入当前子缓冲区（因为没有剩余空间）时，客户端会通过 subbuf_start() 回调被告知即将切换到新的子缓冲区。客户端使用该回调来 1) 在适当时初始化下一个子缓冲区，2) 在适当时终结前一个子缓冲区，3) 返回一个布尔值，指示是否确实要前进到下一个子缓冲区。

要实现“no-overwrite”模式，用户空间客户端提供的 subbuf_start() 回调实现类似如下

```

    static int subbuf_start(struct rchan_buf *buf,
			    void *subbuf,
			    void *prev_subbuf,
			    unsigned int prev_padding)
    {
	    if (prev_subbuf)
		    *((unsigned *)prev_subbuf) = prev_padding;

	    if (relay_buf_full(buf))
		    return 0;

	    subbuf_start_reserve(buf, sizeof(unsigned int));

	    return 1;
    }

```
如果当前缓冲区已满（即所有子缓冲区都仍未消费），回调返回 0，表示尚不应发生缓冲区切换，即直到消费者有机会读取当前这组就绪的子缓冲区。为了让 relay_buf_full() 函数有意义，消费者负责在子缓冲区被消费时通过 relay_subbufs_consumed() 通知 relay 接口。任何后续向缓冲区的写入尝试都会再次以相同参数调用 subbuf_start() 回调；只有当消费者消费了一个或多个就绪的子缓冲区后，relay_buf_full() 才会返回 0，此时缓冲区切换才能继续。

“overwrite”模式下 subbuf_start() 回调的实现

```

    static int subbuf_start(struct rchan_buf *buf,
			    void *subbuf,
			    void *prev_subbuf,
			    size_t prev_padding)
    {
	    if (prev_subbuf)
		    *((unsigned *)prev_subbuf) = prev_padding;

	    subbuf_start_reserve(buf, sizeof(unsigned int));

	    return 1;
    }

```
在这种情况下，relay_buf_full() 检查没有意义，回调总是返回 1，导致缓冲区切换无条件发生。客户端在此模式下使用 relay_subbufs_consumed() 函数也没有意义，因为它从不被查询。

默认的 subbuf_start() 实现（在客户端未定义任何回调，或未定义 subbuf_start() 回调时使用）实现了尽可能简单的“no-overwrite”模式，即它什么也不做，只返回 0。

可以通过在 subbuf_start() 回调内部调用 subbuf_start_reserve() 辅助函数，在每个子缓冲区的开头预留头部信息。这个预留区域可以用来存储客户端想要的任何信息。在上面的示例中，每个子缓冲区中都预留了空间来存储该子缓冲区的填充计数。这个值在 subbuf_start() 实现中为前一个子缓冲区填入；前一个子缓冲区的填充值连同指向它的指针一起传入 subbuf_start() 回调，因为填充值要直到一个子缓冲区被填满后才知道。当通道打开时，subbuf_start() 回调也会为第一个子缓冲区调用，给客户端一个在其中预留空间的机会。在这种情况下，传入回调的前一个子缓冲区指针将为 NULL，因此客户端在写入前一个子缓冲区之前应检查 prev_subbuf 指针的值。

### 写入通道


内核客户端使用 relay_write() 或 __relay_write() 将数据写入当前 CPU 的通道缓冲区。relay_write() 是主要的日志函数——它使用 local_irqsave() 保护缓冲区，如果你可能从中断上下文记录日志，就应使用它。如果你确定永远不会从中断上下文记录日志，可以使用 __relay_write()，它只禁用抢占（preemption）。这些函数不返回值，因此你无法判断它们是否失败——其假设是，无论如何你都不想在快速日志路径中检查返回值，并且除非缓冲区已满且使用的是 no-overwrite 模式，否则它们总会成功；在后一种情况下，你可以通过调用 relay_buf_full() 辅助函数在 subbuf_start() 回调中检测到一次失败的写入。

relay_reserve() 用于在通道缓冲区中预留一个以后可写入的槽位。这通常用于那些需要在不事先将数据暂存到临时缓冲区的情况下，直接写入通道缓冲区应用程序。由于实际写入未必在槽位被预留后立即发生，使用 relay_reserve() 的应用程序可以自行统计实际写入的字节数，既可以利用子缓冲区自身中预留的空间，也可以使用一个独立的数组。关于如何做到这一点，请参阅 relay-apps 压缩包中位于 http://relayfs.sourceforge.net 的“reserve”示例。由于写入由客户端控制，且与预留相分离，relay_reserve() 完全不保护缓冲区——使用 relay_reserve() 时，提供适当的同步机制是客户端的责任。

### 关闭通道


客户端在不再使用通道时调用 relay_close()。当不再有任何对通道缓冲区的引用时，通道及其关联缓冲区被销毁。relay_flush() 强制对所有通道缓冲区进行一次子缓冲区切换，可用于在通道关闭之前终结并处理最后的子缓冲区。

### 杂项


某些应用程序可能希望保留一个通道并在多次使用中复用它，而不必为每次使用都打开并关闭一个新通道。relay_reset() 可用于此目的——它将一个通道重置到其初始状态，而无需重新分配通道缓冲区内存或销毁现有映射。不过它只应在安全时才被调用，即当通道当前未被写入时。

最后，还有几个可用于不同目的的实用回调。每当一个通道缓冲区被用户空间 mmap 时调用 buf_mapped()，而当它被解除映射（unmapped）时调用 buf_unmapped()。客户端可以利用这一通知来触发内核应用程序内的动作，例如启用/禁用向该通道的日志写入。


## 资源


有关新闻、示例代码、邮件列表等信息，请参阅 relay 接口主页：

    http://relayfs.sourceforge.net


## 致谢


relay 接口的想法与规范源于以下人员参与的关于追踪（tracing）的讨论：

Michel Dagenais		<michel.dagenais@polymtl.ca>
Richard Moore		<richardj_moore@uk.ibm.com>
Bob Wisniewski		<bob@watson.ibm.com>
Karim Yaghmour		<karim@opersys.com>
Tom Zanussi		<zanussi@us.ibm.com>

同时感谢 Hubertus Franke 提供的许多有用建议与缺陷报告。
