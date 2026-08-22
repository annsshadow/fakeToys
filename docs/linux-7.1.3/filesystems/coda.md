
## Coda 内核-Venus 接口


   这是描述 Coda 组件的技术文档之一——本文档描述的是客户端内Venus 接口

更多信息请参见：

  http://www.coda.cs.cmu.edu

运行 Coda 所需的用户级软件

  ftp://ftp.coda.cs.cmu.edu

  要运Coda，需要为客户端获取一个用户级缓存管理器，名为 Venus
  以及用于操作 ACL、登录等的工具。客户端在内核配置中需要选中 Coda
  文件系统

  服务器需要一个用户级服务器，目前不依赖内核支持

  Venus 内核接口

  Peter J. Braam

  v1.0锛?997骞?1鏈，鏃。

  本文档描述了 Venus 与内核级文件系统代码之间的通信，这Coda
  文件系统运行所必需的。本文档版本旨在描述当前接口
  （版1.0）以及我们设想的改进


  1. 简

  2. Coda 文件系统调用提供服务

  3. 消息

     3.1 实现细节

  4. 璋冪敤灞傛帴鍙。

     4.1 内核Venus 共享的数据结
     4.2 pioctl 接口
     4.3 root
     4.4 lookup
     4.5 getattr
     4.6 setattr
     4.7 access
     4.8 create
     4.9 mkdir
     4.10 link
     4.11 symlink
     4.12 remove
     4.13 rmdir
     4.14 readlink
     4.15 open
     4.16 close
     4.17 ioctl
     4.18 rename
     4.19 readdir
     4.20 vget
     4.21 fsync
     4.22 inactive
     4.23 rdwr
     4.24 odymount
     4.25 ody_lookup
     4.26 ody_expand
     4.27 prefetch
     4.28 signal

  5. 迷你缓存与下行调用（downcall

     5.1 INVALIDATE
     5.2 FLUSH
     5.3 PURGEUSER
     5.4 ZAPFILE
     5.5 ZAPDIR
     5.6 ZAPVNODE
     5.7 PURGEFID
     5.8 REPLACE

  6. 初始化与清理

     6.1 需

## 1. 简


  Coda 分布式文件系统的关键组件是缓存管理器 Venus

  当启用了 Coda 的系统上的进程访Coda 文件系统中的文件时，请求会被
  导向操作系统中的文件系统层。操作系统将Venus 通信以处理该进程
  请求。Venus 管理一个持久的客户端缓存，并向 Coda 文件服务器及相关
  服务器（如认证服务器）发起远程过程调用，以处理从操作系统收到的这
  请求。当 Venus 处理完一个请求后，会用适当的返回码及与该请求相关的
  其他数据回复操作系统。可选地，Coda 的内核支持可以维护一个最近处
  请求的迷你缓存，以限制与 Venus 的交互次数。Venus 具备在迷你缓存中
  元素不再有效时通知内核的能力

  本文档精确描述内核与 Venus 之间的这种通信。将给出所谓上行调
  （upcall）和下行调用（downcall）的定义，以及它们所处理数据的格式
  我们还将描述由这些调用产生的语义不变量

  历史上，Coda Mach 2.6 BSD 文件系统中实现。内核与 Venus 之间
  接口BSD VFS 接口非常相似。它提供了类似的功能，参数和返回数据
  格式也与 BSD VFS 非常相似。这使得BSD 系统中为 Coda 实现一
  内核级文件系统驱动几乎是一个自然的环境。然而，其他操作系统Linux
  Windows 95 NT 拥有不同接口的虚拟文件系统

  要在这些系统上实Coda，需要对 Venus/内核协议进行一些逆向工程
  同时人们也发现，其他系统可以从协议的某些小优化和修改中显著受益
  为了便于这项工作，并使未来的移植更容易，Venus 与内核之间的通信
  详尽地记录。这就是本文档的目标

