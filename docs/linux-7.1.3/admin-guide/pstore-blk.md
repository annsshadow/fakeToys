## pstore block oops/panic 记录

### 简

pstore block（pstore/blk）是一oops/panic 记录器，它在系统崩溃前将其日志写块设备和非块设备。你可以获取

```
    mount -t pstore pstore /sys/fs/pstore
```


### pstore block 概念


pstore/blk pstore/blk 提供了高效的配置方法，它将所有配置分为两部分：用户配置和
驱动配置
用户配置决定pstore/blk 的工作方式，例如 pmsg_size、kmsg_size 等。它们都同时支持 Kconfig 和模块参数，但模块参数优先于 Kconfig
驱动配置全部关于块设备和非块设备，例如块设备的总大小（total_size）以及读/写操作
### 用户配置


所有这些配置都同时支持 Kconfig 和模块参数，但模块参数优先于 Kconfig
```
        pstore_blk.blkdev=/dev/mmcblk0p7 pstore_blk.kmsg_size=64 best_effort=y
```
每个配置的细节可能会让你感兴趣
#### blkdev


要使用的块设备。大多数情况下，它是块设备的一个分区pstore/blk 需要它。它也被用于 MTD 设备
pstore/blk 被构建为模块时，“blkdev接受以下变体
1. /dev/<disk_name> 表示磁盘的设备号
#. /dev/<disk_name><decimal> 表示分区的设备号 —磁盘   设备号加上分区号
#. /dev/<disk_name>p<decimal> —与上述相同；当分区磁盘的磁盘名以数字结尾时使用此形式
pstore/blk 被构建进内核时，“blkdev接受以下变体
#. <hex_major><hex_minor> 十六进制表示的设备号，不带前0x，例b302#. PARTUUID=00112233-4455-6677-8899-AABBCCDDEEFF 表示分区的唯一 id（如果分区表提供它）。该 UUID 可以   EFI/GPT UUID，或使用格式 SSSSSSSS-PP 引用 MSDOS 分区，其SSSSSSSS 32    “NT disk signature的零填充十六进制表示，PP 1 基分区号的零填充十六进制表示#. PARTUUID=<UUID>/PARTNROFF=<int> 用于相对于具有已知唯一 id 的分区选择分区#. <major>:<minor> 以冒号分隔的设备的主设备号和次设备号
它接受以下用MTD 设备的变体：

1. <device name> MTD 设备名。推荐使“pstore”#. <device number> MTD 设备号
#### kmsg_size


用于 oops/panic 前端（front-end）的块大小（KB 为单位）。它**必须**4 的倍数如果你不关心 oops/panic 日志，它是可选的
根据除其pstore 前端外剩余的空间，oops/panic 前端有多个块
pstore/blk 会逐个记录oops/panic 块，并且如果没有更多空闲块，总是覆盖最旧的块
#### pmsg_size


用于 pmsg 前端（front-end）的块大小（KB 为单位）。它**必须**4 的倍数如果你不关心 pmsg 日志，它是可选的
oops/panic 前端不同，pmsg 前端只有一个块
Pmsg 是一个用户空间可访问pstore 对象。对 **/dev/pmsg0** 的写入会被追加到该块。重启后内容**/sys/fs/pstore/pmsg-pstore-blk-0** 中可用
#### console_size


用于 console 前端（front-end）的块大小（KB 为单位）。它**必须**4 的倍数如果你不关心 console 日志，它是可选的
pmsg 前端类似，console 前端只有一个块
console 的所有日志将被追加到该块。重启后内容**/sys/fs/pstore/console-pstore-blk-0** 中可用
#### ftrace_size


