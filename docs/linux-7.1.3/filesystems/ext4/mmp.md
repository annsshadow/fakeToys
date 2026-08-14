### 多挂载保护（Multiple Mount Protection）


多挂载保护（MMP）是一项保护文件系统免受多台主机同时尝试使用同一文件系统的特性。当文件系统
被打开（用于挂载、fsck 等）时，运行在该节点（称为节点 A）上的 MMP 代码会检查一个序列号。
如果序列号为 EXT4_MMP_SEQ_CLEAN，则打开继续。如果序列号为 EXT4_MMP_SEQ_FSCK，则说明 fsck
（但愿）正在运行，打开立即失败。否则，打开代码将等待两倍的指定 MMP 检查间隔，然后再次检查
序列号。如果序列号发生了变化，则文件系统在另一台机器上处于活动状态，打开失败。如果 MMP 代码
通过了所有这些检查，则生成一个新的 MMP 序列号并写入 MMP 块，挂载继续进行。

在文件系统活动期间，内核设置一个定时器，以指定的 MMP 检查间隔重新检查 MMP 块。为了执行重新
检查，会重新读取 MMP 序列号；如果它与内存中的 MMP 序列号不匹配，则另一个节点（节点 B）已经
挂载了该文件系统，节点 A 将文件系统重新挂载为只读。如果序列号匹配，则序列号在内存和磁盘上
都递增，重新检查完成。

每当打开操作成功时，主机名和设备文件名会被写入 MMP 块。MMP 代码并不使用这些值；提供它们
纯粹是为了提供信息。

校验和是根据 FS UUID 与 MMP 结构计算的。MMP 结构（`struct mmp_struct`）如下：

   :widths: 8 12 20 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __le32
     - mmp_magic
     - MMP 的幻数，0x004D4D50（“MMP”）。
   - - 0x4
     - __le32
     - mmp_seq
     - 序列号，周期性更新。
   - - 0x8
     - __le64
     - mmp_time
     - MMP 块最后一次更新的时间。
   - - 0x10
     - char[^64^]
     - mmp_nodename
     - 打开该文件系统的节点的主机名。
   - - 0x50
     - char[^32^]
     - mmp_bdevname
     - 文件系统的块设备名称。
   - - 0x70
     - __le16
     - mmp_check_interval
     - MMP 重新检查的间隔，以秒为单位。
   - - 0x72
     - __le16
     - mmp_pad1
     - 零。
   - - 0x74
     - __le32[^226^]
     - mmp_pad2
     - 零。
   - - 0x3FC
     - __le32
     - mmp_checksum
     - MMP 块的校验和。
