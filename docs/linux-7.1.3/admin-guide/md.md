## RAID 阵列


### 启动时组装 RAID 阵列


管理 md 设备的工具可在以下位置找到：
   https://www.kernel.org/pub/linux/utils/raid/


你可以使用以下内核命令行参数来用你的 md 设备启动：

```
  md=<md device no.>,<raid level>,<chunk size factor>,<fault level>,dev0,dev1,...,devn

```
```
  md=<md device no.>,dev0,dev1,...,devn

```
```
  md=d<md device no.>,dev0,dev1,...,devn

```
`md device no.`
+++++++++++++++++

md 设备的编号

================= =========
`md device no.` device
================= =========
              0		md0
	      1		md1
	      2		md2
	      3		md3
	      4		md4
================= =========

`raid level`
++++++++++++++

RAID 阵列的级别

=============== =============
`raid level`  level
=============== =============
-1		linear mode
0		striped mode
=============== =============

其他模式仅在具有持久超级块（persistent super blocks）时才受支持

`chunk size factor`
+++++++++++++++++++++

（仅 raid-0 和 raid-1）

将块大小设为 4k << n。

`fault level`
+++++++++++++++

完全被忽略

`dev0` to `devn`
++++++++++++++++++++

e.g. `/dev/hda1`, `/dev/hdc1`, `/dev/sda1`, `/dev/sdb1`

```
	e:\loadlin\loadlin e:\zimage root=/dev/md0 md=0,0,4,0,/dev/hdb2,/dev/hdc3 ro

```
### 启动时自动探测 RAID 阵列


当 md 被编译进内核（而非作为模块）时，类型为 0xfd 的分区会被扫描并自动组装为 RAID 阵列。这种自动探测可以通过内核参数 `raid=noautodetect` 来禁止。自内核 2.6.9 起，只有带有 0 类型超级块的驱动器才能被自动探测并在启动时运行。

内核参数 `raid=partitionable`（或 `raid=part`）意味着所有自动探测到的阵列都被组装为可分区（partitionable）的形式。

### 启动时组装降级/脏阵列


如果一个 raid5 或 raid6 阵列既处于脏（dirty）又处于降级（degraded）状态，它可能会有无法检测到的数据损坏。这是因为它处于 `dirty` 状态意味着奇偶校验不可信，而它处于降级状态意味着某些数据块缺失且无法可靠地重建（因为没有奇偶校验）。

出于这个原因，md 通常会拒绝启动这样的阵列。这需要系统管理员采取行动来显式启动该阵列。

```
   mdadm --assemble --force ....

```
如果阵列上有根文件系统，这个选项实际上不可用。为了支持从这样的阵列启动，md 支持一个模块参数 `start_dirty_degraded`，当将其设为 1 时，会绕过这些检查并允许启动脏的降级阵列。

```
   md-mod.start_dirty_degraded=1

```
### 超级块格式


md 驱动可以支持多种不同的超级块格式。目前，它支持 `0.90.0` 超级块格式以及在内核 2.5 开发系列中引入的 `md-1` 格式。

内核会自动探测正在使用的是哪种格式的超级块。

出于兼容性原因，超级块格式 `0` 的处理方式与其他格式不同——它是原始的超级块格式。


### 通用规则——适用于所有超级块格式


阵列是通过向所有设备写入适当的超级块来 `created`（创建）的。

它是通过将每个设备与特定的 md 虚拟设备关联起来而 `assembled`（组装）的。一旦完全组装完成，它就可以被访问。

阵列应由用户空间工具创建。这会将超级块写入所有设备。它通常会将阵列标记为 `unclean`（不干净），或者标记某些设备缺失，以便内核 md 驱动可以创建适当的冗余（在 raid 1 中复制，在 raid 4/5 中计算奇偶校验）。

当一个阵列被组装时，首先使用 SET_ARRAY_INFO ioctl 进行初始化。它特别包含主版本号和次版本号。主版本号选择要使用的超级块格式。次版本号可能用于调整该格式的处理方式，例如建议在每个设备上何处查找超级块。

然后，使用 ADD_NEW_DISK ioctl 添加每个设备。它特别提供标识要添加设备的主、次设备号。

该阵列通过 RUN_ARRAY ioctl 启动。

