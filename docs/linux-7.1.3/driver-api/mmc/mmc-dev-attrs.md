## SD 与 MMC 块设备属性


这些属性为与 SD 或 MMC 设备相关联的块设备定义。

以下属性可读写。

	========		===============================================
	force_ro		即使写保护开关关闭，也强制只读访问。
	========		===============================================

## SD 与 MMC 设备属性


所有属性均为只读。

	======================	===============================================
	cid			卡识别寄存器（Card Identification Register）
	csd			卡特定数据寄存器（Card Specific Data Register）
	scr			SD 卡配置寄存器（仅 SD）
	date			制造日期（来自 CID 寄存器）
	fwrev			固件/产品修订版本（来自 CID 寄存器）
				（仅 SD 和 MMCv1）
	hwrev			硬件/产品修订版本（来自 CID 寄存器）
				（仅 SD 和 MMCv1）
	manfid			制造商 ID（来自 CID 寄存器）
	name			产品名称（来自 CID 寄存器）
	oemid			OEM/应用 ID（来自 CID 寄存器）
	prv			产品修订版本（来自 CID 寄存器）
				（仅 SD 和 MMCv4）
	serial			产品序列号（来自 CID 寄存器）
	erase_size		擦除组大小
	preferred_erase_size	首选擦除大小
	raw_rpmb_size_mult	RPMB 分区大小
	rel_sectors		可靠写入扇区计数
	ocr 			Operation Conditions Register（工作条件寄存器）
	dsr			Driver Stage Register（驱动级寄存器）
	cmdq_en			命令队列已启用：

					1 => 已启用，0 => 未启用
	======================	===============================================

关于 Erase Size 与 Preferred Erase Size 的说明：

	“erase_size” 是擦除操作的最小字节数。对于 MMC，“erase_size” 是
	卡报告的擦除组大小。注意 “erase_size” 不适用于 trim 或安全 trim
	操作，后者的最小大小始终为一个 512 字节扇区。对于 SD，若卡为块寻址，
	“erase_size” 为 512，否则为 0。

	SD/MMC 卡可以擦除任意大的区域，直至并包括整张卡。擦除大区域时，出于
	以下三个原因，可能希望将其分成更小的块进行：

      1. 单条擦除命令会使卡上的所有其他 I/O 等待。若擦除整张卡这不成问题，
		但擦除一个分区会使同一卡上另一个分区的 I/O 在擦除持续期间等待——
		这可能长达数分钟。
      2. 能够向用户报告擦除进度。
      3. 擦除超时时间变得过大而失去实用价值。因为擦除超时包含一个余量，该
		余量乘以擦除区域的大小，对于大区域该值最终可能达到数分钟。

	“erase_size” 并非最有效的擦除单位（尤其对于 SD，它只是一个扇区），
	因此 “preferred_erase_size” 为擦除大区域提供了一个良好的块大小。

	对于 MMC，“preferred_erase_size” 是卡指定的高容量擦除大小（若指定），
	否则基于卡的容量。

	对于 SD，“preferred_erase_size” 是卡指定的分配单元大小。

	“preferred_erase_size” 以字节为单位。

关于 raw_rpmb_size_mult 的说明：

	“raw_rpmb_size_mult” 是 128kB 块的倍数。

	RPMB 大小（字节）通过以下等式计算：

		RPMB 分区大小 = 128kB x raw_rpmb_size_mult
