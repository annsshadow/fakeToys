## 将交换文件用于软件挂起（swsusp）


	(C) 2006 Rafael J. Wysocki <rjw@sisk.pl>

Linux 内核处理交换文件的方式与处理交换分区几乎相同，这两类交换区之间只有
两个区别：
(1) 交换文件不必连续分布；
(2) 交换文件的头部不在其所在分区的第一块中。从 swsusp 的角度来看，(1) 并不
是问题，因为交换处理代码已经处理了这一点；但 (2) 必须被考虑在内。

原则上，交换文件头部的位置可以通过相应的文件系统驱动来确定。然而不幸的是，
这要求持有交换文件的文件系统处于挂载状态，而如果该文件系统是日志型的，则在
从磁盘恢复（resume）时无法挂载它。因此，为了标识一个交换文件，swsusp 使用
持有该文件的分区名称，以及从分区起始位置到交换文件头部所在位置的偏移量。为
了方便起见，该偏移量以 <PAGE_SIZE> 为单位表示。

要将交换文件用于 swsusp，你需要：

```

    # dd if=/dev/zero of=<swap_file_path> bs=1024 count=<swap_file_size_in_k>
    # mkswap <swap_file_path>
    # swapon <swap_file_path>

```
2) 使用一个应用程序，借助 FIBMAP ioctl 对交换文件进行 bmap，并以从持有该交换
文件的分区起始处算起的偏移量（单位为 <PAGE_SIZE>）来确定文件交换头部的位置。

```

    resume=<swap_file_partition> resume_offset=<swap_file_offset>

```
其中 <swap_file_partition> 是交换文件所在的分区，<swap_file_offset> 是由
第 2 步中应用程序确定的交换头部偏移量（当然，这一步也可以由同一个使用
FIBMAP ioctl 来确定交换文件头部偏移量的应用程序自动完成）。

或者

使用一个用户态挂起应用程序，借助 Documentation/power/userland-swsusp.rst 中
描述的 SNAPSHOT_SET_SWAP_AREA ioctl 来设置分区与偏移量（这是将交换文件挂起、
且允许从 initrd 或 initramfs 镜像发起恢复的唯一方法）。

此后，swsusp 会以与使用交换分区相同的方式使用交换文件。特别地，交换文件必须
处于激活状态（即出现在 /proc/swaps 中），这样它才能被用于挂起。

注意，如果用于挂起的交换文件被删除并重新创建，其头部的位置可能与之前不同。
因此，每当发生这种情况时，都必须更新内核命令行参数 "resume_offset=" 的值。