启动后，可以添加新设备。应先向它们写入适当的超级块，然后通过 ADD_NEW_DISK 传入。

已失败或尚未激活的设备可以使用 HOT_REMOVE_DISK 从阵列中分离。


### 适用于 format-0 超级块阵列以及无超级块（非持久）阵列的特定规则


可以通过在 SET_ARRAY_INFO ioctl 中描述阵列（级别、块大小等）来 `created`（创建）一个阵列。这必须具有 `major_version==0` 且 `raid_disks != 0`。

然后，可以使用 ADD_NEW_DISK 添加未初始化的设备。传给 ADD_NEW_DISK 的结构必须指定设备的状态及其在阵列中的角色。

一旦通过 RUN_ARRAY 启动，就可以使用 HOT_ADD_DISK 添加未初始化的热备盘。


### sysfs 中的 MD 设备


md 设备作为常规块设备出现在 sysfs（`/sys`）中，

```
   /sys/block/md0

```
每个 `md` 设备都包含一个名为 `md` 的子目录，其中存放着关于该设备的更多 md 特定信息。

所有 md 设备都包含：

  level
     一个文本文件，指示 `raid level`（RAID 级别），例如 raid0、raid1、
     raid5、linear、multipath、faulty。
     如果尚未设置 RAID 级别（阵列仍在组装中），该值将反映已写入
     的内容，可能是上述名称之一，也可能是诸如 `0`、`5` 等数字。

  raid_disks
     一个包含简单数字的文本文件，指示一个功能完整的阵列中的
     设备数量。如果尚未可知，该文件为空。如果阵列正在调整大小，
     它将包含新的设备数量。
     某些 RAID 级别允许在阵列处于活动状态时设置此值。这会重新配置
     阵列。否则，只能在组装阵列时设置。
     如果改变此属性会缩小阵列的大小，则不允许更改。要减少
     例如 raid5 中的驱动器数量，必须首先通过设置 `array_size`
     属性来缩小阵列大小。

  chunk_size
     这是 `chunks`（块）的字节大小，仅与涉及条带化（striping）的
     RAID 级别（0、4、5、6、10）相关。阵列的地址空间在概念上被
     划分为块，连续的块被条带化到相邻的设备上。
     该大小应至少为 PAGE_SIZE（4k），并且应为 2 的幂。
     这只能在组装阵列时设置。

  layout
     特定级别下阵列的 `layout`（布局）。这只是一个数字，由不同的
     级别以不同方式解释。它可以在组装阵列时写入。

  array_size
     这可用于人为地将阵列中可用的空间限制为小于合并设备上实际
     可用的空间。写入一个小于可用大小的数字（单位千字节）将设置
     该大小。对阵列的任何重新配置（例如添加设备）都不会导致大小
     改变。写入单词 `default` 会使阵列的有效大小变为基于 `level`、
     `chunk_size` 和 `component_size` 实际可用的任意大小。

     这可用于在减少 raid4/5/6 中设备数量之前先缩小阵列大小，
     或用于支持要求此类裁剪的外部元数据格式。

  logical_block_size
     配置阵列的逻辑块大小（以字节为单位）。此属性仅支持 1.x 元数据。
     在启动阵列之前写入该值。最终阵列的 LBS 取此配置与所有合并设备
     LBS 之间的最大值。注意，在 RAID 支持 folio 之前，LBS 不能超过
     PAGE_SIZE。
     警告：在新内核上创建的阵列由于填充检查无法在旧内核上组装，
     可将模块参数 'check_new_feature' 设为 false 来绕过，但可能会
     导致数据丢失。

  reshape_position
     这是 `none`，或者是阵列设备内 `reshape` 已进行到的扇区号。
     如果设置了此项，上述三个属性（raid_disks、chunk_size、layout）
     可能具有两个值，即旧值和新值。如果它们

