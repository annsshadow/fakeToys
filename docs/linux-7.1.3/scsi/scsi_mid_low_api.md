锘。
## SCSI 中间- 底层驱动接口


## 简

本文档概述了 Linux SCSI 中间层（mid level）与 SCSI 底层驱动（lower level driver）之间的接口。底层驱动（LLD）也被称为主机总线适配器（HBA）驱动和主机驱动（HD）。在此语境下主机（host是计算机 IO 总线（例PCI ISA）与 SCSI 传输层上单个 SCSI 发起者端口之间的桥。发起者（"initiator"）端口（SCSI 术语，参SAM-3，网址 http://www.t10.org）向"目标（targetSCSI 端口（例如磁盘）发SCSI 命令。在一个运行中的系统中可以存在许多 LLD，但每种硬件类型只能有一个。大多数 LLD 可以控制一个或多个 SCSI HBA。某HBA 包含多个主机

在某些情况下，SCSI 传输层是一条在 Linux 中已经拥有自身子系统的外部总线（例USB ieee1394）。在这种情况下，SCSI 子系统的 LLD 是通往另一个驱动子系统的软件桥。例子有 usb-storage 驱动（位drivers/usb/storage 目录）以ieee1394/sbp2 驱动（位drivers/ieee1394 目录）

例如，aic7xxx LLD 控制基于该公7xxx 系列芯片Adaptec SCSI 并行接口（SPI）控制器。aic7xxx LLD 可以编译进内核或作为模块加载。一Linux 系统中只能有一aic7xxx LLD 在运行，但它可能控制许多 HBA。这HBA 可能位于 PCI 子卡上，或集成在主板上（或两者兼有）。某些基aic7xxx HBA 是双控制器，因此代表两个主机。像大多数现HBA 一样，每个 aic7xxx 主机都有自己PCI 设备地址。[SCSI 主机PCI 设备之间的一一对应关系很常见，但并非必需（例ISA 适配器）。]

SCSI 中间层将 LLD SCSI 上层驱动和块层等其他层隔离开来。本文档的此版本大致对应 Linux 内核版本 2.6.8


## 文档

内核源码树中包含一SCSI 文档目录，通常Documentation/scsi。大多数文档采用 reStructuredText 格式。本文件名为 scsi_mid_low_api.rst，可在该目录中找到。本文档较新的副本可https://docs.kernel.org/scsi/scsi_mid_low_api.html 找到。许LLD Documentation/scsi 中有文档（例aic7xxx.rst）。SCSI 中间层在 scsi.rst 中有简要说明，其中包含描述 Linux 内核 2.4 系列 SCSI 子系统的文档URL。该目录中有两份上层驱动的文档：st.rst（SCSI 磁带驱动）和 scsi-generic.rst（针sg 驱动）

某些 LLD 的文档（URL）可以在 C 源码中找到，或者在C 源码相同的目录中找到。例如，要找到关USB 大容量存储驱动的 URL，请查看 /usr/src/linux/drivers/usb/storage 目录


## 驱动结构

传统上，SCSI 子系统的 LLD drivers/scsi 目录中至少有两份文件。例如，名为 "xyz" 的驱动有一个头文件 "xyz.h" 和一个源文件 "xyz.c"。[实际上没有充分的理由不能把所有内容放在一个文件中；头文件是多余的。]一些已移植到多个操作系统的驱动有超过两份文件。例aic7xxx 驱动有为通用代码和特定于操作系统的代码（例如 FreeBSD Linux）分别准备的独立文件。这类驱动往往drivers/scsi 目录下拥有自己的子目录

Linux 添加一个新LLD 时，以下文件（位drivers/scsi 目录中）需要加以注意：Makefile Kconfig。最好是研究现有 LLD 是如何组织的

随着 2.5 系列开发内核演进为 2.6 系列生产内核，此接口也在发生变化。其中一个例子就是驱动初始化代码，现在有两种模型可用。较旧的模型类似Linux 2.4 系列中的做法，基于在 HBA 驱动加载时检测到的主机。这被称被动（passive初始化模型。较新的模型允许LLD 的生命周期内热插拔（以及热拔）HBA，被称为"热插拔（hotplug初始化模型。较新的模型更受青睐，因为它既能处理永久连接的传SCSI 设备，也能处理热插拔的现SCSI"设备（例如通过 USB IEEE 1394 连接的数码相机）。两种初始化模型将在后续各节中讨论

LLD 通过以下几种方式SCSI 子系统交互：

  a) 直接调用中间层提供的函数
  b) 向中间层提供的注册函数传入一组函数指针。中间层随后会在将来的某个时刻调用这些函数。LLD 需要提供这些函数的实现
  c) 直接访问由中间层维护的知名数据结构实

a) 组中的函数在下文名为"中间层提供的函数"的小节中列出

b) 组中的函数在下文名为"接口函数"的小节中列出。它们的函数指针被放置到 "struct scsi_host_template" 的成员中，该结构的一个实例会被传scsi_host_alloc()。对于那LLD 不希望提供的接口函数，应struct scsi_host_template 的相应成员中填入 NULL。在文件作用域定struct scsi_host_template 实例会导致未显式初始化的函数指针成员被填NULL

c) 组中的用法应当谨慎处理，尤其是在"热插环境中。LLD 应当了解与中间层和其他层共享的实例的生命周期

LLD 内定义的所有函数以及文件作用域定义的所有数据都应为 static。例如，名为 "xxx" LLD 中的 sdev_init() 函数可以定义
`static int xxx_sdev_init(struct scsi_device ** sdev) { /** code */ }`


## 热插拔初始化模型

