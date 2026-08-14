## 精简配置（Thin provisioning）


## 简介（Introduction）


本文档描述了一组 device-mapper 目标（target），它们共同实现了精简配置（thin-provisioning）与快照（snapshots）。

与之前的快照实现相比，本实现的主要亮点是它允许将许多虚拟设备存储在同一个数据卷上。这简化了管理，并允许在卷之间共享数据，从而减少磁盘使用量。

另一个重要特性是支持任意深度的递归快照（快照的快照的快照……）。之前的快照实现是通过将查找表链式连接来实现这一点的，因此性能为 O(深度)。本新实现使用单一数据结构来避免这种随深度下降的性能退化。不过，在某些场景下，碎片化仍然可能是个问题。

元数据与数据存储在不同的设备上，这给了管理员一些自由度，例如：

- 通过将元数据存储在一个镜像卷上、而数据存储在非镜像卷上，来提高元数据的弹性。

- 通过将元数据存储到 SSD 上来提高性能。

## 状态（Status）


这些目标被认为可安全用于生产环境。但不同的用例会有不同的性能特征，例如由于数据卷的碎片化。

如果您发现本软件的表现不符合预期，请将详细信息发送至 dm-devel@redhat.com，我们将尽力为您改进。

用于检查和修复元数据的用户空间工具已经开发完成，并作为 'thin_check' 与 'thin_repair' 提供。提供这些工具的软件包名称因发行版而异（在 Red Hat 发行版中它名为 'device-mapper-persistent-data'）。

## 速查手册（Cookbook）


本节描述了一些使用精简配置的快速配方。它们直接使用 dmsetup 程序来控制 device-mapper 驱动。一旦添加支持，最终用户将被建议使用更高层的卷管理器（如 LVM2）。

### 池设备（Pool device）


池设备将元数据卷与数据卷绑定在一起。它将 I/O 线性映射到数据卷，并通过两种机制更新元数据：

- 来自 thin 目标的函数调用

- 来自用户空间的 device-mapper 'messages'，用于控制（除其他事项外）新虚拟设备的创建。

### 建立一个新的池设备


建立一个池设备需要一个有效的元数据设备和一个数据设备。如果您没有现成的元数据设备，可以通过将前 4k 清零来指示其为空元数据。

    dd if=/dev/zero of=$metadata_dev bs=4096 count=1

您需要的元数据量会根据 thin 设备之间共享的块数量（即通过快照共享）而变化。如果共享程度低于平均水平，您将需要一个大于平均大小的元数据设备。

作为参考，我们建议您将元数据设备中使用的字节数计算为 48 * $data_dev_size / $data_block_size，但如果结果小于 2MiB 则向上取整到 2MiB。如果您正在创建大量记录大量变更的快照，可能会发现需要增大该值。

支持的最大大小为 16GiB：如果设备更大，将发出警告，多余的空间不会被使用。

### 重新加载池表（Reloading a pool table）


您可以重新加载一个池的表，实际上，当池空间耗尽时就是这样来调整池的大小的。（注意：虽然目前并不禁止在重新加载时指定不同的元数据设备，但如果它没有将 I/O 路由到与之前完全相同的磁盘位置，事情就会出错。）

### 使用现有的池设备


```
    dmsetup create pool \
	--table "0 20971520 thin-pool $metadata_dev $data_dev \
		 $data_block_size $low_water_mark"

```
$data_block_size 给出了一次可以分配的最小磁盘空间单位，以 512 字节扇区为单位。$data_block_size 必须介于 128（64KiB）与 2097152（1GiB）之间，且为 128（64KiB）的倍数。$data_block_size 在 thin-pool 创建后无法更改。主要对精简配置感兴趣的人可能想使用诸如 1024（512KiB）这样的值。进行大量快照的人可能想要较小的值，例如 128（64KiB）。如果您不对新分配的数据进行清零，则建议使用较大的 $data_block_size，约为 262144（128MiB）。

$low_water_mark 以 $data_block_size 大小的块为单位。如果数据设备上的空闲空间降到此级别以下，则会触发一个 dm 事件，用户空间守护进程应该捕获该事件以扩展池设备。只会发送这样一个事件。

如果刚恢复的设备其空闲空间低于低水位线，则不会触发特殊事件。但是，恢复一个设备总会触发一个事件；用户空间守护进程在处理此事件时应确认空闲空间超过了低水位线。

元数据设备的低水位线由内核维护，如果元数据设备上的空闲空间降到其以下，将触发一个 dm 事件。

### 更新磁盘上的元数据


