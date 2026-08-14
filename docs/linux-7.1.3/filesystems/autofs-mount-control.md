
## autofs 内核模块的杂项设备控制操作


## 问题


autofs 在活动重启（即在存在繁忙挂载时重启 autofs）方面存在一个问题。

在正常操作中，autofs 使用在受管目录上打开的文件描述符，以便能够发出控制操作。
使用文件描述符让 ioctl 操作能够访问存储在超级块中的 autofs 特定信息。这些操作
包括将 autofs 挂载设置为 catatonic（僵死）、设置过期超时以及请求过期检查。正如
下面所解释的，某些类型的 autofs 触发挂载最终可能会覆盖 autofs 挂载本身，这就使
得如果我们还没有持有一个已打开的文件描述符，就无法使用 open(2) 来获取用于这些
操作的文件描述符。

目前 autofs 在重启时使用 "umount -l"（惰性卸载）来清理活动挂载。虽然惰性卸载在
大多数情况下有效，但任何需要回溯挂载树来构造路径的操作，例如 getcwd(2) 以及 proc
文件系统 /proc/<pid>/cwd，将不再工作，因为构造路径所依据的挂载点已经从挂载树中
脱离。

autofs 真正的问题在于它无法重新连接到已有的挂载。人们立刻会想到，只要加上重新
挂载 autofs 文件系统的能力就能解决，但遗憾的是这行不通。这是因为 autofs 直接挂载
以及嵌套挂载树 "按需挂载与过期" 的实现，是将文件系统直接挂载在挂载触发器目录的
dentry 之上。

例如，自动化挂载映射（automount map）有两种类型：直接（direct，在内核模块源码中
你会看到第三种称为 offset 的类型，它只是一种伪装的直接挂载）和间接（indirect）。

```

    /-      /etc/auto.direct
    /test   /etc/auto.indirect

```
```

    /etc/auto.direct:

    /automount/dparse/g6  budgie:/autofs/export1
    /automount/dparse/g1  shark:/autofs/export1
    and so on.

```
```

    g1    shark:/autofs/export1
    g6    budgie:/autofs/export1
    and so on.

```
对于上面的间接映射，在 /test 上挂载了一个 autofs 文件系统，并由 inode 查找操作为
每个子目录键触发挂载。例如，我们看到在 /test/g1 上挂载了 shark:/autofs/export1。

直接挂载的处理方式是在每个完整路径（如 /automount/dparse/g1）上建立一个 autofs
挂载，并将其用作挂载触发器。因此当我们沿着路径走下去时，会将 shark:/autofs/export1
挂载到 "这个挂载点之上"。由于这些永远都是目录，我们可以使用 follow_link inode
操作来触发挂载。

但是，直接映射和间接映射中的每个条目都可以有偏移（offset），从而变成多挂载映射
条目。

```

    g1  \
    /        shark:/autofs/export5/testing/test \
    /s1      shark:/autofs/export/testing/test/s1 \
    /s2      shark:/autofs/export5/testing/test/s2 \
    /s1/ss1  shark:/autofs/export1 \
    /s2/ss2  shark:/autofs/export2

```
```

    /automount/dparse/g1 \
	/       shark:/autofs/export5/testing/test \
	/s1     shark:/autofs/export/testing/test/s1 \
	/s2     shark:/autofs/export5/testing/test/s2 \
	/s1/ss1 shark:/autofs/export2 \
	/s2/ss2 shark:/autofs/export2

```
autofs 第 4 版的一个问题是，当挂载一个带有大量偏移（可能还嵌套）的条目时，我们
需要把所有这些偏移作为一个整体单元来挂载和卸载。这本身不是问题，除了对于那些在
映射条目中有大量偏移的用户。这个机制用于众所周知的 "hosts" 映射，我们已经见过
一些情况（在 2.4 内核中），其中可用的挂载数量或可用的特权端口数量被耗尽。

在第 5 版中，我们只在沿偏移树向下走时才挂载它们，过期时同理，从而解决了上述问题。
实现上还有一些更详细的细节，但就解释问题而言并不需要。一个重要的细节是，这些
偏移使用与上述直接挂载相同的机制实现，因此挂载点会被一个挂载所覆盖。