## 2. Coda 文件系统调用提供服务


  Coda 文件系统服务的请求处理，起源于访Coda 文件、发出陷入（trap
  OS 内核的进P。在 Unix 环境中，这类陷入内核的调用示例包
  `read`、`write`、`open`、`close`、`create`、`mkdir`
  `rmdir`、`chmod`。在 Win32 环境中也存在类似的调用，名为 `CreateFile`

  通常操作系统在虚拟文件系统（VFS）层中处理该请求，在 NT 中称
  I/O Manager，在 Windows 95 中称IFS manager。VFS 负责请求的部
  处理，并定位将为请求各部分提供服务的特定文件系统。通常路径中的信息
  有助于定位正确的 FS 驱动。有时在大量预处理之后，VFS 开始调FS
  驱动中导出的例程。这就是请求FS 特定处理开始之处，也是 Coda 特定
  内核代码发挥作用之处

  Coda FS 层必须暴露并实现多个接口。首先也是最重要的，VFS 必须
  能够Coda FS 层发出所有必要的调用，因Coda FS 驱动必须暴露
  操作系统中适用VFS 接口。这些接口在不同操作系统之间差异很大，但
  共享诸如写以及创建和删除对象等功能。Coda FS 层通过调用缓存管理
  Venus 提供的一个或多个定义明确的服务来处理此类 VFS 请求。当来自
  Venus 的回复返回到 FS 驱动后，VFS 调用的处理继续，并以对内VFS 
  回复结束。最VFS 层返回到进程

  由于这种设计的需要，FS 驱动必须暴露的一个基本接口要允许 Venus 管理
  消息流量。特别是 Venus 必须能够检索和放置消息，并在新消息到达时得
  通知。由Venus 即使在没有消息等待或正在处理时也必须处理其他任务
  因此通知必须通过不会阻塞 Venus 的机制进行

  **Coda FS 驱动的接*

  此外，FS 层提供用户进程与 Venus 之间的一条特殊通信路径，称pioctl
  接口。pioctl 接口用于 Coda 特定的服务，例如请求关于 Venus 管理
  持久缓存的详细信息。这里内核的参与是最小的。它识别调用进程并将
  信息传递给 Venus。当 Venus 回复时，响应以未修改的形式传回调用者

  最后，Venus 允许内核 FS 驱动缓存某些服务的结果。这样做是为了避
  过多的上下文切换，从而得到一个高效的系统。然而，Venus 可能获取
  信息（例如来自网络），这意味着缓存的信息必须被刷新或替换。Venus 于是
  Coda FS 层发出一个下行调用（downcall），以请求刷新或更新缓存
  内核 FS 驱动同步地处理此类请求

  在这些接口中，VFS 接口以及放置、接收消息并获知消息到达的机制是
  平台相关的。我们不会深入讨论导出到 VFS 层的调用，但我们将说
  消息交换机制的需求


## 3. 消息


  在最底层，Venus FS 驱动之间的通信通过消息进行。请Coda 文件服务
  的进程与 Venus 之间的同步依赖于进程的阻塞与唤醒。Coda FS 驱动代表
  进程 P 处理 VFS pioctl 请求，为 Venus 创建消息，等待回复，最
  返回调用者。消息交换的实现是平台相关的，但其语义（到目前为止）看来
  普遍适用。数据缓冲区FS 驱动在内核内存中代表 P 创建，并复制
  Venus 的用户内存中

  FS 驱动在为 P 服务时进行上行调用（upcall）到 Venus。这样一个上行调
  通过创建消息结构分派Venus。该结构包含 P 的标识、消息序号、请
  的大小，以及指向内核内存中请求数据的指针。由于数据缓冲区被复用以
  保存来自 Venus 的回复，因此有一个字段用于回复的大小。消息中使用一
  flags 字段来精确记录消息的状态。其他平台相关结构涉及用于确定消息在
  队列中位置的指针，以及指向同步对象的指针。在 upcall 例程中，消息结构
  被填充，flags 置为 0，并被放**pending**（挂起）队列。调upcall
  的例程负责分配数据缓冲区；其结构将在下一节描述

  必须提供一种机制来通知 Venus 消息已创建，并使OS 中可用的同步对象
  实现。此通知在进P upcall 上下文中完成。当消息在挂起队列上时，
  进程 P 无法upcall 中继续。P 在文件系统请求例程中的（内核模式
  处理必须挂起，直Venus 回复。因此调用线程在 P upcall 中被阻塞
  消息结构中的一个指针将定位 P 正在其上睡眠的同步对象

  Venus 检测到消息已到达的通知，FS 驱动允许 Venus 通过 getmsg_from_kernel
  调用检索该消息。该动作在内核中执行完毕，即将消息放入处理中消息队列
  并将 flags 置为 READ。消息缓冲区的内容被传递给 Venus。getmsg_from_kernel
  调用现在返回，Venus 处理该请求

  稍后某个时刻，FS 驱动Venus 收到一条消息，Venus 调用
  sendmsg_to_kernel 时。此Coda FS 驱动查看消息内容并决定：

  - 消息是挂起线P 的回复。若是，它从处理队列中移除该消息并将消息
    标记WRITTEN。最后，FS 驱动解除 P 的阻塞（仍在 Venus 的内核模
    上下文中），sendmsg_to_kernel 调用返回Venus。进P 将在某个时刻
    被调度，并继续处理其 upcall，此时数据缓冲区已被 Venus 的回复替换

  - 消息是一`downcall`（下行调用）。下行调用是 Venus FS 驱动
    请求。FS 驱动立即处理该请求（通常是一次缓存驱逐或替换），完成
    sendmsg_to_kernel 返回

  现在 P 被唤醒并继续处理 upcall。有一些微妙之处需要考虑。首先，P 
  确定它是否由其他来源signal 唤醒（例如试图终P 的尝试），还是像
  通常情况那样Venus 在其 sendmsg_to_kernel 调用中唤醒。在正常情况
  下，upcall 例程将释放消息结构并返回。FS 例程可以继续其处理


  **睡眠IPC 安排**

  如果 P 是由 signal 而非 Venus 唤醒，它将首先查flags 字段。如
  消息尚未READ，进P 可以在不通知 Venus 的情况下处理signal
  如果 Venus 已经 READ，且请求不应被处理，P 可以Venus 发送一条信
  消息，表明它应忽略先前的消息。此类信号被放在队列头部，由 Venus 首先
  读取。如果消息已被标记为 WRITTEN，则停止处理为时已晚。VFS 例程现在
  将继续。（-- 如果一VFS 请求涉及多个 upcall，这会导致复杂的状态，
  可以在消息结构中添加一个额外字"handle_signals" 以标识已越过
  不可返回点-



