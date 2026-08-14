
## ORANGEFS


OrangeFS 是一个 LGPL 的用户空间横向扩展（scale-out）并行存储系统。它非常适合 HPC、
大数据（BigData）、流媒体视频（Streaming Video）、基因组学（Genomics）、生物信息学
（Bioinformatics）所面临的大规模存储问题。

Orangefs 最初称为 PVFS，于 1993 年由 Walt Ligon 和 Eric Blumer 首次开发，作为一个
并行虚拟机（Parallel Virtual Machine, PVM）的并行文件系统，作为 NASA 资助研究并行
程序 I/O 模式的一部分。

Orangefs 的特性包括：

  - 在多个文件服务器之间分布文件数据
  - 支持多个客户端同时访问
  - 使用本地文件系统和访问方法在服务器上存储文件数据和元数据
  - 用户空间实现易于安装和维护
  - 直接的 MPI 支持
  - 无状态（Stateless）


## 邮件列表归档


http://lists.orangefs.org/pipermail/devel_lists.orangefs.org/


## 邮件列表投稿


devel@lists.orangefs.org


## 文档


http://www.orangefs.org/documentation/

## 在单台服务器上运行 ORANGEFS


OrangeFS 通常在具有多个服务器和客户端的庞大部署中运行，但为了开发和测试，也可以在单台
机器上运行一个完整的文件系统。

```
    dnf -y install orangefs orangefs-server
```

在 /etc/orangefs/orangefs.conf 中有一个示例服务器配置文件。如有必要，将 localhost 改为
你的主机名（hostname）。

关于生成一个用于运行 xfstests 的文件系统，请见下文。

在 /etc/pvfs2tab 中有一个示例客户端配置文件。它是单行。取消其注释，并在必要时更改
主机名。这控制使用 libpvfs2 的客户端。这并不控制 pvfs2-client-core。

```
    pvfs2-server -f /etc/orangefs/orangefs.conf
```

```
    systemctl start orangefs-server
```

```
    pvfs2-ping -m /pvfsmnt
```

启动客户端。在此操作之前，模块必须已编译进内核或已加载：

```
    systemctl start orangefs-client
```

```
    mount -t pvfs2 tcp://localhost:3334/orangefs /pvfsmnt
```

## 用户空间文件系统源码


http://www.orangefs.org/download

2.9.3 之前的 Orangefs 版本与上游版本的内核客户端不兼容。


## 在单台服务器上构建 ORANGEFS


如果 OrangeFS 无法从发行版软件包安装，可以从源码构建。

如果你不介意东西散落在 /usr/local 各处，可以省略 --prefix。从 2.9.6 版本起，OrangeFS
默认使用 Berkeley DB，我们可能很快会将默认值改为 LMDB。

```
    ./configure --prefix=/opt/ofs --with-db-backend=lmdb --disable-usrint

    make

    make install
```

通过运行 pvfs2-genconfig 并指定一个目标配置文件来创建 orangefs 配置文件。Pvfs2-genconfig
会通过提示引导你完成。通常直接采用默认值即可，但你应当使用你的服务器主机名，而不是
“localhost”：

```
    /opt/ofs/bin/pvfs2-genconfig /etc/pvfs2.conf
```

```
    echo tcp://localhost:3334/orangefs /pvfsmnt pvfs2 defaults,noauto 0 0 > \
	/etc/pvfs2tab
```

```
    mkdir /pvfsmnt
```

```
    /opt/ofs/sbin/pvfs2-server -f /etc/pvfs2.conf
```

```
    /opt/ofs/sbin/pvfs2-server /etc/pvfs2.conf
```

现在服务器应当已经在运行。Pvfs2-ls 是一个简单的：

```
    /opt/ofs/bin/pvfs2-ls /pvfsmnt
```

如果一切似乎工作正常，加载内核模块并执行：

```
    /opt/ofs/sbin/pvfs2-client -p /opt/ofs/sbin/pvfs2-client-core
```

```
    mount -t pvfs2 tcp://`hostname`:3334/orangefs /pvfsmnt
```

## 运行 xfstests


