
## autofs - 工作原理


## 目的


autofs 的目标是提供按需挂载以及无竞态的自动卸载各种其他文件系统的能力。
这提供了两个关键优势：

1. 无需延迟启动（boot），直到所有可能需要的文件系统都被挂载。尝试访问那些
   慢速文件系统的进程可能会被延迟，但其他进程可以自由继续。这对于网络文件系统
   （例如 NFS）或存储在带有换盘机械臂的介质上的文件系统尤为重要。

2. 文件系统的名称和位置可以存储在一个远程数据库中，并且可以随时更改。访问时
   该数据库中的内容将被用来为访问提供一个目标。文件系统中名称的解释甚至可以是
   程序化的而非基于数据库的，例如允许通配符，并且可以根据首次访问某个名称的用户
   而变化。


## 背景


“autofs”文件系统模块只是一个 autofs 系统的一部分。还需要有一个用于查找名称
并挂载文件系统的用户空间程序。这通常会是“automount”程序，不过包括“systemd”
在内的其他工具也可以使用“autofs”。本文档仅描述内核模块以及与任何用户空间程序
所需的交互。后续文本将其称为“automount 守护进程”或简称为“守护进程”。

“autofs”是一个 Linux 内核模块，它提供“autofs”文件系统类型。可以挂载多个
“autofs”文件系统，它们可以分别被单独管理，或由同一个守护进程统一管理。


## 内容


一个 autofs 文件系统可以包含 3 种对象：目录、符号链接和挂载陷阱（mount trap）。
挂载陷阱是具有额外属性的目录，如下一节所述。

对象只能由 automount 守护进程创建：符号链接使用常规的 `symlink` 系统调用创建，
而目录和挂载陷阱使用 `mkdir` 创建。一个目录是否应该成为挂载陷阱取决于主映射
（master map）。autofs 会查询该主映射以确定哪些目录是挂载点。挂载点可以是
**direct**（直接）/**indirect**（间接）/**offset**（偏移）。

在大多数系统上，默认的主映射位于 **/etc/auto.master**。

如果既没有给出 **direct** 也没有给出 **offset** 挂载选项（因此该挂载被视为
**indirect**），那么根目录始终是一个常规目录，否则当根目录为空时它是一个挂载陷阱，
非空时是常规目录。注意 **direct** 和 **offset** 被同等对待，所以一个简洁的总结是：
只有当文件系统以 **direct** 挂载且根目录为空时，根目录才是挂载陷阱。

在根目录中创建的目录只有在文件系统以 **indirect** 挂载且它们为空时才是挂载陷阱。

树中更下层的目录取决于 **maxproto** 挂载选项，特别是它是否小于五。

当 **maxproto** 为五时，树中更下层的目录永远不是挂载陷阱，它们始终是常规目录。
当 **maxproto** 为四（或三）时，这些目录恰好在它们为空时才是挂载陷阱。

所以：非空的（即非叶子的）目录永远不是挂载陷阱。空目录有时是挂载陷阱，有时不是，
这取决于它们在树中的位置（根、顶层、或更下层）、**maxproto**，以及挂载是否
为 **indirect**。


## 挂载陷阱


autofs 实现的一个核心要素是由 Linux VFS 提供的挂载陷阱（Mount Trap）。任何由
文件系统提供的目录都可以被指定为陷阱。这涉及两个协同工作以允许 autofs 完成其
工作的独立特性。


**DCACHE_NEED_AUTOMOUNT**

如果一个 dentry 设置了 DCACHE_NEED_AUTOMOUNT 标志（如果 inode 设置了
S_AUTOMOUNT 则会被设置，也可以直接设置），那么它（潜在地）是一个挂载陷阱。
对该目录超出一次 "`stat`" 的任何访问都将（通常）导致调用 `d_op->d_automount()`
dentry 操作。此方法的作用是找到应当挂载在该目录上的文件系统并将其返回。VFS
负责实际将该文件系统的根挂载到该目录上。

