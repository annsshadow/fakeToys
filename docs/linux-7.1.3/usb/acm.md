## Linux ACM 驱动 v0.16


Copyright (c) 1999 Vojtech Pavlik <vojtech@suse.cz>

由 SuSE 赞助

#### 0. 免责声明

本程序是自由软件；您可以在自由软件基金会发布的 GNU 通用公共许可证条款下
重新发布和/或修改它；许可证版本为第 2 版，或（根据您的选择）任何更高版本。

本程序的分发希望它有用，但没有任何担保；甚至没有对适销性或特定用途适用性的
隐含担保。有关更多细节，请参阅 GNU 通用公共许可证。

您应该已经随本程序收到了一份 GNU 通用公共许可证；如果没有，请写信给自由软件
基金会，地址：Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA

如果您需要联系我（作者），可以通过电子邮件——将您的消息发送到
<vojtech@suse.cz>——或者通过纸质邮件：Vojtech Pavlik，Ucitelska 1576,
Prague 8, 182 00 Czech Republic

为方便起见，GNU 通用公共许可证第 2 版的文本已包含在软件包中：请参阅
COPYING 文件。

#### 1. 用法

drivers/usb/class/cdc-acm.c 驱动适用于符合通用串行总线通信设备类抽象控制模型
（USB CDC ACM）规范的 USB 调制解调器和 USB ISDN 终端适配器。

许多调制解调器都符合，以下是我所知道的型号列表：

 - 3Com OfficeConnect 56k
 - 3Com Voice FaxModem Pro
 - 3Com Sportster
 - MultiTech MultiModem 56k
 - Zoom 2986L FaxModem
 - Compaq 56k FaxModem
 - ELSA Microlink 56k

我知道有一款 ISDN TA 可以与 acm 驱动配合使用：

 - 3Com USR ISDN Pro TA

一些手机也通过 USB 连接。我知道以下手机可以工作：

 - SonyEricsson K800i

遗憾的是，许多调制解调器和大多数 ISDN TA 使用专有接口，因此无法与该驱动配合
工作。购买前请确认是否符合 ACM 规范。

```
	usbcore.ko
	uhci-hcd.ko ohci-hcd.ko or ehci-hcd.ko
	cdc-acm.ko
```

之后，调制解调器应当可被访问。您应当能够使用 minicom、ppp 和 mgetty 来操作它们。

#### 2. 验证是否工作


第一步应当检查 /sys/kernel/debug/usb/devices，其内容应当类似如下

```
  T:  Bus=01 Lev=00 Prnt=00 Port=00 Cnt=00 Dev#=  1 Spd=12  MxCh= 2
  B:  Alloc=  0/900 us ( 0%), #Int=  0, #Iso=  0
  D:  Ver= 1.00 Cls=09(hub  ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
  P:  Vendor=0000 ProdID=0000 Rev= 0.00
  S:  Product=USB UHCI Root Hub
  S:  SerialNumber=6800
  C:* #Ifs= 1 Cfg#= 1 Atr=40 MxPwr=  0mA
  I:  If#= 0 Alt= 0 #EPs= 1 Cls=09(hub  ) Sub=00 Prot=00 Driver=hub
  E:  Ad=81(I) Atr=03(Int.) MxPS=   8 Ivl=255ms
  T:  Bus=01 Lev=01 Prnt=01 Port=01 Cnt=01 Dev#=  2 Spd=12  MxCh= 0
  D:  Ver= 1.00 Cls=02(comm.) Sub=00 Prot=00 MxPS= 8 #Cfgs=  2
  P:  Vendor=04c1 ProdID=008f Rev= 2.07
  S:  Manufacturer=3Com Inc.
  S:  Product=3Com U.S. Robotics Pro ISDN TA
  S:  SerialNumber=UFT53A49BVT7
  C:  #Ifs= 1 Cfg#= 1 Atr=60 MxPwr=  0mA
  I:  If#= 0 Alt= 0 #EPs= 3 Cls=ff(vend.) Sub=ff Prot=ff Driver=acm
  E:  Ad=85(I) Atr=02(Bulk) MxPS=  64 Ivl=  0ms
  E:  Ad=04(O) Atr=02(Bulk) MxPS=  64 Ivl=  0ms
  E:  Ad=81(I) Atr=03(Int.) MxPS=  16 Ivl=128ms
  C:* #Ifs= 2 Cfg#= 2 Atr=60 MxPwr=  0mA
  I:  If#= 0 Alt= 0 #EPs= 1 Cls=02(comm.) Sub=02 Prot=01 Driver=acm
  E:  Ad=81(I) Atr=03(Int.) MxPS=  16 Ivl=128ms
  I:  If#= 1 Alt= 0 #EPs= 2 Cls=0a(data ) Sub=00 Prot=00 Driver=acm
  E:  Ad=85(I) Atr=02(Bulk) MxPS=  64 Ivl=  0ms
  E:  Ad=04(O) Atr=02(Bulk) MxPS=  64 Ivl=  0ms
```

这三行（以及 Cls= 'comm' 和 'data' 类）的存在很重要，它表示这是一个 ACM 设备。
Driver=acm 表示 acm 驱动正被用于该设备。如果您只看到 Cls=ff(vend.)，那么您就

```
  D:  Ver= 1.00 Cls=02(comm.) Sub=00 Prot=00 MxPS= 8 #Cfgs=  2
  I:  If#= 0 Alt= 0 #EPs= 1 Cls=02(comm.) Sub=02 Prot=01 Driver=acm
  I:  If#= 1 Alt= 0 #EPs= 2 Cls=0a(data ) Sub=00 Prot=00 Driver=acm
```

```
  usb.c: USB new device connect, assigned device number 2
  usb.c: kmalloc IF c7691fa0, numif 1
  usb.c: kmalloc IF c7b5f3e0, numif 2
  usb.c: skipped 4 class/vendor specific interface descriptors
  usb.c: new device strings: Mfr=1, Product=2, SerialNumber=3
  usb.c: USB device number 2 default language ID 0x409
  Manufacturer: 3Com Inc.
  Product: 3Com U.S. Robotics Pro ISDN TA
  SerialNumber: UFT53A49BVT7
  acm.c: probing config 1
  acm.c: probing config 2
  ttyACM0: USB ACM device
  acm.c: acm_control_msg: rq: 0x22 val: 0x0 len: 0x0 result: 0
  acm.c: acm_control_msg: rq: 0x20 val: 0x0 len: 0x7 result: 7
  usb.c: acm driver claimed interface c7b5f3e0
  usb.c: acm driver claimed interface c7b5f3f8
  usb.c: acm driver claimed interface c7691fa0
```

如果一切看起来正常，启动 minicom 并把它设置为与 ttyACM 设备通信，然后试着输入
'at'。如果它返回 'OK'，那么一切都在正常工作。
