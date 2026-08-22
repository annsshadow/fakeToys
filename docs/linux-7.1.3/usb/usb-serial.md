## USB serial


## 简


  USB 串口（serial）驱动目前支持多种不同的 USB 转串口（USB to serial）转换器
  产品，以及一些从用户空间（userspace）使用串口接口与设备进行通信的设备

  关于不同设备的具体信息，请参见下文各个产品的章节


## 配置


  当前该驱动一次最多可处理 256 个不同的串口接口

    驱动使用的主设备号（major number）为 188，因此要使用此驱动，可执行：
```
	mknod /dev/ttyUSB0 c 188 0
	mknod /dev/ttyUSB1 c 188 1
	mknod /dev/ttyUSB2 c 188 2
	mknod /dev/ttyUSB3 c 188 3
		.
		.
		.
	mknod /dev/ttyUSB254 c 188 254
	mknod /dev/ttyUSB255 c 188 255
```

  当设备被连接并被驱动识别后，驱动会向系统日志打印该设备所绑定到的节点（node）


## 支持的具体设


### ConnectTech WhiteHEAT 四端口转换器


  ConnectTech 非常积极地提供其设备的相关信息，包括提供了一台用于测试的设备

  该驱动由 Connect Tech Inc. 官方支持
  http://www.connecttech.com

  关于此驱动的任何问题或故障，请联Connect Tech 的支持部门：support@connecttech.com


### HandSpring Visor、Palm USB Clié USB 驱动


  该驱动适用于所HandSpring USB、Palm USB Sony Clié USB 设备

  只有当设备尝试连接到主机（host）时，它才会作为一个有效的 USB 设备出现
  主机面前。发生此情况时，设备会被正确枚举（enumerated），分配一个端口，然后
  _应当_ 可以进行通信了。当设备被移除，或设备端取消了连接时，驱动会妥善清理

  注意
    这意味着为了与设备通信，必须在尝试让任何程序与设备通信之前，先按下同步
    （sync）按钮。这pilot-xfer 及其他软件包的当前文档相悖，但由于设备中
    硬件限制，这是唯一可行的方式

  当设备连接后，尝试在第二个端口上与其通信（如果你系统中没有其usb-serial
  设备，通常/dev/ttyUSB1）。系统日志会告诉HotSync 传输应使用哪个端口
  “Generic端口可用于其它设备通信，例如一PPP 链路

  对于一Sony Clié 设备，必须使/dev/ttyUSB0 来与设备通信。所OS 版本 3.5
  的设备，以及大多数已升级到较OS 版本的设备都如此。关于哪个是正确的端口，
  请查看内核系统日志中的信息

  如果在按下同步按钮后，系统日志中没有任何显示，尝试重置设备，先热重置（hot
  reset），必要时再冷重置（cold reset）。有些设备在USB 端口正常通信之前
  需要这样做

  未编译进内核的设备可以通过模块参数指定。例如：
  modprobe visor vendor=0x54c product=0x66

  关于驱动的这一部分，有一个网页和邮件列表
  http://sourceforge.net/projects/usbvisor/

  关于此驱动的任何问题或故障，请联Greg Kroah-Hartman：greg@kroah.com


### PocketPC PDA 驱动


  该驱动可用于通过 USB 线缆/底座（cradle）连接到运行 Windows CE 3.0 
  PocketPC 2002 Compaq iPAQ、HP Jornada、Casio EM500 及其PDA
  大多数被 ActiveSync 支持的设备开箱即可使用。对于其它设备，请使用模块参数指
  产品（product）和厂商（vendor）id。例如：
  modprobe ipaq vendor=0x3f0 product=0x1125

  驱动提供一个串口接口（通常/dev/ttyUSB0 上），可在此之上运行 ppp 并建立与
  PDA TCP/IP 链路。完成后，你可以传输文件、备份、下载邮件等。使USB 最显著
  优势是速度——我可以73 113 kbytes/sec 的速度与我iPAQ 进行下载/上传

  该驱动只是利USB 连接所需的一组组件之一。请访问 http://synce.sourceforge.net
  其中包含所需的软件包以及简单的分步操作指南（howto）

  连接后，你可以使Win CE 端的程序ftpView、Pocket Outlook，以Linux 端的
  xcerdisp、synce 工具

  要使Pocket IE，请按照 http://www.tekguru.co.uk/EM500/usbtonet.htm 给出
  说明，在 Win98 上实现同样的效果。省略代理服务器部分；与 Win98 不同，Linux 完全
  能够转发数据包。至少对 iPAQ 还需要一处修改——通过
  Start/Settings/Connections 菜单禁用自动同步（autosync），取消勾
  “Automatically synchronize ...框。进Start/Programs/Connections，连接线缆并
  选择 “usbdial”（或你为新 USB 连接起的名字）。你最终应当会看到一
  “Connected to usbdial窗口，状态显示为已连接。现在启PIE 并开始浏览

  如果由于某种原因无法工作，请以模块参“debug设为 1 加载 usbserial ipaq
  模块，并检查系统日志。你也可以尝试在建立连接前对你的 PDA 进行软重置（soft-reset）

  根据你的 PDA，可能还有其它功能可用。据 Wes Cilldhaire
  <billybobjoehenrybob@hotmail.com> 所述，Toshiba E570 上，……如果你启动进入
  引导加载程序（bootloader）（在按下重置按钮时按住电源键，并持续按住电源键直到
  显示引导加载程序画面），然后将其放入已加ipaq 驱动的底座中，在 /dev/ttyUSB0
  上打开一个终端，它会给你一“USB Reflash终端，可用于刷写 ROM，以microP
  代码……这样就不需Toshiba 价350 美元的用于刷写的串口线缆了！D
  注意：这尚未经过测试。使用风险自负

  关于驱动的任何问题或故障，请联系 Ganesh Varadarajanganesh@veritas.com>


