
## ZoneFS - Zone filesystem for Zoned block devices（ZoneFS - 面向分区块设备的分区文件系统）

## Introduction（简介）


zonefs 是一个非常简单的文件系统，它将分区块设备（zoned block device）的每个 zone 作为一个文件暴露出来。与具备原生分区块设备支持的常规 POSIX 兼容文件系统（例如 f2fs）不同，zonefs 不会向用户隐藏分区块设备的顺序写约束。代表设备顺序写 zone 的文件必须从文件末尾开始顺序写入（仅追加写）。

因此，zonefs 本质上更接近于原始块设备访问接口，而非功能完备的 POSIX 文件系统。zonefs 的目标是通过以更丰富的文件 API 取代原始块设备文件访问，简化应用程序中对分区块设备支持的实现，避免依赖对开发者而言可能更晦涩的直接块设备文件 ioctl。这种方法的一个例子是在分区块设备上实现 LSM（log-structured merge，日志结构合并）树结构（如 RocksDB 和 LevelDB 中所用），允许将 SSTable 存储在 zone 文件中，类似于常规文件系统而非整个磁盘的一组扇区。引入“一个文件即一个 zone”这一更高层的抽象，有助于减少应用程序所需的改动量，并引入对不同应用程序编程语言的支持。

### Zoned block devices（分区块设备）


分区存储设备属于一类存储设备，其地址空间被划分为 zone。一个 zone 是一组连续的 LBA，且所有 zone 都是连续的（不存在 LBA 间隙）。Zone 可以有不同的类型。

- Conventional zones（常规 zone）：对属于常规 zone 的 LBA 没有访问约束。可以执行任何读或写访问，类似于常规块设备。
- Sequential zones（顺序 zone）：这些 zone 接受随机读，但必须顺序写。每个顺序 zone 有一个由设备维护的写指针，跟踪下一次写入设备的强制起始 LBA 位置。由于这一写约束，顺序 zone 中的 LBA 不能被覆盖。顺序 zone 必须首先使用特殊命令（zone reset，zone 重置）擦除后才能重写。

分区存储设备可以使用各种记录和介质技术实现。当今最常见的分区存储形式是在叠瓦式磁记录（SMR，Shingled Magnetic Recording）HDD 上使用 SCSI Zoned Block Commands (ZBC) 和 Zoned ATA Commands (ZAC) 接口。

固态硬盘（SSD）存储设备也可以实现分区接口，例如，以减少因垃圾回收导致的内部写放大。NVMe Zoned NameSpace (ZNS) 是 NVMe 标准委员会的一项技术提案，旨在为 NVMe 协议添加分区存储接口。

## Zonefs Overview（Zonefs 概述）


Zonefs 将分区块设备的 zone 作为文件暴露出来。代表 zone 的文件按 zone 类型分组，而 zone 类型本身由子目录表示。这个文件结构完全使用设备提供的 zone 信息构建，因此不需要任何复杂的磁盘上元数据结构。

### On-disk metadata（磁盘上元数据）


zonefs 的磁盘上元数据精简为一个不可变的超级块（super block），它持久化存储一个魔数（magic number）以及可选的特性标志和值。挂载时，zonefs 使用 blkdev_report_zones() 获取设备的 zone 配置，并仅基于此信息用一个静态的文件树填充挂载点。文件大小来自设备的 zone 类型以及由设备自身维护的写指针位置。

超级块始终写在磁盘的 0 扇区。存储超级块的设备第一个 zone 永远不会被 zonefs 作为 zone 文件暴露。如果包含超级块的 zone 是顺序 zone，mkzonefs 格式化工具总是“finish”该 zone，即将其转换到 full 状态以变为只读，防止任何数据写入。

### Zone type sub-directories（Zone 类型子目录）


代表同一类型 zone 的文件在挂载时自动分组到同一个子目录下。

对于常规 zone，使用 "cnv" 子目录。然而，仅当设备具有可用的常规 zone 时才创建该目录。如果设备仅在 0 扇区有一个常规 zone，则该 zone 不会被作为文件暴露，因为它会被用于存储 zonefs 超级块。对于此类设备，不会创建 "cnv" 子目录。

对于顺序写 zone，使用 "seq" 子目录。