当前的 autofs 实现使用在挂载点上打开的 ioctl 文件描述符进行控制操作。该描述符
持有的引用会在判断一个挂载是否在使用时被计入，也用于访问保存在挂载超级块中的
autofs 文件系统信息。因此仍需保留文件句柄的使用。


## 解决方案


为了能够在重启 autofs 时让已有的直接、间接和偏移挂载保持原位，我们需要能够获取
这些可能被覆盖的 autofs 挂载点的文件句柄。与其只实现一个孤立的操作，我们决定
重新实现现有的 ioctl 接口，并添加新的操作来提供这个功能。

此外，为了能够重建带有繁忙挂载的挂载树，触发挂载的最后一名用户的 uid 和 gid
需要可用，因为它们可以用作 autofs 映射中的宏替换变量。它们在挂载请求时被记录，
并新增了一个操作来检索它们。

由于我们重新实现控制接口，现有接口上的另外几个问题也得到了解决。首先，当一个
挂载或过期操作完成时，会通过 "send ready（发送就绪）" 或 "send fail（发送失败）"
操作向内核返回一个状态。ioctl 接口的 "send fail" 操作只能发送 ENOENT，因此重新
实现允许用户空间发送实际的状态。对使用超大映射的用户来说，另一个开销很大的操作
是判断一个挂载是否存在。这通常涉及扫描 /proc/mounts，由于需要相当频繁地执行，
在挂载表中有大量条目时会引入显著的额外开销。还新增了一个用于查找挂载点 dentry
（无论是否被覆盖）挂载状态的操作。

当前的内核开发策略建议避免使用 ioctl 机制，转而采用 Netlink 之类的系统。我们
尝试用该系统来实现以评估其适用性，结果发现它在本场景下并不合适。这里使用的是
Generic Netlink 系统，因为原始的 Netlink 会显著增加复杂度。毫无疑问，Generic
Netlink 系统对于常见情况下的 ioctl 函数是一个优雅的解决方案，但它可能并不是一个
完整的替代品，大概是因为它的主要目的是作为一个消息总线实现，而非专门作为 ioctl
的替代品。虽然可以绕过这一点，但有一个顾虑导致了不使用它的决定。这就是守护进程
中的 autofs 过期已经变得过于复杂，原因在于枚举过期候选者，几乎别无原因，只是为了
"计数" 需要调用过期 ioctl 的次数。这涉及扫描挂载表，对于使用大映射的用户已被证明
是很大的开销。改进它的最佳方式是尝试回到很久以前过期的方式。也就是说，当为一个
挂载（文件句柄）发出过期请求时，我们应该持续回调守护进程，直到再也无法卸载任何
挂载，然后才向守护进程返回适当的状态。目前我们一次只过期一个挂载。Generic Netlink
的实现会因为消息总线架构的要求，排除未来开发中的这种可能性。


## autofs 杂项设备挂载控制接口


控制接口是打开一个设备节点，通常是 /dev/autofs。

所有 ioctl 都使用一个通用结构来传递所需参数
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
ioctlfd 字段是一个 autofs 挂载点的挂载点文件描述符。它由 open 调用返回，并被除
"判断给定路径是否为挂载点" 之外的所有调用所使用，在该调用中可以可选地使用它来检查
对应于给定挂载点文件描述符的特定挂载，以及当请求 autofs 文件系统中某个目录内最后
一次成功挂载的 uid 和 gid 时。

上面描述的联合（union）用于传达调用的参数和结果。

path 字段用于在需要的地方传递一个路径，size 字段用于在翻译从用户空间发送的结构时
计入增长后的结构长度。

这个结构可以在设置特定字段之前，通过使用 void 函数调用
init_autofs_dev_ioctl(`struct autofs_dev_ioctl *`) 来初始化。

