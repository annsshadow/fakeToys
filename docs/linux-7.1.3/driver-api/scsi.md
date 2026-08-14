## SCSI Interfaces Guide


:Author: James Bottomley
:Author: Rob Landley

## Introduction


### 协议 对比 总线


一旦 upon 一个 time, the Small Computer 系统 接口 定义 两者 一个
并行 I/O 总线 和 一个 数据 协议 到 connect 一个 wide variety 的
peripherals (disk drives, tape drives, modems, printers, scanners,
optical drives, test equipment, 和 medical 设备) 到 一个 host computer.

尽管 the 旧 并行 (fast/wide/至尊版) SCSI 总线 具有 largely fallen
超出 使用, the SCSI 命令 set 是 更多 widely 使用 比 ever 到
communicate 与 设备 在…上 一个 数字 的 不同 buses.

The `SCSI protocol <https://www.t10.org/scsi-3.htm>`__ 是 一个 big-endian
peer-to-peer 数据包 based 协议. SCSI 命令 是 6, 10, 12, 或 16
bytes long, 通常 followed 由 一个 associated 数据 payload.

SCSI 命令 可 为 transported 在…上 just 关于 任何 kind 的 总线, 和
是 the 默认 协议 用于 storage 设备 attached 到 USB, SATA, SAS,
Fibre Channel, FireWire, 和 ATAPI 设备. SCSI packets 是 也
commonly exchanged 在…上 Infiniband,
TCP/IP (`iSCSI <https://en.wikipedia.org/wiki/ISCSI>`__), even `并行
ports <http://cyberelk.net/tim/parport/parscsi.html>`__.

### Design 的 the Linux SCSI 子系统


The SCSI 子系统 uses 一个 three layer design, 与 upper, mid, 和 low
layers. Every 操作 involving the SCSI 子系统 (例如 reading 一个
扇区 来自 一个 disk) uses one 驱动 在 每个 的 the 3 levels: one upper
layer 驱动, one lower layer 驱动, 和 the SCSI midlayer.

The SCSI upper layer 提供 the 接口 之间 userspace 和 the
内核, 在 the form 的 块 和 char 设备 nodes 用于 I/O 和 ioctl().
The SCSI lower layer 包含 驱动 用于 特定 硬件 设备.

在 之间 是 the SCSI mid-layer, analogous 到 一个 网络 routing layer
例如 the IPv4 栈. The SCSI mid-layer routes 一个 数据包 based 数据
协议 之间 the upper layer's /dev nodes 和 the corresponding
设备 在 the lower layer. 它 manages 命令 queues, 提供 错误
handling 和 电源管理 函数, 和 responds 到 ioctl()
requests.

## SCSI upper layer


The upper layer supports the user-kernel 接口 由 providing 设备
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
函数, 用于 使用 与 HDIO_GETGEO, 等.

   :export:

#### 驱动/SCSI/SCSI_错误.c


通用 SCSI 错误/超时 handling routines.

   :export:

#### 驱动/SCSI/SCSI_devinfo.c


Manage SCSI_dev_info_列出, 其 tracks blacklisted 和 whitelisted
设备.

   :export:

#### 驱动/SCSI/SCSI_ioctl.c


Handle ioctl() calls 用于 SCSI 设备.

   :export:

#### 驱动/SCSI/SCSI_lib.c


SCSI queuing 库.

   :export:

#### 驱动/SCSI/SCSI_lib_dma.c


SCSI 库 函数 depending 在 DMA (map 和 unmap scatter-gather
列表).

   :export:

#### 驱动/SCSI/SCSI_proc.c


The 函数 在 此 文件 提供 一个 接口 之间 the PROC 文件
系统 和 the SCSI 设备 驱动 它是 mainly 使用 用于 debugging,
statistics 和 到 pass information directly 到 the lowlevel 驱动. I.E.
plumbing 到 manage /proc/SCSI/\*


#### 驱动/SCSI/SCSI_netlink.c


Infrastructure 到 提供 async 事件 来自 transports 到 userspace 通过
netlink, 使用 一个 单个 NETLINK_SCSITRANSPORT 协议 用于 全部
transports. 参见 `the original patch submission
<https://lore.kernel.org/linux-scsi/1155070439.6275.5.camel@localhost.localdomain/>`__
用于 更多 details.

   :internal:

#### 驱动/SCSI/SCSI_scan.c


