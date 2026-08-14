## Linux Security Modules: General Security Hooks for Linux


:Author: Stephen Smalley
:Author: Timothy Fraser
:Author: Chris Vance


   本书中描述的 API 已经过时。

## Introduction


2001 年 3 月，美国国家安全局（NSA）在 2.5 Linux Kernel Summit 上做了一个关于 Security-Enhanced Linux（SELinux）的演讲。SELinux 是 Linux 内核中灵活且细粒度非自主访问控制的一种实现，最初作为它自己特定的内核补丁实现。其他几个安全项目（例如 RSBAC、Medusa）也为 Linux 内核开发了灵活的访问控制架构，并且各种项目为 Linux 开发了特定的访问控制模型（例如 LIDS、DTE、SubDomain）。每个项目都开发并维护了自己的内核补丁以支持其安全需求。

作为对 NSA 演讲的回应，Linus Torvalds 发表了一系列评论，描述了一个他愿意考虑纳入主线 Linux 内核的安全框架。他描述了一个通用框架，该框架将提供一组安全钩子来控制对内核对象的操作，并在内核数据结构中提供一组不透明的安全字段以维护安全属性。然后，这个框架可以被可加载内核模块用来实现任何期望的安全模型。Linus 还建议将 Linux capabilities 代码迁移到这样的模块中。

Linux Security Modules（LSM）项目由 WireX 发起，旨在开发这样一个框架。LSM 是多个安全项目（包括 Immunix、SELinux、SGI 和 Janus）以及多个个人（包括 Greg Kroah-Hartman 和 James Morris）联合开发的成果，目的是开发实现该框架的 Linux 内核补丁。该工作在 2003 年 12 月被纳入主线。本技术报告概述了该框架和 capabilities 安全模块。

## LSM Framework


LSM 框架提供了一个通用的内核框架以支持安全模块。具体而言，LSM 框架主要关注支持访问控制模块，尽管未来的开发可能会解决其他安全需求，例如沙箱。框架本身不提供任何额外的安全性；它仅仅提供支持安全模块的基础设施。LSM 框架是可选的，需要启用 `CONFIG_SECURITY`。capabilities 逻辑被实现为一个安全模块。
这个 capabilities 模块在 `LSM Capabilities Module`_ 中进一步讨论。

LSM 框架在内核数据结构中包含安全字段，并在内核代码的关键点调用钩子函数，以管理安全字段并执行访问控制。
它还添加了用于注册安全模块的函数。
接口 `/sys/kernel/security/lsm` 报告系统上处于活动状态的安全模块以逗号分隔的列表。

LSM 安全字段只是 `void*` 指针。
这些数据被称为 blob，它可由框架管理，也可由使用它的各个安全模块管理。
被多个安全模块使用的安全 blob 通常由框架管理。
对于进程和
程序执行的安全信息，安全字段包含在 `struct task_struct <task_struct>` 和 `struct cred <cred>` 中。
对于文件系统
的安全信息，安全字段包含在 :c:type:`struct super_block <super_block>` 中。对于管道、文件和套接字的安全
信息，安全字段包含在 :c:type:`struct inode <inode>` 和 `struct file <file>` 中。
对于 System V IPC 的安全信息，
安全字段被添加到 :c:type:`struct kern_ipc_perm <kern_ipc_perm>` 和 :c:type:`struct msg_msg <msg_msg>` 中；此外，:c:type:`struct msg_msg <msg_msg>`、struct msg_queue 和 struct shmid_kernel 的定义
被移动到头文件中（分别为 `include/linux/msg.h` 和 `include/linux/shm.h`），以允许安全模块使用这些定义。

对于数据包和
网络设备的安全信息，安全字段被添加到 `struct sk_buff <sk_buff>` 和 `struct scm_cookie <scm_cookie>` 中。
与其他安全模块数据不同，这里使用的数据是一个 32 位整数。安全模块需要把这些值映射或以其他方式与真实的安全属性关联起来。

LSM 钩子维护在列表中。每个钩子维护一个列表，钩子按 CONFIG_LSM 指定的顺序调用。
每个钩子的详细文档包含在 `security/security.c` 源文件中。

LSM 框架提供了对通用安全模块堆叠的近似支持。它定义了 security_add_hooks()，每个安全模块向它传递一个 `struct security_hooks_list <security_hooks_list>`，这些被添加到列表中。
LSM 框架不提供移除已注册钩子的机制。SELinux 安全模块实现了一种移除自身的方法，但该特性已被弃用。

钩子可以看作分为两大类：用于管理安全字段的钩子和用于执行访问控制的钩子。第一类钩子的例子包括 security_inode_alloc() 和 security_inode_free()
这些钩子用于为 inode 对象分配和释放安全结构。
第二类钩子的一个例子是 security_inode_permission() 钩子。
该钩子在访问 inode 时检查权限。

## LSM Capabilities Module


POSIX.1e capabilities 逻辑作为存储在 `security/commoncap.c` 文件中的安全模块维护。capabilities 模块使用 `lsm_info` 描述的 order 字段将其标识为要注册的第一个安全模块。
与其他模块不同，capabilities 安全模块不使用通用安全 blob。原因是历史性的，基于开销、复杂性和性能方面的考虑。