### 3.1. 实现细节


  该机制的 Unix 实现是通过实现一个与 Coda 关联的字符设备来完成的
  Venus 通过对设备执read 来检索消息，回复通过 write 发送，通知通过
  对设备文件描述符执行 select 系统调用来实现。进P 被保持在可中
  的等待队列对象上等待

  Windows NT DPMI Windows 95 实现中，使用DeviceIoControl 调用
  DeviceIoControl 调用旨在通过 OPCODES 将缓冲区从用户内存复制到内核内存
  sendmsg_to_kernel 作为同步调用发出，getmsg_from_kernel 调用
  异步的。Windows EventObjects 用于通知消息到达。进P NT 中被保持
  KernelEvent 对象上等待，Windows 95 中被保持在信号量上等待


## 4. 璋冪敤灞傛帴鍙。


  本节描述 Coda FS 驱动可以Venus 进行的上行调用（upcall）。这
  上行调用中的每一个都使用两个结构：inputArgs outputArgs。以BNF
  形式，这些结构如下：

```
	struct inputArgs {
	    u_long opcode;
	    u_long unique;     /* Keep multiple outstanding msgs distinct */
	    u_short pid;                 /* Common to all */
	    u_short pgid;                /* Common to all */
	    struct CodaCred cred;        /* Common to all */

	    <union "in" of call dependent parts of inputArgs>
	};

	struct outputArgs {
	    u_long opcode;
	    u_long unique;       /* Keep multiple outstanding msgs distinct */
	    u_long result;

	    <union "out" of call dependent parts of inputArgs>
	};
```


  在继续之前，让我们阐明各个字段的作用。inputArgs 以定义所请求 Venus
  服务类型opcode 开头。目前大约有 30 个上行调用，我们将逐一讨论
  unique 字段用唯一编号标记 inputArg，该编号将唯一标识该消息。进程和
  进程id 被传递。最后包含调用者的凭证（credentials）

  在深入具体的调用之前，我们需要讨论内核与 Venus 共享的各种数据结构


### 4.1. 内核Venus 共享的数据结


  CodaCred 结构定义了调用进程所设置的多种用户和id。vuid_t 
  vgid_t 32 位无符号整数。它还在数组中定义组成员关系。在 Unix 上，
  CodaCred 已被证明足以实现 Coda 的良好安全语义，但该结构可能必须
  修改

```
	struct CodaCred {
	    vuid_t cr_uid, cr_euid, cr_suid, cr_fsuid; /* Real, effective, set, fs uid */
	    vgid_t cr_gid, cr_egid, cr_sgid, cr_fsgid; /* same for groups */
	    vgid_t cr_groups[NGROUPS];        /* Group membership for caller */
	};
```


  .. Note::

     是否需要在 Venus 中保CodaCreds 值得怀疑。最Venus 并不了解
     组，尽管它确实使用默认的 uid/gid 创建文件。也许组成员关系列表
     多余的


  下一项是用于标识 Coda 文件的基本标识符，即 ViceFid。文件的 fid 
  一cell [1]_ 内唯一地定Coda 文件系统中的文件或目录：

```
	typedef struct ViceFid {
	    VolumeId Volume;
	    VnodeId Vnode;
	    Unique_t Unique;
	} ViceFid;
```

  .. [1] cell 是在单一系统控制机（SCM）的庇护下运行的一Coda 服务器
	 有关 SCM 角色的详细说明，请参Coda 管理手册

  VolumeId、VnodeId Unique_t 这三个组成字段都是无符号 32 位整数
  我们设想需要在前面再加一个字段以标识 Coda cell；这可能采取通过 DNS
  命名 Coda cell Ipv6 大小 IP 地址的形式

  Venus 与内核之间共享的下一个重要结构是文件的属性。使用以下结构来
  交换信息。它为未来的扩展留有空间，例如对设备文件（当Coda 
  不存在）的支持：