```
        new (old)

     and writing will effect the ``new`` value, leaving the ``old``
     unchanged.

  component_size
     For arrays with data redundancy (i.e. not raid0, linear, faulty,
     multipath), all components must be the same size - or at least
     there must a size that they all provide space for.  This is a key
     part or the geometry of the array.  It is measured in sectors
     and can be read from here.  Writing to this value may resize
     the array if the personality supports it (raid1, raid5, raid6),
     and if the component drives are large enough.

  metadata_version
     This indicates the format that is being used to record metadata
     about the array.  It can be 0.90 (traditional format), 1.0, 1.1,
     1.2 (newer format in varying locations) or ``none`` indicating that
     the kernel isn't managing metadata at all.
     Alternately it can be ``external:`` followed by a string which
     is set by user-space.  This indicates that metadata is managed
     by a user-space program.  Any device failure or other event that
     requires a metadata update will cause array activity to be
     suspended until the event is acknowledged.

  resync_start
     The point at which resync should start.  If no resync is needed,
     this will be a very large number (or ``none`` since 2.6.30-rc1).  At
     array creation it will default to 0, though starting the array as
     ``clean`` will set it much larger.

  new_dev
     This file can be written but not read.  The value written should
     be a block device number as major:minor.  e.g. 8:0
     This will cause that device to be attached to the array, if it is
     available.  It will then appear at md/dev-XXX (depending on the
     name of the device) and further configuration is then possible.

  safe_mode_delay
     When an md array has seen no write requests for a certain period
     of time, it will be marked as ``clean``.  When another write
     request arrives, the array is marked as ``dirty`` before the write
     commences.  This is known as ``safe_mode``.
     The ``certain period`` is controlled by this file which stores the
     period as a number of seconds.  The default is 200msec (0.200).
     Writing a value of 0 disables safemode.

  array_state
     This file contains a single word which describes the current
     state of the array.  In many cases, the state can be set by
     writing the word for the desired state, however some states
     cannot be explicitly set, and some transitions are not allowed.

     Select/poll works on this file.  All changes except between
     Active_idle and active (which can be frequent and are not
     very interesting) are notified.  active->active_idle is
     reported if the metadata is externally managed.

     clear
         No devices, no size, no level

         Writing is equivalent to STOP_ARRAY ioctl

     inactive
         May have some settings, but array is not active
         all IO results in error

         When written, doesn't tear down array, but just stops it

     suspended (not supported yet)
         All IO requests will block. The array can be reconfigured.

         Writing this, if accepted, will block until array is quiescent

     readonly
         no resync can happen.  no superblocks get written.

         Write requests fail

     read-auto
         like readonly, but behaves like ``clean`` on a write request.

     clean
         no pending writes, but otherwise active.

         When written to inactive array, starts without resync

         If a write request arrives then
         if metadata is known, mark ``dirty`` and switch to ``active``.
         if not known, block and switch to write-pending

         If written to an active array that has pending writes, then fails.
     active
         fully active: IO and resync can be happening.
         When written to inactive array, starts with resync

     write-pending
         clean, but writes are blocked waiting for ``active`` to be written.

     active-idle
         like active, but no writes have been seen for a while (safe_mode_delay).

  consistency_policy
     This indicates how the array maintains consistency in case of unexpected
     shutdown. It can be:

     none
       Array has no redundancy information, e.g. raid0, linear.

     resync
       Full resync is performed and all redundancy is regenerated when the
       array is started after unclean shutdown.

     bitmap
       Resync assisted by a write-intent bitmap.

     journal
       For raid4/5/6, journal device is used to log transactions and replay
       after unclean shutdown.

     ppl
       For raid5 only, Partial Parity Log is used to close the write hole and
       eliminate resync.

     The accepted values when writing to this file are ``ppl`` and ``resync``,
     used to enable and disable PPL.

  uuid
     This indicates the UUID of the array in the following format:
     xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx

  bitmap_type
     [RW] When read, this file will display the current and available
     bitmap for this array. The currently active bitmap will be enclosed
     in [] brackets. Writing an bitmap name or ID to this file will switch
     control of this array to that new bitmap. Note that writing a new
     bitmap for created array is forbidden.

```
如果 bitmap_type 不为 none，则在 md 设备 KOBJ_CHANGE 事件之后会创建额外的 bitmap 属性 bitmap/xxx 或 llbitmap/xxx。