在此模型中，LLD 控制着 SCSI 主机何时被引入和SCSI 子系统移除。主机最早可以在驱动初始化时引入，最晚可以在驱动关闭时移除。通常，驱动会响应一sysfs probe() 回调，该回调表示检测到一HBA。在确认新设备是 LLD 想要控制的设备后，LLD 会初始化HBA，然后向 SCSI 中间层注册一个新主机

LLD 初始化期间，驱动应当向它所期望找到 HBA 的相IO 总线（例PCI 总线）注册自身。这大概可以通过 sysfs 完成。任何驱动参数（尤其是那些在驱动加载后仍可写的参数）也可以在这一步通过 sysfs 注册。SCSI 中间层是LLD 注册其第一HBA 时才首次得知LLD 的存在

在稍后的某个时刻，LLD 得知一HBA，接下来LLD 与中间层之间典型的调用序列。此示例展示了中间层为新引入HBA 扫描3

```
	HBA PROBE: assume 2 SCSI devices found in scan
    LLD                   mid level                    LLD
    ===-------------------=========--------------------===------
    scsi_host_alloc()  -->
    scsi_add_host()  ---->
    scsi_scan_host()  -------+
			    |
			sdev_init()
			sdev_configure() -->  scsi_change_queue_depth()
			    |
			sdev_init()
			sdev_configure()
			    |
			sdev_init()   ***
			sdev_destroy() ***


    *** For scsi devices that the mid level tries to scan but do not
	respond, a sdev_init(), sdev_destroy() pair is called.

```

如果 LLD 想调整默认队列设置，可以在其 sdev_configure() 例程中调scsi_change_queue_depth()

HBA 被移除时，这可能是与 LLD 模块被卸载（例如使用 "rmmod" 命令）相关的有序关闭的一部分，也可能是响sysfs remove() 回调被调用所表示热拔"。无论哪种情况，序列都是

```
	    HBA REMOVE: assume 2 SCSI devices attached
    LLD                      mid level                 LLD
    ===----------------------=========-----------------===------
    scsi_remove_host() ---------+
				|
			sdev_destroy()
			sdev_destroy()
    scsi_host_put()

```

LLD 跟踪 struct Scsi_Host 实例（指针由 scsi_host_alloc() 返回）可能是有用的。此类实例由中间拥有"。当引用计数降为零时，struct Scsi_Host 实例会在 scsi_host_put() 中被释放

热拔一个控制着在处理已挂载文件系统上的 SCSI 命令的磁盘的 HBA，是一种有趣的情形。中间层正在引入引用计数逻辑来应对所涉及的许多问题。请参阅下文关于引用计数的小节


热插拔的概念可以扩展SCSI 设备。当前，当添加一HBA 时，scsi_scan_host() 函数会触发对连接到该 HBA SCSI 传输层的 SCSI 设备扫描。在较新SCSI 传输层上，HBA 可能在扫描完成_之后_才得知一个新SCSI 设备

```
		    SCSI DEVICE hotplug
    LLD                   mid level                    LLD
    ===-------------------=========--------------------===------
    scsi_add_device()  ------+
			    |
			sdev_init()
			sdev_configure()   [--> scsi_change_queue_depth()]

```

类似地，LLD 可能会得知一SCSI 设备已被移除（拔出），或者到它的连接已被中断。一些现有的 SCSI 传输层（例如 SPI）可能直到后SCSI 命令失败才会得知 SCSI 设备已被移除，而该命令失败很可能会导致中间层将该设备置为离线。检测到 SCSI 设备被移除的 LLD 可以主动将其

```
		    SCSI DEVICE hot unplug
    LLD                      mid level                 LLD
    ===----------------------=========-----------------===------
    scsi_remove_device() -------+
				|
			sdev_destroy()

```

LLD 跟踪 struct scsi_device 实例（指针作sdev_init() sdev_configure() 回调的参数传入）可能是有用的。此类实例由中间拥有"。struct scsi_device 实例会在 sdev_destroy() 之后被释放


## 引用计数

Scsi_Host 结构已经添加了引用计数基础设施。这实际上将 struct Scsi_Host 实例的所有权分散到使用它们的各个 SCSI 层。此前此类实例完全由中间层拥有。LLD 通常不需要直接操作这些引用计数，但在某些情况下可能需要

struct Scsi_Host 相关的、值得关注的引用计数函数有 3 个：

  - scsi_host_alloc()锛。
	返回一个指向新 struct Scsi_Host 实例的指针，其引用计^^ 被设1

  - scsi_host_get()锛。
	将给定实例的引用计数1

  - scsi_host_put()锛。
	将给定实例的引用计数1。如果引用计数达0，则释放该实

scsi_device 结构已经添加了引用计数基础设施。这实际上将 struct scsi_device 实例的所有权分散到使用它们的各个 SCSI 层。此前此类实例完全由中间层拥有。请参阅 include/scsi/scsi_device.h 末尾声明的访问函数。如LLD 想保留一个指scsi_device 实例的指针副本，它应当使scsi_device_get() 来增加其引用计数。当不再需要该指针时，可以使用 scsi_device_put() 来减少其引用计数（并可能将其删除）


   struct Scsi_Host 实际上有 2 个引用计数，由这些函数并行操作


## 约定

首先，Linus Torvalds 关于 C 编码风格的看法可以在 Documentation/process/coding-style.rst 文件中找到

此外，在大多数相gcc 编译器支持的程度上鼓励使C99 增强特性。因此，在适当的地方鼓励使C99 风格的结构和数组初始化器。但不要太过分，变长数组（VLA）尚未得到妥善支持。对此的一个例外是 `//` 风格的注释；Linux 中仍然更偏好 `/**...**/` 风格的注释