这两个目录是 zonefs 中仅有的目录。用户不能创建其他目录，也不能重命名或删除 "cnv" 和 "seq" 子目录。

由 stat() 或 fstat() 系统调用获得的 struct stat 的 st_size 字段所指示的目录大小，表示存在于该目录下的文件数量。

### Zone files（Zone 文件）


Zone 文件以它们所代表的 zone 在特定类型 zone 集合中的编号命名。也就是说，"cnv" 和 "seq" 目录都包含名为 "0"、"1"、"2"、…… 的文件。文件编号也代表设备上递增的 zone 起始扇区。

对所有 zone 文件的读和写操作都不允许超出文件的最大大小，即超出 zone 容量。任何超出 zone 容量的访问都会以 -EFBIG 错误失败。

创建、删除、重命名或修改文件和子目录的任何属性都是不允许的。

由 stat() 和 fstat() 报告的文件的块数表示 zone 文件的容量，换句话说，即最大文件大小。

### Conventional zone files（常规 zone 文件）


常规 zone 文件的大小固定为它们所代表的 zone 的大小。常规 zone 文件不能被截断。

这些文件可以使用任何类型的 I/O 操作随机读和写：缓冲 I/O、直接 I/O、内存映射 I/O（mmap）等。除了上述文件大小限制外，这些文件没有 I/O 约束。

### Sequential zone files（顺序 zone 文件）


分组在 "seq" 子目录中的顺序 zone 文件的大小，表示文件的 zone 写指针位置相对于 zone 起始扇区的偏移。

顺序 zone 文件只能从文件末尾开始顺序写，即写操作只能是追加写。zonefs 不接受随机写，并且会拒绝任何起始偏移不等于文件末尾、或不等于最后发出的仍处于飞行中（对于异步 I/O 操作）的写操作末尾的写请求。

由于页缓存的脏页回写不能保证顺序写模式，zonefs 阻止对顺序文件的缓冲写和可写共享映射。这些文件只接受直接 I/O 写。zonefs 依赖块层电梯实现的向设备的顺序写 I/O 请求投递。必须使用实现了分区块设备顺序写特性（ELEVATOR_F_ZBD_SEQ_WRITE elevator 特性）的电梯。此类电梯（例如 mq-deadline）在设备初始化时默认设置给分区块设备。

顺序 zone 文件的读操作所使用的 I/O 类型没有限制。缓冲 I/O、直接 I/O 和共享读映射都被接受。

顺序 zone 文件只允许截断到 0，这种情况下 zone 被重置，将文件 zone 写指针位置回绕到 zone 起始处；或者截断到 zone 容量，这种情况下文件的 zone 被转换到 FULL 状态（finish zone 操作）。

### Format options（格式化选项）


zonefs 的若干可选特性可以在格式化时启用。

- Conventional zone 聚合：连续的常规 zone 范围可以聚合为单个更大的文件，而不是默认的每个 zone 一个文件。
- 文件所有权：zone 文件的 owner UID 和 GID 默认为 0（root），但可以改为任何有效的 UID/GID。
- 文件访问权限：默认的 640 访问权限可以更改。

### IO error handling（IO 错误处理）

分区块设备可能因与常规块设备类似的原因而失败 I/O 请求，例如由于坏扇区。然而，除了此类已知的 I/O 失败模式外，管辖分区块设备行为的标准还定义了导致 I/O 错误的额外条件。

- 一个 zone 可能转换到只读条件（BLK_ZONE_COND_READONLY）：虽然 zone 中已写入的数据仍然可读，但该 zone 不能再被写入。对 zone 的任何用户操作（zone 管理命令或读/写访问）都不能将 zone 条件改回正常的读/写状态。虽然标准未定义设备将 zone 转换到只读状态的原因，但此类转换的典型原因是 HDD 上 defective 的写磁头（该磁头下的所有 zone 都被改为只读）。

- 一个 zone 可能转换到离线条件（BLK_ZONE_COND_OFFLINE）：离线的 zone 既不能被读也不能被写。任何用户操作都不能将离线 zone 转换回可操作的良好状态。与 zone 只读转换类似，驱动器将 zone 转换到离线条件的原因也未定义。典型原因是 HDD 上 defective 的读写磁头导致坏磁头下盘片上的所有 zone 都不可访问。

