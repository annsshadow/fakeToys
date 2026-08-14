
## dm-pcache — 持久化缓存（Persistent Cache）


**作者：Dongsheng Yang <dongsheng.yang@linux.dev>**

本文档描述 **dm-pcache**，这是一个 Device-Mapper 目标，它让一个可按字节寻址的 **DAX**（持久内存，“pmem”）区域充当位于较慢块设备之前的、高性能且崩溃持久化的缓存。相关代码位于 `drivers/md/dm-pcache/`。

## 特性速览


- **回写（write-back）** 缓存（目前唯一支持的模式）。
- 在 pmem 设备上分配的 **16 MiB 段（segment）**。
- **数据 CRC32** 校验（可选，按缓存设置）。
- 崩溃安全：每个元数据结构都做了双份复制（`PCACHE_META_INDEX_MAX == 2`），并使用 CRC 加序列号进行保护。
- **多树索引**（按逻辑地址分片的索引树），以获得较高的 PMem 并行度
- 纯 **DAX 路径** I/O —— 没有额外的 BIO 往返
- **日志结构回写（log-structured write-back）**，保持后端崩溃一致性


## 构造函数


```

    pcache <cache_dev> <backing_dev> [<number_of_optional_arguments> <cache_mode writeback> <data_crc true|false>]

```
=========================  ====================================================
`cache_dev`               Any DAX-capable block device (`/dev/pmem0`…).
                            All metadata **and** cached blocks are stored here.

`backing_dev`             The slow block device to be cached.

`cache_mode`              Optional, Only `writeback` is accepted at the
                            moment.

`data_crc`                Optional, default to `false`

                            - `true`  – store CRC32 for every cached entry
			      and verify on reads
                            - `false` – skip CRC (faster)
=========================  ====================================================

### 示例



   dmsetup create pcache_sdb --table \
     "0 $(blockdev --getsz /dev/sdb) pcache /dev/pmem0 /dev/sdb 4 cache_mode writeback data_crc true"

首次使用某个 pmem 设备时，dm-pcache 会自动格式化它（超级块、cache_info 等）。


## 状态行


`dmsetup status <device>`（`STATUSTYPE_INFO`）会打印：

```

   <sb_flags> <seg_total> <cache_segs> <segs_used> \
   <gc_percent> <cache_flags> \
   <key_head_seg>:<key_head_off> \
   <dirty_tail_seg>:<dirty_tail_off> \
   <key_tail_seg>:<key_tail_off>

```
### 字段含义


===============================  =============================================
`sb_flags`                     Super-block flags (e.g. endian marker).

`seg_total`                    Number of physical **pmem** segments.

`cache_segs`                   Number of segments used for cache.

`segs_used`                    Segments currently allocated (bitmap weight).

`gc_percent`                   Current GC high-water mark (0-90).

`cache_flags`                  Bit 0 – DATA_CRC enabled
                                 Bit 1 – INIT_DONE (cache initialised)
                                 Bits 2-5 – cache mode (0 == WB).

`key_head`                     Where new key-sets are being written.

`dirty_tail`                   First dirty key-set that still needs
                                 write-back to the backing device.

`key_tail`                     First key-set that may be reclaimed by GC.
===============================  =============================================


## 消息


**更改 GC 触发阈值**

```

   dmsetup message <dev> 0 gc_percent <0-90>


```
## 工作原理


### 子设备


====================  =========================================================
backing_dev             Any block device (SSD/HDD/loop/LVM, etc.).
cache_dev               DAX device; must expose direct-access memory.
====================  =========================================================

### 段与键集合（key-set）


- pmem 空间被划分为 **16 MiB 段（segment）**。
- 每次写入会从段内每个 CPU 的 **data_head** 分配空间。
- 一个 **cache-key（缓存键）** 记录了源设备上的一段逻辑范围，以及它在 pmem 中的位置（段 + 偏移 + 代（generation））。
- 128 个键组成一个 **key-set（kset）**；kset 在 pmem 中顺序写入，并且自身是崩溃安全的（CRC）。
- 这一对 **(key_tail, dirty_tail)** 界定了干净/脏以及存活/死亡 kset 的边界。

### 回写


脏键被排入一棵树中；一个后台工作线程将数据复制回 backing_dev，并推进 **dirty_tail**。来自上层的 FLUSH/FUA bio 会强制立即提交元数据。

### 垃圾回收


当 `segs_used >= seg_total * gc_percent / 100` 时，GC 启动。它从 **key_tail** 开始遍历，释放其中每个键都已失效的段，并推进 **key_tail**。

### CRC 校验


若 `data_crc 已启用`，dm-pcache 会在每次插入时为每个缓存数据范围计算 CRC32，并将其存储在介质上的键中。读取时会在复制到调用者之前验证 CRC。


## 故障处理


- **pmem 介质错误** —— 所有元数据副本都通过 `copy_mc_to_kernel` 读取；不可纠正的错误会记录日志并中止初始化。
- **缓存已满** —— 如果找不到空闲段，写入返回 `-EBUSY`；dm-pcache 会在内部重试（请求延迟）。
- **系统崩溃** —— 在挂载时，驱动会从 **key_tail** 重放 kset 以重建内存中的树；每个段的代（generation）可防止出现悬空（use-after-free）键。


## 限制与 TODO


- 仅 **回写** 模式；其它模式在计划中。
- 仅 FIFO 缓存失效；其它（LRU、ARC……）在计划中。
- 目前不支持表重载（table reload）。
- 丢弃（discard）在计划中。


## 示例工作流



   # 1.  创建设备
   dmsetup create pcache_sdb --table \
     "0 $(blockdev --getsz /dev/sdb) pcache /dev/pmem0 /dev/sdb 4 cache_mode writeback data_crc true"

   # 2.  在其上创建文件系统
   mkfs.ext4 /dev/mapper/pcache_sdb
   mount /dev/mapper/pcache_sdb /mnt

   # 3.  将 GC 阈值调整为 80%
   dmsetup message pcache_sdb 0 gc_percent 80

   # 4.  观察状态
   watch -n1 'dmsetup status pcache_sdb'

   # 5.  关闭
   umount /mnt
   dmsetup remove pcache_sdb


`dm-pcache` 仍在积极开发中；非常欢迎反馈、bug 报告和补丁！
