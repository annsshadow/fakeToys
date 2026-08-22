## /proc/sys/fs/ 文档


Copyright (c) 1998, 1999,  Rik van Riel <riel@nl.linux.org>

Copyright (c) 2009,        Shen Feng<shen@cn.fujitsu.com>

有关一般信息与法律声明，请参见 intro.rst
------------------------------------------------------------------------------

本文件包`/proc/sys/fs/` sysctl 文件与目录的文档。该目录中的文件可用于调整和监控 Linux 内核运行过程中的各种杂项与通用事务。由于其中某些文*确实**可能把你的系统搞乱，建议在实际调整之前同时阅读文档与源代码
## 1. /proc/sys/fs


目前，这些文件可能（取决于你的配置）出现`/proc/sys/fs` 中：



### aio-nr & aio-max-nr


`aio-nr` 显示当前系统范围内异io 请求的数量。`aio-max-nr` 允许你更`aio-nr` 可以增长到的上限。如`aio-nr` 达到 `aio-nr-max`，则 `io_setup` 将因 `EAGAIN` 而失败。注意，提高 `aio-max-nr` 并不会导致任何内核数据结构的预分配或重新调整大小
### dentry-negative


dentry 的策略。设1 表示在文件被删除时总是删除dentry，设0 表示禁用。默认情况下，此行为被禁用
### dentry-state


本文件显`struct dentry_stat_t` 中的值，其定义见
```

  struct dentry_stat_t dentry_stat {
        long nr_dentry;
        long nr_unused;
        long age_limit;         /* age in seconds */
        long want_pages;        /* pages requested by system */
        long nr_negative;       /* # of unused negative dentries */
        long dummy;             /* Reserved for future use */
  };

```
dentry 是动态分配与释放的
`nr_dentry` 显示已分配的 dentry 总数（活- 未使用）。`nr_unused` 显示未被主动使用、但保存LRU 列表中以备将来复用的 dentry 数量
`age_limit` 是以秒为单位的年龄，当内存紧张且 `shrink_dcache_pages()` 已被调用dcache 尚未被修剪时，`want_pages` 在非零时表示系统请求的页数，超过该年龄的 dcache 条目可被回收
`nr_negative` 显示未使用且为负 dentry（不映射到任何文件）dentry 数量。它们有助于加速拒绝用户给出的不存在文件

### file-max & file-nr


`file-max` 中的值表Linux 内核将分配的文件句柄的最大数量。当你收到大量关于文件句柄耗尽的错误消息时，可能希望提高此限制
历史上，内核能够动态分配文件句柄，但无法再次释放它们。`file-nr` 中的三个值分别表示已分配的文件句柄数、已分配但未使用的文件句柄数，以及文件句柄的最大数量。Linux 2.6 及以后版本总是将空闲文件句柄数报告0——这并非错误，只是意味着已分配的文件句柄数与已使用的文件句柄数恰好相等
尝试分配超过 `file-max` 的文件描述符会在内核日志中显示：

```

  VFS: file-max limit <number> reached

```


### inode-nr & inode-state


与文件句柄一样，内核动态分inode 结构，但尚无法释放它们
`inode-nr` 文件包含 `inode-state` 的前两项，因此我们直接跳到该文件…
`inode-state` 包含三个实际数字和四个占位项。实际数字按出现顺序分别`nr_inodes`、`nr_free_inodes` `preshrink`
`nr_inodes` 表示系统已分配的 inode 数量
`nr_free_inodes` 表示空闲 inode 的数量（），当系统需要修inode 列表而不是分配更多时，preshrink 为非零

### mount-max


这表示挂载命名空间中可能存在的最大挂载数量

### nr_open


这表示进程可以分配的文件句柄的最大数量。默认值为 1024*1024048576），对大多数机器应已足够。实际限制取决于 `RLIMIT_NOFILE` 资源限制

### overflowgid & overflowuid


某些文件系统只支16 UID GID，尽管在 Linux UID GID 32 位。当此类文件系统以可写方式挂载时，任何超65535 UID GID 在写入磁盘前会被转换为一个固定值
这些 sysctl 允许你更改该固定 UID GID 的值。默认值为 65534

### pipe-user-pages-hard


非特权用户可为管道分配的最大页面总数。一旦达到此限制，在用量重新降至限制以下之前，不能再分配新管道。设0 时不施加限制，这也是默认设置

### pipe-user-pages-soft


非特权用户可为管道分配的页面最大总数，超过后管道大小会被限制为两页。一旦达到此限制，该用户的新管道大小将被限制为两页，以限制总内存用量；并且使用 `fcntl()` 尝试增大它们的请求会被拒绝，直到用量重新降至限制以下。默认值允许以默认大小分配最1024 个管道。设0 时不施加限制

### protected_fifos


此保护的目的是避免程序本想创建普通文件却意外写入到攻击者控制的 FIFO。设"0" 时，FIFO 的写入不受限制。设"1" 时，禁止在我们不拥有的、位于全局可写 sticky 目录中的 FIFO 上以 `O_CREAT` 打开，除非它们由该目录的所有者拥有。设"2" 时，此规则也适用于组可写 sticky 目录。此保护基于 Openwall 中的限制

### protected_hardlinks


一类长期存在的安全问题是基于硬链接的“检查时使用时间”（TOCTOU）竞态，最常见`/tmp` 这样的全局可写目录。利用此缺陷的常见方法是在跟踪给定硬链接时跨越权限边界（root 进程跟踪另一用户创建的硬链接）。此外，在没有分离分区的系统上，这可阻止未授权用户“钉住”易受攻击的 setuid/setgid 文件以防止被管理员升级，或链接到特殊文件。设"0" 时，硬链接创建行为不受限制。设"1" 时，若用户不拥有源文件，或对其没有读/写访问权限，则用户无法创建硬链接。此保护基于 Openwall grsecurity 中的限制

### protected_regular


此保护类似于 `protected_fifos`_，但它避免程序本想创建一个普通文件却写入到攻击者控制的普通文件。设"0" 时，对普通文件的写入不受限制。设"1" 时，禁止在我们不拥有的、位于全局可写 sticky 目录中的普通文件上`O_CREAT` 打开，除非它们由该目录的所有者拥有。设"2" 时，此规则也适用于组可写 sticky 目录

### protected_symlinks


一类长期存在的安全问题是基于符号链接的“检查时使用时间”（TOCTOU）竞态，最常见`/tmp` 这样的全局可写目录。利用此缺陷的常见方法是在跟踪给定符号链接时跨越权限边界（即 root 进程跟踪属于另一用户的符号链接）。有关多年来数百个例子可能不完整的列表，请参见：https://cve.mitre.org/cgi-bin/cvekey.cgikeyword=/tmp 设为 "0" 时，符号链接跟踪行为不受限制。设"1" 时，仅当位于 sticky 全局可写目录之外，或符号链接uid 与跟踪者的 uid 匹配，或目录所有者与符号链接的所有者匹配时，才允许跟踪符号链接。此保护基于 Openwall grsecurity 中的限制

### suid_dumpable


此值可用于查询和设setuid 或其它受保护/被污染二进制core dump 模式。模式如下：

=   ==========  ===============================================================
0   (default)	传统行为。任何已更改权限级别或仅可执行的进程都不会被转储1   (debug)	所有可能的情况下所有进程都转储 core。core dump 由当前用户拥有，
		不施加任何安全性。这仅用于系统调试场景。Ptrace 不被检查		这是不安全的，因为它允许普通用户检查特权进程的内存内容2   (suidsafe)	任何通常不应被转储的二进制无论如何都会被转储，但前提		`core_pattern` 内核 sysctl（见
		Documentation/admin-guide/sysctl/kernel.rst <core_pattern>		被设置为管道处理程序或完全限定路径。（有关此限制的更多细节		请参CVE-2006-2451。）当管理员试图在正常环境中调试问题时，
		此模式是合适的——此时要么有一个懂得谨慎处理特core dump 		core dump 管道处理程序，要么已定义用于捕获 core dump 的特定目录		如果 core dump 在没有管道处理程序或完全限定路径的情况下发生		将向 syslog 发出一条消息，警告缺少正确设置=   ==========  ===============================================================




## 2. /proc/sys/fs/binfmt_misc


`/proc/sys/fs/binfmt_misc` 中文件的文档位于 Documentation/admin-guide/binfmt-misc.rst

## 3. /proc/sys/fs/mqueue - POSIX 消息队列文件系统



"mqueue" 文件系统提供必要的内核特性，以支持创建一个实POSIX 消息队列 API 的用户空间库（如 POSIX 1003.1-2001 版本 System Interfaces 规范MSG 标记所示）
"mqueue" 文件系统包含用于确定/设置该文件系统所使用资源量的值
`/proc/sys/fs/mqueue/queues_max` 是一个可写文件，用于设置/获取系统允许的最大消息队列数量
`/proc/sys/fs/mqueue/msg_max` 是一个可写文件，用于设置/获取队列中消息数量的最大值。实际上它是另一个（用户）限制的限制值，该限制在 `mq_open` 调用中设置。队列的这一属性必须小于或等于 `msg_max`
`/proc/sys/fs/mqueue/msgsize_max` 是一个可写文件，用于设置/获取最大消息大小值（它是每个消息队列的属性，在创建时设置）
`/proc/sys/fs/mqueue/msg_default` 是一个可写文件，用于设置/获取队列中消息的默认数量（当 `mq_open(2)` `attr` 参数`NULL` 时）。如果超`msg_max`，默认值被初始化为 `msg_max`
`/proc/sys/fs/mqueue/msgsize_default` 是一个可写文件，用于设置/获取默认消息大小值（`mq_open(2)` `attr` 参数`NULL` 时）。如果超`msgsize_max`，默认值被初始化为 `msgsize_max`
## 4. /proc/sys/fs/epoll - epoll 接口的配置选项


此目录包epoll(7) 接口的配置选项
### max_user_watches


每个 epoll 文件描述符可以存储若干被监控以等待事件就绪的文件。这些被监控文件中的每一个构成一个“watch”。此配置选项设置每个用户允许的最大“watch”数量。每个“watch”在 32 位内核上约耗费 90 字节，在 64 位内核上约耗费 160 字节。`max_user_watches` 的当前默认值为可用低端内存4%，除以“watch”的字节开销
## 5. /proc/sys/fs/fuse - FUSE 文件系统的配置选项


此目录包FUSE 文件系统的以下配置选项
`/proc/sys/fs/fuse/max_pages_limit` 是一个可写文件，用于设置/获取可用于服FUSE 中请求的最大页数
`/proc/sys/fs/fuse/default_request_timeout` 是一个可写文件，用于设置/获取 fuse 服务端回复内核发出请求的默认超时时间（秒），适用场景为服务端在挂载时未指定超时。如果服务端设置了超时，default_request_timeout 将被忽略。默"default_request_timeout" 设为 0 表示无默认超时。可设置的最大值为 65535
`/proc/sys/fs/fuse/max_request_timeout` 是一个可写文件，用于设置/获取 fuse 服务端回复内核发出请求的最大超时时间（秒）。大0 的值会自动使服务端采用一个至多设置为 "max_request_timeout" 的超时，即使服务端未指定超时default_request_timeout 设为 0 也是如此。如max_request_timeout 大于 0，且服务端设置的超时大于 max_request_timeout，或 default_request_timeout 被设为大max_request_timeout 的值，则系统将使用 max_request_timeout 作为超时 表示无最大请求超时。可设置的最大值为 65535
关于超时：如果服务端在设定的超时时间耗尽前未响应请求，则fuse 服务端的连接将被中止。请注意，超时并100% 精确（例如你可能设置 60 秒，但超时可能在 70 秒后才触发）。超时的误差上限约为 FUSE_TIMEOUT_TIMER_FREQ 秒