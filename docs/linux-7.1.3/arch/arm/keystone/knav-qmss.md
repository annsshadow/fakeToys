## Texas Instruments Keystone Navigator 队列管理子系统（QMSS）驱

椹卞姩婧愪唬鐮佽矾寰?  drivers/soc/ti/knav_qmss.c
  drivers/soc/ti/knav_qmss_acc.c

Keystone SoC 上的 QMSS（队列管理子系统，Queue Manager Sub System）是构成 Keystone 多核 Navigator 骨干的主要硬件子系统之一。QMSS 由队列管理器、打包数据结构处理器（PDSP）、链RAM、描述符池以及基础设施 Packet DMA 组成队列管理器是一个硬件模块，负责加速数据包队列的管理。数据包通过向特定的内存映射地址写入或读取描述符地址来进行入出队操作。PDSP 执行QMSS 相关的功能，如累积（accumulation）、QoS 或事件管理。链RAM 寄存器用于链接存储在描述RAM 中的描述符。描述符 RAM 可配置为内部或外部内存。QMSS 驱动负责管理 PDSP 的设置、链RAM 区域、队列池管理（分配、压入、弹出与通知）以及描述符池管理
knav qmss 驱动向其他驱动提供一API，用于打开/关闭 qmss 队列、分配描述符池、映射描述符、向队列压入/弹出等。有关可API 的详细信息，请参include/linux/soc/ti/knav_qmss.h

DT 文档位于
Documentation/devicetree/bindings/soc/ti/keystone-navigator-qmss.txt

## 使用 PDSP 固件的累积器 QMSS 队列

QMSS PDSP 固件支持累积器通道，可监视单个队列或多个连续的队列。drivers/soc/ti/knav_qmss_acc.c 是与累积PDSP 交互的驱动。它会配DTS（参DT 文档中的示例）中定义的累积器通道，以每个通道监视 1 32 个队列。有关该固件的更多说明，可查CPPI/QMSS 低层驱动文档（docs/CPPI_QMSS_LLD_SDS.pdf），位于

	git://git.ti.com/keystone-rtos/qmss-lld.git

k2_qmss_pdsp_acc48_k2_le_1_0_0_9.bin 固件最多支48 个累积器通道。该固件位于 firmware.git ti-keystone 目录下，地址
   git://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git

使用时，请将固件镜像复制initramfs ubifs 文件系统lib/firmware 目录，并在文件系统中k2_qmss_pdsp_acc48_k2_le_1_0_0_9.bin 提供符号链接，然后启动内核。若固件成功加载PDSP，用户将在启动日志中看到

 "firmware file ks2_qmss_pdsp_acc48.bin downloaded for PDSP"

使用累积队列要求固件镜像存在于文件系统中。如SoC 中的 PDSP 未运行，驱动不会将累积队列加入受支持的队列范围。如果对累积队列发起队列打开请求PDSP 未运行，API 调用会失败。因此，在使用这些队列类型之前，务必将固件复制到文件系统中