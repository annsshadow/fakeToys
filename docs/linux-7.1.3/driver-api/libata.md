## libATA 开发者指南


:Author: Jeff Garzik

## 简介


libATA 是 Linux 内核内部使用的一个库，用于支持 ATA 主机控制器和设备。libATA 提供
一个 ATA 驱动 API、用于 ATA 和 ATAPI 设备的类传输，以及根据 T10 SAT 规范实现的
ATA 设备的 SCSI<->ATA 转换。

本指南记录了 libATA 驱动 API、库函数、库内部实现，以及几个示例性的 ATA 底层驱动。

## libata 驱动 API


`struct ata_port_operations <ata_port_operations>`
为每个底层 libata 硬件驱动定义，它控制底层驱动如何与 ATA 和 SCSI 层交互。

基于 FIS 的驱动将通过 `->qc_prep()` 和 `->qc_issue()` 高层钩子挂接到系统。行为类似
于 PCI IDE 硬件的硬件可以利用若干通用辅助函数，至少定义 ATA 影子寄存器块的
总线 I/O 地址。

### :c:type:`struct ata_port_operations <ata_port_operations>`


#### 识别（IDENTIFY）后的设备配置


```
    void (*dev_config) (struct ata_port *, struct ata_device *);
```
在对找到的每个设备发出 IDENTIFY [PACKET] DEVICE 之后调用。通常用于在发出 SET
FEATURES - XFER MODE 之前以及操作之前，应用设备特定的修正。

这个入口在 ata_port_operations 中可以被指定为 NULL。

#### 设置 PIO/DMA 模式


```
    void (*set_piomode) (struct ata_port *, struct ata_device *);
    void (*set_dmamode) (struct ata_port *, struct ata_device *);
    void (*post_set_mode) (struct ata_port *);
    unsigned int (*mode_filter) (struct ata_port *, struct ata_device *, unsigned int);
```
在发出 SET FEATURES - XFER MODE 命令之前调用的钩子。可选的 `->mode_filter()` 钩子
在 libata 已经构建出可能模式的掩码时被调用。它会传递给 `->mode_filter()` 函数，该函数
应返回经过过滤、去掉了那些因硬件限制而不合适的模式之后的有效模式掩码。使用这个接口
来添加模式是无效的。

当 `->set_piomode()` 和 `->set_dmamode()` 被调用时，`dev->pio_mode` 和 `dev->dma_mode`
保证有效。此时，共享同一线缆的任何其他驱动器的时序也将有效。也就是说，库会在尝试
设置任何一个驱动器的模式之前，记录下通道上每个驱动器模式的决策。

`->post_set_mode()` 在 SET FEATURES - XFER MODE 命令成功完成之后无条件调用。

`->set_piomode()` 总是被调用（如果存在），但 `->set_dma_mode()` 仅在 DMA 可行时才被调用。

#### 任务文件读/写


```
    void (*sff_tf_load) (struct ata_port *ap, struct ata_taskfile *tf);
    void (*sff_tf_read) (struct ata_port *ap, struct ata_taskfile *tf);
```
`->tf_load()` 被调用以将给定的任务文件加载到硬件寄存器 / DMA 缓冲区。`->tf_read()`
被调用以读取硬件寄存器 / DMA 缓冲区，从而获得当前的一组任务文件寄存器值。大多数
基于任务文件硬件（PIO 或 MMIO）的驱动对这些钩子使用 `ata_sff_tf_load` 和
`ata_sff_tf_read`。

#### PIO 数据读/写


```
    void (*sff_data_xfer) (struct ata_device *, unsigned char *, unsigned int, int);
```
所有 bmdma 风格的驱动都必须实现这个钩子。这是在 PIO 数据传输期间实际复制数据字节的
底层操作。通常驱动会选择 `ata_sff_data_xfer` 或 `ata_sff_data_xfer32` 之一。

#### ATA 命令执行


```
    void (*sff_exec_command)(struct ata_port *ap, struct ata_taskfile *tf);
```
使之前用 `->tf_load()` 加载的 ATA 命令在硬件中启动。大多数基于任务文件硬件的驱动
使用 `ata_sff_exec_command` 作为这个钩子。

#### 每条命令的 ATAPI DMA 能力过滤器


```
    int (*check_atapi_dma) (struct ata_queued_cmd *qc);
```
允许底层驱动过滤 ATA PACKET 命令，返回一个状态，指示是否可以使用 DMA 来执行所提供的
PACKET 命令。

这个钩子可以被指定为 NULL，在这种情况下 libata 将假定支持 atapi dma。

#### 读取特定的 ATA 影子寄存器


