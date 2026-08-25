
## PCI 错误恢复


:Authors: - Linas Vepstas <linasvepstas@gmail.com>
          - Richard Lary <rlary@us.ibm.com>
          - Mike Mason <mmlnx@us.ibm.com>


许多 PCI 总线控制器能够检测总线上各种硬PCI 错误，例如数据和地址总线上的奇偶
校验错误，以SERR PERR 错误。一些更先进的芯片组能够处理这些错误；这些包PCIe 芯片组，以及 IBM Power4、Power5 和基Power6 pSeries 机器上的 PCI 主机桥一个典型的动作是断开受影响的设备，停止对其的所I/O。断开的目的是避免系统损坏例如，防止由于对"地址DMA 而导致系统内存损坏。通常也会提供一种重新连接机制，
以便受影响的 PCI 设备被复位并恢复到工作状态。复位阶段需要受影响的设备驱动和 PCI
控制器芯片之间进行协调。本文档描述了一种通用 API，用于将总线断开通知设备驱动，然执行错误恢复。该 API 目前已在 2.6.16 及更高版本的内核中实现
报告和恢复分几个步骤执行。首先，PCI 硬件错误导致总线断开时，该事件会尽快报告所有受影响的设备驱动，包括多功能卡上设备驱动的多个实例。这使得设备驱动可以避免自旋循环中死锁，等待某个 I/O 空间寄存器改变，而它永远不会改变。这也为驱动提供了在
需要时推迟传入 I/O 的机会
接下来，恢复分几个阶段执行。大部分复杂性是由需要处理多功能设备（即具有多个关联
设备驱动的设备）的需要所强加的。在第一阶段，允许每个驱动指示它想要的复位类型，
选择要么是简单地重新启用 I/O，要么是请求插槽复位
如果任何驱动请求插槽复位，那就会执行插槽复位
在复位和/或重新启I/O 之后，所有驱动再次被通知，以便它们可以执行可能需要的任何
设备设置/配置。在这些都完成之后，会发出一个最终的"恢复正常运行"事件
选择基于内核的实现而非用户空间实现的最大原因，是需要处理连接到存储媒体PCI 设备
的总线断开，特别是持有根文件系统的设备的断开。如果根文件系统被断开，用户空间机将不得不经历大量曲折才能完成恢复。几乎所有当前的 Linux 文件系统都不能容忍与其底块设备的断开/重连。相比之下，总线错误在设备驱动中很容易管理。事实上，大多数设备驱动
已经处理了非常相似的恢复过程；例如，SCSI 通用层已经提供了处理 SCSI 总线错误SCSI
总线复位的重大机制

## 详细设计


下面的设计和实现细节，基circa 5 April 2005 Ben Herrenschmidt 的一系列公开邮件
讨论
错误恢复 API 支持struct pci_driver 中新字段所指向的函数指针结构的形式暴露给驱动未能提供该结构的驱动无感的，所采取的实际恢复步骤取决于平台。arch/powerpc 实现将模拟一PCI 热插拔移添加
```

	struct pci_error_handlers
	{
		int (*error_detected)(struct pci_dev *dev, pci_channel_state_t);
		int (*mmio_enabled)(struct pci_dev *dev);
		int (*slot_reset)(struct pci_dev *dev);
		void (*resume)(struct pci_dev *dev);
		void (*cor_error_detected)(struct pci_dev *dev);
	};

```
```

	typedef enum {
		pci_channel_io_normal,  /* I/O channel is in normal state */
		pci_channel_io_frozen,  /* I/O to channel is blocked */
		pci_channel_io_perm_failure, /* PCI card is dead */
	} pci_channel_state_t;

```
```

	enum pci_ers_result {
		PCI_ERS_RESULT_NONE,        /* no result/none/not supported in device driver */
		PCI_ERS_RESULT_CAN_RECOVER, /* Device driver can recover without slot reset */
		PCI_ERS_RESULT_NEED_RESET,  /* Device driver wants slot to be reset. */
		PCI_ERS_RESULT_DISCONNECT,  /* Device has completely failed, is unrecoverable */
		PCI_ERS_RESULT_RECOVERED,   /* Device driver is fully recovered and operational */
	};

```
驱动不必实现所有这些回调；但是，如果它实现了其中任何一个，就必须实error_detected()如果某个回调未实现，则相应的特性被视为不受支持。例如，如果 mmio_enabled() resume()
不存在，则假定驱动在恢复时不需要这些回调。通常驱动会想知道 slot_reset()
平台PCI 错误事件恢复所采取的实际步骤将取决于平台，但会遵循下面描述的一般顺序
### STEP 0: 错误事件