编写良好、经过测试且有文档的代码，无需为符合上述约定而重新格式化。例如，aic7xxx 驱动是从 FreeBSD Adaptec 自己的实验室来到 Linux 的。毫无疑问，FreeBSD Adaptec 有它们自己的编码约定


## 中间层提供的函数

这些函数SCSI 中间层提供，LLD 使用。这些函数的名称（即入口点）被导出，因此作为模块LLD 可以访问它们。内核会安排在任LLD 初始化之前加载并初始SCSI 中间层。以下函数按字母顺序列出，它们的名称都以 `scsi_` 开头

摘要

  - scsi_add_device - 创建一个新scsi 设备（lu）实
  - scsi_add_host - 执行 sysfs 注册并设置传输类
  - scsi_change_queue_depth - 更改 SCSI 设备上的队列深度
  - scsi_bios_ptable - 返回块设备分区表的副
  - scsi_block_requests - 阻止向给定主机排入更多命
  - scsi_host_alloc - 返回一refcount==1 的新 scsi_host 实例
  - scsi_host_get - 递增 Scsi_Host 实例的引用计
  - scsi_host_put - 递减 Scsi_Host 实例的引用计数（若为 0 则释放）
  - scsi_remove_device - 分离并移除一SCSI 设备
  - scsi_remove_host - 分离并移除主机拥有的所SCSI 设备
  - scsi_report_bus_reset - 报告观察到的 scsi _总线_ 复位
  - scsi_scan_host - 扫描 SCSI 总线
  - scsi_track_queue_full - 跟踪连续QUEUE_FULL 事件
  - scsi_unblock_requests - 允许向给定主机排入更多命