- 未对齐写错误：这些错误源于主机发出的写请求，其起始扇区在执行该写请求时与 zone 写指针位置不对应。尽管 zonefs 对顺序 zone 强制顺序文件写，但在非常大的直接 I/O 操作被拆分为多个 BIO/请求或异步 I/O 操作发生部分失败时，仍可能出现未对齐写错误。如果发往设备的顺序写请求集合中有一个写请求失败，排在其后的所有写请求都会变得未对齐并失败。

- 延迟写错误：与常规块设备类似，如果设备端写缓存启用，当设备写缓存被刷出时（例如在 fsync() 时），先前已完成的写范围内可能发生写错误。与前面的即时未对齐写错误情况类似，延迟写错误可以穿过 zone 的缓存顺序数据流传播，导致引发错误的扇区之后的所有数据被丢弃。

zonefs 检测到的所有 I/O 错误都通过触发或检测到该错误的系统调用的错误码返回给用户。zonefs 针对 I/O 错误所采取的恢复动作取决于 I/O 类型（读 vs 写）以及错误原因（坏扇区、未对齐写或 zone 条件改变）。

- 对于读 I/O 错误，zonefs 不执行任何特定的恢复动作，但前提是文件 zone 仍处于良好条件，且文件 inode 大小与其 zone 写指针位置之间没有不一致。如果检测到问题，则执行 I/O 错误恢复（见下表）。

- 对于写 I/O 错误，zonefs 总是执行 I/O 错误恢复。

- zone 条件改变为只读或离线也总是触发 zonefs 的 I/O 错误恢复。

zonefs 最小 I/O 错误恢复可能会改变文件大小和文件访问权限。

- 文件大小改变：
  顺序 zone 文件中的即时或延迟写错误可能导致文件 inode 大小与成功写入文件 zone 的数据量不一致。例如，多 BIO 大写的写操作的部分失败会导致 zone 写指针部分前进，即使整个写操作会被报告为用户失败。在这种情况下，必须推进文件 inode 大小以反映 zone 写指针的改变，并最终允许用户从文件末尾重新开始写。
  文件大小也可能被减小以反映 fsync() 上检测到的延迟写错误：在这种情况下，zone 中实际写入的数据量可能少于文件 inode 大小原先指示的量。在此类 I/O 错误之后，zonefs 总是修正文件 inode 大小以反映持久存储在文件 zone 中的数据量。

- 访问权限改变：
  zone 条件改变为只读通过更改文件访问权限以使文件变为只读来指示。这会禁用对文件属性的更改和数据的修改。对于离线 zone，对文件的全部权限（读和写）都被禁用。

zonefs I/O 错误恢复的进一步动作可以由用户通过 "errors=xxx" 挂载选项控制。下表根据挂载选项和 zone 条件总结了 zonefs I/O 错误处理的结果

```

    +--------------+-----------+-----------------------------------------+
    |              |           |            Post error state             |
    | "errors=xxx" |  device   |                 access permissions      |
    |    mount     |   zone    | file         file          device zone  |
    |    option    | condition | size     read    write    read    write |
    +--------------+-----------+-----------------------------------------+
    |              | good      | fixed    yes     no       yes     yes   |
    | remount-ro   | read-only | as is    yes     no       yes     no    |
    | (default)    | offline   |   0      no      no       no      no    |
    +--------------+-----------+-----------------------------------------+
    |              | good      | fixed    yes     no       yes     yes   |
    | zone-ro      | read-only | as is    yes     no       yes     no    |
    |              | offline   |   0      no      no       no      no    |
    +--------------+-----------+-----------------------------------------+
    |              | good      |   0      no      no       yes     yes   |
    | zone-offline | read-only |   0      no      no       yes     no    |
    |              | offline   |   0      no      no       no      no    |
    +--------------+-----------+-----------------------------------------+
    |              | good      | fixed    yes     yes      yes     yes   |
    | repair       | read-only | as is    yes     no       yes     no    |
    |              | offline   |   0      no      no       no      no    |
    +--------------+-----------+-----------------------------------------+

```

进一步说明：