PCI 硬件检测到一PCI 总线错误。在 powerpc 上，插槽被隔离，即所I/O 被阻止：所读取返回 0xffffffff，所有写入被忽略
类似地，在支持下游端口遏制（Downstream Port Containment，PCIe r7.0 sec 6.2.11）的平台
上，到包含故障设备的子层次的链路被禁用。子层次中的任何设备都变得不可访问
### STEP 1: 通知

平台在每个受错误影响的驱动的每个实例上调error_detected() 回调
此时，根据平台的不同，设备可能不再可访问（在 powerpc 上插槽将被隔离）。驱动可能已因为一次失败的 I/O 注意了错误，但这是一个适当同步，也就是说，它给驱动
一个机会进行清理，等待待处理的东西（定时器、等等……）完成；它可以获取信号量、调度等…除了不能触碰设备之外的一切。在这个函数内部以及它返回之后，驱动不应再进行任何新IO在任务上下文中调用。这算是一静止"点。参见本文档末尾关于中断的说明
参与此系统的所有驱动必须实现此调用。驱动必须返回以下结果代码之一
  - PCI_ERS_RESULT_RECOVERED
      如果驱动认为设备尽管有错误但仍可用，并且不需要进一步干预，则返回此值  - PCI_ERS_RESULT_CAN_RECOVER
      如果驱动认为它可能仅通过敲打 IO 就能恢复 HW，或者它希望被给予提取一些诊      信息的机会（见下面的 mmio_enable），则返回此值  - PCI_ERS_RESULT_NEED_RESET
      如果它不通过插槽复位就无法恢复，则返回此值  - PCI_ERS_RESULT_DISCONNECT
      如果它根本不想恢复，则返回此值
下一步将取决于驱动返回的结果代码
如果插槽上的所有驱动都返回 PCI_ERS_RESULT_CAN_RECOVER，那么平台应该重新启用插槽上IO（或者如果平台不隔离插槽，则什么都不特别做），并且恢复进入 STEP 2（MMIO 启用）
如果任何驱动请求插槽复位（通过返回 PCI_ERS_RESULT_NEED_RESET），则恢复进STEP 4
（插槽复位）
如果平台无法恢复插槽，下一步是 STEP 6（永久故障）

   当前powerpc 实现假设设备驱动不会在此例程中调度或获取信号量；当前powerpc 实现
   使用一个内核线程来通知所有设备；因此，如果一个设备休调度，所有设备都会受影响   做得更好需要在错误恢复实现中使用复杂的多线程逻辑（例如，在继续恢复之前等待所有通知
   线程"会合"）。这似乎过于复杂，不值得实现
   当前powerpc 实现并不太在意设备此时是否尝I/O。I/O 会失败，读取返回 0xff 值，
   写入会被丢弃。如果对冻结的适配器尝试超EEH_MAX_FAILS I/O，EEH 会假设设备驱   进入了无限循环，并向 syslog 打印一个错误。然后需要重启才能使设备再次工作
### STEP 2: MMIO 启用

平台重新启用到设备的 MMIO（但通常不是 DMA），然后在所有受影响的设备驱动上调用
mmio_enabled() 回调
这是"早期恢复"调用。再次允IO，但 DMA 不允许，有一些限制。这不是驱动重新开始操作的
回调，只是窥拨弄设备、提取诊断信息（如果有），并最终做类似触发设备本地复位之类事情，但不重启操作。如果段上的所有驱动都同意它们可以尝试恢复，并HW 没有执行自动链路
复位，则会进行此回调。如果平台不能在没有插槽复位或链路复位的情况下仅重新启用 IO，它不会调用此回调，而是直接进入 STEP 3（链路复位）STEP 4（插槽复位）

   在支持高级错误报告（Advanced Error Reporting，PCIe r7.0 sec 6.2）的平台上，故障设备
   STEP 1（通知）中可能已经可访问。尽管如此，驱动应将访问推迟STEP 2（MMIO 启用），
   以兼powerpc 上的 EEH s390（在这些平台上设备直STEP 2 才可访问）
   在支持下游端口遏制的平台上，到包含故障设备的子层次的链路STEP 3（链路复位）中被重新
   启用。因此子层次中的设备直到 STEP 4（插槽复位）都不可访问
   对于 Surprise Down（PCIe r7.0 sec 6.2.7）等错误，设备在 STEP 4（插槽复位）中可能甚   不可访问。驱动可以通过检查对设备的读取是否返回全 1（PCI_POSSIBLE_ERROR()）来检测可访问性

   以下为建议；目前尚无平台实现   建议：所I/O 都应在此回调内_同步_完成，它们触发的错误将通过正常pci_check_whatever()
   API 返回，不会由于此处发生的错误而发出新error_detected() 回调。然而，这样的错误可   导致整个段的 IO 被重新阻止，从而使同一段上其他设备可能已经完成的恢复失效，迫使整个   进入下一个状态之一，即链路复位或插槽复位
