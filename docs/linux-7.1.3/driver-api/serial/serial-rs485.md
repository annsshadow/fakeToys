## RS485 串行通信


## 1. 简介


   EIA-485，又称 TIA/EIA-485 或 RS-485，是一个定义用于平衡式数字多点系统的
   驱动器和接收器电气特性的标准。
   该标准广泛用于工业自动化领域的通信，因为它可以有效地用于长距离传输，
   并且在电气噪声环境中也能工作。

## 2. 硬件相关考虑


   某些 CPU/UART（例如 Atmel AT91 或 16C950 UART）内置了半双工模式，能够通过
   切换 RTS 或 DTR 信号自动控制线路方向。这可用于控制外部的半双工硬件，如 RS485
   收发器，或任何连接到 RS232 的半双工设备，如某些调制解调器。

   对于这些微控制器，Linux 驱动应当能够同时工作于两种模式，并且应当在用户层提供
   适当的 ioctl（见后文），以允许从一种模式切换到另一种模式，反之亦然。

## 3. 内核中已有的数据结构


   Linux 内核提供了 struct serial_rs485 来处理 RS485 通信。该数据结构用于在
   平台数据和 ioctl 中设置和配置 RS485 参数。

   设备树也可以提供 RS485 启动参数（[#DT-bindings]_）。当驱动调用
   uart_get_rs485_mode() 时，串行核心会根据设备树给出的值填充 struct serial_rs485。

   任何能够同时工作于 RS232 和 RS485 的设备的驱动都应实现 `struct uart_port` 回调，并在
   `struct uart_port` 中提供 `rs485_supported`。串行核心调用 `rs485_supported` 来响应
   TIOCSRS485 ioctl（见下文）完成设备相关的部分。`struct uart_port` 回调接收一个指向经过
   净化的 struct serial_rs485 的指针。用户空间提供的 struct serial_rs485 在调用
   `struct uart_port` 之前会先经 `rs485_supported` 净化，该回调指示驱动针对 `struct uart_port`
   支持哪些 RS485 特性。TIOCGRS485 ioctl 可用于读回与当前配置匹配的 struct serial_rs485。

   :identifiers: serial_rs485 uart_get_rs485_mode

## 4. 用户层的使用


   在用户层，可以使用前述的接口获取/设置 RS485 配置
```

	#include <linux/serial.h>

	/* Include definition for RS485 ioctls: TIOCGRS485 and TIOCSRS485 */
	#include <sys/ioctl.h>

	/* Open your specific device (e.g., /dev/mydevice): */
	int fd = open ("/dev/mydevice", O_RDWR);
	if (fd < 0) {
		/* Error handling. See errno. */
	}

	struct serial_rs485 rs485conf;

	/* Enable RS485 mode: */
	rs485conf.flags |= SER_RS485_ENABLED;

	/* Set logical level for RTS pin equal to 1 when sending: */
	rs485conf.flags |= SER_RS485_RTS_ON_SEND;
	/* or, set logical level for RTS pin equal to 0 when sending: */
	rs485conf.flags &= ~(SER_RS485_RTS_ON_SEND);

	/* Set logical level for RTS pin equal to 1 after sending: */
	rs485conf.flags |= SER_RS485_RTS_AFTER_SEND;
	/* or, set logical level for RTS pin equal to 0 after sending: */
	rs485conf.flags &= ~(SER_RS485_RTS_AFTER_SEND);

	/* Set rts delay before send, if needed: */
	rs485conf.delay_rts_before_send = ...;

	/* Set rts delay after send, if needed: */
	rs485conf.delay_rts_after_send = ...;

	/* Set this flag if you want to receive data even while sending data */
	rs485conf.flags |= SER_RS485_RX_DURING_TX;

	if (ioctl (fd, TIOCSRS485, &rs485conf) < 0) {
		/* Error handling. See errno. */
	}

	/* Use read() and write() syscalls here... */

	/* Close the device when finished: */
	if (close (fd) < 0) {
		/* Error handling. See errno. */
	}

```
## 5. 多点寻址


   Linux 内核为多点 RS-485 串行通信线路提供了寻址模式。该寻址模式通过在
   struct serial_rs485 中设置 `SER_RS485_ADDRB` 标志来启用。struct serial_rs485 另有两个
   附加标志和字段，用于启用接收地址和目的地址。

   地址模式标志：
 - `SER_RS485_ADDR_DEST`：启用寻址模式（同时设置 termios 中的 ADDRB）。
 - `SER_RS485_ADDR_DEST`：启用接收（过滤）地址。
 - `SER_RS485_ADDR_DEST`：设置目的地址。

   地址字段（由相应的 `addr_dest` 标志启用）：
 - `addr_dest`：接收地址。
 - `addr_dest`：目的地址。

   一旦设置了接收地址，通信就只能与特定设备进行，其他对等方会被过滤掉。是否强制
   执行过滤由接收方决定。若未设置 `SER_RS485_ADDR_RECV`，接收地址将被清除。

   注意：并非所有支持 RS485 的设备都支持多点寻址。

## 6. 参考资料


