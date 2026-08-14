
## SAS 层


SAS 层是一个管理基础设施，用于管理 SAS LLDD。它位于 SCSI Core 与 SAS LLDD 之间。其布局如下：SCSI Core 关注 SAM/SPC 相关事项，而 SAS LLDD+定序器关注 phy/OOB/链路管理，SAS 层则关注：

      - SAS Phy/Port/HA 事件管理（由 LLDD 产生，由 SAS 层处理），
      - SAS 端口管理（创建/销毁），
      - SAS 域发现与重新验证，
      - SAS 域设备管理，
      - SCSI 主机注册/注销，
      - 向 SCSI Core（SAS）或 libata（SATA）注册设备，以及
      - 扩展器管理并向用户空间导出扩展器控制。

SAS LLDD 是一个 PCI 设备驱动。它关注 phy/OOB 管理、厂商相关任务，并向 SAS 层产生事件。

SAS 层完成了 SAS 1.1 规范中概述的大部分 SAS 任务。

sas_ha_struct 向 SAS 层描述 SAS LLDD。它的大部分由 SAS 层使用，但少数字段需要由 LLDD 初始化。

在初始化完硬件之后，你从 probe() 函数中调用 sas_register_ha()。它会将你的 LLDD 注册到 SCSI 子系统，创建一个 SCSI 主机，并将你的 SAS 驱动注册到它创建的 sysfs SAS 树中。然后它返回。接着你启用你的 phys 以实际开始 OOB（此时你的驱动将开始调用 notify_* 事件回调）。

## 结构体说明


### ``struct sas_phy``


通常情况下它静态内嵌在你的驱动的

```
    struct my_phy {
	    blah;
	    struct sas_phy sas_phy;
	    bleh;
    };
```

之中，随后所有 phys 都是你 HA struct 中 my_phy 的数组（如下所示）。

然后随着你逐步初始化你的 phys，你也同时初始化 sas_phy struct，以及你自己的 phy 结构。

一般而言，phys 由 LLDD 管理，端口由 SAS 层管理。因此 phys 由 LLDD 初始化和更新，端口由 SAS 层初始化和更新。

存在一种机制：LLDD 可以读写某些字段，而 SAS 层只能读取这些字段，反之亦然。其目的在于避免不必要的加锁。

enabled
    - 必须设置（0/1）

id
    - 必须设置 [0,MAX_PHYS)]

class, proto, type, role, oob_mode, linkrate
    - 必须设置

oob_mode
    - 你在 OOB 完成后设置此项，然后通知 SAS 层。

sas_addr
    - 通常指向一个数组，该数组保存了 phy 的 sas 地址，可能位于你的 my_phy struct 中的某处。

attached_sas_addr
    - 当你（LLDD）收到一个 IDENTIFY 帧或 FIS 帧时，在通知 SAS 层 _之前_ 设置此项。其思路是，有时 LLDD 可能希望在那个 phy/端口上伪造或提供一个不同的 SAS 地址，这允许它这样做。最好情况下，你应当从 IDENTIFY 帧中复制 sas 地址，或者对直接连接的 SATA 设备生成一个 SAS 地址。该值稍后可能被 Discover 过程改变。

frame_rcvd
    - 这是你收到 IDENTIFY/FIS 帧时复制它的位置；你加锁、复制、设置 frame_rcvd_size 并解锁该锁，然后调用事件。它是一个指针，因为无法 _精确_ 知道你的硬件帧大小，所以你在你的 phy struct 中定义实际的数组并让该指针指向它。你在该锁的保护下将帧从你的可 DMA 内存复制到该区域。

sas_prim
    - 这是收到原语时它们所去往的位置。参见 sas.h。获取锁，设置原语，释放锁，然后通知。

port
    - 如果该 phy 属于某个端口，则它指向 sas_port——LLDD 只读此项。它指向该 phy 所属的 sas_port。由 SAS 层设置。

ha
    - 可以设置；SAS 层无论如何都会设置它。

lldd_phy
    - 你应当将此项设置为指向你的 phy，这样当 SAS 层调用你的某个回调并传给你一个 phy 时，你可以更快地找到位置。如果 sas_phy 是内嵌的，你也可以使用 container_of——随你喜欢。


### ``struct sas_port``


LLDD 不设置该结构体的任何字段——它只读取它们。它们应当是不言自明的。

phy_mask 是 32 位的，目前这应当足够，因为我还没听说过有超过 8 个 phys 的 HA。

lldd_port
    - 我还没找到它的用途——也许其他希望拥有内部端口表示的 LLDD 可以利用它。

### ``struct sas_ha_struct``


它通常在你自己的 LLDD 中静态声明：

```
    struct my_sas_ha {
	blah;
	struct sas_ha_struct sas_ha;
	struct my_phy phys[MAX_PHYS];
	struct sas_port sas_ports[MAX_PHYS]; /* (1) */
	bleh;
    };

    (1) 如果你的 LLDD 没有自己的端口表示。
```