autofs 本身并不查找文件系统，而是向 automount 守护进程发送一条消息，要求它查找
并挂载该文件系统。然后 autofs 的 `d_automount` 方法等待守护进程报告一切就绪。
随后它返回 "`NULL`"，表示挂载已经发生。VFS 不会尝试挂载任何东西，而是沿着已经
存在的挂载向下走。

这个功能对于某些挂载陷阱的使用者（例如 NFS，它创建陷阱以便服务器上的挂载点
能够反映在客户端上）已经足够。然而它对 autofs 来说并不足够。由于挂载到一个目录
被视为“超出一次 `stat`”，automount 守护进程将无法在该“陷阱”目录上挂载文件系统，
除非有某种方法避免陷入陷阱。为此还有另一个标志。


**DCACHE_MANAGE_TRANSIT**

如果一个 dentry 设置了 DCACHE_MANAGE_TRANSIT，则会调用两种非常不同但相关的
行为，两者都使用 `d_op->d_manage()` dentry 操作。

首先，在检查是否有任何文件系统挂载在该目录上之前，将调用 d_manage()，其
`rcu_walk` 参数被设置为 `false`。它可能返回以下三种结果之一：

- 返回值为零表示该 dentry 没有什么特殊之处，应当继续对挂载和自动挂载的正常检查。

   autofs 通常返回零，但首先会等待任何过期（已挂载文件系统的自动卸载）完成。
   这避免了竞态。

- 返回 `-EISDIR` 告诉 VFS 忽略该目录上的任何挂载，并不要考虑调用 `->d_automount()`。
   这实际上禁用了 **DCACHE_NEED_AUTOMOUNT** 标志，从而使该目录最终不是一个挂载陷阱。

   autofs 在检测到执行查找的进程是 automount 守护进程，并且挂载已被请求但尚未完成
   时返回此值。它如何确定这一点将在后面讨论。这使得 automount 守护进程不会被困在
   挂载陷阱中。

   这里有个微妙之处。有可能在第一个 autofs 文件系统之下挂载第二个 autofs 文件系统，
   并且它们都由同一个守护进程管理。为了让守护进程能够在第二个上挂载东西，它必须能够
   “走”过第一个。这意味着 d_manage 不能**总是**为 automount 守护进程返回 -EISDIR。
   它必须仅在挂载已被请求但尚未完成时返回它。

   `d_manage` 在 dentry 不应是挂载陷阱时也会返回 `-EISDIR`，要么因为它是一个符号链接，
   要么因为它不为空。

- 任何其他的负值都被视为错误并返回给调用者。

   autofs 可以返回

   - -ENOENT，如果 automount 守护进程未能挂载任何东西，
   - -ENOMEM，如果内存耗尽，
   - -EINTR，如果在等待过期完成时到达了一个信号
   - 或 automount 守护进程下发的任何其他错误。


第二种用例只发生在“RCU-walk”期间，因此 `rcu_walk` 会被设置。

RCU-walk 是一种快速且轻量的沿文件名路径向下走的过程（即它像是踮着脚尖运行）。
RCU-walk 不能应对所有情况，因此当它发现困难时会回退到“REF-walk”，后者较慢但更
健壮。

RCU-walk 永远不会调用 `->d_automount`；文件系统必须已经挂载，否则 RCU-walk 无法
处理该路径。为了确定一个挂载陷阱对于 RCU-walk 模式是否安全，它会调用
`->d_manage()`，并将 `rcu_walk` 设置为 `true`。

在这种情况下，`d_manage()` 必须避免阻塞，并应尽可能避免获取自旋锁。它的唯一目的是
确定跟随进入任何已挂载的目录是否安全，而它可能不安全的唯一原因是挂载的过期正在进行中。

在 `rcu_walk` 情况下，`d_manage()` 不能返回 -EISDIR 来告诉 VFS 这是一个不需要
d_automount 的目录。如果 `rcu_walk` 看到一个设置了 DCACHE_NEED_AUTOMOUNT 但没有任何
东西挂载的 dentry，它**会**回退到 REF-walk。`d_manage()` 不能让 VFS 保持在 RCU-walk
模式，而只能通过返回 `-ECHILD` 告诉它退出 RCU-walk 模式。

