## 用户态软件挂起接口文

	(C) 2006 Rafael J. Wysocki <rjw@sisk.pl>

首先，swsusp.txt 开头的警告仍然适用
其次，如果你还没有这样做，你现在应该阅读 swsusp.txt 中的 FAQ
现在，要使用用户态接口进行软件挂起，你需要专门的工具来从内核写系内存快照。此类工具可以在 <http://suspend.sourceforge.net> 等处获取。如果你
打算开发自己的挂起/恢复工具，你不妨看一下它们
该接口由一个字符设备组成，提供 open()、release()、read() write() 操作以及若干include/linux/suspend_ioctls.h 中定义的 ioctl() 命令。该设备主设备号和次设备号分别为 10 231，可以从 /sys/class/misc/snapshot/dev
读取
该设备可以被打开用于读取或用于写入。如果为读取而打开，则被视为处于挂模式。否则假设处于恢复模式。该设备不能同时为读写而打开。并且同一时刻也不
可能将该设备打开多次
即使打开设备也有副作用。数据结构会被分配，并且 PM_HIBERNATION_PREPARE /
PM_RESTORE_PREPARE 链会被调用
该设备能识别ioctl() 命令有：

SNAPSHOT_FREEZE
	冻结用户空间进程（当前进程不被冻结）；这是让
	SNAPSHOT_CREATE_IMAGE SNAPSHOT_ATOMIC_RESTORE 成功所必需
SNAPSHOT_UNFREEZE
	解冻SNAPSHOT_FREEZE 冻结的用户空间进
SNAPSHOT_CREATE_IMAGE
	创建系统内存的快照；ioctl() 的最后一个参数应该是指向一	int 变量的指针，其值将指示调用是在创建快照后返回（1）还是在
	从中恢复系统内存状态后返回）（恢复后系统会发现自己再次完成
	SNAPSHOT_CREATE_IMAGE ioctl()）；在快照创建之后，可以使用 read()
	鎿嶄綔灏嗗揩鐓т紶鍑哄唴鏍。
SNAPSHOT_ATOMIC_RESTORE
	从上传的快照映像恢复系统内存状态；在调用它之前，你应该使用
	write() 操作将系统内存快照传回内核；如果内核无法获得快照映像	该调用将不会成功

SNAPSHOT_FREE
	释放为快照映像分配的内存

SNAPSHOT_PREF_IMAGE_SIZE
	设置映像的首选最大尺寸（内核会尽力确保映像尺寸不超过此数值，
	但如果无法实现，内核将创建尽可能小的映像
SNAPSHOT_GET_IMAGE_SIZE
	返回休眠映像的实际大小（最后一个参数应该是指向一loff_t
	变量的指针，若调用成功将包含结果
SNAPSHOT_AVAIL_SWAP_SIZE
	返回可用交换空间的大小（字节）（最后一个参数应该是指向一	loff_t 变量的指针，若调用成功将包含结果
SNAPSHOT_ALLOC_SWAP_PAGE
	从恢复分区分配一个交换页面（最后一个参数应该是指向一loff_t
	变量的指针，若调用成功将包含交换页面偏移量）

SNAPSHOT_FREE_SWAP_PAGES
	释放所有由 SNAPSHOT_ALLOC_SWAP_PAGE 分配的交换页
SNAPSHOT_SET_SWAP_AREA
	设置恢复分区以及交换头部所在位置距分区开头的偏移量（	<PAGE_SIZE> 为单位）（最后一ioctl() 参数应指向一	resume_swap_area 结构，如 kernel/power/suspend_ioctls.h 中所定义	包含恢复设备规范和偏移量）；对于交换分区，偏移量始终0，但对于
	交换文件则不为零（详Documentation/power/swsusp-and-swap-files.rst
SNAPSHOT_PLATFORM_SUPPORT
	根据参数值启禁用休眠平台支持（如果参数非零则启用
