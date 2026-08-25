
## IBM Virtual Management Channel Kernel Driver (IBMVMC)


:作
	Dave Engebretsen <engebret@us.ibm.com>,
	Adam Reznechek <adreznec@linux.vnet.ibm.com>,
	Steven Royer <seroyer@linux.vnet.ibm.com>,
	Bryant G. Ly <bryantly@linux.vnet.ibm.com>,

## Introduction


注意：理解本文档需要具备虚拟化技术知识

一份很好的参考文档是

https://openpowerfoundation.org/wp-content/uploads/2016/05/LoPAPR_DRAFT_v11_24March2016_cmt1.pdf

虚拟管理通道（Virtual Management Channel，VMC）是一个逻辑设备，它提供管理程序
（hypervisor）与管理分区之间的接口。该接口类似于消息传递接口。该管理分区旨在为使
基于硬件管理控制台（HMC）的系统管理方式的系统提供一种替代方案

IBM 开发的主要硬件管理解决方案依赖于一个名为硬件管理控制台（Hardware Management
Console，HMC）的设备服务器，它打包为一个外置塔式或机架式个人计算机。在 Power Systems
环境中，单个 HMC 可以管理多个基于 POWER 处理器的系统

### Management Application


在管理分区中，存在一个管理应用，使系统管理员能够通过命令行接口（CLI）或表述性状态转
应用（REST API）配置系统的分区特性

该管理应用运行在PowerVM 虚拟化的、基POWER8 或更新处理器的服务器上的 Linux 逻辑分区
中。传统上需HMC 的系统配置、维护和控制功能，可以在管理应用中使HMC 到管理程序接
与既有操作系统方法的组合来实现。该工具提供 HMC 所实现功能的一个子集，并支持基本的分区
配置。管理应用组件所支持HMC 到管理程序的消息集合，通过如下定义VMC 接口传递给
管理程序

VMC 使管理分区能够提供基本的分区功能

- 逻辑分区配置
- 各个分区的启动和停止操作
- 显示分区状
- 虚拟以太网管
- 虚拟存储管理
- 基本系统管理

### Virtual Management Channel (VMC)


定义了一个称为虚拟管理通道（Virtual Management Channel，VMC）的逻辑设备，用于管理应用与
管理程序之间的通信。它基本上创建了使虚拟化管理工作成为可能的管道。该设备作为一个虚拟设
呈现给指定的管理分区

该通信设备使用命令/响应队列（CRQ）和远程直接内存访问（RDMA）接口。定义了一个三方握
过程，必须在发接收任何协议消息之前完成，以确认通道的管理程序端和管理分区端都在运行

该驱动还使用传输事件 CRQ（Transport Event CRQ）。当管理程序检测到某个对等分区异常终止
或某一方调H_FREE_CRQ 关闭CRQ 时，会发CRQ 消息。为 VMC 设备引入了两类新CRQ
消息。VMC 管理消息（Administrative message）用于每个使VMC 的分区向其对等方通告能力
HMC 接口消息用于管理分区与管理程序之间实际的 HMC 消息流。由于大多数 HMC 消息远大CRQ
缓冲区的大小，在每个 HMC 接口 CRQ 消息之前都会HMC 消息数据进行一次虚DMA（RMDA）
只有管理分区驱动 RDMA 操作；管理程序绝不直接导致消息数据的移动


### Terminology

RDMA
        Remote Direct Memory Access 是从服务器到其客户端或从服务器到其对等分区的 DMA
        传输。DMA 既指进出内存的物I/O 操作，也指内存到内存的搬移操作
CRQ
        Command/Response Queue，一种用于在对等分区之间通信的设施。由管理程序向分
        发出的传输事件也在此队列中报告

## Example Management Partition VMC Driver Interface


本节为管理应用的实现提供了一个示例，其中使用一个设备驱动来对接 VMC 设备。该驱动包含一
个新设备，例/dev/ibmvmc，它提供VMC 设备进行 open、close、read、write 以及执行
ioctl 的接口

### VMC Interface Initialization


