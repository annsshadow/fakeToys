
## SGI XFS 文件系统


XFS 是一个高性能的日志型文件系统，起源于 SGI IRIX 平台。它完全多线程，能够支持大文件与大容量文件系统、扩展属性、可变块大小，以 extent（区段）为基础，并广泛使用 Btree（目录、extent、空闲空间）来提升性能与可扩展性。

更多细节请参阅 https://xfs.wiki.kernel.org/ 上的文档。此实现在磁盘格式上与 IRIX 版本的 XFS 兼容。

## 挂载选项

挂载 XFS 文件系统时，接受以下选项。

  allocsize=size
	设置进行延迟分配写回（delayed allocation writeout）时的缓冲 I/O 文件尾（end-of-file）预分配大小（默认大小为 64KiB）。
	该选项的有效取值范围为页大小（通常为 4KiB）到 1GiB（含），以 2 的幂为步长递增。

	默认行为是动态文件尾预分配大小，它使用一组启发式方法来根据文件内当前的分配模式以及对文件的访问模式优化预分配大小。指定固定的 `allocsize` 值会关闭动态行为。

  discard 或 nodiscard（默认）
	启用/禁用下发命令，以让块设备回收文件系统释放的空间。这对于 SSD 设备、精简配置的 LUN 以及虚拟机镜像很有用，但可能会带来性能影响。

	注意：目前建议使用 `fstrim` 应用程序来 `discard` 未使用的块，而不是使用 `discard` 挂载选项，因为该选项的性能影响相当严重。

  grpid/bsdgroups 或 nogrpid/sysvgroups（默认）
	这些选项定义新创建的文件获得哪个组 ID。当设置了 `grpid` 时，它取创建所在目录的组 ID；否则取当前进程的 `fsgid`，除非该目录设置了 `setgid` 位，此时它会从父目录取 `gid`，并且如果它自身是目录还会被设置 `setgid` 位。

  filestreams
	使数据分配器在整个文件系统范围内使用 filestreams 分配模式，而不仅仅是在配置为使用该模式的目录上。

  inode32 或 inode64（默认）
	当指定 `inode32` 时，表示 XFS 将 inode 创建限制在不致于产生位数超过 32 位有效性的 inode 编号的位置。

	当指定 `inode64` 时，表示允许 XFS 在文件系统的任意位置创建 inode，包括那些会产生位数超过 32 位有效性的 inode 编号的位置。

	`inode32` 是为了与较旧系统和应用程序的向后兼容而提供的，因为 64 位 inode 编号可能会给某些无法处理大 inode 编号的应用程序带来问题。如果正在使用的应用程序无法处理大于 32 位的 inode 编号，则应指定 `inode32` 选项。

  largeio 或 nolargeio（默认）
	如果指定 `nolargeio`，则 **stat(2)** 在 `st_blksize` 中报告的最佳 I/O 将尽可能小，以允许用户应用程序避免低效的读/修改/写 I/O。这通常就是机器的页大小，因为这是页缓存的粒度。

	如果指定 `largeio`，那么在创建文件系统时指定了 `swidth` 的文件系统将在 `st_blksize` 中返回 `swidth` 值（以字节为单位）。如果文件系统没有指定 `swidth` 但指定了 `allocsize`，则将返回 `allocsize`（以字节为单位）。否则行为与指定了 `nolargeio` 时相同。

  logbufs=value
	设置内存中日志缓冲区的数量。有效数值范围为 2-8（含）。

	默认值为 8 个缓冲区。

	如果在小型系统上 8 个日志缓冲区的内存开销过高，则可以在某些元数据密集型工作负载的性能代价下减小它。下面的 `logbsize` 选项控制每个缓冲区的大小，因此也与这种情况相关。

  lifetime（默认）或 nolifetime
	基于用户提供的写入生命周期提示启用数据放置。当统计上有利于降低垃圾回收成本时，这会开启相似生命周期数据的共同分配（co-allocation）。

	这些选项仅适用于 zoned rt 文件系统。

  logbsize=value
	设置每个内存中日志缓冲区的大小。大小可以以字节指定，或以带 "k" 后缀的千字节指定。版本 1 和版本 2 日志的有效大小为 16384（16k）和 32768（32k）。版本 2 日志的有效大小还包括 65536（64k）、131072（128k）和 262144（256k）。`logbsize` 必须是 **mkfs(8)** 时配置的日志条带单元的整数倍。

	版本 1 日志的默认值为 32768，而版本 2 日志的默认值为 MAX(32768, log_sunit)。

  logdev=device 与 rtdev=device
	使用外部日志（元数据日志）和/或实时（real-time）设备。一个 XFS 文件系统最多由三部分组成：数据段、日志段和实时段。实时段是可选的，日志段可以与数据段分离，也可以包含在数据段之内。

  max_atomic_write=value
	设置原子写入的最大大小。大小可以以字节指定，以带 "k" 后缀的千字节、带 "m" 后缀的兆字节或带 "g" 后缀的吉字节指定。该大小不能大于最大写入大小、大于任何分配组（allocation group）的大小，或大于日志能够原子完成的重映射操作的大小。

	默认值是将最大 I/O 完成大小设置为允许每个 CPU 一次处理一个。

  max_open_zones=value
	指定在 zoned rt 设备上为写入保持打开的最大区域数。较多打开的区域有助于文件数据分离，但可能会影响 HDD 上的性能。

	如果未指定 `max_open_zones`，则该值由 zoned rt 设备的能力和大小决定。

  noalign
	数据分配将不在条带单元边界对齐。这仅与通过 **mkfs(8)** 以非零数据对齐参数（`sunit`、`swidth`）创建的文件系统相关。

  norecovery
	文件系统将在不运行日志恢复的情况下挂载。如果文件系统未能干净卸载，那么在以 `norecovery` 模式挂载时它可能处于不一致状态。由于这个原因，某些文件或目录可能无法访问。以 `norecovery` 挂载的文件系统必须以只读方式挂载，否则挂载将失败。

  nouuid
	不通过文件系统的 `uuid` 检查双重挂载的文件系统。这对于挂载 LVM 快照卷很有用，并且常与 `norecovery` 组合用于挂载只读快照。

  noquota
	强制关闭文件系统内的所有配额记账与强制。

  uquota/usrquota/uqnoenforce/quota
	启用用户磁盘配额记账，并（可选地）强制限制。更多细节请参阅 **xfs_quota(8)**。

  gquota/grpquota/gqnoenforce
	启用组磁盘配额记账并（可选地）强制限制。更多细节请参阅 **xfs_quota(8)**。

  pquota/prjquota/pqnoenforce
	启用项目磁盘配额记账并（可选地）强制限制。更多细节请参阅 **xfs_quota(8)**。

  sunit=value 与 swidth=value
	用于为 RAID 设备或条带卷指定条带单元与条带宽度。"value" 必须以 512 字节块为单位指定。这些选项仅与通过非零数据对齐参数创建的文件系统相关。

	指定的 `sunit` 和 `swidth` 参数必须与现有文件系统的对齐特性兼容。一般而言，这意味着对 `sunit` 唯一有效的更改是将其按 2 的幂倍数增大。有效的 `swidth` 值是任意有效 `sunit` 值的整数倍。

	通常，只有在底层 RAID 设备的几何结构被修改（例如向 RAID5 lun 添加新磁盘并重塑它）时才有必要使用这些挂载选项。

  swalloc
	当当前文件尾被扩展且文件大小大于条带宽度时，数据分配将向上取整到条带宽度的边界。

  wsync
	指定时，所有文件系统命名空间操作都将同步执行。这确保在命名空间操作（创建、取消链接等）完成时，对命名空间的更改已位于稳定存储上。这在 HA（高可用）部署中很有用，因为故障转移不得导致客户端在故障转移期间或之后看到不一致的命名空间呈现。

  errortag=tagname
	指定时，以默认频率启用名为 "tagname" 的错误注入标签。可以多次指定以启用多个 errortag。在重新挂载时指定此选项，如果该标签之前被设置为其他值，则会将其重置为默认值。
	此选项仅在启用了 CONFIG_XFS_DEBUG 时受支持，并且不会反映在 /proc/self/mounts 中。

