
## 术语表（Glossary

   本节的目标是统一 media 用户空间 API 文档中使用的术语。目前仍是进行中的工作（Work In Progress）

    Bridge Driver（桥接驱动）
	一*设备驱动（Device Driver*，实现与媒体硬件通信的主要逻辑
    CEC API
	**消费电子控制 API（Consumer Electronics Control API*

	一种设计用于通过 HDMI CEC 接口接收和发送数据的 API
	参见 cec
    Data Unit（数据单元）

	总线所传输的数据单元。在并行总线上，数据单元由一个或多个相关采样组成；在串行总线上，数据单元是逻辑单元。如果数据单元为图像数据，也可以称为像素（pixel）
    Device Driver（设备驱动）
	Linux 内核的一部分，实现对某个硬件组件的支持
    Device Node（设备节点）
	文件系统中的字符设备节点，用于控制内核驱动并与之进行数据收发
    Digital TV API（数字电API	**曾被称为 DVB API**

	一种设计用于控**Media Hardware（媒体硬件）** 中实现了数字电视（例DVB、ATSC、ISDB 等）的子集的 API
	参见 dvbapi
    DSP（数字信号处理器        **Digital Signal Processor**

	一种专用的**微处理器（Microprocessor*，其架构针对数字信号处理的运算需求进行了优化
    FPGA（现场可编程门阵列）
	**Field-programmable Gate Array**

	一*IC（集成电路）**电路，可在制造完成后由客户或设计者进行配置
	参见 https://en.wikipedia.org/wiki/Field-programmable_gate_array
    Hardware Component（硬件组件）
	**Media Hardware（媒体硬件）**的子集。例如一**I²C** **SPI** 设备，或 **SoC** **FPGA** 内部**IP Block（IP 模块*
    Hardware Peripheral（硬件外设）
	一**Hardware Component（硬件组件）**，共同构成一个面向用户的更大功能外设。例如，**SoC** **ISP** **IP Block** 与外部摄像头传感器共同构成一个摄像头硬件外设
	也称**Peripheral（外设）**
    I虏C
	**Inter-Integrated Circuit（集成电路间总线*

	一种多主多从、包交换、单端、串行的计算机总线，用于控制部分硬件组件（如子设备硬件组件）
	参见 http://www.nxp.com/docs/en/user-guide/UM10204.pdf
    IC（集成电路）
	**Integrated circuit（集成电路）**

	制作在一小片扁平半导体材料（通常为硅）上的电子电路集合
	也称为芯片（chip）
    IP Block（IP 模块	**Intellectual property core（知识产权核*

	在电子设计中，半导体知识产权核是一方可复用的逻辑、单元或集成电路版图设计，属于某一方的知识产权。IP Block 可以授权给另一方使用，也可以由单方独自拥有和使用
	参见 https://en.wikipedia.org/wiki/Semiconductor_intellectual_property_core
    ISP（图像信号处理器	**Image Signal Processor**

	一种专用处理器，实现一组用于处理图像数据的算法。ISP 可能实现镜头阴影校正、去马赛克、缩放和像素格式转换等算法，并为控制算法（例如自动曝光、白平衡和对焦）生成统计信息
    Media API（媒API	一组用于控制媒体硬件的用户空间 API。它由以下部分组成：

   - **CEC API**   - **Digital TV API**   - **MC API**   - **RC API**；以   - **V4L2 API**
	参见 Documentation/userspace-api/media/index.rst
    MC API（媒体控制器 API	**Media Controller API**

	一种设计用于暴露并控制多媒体设备与子设备之间关系的 API
	参见 media_controller
    MC-centric（以媒体控制器为中心	需**MC API** **V4L2 Hardware（V4L2 硬件*设备驱动
	此类驱动会将 `V4L2_CAP_IO_MC` device_caps 字段置位（参VIDIOC_QUERYCAP）
	详见 v4l2_hardware_control
    Media Hardware（媒体硬件）
	Linux Media API 支持的硬件子集
	包括音视频采集与回放硬件、数字与模拟电视、摄像头传感器、ISP、遥控控制器、编解码器、HDMI 消费电子控制、HDMI 采集等
    Microprocessor（微处理器）
	执行计算机程序指令的电子电路，通过对单一集成电路上指令所指定的基本算术、逻辑、控制以及输输出（I/O）操作进行处理来完成
    Peripheral（外设）
	等同**Hardware Peripheral（硬件外设）**
    RC API（遥控控制器 API	**Remote Controller API**

	一种设计用于接收和发送来自遥控控制器的数据的 API
	参见 remote_controllers
    SMBus
	I²C 的一个子集，对总线的使用方式定义了更严格的规范
    SPI（串行外设接口总线	**Serial Peripheral Interface Bus**

	一种同步串行通信接口规范，用于短距离通信，主要应用于嵌入式系统
    SoC（片上系统）
	**System on a Chip**

	一种将计算机或其他电子系统的所有组件集成在一起的集成电路
    Stream（数据流	从初始源到最终汇的一路独立数据流（图像数据或元数据）。初始源可以是例如图像传感器，最终汇可以是例如内存缓冲区
    V4L2 API
	**V4L2 userspace API（V4L2 用户空间 API*

	v4l2spec 中定义的用户空间 API，用于控V4L2 硬件
    V4L2 Device Node（V4L2 设备节点	V4L 驱动相关联的 **Device Node（设备节点）**
	V4L2 设备节点的命名规范见 v4l2_device_naming
    V4L2 Hardware（V4L2 硬件	**V4L2 API** 支持的媒体硬件的一部分
    V4L2 Sub-device（V4L2 子设备）
	不受 **Bridge Driver（桥接驱动）**控制V4L2 硬件组件。参subdev
    Video-node-centric（以视频节点为中心）
	不需要使用媒体控制器即可工作V4L2 设备驱动
	此类驱动会将 `V4L2_CAP_IO_MC` device_caps 字段清零（参VIDIOC_QUERYCAP）
    V4L2 Sub-device API（V4L2 子设API	**V4L2 API** 中用于控**V4L2 sub-devices（V4L2 子设备）**（如传感器、HDMI 接收器、缩放器、去隔行器）的部分
	详见 v4l2_hardware_control