## 分区循环（zloop）块设备


 1) 概述
 2) 创建分区设备
 3) 删除分区设备
 4) 示例


### 1) 概述


分区循环块设备驱动（zloop）允许用户创建分区块设备，为每个区使用一个常规文件作为后端存储。该驱动不直接控制任何硬件，而是通过对文件系统中的常规文件执行读、写和截断操作来模拟分区块设备。

使用 zloop 可以创建具有可配置容量、区大小以及常规区数量的分区块设备。设备每个区的存储都使用一个常规文件实现，其最大大小等于区大小。作为常规区后端的文件大小始终等于区大小。作为顺序写区后端的文件大小则指示已顺序写入该文件的数据量，也就是说，该文件的大小直接指示了区的写指针位置。

重置一个顺序写区时，其后端文件大小会被截断为零。相反，对于区的 finish 操作，后端文件会被截断到区大小。由此，创建的 zloop 分区块设备的最大容量可以配置为大于后端文件系统上可用的存储空间。当然，对于这种配置，写入的数据量超过后端文件系统上可用存储空间时会导致写错误。

分区循环块设备驱动实现了一个完整的区状态转换状态机。也就是说，区可以是空、隐式打开、显式打开、关闭或已满。当前实现不支持对最大打开区数和活动区数施加任何限制。

创建和删除 zloop 设备不需要任何用户态工具。


### 2) 创建分区设备


一旦加载了 zloop 模块（或者 zloop 被编译进内核），就可以使用字符设备文件 /dev/zloop-control 来添加一个 zloop 设备。这是通过直接向 /dev/zloop-control 写入一个 "add" 命令来完成的。


```
	$ modprobe zloop
        $ ls -l /dev/zloop*
        crw-------. 1 root root 10, 123 Jan  6 19:18 /dev/zloop-control

        $ mkdir -p <base directory/<device ID>
        $ echo "add [options]" > /dev/zloop-control
```

可用于 add 命令的选项可以通过读取以下文件列出：


```
	$ cat /dev/zloop-control
        add id=%d,capacity_mb=%u,zone_size_mb=%u,zone_capacity_mb=%u,conv_zones=%u,max_open_zones=%u,base_dir=%s,nr_queues=%u,queue_depth=%u,buffered_io,zone_append=%u,ordered_zone_append,discard_write_cache
        remove id=%d
```

更详细地说，可与 "add" 命令一起使用的选项如下。

====================   =========================================================
id                    设备号（即 /dev/zloopX 中的 X）。
                      默认值：自动分配。
capacity_mb           设备总容量，单位为 MiB。该值总是向上取整到
                      区大小最接近的更高倍数。
                      默认值：16384 MiB（16 GiB）。
zone_size_mb          设备区大小，单位为 MiB。默认值：256 MiB。
zone_capacity_mb      设备区容量（必须始终等于或小于区大小）。默认值：区大小。
conv_zones            从扇区 0 开始的常规区总数。
                      默认值：8
max_open_zones        所需的打开顺序写区的最大数量（0 表示无限制）。
                      默认值：0
base_dir              用于创建包含该设备区文件的目录的基础目录路径。
                      默认值=/var/local/zloop。
                      包含区文件的设备目录总是以设备 ID 命名。例如
                      /dev/zloop0 的默认区文件目录为 /var/local/zloop/0。
nr_queues             分区块设备的 I/O 队列数量。该值总是受在线
                      CPU 数量的上限约束。
                      默认值：1
queue_depth           每个 I/O 队列的最大 I/O 队列深度。
                      默认值：64
buffered_io           执行缓冲 I/O 而非直接 I/O（默认值：false）。
zone_append           启用或禁用 zloop 设备的原生 zone append 支持。
                      默认值：1（启用）。
                      若禁用了原生 zone append 支持，块层将使用常规写
                      操作来模拟该操作。
ordered_zone_append   启用 zloop 对 zone append 重排序的缓解。
                      默认值：禁用。
                      这对于测试文件系统文件数据映射（extent）很有用，
                      因为启用后，可以显著减少文件数据映射所需的
                      数据 extent 数量。
discard_write_cache   设备被移除时，通过将每个区文件截断到上一次刷新
                      操作期间记录的大小，丢弃所有未通过刷新操作
                      显式持久化的数据。这模拟了未提交数据丢失的
                      掉电事件。
====================   =========================================================


### 3) 删除分区设备


删除一个未使用的分区循环块设备是通过发出 "remove" 命令来完成的。


```
        $ echo "remove id=X" > /dev/zloop-control
```

remove 命令没有任何选项。

被移除的分区设备可以在不改变设备区状态的情况下再次添加：设备区会被恢复到设备被移除之前的状态。在设备被移除之后再次添加分区设备时，必须始终使用与首次添加设备时相同的配置。如果检测到区配置发生变化，将返回错误，并且不会创建分区设备。

要彻底删除一个分区设备，在执行 remove 操作后，必须删除包含该设备各区的后端文件的设备基础目录。


### 4) 示例


以下命令序列创建了一个 2GB 的分区设备，其区大小为 64


```
        $ modprobe zloop
        $ mkdir -p /var/local/zloop/0
        $ echo "add capacity_mb=2048,zone_size_mb=64,zone_capacity_mb=63" > /dev/zloop-control
```

对于所创建的设备（/dev/zloop0），其区后端文件全部创建为


```
        $ ls -l /var/local/zloop/0
        total 0
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000000
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000001
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000002
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000003
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000004
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000005
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000006
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000007
        -rw-------. 1 root root        0 Jan  6 22:23 seq-000008
        -rw-------. 1 root root        0 Jan  6 22:23 seq-000009
        ...
```

```
        $ lsblk -z
        NAME   ZONED        ZONE-SZ ZONE-NR ZONE-AMAX ZONE-OMAX ZONE-APP ZONE-WGRAN
        zloop0 host-managed     64M      32         0         0       1M         4K
        $ blkzone report /dev/zloop0
          start: 0x000000000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000020000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000040000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000060000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000080000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x0000a0000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x0000c0000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x0000e0000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000100000, len 0x020000, cap 0x01f800, wptr 0x000000 reset:0 non-seq:0, zcond: 1(em) [type: 2(SEQ_WRITE_REQUIRED)]
          start: 0x000120000, len 0x020000, cap 0x01f800, wptr 0x000000 reset:0 non-seq:0, zcond: 1(em) [type: 2(SEQ_WRITE_REQUIRED)]
          ...
```

```
        $ echo "remove id=0" > /dev/zloop-control
```

被移除的设备可以使用与首次创建设备时相同的 "add" 命令再次添加。要彻底删除一个分区设备，其后端文件


```
        $ rm -r /var/local/zloop/0
```