所有 ioctl 都会将这个结构从用户空间复制到内核空间，如果 size 参数小于结构本身
的大小则返回 -EINVAL，如果内核内存分配失败则返回 -ENOMEM，如果复制本身失败则返回
-EFAULT。其他检查包括将用户空间编译进的版本与模块版本进行版本校验，不匹配会导致
返回 -EINVAL。如果 size 字段大于结构大小，则假定存在一个路径，并检查它是否以 "/"
开头并以 NULL 结尾，否则返回 -EINVAL。在这些检查之后，对于所有 ioctl 命令，除了
AUTOFS_DEV_IOCTL_VERSION_CMD、AUTOFS_DEV_IOCTL_OPENMOUNT_CMD 和
AUTOFS_DEV_IOCTL_CLOSEMOUNT_CMD 之外，都会校验 ioctlfd，如果它不是一个有效的
描述符或不对应一个 autofs 挂载点，则返回 -EBADF、-ENOTTY 或 -EINVAL（不是一个
autofs 描述符）。


## ioctl 命令


使用该接口的实现示例可以在 autofs 5.0.4 及更高版本中看到，位于从 kernel.org 的
/pub/linux/daemons/autofs/v5 目录可下载的发行版 tar 包中的文件
lib/dev-ioctl-lib.c。

该接口实现的设备节点 ioctl 操作如下：


### AUTOFS_DEV_IOCTL_VERSION

获取 autofs 设备 ioctl 内核模块实现的主版本号和次版本号。它需要一个已初始化的
struct autofs_dev_ioctl 作为输入参数，并将版本信息设置到传入的结构中。成功时
返回 0，如果检测到版本不匹配则返回错误 -EINVAL。


### AUTOFS_DEV_IOCTL_PROTOVER_CMD 与 AUTOFS_DEV_IOCTL_PROTOSUBVER_CMD

获取已加载模块所理解的 autofs 协议版本的主版本号和次版本号。该调用需要一个已
初始化的 struct autofs_dev_ioctl，其 ioctlfd 字段设置为一个有效的 autofs 挂载点
描述符，并将请求的版本号设置到 struct args_protover 的 version 字段或
struct args_protosubver 的 sub_version 字段中。这些命令成功时返回 0，如果校验
失败则返回某个负的错误码。


### AUTOFS_DEV_IOCTL_OPENMOUNT 与 AUTOFS_DEV_IOCTL_CLOSEMOUNT

获取并释放一个 autofs 受管挂载点路径的文件描述符。open 调用需要一个已初始化的
struct autofs_dev_ioctl，其 path 字段已设置、size 字段已适当调整，且
struct args_openmount 的 devid 字段设置为 autofs 挂载的设备号。设备号可以从
/proc/mounts 中显示的挂载选项中获得。close 调用需要一个已初始化的
struct autofs_dev_ioct，其 ioctlfd 字段设置为从 open 调用获得的描述符。文件
描述符的释放也可以用 close(2) 完成，因此任何打开的描述符也会在进程退出时被关闭。
close 调用被包含在已实现的操作中，很大程度上是为了完整性，并为一致的用户空间
实现提供支持。


### AUTOFS_DEV_IOCTL_READY_CMD 与 AUTOFS_DEV_IOCTL_FAIL_CMD

从用户空间向内核返回挂载和过期结果状态。这两个调用都需要一个已初始化的
struct autofs_dev_ioctl，其 ioctlfd 字段设置为从 open 调用获得的描述符，且
struct args_ready 或 struct args_fail 的 token 字段设置为等待队列令牌号，该令牌
号由用户空间在前述挂载或过期请求中收到。struct args_fail 的 status 字段被设置为
操作的 errno。成功时设置为 0。


### AUTOFS_DEV_IOCTL_SETPIPEFD_CMD

设置用于内核与守护进程通信的管道文件描述符。通常这在挂载时通过选项设置，但在
重新连接已有挂载时，我们需要使用它来告知 autofs 挂载新的内核管道描述符。为了保护
挂载不被错误地设置管道描述符，我们还要求 autofs 挂载处于 catatonic 状态（见下一个
调用）。

该调用需要一个已初始化的 struct autofs_dev_ioctl，其 ioctlfd 字段设置为从 open
调用获得的描述符，且 struct args_setpipefd 的 pipefd 字段设置为该管道的描述符。
成功时该调用还将用于标识控制进程（例如拥有的 automount(8) 守护进程）的进程组 id
设置为调用者的进程组。