```
    u8   (*sff_check_status)(struct ata_port *ap);
    u8   (*sff_check_altstatus)(struct ata_port *ap);
```
从硬件读取 Status/AltStatus ATA 影子寄存器。在某些硬件上，读取 Status 寄存器具有
清除中断条件的副作用。大多数基于任务文件硬件的驱动使用 `ata_sff_check_status` 作为
这个钩子。

#### 写入特定的 ATA 影子寄存器


```
    void (*sff_set_devctl)(struct ata_port *ap, u8 ctl);
```
将设备控制 ATA 影子寄存器写入硬件。大多数驱动不需要定义这个。

#### 在总线上选择 ATA 设备


```
    void (*sff_dev_select)(struct ata_port *ap, unsigned int device);
```
发出底层硬件命令，使 N 个硬件设备中的一个在 ATA 总线上被视为“被选中”（活跃且可
使用）。这在基于 FIS 的设备上通常没有意义。

大多数基于任务文件硬件的驱动使用 `ata_sff_dev_select` 作为这个钩子。

#### 私有调优方法


```
    void (*set_mode) (struct ata_port *ap);
```
默认情况下，libata 按照 ATA 时序规则进行驱动器和控制器调优，并应用黑名单和线缆限制。
某些控制器需要特殊处理，并有自定义的调优规则，通常是使用 ATA 命令但实际上不进行
驱动器时序调整的 raid 控制器。

    **警告**

    当一个控制器存在怪癖（quirk）时，不应使用这个钩子来替换标准的控制器调优逻辑。
    在这种情况下替换默认的调优逻辑将绕过对驱动器与桥接器怪癖的处理，而这些怪癖可能
    对数据可靠性很重要。如果一个控制器需要过滤模式选择，它应该改用 mode_filter 钩子。

#### 控制 PCI IDE BMDMA 引擎


```
    void (*bmdma_setup) (struct ata_queued_cmd *qc);
    void (*bmdma_start) (struct ata_queued_cmd *qc);
    void (*bmdma_stop) (struct ata_port *ap);
    u8   (*bmdma_status) (struct ata_port *ap);
```
在设置 IDE BMDMA 事务时，这些钩子分别装载（`->bmdma_setup`）、触发（`->bmdma_start`）
和停止（`->bmdma_stop`）硬件的 DMA 引擎。`->bmdma_status` 用于读取标准的 PCI IDE DMA
Status 寄存器。

在基于 FIS 的驱动中，这些钩子通常是空操作，或者干脆不实现。

大多数传统 IDE 驱动使用 `ata_bmdma_setup` 作为 `bmdma_setup` 钩子。`ata_bmdma_setup`
会将指向 PRD 表的指针写入 IDE PRD Table Address 寄存器，在 DMA Command 寄存器中
启用 DMA，并调用 `exec_command` 开始传输。

大多数传统 IDE 驱动使用 `ata_bmdma_start` 作为 `bmdma_start` 钩子。`ata_bmdma_start`
会将 ATA_DMA_START 标志写入 DMA Command 寄存器。

许多传统 IDE 驱动使用 `ata_bmdma_stop` 作为 `bmdma_stop` 钩子。`ata_bmdma_stop` 会
清除 DMA command 寄存器中的 ATA_DMA_START 标志。

许多传统 IDE 驱动使用 `ata_bmdma_status` 作为 `bmdma_status` 钩子。

#### 高层任务文件钩子


```
    enum ata_completion_errors (*qc_prep) (struct ata_queued_cmd *qc);
    int (*qc_issue) (struct ata_queued_cmd *qc);
```
更高层的钩子，这两个钩子有可能取代上面若干任务文件/DMA 引擎钩子。`->qc_prep` 在
缓冲区完成 DMA 映射后被调用，通常用于填充硬件的 DMA 分散/聚集表。一些驱动使用标准的
`ata_bmdma_qc_prep` 和 `ata_bmdma_dumb_qc_prep` 辅助函数，但更高级的驱动会实现自己的。

`->qc_issue` 用于在硬件和 S/G 表准备就绪后，使一个命令变为活跃。IDE BMDMA 驱动使用
辅助函数 `ata_sff_qc_issue` 进行基于任务文件协议的派发。更高级的驱动实现自己的
`->qc_issue`。

`ata_sff_qc_issue` 会根据需要调用 `->sff_tf_load()`、`->bmdma_setup()` 和
`->bmdma_start()` 来发起一次传输。

#### 异常与探测处理（EH）


```
    void (*freeze) (struct ata_port *ap);
    void (*thaw) (struct ata_port *ap);
```
当 HSM 违例或其他某种状况扰乱了端口的正常运行时，会调用 `ata_port_freeze`。一个被冻结
的端口不允许执行任何操作，直到该端口被解冻，而解冻通常发生在一次成功的重置之后。