驱动应返回以下结果代码之一  - PCI_ERS_RESULT_RECOVERED
      如果驱动认为设备已完全正常工作并准备好再次开始正常的驱动操作，则返回此值。不      保证驱动实际上会被允许继续，因为同一段上的另一个驱动可能已失败，从而在支持它的
      平台上触发了插槽复位
  - PCI_ERS_RESULT_NEED_RESET
      如果驱动认为设备在其当前状态下不可恢复，并且需要一个插槽复位才能继续，则返回此值
  - PCI_ERS_RESULT_DISCONNECT
      同上。完全失败，即使复位后驱动也已死。（有待更精确的定义
下一步取决于驱动返回的结果。如果所有驱动都返回PCI_ERS_RESULT_RECOVERED，则平台进入
STEP 3（链路复位）STEP 5（恢复操作）
如果任何驱动返回PCI_ERS_RESULT_NEED_RESET，则平台进入 STEP 4（插槽复位）
### STEP 3: 链路复位

平台复位链路。这是一PCIe 特定的步骤，每当检测到可以通过复位链路解决"的致命错时执行
### STEP 4: 插槽复位


作为PCI_ERS_RESULT_NEED_RESET 返回值的响应，平台将对请求的 PCI 设备执行插槽复位平台执行插槽复位所采取的实际步骤将取决于平台。插槽复位完成后，平台将调用设备 slot_reset()
回调
Powerpc 平台实现了两个级别的插槽复位：软复位（默认）和基础（可选）复位
Powerpc 软复位包括断言适配器的 #RST 线，然后PCI BAR PCI 配置头恢复到一个等效于
全新系统加电后由加电 BIOS/系统固件初始化所达到的状态。软复位也称为热复位
Powerpc 基础复位仅由 PCIe 卡支持，其结果使设备的状态机、硬件逻辑、端口状态和配置寄存初始化为它们的默认条件
对于大多PCI 设备，软复位就足以恢复。提供可选的基础复位是为了支持少数软复位不足以恢复的
PCIe 设备
如果平台支持 PCI 热插拔，则可以通过切换插槽电气电源的开/关来执行复位
平台PCI 配置空间恢复全新加电"状态而非"最后状是很重要的。插槽复位后，设备驱几乎总是使用其标准设备初始化例程，而不寻常的配置空间设置可能导致设备挂起、内核恐慌或
静默数据损坏
此调用给驱动一个重新初始化硬件（重新下载固件等）的机会。此时，驱动可以假设卡处于全状态且完全正常工作。插槽已解除冻结，驱动可以完全访PCI 配置空间、内存映I/O 空间DMA。中断（Legacy、MSI MSI-X）也将可用
驱动此时不应重启正常I/O 处理操作。如果所有设备驱动在此回调上都报告成功，平台将调resume() 来完成序列，并让驱动重启正常I/O 处理
如果驱动在复位后仍无法使设备正常工作，它仍可以为此函数返回严重失败。如果平台之前尝试了
软复位，它现在可能会尝试硬复位（电源循环），然后再次调用 slot_reset()。如果设备仍然无恢复，就无计可施了；在这种情况下平台通常会报永久故障"。此时设备将被视已死"
驱动通常需要在复位后调pci_restore_state() 来重新初始化设备的配置空间寄存器，从将其D0\ `uninitialized` 状态带D0\ `active` 状态（PCIe r7.0 sec 5.3.1.1）。PCI 核心
在枚举时初始化配置空间后调用 pci_save_state()，以确保有保存的状态可用于随后的错误恢复probe 时修改配置空间的驱动可能需要在之后调用 pci_save_state() 以记录这些更改供后续错误
恢复使用。在进入系统挂起时，会为每个 PCI 设备调用 pci_save_state()，该状态不仅会在恢复时
恢复，也会在任何后续错误恢复时恢复。在极少数情况下，挂起时记录的保存状态不适合错误恢复驱动应在恢复时调pci_save_state()
多功能卡的驱动需要在彼此之间协调，由哪个驱动实例执行任何"一次或全局设备初始化。例如，
Symbios sym53cxx2
```

	+       if (PCI_FUNC(pdev->devfn) == 0)
	+               sym_reset_scsi_bus(np, 0);

```
结果代码 - PCI_ERS_RESULT_DISCONNECT
	  同上