- "errors=remount-ro" 挂载选项是 zonefs I/O 错误处理的默认行为（如果未指定任何 errors 挂载选项）。
- 使用 "errors=remount-ro" 挂载选项时，将文件访问权限改为只读适用于所有文件。文件系统被重新挂载为只读。
- 因设备将 zone 转换到离线条件而导致的访问权限和文件大小改变是永久的。使用 mkfs.zonefs (mkzonefs) 重新挂载或重新格式化设备不会将离线 zone 文件改回良好状态。
- 因设备将 zone 转换到只读条件而导致的文件访问权限改为只读是永久的。重新挂载或重新格式化设备不会重新启用文件写访问。
- 由 remount-ro、zone-ro 和 zone-offline 挂载选项隐含的文件访问权限改为只读，对于处于良好条件的 zone 是临时的。卸载并重新挂载文件系统将恢复受影响的文件先前的默认（格式化时的值）访问权限。
- repair 挂载选项只触发最小的一组 I/O 错误恢复动作，即针对处于良好条件的 zone 的文件大小修正。被设备指示为只读或离线的 zone 仍然意味着对 zone 文件访问权限的改变，如上表所述。

### Mount options（挂载选项）


zonefs 定义了几个挂载选项：
- errors=<behavior>
- explicit-open

#### "errors=<behavior>" option（"errors=<behavior>" 选项）


"errors=<behavior>" 挂载选项允许用户指定 zonefs 针对 I/O 错误、inode 大小不一致或 zone 条件改变的行为。定义的行为如下：

- remount-ro（默认）
- zone-ro
- zone-offline
- repair

为每个行为定义的运行时 I/O 错误动作在上节详述。挂载时 I/O 错误会导致挂载操作失败。只读 zone 的处理在挂载时和运行时之间也有所不同。如果在挂载时发现只读 zone，该 zone 总是以与离线 zone 相同的方式处理，即禁用所有访问并将 zone 文件大小设为 0。这是必要的，因为只读 zone 的写指针按 ZBC 和 ZAC 标准定义为无效，使得无法发现已写入该 zone 的数据量。对于运行时发现的只读 zone，如上节所述，zone 文件的大小保持为其最后更新的值不变。

#### "explicit-open" option（"explicit-open" 选项）


分区块设备（例如 NVMe Zoned Namespace 设备）可能对可处于活动状态的 zone 数量有限制，即处于隐式打开、显式打开或关闭条件的 zone。当用户发出写请求时，如果文件的 zone 尚未处于活动状态，这一潜在限制会转化为应用程序因超出该限制而看到写 I/O 错误的风险。

为了避免这些潜在错误，"explicit-open" 挂载选项在文件首次被打开用于写时，强制使用 open zone 命令使 zone 变为活动。如果 zone open 命令成功，则应用程序随后可以保证写请求能够被处理。反之，如果 zone 既非 full 也非空，则 "explicit-open" 挂载选项会在 zone 文件的最后一次 close() 时向设备发出 zone close 命令。

### Runtime sysfs attributes（运行时 sysfs 属性）


zonefs 为已挂载的设备定义了若干 sysfs 属性。所有属性都可由用户读取，可在目录 /sys/fs/zonefs/<dev>/ 中找到，其中 <dev> 是已挂载的分区块设备的名称。

定义的属性如下。

- **max_wro_seq_files**：该属性报告可打开用于写的最大顺序 zone 文件数。该数字对应于设备支持的显式或隐式打开 zone 的最大数量。值为 0 意味着设备没有限制，任何 zone（任何文件）都可以在任何时刻打开用于写并写入，无论其他 zone 的状态如何。当使用 **explicit-open** 挂载选项时，如果已经打开用于写的顺序 zone 文件数已达到 **max_wro_seq_files** 限制，zonefs 将拒绝任何请求打开顺序 zone 文件用于写的 open() 系统调用。
- **nr_wro_seq_files**：该属性报告当前打开用于写的顺序 zone 文件数。当使用 "explicit-open" 挂载选项时，该数字永远不会超过 **max_wro_seq_files**。如果未使用 **explicit-open** 挂载选项，报告的数字可能大于 **max_wro_seq_files**。在这种情况下，由应用程序负责不同时写入超过 **max_wro_seq_files** 个顺序 zone 文件。未能做到这一点可能导致写错误。
- **max_active_seq_files**：该属性报告处于活动状态的最大顺序 zone 文件数，即部分写入（非空也非 full）的顺序 zone 文件，或具有显式打开 zone 的文件（仅当使用 **explicit-open** 挂载选项时发生）。该数字始终等于设备支持的活动 zone 的最大数量。值为 0 意味着已挂载的设备对可活动的顺序 zone 文件数量没有限制。
- **nr_active_seq_files**：该属性报告当前活动的顺序 zone 文件数。如果 **max_active_seq_files** 不为 0，则无论是否使用 **explicit-open** 挂载选项，**nr_active_seq_files** 的值都永远不会超过 **max_active_seq_files** 的值。