将 xfstests 与 scratch 文件系统配合使用很有用。这可以只使用一台服务器来完成。

在服务器配置文件（即 /etc/orangefs/orangefs.conf）中复制一份 FileSystem 段。将 Name
改为 scratch。将 ID 改为与第一个 FileSystem 段的 ID 不同的值（2 通常是个好选择）。

这样就有两个 FileSystem 段：orangefs 和 scratch。

此更改应在创建文件系统之前进行。

```
    pvfs2-server -f /etc/orangefs/orangefs.conf
```

```
    TEST_DIR=/orangefs
    TEST_DEV=tcp://localhost:3334/orangefs
    SCRATCH_MNT=/scratch
    SCRATCH_DEV=tcp://localhost:3334/scratch
```

```
    ./check -pvfs2
```

## 选项


接受以下挂载（mount）选项：

  acl
    允许在文件和目录上使用访问控制列表（Access Control List）。

  intr
    内核客户端与用户空间文件系统之间的一些操作可以被中断（interruptible），例如
    调试（debug）级别的更改和 tunable 参数的设置。

  local_lock
    从 “本” 内核的视角启用 posix 锁定。默认的 file_operations 锁定动作是返回 ENOSYS。
    如果文件系统以 -o local_lock 挂载，则 posix 锁定生效。分布式锁定正在为未来进行
    开发中。


## 调试


如果你想在特定的 GOSSIP 语句中启用调试，则：

```
  echo inode > /sys/kernel/debug/orangefs/kernel-debug
```

```
  echo none > /sys/kernel/debug/orangefs/kernel-debug
```

```
  echo inode,dir > /sys/kernel/debug/orangefs/kernel-debug
```

```
  echo all > /sys/kernel/debug/orangefs/kernel-debug
```

```
  cat /sys/kernel/debug/orangefs/debug-help
```

## 内核模块与用户空间之间的协议


Orangefs 是一个用户空间文件系统以及相关联的内核模块。此后我们将 Orangefs 的用户空间
部分简称为 “userspace”。Orangefs 源自 PVFS，而用户空间代码在函数和变量名中仍然使用
PVFS。用户空间 typedef 了许多重要的结构。内核模块中的函数和变量名已经过渡到
“orangefs”，而且 Linux 编码风格（Coding Style）避免使用 typedef，因此与用户空间结构
对应的内核模块结构没有被 typedef。

内核模块实现了一个伪设备（pseudo device），用户空间可以对其进行读和写。用户空间还可以
通过伪设备用 ioctl 操控内核模块。

### Bufmap（缓冲区映射）


在启动时，用户空间分配两个按页大小对齐（posix_memalign）的 mlocked 内存缓冲区，一个
用于 IO，一个用于 readdir 操作。IO 缓冲区为 41943040 字节，readdir 缓冲区为 4194304
字节。每个缓冲区包含逻辑块（chunk）或分区（partition），并且每个缓冲区的指针被加入其
自己的 PVFS_dev_map_desc 结构中，该结构还描述了其总大小，以及分区的大小和数量。

指向 IO 缓冲区的 PVFS_dev_map_desc 结构的指针通过 ioctl 被发送给内核模块中的一个映射
例程。该结构通过 copy_from_user 从用户空间复制到内核空间，并用于初始化内核模块的
“bufmap”（struct orangefs_bufmap），其随后包含：

  - refcnt
    - 一个引用计数器
  - desc_size - PVFS2_BUFMAP_DEFAULT_DESC_SIZE (4194304) - IO 缓冲区的
    分区大小，代表文件系统的块大小，并用于超级块（super block）中的 s_blocksize。
  - desc_count - PVFS2_BUFMAP_DEFAULT_DESC_COUNT (10) - IO 缓冲区中的分区数量。
  - desc_shift - log2(desc_size)，用于超级块中的 s_blocksize_bits。
  - total_size - IO 缓冲区的总大小。
  - page_count - IO 缓冲区中 4096 字节页的数量。
  - page_array - 指向 `page_count * (sizeof(struct page *))` 字节的 kcalloced
    内存的指针。该内存通过调用 get_user_pages 被用作指向 IO 缓冲区中每个页的指针数组。
  - desc_array - 指向 `desc_count * (sizeof(struct orangefs_bufmap_desc))` 字节的
    kcalloced 内存的指针。该内存被进一步初始化：

      user_desc 是 IO 缓冲区的 ORANGEFS_dev_map_desc 结构的内核副本。
      user_desc->ptr 指向 IO 缓冲区。

