## 数据完整性（Data Integrity


## 1. 简


现代文件系统具备数据和元数据的校验和（checksumming）功能，以防止数据损坏。然而，
损坏的检测是在读取时进行的，而这距离数据写入可能已经过去了数月之久。到那时，应用程
试图写入的原始数据很可能已经丢失

解决办法是确保磁盘实际存储的就是应用程序想要它存储的内容。最近对 SCSI 系列协议
（SBC Data Integrity Field、SCC protection proposal）以SATA/T13（External Path
Protection）的补充，试图通过I/O 上追加完整性元数据（integrity metadata）来支持这一
点。完整性元数据（在 SCSI 术语中称为保护信息，protection information）包含每个扇区的
校验和，以及一个递增的计数器，用于确保各个扇区以正确的顺序写入。对于某些保护方案，它还
确保 I/O 被写入磁盘上正确的位置

当前的存储控制器和设备实现了各种保护措施，例如校验和与清洗（scrubbing）。但这些技
各自工作在彼此孤立的领域中，或者最多只I/O 路径上相邻的节点之间工作。DIF 和其
完整性扩展的有趣之处在于，其保护格式有明确定义，I/O 路径上的每个节点都可以验I/O 
完整性，并在检测到损坏时拒绝它。这不仅允许防止损坏，还能够隔离故障点

## 2. 数据完整性扩展（The Data Integrity Extensions


如前所述，协议扩展只保护控制器与存储设备之间的路径。然而，许多控制器实际上允许操作
系统访问完整性元数据（IMD）。我们一直在与多FC/SAS HBA 厂商合作，以使保护信息能
在其控制器之间传入和传出

SCSI 数据完整性字段（Data Integrity Field）通过为每个扇区追8 字节的保护信息来工作
数据 + 完整性元数据在磁盘上520 字节的扇区存储。在控制器与目标之间传输时，数据 + IMD
是交错存放的。T13 提案与之类似

由于操作系统处理 520（以4104）字节的扇区非常不便，我们联系了多家 HBA 厂商，并鼓励
它们允许将数据与完整性元数据的分聚集列表（scatter-gather lists）分离

控制器会在写入时交错缓冲，在读取时拆分它们。这意味着 Linux 可以将数据缓冲区在主存之
进行 DMA，而无需改动页缓存

此外，SCSI SATA 规范都强制要求的 16 CRC 校验和在软件中计算相当繁重。基准测试发现，
对于许多工作负载，计算该校验和对系统性能有显著影响。一些控制器允许在与操作系统交互
使用更轻量的校验和。例Emulex 支持改用 TCP/IP 校验和。从操作系统收到IP 校验和在
写入时会被转换为 16 CRC，反之亦然。这使得完整性元数据可以Linux 或应用程序以极低
代价生成（可与软RAID5 相比）

IP 校验和在检测位错误方面弱于 CRC。然而，其优势实际上在于将数据缓冲区与完整性元数据
分离。这两个不同的缓冲区必须匹配，一I/O 才能完成

数据与完整性元数据缓冲区的分离，以及对校验和的选择，被称为数据完整性扩展（Data
Integrity Extensions）。由于这些扩展超出了协议组织（T10、T13）的范围，Oracle 及其
合作伙伴正试图在存储网络工业协会（Storage Networking Industry Association）内将其
标准化

## 3. 内核改动


Linux 中的数据完整性框架允许将保护信息固定I/O 上，并发送给支持它的控制器或从控制器
接收

SCSI SATA 中完整性扩展的优点在于，它们使我们能够保护从应用程序到存储设备的整条路径
然而，与此同时这也是最大的缺点。它意味着保护信息必须采用磁盘能够理解的格式

通常，Linux/POSIX 应用程序对它们所访问的存储设备的细节一无所知。虚拟文件系统切换层
（virtual filesystem switch）和块层（block layer）使得硬件扇区大小、传输协议等对应
程序完全透明

然而，在准备要发送给磁盘的保护信息时，却需要这种级别的细节。因此，端到端（end-to-end
保护方案的概念本身就是一个违背分层原则（layering violation）的做法。让应用程序知道
访问的是 SCSI 还是 SATA 磁盘是完全不合理的

Linux 中实现的数据完整性支持试图对应用程序隐藏这一点。就应用程序（以及在某种程度
内核）而言，完整性元数据是附着I/O 上的不透明信息

