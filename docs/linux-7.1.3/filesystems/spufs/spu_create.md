## spu_create


## 名称

       spu_create - 创建一个新的 spu 上下文


## 概要


```

         #include <sys/types.h>
         #include <sys/spu.h>

         int spu_create(const char *pathname, int flags, mode_t mode);

```
## 描述

       spu_create 系统调用用于实现 Cell 宽带引擎架构的 PowerPC 机器，以访问
       协同处理器单元（SPU）。它在 pathname 中为 SPU 创建一个新的逻辑上下文，
       并返回一个与之关联的处理句柄。pathname 必须指向 SPU 文件系统（spufs）
       挂载点中一个不存在的目录。spu_create 成功时，会在 pathname 处创建一个
       目录，并向其中填充文件。

       返回的文件句柄只能传递给 spu_run(2) 或关闭，其他操作未在其上定义。当
       它被关闭时，spufs 中所有关联的目录项会被移除。当指向上下文目录内部或
       指向此文件描述符的最后一个文件句柄被关闭时，该逻辑 SPU 上下文被销毁。

       参数 flags 可以为 0，也可以是下列常量按位或的组合：

       SPU_RAWIO
              允许将 SPU 的部分硬件寄存器映射到用户空间。此标志需要
              CAP_SYS_RAWIO 能力，参见 capabilities(7)。

       mode 参数指定在 spufs 中创建新目录时所使用的权限。mode 会与用户的
       umask(2) 值进行修饰，然后同时用于该目录及其包含的文件。文件权限会
       屏蔽 mode 中的更多位，因为它们通常只支持读或写访问。有关可能的 mode
       值的完整列表，请参见 stat(2)。


## 返回值

       spu_create 返回一个新的文件描述符。它可能返回 -1 以表示错误条件，并
       将 errno 设置为下列错误码之一。


## 错误

       EACCES
              当前用户对 spufs 挂载点没有写访问权限。

       EEXIST 给定路径名处已存在 SPU 上下文。

       EFAULT pathname 在当前地址空间中不是一个有效的字符串指针。

       EINVAL pathname 不是 spufs 挂载点中的目录。

       ELOOP  解析 pathname 时发现了过多符号链接。

       EMFILE 进程已达到其最大打开文件数限制。

       ENAMETOOLONG
              pathname 过长。

       ENFILE 系统已达到全局打开文件数限制。

       ENOENT pathname 的某部分无法解析。

       ENOMEM 内核无法分配所需的全部资源。

       ENOSPC 没有足够的 SPU 资源来创建新上下文，或已达到用户对 SPU 上下文
              数量的特定限制。

       ENOSYS 当前系统未提供该功能，因为硬件未提供 SPU，或 spufs 模块未加载。

       ENOTDIR
              pathname 的某部分不是目录。


## 备注

       spu_create 旨在供实现 SPU 更高层抽象接口的库使用，而非供普通应用程序
       直接使用。有关推荐的库，请参见 http://www.bsc.es/projects/deepcomputing/linuxoncell/。


## 文件

       pathname 必须指向 spufs 挂载点之下的位置。按照惯例，它被挂载在 /spu。


## 遵循标准

       此调用是 Linux 特有的，仅由 ppc64 架构实现。使用该系统调用的程序不可移植。


## 缺陷

       该代码尚未完全实现此处列出的所有特性。


## 作者

       Arnd Bergmann <arndb@de.ibm.com>

## 参见

       capabilities(7), close(2), spu_run(2), spufs(7)