SNAPSHOT_POWER_OFF
	使内核使用平台（例如 ACPI）驱动将系统转换到休眠状	（例ACPI S4
SNAPSHOT_S2RAM
	挂起RAM；使用此调用会使内核立即进入挂起RAM 状态，因此	调用之前必须总是先调SNAPSHOT_FREEZE，并且在系统唤醒之后也需	使用 SNAPSHOT_UNFREEZE 调用。此调用是实suspend-to-both（两者皆挂起	机制所必需的：在该机制中，首先创建挂起映像（就好像系统已被挂起	磁盘），然后将系统挂起到 RAM（这样如果有足够的电池电量，就可以从 RAM
	恢复系统，否则就基于保存的挂起映像恢复其状态）

该设备的 read() 操作可用于将快照映像从内核传出。它具有以下限制
- 一read() 不能读取超过一个虚拟内存页
- 跨页边界read() 是不可能的（即，如果你在上一次调用中读取1/2 页，那么
  在下一次调用中你将**最*只能读取 1/2 页）

该设备的 write() 操作用于将系统内存快照上传到内核。它具有read() 操作相同限制
release() 操作释放为快照映像分配的所有内存，以及SNAPSHOT_ALLOC_SWAP_PAGE
分配的所有交换页面（如果有）。因此，在关闭设备之前没有必要使SNAPSHOT_FREE
SNAPSHOT_FREE_SWAP_PAGES（实际上，如果在设备关闭时仍有被冻结的用户空间进程，
它也会解冻它们）
目前假设，从内核写快照映像的用户态工具会使用称为恢复分区的交换分区，或使用交换文件作为存储空间（如果使用交换文件，则恢复分区是持有该文件的分区）然而，这并非真正必需，因为它们也可以使用，例如，一个特殊的（空白的）挂起分区，
或在 SNAPSHOT_CREATE_IMAGE 之前卸载、之后挂载的分区上的文件
这些工具**绝不*对快照映像内数据的顺序做任何假设。映像的内容完全由内核所有，
其结构在未来的内核版本中可能会改变
快照映像**必须**原封不动地写入内核（即，所有映像数据、元数据和头部都必须以与
读取*完全相同**的数量、形式和顺序写入）。否则，被恢复系统的行为可能完全
无法预测
在执SNAPSHOT_ATOMIC_RESTORE 时，内核会检查快照映像的结构是否与映像头部中
存储的信息一致。如果检测到任何不一致，SNAPSHOT_ATOMIC_RESTORE 将不会成功。不过，
这并非万无一失的机制，使用此接口的用户态工*应当**使用额外的手段（如校验和来确保快照映像的完整性
挂起和恢复工*必须**在调SNAPSHOT_FREEZE 之前将自己锁定在内存中，最好使mlockall()
挂起工具**必须**检SNAPSHOT_CREATE_IMAGE 存储ioctl() 最后一个参数所指向
内存位置的值，并据此继续：

1. 如果该值为 1（即，系统内存快照刚刚被创建，系统已准备好保存它）：

	(a)	挂起工具**不得**关闭快照设备*除非**整个挂起过程要被取消		在这种情况下，如果快照映像已被保存，挂起工具**应当**销毁它		最好是清除其头部。如果挂起不被取消，则在快照映像保存之后，系		**必须**被断电或重启	(b)	挂起工具**不应**尝试对调SNAPSHOT_CREATE_IMAGE 之前已挂载的
		文件系统执行任何文件系统操作（包括读取）。不过，它可以挂载当		未挂载的文件系统并对其执行一些操作（例如，用它来保存映像）
2. 如果该值为 0（即，系统状态刚刚从快照映像恢复），挂起工具**必须**关闭快照
	设备。之后它将被视为一个常规的用户态进程，因此无需退出
恢复工具**不应**尝试挂载任何可在挂起前挂载的文件系统，也**不应**尝试执行涉及
此类文件系统的任何操作
更多细节，请参阅源代码