如果 bitmap_type 为 bitmap，则 md 设备还将包含：

  bitmap/location
     这指示阵列的写意图位图（write-intent bitmap）存储在哪里。

     它可以是 `none`、`file` 或 `[+-]N` 之一。
     `file` 以后可能扩展为 `file:/file/name`。
     `[+-]N` 表示从元数据起始处起那么多扇区。

     这会在所有设备上复制。对于具有外部管理元数据的阵列，
     偏移量是从设备开头算起。

  bitmap/chunksize
     由单个位所表示的块的字节大小。对于 RAID456，它是单个设备的
     一部分；对于 RAID10，它是阵列的一部分；对于 RAID1，两者皆是
     （结果相同）。

  bitmap/time_base
     两次查找位图中待清除位之间的时间间隔（秒）。在当前实现中，
     当所有被覆盖的块已知处于同步（in-sync）状态后，一个位会在
     2 到 3 倍的 `time_base` 时间内被清除。

  bitmap/backlog
     当 RAID1 中存在 write-mostly 设备处于活动状态时，对这些设备的
     写请求在后台进行——文件系统（或设备的其他使用者）不必等待它们。
     `backlog` 设置并发后台写入数量的限制。如果超过此限制，新的
     写入将变为同步的。

  bitmap/metadata
     可以是 `internal` 或 `external`。

     `internal`
       是默认值，意味着位图的元数据存储在所分配空间的前 256 字节中，
       并由 md 模块管理。

     `external`
       意味着位图元数据由内核之外（即由某个用户空间程序）管理。

  bitmap/can_clear
     这是 `true` 或 `false`。如果为 `true`，则当相应块被认为处于
     同步状态时，位图中的位将被清除。如果为 `false`，位将永远不会
     被清除。如果在降级阵列上发生写入，或者阵列在写入期间变为降级，
     此项会自动设为 `false`。当元数据由外部管理时，一旦阵列变为
     非降级状态并且此事实已记录到元数据中，应将其设为 true。

如果 bitmap_type 为 llbitmap，则 md 设备还将包含：

  llbitmap/bits
     只读，显示位图位的状态，即每个值的数量。

  llbitmap/metadata
     只读，显示位图元数据，包括 chunksize、chunkshift、chunks、
     offset 和 daemon_sleep。

  llbitmap/daemon_sleep
     可读写，即守护进程函数被触发以清除脏位的间隔时间（秒）。

  llbitmap/barrier_idle
     可读写，即页面屏障空闲的时间（秒），意味着页面中的脏位
     将被清除。

随着组件设备被添加到 md 阵列，它们会出现在 `md`

```
      dev-XXX

```
其中 `XXX` 是内核所知的该设备名称，例如 hdb1。
每个目录包含：