可选的 `->freeze()` 回调可用于在硬件层面冻结端口（例如屏蔽中断并停止 DMA 引擎）。
如果一个端口无法在硬件层面被冻结，中断处理器必须在端口冻结期间无条件地确认并清除
中断。

可选的 `->thaw()` 回调被调用以执行与 `->freeze()` 相反的操作：再次为端口准备正常的
操作。取消屏蔽中断、启动 DMA 引擎等。

```
    void (*error_handler) (struct ata_port *ap);
```
`->error_handler()` 是驱动接入探测、热插拔、恢复以及其他异常状况的钩子。一个实现的
主要职责是调用 `ata_std_error_handler`。

`ata_std_error_handler` 将执行一个标准的错误处理序列，以复活失败的设备、分离丢失的
设备并添加新设备（如果有的话）。这个函数将在需要时调用一个端口的各种重置操作。这些
操作如下。

- “prereset” 操作（可以为 NULL）在 EH 重置期间、在任何其他动作执行之前被调用。

- “postreset” 钩子（可以为 NULL）在 EH 重置执行之后被调用。基于现有状况、问题的严重
  程度以及硬件能力，

- 要么调用 “softreset” 操作，要么调用 “hardreset” 操作来执行底层的 EH 重置。如果两种
  操作都定义了，则优先并使用 “hardreset”。如果两者都未定义，则不执行任何底层重置，
  EH 假定一个 ATA 类设备通过链路连接。

```
    void (*post_internal_cmd) (struct ata_queued_cmd *qc);
```
执行任何必要的硬件特定动作，以在使用 `ata_exec_internal` 执行一个探测时或 EH 时的命令
之后，完成处理。

#### 硬件中断处理


```
    irqreturn_t (*irq_handler)(int, void *, struct pt_regs *);
    void (*irq_clear) (struct ata_port *);
```
`->irq_handler` 是由 libata 向系统注册的中断处理例程。`->irq_clear` 在探测期间、在
中断处理例程注册之前被调用，以确保硬件处于静默状态。

第二个参数 dev_instance 应该被转换为指向 `struct ata_host_set <ata_host_set>` 的指针。

大多数传统 IDE 驱动使用 `ata_sff_interrupt` 作为 irq_handler 钩子，它会扫描 host_set
中的所有端口，确定哪个排队的命令是活跃的（如果有），并调用 ata_sff_host_intr(ap,qc)。

大多数传统 IDE 驱动使用 `ata_sff_irq_clear` 作为 `irq_clear` 钩子，它只是清除 DMA
status 寄存器中的中断和错误标志。

#### SATA phy 读/写


```
    int (*scr_read) (struct ata_port *ap, unsigned int sc_reg,
             u32 *val);
    int (*scr_write) (struct ata_port *ap, unsigned int sc_reg,
                       u32 val);
```
读取和写入标准 SATA phy 寄存器。sc_reg 是 SCR_STATUS、SCR_CONTROL、SCR_ERROR 或
SCR_ACTIVE 之一。

#### 初始化与关闭


```
    int (*port_start) (struct ata_port *ap);
    void (*port_stop) (struct ata_port *ap);
    void (*host_stop) (struct ata_host_set *host_set);
```
`->port_start()` 在每个端口的数据结构初始化之后立即被调用。通常它用于分配每端口的
DMA 缓冲区 / 表 / 环，启用 DMA 引擎，以及类似的任务。一些驱动也利用这个入口点来为
`ap->private_data` 分配驱动私有内存。

许多驱动使用 `ata_port_start` 作为这个钩子，或从它们自己的 `port_start` 钩子中调用
它。`ata_port_start` 为一个传统的 IDE PRD 表分配空间并返回。

`->port_stop()` 在 `->host_stop()` 之后被调用。它唯一的功能是释放 DMA/内存资源，因为
它们不再被主动使用。许多驱动此时也从端口释放驱动私有数据。

`->host_stop()` 在所有 `->port_stop()` 调用完成后被调用。该钩子必须完成硬件关闭、
释放 DMA 和其他资源等。这个钩子可以被指定为 NULL，在这种情况下它不会被调用。

## 错误处理


本章描述 libata 下如何处理错误。建议读者先阅读 SCSI EH
（Documentation/scsi/scsi_eh.rst）和 ATA 异常文档。

### 命令的来源


在 libata 中，一个命令由 `struct ata_queued_cmd <ata_queued_cmd>`（即 qc）表示。
qc 在端口初始化期间被预分配，并被重复使用于命令执行。当前每个端口只分配一个 qc，但
尚未合并的 NCQ 分支为每个标签分配一个，并将每个 qc 与 NCQ 标签 1 对 1 映射。