## V4 格式的弃用

V4 文件系统格式缺乏 V5 格式所支持的某些特性，例如元数据校验和（checksumming）、增强的元数据验证，以及存储 2038 年之后时间戳的能力。因此，V4 格式已被弃用。所有用户都应通过备份文件、重新格式化并从备份恢复来进行升级。

管理员和用户可以通过对文件系统挂载点运行 xfs_info 并检查是否包含 "crc=" 字符串来检测 V4 文件系统。如果未找到此类字符串，请将 xfsprogs 升级到最新版本并重试。

弃用将分两部分进行。挂载 V4 文件系统的支持现在可以在内核构建时通过 Kconfig 选项禁用。这些选项已在 2025 年 9 月更改为默认关闭。在 2030 年 9 月，对该支持将从代码库中彻底移除。

注意：发行版维护者可能会选择早于上述日期撤回 V4 格式支持。

## 已弃用的挂载选项

============================    ================
  名称				移除时间表
============================    ================
挂载 V4 文件系统     	     September 2030
挂载 ascii-ci 文件系统    September 2030
============================    ================

## 已移除的挂载选项

===========================     =======
  名称				Removed
===========================	=======
  delaylog/nodelaylog		v4.0
  ihashsize			v4.0
  irixsgid			v4.0
  osyncisdsync/osyncisosync	v4.0
  barrier			v4.19
  nobarrier			v4.19
  ikeep/noikeep			v6.18
  attr2/noattr2			v6.18