```
	struct coda_timespec {
		int64_t         tv_sec;         /* seconds */
		long            tv_nsec;        /* nanoseconds */
	};

	struct coda_vattr {
		enum coda_vtype va_type;        /* vnode type (for create) */
		u_short         va_mode;        /* files access mode and type */
		short           va_nlink;       /* number of references to file */
		vuid_t          va_uid;         /* owner user id */
		vgid_t          va_gid;         /* owner group id */
		long            va_fsid;        /* file system id (dev for now) */
		long            va_fileid;      /* file id */
		u_quad_t        va_size;        /* file size in bytes */
		long            va_blocksize;   /* blocksize preferred for i/o */
		struct coda_timespec va_atime;  /* time of last access */
		struct coda_timespec va_mtime;  /* time of last modification */
		struct coda_timespec va_ctime;  /* time file changed */
		u_long          va_gen;         /* generation number of file */
		u_long          va_flags;       /* flags defined for file */
		dev_t           va_rdev;        /* device special file represents */
		u_quad_t        va_bytes;       /* bytes of disk space held by file */
		u_quad_t        va_filerev;     /* file modification number */
		u_int           va_vaflags;     /* operations flags, see below */
		long            va_spare;       /* remain quad aligned */
	};
```


### 4.2. pioctl 接口


  Coda 特定的请求可以由应用程序通过 pioctl 接口发出。pioctl 实现
  对虚构文/coda/.CONTROL 的普ioctl。pioctl 调用打开该文件，获取
  文件句柄并进ioctl 调用。最后它关闭文件

  内核在此处的参与仅限于提供打开和关闭以及传ioctl 消息的能力，
  验证 pioctl 数据缓冲区中的路径是 Coda 文件系统中的文件

```
	struct {
	    const char *path;
	    struct ViceIoctl vidata;
	    int follow;
	} data;
```

  其中

```
	struct ViceIoctl {
		caddr_t in, out;        /* Data to be transferred in, or out */
		short in_size;          /* Size of input buffer <= 2K */
		short out_size;         /* Maximum size of output buffer, <= 2K */
	};
```

  路径必须Coda 文件，否则将不会进行 ioctl 上行调用

  .. Note:: 数据结构和代码一团糟。我们需要清理它


**我们现在着手记录各个调*


### 4.3. root


  参数
     in

	empty

```
		struct cfs_root_out {
		    ViceFid VFid;
		} cfs_root;
```

  描述
    此调用在 Coda 文件系统初始化期间被发往 Venus。如果结果为零，
    cfs_root 结构包含 Coda 文件系统根的 ViceFid。如果产生非零结果，
    其值为平台相关的错误码，指Venus 在定Coda 文件系统根时遇到
    困难


### 4.4. lookup


  摘要
    如果对象存在，查找目录中对象ViceFid 和类型

  参数

```
		struct  cfs_lookup_in {
		    ViceFid     VFid;
		    char        *name;          /* Place holder for data. */
		} cfs_lookup;
```

     out锛。

```
		struct cfs_lookup_out {
		    ViceFid VFid;
		    int vtype;
		} cfs_lookup;
```

  描述
    此调用用于确定目录项ViceFid 和文件类型。所请求的目录项
    名为 'name'，Venus 将搜索由 cfs_lookup_in.VFid 标识的目录。结
    可能指示该名称不存在，或在查找时遇到困难（例如由于断开连接）
    如果结果为零，字cfs_lookup_out.VFid 包含目标ViceFid
    cfs_lookup_out.vtype 包含标识该名称所指对象类型的 coda_vtype

  该对象的名称是最大长度为 CFS_MAXNAMLEN 8 位字符串，当前设
  256（包括一0 结尾符）

  极其重要的是要认识到，Venus 将字cfs_lookup.vtype 按位或上
  CFS_NOCACHE，以指示该对象不应被放入内核名称缓存

  .. Note::

     当前 vtype 的类型是错误的。它应该coda_vtype。Linux 没有
     注意 CFS_NOCACHE。它应该这样做


### 4.5. getattr


  摘要 获取文件的属性

  参数

```
		struct cfs_getattr_in {
		    ViceFid VFid;
		    struct coda_vattr attr; /* XXXXX */
		} cfs_getattr;
```

     out锛。

```
		struct cfs_getattr_out {
		    struct coda_vattr attr;
		} cfs_getattr;
```

  描述
    此调用返回由 fid 标识的文件的属性

  错误
    如果带有fid 的对象不存在、不可访问，或调用者没有获取属性的
    权限，就可能发生错误

  .. Note::

     许多内核 FS 驱动（Linux、NT Windows 95）需要获取属性以
     用于实例化内"inode" "FileHandle" Fid。在此类系统上，通过
     Venus/内核交互层以RPC 层将 lookup getattr 调用合并，可
     显著改善性能

  输入参数中包含的 vattr 结构是多余的，应该删除


### 4.6. setattr


  摘要
    设置文件的属性

  参数