libata 命令可以来自两个源头——libata 自身和 SCSI 中间层。libata 内部命令用于初始化和
错误处理。所有常规的块请求以及用于 SCSI 模拟的命令都作为 SCSI 命令，通过 SCSI host
模板的 queuecommand 回调传递。

### 命令如何被发出


内部命令
    一旦为要执行的命令初始化了已分配 qc 的任务文件。qc 目前有两种机制来通知完成。
    一种是通过 `qc->complete_fn()` 回调，另一种是完成 `qc->waiting`。`qc->complete_fn()`
    回调是由常规 SCSI 转换命令使用的异步路径，而 `qc->waiting` 是由内部命令使用的同步
    （发出者在进程上下文中休眠）路径。

    初始化完成后，获取 host_set 锁，并发布 qc。

SCSI 命令
    所有 libata 驱动都使用 `ata_scsi_queuecmd` 作为 `hostt->queuecommand` 回调。
    scmd 可以被模拟或转换。处理被模拟的 scmd 不涉及任何 qc。结果会立即计算出来，scmd
    也随之完成。

    `qc->complete_fn()` 回调用于完成通知。ATA 命令使用 `ata_scsi_qc_complete`，而
    ATAPI 命令使用 `atapi_qc_complete`。两个函数最终都会调用 `qc->scsidone`，在 qc
    完成时通知上层。转换完成后，qc 通过 `ata_qc_issue` 发布。

    注意，SCSI 中间层在持有 host_set 锁的情况下调用 hostt->queuecommand，因此上述所有
    动作都在持有 host_set 锁时发生。

### 命令如何处理


根据所使用的协议和控制器不同，命令的处理方式也不同。为了便于讨论，假设一个使用
任务文件接口和所有标准回调的控制器。

当前使用了 6 种 ATA 命令协议。根据它们的处理方式，可以分为以下四类。

ATA NO DATA 或 DMA
    ATA_PROT_NODATA 和 ATA_PROT_DMA 属于这一类。这类命令一经发出就不需要任何软件
    干预。设备会在完成时引发中断。

ATA PIO
    ATA_PROT_PIO 属于这一类。libata 目前使用轮询实现 PIO。设置 ATA_NIEN 位以关闭
    中断，ata_wq 上的 pio_task 执行轮询和 IO。

ATAPI NODATA 或 DMA
    ATA_PROT_ATAPI_NODATA 和 ATA_PROT_ATAPI_DMA 属于这一类。发出 PACKET 命令后，
    packet_task 用于轮询 BSY 位。一旦设备关闭 BSY，packet_task 就传输 CDB，并将处理
    移交给中断处理程序。

ATAPI PIO
    ATA_PROT_ATAPI 属于这一类。设置 ATA_NIEN 位，并且与 ATAPI NODATA 或 DMA 一样，
    packet_task 提交 cdb。然而，在提交 cdb 之后，进一步的处理（数据传输）被移交给
    pio_task。

### 命令如何完成


一旦发出，所有 qc 要么以 `ata_qc_complete` 完成，要么超时。对于由中断处理的命令，
`ata_host_intr` 调用 `ata_qc_complete`；对于 PIO 任务，pio_task 调用 `ata_qc_complete`。
在错误情况下，packet_task 也可能完成命令。

`ata_qc_complete` 执行以下操作。

1. 解除 DMA 内存映射。

2. 从 qc->flags 中清除 ATA_QCFLAG_ACTIVE。

3. 调用 :c`qc->complete_fn` 回调。如果回调的返回值不为零。完成被短路，
   `ata_qc_complete` 返回。

4. 调用 `__ata_qc_complete`，它会

   1. 将 `qc->flags` 清零。

   2. 毒化 `ap->active_tag` 和 `qc->tag`。

   3. 清除并完成 `qc->waiting`（按此顺序）。

   4. 通过清除 `ap->qactive` 中的相应位来释放 qc。

所以，它基本上是通知上层并释放 qc。一个例外是第 3 步中的短路路径，它由
`atapi_qc_complete` 使用。

对于所有非 ATAPI 命令，无论是否失败，几乎都走相同的代码路径，并进行非常少的错误处理。
如果成功，则 qc 以成功状态完成；否则以失败状态完成。

然而，失败的 ATAPI 命令需要更多处理，因为需要 REQUEST SENSE 来获取感知数据。如果
一个 ATAPI 命令失败，`ata_qc_complete` 会以错误状态被调用，而它又会通过 `qc->complete_fn()`
回调调用 `atapi_qc_complete`。

