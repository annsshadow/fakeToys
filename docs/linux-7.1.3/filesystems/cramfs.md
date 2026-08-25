
## Cramfs - 将文件系统塞进小容量 ROM


cramfs 被设计为简单且小巧，并能很好地压缩内容
它使zlib 例程一次一页地压缩文件，并允许随机页访问。元数据不被压缩，但以非常简洁的方式表示，使比传统文件系统占用更少磁盘空间
你不能写cramfs 文件系统（使其可压缩且紧凑也使得它_非常_难以即时更新），因此你必须使"mkcramfs"
工具创建磁盘映像

### 使用注意事项


文件大小限制为小16MB
最大文件系统大小略超过 256MB。（文件系统上的最后一个文件允许扩展到 256MB 之外。）

只存gid 的低 8 位。当前版本的 mkcramfs 只是截断8 位，这是一个潜在的安全问题
支持硬链接，但硬链接的文件在 cramfs 映像中仍将具有链接计1
Cramfs 目录没有 `.` `..` 条目。目录（cramfs 上的每个其他文件一样）始终具有链接计数 1。（顺便说一句，
`find` 中无需使用 -noleaf。）

cramfs 中不存储时间戳，因此它们默认为纪元（1970 GMT）。最近访问的文件可能具有更新的时间戳，但该更只持inode 在内存中缓存的时长，之后时间戳回退1970，即时间向后倒退
目前，cramfs 必须以相同字节序的架构写入和读取，并且只能被 PAGE_SIZE == 4096 的内核读取。至少后是一个缺陷，但尚未决定最佳修复方案。目前，如果你有更大的页，只要你不介意文件系统对未来内核变得不可读，
你可以直接修mkcramfs.c 中的 #define

### 内存映射cramfs 映像


CRAMFS_MTD Kconfig 选项增加了直接从一个物理线性内存范围（通常是像 Flash 这样的非易失性存储器）加数据的支持，而不是经过块设备层。这节省了一些内存，因为在解压之前不需要中间缓冲来保存数据
而且，当数据块保持未压缩并适当对齐时，只要可能，它们将自动被直接映射到用户空间，从而提供只读段原地执行（XIP，eXecute-In-Place）。以读写方式映射的数据段（因此它们必须被复制RAM）在 cramfs 映像仍可与未压缩的只读段一同压缩在同一个文件中。MMU 和无 MMU 系统都受支持。这对于内存约束非常紧张的小嵌入式系统尤为方便
cramfs 映像在内存中的位置依赖于系统。你必须知道 cramfs 映像所在的正确物理地址，并为其配置一MTD 设备此外，该 MTD 设备必须受一个实现了 "point" 方法的映射驱动支持。此MTD 驱动的示例有 cfi_cmdset_0001
（Intel/Sharp CFI 闪存）或 physmap（物理内存映射中的闪存设备）。基于此类的 MTD 分区也没问题。然后该设备
应以前缀 "mtd:" 作为挂载设备参数指定。例如，要挂载名```

    $ mount -t cramfs mtd:fs_partition /mnt

```
MTD 设备，要以该文件系统作为根文件系统引导内核，只需在内核命令行上指定类"root=mtd:fs_partition" 即可

### 工具


可以利用上述最新能力的 mkcramfs 版本可以在此处找到：

https://github.com/npitre/cramfs-tools


### 用于 /usr/share/magic


=====	=======================	=======================
0	ulelong	0x28cd3d45	Linux cramfs offset 0
>4	ulelong	x		size %d
>8	ulelong	x		flags 0x%x
>12	ulelong	x		future 0x%x
>16	string	>\0		signature "%.16s"
>32	ulelong	x		fsid.crc 0x%x
>36	ulelong	x		fsid.edition %d
>40	ulelong	x		fsid.blocks %d
>44	ulelong	x		fsid.files %d
>48	string	>\0		name "%.16s"
512	ulelong	0x28cd3d45	Linux cramfs offset 512
>516	ulelong	x		size %d
>520	ulelong	x		flags 0x%x
>524	ulelong	x		future 0x%x
>528	string	>\0		signature "%.16s"
>544	ulelong	x		fsid.crc 0x%x
>548	ulelong	x		fsid.edition %d
>552	ulelong	x		fsid.blocks %d
>556	ulelong	x		fsid.files %d
>560	string	>\0		name "%.16s"
=====	=======================	=======================


### 黑客笔记


参见 fs/cramfs/README 了解文件系统布局与实现说明