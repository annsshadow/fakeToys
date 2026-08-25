## NetWinder 专用文档

本页ARM NetWinder 平台专用的硬件参考文档，列出I/O 端口与中断资源的分配情况，供该平台的内核移植与驱动开发者查阅


NetWinder 是一台小型低功耗计算机，主要设计用于运Linux。它基于 StrongARM RISC 处理器、DC21285 PCI 桥，并围绕其连接PC 类型的硬件
## 端口使用情况


=======  ====== ===============================
最     最描述
=======  ====== ===============================
0x0000   0x000f	DMA1
0x0020   0x0021	PIC1
0x0060   0x006f	键盘
0x0070   0x007f	RTC
0x0080   0x0087	DMA1
0x0088   0x008f	DMA2
0x00a0   0x00a3	PIC2
0x00c0   0x00df	DMA2
0x0180   0x0187	IRDA
0x01f0   0x01f6	ide0
0x0201		游戏端口
0x0203		RWA010 配置0x0220   	SoundBlaster
0x0250   ?	WaveArtist
0x0279		RWA010 配置索引
0x02f8   0x02ff	串口 ttyS1
0x0300   0x031f	Ether10
0x0338		GPIO1
0x033a		GPIO2
0x0370   0x0371	W83977F 閰嶇疆瀵勫瓨鍣?0x0388   ?	AdLib
0x03c0   0x03df	VGA
0x03f6		ide0
0x03f8   0x03ff	串口 ttyS0
0x0400   0x0408	DC21143
0x0480   0x0487	DMA1
0x0488   0x048f	DMA2
0x0a79		RWA010 閰嶇疆鍐?0xe800   0xe80f	ide0/ide1 BM DMA
=======  ====== ===============================


## 中断使用情况


======= ======= ========================
IRQ	类型	描述
======= ======= ========================
 0	ISA	100Hz 定时 1	ISA	键盘
 2	ISA	级联
 3	ISA	串口 ttyS1
 4	ISA	串口 ttyS0
 5	ISA	PS/2 鼠标
 6	ISA	IRDA
 7	ISA	鎵撳嵃鏈? 8	ISA	RTC 闂归挓
 9	ISA
10	ISA	GP10（橙色复位按钮）
11	ISA
12	ISA	WaveArtist
13	ISA
14	ISA	hda1
15	ISA
======= ======= ========================

## DMA 使用情况


======= ======= ===========
DMA	类型	描述
======= ======= ===========
 0	ISA	IRDA
 1	ISA
 2	ISA	级联
 3	ISA	WaveArtist
 4	ISA
 5	ISA
 6	ISA
 7	ISA	WaveArtist
======= ======= ===========