这使得 `atapi_qc_complete` 将 `scmd->result` 设置为 SAM_STAT_CHECK_CONDITION，完成
scmd 并返回 1。由于感知数据为空，但 `scmd->result` 为 CHECK CONDITION，SCSI 中间层将
对该 scmd 调用 EH，而返回 1 使得 `ata_qc_complete` 在不释放 qc 的情况下返回。这将我们
带到了带有部分完成 qc 的 `ata_scsi_error`。

### :c:func:`ata_scsi_error`


`ata_scsi_error` 是 libata 当前的 `transportt->eh_strategy_handler()`。如上所述，它会在
两种情况下被进入——超时和 ATAPI 错误完成。这个函数会检查是否有 qc 处于活跃状态且尚未
失败。这样的 qc 会被标记为 AC_ERR_TIMEOUT，以便 EH 稍后知道如何处理它。然后它调用
底层 libata 驱动的 `error_handler` 回调。

当 `error_handler` 回调被调用时，它会停止 BMDMA 并完成 qc。注意，由于当前处于 EH 中，
我们不能调用 scsi_done。如 SCSI EH 文档所述，一个被恢复的 scmd 应该要么用
`scsi_queue_insert` 重试，要么用 `scsi_finish_command` 完成。这里，我们用
`scsi_finish_command` 覆盖 `qc->scsidone` 并调用 `ata_qc_complete`。

如果 EH 是由于一个失败的 ATAPI qc 而被调用的，这里的 qc 已完成但未释放。这种半完成
的目的，是将 qc 作为占位符，使 EH 代码能够到达此处。这有点 hack，但确实有效。

一旦控制到达此处，qc 会通过显式调用 `__ata_qc_complete` 被释放。然后，为 REQUEST SENSE
发出内部 qc。一旦获取到感知数据，scmd 就通过直接在 scmd 上调用 `scsi_finish_command`
完成。注意，由于我们已经完成并释放了与 scmd 关联的 qc，我们不需要也不能再次调用
`ata_qc_complete`。

### 当前 EH 存在的问题


- 错误表示过于粗糙。当前任何和所有错误状况都由 ATA STATUS 和 ERROR 寄存器表示。不是
  ATA 设备错误的错误通过设置 ATA_ERR 位被当作 ATA 设备错误处理。需要一个能够更好地
  表示 ATA 和其他错误/异常的更好的错误描述符。

- 处理超时时，没有采取任何动作让设备忘记超时的命令，并为新命令做好准备。

- 通过 `ata_scsi_error` 进行的 EH 处理没有与常规命令处理正确隔离。进入 EH 时，设备
  并不处于静默状态。超时的命令可能随时成功或失败。pio_task 和 atapi_task 可能仍在
  运行。

- 错误恢复过于薄弱。导致 HSM 不匹配错误和其他错误的设备/控制器往往需要通过重置来
  返回到已知状态。此外，为了支持 NCQ 和热插拔等特性，也需要高级错误处理。

- ATA 错误直接在中断处理程序中处理，PIO 错误在 pio_task 中处理。这对于高级错误处理
  来说是有问题的，原因如下。

  首先，高级错误处理通常需要上下文和内部 qc 执行。

  其次，即使是一个简单的失败（比如 CRC 错误）也需要信息收集，并可能触发复杂的错误
  处理（比如重置和重新配置）。拥有多条代码路径来收集信息、进入 EH 和触发动作会让
  生活变得痛苦。

  第三，分散的 EH 代码使得实现底层驱动变得困难。底层驱动会覆盖 libata 回调。如果 EH
  分散在多个地方，每个受影响的回调都应执行其错误处理的部分。这容易出错且痛苦。

## libata 库


   :export:

## libata 核心内部实现


   :internal:


## libata SCSI 转换/模拟


   :export:

   :internal:

## ATA 错误与异常


本章试图识别 ATA/ATAPI 设备存在哪些错误/异常状况，并以实现无关的方式描述应如何处理
它们。

术语“error（错误）”用于描述设备报告了显式错误状况或命令已超时的情况。

术语“exception（异常）”要么用于描述非错误的异常状况（例如电源或热插拔事件），要么
用于描述错误和非错误异常状况。在需要明确区分错误和异常的地方，使用术语“non-error
exception（非错误异常）”。

### 异常类别


异常的描述主要相对于传统的任务文件 + 总线主控 IDE 接口。如果一个控制器提供了其他
更好的错误报告机制，将这些映射到下面描述的类别应该不难。

在以下各节中，提到了两个恢复动作——重置和重新配置传输。它们在
`EH recovery actions <#exrec>`__ 中有进一步描述。

#### HSM 违例


