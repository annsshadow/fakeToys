## 设备映射器“unstriped”目


## 简


设备映射器（device-mapper）的“unstriped”目标提供了一种透明机制，用于将设备映射器的“striped”目标解除条带化，以访问底层磁盘，而无需触及真正的后端块设备。它也可用于解除硬件 RAID-0 的条带化以访问后端磁盘

参数
<number of stripes> <chunk size> <stripe #> <dev_path> <offset>

<number of stripes>
        RAID 0 中的条带数量

<chunk size>
	条带化中一个区块（chunk）所包含512B 扇区数量

<dev_path>
	你希望解除条带化的块设备

<stripe #>
        设备中对应于你希望解除条带化的物理驱动器的条带编号。这必须0 起始的索引


## 为何使用此模块？


### 撤销现有 dm-stripe 的一个示


这个小型 bash 脚本将设4 loop 设备，并使用现有striped 目标将这 4 个设备合并为一个。然后它会在 striped 设备之上使用 unstriped 目标来访问各个后端的 loop 设备。我们将数据写入新暴露的 unstriped 设备，并验证写入的数据与正确```

  #!/bin/bash

  MEMBER_SIZE=$((128 * 1024 * 1024))
  NUM=4
  SEQ_END=$((${NUM}-1))
  CHUNK=256
  BS=4096

  RAID_SIZE=$((${MEMBER_SIZE}*${NUM}/512))
  DM_PARMS="0 ${RAID_SIZE} striped ${NUM} ${CHUNK}"
  COUNT=$((${MEMBER_SIZE} / ${BS}))

  for i in $(seq 0 ${SEQ_END}); do
    dd if=/dev/zero of=member-${i} bs=${MEMBER_SIZE} count=1 oflag=direct
    losetup /dev/loop${i} member-${i}
    DM_PARMS+=" /dev/loop${i} 0"
  done

  echo $DM_PARMS | dmsetup create raid0
  for i in $(seq 0 ${SEQ_END}); do
    echo "0 1 unstriped ${NUM} ${CHUNK} ${i} /dev/mapper/raid0 0" | dmsetup create set-${i}
  done;

  for i in $(seq 0 ${SEQ_END}); do
    dd if=/dev/urandom of=/dev/mapper/set-${i} bs=${BS} count=${COUNT} oflag=direct
    diff /dev/mapper/set-${i} member-${i}
  done;

  for i in $(seq 0 ${SEQ_END}); do
    dmsetup remove set-${i}
  done

  dmsetup remove raid0

  for i in $(seq 0 ${SEQ_END}); do
    losetup -d /dev/loop${i}
    rm -f member-${i}
  done

```
### 另一个示


Intel NVMe 驱动器在物理设备上包含两个核心
驱动器的每个核心对其 LBA 范围有隔离的访问
当前LBA 模型在每个核心上有一RAID 0 128k 区块，导```

   Core 0:       Core 1:
  __________    __________
  | LBA 512|    | LBA 768|
  | LBA 0  |    | LBA 256|
  ----------    ----------

```
此解除条带化的目的是在嘈杂邻居环境中提供更好QoS。当在不进行此解除条带化的情况下在聚合驱动器上创建两个分区时，对一个分区的读取会影响另一个分区上的写入。这是因为分区是跨两个核心条带化的。当我们解除此硬RAID 0 的条带化，并在每个新暴露的设备上创建分区时，两个分区现在在物理上是分离的

借助 dm-unstriped 目标，我们能够隔离一fio 脚本，其中的读作业和写作业彼此独立。与在带有分区的合并驱动器上运行测试相比，使用此设备映射器目标，我们将读取延迟降低了 92%


## dmsetup 使用示例


### 在具2 个核心的 Intel NVMe 设备之上解除条带


```

  dmsetup create nvmset0 --table '0 512 unstriped 2 256 0 /dev/nvme0n1 0'
  dmsetup create nvmset1 --table '0 512 unstriped 2 256 1 /dev/nvme0n1 0'

```
现在将有两个设备分别暴露 Intel NVMe 核心 0 1
```

  /dev/mapper/nvmset0
  /dev/mapper/nvmset1

```
### 在具4 个驱动器、使128K 区块大小striped 之上解除条带


```

  dmsetup create raid_disk0 --table '0 512 unstriped 4 256 0 /dev/mapper/striped 0'
  dmsetup create raid_disk1 --table '0 512 unstriped 4 256 1 /dev/mapper/striped 0'
  dmsetup create raid_disk2 --table '0 512 unstriped 4 256 2 /dev/mapper/striped 0'
  dmsetup create raid_disk3 --table '0 512 unstriped 4 256 3 /dev/mapper/striped 0'

```