```

    /**
    * scsi_add_device - creates new scsi device (lu) instance
    * @shost:   pointer to scsi host instance
    * @channel: channel number (rarely other than 0)
    * @id:      target id number
    * @lun:     logical unit number
    *
    *      Returns pointer to new struct scsi_device instance or
    *      ERR_PTR(-ENODEV) (or some other bent pointer) if something is
    *      wrong (e.g. no lu responds at given address)
    *
    *      Might block: yes
    *
    *      Notes: This call is usually performed internally during a scsi
    *      bus scan when an HBA is added (i.e. scsi_scan_host()). So it
    *      should only be called if the HBA becomes aware of a new scsi
    *      device (lu) after scsi_scan_host() has completed. If successful
    *      this call can lead to sdev_init() and sdev_configure() callbacks
    *      into the LLD.
    *
    *      Defined in: drivers/scsi/scsi_scan.c
    **/
    struct scsi_device * scsi_add_device(struct Scsi_Host *shost,
					unsigned int channel,
					unsigned int id, unsigned int lun)


    /**
    * scsi_add_host - perform sysfs registration and set up transport class
    * @shost:   pointer to scsi host instance
    * @dev:     pointer to struct device of type scsi class
    *
    *      Returns 0 on success, negative errno of failure (e.g. -ENOMEM)
    *
    *      Might block: no
    *
    *      Notes: Only required in "hotplug initialization model" after a
    *      successful call to scsi_host_alloc().  This function does not
    *	scan the bus; this can be done by calling scsi_scan_host() or
    *	in some other transport-specific way.  The LLD must set up
    *	the transport template before calling this function and may only
    *	access the transport class data after this function has been called.
    *
    *      Defined in: drivers/scsi/hosts.c
    **/
    int scsi_add_host(struct Scsi_Host *shost, struct device * dev)


    /**
    * scsi_change_queue_depth - allow LLD to change queue depth on a SCSI device
    * @sdev:       pointer to SCSI device to change queue depth on
    * @tags        Number of tags allowed if tagged queuing enabled,
    *              or number of commands the LLD can queue up
    *              in non-tagged mode (as per cmd_per_lun).
    *
    *      Returns nothing
    *
    *      Might block: no
    *
    *      Notes: Can be invoked any time on a SCSI device controlled by this
    *      LLD. [Specifically during and after sdev_configure() and prior to
    *      sdev_destroy().] Can safely be invoked from interrupt code.
    *
    *      Defined in: drivers/scsi/scsi.c [see source code for more notes]
    *
    **/
    int scsi_change_queue_depth(struct scsi_device *sdev, int tags)


    /**
    * scsi_bios_ptable - return copy of block device's partition table
    * @dev:        pointer to gendisk
    *
    *      Returns pointer to partition table, or NULL for failure
    *
    *      Might block: yes
    *
    *      Notes: Caller owns memory returned (free with kfree() )
    *
    *      Defined in: drivers/scsi/scsicam.c
    **/
    unsigned char *scsi_bios_ptable(struct gendisk *dev)


    /**
    * scsi_block_requests - prevent further commands being queued to given host
    *
    * @shost: pointer to host to block commands on
    *
    *      Returns nothing
    *
    *      Might block: no
    *
    *      Notes: There is no timer nor any other means by which the requests
    *      get unblocked other than the LLD calling scsi_unblock_requests().
    *
    *      Defined in: drivers/scsi/scsi_lib.c
    **/
    void scsi_block_requests(struct Scsi_Host * shost)


    /**
    * scsi_host_alloc - create a scsi host adapter instance and perform basic
    *                   initialization.
    * @sht:        pointer to scsi host template
    * @privsize:   extra bytes to allocate in hostdata array (which is the
    *              last member of the returned Scsi_Host instance)
    *
    *      Returns pointer to new Scsi_Host instance or NULL on failure
    *
    *      Might block: yes
    *
    *      Notes: When this call returns to the LLD, the SCSI bus scan on
    *      this host has _not_ yet been done.
    *      The hostdata array (by default zero length) is a per host scratch
    *      area for the LLD's exclusive use.
    *      Both associated refcounting objects have their refcount set to 1.
    *      Full registration (in sysfs) and a bus scan are performed later when
    *      scsi_add_host() and scsi_scan_host() are called.
    *
    *      Defined in: drivers/scsi/hosts.c .
    **/
    struct Scsi_Host * scsi_host_alloc(const struct scsi_host_template * sht,
				    int privsize)


    /**
    * scsi_host_get - increment Scsi_Host instance refcount
    * @shost:   pointer to struct Scsi_Host instance
    *
    *      Returns nothing
    *
    *      Might block: currently may block but may be changed to not block
    *
    *      Notes: Actually increments the counts in two sub-objects
    *
    *      Defined in: drivers/scsi/hosts.c
    **/
    void scsi_host_get(struct Scsi_Host *shost)


    /**
    * scsi_host_put - decrement Scsi_Host instance refcount, free if 0
    * @shost:   pointer to struct Scsi_Host instance
    *
    *      Returns nothing
    *
    *      Might block: currently may block but may be changed to not block
    *
    *      Notes: Actually decrements the counts in two sub-objects. If the
    *      latter refcount reaches 0, the Scsi_Host instance is freed.
    *      The LLD need not worry exactly when the Scsi_Host instance is
    *      freed, it just shouldn't access the instance after it has balanced
    *      out its refcount usage.
    *
    *      Defined in: drivers/scsi/hosts.c
    **/
    void scsi_host_put(struct Scsi_Host *shost)


    /**
    * scsi_remove_device - detach and remove a SCSI device
    * @sdev:      a pointer to a scsi device instance
    *
    *      Returns value: 0 on success, -EINVAL if device not attached
    *
    *      Might block: yes
    *
    *      Notes: If an LLD becomes aware that a scsi device (lu) has
    *      been removed but its host is still present then it can request
    *      the removal of that scsi device. If successful this call will
    *      lead to the sdev_destroy() callback being invoked. sdev is an
    *      invalid pointer after this call.
    *
    *      Defined in: drivers/scsi/scsi_sysfs.c .
    **/
    int scsi_remove_device(struct scsi_device *sdev)


    /**
    * scsi_remove_host - detach and remove all SCSI devices owned by host
    * @shost:      a pointer to a scsi host instance
    *
    *      Returns value: 0 on success, 1 on failure (e.g. LLD busy ?锛?
    *
    *      Might block: yes
    *
    *      Notes: Should only be invoked if the "hotplug initialization
    *      model" is being used. It should be called _prior_ to
    *      calling scsi_host_put().
    *
    *      Defined in: drivers/scsi/hosts.c .
    **/
    int scsi_remove_host(struct Scsi_Host *shost)


    /**
    * scsi_report_bus_reset - report scsi _bus_ reset observed
    * @shost: a pointer to a scsi host involved
    * @channel: channel (within) host on which scsi bus reset occurred
    *
    *      Returns nothing
    *
    *      Might block: no
    *
    *      Notes: This only needs to be called if the reset is one which
    *      originates from an unknown location.  Resets originated by the
    *      mid level itself don't need to call this, but there should be
    *      no harm.  The main purpose of this is to make sure that a
    *      CHECK_CONDITION is properly treated.
    *
    *      Defined in: drivers/scsi/scsi_error.c .
    **/
    void scsi_report_bus_reset(struct Scsi_Host * shost, int channel)


    /**
    * scsi_scan_host - scan SCSI bus
    * @shost: a pointer to a scsi host instance
    *
    *	Might block: yes
    *
    *	Notes: Should be called after scsi_add_host()
    *
    *	Defined in: drivers/scsi/scsi_scan.c
    **/
    void scsi_scan_host(struct Scsi_Host *shost)


    /**
    * scsi_track_queue_full - track successive QUEUE_FULL events on given
    *                      device to determine if and when there is a need
    *                      to adjust the queue depth on the device.
    * @sdev:  pointer to SCSI device instance
    * @depth: Current number of outstanding SCSI commands on this device,
    *         not counting the one returned as QUEUE_FULL.
    *
    *      Returns 0  - no change needed
    *              >0 - adjust queue depth to this new depth
    *              -1 - drop back to untagged operation using host->cmd_per_lun
    *                   as the untagged command depth
    *
    *      Might block: no
    *
    *      Notes: LLDs may call this at any time and we will do "The Right
    *              Thing"; interrupt context safe.
    *
    *      Defined in: drivers/scsi/scsi.c .
    **/
    int scsi_track_queue_full(struct scsi_device *sdev, int depth)


    /**
    * scsi_unblock_requests - allow further commands to be queued to given host
    *
    * @shost: pointer to host to unblock commands on
    *
    *      Returns nothing
    *
    *      Might block: no
    *
    *      Defined in: drivers/scsi/scsi_lib.c .
    **/
    void scsi_unblock_requests(struct Scsi_Host * shost)



```
## 接口函数

接口函数LLD 提供（定义），它们的函数指针被放置到 struct scsi_host_template 的一个实例中，该实例会被传入 scsi_host_alloc()。其中一些是必需的。接口函数应声明static。公认的约定是，驱动 "xyz" 会声明它sdev_configure()

```
    static int xyz_sdev_configure(struct scsi_device * sdev);

```

下文列出的所有接口函数以此类推。指向该函数的指针应被放"struct scsi_host_template" 实例'sdev_configure' 成员中。指向此类实例的指针应被传入中间层的 scsi_host_alloc()

接口函数也在 include/scsi/scsi_host.h 文件中、位"struct scsi_host_template" 中它们定义点的上方有描述。在某些情况下，scsi_host.h 中给出的细节比下文更多