因此，当以设置了 `rcu_walk` 的方式调用时，`d_manage()` 如果有任何理由认为进入已挂载
的文件系统不安全，就应该返回 -ECHILD，否则应该返回 0。

autofs 在文件系统的过期已启动或正在被考虑时返回 `-ECHILD`，否则返回 0。


## 挂载点过期


VFS 有一种自动过期未使用挂载的机制，就像它可以从 dcache 中过期任何未使用的 dentry
信息一样。这由 MNT_SHRINKABLE 标志引导。这仅适用于那些由 `d_automount()` 返回一个
要挂载的文件系统所创建的挂载。由于 autofs 不返回这样的文件系统，而是将挂载留给
automount 守护进程，因此它也必须让 automount 守护进程参与卸载。这也意味着 autofs
对过期有更强的控制。

VFS 也支持使用 MNT_EXPIRE 标志对 `umount` 系统调用进行挂载的“过期”。使用 MNT_EXPIRE
卸载将失败，除非之前已经尝试过一次，并且自那次尝试以来该文件系统一直处于不活动且未被
触碰的状态。autofs 不依赖于此，而是有自己的内部跟踪机制来记录文件系统最近是否被使用。
这使得 autofs 目录中的各个名称可以分别过期。

在协议版本 4 中，automount 守护进程可以随时尝试卸载挂载在 autofs 文件系统上的任何
文件系统，或移除任何符号链接或空目录。如果卸载或移除成功，文件系统将返回到挂载或
创建之前的状态，因此对该名称的任何访问都将触发正常的自动挂载处理。特别是，`rmdir` 和
`unlink` 不会像普通文件系统那样在 dcache 中留下负的（negative）条目，因此对最近被
移除对象的访问会被传递给 autofs 处理。

在版本 5 中，除了从顶层目录卸载之外，这并不安全。由于更下层的目录永远不是挂载陷阱，
其他进程会在文件系统一被卸载时就看到一个空目录。因此通常使用下文描述的 autofs 过期
协议是最安全的。

通常守护进程只希望移除那些有一段时间未被使用的条目。为此，autofs 在每个目录或符号
链接上维护一个 "`last_used`" 时间戳。对于符号链接，它确实记录了符号链接被“使用”或
被跟随以找出其指向位置的最后时间。对于目录，该字段的使用方式略有不同。该字段在挂载时
以及在过期检查期间（如果发现它正在被使用，即有打开的文件描述符或进程工作目录）和路径
遍历期间被更新。在路径遍历期间完成的更新防止了频繁访问的自动挂载被频繁过期并立即重新
挂载。但在 GUI 持续访问或应用程序频繁扫描 autofs 目录树的情况下，可能会累积实际上并
未被使用的挂载。为了应对这种情况，可以使用 "`strictexpire`" autofs 挂载选项来避免在
路径遍历时更新 "`last_used`"，从而防止这种对未真正使用的挂载显然无法过期的情况。

守护进程能够询问 autofs 是否有任何内容即将过期，使用下文将讨论的 `ioctl`。对于
**direct** 挂载，autofs 会考虑整个挂载树是否可以被卸载。对于 **indirect** 挂载，
autofs 会考虑顶层目录中的每个名称，以确定其中是否有任何可以被卸载和清理。

间接挂载有一个选项，用于考虑每个已被挂载的叶子，而不是考虑顶层名称。这原本是为了与
autofs 版本 4 兼容，对于 Sun 格式 automount 映射应视为已弃用。然而，它可能会再次用于
amd 格式的挂载映射（通常是间接映射），因为 amd automounter 允许为各个挂载设置过期
超时。但为此做出所需的更改存在一些困难。

当 autofs 考虑一个目录时，它会检查 `last_used` 时间，并将其与文件系统挂载时设置的
“timeout”值进行比较，尽管在某些情况下会忽略此检查。它还会检查该目录或其下方任何内容
是否正在被使用。对于符号链接，只会考虑 `last_used` 时间。

如果两者都表明支持对目录或符号链接进行过期，就会采取一个动作。