```
	pages_per_desc = bufmap->desc_size / PAGE_SIZE
	offset = 0

        bufmap->desc_array[0].page_array = &bufmap->page_array[offset]
        bufmap->desc_array[0].array_count = pages_per_desc = 1024
        bufmap->desc_array[0].uaddr = (user_desc->ptr) + (0 * 1024 * 4096)
        offset += 1024
                           .
                           .
                           .
        bufmap->desc_array[9].page_array = &bufmap->page_array[offset]
        bufmap->desc_array[9].array_count = pages_per_desc = 1024
        bufmap->desc_array[9].uaddr = (user_desc->ptr) +
                                               (9 * 1024 * 4096)
        offset += 1024

  * buffer_index_array - 一个 desc_count 大小的 int 数组，用于指示 IO 缓冲区的
    哪些分区可供使用。
  * buffer_index_lock - 一个自旋锁（spinlock），用于在更新期间保护 buffer_index_array。
  * readdir_index_array - 一个五（ORANGEFS_READDIR_DEFAULT_DESC_COUNT）元素的 int
    数组，用于指示 readdir 缓冲区的哪些分区可供使用。
  * readdir_index_lock - 一个自旋锁，用于在更新期间保护 readdir_index_array。
```

### 操作（Operations）


当内核模块需要与用户空间通信时，它会构建一个 “op”（struct orangefs_kernel_op_s）。op
的一部分包含向用户空间表达请求的 “upcall（上行调用）”。op 的一部分最终包含表达请求
结果的 “downcall（下行调用）”。

slab 分配器被用来保持一个随时可用的 op 结构缓存。

在初始化时，内核模块定义并初始化一个请求列表（request list）和一个 in_progress 哈希表
（hash table），以跟踪在任何给定时刻所有在途（in flight）的 op。

Op 是有状态的：

 - unknown
     - op 刚刚被初始化
 - waiting
     - op 在 request_list 上（向上等待）
 - inprogr
     - op 正在进行中（等待 downcall）
 - serviced
     - op 有匹配的 downcall；正常
 - purged
     - op 必须启动一个定时器，因为 client-core 在服务于该 op 之前不干净地退出了
 - given up
     - 提交者已放弃等待它

当某个任意的用户空间程序需要在 Orangefs 上执行一个文件系统操作（readdir、I/O、create
或其它）时，会初始化一个 op 结构并打上一个用于区分的 ID 号标签。op 的 upcall 部分被
填充，然后该 op 被传递给 “service_operation” 函数。

service_operation 将 op 的状态改为 “waiting”，将其放入请求列表，并通过等待队列（wait
queue）向 Orangefs 的 file_operations.poll 函数发信号。用户空间正在轮询（poll）伪设备，
从而得知需要被读取的 upcall 请求。

当 Orangefs 的 file_operations.read 函数被触发时，会在请求列表中搜索一个似乎已准备好
处理的 op。该 op 从请求列表中移除。op 的 tag 和已填充的 upcall 结构通过 copy_to_user
复制回用户空间。

如果这些 copy_to_user（以及一些额外的协议）中有任何失败，op 的状态被设为 “waiting”，
并且该 op 被加回请求列表。否则，op 的状态被改为 “in progress”，并且该 op 按其 tag 被
哈希（hash），放到 in_progress 哈希表中该 tag 所哈希到的索引处的列表末尾。

当用户空间组装好对 upcall 的响应后，它将包含该区分 tag 的响应，以一系列 io_vecs 写回
伪设备。这会触发 Orangefs 的 file_operations.write_iter 函数找到具有关联 tag 的 op，并
将其从 in_progress 哈希表中移除。只要该 op 的状态不是 “canceled” 或 “given up”，其
状态就被设为 “serviced”。file_operations.write_iter 函数返回到等待中的 vfs，并经由
wait_for_matching_downcall 返回到 service_operation。