===========================     =======

## sysctls

XFS 文件系统提供以下 sysctl：

  fs.xfs.stats_clear		(Min: 0  Default: 0  Max: 1)
	将此设置为 "1" 会清除 /proc/fs/xfs/stat 中累积的 XFS 统计信息。随后它会立即重置为 "0"。

  fs.xfs.xfssyncd_centisecs	(Min: 100  Default: 3000  Max: 720000)
	文件系统将元数据刷写到磁盘并运行内部缓存清理例程的时间间隔。

  fs.xfs.filestream_centisecs	(Min: 1  Default: 3000  Max: 360000)
	文件系统使 filestreams 缓存引用老化的时间间隔，并将超时的 AG 返回到空闲流池。

  fs.xfs.speculative_prealloc_lifetime
	(Units: seconds   Min: 1  Default: 300  Max: 86400)
	后台扫描带有未使用投机预分配的 inode 的运行间隔。扫描会从干净的 inode 中移除未使用的预分配，并将未使用的空间释放回空闲池。

  fs.xfs.error_level		(Min: 0  Default: 3  Max: 11)
	发生内部错误时用于错误报告的音量旋钮。例如，这将为文件系统关闭（shutdown）生成详细的消息与回溯。当前的阈值值为：

		XFS_ERRLEVEL_OFF:       0
		XFS_ERRLEVEL_LOW:       1
		XFS_ERRLEVEL_HIGH:      5

  fs.xfs.panic_mask		(Min: 0  Default: 0  Max: 511)
	导致某些错误条件调用 BUG()。值是一个位掩码；将代表应引发 panic 的错误的标签进行 OR 运算：

		XFS_NO_PTAG                     0
		XFS_PTAG_IFLUSH                 0x00000001
		XFS_PTAG_LOGRES                 0x00000002
		XFS_PTAG_AILDELETE              0x00000004
		XFS_PTAG_ERROR_REPORT           0x00000008
		XFS_PTAG_SHUTDOWN_CORRUPT       0x00000010
		XFS_PTAG_SHUTDOWN_IOERROR       0x00000020
		XFS_PTAG_SHUTDOWN_LOGERROR      0x00000040
		XFS_PTAG_FSBLOCK_ZERO           0x00000080
		XFS_PTAG_VERIFIER_ERROR         0x00000100

	此选项仅用于调试。

  fs.xfs.inherit_sync		(Min: 0  Default: 1  Max: 1)
	将此设置为 "1" 会使目录上由 **xfs_io(8)** chattr 命令设置的 "sync" 标志被该目录中的文件继承。

  fs.xfs.inherit_nodump		(Min: 0  Default: 1  Max: 1)
	将此设置为 "1" 会使目录上由 **xfs_io(8)** chattr 命令设置的 "nodump" 标志被该目录中的文件继承。

  fs.xfs.inherit_noatime	(Min: 0  Default: 1  Max: 1)
	将此设置为 "1" 会使目录上由 **xfs_io(8)** chattr 命令设置的 "noatime" 标志被该目录中的文件继承。

  fs.xfs.inherit_nosymlinks	(Min: 0  Default: 1  Max: 1)
	将此设置为 "1" 会使目录上由 **xfs_io(8)** chattr 命令设置的 "nosymlinks" 标志被该目录中的文件继承。

  fs.xfs.inherit_nodefrag	(Min: 0  Default: 1  Max: 1)
	将此设置为 "1" 会使目录上由 **xfs_io(8)** chattr 命令设置的 "nodefrag" 标志被该目录中的文件继承。

  fs.xfs.rotorstep		(Min: 1  Default: 1  Max: 256)
	在 "inode32" 分配模式下，此选项决定分配器在移动到下一个分配组之前，尝试在同一分配组中分配多少个文件。其意图是控制在为新文件分配 extent 时，分配器在分配组之间移动的速率。