有两种方式可以请求 autofs 考虑过期。第一种是使用 **AUTOFS_IOC_EXPIRE** ioctl。这只对
间接挂载有效。如果它在根目录中发现有可以过期的东西，它将返回该东西的名称。一旦返回了
一个名称，automount 守护进程通常需要正常卸载该名称下挂载的任何文件系统。如上所述，对于
版本 5 的 autofs，这对于非顶层挂载是不安全的。因此当前的 `automount(8)` 不使用此 ioctl。

第二种机制使用 **AUTOFS_DEV_IOCTL_EXPIRE_CMD** 或 **AUTOFS_IOC_EXPIRE_MULTI** ioctl。
这对直接挂载和间接挂载都有效。如果它选择了一个要过期的对象，它将使用下文描述的
通知机制通知守护进程。这会一直阻塞，直到守护进程确认了该过期通知。这意味着 "`EXPIRE`"
ioctl 必须从与处理通知的线程不同的线程发送。

在 ioctl 阻塞期间，该条目被标记为“expiring（正在过期）”，并且 `d_manage` 会阻塞，直到
守护进程确认卸载已完成（连同可能需要的任何目录的移除），或被中止。


## 与 autofs 通信：检测守护进程


automount 守护进程与文件系统之间有几种通信形式。正如我们已经看到的，守护进程可以使用
普通的文件系统操作创建和移除目录与符号链接。autofs 根据请求某个操作的进程的进程组 id
号（参见 getpgid(1)）来判断该进程是否为守护进程。

当挂载一个 autofs 文件系统时，会记录挂载进程的 pgid，除非给出了 "pgrp=" 选项，在这种
情况下会记录该数字。来自该进程组中任何进程的请求都被视为来自守护进程。如果守护进程
需要被停止并重新启动，可以通过一个 ioctl 提供新的 pgid，如下文所述。


## 与 autofs 通信：事件管道


当挂载一个 autofs 文件系统时，必须使用 'fd=' 挂载选项传入一个管道的 'write' 端。
autofs 会向该管道写入通知消息，供守护进程响应。
```

	struct autofs_v5_packet {
		struct autofs_packet_hdr hdr;
		autofs_wqt_t wait_queue_token;
		__u32 dev;
		__u64 ino;
		__u32 uid;
		__u32 gid;
		__u32 pid;
		__u32 tgid;
		__u32 len;
		char name[NAME_MAX+1];
        };

```

```

	struct autofs_packet_hdr {
		int proto_version;		/* Protocol version */
		int type;			/* Type of packet */
	};

```

```

	autofs_ptype_missing_indirect
	autofs_ptype_expire_indirect
	autofs_ptype_missing_direct
	autofs_ptype_expire_direct

```

因此消息可以指示某个名称缺失（有东西试图访问它但它不在那里）或它已被选中进行过期。

该管道将被设置为“packet mode（包模式）”（等效于向 _pipe2(2)_ 传入 `O_DIRECT`），
以便从管道读取时最多返回一个包，并且包的任何未读部分将被丢弃。

`wait_queue_token` 是一个唯一的数字，可以标识一个待确认的具体请求。当通过管道发送一条
消息时，受影响的 dentry 被标记为“active（活动）”或“expiring（正在过期）”，并且对其的
其他访问会阻塞，直到使用下面某个带有相关 `wait_queue_token` 的 ioctl 确认了该消息。


## 与 autofs 通信：根目录 ioctls


autofs 文件系统的根目录会响应若干 ioctl。发出 ioctl 的进程必须具有 CAP_SYS_ADMIN
 capability，或者必须是 automount 守护进程。

可用的 ioctl 命令有：

- **AUTOFS_IOC_READY**：
	一个通知已被处理。ioctl 命令的参数是与正在被确认的通知相对应的
	“wait_queue_token”数字。
- **AUTOFS_IOC_FAIL**：
	与上述类似，但表示以错误码 `ENOENT` 失败。
