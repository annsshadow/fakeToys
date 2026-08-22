## GSM 0710 tty 多路复用器使用说

该线路规程实现了 GSM 07.10 多路复用协议，详见以3GPP 文档
	https://www.3gpp.org/ftp/Specs/archive/07_series/07.10/0710-720.zip

本文档给出了一些关于如何将该驱动用于连接到物理串口GPRS 3G 调制解调器的提示
## 如何使用


### 配置发起

#. 通过其串口将调制解调器初始化0710 多路复用（mux）模式（通常使用 `AT+CMUX=` 命令）。根据所用调制解调器的不同，可以向该命令传入或多或少参数
#. 使用 `TIOCSETD` ioctl 将串行线路切换为使用 n_gsm 线路规程
#. 如有需要，使用 `GSMIOC_GETCONF_EXT`/`GSMIOC_SETCONF_EXT` ioctl 配置多路复用器
#. 使用 `GSMIOC_GETCONF`/`GSMIOC_SETCONF` ioctl 配置多路复用器
#. 对于非默认配置，使用 `GSMIOC_GETCONF_DLCI`/`GSMIOC_SETCONF_DLCI` ioctl 配置 DLC
#. 获取所用串口的基准 gsmtty 编号
   初始化程序的主要部分
```

      #include <stdio.h>
      #include <stdint.h>
      #include <linux/gsmmux.h>
      #include <linux/tty.h>

      #define DEFAULT_SPEED	B115200
      #define SERIAL_PORT	/dev/ttyS0

      int ldisc = N_GSM0710;
      struct gsm_config c;
      struct gsm_config_ext ce;
      struct gsm_dlci_config dc;
      struct termios configuration;
      uint32_t first;

      /* open the serial port connected to the modem */
      fd = open(SERIAL_PORT, O_RDWR | O_NOCTTY | O_NDELAY);

      /* configure the serial port : speed, flow control ... */

      /* send the AT commands to switch the modem to CMUX mode
         and check that it's successful (should return OK) */
      write(fd, "AT+CMUX=0\r", 10);

      /* experience showed that some modems need some time before
         being able to answer to the first MUX packet so a delay
         may be needed here in some case */
      sleep(3);

      /* use n_gsm line discipline */
      ioctl(fd, TIOCSETD, &ldisc);

      /* get n_gsm extended configuration */
      ioctl(fd, GSMIOC_GETCONF_EXT, &ce);
      /* use keep-alive once every 5s for modem connection supervision */
      ce.keep_alive = 500;
      /* set the new extended configuration */
      ioctl(fd, GSMIOC_SETCONF_EXT, &ce);
      /* get n_gsm configuration */
      ioctl(fd, GSMIOC_GETCONF, &c);
      /* we are initiator and need encoding 0 (basic) */
      c.initiator = 1;
      c.encapsulation = 0;
      /* our modem defaults to a maximum size of 127 bytes */
      c.mru = 127;
      c.mtu = 127;
      /* set the new configuration */
      ioctl(fd, GSMIOC_SETCONF, &c);
      /* get DLC 1 configuration */
      dc.channel = 1;
      ioctl(fd, GSMIOC_GETCONF_DLCI, &dc);
      /* the first user channel gets a higher priority */
      dc.priority = 1;
      /* set the new DLC 1 specific configuration */
      ioctl(fd, GSMIOC_SETCONF_DLCI, &dc);
      /* get first gsmtty device node */
      ioctl(fd, GSMIOC_GETFIRST, &first);
      printf("first muxed line: /dev/gsmtty%i\n", first);

      /* and wait for ever to keep the line discipline enabled */
      daemon(0,0);
      pause();

```
#. 将这些设备当作普通串口使用
   例如，可以：

   - 使用 **gnokii** `ttygsm1` 上发/ 接收短信
   - 使用 **ppp** `ttygsm2` 上建立数据链
#. 在关闭物理端口之前，先关闭所有虚拟端口
   注意，关闭物理端口后调制解调器仍处于多路复用模式。这可能会导致稍后无法成功重新打开该端口。为避免这种情况，可以在初始化多路复用模式之前，在硬件允许的情况下复位调制解调器，或者手动发送断开连接命令```

      0xf9, 0x03, 0xef, 0x03, 0xc3, 0x16, 0xf9

```
### 配置请求

#. 通过其串口接`AT+CMUX=` 命令，初始化多路复用模式配置
#. 使用 `TIOCSETD` ioctl 将串行线路切换为使用 **n_gsm** 线路规程
#. 如有需要，使用 `GSMIOC_GETCONF_EXT`/`GSMIOC_SETCONF_EXT`
   ioctl 配置多路复用器
#. 使用 `GSMIOC_GETCONF`/`GSMIOC_SETCONF` ioctl 配置多路复用器
#. 对于非默认配置，使用 `GSMIOC_GETCONF_DLCI`/`GSMIOC_SETCONF_DLCI` ioctl 配置 DLC
```

        #include <stdio.h>
        #include <stdint.h>
        #include <linux/gsmmux.h>
        #include <linux/tty.h>
        #define DEFAULT_SPEED	B115200
        #define SERIAL_PORT	/dev/ttyS0

	int ldisc = N_GSM0710;
	struct gsm_config c;
	struct gsm_config_ext ce;
	struct gsm_dlci_config dc;
	struct termios configuration;
	uint32_t first;

	/* open the serial port */
	fd = open(SERIAL_PORT, O_RDWR | O_NOCTTY | O_NDELAY);

	/* configure the serial port : speed, flow control ... */

	/* get serial data and check "AT+CMUX=command" parameter ... */

	/* use n_gsm line discipline */
	ioctl(fd, TIOCSETD, &ldisc);

	/* get n_gsm extended configuration */
	ioctl(fd, GSMIOC_GETCONF_EXT, &ce);
	/* use keep-alive once every 5s for peer connection supervision */
	ce.keep_alive = 500;
	/* set the new extended configuration */
	ioctl(fd, GSMIOC_SETCONF_EXT, &ce);
	/* get n_gsm configuration */
	ioctl(fd, GSMIOC_GETCONF, &c);
	/* we are requester and need encoding 0 (basic) */
	c.initiator = 0;
	c.encapsulation = 0;
	/* our modem defaults to a maximum size of 127 bytes */
	c.mru = 127;
	c.mtu = 127;
	/* set the new configuration */
	ioctl(fd, GSMIOC_SETCONF, &c);
	/* get DLC 1 configuration */
	dc.channel = 1;
	ioctl(fd, GSMIOC_GETCONF_DLCI, &dc);
	/* the first user channel gets a higher priority */
	dc.priority = 1;
	/* set the new DLC 1 specific configuration */
	ioctl(fd, GSMIOC_SETCONF_DLCI, &dc);
	/* get first gsmtty device node */
	ioctl(fd, GSMIOC_GETFIRST, &first);
	printf("first muxed line: /dev/gsmtty%i\n", first);

	/* and wait for ever to keep the line discipline enabled */
	daemon(0,0);
	pause();

```
11-03-08 - Eric B茅nard - <eric@eukrea.com>
