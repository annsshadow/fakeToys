
## Fiemap Ioctl


fiemap ioctl 是一种供用户空间高效获取文件区间（extent）映射的方法。与逐块映射（如 bmap）不同，fiemap 返回的是区间列表。

### 请求基础


fiemap 请求编码在 struct fiemap 中：

   :identifiers: fiemap

fm_start 和 fm_length 指定了进程希望获取映射的文件内逻辑范围。返回的区间与磁盘上的镜像一致——也就是说，第一个返回区间的逻辑偏移可能早于 fm_start，最后一个返回区间所覆盖的范围可能晚于 fm_length。所有偏移和长度都以字节为单位。

可以通过设置 fm_flags 中的某些标志来修改查找映射的方式。如果内核不理解某些特定标志，它会返回 EBADR，并且 fm_flags 的内容会包含导致错误的那组标志。如果内核与传入的所有标志都兼容，则 fm_flags 的内容保持不变。由用户空间决定拒绝某个特定标志对其操作是否是致命的。该方案旨在让 fiemap 接口在未来能够扩展，同时又不会与旧软件失去兼容性。

fm_extent_count 指定了 fm_extents[] 数组中可用于返回区间的元素个数。如果 fm_extent_count 为零，则忽略 fm_extents[] 数组（不会返回任何区间），并且 fm_mapped_extents 计数将保存 fm_extents[] 中为保存文件当前映射所需的区间数量。请注意，没有任何机制能阻止文件在两次 FIEMAP 调用之间发生变化。

可以设置到 fm_flags 中的标志如下：

FIEMAP_FLAG_SYNC
  如果设置了该标志，内核会在映射区间之前同步该文件。

FIEMAP_FLAG_XATTR
  如果设置了该标志，返回的区间将描述 inode 的扩展属性查找树，而不是其数据树。

FIEMAP_FLAG_CACHE
  该标志请求对区间进行缓存。

### 区间映射


区间信息在嵌入的 fm_extents 数组中返回，该数组必须由用户空间与 fiemap 结构体一起分配。fiemap_extents[] 数组中的元素个数应通过 fm_extent_count 传入。内核映射的区间数量将通过 fm_mapped_extents 返回。如果分配的 fiemap_extents 数量少于映射所请求范围所需的数量，则会返回 fm_extent[] 数组中能够映射的最大区间数量，且 fm_mapped_extents 会等于 fm_extent_count。在这种情况下，数组中的最后一个区间不会完成所请求的范围，也不会设置 FIEMAP_EXTENT_LAST 标志（见下一节关于区间标志的内容）。

每个区间由 fm_extents 中返回的一个 fiemap_extent 结构体描述：

    :identifiers: fiemap_extent

所有偏移和长度都以字节为单位，并与磁盘上的保持一致。区间的逻辑偏移早于请求或其逻辑长度超出请求都是有效的。除非返回了 FIEMAP_EXTENT_NOT_ALIGNED，否则 fe_logical、fe_physical 和 fe_length 都会与文件系统的块大小对齐。除了被标记为 FIEMAP_EXTENT_MERGED 的区间外，相邻的区间不会被合并。

fe_flags 字段包含描述所返回区间的标志。一个特殊标志 FIEMAP_EXTENT_LAST 总是设置在文件中最后一个区间上，以便发起 fiemap 调用的进程能够判断何时没有更多区间可用，而无需再次调用该 ioctl。

某些标志是有意含糊的，并且只要存在其他更具体的标志就会始终被设置。这样，寻找一般属性的程序就不必知道所有暗示该属性的现有及未来标志。

例如，如果设置了 FIEMAP_EXTENT_DATA_INLINE 或 FIEMAP_EXTENT_DATA_TAIL，也会设置 FIEMAP_EXTENT_NOT_ALIGNED。寻找内联或尾部打包数据的程序可以依据该具体标志。然而，仅仅关心不要去操作未对齐区间的软件可以只依据 FIEMAP_EXTENT_NOT_ALIGNED，而不必担心所有当前和未来的、可能暗示未对齐数据的标志。注意反之不成立——FIEMAP_EXTENT_NOT_ALIGNED 单独出现是有效的。

FIEMAP_EXTENT_LAST
  这通常是文件中的最后一个区间。越过该区间的映射尝试可能返回空。某些实现设置该标志以表示此区间是用户（通过 fiemap->fm_length）查询范围内的最后一个区间。