- **AUTOFS_IOC_CATATONIC**：
	使 autofs 进入“catatonic（强直性）”模式，意味着它停止向守护进程发送通知。
	如果对管道的写入失败，也会进入此模式。
- **AUTOFS_IOC_PROTOVER**：
	返回正在使用的协议版本。
- **AUTOFS_IOC_PROTOSUBVER**：
	返回协议子版本，它实际上是实现的一个版本号。
- **AUTOFS_IOC_SETTIMEOUT**：
	传入一个指向 unsigned long 的指针。该值用于设置过期的超时时间，
	并且当前的超时值通过指针存回。
- **AUTOFS_IOC_ASKUMOUNT**：
	在所指的 `int` 中返回 1，如果文件系统可以被卸载。这只是一个提示，
	因为情况可能在任何时刻改变。此调用可用于避免更昂贵的完整卸载尝试。
- **AUTOFS_IOC_EXPIRE**：
	如上所述，这会询问是否有
```

		struct autofs_packet_expire_multi {
			struct autofs_packet_hdr hdr;
			autofs_wqt_t wait_queue_token;
			int len;
			char name[NAME_MAX+1];
		};

```

所需的内容。它会填入可以被卸载或移除的东西的名称。如果没有任何东西可以过期，
`errno` 被设置为 `EAGAIN`。尽管结构中存在一个 `wait_queue_token`，但并没有建立
“wait queue”，也不需要确认。
- **AUTOFS_IOC_EXPIRE_MULTI**：
	这类似于 **AUTOFS_IOC_EXPIRE**，只是它会向守护进程发送通知，并且会阻塞
	直到守护进程确认。参数是一个可以包含两个不同标志的整数。

	**AUTOFS_EXP_IMMEDIATE** 使 `last_used` 时间被忽略，并且只要对象未被使用就会被过期。

	**AUTOFS_EXP_FORCED** 使在使用中状态被忽略，并且即使对象正在使用中也会被过期。这假设
	守护进程请求此操作是因为它能够执行 umount。

	**AUTOFS_EXP_LEAVES** 将选择一个叶子而不是顶层名称来过期。这仅在 **maxproto** 为 4 时
	才是安全的。


## 与 autofs 通信：字符设备 ioctls


并不总是能够打开 autofs 文件系统的根目录，特别是 **direct** 挂载的文件系统。如果
automount 守护进程被重新启动，它无法使用上述任何通信通道重新获得对现有挂载的控制。
为了满足这一需求，有一个“miscellaneous”字符设备（主设备号 10，次设备号 235），可以
用来直接与 autofs 文件系统通信。它需要 CAP_SYS_ADMIN 才能访问。

可以在该设备上使用的 'ioctl' 在单独的文档 `autofs-mount-control.rst` 中描述，这里
简要总结。
```

        struct autofs_dev_ioctl {
                __u32 ver_major;
                __u32 ver_minor;
                __u32 size;             /* total size of data passed in
                                         * including this struct */
                __s32 ioctlfd;          /* automount command fd */

		/* Command parameters */
		union {
			struct args_protover		protover;
			struct args_protosubver		protosubver;
			struct args_openmount		openmount;
			struct args_ready		ready;
			struct args_fail		fail;
			struct args_setpipefd		setpipefd;
			struct args_timeout		timeout;
			struct args_requester		requester;
			struct args_expire		expire;
			struct args_askumount		askumount;
			struct args_ismountpoint	ismountpoint;
		};

                char path[];
        };

```

对于 **OPEN_MOUNT** 和 **IS_MOUNTPOINT** 命令，目标文件系统由 `path` 标识。所有其他
命令由 `ioctlfd` 标识该文件系统，后者是一个在根目录上打开的文件描述符，并且可以由
**OPEN_MOUNT** 返回。

`ver_major` 和 `ver_minor` 是 in/out 参数，用于检查所请求的版本是否受支持，并报告
内核模块所能支持的最高版本。

命令有：

- **AUTOFS_DEV_IOCTL_VERSION_CMD**：
	什么都不做，只验证并设置版本号。