service_operation 带着 op 的 downcall 部分（对 upcall 的响应）被填充完毕而返回给其调用者。

“client-core” 是内核模块与用户空间之间的桥梁。client-core 是一个守护进程（daemon）。
client-core 有一个相关联的看门狗（watchdog）守护进程。如果 client-core 被信号要求退出，
看门狗守护进程会重启 client-core。即使 client-core 被 “立即” 重启，在此类事件发生期间
仍有一段时间 client-core 是死的。死的 client-core 无法被 Orangefs 的
file_operations.poll 函数触发。在 “死亡期” 间通过 service_operation 的 op 可能会在等待
队列上超时，此时会尝试回收它们一次。显然，如果 client-core 死亡时间过长，试图使用
Orangefs 的那些任意用户空间进程将受到负面影响。无法被服务的等待中的 op 将从请求列表中
移除，并将其状态设为 “given up”。无法被服务中的进行中的 op 将从 in_progress 哈希表中
移除，并将其状态设为 “given up”。

readdir 和 I/O op 在负载（payload）方面是不典型的。

  - readdir op 使用两个预分配、预分区的较小内存缓冲区之一。readdir 缓冲区只能被用户空间
    使用。内核模块在发起 readdir op 之前获取一个空闲分区的索引。用户空间将结果存入该
    索引分区，然后将其写回 pvfs 设备。

  - io（读和写）op 使用两个预分配、预分区的较大内存缓冲区之一。IO 缓冲区既可从用户空间
    也可从内核模块访问。内核模块在发起 io op 之前获取一个空闲分区的索引。内核模块将写
    数据存入索引分区，直接供用户空间消费。用户空间将读请求的结果存入索引分区，直接供
    内核模块消费。

对内核请求的响应都被打包在 pvfs2_downcall_t 结构中。除了少数几个其它成员外，
pvfs2_downcall_t 包含一个结构体联合体（union），其中每个结构体都与一种特定的响应类型
相关联。

联合体外面的几个成员是：

 `int32_t type`
    - 操作类型。
 `int32_t status`
    - 操作的返回码。
 `int64_t trailer_size`
    - 除非是 readdir 操作，否则为 0。
 `char *trailer_buf`
    - 初始化为 NULL，在 readdir 操作期间使用。

联合体内部适当的成员会被针对任何特定响应而填充。

  PVFS2_VFS_OP_FILE_IO
    fill a pvfs2_io_response_t

  PVFS2_VFS_OP_LOOKUP
    fill a PVFS_object_kref

  PVFS2_VFS_OP_CREATE
    fill a PVFS_object_kref

  PVFS2_VFS_OP_SYMLINK
    fill a PVFS_object_kref

  PVFS2_VFS_OP_GETATTR
    fill in a PVFS_sys_attr_s（内核不需要的大量内容）
    当对象是符号链接（symlink）时，用一个包含链接目标的字符串填充。

  PVFS2_VFS_OP_MKDIR
    fill a PVFS_object_kref

  PVFS2_VFS_OP_STATFS
    fill a pvfs2_statfs_response_t with useless info <g>。我们很难及时地知道
    关于我们这个分布式网络文件系统的这些统计信息。

  PVFS2_VFS_OP_FS_MOUNT
    fill a pvfs2_fs_mount_response_t，它与 PVFS_object_kref 类似，只是其成员顺序不同，
    并且 “__pad1” 被替换为 “id”。

  PVFS2_VFS_OP_GETXATTR
    fill a pvfs2_getxattr_response_t

  PVFS2_VFS_OP_LISTXATTR
    fill a pvfs2_listxattr_response_t

  PVFS2_VFS_OP_PARAM
    fill a pvfs2_param_response_t

  PVFS2_VFS_OP_PERF_COUNT
    fill a pvfs2_perf_count_response_t

  PVFS2_VFS_OP_FSKEY
    file a pvfs2_fs_key_response_t

  PVFS2_VFS_OP_READDIR
    jamb everything needed to represent a pvfs2_readdir_response_t into
    the readdir buffer descriptor specified in the upcall。

