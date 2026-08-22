## Marvell CN10K DMA 数据包接口（DPI）驱

## 概述


DPI Marvell CN10K 芯片中的 DMA 数据包接口硬件模块。DPI 硬件包含一个物理功（PF）、其虚拟功能、邮箱逻辑，以及一DMA 引擎DMA 命令队列
DPI PF 功能是一个管理功能，它处理来自其 VF 功能的邮箱请求，并向VF 功能分配
DMA 引擎资源
mrvl_cn10k_dpi.ko misc 驱动DPI PF 设备上加载，并处VF 设备提交的邮箱命令，
相应地初始化 DMA 引擎VF 设备DMA 命令队列。此外，驱动创建 /dev/mrvl-cn10k-dpi
节点，用于设DMA 引擎PEM（PCIe 接口）端口属性，fifo 长度、molr、mps mrrs
DPI PF 驱动只是一个用于设置其 VF 设备队列并分配硬件资源的管理驱动，它不能发起
任何 DMA 操作。只VF 设备被分配了 DMA 能力
## 驱动位置


drivers/misc/mrvl_cn10k_dpi.c

## 驱动 IOCTL


`DPI_MPS_MRRS_CFG`
ioctl，用于设DMA 引擎所连接pem 端口的最大有效负载大小和最大读请求大小
参数
`DPI_ENGINE_CFG`
ioctl，用于设DMA 引擎fifo 大小和最大未完成加载请求阈值
## 用户空间代码示例


DPI VF 设备通过 vfio-pci 驱动从用户空间应用程序探测和访问。下面是一个示dpi
dma 应用程序，演示应用程序如何使用来DPI PF 内核驱动的邮箱和 ioctl 服务
https://github.com/MarvellEmbeddedProcessors/dpi-sample-app