- **AUTOFS_DEV_IOCTL_OPENMOUNT_CMD**：
	返回一个在 autofs 文件系统根目录上打开的文件描述符。该文件系统由名称和
	设备号标识，它们存储在 `openmount.devid` 中。现有文件系统的设备号可以在
	`/proc/self/mountinfo` 中找到。
- **AUTOFS_DEV_IOCTL_CLOSEMOUNT_CMD**：
	与 `close(ioctlfd)` 相同。
- **AUTOFS_DEV_IOCTL_SETPIPEFD_CMD**：
	如果文件系统处于 catatonic 模式，这可以在 `setpipefd.pipefd` 中提供新管道的
	写端，以重新建立与守护进程的通信。调用进程的进程组用于标识守护进程。
- **AUTOFS_DEV_IOCTL_REQUESTER_CMD**：
	`path` 应该是文件系统内一个已被自动挂载在其上的名称。成功返回时，`requester.uid`
	和 `requester.gid` 将是触发该挂载的进程的 UID 和 GID。
- **AUTOFS_DEV_IOCTL_ISMOUNTPOINT_CMD**：
	检查 path 是否是某种特定类型的挂载点——详见单独的文档。

- **AUTOFS_DEV_IOCTL_PROTOVER_CMD**
- **AUTOFS_DEV_IOCTL_PROTOSUBVER_CMD**
- **AUTOFS_DEV_IOCTL_READY_CMD**
- **AUTOFS_DEV_IOCTL_FAIL_CMD**
- **AUTOFS_DEV_IOCTL_CATATONIC_CMD**
- **AUTOFS_DEV_IOCTL_TIMEOUT_CMD**
- **AUTOFS_DEV_IOCTL_EXPIRE_CMD**
- **AUTOFS_DEV_IOCTL_ASKUMOUNT_CMD**

这些都具有与同名 **AUTOFS_IOC** ioctl 相同的功能，只是 **FAIL** 可以在 `fail.status` 中
给出显式的错误号，而不是假定为 `ENOENT`，并且此 **EXPIRE** 命令对应于
**AUTOFS_IOC_EXPIRE_MULTI**。


## Catatonic 模式


如前所述，一个 autofs 挂载可以进入“catatonic（强直性）”模式。如果对通知管道的写入
失败，或者被一个 `ioctl` 显式请求，就会发生这种情况。

进入 catatonic 模式时，管道被关闭，任何待处理的通知都以错误 `ENOENT` 确认。

一旦进入 catatonic 模式，尝试访问不存在的名称将导致 `ENOENT`，而尝试访问已存在的目录
将被以与来自守护进程相同的方式处理，因此挂载陷阱不会触发。

当文件系统被挂载时，可以给出一个 _uid_ 和 _gid_，它们设置目录和符号链接的所有权。当
文件系统处于 catatonic 模式时，任何具有匹配 UID 的进程都可以在根目录中创建目录或符号
链接，但不能在其他目录中创建。

Catatonic 模式只能通过 `/dev/autofs` 上的 **AUTOFS_DEV_IOCTL_OPENMOUNT_CMD** ioctl 离开。


## “ignore” 挂载选项


“ignore”挂载选项可用于向应用程序提供一个通用指示符，表明在显示挂载信息时应当忽略该
挂载条目。

在其他提供 autofs 并且基于内核挂载列表向用户空间提供挂载列表的 OS 上，允许一个无操作
的挂载选项（“ignore” 是最常见 OS 上使用的那个），以便 autofs 文件系统用户可以选择
使用它。

这旨在供用户空间程序在读取挂载列表时排除 autofs 挂载。


## autofs、名称空间和共享挂载


借助绑定挂载（bind mount）和名称空间，一个 autofs 文件系统有可能出现在一个或多个
文件系统名称空间中的多个位置。为了让这合理地工作，autofs 文件系统应该
```

	mount --make-shared /autofs/mount/point

```

automount 守护进程只能管理一个 autofs 文件系统的单一挂载位置，并且如果其上的挂载不是
“shared（共享）”的，其他位置将不会如预期般表现。特别是对那些位置的访问会
```

	Too many levels of symbolic links

```