用于 ftrace 前端（front-end）的块大小（KB 为单位）。它**必须**4 的倍数如果你不关心 ftrace 日志，它是可选的
oops 前端类似，根cpu 处理器的数量，ftrace 前端有多个块。每个块大小等于
ftrace_size / processors_count銆。
ftrace 的所有日志将被追加到该块。重启后内容被合并并**/sys/fs/pstore/ftrace-pstore-blk-0** 中可用
持久函数追踪（Persistent function tracing）可能对调试软件或硬件有
```
 # mount -t pstore pstore /sys/fs/pstore
 # mount -t debugfs debugfs /sys/kernel/debug/
 # echo 1 > /sys/kernel/debug/pstore/record_ftrace
 # reboot -f
 [...]
 # mount -t pstore pstore /sys/fs/pstore
 # tail /sys/fs/pstore/ftrace-pstore-blk-0
 CPU:0 ts:5914676 c0063828  c0063b94  call_cpuidle <- cpu_startup_entry+0x1b8/0x1e0
 CPU:0 ts:5914678 c039ecdc  c006385c  cpuidle_enter_state <- call_cpuidle+0x44/0x48
 CPU:0 ts:5914680 c039e9a0  c039ecf0  cpuidle_enter_freeze <- cpuidle_enter_state+0x304/0x314
 CPU:0 ts:5914681 c0063870  c039ea30  sched_idle_set_state <- cpuidle_enter_state+0x44/0x314
 CPU:1 ts:5916720 c0160f59  c015ee04  kernfs_unmap_bin_file <- __kernfs_remove+0x140/0x204
 CPU:1 ts:5916721 c05ca625  c015ee0c  __mutex_lock_slowpath <- __kernfs_remove+0x148/0x204
 CPU:1 ts:5916723 c05c813d  c05ca630  yield_to <- __mutex_lock_slowpath+0x314/0x358
 CPU:1 ts:5916724 c05ca2d1  c05ca638  __ww_mutex_lock <- __mutex_lock_slowpath+0x31c/0x358
```
#### max_reason


限制存储哪些类型kmsg 转储可以通过 `max_reason` 值来控制，如 include/linux/kmsg_dump.h 中的
`enum kmsg_dump_reason` 所定义。例如，要同时存Oops Panic，`max_reason` 应设置为 2（KMSG_DUMP_OOPS）；
要仅存储 Panic，`max_reason` 应设置为 1（KMSG_DUMP_PANIC）。将其设置为 0
（KMSG_DUMP_UNDEF）意味着原因过滤将由 `printk.always_kmsg_dump` 启动参数控制：如果未设置，则KMSG_DUMP_OOPS否则KMSG_DUMP_MAX
### 驱动配置


设备驱动使用 `register_pstore_device` `struct pstore_device_info` pstore/blk 注册
   :export:

### 压缩与头

块设备对于未压缩oops 数据来说足够大。实际上我们不建议数据压缩，因为 pstore/blk 会向其中插入一些信息：

```
        Panic: Total 16 times
```
这意味着自首次启动以来，这是16 OOPS|Panic有时，自首次启动以来 oops|panic 发生的次数对判断系统是否稳定很重要
```
        Oops#2 Part1
```
这意味着在上次启动时，这是第 2 OOPS
### 读取数据


转储数据可以pstore 文件系统读取。这些文件的格式为：oops/panic 前端`dmesg-pstore-blk-[N]`pmsg 前端`pmsg-pstore-blk-0`，依此类推。转储文件的时间戳记录了触发时间。要从块设备
删除一个存储的记录，只需取消链接（unlink）相应的 pstore 文件
### panic 读写 API 中的注意事项


如果panic 时，内核不会运行太久了，任务将不会被调度，大多数内核资源将停止服务。这
看起来就像在单核计算机上运行的单线程程序
panic 读写 API 需要特别注意以下几点：

1. **不能**分配任何内存   如果你需要内存，就在块驱动初始化时分配，而不是等panic 时#. 必须是轮询（polled），**不是**中断驱动   不再有任何任务调度。块驱动应延迟以确保写入成功，但**不能**睡眠#. **不能**获取任何锁   没有其他任务，也没有任何共享资源；你可以安全地打破所有锁#. 只用 CPU 传输   不要使用 DMA 传输，除非你确定 DMA 不会持有锁#. 直接控制寄存器   请直接控制寄存器，而不是使Linux 内核资源   在初始化时做 I/O 映射，而不是等panic 发生#. 如有必要，重置你的块设备和控制器   如果你不确定 panic 发生时块设备和控制器的状态，可以停止并重置它们是安全的
pstore/blk 支持 psblk_blkdev_info()，它定义**linux/pstore_blk.h** 中，用于获取使用块设备的信息，例设备号、扇区计数以及整个磁盘的起始扇区
### pstore block 内部


供开发者参考，以下是所有重要的结构API
   :internal:

   :internal:

   :internal:
