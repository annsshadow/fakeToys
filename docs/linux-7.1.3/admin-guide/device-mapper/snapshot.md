## 设备映射器快照支持（Device-mapper snapshot support

设备映射器允许你在不进行大量数据复制的情况下
- 创建任意块设备的快照，即可挂载的、已保存的块设备状态，并且这些状态还可写，而不会干扰原始内容；
- 创建设备“分支”（forks），即同一数据流的不同版本- 将块设备的快照合并回该快照的源（origin）设备
在前两种情况下，dm 只复制发生变化的数据块，并使用一个独立的写时复制（COW）块设备进行存储
对于快照合并，COW 存储中的内容会被合并回源设备

共有三个可用dm 目标：snapshot、snapshot-origin snapshot-merge
- snapshot-origin <origin>

通常会在其上建立一个或多个快照。读操作将直接映射到后端设备。对于每次写操作，原始数据将保存在每个快照的 <COW device> 中，以保持其可见内容不变，至少直<COW device> 填满为止

- snapshot <origin> <COW device> <persistent?> <chunksize>
   [<# feature args> [<arg>]*]

将创<origin> 块设备的一个快照。大小为 <chunksize> 扇区的数据块变更将存储在 <COW device> 上。写操作只会写入 <COW device>。读操作对于未更改的数据将来<COW device> <origin>COW device> 通常小于源设备，如果它填满，快照将变得无用并被禁用，返回错误。因此监控空闲空间数量并<COW device> 填满之前对其进行扩展很重要
<persistent> P（Persistent，持久）N（Not persistent，不持久——重启后不保留）。O（Overflow，溢出）可作为持久存储选项添加，以允许用户态通告其支持在快照状态中看到“Overflow”。因此支持的存储类型"P"PO" "N"
持久与瞬态（transient）之间的区别在于：瞬态快照必须保存在磁盘上的元数据更少——它们可以由内核保存在内存中
加载或卸载快照目标时，相应的 snapshot-origin snapshot-merge 目标必须被挂起。未能挂起源目标可能导致数据损坏
可选特性（features）：

   discard_zeroes_cow - 对映射到整个数据块的快照设备发出discard 会将
   快照异常存储中相应的异常（exception）清零
   discard_passdown_origin - 对快照设备发出的 discard 会被向下传   snapshot-origin 的底层设备。这不会导致向快照异常存储复制，
   因为 snapshot-origin 目标被绕过了
   discard_passdown_origin 特性依赖于 discard_zeroes_cow 特性被启用

- snapshot-merge <origin> <COW device> <persistent> <chunksize>
   [<# feature args> [<arg>]*]

除只适用于持久快照外，其表参数与 snapshot 目标相同。该目标承担
"snapshot-origin" 目标的角色，如果<origin> "snapshot-origin"
仍然存在时，不得加载它
创建一个合并快照，通过交接（handover）过程接管现有快照存储在
<COW device> 中的已变更数据块，并将这些数据块合并<origin>一旦合并开始（在后台）origin> 即可被打开，且合并将在 I/O 流向
它时继续进行。对 <origin> 的更改会被推迟，直到合并快照对应的数据块
已被合并。一旦合并开始，"snapshot" 目标关联的快照设备在被访问时
将返-EIO

## LVM2 如何使用快照（How snapshot is used by LVM2

当你创建某个卷的第一LVM2 快照时，会使用四dm 设备
1) 一个包含源卷原始映射表的设备；
2) 一个用<COW device> 的设备；
3) 一"snapshot" 设备，组合了 #1 #2，即可见的快照卷4) 原始original"）卷（使用原始源卷使用的设备号），其表被
   #1 设备"snapshot-origin" 映射所替换
```
  lvcreate -L 1G -n base volumeGroup
  lvcreate -L 100M --snapshot -n snap volumeGroup/base

```
```
  # dmsetup table|grep volumeGroup

  volumeGroup-base-real: 0 2097152 linear 8:19 384
  volumeGroup-snap-cow: 0 204800 linear 8:19 2097536
  volumeGroup-snap: 0 2097152 snapshot 254:11 254:12 P 16
  volumeGroup-base: 0 2097152 snapshot-origin 254:11

  # ls -lL /dev/mapper/volumeGroup-*
  brw-------  1 root root 254, 11 29 ago 18:15 /dev/mapper/volumeGroup-base-real
  brw-------  1 root root 254, 12 29 ago 18:15 /dev/mapper/volumeGroup-snap-cow
  brw-------  1 root root 254, 13 29 ago 18:15 /dev/mapper/volumeGroup-snap
  brw-------  1 root root 254, 10 29 ago 18:14 /dev/mapper/volumeGroup-base


```
## LVM2 如何使用快照合并（How snapshot-merge is used by LVM2
合并快照在合并期间承"snapshot-origin" 的角色。因"snapshot-origin" 被替换为 "snapshot-merge"-real" 设备不变-cow" 设备被重命名<origin name>-cow，以协助 LVM2 在合并快照完成后进行清理。将COW 设备移交"snapshot-merge" "snapshot" 会被停用（除非使lvchange --refresh）；但如果它保持激活状态，则只会返I/O 错误
```
  lvconvert --merge volumeGroup/snap

```
```
  # dmsetup table|grep volumeGroup

  volumeGroup-base-real: 0 2097152 linear 8:19 384
  volumeGroup-base-cow: 0 204800 linear 8:19 2097536
  volumeGroup-base: 0 2097152 snapshot-merge 254:11 254:12 P 16

  # ls -lL /dev/mapper/volumeGroup-*
  brw-------  1 root root 254, 11 29 ago 18:15 /dev/mapper/volumeGroup-base-real
  brw-------  1 root root 254, 12 29 ago 18:16 /dev/mapper/volumeGroup-base-cow
  brw-------  1 root root 254, 10 29 ago 18:16 /dev/mapper/volumeGroup-base


```
## 如何判断合并何时完成（How to determine when a merging is complete
snapshot-merge snapshot 的状态行以以下内容结尾：

  <sectors_allocated>/<total_sectors> <metadata_sectors>

<sectors_allocated> <total_sectors> 都同时包含数据和元数据。在合并过程中，已分配扇区数会越来越小。当保存数据的扇区数为零时，<sectors_allocated> == <metadata_sectors> 时，合并完成
```
  # lvs
    LV      VG          Attr   LSize Origin  Snap%  Move Log Copy%  Convert
    base    volumeGroup owi-a- 4.00g
    snap    volumeGroup swi-a- 1.00g base  18.97

  # dmsetup status volumeGroup-snap
  0 8388608 snapshot 397896/2097152 1560
                                    ^^^^ metadata sectors

  # lvconvert --merge -b volumeGroup/snap
    Merging of volume snap started.

  # lvs volumeGroup/snap
    LV      VG          Attr   LSize Origin  Snap%  Move Log Copy%  Convert
    base    volumeGroup Owi-a- 4.00g          17.23

  # dmsetup status volumeGroup-base
  0 8388608 snapshot-merge 281688/2097152 1104

  # dmsetup status volumeGroup-base
  0 8388608 snapshot-merge 180480/2097152 712

  # dmsetup status volumeGroup-base
  0 8388608 snapshot-merge 16/2097152 16

```
合并已完成（Merging has finished）
```
  # lvs
    LV      VG          Attr   LSize Origin  Snap%  Move Log Copy%  Convert
    base    volumeGroup owi-a- 4.00g


```