接口函数按字母顺序列在下方

摘要

  - bios_param - 获取磁盘的磁头、扇区、柱面信
  - eh_timed_out - 通知主机某个命令的定时器已超
  - eh_abort_handler - 中止给定的命
  - eh_bus_reset_handler - 发起 SCSI 总线复位
  - eh_device_reset_handler - 发起 SCSI 设备复位
  - eh_host_reset_handler - 复位主机（主机总线适配器）
  - info - 提供关于给定主机的信
  - ioctl - 驱动可以响应 ioctl
  - proc_info - 支持 /proc/scsi/{driver_name}/{host_no}
  - queuecommand - scsi 命令入队，完成时调用 'done'
  - sdev_init - 在向新设备发送任何命令之
  - sdev_configure - 设备连接后针对给定设备的驱动微调
  - sdev_destroy - 给定设备即将关闭


```

    /**
    *      bios_param - fetch head, sector, cylinder info for a disk
    *      @sdev: pointer to scsi device context (defined in
    *             include/scsi/scsi_device.h)
    *      @disk: pointer to gendisk (defined in blkdev.h)
    *      @capacity:  device size (in 512 byte sectors)
    *      @params: three element array to place output:
    *              params[0] number of heads (max 255)
    *              params[1] number of sectors (max 63)
    *              params[2] number of cylinders
    *
    *      Return value is ignored
    *
    *      Locks: none
    *
    *      Calling context: process (sd)
    *
    *      Notes: an arbitrary geometry (based on READ CAPACITY) is used
    *      if this function is not provided. The params array is
    *      pre-initialized with made up values just in case this function
    *      doesn't output anything.
    *
    *      Optionally defined in: LLD
    **/
	int bios_param(struct scsi_device * sdev, struct gendisk *disk,
		    sector_t capacity, int params[3])


    /**
    *      eh_timed_out - The timer for the command has just fired
    *      @scp: identifies command timing out
    *
    *      Returns:
    *
    *      EH_HANDLED:             I fixed the error, please complete the command
    *      EH_RESET_TIMER:         I need more time, reset the timer and
    *                              begin counting again
    *      EH_NOT_HANDLED          Begin normal error recovery
    *
    *
    *      Locks: None held
    *
    *      Calling context: interrupt
    *
    *      Notes: This is to give the LLD an opportunity to do local recovery.
    *      This recovery is limited to determining if the outstanding command
    *      will ever complete.  You may not abort and restart the command from
    *      this callback.
    *
    *      Optionally defined in: LLD
    **/
	int eh_timed_out(struct scsi_cmnd * scp)


    /**
    *      eh_abort_handler - abort command associated with scp
    *      @scp: identifies command to be aborted
    *
    *      Returns SUCCESS if command aborted else FAILED
    *
    *      Locks: None held
    *
    *      Calling context: kernel thread
    *
    *      Notes: This is called only for a command that has timed out.
    *
    *      Optionally defined in: LLD
    **/
	int eh_abort_handler(struct scsi_cmnd * scp)


    /**
    *      eh_bus_reset_handler - issue SCSI bus reset
    *      @scp: SCSI bus that contains this device should be reset
    *
    *      Returns SUCCESS if command aborted else FAILED
    *
    *      Locks: None held
    *
    *      Calling context: kernel thread
    *
    *      Notes: Invoked from scsi_eh thread. No other commands will be
    *      queued on current host during eh.
    *
    *      Optionally defined in: LLD
    **/
	int eh_bus_reset_handler(struct scsi_cmnd * scp)


    /**
    *      eh_device_reset_handler - issue SCSI device reset
    *      @scp: identifies SCSI device to be reset
    *
    *      Returns SUCCESS if command aborted else FAILED
    *
    *      Locks: None held
    *
    *      Calling context: kernel thread
    *
    *      Notes: Invoked from scsi_eh thread. No other commands will be
    *      queued on current host during eh.
    *
    *      Optionally defined in: LLD
    **/
	int eh_device_reset_handler(struct scsi_cmnd * scp)


    /**
    *      eh_host_reset_handler - reset host (host bus adapter)
    *      @scp: SCSI host that contains this device should be reset
    *
    *      Returns SUCCESS if command aborted else FAILED
    *
    *      Locks: None held
    *
    *      Calling context: kernel thread
    *
    *      Notes: Invoked from scsi_eh thread. No other commands will be
    *      queued on current host during eh.
    *      With the default eh_strategy in place, if none of the _abort_,
    *      _device_reset_, _bus_reset_ or this eh handler function are
    *      defined (or they all return FAILED) then the device in question
    *      will be set offline whenever eh is invoked.
    *
    *      Optionally defined in: LLD
    **/
	int eh_host_reset_handler(struct scsi_cmnd * scp)


    /**
    *      info - supply information about given host: driver name plus data
    *             to distinguish given host
    *      @shp: host to supply information about
    *
    *      Return ASCII null terminated string. [This driver is assumed to
    *      manage the memory pointed to and maintain it, typically for the
    *      lifetime of this host.]
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: Often supplies PCI or ISA information such as IO addresses
    *      and interrupt numbers. If not supplied struct Scsi_Host::name used
    *      instead. It is assumed the returned information fits on one line
    *      (i.e. does not included embedded newlines).
    *      The SCSI_IOCTL_PROBE_HOST ioctl yields the string returned by this
    *      function (or struct Scsi_Host::name if this function is not
    *      available).
    *
    *      Optionally defined in: LLD
    **/
	const char * info(struct Scsi_Host * shp)


    /**
    *      ioctl - driver can respond to ioctls
    *      @sdp: device that ioctl was issued for
    *      @cmd: ioctl number
    *      @arg: pointer to read or write data from. Since it points to
    *            user space, should use appropriate kernel functions
    *            (e.g. copy_from_user() ). In the Unix style this argument
    *            can also be viewed as an unsigned long.
    *
    *      Returns negative "errno" value when there is a problem. 0 or a
    *      positive value indicates success and is returned to the user space.
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: The SCSI subsystem uses a "trickle down" ioctl model.
    *      The user issues an ioctl() against an upper level driver
    *      (e.g. /dev/sdc) and if the upper level driver doesn't recognize
    *      the 'cmd' then it is passed to the SCSI mid level. If the SCSI
    *      mid level does not recognize it, then the LLD that controls
    *      the device receives the ioctl. According to recent Unix standards
    *      unsupported ioctl() 'cmd' numbers should return -ENOTTY.
    *
    *      Optionally defined in: LLD
    **/
	int ioctl(struct scsi_device *sdp, int cmd, void *arg)


    /**
    *      proc_info - supports /proc/scsi/{driver_name}/{host_no}
    *      @buffer: anchor point to output to (0==writeto1_read0) or fetch from
    *               (1==writeto1_read0).
    *      @start: where "interesting" data is written to. Ignored when
    *              1==writeto1_read0.
    *      @offset: offset within buffer 0==writeto1_read0 is actually
    *               interested in. Ignored when 1==writeto1_read0 .
    *      @length: maximum (or actual) extent of buffer
    *      @host_no: host number of interest (struct Scsi_Host::host_no)
    *      @writeto1_read0: 1 -> data coming from user space towards driver
    *                            (e.g. "echo some_string > /proc/scsi/xyz/2")
    *                       0 -> user what data from this driver
    *                            (e.g. "cat /proc/scsi/xyz/2")
    *
    *      Returns length when 1==writeto1_read0. Otherwise number of chars
    *      output to buffer past offset.
    *
    *      Locks: none held
    *
    *      Calling context: process
    *
    *      Notes: Driven from scsi_proc.c which interfaces to proc_fs. proc_fs
    *      support can now be configured out of the scsi subsystem.
    *
    *      Optionally defined in: LLD
    **/
	int proc_info(char * buffer, char ** start, off_t offset,
		    int length, int host_no, int writeto1_read0)


    /**
    *      queuecommand - queue scsi command, invoke scp->scsi_done on completion
    *      @shost: pointer to the scsi host object
    *      @scp: pointer to scsi command object
    *
    *      Returns 0 on success.
    *
    *      If there's a failure, return either:
    *
    *      SCSI_MLQUEUE_DEVICE_BUSY if the device queue is full, or
    *      SCSI_MLQUEUE_HOST_BUSY if the entire host queue is full
    *
    *      On both of these returns, the mid-layer will requeue the I/O
    *
    *      - if the return is SCSI_MLQUEUE_DEVICE_BUSY, only that particular
    *      device will be paused, and it will be unpaused when a command to
    *      the device returns (or after a brief delay if there are no more
    *      outstanding commands to it).  Commands to other devices continue
    *      to be processed normally.
    *
    *      - if the return is SCSI_MLQUEUE_HOST_BUSY, all I/O to the host
    *      is paused and will be unpaused when any command returns from
    *      the host (or after a brief delay if there are no outstanding
    *      commands to the host).
    *
    *      For compatibility with earlier versions of queuecommand, any
    *      other return value is treated the same as
    *      SCSI_MLQUEUE_HOST_BUSY.
    *
    *      Other types of errors that are detected immediately may be
    *      flagged by setting scp->result to an appropriate value,
    *      invoking the scp->scsi_done callback, and then returning 0
    *      from this function. If the command is not performed
    *      immediately (and the LLD is starting (or will start) the given
    *      command) then this function should place 0 in scp->result and
    *      return 0.
    *
    *      Command ownership.  If the driver returns zero, it owns the
    *      command and must take responsibility for ensuring the
    *      scp->scsi_done callback is executed.  Note: the driver may
    *      call scp->scsi_done before returning zero, but after it has
    *      called scp->scsi_done, it may not return any value other than
    *      zero.  If the driver makes a non-zero return, it must not
    *      execute the command's scsi_done callback at any time.
    *
    *      Locks: up to and including 2.6.36, struct Scsi_Host::host_lock
    *             held on entry (with "irqsave") and is expected to be
    *             held on return. From 2.6.37 onwards, queuecommand is
    *             called without any locks held.
    *
    *      Calling context: in interrupt (soft irq) or process context
    *
    *      Notes: This function should be relatively fast. Normally it
    *      will not wait for IO to complete. Hence the scp->scsi_done
    *      callback is invoked (often directly from an interrupt service
    *      routine) some time after this function has returned. In some
    *      cases (e.g. pseudo adapter drivers that manufacture the
    *      response to a SCSI INQUIRY) the scp->scsi_done callback may be
    *      invoked before this function returns.  If the scp->scsi_done
    *      callback is not invoked within a certain period the SCSI mid
    *      level will commence error processing.  If a status of CHECK
    *      CONDITION is placed in "result" when the scp->scsi_done
    *      callback is invoked, then the LLD driver should perform
    *      autosense and fill in the struct scsi_cmnd::sense_buffer
    *      array. The scsi_cmnd::sense_buffer array is zeroed prior to
    *      the mid level queuing a command to an LLD.
    *
    *      Defined in: LLD
    **/
	enum scsi_qc_status queuecommand(struct Scsi_Host *shost,
					 struct scsi_cmnd *scp)


    /**
    *      sdev_init -   prior to any commands being sent to a new device
    *                      (i.e. just prior to scan) this call is made
    *      @sdp: pointer to new device (about to be scanned)
    *
    *      Returns 0 if ok. Any other return is assumed to be an error and
    *      the device is ignored.
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: Allows the driver to allocate any resources for a device
    *      prior to its initial scan. The corresponding scsi device may not
    *      exist but the mid level is just about to scan for it (i.e. send
    *      and INQUIRY command plus ...). If a device is found then
    *      sdev_configure() will be called while if a device is not found
    *      sdev_destroy() is called.
    *      For more details see the include/scsi/scsi_host.h file.
    *
    *      Optionally defined in: LLD
    **/
	int sdev_init(struct scsi_device *sdp)


    /**
    *      sdev_configure - driver fine tuning for given device just after it
    *                     has been first scanned (i.e. it responded to an
    *                     INQUIRY)
    *      @sdp: device that has just been attached
    *
    *      Returns 0 if ok. Any other return is assumed to be an error and
    *      the device is taken offline. [offline devices will _not_ have
    *      sdev_destroy() called on them so clean up resources.]
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: Allows the driver to inspect the response to the initial
    *      INQUIRY done by the scanning code and take appropriate action.
    *      For more details see the include/scsi/scsi_host.h file.
    *
    *      Optionally defined in: LLD
    **/
	int sdev_configure(struct scsi_device *sdp)


    /**
    *      sdev_destroy - given device is about to be shut down. All
    *                      activity has ceased on this device.
    *      @sdp: device that is about to be shut down
    *
    *      Returns nothing
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: Mid level structures for given device are still in place
    *      but are about to be torn down. Any per device resources allocated
    *      by this driver for given device should be freed now. No further
    *      commands will be sent for this sdp instance. [However the device
    *      could be re-attached in the future in which case a new instance
    *      of struct scsi_device would be supplied by future sdev_init()
    *      and sdev_configure() calls.]
    *
    *      Optionally defined in: LLD
    **/
	void sdev_destroy(struct scsi_device *sdp)



```
## 数据结构

