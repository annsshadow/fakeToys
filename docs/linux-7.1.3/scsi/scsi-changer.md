
## SCSI 介质换片器驱

这是一个用SCSI 介质换片器（Medium Changer）设备的驱动，这些设备在
/proc/scsi/scsi 中以 “Type: Medium Changer列出
这是用于**真正**的自动换片机（Jukebox）的。它**不支*常见的小CD-ROM
换片器，无论是每槽一LUN SCSI 换片器还IDE 驱动器
用户态工具可从这里获取：
	http://linux.bytesex.org/misc/changer.html


### 一般信

先说几句关于换片器如何工作的话：一个换片器2 个（可能更多）SCSI ID。一个用控制机械臂的换片器设备，另一个用于实际读写数据的设备。后者可以是任何东西，一MOD、一CD-ROM、一盘磁带或随便什么。对于换片器设备来说这是“无所谓”的，它***
搬动介质，不做别的事
与（例如）IDE-CD 换片器相比，SCSI 换片器模型更复杂。但它几乎能处理所有可能的情况它知4 种不同类型的换片器元素：

  ===============   ==================================================
  media transport   搬动介质的那个，即传送臂。也称为 “picker”  storage           可容纳介质的槽位  import/export     与上面相同，但可从外部访问，即操作员（你！）可以
                    用它向换片器中装入或取出介质		    sometimes 称为 “mailslot”  data transfer     读写数据的设备，CD-ROM / 磁带 / 随便什么驱动器  ===============   ==================================================

这些元素都不限于一个：一个巨大的自动换片机可以有 123 CD-ROM 的槽位 CD-ROM 读取器（因此共有 6 SCSI ID：换片器加上每个 CD-ROM）以2 条传送臂处理起来毫无问题

### 它是如何实现

我将该驱动实现为一个带 NetBSD 风格 ioctl 接口的字符设备驱动。只是借用NetBSD
的头文件以及另一Linux SCSI 设备驱动作为起点。该接口应与 NetBSD 源代码兼容因此如果有任何（有人知道吗？？？）支BSD 风格换片器驱动的软件，它也应该能与该
驱动配合工作
随着时间的推移又增加了几ioctl，例如卷标（volume tag）支持并未被 NetBSD ioctl API 覆盖

### 当前状

对多于一条传送臂的支持尚未实现（而且到目前为止也没人要求……）
我自己用一个来Grundig 35 CD-ROM 自动换片机测试并使用该驱动。我收到一报告称它与磁带自动加载器（Exabyte、HP DEC）配合正常。有些人将该驱动用于 amanda它可以与小（11 槽）和巨大（4 MO8 槽）的磁光自动换片机正常工作。很可能也适用许多其它换片器，大多数人（但不是全部 :-)）只在该驱动***工作时才给我发邮件…
我没有任何设备列表，既没有黑名单也没有白名单。因此询问我某个具体设备是否受支持是
相当无用的。理论上，每个支SCSI-2 介质换片器命令集的换片器设备都应能开箱即用地该驱动配合工作。如果不能，那就是一bug。要么在驱动内，要么在该换片器设备的固件内

### 使用

这是一个主设备号为 86 的字符设备，因此使用
“mknod /dev/sch0 c 86 0来为该驱动创建设备特殊文件
如果模块找到了换片器，它会打印一些关于该设备的消[ 如果看不到任何东西，试试
“dmesg]，并且应出现/proc/devices 中。如果没有……某些换片器对设备使ID  /
LUN 0，对机械臂机制使ID  / LUN 1。但 Linux 默认***查找0 以外LUN，因有太多损坏的设备。因此你可以尝试
  1) echo "scsi add-single-device 0 0 ID 1" > /proc/scsi/scsi
     （将 ID 替换为设备的 SCSI-ID  2) 在内核命令行上以 “max_scsi_luns=1引导内核
     （在 lilo.conf append="max_scsi_luns=1" 应该可以奏效

### 遇到问题

如果你以 “insmod debug=1加载该驱动，它会变得啰嗦并向 syslog 打印大量内容。以
CONFIG_SCSI_CONSTANTS=y 编译内核会大幅改善错误消息的质量，因为届时内核会将这错误代码翻译成人类可读的字符串
你可以用 dmesg 命令（或检查日志文件）显示这些消息。如果你因为驱动的问题而通过邮件
向我提问，请附上这些消息

### Insmod 选项


debug=0/1
	启用调试消息（见上文，默认：0）
verbose=0/1
	输出详细信息（默认：1）
init=0/1
	insmod 时向换片器发INITIALIZE ELEMENT STATUS 命令
	（默认：1）
timeout_init=<seconds>
	INITIALIZE ELEMENT STATUS 命令的超	（默认：3600）
timeout_move=<seconds>
	所有其它命令的超时（默认：120）
dt_id=<id1>,<id2>,... / dt_lun=<lun1>,<lun2>,...
	这两个选项允许指定数据传输元素SCSI ID LUN。你可能不需	这个，因为自动换片机应提供该信息。但某些设备不提供…
vendor_firsts=, vendor_counts=, vendor_labels=
	这些 insmod 选项可用于告知驱动存在一些厂商特定的元素类型。例	Grundig 就这样做。某些自动换片机带有一台用于给新刻录的 CD 贴标	打印机，它被当作元素 0xc000（类5）来寻址。要告知驱动

```
		$ insmod ch			\
			vendor_firsts=0xc000	\
			vendor_counts=1		\
			vendor_labels=printer

		这三insmod 选项均可接受最多四个逗号分隔的值，这样你就可以
		配置元素类型 5-8。你很可能需要该设备对应SCSI 规范才能找到
		正确的值，因为它们未被 SCSI-2 标准覆盖

```


### 致谢


我使用著名的“向全世界发送补丁”方法编写了该驱动。在（或多或少）以下人士的帮助下
 - Daniel Moehwald <moehwald@hdg.de>
 - Dane Jasper <dane@sonic.net>
 - R. Scott Bailey <sbailey@dsddi.eds.com>
 - Jonathan Corbet <corbet@lwn.net>

特别感谢

 - Martin Kuehne <martin.kuehne@bnbt.de>

提供了一台老旧、二手（但功能完整）CD-ROM 自动换片机，我现在用它来开测试
驱动与工具
祝玩得开心，

   Gerd

Gerd Knorr <kraxel@bytesex.org>