需要初始化哪些内容（示例函数见下）。

##### pcidev


sas_addr
       - 由于 SAS 层不想处理内存分配等事务，此项指向某处静态分配的数组（比如在你的主机适配器结构中），并保存由你或制造商等给出的主机适配器的 SAS 地址。

##### sas_port


sas_phy
      - 一个指向结构体的指针数组。（参见上面关于 sas_addr 的说明）。
	这些必须设置。更多说明见下。

num_phys
       - sas_phy 数组中存在的 phys 数量，以及 sas_port 数组中存在的端口数量。最多可以有 num_phys 个端口（每个端口一个），因此我们去掉 num_ports，只使用 num_phys。

```
	/* LLDD 调用这些来通知类发生了一个事件。 */
	void sas_notify_port_event(struct sas_phy *, enum port_event, gfp_t);
	void sas_notify_phy_event(struct sas_phy *, enum phy_event, gfp_t);
```
```
	/* 类调用这些来通知 LLDD 发生了一个事件。 */
	void (*lldd_port_formed)(struct sas_phy *);
	void (*lldd_port_deformed)(struct sas_phy *);
```
如果 LLDD 希望在端口形成或销毁时收到通知，它将这两项设置为满足该类型的函数。

一个 SAS LLDD 还应当至少实现下列任务管理函数中的一个：

```
	/* 任务管理函数。必须从进程上下文调用。 */
	int (*lldd_abort_task)(struct sas_task *);
	int (*lldd_abort_task_set)(struct domain_device *, u8 *lun);
	int (*lldd_clear_task_set)(struct domain_device *, u8 *lun);
	int (*lldd_I_T_nexus_reset)(struct domain_device *);
	int (*lldd_lu_reset)(struct domain_device *, u8 *lun);
	int (*lldd_query_task)(struct sas_task *);
```
更多信息请阅读 T10.org 上的 SAM。

```
	/* 端口与适配器管理 */
	int (*lldd_clear_nexus_port)(struct sas_port *);
	int (*lldd_clear_nexus_ha)(struct sas_ha_struct *);
```
一个 SAS LLDD 应当至少实现其中之一。

```
	/* Phy 管理 */
	int (*lldd_control_phy)(struct sas_phy *, enum phy_func);
```
lldd_ha
    - 将此设置为指向你的 HA struct。如果你像上面那样内嵌了它，也可以使用 container_of。

一个示例的初始化与注册函数可以像这样（从 probe() 最后调用）：

```
    static int register_sas_ha(struct my_sas_ha *my_ha)
    {
	    int i;
	    static struct sas_phy   *sas_phys[MAX_PHYS];
	    static struct sas_port  *sas_ports[MAX_PHYS];

	    my_ha->sas_ha.sas_addr = &my_ha->sas_addr[0];

	    for (i = 0; i < MAX_PHYS; i++) {
		    sas_phys[i] = &my_ha->phys[i].sas_phy;
		    sas_ports[i] = &my_ha->sas_ports[i];
	    }

	    my_ha->sas_ha.sas_phy  = sas_phys;
	    my_ha->sas_ha.sas_port = sas_ports;
	    my_ha->sas_ha.num_phys = MAX_PHYS;

	    my_ha->sas_ha.lldd_port_formed = my_port_formed;

	    my_ha->sas_ha.lldd_dev_found = my_dev_found;
	    my_ha->sas_ha.lldd_dev_gone = my_dev_gone;

	    my_ha->sas_ha.lldd_execute_task = my_execute_task;

	    my_ha->sas_ha.lldd_abort_task     = my_abort_task;
	    my_ha->sas_ha.lldd_abort_task_set = my_abort_task_set;
	    my_ha->sas_ha.lldd_clear_task_set = my_clear_task_set;
	    my_ha->sas_ha.lldd_I_T_nexus_reset= NULL; (2)
	    my_ha->sas_ha.lldd_lu_reset       = my_lu_reset;
	    my_ha->sas_ha.lldd_query_task     = my_query_task;

	    my_ha->sas_ha.lldd_clear_nexus_port = my_clear_nexus_port;
	    my_ha->sas_ha.lldd_clear_nexus_ha = my_clear_nexus_ha;

	    my_ha->sas_ha.lldd_control_phy = my_control_phy;

	    return sas_register_ha(&my_ha->sas_ha);
    }
```
(2) SAS 1.1 没有定义 I_T Nexus Reset TMF。

## 事件


事件是 SAS LLDD 通知 SAS 层任何事情的 **唯一方式**。LLDD 没有别的方法或途径来告诉 SAS 层其内部或 SAS 域中发生的任何事情。