### struct scsi_host_template

每个 LLD 有一"struct scsi_host_template" 实例 [#]_。它通常作为驱动头文件中的文件作用域 static 被初始化。这样，未显式初始化的成员会被设0 NULL。值得关注的成员：

    name
   - 驱动名称（可包含空格，请限制80 个字符以内）

    proc_name
   - 用于 "/proc/scsi/<proc_name>/<host_no>" 的名称，也由 sysfs 在其某个 "drivers" 目录中使用。因"proc_name" 只能包含 Unix 文件名可接受的字符

   `(*queuecommand)()`
   - 中间层用来向 LLD 注入 SCSI 命令的主要回调

    vendor_id
   - 一个唯一值，用于标识Scsi_Host 提供 LLD 的厂商。最常用于校验厂商特定的消息请求。值由一个标识符类型和一个厂商特定值组成。有效格式说明见 scsi_netlink.h

该结构在 include/scsi/scsi_host.h 中定义并附有注释

      如果它控制几类不同的硬件（例如一个同时处ISA PCI 卡、并为每类硬件单独准备一struct scsi_host_template 实例LLD）

### struct Scsi_Host

LLD 控制的每个主机（HBA）有一struct Scsi_Host 实例。struct Scsi_Host 结构"struct scsi_host_template" 有许多共同成员。当创建一个新struct Scsi_Host 实例时（hosts.c scsi_host_alloc() 中），那些共同成员会从驱动的 struct scsi_host_template 实例初始化而来。值得关注的成员：

    host_no
   - 系统范围内唯一的编号，用于标识此主机。从 0 开始按升序分配
    can_queue
   - 必须大于 0；不要向适配器发送超can_queue 条命令
    this_id
   - 主机scsi id（scsi 发起者），若未知则为 -1
    sg_tablesize
   - 主机允许的最大分聚集（scatter gather）元素数量。将其设SG_ALL 或更小以避免链式 SG 列表。必须至少为 1
    max_sectors
   - 单条 SCSI 命令允许的最大扇区数（通常512 字节）。默认0 会导致设置为 SCSI_DEFAULT_MAX_SECTORS（在 scsi_host.h 中定义），当前设1024。因此当未定max_sectors 时，磁盘的最大传输大小为 512 KB。注意此大小可能不足以进行磁盘固件上传
    cmd_per_lun
   - 主机控制的设备上可以排队的最大命令数。会LLD scsi_change_queue_depth() 的调用覆盖
    hostt
   - 指向生成struct Scsi_Host 实例的驱struct scsi_host_template 的指
    hostt->proc_name
   - LLD 的名称。这sysfs 使用的驱动名称
    transportt
   - 指向驱动 struct scsi_transport_template 实例的指针（如果有）。当前支FC SPI 传输层
    hostdata[^0^]
   - struct Scsi_Host 末尾LLD 保留的区域。大小由传入 scsi_host_alloc() 的第二个参数（名'privsize'）设置

scsi_host 结构include/scsi/scsi_host.h 中定

### struct scsi_device

通常，主机上每个 SCSI 逻辑单元都有一个此结构的实例。连接到主机SCSI 设备由通道号、目id 和逻辑单元号（lun）唯一标识。该结构include/scsi/scsi_device.h 中定义

### struct scsi_cmnd

此结构的实例SCSI 命令传递给 LLD，并将响应返回给中间层。SCSI 中间层会确保排入 LLD SCSI 命令不超**scsi_change_queue_depth()（或 struct Scsi_Host**：cmd_per_lun）所指示的数量。每SCSI 设备至少会有一struct scsi_cmnd 实例可用。值得关注的成员：

    cmnd
   - 包含 SCSI 命令的数
    cmd_len
   - SCSI 命令的长度（字节
    sc_data_direction
   - 数据阶段数据传输的方向。参include/linux/dma-mapping.h 中的 "enum dma_data_direction"
    result
   - 应在调用 'done' 之前LLD 设置。0 表示命令成功完成（且所有数据（如果有）已传向或SCSI 目标设备传出）result' 是一32 位无符号整数，可视为两个相关的字节。SCSI 状态值在最低字节（LSB）中。参include/scsi/scsi.h 中的 status_byte() host_byte() 宏及相关常量
    sense_buffer
   - 一个数组（最大大小：SCSI_SENSE_BUFFERSIZE 字节），SCSI 状态（'result' LSB）被设为 CHECK_CONDITION (2) 时应被写入。当设置CHECK_CONDITION 时，如果 sense_buffer[^0^] 的高半字节值为 7，则中间层会假定 sense_buffer 数组包含有效SCSI sense 缓冲；否则中间层会发出一REQUEST_SENSE SCSI 命令来取sense 缓冲。后一种策略在存在命令排队时容易出错，因此 LLD 应当始终"自动感知（auto-sense
    device
   - 指向此命令所关联scsi_device 对象的指针
    resid_len   （通过调用 scsi_set_resid() / scsi_get_resid() 访问
   - LLD 应将此无符号整数设为请求的传输长度（'request_bufflen'）减去实际传输的字节数resid_len' 预设0，因此如LLD 无法检测欠载（不应报告过载），可以忽略它。LLD 应在调用 'done' 之前设置 'resid_len'。最值得关注的情形是SCSI 目标设备（例READ）传输出来的、发生欠载的数据传输
    underflow
   - 如果实际传输的字节数小于此值，LLD 应将 (DID_ERROR << 16) 放入 'result'。实现此检查的 LLD 不多，而其中一些只是向日志输出一条错误消息，而不是报DID_ERROR。LLD 最好实'resid_len'

建议 LLD 在来SCSI 目标设备（例READ）的数据传输上设'resid_len'。当这类数据传输具有 MEDIUM ERROR HARDWARE ERROR（以及可能的 RECOVERED ERROR）的 sense 键时，设'resid_len' 尤为重要。在这些情况下，如果 LLD 不确定已接收到多少数据，最安全的做法是表明没有接收到任何字节。例如：要表明没有接收到有效数据

```
    scsi_set_resid(SCpnt, scsi_bufflen(SCpnt));

```

其中 'SCpnt' 是指scsi_cmnd 对象的指针。要表明仅有三个 512

```
    scsi_set_resid(SCpnt, scsi_bufflen(SCpnt) - (3 * 512));

```

scsi_cmnd 结构include/scsi/scsi_cmnd.h 中定


## 閿。

每个 struct Scsi_Host 实例都有一个名struct
**Scsi_Host**
: default_lock 的自旋锁，在 scsi_host_alloc() [位于
**hosts.c] 中初始化。在同一函数中，struct Scsi_Host**
: host_lock 指针
被初始化为指default_lock。此后，中间层执行的加锁与解
**操作使用 struct Scsi_Host**
: host_lock
指针。以前驱动可以覆host_lock 指针，但现在不再允许


## 自动感知（Autosense

自动感知（Autosense，或 auto-sense）在 SAM-2 文档中被定义为：当发CHECK CONDITION 状态时SCSI 命令完成时自动将 sense 数据返回给应用程序客户端"。LLD 应当执行自动感知。这应在 LLD 检测到 CHECK CONDITION 状态时通过以下任一方式完成

    a) 指示 SCSI 协议（例SCSI 并行接口（SPI））对此类响应执行一个额外的数据输入阶段
    b) 或者，LLD 自己发出一REQUEST SENSE 命令