当在发出或执行任何 ATA/ATAPI 命令期间，STATUS 值不符合 HSM 要求时，指示此错误。

- 在试图发出一个命令时，ATA_STATUS 不包含 !BSY && DRDY && !DRQ。

- PIO 数据传输期间的 !BSY && !DRQ。

- 命令完成时的 DRQ。

- 在 CDB 传输开始之后、但 CDB 最后一个字节传输之前，!BSY && ERR。ATA/ATAPI 标准在
  PACKET 命令的错误输出描述中声明“设备不应在命令包最后一个字节被写入之前以错误终止
  PACKET 命令”，并且状态图中不包含这样的转换。

在这些情况下，HSM 被违例，并且无法从 STATUS 或 ERROR 寄存器获取关于该错误的太多信息。
换言之，这个错误可以是任何东西——驱动 bug、故障设备、控制器和/或线缆。

由于 HSM 被违例，需要重置来恢复已知状态。将传输重新配置为更低的速度可能也有帮助，
因为传输错误有时会导致这类错误。

#### ATA/ATAPI 设备错误（非 NCQ / 非 CHECK CONDITION）


这些是由 ATA/ATAPI 设备检测并报告的错误，表明设备问题。对于这类错误，STATUS 和
ERROR 寄存器值是有效的，并描述错误状况。注意，某些 ATA 总线错误由 ATA/ATAPI 设备检测
到，并使用与设备错误相同的机制报告。这些情况在本节后面描述。

对于 ATA 命令，这类错误在命令执行期间和完成时由 !BSY && ERR 指示。

对于 ATAPI 命令，

- 发出 PACKET 之后立即出现 !BSY && ERR && ABRT，表示 PACKET 命令不受支持，并属于
  这一类。

- 在 CDB 最后一个字节传输之后出现 !BSY && ERR(==CHK) && !ABRT，表示 CHECK CONDITION，
  不属于这一类。

- 在 CDB 最后一个字节传输之后出现 !BSY && ERR(==CHK) && ABRT，\**probably\** 表示
  CHECK CONDITION，不属于这一类。

在上述检测到的错误中，以下不是 ATA/ATAPI 设备错误，而是 ATA 总线错误，应按
`ATA bus error <#excatATAbusErr>`__ 处理。

数据传输期间的 CRC 错误
    这由 ERROR 寄存器中的 ICRC 位指示，意味着数据传输期间发生了损坏。直到 ATA/ATAPI-7，
    标准规定该位仅适用于 UDMA 传输，但 ATA/ATAPI-8 草案修订版 1f 说该位可能适用于
    多字 DMA 和 PIO。

数据传输期间或完成时的 ABRT 错误
    直到 ATA/ATAPI-7，标准规定 ABRT 可以在 ICRC 错误以及设备无法完成命令的情况下被
    设置。结合 MWDMA 和 PIO 传输错误直到 ATA/ATAPI-7 都不允许使用 ICRC 位这一事实，
    这似乎暗示 ABRT 位单独就可以表示传输错误。

    然而，ATA/ATAPI-8 草案修订版 1f 移除了 ICRC 错误可以打开 ABRT 的部分。所以，这
    属于灰色地带。这里需要一些启发式方法。

ATA/ATAPI 设备错误可进一步分类如下。

介质错误
    这由 ERROR 寄存器中的 UNC 位指示。ATA 设备仅在若干次重试都无法恢复数据后才报告
    UNC 错误，因此除了通知上层外，没有其他太多可做的。

    READ 和 WRITE 命令会报告第一个失败扇区的 CHS 或 LBA，但 ATA/ATAPI 标准规定错误
    完成时传输的数据量是不确定的，因此我们不能假定失败扇区之前的扇区已被传输，从而
    不能像 SCSI 那样成功完成那些扇区。

介质已更改 / 请求介质更改错误
    <<TODO: fill here>>

地址错误
    这由 ERROR 寄存器中的 IDNF 位指示。上报给上层。

其他错误
    这可能是由 ABRT ERROR 位指示的无效命令或参数，或其他一些错误状况。注意，ABRT 位
    可以指示很多事情，包括 ICRC 和地址错误。需要启发式方法。

根据命令不同，并非所有 STATUS/ERROR 位都适用。这些不适用的位在输出描述中用 “na”
标记，但直到 ATA/ATAPI-7 都没有找到 “na” 的定义。然而，ATA/ATAPI-8 草案修订版 1f
如下描述 “N/A”。

    3.2.3.3a N/A
        一个关键字，表示该字段在本标准中没有定义的值，且不应被主机或设备检查。
        N/A 字段应被清零。

所以，合理假设设备会将 “na” 位清零，因此不需要显式屏蔽。