```
	PHYE_LOSS_OF_SIGNAL, (C)
	PHYE_OOB_DONE,
	PHYE_OOB_ERROR,      (C)
	PHYE_SPINUP_HOLD.
```
```
	PORTE_BYTES_DMAED,      (M)
	PORTE_BROADCAST_RCVD,   (E)
	PORTE_LINK_RESET_ERR,   (C)
	PORTE_TIMER_EVENT,      (C)
	PORTE_HARD_RESET.
```
主机适配器事件：
	HAE_RESET

一个 SAS LLDD 应当能够产生

 - 来自 C 组（可选）的至少一个事件，
 - 标记为 M（强制）的事件是强制的（只有一个），
 - 标记为 E（扩展器）的事件，如果它希望 SAS 层处理域重新验证（只有一个这样的事件）。
 - 未标记的事件是可选的。

含义：

HAE_RESET
    - 当你的 HA 发生内部错误并被重置时。

PORTE_BYTES_DMAED
    - 收到 IDENTIFY/FIS 帧时

PORTE_BROADCAST_RCVD
    - 收到一个原语时

PORTE_LINK_RESET_ERR
    - 定时器超时、信号丢失、DWS 丢失等 [^1^]_

PORTE_TIMER_EVENT
    - DWS 重置超时定时器到期 [^1^]_

PORTE_HARD_RESET
    - 收到硬复位原语。

PHYE_LOSS_OF_SIGNAL
    - 设备消失了 [^1^]_

PHYE_OOB_DONE
    - OOB 顺利完成且 oob_mode 有效

PHYE_OOB_ERROR
    - 进行 OOB 时出错，设备可能已断开连接。[^1^]_

PHYE_SPINUP_HOLD
    - 存在 SATA，但未发送 COMWAKE。

       或者也可以从它们的 tasklet 中调用内联的 sas_phy_disconnected()，它只是一个辅助函数。

```
	int (*lldd_execute_task)(struct sas_task *, gfp_t gfp_flags);
```
用于向 SAS LLDD 排队一个任务。@task 是要被执行的任务。@gfp_mask 是定义调用方上下文的 gfp_mask。

该函数应当实现 Execute Command SCSI RPC，

也就是说，当调用 lldd_execute_task() 时，命令 **立即** 在传输层上发出。在 SAS LLDD 中 **不存在** 任何种类、任何层次的排队。

返回：

   - -SAS_QUEUE_FULL、-ENOMEM，未排队任何内容；
   - 0，任务已排队。

```
    struct sas_task {
	    dev -- 该任务所发往的设备
	    task_proto -- enum sas_proto 中的 _一个_
	    scatter -- 指向分散/聚集列表数组的指针
	    num_scatter -- scatter 中的元素个数
	    total_xfer_len -- 预期传输的总字节数
	    data_dir -- PCI_DMA_...
	    task_done -- 任务执行完成时的回调
    };
```

## 发现


sysfs 树有以下用途：

    a) 它显示当前时刻 SAS 域的物理布局，即此刻域在物理世界中的样子。
    b) 显示 _发现时_ 的某些设备参数。

这是一个指向 tree(1) 程序的链接，在查看 SAS 域时非常有用：
ftp://mama.indstate.edu/linux/tree/

我期望用户空间应用程序实际创建它的图形界面。

也就是说，sysfs 域树不显示也不保存状态，例如如果你改变了 READY LED MEANING 设置的含义，但它确实显示域设备的当前连接状态。

保存内部设备状态变化的责任在上层（命令集驱动）和用户空间。

当设备或设备们从域中拔出时，这会立即反映在 sysfs 树中，并且该设备（们）会从系统中移除。

domain_device 结构描述 SAS 域中的任何设备。它完全由 SAS 层管理。一个任务指向一个域设备，SAS LLDD 由此知道将任务发往何处。SAS LLDD 只读取 domain_device 结构的内容，但从不创建或销毁它。

## 来自用户空间的扩展器管理


在 sysfs 中每个扩展器目录下，都有一个名为 "smp_portal" 的文件。它是一个二进制 sysfs 属性文件，实现了一个 SMP portal（注意：这 **不是** 一个 SMP 端口），用户空间应用程序可以向其发送 SMP 请求并接收 SMP 响应。

其功能看似简单实则不然：

1. 构建你想发送的 SMP 帧。格式和布局在 SAS 规范中描述。将 CRC 字段留为 0。

open(2)

2. 以读写模式打开扩展器的 SMP portal sysfs 文件。

write(2)

3. 写入你在第 1 步构建的帧。

read(2)

4. 读取你期望为所构建帧接收的数据量。如果你收到的数据量与期望的不同，则发生了某种错误。

close(2)

整个过程在函数 do_smp_func() 及其调用者中有详细展示，位于 "expander_conf.c" 文件中。

内核功能实现在 "sas_expander.c" 文件中。

程序 "expander_conf.c" 实现了此功能。它接受一个参数，即指向扩展器的 SMP portal 的 sysfs 文件名，并给出扩展器信息，包括路由表。

SMP portal 让你完全控制扩展器，所以请小心。