```
		struct cfs_setattr_in {
		    ViceFid VFid;
		    struct coda_vattr attr;
		} cfs_setattr;
```

     out

	empty

  描述
    结构 attr BSD 风格填充要更改的属性。不更改的属性设-1
    vtype 设为 VNON 外。其他属性设为要赋予的值。FS 驱动可能请求
    更改的唯一属性是 mode、owner、groupid、atime、mtime ctime
    返回值指示成功或失败

  错误
    可能发生各种错误。对象可能不存在、可能不可访问，Venus 可能
    不授予权限


### 4.7. access


  参数

```
		struct cfs_access_in {
		    ViceFid     VFid;
		    int flags;
		} cfs_access;
```

     out

	empty

  描述
    验证对由 VFid 标识的、由 flags 描述的操作的对象访问是否
    允许。结果指示是否将授予访问权限。务必记住，Coda 使用 ACL 来实
    保护，最终由服务器而非客户端来强制系统的安全性。此调用的结果将
    取决于用户是否持token

  错误
    对象可能不存在，或描述保护的 ACL 可能不可访问


### 4.8. create


  摘要
    调用以创建文件

  参数

```
		struct cfs_create_in {
		    ViceFid VFid;
		    struct coda_vattr attr;
		    int excl;
		    int mode;
		    char        *name;          /* Place holder for data. */
		} cfs_create;
```

     out锛。

```
		struct cfs_create_out {
		    ViceFid VFid;
		    struct coda_vattr attr;
		} cfs_create;
```

  描述
    此上行调用被调用以请求创建文件。该文件将在VFid 标识
    目录中创建，其名称为 name，mode mode。如果设置了 excl，且文件
    已存在，则返回错误。如attr 中的 size 字段设为零，文件将被截断
    文件uid gid 通过使用CRTOUID（此宏是平台相关的）CodaCred
    转换uid 来设置。成功后返回文件VFid 和属性。Coda FS 驱动通常
    会在内核层为新对象实例化一vnode、inode 或文件句柄

  错误
    可能发生各种错误。权限可能不足。如果对象存在且不是文件，在
    Unix 下返回错EISDIR

  .. Note::

     参数的打包效率很低，似乎表明系统调用 creat VFS 操作 create
     之间的混淆。VFS 操作 create 仅在创建新对象时被调用。此 create 调用
     Unix 版本的不同之处在于，它不被调用来返回文件描述符。truncate
     exclusive 选项连同 mode，可以简单地Unix 下那样成mode 
     一部分。不应有 flags 参数；flags 用于 open(2) 中以 READ WRITE
     模式返回文件描述符

  由于大小mtime 发生了变化，目录的属性也应返回


### 4.9. mkdir


  摘要
    创建新目录

  参数

```
		struct cfs_mkdir_in {
		    ViceFid     VFid;
		    struct coda_vattr attr;
		    char        *name;          /* Place holder for data. */
		} cfs_mkdir;
```

     out锛。

```
		struct cfs_mkdir_out {
		    ViceFid VFid;
		    struct coda_vattr attr;
		} cfs_mkdir;
```

  描述
    此调用类似于 create，但创建一个目录。输入参数中仅使mode
    字段进行创建。成功创建后，返回的 attr 包含新目录的属性

  错误
    鍚?create銆。

  .. Note::

     输入参数应改mode 而非属性

  父目录的属性应返回，因为大小和 mtime 发生了变化


### 4.10. link


  摘要
    创建到现有文件的链接

  参数

```
		struct cfs_link_in {
		    ViceFid sourceFid;          /* cnode to link *to* */
		    ViceFid destFid;            /* Directory in which to place link */
		    char        *tname;         /* Place holder for data. */
		} cfs_link;
```

     out

	empty

  描述
    此调用在destFid 标识的目录中，以名称 tname 创建
    sourceFid 的链接。源必须驻留在目标的父目录中，即源必须具有父目录
    destFid，也就是Coda 不支持跨目录硬链接。只有返回值相关。它指示
    成功或失败类型

  错误
    常见的错误都可能发生


### 4.11. symlink


  摘要
    创建符号链接

  参数

```
		struct cfs_symlink_in {
		    ViceFid     VFid;          /* Directory to put symlink in */
		    char        *srcname;
		    struct coda_vattr attr;
		    char        *tname;
		} cfs_symlink;
```

     out

	none

  描述
    创建符号链接。该链接将放置在VFid 标识的目录中，并命名
    tname。它应指向路径名 srcname。新创建对象的属性将设为 attr

  .. Note::

     由于目标目录的大小发生变化，应返回其属性


### 4.12. remove


  摘要
    删除文件

  参数

```
		struct cfs_remove_in {
		    ViceFid     VFid;
		    char        *name;          /* Place holder for data. */
		} cfs_remove;
```

     out

	none

  描述
    删除VFid 标识的目录中名为 cfs_remove_in.name 的文件

  .. Note::

     由于目录mtime 和大小可能发生变化，应返回其属性


### 4.13. rmdir


  摘要
    删除目录

  参数

