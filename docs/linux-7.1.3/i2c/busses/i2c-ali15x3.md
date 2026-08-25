## 内核驱动 i2c-ali15x3


支持的适配器：
  - Acer Labs, Inc. ALI 1533 1543C（南桥）

    Datasheet: 目前须签NDA
	http://www.ali.com.tw/

作者：
 - Frodo Looijaard <frodol@dds.nl>,
 - Philip Edelbrock <phil@netroedge.com>,
 - Mark D. Studebaker <mdsxyz123@yahoo.com>

### 模块参数


- force_addr: int
    初始i2c 控制器的基地址

### 说明


force_addr 参数对于 BIOS 中没有设置地址的主板很有用。它不会执行 PCI 强制操作；设备仍然必须存在于
lspci 中。除非驱动提示基地址未被设置，否则不要使用此参数
```

    modprobe i2c-ali15x3 force_addr=0xe800

```
ASUS P5A 主板上，SMBus 会周期性挂起，只能通过断电重启来清除。原因未知（见下文“问题”）
### 描述


这是 Acer Labs Inc. (ALI) M1541 M1543C 南桥SMB 主机控制器的驱动
M1543C 是面向桌面系统的南桥
M1541 是面向便携系统的南桥
它们属于以下 ALI 芯片组：

 - “Aladdin Pro 2包含 M1621 Slot 1 北桥，带 AGP    100MHz CPU 前端总线
 - “Aladdin V包含 M1541 Socket 7 北桥，带 AGP 100MHz
   CPU 前端总线

   一Aladdin V 主板 - Asus P5A
 - Atrend ATC-5220
 - BCM/GVC VP1541
 - Biostar M5ALA
 - Gigabyte GA-5AX（通常无法工作，因BIOS 没有
	  启用 7101 设备！）
 - Iwill XA100 Plus
 - Micronics C200
 - Microstar (MSI) MS-5169

  - “Aladdin IV包含 M1541 Socket 7 北桥    host bus 最83.3 MHz
有关这些芯片的概览，请参http://www.acerlabs.com。目前网站上完整的数据手册受密码保护，但如果联系
ALI 位于圣何塞的办公室，他们可能会提供密码
M1533/M1543C 设备PCI 总线上表现为四个独立的设备。一```

  00:02.0 USB Controller: Acer Laboratories Inc. M5237 (rev 03)
  00:03.0 Bridge: Acer Laboratories Inc. M7101      <= 这是我们需要的那个
  00:07.0 ISA bridge: Acer Laboratories Inc. M1533 (rev c3)
  00:0f.0 IDE interface: Acer Laboratories Inc. M5229 (rev c1)

```

   如果你的板子上装M1533 M1543C，并且你看到
   “ali15x3: Error: Can't detect ali15x3!   那么请运lspci
   如果你看1533 5229 设备但没7101 设备   那么你必须在 BIOS 中启ACPI、PMU、SMB 或类似选项
   如果找不M7101 设备，驱动将无法工作
SMB 控制器是 M7101 设备的一部分，M7101 是一个符ACPI 规范电源管理单元（PMU）
整个 M7101 设备都必须被启用，SMB 才能工作。你不能
只单独启SMB。SMB ACPI 拥有独立I/O 空间我们会确SMB 被启用，ACPI 则保持不动
### 特

该驱动仅控制 SMB 主机。M15X3 上的 SMB 从机
控制器未被启用。该驱动不使用中断
### 问题


该驱动仅SMB 寄存器请I/O 空间它不使用 ACPI 区域
ASUS P5A 主板上，有多份报告称
SMBus 会挂起，且只能通过
关闭计算机电源来解决。在主板温度升高时（例如 CPU 高负载，或夏季）情况似乎更严重该主板可能存在电气问题P5A 上，W83781D 传感器芯片同时位ISA SMBus 上。因此，仅通过 ISA 总线访问 W83781D 通常可以
避免 SMBus 挂起