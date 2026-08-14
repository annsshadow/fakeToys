## dm-raid

device-mapper 的 RAID（dm-raid）目标提供了从 DM 到 MD 的桥梁。它允许使用
device-mapper 接口来访问 MD RAID 驱动。

### 映射表接口

```
  <raid_type> <#raid_params> <raid_params> \
    <#raid_devs> <metadata_dev0> <dev0> [.. <metadata_devN> <devN>]
```

<raid_type>：

  ============= ===============================================================
  raid0		RAID0 条带化（无弹性）
  raid1		RAID1 镜像
  raid4		RAID4，具有专用的最后一块奇偶校验盘
  raid5_n 	RAID5，具有专用的最后一块奇偶校验盘，支持与 raid1 之间的接管
		（takeover）
		与 raid4 相同

  - 用于与 raid1 之间接管的过渡性布局
  raid5_la	RAID5 左侧非对称

  - 数据延续的轮转奇偶校验 0
  raid5_ra	RAID5 右侧非对称

  - 数据延续的轮转奇偶校验 N
  raid5_ls	RAID5 左侧对称

  - 数据重启的轮转奇偶校验 0
  raid5_rs 	RAID5 右侧对称

  - 数据重启的轮转奇偶校验 N
  raid6_zr	RAID6 零重启

  - 数据重启的轮转奇偶校验零（从左到右）
  raid6_nr	RAID6 N 重启

  - 数据重启的轮转奇偶校验 N（从右到左）
  raid6_nc	RAID6 N 延续

  - 数据延续的轮转奇偶校验 N（从右到左）
  raid6_n_6	RAID6，具有专用奇偶校验盘

  - 奇偶校验和 Q 校验（Q-syndrome）位于最后 2 块盘上；
		  用于与 raid0/raid4/raid5_n 之间接管的布局
  raid6_la_6	同 "raid_la" 加上专用的最后一块 Q-syndrome 盘，支持与 raid5 之间的接管

  - 用于 raid5_la 与 raid6 之间接管的布局
  raid6_ra_6	同 "raid5_ra" 专用的最后一块 Q-syndrome 盘

  - 用于 raid5_ra 与 raid6 之间接管的布局
  raid6_ls_6	同 "raid5_ls" 专用的最后一块 Q-syndrome 盘

  - 用于 raid5_ls 与 raid6 之间接管的布局
  raid6_rs_6	同 "raid5_rs" 专用的最后一块 Q-syndrome 盘

  - 用于 raid5_rs 与 raid6 之间接管的布局
  raid10        由附加参数选择的各种受 RAID10 启发的算法
		（参见下面的 raid10_format 和 raid10_copies）

  - RAID10：条带化镜像（即"在镜像之上的条带化"）
  - RAID1E：集成相邻条带镜像
  - RAID1E：集成偏移条带镜像
  - 以及其他类似的 RAID10 变体
  ============= ===============================================================

  参考：第 4 章
  https://www.snia.org/sites/default/files/SNIA_DDF_Technical_Position_v2.0.pdf

<#raid_params>：其后跟随的参数数量。