当前的实现允许块层自动为任何 I/O 生成保护信息。最终的目标是将用户数据的完整性元数据
计算工作移入用户空间。源自内核内部的元数据和其他 I/O 仍将使用自动生成接口

某些存储设备允许为每个硬件扇区打上一16 位的值作为标签。这个标签空间的拥有者是
设备的拥有者。即在大多数情况下是文件系统。文件系统可以利用这块额外空间按自己的需
为扇区打标签。由于标签空间有限，块接口允许通过交错的方式为更大的块打标签。这样，一
典型4KB 文件系统块就可以附带 8*16 位的信息

这也意味着fsck mkfs 这样的应用程序需要访问权限，以便从用户空间操纵这些标签
相关的直通（passthrough）接口正在开发中

## 4. 块层实现细节


### 4.1 Bio


当启CONFIG_BLK_DEV_INTEGRITY 时，数据完整性补丁向 struct bio 添加了一个新字段
bio_integrity(bio) 返回一个指struct bip 的指针，其中包含bio 的完整性载荷
本质上，bip 是一个精简版的 struct bio，它持有一个包含完整性元数据bio_vec 以及所需
管理信息（bvec pool、vector count 等）

内核子系统可以通过调用 bio_integrity_alloc(bio) bio 上启用数据完整性保护。这将分
并把 bip 挂接bio 上

随后可以使用 bio_integrity_add_page() 来附加包含完整性元数据的各个页面

bio_free() 会自动释bip

### 4.2 块设


块设备可以在 queue_limits 结构integrity 子结构中设置完整性信息

分层的块设备需要选择一个适合所有子设备profile。queue_limits_stack_integrity() 可以
提供帮助。目前支DM MD linear、RAID0 以及 RAID1。RAID4/5/6 由于应用标签
（application tag）的原因还需要额外的工作

## 5.0 块层完整API


### 5.1 普通文件系


    普通文件系统并不知道底层块设备能够发接收完整性元数据。在 WRITE 情况下，IMD 将由
    块层submit_bio() 时自动生成。READ 请求会导I/O 完整性在完成时被验证

```

      /sys/block/<bdev>/integrity/write_generate

    and::

      /sys/block/<bdev>/integrity/read_verify

    标志

```
### 5.2 感知完整性的文件系统


    一个感知完整性的文件系统可以准备附带 IMD I/O。如果块设备支持，它也可以使用应
    标签空间

    `bool bio_integrity_prep(bio);`

      要为 WRITE 生成 IMD，并READ 设置缓冲区，文件系统必须调用
      bio_integrity_prep(bio)銆。

      在调用此函数之前，必须设bio 的数据方向和起始扇区，并bio 应已添加所有数据页
      调用方有责任确保 bio I/O 进行期间不会发生变化。如果由于某种原因准备工作失败，
      则以错误完成 bio

### 5.3 传递已有的完整性元数据


    要么自己生成完整性元数据、要么能够从用户空间传输 IMD 的文件系统，可以使用以下调用

    `struct bip * bio_integrity_alloc(bio, gfp_mask, nr_pages);`

      分配 bio 完整性载荷并将其挂到 bio 上。nr_pages 指明需要在完整bio_vec 列表
      存储多少页保护数据（类似bio_alloc()）

      完整性载荷将bio_free() 时被释放

    `int bio_integrity_add_page(bio, page, len, offset);`

      将一个包含完整性元数据的页面附加到已有bio 上。该 bio 必须已有 bip，即必须
      调用bio_integrity_alloc()。对WRITE，页面中的完整性元数据必须采用目标设备
      能够理解的格式，但有一个显著的例外：当请求穿越 I/O 栈时，扇区号会被重映射。这
      意味着通过此调用添加的页面会在 I/O 期间被修改！完整性元数据中的第一个引用标
      其值必须为 bip->bip_sector

      只要 bip bio_vec 数组（nr_pages）中还有空间，就可以使用
      bio_integrity_add_page() 继续添加页面

      READ 操作完成时，附加的页面将包含从存储设备收到的完整性元数据。由接收方负
      处理它们并在完成时验证数据完整性

----------------------------------------------------------------------

2007-12-24 Martin K. Petersen <martin.petersen@oracle.com>