#### ATAPI 设备 CHECK CONDITION


ATAPI 设备 CHECK CONDITION 错误由 PACKET 命令的 CDB 最后一个字节传输之后，STATUS 寄存器
中设置的 CHK 位（ERR 位）指示。对于这类错误，应获取感知数据以收集关于错误的信息。
应使用 REQUEST SENSE packet 命令来获取感知数据。

一旦获取到感知数据，这类错误就可以类似于其他 SCSI 错误来处理。注意，感知数据可能指示
ATA 总线错误（例如 Sense Key 04h HARDWARE ERROR && ASC/ASCQ 47h/00h SCSI PARITY ERROR）。
在这种情况下，该错误应被视为 ATA 总线错误，并按 `ATA bus error <#excatATAbusErr>`__
处理。

#### ATA 设备错误（NCQ）


NCQ 命令错误在 NCQ 命令阶段（一个或多个 NCQ 命令未完成）由清除的 BSY 和设置的 ERR 位
指示。尽管 STATUS 和 ERROR 寄存器将包含描述错误的有效值，但需要 READ LOG EXT 来清除
错误状况、确定哪个命令失败并获取更多信息。

READ LOG EXT 日志页 10h 报告哪个标签失败以及描述该错误的任务文件寄存器值。有了这些信息，
失败的命令可以像 `ATA/ATAPI device error (non-NCQ / non-CHECK CONDITION) <#excatDevErr>`__
中的普通 ATA 命令错误一样处理，并且所有其他在途命令必须被重试。注意，这个重试不应
被计数——如果不是因为失败的命令，以这种方式重试的命令很可能已经正常完成了。

注意，ATA 总线错误也可以报告为 ATA 设备 NCQ 错误。这应按 `ATA bus error
<#excatATAbusErr>`__ 处理。

如果 READ LOG EXT 日志页 10h 失败或报告 NQ，我们就彻底完蛋了。这种情况应按
`HSM violation <#excatHSMviolation>`__ 处理。

#### ATA 总线错误


ATA 总线错误意味着在 ATA 总线（SATA 或 PATA）上传输期间发生了数据损坏。这类错误可以
由以下指示：

- `ATA/ATAPI device error (non-NCQ / non-CHECK CONDITION) <#excatDevErr>`__ 中描述的
  ICRC 或 ABRT 错误。

- 带有指示传输错误的错误信息的控制器特定错误完成。

- 在某些控制器上，命令超时。在这种情况下，可能存在一种机制来确定超时是由于传输错误
  引起的。

- 未知/随机错误、超时以及各种各样的怪异现象。

如上所述，传输错误可能引起从设备 ICRC 错误到随机设备锁死等各种各样的症状，并且在许多
情况下，无法判断错误状况是否由于传输错误引起；因此，在处理错误和超时时需要采用某种
启发式方法。例如，对于已知受支持的命令反复遇到 ABRT 错误，很可能表明 ATA 总线错误。

一旦确定可能已经发生了 ATA 总线错误，降低 ATA 总线传输速度是可能缓解问题的一个动作。
参见 `Reconfigure transport <#exrecReconf>`__ 获取更多信息。

#### PCI 总线错误


在 PCI（或其他系统总线）上传输期间的数据损坏或其他故障。对于标准 BMDMA，这由 BMDMA
Status 寄存器中的 Error 位指示。这类错误必须被记录，因为它表明系统出现了非常严重的
问题。建议重置主机控制器。

#### 迟到完成


当发生超时，且超时处理程序发现超时的命令已经成功完成或带有错误完成时，就会出现这种
情况。这通常是由丢失的中断引起的。这类错误必须被记录。建议重置主机控制器。

#### 未知错误（超时）


这是指发生了超时，而命令仍在处理中，或者主机和设备处于未知状态。发生这种情况时，HSM
可能处于任何有效或无效状态。为了使设备回到已知状态并让它忘记超时的命令，重置是必要的。
超时的命令可以被重试。

超时也可能由传输错误引起。更多细节参见 `ATA bus error <#excatATAbusErr>`__。

#### 热插拔与电源管理异常


<<TODO: fill here>>

### EH 恢复动作


本节讨论几个重要的恢复动作。

#### 清除错误状况


许多控制器要求它的错误寄存器被错误处理程序清除。不同的控制器可能有不同的要求。

对于 SATA，强烈建议在错误处理期间至少清除 SError 寄存器。

#### 重置


在 EH 期间，在以下情况下重置是必要的。

- HSM 处于未知或无效状态

- HBA 处于未知或无效状态

- EH 需要让 HBA/设备忘记在途命令

- HBA/设备行为怪异