```
	     /sys/block/md0/md/dev-hdb1/block -> ../../../../block/hdb/hdb1

```
      super
        一个包含从该设备读取或写入该设备的超级块映像的文件。

      state
	一个记录设备在阵列中当前状态的文件，可以是逗号分隔的列表：

	      faulty
			设备由于检测到故障，或存在未确认的坏块，
			而被移出活动使用。

	      in_sync
			设备是阵列中完全同步的成员。

	      writemostly
			设备仅在无其他可选方案时才会被提交读请求。

			这仅适用于 raid1 阵列。

	      blocked
			设备已失败，且故障尚未被元数据处理程序确认。

			本应写入该设备的写请求（若其未故障）会被阻塞。

	      spare
			设备工作正常，但不是完整成员。

			这包括正在恢复过程中的热备盘。

	      write_error
			设备曾经出现过写错误。

	      want_replacement
			设备（大多）工作正常，但可能应该被替换，无论是
			由于错误还是用户请求。

	      replacement
			设备是用于替换另一个具有相同 raid_disk 的活动设备的
			替代设备。


	此列表将来可能会扩充。

	此文件可写入。

	写入 ``faulty`` 模拟设备上的故障。

	写入 ``remove`` 将设备从阵列中移除。

	写入 ``writemostly`` 设置 writemostly 标志。

	写入 ``-writemostly`` 清除 writemostly 标志。

	写入 ``blocked`` 设置 ``blocked`` 标志。

	写入 ``-blocked`` 清除 ``blocked`` 标志，并允许写请求
	完成，且可能模拟一个错误。

	写入 ``in_sync`` 设置 in_sync 标志。

	写入 ``write_error`` 设置 writeerrorseen 标志。

	写入 ``-write_error`` 清除 writeerrorseen 标志。

	除替换设备或热备盘外，可随时写入 ``want_replacement``。它会设置该标志。

	可随时写入 ``-want_replacement``。它会清除该标志。

	仅在启动阵列之前允许写入 ``replacement`` 或 ``-replacement``。它会设置或清除该标志。


	此文件响应 select/poll。对 ``faulty`` 或 ``blocked`` 的任何更改都会触发一个事件。

      errors
	在此设备上检测到但尚未导致设备被移出阵列的读错误的近似计数
	（可能是因为它们已被纠正，或者因为它们发生在阵列处于只读状态时）。
	当使用 version-1 元数据时，此值会在阵列重启后保持。

	此值可在组装阵列时写入，从而为具有用户空间管理元数据的
	阵列提供一个持续的计数。

      slot
        这给出设备在该阵列中的角色。如果设备不在阵列中活动
        （即它是热备盘或已失败），则为 ``none``，否则为小于阵列的
        ``raid_disks`` 数量的一个整数，指示它当前填充的位置。
        这只能在组装阵列时设置。设置了此项值的设备被视为正常工作。

      offset
        这给出设备中（从起始处起的扇区数）存储阵列数据的位置。
        该偏移量之前的设备部分不会被触碰，除非它用于存储元数据
        （格式 1.1 和 1.2）。

      size
        偏移量之后可用于数据存储的设备容量。通常与
	component_size 相同。这可在组装阵列时写入。如果写入的值
        小于当前 component_size，则会被拒绝。

      recovery_start
        当设备不处于 ``in_sync`` 状态时，这记录从设备起始处起已知的
	正确扇区数。通常为 0，但在恢复操作期间会稳定增加；如果恢复
	被中断，恢复此值可使恢复避免重复较早的块。对于 v1.x 元数据，
	此值会被自动保存和恢复。

	只要设备不是阵列的活动成员，无论是在阵列激活之前还是
	在 ``slot`` 设置之前，都可以设置此项。

	将其设为 ``none`` 等同于设置 ``in_sync``。
	设为任何其他值也会清除 ``in_sync`` 标志。

      bad_blocks
	这以起始地址和长度（单位均为扇区）的形式给出所有已知坏块的
	列表。如果输出过大而无法放入一页，将被截断。向此文件写入
	``sector length`` 会添加新的已确认（即已安全记录到磁盘）坏块。

      unacknowledged_bad_blocks
	这以与 ``bad_blocks`` 相同的形式给出已知但尚未保存到磁盘的
	坏块列表。如果输出过大而无法放入一页，将被截断。写入此文件
	会添加坏块而不确认它们。这主要用于测试。

      ppl_sector, ppl_size
        此设备上用于部分奇偶校验日志（Partial Parity Log）的
        空间的起始位置和大小（单位扇区）。


一个活跃的 md 设备还会包含每个活动设备的条目

```
    rdNN

```
其中 `NN` 是阵列中的位置，从 0 开始。
因此，对于一个由 3 个驱动器组成的阵列，将有 rd0、rd1、rd2。
它们是指向相应 `dev-XXX` 条目的符号链接。

```
       cat /sys/block/md*/md/rd*/state

```
将在每一行显示 `in_sync`。