### Keyspan PDA 串口适配


  单端DB-9 串口适配器，作为 iMac PDA 适配器推广（主要Macintosh 目录
  销售，是一个半透明绿相间的转换器（dongle））。相当简单的设备。固件为自制
  （homebrew）。该驱动也适用Xircom/Entrega 单端口串口适配器

  当前状态：

   可以正常工作的部分：
     - 基本的输输出（以 'cu' 测试
     - 串口线路跟不上时的阻塞写入（blocking write
     - 改变波特率（最115200
     - 获取/设置调制解调器控制引脚（TIOCM{GET,SET,BIS,BIC}
     - 发break（尽管持续时间看起来可疑

   不能正常工作的部分：
     - 设备字符串（内核记录的）带有尾随的二进制垃圾
     - 设备 ID 不正确，可能与其Keyspan 产品冲突
     - 改变波特率应当冲tx/rx 以避免出现残缺的半个字符

   todo 列表中的大项
     - 校验位（parity），每字7 8 位，1 2 个停止位
     - 硬件流控（HW flow control
     - 并非所有标USB 描述符都被处理：
       Get_Status、Set_Feature、O_NONBLOCK、select()

  关于此驱动的任何问题或故障，请联Brian Warner：warner@lothar.com


### Keyspan USA 系列串口适配


  单、双和四端口适配器——驱动使Keyspan 提供的固件，并在其支持下开发

  当前状态：

    USA-18X、USA-28X、USA-19、USA-19W USA-49W 均受支持，并已在不同波特率下
    8-N-1 字符设置进行了相当充分的测试。其它字符长度和校验设置目前尚未测试

    USA-28 尚未支持，但支持它应当相当直接。如果你需要此功能，请联系维护者

  更多信息见：

        http://www.carnationsoftware.com/carnation/Keyspan.html

  关于此驱动的任何问题或故障，请联Hugh Blemings：hugh@misc.nu


### FTDI 单端口串口驱


  这是一个单端口 DB-25 串口适配器

  支持的设备包括：

                - TripNav TN-200 USB GPS
                - Navis Engineering Bureau CH-4711 USB GPS

  关于此驱动的任何问题或故障，请联Bill Ryder


### ZyXEL omni.net lcd plus ISDN TA


  这是一ISDN TA。请将成功与问题都报告给 azummo@towertech.it


### Cypress M8 CY4601 系列串口驱动


  该驱动大部分Neil “koyamaWhelchel 开发。自此前的形式以来已得到改进，以支持
  动态串口线路设置并改善了线路处理。该驱动大部分稳定，并已smp 机器（双 P2
  上测试过

    CY4601 系列支持的芯片组

		CY7C63723, CY7C63742, CY7C63743, CY7C64013

    支持的设备：

  - DeLorme USB Earthmate GPS（SiRF Star II lp 架构
  - Cypress HID->COM RS232 閫傞厤鍣。

		Note:
			Cypress Semiconductor 声称与该
			hid->com 设备没有任何关联

     大多数使CY4601 系列芯片组的设备应当都可与该驱动配合工作，只要它
     遵循 CY4601 usbserial 规范

    技术说明：

        Earthmate 默认4800 8N1 启动……驱动在启动时会初始化为该设置
        usbserial 核心提供其余termios 设置，以及一些自定义termios
        以使输出格式正确且可解析