用户空间使用 writev() 在 /dev/pvfs2-req 上传递对内核侧所发出请求的响应。

一个 buffer_list 包含：

  - 一个指向内核请求响应（struct pvfs2_downcall_t）的指针。
  - 此外，在 readdir 请求的情况下，一个指向包含目标目录中对象描述符的缓冲区的指针。

... 被发送给执行 writev 的函数（PINT_dev_write_list）。

PINT_dev_write_list 有一个局部 iovec 数组：struct iovec io_array[^10^];

io_array 的前四个元素对所有响应都像这样初始化：

```
  io_array[0].iov_base = address of local variable "proto_ver" (int32_t)
  io_array[0].iov_len = sizeof(int32_t)

  io_array[1].iov_base = address of global variable "pdev_magic" (int32_t)
  io_array[1].iov_len = sizeof(int32_t)

  io_array[2].iov_base = address of parameter "tag" (PVFS_id_gen_t)
  io_array[2].iov_len = sizeof(int64_t)

  io_array[3].iov_base = address of out_downcall member (pvfs2_downcall_t)
                         of global variable vfs_request (vfs_request_t)
  io_array[3].iov_len = sizeof(pvfs2_downcall_t)
```

```
  io_array[4].iov_base = contents of member trailer_buf (char *)
                         from out_downcall member of global variable
                         vfs_request
  io_array[4].iov_len = contents of member trailer_size (PVFS_size)
                        from out_downcall member of global variable
                        vfs_request
```

Orangefs 利用 dcache 以避免向用户空间发送冗余请求。我们通过 orangefs_inode_getattr
使对象的 inode 属性保持最新。orangefs_inode_getattr 使用两个参数来帮助它决定是否更新
一个 inode：“new” 和 “bypass”。Orangefs 在对象的 inode 中保存私有数据，其中包括一个
较短的超时值 getattr_time，它使 orangefs_inode_getattr 的任何一次迭代都能知道该 inode
自上次更新以来经过了多久。当对象不是新的（new == 0）且 bypass 标志未设置（bypass == 0）
时，如果 getattr_time 尚未超时，orangefs_inode_getattr 会不经更新直接返回。getattr_time
在每次更新 inode 时被刷新。

创建一个新对象（文件、目录、符号链接）包括对其路径名的解析，结果为该对象的一个负目录项
（negative directory entry）。分配一个新的 inode 并与该 dentry 关联，将其从一个负 dentry
变成 “对社会有贡献的正式一员”。Orangefs 通过 new_inode() 从 Linux 获取新的 inode，并通过
用 d_instantiate() 将该对（inode 和 dentry）送回 Linux 来将 inode 与 dentry 关联。

对对象路径名的解析会对应到其 dentry。如果没有对应的 dentry，则在 dcache 中为它创建一个。
每当一个 dentry 被修改或验证时，Orangefs 会在该 dentry 的 d_time 中存储一个较短的超时值，
在该段时间內该 dentry 会被信任。Orangefs 是一个网络文件系统，对象有可能在带外（out-of-band）
被任何特定的 Orangefs 内核模块实例改变，因此信任 dentry 是有风险的。信任 dentry 的替代
方案是总是从用户空间获取所需信息——至少是一次到 client-core 的往返，或许还要到服务器。
从 dentry 获取信息很便宜，而从用户空间获取信息相对昂贵，这就是尽可能使用 dentry 的动机。

超时值 d_time 和 getattr_time 是基于 jiffy 的，并且：

```
    "一般而言，如果时钟可能已经回绕（wrap around）超过一次，就无法判断已经过去了多少
    时间。然而，如果已知时间 t1 和 t2 相当接近，我们就可以以一种考虑到时钟可能在两次
    时间之间发生过回绕的可能性的方式，可靠地计算出差值。"
```

（引自 Andy Wang 讲师的课程笔记）
