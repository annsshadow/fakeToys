## spi_butterfly - 骞跺彛杞?butterfly 閫傞厤鍣ㄩ┍鍔。

这是一个硬件与软件项目，包含构建和使用一根并行端口适配器线缆，以及使用一“AVR Butterfly”来运行用于用户交互或传感器的固件。Butterfly 是一块价20 美元的电池供电卡片，带有一AVR 微控制器以及许多好东西：传感器、LCD闪存、拨动杆等。你可以AVR-GCC 为其开发固件，并使用该适配器线缆进行烧录
你可以用一根旧的打印机线缆制作这个适配器，并直接将东西焊接Butterfly 上或者（如果你有零件和技能），你可以做出更花哨的东西，为 Butterfly 和打印机
端口提供电路保护，或使用比打印机端口两个信号引脚更好的电源。又或者，你可以用
类似的线缆与许多 AVR 板卡通信，甚至包括面包板
这比 “ISP 编程线缆更强大，因为它让内核SPI 协议驱动能够AVR 交互甚至可以AVR 向它们发出中断。之后，你的协议驱动应该能轻松地与“真正的
SPI 控制器”配合工作，而不是这个位脉冲（bitbanger）器
第一根线缆的连接将把 Linux 连接到一SPI 总线（带AVR 和一DataFlash
芯片），以及 AVR 的复位线。这就是你重新烧录固件所需的全部，且这些引脚是
标准Atmel “ISP连接器引脚（在非 Butterfly AVR 板卡上也使用）。在
并口侧，这类似于 “sp12编程线缆
	======	  =============	  ===================
	Signal	  Butterfly	  Parport (DB-25)
	======	  =============	  ===================
	SCK	  J403.PB1/SCK	  pin 2/D0
	RESET	  J403.nRST	  pin 3/D1
	VCC	  J403.VCC_EXT	  pin 8/D6
	MOSI	  J403.PB2/MOSI	  pin 9/D7
	MISO	  J403.PB3/MISO	  pin 11/S7,nBUSY
	GND	  J403.GND	  pin 23/GND
	======	  =============	  ===================

然后，要Linux 主控该总线以与 DataFlash 芯片通信，你必须 (a) 烧录禁用 SPI
的新固件（设PRR.2，并通过清除 PORTB.[0-3] 禁用上拉）；(b) 配置 mtd_dataflash
驱动；以(c) 接入片选
	======	  ============	  ===================
	Signal	  Butterfly	  Parport (DB-25)
	======	  ============	  ===================
	VCC	  J400.VCC_EXT	  pin 7/D5
	SELECT	  J400.PB0/nSS	  pin 17/C3,nSELECT
	GND	  J400.GND	  pin 24/GND
	======	  ============	  ===================

或者，你可以烧录将 AVR 变为 SPI 从设备的固件（保DataFlash 处于复位），调整 spi_butterfly 驱动使其绑定到你的自定义基于 SPI 的协议驱动
“USI控制器（使用 J405）也可用于第二条 SPI 总线。这将让你使用自定义SPI-with-USI 固件AVR 通信，同时让 Linux AVR 任一方使DataFlash。有
大量空闲的并口引脚可用于连接这条总线，例如：

	======	  =============	  ===================
	Signal	  Butterfly	  Parport (DB-25)
	======	  =============	  ===================
	SCK	  J403.PE4/USCK	  pin 5/D3
	MOSI	  J403.PE5/DI	  pin 6/D4
	MISO	  J403.PE6/DO	  pin 12/S5,nPAPEROUT
	GND	  J403.GND	  pin 22/GND

	IRQ	  J402.PF4	  pin 10/S6,ACK
	GND	  J402.GND(P2)	  pin 25/GND
	======	  =============	  ===================