Scan 一个 host 到 determine 其 (若 任何) 设备 是 attached. The
通用 scanning/probing algorithm 是 作为 follows, exceptions 是 made 到
它 depending 在 设备 特定 标志, compilation 选项, 和 全局
variable (boot 或 模块 加载 time) 设置. 一个 特定 LUN 是 scanned
通过 一个 INQUIRY 命令; 若 the LUN 具有 一个 设备 attached, 一个 SCSI_设备
是 allocated 和 setup 用于 它. 用于 every id 的 every channel 在 the
given host, 启动 由 scanning LUN 0. Skip hosts 该 don't respond 在
全部 到 一个 scan 的 LUN 0. 否则, 若 LUN 0 具有 一个 设备 attached,
allocate 和 setup 一个 SCSI_设备 用于 它. 若 target 是 SCSI-3 或 up,
issue 一个 REPORT LUN, 和 scan 全部 的 the LUNs returned 由 the REPORT LUN;
else, sequentially scan LUNs up 直到 一些 最大 是 reached, 或 一个 LUN
是 seen 该 cannot 具有 一个 设备 attached 到 它.

   :export:

#### 驱动/SCSI/SCSI_sysctl.c


Set up the sysctl 条目: "/dev/SCSI/logging_level"
(DEV_SCSI_LOGGING_LEVEL) 其 sets/returns SCSI_logging_level.

#### 驱动/SCSI/SCSI_sysfs.c


SCSI sysfs 接口 routines.

   :export:

#### 驱动/SCSI/hosts.c


mid 到 lowlevel SCSI 驱动 接口

   :export:

#### 驱动/SCSI/SCSI_通用.c


通用 支持 函数

   :export:

### Transport classes


Transport classes 是 service 库 用于 驱动 在 the SCSI lower
layer, 其 expose transport attributes 在 sysfs.

#### Fibre Channel transport


The 文件 驱动/SCSI/SCSI_transport_fc.c defines transport attributes
用于 Fibre Channel.

   :export:

#### iSCSI transport 类


The 文件 驱动/SCSI/SCSI_transport_iscsi.c defines transport
attributes 用于 the iSCSI 类, 其 sends SCSI packets 在…上 TCP/IP
connections.

   :export:

#### 串行 Attached SCSI (SAS) transport 类


The 文件 驱动/SCSI/SCSI_transport_sas.c defines transport
attributes 用于 串行 Attached SCSI, 一个 variant 的 SATA aimed 在 large
high-end 系统.

The SAS transport 类 包含 通用 code 到 deal 与 SAS HBAs, 一个
approximated representation 的 SAS topologies 在 the 驱动 型号, 和
各种 sysfs attributes 到 expose 这些 topologies 和 管理
interfaces 到 userspace.

此外 到 the 基本 SCSI 核心 objects 此 transport 类
introduces two 额外 intermediate objects: The SAS PHY 作为
represented 由 结构体 sas_phy defines 一个 "outgoing" PHY 在 一个 SAS HBA 或
Expander, 和 the SAS remote PHY represented 由 结构体 sas_rphy defines
一个 "incoming" PHY 在 一个 SAS Expander 或 end 设备. 注意 该 这是
purely 一个 软件 concept, the underlying 硬件 用于 一个 PHY 和 一个
remote PHY 是 the exactly the 相同.

存在 无 concept 的 一个 SAS 端口 在 此 code, users 可 参见 什么 PHYs
form 一个 wide 端口 基于 the 端口_identifier attribute, 其 是 the
相同 用于 全部 PHYs 在 一个 端口.

   :export:

#### SATA transport 类


The SATA transport 是 handled 由 libata, 其 具有 其 own book 的
documentation 在 此 directory.

#### 并行 SCSI (SPI) transport 类


The 文件 驱动/SCSI/SCSI_transport_spi.c defines transport
attributes 用于 traditional (fast/wide/至尊版) SCSI buses.

   :export:

#### SCSI RDMA (SRP) transport 类


The 文件 驱动/SCSI/SCSI_transport_srp.c defines transport
attributes 用于 SCSI 在…上 Remote Direct 内存 Access.

   :export:

## SCSI lower layer


### Host 总线 Adapter transport types


许多 modern 设备 控制器 使用 the SCSI 命令 set 作为 一个 协议 到
communicate 与 它们的 设备 through 许多 不同 types 的 物理
connections.

在 SCSI language 一个 总线 capable 的 carrying SCSI 命令 是 called 一个
"transport", 和 一个 控制器 connecting 到 此类 一个 总线 是 called 一个 "host
总线 adapter" (HBA).

#### Debug transport


The 文件 驱动/SCSI/SCSI_debug.c simulates 一个 host adapter 与 一个
variable 数字 的 disks (或 disk 类似 设备) attached, sharing 一个
通用 amount 的 RAM. 执行 一个 lot 的 checking 到 确保 该 我们 是
不 getting 块 mixed up, 和 panics the 内核 若 anything 超出
the ordinary 是 seen.

到 为 更多 realistic, the simulated 设备 具有 the transport
attributes 的 SAS disks.

用于 documentation 参见 http://sg.danny.cz/sg/scsi_debug.html

#### todo


并行 (fast/wide/至尊版) SCSI, USB, SATA, SAS, Fibre Channel,
FireWire, ATAPI 设备, Infiniband, 并行 ports,
netlink...