磁盘上的元数据在每次写入 FLUSH 或 FUA bio 时提交。如果没有发出此类请求，则每秒提交一次。这意味着精简配置目标的行为类似于具有易失写缓存的物理磁盘。如果断电，您可能会丢失一些最近的写入。尽管发生任何崩溃，元数据应当始终保持一致。

如果数据空间耗尽，池将根据配置报错或排队 IO（参见：error_if_no_space）。如果元数据空间耗尽或元数据操作失败：池将报错 IO，直到池被下线并对元数据执行修复以 1) 修复任何潜在的不一致，以及 2) 清除施加修复要求的标志。一旦池的元数据设备被修复，就可以对其调整大小，这将使池恢复正常操作。请注意，如果一个池被标记为需要修复，则在执行修复之前，池的数据和元数据设备都无法调整大小。还应指出，当池的元数据空间耗尽时，当前的元数据事务会被中止。鉴于池会缓存其完成可能已经向上层 IO（例如文件系统）确认的 IO，强烈建议在需要对池进行修复时，对这些层执行一致性检查（例如 fsck）。

### 精简配置


i) 创建一个新的精简配置卷。

  要创建一个新的精简配置卷，您必须向池发送一条消息，内容如下：

```
    dmsetup message /dev/mapper/pool 0 "create_thin 0"

```
  这里的 '0' 是卷的标识符，一个 24 位数字。由调用者负责分配和管理这些标识符。如果该标识符已在使用中，消息将以 -EEXIST 失败。

```
```
ii) 使用精简配置卷。

```
    dmsetup create thin --table "0 2097152 thin /dev/mapper/pool 0"

```
  最后一个参数是 thinp 设备的标识符。

### 内部快照（Internal snapshots）


i) 创建一个内部快照。

  快照是通过向池发送另一条消息来创建的。

  注意：如果您希望快照的源设备（origin device）处于活动状态，必须在创建快照之前将其挂起（suspend）以避免损坏。这一点目前并未被强制，所以请小心！

```
    dmsetup suspend /dev/mapper/thin
    dmsetup message /dev/mapper/pool 0 "create_snap 1 0"
    dmsetup resume /dev/mapper/thin

```
  这里的 '1' 是卷的标识符，一个 24 位数字。'0' 是源设备的标识符。

```
```
ii) 使用内部快照。

  一旦创建，用户不必担心源与快照之间的任何连接。实际上，该快照与任何其他精简配置设备并无不同，并且可以通过相同的方法对其进行快照。只激活其中之一是完全合法的，并且对激活或移除它们两者没有顺序要求。（这与传统的 device-mapper 快照不同。）

```
    dmsetup create snap --table "0 2097152 thin /dev/mapper/pool 1"

```
### 外部快照（External snapshots）


您可以使用一个外部的**只读**设备作为精简配置卷的源。对 thin 设备未配置区域的任何读取都会透传到该源。写入会像往常一样触发新块的分配。

一个用例是 VM 宿主机希望在精简配置卷上运行客户机，但将基础镜像放在另一个设备上（可能在多个 VM 之间共享）。

如果您使用此技术，绝不能写入源设备！当然，您可以写入 thin 设备并对 thin 卷拍摄内部快照。

i) 创建外部设备的快照

  这与创建一个 thin 设备相同。在此阶段您无需提及源。

```
    dmsetup message /dev/mapper/pool 0 "create_thin 0"

```
ii) 使用外部设备的快照。

```
    dmsetup create snap --table "0 2097152 thin /dev/mapper/pool 0 /dev/image"

```
  注意：此快照的所有后代（内部快照）都需要相同的额外源参数。

### 停用（Deactivation）


所有使用某个池的设备都必须在该池本身之前被停用。

```
    dmsetup remove thin
    dmsetup remove snap
    dmsetup remove pool

```
## 参考（Reference）


### 'thin-pool' 目标


i) 构造函数（Constructor）

```
      thin-pool <metadata dev> <data dev> <data block size (sectors)> \
	        <low water mark (blocks)> [<number of feature args> [<arg>]*]

    Optional feature arguments:

      skip_block_zeroing:
	Skip the zeroing of newly-provisioned blocks.

      ignore_discard:
	Disable discard support.

      no_discard_passdown:
	Don't pass discards down to the underlying
	data device, but just remove the mapping.

      read_only:
		 Don't allow any changes to be made to the pool
		 metadata.  This mode is only available after the
		 thin-pool has been created and first used in full
		 read/write mode.  It cannot be specified on initial
		 thin-pool creation.

      error_if_no_space:
	Error IOs, instead of queueing, if no space.

    Data block size must be between 64KiB (128 sectors) and 1GiB
    (2097152 sectors) inclusive.


```
ii) 状态（Status）