## Zonefs User Space Tools（Zonefs 用户空间工具）


mkzonefs 工具用于格式化分区块设备以配合 zonefs 使用。该工具可在 Github 上获取：

https://github.com/damien-lemoal/zonefs-tools

zonefs-tools 还包含一个测试套件，可针对任何分区块设备运行，包括以 zoned 模式创建的 null_blk 块设备。

### Examples（示例）


以下命令格式化一块具有 256 MB zone 的 15TB 主机管理型 SMR HDD

```

    # mkzonefs -o aggr_cnv /dev/sdX
    # mount -t zonefs /dev/sdX /mnt
    # ls -l /mnt/
    total 0
    dr-xr-xr-x 2 root root     1 Nov 25 13:23 cnv
    dr-xr-xr-x 2 root root 55356 Nov 25 13:23 seq

```
各 zone 文件子目录的大小表示对应类型 zone 的文件数量。在本示例中，只有一个常规 zone 文件（所有常规 zone 都聚合在单个

```

    # ls -l /mnt/cnv
    total 137101312
    -rw-r----- 1 root root 140391743488 Nov 25 13:23 0

```

```

    # mkfs.ext4 /mnt/cnv/0
    # mount -o loop /mnt/cnv/0 /data

```
用于顺序写入 zone 文件的 "seq" 子目录在本示例中

```

    # ls -lv /mnt/seq
    total 14511243264
    -rw-r----- 1 root root 0 Nov 25 13:23 0
    -rw-r----- 1 root root 0 Nov 25 13:23 1
    -rw-r----- 1 root root 0 Nov 25 13:23 2
    ...
    -rw-r----- 1 root root 0 Nov 25 13:23 55354
    -rw-r----- 1 root root 0 Nov 25 13:23 55355

```
对于顺序写入的 zone 文件，随着数据以

```

    # dd if=/dev/zero of=/mnt/seq/0 bs=4096 count=1 conv=notrunc oflag=direct
    1+0 records in
    1+0 records out
    4096 bytes (4.1 kB, 4.0 KiB) copied, 0.00044121 s, 9.3 MB/s

    # ls -l /mnt/seq/0
    -rw-r----- 1 root root 4096 Nov 25 13:23 /mnt/seq/0

```
写入的文件可以被截断到 zone 大小，从而阻止任何进一步的

```

    # truncate -s 268435456 /mnt/seq/0
    # ls -l /mnt/seq/0
    -rw-r----- 1 root root 268435456 Nov 25 13:49 /mnt/seq/0

```
将大小截断为 0 可释放文件的 zone 存储空间并重新

```

    # truncate -s 0 /mnt/seq/0
    # ls -l /mnt/seq/0
    -rw-r----- 1 root root 0 Nov 25 13:49 /mnt/seq/0

```
由于文件静态映射到磁盘上的 zone，stat() 和 fstat() 报告的文件块数即表示文件的容量

```

    # stat /mnt/seq/0
    File: /mnt/seq/0
    Size: 0         	Blocks: 524288     IO Block: 4096   regular empty file
    Device: 870h/2160d	Inode: 50431       Links: 1
    Access: (0640/-rw-r-----)  Uid: (    0/    root)   Gid: (    0/    root)
    Access: 2019-11-25 13:23:57.048971997 +0900
    Modify: 2019-11-25 13:52:25.553805765 +0900
    Change: 2019-11-25 13:52:25.553805765 +0900
    Birth: -

```
文件的块数（"Blocks"）以 512B 块为单位，给出的最大文件大小为 524288 * 512 B = 256 MB，对应该示例中的设备 zone 容量。需要注意的是，"IO block" 字段始终表示写入的最小 I/O 大小，并对应于设备的物理扇区大小。
