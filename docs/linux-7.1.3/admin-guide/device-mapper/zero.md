## dm-zero


Device-Mapper 的“zero”目标提供一个块设备，读取时总是返回零数据，并静默丢写入。其行为类似/dev/zero，但它是块设备而非字符设备
dm-zero 没有目标特定的参数
dm-zero 一个非常有趣的用途是dm-snapshot 配合创建“稀疏”设备。稀疏设报告的设备大小大于该设备实际可用的存储容量。用户可以在稀疏设备内的任意位写入数据，并像普通设备一样读回。对先前未写入区域的读取将返回零缓冲区。当
写入的数据足以填满实际存储容量时，稀疏设备将被停用。这对于测试设备和文系统的限制非常有用
要创建稀疏设备，首先创建一个大小等于所需稀疏设备大小的 dm-zero 设备。本例中
我们假设10TB

```
  TEN_TERABYTES=`expr 10 \* 1024 \* 1024 \* 1024 \* 2`   # 10 TB in sectors
  echo "0 $TEN_TERABYTES zero" | dmsetup create zero1

```
然后创建zero 设备的快照，使用任何可用的块设备作为 COW 设备。COW 设备大小将决定稀疏设备可用的实际空间大小。本例中我们假设/dev/sdb1

```
  echo "0 $TEN_TERABYTES snapshot /dev/mapper/zero1 /dev/sdb1 p 128" | \
     dmsetup create sparse1

```
这将创建一个名/dev/mapper/sparse1 10TB 稀疏设备，实际可用存储空间10GB。如果向该设备写入超10GB 的数据，它将开始返I/O 错误