```
      <transaction id> <used metadata blocks>/<total metadata blocks>
      <used data blocks>/<total data blocks> <held metadata root>
      ro|rw|out_of_data_space [no_]discard_passdown [error|queue]_if_no_space
      needs_check|- metadata_low_watermark

    transaction id:
	A 64-bit number used by userspace to help synchronise with metadata
	from volume managers.

    used data blocks / total data blocks
	If the number of free blocks drops below the pool's low water mark a
	dm event will be sent to userspace.  This event is edge-triggered and
	it will occur only once after each resume so volume manager writers
	should register for the event and then check the target's status.

    held metadata root:
	The location, in blocks, of the metadata root that has been
	'held' for userspace read access.  '-' indicates there is no
	held root.

    discard_passdown|no_discard_passdown
	Whether or not discards are actually being passed down to the
	underlying device.  When this is enabled when loading the table,
	it can get disabled if the underlying device doesn't support it.

    ro|rw|out_of_data_space
	If the pool encounters certain types of device failures it will
	drop into a read-only metadata mode in which no changes to
	the pool metadata (like allocating new blocks) are permitted.

	In serious cases where even a read-only mode is deemed unsafe
	no further I/O will be permitted and the status will just
	contain the string 'Fail'.  The userspace recovery tools
	should then be used.

    error_if_no_space|queue_if_no_space
	If the pool runs out of data or metadata space, the pool will
	either queue or error the IO destined to the data device.  The
	default is to queue the IO until more space is added or the
	'no_space_timeout' expires.  The 'no_space_timeout' dm-thin-pool 模块参数
	可以用来改变此超时 -- it
	defaults to 60 seconds but may be disabled using a value of 0.

    needs_check
	A metadata operation has failed, resulting in the needs_check
	flag being set in the metadata's superblock.  The metadata
	device must be deactivated and checked/repaired before the
	thin-pool can be made fully operational again.  '-' indicates
	needs_check is not set.

    metadata_low_watermark:
	Value of metadata low watermark in blocks.  The kernel sets this
	value internally but userspace needs to know this value to
	determine if an event was caused by crossing this threshold.

```
iii) 消息（Messages）

    create_thin <dev id>
	创建一个新的精简配置设备。
	<dev id> 是由调用方选择的任意唯一的 24 位标识符，
	由调用方选择。

    create_snap <dev id> <origin id>
	创建另一个精简配置设备的快照。
	<dev id> 是由调用方选择的任意唯一的 24 位标识符，
	由调用方选择。
	<origin id> 是被快照的精简配置设备的标识符，
	新设备即该设备的快照。

    delete <dev id>
	删除一个精简设备。不可逆。

    set_transaction_id <current id> <new id>
	用户态卷管理器（如 LVM）需要一种方式来
	将其外部元数据与池目标的内部元数据进行同步。
	池目标。thin-pool 目标提供存储一个
	任意的 64 位事务 ID，并在目标的
	状态行上返回它。为避免竞态，你必须提供你所认为的
	当前事务 ID，才能用这条
	compare-and-swap 消息修改它时。

    reserve_metadata_snap
        为用户态保留一份数据映射 btree 的副本。
        这允许用户态检查执行此消息时
        的映射。使用池的状态命令来
        获取与元数据快照关联的根块。

    release_metadata_snap
        释放之前保留的数据映射 btree 副本。

### 'thin' 目标


i) 构造函数（Constructor）

```
        thin <pool dev> <dev id> [<external origin dev>]

    pool dev:
	the thin-pool device, e.g. /dev/mapper/my_pool or 253:0

    dev id:
	the internal device identifier of the device to be
	activated.

    external origin dev:
	an optional block device outside the pool to be treated as a
	read-only snapshot origin: reads to unprovisioned areas of the
	thin target will be mapped to this device.

```
池不会针对 thin 设备存储任何大小。如果您加载的 thin 目标比之前使用的小，那么您将无法访问映射到末尾之外的块。如果您加载的目标比以前大，则额外的块将在需要时按需配置。

ii) 状态（Status）

    <nr mapped sectors> <highest mapped sector>
	如果池遇到设备错误并失败，其状态
	将仅包含字符串 'Fail'。应当使用用户态恢复
	工具。

    当 <nr mapped sectors> 为 0 时，不存在最高的
    已映射扇区，且 <highest mapped sector> 的值未指定。
