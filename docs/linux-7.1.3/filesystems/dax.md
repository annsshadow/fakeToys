## 文件的直连访问（Direct Access

### 动机


页缓存通常用于缓冲对文件的读取和写入。它也用于提供通过调用 mmap 映射到用户空间的页面
对于类似内存的块设备，页缓存页面将是原始存储的不必要副本。`DAX` 代码通过直接对存储设备进行读写来消除这一额外副本。对于文件映射，存储设备被直接映射到用户空间

### 用法


如果你有一个支`DAX` 的块设备，你可以像往常一样在其上创建文件系统。`DAX` 代码目前只支持块大小等于内核 `PAGE_SIZE` 的文件，因此你在创建文件系统时可能需要指定块大小
目前5 个文件系统支`DAX`：ext2、ext4、xfs、virtiofs erofs。在它们上启`DAX` 的方式各不相同
### ext2 erofs 上启DAX


挂载文件系统时，在命令行使用 `-o dax` 选项，或者将 'dax' 添加`/etc/fstab` 中的选项。这可以在文件系统内的所有文件上启用 `DAX`。它等同于下`-o dax=always` 的行为

### xfs ext4 上启DAX


### 概要


 1. 存在一个内核内的文件访问模式标`S_DAX`，它对应statx 标志 `STATX_ATTR_DAX`。关于此访问模式的细节，请参statx(2) 的手册页
 2. 存在一个持久标`FS_XFLAG_DAX`，可以应用于普通文件和目录。此建议性标志可以随时设置或清除，但这样做不会立即影`S_DAX` 状态
 3. 如果在目录上设置了持`FS_XFLAG_DAX` 标志，此标志将被随后在该目录中创建的所有普通文件和子目录继承。在父目录上设置或清除此标志时已经存在的文件和子目录不受此修改的影响
 4. 存在一dax 挂载选项，可以在设置 `S_DAX` 标志时覆`FS_XFLAG_DAX`。假定底层存储支`DAX`，则以下情况成立
    `-o dax=inode`  表示 “遵`FS_XFLAG_DAX`”，并且是默认值
    `-o dax=never`  表示 “永不设`S_DAX`，忽`FS_XFLAG_DAX`。
    `-o dax=always` 表示 “总是设置 `S_DAX`，忽`FS_XFLAG_DAX`。
    `-o dax`      是一个遗留选项，是 `dax=always` 的别名
```

      The option ``-o dax`` may be removed in the future so ``-o dax=always`` is
      the preferred method for specifying this behavior.

    .. note::

      `FS_XFLAG_DAX` 的修改和继承行为即使在文件系统以 dax 选项挂载时也保持不变。然而，内核态的 inode 状态（`S_DAX`）将被覆盖，直到文件系统dax=inode 重新挂载并且inode 从内核内存中被逐出
 5. `S_DAX` 策略可以通过以下方式更改
    a) 在创建文件之前根据需要设置父目录`FS_XFLAG_DAX`

    b) 设置相应dax="foo" 挂载选项

    c) 更改现有普通文件和目录上的 `FS_XFLAG_DAX` 标志。这有运行时约束和限制，如下6) 所述
 6. 当通过切换持久 `FS_XFLAG_DAX` 标志来更`S_DAX` 策略时，对现有普通文件的更改要到该文件被所有进程关闭后才会生效

```
### 细节


2 个每文件 dax 标志。一个是持久inode 设置（`FS_XFLAG_DAX`），另一个是表示特性激活状态的易失标志（`S_DAX`）
`FS_XFLAG_DAX` 保存在文件系统内。这个持久配置设置可以使`FS_IOC_FS`[`GS`]`ETXATTR` ioctl（见 ioctl_xfs_fsgetxattr(2)）或诸如 'xfs_io' 这样的工具来设置、清除和/或查询
新文件和目录*创建*自动从它们的父目录继`FS_XFLAG_DAX`。因此，在创建目录时设置 `FS_XFLAG_DAX` 可以用来为一个整棵子树设置默认行为
为了说明继承，这里有三个例子
Example A:

  mkdir -p a/b/c
  xfs_io -c 'chattr +x' a
  mkdir a/b/c/d
  mkdir a/e

  ------[outcome]------

  dax: a,e
  no dax: b,c,d

Example B:

  mkdir a
  xfs_io -c 'chattr +x' a
  mkdir -p a/b/c/d

  ------[outcome]------

  dax: a,b,c,d
  no dax:

Example C:

  mkdir -p a/b/c
  xfs_io -c 'chattr +x' c
  mkdir a/b/c/d

  ------[outcome]------

  dax: c,d
  no dax: a,b

当前的启用状态（`S_DAX`）是在文inode 被内核实例化到内存中时设置的。它基于底层介质支持、`FS_XFLAG_DAX` 的值以及文件系统的 dax 挂载选项来设置
可以使用 statx 来查`S_DAX`

  只有普通文件才会被设置 `S_DAX`，因statx 永远不会指示目录上设置了 `S_DAX`
设置 `FS_XFLAG_DAX` 标志（显式地或通过继承）即便底层介质不支持 dax 或文件系统被某个挂载选项覆盖时也会发生

### virtiofs 上启DAX