```
		$PSRF100,<protocol>,<baud>,<databits>,<stopbits>,<parity>*CHECKSUM
		$PSRF100,0,9600,8,1,0*0C

		It should then be sufficient to change the port termios to match this
		to begin communicating.

	As far as I can tell it supports pretty much every sirf command as
	documented online available with firmware 2.31, with some unknown
	message ids.

	The hid->com adapter can run at a maximum baud of 115200bps.  Please note
	that the device has trouble or is incapable of raising line voltage properly.
	It will be fine with null modem links, as long as you do not try to link two
	together without hacking the adapter to set the line high.

	The driver is smp safe.  Performance with the driver is rather low when using
	it for transferring files.  This is being worked on, but I would be willing to
	accept patches.  An urb queue or packet buffer would likely fit the bill here.

	If you have any questions, problems, patches, feature requests, etc. you can
	contact me here via email:

					dignome@gmail.com

		(your problems/patches can alternately be submitted to usb-devel)


```

### Digi AccelePort 驱动


  该驱动支Digi AccelePort USB 2 4 设备，即 2 端口（外加一个并口）4 端口
  USB 串口转换器。该驱动目前尚不支持 Digi AccelePort USB 8

  该驱动在 SMP 下配usb-uhci 驱动工作。它SMP 下配uhci 驱动不工作

  该驱动大体上可工作，尽管我们还有几个 ioctl 待实现，以及最后的测试与调试要做
  USB 2 上的并口作为串口转并口转换器被支持；换句话说，在 Linux 上它表现为另一
  USB 串口，尽管物理上它确实是一个并口。Digi AccelePort USB 8 尚不受支持

  关于此驱动的问题或故障，请联Peter Berger（pberger@brimson.com）或 Al Borchers
  （alborchers@steinerpoint.com）


### Belkin USB 串口适配F5U103


  来自 Belkin 的单端口 DB-9/PS-2 串口适配器，固件eTEK Labs 提供。Peracom 单端
  串口适配器以GoHubs 适配器也可与该驱动配合工作

  当前状态：

    以下项目已测试并可工作：

      - 娉㈢壒鐜?     300-230400
      - 数据     5-8
      - 停止     1-2
      - 校验     N,E,O,M,S
      - 握手        None、Software（XON/XOFF）、Hardware（CTSRTS、CTSDTR）[^1^]_
      - Break        设置与清
      - 线路控制    输入/输出查询与控[^2^]_

  .. [^1^]
         硬件输入流控仅在固件版本高于 2.06 时启用。请阅读描述 Belkin 固件勘误
         （errata）的源代码注释。硬件输出流控在所有固件版本中均可工作

  .. [^2^]
         对输入（CTS、DSR、CD、RI）的查询显示最后一次报告的状态。对输出
         （DTR、RTS）的查询显示最后一次请求的状态，可能不反映由自动硬件流控
         设置的当前状态

  TO DO 列表
    - 添加真正的调制解调器控制线查询能力。当前跟踪中断（interrupt）报告的状
      和请求的状态
    - 添加向应用程序报UART 错误情况的错误报告
    - 添加flush ioctl 的支持
    - 添加所有其它缺失的内容 :)

  关于此驱动的任何问题或故障，请联William Greathouse：wgreathouse@smva.com


### Empeg empeg-car Mark I/II 驱动


  这是一个实验性驱动，用于Empeg empeg-car mp3 播放器的客户端同步工具提供连
  支持

  提示
    - 不要忘记ttyUSB{0,1,2,...} 创建设备节点
    - modprobe empeg（modprobe 是你的好帮手
    - emptool --usb /dev/ttyUSB0（或你为设备节点起的任何名字

  关于此驱动的任何问题或故障，请联Gary Brubaker：xavyer@ix.netcom.com


### MCT USB 单端口串口适配U232


  该驱动适用于来Magic Control Technology Corp. MCT USB-RS232 转换
  5 针，型号 U232-P25）（也有 9 针型U232-P9）。关于该设备的更多信息可
  制造商网站找到：http://www.mct.com.tw

  该驱动大体可工作，但仍需要更多测试。它派生Belkin USB 串口适配F5U103 驱动
  TODO 列表对它也有效

  该驱动也已发现可用于具有相同 Vendor ID 但不Product ID 的其它产品。Sitecom 
  U232-P25 串口转换器使Product ID 0x230 Vendor ID 0x711，并可与该驱动配
  工作。此外，D-Link DU-H3SP USB BAY 也可与该驱动配合工作

  关于此驱动的任何问题或故障，请联Wolfgang Grandegger：wolfgang@ces.ch