FIEMAP_EXTENT_UNKNOWN
  该区间的位置目前未知。这可能表示数据存储在不可访问的卷上，或者尚未为该文件分配存储。

FIEMAP_EXTENT_DELALLOC
  这也会设置 FIEMAP_EXTENT_UNKNOWN。

  延迟分配——虽然该区间已有数据，但其物理位置尚未分配。

FIEMAP_EXTENT_ENCODED
  该区间并非由普通的文件系统块组成，而是经过编码（例如加密或压缩）。通过该块设备进行 I/O 来读取此区间中的数据将产生未定义的结果。

注意，在文件系统的协助下，试图通过写入所指示位置来就地更新数据，或在文件系统已挂载时通过 FIEMAP 接口返回的信息来访问数据，这**总是**未定义的。换言之，用户应用程序只能在文件系统未挂载时通过块设备进行 I/O 读取区间数据，并且仅当 FIEMAP_EXTENT_ENCODED 标志未设置时才可以；在任何其他情况下，用户应用程序都不得试图通过块设备读取或写入文件系统。

FIEMAP_EXTENT_DATA_ENCRYPTED
  这也会设置 FIEMAP_EXTENT_ENCODED
  该区间中的数据已被文件系统加密。

FIEMAP_EXTENT_NOT_ALIGNED
  区间偏移和长度不保证按块对齐。

FIEMAP_EXTENT_DATA_INLINE
  这也会设置 FIEMAP_EXTENT_NOT_ALIGNED
  数据位于一个元数据块中。

FIEMAP_EXTENT_DATA_TAIL
  这也会设置 FIEMAP_EXTENT_NOT_ALIGNED
  数据被打包进一个与其他文件数据共用的块中。

FIEMAP_EXTENT_UNWRITTEN
  未写入区间——该区间已分配但其数据尚未初始化。这表示如果通过文件系统读取，该区间的数据将全为零；如果直接从设备读取，其内容则是未定义的。

FIEMAP_EXTENT_MERGED
  当文件不支持区间（即使用基于块的寻址方案）时会设置。由于将每个块的区间返回给用户空间效率极低，内核会尝试将大多数相邻块合并为“区间”。

FIEMAP_EXTENT_SHARED
  设置该标志以请求空间与其他文件共享。

### VFS -> 文件系统实现


希望支持 fiemap 的文件系统必须在其 inode_operations 结构体上实现 ->fiemap 回调。该 fs ->fiemap 调用负责定义其支持的一组 fiemap 标志，并调用一个辅助函数，具体见

```
  struct inode_operations {
       ...

       int (*fiemap)(struct inode *, struct fiemap_extent_info *, u64 start,
                     u64 len);
```

->fiemap 会传入描述 fiemap 请求的 struct fiemap_extent_info：

    :identifiers: fiemap_extent_info

文件系统的本意是不需要直接访问该结构体的任何成员。文件系统处理程序应该对信号宽容，并在收到致命信号时返回 EINTR。

标志检查应在 ->fiemap 回调开始时通过

```
  int fiemap_prep(struct inode *inode, struct fiemap_extent_info *fieinfo,
		  u64 start, u64 *len, u32 supported_flags);
```

完成。struct fieinfo 应按从 ioctl_fiemap() 收到时的样子传入。文件系统理解的 fiemap 标志集合应通过 fs_flags 传入。如果 fiemap_prep 发现无效的用户标志，它会将错误值放入 fieinfo->fi_flags 并返回 -EBADR。如果文件系统从 fiemap_prep() 得到 -EBADR，它应立即退出，将该错误返回给 ioctl_fiemap()。此外，范围会根据所支持的最大文件大小进行校验。

对于请求范围内的每个区间，文件系统应调用

```
  int fiemap_fill_next_extent(struct fiemap_extent_info *info, u64 logical,
			      u64 phys, u64 len, u32 flags, u32 dev);
```

fiemap_fill_next_extent() 将使用传入的值来填充 fm_extents 数组中的下一个空闲区间。通用的区间标志会根据具体标志自动由调用文件系统设置，从而不会破坏用户空间 API。

fiemap_fill_next_extent() 成功时返回 0，当用户提供的 fm_extents 数组已满时返回 1。如果在将区间复制到用户内存时遇到错误，则返回 -EFAULT。