## 已弃用的 Sysctls

当前无。

## 已移除的 Sysctls

==========================================   =======
  名称                                       Removed
==========================================   =======
  fs.xfs.xfsbufd_centisec                    v4.0
  fs.xfs.age_buffer_centisecs                v4.0
  fs.xfs.irix_symlink_mode                   v6.18
  fs.xfs.irix_sgid_inherit                   v6.18
  fs.xfs.speculative_cow_prealloc_lifetime   v6.18
==========================================   =======

## 错误处理

XFS 可以根据其操作期间发现的错误类型采取不同的行为。该实现向错误处理程序引入了以下概念：

 -failure speed（失败速度）：
	定义当在文件系统操作期间发现特定错误时，XFS 应多快将错误向上传播。它可以立即传播、在经过定义的重试次数后传播、在经过设定的时间段后传播，或者简单地永远重试。

 -error classes（错误类别）：
	指定错误配置将应用的子系统，例如元数据 IO 或内存分配。不同的子系统将有不同的错误处理程序，其行为可以配置。

 -error handlers（错误处理程序）：
	定义针对特定错误的行为。

文件系统在发生错误时的行为可以通过 `sysfs` 文件设置。每个错误处理程序独立工作——错误处理器针对特定类别遇到的第一个条件将导致错误被传播，而不是被重置并重试。

文件系统在传播错误时所采取的行动取决于上下文——对于不可恢复的错误，它可能导致系统关闭（shut down）；它可能回报给用户空间；或者由于对错误无能为力、也无人可报告（例如在卸载期间），它甚至可能被忽略。

配置文件按以下层级为每个已挂载的文件系统组织：

  /sys/fs/xfs/<dev>/error/<class>/<error>/

其中：
  <dev>
	已挂载文件系统的短设备名。这与 XFS 内核错误消息中显示的 "XFS(<dev>): ..." 设备名相同。

  <class>
	错误配置所属的子系统。自 4.9 起，已定义的类别为：

  - "metadata"：应用于元数据缓冲区写 IO

  <error>
	各个错误处理程序配置。

每个文件系统在其顶级目录中都定义了“全局”错误配置选项：

  /sys/fs/xfs/<dev>/error/

  fail_at_unmount		(Min:  0  Default:  1  Max: 1)
	定义在卸载时文件系统的错误行为。

	如果设置为值 1，XFS 将在卸载期间覆盖所有其他错误配置，并将其替换为“立即失败”特性。
	即不重试、无重试超时。这始终允许在存在持久错误时卸载成功。

	如果设置为 0，配置的重试行为将继续，直到所有重试和/或超时耗尽。在存在持久错误时，这将延迟卸载完成，并且在“永远重试”处理程序配置的情况下，它可能阻止文件系统完全卸载。

	注意：无法保证在卸载进行期间可以设置 fail_at_unmount。在“永远重试”错误处理程序配置导致卸载挂起之前，卸载中的文件系统可能已移除 `sysfs` 条目，因此必须在卸载开始之前适当配置文件系统，以防止卸载挂起。

