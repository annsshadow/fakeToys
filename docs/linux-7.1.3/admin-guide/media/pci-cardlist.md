
## PCI 驱动

本文解释媒体设备如何通过 PCI ID（厂设备 ID、子系统 ID）识别板卡，并示范用 lspci 命令查询 PCI ID，同时说明为何部分驱动需card= 参数来匹配相同子系统 ID 的不同产品


PCI 板卡通过称为 PCI ID 的标识来识别。PCI ID 实际上由两部分组成：

 - 厂商 ID（Vendor ID）和设备 ID（device ID）；
 - 子系ID（Subsystem ID）和子系统设ID（Subsystem device ID）；

`lspci -nn` 命令可用于识别厂设备PCI ID

   :emphasize-lines: 3

    $ lspci -nn
    ...
    00:0a.0 Multimedia controller [^0480^]: Philips Semiconductors SAA7131/SAA7133/SAA7135 Video Broadcast Decoder [1131:7133] (rev d1)
    00:0b.0 Multimedia controller [^0480^]: Brooktree Corporation Bt878 Audio Capture [109e:0878] (rev 11)
    01:00.0 Multimedia video controller [^0400^]: Conexant Systems, Inc. CX23887/8 PCIe Broadcast Audio and Video Decoder with 3D Comb [14f1:8880] (rev 0f)
    02:01.0 Multimedia video controller [^0400^]: Internext Compression Inc iTVC15 (CX23415) Video Decoder [4444:0803] (rev 01)
    02:02.0 Multimedia video controller [^0400^]: Conexant Systems, Inc. CX23418 Single-Chip MPEG-2 Encoder with Integrated Analog Video/Broadcast Audio Decoder [14f1:5b7a]
    02:03.0 Multimedia video controller [^0400^]: Brooktree Corporation Bt878 Video Capture [109e:036e] (rev 11)
    ...

子系ID 可以使用 `lspci -vn` 获取

   :emphasize-lines: 4

    $ lspci -vn
    ...
	00:0a.0 0480: 1131:7133 (rev d1)
		Subsystem: 1461:f01d
		Flags: bus master, medium devsel, latency 32, IRQ 209
		Memory at e2002000 (32-bit, non-prefetchable) [size=2K]
		Capabilities: [^40^] Power Management version 2
    ...

在上述示例中，第一块卡使用 `saa7134` 驱动，其厂商/设备 PCI ID `1131:7133`
PCI 子系ID `1461:f01d`（参[Saa7134 卡列saa7134-cardlist>](Saa7134 card list<saa7134-cardlist>)）

遗憾的是，有时不同的产品会使用相同的 PCI 子系ID。因此，若干媒体驱动允许
传入 `card=` 参数，以便设置一个与特定板卡正确设置相匹配的卡号

下面列出了当前受支持PCI/PCIe 卡（不包staging 驱动）\ [#]_

下表汇总了各媒体驱动及其支持的设备说明

================  ========================================================
Driver            Name（支持的设备
================  ========================================================
altera-ci         Altera FPGA CI 模块
b2c2-flexcop-pci  Technisat/B2C2 Air/Sky/Cable2PC PCI 鍗。
bt878             基于 bt878 的电视卡 DVB/ATSC 支持
bttv             BT8x8 Video For Linux 视频采集
cobalt            Cisco Cobalt 设备
cx18              Conexant cx23418 MPEG 编码
cx23885           Conexant cx23885388x 的后继型号）
cx25821           Conexant cx25821 设备
cx88xx            Conexant 2388x（bt878 的后继型号）
ddbridge          Digital Devices 桥接设备
dm1105            基于 SDMC DM1105 PCI 
dt3155            DT3155 帧抓取卡
dvb-ttpci         AV7110 鍗。
earth-pt1         PT1 鍗。
earth-pt3         Earthsoft PT3 鍗。
hexium_gemini     Hexium Gemini 帧抓取卡
hexium_orion      Hexium HV-PCI6 Orion 帧抓取卡
hopper            基于 HOPPER 的卡
ipu3-cio2         Intel ipu3-cio2 驱动
ivtv              Conexant cx23416/cx23415 MPEG 编码/解码
ivtvfb            Conexant cx23415 甯х紦鍐。
mantis            基于 MANTIS 的卡
mgb4              Digiteq Automotive MGB4 帧抓取卡
mxb               Siemens-Nixdorf 多媒体扩展板（MXB
netup-unidvb      NetUP 閫氱敤 DVB 鍗。
ngene             Micronas nGene 设备
pluto2            Pluto2 鍗。
saa7134           Philips SAA7134 设备
saa7164           NXP SAA7164 设备
smipcie           SMI PCIe DVBSky 鍗。
solo6x10          Bluecherry / Softlogic 6x10 采集卡（MPEG-4/H.264
tw5864            Techwell TW5864 视频/音频抓取与编码卡
tw686x            Intersil/Techwell TW686x 设备
tw68              Techwell tw68x Video For Linux 视频采集
zoran             Zoran-36057/36067 JPEG 编解码器
================  ========================================================

其中部分驱动支持多个设备，如下面的卡列表所示：

- [bttv-cardlist](bttv-cardlist)
- [cx18-cardlist](cx18-cardlist)
- [cx23885-cardlist](cx23885-cardlist)
- [cx88-cardlist](cx88-cardlist)
- [ivtv-cardlist](ivtv-cardlist)
- [saa7134-cardlist](saa7134-cardlist)
- [saa7164-cardlist](saa7164-cardlist)
- [zoran-cardlist](zoran-cardlist)