<raid_params> 由以下内容组成：

    必选参数：
        <chunk_size>：
		      以扇区为单位的块大小（chunk size）。该参数通常被称为
		      "stripe size"（条带大小）。它是唯一的必选参数，并
		      放在首位。

    后跟可选参数（任意顺序）：
	[sync|nosync]
		强制或阻止 RAID 初始化。

	[rebuild <idx>]
		重建编号为 'idx' 的驱动器（第一个驱动器为 0）。

	[daemon_sleep <ms>]
		bitmap 守护进程两次运行之间的间隔，用于清除位（bit）。间隔
		越长意味着 bitmap 的 I/O 越少，但故障后的重新同步（resync）
		可能耗时更长。

	[min_recovery_rate <kB/sec/disk>]
		节流 RAID 初始化
	[max_recovery_rate <kB/sec/disk>]
		节流 RAID 初始化
	[write_mostly <idx>]
		将索引为 'idx' 的驱动器标记为 write-mostly。
	[max_write_behind <sectors>]
		参见 '--write-behind='（man mdadm）
	[stripe_cache <sectors>]
		条带缓存大小（仅 RAID 4/5/6）
	[region_size <sectors>]
		region_size 乘以区域（region）数量即为阵列的逻辑大小。
		bitmap 记录每个区域的设备同步状态。

        [raid10_copies   <# copies>], [raid10_format   <near|far|offset>]
		这两个选项用于更改 RAID10 配置的默认布局。可以指定副本数，
		但默认为 2。副本的放置方式还有三种变体——默认为 "near"。
		Near 副本是大多数人想到镜像时所指的。如果未指定这些选项，
		或者给出了 'raid10_copies 2' 和/或 'raid10_format near'，
		那么 2、3 和 4 个设备的布局为：

		========	 ==========	   ==============
		2 drives         3 drives          4 drives
		========	 ==========	   ==============
		A1  A1           A1  A1  A2        A1  A1  A2  A2
		A2  A2           A2  A3  A3        A3  A3  A4  A4
		A3  A3           A4  A4  A5        A5  A5  A6  A6
		A4  A4           A5  A6  A6        A7  A7  A8  A8
		..  ..           ..  ..  ..        ..  ..  ..  ..
		========	 ==========	   ==============

		2 设备布局等价于 2 路 RAID1。4 设备布局即传统 RAID10 的样子。
		3 设备布局即所谓的 'RAID1E - 集成相邻条带镜像'。

		如果为 'raid10_copies 2' 和 'raid10_format far'，那么 2、3 和
		4 个设备的布局为：

		========	     ============	  ===================
		2 drives             3 drives             4 drives
		========	     ============	  ===================
		A1  A2               A1   A2   A3         A1   A2   A3   A4
		A3  A4               A4   A5   A6         A5   A6   A7   A8
		A5  A6               A7   A8   A9         A9   A10  A11  A12
		..  ..               ..   ..   ..         ..   ..   ..   ..
		A2  A1               A3   A1   A2         A2   A1   A4   A3
		A4  A3               A6   A4   A5         A6   A5   A8   A7
		A6  A5               A9   A7   A8         A10  A9   A12  A11
		..  ..               ..   ..   ..         ..   ..   ..   ..
		========	     ============	  ===================

		如果为 'raid10_copies 2' 和 'raid10_format offset'，那么 2、3
		和 4 个设备的布局为：

		========       ==========         ================
		2 drives       3 drives           4 drives
		========       ==========         ================
		A1  A2         A1  A2  A3         A1  A2  A3  A4
		A2  A1         A3  A1  A2         A2  A1  A4  A3
		A3  A4         A4  A5  A6         A5  A6  A7  A8
		A4  A3         A6  A4  A5         A6  A5  A8  A7
		A5  A6         A7  A8  A9         A9  A10 A11 A12
		A6  A5         A9  A7  A8         A10 A9  A12 A11
		..  ..         ..  ..  ..         ..  ..  ..  ..
		========       ==========         ================

		这里我们看到与 'RAID1E - 集成偏移条带镜像' 非常相似的布局。

        [delta_disks <N>]
		delta_disks 选项的值（-251 < N < +251）会触发设备移除（负值）或
		设备添加（正值），作用于任何支持 reshape 的 raid 级别 4/5/6 和
		10。RAID 级别 4/5/6 允许添加和移除设备（元数据与数据设备元组），
		raid10_near 和 raid10_offset 仅允许添加设备。raid10_far 根本
		不支持任何 reshape。
		必须保留最小数量的设备以保证弹性，即对 raid4/5 为 3 个设备，对
		raid6 为 4 个设备。

        [data_offset <sectors>]
		该选项值定义了每个数据设备中数据开始处的偏移量。它用于提供
		原地之外（out-of-place）的 reshape 空间，以避免在改变条带布局时
		覆盖数据，因此任何时刻发生中断/崩溃都不会有丢失数据的风险。
		例如，在前向 reshape 期间向现有 raid 集合添加设备时，原地之外的
		空间将在每个 raid 设备的开头分配。支持此类设备添加的 kernel
		raid4/5/6/10 的 MD personality 将从现有的前几个条带（那些条带
		数较小的）开始，从 data_offset 读入数据，以填充具有更大条带数
		的新条带，计算冗余块（CRC/Q-syndrome）并将该新条带写入偏移 0。
		同样的方式应用于所有其他 N-1 个新条带。此原地之外方案也用于
		更改 RAID 类型（即分配算法），例如从 raid5_ls 更改为 raid5_n。

	[journal_dev <dev>]
		该选项向 raid4/5/6 raid 集合添加一个日志设备，并用它来封堵因对
		组成设备的非原子更新所造成的"写洞"（write hole），后者会在恢复
		期间导致数据丢失。日志设备被用作 writethrough（直写），因此与
		非日志的 raid4/5/6 集合相比，写入会受到节流。
		带有 raid4/5/6 日志设备时不可能进行接管/reshape；在请求这些操作
		之前必须先将其取消配置。

	[journal_mode <mode>]
		该选项将日志化的 raid4/5/6 raid 集合（见上面的 'journal_dev
		<dev>'）上的缓存模式设置为 'writethrough' 或 'writeback'。
		如果选择 'writeback'，日志设备必须具有弹性，且自身不得受"写洞"
		问题影响（例如使用 raid1 或 raid10），以避免单点故障。

<#raid_devs>：组成阵列的设备数量。
	每个设备由两个条目组成。第一个是包含元数据（如果有）的设备；第二个是
	包含数据的设备。目标版本 1.8.0 之前最多支持 64 个元数据/数据设备条目。
	1.9.0 支持最多 253 个，这由所使用的 MD 内核运行时强制限制。

	如果某个驱动器在创建时失败或缺失，可以在给定位置为元数据盘和数据盘
	都给出 '-'。

### 示例映射表

```
  # RAID4 - 4 个数据盘，1 个奇偶校验盘（无元数据设备）
  # 未指定用于保存超级块/bitmap 信息的元数据设备
  # 块大小为 1MiB
  # （为便于阅读而分行）

  0 1960893648 raid \
          raid4 1 2048 \
          5 - 8:17 - 8:33 - 8:49 - 8:65 - 8:81

  # RAID4 - 4 个数据盘，1 个奇偶校验盘（带元数据设备）
  # 块大小为 1MiB，强制 RAID 初始化，
  #       最小恢复速率为 20 kiB/sec/disk

  0 1960893648 raid \
          raid4 4 2048 sync min_recovery_rate 20 \
          5 8:17 8:18 8:33 8:34 8:49 8:50 8:65 8:66 8:81 8:82
```

### 状态输出

'dmsetup table' 显示用于构造映射的表。可选参数总是以上面列出的顺序打印，"sync"
或 "nosync" 总是先于其他参数输出，无论最初加载该表时使用的顺序如何。可以重复的
参数按值排序。

'dmsetup status' 产生关于阵列状态和健康状况的信息。输出如下（通常为单行，但在此
展开以便：

```
  1: <s> <l> raid \
  2:      <raid_type> <#devices> <health_chars> \
  3:      <sync_ratio> <sync_action> <mismatch_cnt>
```

第 1 行是 device-mapper 产生的标准输出。

```
        0 1960893648 raid raid4 5 AAAAA 2/490221568 init 0
```

这里我们可以看到 RAID 类型是 raid4，有 5 个设备——全部 'A'live（存活），并且该
阵列完成了其初始恢复的 2/490221568。下面是对各个字段更完整的描述：

	=============== =========================================================
	<raid_type>     与用于创建阵列的 <raid_type> 相同。
	<health_chars>  每个设备一个字符，表示：

   - 'A' = 存活且同步（in-sync）
   - 'a' = 存活但未同步
   - 'D' = 死亡/失败。
	<sync_ratio>    表示阵列经历了 'sync_action' 所描述过程的多少比例的
			比率。如果 'sync_action' 是 "check" 或 "repair"，那么
			"resync" 或 "recover" 过程可视为完成。
	<sync_action>   以下可能状态之一：

			idle
    - 没有正在执行的同步动作。
			frozen
    - 当前动作已被冻结。
			resync
    - 阵列正在进行其初始同步
				  或在非干净关闭后重新同步
				  （可能借助 bitmap）。
			recover
    - 阵列中的一个设备正在被重建或
				  替换。
			check
    - 正在执行用户发起的阵列全面检查。
				  读取并检查所有块的一致性。发现的
				  不一致数量记录在 <mismatch_cnt> 中。
				  此动作不会对阵列做任何改动。
			repair
    - 与 "check" 相同，但会纠正不一致。
			reshape
    - 阵列正在进行 reshape。
	<mismatch_cnt>  在 RAID1/10 的镜像副本之间发现的不一致数量，或在
			RAID4/5/6 中发现的错误奇偶校验值数量。该值仅在
			对阵列执行了 "check" 之后才有效。健康的阵列其
			'mismatch_cnt' 为 0。
	<data_offset>   到 raid 集合每个组成设备上用户数据起始处的当前数据偏移量
			（参见相应的 raid 参数以支持原地之外 reshape）。
	<journal_char>	- 'A' - 活动的直写（write-through）日志设备。
   - 'a' - 活动的回写（write-back）日志设备。
   - 'D' - 死亡日志设备。
   - '-' - 无日志设备。
	=============== =========================================================

### 消息接口

dm-raid 目标将通过 'message' 接口接受某些动作。（关于消息接口的更多信息参见 'man
dmsetup'。）这些动作包括：

	========= ================================================
	"idle"    暂停当前的同步动作。
	"frozen"  冻结当前的同步动作。
	"resync"  启动/继续一次 resync。
	"recover" 启动/继续一次 recover 过程。
	"check"   启动一次阵列检查（即"擦洗"，scrub）。
	"repair"  启动一次阵列修复。
	========= ================================================

### Discard 支持

各硬件厂商对 discard 支持的实现各不相同。当一个块被丢弃时，某些存储设备在读该块
时会返回零。这些设备设置了 'discard_zeroes_data' 属性。其他设备会返回随机数据。
令人困惑的是，一些宣传 'discard_zeroes_data' 的设备在被读取丢弃块时并不会可靠地
返回零！由于 RAID 4/5/6 使用来自多个设备的块来计算奇偶校验块，并且（出于性能原因）
依赖于 'discard_zeroes_data' 的可靠性，设备保持一致很重要。块可能在 RAID 4/5/6
条带的中间被丢弃，如果随后的读取结果不一致，奇偶校验块可能随时被不同地计算；使得
奇偶校验块对冗余毫无用处。如果你打算在 RAID 4/5/6 上启用 discard，理解你的硬件在
discard 时的行为很重要。

由于存储设备在这方面的行为不可靠，即使报告了 'discard_zeroes_data'，默认情况下
RAID 4/5/6 的 discard 支持是禁用的——这以牺牲一些性能为代价来确保数据完整性。

正确支持 'discard_zeroes_data' 的存储设备越来越多地被内核加入白名单，因此可以
信任。

对于受信任的设备，可以设置以下 dm-raid 模块参数来安全地为 RAID 4/5/6 启用 discard
支持：

    'devices_handle_discards_safely'

### 接管/Reshape 支持

该目标原生支持以下两类 MDRAID 转换：

o 接管（Takeover）：将阵列从一种 RAID 级别转换为另一种

o Reshape：在保持当前 RAID 级别的同时更改内部布局

每个操作仅在现有阵列布局和配置所施加的特定约束下才有效。

接管（Takeover）：
linear -> raid1，N >= 2 个镜像
raid0 -> raid4（添加专用奇偶校验设备）
raid0 -> raid5（添加专用奇偶校验设备）
raid0 -> raid10，采用 near 布局且 N >= 2 个镜像组（raid0 条带必须成为镜像组中的
第一个成员）
raid1 -> linear
raid1 -> raid5，2 个镜像
raid4 -> raid5，带轮转奇偶校验
带专用奇偶校验设备的 raid5 -> raid4
raid5 -> raid6（带专用 Q-syndrome）
带专用 Q-syndrome 的 raid6 -> raid5
带 near 布局且磁盘数为偶数的 raid10 -> raid0（从每个镜像组中选择任意同步设备）

Reshape：
linear：不可能
raid0：不可能
raid1：更改镜像数量
raid4：添加和移除条带（最少 3），更改条带大小（stripesize）
raid5：添加和移除条带（最少 3，raid1 接管的特殊情况为 2），更改轮转奇偶校验算法，
更改条带大小
raid6：添加和移除条带（最少 4），更改轮转校验算法，更改条带大小
raid10 near：添加条带（最少 4），更改条带大小，无法移除条带，更改为 offset 布局
raid10 offset：添加条带，更改条带大小，无法移除条带，更改为 near 布局
raid10 far：不可能

表行示例：

### raid1 -> raid5

#
# raid1 中 2 个设备的限制。
# raid5 personality 能够像 raid1 一样直接映射 2 个设备。
# 接管（takeover）之后进行 reshape 以更改为完整的 raid5 布局

  0 1960886272 raid raid1 3 0 region_size 2048 2 /dev/dm-0 /dev/dm-1 /dev/dm-2 /dev/dm-3

# dm-0 和 dm-2 例如是 4MiB 大小的元数据设备，dm-1 和 dm-3 必须至少为 1960886272 大。
#
# 接管为 raid5 的表行

  0 1960886272 raid raid5 3 0 region_size 2048 2 /dev/dm-0 /dev/dm-1 /dev/dm-2 /dev/dm-3

# 在给定的 2 个数据设备的开头添加所需的原地之外 reshape 空间，
# 为奇偶校验空间再分配一个相同大小的元数据/数据设备元组，
# 并将元数据设备的头 4K 清零。
#
# 例如 dm-1 的一个数据设备的原地之外 reshape 空间添加的示例表

  0 8192 linear 8:0 0 1960903888 #  <- 必须是空闲空间段
  8192 1960886272 linear 8:0 0 2048 # 之前的数据段

# 例如导致 raid 设备大小在 reshape 完成时翻倍fold的 raid5_rs reshape 映射表。
# 检查状态输出（例如 "dmsetup status $RaidDev"）以了解进度。

  0 $((2 * 1960886272)) raid raid5 7 0 region_size 2048 data_offset 8192 delta_disk 1 2 /dev/dm-0 /dev/dm-1 /dev/dm-2 /dev/dm-3

### 版本历史

```
 1.0.0	初始版本。支持 RAID 4/5/6
 1.1.0	增加了对 RAID 1 的支持
 1.2.0	处理包含失败设备的阵列的创建
 1.3.0	增加了对 RAID 10 的支持
 1.3.1	允许 RAID 10 的设备替换/重建
 1.3.2	修复/改进 RAID10 的冗余检查
 1.4.0	非功能性改动。从映射函数中移除参数。
 1.4.1	RAID10 修复冗余验证检查（commit 55ebbb5）。
 1.4.2	添加 RAID10 "far" 和 "offset" 算法支持。
 1.5.0	添加消息接口以允许操作 sync_action。
	新的状态（STATUSTYPE_INFO）字段：sync_action 和 mismatch_cnt。
 1.5.1	添加在 resume 时恢复瞬时失败设备的能力。
 1.5.2	除非 [last_]sync_action 为 "check"，否则 'mismatch_cnt' 为零。
 1.6.0	添加 discard 支持（以及 devices_handle_discard_safely 模块参数）。
 1.7.0	添加对 MD RAID0 映射的支持。
 1.8.0	显式检查超级块元数据中的兼容标志，如果任何标志由更新的
	目标版本设置则拒绝启动 raid 集合，从而避免对正在进行 reshape 的
	raid 集合造成数据损坏。
 1.9.0	添加对 RAID 级别接管/reshape/region size 以及集合大小缩减的支持。
 1.9.1	修复现有 RAID 4/10 映射设备的激活
 1.9.2	在构造函数无法读取超级块的情况下，不在状态表行上输出 '- -'。正确输出
	'maj:min1 maj:min2' 和状态行上的 'D'。如果 '- -' 传入构造函数，则在
	表行上输出 '- -'，在状态行健康字符上输出 '-'。
 1.10.0	添加对 raid4/5/6 日志设备的支持
 1.10.1	修复 reshape 请求上的数据损坏
 1.11.0	修复表行参数顺序（错误的 raid10_copies/raid10_format 顺序）
 1.11.1	通过 journal_mode 选项添加 raid4/5/6 日志回写支持
 1.12.1	修复 mddev_suspend() 与 md_write_start() 之间的 MD 死锁
 1.13.0	修复 "recover" 结束时 dev_health 状态（之前是 'a'，现在是 'A'）
 1.13.1	修复由过早的 md_stop_writes() 引起的死锁。同时修复大小和状态竞争。
 1.13.2	修复 raid 冗余验证，并避免使 raid 集合保持冻结
 1.14.0	修复小设备上的 reshape 竞争。修复添加条带 reshape 的死锁/潜在数据
	损坏。当通过 rebuild 请求特定设备时更新超级块。修复 RAID 分支
	重建错误。
 1.15.0 修复在新分配的 MD bitmap 页情况下大小扩展未被同步的问题；
        同时修复在前一次缩减之后未发生的那些扩展
 1.15.1 修复状态行上 rebuild/write_mostly/journal_(dev|mode) 的参数计数和参数

```