```
		struct cfs_rmdir_in {
		    ViceFid     VFid;
		    char        *name;          /* Place holder for data. */
		} cfs_rmdir;
```

     out

	none

  描述
    从由 VFid 标识的目录中删除名为 'name' 的目录

  .. Note:: 由于父目录的 mtime 和大小可能发生变化，应返回其父目录的属性


### 4.14. readlink


  摘要
    读取符号链接的值

  参数

```
		struct cfs_readlink_in {
		    ViceFid VFid;
		} cfs_readlink;
```

     out锛。

```
		struct cfs_readlink_out {
		    int count;
		    caddr_t     data;           /* Place holder for data. */
		} cfs_readlink;
```

  描述
    此例程将VFid 标识的符号链接的内容读入缓冲data。data
    缓冲区必须能够容纳任何长CFS_MAXNAMLEN 的名称（PATH 还是 NAM？）

  错误
    无异常错误


### 4.15. open


  摘要
    打开文件

  参数

```
		struct cfs_open_in {
		    ViceFid     VFid;
		    int flags;
		} cfs_open;
```

     out锛。

```
		struct cfs_open_out {
		    dev_t       dev;
		    ino_t       inode;
		} cfs_open;
```

  描述
    此请求要Venus VFid 标识的文件放入其缓存，并记下调用
    进程希望open(2) 中的 flags 打开它。返回给内核的值在 Unix 
    Windows 系统之间有所不同。对Unix 系统，Coda FS 驱动被告知容
    文件dev inode 字段中的设备inode 号。对Windows，返
    容器文件的路径给内核

  .. Note::

     当前 cfs_open_out 结构没有正确适配以处Windows 情况。最好实
     两个上行调用，一个以容器文件名称为目标，另一个以容器文件 inode
     为目标


### 4.16. close


  摘要
    关闭文件，在服务器上更新它

  参数

```
		struct cfs_close_in {
		    ViceFid     VFid;
		    int flags;
		} cfs_close;
```

     out

	none

  描述
    关闭VFid 标识的文件

  .. Note::

     flags 参数是伪造的且未被使用。然而，Venus 的代码留有处execp
     输入字段的余地，可能应使用此字段来告Venus 文件已关闭但仍被
     内存映射以执行。Venus vproc_vfscalls 中有关于获取与不获取数据
     注释。这看起来很傻。如果文件正在关闭，容器文件中的数据将成
     新数据。这execp 标志可能又会参与制造混乱：当前 Venus 可能认为
     文件在仍被内存映射时就可以从缓存中刷新。这需要被理解


### 4.17. ioctl


  摘要
    对文件执ioctl。这包括 pioctl 接口

  参数

```
		struct cfs_ioctl_in {
		    ViceFid VFid;
		    int cmd;
		    int len;
		    int rwflag;
		    char *data;                 /* Place holder for data. */
		} cfs_ioctl;
```

     out锛。

```
		struct cfs_ioctl_out {
		    int len;
		    caddr_t     data;           /* Place holder for data. */
		} cfs_ioctl;
```

  描述
    对文件执ioctl 操作。command、len data 参数照常填充
    flags 不被 Venus 使用

  .. Note::

     另一个伪造的参数。flags 未被使用。Venus 代码中关PREFETCHING
     的事是什么？


### 4.18. rename


  摘要
    重命名一fid

  参数

```
		struct cfs_rename_in {
		    ViceFid     sourceFid;
		    char        *srcname;
		    ViceFid destFid;
		    char        *destname;
		} cfs_rename;
```

     out

	none

  描述
    sourceFid 目录中名srcname 的对象重命名destFid 
    destname。重要的是名srcname destname 是以 0 结尾的字符串
    Unix 内核中的字符串并不总是null 结尾


### 4.19. readdir


  摘要
    读取目录项

  参数

```
		struct cfs_readdir_in {
		    ViceFid     VFid;
		    int count;
		    int offset;
		} cfs_readdir;
```

     out锛。

```
		struct cfs_readdir_out {
		    int size;
		    caddr_t     data;           /* Place holder for data. */
		} cfs_readdir;
```

  描述
    VFid 开始，offset 偏移处读取目录项，最多读count
    字节。将数据返回data 中，并将大小返回size 中

  .. Note::

     此调用未被使用。Readdir 操作利用容器文件。我们将在进行中
     目录改造期间重新评估这一点


### 4.20. vget


  摘要
    指示 Venus 执行 FSDB->Get

  参数

```
		struct cfs_vget_in {
		    ViceFid VFid;
		} cfs_vget;
```

     out锛。

```
		struct cfs_vget_out {
		    ViceFid VFid;
		    int vtype;
		} cfs_vget;
```

  描述
    此上行调用要Venus 对由 VFid 标记fsobj 执行 get 操作

  .. Note::

     此操作未被使用。然而，它极其有用，因为它可用于处理写内
     映射文件。这些文件可以使vget Venus 缓存固定"，并使用
     inactive 释放


### 4.21. fsync


  摘要
    告诉 Venus 更新文件RVM 属性

  参数

