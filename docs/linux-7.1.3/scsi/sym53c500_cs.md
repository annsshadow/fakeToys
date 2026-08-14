## sym53c500_cs 驱动


sym53c500_cs 驱动最初是作为 David Hinds 的 pcmcia-cs 软件包的附加组件开发的，由 Tom Corner (tcorner@via.at) 编写。对其进行重写早已是当务之急，当前版本解决了以下问题：

	(1) 2.4 与 2.6 内核之间大量的内核改动。
	(2) 内核之外的 PCMCIA 支持已被弃用。

所有 USE_BIOS 代码均已被移除。这些代码从未被使用过，而且本来也无法工作。USE_DMA 代码也同样被移除。非常感谢 YOKOTA Hiroshi（nsp_cs 驱动）和 David Hinds（qlogic_cs 驱动）提供的代码片段，我在本工作中毫不客气地加以借鉴。同时也感谢 Christoph Hellwig 在我摸索过程中给予的耐心指导。

Symbios Logic 53c500 芯片被用于 New Media Bus Toaster PCMCIA SCSI 控制器的“较新”（约 1997 年）版本中。想必还有其他产品也使用了该芯片，但我从未亲眼见过（更别提亲手接触过）这样的产品。

多年来，该驱动的 pcmcia-cs 版本被多次下载，我想它对那些用户是有效的。它对 Tom Corner 有效，对我也有效。你的使用体验可能会有所不同。

Bob Tracy (rct@frus.com)