DAX virtiofs 上的语义基本等同于在 ext4 xfs 上，除了当指'-o dax=inode' 时，virtiofs 客户端通过 FUSE 协议virtiofs 服务器派生是否启DAX 的提示，而不是使用持久的 `FS_XFLAG_DAX` 标志。也就是说，是否启用 DAX 完全virtiofs 服务器决定，virtiofs 服务器本身可以部署各种算法来做出此决定，例如取决于主机上持久`FS_XFLAG_DAX` 标志
在客户机内部设置和清除持`FS_XFLAG_DAX` 标志仍然受支持，但不能保证相应文件的 DAX 会被启用或禁用。客户机内的用户仍然需要调statx(2) 并检statx 标志 `STATX_ATTR_DAX` 以查看该文件DAX 是否启用

### 给块驱动编写者的实现提示


要在你的块驱动中支持 `DAX`，实'direct_access' 块设备操作。它用于将扇区号（以 512 字节扇区为单位）转换为标识该内存物理页面的页帧号（pfn）。它还返回一个可用于访问该内存的内核虚拟地址
direct_access 方法接受一'size' 参数，指示所请求的字节数。该函数应返回在该偏移处可连续访问的字节数。如果发生错误，它也可以返回负的 errno
为了支持此方法，存储必须随时可由 CPU 按字节访问。如果你的设备使用分页技术通过较小的窗口暴露大量内存，那么你就无法实现 direct_access。同样，如果你的设备偶尔会使 CPU 停顿较长时间，你也不应尝试实direct_access
这些块设备可作为参考：

- pmem：NVDIMM 持久内存驱动


### 给文件系统编写者的实现提示


文件系统的支持包括：

- 通过在内i_flags 中设`S_DAX` 标志，添加将 inode 标记`DAX` 的支- 实现 ->read_iter ->write_iter 操作，当 inode 设置`S_DAX` 标志时使`dax_iomap_rw()`
- `DAX` 文件实现一mmap 文件操作，在 `VMA` 上设`VM_MIXEDMAP` `VM_HUGEPAGE` 标志，并vm_ops 设置为包fault、pmd_fault、page_mkwrite、pfn_mkwrite 的处理程序。这些处理程序可能应该调`dax_iomap_fault()`，传入适当的故障大小和 iomap 操作- `DAX` 文件调用 `iomap_zero_range()` 并传入适当iomap 操作，以代替 `block_truncate_page()`
- 确保读取、写入、截断和缺页之间有充分的加锁

用于分配块的 iomap 处理程序必须确保所分配的块在被返回之前被清零并转换为已写入的区段，以避免通过 mmap 暴露未初始化的数据
这些文件系统可作为参考：


  ext2：见 Documentation/filesystems/ext2.rst


  xfs锛? 瑙?Documentation/admin-guide/xfs.rst


  ext4：见 Documentation/filesystems/ext4/


### 处理介质错误


libnvdimm 子系统为每个 pmem 块设备（gendisk->badblocks 中）存储已知介质错误位置的记录。如果我们在此类位置缺页，或者在一个尚未发现的潜在错误位置缺页，应用程序可以预期收到一`SIGBUS`。Libnvdimm 还允许通过简单地写入受影响的扇区（通过 pmem 驱动，并且底NVDIMM 支持 ACPI 定义clear_poison DSM）来清除这些错误
由于 `DAX` IO 通常不经`driver/bio` 路径，应用程序或系统管理员有以下方式从先前的 `backup/inbuilt` 冗余中恢复丢失的数据
1. 删除受影响的文件，并从备份中恢复（系统管理员途径）：
   这将释放该文件正在使用的文件系统块，下次它们被分配时，会先被清零，这通过驱动发生，并将清除坏扇区
2. 截断或对文件中含有坏块的部分进行打洞（至少要打洞一个完整的对齐扇区，但不一定是一个完整的文件系统块）
这些是允`DAX` 文件系统在存在介质错误的情况下继续运行的两条基本路径。未来可以在此基础上构建更健壮的错误恢复机制，例如涉及在块层通过 DM 提供的冗镜像，或者额外在文件系统级别提供。这些都必须依赖上述两条原则：错误清除既可以通过向驱动发IO 发生，也可以通过清零（同样通过驱动）发生

### 缺点


即使内核或其模块存储在支`DAX` 的文件系统上、并且该文件系统位于支持 `DAX` 的块设备上，它们仍然会被复制RAM 中
DAX 代码在具有虚拟映射缓存（ARM、MIPS SPARC）的架构上不能正确工作
在从 `DAX` 文件 mmap 出来的用户内存区域上调用 `get_user_pages()` 会失败，因为不存在描述这些页面的 'struct page'。某些设备驱动通过为受驱动控制的页面添加可选的 struct page 支持解决了此问题（参`drivers/nvdimm` 中的 `CONFIG_NVDIMM_PFN` 作为如何做到这一点的示例）。在struct page 的情况下，从`DAX` 文件对这些内存范围进行的 `O_DIRECT` 写将会失


  `O_DIRECT` `DAX` 文件的读/写确实可以工作，这里关键是正在被访问的内存）。在struct page 情况下其他不能工作的还包RDMA、`sendfile()` `splice()`