```
		struct cfs_fsync_in {
		    ViceFid VFid;
		} cfs_fsync;
```

     out

	none

  描述
    要求 Venus 更新对象 VFid RVM 属性。这应作为内核级 fsync
    类型调用的一部分被调用。结果指示同步是否成功

  .. Note:: Linux 未实现此调用。它应该实现


### 4.22. inactive


  摘要
    告诉 Venus 一vnode 不再被使用

  参数

```
		struct cfs_inactive_in {
		    ViceFid VFid;
		} cfs_inactive;
```

     out

	none

  描述
    此操作返EOPNOTSUPP

  .. Note:: 这也许应该被删除


### 4.23. rdwr


  摘要
    从文件读或写

  参数

```
		struct cfs_rdwr_in {
		    ViceFid     VFid;
		    int rwflag;
		    int count;
		    int offset;
		    int ioflag;
		    caddr_t     data;           /* Place holder for data. */
		} cfs_rdwr;
```

     out锛。

```
		struct cfs_rdwr_out {
		    int rwflag;
		    int count;
		    caddr_t     data;   /* Place holder for data. */
		} cfs_rdwr;
```

  描述
    此上行调用要Venus 从文件读或写

  .. Note::

    它应该被删除，因为读/写操作永远不会到Venus，这违背Coda
    的理念。有人告诉我该操作不起作用。它当前未被使用


### 4.24. odymount


  摘要
    允许在一Unix 挂载点上挂载多个 Coda "文件系统"

  参数

```
		struct ody_mount_in {
		    char        *name;          /* Place holder for data. */
		} ody_mount;
```

     out锛。

```
		struct ody_mount_out {
		    ViceFid VFid;
		} ody_mount;
```

  描述
    要求 Venus 返回名为 name Coda 系统rootfid。fid 
    VFid 中返回

  .. Note::

     此调用曾David 用于动态集合（dynamic sets）。它应该被删除，
     因为它在 VFS 挂载区域造成指针丛林。Coda 本身不使用它。Venus 
     实现此调用


### 4.25. ody_lookup


  摘要
    查找某物

  参数
     in

	irrelevant

     out

	irrelevant

  .. Note:: 去掉它。Venus 未实现此调用


### 4.26. ody_expand


  摘要
    展开动态集合中的某物

  参数
     in

	irrelevant

     out

	irrelevant

  .. Note:: 去掉它。Venus 未实现此调用


### 4.27. prefetch


  摘要
    预取动态集合

  参数
     in

	Not documented.

     out

	Not documented.

  描述
    Venus worker.cc 支持此调用，尽管注意到它不起作用。毫不奇怪，
    因为内核不支持它。（ODY_PREFETCH 不是已定义的操作）

  .. Note:: 去掉它。它不起作用，且 Coda 未使用它


### 4.28. signal


  摘要
    Venus 发送关于上行调用的信号

  参数
     in

	none

     out

	not applicable.

  描述
    这是一个发Venus 的带外（out-of-band）上行调用，用于通知
    Venus 调用进程Venus 从输入队列读取消息后收到了一signal
    Venus 应清理该操作

  错误
    不给出回复

     我们需要更好地理解 Venus 需要清理什么，以及它是否正确地执行
     清理。我们还需要正确处理每个系统调用多upcall 的情况。了解在
     upcall 之后 Venus 中发生了哪些状态变化很重要，内核负责通知 Venus
     清理这些变化（例open 肯定是这样的状态变化，但许多其他的也许
     不是）


## 5. 迷你缓存与下行调用（downcall


  Coda FS 驱动可以缓存 lookup access 上行调用的结果，以限制上行调
  的频率。上行调用是有代价的，因为需要进行进程上下文切换。缓存信息的
  对应面是，Venus 将通知 FS 驱动缓存条目必须被刷新或重命名

  内核代码通常必须维护一个结构，将内部文件句柄（BSD 中称vnodes
  Linux 中称inodes，在 Windows 中称FileHandles）与 Venus 维护
  ViceFid 关联起来。原因是，为了进行上行调用并使用上行调用的结果，
  需要频繁地来回转换。此类链接对象称cnodes

  当前的迷你缓存实现拥有的缓存条目记录如下

  1. 文件的名

  2. 包含该对象的目录cnode

  3. 允许进行 lookup CodaCred 列表

  4. 该对象的 cnode

  Coda FS 驱动中的 lookup 调用可以通过传递其名称、目录和调用者的
  CodaCred，从缓存请求所需对象cnode。缓存将返回 cnode，或指示找不到
  Coda FS 驱动在修改或删除对象时必须小心地使缓存条目失效

  Venus 获得指示缓存条目不再有效的信息时，它将向内核发出下行调用
  （downcall）。下行调用被 Coda FS 驱动拦截，并导致如下所述的缓存失效
  Coda FS 驱动不返回错误，除非下行调用数据无法读入内核内存