无论哪种方式，当检测到 CHECK CONDITION 状态时，中间层通过检struct
**scsi_cmnd**
: sense_buffer[^0^] 来判LLD 是否已执行自动感知。如果该字节的高半字节为 7（或 0xf），则假定已执行自动感知。如果它是其他值（并且该字节在每条命令之前被初始化0），则中间层会发出一REQUEST SENSE 命令

在存在排队命令的情况下，维护失败命令sense 缓冲数据直到后续 REQUEST SENSE nexus"可能会失去同步。这就是为什LLD 最好执行自动感知


## 相对Linux 内核 2.4 系列的变

io_request_lock 已被若干个更细粒度的锁取代。与 LLD **相关的是 struct Scsi_Host**: host_lock，每SCSI 主机各有一个

旧的错误处理机制已被移除。这意味着 LLD 接口函数 abort() reset() 已被移除*struct scsi_host_template**: use_new_eh_code 标志已被移除

2.4 系列中，SCSI 子系统的配置说明与所有其Linux 子系统的配置说明聚合Documentation/Configure.help 文件中。在 2.6 系列中，SCSI 子系统现在拥有自己的（小得多的）drivers/scsi/Kconfig 文件，其中同时包含配置和帮助信息

struct SHT 已重命名struct scsi_host_template

增加热插拔初始化模型"以及许多用于支持它的额外函数


## 致谢

以下人士对本文档做出了贡献：

 - Mike Anderson <andmike at us dot ibm dot com>
 - James Bottomley <James dot Bottomley at hansenpartnership dot com>
 - Patrick Mansfield <patmans at us dot ibm dot com>
 - Christoph Hellwig <hch at infradead dot org>
 - Doug Ledford <dledford at redhat dot com>
 - Andries Brouwer <Andries dot Brouwer at cwi dot nl>
 - Randy Dunlap <rdunlap at xenotime dot net>
 - Alan Stern <stern at rowland dot harvard dot edu>


Douglas Gilbert
dgilbert at interlog dot com

21st September 2004