设备驱动负责在加载驱动时初始VMC。它首先创建并初始化 CRQ。接下来，进VMC 能力交换
以指示管理分区和管理程序中双方的代码版本和可用资源数量。最后，管理程序请求管理分区创建
一个初始的 VMC 缓冲区池，每个可能的 HMC 连接一个缓冲区，用于管理应用会话初始化。在完成
此初始化序列之前，设备对 open() 调用返回 EBUSY。所open() 失败都返EIO

```

        Management Partition		Hypervisor
                        CRQ INIT
        ---------------------------------------->
        	   CRQ INIT COMPLETE
        <----------------------------------------
        	      CAPABILITIES
        ---------------------------------------->
        	 CAPABILITIES RESPONSE
        <----------------------------------------
              ADD BUFFER (HMC IDX=0,1,..)         _
        <----------------------------------------  |
        	  ADD BUFFER RESPONSE              | - Perform # HMCs Iterations
        ----------------------------------------> -

```
### VMC Interface Open


在基VMC 通道初始化完成后，可以建HMC 会话级连接。应用层VMC 设备执行 open() 
对其执行 ioctl()，指示此会话HMC ID2 字节数据）。如VMC 设备处于无效状态，ioctl()
将返EIO。设备驱动为HMC ID 创建一个新HMC 会话值（范围 1 255）和 HMC 索引
（从索引 0 开始，范围254）。驱动随后将 HMC ID 通过 RDMA 传给管理程序，然后向管理程序
发送一个接口打开（Interface Open）消息，以通过 VMC 建立会话。管理程序收到此信息后，向管
分区发送添加缓冲区（Add Buffer）消息，为新 HMC 连接播下初始缓冲区池。最后，管理程序发
一个接口打开响应（Interface Open Response）消息，表明它已准备好进行正常的运行时消息传递
下面的流程说明了这一 VMC 交互

```

        Management Partition             Hypervisor
        	      RDMA HMC ID
        ---------------------------------------->
        	    Interface Open
        ---------------------------------------->
        	      Add Buffer                  _
        <----------------------------------------  |
        	  Add Buffer Response              | - Perform N Iterations
        ----------------------------------------> -
        	Interface Open Response
        <----------------------------------------

```
### VMC Interface Runtime


在正常运行时，管理应用与管理程序通过 Signal VMC 消息RDMA 操作交换 HMC 消息。向管理程序
发送数据时，管理应用对 VMC 设备执行 write()，驱动将数据进行 RDMA 传给管理程序，然后发
一Signal Message。如果在管理程序VMC 设备缓冲区可用之前尝write()，或者当前没有可
缓冲区，write() 会返EBUSY。对于所有其他错误（例如无效的设备状态），write() 返回 EIO
当管理程序向管理方发送消息时，数据被放入一VMC 缓冲区，并向管理分区中的 VMC 驱动发送一
Signal Message。驱动将缓冲RDMA 到分区中，并通过VMC 设备read() 将数据向上传递给
相应的管理应用。如果没有可供读取的缓冲区，read() 请求会阻塞。管理应用可以使select() 等待
VMC 设备准备好可供读取的数据

```

        Management Partition             Hypervisor
        		MSG RDMA
        ---------------------------------------->
        		SIGNAL MSG
        ---------------------------------------->
        		SIGNAL MSG
        <----------------------------------------
        		MSG RDMA
        <----------------------------------------

```
### VMC Interface Close


当应用层对设备执close() 时，管理分区关闭 HMC 会话级连接。此动作导致一个接口关
（Interface Close）消息流向管理程序，从而终止会话。设备驱动必须释放为HMC 连接分配
缓冲区存储

```

        Management Partition             Hypervisor
        	     INTERFACE CLOSE
        ---------------------------------------->
                INTERFACE CLOSE RESPONSE
        <----------------------------------------

```
## Additional Information


有关 CRQ 消息、VMC 消息、HMC 接口缓冲区以signal 消息的文档的更多信息，请参阅 Linux on
Power Architecture Platform Reference F 节
