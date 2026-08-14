## Opera 固件


作者：Marco Gittler <g.marco@freenet.de>

要为 Opera DVB-S1 USB 机顶盒提取固件，你需要复制以下文件：

2830SCap2.sys
2830SLoad2.sys

从 windriver 光盘复制到本目录。

然后运行：


	scripts/get_dvb_firmware opera1

之后你将得到 2 个文件：

dvb-usb-opera-01.fw
dvb-usb-opera1-fpga-01.fw

在此目录中。

将它们复制到 /lib/firmware/ 。

之后驱动即可加载固件（前提是你已在内核配置中启用了固件加载，并且 hotplug 正在运行）。
