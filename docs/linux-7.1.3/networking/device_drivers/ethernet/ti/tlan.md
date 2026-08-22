## Linux TLAN 驱动


:Version: 1.14a

(C) 1997-1998 Caldera, Inc.

(C) 1998 James Banks

(C) 1999-2001 Torben Mathiasen <tmm@image.dk, torben.mathiasen@compaq.com>

驱动信息/更新请访http://www.compaq.com


## I. 支持的设

    只有 PCI 设备能配合该驱动工作
    支持的设备：

    =========	=========	===========================================
    Vendor ID	Device ID	Name
    =========	=========	===========================================
    0e11	ae32		Compaq Netelligent 10/100 TX PCI UTP
    0e11	ae34		Compaq Netelligent 10 T PCI UTP
    0e11	ae35		Compaq Integrated NetFlex 3/P
    0e11	ae40		Compaq Netelligent Dual 10/100 TX PCI UTP
    0e11	ae43		Compaq Netelligent Integrated 10/100 TX UTP
    0e11	b011		Compaq Netelligent 10/100 TX Embedded UTP
    0e11	b012		Compaq Netelligent 10 T/2 PCI UTP/Coax
    0e11	b030		Compaq Netelligent 10/100 TX UTP
    0e11	f130		Compaq NetFlex 3/P
    0e11	f150		Compaq NetFlex 3/P
    108d	0012		Olicom OC-2325
    108d	0013		Olicom OC-2183
    108d	0014		Olicom OC-2326
    =========	=========	===========================================


    注意事项
    我不确定 100BaseTX 子板（针对那些支持此类扩展的卡）是否能工作。我没有任何可靠
    的证据能表明可以或不可以
    但是，如果一张卡支持 100BaseTx 而无需额外的子板，它应当能100BaseTx 下工作
    “Netelligent 10 T/2 PCI UTP/Coax”（b012）设备未经测试，但我不认为会有任何问题

## II. 驱动选项


 1. 你可以在 insmod 命令行末尾追debug=x 来获取调试信息，其中 x 是一个位域，	   位含义如下：

	   ====		=====================================
	   0x01		开启通用调试信息	   0x02		开启接收调试信息	   0x04		开启发送调试信息	   0x08		开启链表调试信息	   ====		=====================================

 2. 你可以在 insmod 命令行末尾追aui=1，使适配器使AUI 接口而非 10 Base T
	   接口。如果你想在基于 TLAN 的设备上使用 BNC 连接器，也应这么做。（	   没有 AUI/BNC 连接器的设备上设置此选项可能会导致其无法正常工作。）

 3. 你可以设duplex=1 强制半双工，设置 duplex=2 强制全双工
 4. 你可以设speed=10 强制 10Mbs 操作，设speed=100 强制 100Mbs 操作	   （如果一张只支持 10Mbs 的卡被强制进100Mbs 模式，我不清楚会发生什么。）

 5. 你现在必须同时使speed=X duplex=Y。如果你只执行“insmod tlan.o speed=100”，
	   驱动会进行自动协商（Auto-Neg）。要强制一10Mbps 半双工链路，执行
	   鈥渋nsmod tlan.o speed=10 duplex=1鈥濄€?
 6. 如果驱动被编入内核，你可以使用第 3 和第 4 个参数分别设aui debug。例如：
```

		ether=0,0,0x1,0x7,eth0

	   这将 aui 设为 0x1、debug 设为 0x7，假eth0 是一个受支持TLAN 设备
	   第三个字节中的位分配如下
		====   ===============
		0x01   aui
		0x02   使用半双		0x04   使用全双		0x08   使用 10BaseT
		0x10   使用 100BaseTx
		====   ===============

	   在使用内核参数强制速率时，你也需要同时设speed duplex	   ether=0,0,0x12,0,eth0 将强制链路为 100Mbps 半双工
	7. 如果你的系统中有多块 tlan 适配器，你可以基于每块适配器使用上述选项。要强制
	   你的 eth1 适配器为 100Mbit/HD 链路，使:

		insmod tlan speed=0,100 duplex=0,1

	   这样 eth0 将使用自动协商，eth1 将被强制100Mbit/HD。注tlan 驱动最	   支持 8 块适配器

```
## III. 遇到问题时可尝试的事

 1. 确认你的卡的 PCI id 在上面的I 节所列之中 2. 确认路由正确 3. 尝试强制不同speed/duplex 设置

还有一tlan 邮件列表，你可以通过majordomo@vuser.vu.union.edu 发送邮件，正文中写“subscribe tlan”来加入
另有一tlan 网站：http://www.compaq.com
