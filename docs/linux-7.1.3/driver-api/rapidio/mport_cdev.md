## RapidIO 子系mport 字符设备驱动（rio_mport_cdev.c

## 1. 概述


该设备驱动是 RapidIO.org 软件任务组（STG）内 Texas Instruments、FreescaleProdrive Technologies、Nokia Networks、BAE IDT 之间协作的成果。还收到来自 RapidIO.org 其他成员的其他输入。其目标是创建一个字符模式驱动接口，
以允许众多且各异RapidIO 实现能够互操作的方式，将 RapidIO 设备能力直接暴露给应用程序
该驱动（MPORT_CDEV）为用户空间应用程序提供对基RapidIO 子系统操作的
访问。大多数 RapidIO 操作通过 'ioctl' 系统调用支持
加载该设备驱动后，它会为每一个已注册RapidIO mport 设备/dev 目录创建名为 rio_mportX 的文件系统节点。节点名中的 'X' 与分配给每个本地
mport 设备的唯一端口 ID 相匹配
使用可用的一ioctl 命令，用户空间应用程序可以执行以RapidIO 总线子系统操作：

- mport 设备的配置寄存器读取和写  （RIO_MPORT_MAINT_READ_LOCAL/RIO_MPORT_MAINT_WRITE_LOCAL- 向远RapidIO 设备的配置寄存器读取和写入  这些操作RIO 规范中被定义RapidIO 维护写  （RIO_MPORT_MAINT_READ_REMOTE/RIO_MPORT_MAINT_WRITE_REMOTE- mport 设备设置 RapidIO 目标 ID（RIO_MPORT_MAINT_HDID_SET- mport 设备设置 RapidIO 组件标签（Component Tag  （RIO_MPORT_MAINT_COMPTAG_SET- 查询 mport 设备的逻辑索引（RIO_MPORT_MAINT_PORT_IDX_GET- 查询 mport 设备的能力与 RapidIO 链路配置
  （RIO_MPORT_GET_PROPERTIES- 启用/禁用向用户空间应用程序报RapidIO 门铃（doorbell）事  （RIO_ENABLE_DOORBELL_RANGE/RIO_DISABLE_DOORBELL_RANGE- 启用/禁用向用户空间应用程序报RIO 端口写（port-write）事  （RIO_ENABLE_PORTWRITE_RANGE/RIO_DISABLE_PORTWRITE_RANGE- 查询/控制通过该驱动报告的事件类型：门铃、端口写或两  （RIO_SET_EVENT_MASK/RIO_GET_EVENT_MASK- 为特定大小、RapidIO 目标 ID、跳数（hopcount）与请求类型配置/映射 mport   出站请求窗口（RIO_MAP_OUTBOUND/RIO_UNMAP_OUTBOUND- 为特定大小、RapidIO 基地址与本地内存基地址配置/映射 mport   入站请求窗口（RIO_MAP_INBOUND/RIO_UNMAP_INBOUND- 为与远程 RapidIO 设备DMA 数据传输分配/释放连续DMA 一致性内存缓冲区
  （RIO_ALLOC_DMA/RIO_FREE_DMA- 发起与远RapidIO 设备DMA 数据传输（RIO_TRANSFER）  支持阻塞、异步与 posted（即“发射后不管”）数据传输模式- 检等待异步 DMA 数据传输完成（RIO_WAIT_FOR_ASYNC- 管理 RapidIO 子系统支持的设备对象（RIO_DEV_ADD/RIO_DEV_DEL）  这允许将各种 RapidIO 结构（fabric）枚举算法实现为用户空间应用程序  同时使用内核 RapidIO 子系统提供的其余功能
## 2. 硬件兼容

该设备驱动使用内RapidIO 子系统定义的标准接口，因此它可以与任何由
RapidIO 子系统注册的 mport 设备驱动一起使用，限制由可用的 mport 实现设置
目前最常见的限制是特定 mport 设备是否有可用的 RapidIO 专用
DMA 引擎框架。用户在计划使用该驱动时应验证其平台可用功能
- IDT Tsi721 PCIe RapidIO 桥接设备及其 mport 设备驱动与该驱动完全兼容- Freescale SoC 'fsl_rio' mport 驱动没有实现 RapidIO 专用 DMA 引擎支持  因此 mport_cdev 驱动DMA 数据传输不可用
## 3. 模块参数


- 'dma_timeout'
      - DMA 传输完成超时（以毫秒计，默认3000）        该参数设SYNC 模式 DMA 传输请求RIO_WAIT_FOR_ASYNC
        ioctl 请求的最大完成等待时间
- 'dbg_level'
      - 该参数允许控制该设备驱动生成的调试信息量。该参数由一        对应于特定功能块的位掩码构成        有关掩码定义请参'drivers/rapidio/devices/rio_mport_cdev.c'
        该参数可以动态更改        使用 CONFIG_RAPIDIO_DEBUG=y 以在顶层启用调试输出
## 4. 已知问题


  无
## 5. 用户空间应用程序API


使用此设备驱动的 API 库与应用程序可从 RapidIO.org 获取
## 6. 待办列表（TODO List

- 添加对发接收“原始”RapidIO 消息数据包的支持- RapidIO 专用 DMA 不可用时，添加内存映射的 DMA 数据传输作为选项