### 5.1. INVALIDATE


  关于此调用没有可用信息


### 5.2. FLUSH


  参数
    None

  摘要
    完全刷新名称缓存

  描述
    Venus 在启动和退出时发出此调用。这是为了防止保留陈旧的缓存
    信息。某些操作系统允许动态关闭内核名称缓存。当这样做时，会进行
    此下行调用


### 5.3. PURGEUSER


  参数

```
	  struct cfs_purgeuser_out {/* CFS_PURGEUSER is a venus->kernel call */
	      struct CodaCred cred;
	  } cfs_purgeuser;
```

  描述
    移除缓存中所有携带该 Cred 的条目。当用户token 过期或被
    刷新时发出此调用


### 5.4. ZAPFILE


  参数

```
	  struct cfs_zapfile_out {  /* CFS_ZAPFILE is a venus->kernel call */
	      ViceFid CodaFid;
	  } cfs_zapfile;
```

  描述
    移除所有具有（dir vnode, name）对的条目。这是由vnode 
    缓存属性失效而发出的

  .. Note::

     NetBSD Mach 中此调用命名不正确。迷你缓zapfile 例程
     采用不同的参数。Linux 未正确实现属性的失效


### 5.5. ZAPDIR


  参数

```
	  struct cfs_zapdir_out {   /* CFS_ZAPDIR is a venus->kernel call */
	      ViceFid CodaFid;
	  } cfs_zapdir;
```

  描述
    移除缓存中位于目CodaFid 中的所有条目，以及该目录的所
    子项。当 Venus 收到该目录的回调时发出此调用


### 5.6. ZAPVNODE


  参数

```
	  struct cfs_zapvnode_out { /* CFS_ZAPVNODE is a venus->kernel call */
	      struct CodaCred cred;
	      ViceFid VFid;
	  } cfs_zapvnode;
```

  描述
    移除缓存中所有携带参数中 cred VFid 的条目。此下行调用
    可能从未被发出


### 5.7. PURGEFID


  参数

```
	  struct cfs_purgefid_out { /* CFS_PURGEFID is a venus->kernel call */
	      ViceFid CodaFid;
	  } cfs_purgefid;
```

  描述
    刷新文件的属性。如果它是目录（奇数 vnode），则从名称缓存
    中清除其子项，并从名称缓存中移除该文件


### 5.8. REPLACE


  摘要
    替换一组名称的 Fid

  参数

```
	  struct cfs_replace_out { /* cfs_replace is a venus->kernel call */
	      ViceFid NewFid;
	      ViceFid OldFid;
	  } cfs_replace;
```

  描述
    此例程将名称缓存中的 ViceFid 替换为另一个。添加它是为了允
    Venus 在重新集成（reintegration）期间，即使这些 fid 的引用计数不为零
    也能用全局 fid 替换断开连接时本地分配的临时 fid


## 6. 初始化与清理


  本节简要提Coda FS 驱动在启动、关闭或 Venus 故障时应具备的可
  特性。在讨论之前，重申一Coda FS 驱动维护以下数据是有用的


  1. 消息队列

  2. cnodes

  3. 名称缓存条目

     名称缓存条目完全由驱动私有，因此可以轻松操作。消息队列通常
     有明确的初始化和销毁点。cnodes 则要微妙得多。用户进程在 Coda
     文件系统中持有引用计数，清理 cnodes 可能很困难

  它可以通过如下方式收到请求

  1. 消息子系

  2. VFS 灞。

  3. pioctl 接口

     当前 pioctl 通过 Coda VFS 传递，因此我们可以类似地处理这些


### 6.1. 需


  应满足以下需求：

  1. 消息队列应有打开和关闭例程。在 Unix 上，字符设备的打开就是
     此类例程

    - 打开之前，不能放置任何消息

    - 打开将移除任何仍在挂起的旧消息

    - 关闭将通知任何睡眠的进程，它们upcall 无法完成

    - 关闭将释放消息队列分配的所有内存

  2. 在打开时，名称缓存应被初始化为空状态

  3. 在消息队列打开之前，所VFS 操作都将失败。幸运的是，这可以通过
     确保挂载 Coda 文件系统在打开之前不能成功来实现

  4. 关闭队列后，没有任何 VFS 操作能成功。这里需要小心，因为少数操作
     （lookup、read/write、readdir）可以在没有 upcall 的情况下进行
     这些必须被显式阻止

  5. 关闭时，名称缓存应被刷新并禁用

  6. 所有由 cnodes 持有的内存可以在不依upcall 的情况下释放

  7. 卸载文件系统可以在不依赖 upcall 的情况下完成

  8. 如果 Venus 无法获取 rootfid rootfid 的属性，挂载 Coda 文件
     系统应优雅地失败。后者最好由 Venus 在尝试挂载之前获取这些对
     来实现

     NetBSD 尤其Linux 尚未完全实现上述需求。为了顺畅运行，这需
     被纠正