### AUTOFS_DEV_IOCTL_CATATONIC_CMD

使 autofs 挂载点进入 catatonic 状态。autofs 挂载将不再发出挂载请求，内核通信管道
描述符被释放，队列中任何剩余的等待也被释放。

该调用需要一个已初始化的 struct autofs_dev_ioctl，其 ioctlfd 字段设置为从 open
调用获得的描述符。


### AUTOFS_DEV_IOCTL_TIMEOUT_CMD

设置 autofs 挂载点内挂载的过期超时。

该调用需要一个已初始化的 struct autofs_dev_ioctl，其 ioctlfd 字段设置为从 open
调用获得的描述符。


### AUTOFS_DEV_IOCTL_REQUESTER_CMD

返回最后成功在给定路径 dentry 上触发挂载的进程的 uid 和 gid。

该调用需要一个已初始化的 struct autofs_dev_ioctl，其 path 字段设置为相关挂载点，
且 size 字段已适当调整。返回时，struct args_requester 的 uid 字段包含 uid，gid
字段包含 gid。

在重建带有活动挂载的 autofs 挂载树时，我们需要重新连接到那些可能使用了原始进程
uid 和 gid（或它们的字符串变体）进行映射条目内挂载查找的挂载。这个调用提供了获取
该 uid 和 gid 的能力，以便用户空间在挂载映射查找时使用它们。


### AUTOFS_DEV_IOCTL_EXPIRE_CMD

向内核发出一个针对 autofs 挂载的过期请求。通常这个 ioctl 会被反复调用，直到找不到
更多过期候选者。

该调用需要一个已初始化的 struct autofs_dev_ioctl，其 ioctlfd 字段设置为从 open
调用获得的描述符。此外，可以通过把 struct args_expire 的 how 字段设置为
AUTOFS_EXP_IMMEDIATE 或 AUTOFS_EXP_FORCED，分别请求独立于挂载超时的立即过期和
独立于挂载是否繁忙的强制过期。如果找不到过期候选者，该 ioctl 返回 -1 且 errno
被设置为 EAGAIN。

这个调用使内核模块检查对应给定 ioctlfd 的挂载中可过期的挂载，向守护进程发出过期
请求并等待其完成。


### AUTOFS_DEV_IOCTL_ASKUMOUNT_CMD

检查一个 autofs 挂载点是否在使用中。

该调用需要一个已初始化的 struct autofs_dev_ioctl，其 ioctlfd 字段设置为从 open
调用获得的描述符，它将结果返回到 struct args_askumount 的 may_umount 字段，1 表示
繁忙，0 表示否则。


### AUTOFS_DEV_IOCTL_ISMOUNTPOINT_CMD

检查给定路径是否为挂载点。

该调用需要一个已初始化的 struct autofs_dev_ioctl。有两种可能的变体。两者都使用
path 字段设置为要检查的挂载点路径，且 size 字段已适当调整。一种使用 ioctlfd 字段
来标识要检查的具体挂载点，另一种变体使用 path 并可选地用 struct args_ismountpoint
的 in.type 字段设置为某个 autofs 挂载类型。如果该路径是挂载点，调用返回 1，并将
out.devid 字段设置为该挂载的设备号、out.magic 字段设置为相关的超级块魔数（如下
所述），否则返回 0。在两种情况下，设备号（由 new_encode_dev() 返回）都会被返回到
out.devid 字段。

如果提供了一个文件描述符，我们是在查找一个特定的挂载，不一定位于挂载栈的顶端。
在这种情况下，如果该描述符对应的路径本身是一个挂载点，或包含一个挂载（例如没有
根挂载的多挂载），则被视为挂载点。在这种情况下，如果描述符对应一个挂载点，我们
返回 1，并在存在覆盖挂载时返回该覆盖挂载的超级块魔数，否则返回 0。

如果提供了一个路径（且 ioctlfd 字段设置为 -1），则查找该路径并检查它是否是一个
挂载的根。如果还给定了一个类型，我们是在查找一个特定的 autofs 挂载，如果找不到
匹配则返回失败。如果定位到的路径是一个挂载的根，则返回 1 以及该挂载的超级块魔数，
否则返回 0。