支持数据冗余（1、4、5、6、10）级别的活跃 md 设备还包含

   sync_action
     一个可用于监视和控制重建过程的文本文件。它包含一个单词，
     可以是以下之一：

       resync（重新同步）
		在脏关闭或创建之后正在重新计算冗余

       recover（恢复）
		正在构建热备盘以替换失败/缺失的设备

       idle（空闲）
		没有任何操作发生
       check（检查）
		已请求并正在进行冗余的全面检查。这会读取所有块并
                检查它们。对于某些 RAID 级别，也可能进行修复。

       repair（修复）
		正在进行全面的检查和修复。这与 `resync` 类似，但由
                用户请求，并且不使用写意图位图来优化过程。

      该文件可写，每个可读的字符串对于写入都有意义。

	`idle` 将停止活动的重新同步/恢复等操作。无法保证不会再次
	自动启动另一次重新同步/恢复，但需要有某个事件来触发。

	如果操作被 `idle` 停止，可以使用 `resync` 或 `recovery`
        来重新启动相应的操作。

	如果当前状态为 `idle`，`check` 和 `repair` 将启动相应的过程。

      该文件响应 select/poll。值的任何重要变化都会触发一次 poll 事件。
      有时，如果需要恢复但无法完成时，该值会短暂地为 `recover`。
      在这种情况下，向 `recover` 的转换不会被通知，但离开该状态的
      转换会被通知。

   degraded
      这包含阵列降级所缺少的设备数量计数。因此，最优阵列将显示 `0`，
      单个失败/缺失的驱动器将显示 `1`，依此类推。

      该文件响应 select/poll，缺失设备计数的任何增加或减少都会触发事件。

   mismatch_count
      在执行 `check` 和 `repair` 时，以及可能执行 `resync` 时，md 会
      统计发现的错误数量。`mismatch_cnt` 中的计数是被重写，或
      （对于 `check`）本应被重写的扇区数。由于大多数 RAID 级别以页
      而非扇区为单位工作，因此该值可能比实际错误数量大一个页中
      扇区数的倍数。

   bitmap_set_bits
      如果阵列具有写意图位图，则写入此属性可在位图中设置位，
      指示重新同步需要检查相应的块。可以写入单个数字或起始-结束
      对。多个数字可以用空格分隔。

      注意，这些数字是 `bit`（位）编号，而非 `block`（块）编号。
      它们应按 bitmap_chunksize 缩放。

   sync_speed_min, sync_speed_max
     这与 `/proc/sys/dev/raid/speed_limit_{min,max}` 类似，但仅适用于
     特定的阵列。

     如果对这些文件没有写入任何值，或者写入了单词 `system`，则使用
     系统范围的值。如果写入了以 kibibytes-per-second（千字节/秒）为
     单位的值，则使用该值。

     读取这些文件时，它们显示当前活动的值，后跟 `(local)` 或
     `(system)`，具体取决于它是本地设置的值还是系统范围的值。

   sync_completed
     这显示当前 sync_action 已完成处理的扇区数，以及总共可能需要
     处理的扇区数。两个数字以 `/` 分隔，因此实际上显示一个值，
     即已完成的进程比例。

     当重新同步完成时、当达到当前 sync_max（见下）时，以及可能在其他
     时候，对此属性的 `select` 会返回。

   sync_speed
     这显示当前 sync_action 的实际当前速度，单位为 K/秒。它是最近
     30 秒的平均值。

   suspend_lo, suspend_hi
     这两个值以扇区数给出，指示阵列中 IO 将被阻塞的范围。目前
     仅支持 raid4/5/6。

   sync_min, sync_max
     这两个值以扇区数给出，指示 `check`/`repair` 将操作的范围。必须
     为 chunk_size 的倍数。当达到 `sync_max` 时，它会暂停而非完成。
     可以使用 `sync_completed` 上的 `select` 或 `poll` 来等待该数字
     达到 sync_max。然后可以增加 `sync_max`，或向 `sync_action` 写入
     `idle`。

     `sync_max` 的 `max` 值实际上会禁用该限制。当重新同步处于活动
     状态时，该值只能增加，绝不能减少。
     `sync_min` 的最小值为 `0`。



每个活跃的 md 设备还可能具有特定于管理它的 personality 模块的属性。
这些属性特定于该模块的实现，如果实现发生变化，可能会发生
重大改变。

这些目前包括：

  stripe_cache_size  （目前仅 raid5）
      条带缓存（stripe cache）中的条目数。此值可写，但有上下限
      （32768、17）。默认值为 256。

  strip_cache_active （目前仅 raid5）
      条带缓存中活动条目的数量

  preread_bypass_threshold （目前仅 raid5）
      需要预读的条带被不需要预读的条带所绕过的次数。为公平起见，
      默认为 1。将其设为 0 会禁用绕过计数，并要求预读条带等待所有
      全宽条带写入完成。有效值范围为 0 到 stripe_cache_size。

  journal_mode （目前仅 raid5）
      raid5 的缓存模式。raid5 可以包含一个额外的磁盘用于缓存。
      模式可以是 "write-through"（透写）或 "write-back"（回写）。
      默认为 "write-through"。

  ppl_write_hint
      为每个 PPL 写请求设置的 NVMe 流 ID。