每个文件系统都有特定的错误类别处理程序，用于定义特定错误的错误传播行为。还定义了一个“default”错误处理程序，用于定义所有未定义特定处理程序的错误的行为。当为单个错误配置了多个重试约束时，第一个到期的重试配置将导致错误被传播。处理程序配置位于目录：

  /sys/fs/xfs/<dev>/error/<class>/<error>/

  max_retries			(Min: -1  Default: Varies  Max: INTMAX)
	定义在文件系统传播错误之前允许的特定错误重试次数。给定错误上下文（例如特定的元数据缓冲区）的重试计数在每次操作成功完成时重置。

	将值设置为 "-1" 将导致 XFS 针对此特定错误永远重试。

	将值设置为 "0" 将导致 XFS 在报告特定错误时立即失败。

	将值设置为 "N"（其中 0 < N < Max）将使 XFS 在传播错误之前重试操作 "N" 次。

  retry_timeout_seconds		(Min:  -1  Default:  Varies  Max: 1 day)
	定义文件系统在发现特定错误时允许重试其操作的时间量（以秒为单位）。

	将值设置为 "-1" 将允许 XFS 针对此特定错误永远重试。

	将值设置为 "0" 将导致 XFS 在报告特定错误时立即失败。

	将值设置为 "N"（其中 0 < N < Max）将允许 XFS 在传播错误之前重试操作最多 "N" 秒。

**注意：** 特定错误处理程序的默认行为取决于类别和错误上下文。例如，"metadata/ENODEV" 的默认值是 "0" 而非 "-1"，因此该错误处理程序默认为“立即失败”行为。这样做是因为无论元数据 IO 重试多少次，ENODEV 都是一个致命的、不可恢复的错误。

## 工作队列并发

XFS 使用内核工作队列（workqueue）来并行化元数据更新过程。这使其能够利用可以同时服务多个 IO 操作的存储硬件。此接口暴露了 XFS 的内部实现细节，因此明确不属于内核可能向用户空间提供的任何用户空间 API/ABI 保证的一部分。这些是 XFS 用于并发的通用工作队列实现的未文档化特性，此处提供它们纯粹用于诊断和调优目的，并且可能在将来的任何时间发生变化。

文件系统工作队列的控制旋钮按手头任务和数据设备的短名称组织。它们都可以在以下位置找到：

  /sys/bus/workqueue/devices/${task}!${device}

================  ===========
  任务            描述
================  ===========
  xfs_iwalk-$pid  整个文件系统的 inode 扫描。目前仅限于挂载时的 quotacheck。
  xfs-gc          对超出 EOF 或用于写时复制（copy on write）暂存的、被投机分配
                  的磁盘空间进行后台垃圾回收。
================  ===========

例如，/dev/nvme0n1 的 quotacheck 工作队列旋钮可以在 /sys/bus/workqueue/devices/xfs_iwalk-1111!nvme0n1/ 中找到。

XFS 工作队列中值得关注的旋钮如下：

============     ===========
  旋钮           描述
============     ===========
  max_active     可以启动以运行工作的最大后台线程数。
  cpumask        允许线程运行的 CPU。
  nice           调度线程的相对优先级。这些与可以应用于用户空间进程的 nice 级别相同。
============     ===========

## 分区（Zoned）文件系统

对于 zoned 文件系统，以下属性暴露在：

  /sys/fs/xfs/<dev>/zoned/

  max_open_zones		(Min:  1  Default:  Varies  Max:  UINTMAX)
	此只读属性暴露可用于数据放置的最大打开区域数。该值在挂载时确定，并受底层 zoned 设备的能力、文件系统大小以及 max_open_zones 挂载选项的限制。

  nr_open_zones			(Min:  0  Default:  Varies  Max:  UINTMAX)
	此只读属性暴露文件系统当前使用的打开区域数。

  zonegc_low_space		(Min:  0  Default:  0  Max:  100)
	定义 GC 应保持可用于写入的未使用空间的百分比。较高的值将回收更多被未使用块占用的空间，从而针对写入突发创建更大的缓冲区，代价是写入放大增加。无论此值如何，垃圾回收将始终旨在释放最少数量的块，以保持 max_open_zones 个区域打开以用于数据放置目的。
