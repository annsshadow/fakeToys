## ISO7816 串行通信


## 1. 简

  ISO/IEC7816 是一系列规定集成电路卡（ICC，也称为智能卡）的标准
## 2. 与硬件相关的考虑


  某些 CPU/UART（例Microchip AT91）包含一个内置模式，能够处理与智能卡的通信
  对于这些微控制器，Linux 驱动应当被做成能够在两种模式下工作，并且应当在用户层
  提供适当ioctl（见后文），以允许从一种模式切换到另一种模式，反之亦然
## 3. 内核中已有的数据结构


  Linux 内核提供serial_iso7816 结构体（[^1^]）来处理 ISO7816 通信。该数据
  结构用于ioctl 中设置和配置 ISO7816 参数
  任何能够同时作为 RS232 ISO7816 工作的设备的驱动，都应当uart_port 结构体中
  实现 iso7816_config 回调。serial_core 调用 iso7816_config 来完成设备相关的部分  以响TIOCGISO7816 TIOCSISO7816 ioctl（见下文）。iso7816_config 回调接收一  指向 struct serial_iso7816 的指针
## 4. 在用户层的使

  在用户层，可以使用前面的方式获取/设置 ISO7816 配置

```

	#include <linux/serial.h>

	/* 包含 ISO7816 ioctl 的定义：TIOCSISO7816 TIOCGISO7816 */
	#include <sys/ioctl.h>

	/* 打开你的特定设备（例/dev/mydevice）： */
	int fd = open ("/dev/mydevice", O_RDWR);
	if (fd < 0) {
		/* 错误处理。参errno*/
	}

	struct serial_iso7816 iso7816conf;

	/* 保留字段必须清零 */
	memset(&iso7816conf, 0, sizeof(iso7816conf));

	/* 启用 ISO7816 模式*/
	iso7816conf.flags |= SER_ISO7816_ENABLED;

	/* 选择协议*/
	/* T=0 */
	iso7816conf.flags |= SER_ISO7816_T(0);
	/* 鎴?T=1 */
	iso7816conf.flags |= SER_ISO7816_T(1);

	/* 设置保护时间（guard time）： */
	iso7816conf.tg = 2;

	/* 设置时钟频率 */
	iso7816conf.clk = 3571200;

	/* 设置传输因子*/
	iso7816conf.sc_fi = 372;
	iso7816conf.sc_di = 1;

	if (ioctl(fd_usart, TIOCSISO7816, &iso7816conf) < 0) {
		/* 错误处理。参errno*/
	}

	/* 在此使用 read() write() 系统调用... */

	/* 完成后关闭设备： */
	if (close (fd) < 0) {
		/* 错误处理。参errno*/
	}

```
## 5. 参考资

 [^1^]    include/uapi/linux/serial.h