### Inside Out Networks Edgeport 驱动


  该驱动支Inside Out Networks 制造的所有设备，具体为以下型号：

       - Edgeport/4
       - Rapidport/4
       - Edgeport/4t
       - Edgeport/2
       - Edgeport/4i
       - Edgeport/2i
       - Edgeport/421
       - Edgeport/21
       - Edgeport/8
       - Edgeport/8 Dual
       - Edgeport/2D8
       - Edgeport/4D8
       - Edgeport/8i
       - Edgeport/2 DIN
       - Edgeport/4 DIN
       - Edgeport/16 Dual

  关于此驱动的任何问题或故障，请联Greg Kroah-Hartman：greg@kroah.com


### REINER SCT cyberJack pinpad/e-com USB 芯片卡读卡器


  面向 ISO 7816 兼容的接触式芯片卡（contactbased chipcard）的接口，例GSM SIM

  当前状态：

    这是USB 读卡器驱动的 kernel 部分。也提供了一个用CT-API 驱动的用户部
    （user part）。下载站点待定（TBA）。目前，你可以向维护者（linux-usb@sii.li
    索取

  关于此驱动的任何问题或故障，请联linux-usb@sii.li


### Prolific PL2303 驱动


  该驱动支持任何内Prolific PL2303 芯片的设备。这包括大量单端USB 转串
  转换器、超70% USB GPS 设备010 年），以及一USB UPS。来Aten
  （UC-232）和 IO-Data 的设备，以及 DCU-11 手机线缆，都可与该驱动配合工作

  关于此驱动的任何问题或故障，请联Greg Kroah-Hartman：greg@kroah.com


### KL5KUSB105 芯片/ PalmConnect USB 单端口适配


当前状态：

  该驱动是通过观察 Palm Windows 下的驱动所做的 usb 总线事务（bus transaction
  拼凑而成的，因此大量功能仍然缺失。值得注意的是，串ioctl 有时是伪造的或尚
  实现。不过对查询 DSR CTS 线路状态的支持已实现（尽管实现得并不优雅），因
  你常用的 autopilot(1) pilot-manager -daemon 调用可以工作。支持最115200 
  波特率，但不支持握手（软件或硬件），这就是为什么在解决此问题之前，对于大传输量
  明智的做法是降低所用速率

  关于该驱动的最新信息，请见 http://www.uuhaus.de/linux/palmconnect.html

### Winchiphead CH341 驱动


  该驱动适用Winchiphead CH341 USB-RS232 转换器。该芯片还实现了 IEEE 1284 并口
  I2C SPI，但驱动并不支持。协议是Windows 驱动的行为分析得出的，目前没
  可用的数据手册（datasheet）

  制造商网站：http://www.winchiphead.com/

  关于此驱动的任何问题或故障，请联frank@kingswood-consulting.co.uk

### Moschip MCS7720、MCS7715 驱动


  这些芯片出现在多家制造商（如 Syba Cables Unlimited）销售的设备中。可能还
  其它厂商720 提供两个串口715 提供一个串口和一个标PC 并口715 并口
  支持由一个单独的选项启用，除非先在设备驱动（Device Drivers）配置菜单的顶层启用
  并口支持，否则该选项不会出现。目前并口仅支持兼容模式（compatibility mode
  （无 ECP/EPP）

  TODO锛。
    - 为并口实ECP/EPP 模式
    - 高于 115200 的波特率目前有问题
    - 基于 Moschip MCS7703 的单串口设备，只需usb_device_id 表中简单添加一项，
      就可能与此驱动配合工作。我没有这样的设备，因此无法确定

### 通用（Generic）串口驱


  如果你的设备不是上面列出的设备，也不与上述型号兼容，你可以尝“generic
  接口。该接口不提供任何发送给设备的控制消息（control message），也不支持任何
  形式的设备流控。你的设备只需至少具有一个批量输入（bulk in）端点，或一个批量输
  （bulk out）端点

```
	echo <vid> <pid> >/sys/bus/usb-serial/drivers/generic/new_id

  其中 <vid> <pid> 替换为你设备的厂id 和产id 的十六进制表示
  如果驱动编译为模块，你也可以在加载模块时提供一id::

	insmod usbserial vendor=0x#### product=0x####

  该驱动已成功用于连接NetChip USB 开发板，提供了一种无需编写自定义驱动即
  开USB 固件的方式

  关于此驱动的任何问题或故障，请联Greg Kroah-Hartman：greg@kroah.com


```

## 联系方式


  如果任何人在使用上述指定产品中的驱动时遇到问题，请联系上面列出的特定驱动
  作者，或加Linux-USB 邮件列表（关于加入邮件列表的信息，以及其可搜索归档的
  链接，见 http://www.linux-usb.org/ 


Greg Kroah-Hartman
greg@kroah.com
