## SCSI Interfaces Guide


:Author: James Bottomley
:Author: Rob Landley

## Introduction


### 协议 对比 总线


一upon 一time, the Small Computer 系统 接口 定义 两一
并行 I/O 总线 一数据 协议 connect 一wide variety 
peripherals (disk drives, tape drives, modems, printers, scanners,
optical drives, test equipment, medical 设备) 一host computer.

尽管 the 并行 (fast/wide/至尊 SCSI 总线 具有 largely fallen
超出 使用, the SCSI 命令 set 更多 widely 使用 ever 
communicate 设备 在…上 一数字 不同 buses.

The `SCSI protocol <https://www.t10.org/scsi-3.htm>`__ 一big-endian
peer-to-peer 数据based 协议. SCSI 命令 6, 10, 12, 16
bytes long, 通常 followed 一associated 数据 payload.

SCSI 命令 transported 在…上 just 关于 任何 kind 总线, 
the 默认 协议 用于 storage 设备 attached USB, SATA, SAS,
Fibre Channel, FireWire, ATAPI 设备. SCSI packets 
commonly exchanged 在…上 Infiniband,
TCP/IP (`iSCSI <https://en.wikipedia.org/wiki/ISCSI>`__), even `并行
ports <http://cyberelk.net/tim/parport/parscsi.html>`__.

### Design the Linux SCSI 子系


The SCSI 子系uses 一three layer design, upper, mid, low
layers. Every 操作 involving the SCSI 子系(例如 reading 一
扇区 来自 一disk) uses one 驱动 每个 the 3 levels: one upper
layer 椹卞姩, one lower layer 椹卞姩, 鍜?the SCSI midlayer.

The SCSI upper layer 提供 the 接口 之间 userspace the
内核, the form char 设备 nodes 用于 I/O ioctl().
The SCSI lower layer 包含 驱动 用于 特定 硬件 设备.

之间 the SCSI mid-layer, analogous 一网络 routing layer
例如 the IPv4  The SCSI mid-layer routes 一数据based 数据
协议 之间 the upper layer's /dev nodes the corresponding
设备 the lower layer. manages 命令 queues, 提供 错误
handling 电源管理 函数, responds ioctl()
requests.

## SCSI upper layer


The upper layer supports the user-kernel 接口 providing 设备
nodes.

### sd (SCSI Disk)


sd (sd_mod.o)

### sr (SCSI CD-ROM)


sr (sr_mod.o)

### st (SCSI Tape)


st (st.o)

### sg (SCSI Generic)


sg (sg.o)

### ch (SCSI Media Changer)


ch (ch.c)

## SCSI mid layer


### SCSI midlayer implementation


#### 包含/SCSI/SCSI_设备.h


   :internal:

#### 驱动/SCSI/SCSI.c


主要 文件 用于 the SCSI midlayer.

   :export:

#### 驱动/SCSI/scsicam.c


`SCSI 通用 Access
方法 <http://www.t10.org/ftp/t10/drafts/cam/cam-r12b.pdf>`__ 支持
函数, 用于 使用 HDIO_GETGEO, 

   :export:

#### 驱动/SCSI/SCSI_错误.c


通用 SCSI 错误/超时 handling routines.

   :export:

#### 驱动/SCSI/SCSI_devinfo.c


Manage SCSI_dev_info_列出, tracks blacklisted whitelisted
设备.

   :export:

#### 驱动/SCSI/SCSI_ioctl.c


Handle ioctl() calls 用于 SCSI 设备.

   :export:

#### 驱动/SCSI/SCSI_lib.c


SCSI queuing 搴。

   :export:

#### 驱动/SCSI/SCSI_lib_dma.c


SCSI 函数 depending DMA (map unmap scatter-gather
列表).

   :export:

#### 驱动/SCSI/SCSI_proc.c


The 函数 文件 提供 一接口 之间 the PROC 文件
系统 the SCSI 设备 驱动 它是 mainly 使用 用于 debugging,
statistics pass information directly the lowlevel 驱动. I.E.
plumbing 鍒?manage /proc/SCSI/\*


#### 驱动/SCSI/SCSI_netlink.c


Infrastructure 提供 async 事件 来自 transports userspace 通过
netlink, 使用 一单个 NETLINK_SCSITRANSPORT 协议 用于 全部
transports. 参见 `the original patch submission
<https://lore.kernel.org/linux-scsi/1155070439.6275.5.camel@localhost.localdomain/>`__
用于 更多 details.

   :internal:

#### 驱动/SCSI/SCSI_scan.c


Scan 一host determine (任何) 设备 attached. The
通用 scanning/probing algorithm 作为 follows, exceptions made 
depending 设备 特定 标志, compilation 选项, 全局
variable (boot 模块 加载 time) 设置. 一特定 LUN scanned
通过 一INQUIRY 命令; the LUN 具有 一设备 attached, 一SCSI_设备
allocated setup 用于  用于 every id every channel the
given host, 启动 scanning LUN 0. Skip hosts don't respond 
全部 一scan LUN 0. 否则, LUN 0 具有 一设备 attached,
allocate setup 一SCSI_设备 用于  target SCSI-3 up,
issue 一REPORT LUN, scan 全部 the LUNs returned the REPORT LUN;
else, sequentially scan LUNs up 直到 一最reached, 一LUN
seen cannot 具有 一设备 attached 

   :export:

#### 驱动/SCSI/SCSI_sysctl.c


Set up the sysctl 条目: "/dev/SCSI/logging_level"
(DEV_SCSI_LOGGING_LEVEL) 鍏?sets/returns SCSI_logging_level.

#### 驱动/SCSI/SCSI_sysfs.c


SCSI sysfs 接口 routines.

   :export:

#### 驱动/SCSI/hosts.c


mid lowlevel SCSI 驱动 接口

   :export:

#### 驱动/SCSI/SCSI_通用.c


通用 支持 函数

   :export:

### Transport classes


Transport classes service 用于 驱动 the SCSI lower
layer, 鍏?expose transport attributes 鍦?sysfs.

#### Fibre Channel transport


The 文件 驱动/SCSI/SCSI_transport_fc.c defines transport attributes
用于 Fibre Channel.

   :export:

#### iSCSI transport 绫。


The 文件 驱动/SCSI/SCSI_transport_iscsi.c defines transport
attributes 用于 the iSCSI  sends SCSI packets 在…上 TCP/IP
connections.

   :export:

#### 串行 Attached SCSI (SAS) transport 


The 文件 驱动/SCSI/SCSI_transport_sas.c defines transport
attributes 用于 串行 Attached SCSI, 一variant SATA aimed large
high-end 系统.

The SAS transport 包含 通用 code deal SAS HBAs, 一
approximated representation SAS topologies the 驱动 型号, 
各种 sysfs attributes expose 这些 topologies 管理
interfaces 鍒?userspace.

此外 the 基本 SCSI 核心 objects transport 
introduces two 额外 intermediate objects: The SAS PHY 作为
represented 结构sas_phy defines 一"outgoing" PHY 一SAS HBA 
Expander, the SAS remote PHY represented 结构sas_rphy defines
一"incoming" PHY 一SAS Expander end 设备. 注意 这是
purely 一软件 concept, the underlying 硬件 用于 一PHY 一
remote PHY the exactly the 相同.

存在 concept 一SAS 端口 code, users 参见 什PHYs
form 一wide 端口 基于 the 端口_identifier attribute, the
相同 用于 全部 PHYs 一端口.

   :export:

#### SATA transport 绫。


The SATA transport handled libata, 具有 own book 
documentation 鍦，姝?directory.

#### 并行 SCSI (SPI) transport 


The 文件 驱动/SCSI/SCSI_transport_spi.c defines transport
attributes 用于 traditional (fast/wide/至尊 SCSI buses.

   :export:

#### SCSI RDMA (SRP) transport 绫。


The 文件 驱动/SCSI/SCSI_transport_srp.c defines transport
attributes 用于 SCSI 在…上 Remote Direct 内存 Access.

   :export:

## SCSI lower layer


### Host 总线 Adapter transport types


许多 modern 设备 控制使用 the SCSI 命令 set 作为 一协议 
communicate 它们设备 through 许多 不同 types 物理
connections.

SCSI language 一总线 capable carrying SCSI 命令 called 一
"transport", 一控制connecting 此类 一总线 called 一"host
总线 adapter" (HBA).

#### Debug transport


The 文件 驱动/SCSI/SCSI_debug.c simulates 一host adapter 一
variable 数字 disks (disk 类似 设备) attached, sharing 一
通用 amount RAM. 执行 一lot checking 确保 我们 
getting mixed up, panics the 内核 anything 超出
the ordinary 鏄?seen.

更多 realistic, the simulated 设备 具有 the transport
attributes 鐨?SAS disks.

用于 documentation 参见 http://sg.danny.cz/sg/scsi_debug.html

#### todo


并行 (fast/wide/至尊 SCSI, USB, SATA, SAS, Fibre Channel,
FireWire, ATAPI 设备, Infiniband, 并行 ports,
netlink...