需要基础复位PCIe 卡驱动必须在probe 函数中设pci_dev 结构中的 needs_freset 位例如，QLogic qla2xxx 驱动为某```

	+	/* Set EEH reset type to fundamental if required by hba  */
	+	if (IS_QLA24XX(ha) || IS_QLA25XX(ha) || IS_QLA81XX(ha))
	+		pdev->needs_freset = 1;
	+

```
平台进入 STEP 5（恢复操作）STEP 6（永久故障）

   当前powerpc 实现在驱动返PCI_ERS_RESULT_DISCONNECT 时不会尝试电源循环复位。然而，
   它或许应该尝试

### STEP 5: 恢复操作

如果段上的所有驱动都从前面三个回调之一返回PCI_ERS_RESULT_RECOVERED，平台将在所受影响的设备驱动上调resume() 回调。此回调的目的是告诉驱动重新开始活动，一切都已恢并运行。此回调不返回结果代码
此时，如果发生新的错误，平台将重新开始一个新的错误恢复序列
### STEP 6: 永久故障

发生永久故障"，平台无法恢复设备。平台将调用 error_detected()，其 pci_channel_state_t
值为 pci_channel_io_perm_failure
此时，设备驱动应做最坏的打算。它应取消所有待处理I/O，拒绝所有新I/O，向更高层返-EIO。设备驱动然后应清理其所有内存并从内核操作中移除自身，就像在系统关机期间那样
平台通常会以某种方式通知系统操作员发生了永久故障。如果设备支持热插拔，操作员可能想要
移除并更换设备。但请注意，并非所有故障都是真正的"永久"故障。有些是由过热引起的，有些是
由卡没有插好引起的。许PCI 错误事件是由软件错误引起的，例如对野地址DMA，或因编错误导致的虚假拆分事务。有关软件错误原因的更多真实经验细节，请参见
Documentation/arch/powerpc/eeh-pci-error-recovery.rst 中的讨论

### 结论；一般说
回调的调用方式是平台策略。没有插槽复位能力的平台可能想要简单地"忽略"无法恢复的驱（断开它们），并尝试让同一段上的其他卡恢复。但请记住，在大多数实际情况中，每个段只一个驱动
现在，关于中断的一点说明。如果你收到一个中断而你的设备已死或被隔离，那就出问题了 :)
当前的策略是将此变成一种平台策略。也就是说，恢复 API 只要求：

 - 从错误检测到调用 slot_reset 回调之前，不能保证段上任何设备的中断投递能够继续，   该点中断应当完全正常运行
 - 不能保证中断投递被停止，也就是说，在检测到错误之后收到中断的驱动，或者在中断处理程序
   中检测到错误从而阻止了中断的正ack（进而移除源）的驱动，应直接返回 IRQ_NOTHANDLED   如何处理这种情况由平台负责，通常的做法是在错误处理期间屏IRQ 源。期望平知道"哪些
   中断被路由到具备错误管理能力的插槽，并且能够在错误处理期间临时禁用该 IRQ 号（这并   太复杂）。这意味着共享该中断的其他设备会有一IRQ 延迟，但实在没有其他办法。高端平   本来就不应该在许多设备之间共享中:)


   powerpc 平台的实现细节在文件 Documentation/arch/powerpc/eeh-pci-error-recovery.rst    讨论
   截至本文撰写时，实现错误恢复的补丁的设备驱动列表在不断增长。并非所有这些补丁都已在
   mainline 中。这些可以作示例"使用
   - drivers/scsi/ipr
   - drivers/scsi/sym53c8xx_2
   - drivers/scsi/qla2xxx
   - drivers/scsi/lpfc
   - drivers/next/bnx2.c
   - drivers/next/e100.c
   - drivers/net/e1000
   - drivers/net/e1000e
   - drivers/net/ixgbe
   - drivers/net/cxgb3

   cor_error_detected() 回调handle_error_source() 中当错误严重度为"可纠时被调用   该回调是可选的，允许在需要时进行额外的日志记录。参见示例：

   - drivers/cxl/pci.c

### 结束