无论错误状况如何，在 EH 期间进行重置可能都是个好主意，以提高 EH 的健壮性。是重置 HBA
和设备两者，还是仅重置其中之一，取决于具体情况，但推荐以下方案。

- 当已知 HBA 处于就绪状态，但 ATA/ATAPI 设备处于未知状态时，仅重置设备。

- 如果 HBA 处于未知状态，则重置 HBA 和设备两者。

HBA 重置是实现特定的。对于符合任务文件/BMDMA PCI IDE 的控制器，停止活跃的 DMA 事务
可能就足够了——前提是 BMDMA 状态是唯一的 HBA 上下文。但即使是大多符合任务文件/BMDMA
PCI IDE 的控制器，也可能有实现特定的要求和重置自身的机制。这必须由特定驱动来解决。

另一方面，ATA/ATAPI 标准详细描述了重置 ATA/ATAPI 设备的方法。

PATA 硬件重置
    这是由断言的 PATA RESET- 信号发出的、硬件发起的设备重置。虽然没有标准的软件方式
    来发起硬件重置，但某些硬件提供了允许驱动直接操纵 RESET- 信号的寄存器。

软件重置
    这是通过打开 CONTROL SRST 位至少 5us 来实现的。PATA 和 SATA 都支持它，但在 SATA
    的情况下，这可能需要控制器特定的支持，因为应在 BSY 位仍被设置的同时传输用于清除
    SRST 的第二个 Register FIS。注意，在 PATA 上，这会重置通道上的主设备和从设备两者。

EXECUTE DEVICE DIAGNOSTIC 命令
    尽管 ATA/ATAPI 标准没有精确描述，EDD 暗示了某种程度的重置，可能类似于软件重置。
    主机侧的 EDD 协议可以用常规命令处理，并且大多数 SATA 控制器应该能像处理其他命令
    一样处理 EDD。与软件重置一样，EDD 影响 PATA 总线上的两个设备。

    虽然 EDD 确实重置设备，但这不适合错误处理，因为当 BSY 被设置时无法发出 EDD，并且
    当设备处于未知/怪异状态时它将如何表现也不明确。

ATAPI DEVICE RESET 命令
    这与软件重置非常相似，只是重置可以被限制在所选设备上，而不影响共享线缆的其他
    设备。

SATA phy 重置
    这是重置 SATA 设备的首选方式。实际上，它等同于 PATA 硬件重置。注意，这可以通过标准
    的 SCR Control 寄存器完成。因此，它通常比软件重置更容易实现。

重置设备时还有一件事要考虑，那就是重置会清除某些配置参数，需要在重置后将它们设置为
先前或新调整的值。

受影响的参数有。

- 用 INITIALIZE DEVICE PARAMETERS 设置的 CHS（很少使用）

- 用 SET FEATURES 设置的参数，包括传输模式设置

- 用 SET MULTIPLE MODE 设置的块计数

- 其他参数（SET MAX、MEDIA LOCK...）

ATA/ATAPI 标准规定某些参数应在硬件或软件重置期间保持，但并未严格规定所有参数。为了
健壮性，总是需要在重置后重新配置所需参数。注意，这也适用于从深度睡眠（断电）恢复时。

此外，ATA/ATAPI 标准要求在更新任何配置参数或进行一次硬件重置之后，发出 IDENTIFY
DEVICE / IDENTIFY PACKET DEVICE，并将结果用于进一步操作。OS 驱动需要实现重新验证机制
来支持这一点。

#### 重新配置传输


对于 PATA 和 SATA，廉价的连接器、线缆或控制器会省略很多环节，因此看到高传输错误率是
相当常见的。这可以通过降低传输速度来缓解。

以下是 Jeff Garzik 建议的一种可能方案。

    如果 15 分钟内发生超过 $N（3？）次传输错误，

    - 如果是 SATA，降低 SATA PHY 速度。如果无法再降低速度，

    - 降低 UDMA xfer 速度。如果已在 UDMA0，切换到 PIO4，

    - 降低 PIO xfer 速度。如果已在 PIO3，发出抱怨，但继续

## ata_piix 内部实现


   :internal:

## sata_sil 内部实现


   :internal:

## 致谢


大量的 ATA 知识得益于与 Andre Hedrick（www.linux-ide.org）的长谈，以及长时间对 ATA 和
SCSI 规范的研究。

感谢 Alan Cox 指出了 SATA 和 SCSI 之间的相似之处，并总体上为 hack libata 提供了动力。

libata 的设备检测方法 ata_pio_devchk，以及总体上所有早期的探测，都是基于对 Hale Landis
在其 ATADRVR 驱动（www.ata-atapi.com）中的 probe/reset 代码的广泛研究。
