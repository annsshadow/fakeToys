## SD MMC 块设备属

这些属性为SD MMC 设备相关联的块设备定义
以下属性可读写
	========		===============================================
	force_ro		即使写保护开关关闭，也强制只读访问	========		===============================================

## SD MMC 设备属

所有属性均为只读
	======================	===============================================
	cid			卡识别寄存器（Card Identification Register	csd			卡特定数据寄存器（Card Specific Data Register	scr			SD 卡配置寄存器（仅 SD	date			制造日期（来自 CID 寄存器）
	fwrev			固件/产品修订版本（来CID 寄存器）
				（仅 SD MMCv1	hwrev			硬件/产品修订版本（来CID 寄存器）
				（仅 SD MMCv1	manfid			制造商 ID（来CID 寄存器）
	name			产品名称（来CID 寄存器）
	oemid			OEM/应用 ID（来CID 寄存器）
	prv			产品修订版本（来CID 寄存器）
				（仅 SD MMCv4	serial			产品序列号（来自 CID 寄存器）
	erase_size		擦除组大	preferred_erase_size	首选擦除大	raw_rpmb_size_mult	RPMB 分区大小
	rel_sectors		可靠写入扇区计数
	ocr 			Operation Conditions Register（工作条件寄存器	dsr			Driver Stage Register（驱动级寄存器）
	cmdq_en			命令队列已启用：

					1 => 已启用，0 => 未启	======================	===============================================

关于 Erase Size Preferred Erase Size 的说明：

	“erase_size是擦除操作的最小字节数。对MMC，“erase_size	卡报告的擦除组大小。注“erase_size不适用trim 或安trim
	操作，后者的最小大小始终为一512 字节扇区。对SD，若卡为块寻址	“erase_size512，否则为 0
	SD/MMC 卡可以擦除任意大的区域，直至并包括整张卡。擦除大区域时，出于
	以下三个原因，可能希望将其分成更小的块进行：

      1. 单条擦除命令会使卡上的所有其I/O 等待。若擦除整张卡这不成问题		但擦除一个分区会使同一卡上另一个分区的 I/O 在擦除持续期间等待—		这可能长达数分钟      2. 能够向用户报告擦除进度      3. 擦除超时时间变得过大而失去实用价值。因为擦除超时包含一个余量，		余量乘以擦除区域的大小，对于大区域该值最终可能达到数分钟
	“erase_size并非最有效的擦除单位（尤其对于 SD，它只是一个扇区）	因此 “preferred_erase_size为擦除大区域提供了一个良好的块大小
	对于 MMC，“preferred_erase_size是卡指定的高容量擦除大小（若指定），
	否则基于卡的容量
	对于 SD，“preferred_erase_size是卡指定的分配单元大小
	“preferred_erase_size以字节为单位
关于 raw_rpmb_size_mult 的说明：

	“raw_rpmb_size_mult128kB 块的倍数
	RPMB 大小（字节）通过以下等式计算
		RPMB 分区大小 = 128kB x raw_rpmb_size_mult
