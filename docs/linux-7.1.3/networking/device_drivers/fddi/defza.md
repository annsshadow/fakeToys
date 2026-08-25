
## 关于 DEC FDDIcontroller 700（DEFZA-xx）驱动的说明


:Version: v.1.1.4


DEC FDDIcontroller 700 DEC 面向 DECstation 5000
model 200 工作站、于 1990 年设计的第一TURBOchannel FDDI
网卡。该板卡为单连接站（SAS），有两种变体，均受支持
第一种是 SAS MMF DEFZA-AA 选件，采用原始设计实现标准的
MMF-PMD，但使用一ST 连接器而非通常MIC 连接器。另一种是
SAS ThinWire/STP DEFZA-CA 选件，记700-C，其网络介质可通过
开关在采用 BNC 连接器的 DEC 专有 ThinWire-PMD 与采DE-9F
连接器的标准 STP-PMD 之间切换。该选件可连DECconcentrator 500
设备，在使用 STP-PMD 时也可连接其FDDI 设备，其设计目的是通过
提供复用既有线缆的方式，使从现有 IEEE 802.3 10BASE2 以太网与
IEEE 802.5 令牌环网络过渡更为容易
本驱动可处理单个系统中安装的多块板卡。它们会按照 TURBOchannel
槽号递增的顺序被分别分配 fddi0、fddi1 等接口名
该板卡仅支持接收方向DMA。发送则使用 PIO。因此，在较重的
发送负载下，会对系统性能产生明显影响
该板卡支持一64 表项CAM 用于匹配目的地址。其中两个表项被
定向信标（Directed Beacon）与环清除（Ring Purger）组播地址占用其余用作组播过滤器。LLC 帧也支持 all-multi 模式，并在被显式请求
CAM 溢出时被使用。混杂模式对 LLC SMT 帧分别支持独立使能，
但本驱动不支持单独更改它们

已知问题

无

待办

5. MAC 地址修改。该卡不支持修改介质访问控制（Media Access
   Controller）的地址寄存器，但可通过CAM 中添加别名达到类似效果   不过无法禁用对原始地址的匹配
7. SMT 接收/RMC 发送环已满，在驱动中对入站/出站 SMT 帧进行排队。（？）

8. 获取/上报 FDDI/SNMP 统计信息

成功与失败的反馈都欢迎
Maciej W. Rozycki  <macro@orcam.me.uk>
